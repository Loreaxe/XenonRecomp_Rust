pub fn sub_8304F380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8304F380 size=8
    let mut pc: u32 = 0x8304F380;
    'dispatch: loop {
        match pc {
            0x8304F380 => {
    //   block [0x8304F380..0x8304F388)
	// 8304F380: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304F384: 82163198  lwz r16, 0x3198(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(12696 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304F388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304F388 size=348
    let mut pc: u32 = 0x8304F388;
    'dispatch: loop {
        match pc {
            0x8304F388 => {
    //   block [0x8304F388..0x8304F4E4)
	// 8304F388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304F38C: 48158DD1  bl 0x831a815c
	ctx.lr = 0x8304F390;
	sub_831A8130(ctx, base);
	// 8304F390: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 8304F394: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304F398: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 8304F39C: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 8304F3A0: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8304F3A4: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 8304F3A8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304F3AC: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 8304F3B0: 4BFAA381  bl 0x82ff9730
	ctx.lr = 0x8304F3B4;
	sub_82FF9730(ctx, base);
	// 8304F3B4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304F3B8: 41820124  beq 0x8304f4dc
	if ctx.cr[0].eq {
	pc = 0x8304F4DC; continue 'dispatch;
	}
	// 8304F3BC: 81790000  lwz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304F3C0: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 8304F3C4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8304F3C8: 409A005C  bne cr6, 0x8304f424
	if !ctx.cr[6].eq {
	pc = 0x8304F424; continue 'dispatch;
	}
	// 8304F3CC: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 8304F3D0: 40980008  bge cr6, 0x8304f3d8
	if !ctx.cr[6].lt {
	pc = 0x8304F3D8; continue 'dispatch;
	}
	// 8304F3D4: 3B800010  li r28, 0x10
	ctx.r[28].s64 = 16;
	// 8304F3D8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304F3DC: 4BFA8CD5  bl 0x82ff80b0
	ctx.lr = 0x8304F3E0;
	sub_82FF80B0(ctx, base);
	// 8304F3E0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8304F3E4: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 8304F3E8: 909F0054  stw r4, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[4].u32 ) };
	// 8304F3EC: 4BF88EAD  bl 0x82fd8298
	ctx.lr = 0x8304F3F0;
	sub_82FD8298(ctx, base);
	// 8304F3F0: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8304F3F4: 93DF0058  stw r30, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[30].u32 ) };
	// 8304F3F8: 41820024  beq 0x8304f41c
	if ctx.cr[0].eq {
	pc = 0x8304F41C; continue 'dispatch;
	}
	// 8304F3FC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304F400: 4BFA8CB1  bl 0x82ff80b0
	ctx.lr = 0x8304F404;
	sub_82FF80B0(ctx, base);
	// 8304F404: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 8304F408: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 8304F40C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8304F410: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304F414: 4BFFD21D  bl 0x8304c630
	ctx.lr = 0x8304F418;
	sub_8304C630(ctx, base);
	// 8304F418: 48000008  b 0x8304f420
	pc = 0x8304F420; continue 'dispatch;
	// 8304F41C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8304F420: 90790000  stw r3, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8304F424: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304F428: 80990000  lwz r4, 0(r25)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304F42C: 4BFA9A45  bl 0x82ff8e70
	ctx.lr = 0x8304F430;
	sub_82FF8E70(ctx, base);
	// 8304F430: 389F0050  addi r4, r31, 0x50
	ctx.r[4].s64 = ctx.r[31].s64 + 80;
	// 8304F434: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304F438: 935F0050  stw r26, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[26].u32 ) };
	// 8304F43C: 4BFAA13D  bl 0x82ff9578
	ctx.lr = 0x8304F440;
	sub_82FF9578(ctx, base);
	// 8304F440: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8304F444: 7F5CD378  mr r28, r26
	ctx.r[28].u64 = ctx.r[26].u64;
	// 8304F448: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304F44C: 40990090  ble cr6, 0x8304f4dc
	if !ctx.cr[6].gt {
	pc = 0x8304F4DC; continue 'dispatch;
	}
	// 8304F450: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8304F454: 3B6B3FF0  addi r27, r11, 0x3ff0
	ctx.r[27].s64 = ctx.r[11].s64 + 16368;
	// 8304F458: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304F45C: 4BFA8C55  bl 0x82ff80b0
	ctx.lr = 0x8304F460;
	sub_82FF80B0(ctx, base);
	// 8304F460: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8304F464: 3860002C  li r3, 0x2c
	ctx.r[3].s64 = 44;
	// 8304F468: 909F0058  stw r4, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[4].u32 ) };
	// 8304F46C: 4BF88E2D  bl 0x82fd8298
	ctx.lr = 0x8304F470;
	sub_82FD8298(ctx, base);
	// 8304F470: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8304F474: 93DF0054  stw r30, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 8304F478: 4182002C  beq 0x8304f4a4
	if ctx.cr[0].eq {
	pc = 0x8304F4A4; continue 'dispatch;
	}
	// 8304F47C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304F480: 4BFA8C31  bl 0x82ff80b0
	ctx.lr = 0x8304F484;
	sub_82FF80B0(ctx, base);
	// 8304F484: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8304F488: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304F48C: 48004385  bl 0x83053810
	ctx.lr = 0x8304F490;
	sub_83053810(ctx, base);
	// 8304F490: 9B5E0028  stb r26, 0x28(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), ctx.r[26].u8 ) };
	// 8304F494: 937E0000  stw r27, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 8304F498: 9B5E0029  stb r26, 0x29(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(41 as u32), ctx.r[26].u8 ) };
	// 8304F49C: 9B5E002A  stb r26, 0x2a(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(42 as u32), ctx.r[26].u8 ) };
	// 8304F4A0: 48000008  b 0x8304f4a8
	pc = 0x8304F4A8; continue 'dispatch;
	// 8304F4A4: 7F5ED378  mr r30, r26
	ctx.r[30].u64 = ctx.r[26].u64;
	// 8304F4A8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304F4AC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8304F4B0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304F4B4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8304F4B8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8304F4BC: 4E800421  bctrl
	ctx.lr = 0x8304F4C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8304F4C0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8304F4C4: 80790000  lwz r3, 0(r25)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304F4C8: 4BFFD7C1  bl 0x8304cc88
	ctx.lr = 0x8304F4CC;
	sub_8304CC88(ctx, base);
	// 8304F4CC: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8304F4D0: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 8304F4D4: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8304F4D8: 4198FF80  blt cr6, 0x8304f458
	if ctx.cr[6].lt {
	pc = 0x8304F458; continue 'dispatch;
	}
	// 8304F4DC: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 8304F4E0: 48158CCC  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304F4E4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304F4E4 size=44
    let mut pc: u32 = 0x8304F4E4;
    'dispatch: loop {
        match pc {
            0x8304F4E4 => {
    //   block [0x8304F4E4..0x8304F510)
	// 8304F4E4: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 8304F4E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304F4EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304F4F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304F4F4: 809F0054  lwz r4, 0x54(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8304F4F8: 807F0058  lwz r3, 0x58(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 8304F4FC: 4BF88DE5  bl 0x82fd82e0
	ctx.lr = 0x8304F500;
	sub_82FD82E0(ctx, base);
	// 8304F500: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8304F504: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304F508: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304F50C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304F510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304F510 size=44
    let mut pc: u32 = 0x8304F510;
    'dispatch: loop {
        match pc {
            0x8304F510 => {
    //   block [0x8304F510..0x8304F53C)
	// 8304F510: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 8304F514: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304F518: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304F51C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304F520: 809F0058  lwz r4, 0x58(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 8304F524: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8304F528: 4BF88DB9  bl 0x82fd82e0
	ctx.lr = 0x8304F52C;
	sub_82FD82E0(ctx, base);
	// 8304F52C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8304F530: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304F534: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304F538: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304F540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8304F540 size=8
    let mut pc: u32 = 0x8304F540;
    'dispatch: loop {
        match pc {
            0x8304F540 => {
    //   block [0x8304F540..0x8304F548)
	// 8304F540: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304F544: 821631F8  lwz r16, 0x31f8(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(12792 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304F548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304F548 size=328
    let mut pc: u32 = 0x8304F548;
    'dispatch: loop {
        match pc {
            0x8304F548 => {
    //   block [0x8304F548..0x8304F690)
	// 8304F548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304F54C: 48158C15  bl 0x831a8160
	ctx.lr = 0x8304F550;
	sub_831A8130(ctx, base);
	// 8304F550: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 8304F554: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304F558: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 8304F55C: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 8304F560: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8304F564: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8304F568: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304F56C: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 8304F570: 4BFAA1C1  bl 0x82ff9730
	ctx.lr = 0x8304F574;
	sub_82FF9730(ctx, base);
	// 8304F574: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304F578: 41820110  beq 0x8304f688
	if ctx.cr[0].eq {
	pc = 0x8304F688; continue 'dispatch;
	}
	// 8304F57C: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304F580: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8304F584: 409A005C  bne cr6, 0x8304f5e0
	if !ctx.cr[6].eq {
	pc = 0x8304F5E0; continue 'dispatch;
	}
	// 8304F588: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 8304F58C: 40980008  bge cr6, 0x8304f594
	if !ctx.cr[6].lt {
	pc = 0x8304F594; continue 'dispatch;
	}
	// 8304F590: 3B800010  li r28, 0x10
	ctx.r[28].s64 = 16;
	// 8304F594: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304F598: 4BFA8B19  bl 0x82ff80b0
	ctx.lr = 0x8304F59C;
	sub_82FF80B0(ctx, base);
	// 8304F59C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8304F5A0: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 8304F5A4: 909F0054  stw r4, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[4].u32 ) };
	// 8304F5A8: 4BF88CF1  bl 0x82fd8298
	ctx.lr = 0x8304F5AC;
	sub_82FD8298(ctx, base);
	// 8304F5AC: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8304F5B0: 93BF0058  stw r29, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[29].u32 ) };
	// 8304F5B4: 41820024  beq 0x8304f5d8
	if ctx.cr[0].eq {
	pc = 0x8304F5D8; continue 'dispatch;
	}
	// 8304F5B8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304F5BC: 4BFA8AF5  bl 0x82ff80b0
	ctx.lr = 0x8304F5C0;
	sub_82FF80B0(ctx, base);
	// 8304F5C0: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 8304F5C4: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 8304F5C8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8304F5CC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304F5D0: 4BFFD061  bl 0x8304c630
	ctx.lr = 0x8304F5D4;
	sub_8304C630(ctx, base);
	// 8304F5D4: 48000008  b 0x8304f5dc
	pc = 0x8304F5DC; continue 'dispatch;
	// 8304F5D8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8304F5DC: 907A0000  stw r3, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8304F5E0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304F5E4: 809A0000  lwz r4, 0(r26)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304F5E8: 4BFA9889  bl 0x82ff8e70
	ctx.lr = 0x8304F5EC;
	sub_82FF8E70(ctx, base);
	// 8304F5EC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8304F5F0: 389F0050  addi r4, r31, 0x50
	ctx.r[4].s64 = ctx.r[31].s64 + 80;
	// 8304F5F4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304F5F8: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8304F5FC: 4BFA9F7D  bl 0x82ff9578
	ctx.lr = 0x8304F600;
	sub_82FF9578(ctx, base);
	// 8304F600: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8304F604: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8304F608: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304F60C: 4099007C  ble cr6, 0x8304f688
	if !ctx.cr[6].gt {
	pc = 0x8304F688; continue 'dispatch;
	}
	// 8304F610: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304F614: 4BFA8A9D  bl 0x82ff80b0
	ctx.lr = 0x8304F618;
	sub_82FF80B0(ctx, base);
	// 8304F618: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8304F61C: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 8304F620: 909F0058  stw r4, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[4].u32 ) };
	// 8304F624: 4BF88C75  bl 0x82fd8298
	ctx.lr = 0x8304F628;
	sub_82FD8298(ctx, base);
	// 8304F628: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8304F62C: 93BF0054  stw r29, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 8304F630: 41820020  beq 0x8304f650
	if ctx.cr[0].eq {
	pc = 0x8304F650; continue 'dispatch;
	}
	// 8304F634: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304F638: 4BFA8A79  bl 0x82ff80b0
	ctx.lr = 0x8304F63C;
	sub_82FF80B0(ctx, base);
	// 8304F63C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8304F640: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304F644: 48049FCD  bl 0x83099610
	ctx.lr = 0x8304F648;
	sub_83099610(ctx, base);
	// 8304F648: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8304F64C: 48000008  b 0x8304f654
	pc = 0x8304F654; continue 'dispatch;
	// 8304F650: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8304F654: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304F658: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8304F65C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304F660: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8304F664: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8304F668: 4E800421  bctrl
	ctx.lr = 0x8304F66C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8304F66C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8304F670: 807A0000  lwz r3, 0(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304F674: 4BFFD98D  bl 0x8304d000
	ctx.lr = 0x8304F678;
	sub_8304D000(ctx, base);
	// 8304F678: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8304F67C: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 8304F680: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8304F684: 4198FF8C  blt cr6, 0x8304f610
	if ctx.cr[6].lt {
	pc = 0x8304F610; continue 'dispatch;
	}
	// 8304F688: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 8304F68C: 48158B24  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304F690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304F690 size=44
    let mut pc: u32 = 0x8304F690;
    'dispatch: loop {
        match pc {
            0x8304F690 => {
    //   block [0x8304F690..0x8304F6BC)
	// 8304F690: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 8304F694: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304F698: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304F69C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304F6A0: 809F0054  lwz r4, 0x54(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8304F6A4: 807F0058  lwz r3, 0x58(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 8304F6A8: 4BF88C39  bl 0x82fd82e0
	ctx.lr = 0x8304F6AC;
	sub_82FD82E0(ctx, base);
	// 8304F6AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8304F6B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304F6B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304F6B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304F6BC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304F6BC size=44
    let mut pc: u32 = 0x8304F6BC;
    'dispatch: loop {
        match pc {
            0x8304F6BC => {
    //   block [0x8304F6BC..0x8304F6E8)
	// 8304F6BC: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 8304F6C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304F6C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304F6C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304F6CC: 809F0058  lwz r4, 0x58(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 8304F6D0: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8304F6D4: 4BF88C0D  bl 0x82fd82e0
	ctx.lr = 0x8304F6D8;
	sub_82FD82E0(ctx, base);
	// 8304F6D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8304F6DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304F6E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304F6E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304F6E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304F6E8 size=176
    let mut pc: u32 = 0x8304F6E8;
    'dispatch: loop {
        match pc {
            0x8304F6E8 => {
    //   block [0x8304F6E8..0x8304F798)
	// 8304F6E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304F6EC: 48158A79  bl 0x831a8164
	ctx.lr = 0x8304F6F0;
	sub_831A8130(ctx, base);
	// 8304F6F0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304F6F4: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 8304F6F8: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8304F6FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8304F700: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8304F704: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8304F708: 480194F9  bl 0x83068c00
	ctx.lr = 0x8304F70C;
	sub_83068C00(ctx, base);
	// 8304F70C: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8304F710: 41820034  beq 0x8304f744
	if ctx.cr[0].eq {
	pc = 0x8304F744; continue 'dispatch;
	}
	// 8304F714: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8304F718: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8304F71C: 41820018  beq 0x8304f734
	if ctx.cr[0].eq {
	pc = 0x8304F734; continue 'dispatch;
	}
	// 8304F720: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304F724: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8304F728: 4182000C  beq 0x8304f734
	if ctx.cr[0].eq {
	pc = 0x8304F734; continue 'dispatch;
	}
	// 8304F72C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8304F730: 4804AFE9  bl 0x8309a718
	ctx.lr = 0x8304F734;
	sub_8309A718(ctx, base);
	// 8304F734: 937E0000  stw r27, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 8304F738: 93BE0008  stw r29, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 8304F73C: 939E000C  stw r28, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[28].u32 ) };
	// 8304F740: 48000050  b 0x8304f790
	pc = 0x8304F790; continue 'dispatch;
	// 8304F744: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 8304F748: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304F74C: 4BF88B4D  bl 0x82fd8298
	ctx.lr = 0x8304F750;
	sub_82FD8298(ctx, base);
	// 8304F750: 81210050  lwz r9, 0x50(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8304F754: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8304F758: 41820028  beq 0x8304f780
	if ctx.cr[0].eq {
	pc = 0x8304F780; continue 'dispatch;
	}
	// 8304F75C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8304F760: 5528103A  slwi r8, r9, 2
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8304F764: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8304F768: 7D68582E  lwzx r11, r8, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8304F76C: 93630000  stw r27, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 8304F770: 93A30008  stw r29, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 8304F774: 9383000C  stw r28, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[28].u32 ) };
	// 8304F778: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8304F77C: 48000008  b 0x8304f784
	pc = 0x8304F784; continue 'dispatch;
	// 8304F780: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8304F784: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8304F788: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8304F78C: 7D49592E  stwx r10, r9, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u32) };
	// 8304F790: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8304F794: 48158A20  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304F798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304F798 size=148
    let mut pc: u32 = 0x8304F798;
    'dispatch: loop {
        match pc {
            0x8304F798 => {
    //   block [0x8304F798..0x8304F82C)
	// 8304F798: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304F79C: 481589C9  bl 0x831a8164
	ctx.lr = 0x8304F7A0;
	sub_831A8130(ctx, base);
	// 8304F7A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304F7A4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8304F7A8: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8304F7AC: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 8304F7B0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8304F7B4: 40990070  ble cr6, 0x8304f824
	if !ctx.cr[6].gt {
	pc = 0x8304F824; continue 'dispatch;
	}
	// 8304F7B8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8304F7BC: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8304F7C0: 7FEBE82E  lwzx r31, r11, r29
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 8304F7C4: 281F0000  cmplwi r31, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8304F7C8: 4182003C  beq 0x8304f804
	if ctx.cr[0].eq {
	pc = 0x8304F804; continue 'dispatch;
	}
	// 8304F7CC: 897E0004  lbz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8304F7D0: 839F0004  lwz r28, 4(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8304F7D4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8304F7D8: 41820018  beq 0x8304f7f0
	if ctx.cr[0].eq {
	pc = 0x8304F7F0; continue 'dispatch;
	}
	// 8304F7DC: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304F7E0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8304F7E4: 4182000C  beq 0x8304f7f0
	if ctx.cr[0].eq {
	pc = 0x8304F7F0; continue 'dispatch;
	}
	// 8304F7E8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8304F7EC: 4804AF2D  bl 0x8309a718
	ctx.lr = 0x8304F7F0;
	sub_8309A718(ctx, base);
	// 8304F7F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8304F7F4: 4BF88AED  bl 0x82fd82e0
	ctx.lr = 0x8304F7F8;
	sub_82FD82E0(ctx, base);
	// 8304F7F8: 7F9FE378  mr r31, r28
	ctx.r[31].u64 = ctx.r[28].u64;
	// 8304F7FC: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 8304F800: 409AFFCC  bne cr6, 0x8304f7cc
	if !ctx.cr[6].eq {
	pc = 0x8304F7CC; continue 'dispatch;
	}
	// 8304F804: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8304F808: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8304F80C: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 8304F810: 7D4BE92E  stwx r10, r11, r29
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32), ctx.r[10].u32) };
	// 8304F814: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 8304F818: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 8304F81C: 7F1B5840  cmplw cr6, r27, r11
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8304F820: 4198FF9C  blt cr6, 0x8304f7bc
	if ctx.cr[6].lt {
	pc = 0x8304F7BC; continue 'dispatch;
	}
	// 8304F824: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8304F828: 4815898C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304F830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8304F830 size=8
    let mut pc: u32 = 0x8304F830;
    'dispatch: loop {
        match pc {
            0x8304F830 => {
    //   block [0x8304F830..0x8304F838)
	// 8304F830: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304F834: 82163250  lwz r16, 0x3250(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(12880 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304F838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304F838 size=296
    let mut pc: u32 = 0x8304F838;
    'dispatch: loop {
        match pc {
            0x8304F838 => {
    //   block [0x8304F838..0x8304F960)
	// 8304F838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304F83C: 4815892D  bl 0x831a8168
	ctx.lr = 0x8304F840;
	sub_831A8130(ctx, base);
	// 8304F840: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 8304F844: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304F848: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8304F84C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8304F850: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8304F854: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8304F858: 4BFA9E41  bl 0x82ff9698
	ctx.lr = 0x8304F85C;
	sub_82FF9698(ctx, base);
	// 8304F85C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304F860: 418200F8  beq 0x8304f958
	if ctx.cr[0].eq {
	pc = 0x8304F958; continue 'dispatch;
	}
	// 8304F864: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8304F868: 809D000C  lwz r4, 0xc(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 8304F86C: 4BFA9A8D  bl 0x82ff92f8
	ctx.lr = 0x8304F870;
	sub_82FF92F8(ctx, base);
	// 8304F870: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8304F874: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8304F878: 80DD0000  lwz r6, 0(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304F87C: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 8304F880: 4BF972C1  bl 0x82fe6b40
	ctx.lr = 0x8304F884;
	sub_82FE6B40(ctx, base);
	// 8304F884: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8304F888: 817F0068  lwz r11, 0x68(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 8304F88C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8304F890: 409A001C  bne cr6, 0x8304f8ac
	if !ctx.cr[6].eq {
	pc = 0x8304F8AC; continue 'dispatch;
	}
	// 8304F894: 817F0070  lwz r11, 0x70(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 8304F898: 815F006C  lwz r10, 0x6c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 8304F89C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8304F8A0: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8304F8A4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8304F8A8: 419A0008  beq cr6, 0x8304f8b0
	if ctx.cr[6].eq {
	pc = 0x8304F8B0; continue 'dispatch;
	}
	// 8304F8AC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8304F8B0: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304F8B4: 41820014  beq 0x8304f8c8
	if ctx.cr[0].eq {
	pc = 0x8304F8C8; continue 'dispatch;
	}
	// 8304F8B8: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 8304F8BC: 4BFBE0BD  bl 0x8300d978
	ctx.lr = 0x8304F8C0;
	sub_8300D978(ctx, base);
	// 8304F8C0: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 8304F8C4: 4BFFFFC4  b 0x8304f888
	pc = 0x8304F888; continue 'dispatch;
	// 8304F8C8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8304F8CC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8304F8D0: 4BFA9A29  bl 0x82ff92f8
	ctx.lr = 0x8304F8D4;
	sub_82FF92F8(ctx, base);
	// 8304F8D4: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8304F8D8: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 8304F8DC: 917F006C  stw r11, 0x6c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8304F8E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8304F8E4: 917F0068  stw r11, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8304F8E8: 4BFAAE01  bl 0x82ffa6e8
	ctx.lr = 0x8304F8EC;
	sub_82FFA6E8(ctx, base);
	// 8304F8EC: 817F0068  lwz r11, 0x68(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 8304F8F0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8304F8F4: 409A001C  bne cr6, 0x8304f910
	if !ctx.cr[6].eq {
	pc = 0x8304F910; continue 'dispatch;
	}
	// 8304F8F8: 817F0070  lwz r11, 0x70(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 8304F8FC: 815F006C  lwz r10, 0x6c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 8304F900: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8304F904: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8304F908: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8304F90C: 419A0008  beq cr6, 0x8304f914
	if ctx.cr[6].eq {
	pc = 0x8304F914; continue 'dispatch;
	}
	// 8304F910: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8304F914: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304F918: 41820038  beq 0x8304f950
	if ctx.cr[0].eq {
	pc = 0x8304F950; continue 'dispatch;
	}
	// 8304F91C: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 8304F920: 4BFFD841  bl 0x8304d160
	ctx.lr = 0x8304F924;
	sub_8304D160(ctx, base);
	// 8304F924: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8304F928: 38BF0050  addi r5, r31, 0x50
	ctx.r[5].s64 = ctx.r[31].s64 + 80;
	// 8304F92C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304F930: 4BFAAB31  bl 0x82ffa460
	ctx.lr = 0x8304F934;
	sub_82FFA460(ctx, base);
	// 8304F934: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8304F938: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8304F93C: 41820008  beq 0x8304f944
	if ctx.cr[0].eq {
	pc = 0x8304F944; continue 'dispatch;
	}
	// 8304F940: 80830000  lwz r4, 0(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304F944: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8304F948: 4BFF1921  bl 0x83041268
	ctx.lr = 0x8304F94C;
	sub_83041268(ctx, base);
	// 8304F94C: 4BFFFFA0  b 0x8304f8ec
	pc = 0x8304F8EC; continue 'dispatch;
	// 8304F950: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 8304F954: 4BFBE875  bl 0x8300e1c8
	ctx.lr = 0x8304F958;
	sub_8300E1C8(ctx, base);
	// 8304F958: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 8304F95C: 4815885C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304F960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304F960 size=40
    let mut pc: u32 = 0x8304F960;
    'dispatch: loop {
        match pc {
            0x8304F960 => {
    //   block [0x8304F960..0x8304F988)
	// 8304F960: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 8304F964: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304F968: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304F96C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304F970: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 8304F974: 4BFBE855  bl 0x8300e1c8
	ctx.lr = 0x8304F978;
	sub_8300E1C8(ctx, base);
	// 8304F978: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8304F97C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304F980: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304F984: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304F988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8304F988 size=8
    let mut pc: u32 = 0x8304F988;
    'dispatch: loop {
        match pc {
            0x8304F988 => {
    //   block [0x8304F988..0x8304F990)
	// 8304F988: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304F98C: 82163298  lwz r16, 0x3298(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(12952 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304F990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304F990 size=432
    let mut pc: u32 = 0x8304F990;
    'dispatch: loop {
        match pc {
            0x8304F990 => {
    //   block [0x8304F990..0x8304FB40)
	// 8304F990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304F994: 481587C1  bl 0x831a8154
	ctx.lr = 0x8304F998;
	sub_831A8130(ctx, base);
	// 8304F998: 3BE1FF40  addi r31, r1, -0xc0
	ctx.r[31].s64 = ctx.r[1].s64 + -192;
	// 8304F99C: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304F9A0: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 8304F9A4: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 8304F9A8: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 8304F9AC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304F9B0: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8304F9B4: 4BFA9D7D  bl 0x82ff9730
	ctx.lr = 0x8304F9B8;
	sub_82FF9730(ctx, base);
	// 8304F9B8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304F9BC: 4182017C  beq 0x8304fb38
	if ctx.cr[0].eq {
	pc = 0x8304FB38; continue 'dispatch;
	}
	// 8304F9C0: 389F0060  addi r4, r31, 0x60
	ctx.r[4].s64 = ctx.r[31].s64 + 96;
	// 8304F9C4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304F9C8: 4BFA9BB1  bl 0x82ff9578
	ctx.lr = 0x8304F9CC;
	sub_82FF9578(ctx, base);
	// 8304F9CC: 81780000  lwz r11, 0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304F9D0: 3AE00000  li r23, 0
	ctx.r[23].s64 = 0;
	// 8304F9D4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8304F9D8: 409A0050  bne cr6, 0x8304fa28
	if !ctx.cr[6].eq {
	pc = 0x8304FA28; continue 'dispatch;
	}
	// 8304F9DC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304F9E0: 4BFA86D1  bl 0x82ff80b0
	ctx.lr = 0x8304F9E4;
	sub_82FF80B0(ctx, base);
	// 8304F9E4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8304F9E8: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 8304F9EC: 909F005C  stw r4, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[4].u32 ) };
	// 8304F9F0: 4BF888A9  bl 0x82fd8298
	ctx.lr = 0x8304F9F4;
	sub_82FD8298(ctx, base);
	// 8304F9F4: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8304F9F8: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 8304F9FC: 41820024  beq 0x8304fa20
	if ctx.cr[0].eq {
	pc = 0x8304FA20; continue 'dispatch;
	}
	// 8304FA00: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304FA04: 4BFA86AD  bl 0x82ff80b0
	ctx.lr = 0x8304FA08;
	sub_82FF80B0(ctx, base);
	// 8304FA08: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 8304FA0C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8304FA10: 809F0060  lwz r4, 0x60(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 8304FA14: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304FA18: 4BFEB799  bl 0x8303b1b0
	ctx.lr = 0x8304FA1C;
	sub_8303B1B0(ctx, base);
	// 8304FA1C: 48000008  b 0x8304fa24
	pc = 0x8304FA24; continue 'dispatch;
	// 8304FA20: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 8304FA24: 90780000  stw r3, 0(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8304FA28: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304FA2C: 80980000  lwz r4, 0(r24)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304FA30: 4BFA9441  bl 0x82ff8e70
	ctx.lr = 0x8304FA34;
	sub_82FF8E70(ctx, base);
	// 8304FA34: 389F0058  addi r4, r31, 0x58
	ctx.r[4].s64 = ctx.r[31].s64 + 88;
	// 8304FA38: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304FA3C: 92FF0058  stw r23, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[23].u32 ) };
	// 8304FA40: 4BFA9B39  bl 0x82ff9578
	ctx.lr = 0x8304FA44;
	sub_82FF9578(ctx, base);
	// 8304FA44: 817F0058  lwz r11, 0x58(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 8304FA48: 7EF9BB78  mr r25, r23
	ctx.r[25].u64 = ctx.r[23].u64;
	// 8304FA4C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304FA50: 409900E8  ble cr6, 0x8304fb38
	if !ctx.cr[6].gt {
	pc = 0x8304FB38; continue 'dispatch;
	}
	// 8304FA54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8304FA58: 38DF0064  addi r6, r31, 0x64
	ctx.r[6].s64 = ctx.r[31].s64 + 100;
	// 8304FA5C: 38BF0068  addi r5, r31, 0x68
	ctx.r[5].s64 = ctx.r[31].s64 + 104;
	// 8304FA60: 389F0054  addi r4, r31, 0x54
	ctx.r[4].s64 = ctx.r[31].s64 + 84;
	// 8304FA64: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304FA68: 4BFAA0C1  bl 0x82ff9b28
	ctx.lr = 0x8304FA6C;
	sub_82FF9B28(ctx, base);
	// 8304FA6C: 389F005C  addi r4, r31, 0x5c
	ctx.r[4].s64 = ctx.r[31].s64 + 92;
	// 8304FA70: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304FA74: 4BFA9B05  bl 0x82ff9578
	ctx.lr = 0x8304FA78;
	sub_82FF9578(ctx, base);
	// 8304FA78: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 8304FA7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8304FA80: 92FF0050  stw r23, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[23].u32 ) };
	// 8304FA84: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 8304FA88: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 8304FA8C: 4BFFD885  bl 0x8304d310
	ctx.lr = 0x8304FA90;
	sub_8304D310(ctx, base);
	// 8304FA90: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8304FA94: 7EFBBB78  mr r27, r23
	ctx.r[27].u64 = ctx.r[23].u64;
	// 8304FA98: 834B0004  lwz r26, 4(r11)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8304FA9C: 2C1A0000  cmpwi r26, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8304FAA0: 40810074  ble 0x8304fb14
	if !ctx.cr[0].gt {
	pc = 0x8304FB14; continue 'dispatch;
	}
	// 8304FAA4: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8304FAA8: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8304FAAC: 4BFA8625  bl 0x82ff80d0
	ctx.lr = 0x8304FAB0;
	sub_82FF80D0(ctx, base);
	// 8304FAB0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304FAB4: 809F0054  lwz r4, 0x54(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8304FAB8: 816B0050  lwz r11, 0x50(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 8304FABC: 838B0008  lwz r28, 8(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8304FAC0: 83DC0010  lwz r30, 0x10(r28)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(16 as u32) ) } as u64;
	// 8304FAC4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304FAC8: 4BF84179  bl 0x82fd3c40
	ctx.lr = 0x8304FACC;
	sub_82FD3C40(ctx, base);
	// 8304FACC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304FAD0: 41820014  beq 0x8304fae4
	if ctx.cr[0].eq {
	pc = 0x8304FAE4; continue 'dispatch;
	}
	// 8304FAD4: 817C0020  lwz r11, 0x20(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(32 as u32) ) } as u64;
	// 8304FAD8: 815F005C  lwz r10, 0x5c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 8304FADC: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8304FAE0: 419A0014  beq cr6, 0x8304faf4
	if ctx.cr[6].eq {
	pc = 0x8304FAF4; continue 'dispatch;
	}
	// 8304FAE4: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 8304FAE8: 7F1BD000  cmpw cr6, r27, r26
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[26].s32, &mut ctx.xer);
	// 8304FAEC: 4198FFB8  blt cr6, 0x8304faa4
	if ctx.cr[6].lt {
	pc = 0x8304FAA4; continue 'dispatch;
	}
	// 8304FAF0: 48000024  b 0x8304fb14
	pc = 0x8304FB14; continue 'dispatch;
	// 8304FAF4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304FAF8: 4BFA85B9  bl 0x82ff80b0
	ctx.lr = 0x8304FAFC;
	sub_82FF80B0(ctx, base);
	// 8304FAFC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304FB00: 809F0054  lwz r4, 0x54(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8304FB04: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8304FB08: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8304FB0C: 4E800421  bctrl
	ctx.lr = 0x8304FB10;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8304FB10: 93DF0054  stw r30, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 8304FB14: 80DF0050  lwz r6, 0x50(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8304FB18: 80780000  lwz r3, 0(r24)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304FB1C: 80BF005C  lwz r5, 0x5c(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 8304FB20: 809F0054  lwz r4, 0x54(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8304FB24: 4BFFFBC5  bl 0x8304f6e8
	ctx.lr = 0x8304FB28;
	sub_8304F6E8(ctx, base);
	// 8304FB28: 817F0058  lwz r11, 0x58(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 8304FB2C: 3B390001  addi r25, r25, 1
	ctx.r[25].s64 = ctx.r[25].s64 + 1;
	// 8304FB30: 7F195800  cmpw cr6, r25, r11
	ctx.cr[6].compare_i32(ctx.r[25].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8304FB34: 4198FF20  blt cr6, 0x8304fa54
	if ctx.cr[6].lt {
	pc = 0x8304FA54; continue 'dispatch;
	}
	// 8304FB38: 383F00C0  addi r1, r31, 0xc0
	ctx.r[1].s64 = ctx.r[31].s64 + 192;
	// 8304FB3C: 48158668  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304FB40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304FB40 size=44
    let mut pc: u32 = 0x8304FB40;
    'dispatch: loop {
        match pc {
            0x8304FB40 => {
    //   block [0x8304FB40..0x8304FB6C)
	// 8304FB40: 3BECFF40  addi r31, r12, -0xc0
	ctx.r[31].s64 = ctx.r[12].s64 + -192;
	// 8304FB44: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304FB48: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304FB4C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304FB50: 809F005C  lwz r4, 0x5c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 8304FB54: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8304FB58: 4BF88789  bl 0x82fd82e0
	ctx.lr = 0x8304FB5C;
	sub_82FD82E0(ctx, base);
	// 8304FB5C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8304FB60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304FB64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304FB68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304FB70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304FB70 size=100
    let mut pc: u32 = 0x8304FB70;
    'dispatch: loop {
        match pc {
            0x8304FB70 => {
    //   block [0x8304FB70..0x8304FBD4)
	// 8304FB70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304FB74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304FB78: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8304FB7C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304FB80: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8304FB84: 4BFFFC15  bl 0x8304f798
	ctx.lr = 0x8304FB88;
	sub_8304F798(ctx, base);
	// 8304FB88: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304FB8C: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8304FB90: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304FB94: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8304FB98: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8304FB9C: 4E800421  bctrl
	ctx.lr = 0x8304FBA0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8304FBA0: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8304FBA4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8304FBA8: 41820018  beq 0x8304fbc0
	if ctx.cr[0].eq {
	pc = 0x8304FBC0; continue 'dispatch;
	}
	// 8304FBAC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304FBB0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8304FBB4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8304FBB8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8304FBBC: 4E800421  bctrl
	ctx.lr = 0x8304FBC0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8304FBC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8304FBC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304FBC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304FBCC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8304FBD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304FBD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8304FBD8 size=8
    let mut pc: u32 = 0x8304FBD8;
    'dispatch: loop {
        match pc {
            0x8304FBD8 => {
    //   block [0x8304FBD8..0x8304FBE0)
	// 8304FBD8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304FBDC: 821632E0  lwz r16, 0x32e0(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(13024 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304FBE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304FBE0 size=296
    let mut pc: u32 = 0x8304FBE0;
    'dispatch: loop {
        match pc {
            0x8304FBE0 => {
    //   block [0x8304FBE0..0x8304FD08)
	// 8304FBE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304FBE4: 48158585  bl 0x831a8168
	ctx.lr = 0x8304FBE8;
	sub_831A8130(ctx, base);
	// 8304FBE8: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 8304FBEC: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304FBF0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8304FBF4: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8304FBF8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8304FBFC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8304FC00: 4BFA9A99  bl 0x82ff9698
	ctx.lr = 0x8304FC04;
	sub_82FF9698(ctx, base);
	// 8304FC04: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304FC08: 418200F8  beq 0x8304fd00
	if ctx.cr[0].eq {
	pc = 0x8304FD00; continue 'dispatch;
	}
	// 8304FC0C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8304FC10: 809D000C  lwz r4, 0xc(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 8304FC14: 4BFA96E5  bl 0x82ff92f8
	ctx.lr = 0x8304FC18;
	sub_82FF92F8(ctx, base);
	// 8304FC18: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8304FC1C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8304FC20: 80DD0000  lwz r6, 0(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304FC24: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 8304FC28: 4BF96F19  bl 0x82fe6b40
	ctx.lr = 0x8304FC2C;
	sub_82FE6B40(ctx, base);
	// 8304FC2C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8304FC30: 817F0068  lwz r11, 0x68(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 8304FC34: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8304FC38: 409A001C  bne cr6, 0x8304fc54
	if !ctx.cr[6].eq {
	pc = 0x8304FC54; continue 'dispatch;
	}
	// 8304FC3C: 817F0070  lwz r11, 0x70(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 8304FC40: 815F006C  lwz r10, 0x6c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 8304FC44: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8304FC48: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8304FC4C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8304FC50: 419A0008  beq cr6, 0x8304fc58
	if ctx.cr[6].eq {
	pc = 0x8304FC58; continue 'dispatch;
	}
	// 8304FC54: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8304FC58: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304FC5C: 41820014  beq 0x8304fc70
	if ctx.cr[0].eq {
	pc = 0x8304FC70; continue 'dispatch;
	}
	// 8304FC60: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 8304FC64: 4BFBDD15  bl 0x8300d978
	ctx.lr = 0x8304FC68;
	sub_8300D978(ctx, base);
	// 8304FC68: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 8304FC6C: 4BFFFFC4  b 0x8304fc30
	pc = 0x8304FC30; continue 'dispatch;
	// 8304FC70: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8304FC74: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8304FC78: 4BFA9681  bl 0x82ff92f8
	ctx.lr = 0x8304FC7C;
	sub_82FF92F8(ctx, base);
	// 8304FC7C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8304FC80: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 8304FC84: 917F006C  stw r11, 0x6c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8304FC88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8304FC8C: 917F0068  stw r11, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8304FC90: 4BFAAA59  bl 0x82ffa6e8
	ctx.lr = 0x8304FC94;
	sub_82FFA6E8(ctx, base);
	// 8304FC94: 817F0068  lwz r11, 0x68(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 8304FC98: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8304FC9C: 409A001C  bne cr6, 0x8304fcb8
	if !ctx.cr[6].eq {
	pc = 0x8304FCB8; continue 'dispatch;
	}
	// 8304FCA0: 817F0070  lwz r11, 0x70(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 8304FCA4: 815F006C  lwz r10, 0x6c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 8304FCA8: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8304FCAC: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8304FCB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8304FCB4: 419A0008  beq cr6, 0x8304fcbc
	if ctx.cr[6].eq {
	pc = 0x8304FCBC; continue 'dispatch;
	}
	// 8304FCB8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8304FCBC: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304FCC0: 41820038  beq 0x8304fcf8
	if ctx.cr[0].eq {
	pc = 0x8304FCF8; continue 'dispatch;
	}
	// 8304FCC4: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 8304FCC8: 4BFFD499  bl 0x8304d160
	ctx.lr = 0x8304FCCC;
	sub_8304D160(ctx, base);
	// 8304FCCC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8304FCD0: 38BF0050  addi r5, r31, 0x50
	ctx.r[5].s64 = ctx.r[31].s64 + 80;
	// 8304FCD4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304FCD8: 4BFAA789  bl 0x82ffa460
	ctx.lr = 0x8304FCDC;
	sub_82FFA460(ctx, base);
	// 8304FCDC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8304FCE0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8304FCE4: 41820008  beq 0x8304fcec
	if ctx.cr[0].eq {
	pc = 0x8304FCEC; continue 'dispatch;
	}
	// 8304FCE8: 80830000  lwz r4, 0(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304FCEC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8304FCF0: 4BFA9F11  bl 0x82ff9c00
	ctx.lr = 0x8304FCF4;
	sub_82FF9C00(ctx, base);
	// 8304FCF4: 4BFFFFA0  b 0x8304fc94
	pc = 0x8304FC94; continue 'dispatch;
	// 8304FCF8: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 8304FCFC: 4BFBE4CD  bl 0x8300e1c8
	ctx.lr = 0x8304FD00;
	sub_8300E1C8(ctx, base);
	// 8304FD00: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 8304FD04: 481584B4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304FD08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304FD08 size=40
    let mut pc: u32 = 0x8304FD08;
    'dispatch: loop {
        match pc {
            0x8304FD08 => {
    //   block [0x8304FD08..0x8304FD30)
	// 8304FD08: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 8304FD0C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304FD10: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304FD14: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304FD18: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 8304FD1C: 4BFBE4AD  bl 0x8300e1c8
	ctx.lr = 0x8304FD20;
	sub_8300E1C8(ctx, base);
	// 8304FD20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8304FD24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304FD28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304FD2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304FD30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8304FD30 size=8
    let mut pc: u32 = 0x8304FD30;
    'dispatch: loop {
        match pc {
            0x8304FD30 => {
    //   block [0x8304FD30..0x8304FD38)
	// 8304FD30: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304FD34: 82163328  lwz r16, 0x3328(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(13096 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304FD38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304FD38 size=340
    let mut pc: u32 = 0x8304FD38;
    'dispatch: loop {
        match pc {
            0x8304FD38 => {
    //   block [0x8304FD38..0x8304FE8C)
	// 8304FD38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304FD3C: 4815842D  bl 0x831a8168
	ctx.lr = 0x8304FD40;
	sub_831A8130(ctx, base);
	// 8304FD40: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 8304FD44: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304FD48: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8304FD4C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8304FD50: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8304FD54: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304FD58: 4BFA9941  bl 0x82ff9698
	ctx.lr = 0x8304FD5C;
	sub_82FF9698(ctx, base);
	// 8304FD5C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304FD60: 41820124  beq 0x8304fe84
	if ctx.cr[0].eq {
	pc = 0x8304FE84; continue 'dispatch;
	}
	// 8304FD64: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304FD68: 809C000C  lwz r4, 0xc(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(12 as u32) ) } as u64;
	// 8304FD6C: 4BFA958D  bl 0x82ff92f8
	ctx.lr = 0x8304FD70;
	sub_82FF92F8(ctx, base);
	// 8304FD70: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8304FD74: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8304FD78: 80DC0000  lwz r6, 0(r28)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304FD7C: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 8304FD80: 4BF96DC1  bl 0x82fe6b40
	ctx.lr = 0x8304FD84;
	sub_82FE6B40(ctx, base);
	// 8304FD84: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8304FD88: 817F0068  lwz r11, 0x68(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 8304FD8C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8304FD90: 409A001C  bne cr6, 0x8304fdac
	if !ctx.cr[6].eq {
	pc = 0x8304FDAC; continue 'dispatch;
	}
	// 8304FD94: 817F0070  lwz r11, 0x70(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 8304FD98: 815F006C  lwz r10, 0x6c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 8304FD9C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8304FDA0: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8304FDA4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8304FDA8: 419A0008  beq cr6, 0x8304fdb0
	if ctx.cr[6].eq {
	pc = 0x8304FDB0; continue 'dispatch;
	}
	// 8304FDAC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8304FDB0: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304FDB4: 41820014  beq 0x8304fdc8
	if ctx.cr[0].eq {
	pc = 0x8304FDC8; continue 'dispatch;
	}
	// 8304FDB8: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 8304FDBC: 4BFBDBBD  bl 0x8300d978
	ctx.lr = 0x8304FDC0;
	sub_8300D978(ctx, base);
	// 8304FDC0: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 8304FDC4: 4BFFFFC4  b 0x8304fd88
	pc = 0x8304FD88; continue 'dispatch;
	// 8304FDC8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8304FDCC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304FDD0: 4BFA9529  bl 0x82ff92f8
	ctx.lr = 0x8304FDD4;
	sub_82FF92F8(ctx, base);
	// 8304FDD4: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8304FDD8: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 8304FDDC: 917F006C  stw r11, 0x6c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8304FDE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8304FDE4: 917F0068  stw r11, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8304FDE8: 4BFAA901  bl 0x82ffa6e8
	ctx.lr = 0x8304FDEC;
	sub_82FFA6E8(ctx, base);
	// 8304FDEC: 817F0068  lwz r11, 0x68(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 8304FDF0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8304FDF4: 409A001C  bne cr6, 0x8304fe10
	if !ctx.cr[6].eq {
	pc = 0x8304FE10; continue 'dispatch;
	}
	// 8304FDF8: 817F0070  lwz r11, 0x70(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 8304FDFC: 815F006C  lwz r10, 0x6c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 8304FE00: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8304FE04: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8304FE08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8304FE0C: 419A0008  beq cr6, 0x8304fe14
	if ctx.cr[6].eq {
	pc = 0x8304FE14; continue 'dispatch;
	}
	// 8304FE10: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8304FE14: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304FE18: 41820064  beq 0x8304fe7c
	if ctx.cr[0].eq {
	pc = 0x8304FE7C; continue 'dispatch;
	}
	// 8304FE1C: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 8304FE20: 4BFFD341  bl 0x8304d160
	ctx.lr = 0x8304FE24;
	sub_8304D160(ctx, base);
	// 8304FE24: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8304FE28: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304FE2C: 4BFA826D  bl 0x82ff8098
	ctx.lr = 0x8304FE30;
	sub_82FF8098(ctx, base);
	// 8304FE30: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304FE34: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8304FE38: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8304FE3C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8304FE40: 4E800421  bctrl
	ctx.lr = 0x8304FE44;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8304FE44: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8304FE48: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304FE4C: 4BFA94AD  bl 0x82ff92f8
	ctx.lr = 0x8304FE50;
	sub_82FF92F8(ctx, base);
	// 8304FE50: 38BF0050  addi r5, r31, 0x50
	ctx.r[5].s64 = ctx.r[31].s64 + 80;
	// 8304FE54: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8304FE58: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8304FE5C: 4BFAA605  bl 0x82ffa460
	ctx.lr = 0x8304FE60;
	sub_82FFA460(ctx, base);
	// 8304FE60: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8304FE64: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8304FE68: 41820008  beq 0x8304fe70
	if ctx.cr[0].eq {
	pc = 0x8304FE70; continue 'dispatch;
	}
	// 8304FE6C: 80830000  lwz r4, 0(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304FE70: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304FE74: 4BFA9D8D  bl 0x82ff9c00
	ctx.lr = 0x8304FE78;
	sub_82FF9C00(ctx, base);
	// 8304FE78: 4BFFFF74  b 0x8304fdec
	pc = 0x8304FDEC; continue 'dispatch;
	// 8304FE7C: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 8304FE80: 4BFBE349  bl 0x8300e1c8
	ctx.lr = 0x8304FE84;
	sub_8300E1C8(ctx, base);
	// 8304FE84: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 8304FE88: 48158330  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304FE8C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304FE8C size=40
    let mut pc: u32 = 0x8304FE8C;
    'dispatch: loop {
        match pc {
            0x8304FE8C => {
    //   block [0x8304FE8C..0x8304FEB4)
	// 8304FE8C: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 8304FE90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304FE94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304FE98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304FE9C: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 8304FEA0: 4BFBE329  bl 0x8300e1c8
	ctx.lr = 0x8304FEA4;
	sub_8300E1C8(ctx, base);
	// 8304FEA4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8304FEA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304FEAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304FEB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304FEB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8304FEB8 size=8
    let mut pc: u32 = 0x8304FEB8;
    'dispatch: loop {
        match pc {
            0x8304FEB8 => {
    //   block [0x8304FEB8..0x8304FEC0)
	// 8304FEB8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304FEBC: 82163370  lwz r16, 0x3370(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(13168 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304FEC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304FEC0 size=296
    let mut pc: u32 = 0x8304FEC0;
    'dispatch: loop {
        match pc {
            0x8304FEC0 => {
    //   block [0x8304FEC0..0x8304FFE8)
	// 8304FEC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304FEC4: 481582A5  bl 0x831a8168
	ctx.lr = 0x8304FEC8;
	sub_831A8130(ctx, base);
	// 8304FEC8: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 8304FECC: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304FED0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8304FED4: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8304FED8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8304FEDC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8304FEE0: 4BFA97B9  bl 0x82ff9698
	ctx.lr = 0x8304FEE4;
	sub_82FF9698(ctx, base);
	// 8304FEE4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304FEE8: 418200F8  beq 0x8304ffe0
	if ctx.cr[0].eq {
	pc = 0x8304FFE0; continue 'dispatch;
	}
	// 8304FEEC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8304FEF0: 809D000C  lwz r4, 0xc(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 8304FEF4: 4BFA9405  bl 0x82ff92f8
	ctx.lr = 0x8304FEF8;
	sub_82FF92F8(ctx, base);
	// 8304FEF8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8304FEFC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8304FF00: 80DD0000  lwz r6, 0(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304FF04: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 8304FF08: 4BF96C39  bl 0x82fe6b40
	ctx.lr = 0x8304FF0C;
	sub_82FE6B40(ctx, base);
	// 8304FF0C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8304FF10: 817F0068  lwz r11, 0x68(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 8304FF14: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8304FF18: 409A001C  bne cr6, 0x8304ff34
	if !ctx.cr[6].eq {
	pc = 0x8304FF34; continue 'dispatch;
	}
	// 8304FF1C: 817F0070  lwz r11, 0x70(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 8304FF20: 815F006C  lwz r10, 0x6c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 8304FF24: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8304FF28: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8304FF2C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8304FF30: 419A0008  beq cr6, 0x8304ff38
	if ctx.cr[6].eq {
	pc = 0x8304FF38; continue 'dispatch;
	}
	// 8304FF34: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8304FF38: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304FF3C: 41820014  beq 0x8304ff50
	if ctx.cr[0].eq {
	pc = 0x8304FF50; continue 'dispatch;
	}
	// 8304FF40: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 8304FF44: 4BFBDA35  bl 0x8300d978
	ctx.lr = 0x8304FF48;
	sub_8300D978(ctx, base);
	// 8304FF48: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 8304FF4C: 4BFFFFC4  b 0x8304ff10
	pc = 0x8304FF10; continue 'dispatch;
	// 8304FF50: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8304FF54: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8304FF58: 4BFA93A1  bl 0x82ff92f8
	ctx.lr = 0x8304FF5C;
	sub_82FF92F8(ctx, base);
	// 8304FF5C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8304FF60: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 8304FF64: 917F006C  stw r11, 0x6c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8304FF68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8304FF6C: 917F0068  stw r11, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8304FF70: 4BFAA779  bl 0x82ffa6e8
	ctx.lr = 0x8304FF74;
	sub_82FFA6E8(ctx, base);
	// 8304FF74: 817F0068  lwz r11, 0x68(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 8304FF78: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8304FF7C: 409A001C  bne cr6, 0x8304ff98
	if !ctx.cr[6].eq {
	pc = 0x8304FF98; continue 'dispatch;
	}
	// 8304FF80: 817F0070  lwz r11, 0x70(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 8304FF84: 815F006C  lwz r10, 0x6c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 8304FF88: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8304FF8C: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8304FF90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8304FF94: 419A0008  beq cr6, 0x8304ff9c
	if ctx.cr[6].eq {
	pc = 0x8304FF9C; continue 'dispatch;
	}
	// 8304FF98: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8304FF9C: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304FFA0: 41820038  beq 0x8304ffd8
	if ctx.cr[0].eq {
	pc = 0x8304FFD8; continue 'dispatch;
	}
	// 8304FFA4: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 8304FFA8: 4BFFD1B9  bl 0x8304d160
	ctx.lr = 0x8304FFAC;
	sub_8304D160(ctx, base);
	// 8304FFAC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8304FFB0: 38BF0050  addi r5, r31, 0x50
	ctx.r[5].s64 = ctx.r[31].s64 + 80;
	// 8304FFB4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304FFB8: 4BFAA4A9  bl 0x82ffa460
	ctx.lr = 0x8304FFBC;
	sub_82FFA460(ctx, base);
	// 8304FFBC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8304FFC0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8304FFC4: 41820008  beq 0x8304ffcc
	if ctx.cr[0].eq {
	pc = 0x8304FFCC; continue 'dispatch;
	}
	// 8304FFC8: 80830000  lwz r4, 0(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304FFCC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8304FFD0: 48049561  bl 0x83099530
	ctx.lr = 0x8304FFD4;
	sub_83099530(ctx, base);
	// 8304FFD4: 4BFFFFA0  b 0x8304ff74
	pc = 0x8304FF74; continue 'dispatch;
	// 8304FFD8: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 8304FFDC: 4BFBE1ED  bl 0x8300e1c8
	ctx.lr = 0x8304FFE0;
	sub_8300E1C8(ctx, base);
	// 8304FFE0: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 8304FFE4: 481581D4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304FFE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304FFE8 size=40
    let mut pc: u32 = 0x8304FFE8;
    'dispatch: loop {
        match pc {
            0x8304FFE8 => {
    //   block [0x8304FFE8..0x83050010)
	// 8304FFE8: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 8304FFEC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304FFF0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304FFF4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304FFF8: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 8304FFFC: 4BFBE1CD  bl 0x8300e1c8
	ctx.lr = 0x83050000;
	sub_8300E1C8(ctx, base);
	// 83050000: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83050004: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83050008: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8305000C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83050010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83050010 size=8
    let mut pc: u32 = 0x83050010;
    'dispatch: loop {
        match pc {
            0x83050010 => {
    //   block [0x83050010..0x83050018)
	// 83050010: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83050014: 821633C8  lwz r16, 0x33c8(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(13256 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83050018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83050018 size=440
    let mut pc: u32 = 0x83050018;
    'dispatch: loop {
        match pc {
            0x83050018 => {
    //   block [0x83050018..0x830501D0)
	// 83050018: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8305001C: 48158141  bl 0x831a815c
	ctx.lr = 0x83050020;
	sub_831A8130(ctx, base);
	// 83050020: 3BE1FF00  addi r31, r1, -0x100
	ctx.r[31].s64 = ctx.r[1].s64 + -256;
	// 83050024: 9421FF00  stwu r1, -0x100(r1)
	ea = ctx.r[1].u32.wrapping_add(-256 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83050028: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 8305002C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83050030: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 83050034: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83050038: 4BFA9661  bl 0x82ff9698
	ctx.lr = 0x8305003C;
	sub_82FF9698(ctx, base);
	// 8305003C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83050040: 41820188  beq 0x830501c8
	if ctx.cr[0].eq {
	pc = 0x830501C8; continue 'dispatch;
	}
	// 83050044: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83050048: 8099000C  lwz r4, 0xc(r25)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(12 as u32) ) } as u64;
	// 8305004C: 4BFA92AD  bl 0x82ff92f8
	ctx.lr = 0x83050050;
	sub_82FF92F8(ctx, base);
	// 83050050: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83050054: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 83050058: 80D90000  lwz r6, 0(r25)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 8305005C: 387F00A0  addi r3, r31, 0xa0
	ctx.r[3].s64 = ctx.r[31].s64 + 160;
	// 83050060: 4BF96AE1  bl 0x82fe6b40
	ctx.lr = 0x83050064;
	sub_82FE6B40(ctx, base);
	// 83050064: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83050068: 4BFA8049  bl 0x82ff80b0
	ctx.lr = 0x8305006C;
	sub_82FF80B0(ctx, base);
	// 8305006C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 83050070: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 83050074: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 83050078: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8305007C: 48018725  bl 0x830687a0
	ctx.lr = 0x83050080;
	sub_830687A0(ctx, base);
	// 83050080: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83050084: 4BFA802D  bl 0x82ff80b0
	ctx.lr = 0x83050088;
	sub_82FF80B0(ctx, base);
	// 83050088: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8305008C: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 83050090: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 83050094: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83050098: 48018709  bl 0x830687a0
	ctx.lr = 0x8305009C;
	sub_830687A0(ctx, base);
	// 8305009C: 817F00A8  lwz r11, 0xa8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(168 as u32) ) } as u64;
	// 830500A0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830500A4: 409A001C  bne cr6, 0x830500c0
	if !ctx.cr[6].eq {
	pc = 0x830500C0; continue 'dispatch;
	}
	// 830500A8: 817F00B0  lwz r11, 0xb0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(176 as u32) ) } as u64;
	// 830500AC: 815F00AC  lwz r10, 0xac(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) } as u64;
	// 830500B0: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 830500B4: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830500B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830500BC: 419A0008  beq cr6, 0x830500c4
	if ctx.cr[6].eq {
	pc = 0x830500C4; continue 'dispatch;
	}
	// 830500C0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 830500C4: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830500C8: 41820044  beq 0x8305010c
	if ctx.cr[0].eq {
	pc = 0x8305010C; continue 'dispatch;
	}
	// 830500CC: 387F00A0  addi r3, r31, 0xa0
	ctx.r[3].s64 = ctx.r[31].s64 + 160;
	// 830500D0: 4BFFD091  bl 0x8304d160
	ctx.lr = 0x830500D4;
	sub_8304D160(ctx, base);
	// 830500D4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830500D8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830500DC: 909F0054  stw r4, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[4].u32 ) };
	// 830500E0: 4BFA87B1  bl 0x82ff8890
	ctx.lr = 0x830500E4;
	sub_82FF8890(ctx, base);
	// 830500E4: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 830500E8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830500EC: 4182FFB0  beq 0x8305009c
	if ctx.cr[0].eq {
	pc = 0x8305009C; continue 'dispatch;
	}
	// 830500F0: 389F0050  addi r4, r31, 0x50
	ctx.r[4].s64 = ctx.r[31].s64 + 80;
	// 830500F4: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830500F8: 4BF96B11  bl 0x82fe6c08
	ctx.lr = 0x830500FC;
	sub_82FE6C08(ctx, base);
	// 830500FC: 389F0054  addi r4, r31, 0x54
	ctx.r[4].s64 = ctx.r[31].s64 + 84;
	// 83050100: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 83050104: 4BF96B05  bl 0x82fe6c08
	ctx.lr = 0x83050108;
	sub_82FE6C08(ctx, base);
	// 83050108: 4BFFFF94  b 0x8305009c
	pc = 0x8305009C; continue 'dispatch;
	// 8305010C: 835F0064  lwz r26, 0x64(r31)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 83050110: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83050114: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 83050118: 4BFA91E1  bl 0x82ff92f8
	ctx.lr = 0x8305011C;
	sub_82FF92F8(ctx, base);
	// 8305011C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83050120: 2F1A0000  cmpwi cr6, r26, 0
	ctx.cr[6].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 83050124: 4099006C  ble cr6, 0x83050190
	if !ctx.cr[6].gt {
	pc = 0x83050190; continue 'dispatch;
	}
	// 83050128: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8305012C: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 83050130: 4BFA7FA1  bl 0x82ff80d0
	ctx.lr = 0x83050134;
	sub_82FF80D0(ctx, base);
	// 83050134: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83050138: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8305013C: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 83050140: 836B0000  lwz r27, 0(r11)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83050144: 4BFA7F8D  bl 0x82ff80d0
	ctx.lr = 0x83050148;
	sub_82FF80D0(ctx, base);
	// 83050148: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8305014C: 38BF0054  addi r5, r31, 0x54
	ctx.r[5].s64 = ctx.r[31].s64 + 84;
	// 83050150: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 83050154: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83050158: 4BFAA309  bl 0x82ffa460
	ctx.lr = 0x8305015C;
	sub_82FFA460(ctx, base);
	// 8305015C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83050160: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 83050164: 41820008  beq 0x8305016c
	if ctx.cr[0].eq {
	pc = 0x8305016C; continue 'dispatch;
	}
	// 83050168: 83830000  lwz r28, 0(r3)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8305016C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83050170: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83050174: 4BFA9185  bl 0x82ff92f8
	ctx.lr = 0x83050178;
	sub_82FF92F8(ctx, base);
	// 83050178: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8305017C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83050180: 4BFA9A81  bl 0x82ff9c00
	ctx.lr = 0x83050184;
	sub_82FF9C00(ctx, base);
	// 83050184: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 83050188: 7F1ED000  cmpw cr6, r30, r26
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[26].s32, &mut ctx.xer);
	// 8305018C: 4198FF9C  blt cr6, 0x83050128
	if ctx.cr[6].lt {
	pc = 0x83050128; continue 'dispatch;
	}
	// 83050190: 807F0090  lwz r3, 0x90(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(144 as u32) ) } as u64;
	// 83050194: 809F008C  lwz r4, 0x8c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) } as u64;
	// 83050198: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8305019C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830501A0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830501A4: 4E800421  bctrl
	ctx.lr = 0x830501A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830501A8: 807F0070  lwz r3, 0x70(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 830501AC: 809F006C  lwz r4, 0x6c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 830501B0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830501B4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830501B8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830501BC: 4E800421  bctrl
	ctx.lr = 0x830501C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830501C0: 387F00A0  addi r3, r31, 0xa0
	ctx.r[3].s64 = ctx.r[31].s64 + 160;
	// 830501C4: 4BFBE005  bl 0x8300e1c8
	ctx.lr = 0x830501C8;
	sub_8300E1C8(ctx, base);
	// 830501C8: 383F0100  addi r1, r31, 0x100
	ctx.r[1].s64 = ctx.r[31].s64 + 256;
	// 830501CC: 48157FE0  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830501D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830501D0 size=40
    let mut pc: u32 = 0x830501D0;
    'dispatch: loop {
        match pc {
            0x830501D0 => {
    //   block [0x830501D0..0x830501F8)
	// 830501D0: 3BECFF00  addi r31, r12, -0x100
	ctx.r[31].s64 = ctx.r[12].s64 + -256;
	// 830501D4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830501D8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830501DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830501E0: 387F00A0  addi r3, r31, 0xa0
	ctx.r[3].s64 = ctx.r[31].s64 + 160;
	// 830501E4: 4BFBDFE5  bl 0x8300e1c8
	ctx.lr = 0x830501E8;
	sub_8300E1C8(ctx, base);
	// 830501E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830501EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830501F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830501F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830501F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830501F8 size=40
    let mut pc: u32 = 0x830501F8;
    'dispatch: loop {
        match pc {
            0x830501F8 => {
    //   block [0x830501F8..0x83050220)
	// 830501F8: 3BECFF00  addi r31, r12, -0x100
	ctx.r[31].s64 = ctx.r[12].s64 + -256;
	// 830501FC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83050200: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83050204: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83050208: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 8305020C: 48046A0D  bl 0x83096c18
	ctx.lr = 0x83050210;
	sub_83096C18(ctx, base);
	// 83050210: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83050214: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83050218: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8305021C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83050220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83050220 size=40
    let mut pc: u32 = 0x83050220;
    'dispatch: loop {
        match pc {
            0x83050220 => {
    //   block [0x83050220..0x83050248)
	// 83050220: 3BECFF00  addi r31, r12, -0x100
	ctx.r[31].s64 = ctx.r[12].s64 + -256;
	// 83050224: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83050228: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8305022C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83050230: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 83050234: 480469E5  bl 0x83096c18
	ctx.lr = 0x83050238;
	sub_83096C18(ctx, base);
	// 83050238: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8305023C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83050240: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83050244: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83050248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83050248 size=8
    let mut pc: u32 = 0x83050248;
    'dispatch: loop {
        match pc {
            0x83050248 => {
    //   block [0x83050248..0x83050250)
	// 83050248: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8305024C: 82163430  lwz r16, 0x3430(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(13360 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83050250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83050250 size=328
    let mut pc: u32 = 0x83050250;
    'dispatch: loop {
        match pc {
            0x83050250 => {
    //   block [0x83050250..0x83050398)
	// 83050250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83050254: 48157F15  bl 0x831a8168
	ctx.lr = 0x83050258;
	sub_831A8130(ctx, base);
	// 83050258: 3BE1FF50  addi r31, r1, -0xb0
	ctx.r[31].s64 = ctx.r[1].s64 + -176;
	// 8305025C: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83050260: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83050264: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 83050268: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8305026C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83050270: 4BFA9429  bl 0x82ff9698
	ctx.lr = 0x83050274;
	sub_82FF9698(ctx, base);
	// 83050274: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83050278: 41820118  beq 0x83050390
	if ctx.cr[0].eq {
	pc = 0x83050390; continue 'dispatch;
	}
	// 8305027C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83050280: 809D000C  lwz r4, 0xc(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 83050284: 4BFA9075  bl 0x82ff92f8
	ctx.lr = 0x83050288;
	sub_82FF92F8(ctx, base);
	// 83050288: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8305028C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83050290: 80DD0000  lwz r6, 0(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83050294: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 83050298: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8305029C: 4BFBDB0D  bl 0x8300dda8
	ctx.lr = 0x830502A0;
	sub_8300DDA8(ctx, base);
	// 830502A0: 817F0070  lwz r11, 0x70(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 830502A4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830502A8: 409A001C  bne cr6, 0x830502c4
	if !ctx.cr[6].eq {
	pc = 0x830502C4; continue 'dispatch;
	}
	// 830502AC: 817F006C  lwz r11, 0x6c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 830502B0: 815F0074  lwz r10, 0x74(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) } as u64;
	// 830502B4: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 830502B8: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830502BC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830502C0: 419A0008  beq cr6, 0x830502c8
	if ctx.cr[6].eq {
	pc = 0x830502C8; continue 'dispatch;
	}
	// 830502C4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 830502C8: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830502CC: 41820020  beq 0x830502ec
	if ctx.cr[0].eq {
	pc = 0x830502EC; continue 'dispatch;
	}
	// 830502D0: 38DF0050  addi r6, r31, 0x50
	ctx.r[6].s64 = ctx.r[31].s64 + 80;
	// 830502D4: 38BF0054  addi r5, r31, 0x54
	ctx.r[5].s64 = ctx.r[31].s64 + 84;
	// 830502D8: 389F0058  addi r4, r31, 0x58
	ctx.r[4].s64 = ctx.r[31].s64 + 88;
	// 830502DC: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830502E0: 4BFFCBE1  bl 0x8304cec0
	ctx.lr = 0x830502E4;
	sub_8304CEC0(ctx, base);
	// 830502E4: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 830502E8: 4BFFFFB8  b 0x830502a0
	pc = 0x830502A0; continue 'dispatch;
	// 830502EC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830502F0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830502F4: 4BFA9005  bl 0x82ff92f8
	ctx.lr = 0x830502F8;
	sub_82FF92F8(ctx, base);
	// 830502F8: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 830502FC: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 83050300: 917F0074  stw r11, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83050304: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83050308: 917F0070  stw r11, 0x70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 8305030C: 4BFBD005  bl 0x8300d310
	ctx.lr = 0x83050310;
	sub_8300D310(ctx, base);
	// 83050310: 817F0070  lwz r11, 0x70(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 83050314: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83050318: 409A001C  bne cr6, 0x83050334
	if !ctx.cr[6].eq {
	pc = 0x83050334; continue 'dispatch;
	}
	// 8305031C: 817F006C  lwz r11, 0x6c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 83050320: 815F0074  lwz r10, 0x74(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) } as u64;
	// 83050324: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83050328: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8305032C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83050330: 419A0008  beq cr6, 0x83050338
	if ctx.cr[6].eq {
	pc = 0x83050338; continue 'dispatch;
	}
	// 83050334: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83050338: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8305033C: 4182004C  beq 0x83050388
	if ctx.cr[0].eq {
	pc = 0x83050388; continue 'dispatch;
	}
	// 83050340: 38DF0050  addi r6, r31, 0x50
	ctx.r[6].s64 = ctx.r[31].s64 + 80;
	// 83050344: 38BF0054  addi r5, r31, 0x54
	ctx.r[5].s64 = ctx.r[31].s64 + 84;
	// 83050348: 389F0058  addi r4, r31, 0x58
	ctx.r[4].s64 = ctx.r[31].s64 + 88;
	// 8305034C: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 83050350: 4BFFCB71  bl 0x8304cec0
	ctx.lr = 0x83050354;
	sub_8304CEC0(ctx, base);
	// 83050354: 38FF005C  addi r7, r31, 0x5c
	ctx.r[7].s64 = ctx.r[31].s64 + 92;
	// 83050358: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8305035C: 80DF0050  lwz r6, 0x50(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83050360: 80BF0054  lwz r5, 0x54(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 83050364: 809F0058  lwz r4, 0x58(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 83050368: 4BFC7B49  bl 0x83017eb0
	ctx.lr = 0x8305036C;
	sub_83017EB0(ctx, base);
	// 8305036C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83050370: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83050374: 41820008  beq 0x8305037c
	if ctx.cr[0].eq {
	pc = 0x8305037C; continue 'dispatch;
	}
	// 83050378: 80830000  lwz r4, 0(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8305037C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83050380: 4BFA9881  bl 0x82ff9c00
	ctx.lr = 0x83050384;
	sub_82FF9C00(ctx, base);
	// 83050384: 4BFFFF8C  b 0x83050310
	pc = 0x83050310; continue 'dispatch;
	// 83050388: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 8305038C: 4BFBDDAD  bl 0x8300e138
	ctx.lr = 0x83050390;
	sub_8300E138(ctx, base);
	// 83050390: 383F00B0  addi r1, r31, 0xb0
	ctx.r[1].s64 = ctx.r[31].s64 + 176;
	// 83050394: 48157E24  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83050398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83050398 size=40
    let mut pc: u32 = 0x83050398;
    'dispatch: loop {
        match pc {
            0x83050398 => {
    //   block [0x83050398..0x830503C0)
	// 83050398: 3BECFF50  addi r31, r12, -0xb0
	ctx.r[31].s64 = ctx.r[12].s64 + -176;
	// 8305039C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830503A0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830503A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830503A8: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830503AC: 4BFBDD8D  bl 0x8300e138
	ctx.lr = 0x830503B0;
	sub_8300E138(ctx, base);
	// 830503B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830503B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830503B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830503BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830503C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830503C0 size=8
    let mut pc: u32 = 0x830503C0;
    'dispatch: loop {
        match pc {
            0x830503C0 => {
    //   block [0x830503C0..0x830503C8)
	// 830503C0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830503C4: 82163478  lwz r16, 0x3478(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(13432 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830503C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830503C8 size=96
    let mut pc: u32 = 0x830503C8;
    'dispatch: loop {
        match pc {
            0x830503C8 => {
    //   block [0x830503C8..0x83050428)
	// 830503C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830503CC: 48157DA1  bl 0x831a816c
	ctx.lr = 0x830503D0;
	sub_831A8130(ctx, base);
	// 830503D0: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 830503D4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830503D8: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 830503DC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830503E0: 396B2A18  addi r11, r11, 0x2a18
	ctx.r[11].s64 = ctx.r[11].s64 + 10776;
	// 830503E4: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 830503E8: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830503EC: 897E0004  lbz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830503F0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830503F4: 41820020  beq 0x83050414
	if ctx.cr[0].eq {
	pc = 0x83050414; continue 'dispatch;
	}
	// 830503F8: 83BE0010  lwz r29, 0x10(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 830503FC: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83050400: 41820014  beq 0x83050414
	if ctx.cr[0].eq {
	pc = 0x83050414; continue 'dispatch;
	}
	// 83050404: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83050408: 4BFFF769  bl 0x8304fb70
	ctx.lr = 0x8305040C;
	sub_8304FB70(ctx, base);
	// 8305040C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83050410: 4BF87ED1  bl 0x82fd82e0
	ctx.lr = 0x83050414;
	sub_82FD82E0(ctx, base);
	// 83050414: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83050418: 396BA93C  addi r11, r11, -0x56c4
	ctx.r[11].s64 = ctx.r[11].s64 + -22212;
	// 8305041C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83050420: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 83050424: 48157D98  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83050428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83050428 size=40
    let mut pc: u32 = 0x83050428;
    'dispatch: loop {
        match pc {
            0x83050428 => {
    //   block [0x83050428..0x83050450)
	// 83050428: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 8305042C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83050430: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83050434: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83050438: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 8305043C: 4BFFC325  bl 0x8304c760
	ctx.lr = 0x83050440;
	sub_8304C760(ctx, base);
	// 83050440: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83050444: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83050448: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8305044C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83050450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83050450 size=76
    let mut pc: u32 = 0x83050450;
    'dispatch: loop {
        match pc {
            0x83050450 => {
    //   block [0x83050450..0x8305049C)
	// 83050450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83050454: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83050458: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8305045C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83050460: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83050464: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83050468: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8305046C: 4BFFFF5D  bl 0x830503c8
	ctx.lr = 0x83050470;
	sub_830503C8(ctx, base);
	// 83050470: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83050474: 4182000C  beq 0x83050480
	if ctx.cr[0].eq {
	pc = 0x83050480; continue 'dispatch;
	}
	// 83050478: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8305047C: 4BF87E65  bl 0x82fd82e0
	ctx.lr = 0x83050480;
	sub_82FD82E0(ctx, base);
	// 83050480: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83050484: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83050488: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8305048C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83050490: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83050494: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83050498: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830504A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830504A0 size=8
    let mut pc: u32 = 0x830504A0;
    'dispatch: loop {
        match pc {
            0x830504A0 => {
    //   block [0x830504A0..0x830504A8)
	// 830504A0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830504A4: 821634B0  lwz r16, 0x34b0(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(13488 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830504A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830504A8 size=292
    let mut pc: u32 = 0x830504A8;
    'dispatch: loop {
        match pc {
            0x830504A8 => {
    //   block [0x830504A8..0x830505CC)
	// 830504A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830504AC: 48157CBD  bl 0x831a8168
	ctx.lr = 0x830504B0;
	sub_831A8130(ctx, base);
	// 830504B0: 3BE1FF50  addi r31, r1, -0xb0
	ctx.r[31].s64 = ctx.r[1].s64 + -176;
	// 830504B4: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830504B8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830504BC: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 830504C0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830504C4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830504C8: 4BFA91D1  bl 0x82ff9698
	ctx.lr = 0x830504CC;
	sub_82FF9698(ctx, base);
	// 830504CC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830504D0: 418200F4  beq 0x830505c4
	if ctx.cr[0].eq {
	pc = 0x830505C4; continue 'dispatch;
	}
	// 830504D4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830504D8: 809D000C  lwz r4, 0xc(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 830504DC: 4BFA8E1D  bl 0x82ff92f8
	ctx.lr = 0x830504E0;
	sub_82FF92F8(ctx, base);
	// 830504E0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830504E4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830504E8: 80DD0000  lwz r6, 0(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830504EC: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830504F0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 830504F4: 4BFEF8D5  bl 0x8303fdc8
	ctx.lr = 0x830504F8;
	sub_8303FDC8(ctx, base);
	// 830504F8: 817F0068  lwz r11, 0x68(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 830504FC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83050500: 409A001C  bne cr6, 0x8305051c
	if !ctx.cr[6].eq {
	pc = 0x8305051C; continue 'dispatch;
	}
	// 83050504: 817F0070  lwz r11, 0x70(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 83050508: 815F006C  lwz r10, 0x6c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 8305050C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83050510: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83050514: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83050518: 419A0008  beq cr6, 0x83050520
	if ctx.cr[6].eq {
	pc = 0x83050520; continue 'dispatch;
	}
	// 8305051C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83050520: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83050524: 41820014  beq 0x83050538
	if ctx.cr[0].eq {
	pc = 0x83050538; continue 'dispatch;
	}
	// 83050528: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 8305052C: 4BF9370D  bl 0x82fe3c38
	ctx.lr = 0x83050530;
	sub_82FE3C38(ctx, base);
	// 83050530: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 83050534: 4BFFFFC4  b 0x830504f8
	pc = 0x830504F8; continue 'dispatch;
	// 83050538: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8305053C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83050540: 4BFA8DB9  bl 0x82ff92f8
	ctx.lr = 0x83050544;
	sub_82FF92F8(ctx, base);
	// 83050544: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 83050548: 4BFEF951  bl 0x8303fe98
	ctx.lr = 0x8305054C;
	sub_8303FE98(ctx, base);
	// 8305054C: 817F0068  lwz r11, 0x68(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 83050550: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83050554: 409A001C  bne cr6, 0x83050570
	if !ctx.cr[6].eq {
	pc = 0x83050570; continue 'dispatch;
	}
	// 83050558: 817F0070  lwz r11, 0x70(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 8305055C: 815F006C  lwz r10, 0x6c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 83050560: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83050564: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83050568: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8305056C: 419A0008  beq cr6, 0x83050574
	if ctx.cr[6].eq {
	pc = 0x83050574; continue 'dispatch;
	}
	// 83050570: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83050574: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83050578: 41820044  beq 0x830505bc
	if ctx.cr[0].eq {
	pc = 0x830505BC; continue 'dispatch;
	}
	// 8305057C: 38BF0050  addi r5, r31, 0x50
	ctx.r[5].s64 = ctx.r[31].s64 + 80;
	// 83050580: 389F0054  addi r4, r31, 0x54
	ctx.r[4].s64 = ctx.r[31].s64 + 84;
	// 83050584: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 83050588: 4BF93731  bl 0x82fe3cb8
	ctx.lr = 0x8305058C;
	sub_82FE3CB8(ctx, base);
	// 8305058C: 38DF0058  addi r6, r31, 0x58
	ctx.r[6].s64 = ctx.r[31].s64 + 88;
	// 83050590: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83050594: 80BF0050  lwz r5, 0x50(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83050598: 809F0054  lwz r4, 0x54(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8305059C: 48018665  bl 0x83068c00
	ctx.lr = 0x830505A0;
	sub_83068C00(ctx, base);
	// 830505A0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830505A4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830505A8: 41820008  beq 0x830505b0
	if ctx.cr[0].eq {
	pc = 0x830505B0; continue 'dispatch;
	}
	// 830505AC: 80830000  lwz r4, 0(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830505B0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830505B4: 4BFA964D  bl 0x82ff9c00
	ctx.lr = 0x830505B8;
	sub_82FF9C00(ctx, base);
	// 830505B8: 4BFFFF94  b 0x8305054c
	pc = 0x8305054C; continue 'dispatch;
	// 830505BC: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830505C0: 4BFEFFD9  bl 0x83040598
	ctx.lr = 0x830505C4;
	sub_83040598(ctx, base);
	// 830505C4: 383F00B0  addi r1, r31, 0xb0
	ctx.r[1].s64 = ctx.r[31].s64 + 176;
	// 830505C8: 48157BF0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830505CC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830505CC size=40
    let mut pc: u32 = 0x830505CC;
    'dispatch: loop {
        match pc {
            0x830505CC => {
    //   block [0x830505CC..0x830505F4)
	// 830505CC: 3BECFF50  addi r31, r12, -0xb0
	ctx.r[31].s64 = ctx.r[12].s64 + -176;
	// 830505D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830505D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830505D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830505DC: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830505E0: 4BFEFFB9  bl 0x83040598
	ctx.lr = 0x830505E4;
	sub_83040598(ctx, base);
	// 830505E4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830505E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830505EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830505F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830505F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830505F8 size=8
    let mut pc: u32 = 0x830505F8;
    'dispatch: loop {
        match pc {
            0x830505F8 => {
    //   block [0x830505F8..0x83050600)
	// 830505F8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830505FC: 821634F8  lwz r16, 0x34f8(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(13560 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83050600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83050600 size=328
    let mut pc: u32 = 0x83050600;
    'dispatch: loop {
        match pc {
            0x83050600 => {
    //   block [0x83050600..0x83050748)
	// 83050600: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83050604: 48157B61  bl 0x831a8164
	ctx.lr = 0x83050608;
	sub_831A8130(ctx, base);
	// 83050608: 3BE1FF50  addi r31, r1, -0xb0
	ctx.r[31].s64 = ctx.r[1].s64 + -176;
	// 8305060C: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83050610: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 83050614: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 83050618: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8305061C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83050620: 4BFA9079  bl 0x82ff9698
	ctx.lr = 0x83050624;
	sub_82FF9698(ctx, base);
	// 83050624: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83050628: 41820118  beq 0x83050740
	if ctx.cr[0].eq {
	pc = 0x83050740; continue 'dispatch;
	}
	// 8305062C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83050630: 809B000C  lwz r4, 0xc(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(12 as u32) ) } as u64;
	// 83050634: 4BFA8CC5  bl 0x82ff92f8
	ctx.lr = 0x83050638;
	sub_82FF92F8(ctx, base);
	// 83050638: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8305063C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83050640: 80DB0000  lwz r6, 0(r27)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 83050644: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 83050648: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8305064C: 4BFFCB9D  bl 0x8304d1e8
	ctx.lr = 0x83050650;
	sub_8304D1E8(ctx, base);
	// 83050650: 817F0068  lwz r11, 0x68(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 83050654: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83050658: 409A001C  bne cr6, 0x83050674
	if !ctx.cr[6].eq {
	pc = 0x83050674; continue 'dispatch;
	}
	// 8305065C: 817F0070  lwz r11, 0x70(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 83050660: 815F006C  lwz r10, 0x6c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 83050664: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83050668: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8305066C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83050670: 419A0008  beq cr6, 0x83050678
	if ctx.cr[6].eq {
	pc = 0x83050678; continue 'dispatch;
	}
	// 83050674: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83050678: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8305067C: 41820014  beq 0x83050690
	if ctx.cr[0].eq {
	pc = 0x83050690; continue 'dispatch;
	}
	// 83050680: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 83050684: 4BF935B5  bl 0x82fe3c38
	ctx.lr = 0x83050688;
	sub_82FE3C38(ctx, base);
	// 83050688: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 8305068C: 4BFFFFC4  b 0x83050650
	pc = 0x83050650; continue 'dispatch;
	// 83050690: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83050694: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83050698: 4BFA8C61  bl 0x82ff92f8
	ctx.lr = 0x8305069C;
	sub_82FF92F8(ctx, base);
	// 8305069C: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830506A0: 4BFEF7F9  bl 0x8303fe98
	ctx.lr = 0x830506A4;
	sub_8303FE98(ctx, base);
	// 830506A4: 817F0068  lwz r11, 0x68(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 830506A8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830506AC: 409A001C  bne cr6, 0x830506c8
	if !ctx.cr[6].eq {
	pc = 0x830506C8; continue 'dispatch;
	}
	// 830506B0: 817F0070  lwz r11, 0x70(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 830506B4: 815F006C  lwz r10, 0x6c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 830506B8: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 830506BC: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830506C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830506C4: 419A0008  beq cr6, 0x830506cc
	if ctx.cr[6].eq {
	pc = 0x830506CC; continue 'dispatch;
	}
	// 830506C8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 830506CC: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830506D0: 41820068  beq 0x83050738
	if ctx.cr[0].eq {
	pc = 0x83050738; continue 'dispatch;
	}
	// 830506D4: 38BF0054  addi r5, r31, 0x54
	ctx.r[5].s64 = ctx.r[31].s64 + 84;
	// 830506D8: 389F0050  addi r4, r31, 0x50
	ctx.r[4].s64 = ctx.r[31].s64 + 80;
	// 830506DC: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830506E0: 4BF935D9  bl 0x82fe3cb8
	ctx.lr = 0x830506E4;
	sub_82FE3CB8(ctx, base);
	// 830506E4: 83DF0050  lwz r30, 0x50(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830506E8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 830506EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830506F0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830506F4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830506F8: 4BFA9209  bl 0x82ff9900
	ctx.lr = 0x830506FC;
	sub_82FF9900(ctx, base);
	// 830506FC: 83BF0054  lwz r29, 0x54(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 83050700: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83050704: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83050708: 4BFA8BF1  bl 0x82ff92f8
	ctx.lr = 0x8305070C;
	sub_82FF92F8(ctx, base);
	// 8305070C: 38DF0058  addi r6, r31, 0x58
	ctx.r[6].s64 = ctx.r[31].s64 + 88;
	// 83050710: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83050714: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83050718: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8305071C: 480184E5  bl 0x83068c00
	ctx.lr = 0x83050720;
	sub_83068C00(ctx, base);
	// 83050720: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83050724: 41820008  beq 0x8305072c
	if ctx.cr[0].eq {
	pc = 0x8305072C; continue 'dispatch;
	}
	// 83050728: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8305072C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83050730: 4BFFC2A1  bl 0x8304c9d0
	ctx.lr = 0x83050734;
	sub_8304C9D0(ctx, base);
	// 83050734: 4BFFFF70  b 0x830506a4
	pc = 0x830506A4; continue 'dispatch;
	// 83050738: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 8305073C: 4BFFFC8D  bl 0x830503c8
	ctx.lr = 0x83050740;
	sub_830503C8(ctx, base);
	// 83050740: 383F00B0  addi r1, r31, 0xb0
	ctx.r[1].s64 = ctx.r[31].s64 + 176;
	// 83050744: 48157A70  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83050748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83050748 size=40
    let mut pc: u32 = 0x83050748;
    'dispatch: loop {
        match pc {
            0x83050748 => {
    //   block [0x83050748..0x83050770)
	// 83050748: 3BECFF50  addi r31, r12, -0xb0
	ctx.r[31].s64 = ctx.r[12].s64 + -176;
	// 8305074C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83050750: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83050754: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83050758: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 8305075C: 4BFFFC6D  bl 0x830503c8
	ctx.lr = 0x83050760;
	sub_830503C8(ctx, base);
	// 83050760: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83050764: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83050768: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8305076C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83050770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83050770 size=8
    let mut pc: u32 = 0x83050770;
    'dispatch: loop {
        match pc {
            0x83050770 => {
    //   block [0x83050770..0x83050778)
	// 83050770: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83050774: 82163570  lwz r16, 0x3570(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(13680 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83050778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83050778 size=76
    let mut pc: u32 = 0x83050778;
    'dispatch: loop {
        match pc {
            0x83050778 => {
    //   block [0x83050778..0x830507C4)
	// 83050778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8305077C: 481579F1  bl 0x831a816c
	ctx.lr = 0x83050780;
	sub_831A8130(ctx, base);
	// 83050780: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 83050784: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83050788: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8305078C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83050790: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 83050794: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 83050798: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 8305079C: 4BFAD34D  bl 0x82ffdae8
	ctx.lr = 0x830507A0;
	sub_82FFDAE8(ctx, base);
	// 830507A0: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 830507A4: 387E001C  addi r3, r30, 0x1c
	ctx.r[3].s64 = ctx.r[30].s64 + 28;
	// 830507A8: 93BE0018  stw r29, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[29].u32 ) };
	// 830507AC: 396B3538  addi r11, r11, 0x3538
	ctx.r[11].s64 = ctx.r[11].s64 + 13624;
	// 830507B0: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830507B4: 4BFA4F95  bl 0x82ff5748
	ctx.lr = 0x830507B8;
	sub_82FF5748(ctx, base);
	// 830507B8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830507BC: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 830507C0: 481579FC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830507C4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830507C4 size=40
    let mut pc: u32 = 0x830507C4;
    'dispatch: loop {
        match pc {
            0x830507C4 => {
    //   block [0x830507C4..0x830507EC)
	// 830507C4: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 830507C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830507CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830507D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830507D4: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 830507D8: 4BFAD989  bl 0x82ffe160
	ctx.lr = 0x830507DC;
	sub_82FFE160(ctx, base);
	// 830507DC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830507E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830507E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830507E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830507F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830507F0 size=8
    let mut pc: u32 = 0x830507F0;
    'dispatch: loop {
        match pc {
            0x830507F0 => {
    //   block [0x830507F0..0x830507F8)
	// 830507F0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830507F4: 821635A8  lwz r16, 0x35a8(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(13736 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830507F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830507F8 size=84
    let mut pc: u32 = 0x830507F8;
    'dispatch: loop {
        match pc {
            0x830507F8 => {
    //   block [0x830507F8..0x8305084C)
	// 830507F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830507FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83050800: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83050804: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83050808: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 8305080C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83050810: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83050814: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83050818: 396B3538  addi r11, r11, 0x3538
	ctx.r[11].s64 = ctx.r[11].s64 + 13624;
	// 8305081C: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 83050820: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83050824: 387E001C  addi r3, r30, 0x1c
	ctx.r[3].s64 = ctx.r[30].s64 + 28;
	// 83050828: 4BFA4F61  bl 0x82ff5788
	ctx.lr = 0x8305082C;
	sub_82FF5788(ctx, base);
	// 8305082C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83050830: 4BFAD931  bl 0x82ffe160
	ctx.lr = 0x83050834;
	sub_82FFE160(ctx, base);
	// 83050834: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 83050838: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8305083C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83050840: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83050844: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83050848: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8305084C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8305084C size=40
    let mut pc: u32 = 0x8305084C;
    'dispatch: loop {
        match pc {
            0x8305084C => {
    //   block [0x8305084C..0x83050874)
	// 8305084C: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 83050850: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83050854: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83050858: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8305085C: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83050860: 4BFAD901  bl 0x82ffe160
	ctx.lr = 0x83050864;
	sub_82FFE160(ctx, base);
	// 83050864: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83050868: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8305086C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83050870: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83050878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83050878 size=8
    let mut pc: u32 = 0x83050878;
    'dispatch: loop {
        match pc {
            0x83050878 => {
    //   block [0x83050878..0x83050880)
	// 83050878: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8305087C: 821635E8  lwz r16, 0x35e8(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(13800 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83050880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83050880 size=128
    let mut pc: u32 = 0x83050880;
    'dispatch: loop {
        match pc {
            0x83050880 => {
    //   block [0x83050880..0x83050900)
	// 83050880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83050884: 481578E9  bl 0x831a816c
	ctx.lr = 0x83050888;
	sub_831A8130(ctx, base);
	// 83050888: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 8305088C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83050890: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83050894: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83050898: 807E0018  lwz r3, 0x18(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 8305089C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830508A0: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 830508A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830508A8: 4E800421  bctrl
	ctx.lr = 0x830508AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830508AC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830508B0: 40820048  bne 0x830508f8
	if !ctx.cr[0].eq {
	pc = 0x830508F8; continue 'dispatch;
	}
	// 830508B4: 389E001C  addi r4, r30, 0x1c
	ctx.r[4].s64 = ctx.r[30].s64 + 28;
	// 830508B8: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830508BC: 4BFA4F1D  bl 0x82ff57d8
	ctx.lr = 0x830508C0;
	sub_82FF57D8(ctx, base);
	// 830508C0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830508C4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830508C8: 4BFAD791  bl 0x82ffe058
	ctx.lr = 0x830508CC;
	sub_82FFE058(ctx, base);
	// 830508CC: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 830508D0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830508D4: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 830508D8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830508DC: 816A0028  lwz r11, 0x28(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(40 as u32) ) } as u64;
	// 830508E0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830508E4: 4E800421  bctrl
	ctx.lr = 0x830508E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830508E8: 7FC3F214  add r30, r3, r30
	ctx.r[30].u64 = ctx.r[3].u64 + ctx.r[30].u64;
	// 830508EC: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830508F0: 4BFA4F21  bl 0x82ff5810
	ctx.lr = 0x830508F4;
	sub_82FF5810(ctx, base);
	// 830508F4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830508F8: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 830508FC: 481578C0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83050900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83050900 size=40
    let mut pc: u32 = 0x83050900;
    'dispatch: loop {
        match pc {
            0x83050900 => {
    //   block [0x83050900..0x83050928)
	// 83050900: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83050904: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83050908: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8305090C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83050910: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83050914: 4BFA4EFD  bl 0x82ff5810
	ctx.lr = 0x83050918;
	sub_82FF5810(ctx, base);
	// 83050918: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8305091C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83050920: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83050924: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83050928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83050928 size=100
    let mut pc: u32 = 0x83050928;
    'dispatch: loop {
        match pc {
            0x83050928 => {
    //   block [0x83050928..0x8305098C)
	// 83050928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8305092C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83050930: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83050934: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83050938: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8305093C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83050940: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83050944: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83050948: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8305094C: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 83050950: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83050954: 4E800421  bctrl
	ctx.lr = 0x83050958;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83050958: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8305095C: 4182000C  beq 0x83050968
	if ctx.cr[0].eq {
	pc = 0x83050968; continue 'dispatch;
	}
	// 83050960: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83050964: 48000010  b 0x83050974
	pc = 0x83050974; continue 'dispatch;
	// 83050968: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8305096C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83050970: 4BFAD289  bl 0x82ffdbf8
	ctx.lr = 0x83050974;
	sub_82FFDBF8(ctx, base);
	// 83050974: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83050978: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8305097C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83050980: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83050984: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83050988: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83050990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83050990 size=104
    let mut pc: u32 = 0x83050990;
    'dispatch: loop {
        match pc {
            0x83050990 => {
    //   block [0x83050990..0x830509F8)
	// 83050990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83050994: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83050998: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8305099C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830509A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830509A4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830509A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830509AC: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 830509B0: 419A002C  beq cr6, 0x830509dc
	if ctx.cr[6].eq {
	pc = 0x830509DC; continue 'dispatch;
	}
	// 830509B4: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 830509B8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830509BC: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 830509C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830509C4: 4E800421  bctrl
	ctx.lr = 0x830509C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830509C8: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 830509CC: 7D635A14  add r11, r3, r11
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[11].u64;
	// 830509D0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 830509D4: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830509D8: 41980008  blt cr6, 0x830509e0
	if ctx.cr[6].lt {
	pc = 0x830509E0; continue 'dispatch;
	}
	// 830509DC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830509E0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830509E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830509E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830509EC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830509F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830509F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830509F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830509F8 size=4
    let mut pc: u32 = 0x830509F8;
    'dispatch: loop {
        match pc {
            0x830509F8 => {
    //   block [0x830509F8..0x830509FC)
	// 830509F8: 4BFAD238  b 0x82ffdc30
	sub_82FFDC30(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83050A00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83050A00 size=124
    let mut pc: u32 = 0x83050A00;
    'dispatch: loop {
        match pc {
            0x83050A00 => {
    //   block [0x83050A00..0x83050A7C)
	// 83050A00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83050A04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83050A08: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83050A0C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83050A10: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83050A14: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83050A18: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83050A1C: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83050A20: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83050A24: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 83050A28: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83050A2C: 4E800421  bctrl
	ctx.lr = 0x83050A30;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83050A30: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83050A34: 40820030  bne 0x83050a64
	if !ctx.cr[0].eq {
	pc = 0x83050A64; continue 'dispatch;
	}
	// 83050A38: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83050A3C: 83DF0018  lwz r30, 0x18(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83050A40: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83050A44: 4BFAD205  bl 0x82ffdc48
	ctx.lr = 0x83050A48;
	sub_82FFDC48(ctx, base);
	// 83050A48: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83050A4C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83050A50: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83050A54: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 83050A58: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83050A5C: 4E800421  bctrl
	ctx.lr = 0x83050A60;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83050A60: 7C7F1A14  add r3, r31, r3
	ctx.r[3].u64 = ctx.r[31].u64 + ctx.r[3].u64;
	// 83050A64: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83050A68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83050A6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83050A70: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83050A74: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83050A78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83050A80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83050A80 size=136
    let mut pc: u32 = 0x83050A80;
    'dispatch: loop {
        match pc {
            0x83050A80 => {
    //   block [0x83050A80..0x83050B08)
	// 83050A80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83050A84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83050A88: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83050A8C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83050A90: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83050A94: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83050A98: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83050A9C: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83050AA0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83050AA4: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 83050AA8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83050AAC: 4E800421  bctrl
	ctx.lr = 0x83050AB0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83050AB0: 7F1E1840  cmplw cr6, r30, r3
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[3].u32, &mut ctx.xer);
	// 83050AB4: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83050AB8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83050ABC: 41990018  bgt cr6, 0x83050ad4
	if ctx.cr[6].gt {
	pc = 0x83050AD4; continue 'dispatch;
	}
	// 83050AC0: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 83050AC4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83050AC8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83050ACC: 4E800421  bctrl
	ctx.lr = 0x83050AD0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83050AD0: 48000020  b 0x83050af0
	pc = 0x83050AF0; continue 'dispatch;
	// 83050AD4: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 83050AD8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83050ADC: 4E800421  bctrl
	ctx.lr = 0x83050AE0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83050AE0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83050AE4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83050AE8: 7C8BF050  subf r4, r11, r30
	ctx.r[4].s64 = ctx.r[30].s64 - ctx.r[11].s64;
	// 83050AEC: 4BFACD95  bl 0x82ffd880
	ctx.lr = 0x83050AF0;
	sub_82FFD880(ctx, base);
	// 83050AF0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83050AF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83050AF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83050AFC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83050B00: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83050B04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83050B08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83050B08 size=72
    let mut pc: u32 = 0x83050B08;
    'dispatch: loop {
        match pc {
            0x83050B08 => {
    //   block [0x83050B08..0x83050B50)
	// 83050B08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83050B0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83050B10: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83050B14: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83050B18: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83050B1C: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83050B20: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83050B24: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 83050B28: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83050B2C: 4E800421  bctrl
	ctx.lr = 0x83050B30;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83050B30: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83050B34: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 83050B38: 7C635A14  add r3, r3, r11
	ctx.r[3].u64 = ctx.r[3].u64 + ctx.r[11].u64;
	// 83050B3C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83050B40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83050B44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83050B48: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83050B4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83050B50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83050B50 size=76
    let mut pc: u32 = 0x83050B50;
    'dispatch: loop {
        match pc {
            0x83050B50 => {
    //   block [0x83050B50..0x83050B9C)
	// 83050B50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83050B54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83050B58: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83050B5C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83050B60: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83050B64: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83050B68: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83050B6C: 4BFFFC8D  bl 0x830507f8
	ctx.lr = 0x83050B70;
	sub_830507F8(ctx, base);
	// 83050B70: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83050B74: 4182000C  beq 0x83050b80
	if ctx.cr[0].eq {
	pc = 0x83050B80; continue 'dispatch;
	}
	// 83050B78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83050B7C: 4BF87765  bl 0x82fd82e0
	ctx.lr = 0x83050B80;
	sub_82FD82E0(ctx, base);
	// 83050B80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83050B84: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83050B88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83050B8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83050B90: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83050B94: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83050B98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83050BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83050BA0 size=124
    let mut pc: u32 = 0x83050BA0;
    'dispatch: loop {
        match pc {
            0x83050BA0 => {
    //   block [0x83050BA0..0x83050C1C)
	// 83050BA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83050BA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83050BA8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83050BAC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83050BB0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83050BB4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83050BB8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83050BBC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83050BC0: 419A0044  beq cr6, 0x83050c04
	if ctx.cr[6].eq {
	pc = 0x83050C04; continue 'dispatch;
	}
	// 83050BC4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83050BC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83050BCC: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83050BD0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83050BD4: 4E800421  bctrl
	ctx.lr = 0x83050BD8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83050BD8: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83050BDC: 40820028  bne 0x83050c04
	if !ctx.cr[0].eq {
	pc = 0x83050C04; continue 'dispatch;
	}
	// 83050BE0: 807E0020  lwz r3, 0x20(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 83050BE4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83050BE8: 41820018  beq 0x83050c00
	if ctx.cr[0].eq {
	pc = 0x83050C00; continue 'dispatch;
	}
	// 83050BEC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83050BF0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83050BF4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83050BF8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83050BFC: 4E800421  bctrl
	ctx.lr = 0x83050C00;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83050C00: 93FE0020  stw r31, 0x20(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[31].u32 ) };
	// 83050C04: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83050C08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83050C0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83050C10: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83050C14: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83050C18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83050C20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83050C20 size=12
    let mut pc: u32 = 0x83050C20;
    'dispatch: loop {
        match pc {
            0x83050C20 => {
    //   block [0x83050C20..0x83050C2C)
	// 83050C20: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 83050C24: 386B33FC  addi r3, r11, 0x33fc
	ctx.r[3].s64 = ctx.r[11].s64 + 13308;
	// 83050C28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83050C30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83050C30 size=252
    let mut pc: u32 = 0x83050C30;
    'dispatch: loop {
        match pc {
            0x83050C30 => {
    //   block [0x83050C30..0x83050D2C)
	// 83050C30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83050C34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83050C38: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83050C3C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83050C40: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83050C44: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83050C48: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83050C4C: 48076E95  bl 0x830c7ae0
	ctx.lr = 0x83050C50;
	sub_830C7AE0(ctx, base);
	// 83050C50: A97E0000  lha r11, 0(r30)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as i16) as i64;
	// 83050C54: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83050C58: 556BDFFF  rlwinm. r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83050C5C: 41820044  beq 0x83050ca0
	if ctx.cr[0].eq {
	pc = 0x83050CA0; continue 'dispatch;
	}
	// 83050C60: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83050C64: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83050C68: 4BFFE439  bl 0x8304f0a0
	ctx.lr = 0x83050C6C;
	sub_8304F0A0(ctx, base);
	// 83050C6C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83050C70: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83050C74: 4BFFE42D  bl 0x8304f0a0
	ctx.lr = 0x83050C78;
	sub_8304F0A0(ctx, base);
	// 83050C78: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83050C7C: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83050C80: 4BFFE421  bl 0x8304f0a0
	ctx.lr = 0x83050C84;
	sub_8304F0A0(ctx, base);
	// 83050C84: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83050C88: 809F0018  lwz r4, 0x18(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83050C8C: 4BFA866D  bl 0x82ff92f8
	ctx.lr = 0x83050C90;
	sub_82FF92F8(ctx, base);
	// 83050C90: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83050C94: 889F001C  lbz r4, 0x1c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 83050C98: 4BFA8569  bl 0x82ff9200
	ctx.lr = 0x83050C9C;
	sub_82FF9200(ctx, base);
	// 83050C9C: 48000060  b 0x83050cfc
	pc = 0x83050CFC; continue 'dispatch;
	// 83050CA0: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 83050CA4: 38A00080  li r5, 0x80
	ctx.r[5].s64 = 128;
	// 83050CA8: 3880006D  li r4, 0x6d
	ctx.r[4].s64 = 109;
	// 83050CAC: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 83050CB0: 4BFFE531  bl 0x8304f1e0
	ctx.lr = 0x83050CB4;
	sub_8304F1E0(ctx, base);
	// 83050CB4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83050CB8: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 83050CBC: 38A00080  li r5, 0x80
	ctx.r[5].s64 = 128;
	// 83050CC0: 3880006D  li r4, 0x6d
	ctx.r[4].s64 = 109;
	// 83050CC4: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 83050CC8: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 83050CCC: 4BFFE6BD  bl 0x8304f388
	ctx.lr = 0x83050CD0;
	sub_8304F388(ctx, base);
	// 83050CD0: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 83050CD4: 38A00080  li r5, 0x80
	ctx.r[5].s64 = 128;
	// 83050CD8: 3880006D  li r4, 0x6d
	ctx.r[4].s64 = 109;
	// 83050CDC: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 83050CE0: 4BFFE869  bl 0x8304f548
	ctx.lr = 0x83050CE4;
	sub_8304F548(ctx, base);
	// 83050CE4: 389F0018  addi r4, r31, 0x18
	ctx.r[4].s64 = ctx.r[31].s64 + 24;
	// 83050CE8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83050CEC: 4BFA888D  bl 0x82ff9578
	ctx.lr = 0x83050CF0;
	sub_82FF9578(ctx, base);
	// 83050CF0: 389F001C  addi r4, r31, 0x1c
	ctx.r[4].s64 = ctx.r[31].s64 + 28;
	// 83050CF4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83050CF8: 4BFA8789  bl 0x82ff9480
	ctx.lr = 0x83050CFC;
	sub_82FF9480(ctx, base);
	// 83050CFC: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 83050D00: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83050D04: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83050D08: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83050D0C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83050D10: 4E800421  bctrl
	ctx.lr = 0x83050D14;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83050D14: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83050D18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83050D1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83050D20: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83050D24: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83050D28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83050D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83050D30 size=8
    let mut pc: u32 = 0x83050D30;
    'dispatch: loop {
        match pc {
            0x83050D30 => {
    //   block [0x83050D30..0x83050D38)
	// 83050D30: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83050D34: 82163648  lwz r16, 0x3648(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(13896 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83050D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83050D38 size=236
    let mut pc: u32 = 0x83050D38;
    'dispatch: loop {
        match pc {
            0x83050D38 => {
    //   block [0x83050D38..0x83050E24)
	// 83050D38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83050D3C: 48157425  bl 0x831a8160
	ctx.lr = 0x83050D40;
	sub_831A8130(ctx, base);
	// 83050D40: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 83050D44: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83050D48: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83050D4C: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 83050D50: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83050D54: 7D074378  mr r7, r8
	ctx.r[7].u64 = ctx.r[8].u64;
	// 83050D58: 7D3A4B78  mr r26, r9
	ctx.r[26].u64 = ctx.r[9].u64;
	// 83050D5C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83050D60: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 83050D64: 93DF00A4  stw r30, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[30].u32 ) };
	// 83050D68: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83050D6C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83050D70: 4E800421  bctrl
	ctx.lr = 0x83050D74;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83050D74: 7C7B1B79  or. r27, r3, r3
	ctx.r[27].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 83050D78: 40820098  bne 0x83050e10
	if !ctx.cr[0].eq {
	pc = 0x83050E10; continue 'dispatch;
	}
	// 83050D7C: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 83050D80: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83050D84: 4BF87515  bl 0x82fd8298
	ctx.lr = 0x83050D88;
	sub_82FD8298(ctx, base);
	// 83050D88: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83050D8C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83050D90: 41820020  beq 0x83050db0
	if ctx.cr[0].eq {
	pc = 0x83050DB0; continue 'dispatch;
	}
	// 83050D94: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 83050D98: 80FE0004  lwz r7, 4(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83050D9C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83050DA0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83050DA4: 48013A4D  bl 0x830647f0
	ctx.lr = 0x83050DA8;
	sub_830647F0(ctx, base);
	// 83050DA8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83050DAC: 48000008  b 0x83050db4
	pc = 0x83050DB4; continue 'dispatch;
	// 83050DB0: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83050DB4: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83050DB8: 7FBBEB78  mr r27, r29
	ctx.r[27].u64 = ctx.r[29].u64;
	// 83050DBC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83050DC0: 409A0038  bne cr6, 0x83050df8
	if !ctx.cr[6].eq {
	pc = 0x83050DF8; continue 'dispatch;
	}
	// 83050DC4: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 83050DC8: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83050DCC: 4BF874CD  bl 0x82fd8298
	ctx.lr = 0x83050DD0;
	sub_82FD8298(ctx, base);
	// 83050DD0: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83050DD4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83050DD8: 41820018  beq 0x83050df0
	if ctx.cr[0].eq {
	pc = 0x83050DF0; continue 'dispatch;
	}
	// 83050DDC: 38A00080  li r5, 0x80
	ctx.r[5].s64 = 128;
	// 83050DE0: 80DE0004  lwz r6, 4(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83050DE4: 3880001D  li r4, 0x1d
	ctx.r[4].s64 = 29;
	// 83050DE8: 4BFFB849  bl 0x8304c630
	ctx.lr = 0x83050DEC;
	sub_8304C630(ctx, base);
	// 83050DEC: 48000008  b 0x83050df4
	pc = 0x83050DF4; continue 'dispatch;
	// 83050DF0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83050DF4: 907E000C  stw r3, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 83050DF8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83050DFC: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83050E00: 4BFC74D1  bl 0x830182d0
	ctx.lr = 0x83050E04;
	sub_830182D0(ctx, base);
	// 83050E04: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83050E08: 907D0010  stw r3, 0x10(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 83050E0C: 48000008  b 0x83050e14
	pc = 0x83050E14; continue 'dispatch;
	// 83050E10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83050E14: 997A0000  stb r11, 0(r26)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 83050E18: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83050E1C: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 83050E20: 48157390  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83050E24(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83050E24 size=48
    let mut pc: u32 = 0x83050E24;
    'dispatch: loop {
        match pc {
            0x83050E24 => {
    //   block [0x83050E24..0x83050E54)
	// 83050E24: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 83050E28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83050E2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83050E30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83050E34: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 83050E38: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83050E3C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83050E40: 4BF874A1  bl 0x82fd82e0
	ctx.lr = 0x83050E44;
	sub_82FD82E0(ctx, base);
	// 83050E44: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83050E48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83050E4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83050E50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83050E54(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83050E54 size=48
    let mut pc: u32 = 0x83050E54;
    'dispatch: loop {
        match pc {
            0x83050E54 => {
    //   block [0x83050E54..0x83050E84)
	// 83050E54: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 83050E58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83050E5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83050E60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83050E64: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 83050E68: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83050E6C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83050E70: 4BF87471  bl 0x82fd82e0
	ctx.lr = 0x83050E74;
	sub_82FD82E0(ctx, base);
	// 83050E74: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83050E78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83050E7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83050E80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83050E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83050E88 size=8
    let mut pc: u32 = 0x83050E88;
    'dispatch: loop {
        match pc {
            0x83050E88 => {
    //   block [0x83050E88..0x83050E90)
	// 83050E88: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83050E8C: 821636A8  lwz r16, 0x36a8(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(13992 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83050E90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83050E90 size=204
    let mut pc: u32 = 0x83050E90;
    'dispatch: loop {
        match pc {
            0x83050E90 => {
    //   block [0x83050E90..0x83050F5C)
	// 83050E90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83050E94: 481572D1  bl 0x831a8164
	ctx.lr = 0x83050E98;
	sub_831A8130(ctx, base);
	// 83050E98: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 83050E9C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83050EA0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83050EA4: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 83050EA8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83050EAC: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 83050EB0: 7D3B4B78  mr r27, r9
	ctx.r[27].u64 = ctx.r[9].u64;
	// 83050EB4: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83050EB8: 93DF00A4  stw r30, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[30].u32 ) };
	// 83050EBC: 4BF873DD  bl 0x82fd8298
	ctx.lr = 0x83050EC0;
	sub_82FD8298(ctx, base);
	// 83050EC0: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83050EC4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83050EC8: 41820020  beq 0x83050ee8
	if ctx.cr[0].eq {
	pc = 0x83050EE8; continue 'dispatch;
	}
	// 83050ECC: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 83050ED0: 80FE0004  lwz r7, 4(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83050ED4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83050ED8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83050EDC: 48013915  bl 0x830647f0
	ctx.lr = 0x83050EE0;
	sub_830647F0(ctx, base);
	// 83050EE0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83050EE4: 48000008  b 0x83050eec
	pc = 0x83050EEC; continue 'dispatch;
	// 83050EE8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83050EEC: 576B063F  clrlwi. r11, r27, 0x18
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83050EF0: 41820050  beq 0x83050f40
	if ctx.cr[0].eq {
	pc = 0x83050F40; continue 'dispatch;
	}
	// 83050EF4: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83050EF8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83050EFC: 409A0038  bne cr6, 0x83050f34
	if !ctx.cr[6].eq {
	pc = 0x83050F34; continue 'dispatch;
	}
	// 83050F00: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 83050F04: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83050F08: 4BF87391  bl 0x82fd8298
	ctx.lr = 0x83050F0C;
	sub_82FD8298(ctx, base);
	// 83050F0C: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83050F10: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83050F14: 41820018  beq 0x83050f2c
	if ctx.cr[0].eq {
	pc = 0x83050F2C; continue 'dispatch;
	}
	// 83050F18: 38A00080  li r5, 0x80
	ctx.r[5].s64 = 128;
	// 83050F1C: 80DE0004  lwz r6, 4(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83050F20: 3880001D  li r4, 0x1d
	ctx.r[4].s64 = 29;
	// 83050F24: 4BFFB70D  bl 0x8304c630
	ctx.lr = 0x83050F28;
	sub_8304C630(ctx, base);
	// 83050F28: 48000008  b 0x83050f30
	pc = 0x83050F30; continue 'dispatch;
	// 83050F2C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83050F30: 907E000C  stw r3, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 83050F34: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83050F38: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83050F3C: 4800000C  b 0x83050f48
	pc = 0x83050F48; continue 'dispatch;
	// 83050F40: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83050F44: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83050F48: 4BFC7389  bl 0x830182d0
	ctx.lr = 0x83050F4C;
	sub_830182D0(ctx, base);
	// 83050F4C: 907D0010  stw r3, 0x10(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 83050F50: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83050F54: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 83050F58: 4815725C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83050F5C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83050F5C size=48
    let mut pc: u32 = 0x83050F5C;
    'dispatch: loop {
        match pc {
            0x83050F5C => {
    //   block [0x83050F5C..0x83050F8C)
	// 83050F5C: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 83050F60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83050F64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83050F68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83050F6C: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 83050F70: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83050F74: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83050F78: 4BF87369  bl 0x82fd82e0
	ctx.lr = 0x83050F7C;
	sub_82FD82E0(ctx, base);
	// 83050F7C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83050F80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83050F84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83050F88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83050F8C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83050F8C size=48
    let mut pc: u32 = 0x83050F8C;
    'dispatch: loop {
        match pc {
            0x83050F8C => {
    //   block [0x83050F8C..0x83050FBC)
	// 83050F8C: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 83050F90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83050F94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83050F98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83050F9C: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 83050FA0: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83050FA4: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83050FA8: 4BF87339  bl 0x82fd82e0
	ctx.lr = 0x83050FAC;
	sub_82FD82E0(ctx, base);
	// 83050FAC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83050FB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83050FB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83050FB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83050FC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83050FC0 size=88
    let mut pc: u32 = 0x83050FC0;
    'dispatch: loop {
        match pc {
            0x83050FC0 => {
    //   block [0x83050FC0..0x83051018)
	// 83050FC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83050FC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83050FC8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83050FCC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83050FD0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83050FD4: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83050FD8: 4BFC9971  bl 0x8301a948
	ctx.lr = 0x83050FDC;
	sub_8301A948(ctx, base);
	// 83050FDC: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83050FE0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83050FE4: 41820008  beq 0x83050fec
	if ctx.cr[0].eq {
	pc = 0x83050FEC; continue 'dispatch;
	}
	// 83050FE8: 4BFC9961  bl 0x8301a948
	ctx.lr = 0x83050FEC;
	sub_8301A948(ctx, base);
	// 83050FEC: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83050FF0: 4BFC9959  bl 0x8301a948
	ctx.lr = 0x83050FF4;
	sub_8301A948(ctx, base);
	// 83050FF4: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83050FF8: 4BFC9951  bl 0x8301a948
	ctx.lr = 0x83050FFC;
	sub_8301A948(ctx, base);
	// 83050FFC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83051000: 997F001C  stb r11, 0x1c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u8 ) };
	// 83051004: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83051008: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8305100C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83051010: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83051014: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83051018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83051018 size=132
    let mut pc: u32 = 0x83051018;
    'dispatch: loop {
        match pc {
            0x83051018 => {
    //   block [0x83051018..0x8305109C)
	// 83051018: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8305101C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83051020: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83051024: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83051028: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8305102C: 3FE08339  lis r31, -0x7cc7
	ctx.r[31].s64 = -2093416448;
	// 83051030: 83DFB9C8  lwz r30, -0x4638(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-17976 as u32) ) } as u64;
	// 83051034: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83051038: 419A0014  beq cr6, 0x8305104c
	if ctx.cr[6].eq {
	pc = 0x8305104C; continue 'dispatch;
	}
	// 8305103C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83051040: 4BFCBD99  bl 0x8301cdd8
	ctx.lr = 0x83051044;
	sub_8301CDD8(ctx, base);
	// 83051044: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83051048: 4BF87299  bl 0x82fd82e0
	ctx.lr = 0x8305104C;
	sub_82FD82E0(ctx, base);
	// 8305104C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83051050: 917FB9C8  stw r11, -0x4638(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-17976 as u32), ctx.r[11].u32 ) };
	// 83051054: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83051058: 3BEBB9C0  addi r31, r11, -0x4640
	ctx.r[31].s64 = ctx.r[11].s64 + -17984;
	// 8305105C: 83DF0004  lwz r30, 4(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83051060: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83051064: 419A0014  beq cr6, 0x83051078
	if ctx.cr[6].eq {
	pc = 0x83051078; continue 'dispatch;
	}
	// 83051068: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8305106C: 4BFA471D  bl 0x82ff5788
	ctx.lr = 0x83051070;
	sub_82FF5788(ctx, base);
	// 83051070: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83051074: 4BF8726D  bl 0x82fd82e0
	ctx.lr = 0x83051078;
	sub_82FD82E0(ctx, base);
	// 83051078: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8305107C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83051080: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 83051084: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83051088: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8305108C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83051090: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83051094: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83051098: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830510A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830510A0 size=8
    let mut pc: u32 = 0x830510A0;
    'dispatch: loop {
        match pc {
            0x830510A0 => {
    //   block [0x830510A0..0x830510A8)
	// 830510A0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830510A4: 82163740  lwz r16, 0x3740(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(14144 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830510A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830510A8 size=660
    let mut pc: u32 = 0x830510A8;
    'dispatch: loop {
        match pc {
            0x830510A8 => {
    //   block [0x830510A8..0x8305133C)
	// 830510A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830510AC: 481570AD  bl 0x831a8158
	ctx.lr = 0x830510B0;
	sub_831A8130(ctx, base);
	// 830510B0: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 830510B4: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830510B8: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830510BC: 3B0BB9C0  addi r24, r11, -0x4640
	ctx.r[24].s64 = ctx.r[11].s64 + -17984;
	// 830510C0: 89780000  lbz r11, 0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 830510C4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830510C8: 4082026C  bne 0x83051334
	if !ctx.cr[0].eq {
	pc = 0x83051334; continue 'dispatch;
	}
	// 830510CC: 80980004  lwz r4, 4(r24)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(4 as u32) ) } as u64;
	// 830510D0: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 830510D4: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 830510D8: 409A0050  bne cr6, 0x83051128
	if !ctx.cr[6].eq {
	pc = 0x83051128; continue 'dispatch;
	}
	// 830510DC: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830510E0: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830510E4: 808BB7EC  lwz r4, -0x4814(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18452 as u32) ) } as u64;
	// 830510E8: 4BFA46F1  bl 0x82ff57d8
	ctx.lr = 0x830510EC;
	sub_82FF57D8(ctx, base);
	// 830510EC: 81780004  lwz r11, 4(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(4 as u32) ) } as u64;
	// 830510F0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830510F4: 409A0028  bne cr6, 0x8305111c
	if !ctx.cr[6].eq {
	pc = 0x8305111C; continue 'dispatch;
	}
	// 830510F8: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 830510FC: 4BF8714D  bl 0x82fd8248
	ctx.lr = 0x83051100;
	sub_82FD8248(ctx, base);
	// 83051100: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 83051104: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83051108: 4182000C  beq 0x83051114
	if ctx.cr[0].eq {
	pc = 0x83051114; continue 'dispatch;
	}
	// 8305110C: 4BFA463D  bl 0x82ff5748
	ctx.lr = 0x83051110;
	sub_82FF5748(ctx, base);
	// 83051110: 48000008  b 0x83051118
	pc = 0x83051118; continue 'dispatch;
	// 83051114: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 83051118: 90780004  stw r3, 4(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 8305111C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83051120: 4BFA46F1  bl 0x82ff5810
	ctx.lr = 0x83051124;
	sub_82FF5810(ctx, base);
	// 83051124: 80980004  lwz r4, 4(r24)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(4 as u32) ) } as u64;
	// 83051128: 387F0054  addi r3, r31, 0x54
	ctx.r[3].s64 = ctx.r[31].s64 + 84;
	// 8305112C: 4BFA46AD  bl 0x82ff57d8
	ctx.lr = 0x83051130;
	sub_82FF57D8(ctx, base);
	// 83051130: 89780000  lbz r11, 0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 83051134: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83051138: 408201F4  bne 0x8305132c
	if !ctx.cr[0].eq {
	pc = 0x8305132C; continue 'dispatch;
	}
	// 8305113C: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 83051140: 4BF87109  bl 0x82fd8248
	ctx.lr = 0x83051144;
	sub_82FD8248(ctx, base);
	// 83051144: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83051148: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8305114C: 3F608339  lis r27, -0x7cc7
	ctx.r[27].s64 = -2093416448;
	// 83051150: 4182001C  beq 0x8305116c
	if ctx.cr[0].eq {
	pc = 0x8305116C; continue 'dispatch;
	}
	// 83051154: 38A0000C  li r5, 0xc
	ctx.r[5].s64 = 12;
	// 83051158: 80DBB7E8  lwz r6, -0x4818(r27)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 8305115C: 3880000B  li r4, 0xb
	ctx.r[4].s64 = 11;
	// 83051160: 4BFFB4D1  bl 0x8304c630
	ctx.lr = 0x83051164;
	sub_8304C630(ctx, base);
	// 83051164: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83051168: 48000008  b 0x83051170
	pc = 0x83051170; continue 'dispatch;
	// 8305116C: 7F4BD378  mr r11, r26
	ctx.r[11].u64 = ctx.r[26].u64;
	// 83051170: 3F208339  lis r25, -0x7cc7
	ctx.r[25].s64 = -2093416448;
	// 83051174: 3860002C  li r3, 0x2c
	ctx.r[3].s64 = 44;
	// 83051178: 9179B9C8  stw r11, -0x4638(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(-17976 as u32), ctx.r[11].u32 ) };
	// 8305117C: 4BF870CD  bl 0x82fd8248
	ctx.lr = 0x83051180;
	sub_82FD8248(ctx, base);
	// 83051180: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83051184: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 83051188: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8305118C: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 83051190: 3B8B3FF0  addi r28, r11, 0x3ff0
	ctx.r[28].s64 = ctx.r[11].s64 + 16368;
	// 83051194: 41820034  beq 0x830511c8
	if ctx.cr[0].eq {
	pc = 0x830511C8; continue 'dispatch;
	}
	// 83051198: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8305119C: 80DBB7E8  lwz r6, -0x4818(r27)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 830511A0: 38A00026  li r5, 0x26
	ctx.r[5].s64 = 38;
	// 830511A4: 388B8168  addi r4, r11, -0x7e98
	ctx.r[4].s64 = ctx.r[11].s64 + -32408;
	// 830511A8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830511AC: 48002A05  bl 0x83053bb0
	ctx.lr = 0x830511B0;
	sub_83053BB0(ctx, base);
	// 830511B0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830511B4: 9BBE0028  stb r29, 0x28(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), ctx.r[29].u8 ) };
	// 830511B8: 939E0000  stw r28, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 830511BC: 9B5E0029  stb r26, 0x29(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(41 as u32), ctx.r[26].u8 ) };
	// 830511C0: 9BBE002A  stb r29, 0x2a(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(42 as u32), ctx.r[29].u8 ) };
	// 830511C4: 48000008  b 0x830511cc
	pc = 0x830511CC; continue 'dispatch;
	// 830511C8: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 830511CC: 8079B9C8  lwz r3, -0x4638(r25)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(-17976 as u32) ) } as u64;
	// 830511D0: 4BFFBAB9  bl 0x8304cc88
	ctx.lr = 0x830511D4;
	sub_8304CC88(ctx, base);
	// 830511D4: 3860002C  li r3, 0x2c
	ctx.r[3].s64 = 44;
	// 830511D8: 4BF87071  bl 0x82fd8248
	ctx.lr = 0x830511DC;
	sub_82FD8248(ctx, base);
	// 830511DC: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 830511E0: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 830511E4: 41820034  beq 0x83051218
	if ctx.cr[0].eq {
	pc = 0x83051218; continue 'dispatch;
	}
	// 830511E8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830511EC: 80DBB7E8  lwz r6, -0x4818(r27)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 830511F0: 38A0003C  li r5, 0x3c
	ctx.r[5].s64 = 60;
	// 830511F4: 388B8170  addi r4, r11, -0x7e90
	ctx.r[4].s64 = ctx.r[11].s64 + -32400;
	// 830511F8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830511FC: 480029B5  bl 0x83053bb0
	ctx.lr = 0x83051200;
	sub_83053BB0(ctx, base);
	// 83051200: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83051204: 9BBE0028  stb r29, 0x28(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), ctx.r[29].u8 ) };
	// 83051208: 939E0000  stw r28, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 8305120C: 9B5E0029  stb r26, 0x29(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(41 as u32), ctx.r[26].u8 ) };
	// 83051210: 9BBE002A  stb r29, 0x2a(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(42 as u32), ctx.r[29].u8 ) };
	// 83051214: 48000008  b 0x8305121c
	pc = 0x8305121C; continue 'dispatch;
	// 83051218: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8305121C: 8079B9C8  lwz r3, -0x4638(r25)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(-17976 as u32) ) } as u64;
	// 83051220: 4BFFBA69  bl 0x8304cc88
	ctx.lr = 0x83051224;
	sub_8304CC88(ctx, base);
	// 83051224: 3860002C  li r3, 0x2c
	ctx.r[3].s64 = 44;
	// 83051228: 4BF87021  bl 0x82fd8248
	ctx.lr = 0x8305122C;
	sub_82FD8248(ctx, base);
	// 8305122C: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83051230: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 83051234: 41820034  beq 0x83051268
	if ctx.cr[0].eq {
	pc = 0x83051268; continue 'dispatch;
	}
	// 83051238: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8305123C: 80DBB7E8  lwz r6, -0x4818(r27)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83051240: 38A0003E  li r5, 0x3e
	ctx.r[5].s64 = 62;
	// 83051244: 388B8178  addi r4, r11, -0x7e88
	ctx.r[4].s64 = ctx.r[11].s64 + -32392;
	// 83051248: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8305124C: 48002965  bl 0x83053bb0
	ctx.lr = 0x83051250;
	sub_83053BB0(ctx, base);
	// 83051250: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83051254: 9BBE0028  stb r29, 0x28(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), ctx.r[29].u8 ) };
	// 83051258: 939E0000  stw r28, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 8305125C: 9B5E0029  stb r26, 0x29(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(41 as u32), ctx.r[26].u8 ) };
	// 83051260: 9BBE002A  stb r29, 0x2a(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(42 as u32), ctx.r[29].u8 ) };
	// 83051264: 48000008  b 0x8305126c
	pc = 0x8305126C; continue 'dispatch;
	// 83051268: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8305126C: 8079B9C8  lwz r3, -0x4638(r25)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(-17976 as u32) ) } as u64;
	// 83051270: 4BFFBA19  bl 0x8304cc88
	ctx.lr = 0x83051274;
	sub_8304CC88(ctx, base);
	// 83051274: 3860002C  li r3, 0x2c
	ctx.r[3].s64 = 44;
	// 83051278: 4BF86FD1  bl 0x82fd8248
	ctx.lr = 0x8305127C;
	sub_82FD8248(ctx, base);
	// 8305127C: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83051280: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 83051284: 41820034  beq 0x830512b8
	if ctx.cr[0].eq {
	pc = 0x830512B8; continue 'dispatch;
	}
	// 83051288: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8305128C: 80DBB7E8  lwz r6, -0x4818(r27)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83051290: 38A00022  li r5, 0x22
	ctx.r[5].s64 = 34;
	// 83051294: 388B8180  addi r4, r11, -0x7e80
	ctx.r[4].s64 = ctx.r[11].s64 + -32384;
	// 83051298: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8305129C: 48002915  bl 0x83053bb0
	ctx.lr = 0x830512A0;
	sub_83053BB0(ctx, base);
	// 830512A0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830512A4: 9BBE0028  stb r29, 0x28(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), ctx.r[29].u8 ) };
	// 830512A8: 939E0000  stw r28, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 830512AC: 9B5E0029  stb r26, 0x29(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(41 as u32), ctx.r[26].u8 ) };
	// 830512B0: 9BBE002A  stb r29, 0x2a(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(42 as u32), ctx.r[29].u8 ) };
	// 830512B4: 48000008  b 0x830512bc
	pc = 0x830512BC; continue 'dispatch;
	// 830512B8: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 830512BC: 8079B9C8  lwz r3, -0x4638(r25)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(-17976 as u32) ) } as u64;
	// 830512C0: 4BFFB9C9  bl 0x8304cc88
	ctx.lr = 0x830512C4;
	sub_8304CC88(ctx, base);
	// 830512C4: 3860002C  li r3, 0x2c
	ctx.r[3].s64 = 44;
	// 830512C8: 4BF86F81  bl 0x82fd8248
	ctx.lr = 0x830512CC;
	sub_82FD8248(ctx, base);
	// 830512CC: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 830512D0: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 830512D4: 41820034  beq 0x83051308
	if ctx.cr[0].eq {
	pc = 0x83051308; continue 'dispatch;
	}
	// 830512D8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830512DC: 80DBB7E8  lwz r6, -0x4818(r27)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 830512E0: 38A00027  li r5, 0x27
	ctx.r[5].s64 = 39;
	// 830512E4: 388B818C  addi r4, r11, -0x7e74
	ctx.r[4].s64 = ctx.r[11].s64 + -32372;
	// 830512E8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830512EC: 480028C5  bl 0x83053bb0
	ctx.lr = 0x830512F0;
	sub_83053BB0(ctx, base);
	// 830512F0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830512F4: 9BBE0028  stb r29, 0x28(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), ctx.r[29].u8 ) };
	// 830512F8: 939E0000  stw r28, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 830512FC: 9B5E0029  stb r26, 0x29(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(41 as u32), ctx.r[26].u8 ) };
	// 83051300: 9BBE002A  stb r29, 0x2a(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(42 as u32), ctx.r[29].u8 ) };
	// 83051304: 48000008  b 0x8305130c
	pc = 0x8305130C; continue 'dispatch;
	// 83051308: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8305130C: 8079B9C8  lwz r3, -0x4638(r25)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(-17976 as u32) ) } as u64;
	// 83051310: 4BFFB979  bl 0x8304cc88
	ctx.lr = 0x83051314;
	sub_8304CC88(ctx, base);
	// 83051314: 3D608305  lis r11, -0x7cfb
	ctx.r[11].s64 = -2096824320;
	// 83051318: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 8305131C: 388B1018  addi r4, r11, 0x1018
	ctx.r[4].s64 = ctx.r[11].s64 + 4120;
	// 83051320: 386AB9CC  addi r3, r10, -0x4634
	ctx.r[3].s64 = ctx.r[10].s64 + -17972;
	// 83051324: 4BFA6815  bl 0x82ff7b38
	ctx.lr = 0x83051328;
	sub_82FF7B38(ctx, base);
	// 83051328: 9BB80000  stb r29, 0(r24)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[29].u8 ) };
	// 8305132C: 387F0054  addi r3, r31, 0x54
	ctx.r[3].s64 = ctx.r[31].s64 + 84;
	// 83051330: 4BFA44E1  bl 0x82ff5810
	ctx.lr = 0x83051334;
	sub_82FF5810(ctx, base);
	// 83051334: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 83051338: 48156E70  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8305133C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8305133C size=40
    let mut pc: u32 = 0x8305133C;
    'dispatch: loop {
        match pc {
            0x8305133C => {
    //   block [0x8305133C..0x83051364)
	// 8305133C: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 83051340: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83051344: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83051348: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8305134C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83051350: 4BFA44C1  bl 0x82ff5810
	ctx.lr = 0x83051354;
	sub_82FF5810(ctx, base);
	// 83051354: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83051358: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8305135C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83051360: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83051364(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83051364 size=40
    let mut pc: u32 = 0x83051364;
    'dispatch: loop {
        match pc {
            0x83051364 => {
    //   block [0x83051364..0x8305138C)
	// 83051364: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 83051368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8305136C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83051370: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83051374: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 83051378: 4BF86F69  bl 0x82fd82e0
	ctx.lr = 0x8305137C;
	sub_82FD82E0(ctx, base);
	// 8305137C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83051380: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83051384: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83051388: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8305138C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8305138C size=40
    let mut pc: u32 = 0x8305138C;
    'dispatch: loop {
        match pc {
            0x8305138C => {
    //   block [0x8305138C..0x830513B4)
	// 8305138C: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 83051390: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83051394: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83051398: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8305139C: 387F0054  addi r3, r31, 0x54
	ctx.r[3].s64 = ctx.r[31].s64 + 84;
	// 830513A0: 4BFA4471  bl 0x82ff5810
	ctx.lr = 0x830513A4;
	sub_82FF5810(ctx, base);
	// 830513A4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830513A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830513AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830513B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830513B4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830513B4 size=40
    let mut pc: u32 = 0x830513B4;
    'dispatch: loop {
        match pc {
            0x830513B4 => {
    //   block [0x830513B4..0x830513DC)
	// 830513B4: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 830513B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830513BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830513C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830513C4: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830513C8: 4BF86F19  bl 0x82fd82e0
	ctx.lr = 0x830513CC;
	sub_82FD82E0(ctx, base);
	// 830513CC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830513D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830513D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830513D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830513DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830513DC size=40
    let mut pc: u32 = 0x830513DC;
    'dispatch: loop {
        match pc {
            0x830513DC => {
    //   block [0x830513DC..0x83051404)
	// 830513DC: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 830513E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830513E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830513E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830513EC: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830513F0: 4BF86EF1  bl 0x82fd82e0
	ctx.lr = 0x830513F4;
	sub_82FD82E0(ctx, base);
	// 830513F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830513F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830513FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83051400: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83051404(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83051404 size=40
    let mut pc: u32 = 0x83051404;
    'dispatch: loop {
        match pc {
            0x83051404 => {
    //   block [0x83051404..0x8305142C)
	// 83051404: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 83051408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8305140C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83051410: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83051414: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83051418: 4BF86EC9  bl 0x82fd82e0
	ctx.lr = 0x8305141C;
	sub_82FD82E0(ctx, base);
	// 8305141C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83051420: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83051424: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83051428: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8305142C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8305142C size=40
    let mut pc: u32 = 0x8305142C;
    'dispatch: loop {
        match pc {
            0x8305142C => {
    //   block [0x8305142C..0x83051454)
	// 8305142C: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 83051430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83051434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83051438: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8305143C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83051440: 4BF86EA1  bl 0x82fd82e0
	ctx.lr = 0x83051444;
	sub_82FD82E0(ctx, base);
	// 83051444: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83051448: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8305144C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83051450: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83051454(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83051454 size=40
    let mut pc: u32 = 0x83051454;
    'dispatch: loop {
        match pc {
            0x83051454 => {
    //   block [0x83051454..0x8305147C)
	// 83051454: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 83051458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8305145C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83051460: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83051464: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83051468: 4BF86E79  bl 0x82fd82e0
	ctx.lr = 0x8305146C;
	sub_82FD82E0(ctx, base);
	// 8305146C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83051470: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83051474: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83051478: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8305147C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8305147C size=40
    let mut pc: u32 = 0x8305147C;
    'dispatch: loop {
        match pc {
            0x8305147C => {
    //   block [0x8305147C..0x830514A4)
	// 8305147C: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 83051480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83051484: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83051488: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8305148C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83051490: 4BF86E51  bl 0x82fd82e0
	ctx.lr = 0x83051494;
	sub_82FD82E0(ctx, base);
	// 83051494: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83051498: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8305149C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830514A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830514A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830514A8 size=8
    let mut pc: u32 = 0x830514A8;
    'dispatch: loop {
        match pc {
            0x830514A8 => {
    //   block [0x830514A8..0x830514B0)
	// 830514A8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830514AC: 82163878  lwz r16, 0x3878(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(14456 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830514B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830514B0 size=304
    let mut pc: u32 = 0x830514B0;
    'dispatch: loop {
        match pc {
            0x830514B0 => {
    //   block [0x830514B0..0x830515E0)
	// 830514B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830514B4: 48156CB9  bl 0x831a816c
	ctx.lr = 0x830514B8;
	sub_831A8130(ctx, base);
	// 830514B8: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 830514BC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830514C0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830514C4: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 830514C8: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 830514CC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 830514D0: 909E0004  stw r4, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 830514D4: 396B37F8  addi r11, r11, 0x37f8
	ctx.r[11].s64 = ctx.r[11].s64 + 14328;
	// 830514D8: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 830514DC: 93BE000C  stw r29, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 830514E0: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830514E4: 93BE0018  stw r29, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[29].u32 ) };
	// 830514E8: 9BBE001C  stb r29, 0x1c(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[29].u8 ) };
	// 830514EC: 93BE0008  stw r29, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 830514F0: 93BE0010  stw r29, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[29].u32 ) };
	// 830514F4: 93BE0014  stw r29, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[29].u32 ) };
	// 830514F8: 93BE0020  stw r29, 0x20(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[29].u32 ) };
	// 830514FC: 4BF86D9D  bl 0x82fd8298
	ctx.lr = 0x83051500;
	sub_82FD8298(ctx, base);
	// 83051500: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83051504: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83051508: 4182001C  beq 0x83051524
	if ctx.cr[0].eq {
	pc = 0x83051524; continue 'dispatch;
	}
	// 8305150C: 38A00080  li r5, 0x80
	ctx.r[5].s64 = 128;
	// 83051510: 80DE0004  lwz r6, 4(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83051514: 3880006D  li r4, 0x6d
	ctx.r[4].s64 = 109;
	// 83051518: 4BFFB119  bl 0x8304c630
	ctx.lr = 0x8305151C;
	sub_8304C630(ctx, base);
	// 8305151C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83051520: 48000008  b 0x83051528
	pc = 0x83051528; continue 'dispatch;
	// 83051524: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 83051528: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 8305152C: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83051530: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83051534: 4BF86D65  bl 0x82fd8298
	ctx.lr = 0x83051538;
	sub_82FD8298(ctx, base);
	// 83051538: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8305153C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83051540: 4182001C  beq 0x8305155c
	if ctx.cr[0].eq {
	pc = 0x8305155C; continue 'dispatch;
	}
	// 83051544: 38A00080  li r5, 0x80
	ctx.r[5].s64 = 128;
	// 83051548: 80DE0004  lwz r6, 4(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8305154C: 3880006D  li r4, 0x6d
	ctx.r[4].s64 = 109;
	// 83051550: 4BFFB0E1  bl 0x8304c630
	ctx.lr = 0x83051554;
	sub_8304C630(ctx, base);
	// 83051554: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83051558: 48000008  b 0x83051560
	pc = 0x83051560; continue 'dispatch;
	// 8305155C: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 83051560: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 83051564: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83051568: 917E0010  stw r11, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8305156C: 4BF86D2D  bl 0x82fd8298
	ctx.lr = 0x83051570;
	sub_82FD8298(ctx, base);
	// 83051570: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83051574: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83051578: 4182001C  beq 0x83051594
	if ctx.cr[0].eq {
	pc = 0x83051594; continue 'dispatch;
	}
	// 8305157C: 38A00080  li r5, 0x80
	ctx.r[5].s64 = 128;
	// 83051580: 80DE0004  lwz r6, 4(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83051584: 3880006D  li r4, 0x6d
	ctx.r[4].s64 = 109;
	// 83051588: 4BFFB0A9  bl 0x8304c630
	ctx.lr = 0x8305158C;
	sub_8304C630(ctx, base);
	// 8305158C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83051590: 48000008  b 0x83051598
	pc = 0x83051598; continue 'dispatch;
	// 83051594: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 83051598: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 8305159C: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830515A0: 917E0014  stw r11, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 830515A4: 4BF86CF5  bl 0x82fd8298
	ctx.lr = 0x830515A8;
	sub_82FD8298(ctx, base);
	// 830515A8: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 830515AC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830515B0: 41820018  beq 0x830515c8
	if ctx.cr[0].eq {
	pc = 0x830515C8; continue 'dispatch;
	}
	// 830515B4: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830515B8: 80BE0004  lwz r5, 4(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830515BC: 388B815C  addi r4, r11, -0x7ea4
	ctx.r[4].s64 = ctx.r[11].s64 + -32420;
	// 830515C0: 48001669  bl 0x83052c28
	ctx.lr = 0x830515C4;
	sub_83052C28(ctx, base);
	// 830515C4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830515C8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830515CC: 93BE0020  stw r29, 0x20(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[29].u32 ) };
	// 830515D0: 4BFFFAD9  bl 0x830510a8
	ctx.lr = 0x830515D4;
	sub_830510A8(ctx, base);
	// 830515D4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830515D8: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 830515DC: 48156BE0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830515E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830515E0 size=40
    let mut pc: u32 = 0x830515E0;
    'dispatch: loop {
        match pc {
            0x830515E0 => {
    //   block [0x830515E0..0x83051608)
	// 830515E0: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830515E4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830515E8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830515EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830515F0: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 830515F4: 4BFFB16D  bl 0x8304c760
	ctx.lr = 0x830515F8;
	sub_8304C760(ctx, base);
	// 830515F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830515FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83051600: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83051604: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83051608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83051608 size=48
    let mut pc: u32 = 0x83051608;
    'dispatch: loop {
        match pc {
            0x83051608 => {
    //   block [0x83051608..0x83051638)
	// 83051608: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 8305160C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83051610: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83051614: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83051618: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 8305161C: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83051620: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83051624: 4BF86CBD  bl 0x82fd82e0
	ctx.lr = 0x83051628;
	sub_82FD82E0(ctx, base);
	// 83051628: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8305162C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83051630: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83051634: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83051638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83051638 size=48
    let mut pc: u32 = 0x83051638;
    'dispatch: loop {
        match pc {
            0x83051638 => {
    //   block [0x83051638..0x83051668)
	// 83051638: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 8305163C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83051640: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83051644: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83051648: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 8305164C: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83051650: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83051654: 4BF86C8D  bl 0x82fd82e0
	ctx.lr = 0x83051658;
	sub_82FD82E0(ctx, base);
	// 83051658: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8305165C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83051660: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83051664: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83051668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83051668 size=48
    let mut pc: u32 = 0x83051668;
    'dispatch: loop {
        match pc {
            0x83051668 => {
    //   block [0x83051668..0x83051698)
	// 83051668: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 8305166C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83051670: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83051674: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83051678: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 8305167C: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83051680: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83051684: 4BF86C5D  bl 0x82fd82e0
	ctx.lr = 0x83051688;
	sub_82FD82E0(ctx, base);
	// 83051688: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8305168C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83051690: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83051694: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83051698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83051698 size=48
    let mut pc: u32 = 0x83051698;
    'dispatch: loop {
        match pc {
            0x83051698 => {
    //   block [0x83051698..0x830516C8)
	// 83051698: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 8305169C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830516A0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830516A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830516A8: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 830516AC: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830516B0: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830516B4: 4BF86C2D  bl 0x82fd82e0
	ctx.lr = 0x830516B8;
	sub_82FD82E0(ctx, base);
	// 830516B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830516BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830516C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830516C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830516C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830516C8 size=12
    let mut pc: u32 = 0x830516C8;
    'dispatch: loop {
        match pc {
            0x830516C8 => {
    //   block [0x830516C8..0x830516D4)
	// 830516C8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830516CC: 386B8158  addi r3, r11, -0x7ea8
	ctx.r[3].s64 = ctx.r[11].s64 + -32424;
	// 830516D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830516D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830516D8 size=8
    let mut pc: u32 = 0x830516D8;
    'dispatch: loop {
        match pc {
            0x830516D8 => {
    //   block [0x830516D8..0x830516E0)
	// 830516D8: 8863001C  lbz r3, 0x1c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 830516DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830516E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830516E0 size=8
    let mut pc: u32 = 0x830516E0;
    'dispatch: loop {
        match pc {
            0x830516E0 => {
    //   block [0x830516E0..0x830516E8)
	// 830516E0: 9883001C  stb r4, 0x1c(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[4].u8 ) };
	// 830516E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830516E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830516E8 size=104
    let mut pc: u32 = 0x830516E8;
    'dispatch: loop {
        match pc {
            0x830516E8 => {
    //   block [0x830516E8..0x83051750)
	// 830516E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830516EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830516F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830516F4: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 830516F8: 419A002C  beq cr6, 0x83051724
	if ctx.cr[6].eq {
	pc = 0x83051724; continue 'dispatch;
	}
	// 830516FC: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 83051700: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83051704: 41990020  bgt cr6, 0x83051724
	if ctx.cr[6].gt {
	pc = 0x83051724; continue 'dispatch;
	}
	// 83051708: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8305170C: 548A103A  slwi r10, r4, 2
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83051710: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83051714: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83051718: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8305171C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83051720: 4E800020  blr
	return;
	// 83051724: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83051728: 80E30000  lwz r7, 0(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8305172C: 38C00043  li r6, 0x43
	ctx.r[6].s64 = 67;
	// 83051730: 388B23A0  addi r4, r11, 0x23a0
	ctx.r[4].s64 = ctx.r[11].s64 + 9120;
	// 83051734: 38A000D5  li r5, 0xd5
	ctx.r[5].s64 = 213;
	// 83051738: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8305173C: 4BF7F58D  bl 0x82fd0cc8
	ctx.lr = 0x83051740;
	sub_82FD0CC8(ctx, base);
	// 83051740: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83051744: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83051748: 388BC3FC  addi r4, r11, -0x3c04
	ctx.r[4].s64 = ctx.r[11].s64 + -15364;
	// 8305174C: 4815F4DD  bl 0x831b0c28
	ctx.lr = 0x83051750;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83051750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83051750 size=8
    let mut pc: u32 = 0x83051750;
    'dispatch: loop {
        match pc {
            0x83051750 => {
    //   block [0x83051750..0x83051758)
	// 83051750: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 83051754: 4BFFFF94  b 0x830516e8
	sub_830516E8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83051758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83051758 size=80
    let mut pc: u32 = 0x83051758;
    'dispatch: loop {
        match pc {
            0x83051758 => {
    //   block [0x83051758..0x830517A8)
	// 83051758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8305175C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83051760: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83051764: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 83051768: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8305176C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83051770: 4BFC6689  bl 0x83017df8
	ctx.lr = 0x83051774;
	sub_83017DF8(ctx, base);
	// 83051774: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83051778: 41820010  beq 0x83051788
	if ctx.cr[0].eq {
	pc = 0x83051788; continue 'dispatch;
	}
	// 8305177C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83051780: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83051784: 40820010  bne 0x83051794
	if !ctx.cr[0].eq {
	pc = 0x83051794; continue 'dispatch;
	}
	// 83051788: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 8305178C: 806B9770  lwz r3, -0x6890(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26768 as u32) ) } as u64;
	// 83051790: 48000008  b 0x83051798
	pc = 0x83051798; continue 'dispatch;
	// 83051794: 806B0010  lwz r3, 0x10(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83051798: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8305179C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830517A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830517A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830517A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830517A8 size=140
    let mut pc: u32 = 0x830517A8;
    'dispatch: loop {
        match pc {
            0x830517A8 => {
    //   block [0x830517A8..0x83051834)
	// 830517A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830517AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830517B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830517B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830517B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830517BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830517C0: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 830517C4: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 830517C8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830517CC: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830517D0: 4BFC6629  bl 0x83017df8
	ctx.lr = 0x830517D4;
	sub_83017DF8(ctx, base);
	// 830517D4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830517D8: 4082000C  bne 0x830517e4
	if !ctx.cr[0].eq {
	pc = 0x830517E4; continue 'dispatch;
	}
	// 830517DC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830517E0: 48000010  b 0x830517f0
	pc = 0x830517F0; continue 'dispatch;
	// 830517E4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830517E8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830517EC: 4082002C  bne 0x83051818
	if !ctx.cr[0].eq {
	pc = 0x83051818; continue 'dispatch;
	}
	// 830517F0: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 830517F4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830517F8: 41820020  beq 0x83051818
	if ctx.cr[0].eq {
	pc = 0x83051818; continue 'dispatch;
	}
	// 830517FC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83051800: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83051804: 4BFC65F5  bl 0x83017df8
	ctx.lr = 0x83051808;
	sub_83017DF8(ctx, base);
	// 83051808: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8305180C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83051810: 41820008  beq 0x83051818
	if ctx.cr[0].eq {
	pc = 0x83051818; continue 'dispatch;
	}
	// 83051814: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83051818: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8305181C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83051820: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83051824: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83051828: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8305182C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83051830: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83051838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83051838 size=8
    let mut pc: u32 = 0x83051838;
    'dispatch: loop {
        match pc {
            0x83051838 => {
    //   block [0x83051838..0x83051840)
	// 83051838: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8305183C: 821638F0  lwz r16, 0x38f0(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(14576 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83051840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83051840 size=132
    let mut pc: u32 = 0x83051840;
    'dispatch: loop {
        match pc {
            0x83051840 => {
    //   block [0x83051840..0x830518C4)
	// 83051840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83051844: 48156929  bl 0x831a816c
	ctx.lr = 0x83051848;
	sub_831A8130(ctx, base);
	// 83051848: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 8305184C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83051850: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83051854: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83051858: 54AB063F  clrlwi. r11, r5, 0x18
	ctx.r[11].u64 = ctx.r[5].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8305185C: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 83051860: 41820050  beq 0x830518b0
	if ctx.cr[0].eq {
	pc = 0x830518B0; continue 'dispatch;
	}
	// 83051864: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83051868: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8305186C: 409A0038  bne cr6, 0x830518a4
	if !ctx.cr[6].eq {
	pc = 0x830518A4; continue 'dispatch;
	}
	// 83051870: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 83051874: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83051878: 4BF86A21  bl 0x82fd8298
	ctx.lr = 0x8305187C;
	sub_82FD8298(ctx, base);
	// 8305187C: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83051880: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83051884: 41820018  beq 0x8305189c
	if ctx.cr[0].eq {
	pc = 0x8305189C; continue 'dispatch;
	}
	// 83051888: 38A00080  li r5, 0x80
	ctx.r[5].s64 = 128;
	// 8305188C: 80DE0004  lwz r6, 4(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83051890: 3880001D  li r4, 0x1d
	ctx.r[4].s64 = 29;
	// 83051894: 4BFFAD9D  bl 0x8304c630
	ctx.lr = 0x83051898;
	sub_8304C630(ctx, base);
	// 83051898: 48000008  b 0x830518a0
	pc = 0x830518A0; continue 'dispatch;
	// 8305189C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830518A0: 907E000C  stw r3, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 830518A4: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 830518A8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830518AC: 4800000C  b 0x830518b8
	pc = 0x830518B8; continue 'dispatch;
	// 830518B0: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 830518B4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830518B8: 4BFC6A19  bl 0x830182d0
	ctx.lr = 0x830518BC;
	sub_830182D0(ctx, base);
	// 830518BC: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 830518C0: 481568FC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830518C4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830518C4 size=48
    let mut pc: u32 = 0x830518C4;
    'dispatch: loop {
        match pc {
            0x830518C4 => {
    //   block [0x830518C4..0x830518F4)
	// 830518C4: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830518C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830518CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830518D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830518D4: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 830518D8: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830518DC: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830518E0: 4BF86A01  bl 0x82fd82e0
	ctx.lr = 0x830518E4;
	sub_82FD82E0(ctx, base);
	// 830518E4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830518E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830518EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830518F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830518F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830518F8 size=8
    let mut pc: u32 = 0x830518F8;
    'dispatch: loop {
        match pc {
            0x830518F8 => {
    //   block [0x830518F8..0x83051900)
	// 830518F8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830518FC: 82163938  lwz r16, 0x3938(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(14648 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83051900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83051900 size=200
    let mut pc: u32 = 0x83051900;
    'dispatch: loop {
        match pc {
            0x83051900 => {
    //   block [0x83051900..0x830519C8)
	// 83051900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83051904: 48156869  bl 0x831a816c
	ctx.lr = 0x83051908;
	sub_831A8130(ctx, base);
	// 83051908: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 8305190C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83051910: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83051914: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83051918: 396B37F8  addi r11, r11, 0x37f8
	ctx.r[11].s64 = ctx.r[11].s64 + 14328;
	// 8305191C: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 83051920: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83051924: 83BE0008  lwz r29, 8(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83051928: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8305192C: 41820014  beq 0x83051940
	if ctx.cr[0].eq {
	pc = 0x83051940; continue 'dispatch;
	}
	// 83051930: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83051934: 4BFCB4A5  bl 0x8301cdd8
	ctx.lr = 0x83051938;
	sub_8301CDD8(ctx, base);
	// 83051938: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8305193C: 4BF869A5  bl 0x82fd82e0
	ctx.lr = 0x83051940;
	sub_82FD82E0(ctx, base);
	// 83051940: 83BE000C  lwz r29, 0xc(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83051944: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83051948: 41820014  beq 0x8305195c
	if ctx.cr[0].eq {
	pc = 0x8305195C; continue 'dispatch;
	}
	// 8305194C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83051950: 4BFCB489  bl 0x8301cdd8
	ctx.lr = 0x83051954;
	sub_8301CDD8(ctx, base);
	// 83051954: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83051958: 4BF86989  bl 0x82fd82e0
	ctx.lr = 0x8305195C;
	sub_82FD82E0(ctx, base);
	// 8305195C: 83BE0010  lwz r29, 0x10(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83051960: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83051964: 41820014  beq 0x83051978
	if ctx.cr[0].eq {
	pc = 0x83051978; continue 'dispatch;
	}
	// 83051968: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8305196C: 4BFCB46D  bl 0x8301cdd8
	ctx.lr = 0x83051970;
	sub_8301CDD8(ctx, base);
	// 83051970: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83051974: 4BF8696D  bl 0x82fd82e0
	ctx.lr = 0x83051978;
	sub_82FD82E0(ctx, base);
	// 83051978: 83BE0014  lwz r29, 0x14(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 8305197C: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83051980: 41820014  beq 0x83051994
	if ctx.cr[0].eq {
	pc = 0x83051994; continue 'dispatch;
	}
	// 83051984: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83051988: 4BFCB451  bl 0x8301cdd8
	ctx.lr = 0x8305198C;
	sub_8301CDD8(ctx, base);
	// 8305198C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83051990: 4BF86951  bl 0x82fd82e0
	ctx.lr = 0x83051994;
	sub_82FD82E0(ctx, base);
	// 83051994: 807E0020  lwz r3, 0x20(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 83051998: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8305199C: 41820018  beq 0x830519b4
	if ctx.cr[0].eq {
	pc = 0x830519B4; continue 'dispatch;
	}
	// 830519A0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830519A4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830519A8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830519AC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830519B0: 4E800421  bctrl
	ctx.lr = 0x830519B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830519B4: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830519B8: 396BA93C  addi r11, r11, -0x56c4
	ctx.r[11].s64 = ctx.r[11].s64 + -22212;
	// 830519BC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830519C0: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 830519C4: 481567F8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830519C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830519C8 size=40
    let mut pc: u32 = 0x830519C8;
    'dispatch: loop {
        match pc {
            0x830519C8 => {
    //   block [0x830519C8..0x830519F0)
	// 830519C8: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 830519CC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830519D0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830519D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830519D8: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 830519DC: 4BFFAD85  bl 0x8304c760
	ctx.lr = 0x830519E0;
	sub_8304C760(ctx, base);
	// 830519E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830519E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830519E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830519EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830519F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830519F0 size=8
    let mut pc: u32 = 0x830519F0;
    'dispatch: loop {
        match pc {
            0x830519F0 => {
    //   block [0x830519F0..0x830519F8)
	// 830519F0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830519F4: 82163970  lwz r16, 0x3970(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(14704 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830519F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830519F8 size=96
    let mut pc: u32 = 0x830519F8;
    'dispatch: loop {
        match pc {
            0x830519F8 => {
    //   block [0x830519F8..0x83051A58)
	// 830519F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830519FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83051A00: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83051A04: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83051A08: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 83051A0C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83051A10: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83051A14: 38600024  li r3, 0x24
	ctx.r[3].s64 = 36;
	// 83051A18: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83051A1C: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 83051A20: 4BF86879  bl 0x82fd8298
	ctx.lr = 0x83051A24;
	sub_82FD8298(ctx, base);
	// 83051A24: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83051A28: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83051A2C: 41820010  beq 0x83051a3c
	if ctx.cr[0].eq {
	pc = 0x83051A3C; continue 'dispatch;
	}
	// 83051A30: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83051A34: 4BFFFA7D  bl 0x830514b0
	ctx.lr = 0x83051A38;
	sub_830514B0(ctx, base);
	// 83051A38: 48000008  b 0x83051a40
	pc = 0x83051A40; continue 'dispatch;
	// 83051A3C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83051A40: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 83051A44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83051A48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83051A4C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83051A50: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83051A54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83051A58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83051A58 size=44
    let mut pc: u32 = 0x83051A58;
    'dispatch: loop {
        match pc {
            0x83051A58 => {
    //   block [0x83051A58..0x83051A84)
	// 83051A58: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 83051A5C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83051A60: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83051A64: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83051A68: 809F0084  lwz r4, 0x84(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83051A6C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83051A70: 4BF86871  bl 0x82fd82e0
	ctx.lr = 0x83051A74;
	sub_82FD82E0(ctx, base);
	// 83051A74: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83051A78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83051A7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83051A80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83051A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83051A88 size=76
    let mut pc: u32 = 0x83051A88;
    'dispatch: loop {
        match pc {
            0x83051A88 => {
    //   block [0x83051A88..0x83051AD4)
	// 83051A88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83051A8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83051A90: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83051A94: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83051A98: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83051A9C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83051AA0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83051AA4: 4BFFFE5D  bl 0x83051900
	ctx.lr = 0x83051AA8;
	sub_83051900(ctx, base);
	// 83051AA8: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83051AAC: 4182000C  beq 0x83051ab8
	if ctx.cr[0].eq {
	pc = 0x83051AB8; continue 'dispatch;
	}
	// 83051AB0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83051AB4: 4BF8682D  bl 0x82fd82e0
	ctx.lr = 0x83051AB8;
	sub_82FD82E0(ctx, base);
	// 83051AB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83051ABC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83051AC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83051AC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83051AC8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83051ACC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83051AD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83051AD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83051AD8 size=124
    let mut pc: u32 = 0x83051AD8;
    'dispatch: loop {
        match pc {
            0x83051AD8 => {
    //   block [0x83051AD8..0x83051B54)
	// 83051AD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83051ADC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83051AE0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83051AE4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83051AE8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83051AEC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83051AF0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83051AF4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83051AF8: 419A0044  beq cr6, 0x83051b3c
	if ctx.cr[6].eq {
	pc = 0x83051B3C; continue 'dispatch;
	}
	// 83051AFC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83051B00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83051B04: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83051B08: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83051B0C: 4E800421  bctrl
	ctx.lr = 0x83051B10;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83051B10: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83051B14: 409A0028  bne cr6, 0x83051b3c
	if !ctx.cr[6].eq {
	pc = 0x83051B3C; continue 'dispatch;
	}
	// 83051B18: 807E0048  lwz r3, 0x48(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(72 as u32) ) } as u64;
	// 83051B1C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83051B20: 41820018  beq 0x83051b38
	if ctx.cr[0].eq {
	pc = 0x83051B38; continue 'dispatch;
	}
	// 83051B24: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83051B28: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83051B2C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83051B30: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83051B34: 4E800421  bctrl
	ctx.lr = 0x83051B38;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83051B38: 93FE0048  stw r31, 0x48(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(72 as u32), ctx.r[31].u32 ) };
	// 83051B3C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83051B40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83051B44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83051B48: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83051B4C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83051B50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83051B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83051B58 size=12
    let mut pc: u32 = 0x83051B58;
    'dispatch: loop {
        match pc {
            0x83051B58 => {
    //   block [0x83051B58..0x83051B64)
	// 83051B58: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 83051B5C: 386B3404  addi r3, r11, 0x3404
	ctx.r[3].s64 = ctx.r[11].s64 + 13316;
	// 83051B60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83051B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83051B68 size=492
    let mut pc: u32 = 0x83051B68;
    'dispatch: loop {
        match pc {
            0x83051B68 => {
    //   block [0x83051B68..0x83051D54)
	// 83051B68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83051B6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83051B70: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83051B74: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83051B78: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83051B7C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83051B80: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83051B84: 48075F5D  bl 0x830c7ae0
	ctx.lr = 0x83051B88;
	sub_830C7AE0(ctx, base);
	// 83051B88: 387E003C  addi r3, r30, 0x3c
	ctx.r[3].s64 = ctx.r[30].s64 + 60;
	// 83051B8C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83051B90: A97F0000  lha r11, 0(r31)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as i16) as i64;
	// 83051B94: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83051B98: 556BDFFF  rlwinm. r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83051B9C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83051BA0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83051BA4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83051BA8: 41820098  beq 0x83051c40
	if ctx.cr[0].eq {
	pc = 0x83051C40; continue 'dispatch;
	}
	// 83051BAC: 4E800421  bctrl
	ctx.lr = 0x83051BB0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83051BB0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83051BB4: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83051BB8: 4BFFE699  bl 0x83050250
	ctx.lr = 0x83051BBC;
	sub_83050250(ctx, base);
	// 83051BBC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83051BC0: 807E0010  lwz r3, 0x10(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83051BC4: 4BFFE68D  bl 0x83050250
	ctx.lr = 0x83051BC8;
	sub_83050250(ctx, base);
	// 83051BC8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83051BCC: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 83051BD0: 4BFFD4D1  bl 0x8304f0a0
	ctx.lr = 0x83051BD4;
	sub_8304F0A0(ctx, base);
	// 83051BD4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83051BD8: 807E0018  lwz r3, 0x18(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 83051BDC: 4BFFE005  bl 0x8304fbe0
	ctx.lr = 0x83051BE0;
	sub_8304FBE0(ctx, base);
	// 83051BE0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83051BE4: 807E001C  lwz r3, 0x1c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 83051BE8: 4BFFDFF9  bl 0x8304fbe0
	ctx.lr = 0x83051BEC;
	sub_8304FBE0(ctx, base);
	// 83051BEC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83051BF0: 807E0020  lwz r3, 0x20(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 83051BF4: 4BFFE145  bl 0x8304fd38
	ctx.lr = 0x83051BF8;
	sub_8304FD38(ctx, base);
	// 83051BF8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83051BFC: 807E0024  lwz r3, 0x24(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 83051C00: 4BFFDFE1  bl 0x8304fbe0
	ctx.lr = 0x83051C04;
	sub_8304FBE0(ctx, base);
	// 83051C04: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83051C08: 807E002C  lwz r3, 0x2c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 83051C0C: 4BFFE9F5  bl 0x83050600
	ctx.lr = 0x83051C10;
	sub_83050600(ctx, base);
	// 83051C10: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83051C14: 807E004C  lwz r3, 0x4c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(76 as u32) ) } as u64;
	// 83051C18: 4BFFE401  bl 0x83050018
	ctx.lr = 0x83051C1C;
	sub_83050018(ctx, base);
	// 83051C1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83051C20: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83051C24: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83051C28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83051C2C: 4BFA7CD5  bl 0x82ff9900
	ctx.lr = 0x83051C30;
	sub_82FF9900(ctx, base);
	// 83051C30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83051C34: 889E0038  lbz r4, 0x38(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(56 as u32) ) } as u64;
	// 83051C38: 4BFA75C9  bl 0x82ff9200
	ctx.lr = 0x83051C3C;
	sub_82FF9200(ctx, base);
	// 83051C3C: 480000E8  b 0x83051d24
	pc = 0x83051D24; continue 'dispatch;
	// 83051C40: 4E800421  bctrl
	ctx.lr = 0x83051C44;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83051C44: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 83051C48: 38C00080  li r6, 0x80
	ctx.r[6].s64 = 128;
	// 83051C4C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83051C50: 3880006D  li r4, 0x6d
	ctx.r[4].s64 = 109;
	// 83051C54: 387E0008  addi r3, r30, 8
	ctx.r[3].s64 = ctx.r[30].s64 + 8;
	// 83051C58: 4BFFD301  bl 0x8304ef58
	ctx.lr = 0x83051C5C;
	sub_8304EF58(ctx, base);
	// 83051C5C: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 83051C60: 38C00080  li r6, 0x80
	ctx.r[6].s64 = 128;
	// 83051C64: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83051C68: 3880006D  li r4, 0x6d
	ctx.r[4].s64 = 109;
	// 83051C6C: 387E0010  addi r3, r30, 0x10
	ctx.r[3].s64 = ctx.r[30].s64 + 16;
	// 83051C70: 4BFFD2E9  bl 0x8304ef58
	ctx.lr = 0x83051C74;
	sub_8304EF58(ctx, base);
	// 83051C74: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 83051C78: 38A00080  li r5, 0x80
	ctx.r[5].s64 = 128;
	// 83051C7C: 3880006D  li r4, 0x6d
	ctx.r[4].s64 = 109;
	// 83051C80: 387E0014  addi r3, r30, 0x14
	ctx.r[3].s64 = ctx.r[30].s64 + 20;
	// 83051C84: 4BFFD8C5  bl 0x8304f548
	ctx.lr = 0x83051C88;
	sub_8304F548(ctx, base);
	// 83051C88: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 83051C8C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83051C90: 3880001D  li r4, 0x1d
	ctx.r[4].s64 = 29;
	// 83051C94: 387E0018  addi r3, r30, 0x18
	ctx.r[3].s64 = ctx.r[30].s64 + 24;
	// 83051C98: 4BFFC579  bl 0x8304e210
	ctx.lr = 0x83051C9C;
	sub_8304E210(ctx, base);
	// 83051C9C: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 83051CA0: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83051CA4: 3880001D  li r4, 0x1d
	ctx.r[4].s64 = 29;
	// 83051CA8: 387E001C  addi r3, r30, 0x1c
	ctx.r[3].s64 = ctx.r[30].s64 + 28;
	// 83051CAC: 4BFFC7E5  bl 0x8304e490
	ctx.lr = 0x83051CB0;
	sub_8304E490(ctx, base);
	// 83051CB0: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 83051CB4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83051CB8: 3880000D  li r4, 0xd
	ctx.r[4].s64 = 13;
	// 83051CBC: 387E0020  addi r3, r30, 0x20
	ctx.r[3].s64 = ctx.r[30].s64 + 32;
	// 83051CC0: 4BFFC909  bl 0x8304e5c8
	ctx.lr = 0x83051CC4;
	sub_8304E5C8(ctx, base);
	// 83051CC4: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 83051CC8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83051CCC: 3880000D  li r4, 0xd
	ctx.r[4].s64 = 13;
	// 83051CD0: 387E0024  addi r3, r30, 0x24
	ctx.r[3].s64 = ctx.r[30].s64 + 36;
	// 83051CD4: 4BFFCA55  bl 0x8304e728
	ctx.lr = 0x83051CD8;
	sub_8304E728(ctx, base);
	// 83051CD8: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 83051CDC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83051CE0: 3880001D  li r4, 0x1d
	ctx.r[4].s64 = 29;
	// 83051CE4: 387E002C  addi r3, r30, 0x2c
	ctx.r[3].s64 = ctx.r[30].s64 + 44;
	// 83051CE8: 4BFFDCA9  bl 0x8304f990
	ctx.lr = 0x83051CEC;
	sub_8304F990(ctx, base);
	// 83051CEC: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 83051CF0: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83051CF4: 3880001D  li r4, 0x1d
	ctx.r[4].s64 = 29;
	// 83051CF8: 387E004C  addi r3, r30, 0x4c
	ctx.r[3].s64 = ctx.r[30].s64 + 76;
	// 83051CFC: 4BFFCF65  bl 0x8304ec60
	ctx.lr = 0x83051D00;
	sub_8304EC60(ctx, base);
	// 83051D00: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83051D04: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 83051D08: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 83051D0C: 389E0004  addi r4, r30, 4
	ctx.r[4].s64 = ctx.r[30].s64 + 4;
	// 83051D10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83051D14: 4BFA7E15  bl 0x82ff9b28
	ctx.lr = 0x83051D18;
	sub_82FF9B28(ctx, base);
	// 83051D18: 389E0038  addi r4, r30, 0x38
	ctx.r[4].s64 = ctx.r[30].s64 + 56;
	// 83051D1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83051D20: 4BFA7761  bl 0x82ff9480
	ctx.lr = 0x83051D24;
	sub_82FF9480(ctx, base);
	// 83051D24: 807E0048  lwz r3, 0x48(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(72 as u32) ) } as u64;
	// 83051D28: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83051D2C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83051D30: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83051D34: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83051D38: 4E800421  bctrl
	ctx.lr = 0x83051D3C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83051D3C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83051D40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83051D44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83051D48: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83051D4C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83051D50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83051D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83051D58 size=8
    let mut pc: u32 = 0x83051D58;
    'dispatch: loop {
        match pc {
            0x83051D58 => {
    //   block [0x83051D58..0x83051D60)
	// 83051D58: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83051D5C: 821639D8  lwz r16, 0x39d8(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(14808 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83051D60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83051D60 size=272
    let mut pc: u32 = 0x83051D60;
    'dispatch: loop {
        match pc {
            0x83051D60 => {
    //   block [0x83051D60..0x83051E70)
	// 83051D60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83051D64: 481563F5  bl 0x831a8158
	ctx.lr = 0x83051D68;
	sub_831A8130(ctx, base);
	// 83051D68: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 83051D6C: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83051D70: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83051D74: 7D194378  mr r25, r8
	ctx.r[25].u64 = ctx.r[8].u64;
	// 83051D78: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 83051D7C: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 83051D80: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 83051D84: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83051D88: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83051D8C: 93DF00B4  stw r30, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[30].u32 ) };
	// 83051D90: 7D384B78  mr r24, r9
	ctx.r[24].u64 = ctx.r[9].u64;
	// 83051D94: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 83051D98: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83051D9C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83051DA0: 4E800421  bctrl
	ctx.lr = 0x83051DA4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83051DA4: 7C7A1B79  or. r26, r3, r3
	ctx.r[26].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 83051DA8: 408200B4  bne 0x83051e5c
	if !ctx.cr[0].eq {
	pc = 0x83051E5C; continue 'dispatch;
	}
	// 83051DAC: 38600060  li r3, 0x60
	ctx.r[3].s64 = 96;
	// 83051DB0: 809E0034  lwz r4, 0x34(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 83051DB4: 4BF864E5  bl 0x82fd8298
	ctx.lr = 0x83051DB8;
	sub_82FD8298(ctx, base);
	// 83051DB8: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83051DBC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83051DC0: 41820028  beq 0x83051de8
	if ctx.cr[0].eq {
	pc = 0x83051DE8; continue 'dispatch;
	}
	// 83051DC4: 3900FFFF  li r8, -1
	ctx.r[8].s64 = -1;
	// 83051DC8: 813E0034  lwz r9, 0x34(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 83051DCC: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 83051DD0: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 83051DD4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83051DD8: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83051DDC: 48013D3D  bl 0x83065b18
	ctx.lr = 0x83051DE0;
	sub_83065B18(ctx, base);
	// 83051DE0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83051DE4: 48000008  b 0x83051dec
	pc = 0x83051DEC; continue 'dispatch;
	// 83051DE8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83051DEC: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83051DF0: 7FBAEB78  mr r26, r29
	ctx.r[26].u64 = ctx.r[29].u64;
	// 83051DF4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83051DF8: 409A003C  bne cr6, 0x83051e34
	if !ctx.cr[6].eq {
	pc = 0x83051E34; continue 'dispatch;
	}
	// 83051DFC: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 83051E00: 809E0034  lwz r4, 0x34(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 83051E04: 4BF86495  bl 0x82fd8298
	ctx.lr = 0x83051E08;
	sub_82FD8298(ctx, base);
	// 83051E08: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83051E0C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83051E10: 4182001C  beq 0x83051e2c
	if ctx.cr[0].eq {
	pc = 0x83051E2C; continue 'dispatch;
	}
	// 83051E14: 38C00080  li r6, 0x80
	ctx.r[6].s64 = 128;
	// 83051E18: 80FE0034  lwz r7, 0x34(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 83051E1C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83051E20: 3880001D  li r4, 0x1d
	ctx.r[4].s64 = 29;
	// 83051E24: 4BFC6625  bl 0x83018448
	ctx.lr = 0x83051E28;
	sub_83018448(ctx, base);
	// 83051E28: 48000008  b 0x83051e30
	pc = 0x83051E30; continue 'dispatch;
	// 83051E2C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83051E30: 907E000C  stw r3, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 83051E34: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 83051E38: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 83051E3C: 7F26CB78  mr r6, r25
	ctx.r[6].u64 = ctx.r[25].u64;
	// 83051E40: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83051E44: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83051E48: 808B0010  lwz r4, 0x10(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83051E4C: 4BFC66DD  bl 0x83018528
	ctx.lr = 0x83051E50;
	sub_83018528(ctx, base);
	// 83051E50: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83051E54: 907D0010  stw r3, 0x10(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 83051E58: 48000008  b 0x83051e60
	pc = 0x83051E60; continue 'dispatch;
	// 83051E5C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83051E60: 99780000  stb r11, 0(r24)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 83051E64: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 83051E68: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 83051E6C: 4815633C  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83051E70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83051E70 size=48
    let mut pc: u32 = 0x83051E70;
    'dispatch: loop {
        match pc {
            0x83051E70 => {
    //   block [0x83051E70..0x83051EA0)
	// 83051E70: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 83051E74: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83051E78: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83051E7C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83051E80: 817F00B4  lwz r11, 0xb4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 83051E84: 808B0034  lwz r4, 0x34(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 83051E88: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83051E8C: 4BF86455  bl 0x82fd82e0
	ctx.lr = 0x83051E90;
	sub_82FD82E0(ctx, base);
	// 83051E90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83051E94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83051E98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83051E9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83051EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83051EA0 size=48
    let mut pc: u32 = 0x83051EA0;
    'dispatch: loop {
        match pc {
            0x83051EA0 => {
    //   block [0x83051EA0..0x83051ED0)
	// 83051EA0: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 83051EA4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83051EA8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83051EAC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83051EB0: 817F00B4  lwz r11, 0xb4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 83051EB4: 808B0034  lwz r4, 0x34(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 83051EB8: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83051EBC: 4BF86425  bl 0x82fd82e0
	ctx.lr = 0x83051EC0;
	sub_82FD82E0(ctx, base);
	// 83051EC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83051EC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83051EC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83051ECC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83051ED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83051ED0 size=8
    let mut pc: u32 = 0x83051ED0;
    'dispatch: loop {
        match pc {
            0x83051ED0 => {
    //   block [0x83051ED0..0x83051ED8)
	// 83051ED0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83051ED4: 82163A38  lwz r16, 0x3a38(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(14904 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83051ED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83051ED8 size=256
    let mut pc: u32 = 0x83051ED8;
    'dispatch: loop {
        match pc {
            0x83051ED8 => {
    //   block [0x83051ED8..0x83051FD8)
	// 83051ED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83051EDC: 48156281  bl 0x831a815c
	ctx.lr = 0x83051EE0;
	sub_831A8130(ctx, base);
	// 83051EE0: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 83051EE4: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83051EE8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83051EEC: 38600060  li r3, 0x60
	ctx.r[3].s64 = 96;
	// 83051EF0: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 83051EF4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83051EF8: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 83051EFC: 809E0034  lwz r4, 0x34(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 83051F00: 7D194378  mr r25, r8
	ctx.r[25].u64 = ctx.r[8].u64;
	// 83051F04: 93DF00B4  stw r30, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[30].u32 ) };
	// 83051F08: 7D3A4B78  mr r26, r9
	ctx.r[26].u64 = ctx.r[9].u64;
	// 83051F0C: 4BF8638D  bl 0x82fd8298
	ctx.lr = 0x83051F10;
	sub_82FD8298(ctx, base);
	// 83051F10: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83051F14: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83051F18: 41820028  beq 0x83051f40
	if ctx.cr[0].eq {
	pc = 0x83051F40; continue 'dispatch;
	}
	// 83051F1C: 3900FFFF  li r8, -1
	ctx.r[8].s64 = -1;
	// 83051F20: 813E0034  lwz r9, 0x34(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 83051F24: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 83051F28: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 83051F2C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83051F30: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83051F34: 48013BE5  bl 0x83065b18
	ctx.lr = 0x83051F38;
	sub_83065B18(ctx, base);
	// 83051F38: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83051F3C: 48000008  b 0x83051f44
	pc = 0x83051F44; continue 'dispatch;
	// 83051F40: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83051F44: 574B063F  clrlwi. r11, r26, 0x18
	ctx.r[11].u64 = ctx.r[26].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83051F48: 41820064  beq 0x83051fac
	if ctx.cr[0].eq {
	pc = 0x83051FAC; continue 'dispatch;
	}
	// 83051F4C: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83051F50: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83051F54: 409A003C  bne cr6, 0x83051f90
	if !ctx.cr[6].eq {
	pc = 0x83051F90; continue 'dispatch;
	}
	// 83051F58: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 83051F5C: 809E0034  lwz r4, 0x34(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 83051F60: 4BF86339  bl 0x82fd8298
	ctx.lr = 0x83051F64;
	sub_82FD8298(ctx, base);
	// 83051F64: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83051F68: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83051F6C: 4182001C  beq 0x83051f88
	if ctx.cr[0].eq {
	pc = 0x83051F88; continue 'dispatch;
	}
	// 83051F70: 38C00080  li r6, 0x80
	ctx.r[6].s64 = 128;
	// 83051F74: 80FE0034  lwz r7, 0x34(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 83051F78: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83051F7C: 3880001D  li r4, 0x1d
	ctx.r[4].s64 = 29;
	// 83051F80: 4BFC64C9  bl 0x83018448
	ctx.lr = 0x83051F84;
	sub_83018448(ctx, base);
	// 83051F84: 48000008  b 0x83051f8c
	pc = 0x83051F8C; continue 'dispatch;
	// 83051F88: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83051F8C: 907E000C  stw r3, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 83051F90: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 83051F94: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 83051F98: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83051F9C: 7F26CB78  mr r6, r25
	ctx.r[6].u64 = ctx.r[25].u64;
	// 83051FA0: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 83051FA4: 808B0010  lwz r4, 0x10(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83051FA8: 4800001C  b 0x83051fc4
	pc = 0x83051FC4; continue 'dispatch;
	// 83051FAC: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 83051FB0: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 83051FB4: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83051FB8: 7F26CB78  mr r6, r25
	ctx.r[6].u64 = ctx.r[25].u64;
	// 83051FBC: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 83051FC0: 808B0010  lwz r4, 0x10(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83051FC4: 4BFC6565  bl 0x83018528
	ctx.lr = 0x83051FC8;
	sub_83018528(ctx, base);
	// 83051FC8: 907D0010  stw r3, 0x10(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 83051FCC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83051FD0: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 83051FD4: 481561D8  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83051FD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83051FD8 size=48
    let mut pc: u32 = 0x83051FD8;
    'dispatch: loop {
        match pc {
            0x83051FD8 => {
    //   block [0x83051FD8..0x83052008)
	// 83051FD8: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 83051FDC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83051FE0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83051FE4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83051FE8: 817F00B4  lwz r11, 0xb4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 83051FEC: 808B0034  lwz r4, 0x34(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 83051FF0: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83051FF4: 4BF862ED  bl 0x82fd82e0
	ctx.lr = 0x83051FF8;
	sub_82FD82E0(ctx, base);
	// 83051FF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83051FFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83052000: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83052004: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83052008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83052008 size=48
    let mut pc: u32 = 0x83052008;
    'dispatch: loop {
        match pc {
            0x83052008 => {
    //   block [0x83052008..0x83052038)
	// 83052008: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 8305200C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83052010: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83052014: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83052018: 817F00B4  lwz r11, 0xb4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 8305201C: 808B0034  lwz r4, 0x34(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 83052020: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83052024: 4BF862BD  bl 0x82fd82e0
	ctx.lr = 0x83052028;
	sub_82FD82E0(ctx, base);
	// 83052028: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8305202C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83052030: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83052034: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83052038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83052038 size=8
    let mut pc: u32 = 0x83052038;
    'dispatch: loop {
        match pc {
            0x83052038 => {
    //   block [0x83052038..0x83052040)
	// 83052038: 8063004C  lwz r3, 0x4c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(76 as u32) ) } as u64;
	// 8305203C: 4BFDAE0C  b 0x8302ce48
	sub_8302CE48(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83052040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83052040 size=116
    let mut pc: u32 = 0x83052040;
    'dispatch: loop {
        match pc {
            0x83052040 => {
    //   block [0x83052040..0x830520B4)
	// 83052040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83052044: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83052048: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8305204C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83052050: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83052054: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83052058: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8305205C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83052060: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83052064: 807F004C  lwz r3, 0x4c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 83052068: 4BFA83F9  bl 0x82ffa460
	ctx.lr = 0x8305206C;
	sub_82FFA460(ctx, base);
	// 8305206C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83052070: 4182001C  beq 0x8305208c
	if ctx.cr[0].eq {
	pc = 0x8305208C; continue 'dispatch;
	}
	// 83052074: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83052078: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8305207C: 41820010  beq 0x8305208c
	if ctx.cr[0].eq {
	pc = 0x8305208C; continue 'dispatch;
	}
	// 83052080: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83052084: 4BFEEB85  bl 0x83040c08
	ctx.lr = 0x83052088;
	sub_83040C08(ctx, base);
	// 83052088: 48000014  b 0x8305209c
	pc = 0x8305209C; continue 'dispatch;
	// 8305208C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83052090: 807F004C  lwz r3, 0x4c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 83052094: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83052098: 4BFDADB1  bl 0x8302ce48
	ctx.lr = 0x8305209C;
	sub_8302CE48(ctx, base);
	// 8305209C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830520A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830520A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830520A8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830520AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830520B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830520B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830520B8 size=96
    let mut pc: u32 = 0x830520B8;
    'dispatch: loop {
        match pc {
            0x830520B8 => {
    //   block [0x830520B8..0x83052118)
	// 830520B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830520BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830520C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830520C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830520C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830520CC: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830520D0: 4BFBB2D1  bl 0x8300d3a0
	ctx.lr = 0x830520D4;
	sub_8300D3A0(ctx, base);
	// 830520D4: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 830520D8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830520DC: 41820008  beq 0x830520e4
	if ctx.cr[0].eq {
	pc = 0x830520E4; continue 'dispatch;
	}
	// 830520E0: 4BFBB2C1  bl 0x8300d3a0
	ctx.lr = 0x830520E4;
	sub_8300D3A0(ctx, base);
	// 830520E4: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830520E8: 4BFBB2B9  bl 0x8300d3a0
	ctx.lr = 0x830520EC;
	sub_8300D3A0(ctx, base);
	// 830520EC: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 830520F0: 4BFC8859  bl 0x8301a948
	ctx.lr = 0x830520F4;
	sub_8301A948(ctx, base);
	// 830520F4: 807F004C  lwz r3, 0x4c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 830520F8: 4BF94499  bl 0x82fe6590
	ctx.lr = 0x830520FC;
	sub_82FE6590(ctx, base);
	// 830520FC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83052100: 997F0038  stb r11, 0x38(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u8 ) };
	// 83052104: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83052108: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8305210C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83052110: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83052114: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83052118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83052118 size=444
    let mut pc: u32 = 0x83052118;
    'dispatch: loop {
        match pc {
            0x83052118 => {
    //   block [0x83052118..0x830522D4)
	// 83052118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8305211C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83052120: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83052124: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83052128: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8305212C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83052130: 83DF0008  lwz r30, 8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83052134: 281E0000  cmplwi r30, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83052138: 41820014  beq 0x8305214c
	if ctx.cr[0].eq {
	pc = 0x8305214C; continue 'dispatch;
	}
	// 8305213C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83052140: 4BFBB909  bl 0x8300da48
	ctx.lr = 0x83052144;
	sub_8300DA48(ctx, base);
	// 83052144: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83052148: 4BF86199  bl 0x82fd82e0
	ctx.lr = 0x8305214C;
	sub_82FD82E0(ctx, base);
	// 8305214C: 83DF000C  lwz r30, 0xc(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83052150: 281E0000  cmplwi r30, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83052154: 41820014  beq 0x83052168
	if ctx.cr[0].eq {
	pc = 0x83052168; continue 'dispatch;
	}
	// 83052158: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8305215C: 4BFBB8ED  bl 0x8300da48
	ctx.lr = 0x83052160;
	sub_8300DA48(ctx, base);
	// 83052160: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83052164: 4BF8617D  bl 0x82fd82e0
	ctx.lr = 0x83052168;
	sub_82FD82E0(ctx, base);
	// 83052168: 83DF0010  lwz r30, 0x10(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8305216C: 281E0000  cmplwi r30, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83052170: 41820014  beq 0x83052184
	if ctx.cr[0].eq {
	pc = 0x83052184; continue 'dispatch;
	}
	// 83052174: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83052178: 4BFBB8D1  bl 0x8300da48
	ctx.lr = 0x8305217C;
	sub_8300DA48(ctx, base);
	// 8305217C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83052180: 4BF86161  bl 0x82fd82e0
	ctx.lr = 0x83052184;
	sub_82FD82E0(ctx, base);
	// 83052184: 83DF0014  lwz r30, 0x14(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83052188: 281E0000  cmplwi r30, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8305218C: 41820014  beq 0x830521a0
	if ctx.cr[0].eq {
	pc = 0x830521A0; continue 'dispatch;
	}
	// 83052190: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83052194: 4BFCAC45  bl 0x8301cdd8
	ctx.lr = 0x83052198;
	sub_8301CDD8(ctx, base);
	// 83052198: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8305219C: 4BF86145  bl 0x82fd82e0
	ctx.lr = 0x830521A0;
	sub_82FD82E0(ctx, base);
	// 830521A0: 807F0034  lwz r3, 0x34(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 830521A4: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830521A8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830521AC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830521B0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830521B4: 4E800421  bctrl
	ctx.lr = 0x830521B8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830521B8: 83DF0018  lwz r30, 0x18(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 830521BC: 281E0000  cmplwi r30, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830521C0: 41820014  beq 0x830521d4
	if ctx.cr[0].eq {
	pc = 0x830521D4; continue 'dispatch;
	}
	// 830521C4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830521C8: 4BF94AA1  bl 0x82fe6c68
	ctx.lr = 0x830521CC;
	sub_82FE6C68(ctx, base);
	// 830521CC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830521D0: 4BF86111  bl 0x82fd82e0
	ctx.lr = 0x830521D4;
	sub_82FD82E0(ctx, base);
	// 830521D4: 83DF001C  lwz r30, 0x1c(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 830521D8: 281E0000  cmplwi r30, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830521DC: 41820014  beq 0x830521f0
	if ctx.cr[0].eq {
	pc = 0x830521F0; continue 'dispatch;
	}
	// 830521E0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830521E4: 4BF94A85  bl 0x82fe6c68
	ctx.lr = 0x830521E8;
	sub_82FE6C68(ctx, base);
	// 830521E8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830521EC: 4BF860F5  bl 0x82fd82e0
	ctx.lr = 0x830521F0;
	sub_82FD82E0(ctx, base);
	// 830521F0: 83DF0020  lwz r30, 0x20(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 830521F4: 281E0000  cmplwi r30, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830521F8: 41820014  beq 0x8305220c
	if ctx.cr[0].eq {
	pc = 0x8305220C; continue 'dispatch;
	}
	// 830521FC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83052200: 4BF94A69  bl 0x82fe6c68
	ctx.lr = 0x83052204;
	sub_82FE6C68(ctx, base);
	// 83052204: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83052208: 4BF860D9  bl 0x82fd82e0
	ctx.lr = 0x8305220C;
	sub_82FD82E0(ctx, base);
	// 8305220C: 83DF0024  lwz r30, 0x24(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83052210: 281E0000  cmplwi r30, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83052214: 41820014  beq 0x83052228
	if ctx.cr[0].eq {
	pc = 0x83052228; continue 'dispatch;
	}
	// 83052218: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8305221C: 4BF94A4D  bl 0x82fe6c68
	ctx.lr = 0x83052220;
	sub_82FE6C68(ctx, base);
	// 83052220: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83052224: 4BF860BD  bl 0x82fd82e0
	ctx.lr = 0x83052228;
	sub_82FD82E0(ctx, base);
	// 83052228: 83DF0028  lwz r30, 0x28(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8305222C: 281E0000  cmplwi r30, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83052230: 41820014  beq 0x83052244
	if ctx.cr[0].eq {
	pc = 0x83052244; continue 'dispatch;
	}
	// 83052234: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83052238: 48047919  bl 0x83099b50
	ctx.lr = 0x8305223C;
	sub_83099B50(ctx, base);
	// 8305223C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83052240: 4BF860A1  bl 0x82fd82e0
	ctx.lr = 0x83052244;
	sub_82FD82E0(ctx, base);
	// 83052244: 83DF002C  lwz r30, 0x2c(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 83052248: 281E0000  cmplwi r30, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8305224C: 41820014  beq 0x83052260
	if ctx.cr[0].eq {
	pc = 0x83052260; continue 'dispatch;
	}
	// 83052250: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83052254: 4BFFD91D  bl 0x8304fb70
	ctx.lr = 0x83052258;
	sub_8304FB70(ctx, base);
	// 83052258: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8305225C: 4BF86085  bl 0x82fd82e0
	ctx.lr = 0x83052260;
	sub_82FD82E0(ctx, base);
	// 83052260: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 83052264: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83052268: 41820018  beq 0x83052280
	if ctx.cr[0].eq {
	pc = 0x83052280; continue 'dispatch;
	}
	// 8305226C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83052270: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83052274: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83052278: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8305227C: 4E800421  bctrl
	ctx.lr = 0x83052280;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83052280: 807F0048  lwz r3, 0x48(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 83052284: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83052288: 41820018  beq 0x830522a0
	if ctx.cr[0].eq {
	pc = 0x830522A0; continue 'dispatch;
	}
	// 8305228C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83052290: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83052294: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83052298: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8305229C: 4E800421  bctrl
	ctx.lr = 0x830522A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830522A0: 83FF004C  lwz r31, 0x4c(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 830522A4: 281F0000  cmplwi r31, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830522A8: 41820014  beq 0x830522bc
	if ctx.cr[0].eq {
	pc = 0x830522BC; continue 'dispatch;
	}
	// 830522AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830522B0: 4BF949B9  bl 0x82fe6c68
	ctx.lr = 0x830522B4;
	sub_82FE6C68(ctx, base);
	// 830522B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830522B8: 4BF86029  bl 0x82fd82e0
	ctx.lr = 0x830522BC;
	sub_82FD82E0(ctx, base);
	// 830522BC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830522C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830522C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830522C8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830522CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830522D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830522D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830522D8 size=8
    let mut pc: u32 = 0x830522D8;
    'dispatch: loop {
        match pc {
            0x830522D8 => {
    //   block [0x830522D8..0x830522E0)
	// 830522D8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830522DC: 82163B6C  lwz r16, 0x3b6c(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(15212 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830522E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830522E0 size=632
    let mut pc: u32 = 0x830522E0;
    'dispatch: loop {
        match pc {
            0x830522E0 => {
    //   block [0x830522E0..0x83052558)
	// 830522E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830522E4: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 830522E8: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 830522EC: 48155E7D  bl 0x831a8168
	ctx.lr = 0x830522F0;
	sub_831A8130(ctx, base);
	// 830522F0: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 830522F4: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830522F8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830522FC: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 83052300: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83052304: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83052308: 909E0034  stw r4, 0x34(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(52 as u32), ctx.r[4].u32 ) };
	// 8305230C: 396B3A88  addi r11, r11, 0x3a88
	ctx.r[11].s64 = ctx.r[11].s64 + 14984;
	// 83052310: 3B9E003C  addi r28, r30, 0x3c
	ctx.r[28].s64 = ctx.r[30].s64 + 60;
	// 83052314: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83052318: 93BE0004  stw r29, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 8305231C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83052320: 93BE000C  stw r29, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 83052324: 93BE0018  stw r29, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[29].u32 ) };
	// 83052328: 93BE001C  stw r29, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[29].u32 ) };
	// 8305232C: 93BE0020  stw r29, 0x20(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[29].u32 ) };
	// 83052330: 93BE0024  stw r29, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[29].u32 ) };
	// 83052334: 93BE0028  stw r29, 0x28(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), ctx.r[29].u32 ) };
	// 83052338: 93BE002C  stw r29, 0x2c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(44 as u32), ctx.r[29].u32 ) };
	// 8305233C: 9BBE0038  stb r29, 0x38(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(56 as u32), ctx.r[29].u8 ) };
	// 83052340: 93BE0008  stw r29, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 83052344: 93BE0010  stw r29, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[29].u32 ) };
	// 83052348: 93BE0014  stw r29, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[29].u32 ) };
	// 8305234C: 93BE0030  stw r29, 0x30(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(48 as u32), ctx.r[29].u32 ) };
	// 83052350: 4BFBD791  bl 0x8300fae0
	ctx.lr = 0x83052354;
	sub_8300FAE0(ctx, base);
	// 83052354: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 83052358: 809E0034  lwz r4, 0x34(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 8305235C: 93BE0048  stw r29, 0x48(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(72 as u32), ctx.r[29].u32 ) };
	// 83052360: 93BE004C  stw r29, 0x4c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(76 as u32), ctx.r[29].u32 ) };
	// 83052364: 4BF85F35  bl 0x82fd8298
	ctx.lr = 0x83052368;
	sub_82FD8298(ctx, base);
	// 83052368: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8305236C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83052370: 4182001C  beq 0x8305238c
	if ctx.cr[0].eq {
	pc = 0x8305238C; continue 'dispatch;
	}
	// 83052374: 38C00080  li r6, 0x80
	ctx.r[6].s64 = 128;
	// 83052378: 80FE0034  lwz r7, 0x34(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 8305237C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83052380: 3880006D  li r4, 0x6d
	ctx.r[4].s64 = 109;
	// 83052384: 4BFC60C5  bl 0x83018448
	ctx.lr = 0x83052388;
	sub_83018448(ctx, base);
	// 83052388: 48000008  b 0x83052390
	pc = 0x83052390; continue 'dispatch;
	// 8305238C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83052390: 907E0008  stw r3, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 83052394: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 83052398: 809E0034  lwz r4, 0x34(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 8305239C: 4BF85EFD  bl 0x82fd8298
	ctx.lr = 0x830523A0;
	sub_82FD8298(ctx, base);
	// 830523A0: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830523A4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830523A8: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 830523AC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830523B0: 41820028  beq 0x830523d8
	if ctx.cr[0].eq {
	pc = 0x830523D8; continue 'dispatch;
	}
	// 830523B4: 38C00080  li r6, 0x80
	ctx.r[6].s64 = 128;
	// 830523B8: 80FE0034  lwz r7, 0x34(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 830523BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830523C0: 3880006D  li r4, 0x6d
	ctx.r[4].s64 = 109;
	// 830523C4: 4BFC6085  bl 0x83018448
	ctx.lr = 0x830523C8;
	sub_83018448(ctx, base);
	// 830523C8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 830523CC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830523D0: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830523D4: 48000008  b 0x830523dc
	pc = 0x830523DC; continue 'dispatch;
	// 830523D8: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 830523DC: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 830523E0: 809E0034  lwz r4, 0x34(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 830523E4: 917E0010  stw r11, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 830523E8: 4BF85EB1  bl 0x82fd8298
	ctx.lr = 0x830523EC;
	sub_82FD8298(ctx, base);
	// 830523EC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830523F0: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830523F4: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 830523F8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830523FC: 41820024  beq 0x83052420
	if ctx.cr[0].eq {
	pc = 0x83052420; continue 'dispatch;
	}
	// 83052400: 38A00080  li r5, 0x80
	ctx.r[5].s64 = 128;
	// 83052404: 80DE0034  lwz r6, 0x34(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 83052408: 3880006D  li r4, 0x6d
	ctx.r[4].s64 = 109;
	// 8305240C: 4BFFA225  bl 0x8304c630
	ctx.lr = 0x83052410;
	sub_8304C630(ctx, base);
	// 83052410: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83052414: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83052418: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8305241C: 48000008  b 0x83052424
	pc = 0x83052424; continue 'dispatch;
	// 83052420: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 83052424: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 83052428: 809E0034  lwz r4, 0x34(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 8305242C: 917E0014  stw r11, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 83052430: 4BF85E69  bl 0x82fd8298
	ctx.lr = 0x83052434;
	sub_82FD8298(ctx, base);
	// 83052434: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83052438: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8305243C: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83052440: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83052444: 4182001C  beq 0x83052460
	if ctx.cr[0].eq {
	pc = 0x83052460; continue 'dispatch;
	}
	// 83052448: 809E0034  lwz r4, 0x34(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 8305244C: 4BFC3A4D  bl 0x83015e98
	ctx.lr = 0x83052450;
	sub_83015E98(ctx, base);
	// 83052450: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83052454: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83052458: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8305245C: 48000008  b 0x83052464
	pc = 0x83052464; continue 'dispatch;
	// 83052460: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 83052464: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83052468: 917E0030  stw r11, 0x30(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 8305246C: 4BFBEBA5  bl 0x83011010
	ctx.lr = 0x83052470;
	sub_83011010(ctx, base);
	// 83052470: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83052474: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83052478: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 8305247C: 809E0034  lwz r4, 0x34(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 83052480: 4BF85E19  bl 0x82fd8298
	ctx.lr = 0x83052484;
	sub_82FD8298(ctx, base);
	// 83052484: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83052488: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8305248C: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83052490: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83052494: 41820024  beq 0x830524b8
	if ctx.cr[0].eq {
	pc = 0x830524B8; continue 'dispatch;
	}
	// 83052498: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8305249C: 80BE0034  lwz r5, 0x34(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 830524A0: 388B8034  addi r4, r11, -0x7fcc
	ctx.r[4].s64 = ctx.r[11].s64 + -32716;
	// 830524A4: 480010FD  bl 0x830535a0
	ctx.lr = 0x830524A8;
	sub_830535A0(ctx, base);
	// 830524A8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 830524AC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830524B0: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830524B4: 48000008  b 0x830524bc
	pc = 0x830524BC; continue 'dispatch;
	// 830524B8: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 830524BC: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 830524C0: 809E0034  lwz r4, 0x34(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 830524C4: 917E0048  stw r11, 0x48(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 830524C8: 4BF85DD1  bl 0x82fd8298
	ctx.lr = 0x830524CC;
	sub_82FD8298(ctx, base);
	// 830524CC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830524D0: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830524D4: 7C7C1B79  or. r28, r3, r3
	ctx.r[28].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 830524D8: 939F0050  stw r28, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 830524DC: 4182005C  beq 0x83052538
	if ctx.cr[0].eq {
	pc = 0x83052538; continue 'dispatch;
	}
	// 830524E0: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 830524E4: 809E0034  lwz r4, 0x34(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 830524E8: 4BF85DB1  bl 0x82fd8298
	ctx.lr = 0x830524EC;
	sub_82FD8298(ctx, base);
	// 830524EC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830524F0: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830524F4: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 830524F8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830524FC: 41820018  beq 0x83052514
	if ctx.cr[0].eq {
	pc = 0x83052514; continue 'dispatch;
	}
	// 83052500: 4BFAD091  bl 0x82fff590
	ctx.lr = 0x83052504;
	sub_82FFF590(ctx, base);
	// 83052504: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 83052508: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8305250C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83052510: 48000008  b 0x83052518
	pc = 0x83052518; continue 'dispatch;
	// 83052514: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 83052518: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8305251C: 3880001D  li r4, 0x1d
	ctx.r[4].s64 = 29;
	// 83052520: 80FE0034  lwz r7, 0x34(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 83052524: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83052528: 4BFE8BC9  bl 0x8303b0f0
	ctx.lr = 0x8305252C;
	sub_8303B0F0(ctx, base);
	// 8305252C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83052530: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83052534: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83052538: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8305253C: 93BE004C  stw r29, 0x4c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(76 as u32), ctx.r[29].u32 ) };
	// 83052540: 4BFFFB79  bl 0x830520b8
	ctx.lr = 0x83052544;
	sub_830520B8(ctx, base);
	// 83052544: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83052548: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8305254C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83052550: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 83052554: 48155C64  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83052558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83052558 size=8
    let mut pc: u32 = 0x83052558;
    'dispatch: loop {
        match pc {
            0x83052558 => {
    //   block [0x83052558..0x83052560)
	// 83052558: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8305255C: 82163B6C  lwz r16, 0x3b6c(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(15212 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83052560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83052560 size=24
    let mut pc: u32 = 0x83052560;
    'dispatch: loop {
        match pc {
            0x83052560 => {
    //   block [0x83052560..0x83052578)
	// 83052560: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83052564: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83052568: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8305256C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83052570: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83052574: 4815E6B5  bl 0x831b0c28
	ctx.lr = 0x83052578;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83052580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83052580 size=36
    let mut pc: u32 = 0x83052580;
    'dispatch: loop {
        match pc {
            0x83052580 => {
    //   block [0x83052580..0x830525A4)
	// 83052580: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83052584: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83052588: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8305258C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83052590: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83052594: 4BFFFB85  bl 0x83052118
	ctx.lr = 0x83052598;
	sub_83052118(ctx, base);
	// 83052598: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8305259C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830525A0: 4815E689  bl 0x831b0c28
	ctx.lr = 0x830525A4;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830525A4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830525A4 size=40
    let mut pc: u32 = 0x830525A4;
    'dispatch: loop {
        match pc {
            0x830525A4 => {
    //   block [0x830525A4..0x830525CC)
	// 830525A4: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830525A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830525AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830525B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830525B4: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 830525B8: 4BFFA1A9  bl 0x8304c760
	ctx.lr = 0x830525BC;
	sub_8304C760(ctx, base);
	// 830525BC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830525C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830525C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830525C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830525CC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830525CC size=44
    let mut pc: u32 = 0x830525CC;
    'dispatch: loop {
        match pc {
            0x830525CC => {
    //   block [0x830525CC..0x830525F8)
	// 830525CC: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830525D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830525D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830525D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830525DC: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 830525E0: 386B003C  addi r3, r11, 0x3c
	ctx.r[3].s64 = ctx.r[11].s64 + 60;
	// 830525E4: 4BFBE3B5  bl 0x83010998
	ctx.lr = 0x830525E8;
	sub_83010998(ctx, base);
	// 830525E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830525EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830525F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830525F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830525F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830525F8 size=48
    let mut pc: u32 = 0x830525F8;
    'dispatch: loop {
        match pc {
            0x830525F8 => {
    //   block [0x830525F8..0x83052628)
	// 830525F8: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830525FC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83052600: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83052604: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83052608: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 8305260C: 808B0034  lwz r4, 0x34(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 83052610: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83052614: 4BF85CCD  bl 0x82fd82e0
	ctx.lr = 0x83052618;
	sub_82FD82E0(ctx, base);
	// 83052618: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8305261C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83052620: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83052624: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83052628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83052628 size=48
    let mut pc: u32 = 0x83052628;
    'dispatch: loop {
        match pc {
            0x83052628 => {
    //   block [0x83052628..0x83052658)
	// 83052628: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 8305262C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83052630: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83052634: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83052638: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 8305263C: 808B0034  lwz r4, 0x34(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 83052640: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83052644: 4BF85C9D  bl 0x82fd82e0
	ctx.lr = 0x83052648;
	sub_82FD82E0(ctx, base);
	// 83052648: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8305264C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83052650: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83052654: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83052658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83052658 size=48
    let mut pc: u32 = 0x83052658;
    'dispatch: loop {
        match pc {
            0x83052658 => {
    //   block [0x83052658..0x83052688)
	// 83052658: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 8305265C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83052660: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83052664: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83052668: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 8305266C: 808B0034  lwz r4, 0x34(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 83052670: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83052674: 4BF85C6D  bl 0x82fd82e0
	ctx.lr = 0x83052678;
	sub_82FD82E0(ctx, base);
	// 83052678: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8305267C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83052680: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83052684: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83052688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83052688 size=48
    let mut pc: u32 = 0x83052688;
    'dispatch: loop {
        match pc {
            0x83052688 => {
    //   block [0x83052688..0x830526B8)
	// 83052688: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 8305268C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83052690: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83052694: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83052698: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 8305269C: 808B0034  lwz r4, 0x34(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 830526A0: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830526A4: 4BF85C3D  bl 0x82fd82e0
	ctx.lr = 0x830526A8;
	sub_82FD82E0(ctx, base);
	// 830526A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830526AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830526B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830526B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830526B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830526B8 size=48
    let mut pc: u32 = 0x830526B8;
    'dispatch: loop {
        match pc {
            0x830526B8 => {
    //   block [0x830526B8..0x830526E8)
	// 830526B8: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830526BC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830526C0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830526C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830526C8: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 830526CC: 808B0034  lwz r4, 0x34(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 830526D0: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830526D4: 4BF85C0D  bl 0x82fd82e0
	ctx.lr = 0x830526D8;
	sub_82FD82E0(ctx, base);
	// 830526D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830526DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830526E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830526E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830526E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830526E8 size=48
    let mut pc: u32 = 0x830526E8;
    'dispatch: loop {
        match pc {
            0x830526E8 => {
    //   block [0x830526E8..0x83052718)
	// 830526E8: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830526EC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830526F0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830526F4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830526F8: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 830526FC: 808B0034  lwz r4, 0x34(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 83052700: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83052704: 4BF85BDD  bl 0x82fd82e0
	ctx.lr = 0x83052708;
	sub_82FD82E0(ctx, base);
	// 83052708: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8305270C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83052710: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83052714: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83052718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83052718 size=48
    let mut pc: u32 = 0x83052718;
    'dispatch: loop {
        match pc {
            0x83052718 => {
    //   block [0x83052718..0x83052748)
	// 83052718: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 8305271C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83052720: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83052724: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83052728: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 8305272C: 808B0034  lwz r4, 0x34(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 83052730: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 83052734: 4BF85BAD  bl 0x82fd82e0
	ctx.lr = 0x83052738;
	sub_82FD82E0(ctx, base);
	// 83052738: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8305273C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83052740: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83052744: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83052748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83052748 size=8
    let mut pc: u32 = 0x83052748;
    'dispatch: loop {
        match pc {
            0x83052748 => {
    //   block [0x83052748..0x83052750)
	// 83052748: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8305274C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83052750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83052750 size=8
    let mut pc: u32 = 0x83052750;
    'dispatch: loop {
        match pc {
            0x83052750 => {
    //   block [0x83052750..0x83052758)
	// 83052750: 88630038  lbz r3, 0x38(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) } as u64;
	// 83052754: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83052758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83052758 size=8
    let mut pc: u32 = 0x83052758;
    'dispatch: loop {
        match pc {
            0x83052758 => {
    //   block [0x83052758..0x83052760)
	// 83052758: 98830038  stb r4, 0x38(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), ctx.r[4].u8 ) };
	// 8305275C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83052760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83052760 size=104
    let mut pc: u32 = 0x83052760;
    'dispatch: loop {
        match pc {
            0x83052760 => {
    //   block [0x83052760..0x830527C8)
	// 83052760: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83052764: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83052768: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8305276C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 83052770: 419A002C  beq cr6, 0x8305279c
	if ctx.cr[6].eq {
	pc = 0x8305279C; continue 'dispatch;
	}
	// 83052774: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 83052778: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8305277C: 41990020  bgt cr6, 0x8305279c
	if ctx.cr[6].gt {
	pc = 0x8305279C; continue 'dispatch;
	}
	// 83052780: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 83052784: 548A103A  slwi r10, r4, 2
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83052788: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8305278C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83052790: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83052794: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83052798: 4E800020  blr
	return;
	// 8305279C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830527A0: 80E30000  lwz r7, 0(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830527A4: 38C00043  li r6, 0x43
	ctx.r[6].s64 = 67;
	// 830527A8: 388B2368  addi r4, r11, 0x2368
	ctx.r[4].s64 = ctx.r[11].s64 + 9064;
	// 830527AC: 38A00123  li r5, 0x123
	ctx.r[5].s64 = 291;
	// 830527B0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830527B4: 4BF7E515  bl 0x82fd0cc8
	ctx.lr = 0x830527B8;
	sub_82FD0CC8(ctx, base);
	// 830527B8: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830527BC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830527C0: 388BC3FC  addi r4, r11, -0x3c04
	ctx.r[4].s64 = ctx.r[11].s64 + -15364;
	// 830527C4: 4815E465  bl 0x831b0c28
	ctx.lr = 0x830527C8;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830527C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830527C8 size=80
    let mut pc: u32 = 0x830527C8;
    'dispatch: loop {
        match pc {
            0x830527C8 => {
    //   block [0x830527C8..0x83052818)
	// 830527C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830527CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830527D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830527D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830527D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830527DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830527E0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830527E4: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830527E8: 4BFFFF79  bl 0x83052760
	ctx.lr = 0x830527EC;
	sub_83052760(ctx, base);
	// 830527EC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830527F0: 40820010  bne 0x83052800
	if !ctx.cr[0].eq {
	pc = 0x83052800; continue 'dispatch;
	}
	// 830527F4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830527F8: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830527FC: 4BFFFF65  bl 0x83052760
	ctx.lr = 0x83052800;
	sub_83052760(ctx, base);
	// 83052800: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83052804: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83052808: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8305280C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83052810: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83052814: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83052818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83052818 size=140
    let mut pc: u32 = 0x83052818;
    'dispatch: loop {
        match pc {
            0x83052818 => {
    //   block [0x83052818..0x830528A4)
	// 83052818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8305281C: 4815594D  bl 0x831a8168
	ctx.lr = 0x83052820;
	sub_831A8130(ctx, base);
	// 83052820: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83052824: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83052828: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8305282C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83052830: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 83052834: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 83052838: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 8305283C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83052840: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83052844: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83052848: 4BFC5669  bl 0x83017eb0
	ctx.lr = 0x8305284C;
	sub_83017EB0(ctx, base);
	// 8305284C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83052850: 41820010  beq 0x83052860
	if ctx.cr[0].eq {
	pc = 0x83052860; continue 'dispatch;
	}
	// 83052854: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83052858: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8305285C: 4082003C  bne 0x83052898
	if !ctx.cr[0].eq {
	pc = 0x83052898; continue 'dispatch;
	}
	// 83052860: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 83052864: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83052868: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 8305286C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83052870: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83052874: 4BFC563D  bl 0x83017eb0
	ctx.lr = 0x83052878;
	sub_83017EB0(ctx, base);
	// 83052878: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8305287C: 41820010  beq 0x8305288c
	if ctx.cr[0].eq {
	pc = 0x8305288C; continue 'dispatch;
	}
	// 83052880: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83052884: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83052888: 40820010  bne 0x83052898
	if !ctx.cr[0].eq {
	pc = 0x83052898; continue 'dispatch;
	}
	// 8305288C: 3D608217  lis r11, -0x7de9
	ctx.r[11].s64 = -2112421888;
	// 83052890: 806B9770  lwz r3, -0x6890(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26768 as u32) ) } as u64;
	// 83052894: 48000008  b 0x8305289c
	pc = 0x8305289C; continue 'dispatch;
	// 83052898: 806B0010  lwz r3, 0x10(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8305289C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830528A0: 48155918  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830528A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830528A8 size=184
    let mut pc: u32 = 0x830528A8;
    'dispatch: loop {
        match pc {
            0x830528A8 => {
    //   block [0x830528A8..0x83052960)
	// 830528A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830528AC: 481558BD  bl 0x831a8168
	ctx.lr = 0x830528B0;
	sub_831A8130(ctx, base);
	// 830528B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830528B4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 830528B8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830528BC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 830528C0: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 830528C4: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 830528C8: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 830528CC: 807C0008  lwz r3, 8(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 830528D0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 830528D4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830528D8: 4BFC55D9  bl 0x83017eb0
	ctx.lr = 0x830528DC;
	sub_83017EB0(ctx, base);
	// 830528DC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830528E0: 41820010  beq 0x830528f0
	if ctx.cr[0].eq {
	pc = 0x830528F0; continue 'dispatch;
	}
	// 830528E4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830528E8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830528EC: 40820068  bne 0x83052954
	if !ctx.cr[0].eq {
	pc = 0x83052954; continue 'dispatch;
	}
	// 830528F0: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 830528F4: 807C0010  lwz r3, 0x10(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(16 as u32) ) } as u64;
	// 830528F8: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 830528FC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83052900: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83052904: 4BFC55AD  bl 0x83017eb0
	ctx.lr = 0x83052908;
	sub_83017EB0(ctx, base);
	// 83052908: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8305290C: 4082000C  bne 0x83052918
	if !ctx.cr[0].eq {
	pc = 0x83052918; continue 'dispatch;
	}
	// 83052910: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83052914: 48000010  b 0x83052924
	pc = 0x83052924; continue 'dispatch;
	// 83052918: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8305291C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83052920: 40820034  bne 0x83052954
	if !ctx.cr[0].eq {
	pc = 0x83052954; continue 'dispatch;
	}
	// 83052924: 807C000C  lwz r3, 0xc(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(12 as u32) ) } as u64;
	// 83052928: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8305292C: 41820028  beq 0x83052954
	if ctx.cr[0].eq {
	pc = 0x83052954; continue 'dispatch;
	}
	// 83052930: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 83052934: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 83052938: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8305293C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83052940: 4BFC5571  bl 0x83017eb0
	ctx.lr = 0x83052944;
	sub_83017EB0(ctx, base);
	// 83052944: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83052948: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8305294C: 41820008  beq 0x83052954
	if ctx.cr[0].eq {
	pc = 0x83052954; continue 'dispatch;
	}
	// 83052950: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83052954: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 83052958: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8305295C: 4815585C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83052960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83052960 size=8
    let mut pc: u32 = 0x83052960;
    'dispatch: loop {
        match pc {
            0x83052960 => {
    //   block [0x83052960..0x83052968)
	// 83052960: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83052964: 82163D10  lwz r16, 0x3d10(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(15632 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83052968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83052968 size=168
    let mut pc: u32 = 0x83052968;
    'dispatch: loop {
        match pc {
            0x83052968 => {
    //   block [0x83052968..0x83052A10)
	// 83052968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8305296C: 48155801  bl 0x831a816c
	ctx.lr = 0x83052970;
	sub_831A8130(ctx, base);
	// 83052970: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 83052974: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83052978: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8305297C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83052980: 54AB063F  clrlwi. r11, r5, 0x18
	ctx.r[11].u64 = ctx.r[5].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83052984: 93BF0094  stw r29, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[29].u32 ) };
	// 83052988: 41820064  beq 0x830529ec
	if ctx.cr[0].eq {
	pc = 0x830529EC; continue 'dispatch;
	}
	// 8305298C: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 83052990: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83052994: 409A003C  bne cr6, 0x830529d0
	if !ctx.cr[6].eq {
	pc = 0x830529D0; continue 'dispatch;
	}
	// 83052998: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 8305299C: 809D0034  lwz r4, 0x34(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(52 as u32) ) } as u64;
	// 830529A0: 4BF858F9  bl 0x82fd8298
	ctx.lr = 0x830529A4;
	sub_82FD8298(ctx, base);
	// 830529A4: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 830529A8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830529AC: 4182001C  beq 0x830529c8
	if ctx.cr[0].eq {
	pc = 0x830529C8; continue 'dispatch;
	}
	// 830529B0: 38C00080  li r6, 0x80
	ctx.r[6].s64 = 128;
	// 830529B4: 80FD0034  lwz r7, 0x34(r29)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(52 as u32) ) } as u64;
	// 830529B8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 830529BC: 3880001D  li r4, 0x1d
	ctx.r[4].s64 = 29;
	// 830529C0: 4BFC5A89  bl 0x83018448
	ctx.lr = 0x830529C4;
	sub_83018448(ctx, base);
	// 830529C4: 48000008  b 0x830529cc
	pc = 0x830529CC; continue 'dispatch;
	// 830529C8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830529CC: 907D000C  stw r3, 0xc(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 830529D0: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 830529D4: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 830529D8: 80DE0024  lwz r6, 0x24(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 830529DC: 807D000C  lwz r3, 0xc(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 830529E0: 80AB0020  lwz r5, 0x20(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 830529E4: 808B0010  lwz r4, 0x10(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 830529E8: 4800001C  b 0x83052a04
	pc = 0x83052A04; continue 'dispatch;
	// 830529EC: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 830529F0: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 830529F4: 80DE0024  lwz r6, 0x24(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 830529F8: 807D0008  lwz r3, 8(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 830529FC: 80AB0020  lwz r5, 0x20(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 83052A00: 808B0010  lwz r4, 0x10(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83052A04: 4BFC5B25  bl 0x83018528
	ctx.lr = 0x83052A08;
	sub_83018528(ctx, base);
	// 83052A08: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 83052A0C: 481557B0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83052A10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83052A10 size=48
    let mut pc: u32 = 0x83052A10;
    'dispatch: loop {
        match pc {
            0x83052A10 => {
    //   block [0x83052A10..0x83052A40)
	// 83052A10: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83052A14: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83052A18: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83052A1C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83052A20: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83052A24: 808B0034  lwz r4, 0x34(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 83052A28: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83052A2C: 4BF858B5  bl 0x82fd82e0
	ctx.lr = 0x83052A30;
	sub_82FD82E0(ctx, base);
	// 83052A30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83052A34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83052A38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83052A3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83052A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83052A40 size=52
    let mut pc: u32 = 0x83052A40;
    'dispatch: loop {
        match pc {
            0x83052A40 => {
    //   block [0x83052A40..0x83052A74)
	// 83052A40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83052A44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83052A48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83052A4C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83052A50: 80630014  lwz r3, 0x14(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 83052A54: 4BFF9ECD  bl 0x8304c920
	ctx.lr = 0x83052A58;
	sub_8304C920(ctx, base);
	// 83052A58: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83052A5C: 41820008  beq 0x83052a64
	if ctx.cr[0].eq {
	pc = 0x83052A64; continue 'dispatch;
	}
	// 83052A60: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83052A64: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83052A68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83052A6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83052A70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83052A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83052A78 size=8
    let mut pc: u32 = 0x83052A78;
    'dispatch: loop {
        match pc {
            0x83052A78 => {
    //   block [0x83052A78..0x83052A80)
	// 83052A78: 80630014  lwz r3, 0x14(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 83052A7C: 4BFFA584  b 0x8304d000
	sub_8304D000(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83052A80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83052A80 size=8
    let mut pc: u32 = 0x83052A80;
    'dispatch: loop {
        match pc {
            0x83052A80 => {
    //   block [0x83052A80..0x83052A88)
	// 83052A80: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83052A84: 82163D60  lwz r16, 0x3d60(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(15712 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83052A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83052A88 size=92
    let mut pc: u32 = 0x83052A88;
    'dispatch: loop {
        match pc {
            0x83052A88 => {
    //   block [0x83052A88..0x83052AE4)
	// 83052A88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83052A8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83052A90: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83052A94: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83052A98: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 83052A9C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83052AA0: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83052AA4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83052AA8: 396B3A88  addi r11, r11, 0x3a88
	ctx.r[11].s64 = ctx.r[11].s64 + 14984;
	// 83052AAC: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 83052AB0: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83052AB4: 4BFFF665  bl 0x83052118
	ctx.lr = 0x83052AB8;
	sub_83052118(ctx, base);
	// 83052AB8: 387E003C  addi r3, r30, 0x3c
	ctx.r[3].s64 = ctx.r[30].s64 + 60;
	// 83052ABC: 4BFBDEDD  bl 0x83010998
	ctx.lr = 0x83052AC0;
	sub_83010998(ctx, base);
	// 83052AC0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83052AC4: 396BA93C  addi r11, r11, -0x56c4
	ctx.r[11].s64 = ctx.r[11].s64 + -22212;
	// 83052AC8: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83052ACC: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 83052AD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83052AD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83052AD8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83052ADC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83052AE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83052AE4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83052AE4 size=40
    let mut pc: u32 = 0x83052AE4;
    'dispatch: loop {
        match pc {
            0x83052AE4 => {
    //   block [0x83052AE4..0x83052B0C)
	// 83052AE4: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 83052AE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83052AEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83052AF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83052AF4: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83052AF8: 4BFF9C69  bl 0x8304c760
	ctx.lr = 0x83052AFC;
	sub_8304C760(ctx, base);
	// 83052AFC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83052B00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83052B04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83052B08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83052B0C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83052B0C size=44
    let mut pc: u32 = 0x83052B0C;
    'dispatch: loop {
        match pc {
            0x83052B0C => {
    //   block [0x83052B0C..0x83052B38)
	// 83052B0C: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 83052B10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83052B14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83052B18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83052B1C: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83052B20: 386B003C  addi r3, r11, 0x3c
	ctx.r[3].s64 = ctx.r[11].s64 + 60;
	// 83052B24: 4BFBDE75  bl 0x83010998
	ctx.lr = 0x83052B28;
	sub_83010998(ctx, base);
	// 83052B28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83052B2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83052B30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83052B34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83052B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83052B38 size=8
    let mut pc: u32 = 0x83052B38;
    'dispatch: loop {
        match pc {
            0x83052B38 => {
    //   block [0x83052B38..0x83052B40)
	// 83052B38: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83052B3C: 82163DA0  lwz r16, 0x3da0(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(15776 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83052B40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83052B40 size=96
    let mut pc: u32 = 0x83052B40;
    'dispatch: loop {
        match pc {
            0x83052B40 => {
    //   block [0x83052B40..0x83052BA0)
	// 83052B40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83052B44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83052B48: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83052B4C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83052B50: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 83052B54: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83052B58: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83052B5C: 38600050  li r3, 0x50
	ctx.r[3].s64 = 80;
	// 83052B60: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83052B64: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 83052B68: 4BF85731  bl 0x82fd8298
	ctx.lr = 0x83052B6C;
	sub_82FD8298(ctx, base);
	// 83052B6C: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83052B70: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83052B74: 41820010  beq 0x83052b84
	if ctx.cr[0].eq {
	pc = 0x83052B84; continue 'dispatch;
	}
	// 83052B78: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83052B7C: 4BFFF765  bl 0x830522e0
	ctx.lr = 0x83052B80;
	sub_830522E0(ctx, base);
	// 83052B80: 48000008  b 0x83052b88
	pc = 0x83052B88; continue 'dispatch;
	// 83052B84: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83052B88: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 83052B8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83052B90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83052B94: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83052B98: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83052B9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83052BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83052BA0 size=44
    let mut pc: u32 = 0x83052BA0;
    'dispatch: loop {
        match pc {
            0x83052BA0 => {
    //   block [0x83052BA0..0x83052BCC)
	// 83052BA0: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 83052BA4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83052BA8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83052BAC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83052BB0: 809F0084  lwz r4, 0x84(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83052BB4: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83052BB8: 4BF85729  bl 0x82fd82e0
	ctx.lr = 0x83052BBC;
	sub_82FD82E0(ctx, base);
	// 83052BBC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83052BC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83052BC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83052BC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83052BD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83052BD0 size=76
    let mut pc: u32 = 0x83052BD0;
    'dispatch: loop {
        match pc {
            0x83052BD0 => {
    //   block [0x83052BD0..0x83052C1C)
	// 83052BD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83052BD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83052BD8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83052BDC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83052BE0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83052BE4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83052BE8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83052BEC: 4BFFFE9D  bl 0x83052a88
	ctx.lr = 0x83052BF0;
	sub_83052A88(ctx, base);
	// 83052BF0: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83052BF4: 4182000C  beq 0x83052c00
	if ctx.cr[0].eq {
	pc = 0x83052C00; continue 'dispatch;
	}
	// 83052BF8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83052BFC: 4BF856E5  bl 0x82fd82e0
	ctx.lr = 0x83052C00;
	sub_82FD82E0(ctx, base);
	// 83052C00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83052C04: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83052C08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83052C0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83052C10: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83052C14: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83052C18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83052C20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83052C20 size=8
    let mut pc: u32 = 0x83052C20;
    'dispatch: loop {
        match pc {
            0x83052C20 => {
    //   block [0x83052C20..0x83052C28)
	// 83052C20: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83052C24: 82163E18  lwz r16, 0x3e18(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(15896 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83052C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83052C28 size=100
    let mut pc: u32 = 0x83052C28;
    'dispatch: loop {
        match pc {
            0x83052C28 => {
    //   block [0x83052C28..0x83052C8C)
	// 83052C28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83052C2C: 4815553D  bl 0x831a8168
	ctx.lr = 0x83052C30;
	sub_831A8130(ctx, base);
	// 83052C30: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 83052C34: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83052C38: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83052C3C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 83052C40: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83052C44: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83052C48: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 83052C4C: 48047445  bl 0x8309a090
	ctx.lr = 0x83052C50;
	sub_8309A090(ctx, base);
	// 83052C50: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83052C54: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83052C58: 396B3DE8  addi r11, r11, 0x3de8
	ctx.r[11].s64 = ctx.r[11].s64 + 15848;
	// 83052C5C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 83052C60: 915E000C  stw r10, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 83052C64: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83052C68: 915E0008  stw r10, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 83052C6C: 419A0014  beq cr6, 0x83052c80
	if ctx.cr[6].eq {
	pc = 0x83052C80; continue 'dispatch;
	}
	// 83052C70: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83052C74: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83052C78: 4BF7DF09  bl 0x82fd0b80
	ctx.lr = 0x83052C7C;
	sub_82FD0B80(ctx, base);
	// 83052C7C: 907E0008  stw r3, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 83052C80: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83052C84: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 83052C88: 48155530  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83052C8C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83052C8C size=40
    let mut pc: u32 = 0x83052C8C;
    'dispatch: loop {
        match pc {
            0x83052C8C => {
    //   block [0x83052C8C..0x83052CB4)
	// 83052C8C: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83052C90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83052C94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83052C98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83052C9C: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83052CA0: 48047389  bl 0x8309a028
	ctx.lr = 0x83052CA4;
	sub_8309A028(ctx, base);
	// 83052CA4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83052CA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83052CAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83052CB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83052CB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83052CB8 size=8
    let mut pc: u32 = 0x83052CB8;
    'dispatch: loop {
        match pc {
            0x83052CB8 => {
    //   block [0x83052CB8..0x83052CC0)
	// 83052CB8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83052CBC: 82163E50  lwz r16, 0x3e50(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(15952 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83052CC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83052CC0 size=140
    let mut pc: u32 = 0x83052CC0;
    'dispatch: loop {
        match pc {
            0x83052CC0 => {
    //   block [0x83052CC0..0x83052D4C)
	// 83052CC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83052CC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83052CC8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83052CCC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83052CD0: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 83052CD4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83052CD8: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83052CDC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83052CE0: 396B3DE8  addi r11, r11, 0x3de8
	ctx.r[11].s64 = ctx.r[11].s64 + 15848;
	// 83052CE4: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 83052CE8: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83052CEC: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83052CF0: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83052CF4: 41820018  beq 0x83052d0c
	if ctx.cr[0].eq {
	pc = 0x83052D0C; continue 'dispatch;
	}
	// 83052CF8: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83052CFC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83052D00: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83052D04: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83052D08: 4E800421  bctrl
	ctx.lr = 0x83052D0C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83052D0C: 809E000C  lwz r4, 0xc(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83052D10: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83052D14: 41820018  beq 0x83052d2c
	if ctx.cr[0].eq {
	pc = 0x83052D2C; continue 'dispatch;
	}
	// 83052D18: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83052D1C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83052D20: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83052D24: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83052D28: 4E800421  bctrl
	ctx.lr = 0x83052D2C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83052D2C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83052D30: 480472F9  bl 0x8309a028
	ctx.lr = 0x83052D34;
	sub_8309A028(ctx, base);
	// 83052D34: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 83052D38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83052D3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83052D40: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83052D44: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83052D48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83052D4C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83052D4C size=40
    let mut pc: u32 = 0x83052D4C;
    'dispatch: loop {
        match pc {
            0x83052D4C => {
    //   block [0x83052D4C..0x83052D74)
	// 83052D4C: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 83052D50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83052D54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83052D58: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83052D5C: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83052D60: 480472C9  bl 0x8309a028
	ctx.lr = 0x83052D64;
	sub_8309A028(ctx, base);
	// 83052D64: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83052D68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83052D6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83052D70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83052D78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83052D78 size=116
    let mut pc: u32 = 0x83052D78;
    'dispatch: loop {
        match pc {
            0x83052D78 => {
    //   block [0x83052D78..0x83052DEC)
	// 83052D78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83052D7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83052D80: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83052D84: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83052D88: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83052D8C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83052D90: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83052D94: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83052D98: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83052D9C: 41820020  beq 0x83052dbc
	if ctx.cr[0].eq {
	pc = 0x83052DBC; continue 'dispatch;
	}
	// 83052DA0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83052DA4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83052DA8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83052DAC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83052DB0: 4E800421  bctrl
	ctx.lr = 0x83052DB4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83052DB4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83052DB8: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 83052DBC: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83052DC0: 419A0014  beq cr6, 0x83052dd4
	if ctx.cr[6].eq {
	pc = 0x83052DD4; continue 'dispatch;
	}
	// 83052DC4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83052DC8: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83052DCC: 4BF7DDB5  bl 0x82fd0b80
	ctx.lr = 0x83052DD0;
	sub_82FD0B80(ctx, base);
	// 83052DD0: 907F000C  stw r3, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 83052DD4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83052DD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83052DDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83052DE0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83052DE4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83052DE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83052DF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83052DF0 size=116
    let mut pc: u32 = 0x83052DF0;
    'dispatch: loop {
        match pc {
            0x83052DF0 => {
    //   block [0x83052DF0..0x83052E64)
	// 83052DF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83052DF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83052DF8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83052DFC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83052E00: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83052E04: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83052E08: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83052E0C: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83052E10: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83052E14: 41820020  beq 0x83052e34
	if ctx.cr[0].eq {
	pc = 0x83052E34; continue 'dispatch;
	}
	// 83052E18: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83052E1C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83052E20: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83052E24: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83052E28: 4E800421  bctrl
	ctx.lr = 0x83052E2C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83052E2C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83052E30: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83052E34: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83052E38: 419A0014  beq cr6, 0x83052e4c
	if ctx.cr[6].eq {
	pc = 0x83052E4C; continue 'dispatch;
	}
	// 83052E3C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83052E40: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83052E44: 4BF7DD3D  bl 0x82fd0b80
	ctx.lr = 0x83052E48;
	sub_82FD0B80(ctx, base);
	// 83052E48: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 83052E4C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83052E50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83052E54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83052E58: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83052E5C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83052E60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83052E68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83052E68 size=12
    let mut pc: u32 = 0x83052E68;
    'dispatch: loop {
        match pc {
            0x83052E68 => {
    //   block [0x83052E68..0x83052E74)
	// 83052E68: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 83052E6C: 386B340C  addi r3, r11, 0x340c
	ctx.r[3].s64 = ctx.r[11].s64 + 13324;
	// 83052E70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83052E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83052E78 size=132
    let mut pc: u32 = 0x83052E78;
    'dispatch: loop {
        match pc {
            0x83052E78 => {
    //   block [0x83052E78..0x83052EFC)
	// 83052E78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83052E7C: 481552F1  bl 0x831a816c
	ctx.lr = 0x83052E80;
	sub_831A8130(ctx, base);
	// 83052E80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83052E84: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83052E88: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83052E8C: 48047255  bl 0x8309a0e0
	ctx.lr = 0x83052E90;
	sub_8309A0E0(ctx, base);
	// 83052E90: A97D0000  lha r11, 0(r29)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as i16) as i64;
	// 83052E94: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83052E98: 556BDFFF  rlwinm. r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83052E9C: 4182001C  beq 0x83052eb8
	if ctx.cr[0].eq {
	pc = 0x83052EB8; continue 'dispatch;
	}
	// 83052EA0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83052EA4: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83052EA8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83052EAC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83052EB0: 4BFA6A51  bl 0x82ff9900
	ctx.lr = 0x83052EB4;
	sub_82FF9900(ctx, base);
	// 83052EB4: 48000040  b 0x83052ef4
	pc = 0x83052EF4; continue 'dispatch;
	// 83052EB8: 3BDF000C  addi r30, r31, 0xc
	ctx.r[30].s64 = ctx.r[31].s64 + 12;
	// 83052EBC: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83052EC0: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83052EC4: 41820018  beq 0x83052edc
	if ctx.cr[0].eq {
	pc = 0x83052EDC; continue 'dispatch;
	}
	// 83052EC8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83052ECC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83052ED0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83052ED4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83052ED8: 4E800421  bctrl
	ctx.lr = 0x83052EDC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83052EDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83052EE0: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 83052EE4: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 83052EE8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83052EEC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83052EF0: 4BFA6C39  bl 0x82ff9b28
	ctx.lr = 0x83052EF4;
	sub_82FF9B28(ctx, base);
	// 83052EF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83052EF8: 481552C4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83052F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83052F00 size=72
    let mut pc: u32 = 0x83052F00;
    'dispatch: loop {
        match pc {
            0x83052F00 => {
    //   block [0x83052F00..0x83052F48)
	// 83052F00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83052F04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83052F08: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83052F0C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83052F10: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83052F14: 4804717D  bl 0x8309a090
	ctx.lr = 0x83052F18;
	sub_8309A090(ctx, base);
	// 83052F18: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83052F1C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83052F20: 396B3DE8  addi r11, r11, 0x3de8
	ctx.r[11].s64 = ctx.r[11].s64 + 15848;
	// 83052F24: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83052F28: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 83052F2C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83052F30: 915F000C  stw r10, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 83052F34: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83052F38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83052F3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83052F40: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83052F44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83052F48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83052F48 size=76
    let mut pc: u32 = 0x83052F48;
    'dispatch: loop {
        match pc {
            0x83052F48 => {
    //   block [0x83052F48..0x83052F94)
	// 83052F48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83052F4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83052F50: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83052F54: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83052F58: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83052F5C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83052F60: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83052F64: 4BFFFD5D  bl 0x83052cc0
	ctx.lr = 0x83052F68;
	sub_83052CC0(ctx, base);
	// 83052F68: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83052F6C: 4182000C  beq 0x83052f78
	if ctx.cr[0].eq {
	pc = 0x83052F78; continue 'dispatch;
	}
	// 83052F70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83052F74: 4BF8536D  bl 0x82fd82e0
	ctx.lr = 0x83052F78;
	sub_82FD82E0(ctx, base);
	// 83052F78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83052F7C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83052F80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83052F84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83052F88: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83052F8C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83052F90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83052F98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83052F98 size=8
    let mut pc: u32 = 0x83052F98;
    'dispatch: loop {
        match pc {
            0x83052F98 => {
    //   block [0x83052F98..0x83052FA0)
	// 83052F98: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83052F9C: 82163E90  lwz r16, 0x3e90(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(16016 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83052FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83052FA0 size=96
    let mut pc: u32 = 0x83052FA0;
    'dispatch: loop {
        match pc {
            0x83052FA0 => {
    //   block [0x83052FA0..0x83053000)
	// 83052FA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83052FA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83052FA8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83052FAC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83052FB0: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 83052FB4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83052FB8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83052FBC: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 83052FC0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83052FC4: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 83052FC8: 4BF852D1  bl 0x82fd8298
	ctx.lr = 0x83052FCC;
	sub_82FD8298(ctx, base);
	// 83052FCC: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83052FD0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83052FD4: 41820010  beq 0x83052fe4
	if ctx.cr[0].eq {
	pc = 0x83052FE4; continue 'dispatch;
	}
	// 83052FD8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83052FDC: 4BFFFF25  bl 0x83052f00
	ctx.lr = 0x83052FE0;
	sub_83052F00(ctx, base);
	// 83052FE0: 48000008  b 0x83052fe8
	pc = 0x83052FE8; continue 'dispatch;
	// 83052FE4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83052FE8: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 83052FEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83052FF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83052FF4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83052FF8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83052FFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83053000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83053000 size=44
    let mut pc: u32 = 0x83053000;
    'dispatch: loop {
        match pc {
            0x83053000 => {
    //   block [0x83053000..0x8305302C)
	// 83053000: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 83053004: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83053008: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8305300C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83053010: 809F0084  lwz r4, 0x84(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83053014: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83053018: 4BF852C9  bl 0x82fd82e0
	ctx.lr = 0x8305301C;
	sub_82FD82E0(ctx, base);
	// 8305301C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83053020: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83053024: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83053028: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83053030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83053030 size=8
    let mut pc: u32 = 0x83053030;
    'dispatch: loop {
        match pc {
            0x83053030 => {
    //   block [0x83053030..0x83053038)
	// 83053030: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83053034: 82163F30  lwz r16, 0x3f30(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(16176 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83053038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83053038 size=204
    let mut pc: u32 = 0x83053038;
    'dispatch: loop {
        match pc {
            0x83053038 => {
    //   block [0x83053038..0x83053104)
	// 83053038: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8305303C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83053040: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83053044: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83053048: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 8305304C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83053050: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83053054: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83053058: 396B3EE0  addi r11, r11, 0x3ee0
	ctx.r[11].s64 = ctx.r[11].s64 + 16096;
	// 8305305C: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 83053060: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83053064: 809E000C  lwz r4, 0xc(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83053068: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8305306C: 41820018  beq 0x83053084
	if ctx.cr[0].eq {
	pc = 0x83053084; continue 'dispatch;
	}
	// 83053070: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83053074: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83053078: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8305307C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83053080: 4E800421  bctrl
	ctx.lr = 0x83053084;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83053084: 807E0010  lwz r3, 0x10(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83053088: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8305308C: 41820018  beq 0x830530a4
	if ctx.cr[0].eq {
	pc = 0x830530A4; continue 'dispatch;
	}
	// 83053090: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83053094: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83053098: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8305309C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830530A0: 4E800421  bctrl
	ctx.lr = 0x830530A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830530A4: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 830530A8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830530AC: 41820018  beq 0x830530c4
	if ctx.cr[0].eq {
	pc = 0x830530C4; continue 'dispatch;
	}
	// 830530B0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830530B4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830530B8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830530BC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830530C0: 4E800421  bctrl
	ctx.lr = 0x830530C4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830530C4: 807E0018  lwz r3, 0x18(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 830530C8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830530CC: 41820018  beq 0x830530e4
	if ctx.cr[0].eq {
	pc = 0x830530E4; continue 'dispatch;
	}
	// 830530D0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830530D4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830530D8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830530DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830530E0: 4E800421  bctrl
	ctx.lr = 0x830530E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830530E4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830530E8: 48047469  bl 0x8309a550
	ctx.lr = 0x830530EC;
	sub_8309A550(ctx, base);
	// 830530EC: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 830530F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830530F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830530F8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830530FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83053100: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83053104(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83053104 size=40
    let mut pc: u32 = 0x83053104;
    'dispatch: loop {
        match pc {
            0x83053104 => {
    //   block [0x83053104..0x8305312C)
	// 83053104: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 83053108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8305310C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83053110: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83053114: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83053118: 48047439  bl 0x8309a550
	ctx.lr = 0x8305311C;
	sub_8309A550(ctx, base);
	// 8305311C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83053120: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83053124: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83053128: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83053130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83053130 size=16
    let mut pc: u32 = 0x83053130;
    'dispatch: loop {
        match pc {
            0x83053130 => {
    //   block [0x83053130..0x83053140)
	// 83053130: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83053134: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 83053138: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8305313C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83053140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83053140 size=8
    let mut pc: u32 = 0x83053140;
    'dispatch: loop {
        match pc {
            0x83053140 => {
    //   block [0x83053140..0x83053148)
	// 83053140: 80630014  lwz r3, 0x14(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 83053144: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83053148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83053148 size=8
    let mut pc: u32 = 0x83053148;
    'dispatch: loop {
        match pc {
            0x83053148 => {
    //   block [0x83053148..0x83053150)
	// 83053148: 80630018  lwz r3, 0x18(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 8305314C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83053150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83053150 size=8
    let mut pc: u32 = 0x83053150;
    'dispatch: loop {
        match pc {
            0x83053150 => {
    //   block [0x83053150..0x83053158)
	// 83053150: 90830008  stw r4, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 83053154: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83053158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83053158 size=100
    let mut pc: u32 = 0x83053158;
    'dispatch: loop {
        match pc {
            0x83053158 => {
    //   block [0x83053158..0x830531BC)
	// 83053158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8305315C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83053160: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83053164: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83053168: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8305316C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83053170: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83053174: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83053178: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8305317C: 41820018  beq 0x83053194
	if ctx.cr[0].eq {
	pc = 0x83053194; continue 'dispatch;
	}
	// 83053180: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83053184: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83053188: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8305318C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83053190: 4E800421  bctrl
	ctx.lr = 0x83053194;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83053194: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83053198: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8305319C: 4BF7D9E5  bl 0x82fd0b80
	ctx.lr = 0x830531A0;
	sub_82FD0B80(ctx, base);
	// 830531A0: 907F000C  stw r3, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 830531A4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830531A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830531AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830531B0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830531B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830531B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830531C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830531C0 size=8
    let mut pc: u32 = 0x830531C0;
    'dispatch: loop {
        match pc {
            0x830531C0 => {
    //   block [0x830531C0..0x830531C8)
	// 830531C0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830531C4: 82163F70  lwz r16, 0x3f70(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(16240 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830531C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830531C8 size=112
    let mut pc: u32 = 0x830531C8;
    'dispatch: loop {
        match pc {
            0x830531C8 => {
    //   block [0x830531C8..0x83053238)
	// 830531C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830531CC: 48154FA1  bl 0x831a816c
	ctx.lr = 0x830531D0;
	sub_831A8130(ctx, base);
	// 830531D0: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 830531D4: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830531D8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830531DC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830531E0: 807D0014  lwz r3, 0x14(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) } as u64;
	// 830531E4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830531E8: 41820018  beq 0x83053200
	if ctx.cr[0].eq {
	pc = 0x83053200; continue 'dispatch;
	}
	// 830531EC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830531F0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830531F4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830531F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830531FC: 4E800421  bctrl
	ctx.lr = 0x83053200;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83053200: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83053204: 38600024  li r3, 0x24
	ctx.r[3].s64 = 36;
	// 83053208: 909F0050  stw r4, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[4].u32 ) };
	// 8305320C: 4BF8508D  bl 0x82fd8298
	ctx.lr = 0x83053210;
	sub_82FD8298(ctx, base);
	// 83053210: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 83053214: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83053218: 41820010  beq 0x83053228
	if ctx.cr[0].eq {
	pc = 0x83053228; continue 'dispatch;
	}
	// 8305321C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83053220: 4BF8AFD1  bl 0x82fde1f0
	ctx.lr = 0x83053224;
	sub_82FDE1F0(ctx, base);
	// 83053224: 48000008  b 0x8305322c
	pc = 0x8305322C; continue 'dispatch;
	// 83053228: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8305322C: 907D0014  stw r3, 0x14(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 83053230: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 83053234: 48154F88  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83053238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83053238 size=44
    let mut pc: u32 = 0x83053238;
    'dispatch: loop {
        match pc {
            0x83053238 => {
    //   block [0x83053238..0x83053264)
	// 83053238: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 8305323C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83053240: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83053244: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83053248: 809F0050  lwz r4, 0x50(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8305324C: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 83053250: 4BF85091  bl 0x82fd82e0
	ctx.lr = 0x83053254;
	sub_82FD82E0(ctx, base);
	// 83053254: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83053258: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8305325C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83053260: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83053268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83053268 size=8
    let mut pc: u32 = 0x83053268;
    'dispatch: loop {
        match pc {
            0x83053268 => {
    //   block [0x83053268..0x83053270)
	// 83053268: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8305326C: 82163FA8  lwz r16, 0x3fa8(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(16296 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83053270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83053270 size=112
    let mut pc: u32 = 0x83053270;
    'dispatch: loop {
        match pc {
            0x83053270 => {
    //   block [0x83053270..0x830532E0)
	// 83053270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83053274: 48154EF9  bl 0x831a816c
	ctx.lr = 0x83053278;
	sub_831A8130(ctx, base);
	// 83053278: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 8305327C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83053280: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83053284: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83053288: 807D0018  lwz r3, 0x18(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 8305328C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83053290: 41820018  beq 0x830532a8
	if ctx.cr[0].eq {
	pc = 0x830532A8; continue 'dispatch;
	}
	// 83053294: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83053298: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8305329C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830532A0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830532A4: 4E800421  bctrl
	ctx.lr = 0x830532A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830532A8: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830532AC: 38600024  li r3, 0x24
	ctx.r[3].s64 = 36;
	// 830532B0: 909F0050  stw r4, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[4].u32 ) };
	// 830532B4: 4BF84FE5  bl 0x82fd8298
	ctx.lr = 0x830532B8;
	sub_82FD8298(ctx, base);
	// 830532B8: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 830532BC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830532C0: 41820010  beq 0x830532d0
	if ctx.cr[0].eq {
	pc = 0x830532D0; continue 'dispatch;
	}
	// 830532C4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830532C8: 4BF8AF29  bl 0x82fde1f0
	ctx.lr = 0x830532CC;
	sub_82FDE1F0(ctx, base);
	// 830532CC: 48000008  b 0x830532d4
	pc = 0x830532D4; continue 'dispatch;
	// 830532D0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830532D4: 907D0018  stw r3, 0x18(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(24 as u32), ctx.r[3].u32 ) };
	// 830532D8: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 830532DC: 48154EE0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830532E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830532E0 size=44
    let mut pc: u32 = 0x830532E0;
    'dispatch: loop {
        match pc {
            0x830532E0 => {
    //   block [0x830532E0..0x8305330C)
	// 830532E0: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830532E4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830532E8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830532EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830532F0: 809F0050  lwz r4, 0x50(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830532F4: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 830532F8: 4BF84FE9  bl 0x82fd82e0
	ctx.lr = 0x830532FC;
	sub_82FD82E0(ctx, base);
	// 830532FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83053300: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83053304: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83053308: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83053310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83053310 size=8
    let mut pc: u32 = 0x83053310;
    'dispatch: loop {
        match pc {
            0x83053310 => {
    //   block [0x83053310..0x83053318)
	// 83053310: 9083001C  stw r4, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[4].u32 ) };
	// 83053314: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83053318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83053318 size=12
    let mut pc: u32 = 0x83053318;
    'dispatch: loop {
        match pc {
            0x83053318 => {
    //   block [0x83053318..0x83053324)
	// 83053318: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8305331C: 386B3414  addi r3, r11, 0x3414
	ctx.r[3].s64 = ctx.r[11].s64 + 13332;
	// 83053320: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83053328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83053328 size=296
    let mut pc: u32 = 0x83053328;
    'dispatch: loop {
        match pc {
            0x83053328 => {
    //   block [0x83053328..0x83053450)
	// 83053328: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8305332C: 48154E41  bl 0x831a816c
	ctx.lr = 0x83053330;
	sub_831A8130(ctx, base);
	// 83053330: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83053334: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83053338: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8305333C: 48046DA5  bl 0x8309a0e0
	ctx.lr = 0x83053340;
	sub_8309A0E0(ctx, base);
	// 83053340: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83053344: A97E0000  lha r11, 0(r30)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as i16) as i64;
	// 83053348: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8305334C: 556BDFFF  rlwinm. r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83053350: 41820054  beq 0x830533a4
	if ctx.cr[0].eq {
	pc = 0x830533A4; continue 'dispatch;
	}
	// 83053354: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83053358: 4BFA5FA1  bl 0x82ff92f8
	ctx.lr = 0x8305335C;
	sub_82FF92F8(ctx, base);
	// 8305335C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83053360: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83053364: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83053368: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8305336C: 4BFA6595  bl 0x82ff9900
	ctx.lr = 0x83053370;
	sub_82FF9900(ctx, base);
	// 83053370: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83053374: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83053378: 4BFF9749  bl 0x8304cac0
	ctx.lr = 0x8305337C;
	sub_8304CAC0(ctx, base);
	// 8305337C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83053380: 809F0014  lwz r4, 0x14(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83053384: 4BFA687D  bl 0x82ff9c00
	ctx.lr = 0x83053388;
	sub_82FF9C00(ctx, base);
	// 83053388: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8305338C: 809F0018  lwz r4, 0x18(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83053390: 4BFA6871  bl 0x82ff9c00
	ctx.lr = 0x83053394;
	sub_82FF9C00(ctx, base);
	// 83053394: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83053398: 809F001C  lwz r4, 0x1c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 8305339C: 4BFA6865  bl 0x82ff9c00
	ctx.lr = 0x830533A0;
	sub_82FF9C00(ctx, base);
	// 830533A0: 480000A8  b 0x83053448
	pc = 0x83053448; continue 'dispatch;
	// 830533A4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830533A8: 4BFA61D1  bl 0x82ff9578
	ctx.lr = 0x830533AC;
	sub_82FF9578(ctx, base);
	// 830533AC: 3BBF000C  addi r29, r31, 0xc
	ctx.r[29].s64 = ctx.r[31].s64 + 12;
	// 830533B0: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830533B4: 809D0000  lwz r4, 0(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830533B8: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 830533BC: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830533C0: 41820018  beq 0x830533d8
	if ctx.cr[0].eq {
	pc = 0x830533D8; continue 'dispatch;
	}
	// 830533C4: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830533C8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830533CC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830533D0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830533D4: 4E800421  bctrl
	ctx.lr = 0x830533D8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830533D8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 830533DC: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 830533E0: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 830533E4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830533E8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830533EC: 4BFA673D  bl 0x82ff9b28
	ctx.lr = 0x830533F0;
	sub_82FF9B28(ctx, base);
	// 830533F0: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 830533F4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 830533F8: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 830533FC: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 83053400: 4BFFA179  bl 0x8304d578
	ctx.lr = 0x83053404;
	sub_8304D578(ctx, base);
	// 83053404: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 83053408: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8305340C: 3BAB2E10  addi r29, r11, 0x2e10
	ctx.r[29].s64 = ctx.r[11].s64 + 11792;
	// 83053410: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83053414: 4BFA68AD  bl 0x82ff9cc0
	ctx.lr = 0x83053418;
	sub_82FF9CC0(ctx, base);
	// 83053418: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8305341C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83053420: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83053424: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 83053428: 4BFA6899  bl 0x82ff9cc0
	ctx.lr = 0x8305342C;
	sub_82FF9CC0(ctx, base);
	// 8305342C: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 83053430: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 83053434: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83053438: 388B35F0  addi r4, r11, 0x35f0
	ctx.r[4].s64 = ctx.r[11].s64 + 13808;
	// 8305343C: 915F0018  stw r10, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 83053440: 4BFA6881  bl 0x82ff9cc0
	ctx.lr = 0x83053444;
	sub_82FF9CC0(ctx, base);
	// 83053444: 907F001C  stw r3, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[3].u32 ) };
	// 83053448: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8305344C: 48154D70  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83053450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83053450 size=92
    let mut pc: u32 = 0x83053450;
    'dispatch: loop {
        match pc {
            0x83053450 => {
    //   block [0x83053450..0x830534AC)
	// 83053450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83053454: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83053458: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8305345C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83053460: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83053464: 48047155  bl 0x8309a5b8
	ctx.lr = 0x83053468;
	sub_8309A5B8(ctx, base);
	// 83053468: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8305346C: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 83053470: 394B3EE0  addi r10, r11, 0x3ee0
	ctx.r[10].s64 = ctx.r[11].s64 + 16096;
	// 83053474: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83053478: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8305347C: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 83053480: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83053484: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 83053488: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8305348C: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 83053490: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 83053494: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 83053498: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8305349C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830534A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830534A4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830534A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830534B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830534B0 size=76
    let mut pc: u32 = 0x830534B0;
    'dispatch: loop {
        match pc {
            0x830534B0 => {
    //   block [0x830534B0..0x830534FC)
	// 830534B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830534B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830534B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830534BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830534C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830534C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830534C8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830534CC: 4BFFFB6D  bl 0x83053038
	ctx.lr = 0x830534D0;
	sub_83053038(ctx, base);
	// 830534D0: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830534D4: 4182000C  beq 0x830534e0
	if ctx.cr[0].eq {
	pc = 0x830534E0; continue 'dispatch;
	}
	// 830534D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830534DC: 4BF84E05  bl 0x82fd82e0
	ctx.lr = 0x830534E0;
	sub_82FD82E0(ctx, base);
	// 830534E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830534E4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830534E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830534EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830534F0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830534F4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830534F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83053500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83053500 size=8
    let mut pc: u32 = 0x83053500;
    'dispatch: loop {
        match pc {
            0x83053500 => {
    //   block [0x83053500..0x83053508)
	// 83053500: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83053504: 82163FE0  lwz r16, 0x3fe0(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(16352 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83053508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83053508 size=96
    let mut pc: u32 = 0x83053508;
    'dispatch: loop {
        match pc {
            0x83053508 => {
    //   block [0x83053508..0x83053568)
	// 83053508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8305350C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83053510: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83053514: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83053518: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 8305351C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83053520: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83053524: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 83053528: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8305352C: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 83053530: 4BF84D69  bl 0x82fd8298
	ctx.lr = 0x83053534;
	sub_82FD8298(ctx, base);
	// 83053534: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83053538: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8305353C: 41820010  beq 0x8305354c
	if ctx.cr[0].eq {
	pc = 0x8305354C; continue 'dispatch;
	}
	// 83053540: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83053544: 4BFFFF0D  bl 0x83053450
	ctx.lr = 0x83053548;
	sub_83053450(ctx, base);
	// 83053548: 48000008  b 0x83053550
	pc = 0x83053550; continue 'dispatch;
	// 8305354C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83053550: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 83053554: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83053558: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8305355C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83053560: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83053564: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83053568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83053568 size=44
    let mut pc: u32 = 0x83053568;
    'dispatch: loop {
        match pc {
            0x83053568 => {
    //   block [0x83053568..0x83053594)
	// 83053568: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 8305356C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83053570: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83053574: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83053578: 809F0084  lwz r4, 0x84(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 8305357C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83053580: 4BF84D61  bl 0x82fd82e0
	ctx.lr = 0x83053584;
	sub_82FD82E0(ctx, base);
	// 83053584: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83053588: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8305358C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83053590: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83053598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83053598 size=8
    let mut pc: u32 = 0x83053598;
    'dispatch: loop {
        match pc {
            0x83053598 => {
    //   block [0x83053598..0x830535A0)
	// 83053598: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8305359C: 82164020  lwz r16, 0x4020(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(16416 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830535A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830535A0 size=188
    let mut pc: u32 = 0x830535A0;
    'dispatch: loop {
        match pc {
            0x830535A0 => {
    //   block [0x830535A0..0x8305365C)
	// 830535A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830535A4: 48154BC1  bl 0x831a8164
	ctx.lr = 0x830535A8;
	sub_831A8130(ctx, base);
	// 830535A8: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 830535AC: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830535B0: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 830535B4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830535B8: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 830535BC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830535C0: 939F00B4  stw r28, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[28].u32 ) };
	// 830535C4: 93DF00A4  stw r30, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[30].u32 ) };
	// 830535C8: 48046FF1  bl 0x8309a5b8
	ctx.lr = 0x830535CC;
	sub_8309A5B8(ctx, base);
	// 830535CC: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 830535D0: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 830535D4: 396B3EE0  addi r11, r11, 0x3ee0
	ctx.r[11].s64 = ctx.r[11].s64 + 16096;
	// 830535D8: 39400008  li r10, 8
	ctx.r[10].s64 = 8;
	// 830535DC: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 830535E0: 93BE0014  stw r29, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[29].u32 ) };
	// 830535E4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830535E8: 915E0008  stw r10, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 830535EC: 93BE0018  stw r29, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[29].u32 ) };
	// 830535F0: 93BE001C  stw r29, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[29].u32 ) };
	// 830535F4: 93BE000C  stw r29, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 830535F8: 93BE0010  stw r29, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[29].u32 ) };
	// 830535FC: 419A0014  beq cr6, 0x83053610
	if ctx.cr[6].eq {
	pc = 0x83053610; continue 'dispatch;
	}
	// 83053600: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83053604: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83053608: 4BF7D579  bl 0x82fd0b80
	ctx.lr = 0x8305360C;
	sub_82FD0B80(ctx, base);
	// 8305360C: 907E000C  stw r3, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 83053610: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83053614: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 83053618: 4BF84C81  bl 0x82fd8298
	ctx.lr = 0x8305361C;
	sub_82FD8298(ctx, base);
	// 8305361C: 7C7B1B79  or. r27, r3, r3
	ctx.r[27].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 83053620: 937F0050  stw r27, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[27].u32 ) };
	// 83053624: 41820028  beq 0x8305364c
	if ctx.cr[0].eq {
	pc = 0x8305364C; continue 'dispatch;
	}
	// 83053628: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 8305362C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83053630: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 83053634: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83053638: 4BF7F409  bl 0x82fd2a40
	ctx.lr = 0x8305363C;
	sub_82FD2A40(ctx, base);
	// 8305363C: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 83053640: 7F7DDB78  mr r29, r27
	ctx.r[29].u64 = ctx.r[27].u64;
	// 83053644: 396B6C68  addi r11, r11, 0x6c68
	ctx.r[11].s64 = ctx.r[11].s64 + 27752;
	// 83053648: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8305364C: 93BE0010  stw r29, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[29].u32 ) };
	// 83053650: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83053654: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 83053658: 48154B5C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8305365C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8305365C size=40
    let mut pc: u32 = 0x8305365C;
    'dispatch: loop {
        match pc {
            0x8305365C => {
    //   block [0x8305365C..0x83053684)
	// 8305365C: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 83053660: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83053664: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83053668: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8305366C: 807F00A4  lwz r3, 0xa4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 83053670: 48046EE1  bl 0x8309a550
	ctx.lr = 0x83053674;
	sub_8309A550(ctx, base);
	// 83053674: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83053678: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8305367C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83053680: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83053684(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83053684 size=44
    let mut pc: u32 = 0x83053684;
    'dispatch: loop {
        match pc {
            0x83053684 => {
    //   block [0x83053684..0x830536B0)
	// 83053684: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 83053688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8305368C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83053690: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83053694: 809F00B4  lwz r4, 0xb4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 83053698: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8305369C: 4BF84C45  bl 0x82fd82e0
	ctx.lr = 0x830536A0;
	sub_82FD82E0(ctx, base);
	// 830536A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830536A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830536A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830536AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830536B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830536B0 size=64
    let mut pc: u32 = 0x830536B0;
    'dispatch: loop {
        match pc {
            0x830536B0 => {
    //   block [0x830536B0..0x830536F0)
	// 830536B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830536B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830536B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830536BC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830536C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830536C4: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 830536C8: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830536CC: 4BF7D4B5  bl 0x82fd0b80
	ctx.lr = 0x830536D0;
	sub_82FD0B80(ctx, base);
	// 830536D0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830536D4: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830536D8: 4BFE7A79  bl 0x8303b150
	ctx.lr = 0x830536DC;
	sub_8303B150(ctx, base);
	// 830536DC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830536E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830536E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830536E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830536EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830536F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830536F0 size=8
    let mut pc: u32 = 0x830536F0;
    'dispatch: loop {
        match pc {
            0x830536F0 => {
    //   block [0x830536F0..0x830536F8)
	// 830536F0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 830536F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830536F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830536F8 size=12
    let mut pc: u32 = 0x830536F8;
    'dispatch: loop {
        match pc {
            0x830536F8 => {
    //   block [0x830536F8..0x83053704)
	// 830536F8: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 830536FC: 386B341C  addi r3, r11, 0x341c
	ctx.r[3].s64 = ctx.r[11].s64 + 13340;
	// 83053700: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83053708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83053708 size=160
    let mut pc: u32 = 0x83053708;
    'dispatch: loop {
        match pc {
            0x83053708 => {
    //   block [0x83053708..0x830537A8)
	// 83053708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8305370C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83053710: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83053714: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83053718: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8305371C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83053720: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83053724: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83053728: A97F0000  lha r11, 0(r31)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as i16) as i64;
	// 8305372C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83053730: 556BDFFF  rlwinm. r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83053734: 41820030  beq 0x83053764
	if ctx.cr[0].eq {
	pc = 0x83053764; continue 'dispatch;
	}
	// 83053738: 889E0004  lbz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8305373C: 4BFA5AC5  bl 0x82ff9200
	ctx.lr = 0x83053740;
	sub_82FF9200(ctx, base);
	// 83053740: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83053744: 889E0005  lbz r4, 5(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(5 as u32) ) } as u64;
	// 83053748: 4BFA5AB9  bl 0x82ff9200
	ctx.lr = 0x8305374C;
	sub_82FF9200(ctx, base);
	// 8305374C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83053750: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83053754: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83053758: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8305375C: 4BFA61A5  bl 0x82ff9900
	ctx.lr = 0x83053760;
	sub_82FF9900(ctx, base);
	// 83053760: 48000030  b 0x83053790
	pc = 0x83053790; continue 'dispatch;
	// 83053764: 389E0004  addi r4, r30, 4
	ctx.r[4].s64 = ctx.r[30].s64 + 4;
	// 83053768: 4BFA5D19  bl 0x82ff9480
	ctx.lr = 0x8305376C;
	sub_82FF9480(ctx, base);
	// 8305376C: 389E0005  addi r4, r30, 5
	ctx.r[4].s64 = ctx.r[30].s64 + 5;
	// 83053770: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83053774: 4BFA5D0D  bl 0x82ff9480
	ctx.lr = 0x83053778;
	sub_82FF9480(ctx, base);
	// 83053778: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8305377C: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 83053780: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 83053784: 389E0008  addi r4, r30, 8
	ctx.r[4].s64 = ctx.r[30].s64 + 8;
	// 83053788: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8305378C: 4BFA639D  bl 0x82ff9b28
	ctx.lr = 0x83053790;
	sub_82FF9B28(ctx, base);
	// 83053790: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83053794: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83053798: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8305379C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830537A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830537A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830537A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830537A8 size=100
    let mut pc: u32 = 0x830537A8;
    'dispatch: loop {
        match pc {
            0x830537A8 => {
    //   block [0x830537A8..0x8305380C)
	// 830537A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830537AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830537B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830537B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830537B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830537BC: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 830537C0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830537C4: 4BF84AD5  bl 0x82fd8298
	ctx.lr = 0x830537C8;
	sub_82FD8298(ctx, base);
	// 830537C8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830537CC: 41820028  beq 0x830537f4
	if ctx.cr[0].eq {
	pc = 0x830537F4; continue 'dispatch;
	}
	// 830537D0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830537D4: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 830537D8: 394B3B40  addi r10, r11, 0x3b40
	ctx.r[10].s64 = ctx.r[11].s64 + 15168;
	// 830537DC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830537E0: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830537E4: 99630004  stb r11, 4(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u8 ) };
	// 830537E8: 99630005  stb r11, 5(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(5 as u32), ctx.r[11].u8 ) };
	// 830537EC: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 830537F0: 48000008  b 0x830537f8
	pc = 0x830537F8; continue 'dispatch;
	// 830537F4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830537F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830537FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83053800: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83053804: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83053808: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83053810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83053810 size=56
    let mut pc: u32 = 0x83053810;
    'dispatch: loop {
        match pc {
            0x83053810 => {
    //   block [0x83053810..0x83053848)
	// 83053810: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83053814: 90830024  stw r4, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[4].u32 ) };
	// 83053818: 394B4094  addi r10, r11, 0x4094
	ctx.r[10].s64 = ctx.r[11].s64 + 16532;
	// 8305381C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83053820: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83053824: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83053828: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8305382C: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 83053830: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83053834: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 83053838: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 8305383C: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 83053840: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 83053844: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83053848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83053848 size=8
    let mut pc: u32 = 0x83053848;
    'dispatch: loop {
        match pc {
            0x83053848 => {
    //   block [0x83053848..0x83053850)
	// 83053848: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8305384C: 821640B8  lwz r16, 0x40b8(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(16568 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83053850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83053850 size=128
    let mut pc: u32 = 0x83053850;
    'dispatch: loop {
        match pc {
            0x83053850 => {
    //   block [0x83053850..0x830538D0)
	// 83053850: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83053854: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83053858: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8305385C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83053860: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 83053864: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83053868: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8305386C: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 83053870: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 83053874: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83053878: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 8305387C: 90BE0024  stw r5, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[5].u32 ) };
	// 83053880: 394B4094  addi r10, r11, 0x4094
	ctx.r[10].s64 = ctx.r[11].s64 + 16532;
	// 83053884: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83053888: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8305388C: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83053890: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83053894: 917E000C  stw r11, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 83053898: 917E0014  stw r11, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8305389C: 917E0018  stw r11, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 830538A0: 917E001C  stw r11, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 830538A4: 917E0020  stw r11, 0x20(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 830538A8: 917E0010  stw r11, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 830538AC: 4BF7D2D5  bl 0x82fd0b80
	ctx.lr = 0x830538B0;
	sub_82FD0B80(ctx, base);
	// 830538B0: 907E0010  stw r3, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 830538B4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830538B8: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 830538BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830538C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830538C4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830538C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830538CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830538D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830538D0 size=40
    let mut pc: u32 = 0x830538D0;
    'dispatch: loop {
        match pc {
            0x830538D0 => {
    //   block [0x830538D0..0x830538F8)
	// 830538D0: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 830538D4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830538D8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830538DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830538E0: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 830538E4: 4BFF8E7D  bl 0x8304c760
	ctx.lr = 0x830538E8;
	sub_8304C760(ctx, base);
	// 830538E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830538EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830538F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830538F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830538F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830538F8 size=100
    let mut pc: u32 = 0x830538F8;
    'dispatch: loop {
        match pc {
            0x830538F8 => {
    //   block [0x830538F8..0x8305395C)
	// 830538F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830538FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83053900: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83053904: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83053908: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8305390C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83053910: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83053914: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83053918: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8305391C: 41820018  beq 0x83053934
	if ctx.cr[0].eq {
	pc = 0x83053934; continue 'dispatch;
	}
	// 83053920: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83053924: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83053928: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8305392C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83053930: 4E800421  bctrl
	ctx.lr = 0x83053934;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83053934: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83053938: 809F0024  lwz r4, 0x24(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 8305393C: 4BF7D245  bl 0x82fd0b80
	ctx.lr = 0x83053940;
	sub_82FD0B80(ctx, base);
	// 83053940: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 83053944: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83053948: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8305394C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83053950: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83053954: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83053958: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83053960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83053960 size=184
    let mut pc: u32 = 0x83053960;
    'dispatch: loop {
        match pc {
            0x83053960 => {
    //   block [0x83053960..0x83053A18)
	// 83053960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83053964: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83053968: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8305396C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83053970: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83053974: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83053978: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8305397C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83053980: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83053984: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83053988: 4E800421  bctrl
	ctx.lr = 0x8305398C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8305398C: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83053990: 809F0014  lwz r4, 0x14(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83053994: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83053998: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8305399C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830539A0: 4E800421  bctrl
	ctx.lr = 0x830539A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830539A4: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 830539A8: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 830539AC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830539B0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830539B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830539B8: 4E800421  bctrl
	ctx.lr = 0x830539BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830539BC: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 830539C0: 809F0018  lwz r4, 0x18(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 830539C4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830539C8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830539CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830539D0: 4E800421  bctrl
	ctx.lr = 0x830539D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830539D4: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 830539D8: 809F001C  lwz r4, 0x1c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 830539DC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830539E0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830539E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830539E8: 4E800421  bctrl
	ctx.lr = 0x830539EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830539EC: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 830539F0: 809F0020  lwz r4, 0x20(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 830539F4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830539F8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830539FC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83053A00: 4E800421  bctrl
	ctx.lr = 0x83053A04;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83053A04: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83053A08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83053A0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83053A10: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83053A14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83053A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83053A18 size=12
    let mut pc: u32 = 0x83053A18;
    'dispatch: loop {
        match pc {
            0x83053A18 => {
    //   block [0x83053A18..0x83053A24)
	// 83053A18: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 83053A1C: 386B3424  addi r3, r11, 0x3424
	ctx.r[3].s64 = ctx.r[11].s64 + 13348;
	// 83053A20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83053A28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83053A28 size=380
    let mut pc: u32 = 0x83053A28;
    'dispatch: loop {
        match pc {
            0x83053A28 => {
    //   block [0x83053A28..0x83053BA4)
	// 83053A28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83053A2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83053A30: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83053A34: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83053A38: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83053A3C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83053A40: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83053A44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83053A48: A97F0000  lha r11, 0(r31)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as i16) as i64;
	// 83053A4C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83053A50: 556BDFFF  rlwinm. r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83053A54: 41820094  beq 0x83053ae8
	if ctx.cr[0].eq {
	pc = 0x83053AE8; continue 'dispatch;
	}
	// 83053A58: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83053A5C: 4BFA589D  bl 0x82ff92f8
	ctx.lr = 0x83053A60;
	sub_82FF92F8(ctx, base);
	// 83053A60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83053A64: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83053A68: 4BFA5891  bl 0x82ff92f8
	ctx.lr = 0x83053A6C;
	sub_82FF92F8(ctx, base);
	// 83053A6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83053A70: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83053A74: 809E000C  lwz r4, 0xc(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83053A78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83053A7C: 4BFA5E85  bl 0x82ff9900
	ctx.lr = 0x83053A80;
	sub_82FF9900(ctx, base);
	// 83053A80: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83053A84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83053A88: 809E0010  lwz r4, 0x10(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83053A8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83053A90: 4BFA5E71  bl 0x82ff9900
	ctx.lr = 0x83053A94;
	sub_82FF9900(ctx, base);
	// 83053A94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83053A98: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83053A9C: 809E0014  lwz r4, 0x14(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 83053AA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83053AA4: 4BFA5E5D  bl 0x82ff9900
	ctx.lr = 0x83053AA8;
	sub_82FF9900(ctx, base);
	// 83053AA8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83053AAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83053AB0: 809E0018  lwz r4, 0x18(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 83053AB4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83053AB8: 4BFA5E49  bl 0x82ff9900
	ctx.lr = 0x83053ABC;
	sub_82FF9900(ctx, base);
	// 83053ABC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83053AC0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83053AC4: 809E001C  lwz r4, 0x1c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 83053AC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83053ACC: 4BFA5E35  bl 0x82ff9900
	ctx.lr = 0x83053AD0;
	sub_82FF9900(ctx, base);
	// 83053AD0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83053AD4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83053AD8: 809E0020  lwz r4, 0x20(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 83053ADC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83053AE0: 4BFA5E21  bl 0x82ff9900
	ctx.lr = 0x83053AE4;
	sub_82FF9900(ctx, base);
	// 83053AE4: 480000A8  b 0x83053b8c
	pc = 0x83053B8C; continue 'dispatch;
	// 83053AE8: 389E0004  addi r4, r30, 4
	ctx.r[4].s64 = ctx.r[30].s64 + 4;
	// 83053AEC: 4BFA5A8D  bl 0x82ff9578
	ctx.lr = 0x83053AF0;
	sub_82FF9578(ctx, base);
	// 83053AF0: 389E0008  addi r4, r30, 8
	ctx.r[4].s64 = ctx.r[30].s64 + 8;
	// 83053AF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83053AF8: 4BFA5A81  bl 0x82ff9578
	ctx.lr = 0x83053AFC;
	sub_82FF9578(ctx, base);
	// 83053AFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83053B00: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 83053B04: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 83053B08: 389E000C  addi r4, r30, 0xc
	ctx.r[4].s64 = ctx.r[30].s64 + 12;
	// 83053B0C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83053B10: 4BFA6019  bl 0x82ff9b28
	ctx.lr = 0x83053B14;
	sub_82FF9B28(ctx, base);
	// 83053B14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83053B18: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 83053B1C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83053B20: 389E0010  addi r4, r30, 0x10
	ctx.r[4].s64 = ctx.r[30].s64 + 16;
	// 83053B24: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83053B28: 4BFA6001  bl 0x82ff9b28
	ctx.lr = 0x83053B2C;
	sub_82FF9B28(ctx, base);
	// 83053B2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83053B30: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 83053B34: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83053B38: 389E0014  addi r4, r30, 0x14
	ctx.r[4].s64 = ctx.r[30].s64 + 20;
	// 83053B3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83053B40: 4BFA5FE9  bl 0x82ff9b28
	ctx.lr = 0x83053B44;
	sub_82FF9B28(ctx, base);
	// 83053B44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83053B48: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 83053B4C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83053B50: 389E0018  addi r4, r30, 0x18
	ctx.r[4].s64 = ctx.r[30].s64 + 24;
	// 83053B54: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83053B58: 4BFA5FD1  bl 0x82ff9b28
	ctx.lr = 0x83053B5C;
	sub_82FF9B28(ctx, base);
	// 83053B5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83053B60: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 83053B64: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83053B68: 389E001C  addi r4, r30, 0x1c
	ctx.r[4].s64 = ctx.r[30].s64 + 28;
	// 83053B6C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83053B70: 4BFA5FB9  bl 0x82ff9b28
	ctx.lr = 0x83053B74;
	sub_82FF9B28(ctx, base);
	// 83053B74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83053B78: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 83053B7C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83053B80: 389E0020  addi r4, r30, 0x20
	ctx.r[4].s64 = ctx.r[30].s64 + 32;
	// 83053B84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83053B88: 4BFA5FA1  bl 0x82ff9b28
	ctx.lr = 0x83053B8C;
	sub_82FF9B28(ctx, base);
	// 83053B8C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83053B90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83053B94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83053B98: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83053B9C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83053BA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83053BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83053BA8 size=8
    let mut pc: u32 = 0x83053BA8;
    'dispatch: loop {
        match pc {
            0x83053BA8 => {
    //   block [0x83053BA8..0x83053BB0)
	// 83053BA8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83053BAC: 82164134  lwz r16, 0x4134(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(16692 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83053BB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83053BB0 size=172
    let mut pc: u32 = 0x83053BB0;
    'dispatch: loop {
        match pc {
            0x83053BB0 => {
    //   block [0x83053BB0..0x83053C5C)
	// 83053BB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83053BB4: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 83053BB8: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 83053BBC: 481545B1  bl 0x831a816c
	ctx.lr = 0x83053BC0;
	sub_831A8130(ctx, base);
	// 83053BC0: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 83053BC4: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83053BC8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83053BCC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83053BD0: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 83053BD4: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83053BD8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 83053BDC: 90DE0024  stw r6, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[6].u32 ) };
	// 83053BE0: 394B4094  addi r10, r11, 0x4094
	ctx.r[10].s64 = ctx.r[11].s64 + 16532;
	// 83053BE4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83053BE8: 913E0008  stw r9, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 83053BEC: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83053BF0: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83053BF4: 917E0014  stw r11, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 83053BF8: 917E0018  stw r11, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 83053BFC: 917E001C  stw r11, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 83053C00: 917E0020  stw r11, 0x20(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 83053C04: 917E000C  stw r11, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 83053C08: 917E0010  stw r11, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83053C0C: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 83053C10: B17F0052  sth r11, 0x52(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(82 as u32), ctx.r[11].u16 ) };
	// 83053C14: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83053C18: B0BF0050  sth r5, 0x50(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[5].u16 ) };
	// 83053C1C: 4BF7CF65  bl 0x82fd0b80
	ctx.lr = 0x83053C20;
	sub_82FD0B80(ctx, base);
	// 83053C20: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83053C24: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83053C28: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83053C2C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83053C30: 809E0024  lwz r4, 0x24(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 83053C34: 917E000C  stw r11, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 83053C38: 4BF7CF49  bl 0x82fd0b80
	ctx.lr = 0x83053C3C;
	sub_82FD0B80(ctx, base);
	// 83053C3C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83053C40: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83053C44: 907E0010  stw r3, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 83053C48: 48000008  b 0x83053c50
	pc = 0x83053C50; continue 'dispatch;
	// 83053C4C: 83DF0094  lwz r30, 0x94(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83053C50: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83053C54: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 83053C58: 48154564  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83053C5C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83053C5C size=8
    let mut pc: u32 = 0x83053C5C;
    'dispatch: loop {
        match pc {
            0x83053C5C => {
    //   block [0x83053C5C..0x83053C64)
	// 83053C5C: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83053C60: 82164134  lwz r16, 0x4134(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(16692 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83053C64(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83053C64 size=24
    let mut pc: u32 = 0x83053C64;
    'dispatch: loop {
        match pc {
            0x83053C64 => {
    //   block [0x83053C64..0x83053C7C)
	// 83053C64: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83053C68: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83053C6C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83053C70: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83053C74: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83053C78: 4815CFB1  bl 0x831b0c28
	ctx.lr = 0x83053C7C;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83053C84(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83053C84 size=48
    let mut pc: u32 = 0x83053C84;
    'dispatch: loop {
        match pc {
            0x83053C84 => {
    //   block [0x83053C84..0x83053CB4)
	// 83053C84: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83053C88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83053C8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83053C90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83053C94: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83053C98: 4BFFFCC9  bl 0x83053960
	ctx.lr = 0x83053C9C;
	sub_83053960(ctx, base);
	// 83053C9C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83053CA0: 3C608305  lis r3, -0x7cfb
	ctx.r[3].s64 = -2096824320;
	// 83053CA4: 38633C4C  addi r3, r3, 0x3c4c
	ctx.r[3].s64 = ctx.r[3].s64 + 15436;
	// 83053CA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83053CAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83053CB0: 4815450C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83053CB4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83053CB4 size=40
    let mut pc: u32 = 0x83053CB4;
    'dispatch: loop {
        match pc {
            0x83053CB4 => {
    //   block [0x83053CB4..0x83053CDC)
	// 83053CB4: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83053CB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83053CBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83053CC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83053CC4: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83053CC8: 4BFF8A99  bl 0x8304c760
	ctx.lr = 0x83053CCC;
	sub_8304C760(ctx, base);
	// 83053CCC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83053CD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83053CD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83053CD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83053CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83053CE0 size=8
    let mut pc: u32 = 0x83053CE0;
    'dispatch: loop {
        match pc {
            0x83053CE0 => {
    //   block [0x83053CE0..0x83053CE8)
	// 83053CE0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83053CE4: 821641A8  lwz r16, 0x41a8(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(16808 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83053CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83053CE8 size=84
    let mut pc: u32 = 0x83053CE8;
    'dispatch: loop {
        match pc {
            0x83053CE8 => {
    //   block [0x83053CE8..0x83053D3C)
	// 83053CE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83053CEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83053CF0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83053CF4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83053CF8: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 83053CFC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83053D00: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83053D04: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83053D08: 396B4094  addi r11, r11, 0x4094
	ctx.r[11].s64 = ctx.r[11].s64 + 16532;
	// 83053D0C: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 83053D10: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83053D14: 4BFFFC4D  bl 0x83053960
	ctx.lr = 0x83053D18;
	sub_83053960(ctx, base);
	// 83053D18: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83053D1C: 396BA93C  addi r11, r11, -0x56c4
	ctx.r[11].s64 = ctx.r[11].s64 + -22212;
	// 83053D20: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83053D24: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 83053D28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83053D2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83053D30: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83053D34: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83053D38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83053D3C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83053D3C size=40
    let mut pc: u32 = 0x83053D3C;
    'dispatch: loop {
        match pc {
            0x83053D3C => {
    //   block [0x83053D3C..0x83053D64)
	// 83053D3C: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 83053D40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83053D44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83053D48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83053D4C: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83053D50: 4BFF8A11  bl 0x8304c760
	ctx.lr = 0x83053D54;
	sub_8304C760(ctx, base);
	// 83053D54: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83053D58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83053D5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83053D60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83053D68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83053D68 size=76
    let mut pc: u32 = 0x83053D68;
    'dispatch: loop {
        match pc {
            0x83053D68 => {
    //   block [0x83053D68..0x83053DB4)
	// 83053D68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83053D6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83053D70: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83053D74: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83053D78: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83053D7C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83053D80: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83053D84: 4BFFFF65  bl 0x83053ce8
	ctx.lr = 0x83053D88;
	sub_83053CE8(ctx, base);
	// 83053D88: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83053D8C: 4182000C  beq 0x83053d98
	if ctx.cr[0].eq {
	pc = 0x83053D98; continue 'dispatch;
	}
	// 83053D90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83053D94: 4BF8454D  bl 0x82fd82e0
	ctx.lr = 0x83053D98;
	sub_82FD82E0(ctx, base);
	// 83053D98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83053D9C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83053DA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83053DA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83053DA8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83053DAC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83053DB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83053DB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83053DB8 size=8
    let mut pc: u32 = 0x83053DB8;
    'dispatch: loop {
        match pc {
            0x83053DB8 => {
    //   block [0x83053DB8..0x83053DC0)
	// 83053DB8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83053DBC: 821641F0  lwz r16, 0x41f0(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(16880 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83053DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83053DC0 size=108
    let mut pc: u32 = 0x83053DC0;
    'dispatch: loop {
        match pc {
            0x83053DC0 => {
    //   block [0x83053DC0..0x83053E2C)
	// 83053DC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83053DC4: 481543A9  bl 0x831a816c
	ctx.lr = 0x83053DC8;
	sub_831A8130(ctx, base);
	// 83053DC8: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 83053DCC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83053DD0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83053DD4: 3860002C  li r3, 0x2c
	ctx.r[3].s64 = 44;
	// 83053DD8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83053DDC: 93BF0094  stw r29, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[29].u32 ) };
	// 83053DE0: 4BF844B9  bl 0x82fd8298
	ctx.lr = 0x83053DE4;
	sub_82FD8298(ctx, base);
	// 83053DE4: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83053DE8: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 83053DEC: 41820034  beq 0x83053e20
	if ctx.cr[0].eq {
	pc = 0x83053E20; continue 'dispatch;
	}
	// 83053DF0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83053DF4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83053DF8: 4BFFFA19  bl 0x83053810
	ctx.lr = 0x83053DFC;
	sub_83053810(ctx, base);
	// 83053DFC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83053E00: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83053E04: 394B3FF0  addi r10, r11, 0x3ff0
	ctx.r[10].s64 = ctx.r[11].s64 + 16368;
	// 83053E08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83053E0C: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83053E10: 997E0028  stb r11, 0x28(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), ctx.r[11].u8 ) };
	// 83053E14: 997E0029  stb r11, 0x29(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(41 as u32), ctx.r[11].u8 ) };
	// 83053E18: 997E002A  stb r11, 0x2a(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(42 as u32), ctx.r[11].u8 ) };
	// 83053E1C: 48000008  b 0x83053e24
	pc = 0x83053E24; continue 'dispatch;
	// 83053E20: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83053E24: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 83053E28: 48154394  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83053E2C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83053E2C size=44
    let mut pc: u32 = 0x83053E2C;
    'dispatch: loop {
        match pc {
            0x83053E2C => {
    //   block [0x83053E2C..0x83053E58)
	// 83053E2C: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83053E30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83053E34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83053E38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83053E3C: 809F0094  lwz r4, 0x94(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83053E40: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83053E44: 4BF8449D  bl 0x82fd82e0
	ctx.lr = 0x83053E48;
	sub_82FD82E0(ctx, base);
	// 83053E48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83053E4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83053E50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83053E54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83053E58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83053E58 size=12
    let mut pc: u32 = 0x83053E58;
    'dispatch: loop {
        match pc {
            0x83053E58 => {
    //   block [0x83053E58..0x83053E64)
	// 83053E58: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 83053E5C: 386B342C  addi r3, r11, 0x342c
	ctx.r[3].s64 = ctx.r[11].s64 + 13356;
	// 83053E60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83053E68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83053E68 size=144
    let mut pc: u32 = 0x83053E68;
    'dispatch: loop {
        match pc {
            0x83053E68 => {
    //   block [0x83053E68..0x83053EF8)
	// 83053E68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83053E6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83053E70: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83053E74: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83053E78: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83053E7C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83053E80: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83053E84: 4BFFFBA5  bl 0x83053a28
	ctx.lr = 0x83053E88;
	sub_83053A28(ctx, base);
	// 83053E88: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83053E8C: A97F0000  lha r11, 0(r31)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as i16) as i64;
	// 83053E90: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83053E94: 556BDFFF  rlwinm. r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83053E98: 41820028  beq 0x83053ec0
	if ctx.cr[0].eq {
	pc = 0x83053EC0; continue 'dispatch;
	}
	// 83053E9C: 889E0028  lbz r4, 0x28(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 83053EA0: 4BFA5361  bl 0x82ff9200
	ctx.lr = 0x83053EA4;
	sub_82FF9200(ctx, base);
	// 83053EA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83053EA8: 889E0029  lbz r4, 0x29(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(41 as u32) ) } as u64;
	// 83053EAC: 4BFA5355  bl 0x82ff9200
	ctx.lr = 0x83053EB0;
	sub_82FF9200(ctx, base);
	// 83053EB0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83053EB4: 889E002A  lbz r4, 0x2a(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(42 as u32) ) } as u64;
	// 83053EB8: 4BFA5349  bl 0x82ff9200
	ctx.lr = 0x83053EBC;
	sub_82FF9200(ctx, base);
	// 83053EBC: 48000024  b 0x83053ee0
	pc = 0x83053EE0; continue 'dispatch;
	// 83053EC0: 389E0028  addi r4, r30, 0x28
	ctx.r[4].s64 = ctx.r[30].s64 + 40;
	// 83053EC4: 4BFA55BD  bl 0x82ff9480
	ctx.lr = 0x83053EC8;
	sub_82FF9480(ctx, base);
	// 83053EC8: 389E0029  addi r4, r30, 0x29
	ctx.r[4].s64 = ctx.r[30].s64 + 41;
	// 83053ECC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83053ED0: 4BFA55B1  bl 0x82ff9480
	ctx.lr = 0x83053ED4;
	sub_82FF9480(ctx, base);
	// 83053ED4: 389E002A  addi r4, r30, 0x2a
	ctx.r[4].s64 = ctx.r[30].s64 + 42;
	// 83053ED8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83053EDC: 4BFA55A5  bl 0x82ff9480
	ctx.lr = 0x83053EE0;
	sub_82FF9480(ctx, base);
	// 83053EE0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83053EE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83053EE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83053EEC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83053EF0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83053EF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83053EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83053EF8 size=16
    let mut pc: u32 = 0x83053EF8;
    'dispatch: loop {
        match pc {
            0x83053EF8 => {
    //   block [0x83053EF8..0x83053F08)
	// 83053EF8: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83053EFC: 396B4220  addi r11, r11, 0x4220
	ctx.r[11].s64 = ctx.r[11].s64 + 16928;
	// 83053F00: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83053F04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83053F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83053F08 size=68
    let mut pc: u32 = 0x83053F08;
    'dispatch: loop {
        match pc {
            0x83053F08 => {
    //   block [0x83053F08..0x83053F4C)
	// 83053F08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83053F0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83053F10: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83053F14: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83053F18: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83053F1C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83053F20: 396B4220  addi r11, r11, 0x4220
	ctx.r[11].s64 = ctx.r[11].s64 + 16928;
	// 83053F24: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83053F28: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83053F2C: 41820008  beq 0x83053f34
	if ctx.cr[0].eq {
	pc = 0x83053F34; continue 'dispatch;
	}
	// 83053F30: 4BF843B1  bl 0x82fd82e0
	ctx.lr = 0x83053F34;
	sub_82FD82E0(ctx, base);
	// 83053F34: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83053F38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83053F3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83053F40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83053F44: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83053F48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83053F50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83053F50 size=76
    let mut pc: u32 = 0x83053F50;
    'dispatch: loop {
        match pc {
            0x83053F50 => {
    //   block [0x83053F50..0x83053F9C)
	// 83053F50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83053F54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83053F58: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83053F5C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83053F60: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83053F64: 480466A5  bl 0x8309a608
	ctx.lr = 0x83053F68;
	sub_8309A608(ctx, base);
	// 83053F68: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83053F6C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83053F70: 394B4230  addi r10, r11, 0x4230
	ctx.r[10].s64 = ctx.r[11].s64 + 16944;
	// 83053F74: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83053F78: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83053F7C: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 83053F80: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 83053F84: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 83053F88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83053F8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83053F90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83053F94: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83053F98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83053FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83053FA0 size=8
    let mut pc: u32 = 0x83053FA0;
    'dispatch: loop {
        match pc {
            0x83053FA0 => {
    //   block [0x83053FA0..0x83053FA8)
	// 83053FA0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83053FA4: 82164248  lwz r16, 0x4248(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(16968 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83053FA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83053FA8 size=104
    let mut pc: u32 = 0x83053FA8;
    'dispatch: loop {
        match pc {
            0x83053FA8 => {
    //   block [0x83053FA8..0x83054010)
	// 83053FA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83053FAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83053FB0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83053FB4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83053FB8: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 83053FBC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83053FC0: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83053FC4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83053FC8: 396B4230  addi r11, r11, 0x4230
	ctx.r[11].s64 = ctx.r[11].s64 + 16944;
	// 83053FCC: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 83053FD0: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83053FD4: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83053FD8: 809E0014  lwz r4, 0x14(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 83053FDC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83053FE0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83053FE4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83053FE8: 4E800421  bctrl
	ctx.lr = 0x83053FEC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83053FEC: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83053FF0: 396B4220  addi r11, r11, 0x4220
	ctx.r[11].s64 = ctx.r[11].s64 + 16928;
	// 83053FF4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83053FF8: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 83053FFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83054000: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83054004: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83054008: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8305400C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83054010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83054010 size=40
    let mut pc: u32 = 0x83054010;
    'dispatch: loop {
        match pc {
            0x83054010 => {
    //   block [0x83054010..0x83054038)
	// 83054010: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 83054014: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83054018: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8305401C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83054020: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83054024: 4BFFFED5  bl 0x83053ef8
	ctx.lr = 0x83054028;
	sub_83053EF8(ctx, base);
	// 83054028: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8305402C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83054030: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83054034: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83054038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83054038 size=8
    let mut pc: u32 = 0x83054038;
    'dispatch: loop {
        match pc {
            0x83054038 => {
    //   block [0x83054038..0x83054040)
	// 83054038: 80630024  lwz r3, 0x24(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 8305403C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83054040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83054040 size=8
    let mut pc: u32 = 0x83054040;
    'dispatch: loop {
        match pc {
            0x83054040 => {
    //   block [0x83054040..0x83054048)
	// 83054040: 80630028  lwz r3, 0x28(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 83054044: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83054048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83054048 size=132
    let mut pc: u32 = 0x83054048;
    'dispatch: loop {
        match pc {
            0x83054048 => {
    //   block [0x83054048..0x830540CC)
	// 83054048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8305404C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83054050: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83054054: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83054058: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8305405C: 816100D4  lwz r11, 0xd4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(212 as u32) ) } as u64;
	// 83054060: 90DF0008  stw r6, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 83054064: 98FF0020  stb r7, 0x20(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[7].u8 ) };
	// 83054068: 913F0024  stw r9, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[9].u32 ) };
	// 8305406C: 80C100B4  lwz r6, 0xb4(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(180 as u32) ) } as u64;
	// 83054070: 80E100BC  lwz r7, 0xbc(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(188 as u32) ) } as u64;
	// 83054074: 812100C4  lwz r9, 0xc4(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(196 as u32) ) } as u64;
	// 83054078: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8305407C: 909F0018  stw r4, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[4].u32 ) };
	// 83054080: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 83054084: 90BF001C  stw r5, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[5].u32 ) };
	// 83054088: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 8305408C: 911F002C  stw r8, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[8].u32 ) };
	// 83054090: 90DF0034  stw r6, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[6].u32 ) };
	// 83054094: 90FF0010  stw r7, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[7].u32 ) };
	// 83054098: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 8305409C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830540A0: 809F0014  lwz r4, 0x14(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 830540A4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830540A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830540AC: 4E800421  bctrl
	ctx.lr = 0x830540B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830540B0: 816100CC  lwz r11, 0xcc(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(204 as u32) ) } as u64;
	// 830540B4: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 830540B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830540BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830540C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830540C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830540C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830540D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830540D0 size=76
    let mut pc: u32 = 0x830540D0;
    'dispatch: loop {
        match pc {
            0x830540D0 => {
    //   block [0x830540D0..0x8305411C)
	// 830540D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830540D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830540D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830540DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830540E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830540E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830540E8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830540EC: 4BFFFEBD  bl 0x83053fa8
	ctx.lr = 0x830540F0;
	sub_83053FA8(ctx, base);
	// 830540F0: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830540F4: 4182000C  beq 0x83054100
	if ctx.cr[0].eq {
	pc = 0x83054100; continue 'dispatch;
	}
	// 830540F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830540FC: 4BF841E5  bl 0x82fd82e0
	ctx.lr = 0x83054100;
	sub_82FD82E0(ctx, base);
	// 83054100: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83054104: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83054108: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8305410C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83054110: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83054114: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83054118: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83054120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83054120 size=56
    let mut pc: u32 = 0x83054120;
    'dispatch: loop {
        match pc {
            0x83054120 => {
    //   block [0x83054120..0x83054158)
	// 83054120: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83054124: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83054128: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8305412C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83054130: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83054134: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83054138: 48047511  bl 0x8309b648
	ctx.lr = 0x8305413C;
	sub_8309B648(ctx, base);
	// 8305413C: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83054140: 48046699  bl 0x8309a7d8
	ctx.lr = 0x83054144;
	sub_8309A7D8(ctx, base);
	// 83054144: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83054148: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8305414C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83054150: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83054154: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83054158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83054158 size=132
    let mut pc: u32 = 0x83054158;
    'dispatch: loop {
        match pc {
            0x83054158 => {
    //   block [0x83054158..0x830541DC)
	// 83054158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8305415C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83054160: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83054164: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83054168: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8305416C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83054170: 83FE000C  lwz r31, 0xc(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83054174: 281F0000  cmplwi r31, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83054178: 41820014  beq 0x8305418c
	if ctx.cr[0].eq {
	pc = 0x8305418C; continue 'dispatch;
	}
	// 8305417C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83054180: 480465F9  bl 0x8309a778
	ctx.lr = 0x83054184;
	sub_8309A778(ctx, base);
	// 83054184: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83054188: 4BF84159  bl 0x82fd82e0
	ctx.lr = 0x8305418C;
	sub_82FD82E0(ctx, base);
	// 8305418C: 83FE0010  lwz r31, 0x10(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83054190: 281F0000  cmplwi r31, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83054194: 41820014  beq 0x830541a8
	if ctx.cr[0].eq {
	pc = 0x830541A8; continue 'dispatch;
	}
	// 83054198: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8305419C: 480480AD  bl 0x8309c248
	ctx.lr = 0x830541A0;
	sub_8309C248(ctx, base);
	// 830541A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830541A4: 4BF8413D  bl 0x82fd82e0
	ctx.lr = 0x830541A8;
	sub_82FD82E0(ctx, base);
	// 830541A8: 83FE0014  lwz r31, 0x14(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 830541AC: 281F0000  cmplwi r31, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830541B0: 41820014  beq 0x830541c4
	if ctx.cr[0].eq {
	pc = 0x830541C4; continue 'dispatch;
	}
	// 830541B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830541B8: 48048371  bl 0x8309c528
	ctx.lr = 0x830541BC;
	sub_8309C528(ctx, base);
	// 830541BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830541C0: 4BF84121  bl 0x82fd82e0
	ctx.lr = 0x830541C4;
	sub_82FD82E0(ctx, base);
	// 830541C4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830541C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830541CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830541D0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830541D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830541D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830541E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830541E0 size=104
    let mut pc: u32 = 0x830541E0;
    'dispatch: loop {
        match pc {
            0x830541E0 => {
    //   block [0x830541E0..0x83054248)
	// 830541E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830541E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830541E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830541EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830541F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830541F4: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830541F8: 80BF0000  lwz r5, 0(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830541FC: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 83054200: 7F055840  cmplw cr6, r5, r11
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83054204: 409A0018  bne cr6, 0x8305421c
	if !ctx.cr[6].eq {
	pc = 0x8305421C; continue 'dispatch;
	}
	// 83054208: 4BFE6F49  bl 0x8303b150
	ctx.lr = 0x8305420C;
	sub_8303B150(ctx, base);
	// 8305420C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83054210: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83054214: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83054218: 4800001C  b 0x83054234
	pc = 0x83054234; continue 'dispatch;
	// 8305421C: 39650001  addi r11, r5, 1
	ctx.r[11].s64 = ctx.r[5].s64 + 1;
	// 83054220: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83054224: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83054228: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8305422C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83054230: 4E800421  bctrl
	ctx.lr = 0x83054234;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83054234: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83054238: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8305423C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83054240: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83054244: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83054248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83054248 size=8
    let mut pc: u32 = 0x83054248;
    'dispatch: loop {
        match pc {
            0x83054248 => {
    //   block [0x83054248..0x83054250)
	// 83054248: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8305424C: 821642F4  lwz r16, 0x42f4(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(17140 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83054250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83054250 size=292
    let mut pc: u32 = 0x83054250;
    'dispatch: loop {
        match pc {
            0x83054250 => {
    //   block [0x83054250..0x83054374)
	// 83054250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83054254: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 83054258: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 8305425C: 48153F0D  bl 0x831a8168
	ctx.lr = 0x83054260;
	sub_831A8130(ctx, base);
	// 83054260: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 83054264: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83054268: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8305426C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83054270: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83054274: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 83054278: 396B4290  addi r11, r11, 0x4290
	ctx.r[11].s64 = ctx.r[11].s64 + 17040;
	// 8305427C: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 83054280: 90BE0008  stw r5, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 83054284: 939E0004  stw r28, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 83054288: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8305428C: 93BE000C  stw r29, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 83054290: 93BE0010  stw r29, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[29].u32 ) };
	// 83054294: 93BE0014  stw r29, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[29].u32 ) };
	// 83054298: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 8305429C: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 830542A0: 4BF83FF9  bl 0x82fd8298
	ctx.lr = 0x830542A4;
	sub_82FD8298(ctx, base);
	// 830542A4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830542A8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830542AC: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 830542B0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830542B4: 4182001C  beq 0x830542d0
	if ctx.cr[0].eq {
	pc = 0x830542D0; continue 'dispatch;
	}
	// 830542B8: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 830542BC: 4804657D  bl 0x8309a838
	ctx.lr = 0x830542C0;
	sub_8309A838(ctx, base);
	// 830542C0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 830542C4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830542C8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830542CC: 48000008  b 0x830542d4
	pc = 0x830542D4; continue 'dispatch;
	// 830542D0: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 830542D4: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 830542D8: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 830542DC: 917E000C  stw r11, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 830542E0: 4BF83FB9  bl 0x82fd8298
	ctx.lr = 0x830542E4;
	sub_82FD8298(ctx, base);
	// 830542E4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830542E8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830542EC: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 830542F0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830542F4: 4182001C  beq 0x83054310
	if ctx.cr[0].eq {
	pc = 0x83054310; continue 'dispatch;
	}
	// 830542F8: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 830542FC: 48047E95  bl 0x8309c190
	ctx.lr = 0x83054300;
	sub_8309C190(ctx, base);
	// 83054300: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83054304: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83054308: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8305430C: 48000008  b 0x83054314
	pc = 0x83054314; continue 'dispatch;
	// 83054310: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 83054314: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 83054318: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8305431C: 917E0010  stw r11, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83054320: 4BF83F79  bl 0x82fd8298
	ctx.lr = 0x83054324;
	sub_82FD8298(ctx, base);
	// 83054324: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83054328: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8305432C: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83054330: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83054334: 41820024  beq 0x83054358
	if ctx.cr[0].eq {
	pc = 0x83054358; continue 'dispatch;
	}
	// 83054338: 80DE0008  lwz r6, 8(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8305433C: 80BE000C  lwz r5, 0xc(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83054340: 809E0010  lwz r4, 0x10(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83054344: 48047F6D  bl 0x8309c2b0
	ctx.lr = 0x83054348;
	sub_8309C2B0(ctx, base);
	// 83054348: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8305434C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83054350: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83054354: 48000008  b 0x8305435c
	pc = 0x8305435C; continue 'dispatch;
	// 83054358: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	// 8305435C: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83054360: 915E0014  stw r10, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 83054364: 938B0010  stw r28, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[28].u32 ) };
	// 83054368: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8305436C: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 83054370: 48153E48  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83054374(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83054374 size=8
    let mut pc: u32 = 0x83054374;
    'dispatch: loop {
        match pc {
            0x83054374 => {
    //   block [0x83054374..0x8305437C)
	// 83054374: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83054378: 821642F4  lwz r16, 0x42f4(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(17140 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8305437C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8305437C size=24
    let mut pc: u32 = 0x8305437C;
    'dispatch: loop {
        match pc {
            0x8305437C => {
    //   block [0x8305437C..0x83054394)
	// 8305437C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83054380: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83054384: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83054388: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8305438C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83054390: 4815C899  bl 0x831b0c28
	ctx.lr = 0x83054394;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8305439C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8305439C size=36
    let mut pc: u32 = 0x8305439C;
    'dispatch: loop {
        match pc {
            0x8305439C => {
    //   block [0x8305439C..0x830543C0)
	// 8305439C: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830543A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830543A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830543A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830543AC: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 830543B0: 4BFFFDA9  bl 0x83054158
	ctx.lr = 0x830543B4;
	sub_83054158(ctx, base);
	// 830543B4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830543B8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830543BC: 4815C86D  bl 0x831b0c28
	ctx.lr = 0x830543C0;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830543C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830543C0 size=48
    let mut pc: u32 = 0x830543C0;
    'dispatch: loop {
        match pc {
            0x830543C0 => {
    //   block [0x830543C0..0x830543F0)
	// 830543C0: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830543C4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830543C8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830543CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830543D0: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 830543D4: 808B0008  lwz r4, 8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830543D8: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830543DC: 4BF83F05  bl 0x82fd82e0
	ctx.lr = 0x830543E0;
	sub_82FD82E0(ctx, base);
	// 830543E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830543E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830543E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830543EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830543F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830543F0 size=48
    let mut pc: u32 = 0x830543F0;
    'dispatch: loop {
        match pc {
            0x830543F0 => {
    //   block [0x830543F0..0x83054420)
	// 830543F0: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830543F4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830543F8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830543FC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83054400: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83054404: 808B0008  lwz r4, 8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83054408: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8305440C: 4BF83ED5  bl 0x82fd82e0
	ctx.lr = 0x83054410;
	sub_82FD82E0(ctx, base);
	// 83054410: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83054414: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83054418: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8305441C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83054420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83054420 size=48
    let mut pc: u32 = 0x83054420;
    'dispatch: loop {
        match pc {
            0x83054420 => {
    //   block [0x83054420..0x83054450)
	// 83054420: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83054424: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83054428: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8305442C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83054430: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83054434: 808B0008  lwz r4, 8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83054438: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8305443C: 4BF83EA5  bl 0x82fd82e0
	ctx.lr = 0x83054440;
	sub_82FD82E0(ctx, base);
	// 83054440: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83054444: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83054448: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8305444C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83054450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83054450 size=464
    let mut pc: u32 = 0x83054450;
    'dispatch: loop {
        match pc {
            0x83054450 => {
    //   block [0x83054450..0x83054620)
	// 83054450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83054454: 48153D0D  bl 0x831a8160
	ctx.lr = 0x83054458;
	sub_831A8130(ctx, base);
	// 83054458: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8305445C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83054460: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83054464: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 83054468: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 8305446C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83054470: 2C0A0000  cmpwi r10, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83054474: 40820024  bne 0x83054498
	if !ctx.cr[0].eq {
	pc = 0x83054498; continue 'dispatch;
	}
	// 83054478: 817D0048  lwz r11, 0x48(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(72 as u32) ) } as u64;
	// 8305447C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83054480: 4182000C  beq 0x8305448c
	if ctx.cr[0].eq {
	pc = 0x8305448C; continue 'dispatch;
	}
	// 83054484: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83054488: 48000008  b 0x83054490
	pc = 0x83054490; continue 'dispatch;
	// 8305448C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83054490: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83054494: 419A0184  beq cr6, 0x83054618
	if ctx.cr[6].eq {
	pc = 0x83054618; continue 'dispatch;
	}
	// 83054498: 3B6AFFFF  addi r27, r10, -1
	ctx.r[27].s64 = ctx.r[10].s64 + -1;
	// 8305449C: 7F7FDB79  or. r31, r27, r27
	ctx.r[31].u64 = ctx.r[27].u64 | ctx.r[27].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 830544A0: 41800034  blt 0x830544d4
	if ctx.cr[0].lt {
	pc = 0x830544D4; continue 'dispatch;
	}
	// 830544A4: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 830544A8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830544AC: 806B0008  lwz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830544B0: 4BFD83C1  bl 0x8302c870
	ctx.lr = 0x830544B4;
	sub_8302C870(ctx, base);
	// 830544B4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830544B8: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 830544BC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830544C0: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 830544C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830544C8: 4E800421  bctrl
	ctx.lr = 0x830544CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830544CC: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 830544D0: 4080FFD4  bge 0x830544a4
	if !ctx.cr[0].lt {
	pc = 0x830544A4; continue 'dispatch;
	}
	// 830544D4: 83FE000C  lwz r31, 0xc(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 830544D8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830544DC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830544E0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830544E4: 40990010  ble cr6, 0x830544f4
	if !ctx.cr[6].gt {
	pc = 0x830544F4; continue 'dispatch;
	}
	// 830544E8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830544EC: 48001025  bl 0x83055510
	ctx.lr = 0x830544F0;
	sub_83055510(ctx, base);
	// 830544F0: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 830544F4: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 830544F8: 7F7FDB78  mr r31, r27
	ctx.r[31].u64 = ctx.r[27].u64;
	// 830544FC: 834B0000  lwz r26, 0(r11)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83054500: 4800006C  b 0x8305456c
	pc = 0x8305456C; continue 'dispatch;
	// 83054504: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83054508: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8305450C: 806B0008  lwz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83054510: 4BFD8361  bl 0x8302c870
	ctx.lr = 0x83054514;
	sub_8302C870(ctx, base);
	// 83054514: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83054518: 839D001C  lwz r28, 0x1c(r29)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 8305451C: 281C0000  cmplwi r28, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83054520: 41820048  beq 0x83054568
	if ctx.cr[0].eq {
	pc = 0x83054568; continue 'dispatch;
	}
	// 83054524: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 83054528: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8305452C: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83054530: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83054534: 4E800421  bctrl
	ctx.lr = 0x83054538;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83054538: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 8305453C: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 83054540: 419A0028  beq cr6, 0x83054568
	if ctx.cr[6].eq {
	pc = 0x83054568; continue 'dispatch;
	}
	// 83054544: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83054548: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8305454C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83054550: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83054554: 4E800421  bctrl
	ctx.lr = 0x83054558;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83054558: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8305455C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83054560: 807E0010  lwz r3, 0x10(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83054564: 48046EB5  bl 0x8309b418
	ctx.lr = 0x83054568;
	sub_8309B418(ctx, base);
	// 83054568: 3BFFFFFF  addi r31, r31, -1
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	// 8305456C: 7F1FD000  cmpw cr6, r31, r26
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[26].s32, &mut ctx.xer);
	// 83054570: 4098FF94  bge cr6, 0x83054504
	if !ctx.cr[6].lt {
	pc = 0x83054504; continue 'dispatch;
	}
	// 83054574: 7F7CDB78  mr r28, r27
	ctx.r[28].u64 = ctx.r[27].u64;
	// 83054578: 48000090  b 0x83054608
	pc = 0x83054608; continue 'dispatch;
	// 8305457C: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83054580: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83054584: 806B0008  lwz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83054588: 4BFD82E9  bl 0x8302c870
	ctx.lr = 0x8305458C;
	sub_8302C870(ctx, base);
	// 8305458C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83054590: 83BF001C  lwz r29, 0x1c(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 83054594: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83054598: 4182006C  beq 0x83054604
	if ctx.cr[0].eq {
	pc = 0x83054604; continue 'dispatch;
	}
	// 8305459C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830545A0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830545A4: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 830545A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830545AC: 4E800421  bctrl
	ctx.lr = 0x830545B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830545B0: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 830545B4: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 830545B8: 409A004C  bne cr6, 0x83054604
	if !ctx.cr[6].eq {
	pc = 0x83054604; continue 'dispatch;
	}
	// 830545BC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830545C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830545C4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830545C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830545CC: 4E800421  bctrl
	ctx.lr = 0x830545D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830545D0: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 830545D4: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 830545D8: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 830545DC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830545E0: 806B0008  lwz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830545E4: 4801461D  bl 0x83068c00
	ctx.lr = 0x830545E8;
	sub_83068C00(ctx, base);
	// 830545E8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830545EC: 41820018  beq 0x83054604
	if ctx.cr[0].eq {
	pc = 0x83054604; continue 'dispatch;
	}
	// 830545F0: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830545F4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830545F8: 4182000C  beq 0x83054604
	if ctx.cr[0].eq {
	pc = 0x83054604; continue 'dispatch;
	}
	// 830545FC: 809E0010  lwz r4, 0x10(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83054600: 48049041  bl 0x8309d640
	ctx.lr = 0x83054604;
	sub_8309D640(ctx, base);
	// 83054604: 3B9CFFFF  addi r28, r28, -1
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	// 83054608: 7F1CD000  cmpw cr6, r28, r26
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[26].s32, &mut ctx.xer);
	// 8305460C: 4098FF70  bge cr6, 0x8305457c
	if !ctx.cr[6].lt {
	pc = 0x8305457C; continue 'dispatch;
	}
	// 83054610: 807E0010  lwz r3, 0x10(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83054614: 48047715  bl 0x8309bd28
	ctx.lr = 0x83054618;
	sub_8309BD28(ctx, base);
	// 83054618: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8305461C: 48153B94  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83054620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83054620 size=108
    let mut pc: u32 = 0x83054620;
    'dispatch: loop {
        match pc {
            0x83054620 => {
    //   block [0x83054620..0x8305468C)
	// 83054620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83054624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83054628: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8305462C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83054630: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83054634: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83054638: 8064000C  lwz r3, 0xc(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 8305463C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83054640: 41820034  beq 0x83054674
	if ctx.cr[0].eq {
	pc = 0x83054674; continue 'dispatch;
	}
	// 83054644: 80DE0008  lwz r6, 8(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83054648: 809E0014  lwz r4, 0x14(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 8305464C: 4804932D  bl 0x8309d978
	ctx.lr = 0x83054650;
	sub_8309D978(ctx, base);
	// 83054650: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83054654: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83054658: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8305465C: 4BFFFB85  bl 0x830541e0
	ctx.lr = 0x83054660;
	sub_830541E0(ctx, base);
	// 83054660: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83054664: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83054668: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8305466C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83054670: 4E800421  bctrl
	ctx.lr = 0x83054674;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83054674: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83054678: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8305467C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83054680: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83054684: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83054688: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83054690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83054690 size=88
    let mut pc: u32 = 0x83054690;
    'dispatch: loop {
        match pc {
            0x83054690 => {
    //   block [0x83054690..0x830546E8)
	// 83054690: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83054694: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83054698: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8305469C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830546A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830546A4: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 830546A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830546AC: 396B4290  addi r11, r11, 0x4290
	ctx.r[11].s64 = ctx.r[11].s64 + 17040;
	// 830546B0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830546B4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830546B8: 4BFFFAA1  bl 0x83054158
	ctx.lr = 0x830546BC;
	sub_83054158(ctx, base);
	// 830546BC: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830546C0: 4182000C  beq 0x830546cc
	if ctx.cr[0].eq {
	pc = 0x830546CC; continue 'dispatch;
	}
	// 830546C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830546C8: 4BF83C19  bl 0x82fd82e0
	ctx.lr = 0x830546CC;
	sub_82FD82E0(ctx, base);
	// 830546CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830546D0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830546D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830546D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830546DC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830546E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830546E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


