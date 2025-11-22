pub fn sub_82FD8308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD8308 size=16
    let mut pc: u32 = 0x82FD8308;
    'dispatch: loop {
        match pc {
            0x82FD8308 => {
    //   block [0x82FD8308..0x82FD8318)
	// 82FD8308: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FD830C: 396B977C  addi r11, r11, -0x6884
	ctx.r[11].s64 = ctx.r[11].s64 + -26756;
	// 82FD8310: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FD8314: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD8318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD8318 size=68
    let mut pc: u32 = 0x82FD8318;
    'dispatch: loop {
        match pc {
            0x82FD8318 => {
    //   block [0x82FD8318..0x82FD835C)
	// 82FD8318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD831C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD8320: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FD8324: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD8328: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FD832C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FD8330: 396B977C  addi r11, r11, -0x6884
	ctx.r[11].s64 = ctx.r[11].s64 + -26756;
	// 82FD8334: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82FD8338: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FD833C: 41820008  beq 0x82fd8344
	if ctx.cr[0].eq {
	pc = 0x82FD8344; continue 'dispatch;
	}
	// 82FD8340: 4BFFFFA1  bl 0x82fd82e0
	ctx.lr = 0x82FD8344;
	sub_82FD82E0(ctx, base);
	// 82FD8344: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FD8348: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FD834C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD8350: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD8354: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FD8358: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD8360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD8360 size=8
    let mut pc: u32 = 0x82FD8360;
    'dispatch: loop {
        match pc {
            0x82FD8360 => {
    //   block [0x82FD8360..0x82FD8368)
	// 82FD8360: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FD8364: 82139790  lwz r16, -0x6870(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-26736 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD8368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD8368 size=104
    let mut pc: u32 = 0x82FD8368;
    'dispatch: loop {
        match pc {
            0x82FD8368 => {
    //   block [0x82FD8368..0x82FD83D0)
	// 82FD8368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD836C: 481CFDF1  bl 0x831a815c
	ctx.lr = 0x82FD8370;
	sub_831A8130(ctx, base);
	// 82FD8370: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 82FD8374: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD8378: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FD837C: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82FD8380: 80DF00E4  lwz r6, 0xe4(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 82FD8384: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 82FD8388: 7D1B4378  mr r27, r8
	ctx.r[27].u64 = ctx.r[8].u64;
	// 82FD838C: 7D3A4B78  mr r26, r9
	ctx.r[26].u64 = ctx.r[9].u64;
	// 82FD8390: 93DF00A4  stw r30, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[30].u32 ) };
	// 82FD8394: 7D595378  mr r25, r10
	ctx.r[25].u64 = ctx.r[10].u64;
	// 82FD8398: 48000B99  bl 0x82fd8f30
	ctx.lr = 0x82FD839C;
	sub_82FD8F30(ctx, base);
	// 82FD839C: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD83A0: 7F28CB78  mr r8, r25
	ctx.r[8].u64 = ctx.r[25].u64;
	// 82FD83A4: 396B7054  addi r11, r11, 0x7054
	ctx.r[11].s64 = ctx.r[11].s64 + 28756;
	// 82FD83A8: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 82FD83AC: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82FD83B0: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82FD83B4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FD83B8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD83BC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FD83C0: 48000F79  bl 0x82fd9338
	ctx.lr = 0x82FD83C4;
	sub_82FD9338(ctx, base);
	// 82FD83C4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD83C8: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 82FD83CC: 481CFDE0  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD83D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD83D0 size=40
    let mut pc: u32 = 0x82FD83D0;
    'dispatch: loop {
        match pc {
            0x82FD83D0 => {
    //   block [0x82FD83D0..0x82FD83F8)
	// 82FD83D0: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FD83D4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD83D8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD83DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD83E0: 807F00A4  lwz r3, 0xa4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 82FD83E4: 48000A95  bl 0x82fd8e78
	ctx.lr = 0x82FD83E8;
	sub_82FD8E78(ctx, base);
	// 82FD83E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FD83EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD83F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD83F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD83F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD83F8 size=8
    let mut pc: u32 = 0x82FD83F8;
    'dispatch: loop {
        match pc {
            0x82FD83F8 => {
    //   block [0x82FD83F8..0x82FD8400)
	// 82FD83F8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FD83FC: 82139818  lwz r16, -0x67e8(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-26600 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD8400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD8400 size=204
    let mut pc: u32 = 0x82FD8400;
    'dispatch: loop {
        match pc {
            0x82FD8400 => {
    //   block [0x82FD8400..0x82FD84CC)
	// 82FD8400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD8404: 481CFD65  bl 0x831a8168
	ctx.lr = 0x82FD8408;
	sub_831A8130(ctx, base);
	// 82FD8408: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 82FD840C: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD8410: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FD8414: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82FD8418: 93DF00B4  stw r30, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[30].u32 ) };
	// 82FD841C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FD8420: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82FD8424: 90BE0014  stw r5, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[5].u32 ) };
	// 82FD8428: 396B9800  addi r11, r11, -0x6800
	ctx.r[11].s64 = ctx.r[11].s64 + -26624;
	// 82FD842C: 394003FF  li r10, 0x3ff
	ctx.r[10].s64 = 1023;
	// 82FD8430: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82FD8434: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FD8438: 93BE000C  stw r29, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 82FD843C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FD8440: 93BE0004  stw r29, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82FD8444: 93BE0008  stw r29, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82FD8448: 915E0010  stw r10, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82FD844C: 4801C88D  bl 0x82ff4cd8
	ctx.lr = 0x82FD8450;
	sub_82FF4CD8(ctx, base);
	// 82FD8450: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD8454: 907E0004  stw r3, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82FD8458: 40820044  bne 0x82fd849c
	if !ctx.cr[0].eq {
	pc = 0x82FD849C; continue 'dispatch;
	}
	// 82FD845C: 80DE0014  lwz r6, 0x14(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FD8460: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FD8464: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FD8468: 388B97C0  addi r4, r11, -0x6840
	ctx.r[4].s64 = ctx.r[11].s64 + -26688;
	// 82FD846C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82FD8470: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82FD8474: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 82FD8478: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82FD847C: 38C0001F  li r6, 0x1f
	ctx.r[6].s64 = 31;
	// 82FD8480: 38A00048  li r5, 0x48
	ctx.r[5].s64 = 72;
	// 82FD8484: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FD8488: 4BFFFEE1  bl 0x82fd8368
	ctx.lr = 0x82FD848C;
	sub_82FD8368(ctx, base);
	// 82FD848C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FD8490: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FD8494: 388BC514  addi r4, r11, -0x3aec
	ctx.r[4].s64 = ctx.r[11].s64 + -15084;
	// 82FD8498: 481D8791  bl 0x831b0c28
	ctx.lr = 0x82FD849C;
	sub_831B0C28(ctx, base);
	// 82FD849C: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FD84A0: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FD84A4: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 82FD84A8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD84AC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FD84B0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD84B4: 4E800421  bctrl
	ctx.lr = 0x82FD84B8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD84B8: 907E0008  stw r3, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 82FD84BC: 9BA30000  stb r29, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[29].u8 ) };
	// 82FD84C0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD84C4: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 82FD84C8: 481CFCF0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD84CC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD84CC size=40
    let mut pc: u32 = 0x82FD84CC;
    'dispatch: loop {
        match pc {
            0x82FD84CC => {
    //   block [0x82FD84CC..0x82FD84F4)
	// 82FD84CC: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 82FD84D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD84D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD84D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD84DC: 807F00B4  lwz r3, 0xb4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 82FD84E0: 4BFFFE29  bl 0x82fd8308
	ctx.lr = 0x82FD84E4;
	sub_82FD8308(ctx, base);
	// 82FD84E4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FD84E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD84EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD84F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD84F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD84F8 size=124
    let mut pc: u32 = 0x82FD84F8;
    'dispatch: loop {
        match pc {
            0x82FD84F8 => {
    //   block [0x82FD84F8..0x82FD8574)
	// 82FD84F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD84FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD8500: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FD8504: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD8508: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FD850C: 80DF0014  lwz r6, 0x14(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FD8510: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FD8514: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD8518: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FD851C: 4801C93D  bl 0x82ff4e58
	ctx.lr = 0x82FD8520;
	sub_82FF4E58(ctx, base);
	// 82FD8520: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FD8524: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD8528: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82FD852C: 996A0000  stb r11, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82FD8530: 813F000C  lwz r9, 0xc(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FD8534: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD8538: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82FD853C: 996A0001  stb r11, 1(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(1 as u32), ctx.r[11].u8 ) };
	// 82FD8540: 813F000C  lwz r9, 0xc(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FD8544: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD8548: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82FD854C: 996A0002  stb r11, 2(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(2 as u32), ctx.r[11].u8 ) };
	// 82FD8550: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD8554: 813F000C  lwz r9, 0xc(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FD8558: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82FD855C: 996A0003  stb r11, 3(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(3 as u32), ctx.r[11].u8 ) };
	// 82FD8560: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FD8564: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD8568: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD856C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FD8570: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD8578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD8578 size=132
    let mut pc: u32 = 0x82FD8578;
    'dispatch: loop {
        match pc {
            0x82FD8578 => {
    //   block [0x82FD8578..0x82FD85FC)
	// 82FD8578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD857C: 481CFBF1  bl 0x831a816c
	ctx.lr = 0x82FD8580;
	sub_831A8130(ctx, base);
	// 82FD8580: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD8584: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FD8588: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FD858C: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FD8590: 7D6B2214  add r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82FD8594: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82FD8598: 4198005C  blt cr6, 0x82fd85f4
	if ctx.cr[6].lt {
	pc = 0x82FD85F4; continue 'dispatch;
	}
	// 82FD859C: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FD85A0: 557E083C  slwi r30, r11, 1
	ctx.r[30].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82FD85A4: 389E0004  addi r4, r30, 4
	ctx.r[4].s64 = ctx.r[30].s64 + 4;
	// 82FD85A8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD85AC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FD85B0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD85B4: 4E800421  bctrl
	ctx.lr = 0x82FD85B8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD85B8: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FD85BC: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD85C0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FD85C4: 38AB0004  addi r5, r11, 4
	ctx.r[5].s64 = ctx.r[11].s64 + 4;
	// 82FD85C8: 481CFF49  bl 0x831a8510
	ctx.lr = 0x82FD85CC;
	sub_831A8510(ctx, base);
	// 82FD85CC: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FD85D0: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD85D4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD85D8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD85DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD85E0: 4E800421  bctrl
	ctx.lr = 0x82FD85E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD85E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FD85E8: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82FD85EC: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 82FD85F0: 4BFFFF09  bl 0x82fd84f8
	ctx.lr = 0x82FD85F4;
	sub_82FD84F8(ctx, base);
	// 82FD85F4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FD85F8: 481CFBC4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD8600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD8600 size=8
    let mut pc: u32 = 0x82FD8600;
    'dispatch: loop {
        match pc {
            0x82FD8600 => {
    //   block [0x82FD8600..0x82FD8608)
	// 82FD8600: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FD8604: 82139850  lwz r16, -0x67b0(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-26544 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD8608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD8608 size=128
    let mut pc: u32 = 0x82FD8608;
    'dispatch: loop {
        match pc {
            0x82FD8608 => {
    //   block [0x82FD8608..0x82FD8688)
	// 82FD8608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD860C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD8610: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FD8614: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FD8618: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 82FD861C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD8620: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FD8624: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FD8628: 396B9800  addi r11, r11, -0x6800
	ctx.r[11].s64 = ctx.r[11].s64 + -26624;
	// 82FD862C: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 82FD8630: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FD8634: 4BFFFEC5  bl 0x82fd84f8
	ctx.lr = 0x82FD8638;
	sub_82FD84F8(ctx, base);
	// 82FD8638: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FD863C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD8640: 4182000C  beq 0x82fd864c
	if ctx.cr[0].eq {
	pc = 0x82FD864C; continue 'dispatch;
	}
	// 82FD8644: 809E0014  lwz r4, 0x14(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FD8648: 4801C4A9  bl 0x82ff4af0
	ctx.lr = 0x82FD864C;
	sub_82FF4AF0(ctx, base);
	// 82FD864C: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FD8650: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD8654: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD8658: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD865C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD8660: 4E800421  bctrl
	ctx.lr = 0x82FD8664;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD8664: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FD8668: 396B977C  addi r11, r11, -0x6884
	ctx.r[11].s64 = ctx.r[11].s64 + -26756;
	// 82FD866C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FD8670: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 82FD8674: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD8678: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD867C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FD8680: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FD8684: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD8688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD8688 size=40
    let mut pc: u32 = 0x82FD8688;
    'dispatch: loop {
        match pc {
            0x82FD8688 => {
    //   block [0x82FD8688..0x82FD86B0)
	// 82FD8688: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 82FD868C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD8690: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD8694: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD8698: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82FD869C: 4BFFFC6D  bl 0x82fd8308
	ctx.lr = 0x82FD86A0;
	sub_82FD8308(ctx, base);
	// 82FD86A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FD86A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD86A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD86AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD86B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD86B0 size=4
    let mut pc: u32 = 0x82FD86B0;
    'dispatch: loop {
        match pc {
            0x82FD86B0 => {
    //   block [0x82FD86B0..0x82FD86B4)
	// 82FD86B0: 4BFFFE48  b 0x82fd84f8
	sub_82FD84F8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD86B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD86B8 size=84
    let mut pc: u32 = 0x82FD86B8;
    'dispatch: loop {
        match pc {
            0x82FD86B8 => {
    //   block [0x82FD86B8..0x82FD870C)
	// 82FD86B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD86BC: 481CFAB1  bl 0x831a816c
	ctx.lr = 0x82FD86C0;
	sub_831A8130(ctx, base);
	// 82FD86C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD86C4: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82FD86C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FD86CC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82FD86D0: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82FD86D4: 419A0030  beq cr6, 0x82fd8704
	if ctx.cr[6].eq {
	pc = 0x82FD8704; continue 'dispatch;
	}
	// 82FD86D8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FD86DC: 4BFFFE9D  bl 0x82fd8578
	ctx.lr = 0x82FD86E0;
	sub_82FD8578(ctx, base);
	// 82FD86E0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD86E4: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FD86E8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82FD86EC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FD86F0: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82FD86F4: 481CFE1D  bl 0x831a8510
	ctx.lr = 0x82FD86F8;
	sub_831A8510(ctx, base);
	// 82FD86F8: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FD86FC: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82FD8700: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82FD8704: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FD8708: 481CFAB4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD8710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD8710 size=76
    let mut pc: u32 = 0x82FD8710;
    'dispatch: loop {
        match pc {
            0x82FD8710 => {
    //   block [0x82FD8710..0x82FD875C)
	// 82FD8710: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD8714: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD8718: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FD871C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FD8720: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD8724: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FD8728: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FD872C: 4BFFFEDD  bl 0x82fd8608
	ctx.lr = 0x82FD8730;
	sub_82FD8608(ctx, base);
	// 82FD8730: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FD8734: 4182000C  beq 0x82fd8740
	if ctx.cr[0].eq {
	pc = 0x82FD8740; continue 'dispatch;
	}
	// 82FD8738: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FD873C: 4BFFFBA5  bl 0x82fd82e0
	ctx.lr = 0x82FD8740;
	sub_82FD82E0(ctx, base);
	// 82FD8740: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FD8744: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FD8748: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD874C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD8750: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FD8754: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FD8758: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD8760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD8760 size=80
    let mut pc: u32 = 0x82FD8760;
    'dispatch: loop {
        match pc {
            0x82FD8760 => {
    //   block [0x82FD8760..0x82FD87B0)
	// 82FD8760: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD8764: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD8768: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FD876C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD8770: 3FE08339  lis r31, -0x7cc7
	ctx.r[31].s64 = -2093416448;
	// 82FD8774: 807FB7F0  lwz r3, -0x4810(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-18448 as u32) ) } as u64;
	// 82FD8778: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82FD877C: 419A0018  beq cr6, 0x82fd8794
	if ctx.cr[6].eq {
	pc = 0x82FD8794; continue 'dispatch;
	}
	// 82FD8780: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD8784: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82FD8788: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD878C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD8790: 4E800421  bctrl
	ctx.lr = 0x82FD8794;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD8794: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FD8798: 917FB7F0  stw r11, -0x4810(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-18448 as u32), ctx.r[11].u32 ) };
	// 82FD879C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FD87A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD87A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD87A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FD87AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD87B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD87B0 size=84
    let mut pc: u32 = 0x82FD87B0;
    'dispatch: loop {
        match pc {
            0x82FD87B0 => {
    //   block [0x82FD87B0..0x82FD8804)
	// 82FD87B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD87B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD87B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FD87BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FD87C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD87C4: 3FC08339  lis r30, -0x7cc7
	ctx.r[30].s64 = -2093416448;
	// 82FD87C8: 83FEB7F4  lwz r31, -0x480c(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18444 as u32) ) } as u64;
	// 82FD87CC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82FD87D0: 419A0014  beq cr6, 0x82fd87e4
	if ctx.cr[6].eq {
	pc = 0x82FD87E4; continue 'dispatch;
	}
	// 82FD87D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FD87D8: 4801CFB1  bl 0x82ff5788
	ctx.lr = 0x82FD87DC;
	sub_82FF5788(ctx, base);
	// 82FD87DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FD87E0: 4BFFFB01  bl 0x82fd82e0
	ctx.lr = 0x82FD87E4;
	sub_82FD82E0(ctx, base);
	// 82FD87E4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FD87E8: 917EB7F4  stw r11, -0x480c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-18444 as u32), ctx.r[11].u32 ) };
	// 82FD87EC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FD87F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD87F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD87F8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FD87FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FD8800: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD8808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD8808 size=8
    let mut pc: u32 = 0x82FD8808;
    'dispatch: loop {
        match pc {
            0x82FD8808 => {
    //   block [0x82FD8808..0x82FD8810)
	// 82FD8808: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FD880C: 82139890  lwz r16, -0x6770(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-26480 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD8810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD8810 size=164
    let mut pc: u32 = 0x82FD8810;
    'dispatch: loop {
        match pc {
            0x82FD8810 => {
    //   block [0x82FD8810..0x82FD88B4)
	// 82FD8810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD8814: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD8818: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FD881C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FD8820: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 82FD8824: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD8828: 3FC08339  lis r30, -0x7cc7
	ctx.r[30].s64 = -2093416448;
	// 82FD882C: 807EB7F4  lwz r3, -0x480c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18444 as u32) ) } as u64;
	// 82FD8830: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82FD8834: 409A0068  bne cr6, 0x82fd889c
	if !ctx.cr[6].eq {
	pc = 0x82FD889C; continue 'dispatch;
	}
	// 82FD8838: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 82FD883C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FD8840: 808BB7EC  lwz r4, -0x4814(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18452 as u32) ) } as u64;
	// 82FD8844: 4801CF95  bl 0x82ff57d8
	ctx.lr = 0x82FD8848;
	sub_82FF57D8(ctx, base);
	// 82FD8848: 817EB7F4  lwz r11, -0x480c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18444 as u32) ) } as u64;
	// 82FD884C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FD8850: 409A0040  bne cr6, 0x82fd8890
	if !ctx.cr[6].eq {
	pc = 0x82FD8890; continue 'dispatch;
	}
	// 82FD8854: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 82FD8858: 4BFFF9F1  bl 0x82fd8248
	ctx.lr = 0x82FD885C;
	sub_82FD8248(ctx, base);
	// 82FD885C: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 82FD8860: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD8864: 41820010  beq 0x82fd8874
	if ctx.cr[0].eq {
	pc = 0x82FD8874; continue 'dispatch;
	}
	// 82FD8868: 4801CEE1  bl 0x82ff5748
	ctx.lr = 0x82FD886C;
	sub_82FF5748(ctx, base);
	// 82FD886C: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 82FD8870: 48000008  b 0x82fd8878
	pc = 0x82FD8878; continue 'dispatch;
	// 82FD8874: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82FD8878: 3D6082FE  lis r11, -0x7d02
	ctx.r[11].s64 = -2097283072;
	// 82FD887C: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 82FD8880: 913EB7F4  stw r9, -0x480c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-18444 as u32), ctx.r[9].u32 ) };
	// 82FD8884: 388B87B0  addi r4, r11, -0x7850
	ctx.r[4].s64 = ctx.r[11].s64 + -30800;
	// 82FD8888: 386AB7F8  addi r3, r10, -0x4808
	ctx.r[3].s64 = ctx.r[10].s64 + -18440;
	// 82FD888C: 4801F2AD  bl 0x82ff7b38
	ctx.lr = 0x82FD8890;
	sub_82FF7B38(ctx, base);
	// 82FD8890: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FD8894: 4801CF7D  bl 0x82ff5810
	ctx.lr = 0x82FD8898;
	sub_82FF5810(ctx, base);
	// 82FD8898: 807EB7F4  lwz r3, -0x480c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18444 as u32) ) } as u64;
	// 82FD889C: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 82FD88A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD88A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD88A8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FD88AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FD88B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD88B4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD88B4 size=40
    let mut pc: u32 = 0x82FD88B4;
    'dispatch: loop {
        match pc {
            0x82FD88B4 => {
    //   block [0x82FD88B4..0x82FD88DC)
	// 82FD88B4: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 82FD88B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD88BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD88C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD88C4: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FD88C8: 4801CF49  bl 0x82ff5810
	ctx.lr = 0x82FD88CC;
	sub_82FF5810(ctx, base);
	// 82FD88CC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FD88D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD88D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD88D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD88DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD88DC size=40
    let mut pc: u32 = 0x82FD88DC;
    'dispatch: loop {
        match pc {
            0x82FD88DC => {
    //   block [0x82FD88DC..0x82FD8904)
	// 82FD88DC: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 82FD88E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD88E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD88E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD88EC: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD88F0: 4BFFF9F1  bl 0x82fd82e0
	ctx.lr = 0x82FD88F4;
	sub_82FD82E0(ctx, base);
	// 82FD88F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FD88F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD88FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD8900: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD8908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD8908 size=8
    let mut pc: u32 = 0x82FD8908;
    'dispatch: loop {
        match pc {
            0x82FD8908 => {
    //   block [0x82FD8908..0x82FD8910)
	// 82FD8908: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FD890C: 821398E0  lwz r16, -0x6720(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-26400 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD8910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD8910 size=136
    let mut pc: u32 = 0x82FD8910;
    'dispatch: loop {
        match pc {
            0x82FD8910 => {
    //   block [0x82FD8910..0x82FD8998)
	// 82FD8910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD8914: 481CF859  bl 0x831a816c
	ctx.lr = 0x82FD8918;
	sub_831A8130(ctx, base);
	// 82FD8918: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 82FD891C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD8920: 3FA08339  lis r29, -0x7cc7
	ctx.r[29].s64 = -2093416448;
	// 82FD8924: 807DB7F0  lwz r3, -0x4810(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-18448 as u32) ) } as u64;
	// 82FD8928: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82FD892C: 409A0064  bne cr6, 0x82fd8990
	if !ctx.cr[6].eq {
	pc = 0x82FD8990; continue 'dispatch;
	}
	// 82FD8930: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82FD8934: 4BFFF915  bl 0x82fd8248
	ctx.lr = 0x82FD8938;
	sub_82FD8248(ctx, base);
	// 82FD8938: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FD893C: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82FD8940: 41820030  beq 0x82fd8970
	if ctx.cr[0].eq {
	pc = 0x82FD8970; continue 'dispatch;
	}
	// 82FD8944: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 82FD8948: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FD894C: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82FD8950: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD8954: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FD8958: 48073E99  bl 0x8304c7f0
	ctx.lr = 0x82FD895C;
	sub_8304C7F0(ctx, base);
	// 82FD895C: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 82FD8960: 394B2990  addi r10, r11, 0x2990
	ctx.r[10].s64 = ctx.r[11].s64 + 10640;
	// 82FD8964: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82FD8968: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82FD896C: 48000008  b 0x82fd8974
	pc = 0x82FD8974; continue 'dispatch;
	// 82FD8970: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FD8974: 3D4082FE  lis r10, -0x7d02
	ctx.r[10].s64 = -2097283072;
	// 82FD8978: 3D208339  lis r9, -0x7cc7
	ctx.r[9].s64 = -2093416448;
	// 82FD897C: 917DB7F0  stw r11, -0x4810(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(-18448 as u32), ctx.r[11].u32 ) };
	// 82FD8980: 388A8760  addi r4, r10, -0x78a0
	ctx.r[4].s64 = ctx.r[10].s64 + -30880;
	// 82FD8984: 3869B804  addi r3, r9, -0x47fc
	ctx.r[3].s64 = ctx.r[9].s64 + -18428;
	// 82FD8988: 4801F1B1  bl 0x82ff7b38
	ctx.lr = 0x82FD898C;
	sub_82FF7B38(ctx, base);
	// 82FD898C: 807DB7F0  lwz r3, -0x4810(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-18448 as u32) ) } as u64;
	// 82FD8990: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 82FD8994: 481CF828  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD8998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD8998 size=40
    let mut pc: u32 = 0x82FD8998;
    'dispatch: loop {
        match pc {
            0x82FD8998 => {
    //   block [0x82FD8998..0x82FD89C0)
	// 82FD8998: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FD899C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD89A0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD89A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD89A8: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FD89AC: 4BFFF935  bl 0x82fd82e0
	ctx.lr = 0x82FD89B0;
	sub_82FD82E0(ctx, base);
	// 82FD89B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FD89B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD89B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD89BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD89C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD89C0 size=8
    let mut pc: u32 = 0x82FD89C0;
    'dispatch: loop {
        match pc {
            0x82FD89C0 => {
    //   block [0x82FD89C0..0x82FD89C8)
	// 82FD89C0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FD89C4: 82139928  lwz r16, -0x66d8(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-26328 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD89C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD89C8 size=184
    let mut pc: u32 = 0x82FD89C8;
    'dispatch: loop {
        match pc {
            0x82FD89C8 => {
    //   block [0x82FD89C8..0x82FD8A80)
	// 82FD89C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD89CC: 481CF7A1  bl 0x831a816c
	ctx.lr = 0x82FD89D0;
	sub_831A8130(ctx, base);
	// 82FD89D0: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 82FD89D4: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD89D8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FD89DC: 4BFFFE35  bl 0x82fd8810
	ctx.lr = 0x82FD89E0;
	sub_82FD8810(ctx, base);
	// 82FD89E0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FD89E4: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FD89E8: 4801CDF1  bl 0x82ff57d8
	ctx.lr = 0x82FD89EC;
	sub_82FF57D8(ctx, base);
	// 82FD89EC: 4BFFFF25  bl 0x82fd8910
	ctx.lr = 0x82FD89F0;
	sub_82FD8910(ctx, base);
	// 82FD89F0: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD89F4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD89F8: 40820024  bne 0x82fd8a1c
	if !ctx.cr[0].eq {
	pc = 0x82FD8A1C; continue 'dispatch;
	}
	// 82FD89FC: 480172F5  bl 0x82fefcf0
	ctx.lr = 0x82FD8A00;
	sub_82FEFCF0(ctx, base);
	// 82FD8A00: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD8A04: 3BC30004  addi r30, r3, 4
	ctx.r[30].s64 = ctx.r[3].s64 + 4;
	// 82FD8A08: 40820008  bne 0x82fd8a10
	if !ctx.cr[0].eq {
	pc = 0x82FD8A10; continue 'dispatch;
	}
	// 82FD8A0C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82FD8A10: 4BFFFF01  bl 0x82fd8910
	ctx.lr = 0x82FD8A14;
	sub_82FD8910(ctx, base);
	// 82FD8A14: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FD8A18: 48062739  bl 0x8303b150
	ctx.lr = 0x82FD8A1C;
	sub_8303B150(ctx, base);
	// 82FD8A1C: 4BFFFEF5  bl 0x82fd8910
	ctx.lr = 0x82FD8A20;
	sub_82FD8910(ctx, base);
	// 82FD8A20: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD8A24: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD8A28: 41820038  beq 0x82fd8a60
	if ctx.cr[0].eq {
	pc = 0x82FD8A60; continue 'dispatch;
	}
	// 82FD8A2C: 3BCBFFFF  addi r30, r11, -1
	ctx.r[30].s64 = ctx.r[11].s64 + -1;
	// 82FD8A30: 4BFFFEE1  bl 0x82fd8910
	ctx.lr = 0x82FD8A34;
	sub_82FD8910(ctx, base);
	// 82FD8A34: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FD8A38: 48053E39  bl 0x8302c870
	ctx.lr = 0x82FD8A3C;
	sub_8302C870(ctx, base);
	// 82FD8A3C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD8A40: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FD8A44: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FD8A48: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD8A4C: 4E800421  bctrl
	ctx.lr = 0x82FD8A50;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD8A50: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD8A54: 40820014  bne 0x82fd8a68
	if !ctx.cr[0].eq {
	pc = 0x82FD8A68; continue 'dispatch;
	}
	// 82FD8A58: 7FCBF379  or. r11, r30, r30
	ctx.r[11].u64 = ctx.r[30].u64 | ctx.r[30].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FD8A5C: 4082FFD0  bne 0x82fd8a2c
	if !ctx.cr[0].eq {
	pc = 0x82FD8A2C; continue 'dispatch;
	}
	// 82FD8A60: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82FD8A64: 48000008  b 0x82fd8a6c
	pc = 0x82FD8A6C; continue 'dispatch;
	// 82FD8A68: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FD8A6C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FD8A70: 4801CDA1  bl 0x82ff5810
	ctx.lr = 0x82FD8A74;
	sub_82FF5810(ctx, base);
	// 82FD8A74: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD8A78: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 82FD8A7C: 481CF740  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD8A80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD8A80 size=40
    let mut pc: u32 = 0x82FD8A80;
    'dispatch: loop {
        match pc {
            0x82FD8A80 => {
    //   block [0x82FD8A80..0x82FD8AA8)
	// 82FD8A80: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FD8A84: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD8A88: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD8A8C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD8A90: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FD8A94: 4801CD7D  bl 0x82ff5810
	ctx.lr = 0x82FD8A98;
	sub_82FF5810(ctx, base);
	// 82FD8A98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FD8A9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD8AA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD8AA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD8AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD8AA8 size=8
    let mut pc: u32 = 0x82FD8AA8;
    'dispatch: loop {
        match pc {
            0x82FD8AA8 => {
    //   block [0x82FD8AA8..0x82FD8AB0)
	// 82FD8AA8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FD8AAC: 821399A8  lwz r16, -0x6658(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-26200 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD8AB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD8AB0 size=172
    let mut pc: u32 = 0x82FD8AB0;
    'dispatch: loop {
        match pc {
            0x82FD8AB0 => {
    //   block [0x82FD8AB0..0x82FD8B5C)
	// 82FD8AB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD8AB4: 481CF6B1  bl 0x831a8164
	ctx.lr = 0x82FD8AB8;
	sub_831A8130(ctx, base);
	// 82FD8AB8: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 82FD8ABC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD8AC0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FD8AC4: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82FD8AC8: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82FD8ACC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FD8AD0: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82FD8AD4: 93BF0094  stw r29, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[29].u32 ) };
	// 82FD8AD8: 48005399  bl 0x82fdde70
	ctx.lr = 0x82FD8ADC;
	sub_82FDDE70(ctx, base);
	// 82FD8ADC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FD8AE0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FD8AE4: 396B9978  addi r11, r11, -0x6688
	ctx.r[11].s64 = ctx.r[11].s64 + -26248;
	// 82FD8AE8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FD8AEC: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FD8AF0: 481A5999  bl 0x8317e488
	ctx.lr = 0x82FD8AF4;
	sub_8317E488(ctx, base);
	// 82FD8AF4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FD8AF8: 4182001C  beq 0x82fd8b14
	if ctx.cr[0].eq {
	pc = 0x82FD8B14; continue 'dispatch;
	}
	// 82FD8AFC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82FD8B00: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FD8B04: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82FD8B08: 4801CA89  bl 0x82ff5590
	ctx.lr = 0x82FD8B0C;
	sub_82FF5590(ctx, base);
	// 82FD8B0C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82FD8B10: 4800001C  b 0x82fd8b2c
	pc = 0x82FD8B2C; continue 'dispatch;
	// 82FD8B14: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FD8B18: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FD8B1C: 4BFF8065  bl 0x82fd0b80
	ctx.lr = 0x82FD8B20;
	sub_82FD0B80(ctx, base);
	// 82FD8B20: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FD8B24: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82FD8B28: 4801C649  bl 0x82ff5170
	ctx.lr = 0x82FD8B2C;
	sub_82FF5170(ctx, base);
	// 82FD8B2C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FD8B30: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FD8B34: 480052DD  bl 0x82fdde10
	ctx.lr = 0x82FD8B38;
	sub_82FDDE10(ctx, base);
	// 82FD8B38: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD8B3C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD8B40: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FD8B44: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD8B48: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD8B4C: 4E800421  bctrl
	ctx.lr = 0x82FD8B50;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD8B50: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FD8B54: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 82FD8B58: 481CF65C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD8B5C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD8B5C size=40
    let mut pc: u32 = 0x82FD8B5C;
    'dispatch: loop {
        match pc {
            0x82FD8B5C => {
    //   block [0x82FD8B5C..0x82FD8B84)
	// 82FD8B5C: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FD8B60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD8B64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD8B68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD8B6C: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FD8B70: 48005111  bl 0x82fddc80
	ctx.lr = 0x82FD8B74;
	sub_82FDDC80(ctx, base);
	// 82FD8B74: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FD8B78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD8B7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD8B80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD8B88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD8B88 size=8
    let mut pc: u32 = 0x82FD8B88;
    'dispatch: loop {
        match pc {
            0x82FD8B88 => {
    //   block [0x82FD8B88..0x82FD8B90)
	// 82FD8B88: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FD8B8C: 821399E0  lwz r16, -0x6620(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-26144 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD8B90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD8B90 size=408
    let mut pc: u32 = 0x82FD8B90;
    'dispatch: loop {
        match pc {
            0x82FD8B90 => {
    //   block [0x82FD8B90..0x82FD8D28)
	// 82FD8B90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD8B94: 481CF5C9  bl 0x831a815c
	ctx.lr = 0x82FD8B98;
	sub_831A8130(ctx, base);
	// 82FD8B98: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 82FD8B9C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD8BA0: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82FD8BA4: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82FD8BA8: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82FD8BAC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FD8BB0: 933F00A4  stw r25, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[25].u32 ) };
	// 82FD8BB4: 480052BD  bl 0x82fdde70
	ctx.lr = 0x82FD8BB8;
	sub_82FDDE70(ctx, base);
	// 82FD8BB8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FD8BBC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FD8BC0: 396B9978  addi r11, r11, -0x6688
	ctx.r[11].s64 = ctx.r[11].s64 + -26248;
	// 82FD8BC4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82FD8BC8: 91790000  stw r11, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FD8BCC: 481A58BD  bl 0x8317e488
	ctx.lr = 0x82FD8BD0;
	sub_8317E488(ctx, base);
	// 82FD8BD0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FD8BD4: 4182010C  beq 0x82fd8ce0
	if ctx.cr[0].eq {
	pc = 0x82FD8CE0; continue 'dispatch;
	}
	// 82FD8BD8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD8BDC: 4801C305  bl 0x82ff4ee0
	ctx.lr = 0x82FD8BE0;
	sub_82FF4EE0(ctx, base);
	// 82FD8BE0: 7C7C1B79  or. r28, r3, r3
	ctx.r[28].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82FD8BE4: 41820034  beq 0x82fd8c18
	if ctx.cr[0].eq {
	pc = 0x82FD8C18; continue 'dispatch;
	}
	// 82FD8BE8: A17C0000  lhz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD8BEC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD8BF0: 41820028  beq 0x82fd8c18
	if ctx.cr[0].eq {
	pc = 0x82FD8C18; continue 'dispatch;
	}
	// 82FD8BF4: 397C0002  addi r11, r28, 2
	ctx.r[11].s64 = ctx.r[28].s64 + 2;
	// 82FD8BF8: 48000008  b 0x82fd8c00
	pc = 0x82FD8C00; continue 'dispatch;
	// 82FD8BFC: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FD8C00: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD8C04: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD8C08: 4082FFF4  bne 0x82fd8bfc
	if !ctx.cr[0].eq {
	pc = 0x82FD8BFC; continue 'dispatch;
	}
	// 82FD8C0C: 7D7C5850  subf r11, r28, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[28].s64;
	// 82FD8C10: 7D7A0E70  srawi r26, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[26].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82FD8C14: 48000008  b 0x82fd8c1c
	pc = 0x82FD8C1C; continue 'dispatch;
	// 82FD8C18: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82FD8C1C: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82FD8C20: 419A0034  beq cr6, 0x82fd8c54
	if ctx.cr[6].eq {
	pc = 0x82FD8C54; continue 'dispatch;
	}
	// 82FD8C24: A17B0000  lhz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD8C28: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD8C2C: 41820028  beq 0x82fd8c54
	if ctx.cr[0].eq {
	pc = 0x82FD8C54; continue 'dispatch;
	}
	// 82FD8C30: 397B0002  addi r11, r27, 2
	ctx.r[11].s64 = ctx.r[27].s64 + 2;
	// 82FD8C34: 48000008  b 0x82fd8c3c
	pc = 0x82FD8C3C; continue 'dispatch;
	// 82FD8C38: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FD8C3C: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD8C40: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD8C44: 4082FFF4  bne 0x82fd8c38
	if !ctx.cr[0].eq {
	pc = 0x82FD8C38; continue 'dispatch;
	}
	// 82FD8C48: 7D7B5850  subf r11, r27, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[27].s64;
	// 82FD8C4C: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82FD8C50: 48000008  b 0x82fd8c58
	pc = 0x82FD8C58; continue 'dispatch;
	// 82FD8C54: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FD8C58: 7D6BD214  add r11, r11, r26
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[26].u64;
	// 82FD8C5C: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD8C60: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD8C64: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FD8C68: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82FD8C6C: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FD8C70: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD8C74: 4E800421  bctrl
	ctx.lr = 0x82FD8C78;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD8C78: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FD8C7C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FD8C80: 4BFF8EE9  bl 0x82fd1b68
	ctx.lr = 0x82FD8C84;
	sub_82FD1B68(ctx, base);
	// 82FD8C84: 574B083C  slwi r11, r26, 1
	ctx.r[11].u32 = ctx.r[26].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FD8C88: 3940002F  li r10, 0x2f
	ctx.r[10].s64 = 47;
	// 82FD8C8C: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82FD8C90: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82FD8C94: 386B0002  addi r3, r11, 2
	ctx.r[3].s64 = ctx.r[11].s64 + 2;
	// 82FD8C98: B14B0000  sth r10, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 82FD8C9C: 4BFF8ECD  bl 0x82fd1b68
	ctx.lr = 0x82FD8CA0;
	sub_82FD1B68(ctx, base);
	// 82FD8CA0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FD8CA4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FD8CA8: 4801C4C9  bl 0x82ff5170
	ctx.lr = 0x82FD8CAC;
	sub_82FF5170(ctx, base);
	// 82FD8CAC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FD8CB0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FD8CB4: 4801C64D  bl 0x82ff5300
	ctx.lr = 0x82FD8CB8;
	sub_82FF5300(ctx, base);
	// 82FD8CB8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FD8CBC: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82FD8CC0: 48005151  bl 0x82fdde10
	ctx.lr = 0x82FD8CC4;
	sub_82FDDE10(ctx, base);
	// 82FD8CC4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD8CC8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FD8CCC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD8CD0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD8CD4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD8CD8: 4E800421  bctrl
	ctx.lr = 0x82FD8CDC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD8CDC: 48000028  b 0x82fd8d04
	pc = 0x82FD8D04; continue 'dispatch;
	// 82FD8CE0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FD8CE4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82FD8CE8: 4BFF7E99  bl 0x82fd0b80
	ctx.lr = 0x82FD8CEC;
	sub_82FD0B80(ctx, base);
	// 82FD8CEC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FD8CF0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FD8CF4: 4801C47D  bl 0x82ff5170
	ctx.lr = 0x82FD8CF8;
	sub_82FF5170(ctx, base);
	// 82FD8CF8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FD8CFC: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82FD8D00: 48005111  bl 0x82fdde10
	ctx.lr = 0x82FD8D04;
	sub_82FDDE10(ctx, base);
	// 82FD8D04: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD8D08: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD8D0C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FD8D10: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD8D14: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD8D18: 4E800421  bctrl
	ctx.lr = 0x82FD8D1C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD8D1C: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82FD8D20: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 82FD8D24: 481CF488  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD8D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD8D28 size=40
    let mut pc: u32 = 0x82FD8D28;
    'dispatch: loop {
        match pc {
            0x82FD8D28 => {
    //   block [0x82FD8D28..0x82FD8D50)
	// 82FD8D28: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FD8D2C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD8D30: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD8D34: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD8D38: 807F00A4  lwz r3, 0xa4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 82FD8D3C: 48004F45  bl 0x82fddc80
	ctx.lr = 0x82FD8D40;
	sub_82FDDC80(ctx, base);
	// 82FD8D40: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FD8D44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD8D48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD8D4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD8D50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD8D50 size=8
    let mut pc: u32 = 0x82FD8D50;
    'dispatch: loop {
        match pc {
            0x82FD8D50 => {
    //   block [0x82FD8D50..0x82FD8D58)
	// 82FD8D50: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FD8D54: 82139A18  lwz r16, -0x65e8(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-26088 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD8D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD8D58 size=152
    let mut pc: u32 = 0x82FD8D58;
    'dispatch: loop {
        match pc {
            0x82FD8D58 => {
    //   block [0x82FD8D58..0x82FD8DF0)
	// 82FD8D58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD8D5C: 481CF40D  bl 0x831a8168
	ctx.lr = 0x82FD8D60;
	sub_831A8130(ctx, base);
	// 82FD8D60: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 82FD8D64: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD8D68: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FD8D6C: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 82FD8D70: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FD8D74: 909F0050  stw r4, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[4].u32 ) };
	// 82FD8D78: 4BFFF521  bl 0x82fd8298
	ctx.lr = 0x82FD8D7C;
	sub_82FD8298(ctx, base);
	// 82FD8D7C: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82FD8D80: 93BF0054  stw r29, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 82FD8D84: 41820030  beq 0x82fd8db4
	if ctx.cr[0].eq {
	pc = 0x82FD8DB4; continue 'dispatch;
	}
	// 82FD8D88: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD8D8C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD8D90: 839E0004  lwz r28, 4(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FD8D94: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FD8D98: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD8D9C: 4E800421  bctrl
	ctx.lr = 0x82FD8DA0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD8DA0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FD8DA4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FD8DA8: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82FD8DAC: 4801EF05  bl 0x82ff7cb0
	ctx.lr = 0x82FD8DB0;
	sub_82FF7CB0(ctx, base);
	// 82FD8DB0: 48000008  b 0x82fd8db8
	pc = 0x82FD8DB8; continue 'dispatch;
	// 82FD8DB4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FD8DB8: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FD8DBC: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82FD8DC0: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82FD8DC4: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82FD8DC8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD8DCC: 4082001C  bne 0x82fd8de8
	if !ctx.cr[0].eq {
	pc = 0x82FD8DE8; continue 'dispatch;
	}
	// 82FD8DD0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD8DD4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82FD8DD8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD8DDC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD8DE0: 4E800421  bctrl
	ctx.lr = 0x82FD8DE4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD8DE4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FD8DE8: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 82FD8DEC: 481CF3CC  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD8DF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD8DF0 size=44
    let mut pc: u32 = 0x82FD8DF0;
    'dispatch: loop {
        match pc {
            0x82FD8DF0 => {
    //   block [0x82FD8DF0..0x82FD8E1C)
	// 82FD8DF0: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FD8DF4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD8DF8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD8DFC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD8E00: 809F0050  lwz r4, 0x50(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FD8E04: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD8E08: 4BFFF4D9  bl 0x82fd82e0
	ctx.lr = 0x82FD8E0C;
	sub_82FD82E0(ctx, base);
	// 82FD8E0C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FD8E10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD8E14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD8E18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD8E20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD8E20 size=88
    let mut pc: u32 = 0x82FD8E20;
    'dispatch: loop {
        match pc {
            0x82FD8E20 => {
    //   block [0x82FD8E20..0x82FD8E78)
	// 82FD8E20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD8E24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD8E28: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FD8E2C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FD8E30: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD8E34: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FD8E38: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FD8E3C: 396B9978  addi r11, r11, -0x6688
	ctx.r[11].s64 = ctx.r[11].s64 + -26248;
	// 82FD8E40: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FD8E44: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FD8E48: 48004E39  bl 0x82fddc80
	ctx.lr = 0x82FD8E4C;
	sub_82FDDC80(ctx, base);
	// 82FD8E4C: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FD8E50: 4182000C  beq 0x82fd8e5c
	if ctx.cr[0].eq {
	pc = 0x82FD8E5C; continue 'dispatch;
	}
	// 82FD8E54: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FD8E58: 4BFFF489  bl 0x82fd82e0
	ctx.lr = 0x82FD8E5C;
	sub_82FD82E0(ctx, base);
	// 82FD8E5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FD8E60: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FD8E64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD8E68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD8E6C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FD8E70: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FD8E74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD8E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD8E78 size=100
    let mut pc: u32 = 0x82FD8E78;
    'dispatch: loop {
        match pc {
            0x82FD8E78 => {
    //   block [0x82FD8E78..0x82FD8EDC)
	// 82FD8E78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD8E7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD8E80: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FD8E84: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD8E88: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FD8E8C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FD8E90: 396B9A58  addi r11, r11, -0x65a8
	ctx.r[11].s64 = ctx.r[11].s64 + -26024;
	// 82FD8E94: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FD8E98: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FD8E9C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FD8EA0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD8EA4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD8EA8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD8EAC: 4E800421  bctrl
	ctx.lr = 0x82FD8EB0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD8EB0: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FD8EB4: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD8EB8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD8EBC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD8EC0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD8EC4: 4E800421  bctrl
	ctx.lr = 0x82FD8EC8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD8EC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FD8ECC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD8ED0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD8ED4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FD8ED8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD8EE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD8EE0 size=76
    let mut pc: u32 = 0x82FD8EE0;
    'dispatch: loop {
        match pc {
            0x82FD8EE0 => {
    //   block [0x82FD8EE0..0x82FD8F2C)
	// 82FD8EE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD8EE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD8EE8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FD8EEC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FD8EF0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD8EF4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FD8EF8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FD8EFC: 4BFFFF7D  bl 0x82fd8e78
	ctx.lr = 0x82FD8F00;
	sub_82FD8E78(ctx, base);
	// 82FD8F00: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FD8F04: 4182000C  beq 0x82fd8f10
	if ctx.cr[0].eq {
	pc = 0x82FD8F10; continue 'dispatch;
	}
	// 82FD8F08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FD8F0C: 4BFFF3D5  bl 0x82fd82e0
	ctx.lr = 0x82FD8F10;
	sub_82FD82E0(ctx, base);
	// 82FD8F10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FD8F14: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FD8F18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD8F1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD8F20: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FD8F24: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FD8F28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD8F30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD8F30 size=120
    let mut pc: u32 = 0x82FD8F30;
    'dispatch: loop {
        match pc {
            0x82FD8F30 => {
    //   block [0x82FD8F30..0x82FD8FA8)
	// 82FD8F30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD8F34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD8F38: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FD8F3C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD8F40: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FD8F44: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FD8F48: 394B9A58  addi r10, r11, -0x65a8
	ctx.r[10].s64 = ctx.r[11].s64 + -26024;
	// 82FD8F4C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FD8F50: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82FD8F54: 90BF000C  stw r5, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[5].u32 ) };
	// 82FD8F58: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82FD8F5C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82FD8F60: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82FD8F64: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82FD8F68: 90DF0014  stw r6, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[6].u32 ) };
	// 82FD8F6C: 409A0010  bne cr6, 0x82fd8f7c
	if !ctx.cr[6].eq {
	pc = 0x82FD8F7C; continue 'dispatch;
	}
	// 82FD8F70: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 82FD8F74: 816BB7E8  lwz r11, -0x4818(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FD8F78: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82FD8F7C: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82FD8F80: 809F0014  lwz r4, 0x14(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FD8F84: 4BFF8385  bl 0x82fd1308
	ctx.lr = 0x82FD8F88;
	sub_82FD1308(ctx, base);
	// 82FD8F88: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82FD8F8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FD8F90: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82FD8F94: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FD8F98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD8F9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD8FA0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FD8FA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD8FA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD8FA8 size=136
    let mut pc: u32 = 0x82FD8FA8;
    'dispatch: loop {
        match pc {
            0x82FD8FA8 => {
    //   block [0x82FD8FA8..0x82FD9030)
	// 82FD8FA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD8FAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD8FB0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FD8FB4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FD8FB8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD8FBC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FD8FC0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FD8FC4: 396B9A58  addi r11, r11, -0x65a8
	ctx.r[11].s64 = ctx.r[11].s64 + -26024;
	// 82FD8FC8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FD8FCC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FD8FD0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FD8FD4: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FD8FD8: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82FD8FDC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82FD8FE0: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FD8FE4: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82FD8FE8: 809E0014  lwz r4, 0x14(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FD8FEC: 807E0010  lwz r3, 0x10(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FD8FF0: 4BFF7B91  bl 0x82fd0b80
	ctx.lr = 0x82FD8FF4;
	sub_82FD0B80(ctx, base);
	// 82FD8FF4: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 82FD8FF8: 809E0014  lwz r4, 0x14(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FD8FFC: 909F0014  stw r4, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[4].u32 ) };
	// 82FD9000: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD9004: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD9008: 4182000C  beq 0x82fd9014
	if ctx.cr[0].eq {
	pc = 0x82FD9014; continue 'dispatch;
	}
	// 82FD900C: 4BFF82FD  bl 0x82fd1308
	ctx.lr = 0x82FD9010;
	sub_82FD1308(ctx, base);
	// 82FD9010: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 82FD9014: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FD9018: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FD901C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD9020: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD9024: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FD9028: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FD902C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD9030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD9030 size=80
    let mut pc: u32 = 0x82FD9030;
    'dispatch: loop {
        match pc {
            0x82FD9030 => {
    //   block [0x82FD9030..0x82FD9080)
	// 82FD9030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD9034: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD9038: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FD903C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD9040: 3FE08339  lis r31, -0x7cc7
	ctx.r[31].s64 = -2093416448;
	// 82FD9044: 807FB810  lwz r3, -0x47f0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-18416 as u32) ) } as u64;
	// 82FD9048: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82FD904C: 419A0018  beq cr6, 0x82fd9064
	if ctx.cr[6].eq {
	pc = 0x82FD9064; continue 'dispatch;
	}
	// 82FD9050: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD9054: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82FD9058: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD905C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD9060: 4E800421  bctrl
	ctx.lr = 0x82FD9064;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD9064: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FD9068: 917FB810  stw r11, -0x47f0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-18416 as u32), ctx.r[11].u32 ) };
	// 82FD906C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FD9070: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD9074: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD9078: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FD907C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD9080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD9080 size=92
    let mut pc: u32 = 0x82FD9080;
    'dispatch: loop {
        match pc {
            0x82FD9080 => {
    //   block [0x82FD9080..0x82FD90DC)
	// 82FD9080: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD9084: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD9088: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FD908C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FD9090: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD9094: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 82FD9098: 3BEBB814  addi r31, r11, -0x47ec
	ctx.r[31].s64 = ctx.r[11].s64 + -18412;
	// 82FD909C: 83DF0004  lwz r30, 4(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FD90A0: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82FD90A4: 419A0014  beq cr6, 0x82fd90b8
	if ctx.cr[6].eq {
	pc = 0x82FD90B8; continue 'dispatch;
	}
	// 82FD90A8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD90AC: 4801C6DD  bl 0x82ff5788
	ctx.lr = 0x82FD90B0;
	sub_82FF5788(ctx, base);
	// 82FD90B0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD90B4: 4BFFF22D  bl 0x82fd82e0
	ctx.lr = 0x82FD90B8;
	sub_82FD82E0(ctx, base);
	// 82FD90B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FD90BC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82FD90C0: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82FD90C4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FD90C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD90CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD90D0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FD90D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FD90D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD90E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD90E0 size=8
    let mut pc: u32 = 0x82FD90E0;
    'dispatch: loop {
        match pc {
            0x82FD90E0 => {
    //   block [0x82FD90E0..0x82FD90E8)
	// 82FD90E0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FD90E4: 82139A70  lwz r16, -0x6590(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-26000 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD90E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD90E8 size=176
    let mut pc: u32 = 0x82FD90E8;
    'dispatch: loop {
        match pc {
            0x82FD90E8 => {
    //   block [0x82FD90E8..0x82FD9198)
	// 82FD90E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD90EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD90F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FD90F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FD90F8: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 82FD90FC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD9100: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 82FD9104: 3BCBB818  addi r30, r11, -0x47e8
	ctx.r[30].s64 = ctx.r[11].s64 + -18408;
	// 82FD9108: 897EFFFC  lbz r11, -4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82FD910C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD9110: 4082006C  bne 0x82fd917c
	if !ctx.cr[0].eq {
	pc = 0x82FD917C; continue 'dispatch;
	}
	// 82FD9114: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 82FD9118: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FD911C: 808BB7EC  lwz r4, -0x4814(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18452 as u32) ) } as u64;
	// 82FD9120: 4801C6B9  bl 0x82ff57d8
	ctx.lr = 0x82FD9124;
	sub_82FF57D8(ctx, base);
	// 82FD9124: 897EFFFC  lbz r11, -4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82FD9128: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD912C: 40820048  bne 0x82fd9174
	if !ctx.cr[0].eq {
	pc = 0x82FD9174; continue 'dispatch;
	}
	// 82FD9130: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 82FD9134: 4BFFF115  bl 0x82fd8248
	ctx.lr = 0x82FD9138;
	sub_82FD8248(ctx, base);
	// 82FD9138: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 82FD913C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD9140: 41820010  beq 0x82fd9150
	if ctx.cr[0].eq {
	pc = 0x82FD9150; continue 'dispatch;
	}
	// 82FD9144: 4801C605  bl 0x82ff5748
	ctx.lr = 0x82FD9148;
	sub_82FF5748(ctx, base);
	// 82FD9148: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 82FD914C: 48000008  b 0x82fd9154
	pc = 0x82FD9154; continue 'dispatch;
	// 82FD9150: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82FD9154: 3D6082FE  lis r11, -0x7d02
	ctx.r[11].s64 = -2097283072;
	// 82FD9158: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 82FD915C: 913E0000  stw r9, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82FD9160: 388B9080  addi r4, r11, -0x6f80
	ctx.r[4].s64 = ctx.r[11].s64 + -28544;
	// 82FD9164: 386AB828  addi r3, r10, -0x47d8
	ctx.r[3].s64 = ctx.r[10].s64 + -18392;
	// 82FD9168: 4801E9D1  bl 0x82ff7b38
	ctx.lr = 0x82FD916C;
	sub_82FF7B38(ctx, base);
	// 82FD916C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82FD9170: 997EFFFC  stb r11, -4(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(-4 as u32), ctx.r[11].u8 ) };
	// 82FD9174: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FD9178: 4801C699  bl 0x82ff5810
	ctx.lr = 0x82FD917C;
	sub_82FF5810(ctx, base);
	// 82FD917C: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD9180: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 82FD9184: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD9188: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD918C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FD9190: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FD9194: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD9198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD9198 size=40
    let mut pc: u32 = 0x82FD9198;
    'dispatch: loop {
        match pc {
            0x82FD9198 => {
    //   block [0x82FD9198..0x82FD91C0)
	// 82FD9198: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 82FD919C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD91A0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD91A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD91A8: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FD91AC: 4801C665  bl 0x82ff5810
	ctx.lr = 0x82FD91B0;
	sub_82FF5810(ctx, base);
	// 82FD91B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FD91B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD91B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD91BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD91C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD91C0 size=40
    let mut pc: u32 = 0x82FD91C0;
    'dispatch: loop {
        match pc {
            0x82FD91C0 => {
    //   block [0x82FD91C0..0x82FD91E8)
	// 82FD91C0: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 82FD91C4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD91C8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD91CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD91D0: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FD91D4: 4BFFF10D  bl 0x82fd82e0
	ctx.lr = 0x82FD91D8;
	sub_82FD82E0(ctx, base);
	// 82FD91D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FD91DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD91E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD91E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD91E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD91E8 size=8
    let mut pc: u32 = 0x82FD91E8;
    'dispatch: loop {
        match pc {
            0x82FD91E8 => {
    //   block [0x82FD91E8..0x82FD91F0)
	// 82FD91E8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FD91EC: 82139AC0  lwz r16, -0x6540(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-25920 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD91F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD91F0 size=156
    let mut pc: u32 = 0x82FD91F0;
    'dispatch: loop {
        match pc {
            0x82FD91F0 => {
    //   block [0x82FD91F0..0x82FD928C)
	// 82FD91F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD91F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD91F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FD91FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FD9200: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 82FD9204: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD9208: 3FC08339  lis r30, -0x7cc7
	ctx.r[30].s64 = -2093416448;
	// 82FD920C: 807EB810  lwz r3, -0x47f0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18416 as u32) ) } as u64;
	// 82FD9210: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82FD9214: 409A0060  bne cr6, 0x82fd9274
	if !ctx.cr[6].eq {
	pc = 0x82FD9274; continue 'dispatch;
	}
	// 82FD9218: 4BFFFED1  bl 0x82fd90e8
	ctx.lr = 0x82FD921C;
	sub_82FD90E8(ctx, base);
	// 82FD921C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FD9220: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FD9224: 4801C5B5  bl 0x82ff57d8
	ctx.lr = 0x82FD9228;
	sub_82FF57D8(ctx, base);
	// 82FD9228: 817EB810  lwz r11, -0x47f0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18416 as u32) ) } as u64;
	// 82FD922C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FD9230: 409A0038  bne cr6, 0x82fd9268
	if !ctx.cr[6].eq {
	pc = 0x82FD9268; continue 'dispatch;
	}
	// 82FD9234: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FD9238: 386B79B8  addi r3, r11, 0x79b8
	ctx.r[3].s64 = ctx.r[11].s64 + 31160;
	// 82FD923C: 4BFFEE55  bl 0x82fd8090
	ctx.lr = 0x82FD9240;
	sub_82FD8090(ctx, base);
	// 82FD9240: 907EB810  stw r3, -0x47f0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-18416 as u32), ctx.r[3].u32 ) };
	// 82FD9244: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD9248: 4082000C  bne 0x82fd9254
	if !ctx.cr[0].eq {
	pc = 0x82FD9254; continue 'dispatch;
	}
	// 82FD924C: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 82FD9250: 4801B7F9  bl 0x82ff4a48
	ctx.lr = 0x82FD9254;
	sub_82FF4A48(ctx, base);
	// 82FD9254: 3D6082FE  lis r11, -0x7d02
	ctx.r[11].s64 = -2097283072;
	// 82FD9258: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 82FD925C: 388B9030  addi r4, r11, -0x6fd0
	ctx.r[4].s64 = ctx.r[11].s64 + -28624;
	// 82FD9260: 386AB81C  addi r3, r10, -0x47e4
	ctx.r[3].s64 = ctx.r[10].s64 + -18404;
	// 82FD9264: 4801E8D5  bl 0x82ff7b38
	ctx.lr = 0x82FD9268;
	sub_82FF7B38(ctx, base);
	// 82FD9268: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FD926C: 4801C5A5  bl 0x82ff5810
	ctx.lr = 0x82FD9270;
	sub_82FF5810(ctx, base);
	// 82FD9270: 807EB810  lwz r3, -0x47f0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-18416 as u32) ) } as u64;
	// 82FD9274: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 82FD9278: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD927C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD9280: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FD9284: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FD9288: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD928C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD928C size=40
    let mut pc: u32 = 0x82FD928C;
    'dispatch: loop {
        match pc {
            0x82FD928C => {
    //   block [0x82FD928C..0x82FD92B4)
	// 82FD928C: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 82FD9290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD9294: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD9298: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD929C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FD92A0: 4801C571  bl 0x82ff5810
	ctx.lr = 0x82FD92A4;
	sub_82FF5810(ctx, base);
	// 82FD92A4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FD92A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD92AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD92B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD92B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD92B8 size=128
    let mut pc: u32 = 0x82FD92B8;
    'dispatch: loop {
        match pc {
            0x82FD92B8 => {
    //   block [0x82FD92B8..0x82FD9338)
	// 82FD92B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD92BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD92C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FD92C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FD92C8: E981F000  ld r12, -0x1000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-4096 as u32) ) };
	// 82FD92CC: 9421EF90  stwu r1, -0x1070(r1)
	ea = ctx.r[1].u32.wrapping_add(-4208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD92D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FD92D4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FD92D8: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82FD92DC: 4BFFFF15  bl 0x82fd91f0
	ctx.lr = 0x82FD92E0;
	sub_82FD91F0(ctx, base);
	// 82FD92E0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD92E4: 38C007FF  li r6, 0x7ff
	ctx.r[6].s64 = 2047;
	// 82FD92E8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82FD92EC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FD92F0: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FD92F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD92F8: 4E800421  bctrl
	ctx.lr = 0x82FD92FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD92FC: 809F0014  lwz r4, 0x14(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FD9300: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FD9304: 40820010  bne 0x82fd9314
	if !ctx.cr[0].eq {
	pc = 0x82FD9314; continue 'dispatch;
	}
	// 82FD9308: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FD930C: 386B94E4  addi r3, r11, -0x6b1c
	ctx.r[3].s64 = ctx.r[11].s64 + -27420;
	// 82FD9310: 48000008  b 0x82fd9318
	pc = 0x82FD9318; continue 'dispatch;
	// 82FD9314: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FD9318: 4BFF7869  bl 0x82fd0b80
	ctx.lr = 0x82FD931C;
	sub_82FD0B80(ctx, base);
	// 82FD931C: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 82FD9320: 38211070  addi r1, r1, 0x1070
	ctx.r[1].s64 = ctx.r[1].s64 + 4208;
	// 82FD9324: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD9328: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD932C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FD9330: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FD9334: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD9338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD9338 size=148
    let mut pc: u32 = 0x82FD9338;
    'dispatch: loop {
        match pc {
            0x82FD9338 => {
    //   block [0x82FD9338..0x82FD93CC)
	// 82FD9338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD933C: 481CEE25  bl 0x831a8160
	ctx.lr = 0x82FD9340;
	sub_831A8130(ctx, base);
	// 82FD9340: E981F000  ld r12, -0x1000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-4096 as u32) ) };
	// 82FD9344: E981E000  ld r12, -0x2000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8192 as u32) ) };
	// 82FD9348: 9421DF60  stwu r1, -0x20a0(r1)
	ea = ctx.r[1].u32.wrapping_add(-8352 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD934C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FD9350: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FD9354: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82FD9358: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82FD935C: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 82FD9360: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 82FD9364: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82FD9368: 4BFFFE89  bl 0x82fd91f0
	ctx.lr = 0x82FD936C;
	sub_82FD91F0(ctx, base);
	// 82FD936C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FD9370: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD9374: 83DF0014  lwz r30, 0x14(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FD9378: 7F4AD378  mr r10, r26
	ctx.r[10].u64 = ctx.r[26].u64;
	// 82FD937C: 7F69DB78  mr r9, r27
	ctx.r[9].u64 = ctx.r[27].u64;
	// 82FD9380: 7F88E378  mr r8, r28
	ctx.r[8].u64 = ctx.r[28].u64;
	// 82FD9384: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 82FD9388: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD938C: 38C00FFF  li r6, 0xfff
	ctx.r[6].s64 = 4095;
	// 82FD9390: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82FD9394: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 82FD9398: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD939C: 4E800421  bctrl
	ctx.lr = 0x82FD93A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD93A0: 809F0014  lwz r4, 0x14(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FD93A4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FD93A8: 40820010  bne 0x82fd93b8
	if !ctx.cr[0].eq {
	pc = 0x82FD93B8; continue 'dispatch;
	}
	// 82FD93AC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FD93B0: 386B94E4  addi r3, r11, -0x6b1c
	ctx.r[3].s64 = ctx.r[11].s64 + -27420;
	// 82FD93B4: 48000008  b 0x82fd93bc
	pc = 0x82FD93BC; continue 'dispatch;
	// 82FD93B8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82FD93BC: 4BFF77C5  bl 0x82fd0b80
	ctx.lr = 0x82FD93C0;
	sub_82FD0B80(ctx, base);
	// 82FD93C0: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 82FD93C4: 382120A0  addi r1, r1, 0x20a0
	ctx.r[1].s64 = ctx.r[1].s64 + 8352;
	// 82FD93C8: 481CEDE8  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD93D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD93D0 size=148
    let mut pc: u32 = 0x82FD93D0;
    'dispatch: loop {
        match pc {
            0x82FD93D0 => {
    //   block [0x82FD93D0..0x82FD9464)
	// 82FD93D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD93D4: 481CED8D  bl 0x831a8160
	ctx.lr = 0x82FD93D8;
	sub_831A8130(ctx, base);
	// 82FD93D8: E981F000  ld r12, -0x1000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-4096 as u32) ) };
	// 82FD93DC: E981E000  ld r12, -0x2000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8192 as u32) ) };
	// 82FD93E0: 9421DF60  stwu r1, -0x20a0(r1)
	ea = ctx.r[1].u32.wrapping_add(-8352 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD93E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FD93E8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FD93EC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82FD93F0: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82FD93F4: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 82FD93F8: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 82FD93FC: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82FD9400: 4BFFFDF1  bl 0x82fd91f0
	ctx.lr = 0x82FD9404;
	sub_82FD91F0(ctx, base);
	// 82FD9404: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FD9408: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD940C: 83DF0014  lwz r30, 0x14(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FD9410: 7F4AD378  mr r10, r26
	ctx.r[10].u64 = ctx.r[26].u64;
	// 82FD9414: 7F69DB78  mr r9, r27
	ctx.r[9].u64 = ctx.r[27].u64;
	// 82FD9418: 7F88E378  mr r8, r28
	ctx.r[8].u64 = ctx.r[28].u64;
	// 82FD941C: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 82FD9420: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FD9424: 38C00FFF  li r6, 0xfff
	ctx.r[6].s64 = 4095;
	// 82FD9428: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82FD942C: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 82FD9430: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD9434: 4E800421  bctrl
	ctx.lr = 0x82FD9438;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD9438: 809F0014  lwz r4, 0x14(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FD943C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FD9440: 40820010  bne 0x82fd9450
	if !ctx.cr[0].eq {
	pc = 0x82FD9450; continue 'dispatch;
	}
	// 82FD9444: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FD9448: 386B94E4  addi r3, r11, -0x6b1c
	ctx.r[3].s64 = ctx.r[11].s64 + -27420;
	// 82FD944C: 48000008  b 0x82fd9454
	pc = 0x82FD9454; continue 'dispatch;
	// 82FD9450: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82FD9454: 4BFF772D  bl 0x82fd0b80
	ctx.lr = 0x82FD9458;
	sub_82FD0B80(ctx, base);
	// 82FD9458: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 82FD945C: 382120A0  addi r1, r1, 0x20a0
	ctx.r[1].s64 = ctx.r[1].s64 + 8352;
	// 82FD9460: 481CED50  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD9468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD9468 size=260
    let mut pc: u32 = 0x82FD9468;
    'dispatch: loop {
        match pc {
            0x82FD9468 => {
    //   block [0x82FD9468..0x82FD956C)
	// 82FD9468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD946C: 481CED01  bl 0x831a816c
	ctx.lr = 0x82FD9470;
	sub_831A8130(ctx, base);
	// 82FD9470: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD9474: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FD9478: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82FD947C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FD9480: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD9484: 7D7D5A14  add r11, r29, r11
	ctx.r[11].u64 = ctx.r[29].u64 + ctx.r[11].u64;
	// 82FD9488: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82FD948C: 4198007C  blt cr6, 0x82fd9508
	if ctx.cr[6].lt {
	pc = 0x82FD9508; continue 'dispatch;
	}
	// 82FD9490: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FD9494: 557E083C  slwi r30, r11, 1
	ctx.r[30].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82FD9498: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD949C: 4182001C  beq 0x82fd94b8
	if ctx.cr[0].eq {
	pc = 0x82FD94B8; continue 'dispatch;
	}
	// 82FD94A0: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FD94A4: 7F1E5040  cmplw cr6, r30, r10
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82FD94A8: 40990010  ble cr6, 0x82fd94b8
	if !ctx.cr[6].gt {
	pc = 0x82FD94B8; continue 'dispatch;
	}
	// 82FD94AC: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82FD94B0: 41990060  bgt cr6, 0x82fd9510
	if ctx.cr[6].gt {
	pc = 0x82FD9510; continue 'dispatch;
	}
	// 82FD94B4: 7D5E5378  mr r30, r10
	ctx.r[30].u64 = ctx.r[10].u64;
	// 82FD94B8: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FD94BC: 397E0001  addi r11, r30, 1
	ctx.r[11].s64 = ctx.r[30].s64 + 1;
	// 82FD94C0: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82FD94C4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD94C8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FD94CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD94D0: 4E800421  bctrl
	ctx.lr = 0x82FD94D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD94D4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD94D8: 809F0018  lwz r4, 0x18(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FD94DC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FD94E0: 5565083C  slwi r5, r11, 1
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82FD94E4: 481CF02D  bl 0x831a8510
	ctx.lr = 0x82FD94E8;
	sub_831A8510(ctx, base);
	// 82FD94E8: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FD94EC: 809F0018  lwz r4, 0x18(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FD94F0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD94F4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD94F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD94FC: 4E800421  bctrl
	ctx.lr = 0x82FD9500;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD9500: 93BF0018  stw r29, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[29].u32 ) };
	// 82FD9504: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82FD9508: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82FD950C: 481CECB0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 82FD9510: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD9514: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FD9518: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD951C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD9520: 4E800421  bctrl
	ctx.lr = 0x82FD9524;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD9524: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FD9528: 41820018  beq 0x82fd9540
	if ctx.cr[0].eq {
	pc = 0x82FD9540; continue 'dispatch;
	}
	// 82FD952C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FD9530: 83DF0014  lwz r30, 0x14(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FD9534: 7D7D5A14  add r11, r29, r11
	ctx.r[11].u64 = ctx.r[29].u64 + ctx.r[11].u64;
	// 82FD9538: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82FD953C: 4099FF7C  ble cr6, 0x82fd94b8
	if !ctx.cr[6].gt {
	pc = 0x82FD94B8; continue 'dispatch;
	}
	// 82FD9540: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FD9544: 80FF000C  lwz r7, 0xc(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FD9548: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 82FD954C: 388B9AF8  addi r4, r11, -0x6508
	ctx.r[4].s64 = ctx.r[11].s64 + -25864;
	// 82FD9550: 38A00073  li r5, 0x73
	ctx.r[5].s64 = 115;
	// 82FD9554: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FD9558: 4BFF7B01  bl 0x82fd1058
	ctx.lr = 0x82FD955C;
	sub_82FD1058(ctx, base);
	// 82FD955C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FD9560: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FD9564: 388BC3EC  addi r4, r11, -0x3c14
	ctx.r[4].s64 = ctx.r[11].s64 + -15380;
	// 82FD9568: 481D76C1  bl 0x831b0c28
	ctx.lr = 0x82FD956C;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD9570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD9570 size=152
    let mut pc: u32 = 0x82FD9570;
    'dispatch: loop {
        match pc {
            0x82FD9570 => {
    //   block [0x82FD9570..0x82FD9608)
	// 82FD9570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD9574: 481CEBF9  bl 0x831a816c
	ctx.lr = 0x82FD9578;
	sub_831A8130(ctx, base);
	// 82FD9578: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD957C: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82FD9580: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FD9584: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82FD9588: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82FD958C: 409A0040  bne cr6, 0x82fd95cc
	if !ctx.cr[6].eq {
	pc = 0x82FD95CC; continue 'dispatch;
	}
	// 82FD9590: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82FD9594: 419A0034  beq cr6, 0x82fd95c8
	if ctx.cr[6].eq {
	pc = 0x82FD95C8; continue 'dispatch;
	}
	// 82FD9598: A17D0000  lhz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD959C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD95A0: 41820028  beq 0x82fd95c8
	if ctx.cr[0].eq {
	pc = 0x82FD95C8; continue 'dispatch;
	}
	// 82FD95A4: 397D0002  addi r11, r29, 2
	ctx.r[11].s64 = ctx.r[29].s64 + 2;
	// 82FD95A8: 48000008  b 0x82fd95b0
	pc = 0x82FD95B0; continue 'dispatch;
	// 82FD95AC: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FD95B0: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD95B4: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD95B8: 4082FFF4  bne 0x82fd95ac
	if !ctx.cr[0].eq {
	pc = 0x82FD95AC; continue 'dispatch;
	}
	// 82FD95BC: 7D7D5850  subf r11, r29, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	// 82FD95C0: 7D7F0E70  srawi r31, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[31].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82FD95C4: 48000008  b 0x82fd95cc
	pc = 0x82FD95CC; continue 'dispatch;
	// 82FD95C8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82FD95CC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FD95D0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD95D4: 4BFFFE95  bl 0x82fd9468
	ctx.lr = 0x82FD95D8;
	sub_82FD9468(ctx, base);
	// 82FD95D8: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FD95DC: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FD95E0: 57E5083C  slwi r5, r31, 1
	ctx.r[5].u32 = ctx.r[31].u32.wrapping_shl(1);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82FD95E4: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FD95E8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FD95EC: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82FD95F0: 481CEF21  bl 0x831a8510
	ctx.lr = 0x82FD95F4;
	sub_831A8510(ctx, base);
	// 82FD95F4: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FD95F8: 7D7F5A14  add r11, r31, r11
	ctx.r[11].u64 = ctx.r[31].u64 + ctx.r[11].u64;
	// 82FD95FC: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82FD9600: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FD9604: 481CEBB8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD9608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD9608 size=64
    let mut pc: u32 = 0x82FD9608;
    'dispatch: loop {
        match pc {
            0x82FD9608 => {
    //   block [0x82FD9608..0x82FD9648)
	// 82FD9608: 548A083C  slwi r10, r4, 1
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FD960C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82FD9610: 7D4A1A14  add r10, r10, r3
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[3].u64;
	// 82FD9614: 7F035040  cmplw cr6, r3, r10
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82FD9618: 40980028  bge cr6, 0x82fd9640
	if !ctx.cr[6].lt {
	pc = 0x82FD9640; continue 'dispatch;
	}
	// 82FD961C: 3D208332  lis r9, -0x7cce
	ctx.r[9].s64 = -2093875200;
	// 82FD9620: 39292DD8  addi r9, r9, 0x2dd8
	ctx.r[9].s64 = ctx.r[9].s64 + 11736;
	// 82FD9624: A10B0000  lhz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD9628: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FD962C: 7D0848AE  lbzx r8, r8, r9
	ctx.r[8].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82FD9630: 55080031  rlwinm. r8, r8, 0, 0, 0x18
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82FD9634: 41820014  beq 0x82fd9648
	if ctx.cr[0].eq {
		sub_82FD9648(ctx, base);
		return;
	}
	// 82FD9638: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82FD963C: 4198FFE8  blt cr6, 0x82fd9624
	if ctx.cr[6].lt {
	pc = 0x82FD9624; continue 'dispatch;
	}
	// 82FD9640: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82FD9644: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD9648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD9648 size=8
    let mut pc: u32 = 0x82FD9648;
    'dispatch: loop {
        match pc {
            0x82FD9648 => {
    //   block [0x82FD9648..0x82FD9650)
	// 82FD9648: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FD964C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD9650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD9650 size=44
    let mut pc: u32 = 0x82FD9650;
    'dispatch: loop {
        match pc {
            0x82FD9650 => {
    //   block [0x82FD9650..0x82FD967C)
	// 82FD9650: A1430000  lhz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD9654: 548B083C  slwi r11, r4, 1
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FD9658: 7D0B1A14  add r8, r11, r3
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82FD965C: 2B0A003A  cmplwi cr6, r10, 0x3a
	ctx.cr[6].compare_u32(ctx.r[10].u32, 58 as u32, &mut ctx.xer);
	// 82FD9660: 419A0048  beq cr6, 0x82fd96a8
	if ctx.cr[6].eq {
		sub_82FD96A8(ctx, base);
		return;
	}
	// 82FD9664: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82FD9668: 392B2DD8  addi r9, r11, 0x2dd8
	ctx.r[9].s64 = ctx.r[11].s64 + 11736;
	// 82FD966C: 39630002  addi r11, r3, 2
	ctx.r[11].s64 = ctx.r[3].s64 + 2;
	// 82FD9670: 7D4A48AE  lbzx r10, r10, r9
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82FD9674: 554A07BD  rlwinm. r10, r10, 0, 0x1e, 0x1e
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82FD9678: 4800001C  b 0x82fd9694
	sub_82FD967C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD967C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD967C size=44
    let mut pc: u32 = 0x82FD967C;
    'dispatch: loop {
        match pc {
            0x82FD967C => {
    //   block [0x82FD967C..0x82FD96A8)
	// 82FD967C: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD9680: 2B0A003A  cmplwi cr6, r10, 0x3a
	ctx.cr[6].compare_u32(ctx.r[10].u32, 58 as u32, &mut ctx.xer);
	// 82FD9684: 419A0024  beq cr6, 0x82fd96a8
	if ctx.cr[6].eq {
		sub_82FD96A8(ctx, base);
		return;
	}
	// 82FD9688: 7D4A48AE  lbzx r10, r10, r9
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82FD968C: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FD9690: 554A077B  rlwinm. r10, r10, 0, 0x1d, 0x1d
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82FD9694: 41820014  beq 0x82fd96a8
	if ctx.cr[0].eq {
		sub_82FD96A8(ctx, base);
		return;
	}
	// 82FD9698: 7F0B4040  cmplw cr6, r11, r8
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82FD969C: 4198FFE0  blt cr6, 0x82fd967c
	if ctx.cr[6].lt {
	pc = 0x82FD967C; continue 'dispatch;
	}
	// 82FD96A0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82FD96A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD96A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD96A8 size=8
    let mut pc: u32 = 0x82FD96A8;
    'dispatch: loop {
        match pc {
            0x82FD96A8 => {
    //   block [0x82FD96A8..0x82FD96B0)
	// 82FD96A8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FD96AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD96B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD96B0 size=64
    let mut pc: u32 = 0x82FD96B0;
    'dispatch: loop {
        match pc {
            0x82FD96B0 => {
    //   block [0x82FD96B0..0x82FD96F0)
	// 82FD96B0: 548A083C  slwi r10, r4, 1
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FD96B4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82FD96B8: 7D4A1A14  add r10, r10, r3
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[3].u64;
	// 82FD96BC: 7F035040  cmplw cr6, r3, r10
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82FD96C0: 40980028  bge cr6, 0x82fd96e8
	if !ctx.cr[6].lt {
	pc = 0x82FD96E8; continue 'dispatch;
	}
	// 82FD96C4: 3D208332  lis r9, -0x7cce
	ctx.r[9].s64 = -2093875200;
	// 82FD96C8: 39292DD8  addi r9, r9, 0x2dd8
	ctx.r[9].s64 = ctx.r[9].s64 + 11736;
	// 82FD96CC: A10B0000  lhz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD96D0: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FD96D4: 7D0848AE  lbzx r8, r8, r9
	ctx.r[8].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82FD96D8: 5508077B  rlwinm. r8, r8, 0, 0x1d, 0x1d
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82FD96DC: 41820014  beq 0x82fd96f0
	if ctx.cr[0].eq {
		sub_82FD96F0(ctx, base);
		return;
	}
	// 82FD96E0: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82FD96E4: 4198FFE8  blt cr6, 0x82fd96cc
	if ctx.cr[6].lt {
	pc = 0x82FD96CC; continue 'dispatch;
	}
	// 82FD96E8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82FD96EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD96F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD96F0 size=8
    let mut pc: u32 = 0x82FD96F0;
    'dispatch: loop {
        match pc {
            0x82FD96F0 => {
    //   block [0x82FD96F0..0x82FD96F8)
	// 82FD96F0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FD96F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD96F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD96F8 size=44
    let mut pc: u32 = 0x82FD96F8;
    'dispatch: loop {
        match pc {
            0x82FD96F8 => {
    //   block [0x82FD96F8..0x82FD9724)
	// 82FD96F8: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82FD96FC: A1030000  lhz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD9700: 394B2DD8  addi r10, r11, 0x2dd8
	ctx.r[10].s64 = ctx.r[11].s64 + 11736;
	// 82FD9704: 548B083C  slwi r11, r4, 1
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FD9708: 7D2B1A14  add r9, r11, r3
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82FD970C: 39630002  addi r11, r3, 2
	ctx.r[11].s64 = ctx.r[3].s64 + 2;
	// 82FD9710: 7D0850AE  lbzx r8, r8, r10
	ctx.r[8].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82FD9714: 550807BD  rlwinm. r8, r8, 0, 0x1e, 0x1e
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82FD9718: 40820020  bne 0x82fd9738
	if !ctx.cr[0].eq {
		sub_82FD9724(ctx, base);
		return;
	}
	// 82FD971C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FD9720: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD9724(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD9724 size=36
    let mut pc: u32 = 0x82FD9724;
    'dispatch: loop {
        match pc {
            0x82FD9724 => {
    //   block [0x82FD9724..0x82FD9748)
	// 82FD9724: A10B0000  lhz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD9728: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FD972C: 7D0850AE  lbzx r8, r8, r10
	ctx.r[8].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82FD9730: 5508077B  rlwinm. r8, r8, 0, 0x1d, 0x1d
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82FD9734: 4182FFE8  beq 0x82fd971c
	if ctx.cr[0].eq {
		sub_82FD96F8(ctx, base);
		return;
	}
	// 82FD9738: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82FD973C: 4198FFE8  blt cr6, 0x82fd9724
	if ctx.cr[6].lt {
	pc = 0x82FD9724; continue 'dispatch;
	}
	// 82FD9740: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82FD9744: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD9748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD9748 size=136
    let mut pc: u32 = 0x82FD9748;
    'dispatch: loop {
        match pc {
            0x82FD9748 => {
    //   block [0x82FD9748..0x82FD97D0)
	// 82FD9748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD974C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD9750: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FD9754: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FD9758: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD975C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FD9760: 3880003A  li r4, 0x3a
	ctx.r[4].s64 = 58;
	// 82FD9764: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FD9768: 4BFF8649  bl 0x82fd1db0
	ctx.lr = 0x82FD976C;
	sub_82FD1DB0(ctx, base);
	// 82FD976C: 7C641B79  or. r4, r3, r3
	ctx.r[4].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82FD9770: 41820044  beq 0x82fd97b4
	if ctx.cr[0].eq {
	pc = 0x82FD97B4; continue 'dispatch;
	}
	// 82FD9774: 397EFFFF  addi r11, r30, -1
	ctx.r[11].s64 = ctx.r[30].s64 + -1;
	// 82FD9778: 7F045800  cmpw cr6, r4, r11
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82FD977C: 419A0038  beq cr6, 0x82fd97b4
	if ctx.cr[6].eq {
	pc = 0x82FD97B4; continue 'dispatch;
	}
	// 82FD9780: 2F04FFFF  cmpwi cr6, r4, -1
	ctx.cr[6].compare_i32(ctx.r[4].s32, -1, &mut ctx.xer);
	// 82FD9784: 419A0014  beq cr6, 0x82fd9798
	if ctx.cr[6].eq {
	pc = 0x82FD9798; continue 'dispatch;
	}
	// 82FD9788: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FD978C: 4BFFFEC5  bl 0x82fd9650
	ctx.lr = 0x82FD9790;
	sub_82FD9650(ctx, base);
	// 82FD9790: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FD9794: 41820020  beq 0x82fd97b4
	if ctx.cr[0].eq {
	pc = 0x82FD97B4; continue 'dispatch;
	}
	// 82FD9798: 39640001  addi r11, r4, 1
	ctx.r[11].s64 = ctx.r[4].s64 + 1;
	// 82FD979C: 7D44F050  subf r10, r4, r30
	ctx.r[10].s64 = ctx.r[30].s64 - ctx.r[4].s64;
	// 82FD97A0: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FD97A4: 388AFFFF  addi r4, r10, -1
	ctx.r[4].s64 = ctx.r[10].s64 + -1;
	// 82FD97A8: 7C6BFA14  add r3, r11, r31
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82FD97AC: 4BFFFEA5  bl 0x82fd9650
	ctx.lr = 0x82FD97B0;
	sub_82FD9650(ctx, base);
	// 82FD97B0: 48000008  b 0x82fd97b8
	pc = 0x82FD97B8; continue 'dispatch;
	// 82FD97B4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FD97B8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FD97BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD97C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD97C4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FD97C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FD97CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD97D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD97D0 size=68
    let mut pc: u32 = 0x82FD97D0;
    'dispatch: loop {
        match pc {
            0x82FD97D0 => {
    //   block [0x82FD97D0..0x82FD9814)
	// 82FD97D0: 548B043F  clrlwi. r11, r4, 0x10
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x0000FFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FD97D4: 40820074  bne 0x82fd9848
	if !ctx.cr[0].eq {
		sub_82FD9830(ctx, base);
		return;
	}
	// 82FD97D8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FD97DC: 5469043E  clrlwi r9, r3, 0x10
	ctx.r[9].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 82FD97E0: 396B9B2C  addi r11, r11, -0x64d4
	ctx.r[11].s64 = ctx.r[11].s64 + -25812;
	// 82FD97E4: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD97E8: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FD97EC: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82FD97F0: 41980024  blt cr6, 0x82fd9814
	if ctx.cr[6].lt {
		sub_82FD9814(ctx, base);
		return;
	}
	// 82FD97F4: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD97F8: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FD97FC: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82FD9800: 40990028  ble cr6, 0x82fd9828
	if !ctx.cr[6].gt {
		sub_82FD9828(ctx, base);
		return;
	}
	// 82FD9804: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD9808: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD980C: 4082FFD8  bne 0x82fd97e4
	if !ctx.cr[0].eq {
	pc = 0x82FD97E4; continue 'dispatch;
	}
	// 82FD9810: 4800002C  b 0x82fd983c
	sub_82FD9830(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD9814(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD9814 size=20
    let mut pc: u32 = 0x82FD9814;
    'dispatch: loop {
        match pc {
            0x82FD9814 => {
    //   block [0x82FD9814..0x82FD9828)
	// 82FD9814: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD9818: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FD981C: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD9820: 4082FFF4  bne 0x82fd9814
	if !ctx.cr[0].eq {
	pc = 0x82FD9814; continue 'dispatch;
	}
	// 82FD9824: 48000018  b 0x82fd983c
	sub_82FD9830(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD9828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD9828 size=8
    let mut pc: u32 = 0x82FD9828;
    'dispatch: loop {
        match pc {
            0x82FD9828 => {
    //   block [0x82FD9828..0x82FD9830)
	// 82FD9828: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82FD982C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD9830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD9830 size=32
    let mut pc: u32 = 0x82FD9830;
    'dispatch: loop {
        match pc {
            0x82FD9830 => {
    //   block [0x82FD9830..0x82FD9850)
	// 82FD9830: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82FD9834: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FD9838: 419AFFF0  beq cr6, 0x82fd9828
	if ctx.cr[6].eq {
		sub_82FD9828(ctx, base);
		return;
	}
	// 82FD983C: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD9840: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD9844: 4082FFEC  bne 0x82fd9830
	if !ctx.cr[0].eq {
	pc = 0x82FD9830; continue 'dispatch;
	}
	// 82FD9848: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FD984C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD9850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD9850 size=72
    let mut pc: u32 = 0x82FD9850;
    'dispatch: loop {
        match pc {
            0x82FD9850 => {
    //   block [0x82FD9850..0x82FD9898)
	// 82FD9850: 548A083C  slwi r10, r4, 1
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FD9854: A1630000  lhz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD9858: 39230002  addi r9, r3, 2
	ctx.r[9].s64 = ctx.r[3].s64 + 2;
	// 82FD985C: 7D0A1A14  add r8, r10, r3
	ctx.r[8].u64 = ctx.r[10].u64 + ctx.r[3].u64;
	// 82FD9860: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 82FD9864: 2B0BD800  cmplwi cr6, r11, 0xd800
	ctx.cr[6].compare_u32(ctx.r[11].u32, 55296 as u32, &mut ctx.xer);
	// 82FD9868: 38EA2DD8  addi r7, r10, 0x2dd8
	ctx.r[7].s64 = ctx.r[10].s64 + 11736;
	// 82FD986C: 4198002C  blt cr6, 0x82fd9898
	if ctx.cr[6].lt {
		sub_82FD9898(ctx, base);
		return;
	}
	// 82FD9870: 2B0BDB7F  cmplwi cr6, r11, 0xdb7f
	ctx.cr[6].compare_u32(ctx.r[11].u32, 56191 as u32, &mut ctx.xer);
	// 82FD9874: 41990024  bgt cr6, 0x82fd9898
	if ctx.cr[6].gt {
		sub_82FD9898(ctx, base);
		return;
	}
	// 82FD9878: A1690000  lhz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD987C: 39290002  addi r9, r9, 2
	ctx.r[9].s64 = ctx.r[9].s64 + 2;
	// 82FD9880: 2B0BDC00  cmplwi cr6, r11, 0xdc00
	ctx.cr[6].compare_u32(ctx.r[11].u32, 56320 as u32, &mut ctx.xer);
	// 82FD9884: 4198000C  blt cr6, 0x82fd9890
	if ctx.cr[6].lt {
	pc = 0x82FD9890; continue 'dispatch;
	}
	// 82FD9888: 2B0BDFFF  cmplwi cr6, r11, 0xdfff
	ctx.cr[6].compare_u32(ctx.r[11].u32, 57343 as u32, &mut ctx.xer);
	// 82FD988C: 40990084  ble cr6, 0x82fd9910
	if !ctx.cr[6].gt {
		sub_82FD98F4(ctx, base);
		return;
	}
	// 82FD9890: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FD9894: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD9898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD9898 size=20
    let mut pc: u32 = 0x82FD9898;
    'dispatch: loop {
        match pc {
            0x82FD9898 => {
    //   block [0x82FD9898..0x82FD98AC)
	// 82FD9898: 2B0B003A  cmplwi cr6, r11, 0x3a
	ctx.cr[6].compare_u32(ctx.r[11].u32, 58 as u32, &mut ctx.xer);
	// 82FD989C: 419AFFF4  beq cr6, 0x82fd9890
	if ctx.cr[6].eq {
		sub_82FD9850(ctx, base);
		return;
	}
	// 82FD98A0: 7D6B38AE  lbzx r11, r11, r7
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 82FD98A4: 556B07BD  rlwinm. r11, r11, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FD98A8: 48000064  b 0x82fd990c
	sub_82FD98F4(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD98AC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD98AC size=48
    let mut pc: u32 = 0x82FD98AC;
    'dispatch: loop {
        match pc {
            0x82FD98AC => {
    //   block [0x82FD98AC..0x82FD98DC)
	// 82FD98AC: A1690000  lhz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD98B0: 39290002  addi r9, r9, 2
	ctx.r[9].s64 = ctx.r[9].s64 + 2;
	// 82FD98B4: 2B0BD800  cmplwi cr6, r11, 0xd800
	ctx.cr[6].compare_u32(ctx.r[11].u32, 55296 as u32, &mut ctx.xer);
	// 82FD98B8: 41980024  blt cr6, 0x82fd98dc
	if ctx.cr[6].lt {
		sub_82FD98DC(ctx, base);
		return;
	}
	// 82FD98BC: 2B0BDBFF  cmplwi cr6, r11, 0xdbff
	ctx.cr[6].compare_u32(ctx.r[11].u32, 56319 as u32, &mut ctx.xer);
	// 82FD98C0: 4199001C  bgt cr6, 0x82fd98dc
	if ctx.cr[6].gt {
		sub_82FD98DC(ctx, base);
		return;
	}
	// 82FD98C4: 2B0BDB7F  cmplwi cr6, r11, 0xdb7f
	ctx.cr[6].compare_u32(ctx.r[11].u32, 56191 as u32, &mut ctx.xer);
	// 82FD98C8: 4199FFC8  bgt cr6, 0x82fd9890
	if ctx.cr[6].gt {
		sub_82FD9850(ctx, base);
		return;
	}
	// 82FD98CC: 554B063F  clrlwi. r11, r10, 0x18
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FD98D0: 4082FFC0  bne 0x82fd9890
	if !ctx.cr[0].eq {
		sub_82FD9850(ctx, base);
		return;
	}
	// 82FD98D4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82FD98D8: 4800003C  b 0x82fd9914
	sub_82FD98F4(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD98DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD98DC size=24
    let mut pc: u32 = 0x82FD98DC;
    'dispatch: loop {
        match pc {
            0x82FD98DC => {
    //   block [0x82FD98DC..0x82FD98F4)
	// 82FD98DC: 2B0BDC00  cmplwi cr6, r11, 0xdc00
	ctx.cr[6].compare_u32(ctx.r[11].u32, 56320 as u32, &mut ctx.xer);
	// 82FD98E0: 41980014  blt cr6, 0x82fd98f4
	if ctx.cr[6].lt {
		sub_82FD98F4(ctx, base);
		return;
	}
	// 82FD98E4: 2B0BDFFF  cmplwi cr6, r11, 0xdfff
	ctx.cr[6].compare_u32(ctx.r[11].u32, 57343 as u32, &mut ctx.xer);
	// 82FD98E8: 4199000C  bgt cr6, 0x82fd98f4
	if ctx.cr[6].gt {
		sub_82FD98F4(ctx, base);
		return;
	}
	// 82FD98EC: 554B063F  clrlwi. r11, r10, 0x18
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FD98F0: 4800001C  b 0x82fd990c
	sub_82FD98F4(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD98F4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD98F4 size=48
    let mut pc: u32 = 0x82FD98F4;
    'dispatch: loop {
        match pc {
            0x82FD98F4 => {
    //   block [0x82FD98F4..0x82FD9924)
	// 82FD98F4: 554A063F  clrlwi. r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82FD98F8: 4082FF98  bne 0x82fd9890
	if !ctx.cr[0].eq {
		sub_82FD9850(ctx, base);
		return;
	}
	// 82FD98FC: 2B0B003A  cmplwi cr6, r11, 0x3a
	ctx.cr[6].compare_u32(ctx.r[11].u32, 58 as u32, &mut ctx.xer);
	// 82FD9900: 419AFF90  beq cr6, 0x82fd9890
	if ctx.cr[6].eq {
		sub_82FD9850(ctx, base);
		return;
	}
	// 82FD9904: 7D6B38AE  lbzx r11, r11, r7
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 82FD9908: 556B077B  rlwinm. r11, r11, 0, 0x1d, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FD990C: 4182FF84  beq 0x82fd9890
	if ctx.cr[0].eq {
		sub_82FD9850(ctx, base);
		return;
	}
	// 82FD9910: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FD9914: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82FD9918: 4198FF94  blt cr6, 0x82fd98ac
	if ctx.cr[6].lt {
		sub_82FD98AC(ctx, base);
		return;
	}
	// 82FD991C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82FD9920: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD9928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD9928 size=80
    let mut pc: u32 = 0x82FD9928;
    'dispatch: loop {
        match pc {
            0x82FD9928 => {
    //   block [0x82FD9928..0x82FD9978)
	// 82FD9928: 548B083C  slwi r11, r4, 1
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FD992C: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82FD9930: 7D0B1A14  add r8, r11, r3
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82FD9934: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82FD9938: 7F034040  cmplw cr6, r3, r8
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82FD993C: 40980074  bge cr6, 0x82fd99b0
	if !ctx.cr[6].lt {
		sub_82FD9990(ctx, base);
		return;
	}
	// 82FD9940: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82FD9944: 38EB2DD8  addi r7, r11, 0x2dd8
	ctx.r[7].s64 = ctx.r[11].s64 + 11736;
	// 82FD9948: A16A0000  lhz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD994C: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 82FD9950: 2B0BD800  cmplwi cr6, r11, 0xd800
	ctx.cr[6].compare_u32(ctx.r[11].u32, 55296 as u32, &mut ctx.xer);
	// 82FD9954: 41980024  blt cr6, 0x82fd9978
	if ctx.cr[6].lt {
		sub_82FD9978(ctx, base);
		return;
	}
	// 82FD9958: 2B0BDBFF  cmplwi cr6, r11, 0xdbff
	ctx.cr[6].compare_u32(ctx.r[11].u32, 56319 as u32, &mut ctx.xer);
	// 82FD995C: 4199001C  bgt cr6, 0x82fd9978
	if ctx.cr[6].gt {
		sub_82FD9978(ctx, base);
		return;
	}
	// 82FD9960: 2B0BDB7F  cmplwi cr6, r11, 0xdb7f
	ctx.cr[6].compare_u32(ctx.r[11].u32, 56191 as u32, &mut ctx.xer);
	// 82FD9964: 41990054  bgt cr6, 0x82fd99b8
	if ctx.cr[6].gt {
		sub_82FD99B8(ctx, base);
		return;
	}
	// 82FD9968: 552B063F  clrlwi. r11, r9, 0x18
	ctx.r[11].u64 = ctx.r[9].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FD996C: 4082004C  bne 0x82fd99b8
	if !ctx.cr[0].eq {
		sub_82FD99B8(ctx, base);
		return;
	}
	// 82FD9970: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82FD9974: 48000034  b 0x82fd99a8
	sub_82FD9990(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD9978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD9978 size=24
    let mut pc: u32 = 0x82FD9978;
    'dispatch: loop {
        match pc {
            0x82FD9978 => {
    //   block [0x82FD9978..0x82FD9990)
	// 82FD9978: 2B0BDC00  cmplwi cr6, r11, 0xdc00
	ctx.cr[6].compare_u32(ctx.r[11].u32, 56320 as u32, &mut ctx.xer);
	// 82FD997C: 41980014  blt cr6, 0x82fd9990
	if ctx.cr[6].lt {
		sub_82FD9990(ctx, base);
		return;
	}
	// 82FD9980: 2B0BDFFF  cmplwi cr6, r11, 0xdfff
	ctx.cr[6].compare_u32(ctx.r[11].u32, 57343 as u32, &mut ctx.xer);
	// 82FD9984: 4199000C  bgt cr6, 0x82fd9990
	if ctx.cr[6].gt {
		sub_82FD9990(ctx, base);
		return;
	}
	// 82FD9988: 552B063F  clrlwi. r11, r9, 0x18
	ctx.r[11].u64 = ctx.r[9].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FD998C: 48000014  b 0x82fd99a0
	sub_82FD9990(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD9990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD9990 size=40
    let mut pc: u32 = 0x82FD9990;
    'dispatch: loop {
        match pc {
            0x82FD9990 => {
    //   block [0x82FD9990..0x82FD99B8)
	// 82FD9990: 5529063F  clrlwi. r9, r9, 0x18
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82FD9994: 40820024  bne 0x82fd99b8
	if !ctx.cr[0].eq {
		sub_82FD99B8(ctx, base);
		return;
	}
	// 82FD9998: 7D6B38AE  lbzx r11, r11, r7
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 82FD999C: 556B077B  rlwinm. r11, r11, 0, 0x1d, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FD99A0: 41820018  beq 0x82fd99b8
	if ctx.cr[0].eq {
		sub_82FD99B8(ctx, base);
		return;
	}
	// 82FD99A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82FD99A8: 7F0A4040  cmplw cr6, r10, r8
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82FD99AC: 4198FF9C  blt cr6, 0x82fd9948
	if ctx.cr[6].lt {
		sub_82FD9928(ctx, base);
		return;
	}
	// 82FD99B0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82FD99B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD99B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD99B8 size=8
    let mut pc: u32 = 0x82FD99B8;
    'dispatch: loop {
        match pc {
            0x82FD99B8 => {
    //   block [0x82FD99B8..0x82FD99C0)
	// 82FD99B8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FD99BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD99C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD99C0 size=72
    let mut pc: u32 = 0x82FD99C0;
    'dispatch: loop {
        match pc {
            0x82FD99C0 => {
    //   block [0x82FD99C0..0x82FD9A08)
	// 82FD99C0: 548A083C  slwi r10, r4, 1
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FD99C4: A1630000  lhz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD99C8: 39230002  addi r9, r3, 2
	ctx.r[9].s64 = ctx.r[3].s64 + 2;
	// 82FD99CC: 7D0A1A14  add r8, r10, r3
	ctx.r[8].u64 = ctx.r[10].u64 + ctx.r[3].u64;
	// 82FD99D0: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 82FD99D4: 2B0BD800  cmplwi cr6, r11, 0xd800
	ctx.cr[6].compare_u32(ctx.r[11].u32, 55296 as u32, &mut ctx.xer);
	// 82FD99D8: 38EA2DD8  addi r7, r10, 0x2dd8
	ctx.r[7].s64 = ctx.r[10].s64 + 11736;
	// 82FD99DC: 4198002C  blt cr6, 0x82fd9a08
	if ctx.cr[6].lt {
		sub_82FD9A08(ctx, base);
		return;
	}
	// 82FD99E0: 2B0BDB7F  cmplwi cr6, r11, 0xdb7f
	ctx.cr[6].compare_u32(ctx.r[11].u32, 56191 as u32, &mut ctx.xer);
	// 82FD99E4: 41990024  bgt cr6, 0x82fd9a08
	if ctx.cr[6].gt {
		sub_82FD9A08(ctx, base);
		return;
	}
	// 82FD99E8: A1690000  lhz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD99EC: 39290002  addi r9, r9, 2
	ctx.r[9].s64 = ctx.r[9].s64 + 2;
	// 82FD99F0: 2B0BDC00  cmplwi cr6, r11, 0xdc00
	ctx.cr[6].compare_u32(ctx.r[11].u32, 56320 as u32, &mut ctx.xer);
	// 82FD99F4: 4198000C  blt cr6, 0x82fd9a00
	if ctx.cr[6].lt {
	pc = 0x82FD9A00; continue 'dispatch;
	}
	// 82FD99F8: 2B0BDFFF  cmplwi cr6, r11, 0xdfff
	ctx.cr[6].compare_u32(ctx.r[11].u32, 57343 as u32, &mut ctx.xer);
	// 82FD99FC: 40990078  ble cr6, 0x82fd9a74
	if !ctx.cr[6].gt {
		sub_82FD9A60(ctx, base);
		return;
	}
	// 82FD9A00: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FD9A04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD9A08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD9A08 size=16
    let mut pc: u32 = 0x82FD9A08;
    'dispatch: loop {
        match pc {
            0x82FD9A08 => {
    //   block [0x82FD9A08..0x82FD9A18)
	// 82FD9A08: 7D6B38AE  lbzx r11, r11, r7
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 82FD9A0C: 556B07BD  rlwinm. r11, r11, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FD9A10: 40820064  bne 0x82fd9a74
	if !ctx.cr[0].eq {
		sub_82FD9A60(ctx, base);
		return;
	}
	// 82FD9A14: 4BFFFFEC  b 0x82fd9a00
	sub_82FD99C0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD9A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD9A18 size=48
    let mut pc: u32 = 0x82FD9A18;
    'dispatch: loop {
        match pc {
            0x82FD9A18 => {
    //   block [0x82FD9A18..0x82FD9A48)
	// 82FD9A18: A1690000  lhz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD9A1C: 39290002  addi r9, r9, 2
	ctx.r[9].s64 = ctx.r[9].s64 + 2;
	// 82FD9A20: 2B0BD800  cmplwi cr6, r11, 0xd800
	ctx.cr[6].compare_u32(ctx.r[11].u32, 55296 as u32, &mut ctx.xer);
	// 82FD9A24: 41980024  blt cr6, 0x82fd9a48
	if ctx.cr[6].lt {
		sub_82FD9A48(ctx, base);
		return;
	}
	// 82FD9A28: 2B0BDBFF  cmplwi cr6, r11, 0xdbff
	ctx.cr[6].compare_u32(ctx.r[11].u32, 56319 as u32, &mut ctx.xer);
	// 82FD9A2C: 4199001C  bgt cr6, 0x82fd9a48
	if ctx.cr[6].gt {
		sub_82FD9A48(ctx, base);
		return;
	}
	// 82FD9A30: 2B0BDB7F  cmplwi cr6, r11, 0xdb7f
	ctx.cr[6].compare_u32(ctx.r[11].u32, 56191 as u32, &mut ctx.xer);
	// 82FD9A34: 4199FFCC  bgt cr6, 0x82fd9a00
	if ctx.cr[6].gt {
		sub_82FD99C0(ctx, base);
		return;
	}
	// 82FD9A38: 554B063F  clrlwi. r11, r10, 0x18
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FD9A3C: 4082FFC4  bne 0x82fd9a00
	if !ctx.cr[0].eq {
		sub_82FD99C0(ctx, base);
		return;
	}
	// 82FD9A40: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82FD9A44: 48000034  b 0x82fd9a78
	sub_82FD9A60(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD9A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD9A48 size=24
    let mut pc: u32 = 0x82FD9A48;
    'dispatch: loop {
        match pc {
            0x82FD9A48 => {
    //   block [0x82FD9A48..0x82FD9A60)
	// 82FD9A48: 2B0BDC00  cmplwi cr6, r11, 0xdc00
	ctx.cr[6].compare_u32(ctx.r[11].u32, 56320 as u32, &mut ctx.xer);
	// 82FD9A4C: 41980014  blt cr6, 0x82fd9a60
	if ctx.cr[6].lt {
		sub_82FD9A60(ctx, base);
		return;
	}
	// 82FD9A50: 2B0BDFFF  cmplwi cr6, r11, 0xdfff
	ctx.cr[6].compare_u32(ctx.r[11].u32, 57343 as u32, &mut ctx.xer);
	// 82FD9A54: 4199000C  bgt cr6, 0x82fd9a60
	if ctx.cr[6].gt {
		sub_82FD9A60(ctx, base);
		return;
	}
	// 82FD9A58: 554B063F  clrlwi. r11, r10, 0x18
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FD9A5C: 48000014  b 0x82fd9a70
	sub_82FD9A60(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD9A60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD9A60 size=40
    let mut pc: u32 = 0x82FD9A60;
    'dispatch: loop {
        match pc {
            0x82FD9A60 => {
    //   block [0x82FD9A60..0x82FD9A88)
	// 82FD9A60: 554A063F  clrlwi. r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82FD9A64: 4082FF9C  bne 0x82fd9a00
	if !ctx.cr[0].eq {
		sub_82FD99C0(ctx, base);
		return;
	}
	// 82FD9A68: 7D6B38AE  lbzx r11, r11, r7
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 82FD9A6C: 556B077B  rlwinm. r11, r11, 0, 0x1d, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FD9A70: 4182FF90  beq 0x82fd9a00
	if ctx.cr[0].eq {
		sub_82FD99C0(ctx, base);
		return;
	}
	// 82FD9A74: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FD9A78: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82FD9A7C: 4198FF9C  blt cr6, 0x82fd9a18
	if ctx.cr[6].lt {
		sub_82FD9A18(ctx, base);
		return;
	}
	// 82FD9A80: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82FD9A84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD9A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD9A88 size=136
    let mut pc: u32 = 0x82FD9A88;
    'dispatch: loop {
        match pc {
            0x82FD9A88 => {
    //   block [0x82FD9A88..0x82FD9B10)
	// 82FD9A88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD9A8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD9A90: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FD9A94: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FD9A98: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD9A9C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FD9AA0: 3880003A  li r4, 0x3a
	ctx.r[4].s64 = 58;
	// 82FD9AA4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FD9AA8: 4BFF8309  bl 0x82fd1db0
	ctx.lr = 0x82FD9AAC;
	sub_82FD1DB0(ctx, base);
	// 82FD9AAC: 7C641B79  or. r4, r3, r3
	ctx.r[4].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82FD9AB0: 41820044  beq 0x82fd9af4
	if ctx.cr[0].eq {
	pc = 0x82FD9AF4; continue 'dispatch;
	}
	// 82FD9AB4: 397EFFFF  addi r11, r30, -1
	ctx.r[11].s64 = ctx.r[30].s64 + -1;
	// 82FD9AB8: 7F045800  cmpw cr6, r4, r11
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82FD9ABC: 419A0038  beq cr6, 0x82fd9af4
	if ctx.cr[6].eq {
	pc = 0x82FD9AF4; continue 'dispatch;
	}
	// 82FD9AC0: 2F04FFFF  cmpwi cr6, r4, -1
	ctx.cr[6].compare_i32(ctx.r[4].s32, -1, &mut ctx.xer);
	// 82FD9AC4: 419A0014  beq cr6, 0x82fd9ad8
	if ctx.cr[6].eq {
	pc = 0x82FD9AD8; continue 'dispatch;
	}
	// 82FD9AC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FD9ACC: 4BFFFD85  bl 0x82fd9850
	ctx.lr = 0x82FD9AD0;
	sub_82FD9850(ctx, base);
	// 82FD9AD0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FD9AD4: 41820020  beq 0x82fd9af4
	if ctx.cr[0].eq {
	pc = 0x82FD9AF4; continue 'dispatch;
	}
	// 82FD9AD8: 39640001  addi r11, r4, 1
	ctx.r[11].s64 = ctx.r[4].s64 + 1;
	// 82FD9ADC: 7D44F050  subf r10, r4, r30
	ctx.r[10].s64 = ctx.r[30].s64 - ctx.r[4].s64;
	// 82FD9AE0: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FD9AE4: 388AFFFF  addi r4, r10, -1
	ctx.r[4].s64 = ctx.r[10].s64 + -1;
	// 82FD9AE8: 7C6BFA14  add r3, r11, r31
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82FD9AEC: 4BFFFD65  bl 0x82fd9850
	ctx.lr = 0x82FD9AF0;
	sub_82FD9850(ctx, base);
	// 82FD9AF0: 48000008  b 0x82fd9af8
	pc = 0x82FD9AF8; continue 'dispatch;
	// 82FD9AF4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FD9AF8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FD9AFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD9B00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD9B04: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FD9B08: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FD9B0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD9B10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD9B10 size=8
    let mut pc: u32 = 0x82FD9B10;
    'dispatch: loop {
        match pc {
            0x82FD9B10 => {
    //   block [0x82FD9B10..0x82FD9B18)
	// 82FD9B10: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FD9B14: 82139CE8  lwz r16, -0x6318(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-25368 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD9B18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD9B18 size=72
    let mut pc: u32 = 0x82FD9B18;
    'dispatch: loop {
        match pc {
            0x82FD9B18 => {
    //   block [0x82FD9B18..0x82FD9B60)
	// 82FD9B18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD9B1C: 481CE651  bl 0x831a816c
	ctx.lr = 0x82FD9B20;
	sub_831A8130(ctx, base);
	// 82FD9B20: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 82FD9B24: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD9B28: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FD9B2C: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82FD9B30: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 82FD9B34: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 82FD9B38: 4BFFF3F9  bl 0x82fd8f30
	ctx.lr = 0x82FD9B3C;
	sub_82FD8F30(ctx, base);
	// 82FD9B3C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FD9B40: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FD9B44: 396B9CD4  addi r11, r11, -0x632c
	ctx.r[11].s64 = ctx.r[11].s64 + -25388;
	// 82FD9B48: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD9B4C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FD9B50: 4BFFF769  bl 0x82fd92b8
	ctx.lr = 0x82FD9B54;
	sub_82FD92B8(ctx, base);
	// 82FD9B54: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD9B58: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 82FD9B5C: 481CE660  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD9B60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD9B60 size=40
    let mut pc: u32 = 0x82FD9B60;
    'dispatch: loop {
        match pc {
            0x82FD9B60 => {
    //   block [0x82FD9B60..0x82FD9B88)
	// 82FD9B60: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 82FD9B64: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD9B68: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD9B6C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD9B70: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82FD9B74: 4BFFF305  bl 0x82fd8e78
	ctx.lr = 0x82FD9B78;
	sub_82FD8E78(ctx, base);
	// 82FD9B78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FD9B7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD9B80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD9B84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD9B88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD9B88 size=60
    let mut pc: u32 = 0x82FD9B88;
    'dispatch: loop {
        match pc {
            0x82FD9B88 => {
    //   block [0x82FD9B88..0x82FD9BC4)
	// 82FD9B88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD9B8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD9B90: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FD9B94: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD9B98: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FD9B9C: 4BFFF40D  bl 0x82fd8fa8
	ctx.lr = 0x82FD9BA0;
	sub_82FD8FA8(ctx, base);
	// 82FD9BA0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FD9BA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FD9BA8: 396B9CD4  addi r11, r11, -0x632c
	ctx.r[11].s64 = ctx.r[11].s64 + -25388;
	// 82FD9BAC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FD9BB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FD9BB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD9BB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD9BBC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FD9BC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD9BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD9BC8 size=8
    let mut pc: u32 = 0x82FD9BC8;
    'dispatch: loop {
        match pc {
            0x82FD9BC8 => {
    //   block [0x82FD9BC8..0x82FD9BD0)
	// 82FD9BC8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FD9BCC: 82139D20  lwz r16, -0x62e0(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-25312 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD9BD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD9BD0 size=104
    let mut pc: u32 = 0x82FD9BD0;
    'dispatch: loop {
        match pc {
            0x82FD9BD0 => {
    //   block [0x82FD9BD0..0x82FD9C38)
	// 82FD9BD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD9BD4: 481CE589  bl 0x831a815c
	ctx.lr = 0x82FD9BD8;
	sub_831A8130(ctx, base);
	// 82FD9BD8: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 82FD9BDC: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD9BE0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FD9BE4: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82FD9BE8: 80DF00E4  lwz r6, 0xe4(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 82FD9BEC: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 82FD9BF0: 7D1B4378  mr r27, r8
	ctx.r[27].u64 = ctx.r[8].u64;
	// 82FD9BF4: 7D3A4B78  mr r26, r9
	ctx.r[26].u64 = ctx.r[9].u64;
	// 82FD9BF8: 93DF00A4  stw r30, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[30].u32 ) };
	// 82FD9BFC: 7D595378  mr r25, r10
	ctx.r[25].u64 = ctx.r[10].u64;
	// 82FD9C00: 4BFFF331  bl 0x82fd8f30
	ctx.lr = 0x82FD9C04;
	sub_82FD8F30(ctx, base);
	// 82FD9C04: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FD9C08: 7F28CB78  mr r8, r25
	ctx.r[8].u64 = ctx.r[25].u64;
	// 82FD9C0C: 396B9CD4  addi r11, r11, -0x632c
	ctx.r[11].s64 = ctx.r[11].s64 + -25388;
	// 82FD9C10: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 82FD9C14: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82FD9C18: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82FD9C1C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FD9C20: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD9C24: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FD9C28: 4BFFF711  bl 0x82fd9338
	ctx.lr = 0x82FD9C2C;
	sub_82FD9338(ctx, base);
	// 82FD9C2C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD9C30: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 82FD9C34: 481CE578  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD9C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD9C38 size=40
    let mut pc: u32 = 0x82FD9C38;
    'dispatch: loop {
        match pc {
            0x82FD9C38 => {
    //   block [0x82FD9C38..0x82FD9C60)
	// 82FD9C38: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FD9C3C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD9C40: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD9C44: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD9C48: 807F00A4  lwz r3, 0xa4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 82FD9C4C: 4BFFF22D  bl 0x82fd8e78
	ctx.lr = 0x82FD9C50;
	sub_82FD8E78(ctx, base);
	// 82FD9C50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FD9C54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD9C58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD9C5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD9C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD9C60 size=16
    let mut pc: u32 = 0x82FD9C60;
    'dispatch: loop {
        match pc {
            0x82FD9C60 => {
    //   block [0x82FD9C60..0x82FD9C70)
	// 82FD9C60: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FD9C64: 396B9CD4  addi r11, r11, -0x632c
	ctx.r[11].s64 = ctx.r[11].s64 + -25388;
	// 82FD9C68: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FD9C6C: 4BFFF20C  b 0x82fd8e78
	sub_82FD8E78(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD9C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD9C70 size=8
    let mut pc: u32 = 0x82FD9C70;
    'dispatch: loop {
        match pc {
            0x82FD9C70 => {
    //   block [0x82FD9C70..0x82FD9C78)
	// 82FD9C70: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FD9C74: 82139D58  lwz r16, -0x62a8(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-25256 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD9C78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD9C78 size=92
    let mut pc: u32 = 0x82FD9C78;
    'dispatch: loop {
        match pc {
            0x82FD9C78 => {
    //   block [0x82FD9C78..0x82FD9CD4)
	// 82FD9C78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD9C7C: 481CE4F1  bl 0x831a816c
	ctx.lr = 0x82FD9C80;
	sub_831A8130(ctx, base);
	// 82FD9C80: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 82FD9C84: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD9C88: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FD9C8C: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82FD9C90: 809D0014  lwz r4, 0x14(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FD9C94: 93BF0094  stw r29, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[29].u32 ) };
	// 82FD9C98: 4BFFE601  bl 0x82fd8298
	ctx.lr = 0x82FD9C9C;
	sub_82FD8298(ctx, base);
	// 82FD9C9C: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FD9CA0: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82FD9CA4: 41820024  beq 0x82fd9cc8
	if ctx.cr[0].eq {
	pc = 0x82FD9CC8; continue 'dispatch;
	}
	// 82FD9CA8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FD9CAC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD9CB0: 4BFFF2F9  bl 0x82fd8fa8
	ctx.lr = 0x82FD9CB4;
	sub_82FD8FA8(ctx, base);
	// 82FD9CB4: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FD9CB8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FD9CBC: 396B9CD4  addi r11, r11, -0x632c
	ctx.r[11].s64 = ctx.r[11].s64 + -25388;
	// 82FD9CC0: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FD9CC4: 48000008  b 0x82fd9ccc
	pc = 0x82FD9CCC; continue 'dispatch;
	// 82FD9CC8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FD9CCC: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 82FD9CD0: 481CE4EC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD9CD4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD9CD4 size=48
    let mut pc: u32 = 0x82FD9CD4;
    'dispatch: loop {
        match pc {
            0x82FD9CD4 => {
    //   block [0x82FD9CD4..0x82FD9D04)
	// 82FD9CD4: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FD9CD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD9CDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD9CE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD9CE4: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FD9CE8: 808B0014  lwz r4, 0x14(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FD9CEC: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FD9CF0: 4BFFE5F1  bl 0x82fd82e0
	ctx.lr = 0x82FD9CF4;
	sub_82FD82E0(ctx, base);
	// 82FD9CF4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FD9CF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD9CFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD9D00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD9D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD9D08 size=12
    let mut pc: u32 = 0x82FD9D08;
    'dispatch: loop {
        match pc {
            0x82FD9D08 => {
    //   block [0x82FD9D08..0x82FD9D14)
	// 82FD9D08: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FD9D0C: 386B84C0  addi r3, r11, -0x7b40
	ctx.r[3].s64 = ctx.r[11].s64 + -31552;
	// 82FD9D10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD9D18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD9D18 size=88
    let mut pc: u32 = 0x82FD9D18;
    'dispatch: loop {
        match pc {
            0x82FD9D18 => {
    //   block [0x82FD9D18..0x82FD9D70)
	// 82FD9D18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD9D1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD9D20: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FD9D24: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FD9D28: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD9D2C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FD9D30: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FD9D34: 396B9CD4  addi r11, r11, -0x632c
	ctx.r[11].s64 = ctx.r[11].s64 + -25388;
	// 82FD9D38: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FD9D3C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FD9D40: 4BFFF139  bl 0x82fd8e78
	ctx.lr = 0x82FD9D44;
	sub_82FD8E78(ctx, base);
	// 82FD9D44: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FD9D48: 4182000C  beq 0x82fd9d54
	if ctx.cr[0].eq {
	pc = 0x82FD9D54; continue 'dispatch;
	}
	// 82FD9D4C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FD9D50: 4BFFE591  bl 0x82fd82e0
	ctx.lr = 0x82FD9D54;
	sub_82FD82E0(ctx, base);
	// 82FD9D54: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FD9D58: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FD9D5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD9D60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD9D64: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FD9D68: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FD9D6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD9D70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD9D70 size=104
    let mut pc: u32 = 0x82FD9D70;
    'dispatch: loop {
        match pc {
            0x82FD9D70 => {
    //   block [0x82FD9D70..0x82FD9DD8)
	// 82FD9D70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD9D74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD9D78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD9D7C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82FD9D80: 419A0034  beq cr6, 0x82fd9db4
	if ctx.cr[6].eq {
	pc = 0x82FD9DB4; continue 'dispatch;
	}
	// 82FD9D84: A1640000  lhz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD9D88: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD9D8C: 41820028  beq 0x82fd9db4
	if ctx.cr[0].eq {
	pc = 0x82FD9DB4; continue 'dispatch;
	}
	// 82FD9D90: 39640002  addi r11, r4, 2
	ctx.r[11].s64 = ctx.r[4].s64 + 2;
	// 82FD9D94: 48000008  b 0x82fd9d9c
	pc = 0x82FD9D9C; continue 'dispatch;
	// 82FD9D98: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FD9D9C: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD9DA0: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD9DA4: 4082FFF4  bne 0x82fd9d98
	if !ctx.cr[0].eq {
	pc = 0x82FD9D98; continue 'dispatch;
	}
	// 82FD9DA8: 7D645850  subf r11, r4, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[4].s64;
	// 82FD9DAC: 7D650E70  srawi r5, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[5].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82FD9DB0: 48000008  b 0x82fd9db8
	pc = 0x82FD9DB8; continue 'dispatch;
	// 82FD9DB4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FD9DB8: 4BFF7B89  bl 0x82fd1940
	ctx.lr = 0x82FD9DBC;
	sub_82FD1940(ctx, base);
	// 82FD9DBC: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 82FD9DC0: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82FD9DC4: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82FD9DC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FD9DCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD9DD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD9DD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD9DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD9DD8 size=32
    let mut pc: u32 = 0x82FD9DD8;
    'dispatch: loop {
        match pc {
            0x82FD9DD8 => {
    //   block [0x82FD9DD8..0x82FD9DF8)
	// 82FD9DD8: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82FD9DDC: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82FD9DE0: 419A0034  beq cr6, 0x82fd9e14
	if ctx.cr[6].eq {
		sub_82FD9E14(ctx, base);
		return;
	}
	// 82FD9DE4: A1650000  lhz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD9DE8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD9DEC: 41820028  beq 0x82fd9e14
	if ctx.cr[0].eq {
		sub_82FD9E14(ctx, base);
		return;
	}
	// 82FD9DF0: 39650002  addi r11, r5, 2
	ctx.r[11].s64 = ctx.r[5].s64 + 2;
	// 82FD9DF4: 48000008  b 0x82fd9dfc
	sub_82FD9DF8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD9DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD9DF8 size=28
    let mut pc: u32 = 0x82FD9DF8;
    'dispatch: loop {
        match pc {
            0x82FD9DF8 => {
    //   block [0x82FD9DF8..0x82FD9E14)
	// 82FD9DF8: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FD9DFC: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD9E00: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD9E04: 4082FFF4  bne 0x82fd9df8
	if !ctx.cr[0].eq {
	pc = 0x82FD9DF8; continue 'dispatch;
	}
	// 82FD9E08: 7D655850  subf r11, r5, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[5].s64;
	// 82FD9E0C: 7D670E70  srawi r7, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[7].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82FD9E10: 48000008  b 0x82fd9e18
	sub_82FD9E14(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD9E14(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD9E14 size=32
    let mut pc: u32 = 0x82FD9E14;
    'dispatch: loop {
        match pc {
            0x82FD9E14 => {
    //   block [0x82FD9E14..0x82FD9E34)
	// 82FD9E14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82FD9E18: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82FD9E1C: 419A0034  beq cr6, 0x82fd9e50
	if ctx.cr[6].eq {
		sub_82FD9E50(ctx, base);
		return;
	}
	// 82FD9E20: A1630000  lhz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD9E24: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD9E28: 41820028  beq 0x82fd9e50
	if ctx.cr[0].eq {
		sub_82FD9E50(ctx, base);
		return;
	}
	// 82FD9E2C: 39630002  addi r11, r3, 2
	ctx.r[11].s64 = ctx.r[3].s64 + 2;
	// 82FD9E30: 48000008  b 0x82fd9e38
	sub_82FD9E34(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD9E34(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD9E34 size=28
    let mut pc: u32 = 0x82FD9E34;
    'dispatch: loop {
        match pc {
            0x82FD9E34 => {
    //   block [0x82FD9E34..0x82FD9E50)
	// 82FD9E34: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FD9E38: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD9E3C: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD9E40: 4082FFF4  bne 0x82fd9e34
	if !ctx.cr[0].eq {
	pc = 0x82FD9E34; continue 'dispatch;
	}
	// 82FD9E44: 7D635850  subf r11, r3, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[3].s64;
	// 82FD9E48: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82FD9E4C: 48000008  b 0x82fd9e54
	sub_82FD9E50(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD9E50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FD9E50 size=16
    let mut pc: u32 = 0x82FD9E50;
    'dispatch: loop {
        match pc {
            0x82FD9E50 => {
    //   block [0x82FD9E50..0x82FD9E60)
	// 82FD9E50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FD9E54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82FD9E58: 7C875850  subf r4, r7, r11
	ctx.r[4].s64 = ctx.r[11].s64 - ctx.r[7].s64;
	// 82FD9E5C: 4BFF7C3C  b 0x82fd1a98
	sub_82FD1A98(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD9E60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD9E60 size=88
    let mut pc: u32 = 0x82FD9E60;
    'dispatch: loop {
        match pc {
            0x82FD9E60 => {
    //   block [0x82FD9E60..0x82FD9EB8)
	// 82FD9E60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD9E64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD9E68: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FD9E6C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD9E70: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FD9E74: 4BFF7735  bl 0x82fd15a8
	ctx.lr = 0x82FD9E78;
	sub_82FD15A8(ctx, base);
	// 82FD9E78: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FD9E7C: 40820020  bne 0x82fd9e9c
	if !ctx.cr[0].eq {
	pc = 0x82FD9E9C; continue 'dispatch;
	}
	// 82FD9E80: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FD9E84: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FD9E88: 386B9B9C  addi r3, r11, -0x6464
	ctx.r[3].s64 = ctx.r[11].s64 + -25700;
	// 82FD9E8C: 4BFF7F25  bl 0x82fd1db0
	ctx.lr = 0x82FD9E90;
	sub_82FD1DB0(ctx, base);
	// 82FD9E90: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82FD9E94: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FD9E98: 419A0008  beq cr6, 0x82fd9ea0
	if ctx.cr[6].eq {
	pc = 0x82FD9EA0; continue 'dispatch;
	}
	// 82FD9E9C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82FD9EA0: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82FD9EA4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FD9EA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD9EAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD9EB0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FD9EB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD9EB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD9EB8 size=88
    let mut pc: u32 = 0x82FD9EB8;
    'dispatch: loop {
        match pc {
            0x82FD9EB8 => {
    //   block [0x82FD9EB8..0x82FD9F10)
	// 82FD9EB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD9EBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD9EC0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FD9EC4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD9EC8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FD9ECC: 4BFF76DD  bl 0x82fd15a8
	ctx.lr = 0x82FD9ED0;
	sub_82FD15A8(ctx, base);
	// 82FD9ED0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FD9ED4: 40820020  bne 0x82fd9ef4
	if !ctx.cr[0].eq {
	pc = 0x82FD9EF4; continue 'dispatch;
	}
	// 82FD9ED8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FD9EDC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FD9EE0: 386B9B88  addi r3, r11, -0x6478
	ctx.r[3].s64 = ctx.r[11].s64 + -25720;
	// 82FD9EE4: 4BFF7ECD  bl 0x82fd1db0
	ctx.lr = 0x82FD9EE8;
	sub_82FD1DB0(ctx, base);
	// 82FD9EE8: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82FD9EEC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FD9EF0: 419A0008  beq cr6, 0x82fd9ef8
	if ctx.cr[6].eq {
	pc = 0x82FD9EF8; continue 'dispatch;
	}
	// 82FD9EF4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82FD9EF8: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82FD9EFC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FD9F00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FD9F04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FD9F08: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FD9F0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FD9F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FD9F10 size=288
    let mut pc: u32 = 0x82FD9F10;
    'dispatch: loop {
        match pc {
            0x82FD9F10 => {
    //   block [0x82FD9F10..0x82FDA030)
	// 82FD9F10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FD9F14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FD9F18: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FD9F1C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FD9F20: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FD9F24: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FD9F28: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD9F2C: 41820018  beq 0x82fd9f44
	if ctx.cr[0].eq {
	pc = 0x82FD9F44; continue 'dispatch;
	}
	// 82FD9F30: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FD9F34: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD9F38: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD9F3C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD9F40: 4E800421  bctrl
	ctx.lr = 0x82FD9F44;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD9F44: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD9F48: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD9F4C: 41820018  beq 0x82fd9f64
	if ctx.cr[0].eq {
	pc = 0x82FD9F64; continue 'dispatch;
	}
	// 82FD9F50: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FD9F54: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD9F58: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD9F5C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD9F60: 4E800421  bctrl
	ctx.lr = 0x82FD9F64;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD9F64: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FD9F68: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD9F6C: 41820018  beq 0x82fd9f84
	if ctx.cr[0].eq {
	pc = 0x82FD9F84; continue 'dispatch;
	}
	// 82FD9F70: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FD9F74: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD9F78: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD9F7C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD9F80: 4E800421  bctrl
	ctx.lr = 0x82FD9F84;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD9F84: 809F0014  lwz r4, 0x14(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FD9F88: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD9F8C: 41820018  beq 0x82fd9fa4
	if ctx.cr[0].eq {
	pc = 0x82FD9FA4; continue 'dispatch;
	}
	// 82FD9F90: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FD9F94: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD9F98: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD9F9C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD9FA0: 4E800421  bctrl
	ctx.lr = 0x82FD9FA4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD9FA4: 809F0018  lwz r4, 0x18(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FD9FA8: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD9FAC: 41820018  beq 0x82fd9fc4
	if ctx.cr[0].eq {
	pc = 0x82FD9FC4; continue 'dispatch;
	}
	// 82FD9FB0: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FD9FB4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD9FB8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD9FBC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD9FC0: 4E800421  bctrl
	ctx.lr = 0x82FD9FC4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD9FC4: 809F001C  lwz r4, 0x1c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FD9FC8: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD9FCC: 41820018  beq 0x82fd9fe4
	if ctx.cr[0].eq {
	pc = 0x82FD9FE4; continue 'dispatch;
	}
	// 82FD9FD0: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FD9FD4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD9FD8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD9FDC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FD9FE0: 4E800421  bctrl
	ctx.lr = 0x82FD9FE4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FD9FE4: 809F0020  lwz r4, 0x20(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FD9FE8: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FD9FEC: 41820018  beq 0x82fda004
	if ctx.cr[0].eq {
	pc = 0x82FDA004; continue 'dispatch;
	}
	// 82FD9FF0: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FD9FF4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FD9FF8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FD9FFC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDA000: 4E800421  bctrl
	ctx.lr = 0x82FDA004;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDA004: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDA008: 809F0024  lwz r4, 0x24(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FDA00C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDA010: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FDA014: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDA018: 4E800421  bctrl
	ctx.lr = 0x82FDA01C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDA01C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FDA020: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FDA024: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FDA028: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FDA02C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDA030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDA030 size=176
    let mut pc: u32 = 0x82FDA030;
    'dispatch: loop {
        match pc {
            0x82FDA030 => {
    //   block [0x82FDA030..0x82FDA0E0)
	// 82FDA030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDA034: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FDA038: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FDA03C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FDA040: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDA044: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FDA048: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FDA04C: 809E0028  lwz r4, 0x28(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDA050: 909F0028  stw r4, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[4].u32 ) };
	// 82FDA054: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDA058: 4BFF6B29  bl 0x82fd0b80
	ctx.lr = 0x82FDA05C;
	sub_82FD0B80(ctx, base);
	// 82FDA05C: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82FDA060: 809F0028  lwz r4, 0x28(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDA064: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FDA068: 4BFF6B19  bl 0x82fd0b80
	ctx.lr = 0x82FDA06C;
	sub_82FD0B80(ctx, base);
	// 82FDA06C: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 82FDA070: 809F0028  lwz r4, 0x28(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDA074: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FDA078: 4BFF6B09  bl 0x82fd0b80
	ctx.lr = 0x82FDA07C;
	sub_82FD0B80(ctx, base);
	// 82FDA07C: 907F000C  stw r3, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 82FDA080: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FDA084: 809F0028  lwz r4, 0x28(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDA088: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82FDA08C: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FDA090: 4BFF6AF1  bl 0x82fd0b80
	ctx.lr = 0x82FDA094;
	sub_82FD0B80(ctx, base);
	// 82FDA094: 907F0014  stw r3, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 82FDA098: 809F0028  lwz r4, 0x28(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDA09C: 807E0018  lwz r3, 0x18(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FDA0A0: 4BFF6AE1  bl 0x82fd0b80
	ctx.lr = 0x82FDA0A4;
	sub_82FD0B80(ctx, base);
	// 82FDA0A4: 907F0018  stw r3, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[3].u32 ) };
	// 82FDA0A8: 809F0028  lwz r4, 0x28(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDA0AC: 807E001C  lwz r3, 0x1c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FDA0B0: 4BFF6AD1  bl 0x82fd0b80
	ctx.lr = 0x82FDA0B4;
	sub_82FD0B80(ctx, base);
	// 82FDA0B4: 907F001C  stw r3, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[3].u32 ) };
	// 82FDA0B8: 809F0028  lwz r4, 0x28(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDA0BC: 807E0020  lwz r3, 0x20(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FDA0C0: 4BFF6AC1  bl 0x82fd0b80
	ctx.lr = 0x82FDA0C4;
	sub_82FD0B80(ctx, base);
	// 82FDA0C4: 907F0020  stw r3, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[3].u32 ) };
	// 82FDA0C8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FDA0CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FDA0D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FDA0D4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FDA0D8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FDA0DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDA0E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDA0E0 size=1788
    let mut pc: u32 = 0x82FDA0E0;
    'dispatch: loop {
        match pc {
            0x82FDA0E0 => {
    //   block [0x82FDA0E0..0x82FDA7DC)
	// 82FDA0E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDA0E4: 481CE071  bl 0x831a8154
	ctx.lr = 0x82FDA0E8;
	sub_831A8130(ctx, base);
	// 82FDA0E8: 9421FC60  stwu r1, -0x3a0(r1)
	ea = ctx.r[1].u32.wrapping_add(-928 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDA0EC: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82FDA0F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FDA0F4: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82FDA0F8: 409A004C  bne cr6, 0x82fda144
	if !ctx.cr[6].eq {
	pc = 0x82FDA144; continue 'dispatch;
	}
	// 82FDA0FC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FDA100: 80DF0028  lwz r6, 0x28(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDA104: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FDA108: 396B9C84  addi r11, r11, -0x637c
	ctx.r[11].s64 = ctx.r[11].s64 + -25468;
	// 82FDA10C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82FDA110: 38EBFFE8  addi r7, r11, -0x18
	ctx.r[7].s64 = ctx.r[11].s64 + -24;
	// 82FDA114: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FDA118: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 82FDA11C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82FDA120: 388B9D88  addi r4, r11, -0x6278
	ctx.r[4].s64 = ctx.r[11].s64 + -25208;
	// 82FDA124: 38C0010A  li r6, 0x10a
	ctx.r[6].s64 = 266;
	// 82FDA128: 38A00353  li r5, 0x353
	ctx.r[5].s64 = 851;
	// 82FDA12C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82FDA130: 4BFFFAA1  bl 0x82fd9bd0
	ctx.lr = 0x82FDA134;
	sub_82FD9BD0(ctx, base);
	// 82FDA134: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FDA138: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82FDA13C: 388BC5E4  addi r4, r11, -0x3a1c
	ctx.r[4].s64 = ctx.r[11].s64 + -14876;
	// 82FDA140: 481D6AE9  bl 0x831b0c28
	ctx.lr = 0x82FDA144;
	sub_831B0C28(ctx, base);
	// 82FDA144: A15C0000  lhz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDA148: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 82FDA14C: 7F1EC378  mr r30, r24
	ctx.r[30].u64 = ctx.r[24].u64;
	// 82FDA150: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDA154: 41820028  beq 0x82fda17c
	if ctx.cr[0].eq {
	pc = 0x82FDA17C; continue 'dispatch;
	}
	// 82FDA158: 397C0002  addi r11, r28, 2
	ctx.r[11].s64 = ctx.r[28].s64 + 2;
	// 82FDA15C: 48000008  b 0x82fda164
	pc = 0x82FDA164; continue 'dispatch;
	// 82FDA160: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FDA164: A12B0000  lhz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDA168: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDA16C: 4082FFF4  bne 0x82fda160
	if !ctx.cr[0].eq {
	pc = 0x82FDA160; continue 'dispatch;
	}
	// 82FDA170: 7D7C5850  subf r11, r28, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[28].s64;
	// 82FDA174: 7D770E70  srawi r23, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[23].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82FDA178: 48000008  b 0x82fda180
	pc = 0x82FDA180; continue 'dispatch;
	// 82FDA17C: 7F17C378  mr r23, r24
	ctx.r[23].u64 = ctx.r[24].u64;
	// 82FDA180: 2F170000  cmpwi cr6, r23, 0
	ctx.cr[6].compare_i32(ctx.r[23].s32, 0, &mut ctx.xer);
	// 82FDA184: 409902AC  ble cr6, 0x82fda430
	if !ctx.cr[6].gt {
	pc = 0x82FDA430; continue 'dispatch;
	}
	// 82FDA188: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDA18C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDA190: 419A0140  beq cr6, 0x82fda2d0
	if ctx.cr[6].eq {
	pc = 0x82FDA2D0; continue 'dispatch;
	}
	// 82FDA194: 2B0A002F  cmplwi cr6, r10, 0x2f
	ctx.cr[6].compare_u32(ctx.r[10].u32, 47 as u32, &mut ctx.xer);
	// 82FDA198: 419A0138  beq cr6, 0x82fda2d0
	if ctx.cr[6].eq {
	pc = 0x82FDA2D0; continue 'dispatch;
	}
	// 82FDA19C: 3BBC0004  addi r29, r28, 4
	ctx.r[29].s64 = ctx.r[28].s64 + 4;
	// 82FDA1A0: A37DFFFC  lhz r27, -4(r29)
	ctx.r[27].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82FDA1A4: 7F6BDB78  mr r11, r27
	ctx.r[11].u64 = ctx.r[27].u64;
	// 82FDA1A8: 2B0B003F  cmplwi cr6, r11, 0x3f
	ctx.cr[6].compare_u32(ctx.r[11].u32, 63 as u32, &mut ctx.xer);
	// 82FDA1AC: 419A0288  beq cr6, 0x82fda434
	if ctx.cr[6].eq {
	pc = 0x82FDA434; continue 'dispatch;
	}
	// 82FDA1B0: 2B0B0023  cmplwi cr6, r11, 0x23
	ctx.cr[6].compare_u32(ctx.r[11].u32, 35 as u32, &mut ctx.xer);
	// 82FDA1B4: 419A0280  beq cr6, 0x82fda434
	if ctx.cr[6].eq {
	pc = 0x82FDA434; continue 'dispatch;
	}
	// 82FDA1B8: 2B0B0025  cmplwi cr6, r11, 0x25
	ctx.cr[6].compare_u32(ctx.r[11].u32, 37 as u32, &mut ctx.xer);
	// 82FDA1BC: 409A00A0  bne cr6, 0x82fda25c
	if !ctx.cr[6].eq {
	pc = 0x82FDA25C; continue 'dispatch;
	}
	// 82FDA1C0: 397E0002  addi r11, r30, 2
	ctx.r[11].s64 = ctx.r[30].s64 + 2;
	// 82FDA1C4: 7F0BB800  cmpw cr6, r11, r23
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[23].s32, &mut ctx.xer);
	// 82FDA1C8: 40980024  bge cr6, 0x82fda1ec
	if !ctx.cr[6].lt {
	pc = 0x82FDA1EC; continue 'dispatch;
	}
	// 82FDA1CC: A07DFFFE  lhz r3, -2(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(-2 as u32) ) } as u64;
	// 82FDA1D0: 4BFF7441  bl 0x82fd1610
	ctx.lr = 0x82FDA1D4;
	sub_82FD1610(ctx, base);
	// 82FDA1D4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDA1D8: 41820014  beq 0x82fda1ec
	if ctx.cr[0].eq {
	pc = 0x82FDA1EC; continue 'dispatch;
	}
	// 82FDA1DC: A07D0000  lhz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDA1E0: 4BFF7431  bl 0x82fd1610
	ctx.lr = 0x82FDA1E4;
	sub_82FD1610(ctx, base);
	// 82FDA1E4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDA1E8: 40820084  bne 0x82fda26c
	if !ctx.cr[0].eq {
	pc = 0x82FDA26C; continue 'dispatch;
	}
	// 82FDA1EC: 57CA083C  slwi r10, r30, 1
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FDA1F0: 813F0028  lwz r9, 0x28(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDA1F4: 39610230  addi r11, r1, 0x230
	ctx.r[11].s64 = ctx.r[1].s64 + 560;
	// 82FDA1F8: 7FEAE214  add r31, r10, r28
	ctx.r[31].u64 = ctx.r[10].u64 + ctx.r[28].u64;
	// 82FDA1FC: 3D408214  lis r10, -0x7dec
	ctx.r[10].s64 = -2112618496;
	// 82FDA200: 39010230  addi r8, r1, 0x230
	ctx.r[8].s64 = ctx.r[1].s64 + 560;
	// 82FDA204: 394A9C84  addi r10, r10, -0x637c
	ctx.r[10].s64 = ctx.r[10].s64 + -25468;
	// 82FDA208: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82FDA20C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82FDA210: A3DF0000  lhz r30, 0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDA214: 38EAFFE8  addi r7, r10, -0x18
	ctx.r[7].s64 = ctx.r[10].s64 + -24;
	// 82FDA218: A3BF0002  lhz r29, 2(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 82FDA21C: 3D408214  lis r10, -0x7dec
	ctx.r[10].s64 = -2112618496;
	// 82FDA220: A3FF0004  lhz r31, 4(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDA224: 38C0010C  li r6, 0x10c
	ctx.r[6].s64 = 268;
	// 82FDA228: 388A9D88  addi r4, r10, -0x6278
	ctx.r[4].s64 = ctx.r[10].s64 + -25208;
	// 82FDA22C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FDA230: B3CB0000  sth r30, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u16 ) };
	// 82FDA234: 38A003A7  li r5, 0x3a7
	ctx.r[5].s64 = 935;
	// 82FDA238: B3AB0002  sth r29, 2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(2 as u32), ctx.r[29].u16 ) };
	// 82FDA23C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82FDA240: B3EB0004  sth r31, 4(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[31].u16 ) };
	// 82FDA244: B3010236  sth r24, 0x236(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(566 as u32), ctx.r[24].u16 ) };
	// 82FDA248: 4BFFF989  bl 0x82fd9bd0
	ctx.lr = 0x82FDA24C;
	sub_82FD9BD0(ctx, base);
	// 82FDA24C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FDA250: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82FDA254: 388BC5E4  addi r4, r11, -0x3a1c
	ctx.r[4].s64 = ctx.r[11].s64 + -14876;
	// 82FDA258: 481D69D1  bl 0x831b0c28
	ctx.lr = 0x82FDA25C;
	sub_831B0C28(ctx, base);
	// 82FDA25C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82FDA260: 4BFFFC01  bl 0x82fd9e60
	ctx.lr = 0x82FDA264;
	sub_82FD9E60(ctx, base);
	// 82FDA264: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDA268: 41820018  beq 0x82fda280
	if ctx.cr[0].eq {
	pc = 0x82FDA280; continue 'dispatch;
	}
	// 82FDA26C: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82FDA270: 3BBD0002  addi r29, r29, 2
	ctx.r[29].s64 = ctx.r[29].s64 + 2;
	// 82FDA274: 7F1EB800  cmpw cr6, r30, r23
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[23].s32, &mut ctx.xer);
	// 82FDA278: 4198FF28  blt cr6, 0x82fda1a0
	if ctx.cr[6].lt {
	pc = 0x82FDA1A0; continue 'dispatch;
	}
	// 82FDA27C: 480001B8  b 0x82fda434
	pc = 0x82FDA434; continue 'dispatch;
	// 82FDA280: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FDA284: 80DF0028  lwz r6, 0x28(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDA288: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FDA28C: B3610080  sth r27, 0x80(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[27].u16 ) };
	// 82FDA290: 396B9C84  addi r11, r11, -0x637c
	ctx.r[11].s64 = ctx.r[11].s64 + -25468;
	// 82FDA294: B3010082  sth r24, 0x82(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(130 as u32), ctx.r[24].u16 ) };
	// 82FDA298: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82FDA29C: 38EBFFE8  addi r7, r11, -0x18
	ctx.r[7].s64 = ctx.r[11].s64 + -24;
	// 82FDA2A0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FDA2A4: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 82FDA2A8: 39010080  addi r8, r1, 0x80
	ctx.r[8].s64 = ctx.r[1].s64 + 128;
	// 82FDA2AC: 388B9D88  addi r4, r11, -0x6278
	ctx.r[4].s64 = ctx.r[11].s64 + -25208;
	// 82FDA2B0: 38C0010D  li r6, 0x10d
	ctx.r[6].s64 = 269;
	// 82FDA2B4: 38A003B8  li r5, 0x3b8
	ctx.r[5].s64 = 952;
	// 82FDA2B8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82FDA2BC: 4BFFF915  bl 0x82fd9bd0
	ctx.lr = 0x82FDA2C0;
	sub_82FD9BD0(ctx, base);
	// 82FDA2C0: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FDA2C4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82FDA2C8: 388BC5E4  addi r4, r11, -0x3a1c
	ctx.r[4].s64 = ctx.r[11].s64 + -14876;
	// 82FDA2CC: 481D695D  bl 0x831b0c28
	ctx.lr = 0x82FDA2D0;
	sub_831B0C28(ctx, base);
	// 82FDA2D0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FDA2D4: 3BBC0004  addi r29, r28, 4
	ctx.r[29].s64 = ctx.r[28].s64 + 4;
	// 82FDA2D8: 3B4B9BF4  addi r26, r11, -0x640c
	ctx.r[26].s64 = ctx.r[11].s64 + -25612;
	// 82FDA2DC: A37DFFFC  lhz r27, -4(r29)
	ctx.r[27].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82FDA2E0: 7F6BDB78  mr r11, r27
	ctx.r[11].u64 = ctx.r[27].u64;
	// 82FDA2E4: 2B0B003F  cmplwi cr6, r11, 0x3f
	ctx.cr[6].compare_u32(ctx.r[11].u32, 63 as u32, &mut ctx.xer);
	// 82FDA2E8: 419A014C  beq cr6, 0x82fda434
	if ctx.cr[6].eq {
	pc = 0x82FDA434; continue 'dispatch;
	}
	// 82FDA2EC: 2B0B0023  cmplwi cr6, r11, 0x23
	ctx.cr[6].compare_u32(ctx.r[11].u32, 35 as u32, &mut ctx.xer);
	// 82FDA2F0: 419A0144  beq cr6, 0x82fda434
	if ctx.cr[6].eq {
	pc = 0x82FDA434; continue 'dispatch;
	}
	// 82FDA2F4: 2B0B0025  cmplwi cr6, r11, 0x25
	ctx.cr[6].compare_u32(ctx.r[11].u32, 37 as u32, &mut ctx.xer);
	// 82FDA2F8: 409A00A0  bne cr6, 0x82fda398
	if !ctx.cr[6].eq {
	pc = 0x82FDA398; continue 'dispatch;
	}
	// 82FDA2FC: 397E0002  addi r11, r30, 2
	ctx.r[11].s64 = ctx.r[30].s64 + 2;
	// 82FDA300: 7F0BB800  cmpw cr6, r11, r23
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[23].s32, &mut ctx.xer);
	// 82FDA304: 40980024  bge cr6, 0x82fda328
	if !ctx.cr[6].lt {
	pc = 0x82FDA328; continue 'dispatch;
	}
	// 82FDA308: A07DFFFE  lhz r3, -2(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(-2 as u32) ) } as u64;
	// 82FDA30C: 4BFF7305  bl 0x82fd1610
	ctx.lr = 0x82FDA310;
	sub_82FD1610(ctx, base);
	// 82FDA310: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDA314: 41820014  beq 0x82fda328
	if ctx.cr[0].eq {
	pc = 0x82FDA328; continue 'dispatch;
	}
	// 82FDA318: A07D0000  lhz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDA31C: 4BFF72F5  bl 0x82fd1610
	ctx.lr = 0x82FDA320;
	sub_82FD1610(ctx, base);
	// 82FDA320: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDA324: 408200A8  bne 0x82fda3cc
	if !ctx.cr[0].eq {
	pc = 0x82FDA3CC; continue 'dispatch;
	}
	// 82FDA328: 57CA083C  slwi r10, r30, 1
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FDA32C: 813F0028  lwz r9, 0x28(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDA330: 39610110  addi r11, r1, 0x110
	ctx.r[11].s64 = ctx.r[1].s64 + 272;
	// 82FDA334: 7FEAE214  add r31, r10, r28
	ctx.r[31].u64 = ctx.r[10].u64 + ctx.r[28].u64;
	// 82FDA338: 3D408214  lis r10, -0x7dec
	ctx.r[10].s64 = -2112618496;
	// 82FDA33C: 39010110  addi r8, r1, 0x110
	ctx.r[8].s64 = ctx.r[1].s64 + 272;
	// 82FDA340: 394A9C84  addi r10, r10, -0x637c
	ctx.r[10].s64 = ctx.r[10].s64 + -25468;
	// 82FDA344: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82FDA348: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82FDA34C: A3DF0000  lhz r30, 0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDA350: 38EAFFE8  addi r7, r10, -0x18
	ctx.r[7].s64 = ctx.r[10].s64 + -24;
	// 82FDA354: A3BF0002  lhz r29, 2(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 82FDA358: 3D408214  lis r10, -0x7dec
	ctx.r[10].s64 = -2112618496;
	// 82FDA35C: A3FF0004  lhz r31, 4(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDA360: 38C0010C  li r6, 0x10c
	ctx.r[6].s64 = 268;
	// 82FDA364: 388A9D88  addi r4, r10, -0x6278
	ctx.r[4].s64 = ctx.r[10].s64 + -25208;
	// 82FDA368: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FDA36C: B3CB0000  sth r30, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u16 ) };
	// 82FDA370: 38A0037A  li r5, 0x37a
	ctx.r[5].s64 = 890;
	// 82FDA374: B3AB0002  sth r29, 2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(2 as u32), ctx.r[29].u16 ) };
	// 82FDA378: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82FDA37C: B3EB0004  sth r31, 4(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[31].u16 ) };
	// 82FDA380: B3010116  sth r24, 0x116(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(278 as u32), ctx.r[24].u16 ) };
	// 82FDA384: 4BFFF84D  bl 0x82fd9bd0
	ctx.lr = 0x82FDA388;
	sub_82FD9BD0(ctx, base);
	// 82FDA388: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FDA38C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82FDA390: 388BC5E4  addi r4, r11, -0x3a1c
	ctx.r[4].s64 = ctx.r[11].s64 + -14876;
	// 82FDA394: 481D6895  bl 0x831b0c28
	ctx.lr = 0x82FDA398;
	sub_831B0C28(ctx, base);
	// 82FDA398: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82FDA39C: 4BFFFB1D  bl 0x82fd9eb8
	ctx.lr = 0x82FDA3A0;
	sub_82FD9EB8(ctx, base);
	// 82FDA3A0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDA3A4: 40820028  bne 0x82fda3cc
	if !ctx.cr[0].eq {
	pc = 0x82FDA3CC; continue 'dispatch;
	}
	// 82FDA3A8: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82FDA3AC: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82FDA3B0: 4BFF7A01  bl 0x82fd1db0
	ctx.lr = 0x82FDA3B4;
	sub_82FD1DB0(ctx, base);
	// 82FDA3B4: 39630001  addi r11, r3, 1
	ctx.r[11].s64 = ctx.r[3].s64 + 1;
	// 82FDA3B8: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82FDA3BC: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82FDA3C0: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82FDA3C4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDA3C8: 41820018  beq 0x82fda3e0
	if ctx.cr[0].eq {
	pc = 0x82FDA3E0; continue 'dispatch;
	}
	// 82FDA3CC: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82FDA3D0: 3BBD0002  addi r29, r29, 2
	ctx.r[29].s64 = ctx.r[29].s64 + 2;
	// 82FDA3D4: 7F1EB800  cmpw cr6, r30, r23
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[23].s32, &mut ctx.xer);
	// 82FDA3D8: 4198FF04  blt cr6, 0x82fda2dc
	if ctx.cr[6].lt {
	pc = 0x82FDA2DC; continue 'dispatch;
	}
	// 82FDA3DC: 48000058  b 0x82fda434
	pc = 0x82FDA434; continue 'dispatch;
	// 82FDA3E0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FDA3E4: 80DF0028  lwz r6, 0x28(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDA3E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FDA3EC: B3610080  sth r27, 0x80(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[27].u16 ) };
	// 82FDA3F0: 396B9C84  addi r11, r11, -0x637c
	ctx.r[11].s64 = ctx.r[11].s64 + -25468;
	// 82FDA3F4: B3010082  sth r24, 0x82(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(130 as u32), ctx.r[24].u16 ) };
	// 82FDA3F8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82FDA3FC: 38EBFFE8  addi r7, r11, -0x18
	ctx.r[7].s64 = ctx.r[11].s64 + -24;
	// 82FDA400: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FDA404: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 82FDA408: 39010080  addi r8, r1, 0x80
	ctx.r[8].s64 = ctx.r[1].s64 + 128;
	// 82FDA40C: 388B9D88  addi r4, r11, -0x6278
	ctx.r[4].s64 = ctx.r[11].s64 + -25208;
	// 82FDA410: 38C0010D  li r6, 0x10d
	ctx.r[6].s64 = 269;
	// 82FDA414: 38A00387  li r5, 0x387
	ctx.r[5].s64 = 903;
	// 82FDA418: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82FDA41C: 4BFFF7B5  bl 0x82fd9bd0
	ctx.lr = 0x82FDA420;
	sub_82FD9BD0(ctx, base);
	// 82FDA420: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FDA424: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82FDA428: 388BC5E4  addi r4, r11, -0x3a1c
	ctx.r[4].s64 = ctx.r[11].s64 + -14876;
	// 82FDA42C: 481D67FD  bl 0x831b0c28
	ctx.lr = 0x82FDA430;
	sub_831B0C28(ctx, base);
	// 82FDA430: A3610078  lhz r27, 0x78(r1)
	ctx.r[27].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 82FDA434: 809F0018  lwz r4, 0x18(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FDA438: 2C040000  cmpwi r4, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82FDA43C: 41820018  beq 0x82fda454
	if ctx.cr[0].eq {
	pc = 0x82FDA454; continue 'dispatch;
	}
	// 82FDA440: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDA444: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDA448: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FDA44C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDA450: 4E800421  bctrl
	ctx.lr = 0x82FDA454;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDA454: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDA458: 397E0001  addi r11, r30, 1
	ctx.r[11].s64 = ctx.r[30].s64 + 1;
	// 82FDA45C: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82FDA460: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDA464: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDA468: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDA46C: 4E800421  bctrl
	ctx.lr = 0x82FDA470;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDA470: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82FDA474: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FDA478: 80FF0028  lwz r7, 0x28(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDA47C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FDA480: 907F0018  stw r3, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[3].u32 ) };
	// 82FDA484: 4BFF7C05  bl 0x82fd2088
	ctx.lr = 0x82FDA488;
	sub_82FD2088(ctx, base);
	// 82FDA488: 576B043E  clrlwi r11, r27, 0x10
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0x0000FFFFu64;
	// 82FDA48C: 2B0B003F  cmplwi cr6, r11, 0x3f
	ctx.cr[6].compare_u32(ctx.r[11].u32, 63 as u32, &mut ctx.xer);
	// 82FDA490: 409A0150  bne cr6, 0x82fda5e0
	if !ctx.cr[6].eq {
	pc = 0x82FDA5E0; continue 'dispatch;
	}
	// 82FDA494: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82FDA498: 7FD9F378  mr r25, r30
	ctx.r[25].u64 = ctx.r[30].u64;
	// 82FDA49C: 7F1EB800  cmpw cr6, r30, r23
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[23].s32, &mut ctx.xer);
	// 82FDA4A0: 409800E8  bge cr6, 0x82fda588
	if !ctx.cr[6].lt {
	pc = 0x82FDA588; continue 'dispatch;
	}
	// 82FDA4A4: 397E0002  addi r11, r30, 2
	ctx.r[11].s64 = ctx.r[30].s64 + 2;
	// 82FDA4A8: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FDA4AC: 7D7A5B78  mr r26, r11
	ctx.r[26].u64 = ctx.r[11].u64;
	// 82FDA4B0: 7FAAE214  add r29, r10, r28
	ctx.r[29].u64 = ctx.r[10].u64 + ctx.r[28].u64;
	// 82FDA4B4: A37DFFFC  lhz r27, -4(r29)
	ctx.r[27].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82FDA4B8: 7F6BDB78  mr r11, r27
	ctx.r[11].u64 = ctx.r[27].u64;
	// 82FDA4BC: 2B0B0023  cmplwi cr6, r11, 0x23
	ctx.cr[6].compare_u32(ctx.r[11].u32, 35 as u32, &mut ctx.xer);
	// 82FDA4C0: 419A00C8  beq cr6, 0x82fda588
	if ctx.cr[6].eq {
	pc = 0x82FDA588; continue 'dispatch;
	}
	// 82FDA4C4: 2B0B0025  cmplwi cr6, r11, 0x25
	ctx.cr[6].compare_u32(ctx.r[11].u32, 37 as u32, &mut ctx.xer);
	// 82FDA4C8: 409A009C  bne cr6, 0x82fda564
	if !ctx.cr[6].eq {
	pc = 0x82FDA564; continue 'dispatch;
	}
	// 82FDA4CC: 7F1AB800  cmpw cr6, r26, r23
	ctx.cr[6].compare_i32(ctx.r[26].s32, ctx.r[23].s32, &mut ctx.xer);
	// 82FDA4D0: 40980024  bge cr6, 0x82fda4f4
	if !ctx.cr[6].lt {
	pc = 0x82FDA4F4; continue 'dispatch;
	}
	// 82FDA4D4: A07DFFFE  lhz r3, -2(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(-2 as u32) ) } as u64;
	// 82FDA4D8: 4BFF7139  bl 0x82fd1610
	ctx.lr = 0x82FDA4DC;
	sub_82FD1610(ctx, base);
	// 82FDA4DC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDA4E0: 41820014  beq 0x82fda4f4
	if ctx.cr[0].eq {
	pc = 0x82FDA4F4; continue 'dispatch;
	}
	// 82FDA4E4: A07D0000  lhz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDA4E8: 4BFF7129  bl 0x82fd1610
	ctx.lr = 0x82FDA4EC;
	sub_82FD1610(ctx, base);
	// 82FDA4EC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDA4F0: 40820084  bne 0x82fda574
	if !ctx.cr[0].eq {
	pc = 0x82FDA574; continue 'dispatch;
	}
	// 82FDA4F4: 57CA083C  slwi r10, r30, 1
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FDA4F8: 813F0028  lwz r9, 0x28(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDA4FC: 396101A0  addi r11, r1, 0x1a0
	ctx.r[11].s64 = ctx.r[1].s64 + 416;
	// 82FDA500: 7FEAE214  add r31, r10, r28
	ctx.r[31].u64 = ctx.r[10].u64 + ctx.r[28].u64;
	// 82FDA504: 3D408214  lis r10, -0x7dec
	ctx.r[10].s64 = -2112618496;
	// 82FDA508: 390101A0  addi r8, r1, 0x1a0
	ctx.r[8].s64 = ctx.r[1].s64 + 416;
	// 82FDA50C: 394A9C84  addi r10, r10, -0x637c
	ctx.r[10].s64 = ctx.r[10].s64 + -25468;
	// 82FDA510: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82FDA514: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82FDA518: A3DF0000  lhz r30, 0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDA51C: 38EAFFF4  addi r7, r10, -0xc
	ctx.r[7].s64 = ctx.r[10].s64 + -12;
	// 82FDA520: A3BF0002  lhz r29, 2(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 82FDA524: 3D408214  lis r10, -0x7dec
	ctx.r[10].s64 = -2112618496;
	// 82FDA528: A3FF0004  lhz r31, 4(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDA52C: 38C0010C  li r6, 0x10c
	ctx.r[6].s64 = 268;
	// 82FDA530: 388A9D88  addi r4, r10, -0x6278
	ctx.r[4].s64 = ctx.r[10].s64 + -25208;
	// 82FDA534: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FDA538: B3CB0000  sth r30, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u16 ) };
	// 82FDA53C: 38A003E2  li r5, 0x3e2
	ctx.r[5].s64 = 994;
	// 82FDA540: B3AB0002  sth r29, 2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(2 as u32), ctx.r[29].u16 ) };
	// 82FDA544: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82FDA548: B3EB0004  sth r31, 4(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[31].u16 ) };
	// 82FDA54C: B30101A6  sth r24, 0x1a6(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(422 as u32), ctx.r[24].u16 ) };
	// 82FDA550: 4BFFF681  bl 0x82fd9bd0
	ctx.lr = 0x82FDA554;
	sub_82FD9BD0(ctx, base);
	// 82FDA554: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FDA558: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82FDA55C: 388BC5E4  addi r4, r11, -0x3a1c
	ctx.r[4].s64 = ctx.r[11].s64 + -14876;
	// 82FDA560: 481D66C9  bl 0x831b0c28
	ctx.lr = 0x82FDA564;
	sub_831B0C28(ctx, base);
	// 82FDA564: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82FDA568: 4BFFF8F9  bl 0x82fd9e60
	ctx.lr = 0x82FDA56C;
	sub_82FD9E60(ctx, base);
	// 82FDA56C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDA570: 4182013C  beq 0x82fda6ac
	if ctx.cr[0].eq {
	pc = 0x82FDA6AC; continue 'dispatch;
	}
	// 82FDA574: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82FDA578: 3BBD0002  addi r29, r29, 2
	ctx.r[29].s64 = ctx.r[29].s64 + 2;
	// 82FDA57C: 3B5A0001  addi r26, r26, 1
	ctx.r[26].s64 = ctx.r[26].s64 + 1;
	// 82FDA580: 7F1EB800  cmpw cr6, r30, r23
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[23].s32, &mut ctx.xer);
	// 82FDA584: 4198FF30  blt cr6, 0x82fda4b4
	if ctx.cr[6].lt {
	pc = 0x82FDA4B4; continue 'dispatch;
	}
	// 82FDA588: 809F001C  lwz r4, 0x1c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FDA58C: 2C040000  cmpwi r4, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82FDA590: 41820018  beq 0x82fda5a8
	if ctx.cr[0].eq {
	pc = 0x82FDA5A8; continue 'dispatch;
	}
	// 82FDA594: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDA598: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDA59C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FDA5A0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDA5A4: 4E800421  bctrl
	ctx.lr = 0x82FDA5A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDA5A8: 7D79F050  subf r11, r25, r30
	ctx.r[11].s64 = ctx.r[30].s64 - ctx.r[25].s64;
	// 82FDA5AC: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDA5B0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FDA5B4: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82FDA5B8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDA5BC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDA5C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDA5C4: 4E800421  bctrl
	ctx.lr = 0x82FDA5C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDA5C8: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82FDA5CC: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 82FDA5D0: 80FF0028  lwz r7, 0x28(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDA5D4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FDA5D8: 907F001C  stw r3, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[3].u32 ) };
	// 82FDA5DC: 4BFF7AAD  bl 0x82fd2088
	ctx.lr = 0x82FDA5E0;
	sub_82FD2088(ctx, base);
	// 82FDA5E0: 576B043E  clrlwi r11, r27, 0x10
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0x0000FFFFu64;
	// 82FDA5E4: 2B0B0023  cmplwi cr6, r11, 0x23
	ctx.cr[6].compare_u32(ctx.r[11].u32, 35 as u32, &mut ctx.xer);
	// 82FDA5E8: 409A01EC  bne cr6, 0x82fda7d4
	if !ctx.cr[6].eq {
	pc = 0x82FDA7D4; continue 'dispatch;
	}
	// 82FDA5EC: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82FDA5F0: 7FD9F378  mr r25, r30
	ctx.r[25].u64 = ctx.r[30].u64;
	// 82FDA5F4: 7F1EB800  cmpw cr6, r30, r23
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[23].s32, &mut ctx.xer);
	// 82FDA5F8: 40980128  bge cr6, 0x82fda720
	if !ctx.cr[6].lt {
	pc = 0x82FDA720; continue 'dispatch;
	}
	// 82FDA5FC: 397E0002  addi r11, r30, 2
	ctx.r[11].s64 = ctx.r[30].s64 + 2;
	// 82FDA600: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FDA604: 7D7A5B78  mr r26, r11
	ctx.r[26].u64 = ctx.r[11].u64;
	// 82FDA608: 7FAAE214  add r29, r10, r28
	ctx.r[29].u64 = ctx.r[10].u64 + ctx.r[28].u64;
	// 82FDA60C: A37DFFFC  lhz r27, -4(r29)
	ctx.r[27].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82FDA610: 2B1B0025  cmplwi cr6, r27, 0x25
	ctx.cr[6].compare_u32(ctx.r[27].u32, 37 as u32, &mut ctx.xer);
	// 82FDA614: 409A00E8  bne cr6, 0x82fda6fc
	if !ctx.cr[6].eq {
	pc = 0x82FDA6FC; continue 'dispatch;
	}
	// 82FDA618: 7F1AB800  cmpw cr6, r26, r23
	ctx.cr[6].compare_i32(ctx.r[26].s32, ctx.r[23].s32, &mut ctx.xer);
	// 82FDA61C: 40980024  bge cr6, 0x82fda640
	if !ctx.cr[6].lt {
	pc = 0x82FDA640; continue 'dispatch;
	}
	// 82FDA620: A07DFFFE  lhz r3, -2(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(-2 as u32) ) } as u64;
	// 82FDA624: 4BFF6FED  bl 0x82fd1610
	ctx.lr = 0x82FDA628;
	sub_82FD1610(ctx, base);
	// 82FDA628: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDA62C: 41820014  beq 0x82fda640
	if ctx.cr[0].eq {
	pc = 0x82FDA640; continue 'dispatch;
	}
	// 82FDA630: A07D0000  lhz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDA634: 4BFF6FDD  bl 0x82fd1610
	ctx.lr = 0x82FDA638;
	sub_82FD1610(ctx, base);
	// 82FDA638: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDA63C: 408200D0  bne 0x82fda70c
	if !ctx.cr[0].eq {
	pc = 0x82FDA70C; continue 'dispatch;
	}
	// 82FDA640: 57CA083C  slwi r10, r30, 1
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FDA644: 813F0028  lwz r9, 0x28(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDA648: 396102C0  addi r11, r1, 0x2c0
	ctx.r[11].s64 = ctx.r[1].s64 + 704;
	// 82FDA64C: 7FEAE214  add r31, r10, r28
	ctx.r[31].u64 = ctx.r[10].u64 + ctx.r[28].u64;
	// 82FDA650: 3D408214  lis r10, -0x7dec
	ctx.r[10].s64 = -2112618496;
	// 82FDA654: 390102C0  addi r8, r1, 0x2c0
	ctx.r[8].s64 = ctx.r[1].s64 + 704;
	// 82FDA658: 38EA9C84  addi r7, r10, -0x637c
	ctx.r[7].s64 = ctx.r[10].s64 + -25468;
	// 82FDA65C: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82FDA660: 3D408214  lis r10, -0x7dec
	ctx.r[10].s64 = -2112618496;
	// 82FDA664: A3DF0000  lhz r30, 0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDA668: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82FDA66C: A3BF0002  lhz r29, 2(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 82FDA670: 388A9D88  addi r4, r10, -0x6278
	ctx.r[4].s64 = ctx.r[10].s64 + -25208;
	// 82FDA674: A3FF0004  lhz r31, 4(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDA678: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FDA67C: 38C0010C  li r6, 0x10c
	ctx.r[6].s64 = 268;
	// 82FDA680: 38A00415  li r5, 0x415
	ctx.r[5].s64 = 1045;
	// 82FDA684: B3CB0000  sth r30, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u16 ) };
	// 82FDA688: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82FDA68C: B3AB0002  sth r29, 2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(2 as u32), ctx.r[29].u16 ) };
	// 82FDA690: B3EB0004  sth r31, 4(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[31].u16 ) };
	// 82FDA694: B30102C6  sth r24, 0x2c6(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(710 as u32), ctx.r[24].u16 ) };
	// 82FDA698: 4BFFF539  bl 0x82fd9bd0
	ctx.lr = 0x82FDA69C;
	sub_82FD9BD0(ctx, base);
	// 82FDA69C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FDA6A0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82FDA6A4: 388BC5E4  addi r4, r11, -0x3a1c
	ctx.r[4].s64 = ctx.r[11].s64 + -14876;
	// 82FDA6A8: 481D6581  bl 0x831b0c28
	ctx.lr = 0x82FDA6AC;
	sub_831B0C28(ctx, base);
	// 82FDA6AC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FDA6B0: 80DF0028  lwz r6, 0x28(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDA6B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FDA6B8: B3610080  sth r27, 0x80(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[27].u16 ) };
	// 82FDA6BC: 396B9C84  addi r11, r11, -0x637c
	ctx.r[11].s64 = ctx.r[11].s64 + -25468;
	// 82FDA6C0: B3010082  sth r24, 0x82(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(130 as u32), ctx.r[24].u16 ) };
	// 82FDA6C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82FDA6C8: 38EBFFF4  addi r7, r11, -0xc
	ctx.r[7].s64 = ctx.r[11].s64 + -12;
	// 82FDA6CC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FDA6D0: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 82FDA6D4: 39010080  addi r8, r1, 0x80
	ctx.r[8].s64 = ctx.r[1].s64 + 128;
	// 82FDA6D8: 388B9D88  addi r4, r11, -0x6278
	ctx.r[4].s64 = ctx.r[11].s64 + -25208;
	// 82FDA6DC: 38C0010D  li r6, 0x10d
	ctx.r[6].s64 = 269;
	// 82FDA6E0: 38A003EE  li r5, 0x3ee
	ctx.r[5].s64 = 1006;
	// 82FDA6E4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82FDA6E8: 4BFFF4E9  bl 0x82fd9bd0
	ctx.lr = 0x82FDA6EC;
	sub_82FD9BD0(ctx, base);
	// 82FDA6EC: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FDA6F0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82FDA6F4: 388BC5E4  addi r4, r11, -0x3a1c
	ctx.r[4].s64 = ctx.r[11].s64 + -14876;
	// 82FDA6F8: 481D6531  bl 0x831b0c28
	ctx.lr = 0x82FDA6FC;
	sub_831B0C28(ctx, base);
	// 82FDA6FC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82FDA700: 4BFFF761  bl 0x82fd9e60
	ctx.lr = 0x82FDA704;
	sub_82FD9E60(ctx, base);
	// 82FDA704: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDA708: 4182007C  beq 0x82fda784
	if ctx.cr[0].eq {
	pc = 0x82FDA784; continue 'dispatch;
	}
	// 82FDA70C: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82FDA710: 3BBD0002  addi r29, r29, 2
	ctx.r[29].s64 = ctx.r[29].s64 + 2;
	// 82FDA714: 3B5A0001  addi r26, r26, 1
	ctx.r[26].s64 = ctx.r[26].s64 + 1;
	// 82FDA718: 7F1EB800  cmpw cr6, r30, r23
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[23].s32, &mut ctx.xer);
	// 82FDA71C: 4198FEF0  blt cr6, 0x82fda60c
	if ctx.cr[6].lt {
	pc = 0x82FDA60C; continue 'dispatch;
	}
	// 82FDA720: 809F0020  lwz r4, 0x20(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FDA724: 2C040000  cmpwi r4, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82FDA728: 41820018  beq 0x82fda740
	if ctx.cr[0].eq {
	pc = 0x82FDA740; continue 'dispatch;
	}
	// 82FDA72C: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDA730: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDA734: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FDA738: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDA73C: 4E800421  bctrl
	ctx.lr = 0x82FDA740;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDA740: 7F1EC800  cmpw cr6, r30, r25
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[25].s32, &mut ctx.xer);
	// 82FDA744: 4099008C  ble cr6, 0x82fda7d0
	if !ctx.cr[6].gt {
	pc = 0x82FDA7D0; continue 'dispatch;
	}
	// 82FDA748: 7D79F050  subf r11, r25, r30
	ctx.r[11].s64 = ctx.r[30].s64 - ctx.r[25].s64;
	// 82FDA74C: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDA750: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FDA754: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82FDA758: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDA75C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDA760: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDA764: 4E800421  bctrl
	ctx.lr = 0x82FDA768;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDA768: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82FDA76C: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 82FDA770: 80FF0028  lwz r7, 0x28(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDA774: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FDA778: 907F0020  stw r3, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[3].u32 ) };
	// 82FDA77C: 4BFF790D  bl 0x82fd2088
	ctx.lr = 0x82FDA780;
	sub_82FD2088(ctx, base);
	// 82FDA780: 48000054  b 0x82fda7d4
	pc = 0x82FDA7D4; continue 'dispatch;
	// 82FDA784: 80DF0028  lwz r6, 0x28(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDA788: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FDA78C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FDA790: B3610080  sth r27, 0x80(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[27].u16 ) };
	// 82FDA794: 38EB9C84  addi r7, r11, -0x637c
	ctx.r[7].s64 = ctx.r[11].s64 + -25468;
	// 82FDA798: B3010082  sth r24, 0x82(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(130 as u32), ctx.r[24].u16 ) };
	// 82FDA79C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FDA7A0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82FDA7A4: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 82FDA7A8: 388B9D88  addi r4, r11, -0x6278
	ctx.r[4].s64 = ctx.r[11].s64 + -25208;
	// 82FDA7AC: 39010080  addi r8, r1, 0x80
	ctx.r[8].s64 = ctx.r[1].s64 + 128;
	// 82FDA7B0: 38C0010D  li r6, 0x10d
	ctx.r[6].s64 = 269;
	// 82FDA7B4: 38A00421  li r5, 0x421
	ctx.r[5].s64 = 1057;
	// 82FDA7B8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82FDA7BC: 4BFFF415  bl 0x82fd9bd0
	ctx.lr = 0x82FDA7C0;
	sub_82FD9BD0(ctx, base);
	// 82FDA7C0: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FDA7C4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82FDA7C8: 388BC5E4  addi r4, r11, -0x3a1c
	ctx.r[4].s64 = ctx.r[11].s64 + -14876;
	// 82FDA7CC: 481D645D  bl 0x831b0c28
	ctx.lr = 0x82FDA7D0;
	sub_831B0C28(ctx, base);
	// 82FDA7D0: 931F0020  stw r24, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[24].u32 ) };
	// 82FDA7D4: 382103A0  addi r1, r1, 0x3a0
	ctx.r[1].s64 = ctx.r[1].s64 + 928;
	// 82FDA7D8: 481CD9CC  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDA7E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDA7E0 size=156
    let mut pc: u32 = 0x82FDA7E0;
    'dispatch: loop {
        match pc {
            0x82FDA7E0 => {
    //   block [0x82FDA7E0..0x82FDA87C)
	// 82FDA7E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDA7E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FDA7E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FDA7EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FDA7F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDA7F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FDA7F8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82FDA7FC: 409A000C  bne cr6, 0x82fda808
	if !ctx.cr[6].eq {
	pc = 0x82FDA808; continue 'dispatch;
	}
	// 82FDA800: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FDA804: 48000060  b 0x82fda864
	pc = 0x82FDA864; continue 'dispatch;
	// 82FDA808: A07F0000  lhz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDA80C: 4BFF6D45  bl 0x82fd1550
	ctx.lr = 0x82FDA810;
	sub_82FD1550(ctx, base);
	// 82FDA810: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDA814: 4182FFEC  beq 0x82fda800
	if ctx.cr[0].eq {
	pc = 0x82FDA800; continue 'dispatch;
	}
	// 82FDA818: 3BFF0002  addi r31, r31, 2
	ctx.r[31].s64 = ctx.r[31].s64 + 2;
	// 82FDA81C: A07F0000  lhz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDA820: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDA824: 4182003C  beq 0x82fda860
	if ctx.cr[0].eq {
	pc = 0x82FDA860; continue 'dispatch;
	}
	// 82FDA828: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FDA82C: 3BCB9BC8  addi r30, r11, -0x6438
	ctx.r[30].s64 = ctx.r[11].s64 + -25656;
	// 82FDA830: 4BFF6D79  bl 0x82fd15a8
	ctx.lr = 0x82FDA834;
	sub_82FD15A8(ctx, base);
	// 82FDA834: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDA838: 40820018  bne 0x82fda850
	if !ctx.cr[0].eq {
	pc = 0x82FDA850; continue 'dispatch;
	}
	// 82FDA83C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FDA840: A09F0000  lhz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDA844: 4BFF756D  bl 0x82fd1db0
	ctx.lr = 0x82FDA848;
	sub_82FD1DB0(ctx, base);
	// 82FDA848: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82FDA84C: 419AFFB4  beq cr6, 0x82fda800
	if ctx.cr[6].eq {
	pc = 0x82FDA800; continue 'dispatch;
	}
	// 82FDA850: 3BFF0002  addi r31, r31, 2
	ctx.r[31].s64 = ctx.r[31].s64 + 2;
	// 82FDA854: A07F0000  lhz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDA858: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDA85C: 4082FFD4  bne 0x82fda830
	if !ctx.cr[0].eq {
	pc = 0x82FDA830; continue 'dispatch;
	}
	// 82FDA860: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82FDA864: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FDA868: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FDA86C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FDA870: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FDA874: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FDA878: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDA880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDA880 size=324
    let mut pc: u32 = 0x82FDA880;
    'dispatch: loop {
        match pc {
            0x82FDA880 => {
    //   block [0x82FDA880..0x82FDA9C4)
	// 82FDA880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDA884: 481CD8E5  bl 0x831a8168
	ctx.lr = 0x82FDA888;
	sub_831A8130(ctx, base);
	// 82FDA888: 9421FED0  stwu r1, -0x130(r1)
	ea = ctx.r[1].u32.wrapping_add(-304 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDA88C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FDA890: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82FDA894: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82FDA898: 419A0084  beq cr6, 0x82fda91c
	if ctx.cr[6].eq {
	pc = 0x82FDA91C; continue 'dispatch;
	}
	// 82FDA89C: A17E0000  lhz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDA8A0: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 82FDA8A4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDA8A8: 41820074  beq 0x82fda91c
	if ctx.cr[0].eq {
	pc = 0x82FDA91C; continue 'dispatch;
	}
	// 82FDA8AC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FDA8B0: 3B8B9BD0  addi r28, r11, -0x6430
	ctx.r[28].s64 = ctx.r[11].s64 + -25648;
	// 82FDA8B4: A07F0000  lhz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDA8B8: 4BFFF601  bl 0x82fd9eb8
	ctx.lr = 0x82FDA8BC;
	sub_82FD9EB8(ctx, base);
	// 82FDA8BC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDA8C0: 4082004C  bne 0x82fda90c
	if !ctx.cr[0].eq {
	pc = 0x82FDA90C; continue 'dispatch;
	}
	// 82FDA8C4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FDA8C8: A09F0000  lhz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDA8CC: 4BFF74E5  bl 0x82fd1db0
	ctx.lr = 0x82FDA8D0;
	sub_82FD1DB0(ctx, base);
	// 82FDA8D0: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82FDA8D4: 409A0038  bne cr6, 0x82fda90c
	if !ctx.cr[6].eq {
	pc = 0x82FDA90C; continue 'dispatch;
	}
	// 82FDA8D8: A17F0000  lhz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDA8DC: 2B0B0025  cmplwi cr6, r11, 0x25
	ctx.cr[6].compare_u32(ctx.r[11].u32, 37 as u32, &mut ctx.xer);
	// 82FDA8E0: 409A00A4  bne cr6, 0x82fda984
	if !ctx.cr[6].eq {
	pc = 0x82FDA984; continue 'dispatch;
	}
	// 82FDA8E4: A07F0002  lhz r3, 2(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 82FDA8E8: 4BFF6D29  bl 0x82fd1610
	ctx.lr = 0x82FDA8EC;
	sub_82FD1610(ctx, base);
	// 82FDA8EC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDA8F0: 41820034  beq 0x82fda924
	if ctx.cr[0].eq {
	pc = 0x82FDA924; continue 'dispatch;
	}
	// 82FDA8F4: A07F0004  lhz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDA8F8: 4BFF6D19  bl 0x82fd1610
	ctx.lr = 0x82FDA8FC;
	sub_82FD1610(ctx, base);
	// 82FDA8FC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDA900: 41820024  beq 0x82fda924
	if ctx.cr[0].eq {
	pc = 0x82FDA924; continue 'dispatch;
	}
	// 82FDA904: 3BFF0006  addi r31, r31, 6
	ctx.r[31].s64 = ctx.r[31].s64 + 6;
	// 82FDA908: 48000008  b 0x82fda910
	pc = 0x82FDA910; continue 'dispatch;
	// 82FDA90C: 3BFF0002  addi r31, r31, 2
	ctx.r[31].s64 = ctx.r[31].s64 + 2;
	// 82FDA910: A17F0000  lhz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDA914: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDA918: 4082FF9C  bne 0x82fda8b4
	if !ctx.cr[0].eq {
	pc = 0x82FDA8B4; continue 'dispatch;
	}
	// 82FDA91C: 38210130  addi r1, r1, 0x130
	ctx.r[1].s64 = ctx.r[1].s64 + 304;
	// 82FDA920: 481CD898  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 82FDA924: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FDA928: 93A10054  stw r29, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 82FDA92C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FDA930: 38EB9C30  addi r7, r11, -0x63d0
	ctx.r[7].s64 = ctx.r[11].s64 + -25552;
	// 82FDA934: A17F0002  lhz r11, 2(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 82FDA938: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82FDA93C: 39010080  addi r8, r1, 0x80
	ctx.r[8].s64 = ctx.r[1].s64 + 128;
	// 82FDA940: 38C0010C  li r6, 0x10c
	ctx.r[6].s64 = 268;
	// 82FDA944: 38A005A2  li r5, 0x5a2
	ctx.r[5].s64 = 1442;
	// 82FDA948: B1610082  sth r11, 0x82(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(130 as u32), ctx.r[11].u16 ) };
	// 82FDA94C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FDA950: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82FDA954: 388B9D88  addi r4, r11, -0x6278
	ctx.r[4].s64 = ctx.r[11].s64 + -25208;
	// 82FDA958: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDA95C: B1610084  sth r11, 0x84(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[11].u16 ) };
	// 82FDA960: 39600025  li r11, 0x25
	ctx.r[11].s64 = 37;
	// 82FDA964: B1610080  sth r11, 0x80(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[11].u16 ) };
	// 82FDA968: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FDA96C: B1610086  sth r11, 0x86(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(134 as u32), ctx.r[11].u16 ) };
	// 82FDA970: 4BFFF261  bl 0x82fd9bd0
	ctx.lr = 0x82FDA974;
	sub_82FD9BD0(ctx, base);
	// 82FDA974: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FDA978: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82FDA97C: 388BC5E4  addi r4, r11, -0x3a1c
	ctx.r[4].s64 = ctx.r[11].s64 + -14876;
	// 82FDA980: 481D62A9  bl 0x831b0c28
	ctx.lr = 0x82FDA984;
	sub_831B0C28(ctx, base);
	// 82FDA984: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FDA988: 93A10054  stw r29, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 82FDA98C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FDA990: 38EB9C30  addi r7, r11, -0x63d0
	ctx.r[7].s64 = ctx.r[11].s64 + -25552;
	// 82FDA994: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FDA998: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82FDA99C: 388B9D88  addi r4, r11, -0x6278
	ctx.r[4].s64 = ctx.r[11].s64 + -25208;
	// 82FDA9A0: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 82FDA9A4: 38C0010D  li r6, 0x10d
	ctx.r[6].s64 = 269;
	// 82FDA9A8: 38A005AB  li r5, 0x5ab
	ctx.r[5].s64 = 1451;
	// 82FDA9AC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82FDA9B0: 4BFFF221  bl 0x82fd9bd0
	ctx.lr = 0x82FDA9B4;
	sub_82FD9BD0(ctx, base);
	// 82FDA9B4: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FDA9B8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82FDA9BC: 388BC5E4  addi r4, r11, -0x3a1c
	ctx.r[4].s64 = ctx.r[11].s64 + -14876;
	// 82FDA9C0: 481D6269  bl 0x831b0c28
	ctx.lr = 0x82FDA9C4;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDA9C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDA9C8 size=180
    let mut pc: u32 = 0x82FDA9C8;
    'dispatch: loop {
        match pc {
            0x82FDA9C8 => {
    //   block [0x82FDA9C8..0x82FDAA7C)
	// 82FDA9C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDA9CC: 481CD799  bl 0x831a8164
	ctx.lr = 0x82FDA9D0;
	sub_831A8130(ctx, base);
	// 82FDA9D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDA9D4: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82FDA9D8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82FDA9DC: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 82FDA9E0: 40990088  ble cr6, 0x82fdaa68
	if !ctx.cr[6].gt {
	pc = 0x82FDAA68; continue 'dispatch;
	}
	// 82FDA9E4: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FDA9E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FDA9EC: 3BC30004  addi r30, r3, 4
	ctx.r[30].s64 = ctx.r[3].s64 + 4;
	// 82FDA9F0: 3B8B9BE0  addi r28, r11, -0x6420
	ctx.r[28].s64 = ctx.r[11].s64 + -25632;
	// 82FDA9F4: A07F0000  lhz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDA9F8: 4BFFF4C1  bl 0x82fd9eb8
	ctx.lr = 0x82FDA9FC;
	sub_82FD9EB8(ctx, base);
	// 82FDA9FC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDAA00: 40820054  bne 0x82fdaa54
	if !ctx.cr[0].eq {
	pc = 0x82FDAA54; continue 'dispatch;
	}
	// 82FDAA04: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FDAA08: A09F0000  lhz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDAA0C: 4BFF73A5  bl 0x82fd1db0
	ctx.lr = 0x82FDAA10;
	sub_82FD1DB0(ctx, base);
	// 82FDAA10: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82FDAA14: 409A0040  bne cr6, 0x82fdaa54
	if !ctx.cr[6].eq {
	pc = 0x82FDAA54; continue 'dispatch;
	}
	// 82FDAA18: A17F0000  lhz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDAA1C: 2B0B0025  cmplwi cr6, r11, 0x25
	ctx.cr[6].compare_u32(ctx.r[11].u32, 37 as u32, &mut ctx.xer);
	// 82FDAA20: 409A0054  bne cr6, 0x82fdaa74
	if !ctx.cr[6].eq {
	pc = 0x82FDAA74; continue 'dispatch;
	}
	// 82FDAA24: A07F0002  lhz r3, 2(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 82FDAA28: 4BFF6BE9  bl 0x82fd1610
	ctx.lr = 0x82FDAA2C;
	sub_82FD1610(ctx, base);
	// 82FDAA2C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDAA30: 41820044  beq 0x82fdaa74
	if ctx.cr[0].eq {
	pc = 0x82FDAA74; continue 'dispatch;
	}
	// 82FDAA34: A07E0000  lhz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDAA38: 4BFF6BD9  bl 0x82fd1610
	ctx.lr = 0x82FDAA3C;
	sub_82FD1610(ctx, base);
	// 82FDAA3C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDAA40: 41820034  beq 0x82fdaa74
	if ctx.cr[0].eq {
	pc = 0x82FDAA74; continue 'dispatch;
	}
	// 82FDAA44: 3BBD0003  addi r29, r29, 3
	ctx.r[29].s64 = ctx.r[29].s64 + 3;
	// 82FDAA48: 3BFF0006  addi r31, r31, 6
	ctx.r[31].s64 = ctx.r[31].s64 + 6;
	// 82FDAA4C: 3BDE0006  addi r30, r30, 6
	ctx.r[30].s64 = ctx.r[30].s64 + 6;
	// 82FDAA50: 48000010  b 0x82fdaa60
	pc = 0x82FDAA60; continue 'dispatch;
	// 82FDAA54: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82FDAA58: 3BFF0002  addi r31, r31, 2
	ctx.r[31].s64 = ctx.r[31].s64 + 2;
	// 82FDAA5C: 3BDE0002  addi r30, r30, 2
	ctx.r[30].s64 = ctx.r[30].s64 + 2;
	// 82FDAA60: 7F1DD800  cmpw cr6, r29, r27
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[27].s32, &mut ctx.xer);
	// 82FDAA64: 4198FF90  blt cr6, 0x82fda9f4
	if ctx.cr[6].lt {
	pc = 0x82FDA9F4; continue 'dispatch;
	}
	// 82FDAA68: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82FDAA6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FDAA70: 481CD744  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 82FDAA74: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FDAA78: 4BFFFFF4  b 0x82fdaa6c
	pc = 0x82FDAA6C; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDAA80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDAA80 size=192
    let mut pc: u32 = 0x82FDAA80;
    'dispatch: loop {
        match pc {
            0x82FDAA80 => {
    //   block [0x82FDAA80..0x82FDAB40)
	// 82FDAA80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDAA84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FDAA88: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FDAA8C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FDAA90: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDAA94: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82FDAA98: 409A000C  bne cr6, 0x82fdaaa4
	if !ctx.cr[6].eq {
	pc = 0x82FDAAA4; continue 'dispatch;
	}
	// 82FDAA9C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FDAAA0: 48000088  b 0x82fdab28
	pc = 0x82FDAB28; continue 'dispatch;
	// 82FDAAA4: A1630000  lhz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDAAA8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FDAAAC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDAAB0: 41820074  beq 0x82fdab24
	if ctx.cr[0].eq {
	pc = 0x82FDAB24; continue 'dispatch;
	}
	// 82FDAAB4: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FDAAB8: 3BCB9BE0  addi r30, r11, -0x6420
	ctx.r[30].s64 = ctx.r[11].s64 + -25632;
	// 82FDAABC: A07F0000  lhz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDAAC0: 4BFFF3F9  bl 0x82fd9eb8
	ctx.lr = 0x82FDAAC4;
	sub_82FD9EB8(ctx, base);
	// 82FDAAC4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDAAC8: 4082004C  bne 0x82fdab14
	if !ctx.cr[0].eq {
	pc = 0x82FDAB14; continue 'dispatch;
	}
	// 82FDAACC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FDAAD0: A09F0000  lhz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDAAD4: 4BFF72DD  bl 0x82fd1db0
	ctx.lr = 0x82FDAAD8;
	sub_82FD1DB0(ctx, base);
	// 82FDAAD8: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82FDAADC: 409A0038  bne cr6, 0x82fdab14
	if !ctx.cr[6].eq {
	pc = 0x82FDAB14; continue 'dispatch;
	}
	// 82FDAAE0: A17F0000  lhz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDAAE4: 2B0B0025  cmplwi cr6, r11, 0x25
	ctx.cr[6].compare_u32(ctx.r[11].u32, 37 as u32, &mut ctx.xer);
	// 82FDAAE8: 409AFFB4  bne cr6, 0x82fdaa9c
	if !ctx.cr[6].eq {
	pc = 0x82FDAA9C; continue 'dispatch;
	}
	// 82FDAAEC: A07F0002  lhz r3, 2(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 82FDAAF0: 4BFF6B21  bl 0x82fd1610
	ctx.lr = 0x82FDAAF4;
	sub_82FD1610(ctx, base);
	// 82FDAAF4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDAAF8: 4182FFA4  beq 0x82fdaa9c
	if ctx.cr[0].eq {
	pc = 0x82FDAA9C; continue 'dispatch;
	}
	// 82FDAAFC: A07F0004  lhz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDAB00: 4BFF6B11  bl 0x82fd1610
	ctx.lr = 0x82FDAB04;
	sub_82FD1610(ctx, base);
	// 82FDAB04: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDAB08: 4182FF94  beq 0x82fdaa9c
	if ctx.cr[0].eq {
	pc = 0x82FDAA9C; continue 'dispatch;
	}
	// 82FDAB0C: 3BFF0006  addi r31, r31, 6
	ctx.r[31].s64 = ctx.r[31].s64 + 6;
	// 82FDAB10: 48000008  b 0x82fdab18
	pc = 0x82FDAB18; continue 'dispatch;
	// 82FDAB14: 3BFF0002  addi r31, r31, 2
	ctx.r[31].s64 = ctx.r[31].s64 + 2;
	// 82FDAB18: A17F0000  lhz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDAB1C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDAB20: 4082FF9C  bne 0x82fdaabc
	if !ctx.cr[0].eq {
	pc = 0x82FDAABC; continue 'dispatch;
	}
	// 82FDAB24: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82FDAB28: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FDAB2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FDAB30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FDAB34: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FDAB38: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FDAB3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDAB40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDAB40 size=156
    let mut pc: u32 = 0x82FDAB40;
    'dispatch: loop {
        match pc {
            0x82FDAB40 => {
    //   block [0x82FDAB40..0x82FDABDC)
	// 82FDAB40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDAB44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FDAB48: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FDAB4C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDAB50: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82FDAB54: 419A0070  beq cr6, 0x82fdabc4
	if ctx.cr[6].eq {
	pc = 0x82FDABC4; continue 'dispatch;
	}
	// 82FDAB58: A1630000  lhz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDAB5C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDAB60: 41820064  beq 0x82fdabc4
	if ctx.cr[0].eq {
	pc = 0x82FDABC4; continue 'dispatch;
	}
	// 82FDAB64: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FDAB68: A07F0000  lhz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDAB6C: 4BFFF2F5  bl 0x82fd9e60
	ctx.lr = 0x82FDAB70;
	sub_82FD9E60(ctx, base);
	// 82FDAB70: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDAB74: 4182000C  beq 0x82fdab80
	if ctx.cr[0].eq {
	pc = 0x82FDAB80; continue 'dispatch;
	}
	// 82FDAB78: 3BFF0002  addi r31, r31, 2
	ctx.r[31].s64 = ctx.r[31].s64 + 2;
	// 82FDAB7C: 48000034  b 0x82fdabb0
	pc = 0x82FDABB0; continue 'dispatch;
	// 82FDAB80: A17F0000  lhz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDAB84: 2B0B0025  cmplwi cr6, r11, 0x25
	ctx.cr[6].compare_u32(ctx.r[11].u32, 37 as u32, &mut ctx.xer);
	// 82FDAB88: 409A003C  bne cr6, 0x82fdabc4
	if !ctx.cr[6].eq {
	pc = 0x82FDABC4; continue 'dispatch;
	}
	// 82FDAB8C: A07F0002  lhz r3, 2(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 82FDAB90: 4BFF6A81  bl 0x82fd1610
	ctx.lr = 0x82FDAB94;
	sub_82FD1610(ctx, base);
	// 82FDAB94: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDAB98: 4182002C  beq 0x82fdabc4
	if ctx.cr[0].eq {
	pc = 0x82FDABC4; continue 'dispatch;
	}
	// 82FDAB9C: A07F0004  lhz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDABA0: 4BFF6A71  bl 0x82fd1610
	ctx.lr = 0x82FDABA4;
	sub_82FD1610(ctx, base);
	// 82FDABA4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDABA8: 4182001C  beq 0x82fdabc4
	if ctx.cr[0].eq {
	pc = 0x82FDABC4; continue 'dispatch;
	}
	// 82FDABAC: 3BFF0006  addi r31, r31, 6
	ctx.r[31].s64 = ctx.r[31].s64 + 6;
	// 82FDABB0: A17F0000  lhz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDABB4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDABB8: 4082FFB0  bne 0x82fdab68
	if !ctx.cr[0].eq {
	pc = 0x82FDAB68; continue 'dispatch;
	}
	// 82FDABBC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82FDABC0: 48000008  b 0x82fdabc8
	pc = 0x82FDABC8; continue 'dispatch;
	// 82FDABC4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FDABC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FDABCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FDABD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FDABD4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FDABD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDABE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDABE0 size=228
    let mut pc: u32 = 0x82FDABE0;
    'dispatch: loop {
        match pc {
            0x82FDABE0 => {
    //   block [0x82FDABE0..0x82FDACC4)
	// 82FDABE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDABE4: 481CD581  bl 0x831a8164
	ctx.lr = 0x82FDABE8;
	sub_831A8130(ctx, base);
	// 82FDABE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDABEC: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82FDABF0: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82FDABF4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82FDABF8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82FDABFC: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82FDAC00: 409900A4  ble cr6, 0x82fdaca4
	if !ctx.cr[6].gt {
	pc = 0x82FDACA4; continue 'dispatch;
	}
	// 82FDAC04: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FDAC08: A07F0000  lhz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDAC0C: 2B03002E  cmplwi cr6, r3, 0x2e
	ctx.cr[6].compare_u32(ctx.r[3].u32, 46 as u32, &mut ctx.xer);
	// 82FDAC10: 409A0038  bne cr6, 0x82fdac48
	if !ctx.cr[6].eq {
	pc = 0x82FDAC48; continue 'dispatch;
	}
	// 82FDAC14: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FDAC18: 419A00A4  beq cr6, 0x82fdacbc
	if ctx.cr[6].eq {
	pc = 0x82FDACBC; continue 'dispatch;
	}
	// 82FDAC1C: 397E0001  addi r11, r30, 1
	ctx.r[11].s64 = ctx.r[30].s64 + 1;
	// 82FDAC20: 7F0BE000  cmpw cr6, r11, r28
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[28].s32, &mut ctx.xer);
	// 82FDAC24: 419A0098  beq cr6, 0x82fdacbc
	if ctx.cr[6].eq {
	pc = 0x82FDACBC; continue 'dispatch;
	}
	// 82FDAC28: A07F0002  lhz r3, 2(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 82FDAC2C: 4BFF695D  bl 0x82fd1588
	ctx.lr = 0x82FDAC30;
	sub_82FD1588(ctx, base);
	// 82FDAC30: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDAC34: 41820088  beq 0x82fdacbc
	if ctx.cr[0].eq {
	pc = 0x82FDACBC; continue 'dispatch;
	}
	// 82FDAC38: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 82FDAC3C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82FDAC40: 2F1B0003  cmpwi cr6, r27, 3
	ctx.cr[6].compare_i32(ctx.r[27].s32, 3, &mut ctx.xer);
	// 82FDAC44: 4800004C  b 0x82fdac90
	pc = 0x82FDAC90; continue 'dispatch;
	// 82FDAC48: 4BFF6941  bl 0x82fd1588
	ctx.lr = 0x82FDAC4C;
	sub_82FD1588(ctx, base);
	// 82FDAC4C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDAC50: 4182006C  beq 0x82fdacbc
	if ctx.cr[0].eq {
	pc = 0x82FDACBC; continue 'dispatch;
	}
	// 82FDAC54: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82FDAC58: 2F1D0003  cmpwi cr6, r29, 3
	ctx.cr[6].compare_i32(ctx.r[29].s32, 3, &mut ctx.xer);
	// 82FDAC5C: 41990060  bgt cr6, 0x82fdacbc
	if ctx.cr[6].gt {
	pc = 0x82FDACBC; continue 'dispatch;
	}
	// 82FDAC60: 409A0034  bne cr6, 0x82fdac94
	if !ctx.cr[6].eq {
	pc = 0x82FDAC94; continue 'dispatch;
	}
	// 82FDAC64: A17FFFFC  lhz r11, -4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82FDAC68: A15FFFFE  lhz r10, -2(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(-2 as u32) ) } as u64;
	// 82FDAC6C: 2B0B0032  cmplwi cr6, r11, 0x32
	ctx.cr[6].compare_u32(ctx.r[11].u32, 50 as u32, &mut ctx.xer);
	// 82FDAC70: 41980024  blt cr6, 0x82fdac94
	if ctx.cr[6].lt {
	pc = 0x82FDAC94; continue 'dispatch;
	}
	// 82FDAC74: 409A0048  bne cr6, 0x82fdacbc
	if !ctx.cr[6].eq {
	pc = 0x82FDACBC; continue 'dispatch;
	}
	// 82FDAC78: 554B043E  clrlwi r11, r10, 0x10
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 82FDAC7C: 2B0B0035  cmplwi cr6, r11, 0x35
	ctx.cr[6].compare_u32(ctx.r[11].u32, 53 as u32, &mut ctx.xer);
	// 82FDAC80: 41980014  blt cr6, 0x82fdac94
	if ctx.cr[6].lt {
	pc = 0x82FDAC94; continue 'dispatch;
	}
	// 82FDAC84: 409A0038  bne cr6, 0x82fdacbc
	if !ctx.cr[6].eq {
	pc = 0x82FDACBC; continue 'dispatch;
	}
	// 82FDAC88: A17F0000  lhz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDAC8C: 2B0B0035  cmplwi cr6, r11, 0x35
	ctx.cr[6].compare_u32(ctx.r[11].u32, 53 as u32, &mut ctx.xer);
	// 82FDAC90: 4199002C  bgt cr6, 0x82fdacbc
	if ctx.cr[6].gt {
	pc = 0x82FDACBC; continue 'dispatch;
	}
	// 82FDAC94: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82FDAC98: 3BFF0002  addi r31, r31, 2
	ctx.r[31].s64 = ctx.r[31].s64 + 2;
	// 82FDAC9C: 7F1EE000  cmpw cr6, r30, r28
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[28].s32, &mut ctx.xer);
	// 82FDACA0: 4198FF68  blt cr6, 0x82fdac08
	if ctx.cr[6].lt {
	pc = 0x82FDAC08; continue 'dispatch;
	}
	// 82FDACA4: 397BFFFD  addi r11, r27, -3
	ctx.r[11].s64 = ctx.r[27].s64 + -3;
	// 82FDACA8: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82FDACAC: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82FDACB0: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82FDACB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FDACB8: 481CD4FC  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 82FDACBC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FDACC0: 4BFFFFF4  b 0x82fdacb4
	pc = 0x82FDACB4; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDACC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDACC8 size=292
    let mut pc: u32 = 0x82FDACC8;
    'dispatch: loop {
        match pc {
            0x82FDACC8 => {
    //   block [0x82FDACC8..0x82FDADEC)
	// 82FDACC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDACCC: 481CD48D  bl 0x831a8158
	ctx.lr = 0x82FDACD0;
	sub_831A8130(ctx, base);
	// 82FDACD0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDACD4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82FDACD8: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 82FDACDC: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 82FDACE0: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82FDACE4: 7FF8FB78  mr r24, r31
	ctx.r[24].u64 = ctx.r[31].u64;
	// 82FDACE8: 7F1FD000  cmpw cr6, r31, r26
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[26].s32, &mut ctx.xer);
	// 82FDACEC: 409800AC  bge cr6, 0x82fdad98
	if !ctx.cr[6].lt {
	pc = 0x82FDAD98; continue 'dispatch;
	}
	// 82FDACF0: 57EB083C  slwi r11, r31, 1
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FDACF4: 3B7F0001  addi r27, r31, 1
	ctx.r[27].s64 = ctx.r[31].s64 + 1;
	// 82FDACF8: 7FAB1A14  add r29, r11, r3
	ctx.r[29].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82FDACFC: A3DD0000  lhz r30, 0(r29)
	ctx.r[30].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDAD00: 2B1E003A  cmplwi cr6, r30, 0x3a
	ctx.cr[6].compare_u32(ctx.r[30].u32, 58 as u32, &mut ctx.xer);
	// 82FDAD04: 409A0044  bne cr6, 0x82fdad48
	if !ctx.cr[6].eq {
	pc = 0x82FDAD48; continue 'dispatch;
	}
	// 82FDAD08: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82FDAD0C: 4099001C  ble cr6, 0x82fdad28
	if !ctx.cr[6].gt {
	pc = 0x82FDAD28; continue 'dispatch;
	}
	// 82FDAD10: 81790000  lwz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDAD14: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FDAD18: 2F0B0008  cmpwi cr6, r11, 8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 8, &mut ctx.xer);
	// 82FDAD1C: 91790000  stw r11, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FDAD20: 41990078  bgt cr6, 0x82fdad98
	if ctx.cr[6].gt {
	pc = 0x82FDAD98; continue 'dispatch;
	}
	// 82FDAD24: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82FDAD28: 419A007C  beq cr6, 0x82fdada4
	if ctx.cr[6].eq {
	pc = 0x82FDADA4; continue 'dispatch;
	}
	// 82FDAD2C: 7F1BD000  cmpw cr6, r27, r26
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[26].s32, &mut ctx.xer);
	// 82FDAD30: 40980010  bge cr6, 0x82fdad40
	if !ctx.cr[6].lt {
	pc = 0x82FDAD40; continue 'dispatch;
	}
	// 82FDAD34: A17D0002  lhz r11, 2(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(2 as u32) ) } as u64;
	// 82FDAD38: 2B0B003A  cmplwi cr6, r11, 0x3a
	ctx.cr[6].compare_u32(ctx.r[11].u32, 58 as u32, &mut ctx.xer);
	// 82FDAD3C: 419A0068  beq cr6, 0x82fdada4
	if ctx.cr[6].eq {
	pc = 0x82FDADA4; continue 'dispatch;
	}
	// 82FDAD40: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82FDAD44: 48000020  b 0x82fdad64
	pc = 0x82FDAD64; continue 'dispatch;
	// 82FDAD48: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FDAD4C: 4BFF68C5  bl 0x82fd1610
	ctx.lr = 0x82FDAD50;
	sub_82FD1610(ctx, base);
	// 82FDAD50: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDAD54: 41820058  beq 0x82fdadac
	if ctx.cr[0].eq {
	pc = 0x82FDADAC; continue 'dispatch;
	}
	// 82FDAD58: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82FDAD5C: 2F1C0004  cmpwi cr6, r28, 4
	ctx.cr[6].compare_i32(ctx.r[28].s32, 4, &mut ctx.xer);
	// 82FDAD60: 41990038  bgt cr6, 0x82fdad98
	if ctx.cr[6].gt {
	pc = 0x82FDAD98; continue 'dispatch;
	}
	// 82FDAD64: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82FDAD68: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 82FDAD6C: 3BBD0002  addi r29, r29, 2
	ctx.r[29].s64 = ctx.r[29].s64 + 2;
	// 82FDAD70: 7F1FD000  cmpw cr6, r31, r26
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[26].s32, &mut ctx.xer);
	// 82FDAD74: 4198FF88  blt cr6, 0x82fdacfc
	if ctx.cr[6].lt {
	pc = 0x82FDACFC; continue 'dispatch;
	}
	// 82FDAD78: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82FDAD7C: 4099001C  ble cr6, 0x82fdad98
	if !ctx.cr[6].gt {
	pc = 0x82FDAD98; continue 'dispatch;
	}
	// 82FDAD80: 81790000  lwz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDAD84: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82FDAD88: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FDAD8C: 2F0B0008  cmpwi cr6, r11, 8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 8, &mut ctx.xer);
	// 82FDAD90: 91790000  stw r11, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FDAD94: 40990008  ble cr6, 0x82fdad9c
	if !ctx.cr[6].gt {
	pc = 0x82FDAD9C; continue 'dispatch;
	}
	// 82FDAD98: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82FDAD9C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82FDADA0: 481CD408  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
	// 82FDADA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FDADA8: 4BFFFFF4  b 0x82fdad9c
	pc = 0x82FDAD9C; continue 'dispatch;
	// 82FDADAC: 57CB043E  clrlwi r11, r30, 0x10
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x0000FFFFu64;
	// 82FDADB0: 2B0B002E  cmplwi cr6, r11, 0x2e
	ctx.cr[6].compare_u32(ctx.r[11].u32, 46 as u32, &mut ctx.xer);
	// 82FDADB4: 409AFFE4  bne cr6, 0x82fdad98
	if !ctx.cr[6].eq {
	pc = 0x82FDAD98; continue 'dispatch;
	}
	// 82FDADB8: 397CFFFF  addi r11, r28, -1
	ctx.r[11].s64 = ctx.r[28].s64 + -1;
	// 82FDADBC: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 82FDADC0: 4199FFD8  bgt cr6, 0x82fdad98
	if ctx.cr[6].gt {
	pc = 0x82FDAD98; continue 'dispatch;
	}
	// 82FDADC4: 81790000  lwz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDADC8: 2F0B0006  cmpwi cr6, r11, 6
	ctx.cr[6].compare_i32(ctx.r[11].s32, 6, &mut ctx.xer);
	// 82FDADCC: 4199FFCC  bgt cr6, 0x82fdad98
	if ctx.cr[6].gt {
	pc = 0x82FDAD98; continue 'dispatch;
	}
	// 82FDADD0: 7D7CF850  subf r11, r28, r31
	ctx.r[11].s64 = ctx.r[31].s64 - ctx.r[28].s64;
	// 82FDADD4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82FDADD8: 7F0BC000  cmpw cr6, r11, r24
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[24].s32, &mut ctx.xer);
	// 82FDADDC: 40980008  bge cr6, 0x82fdade4
	if !ctx.cr[6].lt {
	pc = 0x82FDADE4; continue 'dispatch;
	}
	// 82FDADE0: 7F0BC378  mr r11, r24
	ctx.r[11].u64 = ctx.r[24].u64;
	// 82FDADE4: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82FDADE8: 4BFFFFB4  b 0x82fdad9c
	pc = 0x82FDAD9C; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDADF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDADF0 size=1364
    let mut pc: u32 = 0x82FDADF0;
    'dispatch: loop {
        match pc {
            0x82FDADF0 => {
    //   block [0x82FDADF0..0x82FDB344)
	// 82FDADF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDADF4: 481CD365  bl 0x831a8158
	ctx.lr = 0x82FDADF8;
	sub_831A8130(ctx, base);
	// 82FDADF8: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDADFC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FDAE00: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FDAE04: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDAE08: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 82FDAE0C: 40820008  bne 0x82fdae14
	if !ctx.cr[0].eq {
	pc = 0x82FDAE14; continue 'dispatch;
	}
	// 82FDAE10: 813E0014  lwz r9, 0x14(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FDAE14: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDAE18: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 82FDAE1C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDAE20: 41820034  beq 0x82fdae54
	if ctx.cr[0].eq {
	pc = 0x82FDAE54; continue 'dispatch;
	}
	// 82FDAE24: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDAE28: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDAE2C: 41820028  beq 0x82fdae54
	if ctx.cr[0].eq {
	pc = 0x82FDAE54; continue 'dispatch;
	}
	// 82FDAE30: 394B0002  addi r10, r11, 2
	ctx.r[10].s64 = ctx.r[11].s64 + 2;
	// 82FDAE34: 48000008  b 0x82fdae3c
	pc = 0x82FDAE3C; continue 'dispatch;
	// 82FDAE38: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 82FDAE3C: A10A0000  lhz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDAE40: 28080000  cmplwi r8, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDAE44: 4082FFF4  bne 0x82fdae38
	if !ctx.cr[0].eq {
	pc = 0x82FDAE38; continue 'dispatch;
	}
	// 82FDAE48: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82FDAE4C: 7D790E70  srawi r25, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[25].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82FDAE50: 48000008  b 0x82fdae58
	pc = 0x82FDAE58; continue 'dispatch;
	// 82FDAE54: 7F19C378  mr r25, r24
	ctx.r[25].u64 = ctx.r[24].u64;
	// 82FDAE58: 815E0020  lwz r10, 0x20(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FDAE5C: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDAE60: 41820034  beq 0x82fdae94
	if ctx.cr[0].eq {
	pc = 0x82FDAE94; continue 'dispatch;
	}
	// 82FDAE64: A16A0000  lhz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDAE68: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDAE6C: 41820028  beq 0x82fdae94
	if ctx.cr[0].eq {
	pc = 0x82FDAE94; continue 'dispatch;
	}
	// 82FDAE70: 396A0002  addi r11, r10, 2
	ctx.r[11].s64 = ctx.r[10].s64 + 2;
	// 82FDAE74: 48000008  b 0x82fdae7c
	pc = 0x82FDAE7C; continue 'dispatch;
	// 82FDAE78: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FDAE7C: A10B0000  lhz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDAE80: 28080000  cmplwi r8, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDAE84: 4082FFF4  bne 0x82fdae78
	if !ctx.cr[0].eq {
	pc = 0x82FDAE78; continue 'dispatch;
	}
	// 82FDAE88: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82FDAE8C: 7D7A0E70  srawi r26, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[26].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82FDAE90: 48000008  b 0x82fdae98
	pc = 0x82FDAE98; continue 'dispatch;
	// 82FDAE94: 7F1AC378  mr r26, r24
	ctx.r[26].u64 = ctx.r[24].u64;
	// 82FDAE98: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82FDAE9C: 419A0034  beq cr6, 0x82fdaed0
	if ctx.cr[6].eq {
	pc = 0x82FDAED0; continue 'dispatch;
	}
	// 82FDAEA0: A1690000  lhz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDAEA4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDAEA8: 41820028  beq 0x82fdaed0
	if ctx.cr[0].eq {
	pc = 0x82FDAED0; continue 'dispatch;
	}
	// 82FDAEAC: 39690002  addi r11, r9, 2
	ctx.r[11].s64 = ctx.r[9].s64 + 2;
	// 82FDAEB0: 48000008  b 0x82fdaeb8
	pc = 0x82FDAEB8; continue 'dispatch;
	// 82FDAEB4: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FDAEB8: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDAEBC: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDAEC0: 4082FFF4  bne 0x82fdaeb4
	if !ctx.cr[0].eq {
	pc = 0x82FDAEB4; continue 'dispatch;
	}
	// 82FDAEC4: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 82FDAEC8: 7D7B0E70  srawi r27, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[27].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82FDAECC: 48000008  b 0x82fdaed4
	pc = 0x82FDAED4; continue 'dispatch;
	// 82FDAED0: 7F1BC378  mr r27, r24
	ctx.r[27].u64 = ctx.r[24].u64;
	// 82FDAED4: 815E0018  lwz r10, 0x18(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FDAED8: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDAEDC: 41820034  beq 0x82fdaf10
	if ctx.cr[0].eq {
	pc = 0x82FDAF10; continue 'dispatch;
	}
	// 82FDAEE0: A16A0000  lhz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDAEE4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDAEE8: 41820028  beq 0x82fdaf10
	if ctx.cr[0].eq {
	pc = 0x82FDAF10; continue 'dispatch;
	}
	// 82FDAEEC: 396A0002  addi r11, r10, 2
	ctx.r[11].s64 = ctx.r[10].s64 + 2;
	// 82FDAEF0: 48000008  b 0x82fdaef8
	pc = 0x82FDAEF8; continue 'dispatch;
	// 82FDAEF4: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FDAEF8: A12B0000  lhz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDAEFC: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDAF00: 4082FFF4  bne 0x82fdaef4
	if !ctx.cr[0].eq {
	pc = 0x82FDAEF4; continue 'dispatch;
	}
	// 82FDAF04: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82FDAF08: 7D7C0E70  srawi r28, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[28].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82FDAF0C: 48000008  b 0x82fdaf14
	pc = 0x82FDAF14; continue 'dispatch;
	// 82FDAF10: 7F1CC378  mr r28, r24
	ctx.r[28].u64 = ctx.r[24].u64;
	// 82FDAF14: 815E001C  lwz r10, 0x1c(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FDAF18: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDAF1C: 41820034  beq 0x82fdaf50
	if ctx.cr[0].eq {
	pc = 0x82FDAF50; continue 'dispatch;
	}
	// 82FDAF20: A16A0000  lhz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDAF24: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDAF28: 41820028  beq 0x82fdaf50
	if ctx.cr[0].eq {
	pc = 0x82FDAF50; continue 'dispatch;
	}
	// 82FDAF2C: 396A0002  addi r11, r10, 2
	ctx.r[11].s64 = ctx.r[10].s64 + 2;
	// 82FDAF30: 48000008  b 0x82fdaf38
	pc = 0x82FDAF38; continue 'dispatch;
	// 82FDAF34: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FDAF38: A12B0000  lhz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDAF3C: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDAF40: 4082FFF4  bne 0x82fdaf34
	if !ctx.cr[0].eq {
	pc = 0x82FDAF34; continue 'dispatch;
	}
	// 82FDAF44: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82FDAF48: 7D7D0E70  srawi r29, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[29].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82FDAF4C: 48000008  b 0x82fdaf54
	pc = 0x82FDAF54; continue 'dispatch;
	// 82FDAF50: 7F1DC378  mr r29, r24
	ctx.r[29].u64 = ctx.r[24].u64;
	// 82FDAF54: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FDAF58: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDAF5C: 41820034  beq 0x82fdaf90
	if ctx.cr[0].eq {
	pc = 0x82FDAF90; continue 'dispatch;
	}
	// 82FDAF60: A16A0000  lhz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDAF64: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDAF68: 41820028  beq 0x82fdaf90
	if ctx.cr[0].eq {
	pc = 0x82FDAF90; continue 'dispatch;
	}
	// 82FDAF6C: 396A0002  addi r11, r10, 2
	ctx.r[11].s64 = ctx.r[10].s64 + 2;
	// 82FDAF70: 48000008  b 0x82fdaf78
	pc = 0x82FDAF78; continue 'dispatch;
	// 82FDAF74: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FDAF78: A12B0000  lhz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDAF7C: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDAF80: 4082FFF4  bne 0x82fdaf74
	if !ctx.cr[0].eq {
	pc = 0x82FDAF74; continue 'dispatch;
	}
	// 82FDAF84: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82FDAF88: 7D7F0E70  srawi r31, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[31].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82FDAF8C: 48000008  b 0x82fdaf94
	pc = 0x82FDAF94; continue 'dispatch;
	// 82FDAF90: 7F1FC378  mr r31, r24
	ctx.r[31].u64 = ctx.r[24].u64;
	// 82FDAF94: 807E0028  lwz r3, 0x28(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDAF98: 809E0024  lwz r4, 0x24(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FDAF9C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDAFA0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FDAFA4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDAFA8: 4E800421  bctrl
	ctx.lr = 0x82FDAFAC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDAFAC: 7D7FEA14  add r11, r31, r29
	ctx.r[11].u64 = ctx.r[31].u64 + ctx.r[29].u64;
	// 82FDAFB0: 807E0028  lwz r3, 0x28(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDAFB4: 7D6BE214  add r11, r11, r28
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 82FDAFB8: 7D6BDA14  add r11, r11, r27
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 82FDAFBC: 7D6BD214  add r11, r11, r26
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[26].u64;
	// 82FDAFC0: 7D6BCA14  add r11, r11, r25
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[25].u64;
	// 82FDAFC4: 396B0026  addi r11, r11, 0x26
	ctx.r[11].s64 = ctx.r[11].s64 + 38;
	// 82FDAFC8: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82FDAFCC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDAFD0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDAFD4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDAFD8: 4E800421  bctrl
	ctx.lr = 0x82FDAFDC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDAFDC: 907E0024  stw r3, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[3].u32 ) };
	// 82FDAFE0: B3030000  sth r24, 0(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[24].u16 ) };
	// 82FDAFE4: 3BA0003A  li r29, 0x3a
	ctx.r[29].s64 = 58;
	// 82FDAFE8: 807E0024  lwz r3, 0x24(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FDAFEC: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDAFF0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FDAFF4: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDAFF8: 41820058  beq 0x82fdb050
	if ctx.cr[0].eq {
	pc = 0x82FDB050; continue 'dispatch;
	}
	// 82FDAFFC: 4BFF689D  bl 0x82fd1898
	ctx.lr = 0x82FDB000;
	sub_82FD1898(ctx, base);
	// 82FDB000: 817E0024  lwz r11, 0x24(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FDB004: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDB008: 41820034  beq 0x82fdb03c
	if ctx.cr[0].eq {
	pc = 0x82FDB03C; continue 'dispatch;
	}
	// 82FDB00C: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDB010: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDB014: 41820028  beq 0x82fdb03c
	if ctx.cr[0].eq {
	pc = 0x82FDB03C; continue 'dispatch;
	}
	// 82FDB018: 394B0002  addi r10, r11, 2
	ctx.r[10].s64 = ctx.r[11].s64 + 2;
	// 82FDB01C: 48000008  b 0x82fdb024
	pc = 0x82FDB024; continue 'dispatch;
	// 82FDB020: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 82FDB024: A12A0000  lhz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDB028: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDB02C: 4082FFF4  bne 0x82fdb020
	if !ctx.cr[0].eq {
	pc = 0x82FDB020; continue 'dispatch;
	}
	// 82FDB030: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82FDB034: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82FDB038: 48000008  b 0x82fdb040
	pc = 0x82FDB040; continue 'dispatch;
	// 82FDB03C: 7F0BC378  mr r11, r24
	ctx.r[11].u64 = ctx.r[24].u64;
	// 82FDB040: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FDB044: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82FDB048: 3BEB0002  addi r31, r11, 2
	ctx.r[31].s64 = ctx.r[11].s64 + 2;
	// 82FDB04C: B3AB0000  sth r29, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u16 ) };
	// 82FDB050: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FDB054: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FDB058: 409A0010  bne cr6, 0x82fdb068
	if !ctx.cr[6].eq {
	pc = 0x82FDB068; continue 'dispatch;
	}
	// 82FDB05C: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FDB060: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FDB064: 419A01A0  beq cr6, 0x82fdb204
	if ctx.cr[6].eq {
	pc = 0x82FDB204; continue 'dispatch;
	}
	// 82FDB068: 3940002F  li r10, 0x2f
	ctx.r[10].s64 = 47;
	// 82FDB06C: 397F0002  addi r11, r31, 2
	ctx.r[11].s64 = ctx.r[31].s64 + 2;
	// 82FDB070: B15F0000  sth r10, 0(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 82FDB074: 3BEB0002  addi r31, r11, 2
	ctx.r[31].s64 = ctx.r[11].s64 + 2;
	// 82FDB078: B14B0000  sth r10, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 82FDB07C: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FDB080: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FDB084: 419A012C  beq cr6, 0x82fdb1b0
	if ctx.cr[6].eq {
	pc = 0x82FDB1B0; continue 'dispatch;
	}
	// 82FDB088: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FDB08C: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDB090: 41820060  beq 0x82fdb0f0
	if ctx.cr[0].eq {
	pc = 0x82FDB0F0; continue 'dispatch;
	}
	// 82FDB094: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FDB098: 4BFF6AD1  bl 0x82fd1b68
	ctx.lr = 0x82FDB09C;
	sub_82FD1B68(ctx, base);
	// 82FDB09C: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FDB0A0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDB0A4: 41820034  beq 0x82fdb0d8
	if ctx.cr[0].eq {
	pc = 0x82FDB0D8; continue 'dispatch;
	}
	// 82FDB0A8: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDB0AC: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDB0B0: 41820028  beq 0x82fdb0d8
	if ctx.cr[0].eq {
	pc = 0x82FDB0D8; continue 'dispatch;
	}
	// 82FDB0B4: 394B0002  addi r10, r11, 2
	ctx.r[10].s64 = ctx.r[11].s64 + 2;
	// 82FDB0B8: 48000008  b 0x82fdb0c0
	pc = 0x82FDB0C0; continue 'dispatch;
	// 82FDB0BC: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 82FDB0C0: A12A0000  lhz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDB0C4: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDB0C8: 4082FFF4  bne 0x82fdb0bc
	if !ctx.cr[0].eq {
	pc = 0x82FDB0BC; continue 'dispatch;
	}
	// 82FDB0CC: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82FDB0D0: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82FDB0D4: 48000008  b 0x82fdb0dc
	pc = 0x82FDB0DC; continue 'dispatch;
	// 82FDB0D8: 7F0BC378  mr r11, r24
	ctx.r[11].u64 = ctx.r[24].u64;
	// 82FDB0DC: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FDB0E0: 39400040  li r10, 0x40
	ctx.r[10].s64 = 64;
	// 82FDB0E4: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82FDB0E8: 3BEB0002  addi r31, r11, 2
	ctx.r[31].s64 = ctx.r[11].s64 + 2;
	// 82FDB0EC: B14B0000  sth r10, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 82FDB0F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FDB0F4: 809E000C  lwz r4, 0xc(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FDB0F8: 4BFF6A71  bl 0x82fd1b68
	ctx.lr = 0x82FDB0FC;
	sub_82FD1B68(ctx, base);
	// 82FDB0FC: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FDB100: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDB104: 41820034  beq 0x82fdb138
	if ctx.cr[0].eq {
	pc = 0x82FDB138; continue 'dispatch;
	}
	// 82FDB108: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDB10C: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDB110: 41820028  beq 0x82fdb138
	if ctx.cr[0].eq {
	pc = 0x82FDB138; continue 'dispatch;
	}
	// 82FDB114: 394B0002  addi r10, r11, 2
	ctx.r[10].s64 = ctx.r[11].s64 + 2;
	// 82FDB118: 48000008  b 0x82fdb120
	pc = 0x82FDB120; continue 'dispatch;
	// 82FDB11C: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 82FDB120: A12A0000  lhz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDB124: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDB128: 4082FFF4  bne 0x82fdb11c
	if !ctx.cr[0].eq {
	pc = 0x82FDB11C; continue 'dispatch;
	}
	// 82FDB12C: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82FDB130: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82FDB134: 48000008  b 0x82fdb13c
	pc = 0x82FDB13C; continue 'dispatch;
	// 82FDB138: 7F0BC378  mr r11, r24
	ctx.r[11].u64 = ctx.r[24].u64;
	// 82FDB13C: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FDB140: 7FEBFA14  add r31, r11, r31
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82FDB144: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FDB148: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 82FDB14C: 419A00B8  beq cr6, 0x82fdb204
	if ctx.cr[6].eq {
	pc = 0x82FDB204; continue 'dispatch;
	}
	// 82FDB150: 38C0000A  li r6, 0xa
	ctx.r[6].s64 = 10;
	// 82FDB154: B3BF0000  sth r29, 0(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[29].u16 ) };
	// 82FDB158: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 82FDB15C: 80FE0028  lwz r7, 0x28(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDB160: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82FDB164: 807E0010  lwz r3, 0x10(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FDB168: 3BFF0002  addi r31, r31, 2
	ctx.r[31].s64 = ctx.r[31].s64 + 2;
	// 82FDB16C: 4BFF6705  bl 0x82fd1870
	ctx.lr = 0x82FDB170;
	sub_82FD1870(ctx, base);
	// 82FDB170: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82FDB174: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FDB178: 4BFF69F1  bl 0x82fd1b68
	ctx.lr = 0x82FDB17C;
	sub_82FD1B68(ctx, base);
	// 82FDB17C: A1610050  lhz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FDB180: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDB184: 41820074  beq 0x82fdb1f8
	if ctx.cr[0].eq {
	pc = 0x82FDB1F8; continue 'dispatch;
	}
	// 82FDB188: A1410052  lhz r10, 0x52(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(82 as u32) ) } as u64;
	// 82FDB18C: 39610052  addi r11, r1, 0x52
	ctx.r[11].s64 = ctx.r[1].s64 + 82;
	// 82FDB190: 4800000C  b 0x82fdb19c
	pc = 0x82FDB19C; continue 'dispatch;
	// 82FDB194: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FDB198: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDB19C: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDB1A0: 4082FFF4  bne 0x82fdb194
	if !ctx.cr[0].eq {
	pc = 0x82FDB194; continue 'dispatch;
	}
	// 82FDB1A4: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82FDB1A8: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82FDB1AC: 48000044  b 0x82fdb1f0
	pc = 0x82FDB1F0; continue 'dispatch;
	// 82FDB1B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FDB1B4: 809E0014  lwz r4, 0x14(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FDB1B8: 4BFF69B1  bl 0x82fd1b68
	ctx.lr = 0x82FDB1BC;
	sub_82FD1B68(ctx, base);
	// 82FDB1BC: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FDB1C0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDB1C4: 41820034  beq 0x82fdb1f8
	if ctx.cr[0].eq {
	pc = 0x82FDB1F8; continue 'dispatch;
	}
	// 82FDB1C8: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDB1CC: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDB1D0: 41820028  beq 0x82fdb1f8
	if ctx.cr[0].eq {
	pc = 0x82FDB1F8; continue 'dispatch;
	}
	// 82FDB1D4: 394B0002  addi r10, r11, 2
	ctx.r[10].s64 = ctx.r[11].s64 + 2;
	// 82FDB1D8: 48000008  b 0x82fdb1e0
	pc = 0x82FDB1E0; continue 'dispatch;
	// 82FDB1DC: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 82FDB1E0: A12A0000  lhz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDB1E4: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDB1E8: 4082FFF4  bne 0x82fdb1dc
	if !ctx.cr[0].eq {
	pc = 0x82FDB1DC; continue 'dispatch;
	}
	// 82FDB1EC: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82FDB1F0: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82FDB1F4: 48000008  b 0x82fdb1fc
	pc = 0x82FDB1FC; continue 'dispatch;
	// 82FDB1F8: 7F0BC378  mr r11, r24
	ctx.r[11].u64 = ctx.r[24].u64;
	// 82FDB1FC: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FDB200: 7FEBFA14  add r31, r11, r31
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82FDB204: 809E0018  lwz r4, 0x18(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FDB208: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDB20C: 41820054  beq 0x82fdb260
	if ctx.cr[0].eq {
	pc = 0x82FDB260; continue 'dispatch;
	}
	// 82FDB210: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FDB214: 4BFF6955  bl 0x82fd1b68
	ctx.lr = 0x82FDB218;
	sub_82FD1B68(ctx, base);
	// 82FDB218: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FDB21C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDB220: 41820034  beq 0x82fdb254
	if ctx.cr[0].eq {
	pc = 0x82FDB254; continue 'dispatch;
	}
	// 82FDB224: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDB228: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDB22C: 41820028  beq 0x82fdb254
	if ctx.cr[0].eq {
	pc = 0x82FDB254; continue 'dispatch;
	}
	// 82FDB230: 394B0002  addi r10, r11, 2
	ctx.r[10].s64 = ctx.r[11].s64 + 2;
	// 82FDB234: 48000008  b 0x82fdb23c
	pc = 0x82FDB23C; continue 'dispatch;
	// 82FDB238: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 82FDB23C: A12A0000  lhz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDB240: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDB244: 4082FFF4  bne 0x82fdb238
	if !ctx.cr[0].eq {
	pc = 0x82FDB238; continue 'dispatch;
	}
	// 82FDB248: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82FDB24C: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82FDB250: 48000008  b 0x82fdb258
	pc = 0x82FDB258; continue 'dispatch;
	// 82FDB254: 7F0BC378  mr r11, r24
	ctx.r[11].u64 = ctx.r[24].u64;
	// 82FDB258: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FDB25C: 7FEBFA14  add r31, r11, r31
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82FDB260: 817E001C  lwz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FDB264: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FDB268: 419A0064  beq cr6, 0x82fdb2cc
	if ctx.cr[6].eq {
	pc = 0x82FDB2CC; continue 'dispatch;
	}
	// 82FDB26C: 3960003F  li r11, 0x3f
	ctx.r[11].s64 = 63;
	// 82FDB270: B17F0000  sth r11, 0(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82FDB274: 3BFF0002  addi r31, r31, 2
	ctx.r[31].s64 = ctx.r[31].s64 + 2;
	// 82FDB278: 809E001C  lwz r4, 0x1c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FDB27C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FDB280: 4BFF68E9  bl 0x82fd1b68
	ctx.lr = 0x82FDB284;
	sub_82FD1B68(ctx, base);
	// 82FDB284: 817E001C  lwz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FDB288: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDB28C: 41820034  beq 0x82fdb2c0
	if ctx.cr[0].eq {
	pc = 0x82FDB2C0; continue 'dispatch;
	}
	// 82FDB290: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDB294: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDB298: 41820028  beq 0x82fdb2c0
	if ctx.cr[0].eq {
	pc = 0x82FDB2C0; continue 'dispatch;
	}
	// 82FDB29C: 394B0002  addi r10, r11, 2
	ctx.r[10].s64 = ctx.r[11].s64 + 2;
	// 82FDB2A0: 48000008  b 0x82fdb2a8
	pc = 0x82FDB2A8; continue 'dispatch;
	// 82FDB2A4: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 82FDB2A8: A12A0000  lhz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDB2AC: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDB2B0: 4082FFF4  bne 0x82fdb2a4
	if !ctx.cr[0].eq {
	pc = 0x82FDB2A4; continue 'dispatch;
	}
	// 82FDB2B4: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82FDB2B8: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82FDB2BC: 48000008  b 0x82fdb2c4
	pc = 0x82FDB2C4; continue 'dispatch;
	// 82FDB2C0: 7F0BC378  mr r11, r24
	ctx.r[11].u64 = ctx.r[24].u64;
	// 82FDB2C4: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FDB2C8: 7FEBFA14  add r31, r11, r31
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82FDB2CC: 817E0020  lwz r11, 0x20(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FDB2D0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FDB2D4: 419A0064  beq cr6, 0x82fdb338
	if ctx.cr[6].eq {
	pc = 0x82FDB338; continue 'dispatch;
	}
	// 82FDB2D8: 39600023  li r11, 0x23
	ctx.r[11].s64 = 35;
	// 82FDB2DC: B17F0000  sth r11, 0(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82FDB2E0: 3BFF0002  addi r31, r31, 2
	ctx.r[31].s64 = ctx.r[31].s64 + 2;
	// 82FDB2E4: 809E0020  lwz r4, 0x20(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FDB2E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FDB2EC: 4BFF687D  bl 0x82fd1b68
	ctx.lr = 0x82FDB2F0;
	sub_82FD1B68(ctx, base);
	// 82FDB2F0: 817E0020  lwz r11, 0x20(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FDB2F4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDB2F8: 41820034  beq 0x82fdb32c
	if ctx.cr[0].eq {
	pc = 0x82FDB32C; continue 'dispatch;
	}
	// 82FDB2FC: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDB300: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDB304: 41820028  beq 0x82fdb32c
	if ctx.cr[0].eq {
	pc = 0x82FDB32C; continue 'dispatch;
	}
	// 82FDB308: 394B0002  addi r10, r11, 2
	ctx.r[10].s64 = ctx.r[11].s64 + 2;
	// 82FDB30C: 48000008  b 0x82fdb314
	pc = 0x82FDB314; continue 'dispatch;
	// 82FDB310: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 82FDB314: A12A0000  lhz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDB318: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDB31C: 4082FFF4  bne 0x82fdb310
	if !ctx.cr[0].eq {
	pc = 0x82FDB310; continue 'dispatch;
	}
	// 82FDB320: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82FDB324: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82FDB328: 48000008  b 0x82fdb330
	pc = 0x82FDB330; continue 'dispatch;
	// 82FDB32C: 7F0BC378  mr r11, r24
	ctx.r[11].u64 = ctx.r[24].u64;
	// 82FDB330: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FDB334: 7FEBFA14  add r31, r11, r31
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82FDB338: B31F0000  sth r24, 0(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[24].u16 ) };
	// 82FDB33C: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82FDB340: 481CCE68  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDB348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDB348 size=132
    let mut pc: u32 = 0x82FDB348;
    'dispatch: loop {
        match pc {
            0x82FDB348 => {
    //   block [0x82FDB348..0x82FDB3CC)
	// 82FDB348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDB34C: 481CCE1D  bl 0x831a8168
	ctx.lr = 0x82FDB350;
	sub_831A8130(ctx, base);
	// 82FDB350: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDB354: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FDB358: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82FDB35C: A07F0000  lhz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDB360: 4BFF61F1  bl 0x82fd1550
	ctx.lr = 0x82FDB364;
	sub_82FD1550(ctx, base);
	// 82FDB364: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDB368: 4082000C  bne 0x82fdb374
	if !ctx.cr[0].eq {
	pc = 0x82FDB374; continue 'dispatch;
	}
	// 82FDB36C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FDB370: 48000054  b 0x82fdb3c4
	pc = 0x82FDB3C4; continue 'dispatch;
	// 82FDB374: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82FDB378: 2F1C0001  cmpwi cr6, r28, 1
	ctx.cr[6].compare_i32(ctx.r[28].s32, 1, &mut ctx.xer);
	// 82FDB37C: 40990044  ble cr6, 0x82fdb3c0
	if !ctx.cr[6].gt {
	pc = 0x82FDB3C0; continue 'dispatch;
	}
	// 82FDB380: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FDB384: 3BFF0002  addi r31, r31, 2
	ctx.r[31].s64 = ctx.r[31].s64 + 2;
	// 82FDB388: 3BAB9BC8  addi r29, r11, -0x6438
	ctx.r[29].s64 = ctx.r[11].s64 + -25656;
	// 82FDB38C: A07F0000  lhz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDB390: 4BFF6219  bl 0x82fd15a8
	ctx.lr = 0x82FDB394;
	sub_82FD15A8(ctx, base);
	// 82FDB394: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDB398: 40820018  bne 0x82fdb3b0
	if !ctx.cr[0].eq {
	pc = 0x82FDB3B0; continue 'dispatch;
	}
	// 82FDB39C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FDB3A0: A09F0000  lhz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDB3A4: 4BFF6A0D  bl 0x82fd1db0
	ctx.lr = 0x82FDB3A8;
	sub_82FD1DB0(ctx, base);
	// 82FDB3A8: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82FDB3AC: 419AFFC0  beq cr6, 0x82fdb36c
	if ctx.cr[6].eq {
	pc = 0x82FDB36C; continue 'dispatch;
	}
	// 82FDB3B0: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82FDB3B4: 3BFF0002  addi r31, r31, 2
	ctx.r[31].s64 = ctx.r[31].s64 + 2;
	// 82FDB3B8: 7F1EE000  cmpw cr6, r30, r28
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[28].s32, &mut ctx.xer);
	// 82FDB3BC: 4198FFD0  blt cr6, 0x82fdb38c
	if ctx.cr[6].lt {
	pc = 0x82FDB38C; continue 'dispatch;
	}
	// 82FDB3C0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82FDB3C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FDB3C8: 481CCDF0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDB3D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDB3D0 size=492
    let mut pc: u32 = 0x82FDB3D0;
    'dispatch: loop {
        match pc {
            0x82FDB3D0 => {
    //   block [0x82FDB3D0..0x82FDB5BC)
	// 82FDB3D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDB3D4: 481CCD81  bl 0x831a8154
	ctx.lr = 0x82FDB3D8;
	sub_831A8130(ctx, base);
	// 82FDB3D8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDB3DC: 7C972378  mr r23, r4
	ctx.r[23].u64 = ctx.r[4].u64;
	// 82FDB3E0: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82FDB3E4: 2F170000  cmpwi cr6, r23, 0
	ctx.cr[6].compare_i32(ctx.r[23].s32, 0, &mut ctx.xer);
	// 82FDB3E8: 419A01C8  beq cr6, 0x82fdb5b0
	if ctx.cr[6].eq {
	pc = 0x82FDB5B0; continue 'dispatch;
	}
	// 82FDB3EC: 54AB063F  clrlwi. r11, r5, 0x18
	ctx.r[11].u64 = ctx.r[5].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDB3F0: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82FDB3F4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82FDB3F8: 41820014  beq 0x82fdb40c
	if ctx.cr[0].eq {
	pc = 0x82FDB40C; continue 'dispatch;
	}
	// 82FDB3FC: A1780000  lhz r11, 0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDB400: 2B0B002F  cmplwi cr6, r11, 0x2f
	ctx.cr[6].compare_u32(ctx.r[11].u32, 47 as u32, &mut ctx.xer);
	// 82FDB404: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FDB408: 409A0008  bne cr6, 0x82fdb410
	if !ctx.cr[6].eq {
	pc = 0x82FDB410; continue 'dispatch;
	}
	// 82FDB40C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82FDB410: 5579063E  clrlwi r25, r11, 0x18
	ctx.r[25].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82FDB414: 2F170000  cmpwi cr6, r23, 0
	ctx.cr[6].compare_i32(ctx.r[23].s32, 0, &mut ctx.xer);
	// 82FDB418: 409900DC  ble cr6, 0x82fdb4f4
	if !ctx.cr[6].gt {
	pc = 0x82FDB4F4; continue 'dispatch;
	}
	// 82FDB41C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FDB420: 3BB80004  addi r29, r24, 4
	ctx.r[29].s64 = ctx.r[24].s64 + 4;
	// 82FDB424: 3B8B9B6C  addi r28, r11, -0x6494
	ctx.r[28].s64 = ctx.r[11].s64 + -25748;
	// 82FDB428: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FDB42C: 3B6B9BF4  addi r27, r11, -0x640c
	ctx.r[27].s64 = ctx.r[11].s64 + -25612;
	// 82FDB430: A3DDFFFC  lhz r30, -4(r29)
	ctx.r[30].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82FDB434: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82FDB438: 2B0B003F  cmplwi cr6, r11, 0x3f
	ctx.cr[6].compare_u32(ctx.r[11].u32, 63 as u32, &mut ctx.xer);
	// 82FDB43C: 419A00B8  beq cr6, 0x82fdb4f4
	if ctx.cr[6].eq {
	pc = 0x82FDB4F4; continue 'dispatch;
	}
	// 82FDB440: 2B0B0023  cmplwi cr6, r11, 0x23
	ctx.cr[6].compare_u32(ctx.r[11].u32, 35 as u32, &mut ctx.xer);
	// 82FDB444: 419A00B0  beq cr6, 0x82fdb4f4
	if ctx.cr[6].eq {
	pc = 0x82FDB4F4; continue 'dispatch;
	}
	// 82FDB448: 2B0B0025  cmplwi cr6, r11, 0x25
	ctx.cr[6].compare_u32(ctx.r[11].u32, 37 as u32, &mut ctx.xer);
	// 82FDB44C: 409A0030  bne cr6, 0x82fdb47c
	if !ctx.cr[6].eq {
	pc = 0x82FDB47C; continue 'dispatch;
	}
	// 82FDB450: 397A0002  addi r11, r26, 2
	ctx.r[11].s64 = ctx.r[26].s64 + 2;
	// 82FDB454: 7F0BB800  cmpw cr6, r11, r23
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[23].s32, &mut ctx.xer);
	// 82FDB458: 40980100  bge cr6, 0x82fdb558
	if !ctx.cr[6].lt {
	pc = 0x82FDB558; continue 'dispatch;
	}
	// 82FDB45C: A07DFFFE  lhz r3, -2(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(-2 as u32) ) } as u64;
	// 82FDB460: 4BFF61B1  bl 0x82fd1610
	ctx.lr = 0x82FDB464;
	sub_82FD1610(ctx, base);
	// 82FDB464: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDB468: 418200F0  beq 0x82fdb558
	if ctx.cr[0].eq {
	pc = 0x82FDB558; continue 'dispatch;
	}
	// 82FDB46C: A07D0000  lhz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDB470: 4BFF61A1  bl 0x82fd1610
	ctx.lr = 0x82FDB474;
	sub_82FD1610(ctx, base);
	// 82FDB474: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDB478: 48000068  b 0x82fdb4e0
	pc = 0x82FDB4E0; continue 'dispatch;
	// 82FDB47C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FDB480: 4BFFEA39  bl 0x82fd9eb8
	ctx.lr = 0x82FDB484;
	sub_82FD9EB8(ctx, base);
	// 82FDB484: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDB488: 4082005C  bne 0x82fdb4e4
	if !ctx.cr[0].eq {
	pc = 0x82FDB4E4; continue 'dispatch;
	}
	// 82FDB48C: 573F063F  clrlwi. r31, r25, 0x18
	ctx.r[31].u64 = ctx.r[25].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82FDB490: 41820028  beq 0x82fdb4b8
	if ctx.cr[0].eq {
	pc = 0x82FDB4B8; continue 'dispatch;
	}
	// 82FDB494: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FDB498: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82FDB49C: 4BFF6915  bl 0x82fd1db0
	ctx.lr = 0x82FDB4A0;
	sub_82FD1DB0(ctx, base);
	// 82FDB4A0: 39630001  addi r11, r3, 1
	ctx.r[11].s64 = ctx.r[3].s64 + 1;
	// 82FDB4A4: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82FDB4A8: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82FDB4AC: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82FDB4B0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDB4B4: 418200A4  beq 0x82fdb558
	if ctx.cr[0].eq {
	pc = 0x82FDB558; continue 'dispatch;
	}
	// 82FDB4B8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82FDB4BC: 409A0028  bne cr6, 0x82fdb4e4
	if !ctx.cr[6].eq {
	pc = 0x82FDB4E4; continue 'dispatch;
	}
	// 82FDB4C0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FDB4C4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FDB4C8: 4BFF68E9  bl 0x82fd1db0
	ctx.lr = 0x82FDB4CC;
	sub_82FD1DB0(ctx, base);
	// 82FDB4CC: 39630001  addi r11, r3, 1
	ctx.r[11].s64 = ctx.r[3].s64 + 1;
	// 82FDB4D0: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82FDB4D4: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82FDB4D8: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82FDB4DC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDB4E0: 41820078  beq 0x82fdb558
	if ctx.cr[0].eq {
	pc = 0x82FDB558; continue 'dispatch;
	}
	// 82FDB4E4: 3B5A0001  addi r26, r26, 1
	ctx.r[26].s64 = ctx.r[26].s64 + 1;
	// 82FDB4E8: 3BBD0002  addi r29, r29, 2
	ctx.r[29].s64 = ctx.r[29].s64 + 2;
	// 82FDB4EC: 7F1AB800  cmpw cr6, r26, r23
	ctx.cr[6].compare_i32(ctx.r[26].s32, ctx.r[23].s32, &mut ctx.xer);
	// 82FDB4F0: 4198FF40  blt cr6, 0x82fdb430
	if ctx.cr[6].lt {
	pc = 0x82FDB430; continue 'dispatch;
	}
	// 82FDB4F4: 57CB043E  clrlwi r11, r30, 0x10
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x0000FFFFu64;
	// 82FDB4F8: 394BFFC1  addi r10, r11, -0x3f
	ctx.r[10].s64 = ctx.r[11].s64 + -63;
	// 82FDB4FC: 7D4A0034  cntlzw r10, r10
	ctx.r[10].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 82FDB500: 554ADFFE  rlwinm r10, r10, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 82FDB504: 555B063F  clrlwi. r27, r10, 0x18
	ctx.r[27].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 82FDB508: 4082000C  bne 0x82fdb514
	if !ctx.cr[0].eq {
	pc = 0x82FDB514; continue 'dispatch;
	}
	// 82FDB50C: 2B0B0023  cmplwi cr6, r11, 0x23
	ctx.cr[6].compare_u32(ctx.r[11].u32, 35 as u32, &mut ctx.xer);
	// 82FDB510: 409A00A0  bne cr6, 0x82fdb5b0
	if !ctx.cr[6].eq {
	pc = 0x82FDB5B0; continue 'dispatch;
	}
	// 82FDB514: 3BFA0001  addi r31, r26, 1
	ctx.r[31].s64 = ctx.r[26].s64 + 1;
	// 82FDB518: 7F1FB800  cmpw cr6, r31, r23
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[23].s32, &mut ctx.xer);
	// 82FDB51C: 40980094  bge cr6, 0x82fdb5b0
	if !ctx.cr[6].lt {
	pc = 0x82FDB5B0; continue 'dispatch;
	}
	// 82FDB520: 57EA083C  slwi r10, r31, 1
	ctx.r[10].u32 = ctx.r[31].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FDB524: 397F0002  addi r11, r31, 2
	ctx.r[11].s64 = ctx.r[31].s64 + 2;
	// 82FDB528: 7FCAC214  add r30, r10, r24
	ctx.r[30].u64 = ctx.r[10].u64 + ctx.r[24].u64;
	// 82FDB52C: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FDB530: 7D7D5B78  mr r29, r11
	ctx.r[29].u64 = ctx.r[11].u64;
	// 82FDB534: 7F8AC214  add r28, r10, r24
	ctx.r[28].u64 = ctx.r[10].u64 + ctx.r[24].u64;
	// 82FDB538: A07E0000  lhz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDB53C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82FDB540: 2B0B0023  cmplwi cr6, r11, 0x23
	ctx.cr[6].compare_u32(ctx.r[11].u32, 35 as u32, &mut ctx.xer);
	// 82FDB544: 409A001C  bne cr6, 0x82fdb560
	if !ctx.cr[6].eq {
	pc = 0x82FDB560; continue 'dispatch;
	}
	// 82FDB548: 576A063F  clrlwi. r10, r27, 0x18
	ctx.r[10].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82FDB54C: 41820014  beq 0x82fdb560
	if ctx.cr[0].eq {
	pc = 0x82FDB560; continue 'dispatch;
	}
	// 82FDB550: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82FDB554: 48000044  b 0x82fdb598
	pc = 0x82FDB598; continue 'dispatch;
	// 82FDB558: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FDB55C: 48000058  b 0x82fdb5b4
	pc = 0x82FDB5B4; continue 'dispatch;
	// 82FDB560: 2B0B0025  cmplwi cr6, r11, 0x25
	ctx.cr[6].compare_u32(ctx.r[11].u32, 37 as u32, &mut ctx.xer);
	// 82FDB564: 409A0028  bne cr6, 0x82fdb58c
	if !ctx.cr[6].eq {
	pc = 0x82FDB58C; continue 'dispatch;
	}
	// 82FDB568: 7F1DB800  cmpw cr6, r29, r23
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[23].s32, &mut ctx.xer);
	// 82FDB56C: 4098FFEC  bge cr6, 0x82fdb558
	if !ctx.cr[6].lt {
	pc = 0x82FDB558; continue 'dispatch;
	}
	// 82FDB570: A07E0002  lhz r3, 2(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(2 as u32) ) } as u64;
	// 82FDB574: 4BFF609D  bl 0x82fd1610
	ctx.lr = 0x82FDB578;
	sub_82FD1610(ctx, base);
	// 82FDB578: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDB57C: 4182FFDC  beq 0x82fdb558
	if ctx.cr[0].eq {
	pc = 0x82FDB558; continue 'dispatch;
	}
	// 82FDB580: A07C0000  lhz r3, 0(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDB584: 4BFF608D  bl 0x82fd1610
	ctx.lr = 0x82FDB588;
	sub_82FD1610(ctx, base);
	// 82FDB588: 48000008  b 0x82fdb590
	pc = 0x82FDB590; continue 'dispatch;
	// 82FDB58C: 4BFFE8D5  bl 0x82fd9e60
	ctx.lr = 0x82FDB590;
	sub_82FD9E60(ctx, base);
	// 82FDB590: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDB594: 4182FFC4  beq 0x82fdb558
	if ctx.cr[0].eq {
	pc = 0x82FDB558; continue 'dispatch;
	}
	// 82FDB598: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82FDB59C: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82FDB5A0: 3B9C0002  addi r28, r28, 2
	ctx.r[28].s64 = ctx.r[28].s64 + 2;
	// 82FDB5A4: 3BDE0002  addi r30, r30, 2
	ctx.r[30].s64 = ctx.r[30].s64 + 2;
	// 82FDB5A8: 7F1FB800  cmpw cr6, r31, r23
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[23].s32, &mut ctx.xer);
	// 82FDB5AC: 4198FF8C  blt cr6, 0x82fdb538
	if ctx.cr[6].lt {
	pc = 0x82FDB538; continue 'dispatch;
	}
	// 82FDB5B0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82FDB5B4: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82FDB5B8: 481CCBEC  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDB5C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDB5C0 size=156
    let mut pc: u32 = 0x82FDB5C0;
    'dispatch: loop {
        match pc {
            0x82FDB5C0 => {
    //   block [0x82FDB5C0..0x82FDB65C)
	// 82FDB5C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDB5C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FDB5C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FDB5CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FDB5D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDB5D4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FDB5D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FDB5DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FDB5E0: 815E0018  lwz r10, 0x18(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FDB5E4: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82FDB5E8: B16A0000  sth r11, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82FDB5EC: A1630000  lhz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDB5F0: 4800004C  b 0x82fdb63c
	pc = 0x82FDB63C; continue 'dispatch;
	// 82FDB5F4: A09F0000  lhz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDB5F8: 2B040025  cmplwi cr6, r4, 0x25
	ctx.cr[6].compare_u32(ctx.r[4].u32, 37 as u32, &mut ctx.xer);
	// 82FDB5FC: 409A0030  bne cr6, 0x82fdb62c
	if !ctx.cr[6].eq {
	pc = 0x82FDB62C; continue 'dispatch;
	}
	// 82FDB600: A17F0002  lhz r11, 2(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 82FDB604: 2B0B0032  cmplwi cr6, r11, 0x32
	ctx.cr[6].compare_u32(ctx.r[11].u32, 50 as u32, &mut ctx.xer);
	// 82FDB608: 409A0024  bne cr6, 0x82fdb62c
	if !ctx.cr[6].eq {
	pc = 0x82FDB62C; continue 'dispatch;
	}
	// 82FDB60C: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDB610: 2B0B0030  cmplwi cr6, r11, 0x30
	ctx.cr[6].compare_u32(ctx.r[11].u32, 48 as u32, &mut ctx.xer);
	// 82FDB614: 409A0018  bne cr6, 0x82fdb62c
	if !ctx.cr[6].eq {
	pc = 0x82FDB62C; continue 'dispatch;
	}
	// 82FDB618: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 82FDB61C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FDB620: 3BFF0006  addi r31, r31, 6
	ctx.r[31].s64 = ctx.r[31].s64 + 6;
	// 82FDB624: 4BFF54F5  bl 0x82fd0b18
	ctx.lr = 0x82FDB628;
	sub_82FD0B18(ctx, base);
	// 82FDB628: 48000010  b 0x82fdb638
	pc = 0x82FDB638; continue 'dispatch;
	// 82FDB62C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FDB630: 4BFF54E9  bl 0x82fd0b18
	ctx.lr = 0x82FDB634;
	sub_82FD0B18(ctx, base);
	// 82FDB634: 3BFF0002  addi r31, r31, 2
	ctx.r[31].s64 = ctx.r[31].s64 + 2;
	// 82FDB638: A17F0000  lhz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDB63C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDB640: 4082FFB4  bne 0x82fdb5f4
	if !ctx.cr[0].eq {
	pc = 0x82FDB5F4; continue 'dispatch;
	}
	// 82FDB644: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FDB648: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FDB64C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FDB650: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FDB654: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FDB658: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDB660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FDB660 size=12
    let mut pc: u32 = 0x82FDB660;
    'dispatch: loop {
        match pc {
            0x82FDB660 => {
    //   block [0x82FDB660..0x82FDB66C)
	// 82FDB660: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82FDB664: 386B2DD8  addi r3, r11, 0x2dd8
	ctx.r[3].s64 = ctx.r[11].s64 + 11736;
	// 82FDB668: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDB670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDB670 size=444
    let mut pc: u32 = 0x82FDB670;
    'dispatch: loop {
        match pc {
            0x82FDB670 => {
    //   block [0x82FDB670..0x82FDB82C)
	// 82FDB670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDB674: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FDB678: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FDB67C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FDB680: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDB684: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82FDB688: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FDB68C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FDB690: A97F0000  lha r11, 0(r31)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as i16) as i64;
	// 82FDB694: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82FDB698: 556BDFFF  rlwinm. r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDB69C: 418200B0  beq 0x82fdb74c
	if ctx.cr[0].eq {
	pc = 0x82FDB74C; continue 'dispatch;
	}
	// 82FDB6A0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82FDB6A4: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDB6A8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FDB6AC: 4801E255  bl 0x82ff9900
	ctx.lr = 0x82FDB6B0;
	sub_82FF9900(ctx, base);
	// 82FDB6B0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82FDB6B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FDB6B8: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FDB6BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FDB6C0: 4801E241  bl 0x82ff9900
	ctx.lr = 0x82FDB6C4;
	sub_82FF9900(ctx, base);
	// 82FDB6C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82FDB6C8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FDB6CC: 809E000C  lwz r4, 0xc(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FDB6D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FDB6D4: 4801E22D  bl 0x82ff9900
	ctx.lr = 0x82FDB6D8;
	sub_82FF9900(ctx, base);
	// 82FDB6D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FDB6DC: 809E0010  lwz r4, 0x10(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FDB6E0: 4801DC19  bl 0x82ff92f8
	ctx.lr = 0x82FDB6E4;
	sub_82FF92F8(ctx, base);
	// 82FDB6E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82FDB6E8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FDB6EC: 809E0014  lwz r4, 0x14(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FDB6F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FDB6F4: 4801E20D  bl 0x82ff9900
	ctx.lr = 0x82FDB6F8;
	sub_82FF9900(ctx, base);
	// 82FDB6F8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82FDB6FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FDB700: 809E0018  lwz r4, 0x18(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FDB704: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FDB708: 4801E1F9  bl 0x82ff9900
	ctx.lr = 0x82FDB70C;
	sub_82FF9900(ctx, base);
	// 82FDB70C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82FDB710: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FDB714: 809E001C  lwz r4, 0x1c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FDB718: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FDB71C: 4801E1E5  bl 0x82ff9900
	ctx.lr = 0x82FDB720;
	sub_82FF9900(ctx, base);
	// 82FDB720: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82FDB724: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FDB728: 809E0020  lwz r4, 0x20(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FDB72C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FDB730: 4801E1D1  bl 0x82ff9900
	ctx.lr = 0x82FDB734;
	sub_82FF9900(ctx, base);
	// 82FDB734: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82FDB738: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FDB73C: 809E0024  lwz r4, 0x24(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FDB740: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FDB744: 4801E1BD  bl 0x82ff9900
	ctx.lr = 0x82FDB748;
	sub_82FF9900(ctx, base);
	// 82FDB748: 480000CC  b 0x82fdb814
	pc = 0x82FDB814; continue 'dispatch;
	// 82FDB74C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82FDB750: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82FDB754: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 82FDB758: 389E0004  addi r4, r30, 4
	ctx.r[4].s64 = ctx.r[30].s64 + 4;
	// 82FDB75C: 4801E3CD  bl 0x82ff9b28
	ctx.lr = 0x82FDB760;
	sub_82FF9B28(ctx, base);
	// 82FDB760: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82FDB764: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 82FDB768: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82FDB76C: 389E0008  addi r4, r30, 8
	ctx.r[4].s64 = ctx.r[30].s64 + 8;
	// 82FDB770: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FDB774: 4801E3B5  bl 0x82ff9b28
	ctx.lr = 0x82FDB778;
	sub_82FF9B28(ctx, base);
	// 82FDB778: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82FDB77C: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 82FDB780: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82FDB784: 389E000C  addi r4, r30, 0xc
	ctx.r[4].s64 = ctx.r[30].s64 + 12;
	// 82FDB788: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FDB78C: 4801E39D  bl 0x82ff9b28
	ctx.lr = 0x82FDB790;
	sub_82FF9B28(ctx, base);
	// 82FDB790: 389E0010  addi r4, r30, 0x10
	ctx.r[4].s64 = ctx.r[30].s64 + 16;
	// 82FDB794: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FDB798: 4801DDE1  bl 0x82ff9578
	ctx.lr = 0x82FDB79C;
	sub_82FF9578(ctx, base);
	// 82FDB79C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82FDB7A0: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 82FDB7A4: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82FDB7A8: 389E0014  addi r4, r30, 0x14
	ctx.r[4].s64 = ctx.r[30].s64 + 20;
	// 82FDB7AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FDB7B0: 4801E379  bl 0x82ff9b28
	ctx.lr = 0x82FDB7B4;
	sub_82FF9B28(ctx, base);
	// 82FDB7B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82FDB7B8: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 82FDB7BC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82FDB7C0: 389E0018  addi r4, r30, 0x18
	ctx.r[4].s64 = ctx.r[30].s64 + 24;
	// 82FDB7C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FDB7C8: 4801E361  bl 0x82ff9b28
	ctx.lr = 0x82FDB7CC;
	sub_82FF9B28(ctx, base);
	// 82FDB7CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82FDB7D0: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 82FDB7D4: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82FDB7D8: 389E001C  addi r4, r30, 0x1c
	ctx.r[4].s64 = ctx.r[30].s64 + 28;
	// 82FDB7DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FDB7E0: 4801E349  bl 0x82ff9b28
	ctx.lr = 0x82FDB7E4;
	sub_82FF9B28(ctx, base);
	// 82FDB7E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82FDB7E8: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 82FDB7EC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82FDB7F0: 389E0020  addi r4, r30, 0x20
	ctx.r[4].s64 = ctx.r[30].s64 + 32;
	// 82FDB7F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FDB7F8: 4801E331  bl 0x82ff9b28
	ctx.lr = 0x82FDB7FC;
	sub_82FF9B28(ctx, base);
	// 82FDB7FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82FDB800: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 82FDB804: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82FDB808: 389E0024  addi r4, r30, 0x24
	ctx.r[4].s64 = ctx.r[30].s64 + 36;
	// 82FDB80C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FDB810: 4801E319  bl 0x82ff9b28
	ctx.lr = 0x82FDB814;
	sub_82FF9B28(ctx, base);
	// 82FDB814: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FDB818: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FDB81C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FDB820: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FDB824: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FDB828: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDB830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FDB830 size=8
    let mut pc: u32 = 0x82FDB830;
    'dispatch: loop {
        match pc {
            0x82FDB830 => {
    //   block [0x82FDB830..0x82FDB838)
	// 82FDB830: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FDB834: 82139DD0  lwz r16, -0x6230(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-25136 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDB838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDB838 size=84
    let mut pc: u32 = 0x82FDB838;
    'dispatch: loop {
        match pc {
            0x82FDB838 => {
    //   block [0x82FDB838..0x82FDB88C)
	// 82FDB838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDB83C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FDB840: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FDB844: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FDB848: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 82FDB84C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDB850: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FDB854: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FDB858: 396B9DB4  addi r11, r11, -0x624c
	ctx.r[11].s64 = ctx.r[11].s64 + -25164;
	// 82FDB85C: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 82FDB860: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FDB864: 4BFFE6AD  bl 0x82fd9f10
	ctx.lr = 0x82FDB868;
	sub_82FD9F10(ctx, base);
	// 82FDB868: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FDB86C: 396BA93C  addi r11, r11, -0x56c4
	ctx.r[11].s64 = ctx.r[11].s64 + -22212;
	// 82FDB870: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FDB874: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 82FDB878: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FDB87C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FDB880: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FDB884: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FDB888: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDB88C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDB88C size=40
    let mut pc: u32 = 0x82FDB88C;
    'dispatch: loop {
        match pc {
            0x82FDB88C => {
    //   block [0x82FDB88C..0x82FDB8B4)
	// 82FDB88C: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 82FDB890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDB894: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FDB898: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDB89C: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82FDB8A0: 48070EC1  bl 0x8304c760
	ctx.lr = 0x82FDB8A4;
	sub_8304C760(ctx, base);
	// 82FDB8A4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FDB8A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FDB8AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FDB8B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDB8B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDB8B8 size=264
    let mut pc: u32 = 0x82FDB8B8;
    'dispatch: loop {
        match pc {
            0x82FDB8B8 => {
    //   block [0x82FDB8B8..0x82FDB9C0)
	// 82FDB8B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDB8BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FDB8C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FDB8C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FDB8C8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDB8CC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FDB8D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FDB8D4: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82FDB8D8: 409A0048  bne cr6, 0x82fdb920
	if !ctx.cr[6].eq {
	pc = 0x82FDB920; continue 'dispatch;
	}
	// 82FDB8DC: 80DF0028  lwz r6, 0x28(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDB8E0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FDB8E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FDB8E8: 38EB9C08  addi r7, r11, -0x63f8
	ctx.r[7].s64 = ctx.r[11].s64 + -25592;
	// 82FDB8EC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FDB8F0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82FDB8F4: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 82FDB8F8: 388B9D88  addi r4, r11, -0x6278
	ctx.r[4].s64 = ctx.r[11].s64 + -25208;
	// 82FDB8FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82FDB900: 38C0010E  li r6, 0x10e
	ctx.r[6].s64 = 270;
	// 82FDB904: 38A0044D  li r5, 0x44d
	ctx.r[5].s64 = 1101;
	// 82FDB908: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82FDB90C: 4BFFE2C5  bl 0x82fd9bd0
	ctx.lr = 0x82FDB910;
	sub_82FD9BD0(ctx, base);
	// 82FDB910: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FDB914: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82FDB918: 388BC5E4  addi r4, r11, -0x3a1c
	ctx.r[4].s64 = ctx.r[11].s64 + -14876;
	// 82FDB91C: 481D530D  bl 0x831b0c28
	ctx.lr = 0x82FDB920;
	sub_831B0C28(ctx, base);
	// 82FDB920: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FDB924: 4BFFEEBD  bl 0x82fda7e0
	ctx.lr = 0x82FDB928;
	sub_82FDA7E0(ctx, base);
	// 82FDB928: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDB92C: 40820048  bne 0x82fdb974
	if !ctx.cr[0].eq {
	pc = 0x82FDB974; continue 'dispatch;
	}
	// 82FDB930: 80DF0028  lwz r6, 0x28(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDB934: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FDB938: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FDB93C: 38EB9C08  addi r7, r11, -0x63f8
	ctx.r[7].s64 = ctx.r[11].s64 + -25592;
	// 82FDB940: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FDB944: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82FDB948: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 82FDB94C: 388B9D88  addi r4, r11, -0x6278
	ctx.r[4].s64 = ctx.r[11].s64 + -25208;
	// 82FDB950: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 82FDB954: 38C0010F  li r6, 0x10f
	ctx.r[6].s64 = 271;
	// 82FDB958: 38A00456  li r5, 0x456
	ctx.r[5].s64 = 1110;
	// 82FDB95C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82FDB960: 4BFFE271  bl 0x82fd9bd0
	ctx.lr = 0x82FDB964;
	sub_82FD9BD0(ctx, base);
	// 82FDB964: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FDB968: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82FDB96C: 388BC5E4  addi r4, r11, -0x3a1c
	ctx.r[4].s64 = ctx.r[11].s64 + -14876;
	// 82FDB970: 481D52B9  bl 0x831b0c28
	ctx.lr = 0x82FDB974;
	sub_831B0C28(ctx, base);
	// 82FDB974: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDB978: 2C040000  cmpwi r4, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82FDB97C: 41820018  beq 0x82fdb994
	if ctx.cr[0].eq {
	pc = 0x82FDB994; continue 'dispatch;
	}
	// 82FDB980: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDB984: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDB988: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FDB98C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDB990: 4E800421  bctrl
	ctx.lr = 0x82FDB994;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDB994: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FDB998: 809F0028  lwz r4, 0x28(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDB99C: 4BFF51E5  bl 0x82fd0b80
	ctx.lr = 0x82FDB9A0;
	sub_82FD0B80(ctx, base);
	// 82FDB9A0: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82FDB9A4: 4BFF66C5  bl 0x82fd2068
	ctx.lr = 0x82FDB9A8;
	sub_82FD2068(ctx, base);
	// 82FDB9A8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82FDB9AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FDB9B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FDB9B4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FDB9B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FDB9BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDB9C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FDB9C0 size=8
    let mut pc: u32 = 0x82FDB9C0;
    'dispatch: loop {
        match pc {
            0x82FDB9C0 => {
    //   block [0x82FDB9C0..0x82FDB9C8)
	// 82FDB9C0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FDB9C4: 82139E44  lwz r16, -0x61bc(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-25020 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDB9C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDB9C8 size=228
    let mut pc: u32 = 0x82FDB9C8;
    'dispatch: loop {
        match pc {
            0x82FDB9C8 => {
    //   block [0x82FDB9C8..0x82FDBAAC)
	// 82FDB9C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDB9CC: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 82FDB9D0: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 82FDB9D4: 481CC799  bl 0x831a816c
	ctx.lr = 0x82FDB9D8;
	sub_831A8130(ctx, base);
	// 82FDB9D8: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 82FDB9DC: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDB9E0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82FDB9E4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FDB9E8: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82FDB9EC: 419A0054  beq cr6, 0x82fdba40
	if ctx.cr[6].eq {
	pc = 0x82FDBA40; continue 'dispatch;
	}
	// 82FDB9F0: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FDB9F4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDB9F8: 40820048  bne 0x82fdba40
	if !ctx.cr[0].eq {
	pc = 0x82FDBA40; continue 'dispatch;
	}
	// 82FDB9FC: 80DE0028  lwz r6, 0x28(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDBA00: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FDBA04: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FDBA08: 38EB9C30  addi r7, r11, -0x63d0
	ctx.r[7].s64 = ctx.r[11].s64 + -25552;
	// 82FDBA0C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FDBA10: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82FDBA14: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 82FDBA18: 388B9D88  addi r4, r11, -0x6278
	ctx.r[4].s64 = ctx.r[11].s64 + -25208;
	// 82FDBA1C: 7FA8EB78  mr r8, r29
	ctx.r[8].u64 = ctx.r[29].u64;
	// 82FDBA20: 38C00111  li r6, 0x111
	ctx.r[6].s64 = 273;
	// 82FDBA24: 38A00475  li r5, 0x475
	ctx.r[5].s64 = 1141;
	// 82FDBA28: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FDBA2C: 4BFFE1A5  bl 0x82fd9bd0
	ctx.lr = 0x82FDBA30;
	sub_82FD9BD0(ctx, base);
	// 82FDBA30: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FDBA34: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FDBA38: 388BC5E4  addi r4, r11, -0x3a1c
	ctx.r[4].s64 = ctx.r[11].s64 + -14876;
	// 82FDBA3C: 481D51ED  bl 0x831b0c28
	ctx.lr = 0x82FDBA40;
	sub_831B0C28(ctx, base);
	// 82FDBA40: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FDBA44: 809E0028  lwz r4, 0x28(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDBA48: 4BFFEE39  bl 0x82fda880
	ctx.lr = 0x82FDBA4C;
	sub_82FDA880(ctx, base);
	// 82FDBA4C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FDBA50: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FDBA54: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FDBA58: 2C040000  cmpwi r4, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82FDBA5C: 41820018  beq 0x82fdba74
	if ctx.cr[0].eq {
	pc = 0x82FDBA74; continue 'dispatch;
	}
	// 82FDBA60: 807E0028  lwz r3, 0x28(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDBA64: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDBA68: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FDBA6C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDBA70: 4E800421  bctrl
	ctx.lr = 0x82FDBA74;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDBA74: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82FDBA78: 419A0024  beq cr6, 0x82fdba9c
	if ctx.cr[6].eq {
	pc = 0x82FDBA9C; continue 'dispatch;
	}
	// 82FDBA7C: A17D0000  lhz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDBA80: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDBA84: 41820018  beq 0x82fdba9c
	if ctx.cr[0].eq {
	pc = 0x82FDBA9C; continue 'dispatch;
	}
	// 82FDBA88: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FDBA8C: 809E0028  lwz r4, 0x28(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDBA90: 4BFF50F1  bl 0x82fd0b80
	ctx.lr = 0x82FDBA94;
	sub_82FD0B80(ctx, base);
	// 82FDBA94: 907E0008  stw r3, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 82FDBA98: 4800000C  b 0x82fdbaa4
	pc = 0x82FDBAA4; continue 'dispatch;
	// 82FDBA9C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FDBAA0: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82FDBAA4: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 82FDBAA8: 481CC714  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDBAB4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDBAB4 size=24
    let mut pc: u32 = 0x82FDBAB4;
    'dispatch: loop {
        match pc {
            0x82FDBAB4 => {
    //   block [0x82FDBAB4..0x82FDBACC)
	// 82FDBAB4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDBAB8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FDBABC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDBAC0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FDBAC4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FDBAC8: 481D5161  bl 0x831b0c28
	ctx.lr = 0x82FDBACC;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDBAD4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDBAD4 size=24
    let mut pc: u32 = 0x82FDBAD4;
    'dispatch: loop {
        match pc {
            0x82FDBAD4 => {
    //   block [0x82FDBAD4..0x82FDBAEC)
	// 82FDBAD4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDBAD8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FDBADC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDBAE0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FDBAE4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FDBAE8: 481D5141  bl 0x831b0c28
	ctx.lr = 0x82FDBAEC;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDBAF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDBAF0 size=316
    let mut pc: u32 = 0x82FDBAF0;
    'dispatch: loop {
        match pc {
            0x82FDBAF0 => {
    //   block [0x82FDBAF0..0x82FDBC2C)
	// 82FDBAF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDBAF4: 481CC679  bl 0x831a816c
	ctx.lr = 0x82FDBAF8;
	sub_831A8130(ctx, base);
	// 82FDBAF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDBAFC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FDBB00: 3BE4FFFF  addi r31, r4, -1
	ctx.r[31].s64 = ctx.r[4].s64 + -1;
	// 82FDBB04: 2F040002  cmpwi cr6, r4, 2
	ctx.cr[6].compare_i32(ctx.r[4].s32, 2, &mut ctx.xer);
	// 82FDBB08: 40990118  ble cr6, 0x82fdbc20
	if !ctx.cr[6].gt {
	pc = 0x82FDBC20; continue 'dispatch;
	}
	// 82FDBB0C: A17D0000  lhz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDBB10: 2B0B005B  cmplwi cr6, r11, 0x5b
	ctx.cr[6].compare_u32(ctx.r[11].u32, 91 as u32, &mut ctx.xer);
	// 82FDBB14: 409A010C  bne cr6, 0x82fdbc20
	if !ctx.cr[6].eq {
	pc = 0x82FDBC20; continue 'dispatch;
	}
	// 82FDBB18: 57EB083C  slwi r11, r31, 1
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FDBB1C: 7D6BEA2E  lhzx r11, r11, r29
	ctx.r[11].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82FDBB20: 2B0B005D  cmplwi cr6, r11, 0x5d
	ctx.cr[6].compare_u32(ctx.r[11].u32, 93 as u32, &mut ctx.xer);
	// 82FDBB24: 409A00FC  bne cr6, 0x82fdbc20
	if !ctx.cr[6].eq {
	pc = 0x82FDBC20; continue 'dispatch;
	}
	// 82FDBB28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FDBB2C: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82FDBB30: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82FDBB34: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82FDBB38: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82FDBB3C: 4BFFF18D  bl 0x82fdacc8
	ctx.lr = 0x82FDBB40;
	sub_82FDACC8(ctx, base);
	// 82FDBB40: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82FDBB44: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 82FDBB48: 419A00D8  beq cr6, 0x82fdbc20
	if ctx.cr[6].eq {
	pc = 0x82FDBC20; continue 'dispatch;
	}
	// 82FDBB4C: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82FDBB50: 409A001C  bne cr6, 0x82fdbb6c
	if !ctx.cr[6].eq {
	pc = 0x82FDBB6C; continue 'dispatch;
	}
	// 82FDBB54: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FDBB58: 396BFFF8  addi r11, r11, -8
	ctx.r[11].s64 = ctx.r[11].s64 + -8;
	// 82FDBB5C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82FDBB60: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82FDBB64: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82FDBB68: 480000BC  b 0x82fdbc24
	pc = 0x82FDBC24; continue 'dispatch;
	// 82FDBB6C: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 82FDBB70: 7F0AF800  cmpw cr6, r10, r31
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82FDBB74: 409800AC  bge cr6, 0x82fdbc20
	if !ctx.cr[6].lt {
	pc = 0x82FDBC20; continue 'dispatch;
	}
	// 82FDBB78: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FDBB7C: 7D4AEA14  add r10, r10, r29
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[29].u64;
	// 82FDBB80: A12A0000  lhz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDBB84: 2B09003A  cmplwi cr6, r9, 0x3a
	ctx.cr[6].compare_u32(ctx.r[9].u32, 58 as u32, &mut ctx.xer);
	// 82FDBB88: 409A0098  bne cr6, 0x82fdbc20
	if !ctx.cr[6].eq {
	pc = 0x82FDBC20; continue 'dispatch;
	}
	// 82FDBB8C: 386A0002  addi r3, r10, 2
	ctx.r[3].s64 = ctx.r[10].s64 + 2;
	// 82FDBB90: A1430000  lhz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDBB94: 2B0A003A  cmplwi cr6, r10, 0x3a
	ctx.cr[6].compare_u32(ctx.r[10].u32, 58 as u32, &mut ctx.xer);
	// 82FDBB98: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FDBB9C: 409A0070  bne cr6, 0x82fdbc0c
	if !ctx.cr[6].eq {
	pc = 0x82FDBC0C; continue 'dispatch;
	}
	// 82FDBBA0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82FDBBA4: 2F0A0008  cmpwi cr6, r10, 8
	ctx.cr[6].compare_i32(ctx.r[10].s32, 8, &mut ctx.xer);
	// 82FDBBA8: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82FDBBAC: 41990074  bgt cr6, 0x82fdbc20
	if ctx.cr[6].gt {
	pc = 0x82FDBC20; continue 'dispatch;
	}
	// 82FDBBB0: 388B0002  addi r4, r11, 2
	ctx.r[4].s64 = ctx.r[11].s64 + 2;
	// 82FDBBB4: 7F04F800  cmpw cr6, r4, r31
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82FDBBB8: 409A000C  bne cr6, 0x82fdbbc4
	if !ctx.cr[6].eq {
	pc = 0x82FDBBC4; continue 'dispatch;
	}
	// 82FDBBBC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82FDBBC0: 48000064  b 0x82fdbc24
	pc = 0x82FDBC24; continue 'dispatch;
	// 82FDBBC4: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82FDBBC8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82FDBBCC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FDBBD0: 7D5E5378  mr r30, r10
	ctx.r[30].u64 = ctx.r[10].u64;
	// 82FDBBD4: 4BFFF0F5  bl 0x82fdacc8
	ctx.lr = 0x82FDBBD8;
	sub_82FDACC8(ctx, base);
	// 82FDBBD8: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82FDBBDC: 419A0044  beq cr6, 0x82fdbc20
	if ctx.cr[6].eq {
	pc = 0x82FDBC20; continue 'dispatch;
	}
	// 82FDBBE0: 7F03F800  cmpw cr6, r3, r31
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82FDBBE4: 419AFFD8  beq cr6, 0x82fdbbbc
	if ctx.cr[6].eq {
	pc = 0x82FDBBBC; continue 'dispatch;
	}
	// 82FDBBE8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FDBBEC: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82FDBBF0: 40990008  ble cr6, 0x82fdbbf8
	if !ctx.cr[6].gt {
	pc = 0x82FDBBF8; continue 'dispatch;
	}
	// 82FDBBF4: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 82FDBBF8: 546B083C  slwi r11, r3, 1
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FDBBFC: 7C83F850  subf r4, r3, r31
	ctx.r[4].s64 = ctx.r[31].s64 - ctx.r[3].s64;
	// 82FDBC00: 7C6BEA14  add r3, r11, r29
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82FDBC04: 4BFFEFDD  bl 0x82fdabe0
	ctx.lr = 0x82FDBC08;
	sub_82FDABE0(ctx, base);
	// 82FDBC08: 4800001C  b 0x82fdbc24
	pc = 0x82FDBC24; continue 'dispatch;
	// 82FDBC0C: 2F0A0006  cmpwi cr6, r10, 6
	ctx.cr[6].compare_i32(ctx.r[10].s32, 6, &mut ctx.xer);
	// 82FDBC10: 409A0010  bne cr6, 0x82fdbc20
	if !ctx.cr[6].eq {
	pc = 0x82FDBC20; continue 'dispatch;
	}
	// 82FDBC14: 7D6BF850  subf r11, r11, r31
	ctx.r[11].s64 = ctx.r[31].s64 - ctx.r[11].s64;
	// 82FDBC18: 388BFFFF  addi r4, r11, -1
	ctx.r[4].s64 = ctx.r[11].s64 + -1;
	// 82FDBC1C: 4BFFFFE8  b 0x82fdbc04
	pc = 0x82FDBC04; continue 'dispatch;
	// 82FDBC20: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FDBC24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FDBC28: 481CC594  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDBC30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDBC30 size=368
    let mut pc: u32 = 0x82FDBC30;
    'dispatch: loop {
        match pc {
            0x82FDBC30 => {
    //   block [0x82FDBC30..0x82FDBDA0)
	// 82FDBC30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDBC34: 481CC535  bl 0x831a8168
	ctx.lr = 0x82FDBC38;
	sub_831A8130(ctx, base);
	// 82FDBC38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDBC3C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82FDBC40: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FDBC44: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82FDBC48: 419A014C  beq cr6, 0x82fdbd94
	if ctx.cr[6].eq {
	pc = 0x82FDBD94; continue 'dispatch;
	}
	// 82FDBC4C: A17F0000  lhz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDBC50: 2B0B005B  cmplwi cr6, r11, 0x5b
	ctx.cr[6].compare_u32(ctx.r[11].u32, 91 as u32, &mut ctx.xer);
	// 82FDBC54: 409A000C  bne cr6, 0x82fdbc60
	if !ctx.cr[6].eq {
	pc = 0x82FDBC60; continue 'dispatch;
	}
	// 82FDBC58: 4BFFFE99  bl 0x82fdbaf0
	ctx.lr = 0x82FDBC5C;
	sub_82FDBAF0(ctx, base);
	// 82FDBC5C: 4800013C  b 0x82fdbd98
	pc = 0x82FDBD98; continue 'dispatch;
	// 82FDBC60: 2B0B002E  cmplwi cr6, r11, 0x2e
	ctx.cr[6].compare_u32(ctx.r[11].u32, 46 as u32, &mut ctx.xer);
	// 82FDBC64: 419A0130  beq cr6, 0x82fdbd94
	if ctx.cr[6].eq {
	pc = 0x82FDBD94; continue 'dispatch;
	}
	// 82FDBC68: 2B0B002D  cmplwi cr6, r11, 0x2d
	ctx.cr[6].compare_u32(ctx.r[11].u32, 45 as u32, &mut ctx.xer);
	// 82FDBC6C: 419A0128  beq cr6, 0x82fdbd94
	if ctx.cr[6].eq {
	pc = 0x82FDBD94; continue 'dispatch;
	}
	// 82FDBC70: 57AB083C  slwi r11, r29, 1
	ctx.r[11].u32 = ctx.r[29].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FDBC74: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82FDBC78: A16BFFFE  lhz r11, -2(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(-2 as u32) ) } as u64;
	// 82FDBC7C: 2B0B002D  cmplwi cr6, r11, 0x2d
	ctx.cr[6].compare_u32(ctx.r[11].u32, 45 as u32, &mut ctx.xer);
	// 82FDBC80: 419A0114  beq cr6, 0x82fdbd94
	if ctx.cr[6].eq {
	pc = 0x82FDBD94; continue 'dispatch;
	}
	// 82FDBC84: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82FDBC88: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FDBC8C: 3860002E  li r3, 0x2e
	ctx.r[3].s64 = 46;
	// 82FDBC90: 4BFF6239  bl 0x82fd1ec8
	ctx.lr = 0x82FDBC94;
	sub_82FD1EC8(ctx, base);
	// 82FDBC94: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FDBC98: 397E0001  addi r11, r30, 1
	ctx.r[11].s64 = ctx.r[30].s64 + 1;
	// 82FDBC9C: 7F0BE800  cmpw cr6, r11, r29
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82FDBCA0: 409A0030  bne cr6, 0x82fdbcd0
	if !ctx.cr[6].eq {
	pc = 0x82FDBCD0; continue 'dispatch;
	}
	// 82FDBCA4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82FDBCA8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FDBCAC: 3860002E  li r3, 0x2e
	ctx.r[3].s64 = 46;
	// 82FDBCB0: 4BFF6219  bl 0x82fd1ec8
	ctx.lr = 0x82FDBCB4;
	sub_82FD1EC8(ctx, base);
	// 82FDBCB4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FDBCB8: 397E0001  addi r11, r30, 1
	ctx.r[11].s64 = ctx.r[30].s64 + 1;
	// 82FDBCBC: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FDBCC0: 7C6BFA2E  lhzx r3, r11, r31
	ctx.r[3].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82FDBCC4: 4BFF58C5  bl 0x82fd1588
	ctx.lr = 0x82FDBCC8;
	sub_82FD1588(ctx, base);
	// 82FDBCC8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDBCCC: 408200C8  bne 0x82fdbd94
	if !ctx.cr[0].eq {
	pc = 0x82FDBD94; continue 'dispatch;
	}
	// 82FDBCD0: 397E0001  addi r11, r30, 1
	ctx.r[11].s64 = ctx.r[30].s64 + 1;
	// 82FDBCD4: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FDBCD8: 7C6BFA2E  lhzx r3, r11, r31
	ctx.r[3].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82FDBCDC: 4BFF58AD  bl 0x82fd1588
	ctx.lr = 0x82FDBCE0;
	sub_82FD1588(ctx, base);
	// 82FDBCE0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDBCE4: 41820014  beq 0x82fdbcf8
	if ctx.cr[0].eq {
	pc = 0x82FDBCF8; continue 'dispatch;
	}
	// 82FDBCE8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FDBCEC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FDBCF0: 4BFFEEF1  bl 0x82fdabe0
	ctx.lr = 0x82FDBCF4;
	sub_82FDABE0(ctx, base);
	// 82FDBCF4: 480000A4  b 0x82fdbd98
	pc = 0x82FDBD98; continue 'dispatch;
	// 82FDBCF8: 2F1D00FF  cmpwi cr6, r29, 0xff
	ctx.cr[6].compare_i32(ctx.r[29].s32, 255, &mut ctx.xer);
	// 82FDBCFC: 41990098  bgt cr6, 0x82fdbd94
	if ctx.cr[6].gt {
	pc = 0x82FDBD94; continue 'dispatch;
	}
	// 82FDBD00: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82FDBD04: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82FDBD08: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82FDBD0C: 40990080  ble cr6, 0x82fdbd8c
	if !ctx.cr[6].gt {
	pc = 0x82FDBD8C; continue 'dispatch;
	}
	// 82FDBD10: A07F0000  lhz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDBD14: 2B03002E  cmplwi cr6, r3, 0x2e
	ctx.cr[6].compare_u32(ctx.r[3].u32, 46 as u32, &mut ctx.xer);
	// 82FDBD18: 409A0040  bne cr6, 0x82fdbd58
	if !ctx.cr[6].eq {
	pc = 0x82FDBD58; continue 'dispatch;
	}
	// 82FDBD1C: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FDBD20: 40990014  ble cr6, 0x82fdbd34
	if !ctx.cr[6].gt {
	pc = 0x82FDBD34; continue 'dispatch;
	}
	// 82FDBD24: A07FFFFE  lhz r3, -2(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(-2 as u32) ) } as u64;
	// 82FDBD28: 4BFF5881  bl 0x82fd15a8
	ctx.lr = 0x82FDBD2C;
	sub_82FD15A8(ctx, base);
	// 82FDBD2C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDBD30: 41820064  beq 0x82fdbd94
	if ctx.cr[0].eq {
	pc = 0x82FDBD94; continue 'dispatch;
	}
	// 82FDBD34: 397E0001  addi r11, r30, 1
	ctx.r[11].s64 = ctx.r[30].s64 + 1;
	// 82FDBD38: 7F0BE800  cmpw cr6, r11, r29
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82FDBD3C: 40980014  bge cr6, 0x82fdbd50
	if !ctx.cr[6].lt {
	pc = 0x82FDBD50; continue 'dispatch;
	}
	// 82FDBD40: A07F0002  lhz r3, 2(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 82FDBD44: 4BFF5865  bl 0x82fd15a8
	ctx.lr = 0x82FDBD48;
	sub_82FD15A8(ctx, base);
	// 82FDBD48: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDBD4C: 41820048  beq 0x82fdbd94
	if ctx.cr[0].eq {
	pc = 0x82FDBD94; continue 'dispatch;
	}
	// 82FDBD50: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82FDBD54: 48000028  b 0x82fdbd7c
	pc = 0x82FDBD7C; continue 'dispatch;
	// 82FDBD58: 4BFF5851  bl 0x82fd15a8
	ctx.lr = 0x82FDBD5C;
	sub_82FD15A8(ctx, base);
	// 82FDBD5C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDBD60: 40820010  bne 0x82fdbd70
	if !ctx.cr[0].eq {
	pc = 0x82FDBD70; continue 'dispatch;
	}
	// 82FDBD64: A17F0000  lhz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDBD68: 2B0B002D  cmplwi cr6, r11, 0x2d
	ctx.cr[6].compare_u32(ctx.r[11].u32, 45 as u32, &mut ctx.xer);
	// 82FDBD6C: 409A0028  bne cr6, 0x82fdbd94
	if !ctx.cr[6].eq {
	pc = 0x82FDBD94; continue 'dispatch;
	}
	// 82FDBD70: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82FDBD74: 2B1C003F  cmplwi cr6, r28, 0x3f
	ctx.cr[6].compare_u32(ctx.r[28].u32, 63 as u32, &mut ctx.xer);
	// 82FDBD78: 4199001C  bgt cr6, 0x82fdbd94
	if ctx.cr[6].gt {
	pc = 0x82FDBD94; continue 'dispatch;
	}
	// 82FDBD7C: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82FDBD80: 3BFF0002  addi r31, r31, 2
	ctx.r[31].s64 = ctx.r[31].s64 + 2;
	// 82FDBD84: 7F1EE800  cmpw cr6, r30, r29
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82FDBD88: 4198FF88  blt cr6, 0x82fdbd10
	if ctx.cr[6].lt {
	pc = 0x82FDBD10; continue 'dispatch;
	}
	// 82FDBD8C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82FDBD90: 48000008  b 0x82fdbd98
	pc = 0x82FDBD98; continue 'dispatch;
	// 82FDBD94: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FDBD98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FDBD9C: 481CC41C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDBDA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDBDA0 size=100
    let mut pc: u32 = 0x82FDBDA0;
    'dispatch: loop {
        match pc {
            0x82FDBDA0 => {
    //   block [0x82FDBDA0..0x82FDBE04)
	// 82FDBDA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDBDA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FDBDA8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FDBDAC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FDBDB0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDBDB4: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FDBDB8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FDBDBC: 388B9CC8  addi r4, r11, -0x6338
	ctx.r[4].s64 = ctx.r[11].s64 + -25400;
	// 82FDBDC0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FDBDC4: 4BFF5ED5  bl 0x82fd1c98
	ctx.lr = 0x82FDBDC8;
	sub_82FD1C98(ctx, base);
	// 82FDBDC8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDBDCC: 4182001C  beq 0x82fdbde8
	if ctx.cr[0].eq {
	pc = 0x82FDBDE8; continue 'dispatch;
	}
	// 82FDBDD0: 7D7F1850  subf r11, r31, r3
	ctx.r[11].s64 = ctx.r[3].s64 - ctx.r[31].s64;
	// 82FDBDD4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FDBDD8: 7D640E70  srawi r4, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[4].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82FDBDDC: 909E0000  stw r4, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82FDBDE0: 4BFFF569  bl 0x82fdb348
	ctx.lr = 0x82FDBDE4;
	sub_82FDB348(ctx, base);
	// 82FDBDE4: 48000008  b 0x82fdbdec
	pc = 0x82FDBDEC; continue 'dispatch;
	// 82FDBDE8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FDBDEC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FDBDF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FDBDF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FDBDF8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FDBDFC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FDBE00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDBE08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDBE08 size=128
    let mut pc: u32 = 0x82FDBE08;
    'dispatch: loop {
        match pc {
            0x82FDBE08 => {
    //   block [0x82FDBE08..0x82FDBE88)
	// 82FDBE08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDBE0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FDBE10: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FDBE14: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDBE18: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FDBE1C: 3860002C  li r3, 0x2c
	ctx.r[3].s64 = 44;
	// 82FDBE20: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FDBE24: 4BFFC475  bl 0x82fd8298
	ctx.lr = 0x82FDBE28;
	sub_82FD8298(ctx, base);
	// 82FDBE28: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDBE2C: 41820044  beq 0x82fdbe70
	if ctx.cr[0].eq {
	pc = 0x82FDBE70; continue 'dispatch;
	}
	// 82FDBE30: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FDBE34: 93E30028  stw r31, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[31].u32 ) };
	// 82FDBE38: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 82FDBE3C: 394B9DB4  addi r10, r11, -0x624c
	ctx.r[10].s64 = ctx.r[11].s64 + -25164;
	// 82FDBE40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FDBE44: 91230010  stw r9, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 82FDBE48: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82FDBE4C: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82FDBE50: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82FDBE54: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82FDBE58: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82FDBE5C: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82FDBE60: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82FDBE64: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82FDBE68: 91630024  stw r11, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82FDBE6C: 48000008  b 0x82fdbe74
	pc = 0x82FDBE74; continue 'dispatch;
	// 82FDBE70: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FDBE74: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FDBE78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FDBE7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FDBE80: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FDBE84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDBE88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDBE88 size=76
    let mut pc: u32 = 0x82FDBE88;
    'dispatch: loop {
        match pc {
            0x82FDBE88 => {
    //   block [0x82FDBE88..0x82FDBED4)
	// 82FDBE88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDBE8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FDBE90: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FDBE94: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FDBE98: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDBE9C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FDBEA0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FDBEA4: 4BFFF995  bl 0x82fdb838
	ctx.lr = 0x82FDBEA8;
	sub_82FDB838(ctx, base);
	// 82FDBEA8: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDBEAC: 4182000C  beq 0x82fdbeb8
	if ctx.cr[0].eq {
	pc = 0x82FDBEB8; continue 'dispatch;
	}
	// 82FDBEB0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FDBEB4: 4BFFC42D  bl 0x82fd82e0
	ctx.lr = 0x82FDBEB8;
	sub_82FD82E0(ctx, base);
	// 82FDBEB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FDBEBC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FDBEC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FDBEC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FDBEC8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FDBECC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FDBED0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDBED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FDBED8 size=8
    let mut pc: u32 = 0x82FDBED8;
    'dispatch: loop {
        match pc {
            0x82FDBED8 => {
    //   block [0x82FDBED8..0x82FDBEE0)
	// 82FDBED8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FDBEDC: 82139E98  lwz r16, -0x6168(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-24936 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDBEE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDBEE0 size=248
    let mut pc: u32 = 0x82FDBEE0;
    'dispatch: loop {
        match pc {
            0x82FDBEE0 => {
    //   block [0x82FDBEE0..0x82FDBFD8)
	// 82FDBEE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDBEE4: 481CC281  bl 0x831a8164
	ctx.lr = 0x82FDBEE8;
	sub_831A8130(ctx, base);
	// 82FDBEE8: 3BE1FF50  addi r31, r1, -0xb0
	ctx.r[31].s64 = ctx.r[1].s64 + -176;
	// 82FDBEEC: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDBEF0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FDBEF4: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FDBEF8: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82FDBEFC: 388B9CC8  addi r4, r11, -0x6338
	ctx.r[4].s64 = ctx.r[11].s64 + -25400;
	// 82FDBF00: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FDBF04: 4BFF5D95  bl 0x82fd1c98
	ctx.lr = 0x82FDBF08;
	sub_82FD1C98(ctx, base);
	// 82FDBF08: 7C7B1B79  or. r27, r3, r3
	ctx.r[27].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 82FDBF0C: 40820030  bne 0x82fdbf3c
	if !ctx.cr[0].eq {
	pc = 0x82FDBF3C; continue 'dispatch;
	}
	// 82FDBF10: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FDBF14: 80FC0028  lwz r7, 0x28(r28)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDBF18: 38C00110  li r6, 0x110
	ctx.r[6].s64 = 272;
	// 82FDBF1C: 388B9D88  addi r4, r11, -0x6278
	ctx.r[4].s64 = ctx.r[11].s64 + -25208;
	// 82FDBF20: 38A0033D  li r5, 0x33d
	ctx.r[5].s64 = 829;
	// 82FDBF24: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FDBF28: 4BFFDBF1  bl 0x82fd9b18
	ctx.lr = 0x82FDBF2C;
	sub_82FD9B18(ctx, base);
	// 82FDBF2C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FDBF30: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FDBF34: 388BC5E4  addi r4, r11, -0x3a1c
	ctx.r[4].s64 = ctx.r[11].s64 + -14876;
	// 82FDBF38: 481D4CF1  bl 0x831b0c28
	ctx.lr = 0x82FDBF3C;
	sub_831B0C28(ctx, base);
	// 82FDBF3C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82FDBF40: 419A0034  beq cr6, 0x82fdbf74
	if ctx.cr[6].eq {
	pc = 0x82FDBF74; continue 'dispatch;
	}
	// 82FDBF44: A17E0000  lhz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDBF48: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDBF4C: 41820028  beq 0x82fdbf74
	if ctx.cr[0].eq {
	pc = 0x82FDBF74; continue 'dispatch;
	}
	// 82FDBF50: 397E0002  addi r11, r30, 2
	ctx.r[11].s64 = ctx.r[30].s64 + 2;
	// 82FDBF54: 48000008  b 0x82fdbf5c
	pc = 0x82FDBF5C; continue 'dispatch;
	// 82FDBF58: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FDBF5C: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDBF60: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDBF64: 4082FFF4  bne 0x82fdbf58
	if !ctx.cr[0].eq {
	pc = 0x82FDBF58; continue 'dispatch;
	}
	// 82FDBF68: 7D7E5850  subf r11, r30, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	// 82FDBF6C: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82FDBF70: 48000008  b 0x82fdbf78
	pc = 0x82FDBF78; continue 'dispatch;
	// 82FDBF74: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FDBF78: 807C0028  lwz r3, 0x28(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDBF7C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FDBF80: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82FDBF84: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDBF88: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDBF8C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDBF90: 4E800421  bctrl
	ctx.lr = 0x82FDBF94;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDBF94: 80FC0028  lwz r7, 0x28(r28)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDBF98: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FDBF9C: 90FF0054  stw r7, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82FDBFA0: 93BF0050  stw r29, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 82FDBFA4: 7D7ED850  subf r11, r30, r27
	ctx.r[11].s64 = ctx.r[27].s64 - ctx.r[30].s64;
	// 82FDBFA8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FDBFAC: 7D660E70  srawi r6, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[6].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82FDBFB0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FDBFB4: 4BFF60D5  bl 0x82fd2088
	ctx.lr = 0x82FDBFB8;
	sub_82FD2088(ctx, base);
	// 82FDBFB8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FDBFBC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FDBFC0: 4BFFF8F9  bl 0x82fdb8b8
	ctx.lr = 0x82FDBFC4;
	sub_82FDB8B8(ctx, base);
	// 82FDBFC4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FDBFC8: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FDBFCC: 4BFF6AF5  bl 0x82fd2ac0
	ctx.lr = 0x82FDBFD0;
	sub_82FD2AC0(ctx, base);
	// 82FDBFD0: 383F00B0  addi r1, r31, 0xb0
	ctx.r[1].s64 = ctx.r[31].s64 + 176;
	// 82FDBFD4: 481CC1E0  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDBFD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDBFD8 size=40
    let mut pc: u32 = 0x82FDBFD8;
    'dispatch: loop {
        match pc {
            0x82FDBFD8 => {
    //   block [0x82FDBFD8..0x82FDC000)
	// 82FDBFD8: 3BECFF50  addi r31, r12, -0xb0
	ctx.r[31].s64 = ctx.r[12].s64 + -176;
	// 82FDBFDC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDBFE0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FDBFE4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDBFE8: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FDBFEC: 4BFF6E6D  bl 0x82fd2e58
	ctx.lr = 0x82FDBFF0;
	sub_82FD2E58(ctx, base);
	// 82FDBFF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FDBFF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FDBFF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FDBFFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDC000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDC000 size=228
    let mut pc: u32 = 0x82FDC000;
    'dispatch: loop {
        match pc {
            0x82FDC000 => {
    //   block [0x82FDC000..0x82FDC0E4)
	// 82FDC000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDC004: 481CC161  bl 0x831a8164
	ctx.lr = 0x82FDC008;
	sub_831A8130(ctx, base);
	// 82FDC008: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDC00C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82FDC010: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82FDC014: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 82FDC018: 4BFFFC19  bl 0x82fdbc30
	ctx.lr = 0x82FDC01C;
	sub_82FDBC30(ctx, base);
	// 82FDC01C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDC020: 418200B8  beq 0x82fdc0d8
	if ctx.cr[0].eq {
	pc = 0x82FDC0D8; continue 'dispatch;
	}
	// 82FDC024: 3D600000  lis r11, 0
	ctx.r[11].s64 = 0;
	// 82FDC028: 616BFFFF  ori r11, r11, 0xffff
	ctx.r[11].u64 = ctx.r[11].u64 | 65535;
	// 82FDC02C: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82FDC030: 419900A8  bgt cr6, 0x82fdc0d8
	if ctx.cr[6].gt {
	pc = 0x82FDC0D8; continue 'dispatch;
	}
	// 82FDC034: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FDC038: 4098000C  bge cr6, 0x82fdc044
	if !ctx.cr[6].lt {
	pc = 0x82FDC044; continue 'dispatch;
	}
	// 82FDC03C: 2F1EFFFF  cmpwi cr6, r30, -1
	ctx.cr[6].compare_i32(ctx.r[30].s32, -1, &mut ctx.xer);
	// 82FDC040: 409A0098  bne cr6, 0x82fdc0d8
	if !ctx.cr[6].eq {
	pc = 0x82FDC0D8; continue 'dispatch;
	}
	// 82FDC044: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82FDC048: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 82FDC04C: 40990084  ble cr6, 0x82fdc0d0
	if !ctx.cr[6].gt {
	pc = 0x82FDC0D0; continue 'dispatch;
	}
	// 82FDC050: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FDC054: 3BDF0004  addi r30, r31, 4
	ctx.r[30].s64 = ctx.r[31].s64 + 4;
	// 82FDC058: 3B8B9BD0  addi r28, r11, -0x6430
	ctx.r[28].s64 = ctx.r[11].s64 + -25648;
	// 82FDC05C: A07F0000  lhz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDC060: 4BFFDE59  bl 0x82fd9eb8
	ctx.lr = 0x82FDC064;
	sub_82FD9EB8(ctx, base);
	// 82FDC064: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDC068: 40820054  bne 0x82fdc0bc
	if !ctx.cr[0].eq {
	pc = 0x82FDC0BC; continue 'dispatch;
	}
	// 82FDC06C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FDC070: A09F0000  lhz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDC074: 4BFF5D3D  bl 0x82fd1db0
	ctx.lr = 0x82FDC078;
	sub_82FD1DB0(ctx, base);
	// 82FDC078: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82FDC07C: 409A0040  bne cr6, 0x82fdc0bc
	if !ctx.cr[6].eq {
	pc = 0x82FDC0BC; continue 'dispatch;
	}
	// 82FDC080: A17F0000  lhz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDC084: 2B0B0025  cmplwi cr6, r11, 0x25
	ctx.cr[6].compare_u32(ctx.r[11].u32, 37 as u32, &mut ctx.xer);
	// 82FDC088: 409A0050  bne cr6, 0x82fdc0d8
	if !ctx.cr[6].eq {
	pc = 0x82FDC0D8; continue 'dispatch;
	}
	// 82FDC08C: A07F0002  lhz r3, 2(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 82FDC090: 4BFF5581  bl 0x82fd1610
	ctx.lr = 0x82FDC094;
	sub_82FD1610(ctx, base);
	// 82FDC094: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDC098: 41820040  beq 0x82fdc0d8
	if ctx.cr[0].eq {
	pc = 0x82FDC0D8; continue 'dispatch;
	}
	// 82FDC09C: A07E0000  lhz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDC0A0: 4BFF5571  bl 0x82fd1610
	ctx.lr = 0x82FDC0A4;
	sub_82FD1610(ctx, base);
	// 82FDC0A4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDC0A8: 41820030  beq 0x82fdc0d8
	if ctx.cr[0].eq {
	pc = 0x82FDC0D8; continue 'dispatch;
	}
	// 82FDC0AC: 3BBD0003  addi r29, r29, 3
	ctx.r[29].s64 = ctx.r[29].s64 + 3;
	// 82FDC0B0: 3BFF0006  addi r31, r31, 6
	ctx.r[31].s64 = ctx.r[31].s64 + 6;
	// 82FDC0B4: 3BDE0006  addi r30, r30, 6
	ctx.r[30].s64 = ctx.r[30].s64 + 6;
	// 82FDC0B8: 48000010  b 0x82fdc0c8
	pc = 0x82FDC0C8; continue 'dispatch;
	// 82FDC0BC: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82FDC0C0: 3BFF0002  addi r31, r31, 2
	ctx.r[31].s64 = ctx.r[31].s64 + 2;
	// 82FDC0C4: 3BDE0002  addi r30, r30, 2
	ctx.r[30].s64 = ctx.r[30].s64 + 2;
	// 82FDC0C8: 7F1DD800  cmpw cr6, r29, r27
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[27].s32, &mut ctx.xer);
	// 82FDC0CC: 4198FF90  blt cr6, 0x82fdc05c
	if ctx.cr[6].lt {
	pc = 0x82FDC05C; continue 'dispatch;
	}
	// 82FDC0D0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82FDC0D4: 48000008  b 0x82fdc0dc
	pc = 0x82FDC0DC; continue 'dispatch;
	// 82FDC0D8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FDC0DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FDC0E0: 481CC0D4  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDC0E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDC0E8 size=568
    let mut pc: u32 = 0x82FDC0E8;
    'dispatch: loop {
        match pc {
            0x82FDC0E8 => {
    //   block [0x82FDC0E8..0x82FDC320)
	// 82FDC0E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDC0EC: 481CC079  bl 0x831a8164
	ctx.lr = 0x82FDC0F0;
	sub_831A8130(ctx, base);
	// 82FDC0F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDC0F4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FDC0F8: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82FDC0FC: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82FDC100: 419A0070  beq cr6, 0x82fdc170
	if ctx.cr[6].eq {
	pc = 0x82FDC170; continue 'dispatch;
	}
	// 82FDC104: A13D0000  lhz r9, 0(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDC108: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDC10C: 41820064  beq 0x82fdc170
	if ctx.cr[0].eq {
	pc = 0x82FDC170; continue 'dispatch;
	}
	// 82FDC110: 397D0002  addi r11, r29, 2
	ctx.r[11].s64 = ctx.r[29].s64 + 2;
	// 82FDC114: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82FDC118: 48000008  b 0x82fdc120
	pc = 0x82FDC120; continue 'dispatch;
	// 82FDC11C: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 82FDC120: A10A0000  lhz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDC124: 28080000  cmplwi r8, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDC128: 4082FFF4  bne 0x82fdc11c
	if !ctx.cr[0].eq {
	pc = 0x82FDC11C; continue 'dispatch;
	}
	// 82FDC12C: 7D5D5050  subf r10, r29, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[29].s64;
	// 82FDC130: 2B09005B  cmplwi cr6, r9, 0x5b
	ctx.cr[6].compare_u32(ctx.r[9].u32, 91 as u32, &mut ctx.xer);
	// 82FDC134: 7D5B0E70  srawi r27, r10, 1
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[27].s64 = (ctx.r[10].s32 >> 1) as i64;
	// 82FDC138: 409A0014  bne cr6, 0x82fdc14c
	if !ctx.cr[6].eq {
	pc = 0x82FDC14C; continue 'dispatch;
	}
	// 82FDC13C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82FDC140: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FDC144: 4BFFF9AD  bl 0x82fdbaf0
	ctx.lr = 0x82FDC148;
	sub_82FDBAF0(ctx, base);
	// 82FDC148: 4800002C  b 0x82fdc174
	pc = 0x82FDC174; continue 'dispatch;
	// 82FDC14C: 2B09002E  cmplwi cr6, r9, 0x2e
	ctx.cr[6].compare_u32(ctx.r[9].u32, 46 as u32, &mut ctx.xer);
	// 82FDC150: 419A0020  beq cr6, 0x82fdc170
	if ctx.cr[6].eq {
	pc = 0x82FDC170; continue 'dispatch;
	}
	// 82FDC154: 2B09002D  cmplwi cr6, r9, 0x2d
	ctx.cr[6].compare_u32(ctx.r[9].u32, 45 as u32, &mut ctx.xer);
	// 82FDC158: 419A0018  beq cr6, 0x82fdc170
	if ctx.cr[6].eq {
	pc = 0x82FDC170; continue 'dispatch;
	}
	// 82FDC15C: 577E083C  slwi r30, r27, 1
	ctx.r[30].u32 = ctx.r[27].u32.wrapping_shl(1);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82FDC160: 7D5EEA14  add r10, r30, r29
	ctx.r[10].u64 = ctx.r[30].u64 + ctx.r[29].u64;
	// 82FDC164: A14AFFFE  lhz r10, -2(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(-2 as u32) ) } as u64;
	// 82FDC168: 2B0A002D  cmplwi cr6, r10, 0x2d
	ctx.cr[6].compare_u32(ctx.r[10].u32, 45 as u32, &mut ctx.xer);
	// 82FDC16C: 409A0014  bne cr6, 0x82fdc180
	if !ctx.cr[6].eq {
	pc = 0x82FDC180; continue 'dispatch;
	}
	// 82FDC170: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FDC174: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FDC178: 481CC03C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 82FDC17C: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FDC180: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDC184: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDC188: 4082FFF4  bne 0x82fdc17c
	if !ctx.cr[0].eq {
	pc = 0x82FDC17C; continue 'dispatch;
	}
	// 82FDC18C: 7D7D5850  subf r11, r29, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	// 82FDC190: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FDC194: 7D650E70  srawi r5, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[5].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82FDC198: 3860002E  li r3, 0x2e
	ctx.r[3].s64 = 46;
	// 82FDC19C: 4BFF5D2D  bl 0x82fd1ec8
	ctx.lr = 0x82FDC1A0;
	sub_82FD1EC8(ctx, base);
	// 82FDC1A0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FDC1A4: 397F0001  addi r11, r31, 1
	ctx.r[11].s64 = ctx.r[31].s64 + 1;
	// 82FDC1A8: 7F0BD800  cmpw cr6, r11, r27
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[27].s32, &mut ctx.xer);
	// 82FDC1AC: 409A00B0  bne cr6, 0x82fdc25c
	if !ctx.cr[6].eq {
	pc = 0x82FDC25C; continue 'dispatch;
	}
	// 82FDC1B0: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDC1B4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FDC1B8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FDC1BC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDC1C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDC1C4: 4E800421  bctrl
	ctx.lr = 0x82FDC1C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDC1C8: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82FDC1CC: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82FDC1D0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FDC1D4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FDC1D8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FDC1DC: 4BFF5EAD  bl 0x82fd2088
	ctx.lr = 0x82FDC1E0;
	sub_82FD2088(ctx, base);
	// 82FDC1E0: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82FDC1E4: 419A0034  beq cr6, 0x82fdc218
	if ctx.cr[6].eq {
	pc = 0x82FDC218; continue 'dispatch;
	}
	// 82FDC1E8: A17E0000  lhz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDC1EC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDC1F0: 41820028  beq 0x82fdc218
	if ctx.cr[0].eq {
	pc = 0x82FDC218; continue 'dispatch;
	}
	// 82FDC1F4: 397E0002  addi r11, r30, 2
	ctx.r[11].s64 = ctx.r[30].s64 + 2;
	// 82FDC1F8: 48000008  b 0x82fdc200
	pc = 0x82FDC200; continue 'dispatch;
	// 82FDC1FC: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FDC200: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDC204: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDC208: 4082FFF4  bne 0x82fdc1fc
	if !ctx.cr[0].eq {
	pc = 0x82FDC1FC; continue 'dispatch;
	}
	// 82FDC20C: 7D7E5850  subf r11, r30, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	// 82FDC210: 7D650E70  srawi r5, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[5].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82FDC214: 48000008  b 0x82fdc21c
	pc = 0x82FDC21C; continue 'dispatch;
	// 82FDC218: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FDC21C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FDC220: 3860002E  li r3, 0x2e
	ctx.r[3].s64 = 46;
	// 82FDC224: 4BFF5CA5  bl 0x82fd1ec8
	ctx.lr = 0x82FDC228;
	sub_82FD1EC8(ctx, base);
	// 82FDC228: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDC22C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FDC230: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FDC234: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FDC238: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FDC23C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDC240: 4E800421  bctrl
	ctx.lr = 0x82FDC244;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDC244: 397F0001  addi r11, r31, 1
	ctx.r[11].s64 = ctx.r[31].s64 + 1;
	// 82FDC248: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FDC24C: 7C6BEA2E  lhzx r3, r11, r29
	ctx.r[3].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82FDC250: 4BFF5339  bl 0x82fd1588
	ctx.lr = 0x82FDC254;
	sub_82FD1588(ctx, base);
	// 82FDC254: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDC258: 4082FF18  bne 0x82fdc170
	if !ctx.cr[0].eq {
	pc = 0x82FDC170; continue 'dispatch;
	}
	// 82FDC25C: 397F0001  addi r11, r31, 1
	ctx.r[11].s64 = ctx.r[31].s64 + 1;
	// 82FDC260: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FDC264: 7C6BEA2E  lhzx r3, r11, r29
	ctx.r[3].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82FDC268: 4BFF5321  bl 0x82fd1588
	ctx.lr = 0x82FDC26C;
	sub_82FD1588(ctx, base);
	// 82FDC26C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDC270: 41820014  beq 0x82fdc284
	if ctx.cr[0].eq {
	pc = 0x82FDC284; continue 'dispatch;
	}
	// 82FDC274: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82FDC278: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FDC27C: 4BFFE965  bl 0x82fdabe0
	ctx.lr = 0x82FDC280;
	sub_82FDABE0(ctx, base);
	// 82FDC280: 4BFFFEF4  b 0x82fdc174
	pc = 0x82FDC174; continue 'dispatch;
	// 82FDC284: 2F1B00FF  cmpwi cr6, r27, 0xff
	ctx.cr[6].compare_i32(ctx.r[27].s32, 255, &mut ctx.xer);
	// 82FDC288: 4199FEE8  bgt cr6, 0x82fdc170
	if ctx.cr[6].gt {
	pc = 0x82FDC170; continue 'dispatch;
	}
	// 82FDC28C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82FDC290: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82FDC294: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 82FDC298: 40990080  ble cr6, 0x82fdc318
	if !ctx.cr[6].gt {
	pc = 0x82FDC318; continue 'dispatch;
	}
	// 82FDC29C: A07D0000  lhz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDC2A0: 2B03002E  cmplwi cr6, r3, 0x2e
	ctx.cr[6].compare_u32(ctx.r[3].u32, 46 as u32, &mut ctx.xer);
	// 82FDC2A4: 409A0040  bne cr6, 0x82fdc2e4
	if !ctx.cr[6].eq {
	pc = 0x82FDC2E4; continue 'dispatch;
	}
	// 82FDC2A8: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82FDC2AC: 40990014  ble cr6, 0x82fdc2c0
	if !ctx.cr[6].gt {
	pc = 0x82FDC2C0; continue 'dispatch;
	}
	// 82FDC2B0: A07DFFFE  lhz r3, -2(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(-2 as u32) ) } as u64;
	// 82FDC2B4: 4BFF52F5  bl 0x82fd15a8
	ctx.lr = 0x82FDC2B8;
	sub_82FD15A8(ctx, base);
	// 82FDC2B8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDC2BC: 4182FEB4  beq 0x82fdc170
	if ctx.cr[0].eq {
	pc = 0x82FDC170; continue 'dispatch;
	}
	// 82FDC2C0: 397F0001  addi r11, r31, 1
	ctx.r[11].s64 = ctx.r[31].s64 + 1;
	// 82FDC2C4: 7F0BD800  cmpw cr6, r11, r27
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[27].s32, &mut ctx.xer);
	// 82FDC2C8: 40980014  bge cr6, 0x82fdc2dc
	if !ctx.cr[6].lt {
	pc = 0x82FDC2DC; continue 'dispatch;
	}
	// 82FDC2CC: A07D0002  lhz r3, 2(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(2 as u32) ) } as u64;
	// 82FDC2D0: 4BFF52D9  bl 0x82fd15a8
	ctx.lr = 0x82FDC2D4;
	sub_82FD15A8(ctx, base);
	// 82FDC2D4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDC2D8: 4182FE98  beq 0x82fdc170
	if ctx.cr[0].eq {
	pc = 0x82FDC170; continue 'dispatch;
	}
	// 82FDC2DC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82FDC2E0: 48000028  b 0x82fdc308
	pc = 0x82FDC308; continue 'dispatch;
	// 82FDC2E4: 4BFF52C5  bl 0x82fd15a8
	ctx.lr = 0x82FDC2E8;
	sub_82FD15A8(ctx, base);
	// 82FDC2E8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDC2EC: 40820010  bne 0x82fdc2fc
	if !ctx.cr[0].eq {
	pc = 0x82FDC2FC; continue 'dispatch;
	}
	// 82FDC2F0: A17D0000  lhz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDC2F4: 2B0B002D  cmplwi cr6, r11, 0x2d
	ctx.cr[6].compare_u32(ctx.r[11].u32, 45 as u32, &mut ctx.xer);
	// 82FDC2F8: 409AFE78  bne cr6, 0x82fdc170
	if !ctx.cr[6].eq {
	pc = 0x82FDC170; continue 'dispatch;
	}
	// 82FDC2FC: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82FDC300: 2B1E003F  cmplwi cr6, r30, 0x3f
	ctx.cr[6].compare_u32(ctx.r[30].u32, 63 as u32, &mut ctx.xer);
	// 82FDC304: 4199FE6C  bgt cr6, 0x82fdc170
	if ctx.cr[6].gt {
	pc = 0x82FDC170; continue 'dispatch;
	}
	// 82FDC308: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82FDC30C: 3BBD0002  addi r29, r29, 2
	ctx.r[29].s64 = ctx.r[29].s64 + 2;
	// 82FDC310: 7F1FD800  cmpw cr6, r31, r27
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[27].s32, &mut ctx.xer);
	// 82FDC314: 4198FF88  blt cr6, 0x82fdc29c
	if ctx.cr[6].lt {
	pc = 0x82FDC29C; continue 'dispatch;
	}
	// 82FDC318: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82FDC31C: 4BFFFE58  b 0x82fdc174
	pc = 0x82FDC174; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDC320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDC320 size=424
    let mut pc: u32 = 0x82FDC320;
    'dispatch: loop {
        match pc {
            0x82FDC320 => {
    //   block [0x82FDC320..0x82FDC4C8)
	// 82FDC320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDC324: 481CBE3D  bl 0x831a8160
	ctx.lr = 0x82FDC328;
	sub_831A8130(ctx, base);
	// 82FDC328: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDC32C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FDC330: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 82FDC334: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82FDC338: 4BFF5A79  bl 0x82fd1db0
	ctx.lr = 0x82FDC33C;
	sub_82FD1DB0(ctx, base);
	// 82FDC33C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82FDC340: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82FDC344: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82FDC348: 419A001C  beq cr6, 0x82fdc364
	if ctx.cr[6].eq {
	pc = 0x82FDC364; continue 'dispatch;
	}
	// 82FDC34C: 7F03F000  cmpw cr6, r3, r30
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82FDC350: 40980014  bge cr6, 0x82fdc364
	if !ctx.cr[6].lt {
	pc = 0x82FDC364; continue 'dispatch;
	}
	// 82FDC354: 7F9BE378  mr r27, r28
	ctx.r[27].u64 = ctx.r[28].u64;
	// 82FDC358: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82FDC35C: 3BE30001  addi r31, r3, 1
	ctx.r[31].s64 = ctx.r[3].s64 + 1;
	// 82FDC360: 4800000C  b 0x82fdc36c
	pc = 0x82FDC36C; continue 'dispatch;
	// 82FDC364: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FDC368: 3B6B8158  addi r27, r11, -0x7ea8
	ctx.r[27].s64 = ctx.r[11].s64 + -32424;
	// 82FDC36C: 7F1FF000  cmpw cr6, r31, r30
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82FDC370: 40980060  bge cr6, 0x82fdc3d0
	if !ctx.cr[6].lt {
	pc = 0x82FDC3D0; continue 'dispatch;
	}
	// 82FDC374: 57EB083C  slwi r11, r31, 1
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FDC378: 7FABE214  add r29, r11, r28
	ctx.r[29].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 82FDC37C: A17D0000  lhz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDC380: 2B0B005B  cmplwi cr6, r11, 0x5b
	ctx.cr[6].compare_u32(ctx.r[11].u32, 91 as u32, &mut ctx.xer);
	// 82FDC384: 409A004C  bne cr6, 0x82fdc3d0
	if !ctx.cr[6].eq {
	pc = 0x82FDC3D0; continue 'dispatch;
	}
	// 82FDC388: 3880005D  li r4, 0x5d
	ctx.r[4].s64 = 93;
	// 82FDC38C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FDC390: 4BFF5A21  bl 0x82fd1db0
	ctx.lr = 0x82FDC394;
	sub_82FD1DB0(ctx, base);
	// 82FDC394: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82FDC398: 419A0070  beq cr6, 0x82fdc408
	if ctx.cr[6].eq {
	pc = 0x82FDC408; continue 'dispatch;
	}
	// 82FDC39C: 7F03F000  cmpw cr6, r3, r30
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82FDC3A0: 40980050  bge cr6, 0x82fdc3f0
	if !ctx.cr[6].lt {
	pc = 0x82FDC3F0; continue 'dispatch;
	}
	// 82FDC3A4: 7D7F1A14  add r11, r31, r3
	ctx.r[11].u64 = ctx.r[31].u64 + ctx.r[3].u64;
	// 82FDC3A8: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 82FDC3AC: 7F0AF000  cmpw cr6, r10, r30
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82FDC3B0: 4098003C  bge cr6, 0x82fdc3ec
	if !ctx.cr[6].lt {
	pc = 0x82FDC3EC; continue 'dispatch;
	}
	// 82FDC3B4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FDC3B8: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FDC3BC: 7D6BE22E  lhzx r11, r11, r28
	ctx.r[11].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82FDC3C0: 2B0B003A  cmplwi cr6, r11, 0x3a
	ctx.cr[6].compare_u32(ctx.r[11].u32, 58 as u32, &mut ctx.xer);
	// 82FDC3C4: 409A0028  bne cr6, 0x82fdc3ec
	if !ctx.cr[6].eq {
	pc = 0x82FDC3EC; continue 'dispatch;
	}
	// 82FDC3C8: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 82FDC3CC: 48000024  b 0x82fdc3f0
	pc = 0x82FDC3F0; continue 'dispatch;
	// 82FDC3D0: 57EB083C  slwi r11, r31, 1
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FDC3D4: 3880003A  li r4, 0x3a
	ctx.r[4].s64 = 58;
	// 82FDC3D8: 7FABE214  add r29, r11, r28
	ctx.r[29].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 82FDC3DC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FDC3E0: 4BFF59D1  bl 0x82fd1db0
	ctx.lr = 0x82FDC3E4;
	sub_82FD1DB0(ctx, base);
	// 82FDC3E4: 7F03F000  cmpw cr6, r3, r30
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82FDC3E8: 41980008  blt cr6, 0x82fdc3f0
	if ctx.cr[6].lt {
	pc = 0x82FDC3F0; continue 'dispatch;
	}
	// 82FDC3EC: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82FDC3F0: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82FDC3F4: 419A0014  beq cr6, 0x82fdc408
	if ctx.cr[6].eq {
	pc = 0x82FDC408; continue 'dispatch;
	}
	// 82FDC3F8: 7D7F1A14  add r11, r31, r3
	ctx.r[11].u64 = ctx.r[31].u64 + ctx.r[3].u64;
	// 82FDC3FC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FDC400: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FDC404: 4800000C  b 0x82fdc410
	pc = 0x82FDC410; continue 'dispatch;
	// 82FDC408: 7C9FF050  subf r4, r31, r30
	ctx.r[4].s64 = ctx.r[30].s64 - ctx.r[31].s64;
	// 82FDC40C: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82FDC410: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82FDC414: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82FDC418: 419A0068  beq cr6, 0x82fdc480
	if ctx.cr[6].eq {
	pc = 0x82FDC480; continue 'dispatch;
	}
	// 82FDC41C: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82FDC420: 419A0060  beq cr6, 0x82fdc480
	if ctx.cr[6].eq {
	pc = 0x82FDC480; continue 'dispatch;
	}
	// 82FDC424: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82FDC428: 40980058  bge cr6, 0x82fdc480
	if !ctx.cr[6].lt {
	pc = 0x82FDC480; continue 'dispatch;
	}
	// 82FDC42C: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FDC430: 7D4AE214  add r10, r10, r28
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[28].u64;
	// 82FDC434: A12A0000  lhz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDC438: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDC43C: 41820044  beq 0x82fdc480
	if ctx.cr[0].eq {
	pc = 0x82FDC480; continue 'dispatch;
	}
	// 82FDC440: 7D0BF051  subf. r8, r11, r30
	ctx.r[8].s64 = ctx.r[30].s64 - ctx.r[11].s64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82FDC444: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FDC448: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FDC44C: 40810034  ble 0x82fdc480
	if !ctx.cr[0].gt {
	pc = 0x82FDC480; continue 'dispatch;
	}
	// 82FDC450: A0EA0000  lhz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDC454: 2B070030  cmplwi cr6, r7, 0x30
	ctx.cr[6].compare_u32(ctx.r[7].u32, 48 as u32, &mut ctx.xer);
	// 82FDC458: 41980068  blt cr6, 0x82fdc4c0
	if ctx.cr[6].lt {
	pc = 0x82FDC4C0; continue 'dispatch;
	}
	// 82FDC45C: 2B070039  cmplwi cr6, r7, 0x39
	ctx.cr[6].compare_u32(ctx.r[7].u32, 57 as u32, &mut ctx.xer);
	// 82FDC460: 41990060  bgt cr6, 0x82fdc4c0
	if ctx.cr[6].gt {
	pc = 0x82FDC4C0; continue 'dispatch;
	}
	// 82FDC464: 1D25000A  mulli r9, r5, 0xa
	ctx.r[9].s64 = ctx.r[5].s64 * 10;
	// 82FDC468: 7D293A14  add r9, r9, r7
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[7].u64;
	// 82FDC46C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FDC470: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 82FDC474: 38A9FFD0  addi r5, r9, -0x30
	ctx.r[5].s64 = ctx.r[9].s64 + -48;
	// 82FDC478: 7F0B4000  cmpw cr6, r11, r8
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82FDC47C: 4198FFD4  blt cr6, 0x82fdc450
	if ctx.cr[6].lt {
	pc = 0x82FDC450; continue 'dispatch;
	}
	// 82FDC480: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 82FDC484: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82FDC488: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FDC48C: 4BFFFB75  bl 0x82fdc000
	ctx.lr = 0x82FDC490;
	sub_82FDC000(ctx, base);
	// 82FDC490: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDC494: 4082001C  bne 0x82fdc4b0
	if !ctx.cr[0].eq {
	pc = 0x82FDC4B0; continue 'dispatch;
	}
	// 82FDC498: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FDC49C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FDC4A0: 4BFFE529  bl 0x82fda9c8
	ctx.lr = 0x82FDC4A4;
	sub_82FDA9C8(ctx, base);
	// 82FDC4A4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDC4A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FDC4AC: 41820008  beq 0x82fdc4b4
	if ctx.cr[0].eq {
	pc = 0x82FDC4B4; continue 'dispatch;
	}
	// 82FDC4B0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82FDC4B4: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82FDC4B8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82FDC4BC: 481CBCF4  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
	// 82FDC4C0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FDC4C4: 4BFFFFF4  b 0x82fdc4b8
	pc = 0x82FDC4B8; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDC4C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDC4C8 size=288
    let mut pc: u32 = 0x82FDC4C8;
    'dispatch: loop {
        match pc {
            0x82FDC4C8 => {
    //   block [0x82FDC4C8..0x82FDC5E8)
	// 82FDC4C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDC4CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FDC4D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FDC4D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FDC4D8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDC4DC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FDC4E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FDC4E4: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82FDC4E8: 409A0044  bne cr6, 0x82fdc52c
	if !ctx.cr[6].eq {
	pc = 0x82FDC52C; continue 'dispatch;
	}
	// 82FDC4EC: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FDC4F0: 2C040000  cmpwi r4, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82FDC4F4: 41820018  beq 0x82fdc50c
	if ctx.cr[0].eq {
	pc = 0x82FDC50C; continue 'dispatch;
	}
	// 82FDC4F8: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDC4FC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDC500: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FDC504: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDC508: 4E800421  bctrl
	ctx.lr = 0x82FDC50C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDC50C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FDC510: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FDC514: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FDC518: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82FDC51C: 4BFFF4AD  bl 0x82fdb9c8
	ctx.lr = 0x82FDC520;
	sub_82FDB9C8(ctx, base);
	// 82FDC520: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82FDC524: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82FDC528: 480000A8  b 0x82fdc5d0
	pc = 0x82FDC5D0; continue 'dispatch;
	// 82FDC52C: A17E0000  lhz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDC530: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDC534: 4182005C  beq 0x82fdc590
	if ctx.cr[0].eq {
	pc = 0x82FDC590; continue 'dispatch;
	}
	// 82FDC538: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FDC53C: 809F0028  lwz r4, 0x28(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDC540: 4BFFFBA9  bl 0x82fdc0e8
	ctx.lr = 0x82FDC544;
	sub_82FDC0E8(ctx, base);
	// 82FDC544: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDC548: 40820048  bne 0x82fdc590
	if !ctx.cr[0].eq {
	pc = 0x82FDC590; continue 'dispatch;
	}
	// 82FDC54C: 80DF0028  lwz r6, 0x28(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDC550: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FDC554: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FDC558: 38EB9C44  addi r7, r11, -0x63bc
	ctx.r[7].s64 = ctx.r[11].s64 + -25532;
	// 82FDC55C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FDC560: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82FDC564: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 82FDC568: 388B9D88  addi r4, r11, -0x6278
	ctx.r[4].s64 = ctx.r[11].s64 + -25208;
	// 82FDC56C: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 82FDC570: 38C0010F  li r6, 0x10f
	ctx.r[6].s64 = 271;
	// 82FDC574: 38A004A8  li r5, 0x4a8
	ctx.r[5].s64 = 1192;
	// 82FDC578: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82FDC57C: 4BFFD655  bl 0x82fd9bd0
	ctx.lr = 0x82FDC580;
	sub_82FD9BD0(ctx, base);
	// 82FDC580: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FDC584: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82FDC588: 388BC5E4  addi r4, r11, -0x3a1c
	ctx.r[4].s64 = ctx.r[11].s64 + -14876;
	// 82FDC58C: 481D469D  bl 0x831b0c28
	ctx.lr = 0x82FDC590;
	sub_831B0C28(ctx, base);
	// 82FDC590: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FDC594: 2C040000  cmpwi r4, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82FDC598: 41820018  beq 0x82fdc5b0
	if ctx.cr[0].eq {
	pc = 0x82FDC5B0; continue 'dispatch;
	}
	// 82FDC59C: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDC5A0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDC5A4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FDC5A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDC5AC: 4E800421  bctrl
	ctx.lr = 0x82FDC5B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDC5B0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FDC5B4: 809F0028  lwz r4, 0x28(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDC5B8: 4BFF45C9  bl 0x82fd0b80
	ctx.lr = 0x82FDC5BC;
	sub_82FD0B80(ctx, base);
	// 82FDC5BC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82FDC5C0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FDC5C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FDC5C8: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82FDC5CC: 4800001D  bl 0x82fdc5e8
	ctx.lr = 0x82FDC5D0;
	sub_82FDC5E8(ctx, base);
	// 82FDC5D0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82FDC5D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FDC5D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FDC5DC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FDC5E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FDC5E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDC5E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDC5E8 size=264
    let mut pc: u32 = 0x82FDC5E8;
    'dispatch: loop {
        match pc {
            0x82FDC5E8 => {
    //   block [0x82FDC5E8..0x82FDC6F0)
	// 82FDC5E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDC5EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FDC5F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FDC5F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FDC5F8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDC5FC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FDC600: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FDC604: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82FDC608: 409A0044  bne cr6, 0x82fdc64c
	if !ctx.cr[6].eq {
	pc = 0x82FDC64C; continue 'dispatch;
	}
	// 82FDC60C: 809F0014  lwz r4, 0x14(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FDC610: 2C040000  cmpwi r4, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82FDC614: 41820018  beq 0x82fdc62c
	if ctx.cr[0].eq {
	pc = 0x82FDC62C; continue 'dispatch;
	}
	// 82FDC618: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDC61C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDC620: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FDC624: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDC628: 4E800421  bctrl
	ctx.lr = 0x82FDC62C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDC62C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FDC630: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82FDC634: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82FDC638: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FDC63C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FDC640: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FDC644: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FDC648: 4E800020  blr
	return;
	// 82FDC64C: A17E0000  lhz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDC650: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDC654: 41820058  beq 0x82fdc6ac
	if ctx.cr[0].eq {
	pc = 0x82FDC6AC; continue 'dispatch;
	}
	// 82FDC658: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FDC65C: 4BFFE425  bl 0x82fdaa80
	ctx.lr = 0x82FDC660;
	sub_82FDAA80(ctx, base);
	// 82FDC660: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDC664: 41820048  beq 0x82fdc6ac
	if ctx.cr[0].eq {
	pc = 0x82FDC6AC; continue 'dispatch;
	}
	// 82FDC668: 809F0014  lwz r4, 0x14(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FDC66C: 2C040000  cmpwi r4, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82FDC670: 41820018  beq 0x82fdc688
	if ctx.cr[0].eq {
	pc = 0x82FDC688; continue 'dispatch;
	}
	// 82FDC674: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDC678: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDC67C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FDC680: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDC684: 4E800421  bctrl
	ctx.lr = 0x82FDC688;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDC688: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FDC68C: 809F0028  lwz r4, 0x28(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDC690: 4BFF44F1  bl 0x82fd0b80
	ctx.lr = 0x82FDC694;
	sub_82FD0B80(ctx, base);
	// 82FDC694: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82FDC698: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FDC69C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FDC6A0: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82FDC6A4: 4BFFFE25  bl 0x82fdc4c8
	ctx.lr = 0x82FDC6A8;
	sub_82FDC4C8(ctx, base);
	// 82FDC6A8: 4BFFFF8C  b 0x82fdc634
	pc = 0x82FDC634; continue 'dispatch;
	// 82FDC6AC: 80DF0028  lwz r6, 0x28(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDC6B0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FDC6B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FDC6B8: 38EB9C5C  addi r7, r11, -0x63a4
	ctx.r[7].s64 = ctx.r[11].s64 + -25508;
	// 82FDC6BC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FDC6C0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82FDC6C4: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 82FDC6C8: 388B9D88  addi r4, r11, -0x6278
	ctx.r[4].s64 = ctx.r[11].s64 + -25208;
	// 82FDC6CC: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 82FDC6D0: 38C0010F  li r6, 0x10f
	ctx.r[6].s64 = 271;
	// 82FDC6D4: 38A004E2  li r5, 0x4e2
	ctx.r[5].s64 = 1250;
	// 82FDC6D8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82FDC6DC: 4BFFD4F5  bl 0x82fd9bd0
	ctx.lr = 0x82FDC6E0;
	sub_82FD9BD0(ctx, base);
	// 82FDC6E0: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FDC6E4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82FDC6E8: 388BC5E4  addi r4, r11, -0x3a1c
	ctx.r[4].s64 = ctx.r[11].s64 + -14876;
	// 82FDC6EC: 481D453D  bl 0x831b0c28
	ctx.lr = 0x82FDC6F0;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDC6F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDC6F0 size=248
    let mut pc: u32 = 0x82FDC6F0;
    'dispatch: loop {
        match pc {
            0x82FDC6F0 => {
    //   block [0x82FDC6F0..0x82FDC7E8)
	// 82FDC6F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDC6F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FDC6F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FDC6FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FDC700: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDC704: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FDC708: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 82FDC70C: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82FDC710: 4BFFF9D9  bl 0x82fdc0e8
	ctx.lr = 0x82FDC714;
	sub_82FDC0E8(ctx, base);
	// 82FDC714: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDC718: 418200B4  beq 0x82fdc7cc
	if ctx.cr[0].eq {
	pc = 0x82FDC7CC; continue 'dispatch;
	}
	// 82FDC71C: 3D600000  lis r11, 0
	ctx.r[11].s64 = 0;
	// 82FDC720: 616BFFFF  ori r11, r11, 0xffff
	ctx.r[11].u64 = ctx.r[11].u64 | 65535;
	// 82FDC724: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82FDC728: 419900A4  bgt cr6, 0x82fdc7cc
	if ctx.cr[6].gt {
	pc = 0x82FDC7CC; continue 'dispatch;
	}
	// 82FDC72C: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FDC730: 4098000C  bge cr6, 0x82fdc73c
	if !ctx.cr[6].lt {
	pc = 0x82FDC73C; continue 'dispatch;
	}
	// 82FDC734: 2F1EFFFF  cmpwi cr6, r30, -1
	ctx.cr[6].compare_i32(ctx.r[30].s32, -1, &mut ctx.xer);
	// 82FDC738: 409A0094  bne cr6, 0x82fdc7cc
	if !ctx.cr[6].eq {
	pc = 0x82FDC7CC; continue 'dispatch;
	}
	// 82FDC73C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82FDC740: 409A000C  bne cr6, 0x82fdc74c
	if !ctx.cr[6].eq {
	pc = 0x82FDC74C; continue 'dispatch;
	}
	// 82FDC744: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82FDC748: 48000088  b 0x82fdc7d0
	pc = 0x82FDC7D0; continue 'dispatch;
	// 82FDC74C: A17F0000  lhz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDC750: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDC754: 4182FFF0  beq 0x82fdc744
	if ctx.cr[0].eq {
	pc = 0x82FDC744; continue 'dispatch;
	}
	// 82FDC758: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FDC75C: 3BCB9BD0  addi r30, r11, -0x6430
	ctx.r[30].s64 = ctx.r[11].s64 + -25648;
	// 82FDC760: A07F0000  lhz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDC764: 4BFFD755  bl 0x82fd9eb8
	ctx.lr = 0x82FDC768;
	sub_82FD9EB8(ctx, base);
	// 82FDC768: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDC76C: 4082004C  bne 0x82fdc7b8
	if !ctx.cr[0].eq {
	pc = 0x82FDC7B8; continue 'dispatch;
	}
	// 82FDC770: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FDC774: A09F0000  lhz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDC778: 4BFF5639  bl 0x82fd1db0
	ctx.lr = 0x82FDC77C;
	sub_82FD1DB0(ctx, base);
	// 82FDC77C: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82FDC780: 409A0038  bne cr6, 0x82fdc7b8
	if !ctx.cr[6].eq {
	pc = 0x82FDC7B8; continue 'dispatch;
	}
	// 82FDC784: A17F0000  lhz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDC788: 2B0B0025  cmplwi cr6, r11, 0x25
	ctx.cr[6].compare_u32(ctx.r[11].u32, 37 as u32, &mut ctx.xer);
	// 82FDC78C: 409A0040  bne cr6, 0x82fdc7cc
	if !ctx.cr[6].eq {
	pc = 0x82FDC7CC; continue 'dispatch;
	}
	// 82FDC790: A07F0002  lhz r3, 2(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 82FDC794: 4BFF4E7D  bl 0x82fd1610
	ctx.lr = 0x82FDC798;
	sub_82FD1610(ctx, base);
	// 82FDC798: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDC79C: 41820030  beq 0x82fdc7cc
	if ctx.cr[0].eq {
	pc = 0x82FDC7CC; continue 'dispatch;
	}
	// 82FDC7A0: A07F0004  lhz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDC7A4: 4BFF4E6D  bl 0x82fd1610
	ctx.lr = 0x82FDC7A8;
	sub_82FD1610(ctx, base);
	// 82FDC7A8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDC7AC: 41820020  beq 0x82fdc7cc
	if ctx.cr[0].eq {
	pc = 0x82FDC7CC; continue 'dispatch;
	}
	// 82FDC7B0: 3BFF0006  addi r31, r31, 6
	ctx.r[31].s64 = ctx.r[31].s64 + 6;
	// 82FDC7B4: 48000008  b 0x82fdc7bc
	pc = 0x82FDC7BC; continue 'dispatch;
	// 82FDC7B8: 3BFF0002  addi r31, r31, 2
	ctx.r[31].s64 = ctx.r[31].s64 + 2;
	// 82FDC7BC: A17F0000  lhz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDC7C0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDC7C4: 4082FF9C  bne 0x82fdc760
	if !ctx.cr[0].eq {
	pc = 0x82FDC760; continue 'dispatch;
	}
	// 82FDC7C8: 4BFFFF7C  b 0x82fdc744
	pc = 0x82FDC744; continue 'dispatch;
	// 82FDC7CC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FDC7D0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FDC7D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FDC7D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FDC7DC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FDC7E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FDC7E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDC7E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDC7E8 size=600
    let mut pc: u32 = 0x82FDC7E8;
    'dispatch: loop {
        match pc {
            0x82FDC7E8 => {
    //   block [0x82FDC7E8..0x82FDCA40)
	// 82FDC7E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDC7EC: 481CB96D  bl 0x831a8158
	ctx.lr = 0x82FDC7F0;
	sub_831A8130(ctx, base);
	// 82FDC7F0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDC7F4: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82FDC7F8: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82FDC7FC: 394B2DD8  addi r10, r11, 0x2dd8
	ctx.r[10].s64 = ctx.r[11].s64 + 11736;
	// 82FDC800: A1640000  lhz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDC804: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FDC808: 4800000C  b 0x82fdc814
	pc = 0x82FDC814; continue 'dispatch;
	// 82FDC80C: 3BDE0002  addi r30, r30, 2
	ctx.r[30].s64 = ctx.r[30].s64 + 2;
	// 82FDC810: A17E0000  lhz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDC814: 7D6B50AE  lbzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82FDC818: 556B0031  rlwinm. r11, r11, 0, 0, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDC81C: 4082FFF0  bne 0x82fdc80c
	if !ctx.cr[0].eq {
	pc = 0x82FDC80C; continue 'dispatch;
	}
	// 82FDC820: A17E0000  lhz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDC824: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDC828: 41820028  beq 0x82fdc850
	if ctx.cr[0].eq {
	pc = 0x82FDC850; continue 'dispatch;
	}
	// 82FDC82C: 397E0002  addi r11, r30, 2
	ctx.r[11].s64 = ctx.r[30].s64 + 2;
	// 82FDC830: 48000008  b 0x82fdc838
	pc = 0x82FDC838; continue 'dispatch;
	// 82FDC834: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FDC838: A12B0000  lhz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDC83C: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDC840: 4082FFF4  bne 0x82fdc834
	if !ctx.cr[0].eq {
	pc = 0x82FDC834; continue 'dispatch;
	}
	// 82FDC844: 7D7E5850  subf r11, r30, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	// 82FDC848: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82FDC84C: 48000008  b 0x82fdc854
	pc = 0x82FDC854; continue 'dispatch;
	// 82FDC850: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FDC854: 7D7A5B78  mr r26, r11
	ctx.r[26].u64 = ctx.r[11].u64;
	// 82FDC858: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDC85C: 419A0034  beq cr6, 0x82fdc890
	if ctx.cr[6].eq {
	pc = 0x82FDC890; continue 'dispatch;
	}
	// 82FDC860: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FDC864: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82FDC868: 396BFFFE  addi r11, r11, -2
	ctx.r[11].s64 = ctx.r[11].s64 + -2;
	// 82FDC86C: A12B0000  lhz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDC870: 7D2950AE  lbzx r9, r9, r10
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82FDC874: 55290031  rlwinm. r9, r9, 0, 0, 0x18
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82FDC878: 41820010  beq 0x82fdc888
	if ctx.cr[0].eq {
	pc = 0x82FDC888; continue 'dispatch;
	}
	// 82FDC87C: 375AFFFF  addic. r26, r26, -1
	ctx.xer.ca = (ctx.r[26].u32 > (!(-1 as u32)));
	ctx.r[26].s64 = ctx.r[26].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 82FDC880: 396BFFFE  addi r11, r11, -2
	ctx.r[11].s64 = ctx.r[11].s64 + -2;
	// 82FDC884: 4082FFE8  bne 0x82fdc86c
	if !ctx.cr[0].eq {
	pc = 0x82FDC86C; continue 'dispatch;
	}
	// 82FDC888: 2F1A0000  cmpwi cr6, r26, 0
	ctx.cr[6].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 82FDC88C: 409A0018  bne cr6, 0x82fdc8a4
	if !ctx.cr[6].eq {
	pc = 0x82FDC8A4; continue 'dispatch;
	}
	// 82FDC890: 570B063E  clrlwi r11, r24, 0x18
	ctx.r[11].u64 = ctx.r[24].u32 as u64 & 0x000000FFu64;
	// 82FDC894: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82FDC898: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82FDC89C: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 82FDC8A0: 48000198  b 0x82fdca38
	pc = 0x82FDCA38; continue 'dispatch;
	// 82FDC8A4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82FDC8A8: 3880003A  li r4, 0x3a
	ctx.r[4].s64 = 58;
	// 82FDC8AC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FDC8B0: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 82FDC8B4: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82FDC8B8: 4BFF54F9  bl 0x82fd1db0
	ctx.lr = 0x82FDC8BC;
	sub_82FD1DB0(ctx, base);
	// 82FDC8BC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FDC8C0: 3880002F  li r4, 0x2f
	ctx.r[4].s64 = 47;
	// 82FDC8C4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FDC8C8: 4BFF54E9  bl 0x82fd1db0
	ctx.lr = 0x82FDC8CC;
	sub_82FD1DB0(ctx, base);
	// 82FDC8CC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82FDC8D0: 3880003F  li r4, 0x3f
	ctx.r[4].s64 = 63;
	// 82FDC8D4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FDC8D8: 4BFF54D9  bl 0x82fd1db0
	ctx.lr = 0x82FDC8DC;
	sub_82FD1DB0(ctx, base);
	// 82FDC8DC: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82FDC8E0: 38800023  li r4, 0x23
	ctx.r[4].s64 = 35;
	// 82FDC8E4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FDC8E8: 4BFF54C9  bl 0x82fd1db0
	ctx.lr = 0x82FDC8EC;
	sub_82FD1DB0(ctx, base);
	// 82FDC8EC: 2F1D0002  cmpwi cr6, r29, 2
	ctx.cr[6].compare_i32(ctx.r[29].s32, 2, &mut ctx.xer);
	// 82FDC8F0: 41980058  blt cr6, 0x82fdc948
	if ctx.cr[6].lt {
	pc = 0x82FDC948; continue 'dispatch;
	}
	// 82FDC8F4: 7F1DE000  cmpw cr6, r29, r28
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[28].s32, &mut ctx.xer);
	// 82FDC8F8: 4099000C  ble cr6, 0x82fdc904
	if !ctx.cr[6].gt {
	pc = 0x82FDC904; continue 'dispatch;
	}
	// 82FDC8FC: 2F1CFFFF  cmpwi cr6, r28, -1
	ctx.cr[6].compare_i32(ctx.r[28].s32, -1, &mut ctx.xer);
	// 82FDC900: 409A0048  bne cr6, 0x82fdc948
	if !ctx.cr[6].eq {
	pc = 0x82FDC948; continue 'dispatch;
	}
	// 82FDC904: 7F1DD800  cmpw cr6, r29, r27
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[27].s32, &mut ctx.xer);
	// 82FDC908: 4099000C  ble cr6, 0x82fdc914
	if !ctx.cr[6].gt {
	pc = 0x82FDC914; continue 'dispatch;
	}
	// 82FDC90C: 2F1BFFFF  cmpwi cr6, r27, -1
	ctx.cr[6].compare_i32(ctx.r[27].s32, -1, &mut ctx.xer);
	// 82FDC910: 409A0038  bne cr6, 0x82fdc948
	if !ctx.cr[6].eq {
	pc = 0x82FDC948; continue 'dispatch;
	}
	// 82FDC914: 7F1D1800  cmpw cr6, r29, r3
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[3].s32, &mut ctx.xer);
	// 82FDC918: 4099000C  ble cr6, 0x82fdc924
	if !ctx.cr[6].gt {
	pc = 0x82FDC924; continue 'dispatch;
	}
	// 82FDC91C: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82FDC920: 409A0028  bne cr6, 0x82fdc948
	if !ctx.cr[6].eq {
	pc = 0x82FDC948; continue 'dispatch;
	}
	// 82FDC924: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82FDC928: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FDC92C: 4BFFF475  bl 0x82fdbda0
	ctx.lr = 0x82FDC930;
	sub_82FDBDA0(ctx, base);
	// 82FDC930: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDC934: 41820100  beq 0x82fdca34
	if ctx.cr[0].eq {
	pc = 0x82FDCA34; continue 'dispatch;
	}
	// 82FDC938: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FDC93C: 3B200001  li r25, 1
	ctx.r[25].s64 = 1;
	// 82FDC940: 3BEB0001  addi r31, r11, 1
	ctx.r[31].s64 = ctx.r[11].s64 + 1;
	// 82FDC944: 4800001C  b 0x82fdc960
	pc = 0x82FDC960; continue 'dispatch;
	// 82FDC948: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82FDC94C: 419A00E8  beq cr6, 0x82fdca34
	if ctx.cr[6].eq {
	pc = 0x82FDCA34; continue 'dispatch;
	}
	// 82FDC950: 570B063F  clrlwi. r11, r24, 0x18
	ctx.r[11].u64 = ctx.r[24].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDC954: 4082000C  bne 0x82fdc960
	if !ctx.cr[0].eq {
	pc = 0x82FDC960; continue 'dispatch;
	}
	// 82FDC958: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82FDC95C: 409A00D8  bne cr6, 0x82fdca34
	if !ctx.cr[6].eq {
	pc = 0x82FDCA34; continue 'dispatch;
	}
	// 82FDC960: 7F1FD000  cmpw cr6, r31, r26
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[26].s32, &mut ctx.xer);
	// 82FDC964: 419A00D0  beq cr6, 0x82fdca34
	if ctx.cr[6].eq {
	pc = 0x82FDCA34; continue 'dispatch;
	}
	// 82FDC968: 572B063F  clrlwi. r11, r25, 0x18
	ctx.r[11].u64 = ctx.r[25].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDC96C: 41820014  beq 0x82fdc980
	if ctx.cr[0].eq {
	pc = 0x82FDC980; continue 'dispatch;
	}
	// 82FDC970: 57EB083C  slwi r11, r31, 1
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FDC974: 7D6BF22E  lhzx r11, r11, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82FDC978: 2B0B0023  cmplwi cr6, r11, 0x23
	ctx.cr[6].compare_u32(ctx.r[11].u32, 35 as u32, &mut ctx.xer);
	// 82FDC97C: 419A00B8  beq cr6, 0x82fdca34
	if ctx.cr[6].eq {
	pc = 0x82FDCA34; continue 'dispatch;
	}
	// 82FDC980: 397F0001  addi r11, r31, 1
	ctx.r[11].s64 = ctx.r[31].s64 + 1;
	// 82FDC984: 7F0BD000  cmpw cr6, r11, r26
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[26].s32, &mut ctx.xer);
	// 82FDC988: 40980080  bge cr6, 0x82fdca08
	if !ctx.cr[6].lt {
	pc = 0x82FDCA08; continue 'dispatch;
	}
	// 82FDC98C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FDC990: 388B9C98  addi r4, r11, -0x6368
	ctx.r[4].s64 = ctx.r[11].s64 + -25448;
	// 82FDC994: 57EB083C  slwi r11, r31, 1
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FDC998: 7C6BF214  add r3, r11, r30
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82FDC99C: 4BFFD3D5  bl 0x82fd9d70
	ctx.lr = 0x82FDC9A0;
	sub_82FD9D70(ctx, base);
	// 82FDC9A0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDC9A4: 41820064  beq 0x82fdca08
	if ctx.cr[0].eq {
	pc = 0x82FDCA08; continue 'dispatch;
	}
	// 82FDC9A8: 3BFF0002  addi r31, r31, 2
	ctx.r[31].s64 = ctx.r[31].s64 + 2;
	// 82FDC9AC: 7FE8FB78  mr r8, r31
	ctx.r[8].u64 = ctx.r[31].u64;
	// 82FDC9B0: 7F1FD000  cmpw cr6, r31, r26
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[26].s32, &mut ctx.xer);
	// 82FDC9B4: 40980078  bge cr6, 0x82fdca2c
	if !ctx.cr[6].lt {
	pc = 0x82FDCA2C; continue 'dispatch;
	}
	// 82FDC9B8: 57E9083C  slwi r9, r31, 1
	ctx.r[9].u32 = ctx.r[31].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82FDC9BC: 7D69F214  add r11, r9, r30
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[30].u64;
	// 82FDC9C0: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDC9C4: 2B0A002F  cmplwi cr6, r10, 0x2f
	ctx.cr[6].compare_u32(ctx.r[10].u32, 47 as u32, &mut ctx.xer);
	// 82FDC9C8: 419A0024  beq cr6, 0x82fdc9ec
	if ctx.cr[6].eq {
	pc = 0x82FDC9EC; continue 'dispatch;
	}
	// 82FDC9CC: 2B0A003F  cmplwi cr6, r10, 0x3f
	ctx.cr[6].compare_u32(ctx.r[10].u32, 63 as u32, &mut ctx.xer);
	// 82FDC9D0: 419A001C  beq cr6, 0x82fdc9ec
	if ctx.cr[6].eq {
	pc = 0x82FDC9EC; continue 'dispatch;
	}
	// 82FDC9D4: 2B0A0023  cmplwi cr6, r10, 0x23
	ctx.cr[6].compare_u32(ctx.r[10].u32, 35 as u32, &mut ctx.xer);
	// 82FDC9D8: 419A0014  beq cr6, 0x82fdc9ec
	if ctx.cr[6].eq {
	pc = 0x82FDC9EC; continue 'dispatch;
	}
	// 82FDC9DC: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82FDC9E0: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FDC9E4: 7F1FD000  cmpw cr6, r31, r26
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[26].s32, &mut ctx.xer);
	// 82FDC9E8: 4198FFD8  blt cr6, 0x82fdc9c0
	if ctx.cr[6].lt {
	pc = 0x82FDC9C0; continue 'dispatch;
	}
	// 82FDC9EC: 7F1F4000  cmpw cr6, r31, r8
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82FDC9F0: 40990018  ble cr6, 0x82fdca08
	if !ctx.cr[6].gt {
	pc = 0x82FDCA08; continue 'dispatch;
	}
	// 82FDC9F4: 7C88F850  subf r4, r8, r31
	ctx.r[4].s64 = ctx.r[31].s64 - ctx.r[8].s64;
	// 82FDC9F8: 7C69F214  add r3, r9, r30
	ctx.r[3].u64 = ctx.r[9].u64 + ctx.r[30].u64;
	// 82FDC9FC: 4BFFF925  bl 0x82fdc320
	ctx.lr = 0x82FDCA00;
	sub_82FDC320(ctx, base);
	// 82FDCA00: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDCA04: 41820030  beq 0x82fdca34
	if ctx.cr[0].eq {
	pc = 0x82FDCA34; continue 'dispatch;
	}
	// 82FDCA08: 7F1FD000  cmpw cr6, r31, r26
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[26].s32, &mut ctx.xer);
	// 82FDCA0C: 40980020  bge cr6, 0x82fdca2c
	if !ctx.cr[6].lt {
	pc = 0x82FDCA2C; continue 'dispatch;
	}
	// 82FDCA10: 57EB083C  slwi r11, r31, 1
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FDCA14: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 82FDCA18: 7C9FD050  subf r4, r31, r26
	ctx.r[4].s64 = ctx.r[26].s64 - ctx.r[31].s64;
	// 82FDCA1C: 7C6BF214  add r3, r11, r30
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82FDCA20: 4BFFE9B1  bl 0x82fdb3d0
	ctx.lr = 0x82FDCA24;
	sub_82FDB3D0(ctx, base);
	// 82FDCA24: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDCA28: 4182000C  beq 0x82fdca34
	if ctx.cr[0].eq {
	pc = 0x82FDCA34; continue 'dispatch;
	}
	// 82FDCA2C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82FDCA30: 48000008  b 0x82fdca38
	pc = 0x82FDCA38; continue 'dispatch;
	// 82FDCA34: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FDCA38: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82FDCA3C: 481CB76C  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDCA40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FDCA40 size=8
    let mut pc: u32 = 0x82FDCA40;
    'dispatch: loop {
        match pc {
            0x82FDCA40 => {
    //   block [0x82FDCA40..0x82FDCA48)
	// 82FDCA40: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FDCA44: 82139F44  lwz r16, -0x60bc(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-24764 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDCA48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDCA48 size=804
    let mut pc: u32 = 0x82FDCA48;
    'dispatch: loop {
        match pc {
            0x82FDCA48 => {
    //   block [0x82FDCA48..0x82FDCD6C)
	// 82FDCA48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDCA4C: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 82FDCA50: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 82FDCA54: 481CB6FD  bl 0x831a8150
	ctx.lr = 0x82FDCA58;
	sub_831A8130(ctx, base);
	// 82FDCA58: 3BE1FF40  addi r31, r1, -0xc0
	ctx.r[31].s64 = ctx.r[1].s64 + -192;
	// 82FDCA5C: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDCA60: 7C972378  mr r23, r4
	ctx.r[23].u64 = ctx.r[4].u64;
	// 82FDCA64: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FDCA68: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82FDCA6C: 2B170000  cmplwi cr6, r23, 0
	ctx.cr[6].compare_u32(ctx.r[23].u32, 0 as u32, &mut ctx.xer);
	// 82FDCA70: 419A0034  beq cr6, 0x82fdcaa4
	if ctx.cr[6].eq {
	pc = 0x82FDCAA4; continue 'dispatch;
	}
	// 82FDCA74: A1770000  lhz r11, 0(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDCA78: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDCA7C: 41820028  beq 0x82fdcaa4
	if ctx.cr[0].eq {
	pc = 0x82FDCAA4; continue 'dispatch;
	}
	// 82FDCA80: 39770002  addi r11, r23, 2
	ctx.r[11].s64 = ctx.r[23].s64 + 2;
	// 82FDCA84: 48000008  b 0x82fdca8c
	pc = 0x82FDCA8C; continue 'dispatch;
	// 82FDCA88: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FDCA8C: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDCA90: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDCA94: 4082FFF4  bne 0x82fdca88
	if !ctx.cr[0].eq {
	pc = 0x82FDCA88; continue 'dispatch;
	}
	// 82FDCA98: 7D775850  subf r11, r23, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[23].s64;
	// 82FDCA9C: 7D7A0E70  srawi r26, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[26].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82FDCAA0: 48000008  b 0x82fdcaa8
	pc = 0x82FDCAA8; continue 'dispatch;
	// 82FDCAA4: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82FDCAA8: 807E0028  lwz r3, 0x28(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDCAAC: 397A0001  addi r11, r26, 1
	ctx.r[11].s64 = ctx.r[26].s64 + 1;
	// 82FDCAB0: 5579083C  slwi r25, r11, 1
	ctx.r[25].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[25].u64 = ctx.r[25].u32 as u64;
	// 82FDCAB4: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82FDCAB8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDCABC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDCAC0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDCAC4: 4E800421  bctrl
	ctx.lr = 0x82FDCAC8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDCAC8: 817E0028  lwz r11, 0x28(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDCACC: 7C761B78  mr r22, r3
	ctx.r[22].u64 = ctx.r[3].u64;
	// 82FDCAD0: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82FDCAD4: 92DF0060  stw r22, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[22].u32 ) };
	// 82FDCAD8: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 82FDCADC: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82FDCAE0: 4BFF52D1  bl 0x82fd1db0
	ctx.lr = 0x82FDCAE4;
	sub_82FD1DB0(ctx, base);
	// 82FDCAE4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FDCAE8: 2F1DFFFF  cmpwi cr6, r29, -1
	ctx.cr[6].compare_i32(ctx.r[29].s32, -1, &mut ctx.xer);
	// 82FDCAEC: 419A0024  beq cr6, 0x82fdcb10
	if ctx.cr[6].eq {
	pc = 0x82FDCB10; continue 'dispatch;
	}
	// 82FDCAF0: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82FDCAF4: 80FE0028  lwz r7, 0x28(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDCAF8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FDCAFC: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 82FDCB00: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 82FDCB04: 4BFF5585  bl 0x82fd2088
	ctx.lr = 0x82FDCB08;
	sub_82FD2088(ctx, base);
	// 82FDCB08: 3B7D0001  addi r27, r29, 1
	ctx.r[27].s64 = ctx.r[29].s64 + 1;
	// 82FDCB0C: 48000008  b 0x82fdcb14
	pc = 0x82FDCB14; continue 'dispatch;
	// 82FDCB10: 3AC00000  li r22, 0
	ctx.r[22].s64 = 0;
	// 82FDCB14: 807E0028  lwz r3, 0x28(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDCB18: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82FDCB1C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDCB20: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDCB24: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDCB28: 4E800421  bctrl
	ctx.lr = 0x82FDCB2C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDCB2C: 817E0028  lwz r11, 0x28(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDCB30: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82FDCB34: 917F005C  stw r11, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82FDCB38: 931F0058  stw r24, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[24].u32 ) };
	// 82FDCB3C: 7F1BD000  cmpw cr6, r27, r26
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[26].s32, &mut ctx.xer);
	// 82FDCB40: 40980064  bge cr6, 0x82fdcba4
	if !ctx.cr[6].lt {
	pc = 0x82FDCBA4; continue 'dispatch;
	}
	// 82FDCB44: 576B083C  slwi r11, r27, 1
	ctx.r[11].u32 = ctx.r[27].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FDCB48: 7FABBA14  add r29, r11, r23
	ctx.r[29].u64 = ctx.r[11].u64 + ctx.r[23].u64;
	// 82FDCB4C: A17D0000  lhz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDCB50: 2B0B005B  cmplwi cr6, r11, 0x5b
	ctx.cr[6].compare_u32(ctx.r[11].u32, 91 as u32, &mut ctx.xer);
	// 82FDCB54: 409A0050  bne cr6, 0x82fdcba4
	if !ctx.cr[6].eq {
	pc = 0x82FDCBA4; continue 'dispatch;
	}
	// 82FDCB58: 3880005D  li r4, 0x5d
	ctx.r[4].s64 = 93;
	// 82FDCB5C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FDCB60: 4BFF5251  bl 0x82fd1db0
	ctx.lr = 0x82FDCB64;
	sub_82FD1DB0(ctx, base);
	// 82FDCB64: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82FDCB68: 2F1CFFFF  cmpwi cr6, r28, -1
	ctx.cr[6].compare_i32(ctx.r[28].s32, -1, &mut ctx.xer);
	// 82FDCB6C: 419A007C  beq cr6, 0x82fdcbe8
	if ctx.cr[6].eq {
	pc = 0x82FDCBE8; continue 'dispatch;
	}
	// 82FDCB70: 7D7BE214  add r11, r27, r28
	ctx.r[11].u64 = ctx.r[27].u64 + ctx.r[28].u64;
	// 82FDCB74: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 82FDCB78: 7F0AD000  cmpw cr6, r10, r26
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[26].s32, &mut ctx.xer);
	// 82FDCB7C: 40980020  bge cr6, 0x82fdcb9c
	if !ctx.cr[6].lt {
	pc = 0x82FDCB9C; continue 'dispatch;
	}
	// 82FDCB80: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FDCB84: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FDCB88: 7D6BBA2E  lhzx r11, r11, r23
	ctx.r[11].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82FDCB8C: 2B0B003A  cmplwi cr6, r11, 0x3a
	ctx.cr[6].compare_u32(ctx.r[11].u32, 58 as u32, &mut ctx.xer);
	// 82FDCB90: 409A000C  bne cr6, 0x82fdcb9c
	if !ctx.cr[6].eq {
	pc = 0x82FDCB9C; continue 'dispatch;
	}
	// 82FDCB94: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82FDCB98: 48000024  b 0x82fdcbbc
	pc = 0x82FDCBBC; continue 'dispatch;
	// 82FDCB9C: 3B80FFFF  li r28, -1
	ctx.r[28].s64 = -1;
	// 82FDCBA0: 4800001C  b 0x82fdcbbc
	pc = 0x82FDCBBC; continue 'dispatch;
	// 82FDCBA4: 576B083C  slwi r11, r27, 1
	ctx.r[11].u32 = ctx.r[27].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FDCBA8: 3880003A  li r4, 0x3a
	ctx.r[4].s64 = 58;
	// 82FDCBAC: 7FABBA14  add r29, r11, r23
	ctx.r[29].u64 = ctx.r[11].u64 + ctx.r[23].u64;
	// 82FDCBB0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FDCBB4: 4BFF51FD  bl 0x82fd1db0
	ctx.lr = 0x82FDCBB8;
	sub_82FD1DB0(ctx, base);
	// 82FDCBB8: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82FDCBBC: 2F1CFFFF  cmpwi cr6, r28, -1
	ctx.cr[6].compare_i32(ctx.r[28].s32, -1, &mut ctx.xer);
	// 82FDCBC0: 419A0028  beq cr6, 0x82fdcbe8
	if ctx.cr[6].eq {
	pc = 0x82FDCBE8; continue 'dispatch;
	}
	// 82FDCBC4: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82FDCBC8: 80FE0028  lwz r7, 0x28(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDCBCC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FDCBD0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FDCBD4: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82FDCBD8: 4BFF54B1  bl 0x82fd2088
	ctx.lr = 0x82FDCBDC;
	sub_82FD2088(ctx, base);
	// 82FDCBDC: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82FDCBE0: 7F7BE214  add r27, r27, r28
	ctx.r[27].u64 = ctx.r[27].u64 + ctx.r[28].u64;
	// 82FDCBE4: 48000020  b 0x82fdcc04
	pc = 0x82FDCC04; continue 'dispatch;
	// 82FDCBE8: 7CDBD050  subf r6, r27, r26
	ctx.r[6].s64 = ctx.r[26].s64 - ctx.r[27].s64;
	// 82FDCBEC: 80FE0028  lwz r7, 0x28(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDCBF0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FDCBF4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FDCBF8: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82FDCBFC: 4BFF548D  bl 0x82fd2088
	ctx.lr = 0x82FDCC00;
	sub_82FD2088(ctx, base);
	// 82FDCC00: 7F5BD378  mr r27, r26
	ctx.r[27].u64 = ctx.r[26].u64;
	// 82FDCC04: 807E0028  lwz r3, 0x28(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDCC08: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82FDCC0C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDCC10: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDCC14: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDCC18: 4E800421  bctrl
	ctx.lr = 0x82FDCC1C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDCC1C: 80FE0028  lwz r7, 0x28(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDCC20: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FDCC24: 90FF0054  stw r7, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82FDCC28: 93BF0050  stw r29, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 82FDCC2C: 3B20FFFF  li r25, -1
	ctx.r[25].s64 = -1;
	// 82FDCC30: 2B180000  cmplwi cr6, r24, 0
	ctx.cr[6].compare_u32(ctx.r[24].u32, 0 as u32, &mut ctx.xer);
	// 82FDCC34: 419A0060  beq cr6, 0x82fdcc94
	if ctx.cr[6].eq {
	pc = 0x82FDCC94; continue 'dispatch;
	}
	// 82FDCC38: A1780000  lhz r11, 0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDCC3C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDCC40: 41820054  beq 0x82fdcc94
	if ctx.cr[0].eq {
	pc = 0x82FDCC94; continue 'dispatch;
	}
	// 82FDCC44: 2F1CFFFF  cmpwi cr6, r28, -1
	ctx.cr[6].compare_i32(ctx.r[28].s32, -1, &mut ctx.xer);
	// 82FDCC48: 419A004C  beq cr6, 0x82fdcc94
	if ctx.cr[6].eq {
	pc = 0x82FDCC94; continue 'dispatch;
	}
	// 82FDCC4C: 7F1BD000  cmpw cr6, r27, r26
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[26].s32, &mut ctx.xer);
	// 82FDCC50: 40980044  bge cr6, 0x82fdcc94
	if !ctx.cr[6].lt {
	pc = 0x82FDCC94; continue 'dispatch;
	}
	// 82FDCC54: 576B083C  slwi r11, r27, 1
	ctx.r[11].u32 = ctx.r[27].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FDCC58: 7CDBD050  subf r6, r27, r26
	ctx.r[6].s64 = ctx.r[26].s64 - ctx.r[27].s64;
	// 82FDCC5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FDCC60: 7C8BBA14  add r4, r11, r23
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[23].u64;
	// 82FDCC64: 4BFF5425  bl 0x82fd2088
	ctx.lr = 0x82FDCC68;
	sub_82FD2088(ctx, base);
	// 82FDCC68: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82FDCC6C: 419A0028  beq cr6, 0x82fdcc94
	if ctx.cr[6].eq {
	pc = 0x82FDCC94; continue 'dispatch;
	}
	// 82FDCC70: A17D0000  lhz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDCC74: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDCC78: 4182001C  beq 0x82fdcc94
	if ctx.cr[0].eq {
	pc = 0x82FDCC94; continue 'dispatch;
	}
	// 82FDCC7C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FDCC80: 809E0028  lwz r4, 0x28(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDCC84: 4BFF6945  bl 0x82fd35c8
	ctx.lr = 0x82FDCC88;
	sub_82FD35C8(ctx, base);
	// 82FDCC88: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FDCC8C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FDCC90: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82FDCC94: 7EC5B378  mr r5, r22
	ctx.r[5].u64 = ctx.r[22].u64;
	// 82FDCC98: 80DE0028  lwz r6, 0x28(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDCC9C: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82FDCCA0: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82FDCCA4: 4BFFFA4D  bl 0x82fdc6f0
	ctx.lr = 0x82FDCCA8;
	sub_82FDC6F0(ctx, base);
	// 82FDCCA8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDCCAC: 41820088  beq 0x82fdcd34
	if ctx.cr[0].eq {
	pc = 0x82FDCD34; continue 'dispatch;
	}
	// 82FDCCB0: 809E000C  lwz r4, 0xc(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FDCCB4: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDCCB8: 41820018  beq 0x82fdccd0
	if ctx.cr[0].eq {
	pc = 0x82FDCCD0; continue 'dispatch;
	}
	// 82FDCCBC: 807E0028  lwz r3, 0x28(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDCCC0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDCCC4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FDCCC8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDCCCC: 4E800421  bctrl
	ctx.lr = 0x82FDCCD0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDCCD0: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FDCCD4: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDCCD8: 41820018  beq 0x82fdccf0
	if ctx.cr[0].eq {
	pc = 0x82FDCCF0; continue 'dispatch;
	}
	// 82FDCCDC: 807E0028  lwz r3, 0x28(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDCCE0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDCCE4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FDCCE8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDCCEC: 4E800421  bctrl
	ctx.lr = 0x82FDCCF0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDCCF0: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82FDCCF4: 809E0028  lwz r4, 0x28(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDCCF8: 4BFF3E89  bl 0x82fd0b80
	ctx.lr = 0x82FDCCFC;
	sub_82FD0B80(ctx, base);
	// 82FDCCFC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82FDCD00: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 82FDCD04: 809E0028  lwz r4, 0x28(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDCD08: 933E0010  stw r25, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[25].u32 ) };
	// 82FDCD0C: 917E000C  stw r11, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82FDCD10: 4BFF3E71  bl 0x82fd0b80
	ctx.lr = 0x82FDCD14;
	sub_82FD0B80(ctx, base);
	// 82FDCD14: 907E0008  stw r3, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 82FDCD18: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FDCD1C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FDCD20: 4BFF5DA1  bl 0x82fd2ac0
	ctx.lr = 0x82FDCD24;
	sub_82FD2AC0(ctx, base);
	// 82FDCD24: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FDCD28: 387F0058  addi r3, r31, 0x58
	ctx.r[3].s64 = ctx.r[31].s64 + 88;
	// 82FDCD2C: 4BFF5D95  bl 0x82fd2ac0
	ctx.lr = 0x82FDCD30;
	sub_82FD2AC0(ctx, base);
	// 82FDCD30: 48000028  b 0x82fdcd58
	pc = 0x82FDCD58; continue 'dispatch;
	// 82FDCD34: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 82FDCD38: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FDCD3C: 4BFFF8AD  bl 0x82fdc5e8
	ctx.lr = 0x82FDCD40;
	sub_82FDC5E8(ctx, base);
	// 82FDCD40: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FDCD44: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FDCD48: 4BFF5D79  bl 0x82fd2ac0
	ctx.lr = 0x82FDCD4C;
	sub_82FD2AC0(ctx, base);
	// 82FDCD4C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FDCD50: 387F0058  addi r3, r31, 0x58
	ctx.r[3].s64 = ctx.r[31].s64 + 88;
	// 82FDCD54: 4BFF5D6D  bl 0x82fd2ac0
	ctx.lr = 0x82FDCD58;
	sub_82FD2AC0(ctx, base);
	// 82FDCD58: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FDCD5C: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FDCD60: 4BFF5D61  bl 0x82fd2ac0
	ctx.lr = 0x82FDCD64;
	sub_82FD2AC0(ctx, base);
	// 82FDCD64: 383F00C0  addi r1, r31, 0xc0
	ctx.r[1].s64 = ctx.r[31].s64 + 192;
	// 82FDCD68: 481CB438  b 0x831a81a0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDCD6C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FDCD6C size=8
    let mut pc: u32 = 0x82FDCD6C;
    'dispatch: loop {
        match pc {
            0x82FDCD6C => {
    //   block [0x82FDCD6C..0x82FDCD74)
	// 82FDCD6C: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FDCD70: 82139F44  lwz r16, -0x60bc(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-24764 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDCD74(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDCD74 size=24
    let mut pc: u32 = 0x82FDCD74;
    'dispatch: loop {
        match pc {
            0x82FDCD74 => {
    //   block [0x82FDCD74..0x82FDCD8C)
	// 82FDCD74: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDCD78: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FDCD7C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDCD80: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FDCD84: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FDCD88: 481D3EA1  bl 0x831b0c28
	ctx.lr = 0x82FDCD8C;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDCD94(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDCD94 size=24
    let mut pc: u32 = 0x82FDCD94;
    'dispatch: loop {
        match pc {
            0x82FDCD94 => {
    //   block [0x82FDCD94..0x82FDCDAC)
	// 82FDCD94: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDCD98: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FDCD9C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDCDA0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FDCDA4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FDCDA8: 481D3E81  bl 0x831b0c28
	ctx.lr = 0x82FDCDAC;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDCDAC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDCDAC size=40
    let mut pc: u32 = 0x82FDCDAC;
    'dispatch: loop {
        match pc {
            0x82FDCDAC => {
    //   block [0x82FDCDAC..0x82FDCDD4)
	// 82FDCDAC: 3BECFF40  addi r31, r12, -0xc0
	ctx.r[31].s64 = ctx.r[12].s64 + -192;
	// 82FDCDB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDCDB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FDCDB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDCDBC: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FDCDC0: 4BFF6099  bl 0x82fd2e58
	ctx.lr = 0x82FDCDC4;
	sub_82FD2E58(ctx, base);
	// 82FDCDC4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FDCDC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FDCDCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FDCDD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDCDD4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDCDD4 size=40
    let mut pc: u32 = 0x82FDCDD4;
    'dispatch: loop {
        match pc {
            0x82FDCDD4 => {
    //   block [0x82FDCDD4..0x82FDCDFC)
	// 82FDCDD4: 3BECFF40  addi r31, r12, -0xc0
	ctx.r[31].s64 = ctx.r[12].s64 + -192;
	// 82FDCDD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDCDDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FDCDE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDCDE4: 387F0058  addi r3, r31, 0x58
	ctx.r[3].s64 = ctx.r[31].s64 + 88;
	// 82FDCDE8: 4BFF6071  bl 0x82fd2e58
	ctx.lr = 0x82FDCDEC;
	sub_82FD2E58(ctx, base);
	// 82FDCDEC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FDCDF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FDCDF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FDCDF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDCDFC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDCDFC size=40
    let mut pc: u32 = 0x82FDCDFC;
    'dispatch: loop {
        match pc {
            0x82FDCDFC => {
    //   block [0x82FDCDFC..0x82FDCE24)
	// 82FDCDFC: 3BECFF40  addi r31, r12, -0xc0
	ctx.r[31].s64 = ctx.r[12].s64 + -192;
	// 82FDCE00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDCE04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FDCE08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDCE0C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FDCE10: 4BFF6049  bl 0x82fd2e58
	ctx.lr = 0x82FDCE14;
	sub_82FD2E58(ctx, base);
	// 82FDCE14: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FDCE18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FDCE1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FDCE20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDCE28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FDCE28 size=8
    let mut pc: u32 = 0x82FDCE28;
    'dispatch: loop {
        match pc {
            0x82FDCE28 => {
    //   block [0x82FDCE28..0x82FDCE30)
	// 82FDCE28: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FDCE2C: 8213A038  lwz r16, -0x5fc8(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-24520 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDCE30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDCE30 size=2848
    let mut pc: u32 = 0x82FDCE30;
    'dispatch: loop {
        match pc {
            0x82FDCE30 => {
    //   block [0x82FDCE30..0x82FDD950)
	// 82FDCE30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDCE34: 481CB315  bl 0x831a8148
	ctx.lr = 0x82FDCE38;
	sub_831A8130(ctx, base);
	// 82FDCE38: 3BE1FEE0  addi r31, r1, -0x120
	ctx.r[31].s64 = ctx.r[1].s64 + -288;
	// 82FDCE3C: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDCE40: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FDCE44: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82FDCE48: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82FDCE4C: 809E0028  lwz r4, 0x28(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDCE50: 4BFF3D31  bl 0x82fd0b80
	ctx.lr = 0x82FDCE54;
	sub_82FD0B80(ctx, base);
	// 82FDCE54: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82FDCE58: 4BFF50B1  bl 0x82fd1f08
	ctx.lr = 0x82FDCE5C;
	sub_82FD1F08(ctx, base);
	// 82FDCE5C: 815E0028  lwz r10, 0x28(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDCE60: 939F0060  stw r28, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[28].u32 ) };
	// 82FDCE64: 915F0064  stw r10, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[10].u32 ) };
	// 82FDCE68: 3A800000  li r20, 0
	ctx.r[20].s64 = 0;
	// 82FDCE6C: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82FDCE70: 419A0034  beq cr6, 0x82fdcea4
	if ctx.cr[6].eq {
	pc = 0x82FDCEA4; continue 'dispatch;
	}
	// 82FDCE74: A17C0000  lhz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDCE78: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDCE7C: 41820028  beq 0x82fdcea4
	if ctx.cr[0].eq {
	pc = 0x82FDCEA4; continue 'dispatch;
	}
	// 82FDCE80: 397C0002  addi r11, r28, 2
	ctx.r[11].s64 = ctx.r[28].s64 + 2;
	// 82FDCE84: 48000008  b 0x82fdce8c
	pc = 0x82FDCE8C; continue 'dispatch;
	// 82FDCE88: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FDCE8C: A12B0000  lhz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDCE90: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDCE94: 4082FFF4  bne 0x82fdce88
	if !ctx.cr[0].eq {
	pc = 0x82FDCE88; continue 'dispatch;
	}
	// 82FDCE98: 7D7C5850  subf r11, r28, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[28].s64;
	// 82FDCE9C: 7D760E70  srawi r22, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[22].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82FDCEA0: 48000008  b 0x82fdcea8
	pc = 0x82FDCEA8; continue 'dispatch;
	// 82FDCEA4: 7E96A378  mr r22, r20
	ctx.r[22].u64 = ctx.r[20].u64;
	// 82FDCEA8: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 82FDCEAC: 409A0058  bne cr6, 0x82fdcf04
	if !ctx.cr[6].eq {
	pc = 0x82FDCF04; continue 'dispatch;
	}
	// 82FDCEB0: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82FDCEB4: 419A000C  beq cr6, 0x82fdcec0
	if ctx.cr[6].eq {
	pc = 0x82FDCEC0; continue 'dispatch;
	}
	// 82FDCEB8: 2F160000  cmpwi cr6, r22, 0
	ctx.cr[6].compare_i32(ctx.r[22].s32, 0, &mut ctx.xer);
	// 82FDCEBC: 409A0048  bne cr6, 0x82fdcf04
	if !ctx.cr[6].eq {
	pc = 0x82FDCF04; continue 'dispatch;
	}
	// 82FDCEC0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FDCEC4: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82FDCEC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FDCECC: 396B9CC0  addi r11, r11, -0x6340
	ctx.r[11].s64 = ctx.r[11].s64 + -25408;
	// 82FDCED0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82FDCED4: 38EBFF58  addi r7, r11, -0xa8
	ctx.r[7].s64 = ctx.r[11].s64 + -168;
	// 82FDCED8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FDCEDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82FDCEE0: 388B9D88  addi r4, r11, -0x6278
	ctx.r[4].s64 = ctx.r[11].s64 + -25208;
	// 82FDCEE4: 38C0010A  li r6, 0x10a
	ctx.r[6].s64 = 266;
	// 82FDCEE8: 38A00196  li r5, 0x196
	ctx.r[5].s64 = 406;
	// 82FDCEEC: 387F00A0  addi r3, r31, 0xa0
	ctx.r[3].s64 = ctx.r[31].s64 + 160;
	// 82FDCEF0: 4BFFCCE1  bl 0x82fd9bd0
	ctx.lr = 0x82FDCEF4;
	sub_82FD9BD0(ctx, base);
	// 82FDCEF4: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FDCEF8: 387F00A0  addi r3, r31, 0xa0
	ctx.r[3].s64 = ctx.r[31].s64 + 160;
	// 82FDCEFC: 388BC5E4  addi r4, r11, -0x3a1c
	ctx.r[4].s64 = ctx.r[11].s64 + -14876;
	// 82FDCF00: 481D3D29  bl 0x831b0c28
	ctx.lr = 0x82FDCF04;
	sub_831B0C28(ctx, base);
	// 82FDCF04: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82FDCF08: 419A0A28  beq cr6, 0x82fdd930
	if ctx.cr[6].eq {
	pc = 0x82FDD930; continue 'dispatch;
	}
	// 82FDCF0C: 2F160000  cmpwi cr6, r22, 0
	ctx.cr[6].compare_i32(ctx.r[22].s32, 0, &mut ctx.xer);
	// 82FDCF10: 419A0A20  beq cr6, 0x82fdd930
	if ctx.cr[6].eq {
	pc = 0x82FDD930; continue 'dispatch;
	}
	// 82FDCF14: 3880003A  li r4, 0x3a
	ctx.r[4].s64 = 58;
	// 82FDCF18: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FDCF1C: 7E9DA378  mr r29, r20
	ctx.r[29].u64 = ctx.r[20].u64;
	// 82FDCF20: 7E97A378  mr r23, r20
	ctx.r[23].u64 = ctx.r[20].u64;
	// 82FDCF24: 4BFF4E8D  bl 0x82fd1db0
	ctx.lr = 0x82FDCF28;
	sub_82FD1DB0(ctx, base);
	// 82FDCF28: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82FDCF2C: 3880002F  li r4, 0x2f
	ctx.r[4].s64 = 47;
	// 82FDCF30: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FDCF34: 4BFF4E7D  bl 0x82fd1db0
	ctx.lr = 0x82FDCF38;
	sub_82FD1DB0(ctx, base);
	// 82FDCF38: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82FDCF3C: 3880003F  li r4, 0x3f
	ctx.r[4].s64 = 63;
	// 82FDCF40: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FDCF44: 4BFF4E6D  bl 0x82fd1db0
	ctx.lr = 0x82FDCF48;
	sub_82FD1DB0(ctx, base);
	// 82FDCF48: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82FDCF4C: 38800023  li r4, 0x23
	ctx.r[4].s64 = 35;
	// 82FDCF50: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FDCF54: 4BFF4E5D  bl 0x82fd1db0
	ctx.lr = 0x82FDCF58;
	sub_82FD1DB0(ctx, base);
	// 82FDCF58: 2F1B0002  cmpwi cr6, r27, 2
	ctx.cr[6].compare_i32(ctx.r[27].s32, 2, &mut ctx.xer);
	// 82FDCF5C: 4198008C  blt cr6, 0x82fdcfe8
	if ctx.cr[6].lt {
	pc = 0x82FDCFE8; continue 'dispatch;
	}
	// 82FDCF60: 7F1BC800  cmpw cr6, r27, r25
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[25].s32, &mut ctx.xer);
	// 82FDCF64: 4099000C  ble cr6, 0x82fdcf70
	if !ctx.cr[6].gt {
	pc = 0x82FDCF70; continue 'dispatch;
	}
	// 82FDCF68: 2F19FFFF  cmpwi cr6, r25, -1
	ctx.cr[6].compare_i32(ctx.r[25].s32, -1, &mut ctx.xer);
	// 82FDCF6C: 409A007C  bne cr6, 0x82fdcfe8
	if !ctx.cr[6].eq {
	pc = 0x82FDCFE8; continue 'dispatch;
	}
	// 82FDCF70: 7F1BC000  cmpw cr6, r27, r24
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[24].s32, &mut ctx.xer);
	// 82FDCF74: 4099000C  ble cr6, 0x82fdcf80
	if !ctx.cr[6].gt {
	pc = 0x82FDCF80; continue 'dispatch;
	}
	// 82FDCF78: 2F18FFFF  cmpwi cr6, r24, -1
	ctx.cr[6].compare_i32(ctx.r[24].s32, -1, &mut ctx.xer);
	// 82FDCF7C: 409A006C  bne cr6, 0x82fdcfe8
	if !ctx.cr[6].eq {
	pc = 0x82FDCFE8; continue 'dispatch;
	}
	// 82FDCF80: 7F1B1800  cmpw cr6, r27, r3
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[3].s32, &mut ctx.xer);
	// 82FDCF84: 4099000C  ble cr6, 0x82fdcf90
	if !ctx.cr[6].gt {
	pc = 0x82FDCF90; continue 'dispatch;
	}
	// 82FDCF88: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82FDCF8C: 409A005C  bne cr6, 0x82fdcfe8
	if !ctx.cr[6].eq {
	pc = 0x82FDCFE8; continue 'dispatch;
	}
	// 82FDCF90: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FDCF94: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FDCF98: 3AE00001  li r23, 1
	ctx.r[23].s64 = 1;
	// 82FDCF9C: 4BFFEF45  bl 0x82fdbee0
	ctx.lr = 0x82FDCFA0;
	sub_82FDBEE0(ctx, base);
	// 82FDCFA0: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDCFA4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDCFA8: 41820034  beq 0x82fdcfdc
	if ctx.cr[0].eq {
	pc = 0x82FDCFDC; continue 'dispatch;
	}
	// 82FDCFAC: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDCFB0: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDCFB4: 41820028  beq 0x82fdcfdc
	if ctx.cr[0].eq {
	pc = 0x82FDCFDC; continue 'dispatch;
	}
	// 82FDCFB8: 394B0002  addi r10, r11, 2
	ctx.r[10].s64 = ctx.r[11].s64 + 2;
	// 82FDCFBC: 48000008  b 0x82fdcfc4
	pc = 0x82FDCFC4; continue 'dispatch;
	// 82FDCFC0: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 82FDCFC4: A12A0000  lhz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDCFC8: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDCFCC: 4082FFF4  bne 0x82fdcfc0
	if !ctx.cr[0].eq {
	pc = 0x82FDCFC0; continue 'dispatch;
	}
	// 82FDCFD0: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82FDCFD4: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82FDCFD8: 48000008  b 0x82fdcfe0
	pc = 0x82FDCFE0; continue 'dispatch;
	// 82FDCFDC: 7E8BA378  mr r11, r20
	ctx.r[11].u64 = ctx.r[20].u64;
	// 82FDCFE0: 3BAB0001  addi r29, r11, 1
	ctx.r[29].s64 = ctx.r[11].s64 + 1;
	// 82FDCFE4: 4800001C  b 0x82fdd000
	pc = 0x82FDD000; continue 'dispatch;
	// 82FDCFE8: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 82FDCFEC: 419A0918  beq cr6, 0x82fdd904
	if ctx.cr[6].eq {
	pc = 0x82FDD904; continue 'dispatch;
	}
	// 82FDCFF0: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 82FDCFF4: 409A000C  bne cr6, 0x82fdd000
	if !ctx.cr[6].eq {
	pc = 0x82FDD000; continue 'dispatch;
	}
	// 82FDCFF8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82FDCFFC: 409A0908  bne cr6, 0x82fdd904
	if !ctx.cr[6].eq {
	pc = 0x82FDD904; continue 'dispatch;
	}
	// 82FDD000: 7F1DB000  cmpw cr6, r29, r22
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[22].s32, &mut ctx.xer);
	// 82FDD004: 419A08B8  beq cr6, 0x82fdd8bc
	if ctx.cr[6].eq {
	pc = 0x82FDD8BC; continue 'dispatch;
	}
	// 82FDD008: 56EB063F  clrlwi. r11, r23, 0x18
	ctx.r[11].u64 = ctx.r[23].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDD00C: 41820014  beq 0x82fdd020
	if ctx.cr[0].eq {
	pc = 0x82FDD020; continue 'dispatch;
	}
	// 82FDD010: 57AB083C  slwi r11, r29, 1
	ctx.r[11].u32 = ctx.r[29].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FDD014: 7D6BE22E  lhzx r11, r11, r28
	ctx.r[11].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82FDD018: 2B0B0023  cmplwi cr6, r11, 0x23
	ctx.cr[6].compare_u32(ctx.r[11].u32, 35 as u32, &mut ctx.xer);
	// 82FDD01C: 419A08A0  beq cr6, 0x82fdd8bc
	if ctx.cr[6].eq {
	pc = 0x82FDD8BC; continue 'dispatch;
	}
	// 82FDD020: 807E0028  lwz r3, 0x28(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDD024: 39760001  addi r11, r22, 1
	ctx.r[11].s64 = ctx.r[22].s64 + 1;
	// 82FDD028: 5579083C  slwi r25, r11, 1
	ctx.r[25].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[25].u64 = ctx.r[25].u32 as u64;
	// 82FDD02C: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82FDD030: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDD034: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDD038: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDD03C: 4E800421  bctrl
	ctx.lr = 0x82FDD040;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDD040: 80FE0028  lwz r7, 0x28(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDD044: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82FDD048: 90FF006C  stw r7, 0x6c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[7].u32 ) };
	// 82FDD04C: 937F0068  stw r27, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[27].u32 ) };
	// 82FDD050: 7EC6B378  mr r6, r22
	ctx.r[6].u64 = ctx.r[22].u64;
	// 82FDD054: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82FDD058: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FDD05C: 4BFF502D  bl 0x82fd2088
	ctx.lr = 0x82FDD060;
	sub_82FD2088(ctx, base);
	// 82FDD060: 397D0001  addi r11, r29, 1
	ctx.r[11].s64 = ctx.r[29].s64 + 1;
	// 82FDD064: 7F0BB000  cmpw cr6, r11, r22
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[22].s32, &mut ctx.xer);
	// 82FDD068: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FDD06C: 3AAB9CC0  addi r21, r11, -0x6340
	ctx.r[21].s64 = ctx.r[11].s64 + -25408;
	// 82FDD070: 40980098  bge cr6, 0x82fdd108
	if !ctx.cr[6].lt {
	pc = 0x82FDD108; continue 'dispatch;
	}
	// 82FDD074: 3895FFD8  addi r4, r21, -0x28
	ctx.r[4].s64 = ctx.r[21].s64 + -40;
	// 82FDD078: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82FDD07C: 4BFFCCF5  bl 0x82fd9d70
	ctx.lr = 0x82FDD080;
	sub_82FD9D70(ctx, base);
	// 82FDD080: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDD084: 41820084  beq 0x82fdd108
	if ctx.cr[0].eq {
	pc = 0x82FDD108; continue 'dispatch;
	}
	// 82FDD088: 3BBD0002  addi r29, r29, 2
	ctx.r[29].s64 = ctx.r[29].s64 + 2;
	// 82FDD08C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82FDD090: 7F1DB000  cmpw cr6, r29, r22
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[22].s32, &mut ctx.xer);
	// 82FDD094: 40980064  bge cr6, 0x82fdd0f8
	if !ctx.cr[6].lt {
	pc = 0x82FDD0F8; continue 'dispatch;
	}
	// 82FDD098: 57AB083C  slwi r11, r29, 1
	ctx.r[11].u32 = ctx.r[29].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FDD09C: 7D6BE214  add r11, r11, r28
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 82FDD0A0: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDD0A4: 2B0A002F  cmplwi cr6, r10, 0x2f
	ctx.cr[6].compare_u32(ctx.r[10].u32, 47 as u32, &mut ctx.xer);
	// 82FDD0A8: 419A0024  beq cr6, 0x82fdd0cc
	if ctx.cr[6].eq {
	pc = 0x82FDD0CC; continue 'dispatch;
	}
	// 82FDD0AC: 2B0A003F  cmplwi cr6, r10, 0x3f
	ctx.cr[6].compare_u32(ctx.r[10].u32, 63 as u32, &mut ctx.xer);
	// 82FDD0B0: 419A001C  beq cr6, 0x82fdd0cc
	if ctx.cr[6].eq {
	pc = 0x82FDD0CC; continue 'dispatch;
	}
	// 82FDD0B4: 2B0A0023  cmplwi cr6, r10, 0x23
	ctx.cr[6].compare_u32(ctx.r[10].u32, 35 as u32, &mut ctx.xer);
	// 82FDD0B8: 419A0014  beq cr6, 0x82fdd0cc
	if ctx.cr[6].eq {
	pc = 0x82FDD0CC; continue 'dispatch;
	}
	// 82FDD0BC: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82FDD0C0: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FDD0C4: 7F1DB000  cmpw cr6, r29, r22
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[22].s32, &mut ctx.xer);
	// 82FDD0C8: 4198FFD8  blt cr6, 0x82fdd0a0
	if ctx.cr[6].lt {
	pc = 0x82FDD0A0; continue 'dispatch;
	}
	// 82FDD0CC: 7F1D2800  cmpw cr6, r29, r5
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[5].s32, &mut ctx.xer);
	// 82FDD0D0: 40990028  ble cr6, 0x82fdd0f8
	if !ctx.cr[6].gt {
	pc = 0x82FDD0F8; continue 'dispatch;
	}
	// 82FDD0D4: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82FDD0D8: 80FE0028  lwz r7, 0x28(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDD0DC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FDD0E0: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82FDD0E4: 4BFF4FA5  bl 0x82fd2088
	ctx.lr = 0x82FDD0E8;
	sub_82FD2088(ctx, base);
	// 82FDD0E8: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82FDD0EC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FDD0F0: 4BFFF959  bl 0x82fdca48
	ctx.lr = 0x82FDD0F4;
	sub_82FDCA48(ctx, base);
	// 82FDD0F4: 48000014  b 0x82fdd108
	pc = 0x82FDD108; continue 'dispatch;
	// 82FDD0F8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FDD0FC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FDD100: 388B8158  addi r4, r11, -0x7ea8
	ctx.r[4].s64 = ctx.r[11].s64 + -32424;
	// 82FDD104: 4BFFF3C5  bl 0x82fdc4c8
	ctx.lr = 0x82FDD108;
	sub_82FDC4C8(ctx, base);
	// 82FDD108: 7F1DB000  cmpw cr6, r29, r22
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[22].s32, &mut ctx.xer);
	// 82FDD10C: 41980014  blt cr6, 0x82fdd120
	if ctx.cr[6].lt {
	pc = 0x82FDD120; continue 'dispatch;
	}
	// 82FDD110: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FDD114: 387F0068  addi r3, r31, 0x68
	ctx.r[3].s64 = ctx.r[31].s64 + 104;
	// 82FDD118: 4BFF59A9  bl 0x82fd2ac0
	ctx.lr = 0x82FDD11C;
	sub_82FD2AC0(ctx, base);
	// 82FDD11C: 48000820  b 0x82fdd93c
	pc = 0x82FDD93C; continue 'dispatch;
	// 82FDD120: 807E0028  lwz r3, 0x28(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDD124: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82FDD128: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDD12C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDD130: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDD134: 4E800421  bctrl
	ctx.lr = 0x82FDD138;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDD138: 80FE0028  lwz r7, 0x28(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDD13C: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82FDD140: 90FF0074  stw r7, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[7].u32 ) };
	// 82FDD144: 937F0070  stw r27, 0x70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[27].u32 ) };
	// 82FDD148: 7EC6B378  mr r6, r22
	ctx.r[6].u64 = ctx.r[22].u64;
	// 82FDD14C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82FDD150: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FDD154: 4BFF4F35  bl 0x82fd2088
	ctx.lr = 0x82FDD158;
	sub_82FD2088(ctx, base);
	// 82FDD158: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82FDD15C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FDD160: 4BFFCF81  bl 0x82fda0e0
	ctx.lr = 0x82FDD164;
	sub_82FDA0E0(ctx, base);
	// 82FDD164: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 82FDD168: 419A0700  beq cr6, 0x82fdd868
	if ctx.cr[6].eq {
	pc = 0x82FDD868; continue 'dispatch;
	}
	// 82FDD16C: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FDD170: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDD174: 41820010  beq 0x82fdd184
	if ctx.cr[0].eq {
	pc = 0x82FDD184; continue 'dispatch;
	}
	// 82FDD178: A16B0000  lhz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDD17C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDD180: 408200F0  bne 0x82fdd270
	if !ctx.cr[0].eq {
	pc = 0x82FDD270; continue 'dispatch;
	}
	// 82FDD184: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDD188: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FDD18C: 409A00E4  bne cr6, 0x82fdd270
	if !ctx.cr[6].eq {
	pc = 0x82FDD270; continue 'dispatch;
	}
	// 82FDD190: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FDD194: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FDD198: 409A00D8  bne cr6, 0x82fdd270
	if !ctx.cr[6].eq {
	pc = 0x82FDD270; continue 'dispatch;
	}
	// 82FDD19C: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FDD1A0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FDD1A4: 409A00CC  bne cr6, 0x82fdd270
	if !ctx.cr[6].eq {
	pc = 0x82FDD270; continue 'dispatch;
	}
	// 82FDD1A8: 807A0004  lwz r3, 4(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDD1AC: 809E0028  lwz r4, 0x28(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDD1B0: 4BFF39D1  bl 0x82fd0b80
	ctx.lr = 0x82FDD1B4;
	sub_82FD0B80(ctx, base);
	// 82FDD1B4: 817E0028  lwz r11, 0x28(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDD1B8: 907E0004  stw r3, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82FDD1BC: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82FDD1C0: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FDD1C4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDD1C8: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FDD1CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDD1D0: 4E800421  bctrl
	ctx.lr = 0x82FDD1D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDD1D4: 807A0008  lwz r3, 8(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FDD1D8: 809E0028  lwz r4, 0x28(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDD1DC: 4BFF39A5  bl 0x82fd0b80
	ctx.lr = 0x82FDD1E0;
	sub_82FD0B80(ctx, base);
	// 82FDD1E0: 907E0008  stw r3, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 82FDD1E4: 809E0028  lwz r4, 0x28(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDD1E8: 807A000C  lwz r3, 0xc(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FDD1EC: 4BFF3995  bl 0x82fd0b80
	ctx.lr = 0x82FDD1F0;
	sub_82FD0B80(ctx, base);
	// 82FDD1F0: 907E000C  stw r3, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 82FDD1F4: 817A0010  lwz r11, 0x10(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FDD1F8: 809E0028  lwz r4, 0x28(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDD1FC: 917E0010  stw r11, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82FDD200: 807A0014  lwz r3, 0x14(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FDD204: 4BFF397D  bl 0x82fd0b80
	ctx.lr = 0x82FDD208;
	sub_82FD0B80(ctx, base);
	// 82FDD208: 817E0028  lwz r11, 0x28(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDD20C: 907E0014  stw r3, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 82FDD210: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82FDD214: 809E0018  lwz r4, 0x18(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FDD218: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDD21C: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FDD220: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDD224: 4E800421  bctrl
	ctx.lr = 0x82FDD228;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDD228: 807A0018  lwz r3, 0x18(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FDD22C: 809E0028  lwz r4, 0x28(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDD230: 4BFF3951  bl 0x82fd0b80
	ctx.lr = 0x82FDD234;
	sub_82FD0B80(ctx, base);
	// 82FDD234: 817E001C  lwz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FDD238: 907E0018  stw r3, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[3].u32 ) };
	// 82FDD23C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FDD240: 409A0014  bne cr6, 0x82fdd254
	if !ctx.cr[6].eq {
	pc = 0x82FDD254; continue 'dispatch;
	}
	// 82FDD244: 807A001C  lwz r3, 0x1c(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FDD248: 809E0028  lwz r4, 0x28(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDD24C: 4BFF3935  bl 0x82fd0b80
	ctx.lr = 0x82FDD250;
	sub_82FD0B80(ctx, base);
	// 82FDD250: 907E001C  stw r3, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[3].u32 ) };
	// 82FDD254: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FDD258: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 82FDD25C: 4BFF5865  bl 0x82fd2ac0
	ctx.lr = 0x82FDD260;
	sub_82FD2AC0(ctx, base);
	// 82FDD260: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FDD264: 387F0068  addi r3, r31, 0x68
	ctx.r[3].s64 = ctx.r[31].s64 + 104;
	// 82FDD268: 4BFF5859  bl 0x82fd2ac0
	ctx.lr = 0x82FDD26C;
	sub_82FD2AC0(ctx, base);
	// 82FDD26C: 480006D0  b 0x82fdd93c
	pc = 0x82FDD93C; continue 'dispatch;
	// 82FDD270: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDD274: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FDD278: 409A0628  bne cr6, 0x82fdd8a0
	if !ctx.cr[6].eq {
	pc = 0x82FDD8A0; continue 'dispatch;
	}
	// 82FDD27C: 807A0004  lwz r3, 4(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDD280: 809E0028  lwz r4, 0x28(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDD284: 4BFF38FD  bl 0x82fd0b80
	ctx.lr = 0x82FDD288;
	sub_82FD0B80(ctx, base);
	// 82FDD288: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FDD28C: 907E0004  stw r3, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82FDD290: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FDD294: 409A05F0  bne cr6, 0x82fdd884
	if !ctx.cr[6].eq {
	pc = 0x82FDD884; continue 'dispatch;
	}
	// 82FDD298: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FDD29C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FDD2A0: 409A05E4  bne cr6, 0x82fdd884
	if !ctx.cr[6].eq {
	pc = 0x82FDD884; continue 'dispatch;
	}
	// 82FDD2A4: 807E0028  lwz r3, 0x28(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDD2A8: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FDD2AC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDD2B0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FDD2B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDD2B8: 4E800421  bctrl
	ctx.lr = 0x82FDD2BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDD2BC: 807A0008  lwz r3, 8(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FDD2C0: 809E0028  lwz r4, 0x28(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDD2C4: 4BFF38BD  bl 0x82fd0b80
	ctx.lr = 0x82FDD2C8;
	sub_82FD0B80(ctx, base);
	// 82FDD2C8: 907E0008  stw r3, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 82FDD2CC: 809E0028  lwz r4, 0x28(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDD2D0: 807A000C  lwz r3, 0xc(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FDD2D4: 4BFF38AD  bl 0x82fd0b80
	ctx.lr = 0x82FDD2D8;
	sub_82FD0B80(ctx, base);
	// 82FDD2D8: 907E000C  stw r3, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 82FDD2DC: 817A0010  lwz r11, 0x10(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FDD2E0: 809E0028  lwz r4, 0x28(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDD2E4: 917E0010  stw r11, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82FDD2E8: 807A0014  lwz r3, 0x14(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FDD2EC: 4BFF3895  bl 0x82fd0b80
	ctx.lr = 0x82FDD2F0;
	sub_82FD0B80(ctx, base);
	// 82FDD2F0: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FDD2F4: 907E0014  stw r3, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 82FDD2F8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDD2FC: 41820040  beq 0x82fdd33c
	if ctx.cr[0].eq {
	pc = 0x82FDD33C; continue 'dispatch;
	}
	// 82FDD300: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDD304: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDD308: 41820034  beq 0x82fdd33c
	if ctx.cr[0].eq {
	pc = 0x82FDD33C; continue 'dispatch;
	}
	// 82FDD30C: 3895FFE0  addi r4, r21, -0x20
	ctx.r[4].s64 = ctx.r[21].s64 + -32;
	// 82FDD310: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82FDD314: 4BFFCA5D  bl 0x82fd9d70
	ctx.lr = 0x82FDD318;
	sub_82FD9D70(ctx, base);
	// 82FDD318: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDD31C: 41820020  beq 0x82fdd33c
	if ctx.cr[0].eq {
	pc = 0x82FDD33C; continue 'dispatch;
	}
	// 82FDD320: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FDD324: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 82FDD328: 4BFF5799  bl 0x82fd2ac0
	ctx.lr = 0x82FDD32C;
	sub_82FD2AC0(ctx, base);
	// 82FDD32C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FDD330: 387F0068  addi r3, r31, 0x68
	ctx.r[3].s64 = ctx.r[31].s64 + 104;
	// 82FDD334: 4BFF578D  bl 0x82fd2ac0
	ctx.lr = 0x82FDD338;
	sub_82FD2AC0(ctx, base);
	// 82FDD338: 48000604  b 0x82fdd93c
	pc = 0x82FDD93C; continue 'dispatch;
	// 82FDD33C: 807A0018  lwz r3, 0x18(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FDD340: 809E0028  lwz r4, 0x28(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDD344: 4BFF383D  bl 0x82fd0b80
	ctx.lr = 0x82FDD348;
	sub_82FD0B80(ctx, base);
	// 82FDD348: 815E0028  lwz r10, 0x28(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDD34C: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82FDD350: 937F0090  stw r27, 0x90(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), ctx.r[27].u32 ) };
	// 82FDD354: 915F0094  stw r10, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[10].u32 ) };
	// 82FDD358: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FDD35C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDD360: 41820034  beq 0x82fdd394
	if ctx.cr[0].eq {
	pc = 0x82FDD394; continue 'dispatch;
	}
	// 82FDD364: A12B0000  lhz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDD368: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDD36C: 41820028  beq 0x82fdd394
	if ctx.cr[0].eq {
	pc = 0x82FDD394; continue 'dispatch;
	}
	// 82FDD370: 392B0002  addi r9, r11, 2
	ctx.r[9].s64 = ctx.r[11].s64 + 2;
	// 82FDD374: 48000008  b 0x82fdd37c
	pc = 0x82FDD37C; continue 'dispatch;
	// 82FDD378: 39290002  addi r9, r9, 2
	ctx.r[9].s64 = ctx.r[9].s64 + 2;
	// 82FDD37C: A1090000  lhz r8, 0(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDD380: 28080000  cmplwi r8, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDD384: 4082FFF4  bne 0x82fdd378
	if !ctx.cr[0].eq {
	pc = 0x82FDD378; continue 'dispatch;
	}
	// 82FDD388: 7D6B4850  subf r11, r11, r9
	ctx.r[11].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 82FDD38C: 7D690E70  srawi r9, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82FDD390: 48000008  b 0x82fdd398
	pc = 0x82FDD398; continue 'dispatch;
	// 82FDD394: 7E89A378  mr r9, r20
	ctx.r[9].u64 = ctx.r[20].u64;
	// 82FDD398: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82FDD39C: 419A0034  beq cr6, 0x82fdd3d0
	if ctx.cr[6].eq {
	pc = 0x82FDD3D0; continue 'dispatch;
	}
	// 82FDD3A0: A17B0000  lhz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDD3A4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDD3A8: 41820028  beq 0x82fdd3d0
	if ctx.cr[0].eq {
	pc = 0x82FDD3D0; continue 'dispatch;
	}
	// 82FDD3AC: 397B0002  addi r11, r27, 2
	ctx.r[11].s64 = ctx.r[27].s64 + 2;
	// 82FDD3B0: 48000008  b 0x82fdd3b8
	pc = 0x82FDD3B8; continue 'dispatch;
	// 82FDD3B4: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FDD3B8: A10B0000  lhz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDD3BC: 28080000  cmplwi r8, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDD3C0: 4082FFF4  bne 0x82fdd3b4
	if !ctx.cr[0].eq {
	pc = 0x82FDD3B4; continue 'dispatch;
	}
	// 82FDD3C4: 7D7B5850  subf r11, r27, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[27].s64;
	// 82FDD3C8: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82FDD3CC: 48000008  b 0x82fdd3d4
	pc = 0x82FDD3D4; continue 'dispatch;
	// 82FDD3D0: 7E8BA378  mr r11, r20
	ctx.r[11].u64 = ctx.r[20].u64;
	// 82FDD3D4: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82FDD3D8: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDD3DC: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 82FDD3E0: 7D6BB214  add r11, r11, r22
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[22].u64;
	// 82FDD3E4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FDD3E8: 81490004  lwz r10, 4(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDD3EC: 557C083C  slwi r28, r11, 1
	ctx.r[28].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 82FDD3F0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FDD3F4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82FDD3F8: 4E800421  bctrl
	ctx.lr = 0x82FDD3FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDD3FC: 817E0028  lwz r11, 0x28(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDD400: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FDD404: 917F008C  stw r11, 0x8c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(140 as u32), ctx.r[11].u32 ) };
	// 82FDD408: 93BF0088  stw r29, 0x88(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(136 as u32), ctx.r[29].u32 ) };
	// 82FDD40C: B29D0000  sth r20, 0(r29)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[20].u16 ) };
	// 82FDD410: 807E0028  lwz r3, 0x28(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDD414: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FDD418: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDD41C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDD420: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDD424: 4E800421  bctrl
	ctx.lr = 0x82FDD428;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDD428: 817E0028  lwz r11, 0x28(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDD42C: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82FDD430: 917F0084  stw r11, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 82FDD434: 935F0080  stw r26, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[26].u32 ) };
	// 82FDD438: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDD43C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82FDD440: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FDD444: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDD448: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDD44C: 4E800421  bctrl
	ctx.lr = 0x82FDD450;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDD450: 817E0028  lwz r11, 0x28(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDD454: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82FDD458: 917F007C  stw r11, 0x7c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 82FDD45C: 931F0078  stw r24, 0x78(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(120 as u32), ctx.r[24].u32 ) };
	// 82FDD460: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82FDD464: 419A0064  beq cr6, 0x82fdd4c8
	if ctx.cr[6].eq {
	pc = 0x82FDD4C8; continue 'dispatch;
	}
	// 82FDD468: A17B0000  lhz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDD46C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDD470: 41820028  beq 0x82fdd498
	if ctx.cr[0].eq {
	pc = 0x82FDD498; continue 'dispatch;
	}
	// 82FDD474: 397B0002  addi r11, r27, 2
	ctx.r[11].s64 = ctx.r[27].s64 + 2;
	// 82FDD478: 48000008  b 0x82fdd480
	pc = 0x82FDD480; continue 'dispatch;
	// 82FDD47C: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FDD480: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDD484: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDD488: 4082FFF4  bne 0x82fdd47c
	if !ctx.cr[0].eq {
	pc = 0x82FDD47C; continue 'dispatch;
	}
	// 82FDD48C: 7D7B5850  subf r11, r27, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[27].s64;
	// 82FDD490: 7D650E70  srawi r5, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[5].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82FDD494: 48000008  b 0x82fdd49c
	pc = 0x82FDD49C; continue 'dispatch;
	// 82FDD498: 7E85A378  mr r5, r20
	ctx.r[5].u64 = ctx.r[20].u64;
	// 82FDD49C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82FDD4A0: 3860002F  li r3, 0x2f
	ctx.r[3].s64 = 47;
	// 82FDD4A4: 4BFF4A25  bl 0x82fd1ec8
	ctx.lr = 0x82FDD4A8;
	sub_82FD1EC8(ctx, base);
	// 82FDD4A8: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82FDD4AC: 419A001C  beq cr6, 0x82fdd4c8
	if ctx.cr[6].eq {
	pc = 0x82FDD4C8; continue 'dispatch;
	}
	// 82FDD4B0: 38C30001  addi r6, r3, 1
	ctx.r[6].s64 = ctx.r[3].s64 + 1;
	// 82FDD4B4: 80FE0028  lwz r7, 0x28(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDD4B8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FDD4BC: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82FDD4C0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FDD4C4: 4BFF4BC5  bl 0x82fd2088
	ctx.lr = 0x82FDD4C8;
	sub_82FD2088(ctx, base);
	// 82FDD4C8: 809E0018  lwz r4, 0x18(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FDD4CC: 48000078  b 0x82fdd544
	pc = 0x82FDD544; continue 'dispatch;
	// 82FDD4D0: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82FDD4D4: 80FE0028  lwz r7, 0x28(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDD4D8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FDD4DC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FDD4E0: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82FDD4E4: 4BFF4BA5  bl 0x82fd2088
	ctx.lr = 0x82FDD4E8;
	sub_82FD2088(ctx, base);
	// 82FDD4E8: A17D0000  lhz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDD4EC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDD4F0: 41820028  beq 0x82fdd518
	if ctx.cr[0].eq {
	pc = 0x82FDD518; continue 'dispatch;
	}
	// 82FDD4F4: 397D0002  addi r11, r29, 2
	ctx.r[11].s64 = ctx.r[29].s64 + 2;
	// 82FDD4F8: 48000008  b 0x82fdd500
	pc = 0x82FDD500; continue 'dispatch;
	// 82FDD4FC: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FDD500: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDD504: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDD508: 4082FFF4  bne 0x82fdd4fc
	if !ctx.cr[0].eq {
	pc = 0x82FDD4FC; continue 'dispatch;
	}
	// 82FDD50C: 7D7D5850  subf r11, r29, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	// 82FDD510: 7D660E70  srawi r6, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[6].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82FDD514: 48000008  b 0x82fdd51c
	pc = 0x82FDD51C; continue 'dispatch;
	// 82FDD518: 7E86A378  mr r6, r20
	ctx.r[6].u64 = ctx.r[20].u64;
	// 82FDD51C: 38BC0002  addi r5, r28, 2
	ctx.r[5].s64 = ctx.r[28].s64 + 2;
	// 82FDD520: 80FE0028  lwz r7, 0x28(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDD524: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FDD528: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82FDD52C: 4BFF4B5D  bl 0x82fd2088
	ctx.lr = 0x82FDD530;
	sub_82FD2088(ctx, base);
	// 82FDD530: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82FDD534: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FDD538: B29D0000  sth r20, 0(r29)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[20].u16 ) };
	// 82FDD53C: 4BFF435D  bl 0x82fd1898
	ctx.lr = 0x82FDD540;
	sub_82FD1898(ctx, base);
	// 82FDD540: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82FDD544: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FDD548: 4BFF4351  bl 0x82fd1898
	ctx.lr = 0x82FDD54C;
	sub_82FD1898(ctx, base);
	// 82FDD54C: 3895FFE4  addi r4, r21, -0x1c
	ctx.r[4].s64 = ctx.r[21].s64 + -28;
	// 82FDD550: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FDD554: 4BFF479D  bl 0x82fd1cf0
	ctx.lr = 0x82FDD558;
	sub_82FD1CF0(ctx, base);
	// 82FDD558: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82FDD55C: 2F1CFFFF  cmpwi cr6, r28, -1
	ctx.cr[6].compare_i32(ctx.r[28].s32, -1, &mut ctx.xer);
	// 82FDD560: 409AFF70  bne cr6, 0x82fdd4d0
	if !ctx.cr[6].eq {
	pc = 0x82FDD4D0; continue 'dispatch;
	}
	// 82FDD564: 3895FFEC  addi r4, r21, -0x14
	ctx.r[4].s64 = ctx.r[21].s64 + -20;
	// 82FDD568: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FDD56C: 4BFFC86D  bl 0x82fd9dd8
	ctx.lr = 0x82FDD570;
	sub_82FD9DD8(ctx, base);
	// 82FDD570: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDD574: 41820044  beq 0x82fdd5b8
	if ctx.cr[0].eq {
	pc = 0x82FDD5B8; continue 'dispatch;
	}
	// 82FDD578: A17D0000  lhz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDD57C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDD580: 41820028  beq 0x82fdd5a8
	if ctx.cr[0].eq {
	pc = 0x82FDD5A8; continue 'dispatch;
	}
	// 82FDD584: 397D0002  addi r11, r29, 2
	ctx.r[11].s64 = ctx.r[29].s64 + 2;
	// 82FDD588: 48000008  b 0x82fdd590
	pc = 0x82FDD590; continue 'dispatch;
	// 82FDD58C: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FDD590: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDD594: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDD598: 4082FFF4  bne 0x82fdd58c
	if !ctx.cr[0].eq {
	pc = 0x82FDD58C; continue 'dispatch;
	}
	// 82FDD59C: 7D7D5850  subf r11, r29, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	// 82FDD5A0: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82FDD5A4: 48000008  b 0x82fdd5ac
	pc = 0x82FDD5AC; continue 'dispatch;
	// 82FDD5A8: 7E8BA378  mr r11, r20
	ctx.r[11].u64 = ctx.r[20].u64;
	// 82FDD5AC: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FDD5B0: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82FDD5B4: B28BFFFE  sth r20, -2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(-2 as u32), ctx.r[20].u16 ) };
	// 82FDD5B8: 3AFD0002  addi r23, r29, 2
	ctx.r[23].s64 = ctx.r[29].s64 + 2;
	// 82FDD5BC: 3B600001  li r27, 1
	ctx.r[27].s64 = 1;
	// 82FDD5C0: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82FDD5C4: 48000148  b 0x82fdd70c
	pc = 0x82FDD70C; continue 'dispatch;
	// 82FDD5C8: 7F3B1A14  add r25, r27, r3
	ctx.r[25].u64 = ctx.r[27].u64 + ctx.r[3].u64;
	// 82FDD5CC: 80FE0028  lwz r7, 0x28(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDD5D0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FDD5D4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FDD5D8: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82FDD5DC: 38D9FFFF  addi r6, r25, -1
	ctx.r[6].s64 = ctx.r[25].s64 + -1;
	// 82FDD5E0: 4BFF4AA9  bl 0x82fd2088
	ctx.lr = 0x82FDD5E4;
	sub_82FD2088(ctx, base);
	// 82FDD5E4: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 82FDD5E8: 419A0034  beq cr6, 0x82fdd61c
	if ctx.cr[6].eq {
	pc = 0x82FDD61C; continue 'dispatch;
	}
	// 82FDD5EC: A17A0000  lhz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDD5F0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDD5F4: 41820028  beq 0x82fdd61c
	if ctx.cr[0].eq {
	pc = 0x82FDD61C; continue 'dispatch;
	}
	// 82FDD5F8: 397A0002  addi r11, r26, 2
	ctx.r[11].s64 = ctx.r[26].s64 + 2;
	// 82FDD5FC: 48000008  b 0x82fdd604
	pc = 0x82FDD604; continue 'dispatch;
	// 82FDD600: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FDD604: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDD608: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDD60C: 4082FFF4  bne 0x82fdd600
	if !ctx.cr[0].eq {
	pc = 0x82FDD600; continue 'dispatch;
	}
	// 82FDD610: 7D7A5850  subf r11, r26, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[26].s64;
	// 82FDD614: 7D650E70  srawi r5, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[5].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82FDD618: 48000008  b 0x82fdd620
	pc = 0x82FDD620; continue 'dispatch;
	// 82FDD61C: 7E85A378  mr r5, r20
	ctx.r[5].u64 = ctx.r[20].u64;
	// 82FDD620: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82FDD624: 3860002F  li r3, 0x2f
	ctx.r[3].s64 = 47;
	// 82FDD628: 4BFF48A1  bl 0x82fd1ec8
	ctx.lr = 0x82FDD62C;
	sub_82FD1EC8(ctx, base);
	// 82FDD62C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82FDD630: 2F1CFFFF  cmpwi cr6, r28, -1
	ctx.cr[6].compare_i32(ctx.r[28].s32, -1, &mut ctx.xer);
	// 82FDD634: 419A00CC  beq cr6, 0x82fdd700
	if ctx.cr[6].eq {
	pc = 0x82FDD700; continue 'dispatch;
	}
	// 82FDD638: 397C0001  addi r11, r28, 1
	ctx.r[11].s64 = ctx.r[28].s64 + 1;
	// 82FDD63C: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FDD640: 7D6BEA2E  lhzx r11, r11, r29
	ctx.r[11].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82FDD644: 2B0B002E  cmplwi cr6, r11, 0x2e
	ctx.cr[6].compare_u32(ctx.r[11].u32, 46 as u32, &mut ctx.xer);
	// 82FDD648: 409A0024  bne cr6, 0x82fdd66c
	if !ctx.cr[6].eq {
	pc = 0x82FDD66C; continue 'dispatch;
	}
	// 82FDD64C: 397C0002  addi r11, r28, 2
	ctx.r[11].s64 = ctx.r[28].s64 + 2;
	// 82FDD650: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FDD654: 7D6BEA2E  lhzx r11, r11, r29
	ctx.r[11].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82FDD658: 2B0B002E  cmplwi cr6, r11, 0x2e
	ctx.cr[6].compare_u32(ctx.r[11].u32, 46 as u32, &mut ctx.xer);
	// 82FDD65C: 409A0010  bne cr6, 0x82fdd66c
	if !ctx.cr[6].eq {
	pc = 0x82FDD66C; continue 'dispatch;
	}
	// 82FDD660: 397C0003  addi r11, r28, 3
	ctx.r[11].s64 = ctx.r[28].s64 + 3;
	// 82FDD664: 7F0BC800  cmpw cr6, r11, r25
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[25].s32, &mut ctx.xer);
	// 82FDD668: 419A0098  beq cr6, 0x82fdd700
	if ctx.cr[6].eq {
	pc = 0x82FDD700; continue 'dispatch;
	}
	// 82FDD66C: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82FDD670: 80FE0028  lwz r7, 0x28(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDD674: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FDD678: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FDD67C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82FDD680: 4BFF4A09  bl 0x82fd2088
	ctx.lr = 0x82FDD684;
	sub_82FD2088(ctx, base);
	// 82FDD684: A17D0000  lhz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDD688: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDD68C: 41820028  beq 0x82fdd6b4
	if ctx.cr[0].eq {
	pc = 0x82FDD6B4; continue 'dispatch;
	}
	// 82FDD690: 7EEBBB78  mr r11, r23
	ctx.r[11].u64 = ctx.r[23].u64;
	// 82FDD694: 48000008  b 0x82fdd69c
	pc = 0x82FDD69C; continue 'dispatch;
	// 82FDD698: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FDD69C: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDD6A0: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDD6A4: 4082FFF4  bne 0x82fdd698
	if !ctx.cr[0].eq {
	pc = 0x82FDD698; continue 'dispatch;
	}
	// 82FDD6A8: 7D7D5850  subf r11, r29, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	// 82FDD6AC: 7D660E70  srawi r6, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[6].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82FDD6B0: 48000008  b 0x82fdd6b8
	pc = 0x82FDD6B8; continue 'dispatch;
	// 82FDD6B4: 7E86A378  mr r6, r20
	ctx.r[6].u64 = ctx.r[20].u64;
	// 82FDD6B8: 38B90003  addi r5, r25, 3
	ctx.r[5].s64 = ctx.r[25].s64 + 3;
	// 82FDD6BC: 80FE0028  lwz r7, 0x28(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDD6C0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FDD6C4: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82FDD6C8: 4BFF49C1  bl 0x82fd2088
	ctx.lr = 0x82FDD6CC;
	sub_82FD2088(ctx, base);
	// 82FDD6CC: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82FDD6D0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FDD6D4: B29D0000  sth r20, 0(r29)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[20].u16 ) };
	// 82FDD6D8: 4BFF41C1  bl 0x82fd1898
	ctx.lr = 0x82FDD6DC;
	sub_82FD1898(ctx, base);
	// 82FDD6DC: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82FDD6E0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FDD6E4: 4BFF41B5  bl 0x82fd1898
	ctx.lr = 0x82FDD6E8;
	sub_82FD1898(ctx, base);
	// 82FDD6E8: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82FDD6EC: 409A000C  bne cr6, 0x82fdd6f8
	if !ctx.cr[6].eq {
	pc = 0x82FDD6F8; continue 'dispatch;
	}
	// 82FDD6F0: 3B600001  li r27, 1
	ctx.r[27].s64 = 1;
	// 82FDD6F4: 48000010  b 0x82fdd704
	pc = 0x82FDD704; continue 'dispatch;
	// 82FDD6F8: 7F9BE378  mr r27, r28
	ctx.r[27].u64 = ctx.r[28].u64;
	// 82FDD6FC: 48000008  b 0x82fdd704
	pc = 0x82FDD704; continue 'dispatch;
	// 82FDD700: 3B7B0004  addi r27, r27, 4
	ctx.r[27].s64 = ctx.r[27].s64 + 4;
	// 82FDD704: 576B083C  slwi r11, r27, 1
	ctx.r[11].u32 = ctx.r[27].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FDD708: 7C6BEA14  add r3, r11, r29
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82FDD70C: 3895FFF4  addi r4, r21, -0xc
	ctx.r[4].s64 = ctx.r[21].s64 + -12;
	// 82FDD710: 4BFF45E1  bl 0x82fd1cf0
	ctx.lr = 0x82FDD714;
	sub_82FD1CF0(ctx, base);
	// 82FDD714: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82FDD718: 409AFEB0  bne cr6, 0x82fdd5c8
	if !ctx.cr[6].eq {
	pc = 0x82FDD5C8; continue 'dispatch;
	}
	// 82FDD71C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FDD720: 7EA4AB78  mr r4, r21
	ctx.r[4].u64 = ctx.r[21].u64;
	// 82FDD724: 4BFFC6B5  bl 0x82fd9dd8
	ctx.lr = 0x82FDD728;
	sub_82FD9DD8(ctx, base);
	// 82FDD728: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDD72C: 418200DC  beq 0x82fdd808
	if ctx.cr[0].eq {
	pc = 0x82FDD808; continue 'dispatch;
	}
	// 82FDD730: A17D0000  lhz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDD734: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDD738: 41820028  beq 0x82fdd760
	if ctx.cr[0].eq {
	pc = 0x82FDD760; continue 'dispatch;
	}
	// 82FDD73C: 7EEBBB78  mr r11, r23
	ctx.r[11].u64 = ctx.r[23].u64;
	// 82FDD740: 48000008  b 0x82fdd748
	pc = 0x82FDD748; continue 'dispatch;
	// 82FDD744: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FDD748: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDD74C: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDD750: 4082FFF4  bne 0x82fdd744
	if !ctx.cr[0].eq {
	pc = 0x82FDD744; continue 'dispatch;
	}
	// 82FDD754: 7D7D5850  subf r11, r29, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	// 82FDD758: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82FDD75C: 48000008  b 0x82fdd764
	pc = 0x82FDD764; continue 'dispatch;
	// 82FDD760: 7E8BA378  mr r11, r20
	ctx.r[11].u64 = ctx.r[20].u64;
	// 82FDD764: 3B8BFFFD  addi r28, r11, -3
	ctx.r[28].s64 = ctx.r[11].s64 + -3;
	// 82FDD768: 80FE0028  lwz r7, 0x28(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDD76C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FDD770: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FDD774: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82FDD778: 38DCFFFF  addi r6, r28, -1
	ctx.r[6].s64 = ctx.r[28].s64 + -1;
	// 82FDD77C: 4BFF490D  bl 0x82fd2088
	ctx.lr = 0x82FDD780;
	sub_82FD2088(ctx, base);
	// 82FDD780: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 82FDD784: 419A0034  beq cr6, 0x82fdd7b8
	if ctx.cr[6].eq {
	pc = 0x82FDD7B8; continue 'dispatch;
	}
	// 82FDD788: A17A0000  lhz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDD78C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDD790: 41820028  beq 0x82fdd7b8
	if ctx.cr[0].eq {
	pc = 0x82FDD7B8; continue 'dispatch;
	}
	// 82FDD794: 397A0002  addi r11, r26, 2
	ctx.r[11].s64 = ctx.r[26].s64 + 2;
	// 82FDD798: 48000008  b 0x82fdd7a0
	pc = 0x82FDD7A0; continue 'dispatch;
	// 82FDD79C: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FDD7A0: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDD7A4: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDD7A8: 4082FFF4  bne 0x82fdd79c
	if !ctx.cr[0].eq {
	pc = 0x82FDD79C; continue 'dispatch;
	}
	// 82FDD7AC: 7D7A5850  subf r11, r26, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[26].s64;
	// 82FDD7B0: 7D650E70  srawi r5, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[5].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82FDD7B4: 48000008  b 0x82fdd7bc
	pc = 0x82FDD7BC; continue 'dispatch;
	// 82FDD7B8: 7E85A378  mr r5, r20
	ctx.r[5].u64 = ctx.r[20].u64;
	// 82FDD7BC: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82FDD7C0: 3860002F  li r3, 0x2f
	ctx.r[3].s64 = 47;
	// 82FDD7C4: 4BFF4705  bl 0x82fd1ec8
	ctx.lr = 0x82FDD7C8;
	sub_82FD1EC8(ctx, base);
	// 82FDD7C8: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82FDD7CC: 419A003C  beq cr6, 0x82fdd808
	if ctx.cr[6].eq {
	pc = 0x82FDD808; continue 'dispatch;
	}
	// 82FDD7D0: 39630001  addi r11, r3, 1
	ctx.r[11].s64 = ctx.r[3].s64 + 1;
	// 82FDD7D4: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FDD7D8: 7D4BEA2E  lhzx r10, r11, r29
	ctx.r[10].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82FDD7DC: 2B0A002E  cmplwi cr6, r10, 0x2e
	ctx.cr[6].compare_u32(ctx.r[10].u32, 46 as u32, &mut ctx.xer);
	// 82FDD7E0: 409A0024  bne cr6, 0x82fdd804
	if !ctx.cr[6].eq {
	pc = 0x82FDD804; continue 'dispatch;
	}
	// 82FDD7E4: 39430002  addi r10, r3, 2
	ctx.r[10].s64 = ctx.r[3].s64 + 2;
	// 82FDD7E8: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FDD7EC: 7D4AEA2E  lhzx r10, r10, r29
	ctx.r[10].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82FDD7F0: 2B0A002E  cmplwi cr6, r10, 0x2e
	ctx.cr[6].compare_u32(ctx.r[10].u32, 46 as u32, &mut ctx.xer);
	// 82FDD7F4: 409A0010  bne cr6, 0x82fdd804
	if !ctx.cr[6].eq {
	pc = 0x82FDD804; continue 'dispatch;
	}
	// 82FDD7F8: 39430003  addi r10, r3, 3
	ctx.r[10].s64 = ctx.r[3].s64 + 3;
	// 82FDD7FC: 7F0AE000  cmpw cr6, r10, r28
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[28].s32, &mut ctx.xer);
	// 82FDD800: 419A0008  beq cr6, 0x82fdd808
	if ctx.cr[6].eq {
	pc = 0x82FDD808; continue 'dispatch;
	}
	// 82FDD804: 7E8BEB2E  sthx r20, r11, r29
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32), ctx.r[20].u16) };
	// 82FDD808: 809E0018  lwz r4, 0x18(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FDD80C: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDD810: 41820018  beq 0x82fdd828
	if ctx.cr[0].eq {
	pc = 0x82FDD828; continue 'dispatch;
	}
	// 82FDD814: 807E0028  lwz r3, 0x28(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDD818: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDD81C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FDD820: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDD824: 4E800421  bctrl
	ctx.lr = 0x82FDD828;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDD828: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FDD82C: 809E0028  lwz r4, 0x28(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDD830: 4BFF3351  bl 0x82fd0b80
	ctx.lr = 0x82FDD834;
	sub_82FD0B80(ctx, base);
	// 82FDD834: 907E0018  stw r3, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[3].u32 ) };
	// 82FDD838: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FDD83C: 387F0078  addi r3, r31, 0x78
	ctx.r[3].s64 = ctx.r[31].s64 + 120;
	// 82FDD840: 4BFF5281  bl 0x82fd2ac0
	ctx.lr = 0x82FDD844;
	sub_82FD2AC0(ctx, base);
	// 82FDD844: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FDD848: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 82FDD84C: 4BFF5275  bl 0x82fd2ac0
	ctx.lr = 0x82FDD850;
	sub_82FD2AC0(ctx, base);
	// 82FDD850: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FDD854: 387F0088  addi r3, r31, 0x88
	ctx.r[3].s64 = ctx.r[31].s64 + 136;
	// 82FDD858: 4BFF5269  bl 0x82fd2ac0
	ctx.lr = 0x82FDD85C;
	sub_82FD2AC0(ctx, base);
	// 82FDD85C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FDD860: 387F0090  addi r3, r31, 0x90
	ctx.r[3].s64 = ctx.r[31].s64 + 144;
	// 82FDD864: 4BFF525D  bl 0x82fd2ac0
	ctx.lr = 0x82FDD868;
	sub_82FD2AC0(ctx, base);
	// 82FDD868: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FDD86C: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 82FDD870: 4BFF5251  bl 0x82fd2ac0
	ctx.lr = 0x82FDD874;
	sub_82FD2AC0(ctx, base);
	// 82FDD874: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FDD878: 387F0068  addi r3, r31, 0x68
	ctx.r[3].s64 = ctx.r[31].s64 + 104;
	// 82FDD87C: 4BFF5245  bl 0x82fd2ac0
	ctx.lr = 0x82FDD880;
	sub_82FD2AC0(ctx, base);
	// 82FDD880: 480000BC  b 0x82fdd93c
	pc = 0x82FDD93C; continue 'dispatch;
	// 82FDD884: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FDD888: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 82FDD88C: 4BFF5235  bl 0x82fd2ac0
	ctx.lr = 0x82FDD890;
	sub_82FD2AC0(ctx, base);
	// 82FDD890: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FDD894: 387F0068  addi r3, r31, 0x68
	ctx.r[3].s64 = ctx.r[31].s64 + 104;
	// 82FDD898: 4BFF5229  bl 0x82fd2ac0
	ctx.lr = 0x82FDD89C;
	sub_82FD2AC0(ctx, base);
	// 82FDD89C: 480000A0  b 0x82fdd93c
	pc = 0x82FDD93C; continue 'dispatch;
	// 82FDD8A0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FDD8A4: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 82FDD8A8: 4BFF5219  bl 0x82fd2ac0
	ctx.lr = 0x82FDD8AC;
	sub_82FD2AC0(ctx, base);
	// 82FDD8AC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FDD8B0: 387F0068  addi r3, r31, 0x68
	ctx.r[3].s64 = ctx.r[31].s64 + 104;
	// 82FDD8B4: 4BFF520D  bl 0x82fd2ac0
	ctx.lr = 0x82FDD8B8;
	sub_82FD2AC0(ctx, base);
	// 82FDD8B8: 48000084  b 0x82fdd93c
	pc = 0x82FDD93C; continue 'dispatch;
	// 82FDD8BC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FDD8C0: 80DE0028  lwz r6, 0x28(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDD8C4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FDD8C8: 396B9CC0  addi r11, r11, -0x6340
	ctx.r[11].s64 = ctx.r[11].s64 + -25408;
	// 82FDD8CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82FDD8D0: 38EBFFAC  addi r7, r11, -0x54
	ctx.r[7].s64 = ctx.r[11].s64 + -84;
	// 82FDD8D4: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FDD8D8: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 82FDD8DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82FDD8E0: 388B9D88  addi r4, r11, -0x6278
	ctx.r[4].s64 = ctx.r[11].s64 + -25208;
	// 82FDD8E4: 38C0010A  li r6, 0x10a
	ctx.r[6].s64 = 266;
	// 82FDD8E8: 38A001C3  li r5, 0x1c3
	ctx.r[5].s64 = 451;
	// 82FDD8EC: 387F00A0  addi r3, r31, 0xa0
	ctx.r[3].s64 = ctx.r[31].s64 + 160;
	// 82FDD8F0: 4BFFC2E1  bl 0x82fd9bd0
	ctx.lr = 0x82FDD8F4;
	sub_82FD9BD0(ctx, base);
	// 82FDD8F4: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FDD8F8: 387F00A0  addi r3, r31, 0xa0
	ctx.r[3].s64 = ctx.r[31].s64 + 160;
	// 82FDD8FC: 388BC5E4  addi r4, r11, -0x3a1c
	ctx.r[4].s64 = ctx.r[11].s64 + -14876;
	// 82FDD900: 481D3329  bl 0x831b0c28
	ctx.lr = 0x82FDD904;
	sub_831B0C28(ctx, base);
	// 82FDD904: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FDD908: 80FE0028  lwz r7, 0x28(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDD90C: 38C00110  li r6, 0x110
	ctx.r[6].s64 = 272;
	// 82FDD910: 388B9D88  addi r4, r11, -0x6278
	ctx.r[4].s64 = ctx.r[11].s64 + -25208;
	// 82FDD914: 38A001B3  li r5, 0x1b3
	ctx.r[5].s64 = 435;
	// 82FDD918: 387F00A0  addi r3, r31, 0xa0
	ctx.r[3].s64 = ctx.r[31].s64 + 160;
	// 82FDD91C: 4BFFC1FD  bl 0x82fd9b18
	ctx.lr = 0x82FDD920;
	sub_82FD9B18(ctx, base);
	// 82FDD920: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FDD924: 387F00A0  addi r3, r31, 0xa0
	ctx.r[3].s64 = ctx.r[31].s64 + 160;
	// 82FDD928: 388BC5E4  addi r4, r11, -0x3a1c
	ctx.r[4].s64 = ctx.r[11].s64 + -14876;
	// 82FDD92C: 481D32FD  bl 0x831b0c28
	ctx.lr = 0x82FDD930;
	sub_831B0C28(ctx, base);
	// 82FDD930: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82FDD934: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FDD938: 4BFFC6F9  bl 0x82fda030
	ctx.lr = 0x82FDD93C;
	sub_82FDA030(ctx, base);
	// 82FDD93C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FDD940: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FDD944: 4BFF517D  bl 0x82fd2ac0
	ctx.lr = 0x82FDD948;
	sub_82FD2AC0(ctx, base);
	// 82FDD948: 383F0120  addi r1, r31, 0x120
	ctx.r[1].s64 = ctx.r[31].s64 + 288;
	// 82FDD94C: 481CA84C  b 0x831a8198
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDD950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDD950 size=40
    let mut pc: u32 = 0x82FDD950;
    'dispatch: loop {
        match pc {
            0x82FDD950 => {
    //   block [0x82FDD950..0x82FDD978)
	// 82FDD950: 3BECFEE0  addi r31, r12, -0x120
	ctx.r[31].s64 = ctx.r[12].s64 + -288;
	// 82FDD954: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDD958: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FDD95C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDD960: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FDD964: 4BFF54F5  bl 0x82fd2e58
	ctx.lr = 0x82FDD968;
	sub_82FD2E58(ctx, base);
	// 82FDD968: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FDD96C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FDD970: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FDD974: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDD978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDD978 size=40
    let mut pc: u32 = 0x82FDD978;
    'dispatch: loop {
        match pc {
            0x82FDD978 => {
    //   block [0x82FDD978..0x82FDD9A0)
	// 82FDD978: 3BECFEE0  addi r31, r12, -0x120
	ctx.r[31].s64 = ctx.r[12].s64 + -288;
	// 82FDD97C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDD980: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FDD984: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDD988: 387F0068  addi r3, r31, 0x68
	ctx.r[3].s64 = ctx.r[31].s64 + 104;
	// 82FDD98C: 4BFF54CD  bl 0x82fd2e58
	ctx.lr = 0x82FDD990;
	sub_82FD2E58(ctx, base);
	// 82FDD990: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FDD994: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FDD998: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FDD99C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDD9A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDD9A0 size=40
    let mut pc: u32 = 0x82FDD9A0;
    'dispatch: loop {
        match pc {
            0x82FDD9A0 => {
    //   block [0x82FDD9A0..0x82FDD9C8)
	// 82FDD9A0: 3BECFEE0  addi r31, r12, -0x120
	ctx.r[31].s64 = ctx.r[12].s64 + -288;
	// 82FDD9A4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDD9A8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FDD9AC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDD9B0: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 82FDD9B4: 4BFF54A5  bl 0x82fd2e58
	ctx.lr = 0x82FDD9B8;
	sub_82FD2E58(ctx, base);
	// 82FDD9B8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FDD9BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FDD9C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FDD9C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDD9C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDD9C8 size=40
    let mut pc: u32 = 0x82FDD9C8;
    'dispatch: loop {
        match pc {
            0x82FDD9C8 => {
    //   block [0x82FDD9C8..0x82FDD9F0)
	// 82FDD9C8: 3BECFEE0  addi r31, r12, -0x120
	ctx.r[31].s64 = ctx.r[12].s64 + -288;
	// 82FDD9CC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDD9D0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FDD9D4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDD9D8: 387F0090  addi r3, r31, 0x90
	ctx.r[3].s64 = ctx.r[31].s64 + 144;
	// 82FDD9DC: 4BFF547D  bl 0x82fd2e58
	ctx.lr = 0x82FDD9E0;
	sub_82FD2E58(ctx, base);
	// 82FDD9E0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FDD9E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FDD9E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FDD9EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDD9F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDD9F0 size=40
    let mut pc: u32 = 0x82FDD9F0;
    'dispatch: loop {
        match pc {
            0x82FDD9F0 => {
    //   block [0x82FDD9F0..0x82FDDA18)
	// 82FDD9F0: 3BECFEE0  addi r31, r12, -0x120
	ctx.r[31].s64 = ctx.r[12].s64 + -288;
	// 82FDD9F4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDD9F8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FDD9FC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDDA00: 387F0088  addi r3, r31, 0x88
	ctx.r[3].s64 = ctx.r[31].s64 + 136;
	// 82FDDA04: 4BFF5455  bl 0x82fd2e58
	ctx.lr = 0x82FDDA08;
	sub_82FD2E58(ctx, base);
	// 82FDDA08: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FDDA0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FDDA10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FDDA14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDDA18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDDA18 size=40
    let mut pc: u32 = 0x82FDDA18;
    'dispatch: loop {
        match pc {
            0x82FDDA18 => {
    //   block [0x82FDDA18..0x82FDDA40)
	// 82FDDA18: 3BECFEE0  addi r31, r12, -0x120
	ctx.r[31].s64 = ctx.r[12].s64 + -288;
	// 82FDDA1C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDDA20: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FDDA24: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDDA28: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 82FDDA2C: 4BFF542D  bl 0x82fd2e58
	ctx.lr = 0x82FDDA30;
	sub_82FD2E58(ctx, base);
	// 82FDDA30: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FDDA34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FDDA38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FDDA3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDDA40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDDA40 size=40
    let mut pc: u32 = 0x82FDDA40;
    'dispatch: loop {
        match pc {
            0x82FDDA40 => {
    //   block [0x82FDDA40..0x82FDDA68)
	// 82FDDA40: 3BECFEE0  addi r31, r12, -0x120
	ctx.r[31].s64 = ctx.r[12].s64 + -288;
	// 82FDDA44: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDDA48: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FDDA4C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDDA50: 387F0078  addi r3, r31, 0x78
	ctx.r[3].s64 = ctx.r[31].s64 + 120;
	// 82FDDA54: 4BFF5405  bl 0x82fd2e58
	ctx.lr = 0x82FDDA58;
	sub_82FD2E58(ctx, base);
	// 82FDDA58: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FDDA5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FDDA60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FDDA64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDDA68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FDDA68 size=8
    let mut pc: u32 = 0x82FDDA68;
    'dispatch: loop {
        match pc {
            0x82FDDA68 => {
    //   block [0x82FDDA68..0x82FDDA70)
	// 82FDDA68: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FDDA6C: 8213A28C  lwz r16, -0x5d74(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-23924 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDDA70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDDA70 size=148
    let mut pc: u32 = 0x82FDDA70;
    'dispatch: loop {
        match pc {
            0x82FDDA70 => {
    //   block [0x82FDDA70..0x82FDDB04)
	// 82FDDA70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDDA74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FDDA78: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 82FDDA7C: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 82FDDA80: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FDDA84: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FDDA88: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 82FDDA8C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDDA90: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FDDA94: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 82FDDA98: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FDDA9C: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 82FDDAA0: 90BE0028  stw r5, 0x28(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), ctx.r[5].u32 ) };
	// 82FDDAA4: 394B9DB4  addi r10, r11, -0x624c
	ctx.r[10].s64 = ctx.r[11].s64 + -25164;
	// 82FDDAA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FDDAAC: 913E0010  stw r9, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 82FDDAB0: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82FDDAB4: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82FDDAB8: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82FDDABC: 917E000C  stw r11, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82FDDAC0: 917E0014  stw r11, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82FDDAC4: 917E0018  stw r11, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82FDDAC8: 917E001C  stw r11, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82FDDACC: 917E0020  stw r11, 0x20(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82FDDAD0: 917E0024  stw r11, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82FDDAD4: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82FDDAD8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FDDADC: 4BFFF355  bl 0x82fdce30
	ctx.lr = 0x82FDDAE0;
	sub_82FDCE30(ctx, base);
	// 82FDDAE0: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FDDAE4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FDDAE8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FDDAEC: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 82FDDAF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FDDAF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FDDAF8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FDDAFC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FDDB00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDDB04(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FDDB04 size=8
    let mut pc: u32 = 0x82FDDB04;
    'dispatch: loop {
        match pc {
            0x82FDDB04 => {
    //   block [0x82FDDB04..0x82FDDB0C)
	// 82FDDB04: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FDDB08: 8213A28C  lwz r16, -0x5d74(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-23924 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDDB0C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDDB0C size=24
    let mut pc: u32 = 0x82FDDB0C;
    'dispatch: loop {
        match pc {
            0x82FDDB0C => {
    //   block [0x82FDDB0C..0x82FDDB24)
	// 82FDDB0C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDDB10: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FDDB14: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDDB18: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FDDB1C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FDDB20: 481D3109  bl 0x831b0c28
	ctx.lr = 0x82FDDB24;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDDB2C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDDB2C size=36
    let mut pc: u32 = 0x82FDDB2C;
    'dispatch: loop {
        match pc {
            0x82FDDB2C => {
    //   block [0x82FDDB2C..0x82FDDB50)
	// 82FDDB2C: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 82FDDB30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDDB34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FDDB38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDDB3C: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82FDDB40: 4BFFC3D1  bl 0x82fd9f10
	ctx.lr = 0x82FDDB44;
	sub_82FD9F10(ctx, base);
	// 82FDDB44: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FDDB48: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FDDB4C: 481D30DD  bl 0x831b0c28
	ctx.lr = 0x82FDDB50;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDDB50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDDB50 size=40
    let mut pc: u32 = 0x82FDDB50;
    'dispatch: loop {
        match pc {
            0x82FDDB50 => {
    //   block [0x82FDDB50..0x82FDDB78)
	// 82FDDB50: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 82FDDB54: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDDB58: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FDDB5C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDDB60: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82FDDB64: 4806EBFD  bl 0x8304c760
	ctx.lr = 0x82FDDB68;
	sub_8304C760(ctx, base);
	// 82FDDB68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FDDB6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FDDB70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FDDB74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDDB78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FDDB78 size=8
    let mut pc: u32 = 0x82FDDB78;
    'dispatch: loop {
        match pc {
            0x82FDDB78 => {
    //   block [0x82FDDB78..0x82FDDB80)
	// 82FDDB78: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FDDB7C: 8213A31C  lwz r16, -0x5ce4(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-23780 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDDB80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDDB80 size=140
    let mut pc: u32 = 0x82FDDB80;
    'dispatch: loop {
        match pc {
            0x82FDDB80 => {
    //   block [0x82FDDB80..0x82FDDC0C)
	// 82FDDB80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDDB84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FDDB88: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 82FDDB8C: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 82FDDB90: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FDDB94: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FDDB98: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 82FDDB9C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDDBA0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FDDBA4: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 82FDDBA8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FDDBAC: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 82FDDBB0: 90DE0028  stw r6, 0x28(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), ctx.r[6].u32 ) };
	// 82FDDBB4: 394B9DB4  addi r10, r11, -0x624c
	ctx.r[10].s64 = ctx.r[11].s64 + -25164;
	// 82FDDBB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FDDBBC: 913E0010  stw r9, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 82FDDBC0: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82FDDBC4: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82FDDBC8: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82FDDBCC: 917E000C  stw r11, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82FDDBD0: 917E0014  stw r11, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82FDDBD4: 917E0018  stw r11, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82FDDBD8: 917E001C  stw r11, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82FDDBDC: 917E0020  stw r11, 0x20(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82FDDBE0: 917E0024  stw r11, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82FDDBE4: 4BFFF24D  bl 0x82fdce30
	ctx.lr = 0x82FDDBE8;
	sub_82FDCE30(ctx, base);
	// 82FDDBE8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FDDBEC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FDDBF0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FDDBF4: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 82FDDBF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FDDBFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FDDC00: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FDDC04: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FDDC08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDDC0C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FDDC0C size=8
    let mut pc: u32 = 0x82FDDC0C;
    'dispatch: loop {
        match pc {
            0x82FDDC0C => {
    //   block [0x82FDDC0C..0x82FDDC14)
	// 82FDDC0C: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FDDC10: 8213A31C  lwz r16, -0x5ce4(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-23780 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDDC14(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDDC14 size=24
    let mut pc: u32 = 0x82FDDC14;
    'dispatch: loop {
        match pc {
            0x82FDDC14 => {
    //   block [0x82FDDC14..0x82FDDC2C)
	// 82FDDC14: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDDC18: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FDDC1C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDDC20: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FDDC24: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FDDC28: 481D3001  bl 0x831b0c28
	ctx.lr = 0x82FDDC2C;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDDC34(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDDC34 size=36
    let mut pc: u32 = 0x82FDDC34;
    'dispatch: loop {
        match pc {
            0x82FDDC34 => {
    //   block [0x82FDDC34..0x82FDDC58)
	// 82FDDC34: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 82FDDC38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDDC3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FDDC40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDDC44: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82FDDC48: 4BFFC2C9  bl 0x82fd9f10
	ctx.lr = 0x82FDDC4C;
	sub_82FD9F10(ctx, base);
	// 82FDDC4C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FDDC50: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FDDC54: 481D2FD5  bl 0x831b0c28
	ctx.lr = 0x82FDDC58;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDDC58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDDC58 size=40
    let mut pc: u32 = 0x82FDDC58;
    'dispatch: loop {
        match pc {
            0x82FDDC58 => {
    //   block [0x82FDDC58..0x82FDDC80)
	// 82FDDC58: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 82FDDC5C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDDC60: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FDDC64: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDDC68: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82FDDC6C: 4806EAF5  bl 0x8304c760
	ctx.lr = 0x82FDDC70;
	sub_8304C760(ctx, base);
	// 82FDDC70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FDDC74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FDDC78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FDDC7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDDC80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDDC80 size=124
    let mut pc: u32 = 0x82FDDC80;
    'dispatch: loop {
        match pc {
            0x82FDDC80 => {
    //   block [0x82FDDC80..0x82FDDCFC)
	// 82FDDC80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDDC84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FDDC88: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FDDC8C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDDC90: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FDDC94: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FDDC98: 396BA360  addi r11, r11, -0x5ca0
	ctx.r[11].s64 = ctx.r[11].s64 + -23712;
	// 82FDDC9C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDDCA0: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FDDCA4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FDDCA8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDDCAC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FDDCB0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDDCB4: 4E800421  bctrl
	ctx.lr = 0x82FDDCB8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDDCB8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDDCBC: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FDDCC0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDDCC4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FDDCC8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDDCCC: 4E800421  bctrl
	ctx.lr = 0x82FDDCD0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDDCD0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDDCD4: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FDDCD8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDDCDC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FDDCE0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDDCE4: 4E800421  bctrl
	ctx.lr = 0x82FDDCE8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDDCE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FDDCEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FDDCF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FDDCF4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FDDCF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDDD00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDDD00 size=76
    let mut pc: u32 = 0x82FDDD00;
    'dispatch: loop {
        match pc {
            0x82FDDD00 => {
    //   block [0x82FDDD00..0x82FDDD4C)
	// 82FDDD00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDDD04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FDDD08: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FDDD0C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FDDD10: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDDD14: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FDDD18: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FDDD1C: 4BFFFF65  bl 0x82fddc80
	ctx.lr = 0x82FDDD20;
	sub_82FDDC80(ctx, base);
	// 82FDDD20: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDDD24: 4182000C  beq 0x82fddd30
	if ctx.cr[0].eq {
	pc = 0x82FDDD30; continue 'dispatch;
	}
	// 82FDDD28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FDDD2C: 4BFFA5B5  bl 0x82fd82e0
	ctx.lr = 0x82FDDD30;
	sub_82FD82E0(ctx, base);
	// 82FDDD30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FDDD34: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FDDD38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FDDD3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FDDD40: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FDDD44: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FDDD48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDDD50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDDD50 size=92
    let mut pc: u32 = 0x82FDDD50;
    'dispatch: loop {
        match pc {
            0x82FDDD50 => {
    //   block [0x82FDDD50..0x82FDDDAC)
	// 82FDDD50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDDD54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FDDD58: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FDDD5C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FDDD60: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDDD64: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FDDD68: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FDDD6C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDDD70: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FDDD74: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDDD78: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FDDD7C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDDD80: 4E800421  bctrl
	ctx.lr = 0x82FDDD84;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDDD84: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FDDD88: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDDD8C: 4BFF2DF5  bl 0x82fd0b80
	ctx.lr = 0x82FDDD90;
	sub_82FD0B80(ctx, base);
	// 82FDDD90: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 82FDDD94: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FDDD98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FDDD9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FDDDA0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FDDDA4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FDDDA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDDDB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDDDB0 size=92
    let mut pc: u32 = 0x82FDDDB0;
    'dispatch: loop {
        match pc {
            0x82FDDDB0 => {
    //   block [0x82FDDDB0..0x82FDDE0C)
	// 82FDDDB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDDDB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FDDDB8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FDDDBC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FDDDC0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDDDC4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FDDDC8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FDDDCC: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDDDD0: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FDDDD4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDDDD8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FDDDDC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDDDE0: 4E800421  bctrl
	ctx.lr = 0x82FDDDE4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDDDE4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FDDDE8: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDDDEC: 4BFF2D95  bl 0x82fd0b80
	ctx.lr = 0x82FDDDF0;
	sub_82FD0B80(ctx, base);
	// 82FDDDF0: 907F000C  stw r3, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 82FDDDF4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FDDDF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FDDDFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FDDE00: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FDDE04: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FDDE08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDDE10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDDE10 size=92
    let mut pc: u32 = 0x82FDDE10;
    'dispatch: loop {
        match pc {
            0x82FDDE10 => {
    //   block [0x82FDDE10..0x82FDDE6C)
	// 82FDDE10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDDE14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FDDE18: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FDDE1C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FDDE20: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDDE24: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FDDE28: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FDDE2C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDDE30: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FDDE34: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDDE38: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FDDE3C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDDE40: 4E800421  bctrl
	ctx.lr = 0x82FDDE44;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDDE44: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FDDE48: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDDE4C: 4BFF2D35  bl 0x82fd0b80
	ctx.lr = 0x82FDDE50;
	sub_82FD0B80(ctx, base);
	// 82FDDE50: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 82FDDE54: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FDDE58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FDDE5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FDDE60: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FDDE64: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FDDE68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDDE70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FDDE70 size=44
    let mut pc: u32 = 0x82FDDE70;
    'dispatch: loop {
        match pc {
            0x82FDDE70 => {
    //   block [0x82FDDE70..0x82FDDE9C)
	// 82FDDE70: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FDDE74: 90830004  stw r4, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82FDDE78: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82FDDE7C: 394BA360  addi r10, r11, -0x5ca0
	ctx.r[10].s64 = ctx.r[11].s64 + -23712;
	// 82FDDE80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FDDE84: 99230014  stb r9, 0x14(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[9].u8 ) };
	// 82FDDE88: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82FDDE8C: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82FDDE90: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82FDDE94: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82FDDE98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDDEA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDDEA0 size=104
    let mut pc: u32 = 0x82FDDEA0;
    'dispatch: loop {
        match pc {
            0x82FDDEA0 => {
    //   block [0x82FDDEA0..0x82FDDF08)
	// 82FDDEA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDDEA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FDDEA8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FDDEAC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDDEB0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FDDEB4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FDDEB8: 394BA360  addi r10, r11, -0x5ca0
	ctx.r[10].s64 = ctx.r[11].s64 + -23712;
	// 82FDDEBC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FDDEC0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82FDDEC4: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82FDDEC8: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82FDDECC: 90BF0004  stw r5, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82FDDED0: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82FDDED4: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82FDDED8: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82FDDEDC: 993F0014  stb r9, 0x14(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u8 ) };
	// 82FDDEE0: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82FDDEE4: 4BFF2C9D  bl 0x82fd0b80
	ctx.lr = 0x82FDDEE8;
	sub_82FD0B80(ctx, base);
	// 82FDDEE8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82FDDEEC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FDDEF0: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82FDDEF4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FDDEF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FDDEFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FDDF00: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FDDF04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDDF08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDDF08 size=104
    let mut pc: u32 = 0x82FDDF08;
    'dispatch: loop {
        match pc {
            0x82FDDF08 => {
    //   block [0x82FDDF08..0x82FDDF70)
	// 82FDDF08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDDF0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FDDF10: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FDDF14: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDDF18: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FDDF1C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FDDF20: 394BA360  addi r10, r11, -0x5ca0
	ctx.r[10].s64 = ctx.r[11].s64 + -23712;
	// 82FDDF24: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FDDF28: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82FDDF2C: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82FDDF30: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82FDDF34: 90BF0004  stw r5, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82FDDF38: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82FDDF3C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82FDDF40: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82FDDF44: 993F0014  stb r9, 0x14(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u8 ) };
	// 82FDDF48: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82FDDF4C: 4BFF34C5  bl 0x82fd1410
	ctx.lr = 0x82FDDF50;
	sub_82FD1410(ctx, base);
	// 82FDDF50: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82FDDF54: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FDDF58: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82FDDF5C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FDDF60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FDDF64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FDDF68: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FDDF6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDDF70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FDDF70 size=8
    let mut pc: u32 = 0x82FDDF70;
    'dispatch: loop {
        match pc {
            0x82FDDF70 => {
    //   block [0x82FDDF70..0x82FDDF78)
	// 82FDDF70: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FDDF74: 8213A3A0  lwz r16, -0x5c60(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-23648 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDDF78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDDF78 size=152
    let mut pc: u32 = 0x82FDDF78;
    'dispatch: loop {
        match pc {
            0x82FDDF78 => {
    //   block [0x82FDDF78..0x82FDE010)
	// 82FDDF78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDDF7C: 481CA1E5  bl 0x831a8160
	ctx.lr = 0x82FDDF80;
	sub_831A8130(ctx, base);
	// 82FDDF80: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 82FDDF84: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDDF88: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FDDF8C: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82FDDF90: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82FDDF94: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82FDDF98: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 82FDDF9C: 93DF00A4  stw r30, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[30].u32 ) };
	// 82FDDFA0: 4801BE19  bl 0x82ff9db8
	ctx.lr = 0x82FDDFA4;
	sub_82FF9DB8(ctx, base);
	// 82FDDFA4: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FDDFA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FDDFAC: 937E0008  stw r27, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[27].u32 ) };
	// 82FDDFB0: 396BA388  addi r11, r11, -0x5c78
	ctx.r[11].s64 = ctx.r[11].s64 + -23672;
	// 82FDDFB4: 93BE000C  stw r29, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 82FDDFB8: 2F1B0001  cmpwi cr6, r27, 1
	ctx.cr[6].compare_i32(ctx.r[27].s32, 1, &mut ctx.xer);
	// 82FDDFBC: 939E0014  stw r28, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[28].u32 ) };
	// 82FDDFC0: 915E0010  stw r10, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82FDDFC4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FDDFC8: 915E0004  stw r10, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82FDDFCC: 409A0034  bne cr6, 0x82fde000
	if !ctx.cr[6].eq {
	pc = 0x82FDE000; continue 'dispatch;
	}
	// 82FDDFD0: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDDFD4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FDDFD8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FDDFDC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDDFE0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDDFE4: 4E800421  bctrl
	ctx.lr = 0x82FDDFE8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDDFE8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82FDDFEC: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82FDDFF0: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82FDDFF4: 481CA51D  bl 0x831a8510
	ctx.lr = 0x82FDDFF8;
	sub_831A8510(ctx, base);
	// 82FDDFF8: 939E0004  stw r28, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82FDDFFC: 48000008  b 0x82fde004
	pc = 0x82FDE004; continue 'dispatch;
	// 82FDE000: 935E0004  stw r26, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[26].u32 ) };
	// 82FDE004: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FDE008: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 82FDE00C: 481CA1A4  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDE010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDE010 size=40
    let mut pc: u32 = 0x82FDE010;
    'dispatch: loop {
        match pc {
            0x82FDE010 => {
    //   block [0x82FDE010..0x82FDE038)
	// 82FDE010: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FDE014: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDE018: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FDE01C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDE020: 807F00A4  lwz r3, 0xa4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 82FDE024: 4801BD95  bl 0x82ff9db8
	ctx.lr = 0x82FDE028;
	sub_82FF9DB8(ctx, base);
	// 82FDE028: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FDE02C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FDE030: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FDE034: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDE038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FDE038 size=8
    let mut pc: u32 = 0x82FDE038;
    'dispatch: loop {
        match pc {
            0x82FDE038 => {
    //   block [0x82FDE038..0x82FDE040)
	// 82FDE038: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FDE03C: 8213A3D8  lwz r16, -0x5c28(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-23592 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDE040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDE040 size=120
    let mut pc: u32 = 0x82FDE040;
    'dispatch: loop {
        match pc {
            0x82FDE040 => {
    //   block [0x82FDE040..0x82FDE0B8)
	// 82FDE040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDE044: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FDE048: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FDE04C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FDE050: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 82FDE054: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDE058: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FDE05C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FDE060: 396BA388  addi r11, r11, -0x5c78
	ctx.r[11].s64 = ctx.r[11].s64 + -23672;
	// 82FDE064: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 82FDE068: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FDE06C: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FDE070: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82FDE074: 4182000C  beq 0x82fde080
	if ctx.cr[0].eq {
	pc = 0x82FDE080; continue 'dispatch;
	}
	// 82FDE078: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82FDE07C: 409A001C  bne cr6, 0x82fde098
	if !ctx.cr[6].eq {
	pc = 0x82FDE098; continue 'dispatch;
	}
	// 82FDE080: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FDE084: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDE088: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDE08C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FDE090: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDE094: 4E800421  bctrl
	ctx.lr = 0x82FDE098;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDE098: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FDE09C: 4801BD1D  bl 0x82ff9db8
	ctx.lr = 0x82FDE0A0;
	sub_82FF9DB8(ctx, base);
	// 82FDE0A0: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 82FDE0A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FDE0A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FDE0AC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FDE0B0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FDE0B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDE0B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDE0B8 size=40
    let mut pc: u32 = 0x82FDE0B8;
    'dispatch: loop {
        match pc {
            0x82FDE0B8 => {
    //   block [0x82FDE0B8..0x82FDE0E0)
	// 82FDE0B8: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 82FDE0BC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDE0C0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FDE0C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDE0C8: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82FDE0CC: 4801BCED  bl 0x82ff9db8
	ctx.lr = 0x82FDE0D0;
	sub_82FF9DB8(ctx, base);
	// 82FDE0D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FDE0D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FDE0D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FDE0DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDE0E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDE0E0 size=124
    let mut pc: u32 = 0x82FDE0E0;
    'dispatch: loop {
        match pc {
            0x82FDE0E0 => {
    //   block [0x82FDE0E0..0x82FDE15C)
	// 82FDE0E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDE0E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FDE0E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FDE0EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FDE0F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDE0F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FDE0F8: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FDE0FC: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FDE100: 7D6A5851  subf. r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDE104: 4082000C  bne 0x82fde110
	if !ctx.cr[0].eq {
	pc = 0x82FDE110; continue 'dispatch;
	}
	// 82FDE108: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FDE10C: 48000038  b 0x82fde144
	pc = 0x82FDE144; continue 'dispatch;
	// 82FDE110: 7F0B2840  cmplw cr6, r11, r5
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[5].u32, &mut ctx.xer);
	// 82FDE114: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 82FDE118: 41980008  blt cr6, 0x82fde120
	if ctx.cr[6].lt {
	pc = 0x82FDE120; continue 'dispatch;
	}
	// 82FDE11C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82FDE120: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDE124: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82FDE128: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82FDE12C: 7C8B5214  add r4, r11, r10
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82FDE130: 481CA3E1  bl 0x831a8510
	ctx.lr = 0x82FDE134;
	sub_831A8510(ctx, base);
	// 82FDE134: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FDE138: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FDE13C: 7D7E5A14  add r11, r30, r11
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 82FDE140: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82FDE144: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FDE148: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FDE14C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FDE150: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FDE154: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FDE158: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDE160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDE160 size=76
    let mut pc: u32 = 0x82FDE160;
    'dispatch: loop {
        match pc {
            0x82FDE160 => {
    //   block [0x82FDE160..0x82FDE1AC)
	// 82FDE160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDE164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FDE168: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FDE16C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FDE170: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDE174: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FDE178: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FDE17C: 4BFFFEC5  bl 0x82fde040
	ctx.lr = 0x82FDE180;
	sub_82FDE040(ctx, base);
	// 82FDE180: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDE184: 4182000C  beq 0x82fde190
	if ctx.cr[0].eq {
	pc = 0x82FDE190; continue 'dispatch;
	}
	// 82FDE188: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FDE18C: 4BFFA155  bl 0x82fd82e0
	ctx.lr = 0x82FDE190;
	sub_82FD82E0(ctx, base);
	// 82FDE190: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FDE194: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FDE198: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FDE19C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FDE1A0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FDE1A4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FDE1A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDE1B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FDE1B0 size=52
    let mut pc: u32 = 0x82FDE1B0;
    'dispatch: loop {
        match pc {
            0x82FDE1B0 => {
    //   block [0x82FDE1B0..0x82FDE1E4)
	// 82FDE1B0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FDE1B4: 90830004  stw r4, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82FDE1B8: 394BA430  addi r10, r11, -0x5bd0
	ctx.r[10].s64 = ctx.r[11].s64 + -23504;
	// 82FDE1BC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FDE1C0: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82FDE1C4: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82FDE1C8: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82FDE1CC: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82FDE1D0: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82FDE1D4: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82FDE1D8: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82FDE1DC: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82FDE1E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDE1E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FDE1E8 size=8
    let mut pc: u32 = 0x82FDE1E8;
    'dispatch: loop {
        match pc {
            0x82FDE1E8 => {
    //   block [0x82FDE1E8..0x82FDE1F0)
	// 82FDE1E8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FDE1EC: 8213A448  lwz r16, -0x5bb8(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-23480 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDE1F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDE1F0 size=332
    let mut pc: u32 = 0x82FDE1F0;
    'dispatch: loop {
        match pc {
            0x82FDE1F0 => {
    //   block [0x82FDE1F0..0x82FDE33C)
	// 82FDE1F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDE1F4: 481C9F71  bl 0x831a8164
	ctx.lr = 0x82FDE1F8;
	sub_831A8130(ctx, base);
	// 82FDE1F8: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 82FDE1FC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDE200: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FDE204: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82FDE208: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 82FDE20C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FDE210: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82FDE214: 396BA430  addi r11, r11, -0x5bd0
	ctx.r[11].s64 = ctx.r[11].s64 + -23504;
	// 82FDE218: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FDE21C: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDE220: 939E0018  stw r28, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[28].u32 ) };
	// 82FDE224: 939E001C  stw r28, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[28].u32 ) };
	// 82FDE228: 939E0008  stw r28, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 82FDE22C: 939E000C  stw r28, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[28].u32 ) };
	// 82FDE230: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82FDE234: 939E0010  stw r28, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[28].u32 ) };
	// 82FDE238: 939E0014  stw r28, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[28].u32 ) };
	// 82FDE23C: 939E0020  stw r28, 0x20(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[28].u32 ) };
	// 82FDE240: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FDE244: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDE248: 41820034  beq 0x82fde27c
	if ctx.cr[0].eq {
	pc = 0x82FDE27C; continue 'dispatch;
	}
	// 82FDE24C: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDE250: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDE254: 41820028  beq 0x82fde27c
	if ctx.cr[0].eq {
	pc = 0x82FDE27C; continue 'dispatch;
	}
	// 82FDE258: 394B0002  addi r10, r11, 2
	ctx.r[10].s64 = ctx.r[11].s64 + 2;
	// 82FDE25C: 48000008  b 0x82fde264
	pc = 0x82FDE264; continue 'dispatch;
	// 82FDE260: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 82FDE264: A12A0000  lhz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDE268: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDE26C: 4082FFF4  bne 0x82fde260
	if !ctx.cr[0].eq {
	pc = 0x82FDE260; continue 'dispatch;
	}
	// 82FDE270: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82FDE274: 7D7B0E70  srawi r27, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[27].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82FDE278: 48000008  b 0x82fde280
	pc = 0x82FDE280; continue 'dispatch;
	// 82FDE27C: 7F9BE378  mr r27, r28
	ctx.r[27].u64 = ctx.r[28].u64;
	// 82FDE280: 397B0008  addi r11, r27, 8
	ctx.r[11].s64 = ctx.r[27].s64 + 8;
	// 82FDE284: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDE288: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 82FDE28C: 5544083C  slwi r4, r10, 1
	ctx.r[4].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82FDE290: 917E0014  stw r11, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82FDE294: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDE298: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDE29C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDE2A0: 4E800421  bctrl
	ctx.lr = 0x82FDE2A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDE2A4: 397B0001  addi r11, r27, 1
	ctx.r[11].s64 = ctx.r[27].s64 + 1;
	// 82FDE2A8: 907E0010  stw r3, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 82FDE2AC: 5565083C  slwi r5, r11, 1
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82FDE2B0: 809D0010  lwz r4, 0x10(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FDE2B4: 481CA25D  bl 0x831a8510
	ctx.lr = 0x82FDE2B8;
	sub_831A8510(ctx, base);
	// 82FDE2B8: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FDE2BC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDE2C0: 41820030  beq 0x82fde2f0
	if ctx.cr[0].eq {
	pc = 0x82FDE2F0; continue 'dispatch;
	}
	// 82FDE2C4: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDE2C8: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDE2CC: 41820024  beq 0x82fde2f0
	if ctx.cr[0].eq {
	pc = 0x82FDE2F0; continue 'dispatch;
	}
	// 82FDE2D0: 394B0002  addi r10, r11, 2
	ctx.r[10].s64 = ctx.r[11].s64 + 2;
	// 82FDE2D4: 48000008  b 0x82fde2dc
	pc = 0x82FDE2DC; continue 'dispatch;
	// 82FDE2D8: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 82FDE2DC: A12A0000  lhz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDE2E0: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDE2E4: 4082FFF4  bne 0x82fde2d8
	if !ctx.cr[0].eq {
	pc = 0x82FDE2D8; continue 'dispatch;
	}
	// 82FDE2E8: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82FDE2EC: 7D7C0E70  srawi r28, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[28].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82FDE2F0: 397C0008  addi r11, r28, 8
	ctx.r[11].s64 = ctx.r[28].s64 + 8;
	// 82FDE2F4: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDE2F8: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 82FDE2FC: 5544083C  slwi r4, r10, 1
	ctx.r[4].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82FDE300: 917E000C  stw r11, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82FDE304: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDE308: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDE30C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDE310: 4E800421  bctrl
	ctx.lr = 0x82FDE314;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDE314: 397C0001  addi r11, r28, 1
	ctx.r[11].s64 = ctx.r[28].s64 + 1;
	// 82FDE318: 907E0008  stw r3, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 82FDE31C: 5565083C  slwi r5, r11, 1
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82FDE320: 809D0008  lwz r4, 8(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FDE324: 481CA1ED  bl 0x831a8510
	ctx.lr = 0x82FDE328;
	sub_831A8510(ctx, base);
	// 82FDE328: 817D0020  lwz r11, 0x20(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FDE32C: 917E0020  stw r11, 0x20(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82FDE330: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FDE334: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 82FDE338: 481C9E7C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDE33C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDE33C size=40
    let mut pc: u32 = 0x82FDE33C;
    'dispatch: loop {
        match pc {
            0x82FDE33C => {
    //   block [0x82FDE33C..0x82FDE364)
	// 82FDE33C: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FDE340: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDE344: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FDE348: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDE34C: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FDE350: 4806E411  bl 0x8304c760
	ctx.lr = 0x82FDE354;
	sub_8304C760(ctx, base);
	// 82FDE354: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FDE358: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FDE35C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FDE360: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDE368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDE368 size=312
    let mut pc: u32 = 0x82FDE368;
    'dispatch: loop {
        match pc {
            0x82FDE368 => {
    //   block [0x82FDE368..0x82FDE4A0)
	// 82FDE368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDE36C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FDE370: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FDE374: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FDE378: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDE37C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FDE380: 809F0018  lwz r4, 0x18(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FDE384: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDE388: 41820010  beq 0x82fde398
	if ctx.cr[0].eq {
	pc = 0x82FDE398; continue 'dispatch;
	}
	// 82FDE38C: A1640000  lhz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDE390: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDE394: 408200E8  bne 0x82fde47c
	if !ctx.cr[0].eq {
	pc = 0x82FDE47C; continue 'dispatch;
	}
	// 82FDE398: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FDE39C: A16B0000  lhz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDE3A0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDE3A4: 418200E0  beq 0x82fde484
	if ctx.cr[0].eq {
	pc = 0x82FDE484; continue 'dispatch;
	}
	// 82FDE3A8: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FDE3AC: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82FDE3B0: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FDE3B4: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82FDE3B8: 3BCB0001  addi r30, r11, 1
	ctx.r[30].s64 = ctx.r[11].s64 + 1;
	// 82FDE3BC: 419A0010  beq cr6, 0x82fde3cc
	if ctx.cr[6].eq {
	pc = 0x82FDE3CC; continue 'dispatch;
	}
	// 82FDE3C0: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FDE3C4: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FDE3C8: 40990044  ble cr6, 0x82fde40c
	if !ctx.cr[6].gt {
	pc = 0x82FDE40C; continue 'dispatch;
	}
	// 82FDE3CC: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDE3D0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDE3D4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FDE3D8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDE3DC: 4E800421  bctrl
	ctx.lr = 0x82FDE3E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDE3E0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDE3E4: 397E0001  addi r11, r30, 1
	ctx.r[11].s64 = ctx.r[30].s64 + 1;
	// 82FDE3E8: 93DF001C  stw r30, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u32 ) };
	// 82FDE3EC: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82FDE3F0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDE3F4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDE3F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDE3FC: 4E800421  bctrl
	ctx.lr = 0x82FDE400;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDE400: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FDE404: 907F0018  stw r3, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[3].u32 ) };
	// 82FDE408: B1630000  sth r11, 0(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 82FDE40C: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FDE410: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDE414: 41820034  beq 0x82fde448
	if ctx.cr[0].eq {
	pc = 0x82FDE448; continue 'dispatch;
	}
	// 82FDE418: A1640000  lhz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDE41C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDE420: 41820028  beq 0x82fde448
	if ctx.cr[0].eq {
	pc = 0x82FDE448; continue 'dispatch;
	}
	// 82FDE424: 39640002  addi r11, r4, 2
	ctx.r[11].s64 = ctx.r[4].s64 + 2;
	// 82FDE428: 48000008  b 0x82fde430
	pc = 0x82FDE430; continue 'dispatch;
	// 82FDE42C: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FDE430: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDE434: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDE438: 4082FFF4  bne 0x82fde42c
	if !ctx.cr[0].eq {
	pc = 0x82FDE42C; continue 'dispatch;
	}
	// 82FDE43C: 7D645850  subf r11, r4, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[4].s64;
	// 82FDE440: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82FDE444: 48000008  b 0x82fde44c
	pc = 0x82FDE44C; continue 'dispatch;
	// 82FDE448: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FDE44C: 557E083C  slwi r30, r11, 1
	ctx.r[30].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82FDE450: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FDE454: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82FDE458: 481CA0B9  bl 0x831a8510
	ctx.lr = 0x82FDE45C;
	sub_831A8510(ctx, base);
	// 82FDE45C: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FDE460: 3940003A  li r10, 0x3a
	ctx.r[10].s64 = 58;
	// 82FDE464: 7D5E5B2E  sthx r10, r30, r11
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u16) };
	// 82FDE468: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FDE46C: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FDE470: 7D7E5A14  add r11, r30, r11
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 82FDE474: 386B0002  addi r3, r11, 2
	ctx.r[3].s64 = ctx.r[11].s64 + 2;
	// 82FDE478: 4BFF36F1  bl 0x82fd1b68
	ctx.lr = 0x82FDE47C;
	sub_82FD1B68(ctx, base);
	// 82FDE47C: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FDE480: 48000008  b 0x82fde488
	pc = 0x82FDE488; continue 'dispatch;
	// 82FDE484: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FDE488: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FDE48C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FDE490: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FDE494: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FDE498: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FDE49C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDE4A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDE4A0 size=192
    let mut pc: u32 = 0x82FDE4A0;
    'dispatch: loop {
        match pc {
            0x82FDE4A0 => {
    //   block [0x82FDE4A0..0x82FDE560)
	// 82FDE4A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDE4A4: 481C9CC9  bl 0x831a816c
	ctx.lr = 0x82FDE4A8;
	sub_831A8130(ctx, base);
	// 82FDE4A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDE4AC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82FDE4B0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FDE4B4: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82FDE4B8: 419A0034  beq cr6, 0x82fde4ec
	if ctx.cr[6].eq {
	pc = 0x82FDE4EC; continue 'dispatch;
	}
	// 82FDE4BC: A17D0000  lhz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDE4C0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDE4C4: 41820028  beq 0x82fde4ec
	if ctx.cr[0].eq {
	pc = 0x82FDE4EC; continue 'dispatch;
	}
	// 82FDE4C8: 397D0002  addi r11, r29, 2
	ctx.r[11].s64 = ctx.r[29].s64 + 2;
	// 82FDE4CC: 48000008  b 0x82fde4d4
	pc = 0x82FDE4D4; continue 'dispatch;
	// 82FDE4D0: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FDE4D4: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDE4D8: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDE4DC: 4082FFF4  bne 0x82fde4d0
	if !ctx.cr[0].eq {
	pc = 0x82FDE4D0; continue 'dispatch;
	}
	// 82FDE4E0: 7D7D5850  subf r11, r29, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	// 82FDE4E4: 7D7E0E70  srawi r30, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[30].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82FDE4E8: 48000008  b 0x82fde4f0
	pc = 0x82FDE4F0; continue 'dispatch;
	// 82FDE4EC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82FDE4F0: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FDE4F4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDE4F8: 4182000C  beq 0x82fde504
	if ctx.cr[0].eq {
	pc = 0x82FDE504; continue 'dispatch;
	}
	// 82FDE4FC: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FDE500: 40990044  ble cr6, 0x82fde544
	if !ctx.cr[6].gt {
	pc = 0x82FDE544; continue 'dispatch;
	}
	// 82FDE504: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDE508: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FDE50C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDE510: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FDE514: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDE518: 4E800421  bctrl
	ctx.lr = 0x82FDE51C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDE51C: 397E0008  addi r11, r30, 8
	ctx.r[11].s64 = ctx.r[30].s64 + 8;
	// 82FDE520: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDE524: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 82FDE528: 5544083C  slwi r4, r10, 1
	ctx.r[4].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82FDE52C: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82FDE530: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDE534: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDE538: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDE53C: 4E800421  bctrl
	ctx.lr = 0x82FDE540;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDE540: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 82FDE544: 397E0001  addi r11, r30, 1
	ctx.r[11].s64 = ctx.r[30].s64 + 1;
	// 82FDE548: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FDE54C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FDE550: 5565083C  slwi r5, r11, 1
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82FDE554: 481C9FBD  bl 0x831a8510
	ctx.lr = 0x82FDE558;
	sub_831A8510(ctx, base);
	// 82FDE558: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FDE55C: 481C9C60  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDE560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDE560 size=148
    let mut pc: u32 = 0x82FDE560;
    'dispatch: loop {
        match pc {
            0x82FDE560 => {
    //   block [0x82FDE560..0x82FDE5F4)
	// 82FDE560: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDE564: 481C9C09  bl 0x831a816c
	ctx.lr = 0x82FDE568;
	sub_831A8130(ctx, base);
	// 82FDE568: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDE56C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FDE570: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82FDE574: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82FDE578: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FDE57C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDE580: 4182000C  beq 0x82fde58c
	if ctx.cr[0].eq {
	pc = 0x82FDE58C; continue 'dispatch;
	}
	// 82FDE584: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FDE588: 40990044  ble cr6, 0x82fde5cc
	if !ctx.cr[6].gt {
	pc = 0x82FDE5CC; continue 'dispatch;
	}
	// 82FDE58C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDE590: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FDE594: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDE598: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FDE59C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDE5A0: 4E800421  bctrl
	ctx.lr = 0x82FDE5A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDE5A4: 397E0008  addi r11, r30, 8
	ctx.r[11].s64 = ctx.r[30].s64 + 8;
	// 82FDE5A8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDE5AC: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 82FDE5B0: 5544083C  slwi r4, r10, 1
	ctx.r[4].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82FDE5B4: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82FDE5B8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDE5BC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDE5C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDE5C4: 4E800421  bctrl
	ctx.lr = 0x82FDE5C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDE5C8: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 82FDE5CC: 57DE083C  slwi r30, r30, 1
	ctx.r[30].u32 = ctx.r[30].u32.wrapping_shl(1);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82FDE5D0: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FDE5D4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FDE5D8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82FDE5DC: 481C9F35  bl 0x831a8510
	ctx.lr = 0x82FDE5E0;
	sub_831A8510(ctx, base);
	// 82FDE5E0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FDE5E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FDE5E8: 7D4BF32E  sthx r10, r11, r30
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32), ctx.r[10].u16) };
	// 82FDE5EC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FDE5F0: 481C9BCC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDE5F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDE5F8 size=192
    let mut pc: u32 = 0x82FDE5F8;
    'dispatch: loop {
        match pc {
            0x82FDE5F8 => {
    //   block [0x82FDE5F8..0x82FDE6B8)
	// 82FDE5F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDE5FC: 481C9B71  bl 0x831a816c
	ctx.lr = 0x82FDE600;
	sub_831A8130(ctx, base);
	// 82FDE600: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDE604: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82FDE608: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FDE60C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82FDE610: 419A0034  beq cr6, 0x82fde644
	if ctx.cr[6].eq {
	pc = 0x82FDE644; continue 'dispatch;
	}
	// 82FDE614: A17D0000  lhz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDE618: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDE61C: 41820028  beq 0x82fde644
	if ctx.cr[0].eq {
	pc = 0x82FDE644; continue 'dispatch;
	}
	// 82FDE620: 397D0002  addi r11, r29, 2
	ctx.r[11].s64 = ctx.r[29].s64 + 2;
	// 82FDE624: 48000008  b 0x82fde62c
	pc = 0x82FDE62C; continue 'dispatch;
	// 82FDE628: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FDE62C: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDE630: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDE634: 4082FFF4  bne 0x82fde628
	if !ctx.cr[0].eq {
	pc = 0x82FDE628; continue 'dispatch;
	}
	// 82FDE638: 7D7D5850  subf r11, r29, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	// 82FDE63C: 7D7E0E70  srawi r30, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[30].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82FDE640: 48000008  b 0x82fde648
	pc = 0x82FDE648; continue 'dispatch;
	// 82FDE644: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82FDE648: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FDE64C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDE650: 4182000C  beq 0x82fde65c
	if ctx.cr[0].eq {
	pc = 0x82FDE65C; continue 'dispatch;
	}
	// 82FDE654: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FDE658: 40990044  ble cr6, 0x82fde69c
	if !ctx.cr[6].gt {
	pc = 0x82FDE69C; continue 'dispatch;
	}
	// 82FDE65C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDE660: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FDE664: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDE668: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FDE66C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDE670: 4E800421  bctrl
	ctx.lr = 0x82FDE674;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDE674: 397E0008  addi r11, r30, 8
	ctx.r[11].s64 = ctx.r[30].s64 + 8;
	// 82FDE678: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDE67C: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 82FDE680: 5544083C  slwi r4, r10, 1
	ctx.r[4].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82FDE684: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82FDE688: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDE68C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDE690: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDE694: 4E800421  bctrl
	ctx.lr = 0x82FDE698;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDE698: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 82FDE69C: 397E0001  addi r11, r30, 1
	ctx.r[11].s64 = ctx.r[30].s64 + 1;
	// 82FDE6A0: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FDE6A4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FDE6A8: 5565083C  slwi r5, r11, 1
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82FDE6AC: 481C9E65  bl 0x831a8510
	ctx.lr = 0x82FDE6B0;
	sub_831A8510(ctx, base);
	// 82FDE6B0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FDE6B4: 481C9B08  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDE6B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDE6B8 size=148
    let mut pc: u32 = 0x82FDE6B8;
    'dispatch: loop {
        match pc {
            0x82FDE6B8 => {
    //   block [0x82FDE6B8..0x82FDE74C)
	// 82FDE6B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDE6BC: 481C9AB1  bl 0x831a816c
	ctx.lr = 0x82FDE6C0;
	sub_831A8130(ctx, base);
	// 82FDE6C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDE6C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FDE6C8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82FDE6CC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82FDE6D0: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FDE6D4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDE6D8: 4182000C  beq 0x82fde6e4
	if ctx.cr[0].eq {
	pc = 0x82FDE6E4; continue 'dispatch;
	}
	// 82FDE6DC: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FDE6E0: 40990044  ble cr6, 0x82fde724
	if !ctx.cr[6].gt {
	pc = 0x82FDE724; continue 'dispatch;
	}
	// 82FDE6E4: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDE6E8: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FDE6EC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDE6F0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FDE6F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDE6F8: 4E800421  bctrl
	ctx.lr = 0x82FDE6FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDE6FC: 397E0008  addi r11, r30, 8
	ctx.r[11].s64 = ctx.r[30].s64 + 8;
	// 82FDE700: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDE704: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 82FDE708: 5544083C  slwi r4, r10, 1
	ctx.r[4].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82FDE70C: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82FDE710: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDE714: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDE718: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDE71C: 4E800421  bctrl
	ctx.lr = 0x82FDE720;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDE720: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 82FDE724: 57DE083C  slwi r30, r30, 1
	ctx.r[30].u32 = ctx.r[30].u32.wrapping_shl(1);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82FDE728: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FDE72C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FDE730: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82FDE734: 481C9DDD  bl 0x831a8510
	ctx.lr = 0x82FDE738;
	sub_831A8510(ctx, base);
	// 82FDE738: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FDE73C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FDE740: 7D4BF32E  sthx r10, r11, r30
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32), ctx.r[10].u16) };
	// 82FDE744: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FDE748: 481C9A74  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDE750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDE750 size=80
    let mut pc: u32 = 0x82FDE750;
    'dispatch: loop {
        match pc {
            0x82FDE750 => {
    //   block [0x82FDE750..0x82FDE7A0)
	// 82FDE750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDE754: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FDE758: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FDE75C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FDE760: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDE764: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FDE768: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FDE76C: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FDE770: 4BFFFD31  bl 0x82fde4a0
	ctx.lr = 0x82FDE774;
	sub_82FDE4A0(ctx, base);
	// 82FDE774: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FDE778: 809E0010  lwz r4, 0x10(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FDE77C: 4BFFFE7D  bl 0x82fde5f8
	ctx.lr = 0x82FDE780;
	sub_82FDE5F8(ctx, base);
	// 82FDE780: 817E0020  lwz r11, 0x20(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FDE784: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82FDE788: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FDE78C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FDE790: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FDE794: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FDE798: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FDE79C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDE7A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDE7A0 size=136
    let mut pc: u32 = 0x82FDE7A0;
    'dispatch: loop {
        match pc {
            0x82FDE7A0 => {
    //   block [0x82FDE7A0..0x82FDE828)
	// 82FDE7A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDE7A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FDE7A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FDE7AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FDE7B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDE7B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FDE7B8: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FDE7BC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDE7C0: 40820024  bne 0x82fde7e4
	if !ctx.cr[0].eq {
	pc = 0x82FDE7E4; continue 'dispatch;
	}
	// 82FDE7C4: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82FDE7C8: 4BFFFBA1  bl 0x82fde368
	ctx.lr = 0x82FDE7CC;
	sub_82FDE368(ctx, base);
	// 82FDE7CC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FDE7D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FDE7D4: 4BFFFB95  bl 0x82fde368
	ctx.lr = 0x82FDE7D8;
	sub_82FDE368(ctx, base);
	// 82FDE7D8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FDE7DC: 4BFF5465  bl 0x82fd3c40
	ctx.lr = 0x82FDE7E0;
	sub_82FD3C40(ctx, base);
	// 82FDE7E0: 48000030  b 0x82fde810
	pc = 0x82FDE810; continue 'dispatch;
	// 82FDE7E4: 81440020  lwz r10, 0x20(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FDE7E8: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82FDE7EC: 409A001C  bne cr6, 0x82fde808
	if !ctx.cr[6].eq {
	pc = 0x82FDE808; continue 'dispatch;
	}
	// 82FDE7F0: 80840010  lwz r4, 0x10(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FDE7F4: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FDE7F8: 4BFF5449  bl 0x82fd3c40
	ctx.lr = 0x82FDE7FC;
	sub_82FD3C40(ctx, base);
	// 82FDE7FC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDE800: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82FDE804: 40820008  bne 0x82fde80c
	if !ctx.cr[0].eq {
	pc = 0x82FDE80C; continue 'dispatch;
	}
	// 82FDE808: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FDE80C: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82FDE810: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FDE814: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FDE818: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FDE81C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FDE820: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FDE824: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDE828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDE828 size=128
    let mut pc: u32 = 0x82FDE828;
    'dispatch: loop {
        match pc {
            0x82FDE828 => {
    //   block [0x82FDE828..0x82FDE8A8)
	// 82FDE828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDE82C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FDE830: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FDE834: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDE838: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FDE83C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDE840: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FDE844: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDE848: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FDE84C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDE850: 4E800421  bctrl
	ctx.lr = 0x82FDE854;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDE854: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDE858: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FDE85C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDE860: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FDE864: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDE868: 4E800421  bctrl
	ctx.lr = 0x82FDE86C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDE86C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDE870: 809F0018  lwz r4, 0x18(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FDE874: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDE878: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FDE87C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDE880: 4E800421  bctrl
	ctx.lr = 0x82FDE884;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDE884: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FDE888: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82FDE88C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82FDE890: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82FDE894: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FDE898: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FDE89C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FDE8A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FDE8A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDE8A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDE8A8 size=116
    let mut pc: u32 = 0x82FDE8A8;
    'dispatch: loop {
        match pc {
            0x82FDE8A8 => {
    //   block [0x82FDE8A8..0x82FDE91C)
	// 82FDE8A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDE8AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FDE8B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FDE8B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDE8B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FDE8BC: 38600024  li r3, 0x24
	ctx.r[3].s64 = 36;
	// 82FDE8C0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FDE8C4: 4BFF99D5  bl 0x82fd8298
	ctx.lr = 0x82FDE8C8;
	sub_82FD8298(ctx, base);
	// 82FDE8C8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDE8CC: 41820038  beq 0x82fde904
	if ctx.cr[0].eq {
	pc = 0x82FDE904; continue 'dispatch;
	}
	// 82FDE8D0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FDE8D4: 93E30004  stw r31, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82FDE8D8: 394BA430  addi r10, r11, -0x5bd0
	ctx.r[10].s64 = ctx.r[11].s64 + -23504;
	// 82FDE8DC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FDE8E0: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82FDE8E4: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82FDE8E8: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82FDE8EC: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82FDE8F0: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82FDE8F4: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82FDE8F8: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82FDE8FC: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82FDE900: 48000008  b 0x82fde908
	pc = 0x82FDE908; continue 'dispatch;
	// 82FDE904: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FDE908: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FDE90C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FDE910: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FDE914: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FDE918: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDE920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FDE920 size=12
    let mut pc: u32 = 0x82FDE920;
    'dispatch: loop {
        match pc {
            0x82FDE920 => {
    //   block [0x82FDE920..0x82FDE92C)
	// 82FDE920: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 82FDE924: 386B2E10  addi r3, r11, 0x2e10
	ctx.r[3].s64 = ctx.r[11].s64 + 11792;
	// 82FDE928: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDE930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDE930 size=180
    let mut pc: u32 = 0x82FDE930;
    'dispatch: loop {
        match pc {
            0x82FDE930 => {
    //   block [0x82FDE930..0x82FDE9E4)
	// 82FDE930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDE934: 481C9835  bl 0x831a8168
	ctx.lr = 0x82FDE938;
	sub_831A8130(ctx, base);
	// 82FDE938: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDE93C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FDE940: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FDE944: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FDE948: A97E0000  lha r11, 0(r30)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as i16) as i64;
	// 82FDE94C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82FDE950: 556BDFFF  rlwinm. r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDE954: 4182003C  beq 0x82fde990
	if ctx.cr[0].eq {
	pc = 0x82FDE990; continue 'dispatch;
	}
	// 82FDE958: 3FA08214  lis r29, -0x7dec
	ctx.r[29].s64 = -2112618496;
	// 82FDE95C: 80BF000C  lwz r5, 0xc(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FDE960: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FDE964: 88DDF638  lbz r6, -0x9c8(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(-2504 as u32) ) } as u64;
	// 82FDE968: 4801AF99  bl 0x82ff9900
	ctx.lr = 0x82FDE96C;
	sub_82FF9900(ctx, base);
	// 82FDE96C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FDE970: 88DDF638  lbz r6, -0x9c8(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(-2504 as u32) ) } as u64;
	// 82FDE974: 80BF0014  lwz r5, 0x14(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FDE978: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FDE97C: 4801AF85  bl 0x82ff9900
	ctx.lr = 0x82FDE980;
	sub_82FF9900(ctx, base);
	// 82FDE980: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FDE984: 809F0020  lwz r4, 0x20(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FDE988: 4801A971  bl 0x82ff92f8
	ctx.lr = 0x82FDE98C;
	sub_82FF92F8(ctx, base);
	// 82FDE98C: 48000050  b 0x82fde9dc
	pc = 0x82FDE9DC; continue 'dispatch;
	// 82FDE990: 3FA08214  lis r29, -0x7dec
	ctx.r[29].s64 = -2112618496;
	// 82FDE994: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82FDE998: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82FDE99C: 38BF000C  addi r5, r31, 0xc
	ctx.r[5].s64 = ctx.r[31].s64 + 12;
	// 82FDE9A0: 389F0008  addi r4, r31, 8
	ctx.r[4].s64 = ctx.r[31].s64 + 8;
	// 82FDE9A4: 88FDF639  lbz r7, -0x9c7(r29)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(-2503 as u32) ) } as u64;
	// 82FDE9A8: 93810050  stw r28, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 82FDE9AC: 4801B17D  bl 0x82ff9b28
	ctx.lr = 0x82FDE9B0;
	sub_82FF9B28(ctx, base);
	// 82FDE9B0: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82FDE9B4: 88FDF639  lbz r7, -0x9c7(r29)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(-2503 as u32) ) } as u64;
	// 82FDE9B8: 38BF0014  addi r5, r31, 0x14
	ctx.r[5].s64 = ctx.r[31].s64 + 20;
	// 82FDE9BC: 389F0010  addi r4, r31, 0x10
	ctx.r[4].s64 = ctx.r[31].s64 + 16;
	// 82FDE9C0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FDE9C4: 4801B165  bl 0x82ff9b28
	ctx.lr = 0x82FDE9C8;
	sub_82FF9B28(ctx, base);
	// 82FDE9C8: 389F0020  addi r4, r31, 0x20
	ctx.r[4].s64 = ctx.r[31].s64 + 32;
	// 82FDE9CC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FDE9D0: 939F001C  stw r28, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[28].u32 ) };
	// 82FDE9D4: 939F0018  stw r28, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[28].u32 ) };
	// 82FDE9D8: 4801ABA1  bl 0x82ff9578
	ctx.lr = 0x82FDE9DC;
	sub_82FF9578(ctx, base);
	// 82FDE9DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FDE9E0: 481C97D8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDE9E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FDE9E8 size=8
    let mut pc: u32 = 0x82FDE9E8;
    'dispatch: loop {
        match pc {
            0x82FDE9E8 => {
    //   block [0x82FDE9E8..0x82FDE9F0)
	// 82FDE9E8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FDE9EC: 8213A490  lwz r16, -0x5b70(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-23408 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDE9F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDE9F0 size=84
    let mut pc: u32 = 0x82FDE9F0;
    'dispatch: loop {
        match pc {
            0x82FDE9F0 => {
    //   block [0x82FDE9F0..0x82FDEA44)
	// 82FDE9F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDE9F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FDE9F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FDE9FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FDEA00: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 82FDEA04: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDEA08: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FDEA0C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FDEA10: 396BA430  addi r11, r11, -0x5bd0
	ctx.r[11].s64 = ctx.r[11].s64 + -23504;
	// 82FDEA14: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 82FDEA18: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FDEA1C: 4BFFFE0D  bl 0x82fde828
	ctx.lr = 0x82FDEA20;
	sub_82FDE828(ctx, base);
	// 82FDEA20: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FDEA24: 396BA93C  addi r11, r11, -0x56c4
	ctx.r[11].s64 = ctx.r[11].s64 + -22212;
	// 82FDEA28: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FDEA2C: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 82FDEA30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FDEA34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FDEA38: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FDEA3C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FDEA40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDEA44(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDEA44 size=40
    let mut pc: u32 = 0x82FDEA44;
    'dispatch: loop {
        match pc {
            0x82FDEA44 => {
    //   block [0x82FDEA44..0x82FDEA6C)
	// 82FDEA44: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 82FDEA48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDEA4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FDEA50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDEA54: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82FDEA58: 4806DD09  bl 0x8304c760
	ctx.lr = 0x82FDEA5C;
	sub_8304C760(ctx, base);
	// 82FDEA5C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FDEA60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FDEA64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FDEA68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDEA70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDEA70 size=72
    let mut pc: u32 = 0x82FDEA70;
    'dispatch: loop {
        match pc {
            0x82FDEA70 => {
    //   block [0x82FDEA70..0x82FDEAB8)
	// 82FDEA70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDEA74: 481C96F9  bl 0x831a816c
	ctx.lr = 0x82FDEA78;
	sub_831A8130(ctx, base);
	// 82FDEA78: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDEA7C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FDEA80: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82FDEA84: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82FDEA88: 4BFFFA19  bl 0x82fde4a0
	ctx.lr = 0x82FDEA8C;
	sub_82FDE4A0(ctx, base);
	// 82FDEA8C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FDEA90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FDEA94: 4BFFFB65  bl 0x82fde5f8
	ctx.lr = 0x82FDEA98;
	sub_82FDE5F8(ctx, base);
	// 82FDEA98: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FDEA9C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDEAA0: 4182000C  beq 0x82fdeaac
	if ctx.cr[0].eq {
	pc = 0x82FDEAAC; continue 'dispatch;
	}
	// 82FDEAA4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FDEAA8: B14B0000  sth r10, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 82FDEAAC: 93BF0020  stw r29, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[29].u32 ) };
	// 82FDEAB0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FDEAB4: 481C9708  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDEAB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDEAB8 size=280
    let mut pc: u32 = 0x82FDEAB8;
    'dispatch: loop {
        match pc {
            0x82FDEAB8 => {
    //   block [0x82FDEAB8..0x82FDEBD0)
	// 82FDEAB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDEABC: 481C96A9  bl 0x831a8164
	ctx.lr = 0x82FDEAC0;
	sub_831A8130(ctx, base);
	// 82FDEAC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDEAC4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82FDEAC8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FDEACC: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82FDEAD0: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82FDEAD4: 419A0034  beq cr6, 0x82fdeb08
	if ctx.cr[6].eq {
	pc = 0x82FDEB08; continue 'dispatch;
	}
	// 82FDEAD8: A17D0000  lhz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDEADC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDEAE0: 41820028  beq 0x82fdeb08
	if ctx.cr[0].eq {
	pc = 0x82FDEB08; continue 'dispatch;
	}
	// 82FDEAE4: 397D0002  addi r11, r29, 2
	ctx.r[11].s64 = ctx.r[29].s64 + 2;
	// 82FDEAE8: 48000008  b 0x82fdeaf0
	pc = 0x82FDEAF0; continue 'dispatch;
	// 82FDEAEC: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FDEAF0: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDEAF4: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDEAF8: 4082FFF4  bne 0x82fdeaec
	if !ctx.cr[0].eq {
	pc = 0x82FDEAEC; continue 'dispatch;
	}
	// 82FDEAFC: 7D7D5850  subf r11, r29, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	// 82FDEB00: 7D7C0E70  srawi r28, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[28].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82FDEB04: 48000008  b 0x82fdeb0c
	pc = 0x82FDEB0C; continue 'dispatch;
	// 82FDEB08: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82FDEB0C: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FDEB10: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDEB14: 4182000C  beq 0x82fdeb20
	if ctx.cr[0].eq {
	pc = 0x82FDEB20; continue 'dispatch;
	}
	// 82FDEB18: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FDEB1C: 40990044  ble cr6, 0x82fdeb60
	if !ctx.cr[6].gt {
	pc = 0x82FDEB60; continue 'dispatch;
	}
	// 82FDEB20: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDEB24: 809F0018  lwz r4, 0x18(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FDEB28: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDEB2C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FDEB30: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDEB34: 4E800421  bctrl
	ctx.lr = 0x82FDEB38;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDEB38: 397C0008  addi r11, r28, 8
	ctx.r[11].s64 = ctx.r[28].s64 + 8;
	// 82FDEB3C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDEB40: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 82FDEB44: 5544083C  slwi r4, r10, 1
	ctx.r[4].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82FDEB48: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82FDEB4C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDEB50: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDEB54: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDEB58: 4E800421  bctrl
	ctx.lr = 0x82FDEB5C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDEB5C: 907F0018  stw r3, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[3].u32 ) };
	// 82FDEB60: 397C0001  addi r11, r28, 1
	ctx.r[11].s64 = ctx.r[28].s64 + 1;
	// 82FDEB64: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FDEB68: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FDEB6C: 5565083C  slwi r5, r11, 1
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82FDEB70: 481C99A1  bl 0x831a8510
	ctx.lr = 0x82FDEB74;
	sub_831A8510(ctx, base);
	// 82FDEB74: 3880003A  li r4, 0x3a
	ctx.r[4].s64 = 58;
	// 82FDEB78: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FDEB7C: 4BFF3235  bl 0x82fd1db0
	ctx.lr = 0x82FDEB80;
	sub_82FD1DB0(ctx, base);
	// 82FDEB80: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FDEB84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FDEB88: 41800014  blt 0x82fdeb9c
	if ctx.cr[0].lt {
	pc = 0x82FDEB9C; continue 'dispatch;
	}
	// 82FDEB8C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82FDEB90: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FDEB94: 4BFFF9CD  bl 0x82fde560
	ctx.lr = 0x82FDEB98;
	sub_82FDE560(ctx, base);
	// 82FDEB98: 48000010  b 0x82fdeba8
	pc = 0x82FDEBA8; continue 'dispatch;
	// 82FDEB9C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FDEBA0: 388B8158  addi r4, r11, -0x7ea8
	ctx.r[4].s64 = ctx.r[11].s64 + -32424;
	// 82FDEBA4: 4BFFF8FD  bl 0x82fde4a0
	ctx.lr = 0x82FDEBA8;
	sub_82FDE4A0(ctx, base);
	// 82FDEBA8: 397E0001  addi r11, r30, 1
	ctx.r[11].s64 = ctx.r[30].s64 + 1;
	// 82FDEBAC: 7D5EE050  subf r10, r30, r28
	ctx.r[10].s64 = ctx.r[28].s64 - ctx.r[30].s64;
	// 82FDEBB0: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FDEBB4: 38AAFFFF  addi r5, r10, -1
	ctx.r[5].s64 = ctx.r[10].s64 + -1;
	// 82FDEBB8: 7C8BEA14  add r4, r11, r29
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82FDEBBC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FDEBC0: 4BFFFAF9  bl 0x82fde6b8
	ctx.lr = 0x82FDEBC4;
	sub_82FDE6B8(ctx, base);
	// 82FDEBC4: 937F0020  stw r27, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[27].u32 ) };
	// 82FDEBC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FDEBCC: 481C95E8  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDEBD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDEBD0 size=76
    let mut pc: u32 = 0x82FDEBD0;
    'dispatch: loop {
        match pc {
            0x82FDEBD0 => {
    //   block [0x82FDEBD0..0x82FDEC1C)
	// 82FDEBD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDEBD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FDEBD8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FDEBDC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FDEBE0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDEBE4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FDEBE8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FDEBEC: 4BFFFE05  bl 0x82fde9f0
	ctx.lr = 0x82FDEBF0;
	sub_82FDE9F0(ctx, base);
	// 82FDEBF0: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDEBF4: 4182000C  beq 0x82fdec00
	if ctx.cr[0].eq {
	pc = 0x82FDEC00; continue 'dispatch;
	}
	// 82FDEBF8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FDEBFC: 4BFF96E5  bl 0x82fd82e0
	ctx.lr = 0x82FDEC00;
	sub_82FD82E0(ctx, base);
	// 82FDEC00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FDEC04: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FDEC08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FDEC0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FDEC10: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FDEC14: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FDEC18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDEC20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FDEC20 size=8
    let mut pc: u32 = 0x82FDEC20;
    'dispatch: loop {
        match pc {
            0x82FDEC20 => {
    //   block [0x82FDEC20..0x82FDEC28)
	// 82FDEC20: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FDEC24: 8213A50C  lwz r16, -0x5af4(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-23284 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDEC28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDEC28 size=160
    let mut pc: u32 = 0x82FDEC28;
    'dispatch: loop {
        match pc {
            0x82FDEC28 => {
    //   block [0x82FDEC28..0x82FDECC8)
	// 82FDEC28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDEC2C: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 82FDEC30: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 82FDEC34: 481C9531  bl 0x831a8164
	ctx.lr = 0x82FDEC38;
	sub_831A8130(ctx, base);
	// 82FDEC38: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 82FDEC3C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDEC40: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FDEC44: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82FDEC48: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82FDEC4C: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 82FDEC50: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FDEC54: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82FDEC58: 90FE0004  stw r7, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82FDEC5C: 396BA430  addi r11, r11, -0x5bd0
	ctx.r[11].s64 = ctx.r[11].s64 + -23504;
	// 82FDEC60: 93BE0008  stw r29, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82FDEC64: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FDEC68: 93BE000C  stw r29, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 82FDEC6C: 93BE0010  stw r29, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[29].u32 ) };
	// 82FDEC70: 93BE0014  stw r29, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[29].u32 ) };
	// 82FDEC74: 93BE001C  stw r29, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[29].u32 ) };
	// 82FDEC78: 93BE0018  stw r29, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[29].u32 ) };
	// 82FDEC7C: 93BE0020  stw r29, 0x20(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[29].u32 ) };
	// 82FDEC80: 4BFFF821  bl 0x82fde4a0
	ctx.lr = 0x82FDEC84;
	sub_82FDE4A0(ctx, base);
	// 82FDEC84: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FDEC88: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FDEC8C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FDEC90: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FDEC94: 4BFFF965  bl 0x82fde5f8
	ctx.lr = 0x82FDEC98;
	sub_82FDE5F8(ctx, base);
	// 82FDEC98: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FDEC9C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FDECA0: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FDECA4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDECA8: 41820008  beq 0x82fdecb0
	if ctx.cr[0].eq {
	pc = 0x82FDECB0; continue 'dispatch;
	}
	// 82FDECAC: B3AB0000  sth r29, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u16 ) };
	// 82FDECB0: 937E0020  stw r27, 0x20(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[27].u32 ) };
	// 82FDECB4: 48000008  b 0x82fdecbc
	pc = 0x82FDECBC; continue 'dispatch;
	// 82FDECB8: 83DF0094  lwz r30, 0x94(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FDECBC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FDECC0: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 82FDECC4: 481C94F0  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDECC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FDECC8 size=8
    let mut pc: u32 = 0x82FDECC8;
    'dispatch: loop {
        match pc {
            0x82FDECC8 => {
    //   block [0x82FDECC8..0x82FDECD0)
	// 82FDECC8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FDECCC: 8213A50C  lwz r16, -0x5af4(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-23284 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDECD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDECD0 size=24
    let mut pc: u32 = 0x82FDECD0;
    'dispatch: loop {
        match pc {
            0x82FDECD0 => {
    //   block [0x82FDECD0..0x82FDECE8)
	// 82FDECD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDECD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FDECD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDECDC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FDECE0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FDECE4: 481D1F45  bl 0x831b0c28
	ctx.lr = 0x82FDECE8;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDECF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDECF0 size=48
    let mut pc: u32 = 0x82FDECF0;
    'dispatch: loop {
        match pc {
            0x82FDECF0 => {
    //   block [0x82FDECF0..0x82FDED20)
	// 82FDECF0: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FDECF4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDECF8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FDECFC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDED00: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FDED04: 4BFFFB25  bl 0x82fde828
	ctx.lr = 0x82FDED08;
	sub_82FDE828(ctx, base);
	// 82FDED08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FDED0C: 3C6082FE  lis r3, -0x7d02
	ctx.r[3].s64 = -2097283072;
	// 82FDED10: 3863ECB8  addi r3, r3, -0x1348
	ctx.r[3].s64 = ctx.r[3].s64 + -4936;
	// 82FDED14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FDED18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FDED1C: 481C9498  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDED20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDED20 size=40
    let mut pc: u32 = 0x82FDED20;
    'dispatch: loop {
        match pc {
            0x82FDED20 => {
    //   block [0x82FDED20..0x82FDED48)
	// 82FDED20: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FDED24: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDED28: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FDED2C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDED30: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FDED34: 4806DA2D  bl 0x8304c760
	ctx.lr = 0x82FDED38;
	sub_8304C760(ctx, base);
	// 82FDED38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FDED3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FDED40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FDED44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDED48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FDED48 size=8
    let mut pc: u32 = 0x82FDED48;
    'dispatch: loop {
        match pc {
            0x82FDED48 => {
    //   block [0x82FDED48..0x82FDED50)
	// 82FDED48: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FDED4C: 8213A5C4  lwz r16, -0x5a3c(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-23100 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDED50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDED50 size=136
    let mut pc: u32 = 0x82FDED50;
    'dispatch: loop {
        match pc {
            0x82FDED50 => {
    //   block [0x82FDED50..0x82FDEDD8)
	// 82FDED50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDED54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FDED58: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 82FDED5C: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 82FDED60: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FDED64: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FDED68: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 82FDED6C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDED70: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FDED74: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 82FDED78: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FDED7C: 90DE0004  stw r6, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 82FDED80: 394BA430  addi r10, r11, -0x5bd0
	ctx.r[10].s64 = ctx.r[11].s64 + -23504;
	// 82FDED84: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FDED88: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82FDED8C: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82FDED90: 917E000C  stw r11, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82FDED94: 917E0010  stw r11, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82FDED98: 917E0014  stw r11, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82FDED9C: 917E0018  stw r11, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82FDEDA0: 917E001C  stw r11, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82FDEDA4: 917E0020  stw r11, 0x20(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82FDEDA8: 4BFFFD11  bl 0x82fdeab8
	ctx.lr = 0x82FDEDAC;
	sub_82FDEAB8(ctx, base);
	// 82FDEDAC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FDEDB0: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FDEDB4: 48000008  b 0x82fdedbc
	pc = 0x82FDEDBC; continue 'dispatch;
	// 82FDEDB8: 83DF0084  lwz r30, 0x84(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82FDEDBC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FDEDC0: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 82FDEDC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FDEDC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FDEDCC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FDEDD0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FDEDD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDEDD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FDEDD8 size=8
    let mut pc: u32 = 0x82FDEDD8;
    'dispatch: loop {
        match pc {
            0x82FDEDD8 => {
    //   block [0x82FDEDD8..0x82FDEDE0)
	// 82FDEDD8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FDEDDC: 8213A5C4  lwz r16, -0x5a3c(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-23100 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDEDE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDEDE0 size=24
    let mut pc: u32 = 0x82FDEDE0;
    'dispatch: loop {
        match pc {
            0x82FDEDE0 => {
    //   block [0x82FDEDE0..0x82FDEDF8)
	// 82FDEDE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDEDE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FDEDE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDEDEC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FDEDF0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FDEDF4: 481D1E35  bl 0x831b0c28
	ctx.lr = 0x82FDEDF8;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDEE00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDEE00 size=48
    let mut pc: u32 = 0x82FDEE00;
    'dispatch: loop {
        match pc {
            0x82FDEE00 => {
    //   block [0x82FDEE00..0x82FDEE30)
	// 82FDEE00: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 82FDEE04: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDEE08: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FDEE0C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDEE10: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82FDEE14: 4BFFFA15  bl 0x82fde828
	ctx.lr = 0x82FDEE18;
	sub_82FDE828(ctx, base);
	// 82FDEE18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FDEE1C: 3C6082FE  lis r3, -0x7d02
	ctx.r[3].s64 = -2097283072;
	// 82FDEE20: 3863EDB8  addi r3, r3, -0x1248
	ctx.r[3].s64 = ctx.r[3].s64 + -4680;
	// 82FDEE24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FDEE28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FDEE2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDEE30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDEE30 size=40
    let mut pc: u32 = 0x82FDEE30;
    'dispatch: loop {
        match pc {
            0x82FDEE30 => {
    //   block [0x82FDEE30..0x82FDEE58)
	// 82FDEE30: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 82FDEE34: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDEE38: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FDEE3C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDEE40: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82FDEE44: 4806D91D  bl 0x8304c760
	ctx.lr = 0x82FDEE48;
	sub_8304C760(ctx, base);
	// 82FDEE48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FDEE4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FDEE50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FDEE54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDEE58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDEE58 size=124
    let mut pc: u32 = 0x82FDEE58;
    'dispatch: loop {
        match pc {
            0x82FDEE58 => {
    //   block [0x82FDEE58..0x82FDEED4)
	// 82FDEE58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDEE5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FDEE60: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FDEE64: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FDEE68: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDEE6C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FDEE70: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82FDEE74: 39640001  addi r11, r4, 1
	ctx.r[11].s64 = ctx.r[4].s64 + 1;
	// 82FDEE78: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82FDEE7C: 909F0008  stw r4, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 82FDEE80: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82FDEE84: 9BDF0000  stb r30, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u8 ) };
	// 82FDEE88: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82FDEE8C: 90BF000C  stw r5, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[5].u32 ) };
	// 82FDEE90: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 82FDEE94: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 82FDEE98: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 82FDEE9C: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDEEA0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDEEA4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDEEA8: 4E800421  bctrl
	ctx.lr = 0x82FDEEAC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDEEAC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82FDEEB0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FDEEB4: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82FDEEB8: B3CB0000  sth r30, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u16 ) };
	// 82FDEEBC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FDEEC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FDEEC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FDEEC8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FDEECC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FDEED0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDEED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FDEED8 size=28
    let mut pc: u32 = 0x82FDEED8;
    'dispatch: loop {
        match pc {
            0x82FDEED8 => {
    //   block [0x82FDEED8..0x82FDEEF4)
	// 82FDEED8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82FDEEDC: 806B000C  lwz r3, 0xc(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FDEEE0: 808B0018  lwz r4, 0x18(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FDEEE4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDEEE8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FDEEEC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDEEF0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDEEF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDEEF8 size=148
    let mut pc: u32 = 0x82FDEEF8;
    'dispatch: loop {
        match pc {
            0x82FDEEF8 => {
    //   block [0x82FDEEF8..0x82FDEF8C)
	// 82FDEEF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDEEFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FDEF00: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FDEF04: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FDEF08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDEF0C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FDEF10: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82FDEF14: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
	// 82FDEF18: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82FDEF1C: 38800080  li r4, 0x80
	ctx.r[4].s64 = 128;
	// 82FDEF20: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82FDEF24: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FDEF28: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82FDEF2C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDEF30: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDEF34: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDEF38: 4E800421  bctrl
	ctx.lr = 0x82FDEF3C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDEF3C: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDEF40: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82FDEF44: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 82FDEF48: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82FDEF4C: 40990024  ble cr6, 0x82fdef70
	if !ctx.cr[6].gt {
	pc = 0x82FDEF70; continue 'dispatch;
	}
	// 82FDEF50: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 82FDEF54: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FDEF58: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FDEF5C: 7FC9512E  stwx r30, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 82FDEF60: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82FDEF64: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDEF68: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82FDEF6C: 4198FFE8  blt cr6, 0x82fdef54
	if ctx.cr[6].lt {
	pc = 0x82FDEF54; continue 'dispatch;
	}
	// 82FDEF70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FDEF74: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FDEF78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FDEF7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FDEF80: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FDEF84: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FDEF88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDEF90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDEF90 size=96
    let mut pc: u32 = 0x82FDEF90;
    'dispatch: loop {
        match pc {
            0x82FDEF90 => {
    //   block [0x82FDEF90..0x82FDEFF0)
	// 82FDEF90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDEF94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FDEF98: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FDEF9C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FDEFA0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDEFA4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FDEFA8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FDEFAC: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FDEFB0: 809F0018  lwz r4, 0x18(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FDEFB4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDEFB8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FDEFBC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDEFC0: 4E800421  bctrl
	ctx.lr = 0x82FDEFC4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDEFC4: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDEFC8: 4182000C  beq 0x82fdefd4
	if ctx.cr[0].eq {
	pc = 0x82FDEFD4; continue 'dispatch;
	}
	// 82FDEFCC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FDEFD0: 4BFF9311  bl 0x82fd82e0
	ctx.lr = 0x82FDEFD4;
	sub_82FD82E0(ctx, base);
	// 82FDEFD4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FDEFD8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FDEFDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FDEFE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FDEFE4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FDEFE8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FDEFEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDEFF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FDEFF0 size=8
    let mut pc: u32 = 0x82FDEFF0;
    'dispatch: loop {
        match pc {
            0x82FDEFF0 => {
    //   block [0x82FDEFF0..0x82FDEFF8)
	// 82FDEFF0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FDEFF4: 8213A648  lwz r16, -0x59b8(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-22968 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDEFF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDEFF8 size=252
    let mut pc: u32 = 0x82FDEFF8;
    'dispatch: loop {
        match pc {
            0x82FDEFF8 => {
    //   block [0x82FDEFF8..0x82FDF0F4)
	// 82FDEFF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDEFFC: 481C916D  bl 0x831a8168
	ctx.lr = 0x82FDF000;
	sub_831A8130(ctx, base);
	// 82FDF000: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 82FDF004: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDF008: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FDF00C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82FDF010: 7F9DE378  mr r29, r28
	ctx.r[29].u64 = ctx.r[28].u64;
	// 82FDF014: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDF018: 93DF00B4  stw r30, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[30].u32 ) };
	// 82FDF01C: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDF020: 41820034  beq 0x82fdf054
	if ctx.cr[0].eq {
	pc = 0x82FDF054; continue 'dispatch;
	}
	// 82FDF024: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FDF028: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDF02C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82FDF030: 419A0050  beq cr6, 0x82fdf080
	if ctx.cr[6].eq {
	pc = 0x82FDF080; continue 'dispatch;
	}
	// 82FDF034: 5529003E  slwi r9, r9, 0
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(0);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82FDF038: 89290000  lbz r9, 0(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDF03C: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDF040: 41820098  beq 0x82fdf0d8
	if ctx.cr[0].eq {
	pc = 0x82FDF0D8; continue 'dispatch;
	}
	// 82FDF044: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82FDF048: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82FDF04C: 7F1D5040  cmplw cr6, r29, r10
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82FDF050: 4198FFD8  blt cr6, 0x82fdf028
	if ctx.cr[6].lt {
	pc = 0x82FDF028; continue 'dispatch;
	}
	// 82FDF054: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FDF058: 80FE0004  lwz r7, 4(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDF05C: 38C0000D  li r6, 0xd
	ctx.r[6].s64 = 13;
	// 82FDF060: 388BA608  addi r4, r11, -0x59f8
	ctx.r[4].s64 = ctx.r[11].s64 + -23032;
	// 82FDF064: 38A00079  li r5, 0x79
	ctx.r[5].s64 = 121;
	// 82FDF068: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FDF06C: 4BFF1FED  bl 0x82fd1058
	ctx.lr = 0x82FDF070;
	sub_82FD1058(ctx, base);
	// 82FDF070: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FDF074: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FDF078: 388BC3EC  addi r4, r11, -0x3c14
	ctx.r[4].s64 = ctx.r[11].s64 + -15380;
	// 82FDF07C: 481D1BAD  bl 0x831b0c28
	ctx.lr = 0x82FDF080;
	sub_831B0C28(ctx, base);
	// 82FDF080: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 82FDF084: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDF088: 4BFF9211  bl 0x82fd8298
	ctx.lr = 0x82FDF08C;
	sub_82FD8298(ctx, base);
	// 82FDF08C: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FDF090: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDF094: 41820014  beq 0x82fdf0a8
	if ctx.cr[0].eq {
	pc = 0x82FDF0A8; continue 'dispatch;
	}
	// 82FDF098: 388003FF  li r4, 0x3ff
	ctx.r[4].s64 = 1023;
	// 82FDF09C: 80BE0004  lwz r5, 4(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDF0A0: 4BFFFDB9  bl 0x82fdee58
	ctx.lr = 0x82FDF0A4;
	sub_82FDEE58(ctx, base);
	// 82FDF0A4: 48000008  b 0x82fdf0ac
	pc = 0x82FDF0AC; continue 'dispatch;
	// 82FDF0A8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FDF0AC: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FDF0B0: 57AB103A  slwi r11, r29, 2
	ctx.r[11].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FDF0B4: 7C6B512E  stwx r3, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[3].u32) };
	// 82FDF0B8: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FDF0BC: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82FDF0C0: 7D4B502E  lwzx r10, r11, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82FDF0C4: 992A0000  stb r9, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 82FDF0C8: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FDF0CC: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82FDF0D0: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 82FDF0D4: 481C90E4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 82FDF0D8: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FDF0DC: 57AB103A  slwi r11, r29, 2
	ctx.r[11].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FDF0E0: 7D4B502E  lwzx r10, r11, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82FDF0E4: 810A0018  lwz r8, 0x18(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FDF0E8: 938A0004  stw r28, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82FDF0EC: B3880000  sth r28, 0(r8)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[28].u16 ) };
	// 82FDF0F0: 4BFFFFC8  b 0x82fdf0b8
	pc = 0x82FDF0B8; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDF0F4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDF0F4 size=48
    let mut pc: u32 = 0x82FDF0F4;
    'dispatch: loop {
        match pc {
            0x82FDF0F4 => {
    //   block [0x82FDF0F4..0x82FDF124)
	// 82FDF0F4: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 82FDF0F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDF0FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FDF100: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDF104: 817F00B4  lwz r11, 0xb4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 82FDF108: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDF10C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FDF110: 4BFF91D1  bl 0x82fd82e0
	ctx.lr = 0x82FDF114;
	sub_82FD82E0(ctx, base);
	// 82FDF114: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FDF118: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FDF11C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FDF120: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDF128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDF128 size=128
    let mut pc: u32 = 0x82FDF128;
    'dispatch: loop {
        match pc {
            0x82FDF128 => {
    //   block [0x82FDF128..0x82FDF1A8)
	// 82FDF128: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDF12C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FDF130: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDF134: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDF138: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FDF13C: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDF140: 41820024  beq 0x82fdf164
	if ctx.cr[0].eq {
	pc = 0x82FDF164; continue 'dispatch;
	}
	// 82FDF144: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FDF148: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDF14C: 7F082040  cmplw cr6, r8, r4
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82FDF150: 419A0040  beq cr6, 0x82fdf190
	if ctx.cr[6].eq {
	pc = 0x82FDF190; continue 'dispatch;
	}
	// 82FDF154: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FDF158: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82FDF15C: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82FDF160: 4198FFE8  blt cr6, 0x82fdf148
	if ctx.cr[6].lt {
	pc = 0x82FDF148; continue 'dispatch;
	}
	// 82FDF164: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FDF168: 80E30004  lwz r7, 4(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDF16C: 38C0000E  li r6, 0xe
	ctx.r[6].s64 = 14;
	// 82FDF170: 388BA608  addi r4, r11, -0x59f8
	ctx.r[4].s64 = ctx.r[11].s64 + -23032;
	// 82FDF174: 38A0008E  li r5, 0x8e
	ctx.r[5].s64 = 142;
	// 82FDF178: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FDF17C: 4BFF1EDD  bl 0x82fd1058
	ctx.lr = 0x82FDF180;
	sub_82FD1058(ctx, base);
	// 82FDF180: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FDF184: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FDF188: 388BC3EC  addi r4, r11, -0x3c14
	ctx.r[4].s64 = ctx.r[11].s64 + -15380;
	// 82FDF18C: 481D1A9D  bl 0x831b0c28
	ctx.lr = 0x82FDF190;
	sub_831B0C28(ctx, base);
	// 82FDF190: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FDF194: 99640000  stb r11, 0(r4)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82FDF198: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FDF19C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FDF1A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FDF1A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDF1A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDF1A8 size=112
    let mut pc: u32 = 0x82FDF1A8;
    'dispatch: loop {
        match pc {
            0x82FDF1A8 => {
    //   block [0x82FDF1A8..0x82FDF218)
	// 82FDF1A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDF1AC: 481C8FC1  bl 0x831a816c
	ctx.lr = 0x82FDF1B0;
	sub_831A8130(ctx, base);
	// 82FDF1B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDF1B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FDF1B8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82FDF1BC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDF1C0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FDF1C4: 40990034  ble cr6, 0x82fdf1f8
	if !ctx.cr[6].gt {
	pc = 0x82FDF1F8; continue 'dispatch;
	}
	// 82FDF1C8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82FDF1CC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FDF1D0: 7C7E582E  lwzx r3, r30, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FDF1D4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDF1D8: 4182000C  beq 0x82fdf1e4
	if ctx.cr[0].eq {
	pc = 0x82FDF1E4; continue 'dispatch;
	}
	// 82FDF1DC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82FDF1E0: 4BFFFDB1  bl 0x82fdef90
	ctx.lr = 0x82FDF1E4;
	sub_82FDEF90(ctx, base);
	// 82FDF1E4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDF1E8: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82FDF1EC: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82FDF1F0: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FDF1F4: 4198FFD8  blt cr6, 0x82fdf1cc
	if ctx.cr[6].lt {
	pc = 0x82FDF1CC; continue 'dispatch;
	}
	// 82FDF1F8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDF1FC: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FDF200: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDF204: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FDF208: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDF20C: 4E800421  bctrl
	ctx.lr = 0x82FDF210;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDF210: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FDF214: 481C8FA8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDF218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FDF218 size=16
    let mut pc: u32 = 0x82FDF218;
    'dispatch: loop {
        match pc {
            0x82FDF218 => {
    //   block [0x82FDF218..0x82FDF228)
	// 82FDF218: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FDF21C: 90830000  stw r4, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82FDF220: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82FDF224: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDF228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDF228 size=124
    let mut pc: u32 = 0x82FDF228;
    'dispatch: loop {
        match pc {
            0x82FDF228 => {
    //   block [0x82FDF228..0x82FDF2A4)
	// 82FDF228: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDF22C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FDF230: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FDF234: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDF238: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FDF23C: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDF240: 556AB2BE  srwi r10, r11, 0xa
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shr(10);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FDF244: 554A07FF  clrlwi. r10, r10, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82FDF248: 4082000C  bne 0x82fdf254
	if !ctx.cr[0].eq {
	pc = 0x82FDF254; continue 'dispatch;
	}
	// 82FDF24C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FDF250: 48000040  b 0x82fdf290
	pc = 0x82FDF290; continue 'dispatch;
	// 82FDF254: 556BECFE  rlwinm r11, r11, 0x1d, 0x13, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000007u64;
	// 82FDF258: 556B07FF  clrlwi. r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDF25C: 41820020  beq 0x82fdf27c
	if ctx.cr[0].eq {
	pc = 0x82FDF27C; continue 'dispatch;
	}
	// 82FDF260: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDF264: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDF268: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82FDF26C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDF270: 4E800421  bctrl
	ctx.lr = 0x82FDF274;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDF274: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDF278: 40820018  bne 0x82fdf290
	if !ctx.cr[0].eq {
	pc = 0x82FDF290; continue 'dispatch;
	}
	// 82FDF27C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDF280: 386BFFF4  addi r3, r11, -0xc
	ctx.r[3].s64 = ctx.r[11].s64 + -12;
	// 82FDF284: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDF288: 40820008  bne 0x82fdf290
	if !ctx.cr[0].eq {
	pc = 0x82FDF290; continue 'dispatch;
	}
	// 82FDF28C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FDF290: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FDF294: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FDF298: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FDF29C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FDF2A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDF2A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FDF2A8 size=16
    let mut pc: u32 = 0x82FDF2A8;
    'dispatch: loop {
        match pc {
            0x82FDF2A8 => {
    //   block [0x82FDF2A8..0x82FDF2B8)
	// 82FDF2A8: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDF2AC: 556BE8FE  srwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shr(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FDF2B0: 556B07FF  clrlwi. r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDF2B4: 4C820020  bnelr
	if !ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDF2B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FDF2B8 size=24
    let mut pc: u32 = 0x82FDF2B8;
    'dispatch: loop {
        match pc {
            0x82FDF2B8 => {
    //   block [0x82FDF2B8..0x82FDF2D0)
	// 82FDF2B8: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82FDF2BC: 3964000C  addi r11, r4, 0xc
	ctx.r[11].s64 = ctx.r[4].s64 + 12;
	// 82FDF2C0: 409A0008  bne cr6, 0x82fdf2c8
	if !ctx.cr[6].eq {
	pc = 0x82FDF2C8; continue 'dispatch;
	}
	// 82FDF2C4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FDF2C8: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FDF2CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDF2D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDF2D0 size=96
    let mut pc: u32 = 0x82FDF2D0;
    'dispatch: loop {
        match pc {
            0x82FDF2D0 => {
    //   block [0x82FDF2D0..0x82FDF330)
	// 82FDF2D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDF2D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FDF2D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FDF2DC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDF2E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FDF2E4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82FDF2E8: 419A0020  beq cr6, 0x82fdf308
	if ctx.cr[6].eq {
	pc = 0x82FDF308; continue 'dispatch;
	}
	// 82FDF2EC: 4BFFFF3D  bl 0x82fdf228
	ctx.lr = 0x82FDF2F0;
	sub_82FDF228(ctx, base);
	// 82FDF2F0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDF2F4: 41820014  beq 0x82fdf308
	if ctx.cr[0].eq {
	pc = 0x82FDF308; continue 'dispatch;
	}
	// 82FDF2F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FDF2FC: 4BFFFF2D  bl 0x82fdf228
	ctx.lr = 0x82FDF300;
	sub_82FDF228(ctx, base);
	// 82FDF300: 80C30090  lwz r6, 0x90(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FDF304: 4800000C  b 0x82fdf310
	pc = 0x82FDF310; continue 'dispatch;
	// 82FDF308: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 82FDF30C: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FDF310: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FDF314: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82FDF318: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FDF31C: 4801ABB5  bl 0x82ff9ed0
	ctx.lr = 0x82FDF320;
	sub_82FF9ED0(ctx, base);
	// 82FDF320: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FDF324: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FDF328: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 82FDF32C: 481D18FD  bl 0x831b0c28
	ctx.lr = 0x82FDF330;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDF330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDF330 size=96
    let mut pc: u32 = 0x82FDF330;
    'dispatch: loop {
        match pc {
            0x82FDF330 => {
    //   block [0x82FDF330..0x82FDF390)
	// 82FDF330: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDF334: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FDF338: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FDF33C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDF340: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FDF344: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82FDF348: 419A0020  beq cr6, 0x82fdf368
	if ctx.cr[6].eq {
	pc = 0x82FDF368; continue 'dispatch;
	}
	// 82FDF34C: 4BFFFEDD  bl 0x82fdf228
	ctx.lr = 0x82FDF350;
	sub_82FDF228(ctx, base);
	// 82FDF350: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDF354: 41820014  beq 0x82fdf368
	if ctx.cr[0].eq {
	pc = 0x82FDF368; continue 'dispatch;
	}
	// 82FDF358: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FDF35C: 4BFFFECD  bl 0x82fdf228
	ctx.lr = 0x82FDF360;
	sub_82FDF228(ctx, base);
	// 82FDF360: 80C30090  lwz r6, 0x90(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FDF364: 4800000C  b 0x82fdf370
	pc = 0x82FDF370; continue 'dispatch;
	// 82FDF368: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 82FDF36C: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FDF370: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FDF374: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 82FDF378: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FDF37C: 4801AB55  bl 0x82ff9ed0
	ctx.lr = 0x82FDF380;
	sub_82FF9ED0(ctx, base);
	// 82FDF380: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FDF384: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FDF388: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 82FDF38C: 481D189D  bl 0x831b0c28
	ctx.lr = 0x82FDF390;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDF390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDF390 size=240
    let mut pc: u32 = 0x82FDF390;
    'dispatch: loop {
        match pc {
            0x82FDF390 => {
    //   block [0x82FDF390..0x82FDF480)
	// 82FDF390: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDF394: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FDF398: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FDF39C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FDF3A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDF3A4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FDF3A8: 57CB063F  clrlwi. r11, r30, 0x18
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDF3AC: 41820010  beq 0x82fdf3bc
	if ctx.cr[0].eq {
	pc = 0x82FDF3BC; continue 'dispatch;
	}
	// 82FDF3B0: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDF3B4: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 82FDF3B8: 4800000C  b 0x82fdf3c4
	pc = 0x82FDF3C4; continue 'dispatch;
	// 82FDF3BC: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDF3C0: 556B003C  rlwinm r11, r11, 0, 0, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82FDF3C4: 54AA063F  clrlwi. r10, r5, 0x18
	ctx.r[10].u64 = ctx.r[5].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82FDF3C8: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82FDF3CC: 4182009C  beq 0x82fdf468
	if ctx.cr[0].eq {
	pc = 0x82FDF468; continue 'dispatch;
	}
	// 82FDF3D0: 3863FFFC  addi r3, r3, -4
	ctx.r[3].s64 = ctx.r[3].s64 + -4;
	// 82FDF3D4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDF3D8: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FDF3DC: 4800007C  b 0x82fdf458
	pc = 0x82FDF458; continue 'dispatch;
	// 82FDF3E0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDF3E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FDF3E8: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FDF3EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDF3F0: 4E800421  bctrl
	ctx.lr = 0x82FDF3F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDF3F4: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 82FDF3F8: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82FDF3FC: 419A0034  beq cr6, 0x82fdf430
	if ctx.cr[6].eq {
	pc = 0x82FDF430; continue 'dispatch;
	}
	// 82FDF400: 2F0B0005  cmpwi cr6, r11, 5
	ctx.cr[6].compare_i32(ctx.r[11].s32, 5, &mut ctx.xer);
	// 82FDF404: 419A0048  beq cr6, 0x82fdf44c
	if ctx.cr[6].eq {
	pc = 0x82FDF44C; continue 'dispatch;
	}
	// 82FDF408: 2F0B000A  cmpwi cr6, r11, 0xa
	ctx.cr[6].compare_i32(ctx.r[11].s32, 10, &mut ctx.xer);
	// 82FDF40C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82FDF410: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FDF414: 419A0010  beq cr6, 0x82fdf424
	if ctx.cr[6].eq {
	pc = 0x82FDF424; continue 'dispatch;
	}
	// 82FDF418: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82FDF41C: 4BFFFF75  bl 0x82fdf390
	ctx.lr = 0x82FDF420;
	sub_82FDF390(ctx, base);
	// 82FDF420: 4800002C  b 0x82fdf44c
	pc = 0x82FDF44C; continue 'dispatch;
	// 82FDF424: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDF428: 816B00C8  lwz r11, 0xc8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(200 as u32) ) } as u64;
	// 82FDF42C: 48000014  b 0x82fdf440
	pc = 0x82FDF440; continue 'dispatch;
	// 82FDF430: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDF434: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82FDF438: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FDF43C: 816B00EC  lwz r11, 0xec(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(236 as u32) ) } as u64;
	// 82FDF440: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FDF444: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDF448: 4E800421  bctrl
	ctx.lr = 0x82FDF44C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDF44C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDF450: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FDF454: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FDF458: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDF45C: 4E800421  bctrl
	ctx.lr = 0x82FDF460;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDF460: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82FDF464: 4082FF7C  bne 0x82fdf3e0
	if !ctx.cr[0].eq {
	pc = 0x82FDF3E0; continue 'dispatch;
	}
	// 82FDF468: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FDF46C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FDF470: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FDF474: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FDF478: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FDF47C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDF480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDF480 size=80
    let mut pc: u32 = 0x82FDF480;
    'dispatch: loop {
        match pc {
            0x82FDF480 => {
    //   block [0x82FDF480..0x82FDF4D0)
	// 82FDF480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDF484: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FDF488: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FDF48C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FDF490: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDF494: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82FDF498: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82FDF49C: 4801093D  bl 0x82fefdd8
	ctx.lr = 0x82FDF4A0;
	sub_82FEFDD8(ctx, base);
	// 82FDF4A0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDF4A4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82FDF4A8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FDF4AC: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FDF4B0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDF4B4: 4E800421  bctrl
	ctx.lr = 0x82FDF4B8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDF4B8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FDF4BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FDF4C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FDF4C4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FDF4C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FDF4CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDF4D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDF4D0 size=96
    let mut pc: u32 = 0x82FDF4D0;
    'dispatch: loop {
        match pc {
            0x82FDF4D0 => {
    //   block [0x82FDF4D0..0x82FDF530)
	// 82FDF4D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDF4D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FDF4D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FDF4DC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDF4E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FDF4E4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82FDF4E8: 419A0020  beq cr6, 0x82fdf508
	if ctx.cr[6].eq {
	pc = 0x82FDF508; continue 'dispatch;
	}
	// 82FDF4EC: 4BFFFD3D  bl 0x82fdf228
	ctx.lr = 0x82FDF4F0;
	sub_82FDF228(ctx, base);
	// 82FDF4F0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDF4F4: 41820014  beq 0x82fdf508
	if ctx.cr[0].eq {
	pc = 0x82FDF508; continue 'dispatch;
	}
	// 82FDF4F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FDF4FC: 4BFFFD2D  bl 0x82fdf228
	ctx.lr = 0x82FDF500;
	sub_82FDF228(ctx, base);
	// 82FDF500: 80C30090  lwz r6, 0x90(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FDF504: 4800000C  b 0x82fdf510
	pc = 0x82FDF510; continue 'dispatch;
	// 82FDF508: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 82FDF50C: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FDF510: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FDF514: 3880000E  li r4, 0xe
	ctx.r[4].s64 = 14;
	// 82FDF518: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FDF51C: 4801A9B5  bl 0x82ff9ed0
	ctx.lr = 0x82FDF520;
	sub_82FF9ED0(ctx, base);
	// 82FDF520: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FDF524: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FDF528: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 82FDF52C: 481D16FD  bl 0x831b0c28
	ctx.lr = 0x82FDF530;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDF530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FDF530 size=12
    let mut pc: u32 = 0x82FDF530;
    'dispatch: loop {
        match pc {
            0x82FDF530 => {
    //   block [0x82FDF530..0x82FDF53C)
	// 82FDF530: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FDF534: 386B7F24  addi r3, r11, 0x7f24
	ctx.r[3].s64 = ctx.r[11].s64 + 32548;
	// 82FDF538: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDF540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FDF540 size=12
    let mut pc: u32 = 0x82FDF540;
    'dispatch: loop {
        match pc {
            0x82FDF540 => {
    //   block [0x82FDF540..0x82FDF54C)
	// 82FDF540: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FDF544: 386B80C8  addi r3, r11, -0x7f38
	ctx.r[3].s64 = ctx.r[11].s64 + -32568;
	// 82FDF548: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDF550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FDF550 size=12
    let mut pc: u32 = 0x82FDF550;
    'dispatch: loop {
        match pc {
            0x82FDF550 => {
    //   block [0x82FDF550..0x82FDF55C)
	// 82FDF550: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FDF554: 386B8018  addi r3, r11, -0x7fe8
	ctx.r[3].s64 = ctx.r[11].s64 + -32744;
	// 82FDF558: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDF560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FDF560 size=12
    let mut pc: u32 = 0x82FDF560;
    'dispatch: loop {
        match pc {
            0x82FDF560 => {
    //   block [0x82FDF560..0x82FDF56C)
	// 82FDF560: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FDF564: 386B8034  addi r3, r11, -0x7fcc
	ctx.r[3].s64 = ctx.r[11].s64 + -32716;
	// 82FDF568: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDF570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDF570 size=308
    let mut pc: u32 = 0x82FDF570;
    'dispatch: loop {
        match pc {
            0x82FDF570 => {
    //   block [0x82FDF570..0x82FDF6A4)
	// 82FDF570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDF574: 481C8BF9  bl 0x831a816c
	ctx.lr = 0x82FDF578;
	sub_831A8130(ctx, base);
	// 82FDF578: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDF57C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FDF580: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FDF584: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82FDF588: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82FDF58C: 409A0010  bne cr6, 0x82fdf59c
	if !ctx.cr[6].eq {
	pc = 0x82FDF59C; continue 'dispatch;
	}
	// 82FDF590: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FDF594: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82FDF598: 481C8C24  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 82FDF59C: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FDF5A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FDF5A4: 388B7F24  addi r4, r11, 0x7f24
	ctx.r[4].s64 = ctx.r[11].s64 + 32548;
	// 82FDF5A8: 4BFF4699  bl 0x82fd3c40
	ctx.lr = 0x82FDF5AC;
	sub_82FD3C40(ctx, base);
	// 82FDF5AC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDF5B0: 41820050  beq 0x82fdf600
	if ctx.cr[0].eq {
	pc = 0x82FDF600; continue 'dispatch;
	}
	// 82FDF5B4: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FDF5B8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FDF5BC: 3BEB80C8  addi r31, r11, -0x7f38
	ctx.r[31].s64 = ctx.r[11].s64 + -32568;
	// 82FDF5C0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FDF5C4: 4BFF467D  bl 0x82fd3c40
	ctx.lr = 0x82FDF5C8;
	sub_82FD3C40(ctx, base);
	// 82FDF5C8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDF5CC: 4182000C  beq 0x82fdf5d8
	if ctx.cr[0].eq {
	pc = 0x82FDF5D8; continue 'dispatch;
	}
	// 82FDF5D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FDF5D4: 4BFFFFC0  b 0x82fdf594
	pc = 0x82FDF594; continue 'dispatch;
	// 82FDF5D8: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 82FDF5DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FDF5E0: 3880000E  li r4, 0xe
	ctx.r[4].s64 = 14;
	// 82FDF5E4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FDF5E8: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FDF5EC: 4801A8E5  bl 0x82ff9ed0
	ctx.lr = 0x82FDF5F0;
	sub_82FF9ED0(ctx, base);
	// 82FDF5F0: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FDF5F4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FDF5F8: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 82FDF5FC: 481D162D  bl 0x831b0c28
	ctx.lr = 0x82FDF600;
	sub_831B0C28(ctx, base);
	// 82FDF600: 7FAB0734  extsh r11, r29
	ctx.r[11].s64 = ctx.r[29].s16 as i64;
	// 82FDF604: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82FDF608: 409A0060  bne cr6, 0x82fdf668
	if !ctx.cr[6].eq {
	pc = 0x82FDF668; continue 'dispatch;
	}
	// 82FDF60C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FDF610: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FDF614: 388B8018  addi r4, r11, -0x7fe8
	ctx.r[4].s64 = ctx.r[11].s64 + -32744;
	// 82FDF618: 4BFF4629  bl 0x82fd3c40
	ctx.lr = 0x82FDF61C;
	sub_82FD3C40(ctx, base);
	// 82FDF61C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDF620: 41820048  beq 0x82fdf668
	if ctx.cr[0].eq {
	pc = 0x82FDF668; continue 'dispatch;
	}
	// 82FDF624: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FDF628: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FDF62C: 3BEB8034  addi r31, r11, -0x7fcc
	ctx.r[31].s64 = ctx.r[11].s64 + -32716;
	// 82FDF630: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FDF634: 4BFF460D  bl 0x82fd3c40
	ctx.lr = 0x82FDF638;
	sub_82FD3C40(ctx, base);
	// 82FDF638: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDF63C: 4082FF94  bne 0x82fdf5d0
	if !ctx.cr[0].eq {
	pc = 0x82FDF5D0; continue 'dispatch;
	}
	// 82FDF640: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 82FDF644: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FDF648: 3880000E  li r4, 0xe
	ctx.r[4].s64 = 14;
	// 82FDF64C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FDF650: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FDF654: 4801A87D  bl 0x82ff9ed0
	ctx.lr = 0x82FDF658;
	sub_82FF9ED0(ctx, base);
	// 82FDF658: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FDF65C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FDF660: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 82FDF664: 481D15C5  bl 0x831b0c28
	ctx.lr = 0x82FDF668;
	sub_831B0C28(ctx, base);
	// 82FDF668: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82FDF66C: 419A0010  beq cr6, 0x82fdf67c
	if ctx.cr[6].eq {
	pc = 0x82FDF67C; continue 'dispatch;
	}
	// 82FDF670: A17E0000  lhz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDF674: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDF678: 4082FF18  bne 0x82fdf590
	if !ctx.cr[0].eq {
	pc = 0x82FDF590; continue 'dispatch;
	}
	// 82FDF67C: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 82FDF680: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FDF684: 3880000E  li r4, 0xe
	ctx.r[4].s64 = 14;
	// 82FDF688: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FDF68C: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FDF690: 4801A841  bl 0x82ff9ed0
	ctx.lr = 0x82FDF694;
	sub_82FF9ED0(ctx, base);
	// 82FDF694: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FDF698: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FDF69C: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 82FDF6A0: 481D1589  bl 0x831b0c28
	ctx.lr = 0x82FDF6A4;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDF6A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDF6A8 size=108
    let mut pc: u32 = 0x82FDF6A8;
    'dispatch: loop {
        match pc {
            0x82FDF6A8 => {
    //   block [0x82FDF6A8..0x82FDF714)
	// 82FDF6A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDF6AC: 481C8ABD  bl 0x831a8168
	ctx.lr = 0x82FDF6B0;
	sub_831A8130(ctx, base);
	// 82FDF6B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDF6B4: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82FDF6B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FDF6BC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82FDF6C0: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82FDF6C4: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82FDF6C8: 409A001C  bne cr6, 0x82fdf6e4
	if !ctx.cr[6].eq {
	pc = 0x82FDF6E4; continue 'dispatch;
	}
	// 82FDF6CC: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDF6D0: 556BBA7E  srwi r11, r11, 9
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shr(9);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FDF6D4: 556B07FF  clrlwi. r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDF6D8: 4082000C  bne 0x82fdf6e4
	if !ctx.cr[0].eq {
	pc = 0x82FDF6E4; continue 'dispatch;
	}
	// 82FDF6DC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FDF6E0: 4800002C  b 0x82fdf70c
	pc = 0x82FDF70C; continue 'dispatch;
	// 82FDF6E4: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDF6E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FDF6EC: 616B0200  ori r11, r11, 0x200
	ctx.r[11].u64 = ctx.r[11].u64 | 512;
	// 82FDF6F0: B17F0004  sth r11, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82FDF6F4: 4BFFFB35  bl 0x82fdf228
	ctx.lr = 0x82FDF6F8;
	sub_82FDF228(ctx, base);
	// 82FDF6F8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FDF6FC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82FDF700: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82FDF704: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82FDF708: 48006A21  bl 0x82fe6128
	ctx.lr = 0x82FDF70C;
	sub_82FE6128(ctx, base);
	// 82FDF70C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FDF710: 481C8AA8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDF718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDF718 size=92
    let mut pc: u32 = 0x82FDF718;
    'dispatch: loop {
        match pc {
            0x82FDF718 => {
    //   block [0x82FDF718..0x82FDF774)
	// 82FDF718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDF71C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FDF720: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FDF724: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FDF728: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDF72C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FDF730: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FDF734: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDF738: 556BBA7E  srwi r11, r11, 9
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shr(9);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FDF73C: 556B07FF  clrlwi. r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDF740: 41820018  beq 0x82fdf758
	if ctx.cr[0].eq {
	pc = 0x82FDF758; continue 'dispatch;
	}
	// 82FDF744: 4BFFFAE5  bl 0x82fdf228
	ctx.lr = 0x82FDF748;
	sub_82FDF228(ctx, base);
	// 82FDF748: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FDF74C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82FDF750: 480050B1  bl 0x82fe4800
	ctx.lr = 0x82FDF754;
	sub_82FE4800(ctx, base);
	// 82FDF754: 48000008  b 0x82fdf75c
	pc = 0x82FDF75C; continue 'dispatch;
	// 82FDF758: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FDF75C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FDF760: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FDF764: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FDF768: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FDF76C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FDF770: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDF778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDF778 size=68
    let mut pc: u32 = 0x82FDF778;
    'dispatch: loop {
        match pc {
            0x82FDF778 => {
    //   block [0x82FDF778..0x82FDF7BC)
	// 82FDF778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDF77C: 481C89ED  bl 0x831a8168
	ctx.lr = 0x82FDF780;
	sub_831A8130(ctx, base);
	// 82FDF780: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDF784: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FDF788: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FDF78C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82FDF790: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82FDF794: 4BFFFA95  bl 0x82fdf228
	ctx.lr = 0x82FDF798;
	sub_82FDF228(ctx, base);
	// 82FDF798: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDF79C: 41820018  beq 0x82fdf7b4
	if ctx.cr[0].eq {
	pc = 0x82FDF7B4; continue 'dispatch;
	}
	// 82FDF7A0: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82FDF7A4: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82FDF7A8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82FDF7AC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FDF7B0: 48006BC1  bl 0x82fe6370
	ctx.lr = 0x82FDF7B4;
	sub_82FE6370(ctx, base);
	// 82FDF7B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FDF7B8: 481C8A00  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDF7C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FDF7C0 size=24
    let mut pc: u32 = 0x82FDF7C0;
    'dispatch: loop {
        match pc {
            0x82FDF7C0 => {
    //   block [0x82FDF7C0..0x82FDF7D8)
	// 82FDF7C0: 3963FFFC  addi r11, r3, -4
	ctx.r[11].s64 = ctx.r[3].s64 + -4;
	// 82FDF7C4: 7D6B2050  subf r11, r11, r4
	ctx.r[11].s64 = ctx.r[4].s64 - ctx.r[11].s64;
	// 82FDF7C8: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82FDF7CC: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82FDF7D0: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82FDF7D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


