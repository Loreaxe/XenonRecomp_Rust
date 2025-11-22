pub fn sub_83049690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83049690 size=88
    let mut pc: u32 = 0x83049690;
    'dispatch: loop {
        match pc {
            0x83049690 => {
    //   block [0x83049690..0x830496E8)
	// 83049690: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83049694: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83049698: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8304969C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830496A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830496A4: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 830496A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830496AC: 396B16D0  addi r11, r11, 0x16d0
	ctx.r[11].s64 = ctx.r[11].s64 + 5840;
	// 830496B0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830496B4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830496B8: 48048FE1  bl 0x83092698
	ctx.lr = 0x830496BC;
	sub_83092698(ctx, base);
	// 830496BC: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830496C0: 4182000C  beq 0x830496cc
	if ctx.cr[0].eq {
	pc = 0x830496CC; continue 'dispatch;
	}
	// 830496C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830496C8: 4BF8EC19  bl 0x82fd82e0
	ctx.lr = 0x830496CC;
	sub_82FD82E0(ctx, base);
	// 830496CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830496D0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830496D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830496D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830496DC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830496E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830496E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830496E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830496E8 size=8
    let mut pc: u32 = 0x830496E8;
    'dispatch: loop {
        match pc {
            0x830496E8 => {
    //   block [0x830496E8..0x830496F0)
	// 830496E8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830496EC: 821619D8  lwz r16, 0x19d8(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(6616 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830496F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830496F0 size=492
    let mut pc: u32 = 0x830496F0;
    'dispatch: loop {
        match pc {
            0x830496F0 => {
    //   block [0x830496F0..0x830498DC)
	// 830496F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830496F4: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 830496F8: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 830496FC: 4815EA61  bl 0x831a815c
	ctx.lr = 0x83049700;
	sub_831A8130(ctx, base);
	// 83049700: 3BE1FEA0  addi r31, r1, -0x160
	ctx.r[31].s64 = ctx.r[1].s64 + -352;
	// 83049704: 9421FEA0  stwu r1, -0x160(r1)
	ea = ctx.r[1].u32.wrapping_add(-352 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83049708: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8304970C: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 83049710: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 83049714: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 83049718: 807E001C  lwz r3, 0x1c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 8304971C: 93DF0174  stw r30, 0x174(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(372 as u32), ctx.r[30].u32 ) };
	// 83049720: 935F0194  stw r26, 0x194(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(404 as u32), ctx.r[26].u32 ) };
	// 83049724: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83049728: 41820018  beq 0x83049740
	if ctx.cr[0].eq {
	pc = 0x83049740; continue 'dispatch;
	}
	// 8304972C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83049730: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 83049734: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 83049738: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8304973C: 4E800421  bctrl
	ctx.lr = 0x83049740;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83049740: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83049744: 556B0739  rlwinm. r11, r11, 0, 0x1c, 0x1c
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83049748: 418200A4  beq 0x830497ec
	if ctx.cr[0].eq {
	pc = 0x830497EC; continue 'dispatch;
	}
	// 8304974C: 817E0028  lwz r11, 0x28(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 83049750: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83049754: 40820044  bne 0x83049798
	if !ctx.cr[0].eq {
	pc = 0x83049798; continue 'dispatch;
	}
	// 83049758: 38600040  li r3, 0x40
	ctx.r[3].s64 = 64;
	// 8304975C: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83049760: 4BF8EB39  bl 0x82fd8298
	ctx.lr = 0x83049764;
	sub_82FD8298(ctx, base);
	// 83049764: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83049768: 907F0064  stw r3, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[3].u32 ) };
	// 8304976C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83049770: 41820020  beq 0x83049790
	if ctx.cr[0].eq {
	pc = 0x83049790; continue 'dispatch;
	}
	// 83049774: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83049778: 809E0024  lwz r4, 0x24(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 8304977C: 80DE0004  lwz r6, 4(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83049780: 38ABD918  addi r5, r11, -0x26e8
	ctx.r[5].s64 = ctx.r[11].s64 + -9960;
	// 83049784: 48040965  bl 0x8308a0e8
	ctx.lr = 0x83049788;
	sub_8308A0E8(ctx, base);
	// 83049788: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8304978C: 48000008  b 0x83049794
	pc = 0x83049794; continue 'dispatch;
	// 83049790: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83049794: 907E0028  stw r3, 0x28(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), ctx.r[3].u32 ) };
	// 83049798: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 8304979C: 807E0028  lwz r3, 0x28(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 830497A0: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 830497A4: 48040F6D  bl 0x8308a710
	ctx.lr = 0x830497A8;
	sub_8308A710(ctx, base);
	// 830497A8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830497AC: 40820040  bne 0x830497ec
	if !ctx.cr[0].eq {
	pc = 0x830497EC; continue 'dispatch;
	}
	// 830497B0: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 830497B4: 811E0024  lwz r8, 0x24(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 830497B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830497BC: 93410054  stw r26, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[26].u32 ) };
	// 830497C0: 388B1910  addi r4, r11, 0x1910
	ctx.r[4].s64 = ctx.r[11].s64 + 6416;
	// 830497C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 830497C8: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 830497CC: 38C000EE  li r6, 0xee
	ctx.r[6].s64 = 238;
	// 830497D0: 38A00104  li r5, 0x104
	ctx.r[5].s64 = 260;
	// 830497D4: 387F00D0  addi r3, r31, 0xd0
	ctx.r[3].s64 = ctx.r[31].s64 + 208;
	// 830497D8: 4BFCC3B1  bl 0x83015b88
	ctx.lr = 0x830497DC;
	sub_83015B88(ctx, base);
	// 830497DC: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830497E0: 387F00D0  addi r3, r31, 0xd0
	ctx.r[3].s64 = ctx.r[31].s64 + 208;
	// 830497E4: 388BC8B0  addi r4, r11, -0x3750
	ctx.r[4].s64 = ctx.r[11].s64 + -14160;
	// 830497E8: 48167441  bl 0x831b0c28
	ctx.lr = 0x830497EC;
	sub_831B0C28(ctx, base);
	// 830497EC: 57AB063F  clrlwi. r11, r29, 0x18
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830497F0: 408200E4  bne 0x830498d4
	if !ctx.cr[0].eq {
	pc = 0x830498D4; continue 'dispatch;
	}
	// 830497F4: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 830497F8: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 830497FC: 387F00F0  addi r3, r31, 0xf0
	ctx.r[3].s64 = ctx.r[31].s64 + 240;
	// 83049800: 480496B1  bl 0x83092eb0
	ctx.lr = 0x83049804;
	sub_83092EB0(ctx, base);
	// 83049804: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83049808: 817E0058  lwz r11, 0x58(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(88 as u32) ) } as u64;
	// 8304980C: 3B7F00F0  addi r27, r31, 0xf0
	ctx.r[27].s64 = ctx.r[31].s64 + 240;
	// 83049810: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83049814: 418200A0  beq 0x830498b4
	if ctx.cr[0].eq {
	pc = 0x830498B4; continue 'dispatch;
	}
	// 83049818: 838B0008  lwz r28, 8(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8304981C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83049820: 7F1DE000  cmpw cr6, r29, r28
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[28].s32, &mut ctx.xer);
	// 83049824: 40980044  bge cr6, 0x83049868
	if !ctx.cr[6].lt {
	pc = 0x83049868; continue 'dispatch;
	}
	// 83049828: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8304982C: 807E0058  lwz r3, 0x58(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(88 as u32) ) } as u64;
	// 83049830: 4BFE3041  bl 0x8302c870
	ctx.lr = 0x83049834;
	sub_8302C870(ctx, base);
	// 83049834: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 83049838: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8304983C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83049840: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83049844: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83049848: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 8304984C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83049850: 4E800421  bctrl
	ctx.lr = 0x83049854;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83049854: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83049858: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8304985C: 419A000C  beq cr6, 0x83049868
	if ctx.cr[6].eq {
	pc = 0x83049868; continue 'dispatch;
	}
	// 83049860: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 83049864: 4BFFFFBC  b 0x83049820
	pc = 0x83049820; continue 'dispatch;
	// 83049868: 7F1DE000  cmpw cr6, r29, r28
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[28].s32, &mut ctx.xer);
	// 8304986C: 409A0048  bne cr6, 0x830498b4
	if !ctx.cr[6].eq {
	pc = 0x830498B4; continue 'dispatch;
	}
	// 83049870: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83049874: 93410054  stw r26, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[26].u32 ) };
	// 83049878: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8304987C: 388B1910  addi r4, r11, 0x1910
	ctx.r[4].s64 = ctx.r[11].s64 + 6416;
	// 83049880: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83049884: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83049888: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 8304988C: 38C000F4  li r6, 0xf4
	ctx.r[6].s64 = 244;
	// 83049890: 38A0011C  li r5, 0x11c
	ctx.r[5].s64 = 284;
	// 83049894: 387F0090  addi r3, r31, 0x90
	ctx.r[3].s64 = ctx.r[31].s64 + 144;
	// 83049898: 4BFCC2F1  bl 0x83015b88
	ctx.lr = 0x8304989C;
	sub_83015B88(ctx, base);
	// 8304989C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830498A0: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830498A4: 387F0090  addi r3, r31, 0x90
	ctx.r[3].s64 = ctx.r[31].s64 + 144;
	// 830498A8: 388BC8B0  addi r4, r11, -0x3750
	ctx.r[4].s64 = ctx.r[11].s64 + -14160;
	// 830498AC: 4816737D  bl 0x831b0c28
	ctx.lr = 0x830498B0;
	sub_831B0C28(ctx, base);
	// 830498B0: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830498B4: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 830498B8: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 830498BC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830498C0: 48048EA1  bl 0x83092760
	ctx.lr = 0x830498C4;
	sub_83092760(ctx, base);
	// 830498C4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830498C8: 387F00F0  addi r3, r31, 0xf0
	ctx.r[3].s64 = ctx.r[31].s64 + 240;
	// 830498CC: 48049655  bl 0x83092f20
	ctx.lr = 0x830498D0;
	sub_83092F20(ctx, base);
	// 830498D0: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 830498D4: 383F0160  addi r1, r31, 0x160
	ctx.r[1].s64 = ctx.r[31].s64 + 352;
	// 830498D8: 4815E8D4  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830498DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830498DC size=8
    let mut pc: u32 = 0x830498DC;
    'dispatch: loop {
        match pc {
            0x830498DC => {
    //   block [0x830498DC..0x830498E4)
	// 830498DC: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830498E0: 821619D8  lwz r16, 0x19d8(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(6616 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830498E4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830498E4 size=88
    let mut pc: u32 = 0x830498E4;
    'dispatch: loop {
        match pc {
            0x830498E4 => {
    //   block [0x830498E4..0x8304993C)
	// 830498E4: 3BECFEA0  addi r31, r12, -0x160
	ctx.r[31].s64 = ctx.r[12].s64 + -352;
	// 830498E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830498EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830498F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830498F4: 817F0174  lwz r11, 0x174(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(372 as u32) ) } as u64;
	// 830498F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830498FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83049900: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83049904: 38C0012B  li r6, 0x12b
	ctx.r[6].s64 = 299;
	// 83049908: 80AB0004  lwz r5, 4(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8304990C: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83049910: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 83049914: 388B1910  addi r4, r11, 0x1910
	ctx.r[4].s64 = ctx.r[11].s64 + 6416;
	// 83049918: 817F0060  lwz r11, 0x60(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 8304991C: 90A10054  stw r5, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[5].u32 ) };
	// 83049920: 38A000FA  li r5, 0xfa
	ctx.r[5].s64 = 250;
	// 83049924: 80EB0010  lwz r7, 0x10(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83049928: 4BFCC261  bl 0x83015b88
	ctx.lr = 0x8304992C;
	sub_83015B88(ctx, base);
	// 8304992C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83049930: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 83049934: 388BC8B0  addi r4, r11, -0x3750
	ctx.r[4].s64 = ctx.r[11].s64 + -14160;
	// 83049938: 481672F1  bl 0x831b0c28
	ctx.lr = 0x8304993C;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83049944(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83049944 size=84
    let mut pc: u32 = 0x83049944;
    'dispatch: loop {
        match pc {
            0x83049944 => {
    //   block [0x83049944..0x83049998)
	// 83049944: 3BECFEA0  addi r31, r12, -0x160
	ctx.r[31].s64 = ctx.r[12].s64 + -352;
	// 83049948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304994C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83049950: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83049954: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83049958: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8304995C: 388B1910  addi r4, r11, 0x1910
	ctx.r[4].s64 = ctx.r[11].s64 + 6416;
	// 83049960: 817F0068  lwz r11, 0x68(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 83049964: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83049968: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8304996C: 38C0012B  li r6, 0x12b
	ctx.r[6].s64 = 299;
	// 83049970: 38A00124  li r5, 0x124
	ctx.r[5].s64 = 292;
	// 83049974: 80EB0010  lwz r7, 0x10(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83049978: 387F00B0  addi r3, r31, 0xb0
	ctx.r[3].s64 = ctx.r[31].s64 + 176;
	// 8304997C: 817F0194  lwz r11, 0x194(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(404 as u32) ) } as u64;
	// 83049980: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 83049984: 4BFF8195  bl 0x83041b18
	ctx.lr = 0x83049988;
	sub_83041B18(ctx, base);
	// 83049988: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8304998C: 387F00B0  addi r3, r31, 0xb0
	ctx.r[3].s64 = ctx.r[31].s64 + 176;
	// 83049990: 388BC990  addi r4, r11, -0x3670
	ctx.r[4].s64 = ctx.r[11].s64 + -13936;
	// 83049994: 48167295  bl 0x831b0c28
	ctx.lr = 0x83049998;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83049998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83049998 size=48
    let mut pc: u32 = 0x83049998;
    'dispatch: loop {
        match pc {
            0x83049998 => {
    //   block [0x83049998..0x830499C8)
	// 83049998: 3BECFEA0  addi r31, r12, -0x160
	ctx.r[31].s64 = ctx.r[12].s64 + -352;
	// 8304999C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830499A0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830499A4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830499A8: 817F0174  lwz r11, 0x174(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(372 as u32) ) } as u64;
	// 830499AC: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830499B0: 807F0064  lwz r3, 0x64(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 830499B4: 4BF8E92D  bl 0x82fd82e0
	ctx.lr = 0x830499B8;
	sub_82FD82E0(ctx, base);
	// 830499B8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830499BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830499C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830499C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830499C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830499C8 size=40
    let mut pc: u32 = 0x830499C8;
    'dispatch: loop {
        match pc {
            0x830499C8 => {
    //   block [0x830499C8..0x830499F0)
	// 830499C8: 3BECFEA0  addi r31, r12, -0x160
	ctx.r[31].s64 = ctx.r[12].s64 + -352;
	// 830499CC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830499D0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830499D4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830499D8: 387F00F0  addi r3, r31, 0xf0
	ctx.r[3].s64 = ctx.r[31].s64 + 240;
	// 830499DC: 48049545  bl 0x83092f20
	ctx.lr = 0x830499E0;
	sub_83092F20(ctx, base);
	// 830499E0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830499E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830499E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830499EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830499F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830499F0 size=8
    let mut pc: u32 = 0x830499F0;
    'dispatch: loop {
        match pc {
            0x830499F0 => {
    //   block [0x830499F0..0x830499F8)
	// 830499F0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830499F4: 82161B14  lwz r16, 0x1b14(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(6932 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830499F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830499F8 size=412
    let mut pc: u32 = 0x830499F8;
    'dispatch: loop {
        match pc {
            0x830499F8 => {
    //   block [0x830499F8..0x83049B94)
	// 830499F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830499FC: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 83049A00: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 83049A04: 4815E75D  bl 0x831a8160
	ctx.lr = 0x83049A08;
	sub_831A8130(ctx, base);
	// 83049A08: 3BE1FF40  addi r31, r1, -0xc0
	ctx.r[31].s64 = ctx.r[1].s64 + -192;
	// 83049A0C: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83049A10: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83049A14: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 83049A18: 817D005C  lwz r11, 0x5c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(92 as u32) ) } as u64;
	// 83049A1C: 93BF00D4  stw r29, 0xd4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(212 as u32), ctx.r[29].u32 ) };
	// 83049A20: 935F00DC  stw r26, 0xdc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(220 as u32), ctx.r[26].u32 ) };
	// 83049A24: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83049A28: 41820164  beq 0x83049b8c
	if ctx.cr[0].eq {
	pc = 0x83049B8C; continue 'dispatch;
	}
	// 83049A2C: 839D001C  lwz r28, 0x1c(r29)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 83049A30: 836B0008  lwz r27, 8(r11)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83049A34: 281C0000  cmplwi r28, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83049A38: 41820058  beq 0x83049a90
	if ctx.cr[0].eq {
	pc = 0x83049A90; continue 'dispatch;
	}
	// 83049A3C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83049A40: 93DF0060  stw r30, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[30].u32 ) };
	// 83049A44: 7F1ED800  cmpw cr6, r30, r27
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[27].s32, &mut ctx.xer);
	// 83049A48: 40980048  bge cr6, 0x83049a90
	if !ctx.cr[6].lt {
	pc = 0x83049A90; continue 'dispatch;
	}
	// 83049A4C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83049A50: 807D005C  lwz r3, 0x5c(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(92 as u32) ) } as u64;
	// 83049A54: 4BFE2E1D  bl 0x8302c870
	ctx.lr = 0x83049A58;
	sub_8302C870(ctx, base);
	// 83049A58: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83049A5C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83049A60: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 83049A64: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 83049A68: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83049A6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83049A70: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83049A74: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 83049A78: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83049A7C: 4E800421  bctrl
	ctx.lr = 0x83049A80;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83049A80: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 83049A84: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 83049A88: 93DF0060  stw r30, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[30].u32 ) };
	// 83049A8C: 4BFFFFB8  b 0x83049a44
	pc = 0x83049A44; continue 'dispatch;
	// 83049A90: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83049A94: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 83049A98: 40990040  ble cr6, 0x83049ad8
	if !ctx.cr[6].gt {
	pc = 0x83049AD8; continue 'dispatch;
	}
	// 83049A9C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83049AA0: 839D0000  lwz r28, 0(r29)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83049AA4: 807D005C  lwz r3, 0x5c(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(92 as u32) ) } as u64;
	// 83049AA8: 4BFE2DC9  bl 0x8302c870
	ctx.lr = 0x83049AAC;
	sub_8302C870(ctx, base);
	// 83049AAC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83049AB0: 817C0040  lwz r11, 0x40(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(64 as u32) ) } as u64;
	// 83049AB4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83049AB8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83049ABC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83049AC0: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 83049AC4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83049AC8: 4E800421  bctrl
	ctx.lr = 0x83049ACC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83049ACC: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 83049AD0: 7F1ED800  cmpw cr6, r30, r27
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[27].s32, &mut ctx.xer);
	// 83049AD4: 4198FFC8  blt cr6, 0x83049a9c
	if ctx.cr[6].lt {
	pc = 0x83049A9C; continue 'dispatch;
	}
	// 83049AD8: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 83049ADC: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 83049AE0: 4BF8E7B9  bl 0x82fd8298
	ctx.lr = 0x83049AE4;
	sub_82FD8298(ctx, base);
	// 83049AE4: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83049AE8: 93DF0060  stw r30, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[30].u32 ) };
	// 83049AEC: 4182002C  beq 0x83049b18
	if ctx.cr[0].eq {
	pc = 0x83049B18; continue 'dispatch;
	}
	// 83049AF0: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 83049AF4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83049AF8: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83049AFC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83049B00: 48002CF1  bl 0x8304c7f0
	ctx.lr = 0x83049B04;
	sub_8304C7F0(ctx, base);
	// 83049B04: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83049B08: 394B2990  addi r10, r11, 0x2990
	ctx.r[10].s64 = ctx.r[11].s64 + 10640;
	// 83049B0C: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 83049B10: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83049B14: 48000008  b 0x83049b1c
	pc = 0x83049B1C; continue 'dispatch;
	// 83049B18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83049B1C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83049B20: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83049B24: 917D0058  stw r11, 0x58(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 83049B28: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 83049B2C: 995D0044  stb r10, 0x44(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(68 as u32), ctx.r[10].u8 ) };
	// 83049B30: 4099005C  ble cr6, 0x83049b8c
	if !ctx.cr[6].gt {
	pc = 0x83049B8C; continue 'dispatch;
	}
	// 83049B34: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 83049B38: 38600028  li r3, 0x28
	ctx.r[3].s64 = 40;
	// 83049B3C: 4BF8E75D  bl 0x82fd8298
	ctx.lr = 0x83049B40;
	sub_82FD8298(ctx, base);
	// 83049B40: 7C7C1B79  or. r28, r3, r3
	ctx.r[28].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 83049B44: 939F0060  stw r28, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[28].u32 ) };
	// 83049B48: 41820028  beq 0x83049b70
	if ctx.cr[0].eq {
	pc = 0x83049B70; continue 'dispatch;
	}
	// 83049B4C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83049B50: 807D005C  lwz r3, 0x5c(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(92 as u32) ) } as u64;
	// 83049B54: 4BFE2D1D  bl 0x8302c870
	ctx.lr = 0x83049B58;
	sub_8302C870(ctx, base);
	// 83049B58: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83049B5C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83049B60: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 83049B64: 4804934D  bl 0x83092eb0
	ctx.lr = 0x83049B68;
	sub_83092EB0(ctx, base);
	// 83049B68: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83049B6C: 48000008  b 0x83049b74
	pc = 0x83049B74; continue 'dispatch;
	// 83049B70: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83049B74: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83049B78: 807D0058  lwz r3, 0x58(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(88 as u32) ) } as u64;
	// 83049B7C: 4BFFF235  bl 0x83048db0
	ctx.lr = 0x83049B80;
	sub_83048DB0(ctx, base);
	// 83049B80: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 83049B84: 7F1ED800  cmpw cr6, r30, r27
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[27].s32, &mut ctx.xer);
	// 83049B88: 4198FFAC  blt cr6, 0x83049b34
	if ctx.cr[6].lt {
	pc = 0x83049B34; continue 'dispatch;
	}
	// 83049B8C: 383F00C0  addi r1, r31, 0xc0
	ctx.r[1].s64 = ctx.r[31].s64 + 192;
	// 83049B90: 4815E620  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83049B94(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83049B94 size=8
    let mut pc: u32 = 0x83049B94;
    'dispatch: loop {
        match pc {
            0x83049B94 => {
    //   block [0x83049B94..0x83049B9C)
	// 83049B94: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83049B98: 82161B14  lwz r16, 0x1b14(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(6932 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83049B9C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83049B9C size=96
    let mut pc: u32 = 0x83049B9C;
    'dispatch: loop {
        match pc {
            0x83049B9C => {
    //   block [0x83049B9C..0x83049BFC)
	// 83049B9C: 3BECFF40  addi r31, r12, -0xc0
	ctx.r[31].s64 = ctx.r[12].s64 + -192;
	// 83049BA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83049BA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83049BA8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83049BAC: 817F00D4  lwz r11, 0xd4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(212 as u32) ) } as u64;
	// 83049BB0: 809F0060  lwz r4, 0x60(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83049BB4: 83DF00DC  lwz r30, 0xdc(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(220 as u32) ) } as u64;
	// 83049BB8: 806B005C  lwz r3, 0x5c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 83049BBC: 4BFE2CB5  bl 0x8302c870
	ctx.lr = 0x83049BC0;
	sub_8302C870(ctx, base);
	// 83049BC0: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83049BC4: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 83049BC8: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 83049BCC: 388B1910  addi r4, r11, 0x1910
	ctx.r[4].s64 = ctx.r[11].s64 + 6416;
	// 83049BD0: 38C000B5  li r6, 0xb5
	ctx.r[6].s64 = 181;
	// 83049BD4: 38A000CB  li r5, 0xcb
	ctx.r[5].s64 = 203;
	// 83049BD8: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 83049BDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83049BE0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83049BE4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83049BE8: 4BFF7F31  bl 0x83041b18
	ctx.lr = 0x83049BEC;
	sub_83041B18(ctx, base);
	// 83049BEC: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83049BF0: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 83049BF4: 388BC990  addi r4, r11, -0x3670
	ctx.r[4].s64 = ctx.r[11].s64 + -13936;
	// 83049BF8: 48167031  bl 0x831b0c28
	ctx.lr = 0x83049BFC;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83049BFC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83049BFC size=44
    let mut pc: u32 = 0x83049BFC;
    'dispatch: loop {
        match pc {
            0x83049BFC => {
    //   block [0x83049BFC..0x83049C28)
	// 83049BFC: 3BECFF40  addi r31, r12, -0xc0
	ctx.r[31].s64 = ctx.r[12].s64 + -192;
	// 83049C00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83049C04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83049C08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83049C0C: 809F00DC  lwz r4, 0xdc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(220 as u32) ) } as u64;
	// 83049C10: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83049C14: 4BF8E6CD  bl 0x82fd82e0
	ctx.lr = 0x83049C18;
	sub_82FD82E0(ctx, base);
	// 83049C18: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83049C1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83049C20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83049C24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83049C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83049C28 size=44
    let mut pc: u32 = 0x83049C28;
    'dispatch: loop {
        match pc {
            0x83049C28 => {
    //   block [0x83049C28..0x83049C54)
	// 83049C28: 3BECFF40  addi r31, r12, -0xc0
	ctx.r[31].s64 = ctx.r[12].s64 + -192;
	// 83049C2C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83049C30: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83049C34: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83049C38: 809F00DC  lwz r4, 0xdc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(220 as u32) ) } as u64;
	// 83049C3C: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83049C40: 4BF8E6A1  bl 0x82fd82e0
	ctx.lr = 0x83049C44;
	sub_82FD82E0(ctx, base);
	// 83049C44: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83049C48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83049C4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83049C50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83049C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83049C58 size=80
    let mut pc: u32 = 0x83049C58;
    'dispatch: loop {
        match pc {
            0x83049C58 => {
    //   block [0x83049C58..0x83049CA8)
	// 83049C58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83049C5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83049C60: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83049C64: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83049C68: 7C882378  mr r8, r4
	ctx.r[8].u64 = ctx.r[4].u64;
	// 83049C6C: 38E0000A  li r7, 0xa
	ctx.r[7].s64 = 10;
	// 83049C70: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83049C74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83049C78: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83049C7C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83049C80: 48040C49  bl 0x8308a8c8
	ctx.lr = 0x83049C84;
	sub_8308A8C8(ctx, base);
	// 83049C84: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83049C88: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83049C8C: 396B1BA0  addi r11, r11, 0x1ba0
	ctx.r[11].s64 = ctx.r[11].s64 + 7072;
	// 83049C90: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83049C94: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83049C98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83049C9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83049CA0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83049CA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83049CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83049CA8 size=8
    let mut pc: u32 = 0x83049CA8;
    'dispatch: loop {
        match pc {
            0x83049CA8 => {
    //   block [0x83049CA8..0x83049CB0)
	// 83049CA8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83049CAC: 82161C00  lwz r16, 0x1c00(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(7168 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83049CB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83049CB0 size=84
    let mut pc: u32 = 0x83049CB0;
    'dispatch: loop {
        match pc {
            0x83049CB0 => {
    //   block [0x83049CB0..0x83049D04)
	// 83049CB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83049CB4: 4815E4B5  bl 0x831a8168
	ctx.lr = 0x83049CB8;
	sub_831A8130(ctx, base);
	// 83049CB8: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 83049CBC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83049CC0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83049CC4: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 83049CC8: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 83049CCC: 38E0000A  li r7, 0xa
	ctx.r[7].s64 = 10;
	// 83049CD0: 7D1D4378  mr r29, r8
	ctx.r[29].u64 = ctx.r[8].u64;
	// 83049CD4: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 83049CD8: 48040BF1  bl 0x8308a8c8
	ctx.lr = 0x83049CDC;
	sub_8308A8C8(ctx, base);
	// 83049CDC: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83049CE0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83049CE4: 396B1BA0  addi r11, r11, 0x1ba0
	ctx.r[11].s64 = ctx.r[11].s64 + 7072;
	// 83049CE8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83049CEC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83049CF0: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83049CF4: 48041F65  bl 0x8308bc58
	ctx.lr = 0x83049CF8;
	sub_8308BC58(ctx, base);
	// 83049CF8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83049CFC: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 83049D00: 4815E4B8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83049D04(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83049D04 size=40
    let mut pc: u32 = 0x83049D04;
    'dispatch: loop {
        match pc {
            0x83049D04 => {
    //   block [0x83049D04..0x83049D2C)
	// 83049D04: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83049D08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83049D0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83049D10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83049D14: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83049D18: 48040AB9  bl 0x8308a7d0
	ctx.lr = 0x83049D1C;
	sub_8308A7D0(ctx, base);
	// 83049D1C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83049D20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83049D24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83049D28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83049D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83049D30 size=8
    let mut pc: u32 = 0x83049D30;
    'dispatch: loop {
        match pc {
            0x83049D30 => {
    //   block [0x83049D30..0x83049D38)
	// 83049D30: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83049D34: 82161C38  lwz r16, 0x1c38(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(7224 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83049D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83049D38 size=104
    let mut pc: u32 = 0x83049D38;
    'dispatch: loop {
        match pc {
            0x83049D38 => {
    //   block [0x83049D38..0x83049DA0)
	// 83049D38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83049D3C: 4815E425  bl 0x831a8160
	ctx.lr = 0x83049D40;
	sub_831A8130(ctx, base);
	// 83049D40: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 83049D44: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83049D48: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 83049D4C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83049D50: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 83049D54: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83049D58: 38600054  li r3, 0x54
	ctx.r[3].s64 = 84;
	// 83049D5C: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 83049D60: 93DF00C4  stw r30, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[30].u32 ) };
	// 83049D64: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 83049D68: 4BF8E531  bl 0x82fd8298
	ctx.lr = 0x83049D6C;
	sub_82FD8298(ctx, base);
	// 83049D6C: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83049D70: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83049D74: 41820020  beq 0x83049d94
	if ctx.cr[0].eq {
	pc = 0x83049D94; continue 'dispatch;
	}
	// 83049D78: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 83049D7C: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 83049D80: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 83049D84: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83049D88: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83049D8C: 4BFFFF25  bl 0x83049cb0
	ctx.lr = 0x83049D90;
	sub_83049CB0(ctx, base);
	// 83049D90: 48000008  b 0x83049d98
	pc = 0x83049D98; continue 'dispatch;
	// 83049D94: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83049D98: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 83049D9C: 4815E414  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83049DA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83049DA0 size=44
    let mut pc: u32 = 0x83049DA0;
    'dispatch: loop {
        match pc {
            0x83049DA0 => {
    //   block [0x83049DA0..0x83049DCC)
	// 83049DA0: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 83049DA4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83049DA8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83049DAC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83049DB0: 809F00C4  lwz r4, 0xc4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 83049DB4: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83049DB8: 4BF8E529  bl 0x82fd82e0
	ctx.lr = 0x83049DBC;
	sub_82FD82E0(ctx, base);
	// 83049DBC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83049DC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83049DC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83049DC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83049DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83049DD0 size=136
    let mut pc: u32 = 0x83049DD0;
    'dispatch: loop {
        match pc {
            0x83049DD0 => {
    //   block [0x83049DD0..0x83049E58)
	// 83049DD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83049DD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83049DD8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83049DDC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83049DE0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83049DE4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83049DE8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83049DEC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 83049DF0: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 83049DF4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83049DF8: 4E800421  bctrl
	ctx.lr = 0x83049DFC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83049DFC: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83049E00: 41810040  bgt 0x83049e40
	if ctx.cr[0].gt {
	pc = 0x83049E40; continue 'dispatch;
	}
	// 83049E04: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83049E08: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 83049E0C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83049E10: 388B1C68  addi r4, r11, 0x1c68
	ctx.r[4].s64 = ctx.r[11].s64 + 7272;
	// 83049E14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83049E18: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83049E1C: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 83049E20: 38C000EF  li r6, 0xef
	ctx.r[6].s64 = 239;
	// 83049E24: 38A0008E  li r5, 0x8e
	ctx.r[5].s64 = 142;
	// 83049E28: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 83049E2C: 4BFCBD5D  bl 0x83015b88
	ctx.lr = 0x83049E30;
	sub_83015B88(ctx, base);
	// 83049E30: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83049E34: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 83049E38: 388BC8B0  addi r4, r11, -0x3750
	ctx.r[4].s64 = ctx.r[11].s64 + -14160;
	// 83049E3C: 48166DED  bl 0x831b0c28
	ctx.lr = 0x83049E40;
	sub_831B0C28(ctx, base);
	// 83049E40: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83049E44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83049E48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83049E4C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83049E50: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83049E54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83049E58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83049E58 size=16
    let mut pc: u32 = 0x83049E58;
    'dispatch: loop {
        match pc {
            0x83049E58 => {
    //   block [0x83049E58..0x83049E68)
	// 83049E58: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 83049E5C: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 83049E60: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83049E64: 48049BBC  b 0x83093a20
	sub_83093A20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83049E68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83049E68 size=12
    let mut pc: u32 = 0x83049E68;
    'dispatch: loop {
        match pc {
            0x83049E68 => {
    //   block [0x83049E68..0x83049E74)
	// 83049E68: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 83049E6C: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 83049E70: 4BF88500  b 0x82fd2370
	sub_82FD2370(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83049E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83049E78 size=8
    let mut pc: u32 = 0x83049E78;
    'dispatch: loop {
        match pc {
            0x83049E78 => {
    //   block [0x83049E78..0x83049E80)
	// 83049E78: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83049E7C: 82161CC8  lwz r16, 0x1cc8(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(7368 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83049E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83049E80 size=96
    let mut pc: u32 = 0x83049E80;
    'dispatch: loop {
        match pc {
            0x83049E80 => {
    //   block [0x83049E80..0x83049EE0)
	// 83049E80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83049E84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83049E88: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83049E8C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83049E90: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 83049E94: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83049E98: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83049E9C: 38600054  li r3, 0x54
	ctx.r[3].s64 = 84;
	// 83049EA0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83049EA4: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 83049EA8: 4BF8E3F1  bl 0x82fd8298
	ctx.lr = 0x83049EAC;
	sub_82FD8298(ctx, base);
	// 83049EAC: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83049EB0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83049EB4: 41820010  beq 0x83049ec4
	if ctx.cr[0].eq {
	pc = 0x83049EC4; continue 'dispatch;
	}
	// 83049EB8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83049EBC: 4BFFFD9D  bl 0x83049c58
	ctx.lr = 0x83049EC0;
	sub_83049C58(ctx, base);
	// 83049EC0: 48000008  b 0x83049ec8
	pc = 0x83049EC8; continue 'dispatch;
	// 83049EC4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83049EC8: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 83049ECC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83049ED0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83049ED4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83049ED8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83049EDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83049EE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83049EE0 size=44
    let mut pc: u32 = 0x83049EE0;
    'dispatch: loop {
        match pc {
            0x83049EE0 => {
    //   block [0x83049EE0..0x83049F0C)
	// 83049EE0: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 83049EE4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83049EE8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83049EEC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83049EF0: 809F0084  lwz r4, 0x84(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83049EF4: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83049EF8: 4BF8E3E9  bl 0x82fd82e0
	ctx.lr = 0x83049EFC;
	sub_82FD82E0(ctx, base);
	// 83049EFC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83049F00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83049F04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83049F08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83049F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83049F10 size=12
    let mut pc: u32 = 0x83049F10;
    'dispatch: loop {
        match pc {
            0x83049F10 => {
    //   block [0x83049F10..0x83049F1C)
	// 83049F10: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 83049F14: 386B33CC  addi r3, r11, 0x33cc
	ctx.r[3].s64 = ctx.r[11].s64 + 13260;
	// 83049F18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83049F20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83049F20 size=88
    let mut pc: u32 = 0x83049F20;
    'dispatch: loop {
        match pc {
            0x83049F20 => {
    //   block [0x83049F20..0x83049F78)
	// 83049F20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83049F24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83049F28: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83049F2C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83049F30: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83049F34: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83049F38: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83049F3C: 396B1BA0  addi r11, r11, 0x1ba0
	ctx.r[11].s64 = ctx.r[11].s64 + 7072;
	// 83049F40: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83049F44: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83049F48: 48040889  bl 0x8308a7d0
	ctx.lr = 0x83049F4C;
	sub_8308A7D0(ctx, base);
	// 83049F4C: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83049F50: 4182000C  beq 0x83049f5c
	if ctx.cr[0].eq {
	pc = 0x83049F5C; continue 'dispatch;
	}
	// 83049F54: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83049F58: 4BF8E389  bl 0x82fd82e0
	ctx.lr = 0x83049F5C;
	sub_82FD82E0(ctx, base);
	// 83049F5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83049F60: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83049F64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83049F68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83049F6C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83049F70: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83049F74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83049F78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83049F78 size=80
    let mut pc: u32 = 0x83049F78;
    'dispatch: loop {
        match pc {
            0x83049F78 => {
    //   block [0x83049F78..0x83049FC8)
	// 83049F78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83049F7C: 4815E1ED  bl 0x831a8168
	ctx.lr = 0x83049F80;
	sub_831A8130(ctx, base);
	// 83049F80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83049F84: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83049F88: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 83049F8C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83049F90: 817E0050  lwz r11, 0x50(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(80 as u32) ) } as u64;
	// 83049F94: 83AB0008  lwz r29, 8(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83049F98: 2C1D0000  cmpwi r29, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83049F9C: 40810024  ble 0x83049fc0
	if !ctx.cr[0].gt {
	pc = 0x83049FC0; continue 'dispatch;
	}
	// 83049FA0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83049FA4: 807E0050  lwz r3, 0x50(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(80 as u32) ) } as u64;
	// 83049FA8: 4BFE28C9  bl 0x8302c870
	ctx.lr = 0x83049FAC;
	sub_8302C870(ctx, base);
	// 83049FAC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83049FB0: 4BF883C1  bl 0x82fd2370
	ctx.lr = 0x83049FB4;
	sub_82FD2370(ctx, base);
	// 83049FB4: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 83049FB8: 7F1FE800  cmpw cr6, r31, r29
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[29].s32, &mut ctx.xer);
	// 83049FBC: 4198FFE4  blt cr6, 0x83049fa0
	if ctx.cr[6].lt {
	pc = 0x83049FA0; continue 'dispatch;
	}
	// 83049FC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83049FC4: 4815E1F4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83049FC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83049FC8 size=80
    let mut pc: u32 = 0x83049FC8;
    'dispatch: loop {
        match pc {
            0x83049FC8 => {
    //   block [0x83049FC8..0x8304A018)
	// 83049FC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83049FCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83049FD0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83049FD4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83049FD8: 7C882378  mr r8, r4
	ctx.r[8].u64 = ctx.r[4].u64;
	// 83049FDC: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 83049FE0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83049FE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83049FE8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83049FEC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83049FF0: 480408D9  bl 0x8308a8c8
	ctx.lr = 0x83049FF4;
	sub_8308A8C8(ctx, base);
	// 83049FF4: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83049FF8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83049FFC: 396B1D18  addi r11, r11, 0x1d18
	ctx.r[11].s64 = ctx.r[11].s64 + 7448;
	// 8304A000: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8304A004: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8304A008: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304A00C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304A010: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8304A014: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304A018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8304A018 size=8
    let mut pc: u32 = 0x8304A018;
    'dispatch: loop {
        match pc {
            0x8304A018 => {
    //   block [0x8304A018..0x8304A020)
	// 8304A018: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304A01C: 82161D78  lwz r16, 0x1d78(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(7544 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304A020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304A020 size=84
    let mut pc: u32 = 0x8304A020;
    'dispatch: loop {
        match pc {
            0x8304A020 => {
    //   block [0x8304A020..0x8304A074)
	// 8304A020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304A024: 4815E145  bl 0x831a8168
	ctx.lr = 0x8304A028;
	sub_831A8130(ctx, base);
	// 8304A028: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 8304A02C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304A030: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8304A034: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8304A038: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 8304A03C: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 8304A040: 7D1D4378  mr r29, r8
	ctx.r[29].u64 = ctx.r[8].u64;
	// 8304A044: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 8304A048: 48040881  bl 0x8308a8c8
	ctx.lr = 0x8304A04C;
	sub_8308A8C8(ctx, base);
	// 8304A04C: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8304A050: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8304A054: 396B1D18  addi r11, r11, 0x1d18
	ctx.r[11].s64 = ctx.r[11].s64 + 7448;
	// 8304A058: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8304A05C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304A060: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8304A064: 48041BF5  bl 0x8308bc58
	ctx.lr = 0x8304A068;
	sub_8308BC58(ctx, base);
	// 8304A068: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304A06C: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 8304A070: 4815E148  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304A074(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304A074 size=40
    let mut pc: u32 = 0x8304A074;
    'dispatch: loop {
        match pc {
            0x8304A074 => {
    //   block [0x8304A074..0x8304A09C)
	// 8304A074: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 8304A078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304A07C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304A080: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304A084: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 8304A088: 48040749  bl 0x8308a7d0
	ctx.lr = 0x8304A08C;
	sub_8308A7D0(ctx, base);
	// 8304A08C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8304A090: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304A094: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304A098: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304A0A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8304A0A0 size=8
    let mut pc: u32 = 0x8304A0A0;
    'dispatch: loop {
        match pc {
            0x8304A0A0 => {
    //   block [0x8304A0A0..0x8304A0A8)
	// 8304A0A0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304A0A4: 82161DB0  lwz r16, 0x1db0(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(7600 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304A0A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304A0A8 size=104
    let mut pc: u32 = 0x8304A0A8;
    'dispatch: loop {
        match pc {
            0x8304A0A8 => {
    //   block [0x8304A0A8..0x8304A110)
	// 8304A0A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304A0AC: 4815E0B5  bl 0x831a8160
	ctx.lr = 0x8304A0B0;
	sub_831A8130(ctx, base);
	// 8304A0B0: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 8304A0B4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304A0B8: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 8304A0BC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8304A0C0: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8304A0C4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8304A0C8: 38600054  li r3, 0x54
	ctx.r[3].s64 = 84;
	// 8304A0CC: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 8304A0D0: 93DF00C4  stw r30, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[30].u32 ) };
	// 8304A0D4: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 8304A0D8: 4BF8E1C1  bl 0x82fd8298
	ctx.lr = 0x8304A0DC;
	sub_82FD8298(ctx, base);
	// 8304A0DC: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8304A0E0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8304A0E4: 41820020  beq 0x8304a104
	if ctx.cr[0].eq {
	pc = 0x8304A104; continue 'dispatch;
	}
	// 8304A0E8: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 8304A0EC: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 8304A0F0: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 8304A0F4: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8304A0F8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8304A0FC: 4BFFFF25  bl 0x8304a020
	ctx.lr = 0x8304A100;
	sub_8304A020(ctx, base);
	// 8304A100: 48000008  b 0x8304a108
	pc = 0x8304A108; continue 'dispatch;
	// 8304A104: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8304A108: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 8304A10C: 4815E0A4  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304A110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304A110 size=44
    let mut pc: u32 = 0x8304A110;
    'dispatch: loop {
        match pc {
            0x8304A110 => {
    //   block [0x8304A110..0x8304A13C)
	// 8304A110: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 8304A114: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304A118: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304A11C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304A120: 809F00C4  lwz r4, 0xc4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 8304A124: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8304A128: 4BF8E1B9  bl 0x82fd82e0
	ctx.lr = 0x8304A12C;
	sub_82FD82E0(ctx, base);
	// 8304A12C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8304A130: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304A134: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304A138: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304A140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304A140 size=136
    let mut pc: u32 = 0x8304A140;
    'dispatch: loop {
        match pc {
            0x8304A140 => {
    //   block [0x8304A140..0x8304A1C8)
	// 8304A140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304A144: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304A148: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8304A14C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8304A150: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304A154: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304A158: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8304A15C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8304A160: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 8304A164: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8304A168: 4E800421  bctrl
	ctx.lr = 0x8304A16C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8304A16C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8304A170: 41810040  bgt 0x8304a1b0
	if ctx.cr[0].gt {
	pc = 0x8304A1B0; continue 'dispatch;
	}
	// 8304A174: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8304A178: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 8304A17C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8304A180: 388B1DE0  addi r4, r11, 0x1de0
	ctx.r[4].s64 = ctx.r[11].s64 + 7648;
	// 8304A184: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8304A188: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8304A18C: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 8304A190: 38C000F0  li r6, 0xf0
	ctx.r[6].s64 = 240;
	// 8304A194: 38A00088  li r5, 0x88
	ctx.r[5].s64 = 136;
	// 8304A198: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8304A19C: 4BFCB9ED  bl 0x83015b88
	ctx.lr = 0x8304A1A0;
	sub_83015B88(ctx, base);
	// 8304A1A0: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8304A1A4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8304A1A8: 388BC8B0  addi r4, r11, -0x3750
	ctx.r[4].s64 = ctx.r[11].s64 + -14160;
	// 8304A1AC: 48166A7D  bl 0x831b0c28
	ctx.lr = 0x8304A1B0;
	sub_831B0C28(ctx, base);
	// 8304A1B0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8304A1B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304A1B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304A1BC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8304A1C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8304A1C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304A1C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8304A1C8 size=8
    let mut pc: u32 = 0x8304A1C8;
    'dispatch: loop {
        match pc {
            0x8304A1C8 => {
    //   block [0x8304A1C8..0x8304A1D0)
	// 8304A1C8: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8304A1CC: 48049A1C  b 0x83093be8
	sub_83093BE8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304A1D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8304A1D0 size=8
    let mut pc: u32 = 0x8304A1D0;
    'dispatch: loop {
        match pc {
            0x8304A1D0 => {
    //   block [0x8304A1D0..0x8304A1D8)
	// 8304A1D0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304A1D4: 82161E38  lwz r16, 0x1e38(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(7736 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304A1D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304A1D8 size=96
    let mut pc: u32 = 0x8304A1D8;
    'dispatch: loop {
        match pc {
            0x8304A1D8 => {
    //   block [0x8304A1D8..0x8304A238)
	// 8304A1D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304A1DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304A1E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8304A1E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8304A1E8: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 8304A1EC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304A1F0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8304A1F4: 38600054  li r3, 0x54
	ctx.r[3].s64 = 84;
	// 8304A1F8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8304A1FC: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 8304A200: 4BF8E099  bl 0x82fd8298
	ctx.lr = 0x8304A204;
	sub_82FD8298(ctx, base);
	// 8304A204: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8304A208: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8304A20C: 41820010  beq 0x8304a21c
	if ctx.cr[0].eq {
	pc = 0x8304A21C; continue 'dispatch;
	}
	// 8304A210: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8304A214: 4BFFFDB5  bl 0x83049fc8
	ctx.lr = 0x8304A218;
	sub_83049FC8(ctx, base);
	// 8304A218: 48000008  b 0x8304a220
	pc = 0x8304A220; continue 'dispatch;
	// 8304A21C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8304A220: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 8304A224: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304A228: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304A22C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8304A230: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8304A234: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304A238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304A238 size=44
    let mut pc: u32 = 0x8304A238;
    'dispatch: loop {
        match pc {
            0x8304A238 => {
    //   block [0x8304A238..0x8304A264)
	// 8304A238: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 8304A23C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304A240: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304A244: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304A248: 809F0084  lwz r4, 0x84(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 8304A24C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8304A250: 4BF8E091  bl 0x82fd82e0
	ctx.lr = 0x8304A254;
	sub_82FD82E0(ctx, base);
	// 8304A254: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8304A258: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304A25C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304A260: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304A268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8304A268 size=12
    let mut pc: u32 = 0x8304A268;
    'dispatch: loop {
        match pc {
            0x8304A268 => {
    //   block [0x8304A268..0x8304A274)
	// 8304A268: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8304A26C: 386B33D4  addi r3, r11, 0x33d4
	ctx.r[3].s64 = ctx.r[11].s64 + 13268;
	// 8304A270: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304A278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304A278 size=88
    let mut pc: u32 = 0x8304A278;
    'dispatch: loop {
        match pc {
            0x8304A278 => {
    //   block [0x8304A278..0x8304A2D0)
	// 8304A278: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304A27C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304A280: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8304A284: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8304A288: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304A28C: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8304A290: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8304A294: 396B1D18  addi r11, r11, 0x1d18
	ctx.r[11].s64 = ctx.r[11].s64 + 7448;
	// 8304A298: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8304A29C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8304A2A0: 48040531  bl 0x8308a7d0
	ctx.lr = 0x8304A2A4;
	sub_8308A7D0(ctx, base);
	// 8304A2A4: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304A2A8: 4182000C  beq 0x8304a2b4
	if ctx.cr[0].eq {
	pc = 0x8304A2B4; continue 'dispatch;
	}
	// 8304A2AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8304A2B0: 4BF8E031  bl 0x82fd82e0
	ctx.lr = 0x8304A2B4;
	sub_82FD82E0(ctx, base);
	// 8304A2B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8304A2B8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8304A2BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304A2C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304A2C4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8304A2C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8304A2CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304A2D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304A2D0 size=108
    let mut pc: u32 = 0x8304A2D0;
    'dispatch: loop {
        match pc {
            0x8304A2D0 => {
    //   block [0x8304A2D0..0x8304A33C)
	// 8304A2D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304A2D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304A2D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8304A2DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304A2E0: 7C882378  mr r8, r4
	ctx.r[8].u64 = ctx.r[4].u64;
	// 8304A2E4: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 8304A2E8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8304A2EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8304A2F0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8304A2F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8304A2F8: 48048409  bl 0x83092700
	ctx.lr = 0x8304A2FC;
	sub_83092700(ctx, base);
	// 8304A2FC: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8304A300: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8304A304: 396B1E88  addi r11, r11, 0x1e88
	ctx.r[11].s64 = ctx.r[11].s64 + 7816;
	// 8304A308: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8304A30C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8304A310: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8304A314: 915F0060  stw r10, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 8304A318: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8304A31C: 915F0064  stw r10, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[10].u32 ) };
	// 8304A320: 913F0038  stw r9, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[9].u32 ) };
	// 8304A324: 991F003E  stb r8, 0x3e(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(62 as u32), ctx.r[8].u8 ) };
	// 8304A328: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8304A32C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304A330: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304A334: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8304A338: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304A340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8304A340 size=8
    let mut pc: u32 = 0x8304A340;
    'dispatch: loop {
        match pc {
            0x8304A340 => {
    //   block [0x8304A340..0x8304A348)
	// 8304A340: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304A344: 82161EE8  lwz r16, 0x1ee8(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(7912 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304A348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304A348 size=96
    let mut pc: u32 = 0x8304A348;
    'dispatch: loop {
        match pc {
            0x8304A348 => {
    //   block [0x8304A348..0x8304A3A8)
	// 8304A348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304A34C: 4815DE1D  bl 0x831a8168
	ctx.lr = 0x8304A350;
	sub_831A8130(ctx, base);
	// 8304A350: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 8304A354: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304A358: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8304A35C: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8304A360: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 8304A364: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 8304A368: 7D1D4378  mr r29, r8
	ctx.r[29].u64 = ctx.r[8].u64;
	// 8304A36C: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 8304A370: 48048391  bl 0x83092700
	ctx.lr = 0x8304A374;
	sub_83092700(ctx, base);
	// 8304A374: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8304A378: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8304A37C: 396B1E88  addi r11, r11, 0x1e88
	ctx.r[11].s64 = ctx.r[11].s64 + 7816;
	// 8304A380: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8304A384: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8304A388: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304A38C: 915E0060  stw r10, 0x60(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 8304A390: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8304A394: 915E0064  stw r10, 0x64(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(100 as u32), ctx.r[10].u32 ) };
	// 8304A398: 48044209  bl 0x8308e5a0
	ctx.lr = 0x8304A39C;
	sub_8308E5A0(ctx, base);
	// 8304A39C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304A3A0: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 8304A3A4: 4815DE14  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304A3A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304A3A8 size=40
    let mut pc: u32 = 0x8304A3A8;
    'dispatch: loop {
        match pc {
            0x8304A3A8 => {
    //   block [0x8304A3A8..0x8304A3D0)
	// 8304A3A8: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 8304A3AC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304A3B0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304A3B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304A3B8: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 8304A3BC: 480482DD  bl 0x83092698
	ctx.lr = 0x8304A3C0;
	sub_83092698(ctx, base);
	// 8304A3C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8304A3C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304A3C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304A3CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304A3D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8304A3D0 size=8
    let mut pc: u32 = 0x8304A3D0;
    'dispatch: loop {
        match pc {
            0x8304A3D0 => {
    //   block [0x8304A3D0..0x8304A3D8)
	// 8304A3D0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304A3D4: 82161F28  lwz r16, 0x1f28(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(7976 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304A3D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304A3D8 size=116
    let mut pc: u32 = 0x8304A3D8;
    'dispatch: loop {
        match pc {
            0x8304A3D8 => {
    //   block [0x8304A3D8..0x8304A44C)
	// 8304A3D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304A3DC: 4815DD8D  bl 0x831a8168
	ctx.lr = 0x8304A3E0;
	sub_831A8130(ctx, base);
	// 8304A3E0: 3BE1FF40  addi r31, r1, -0xc0
	ctx.r[31].s64 = ctx.r[1].s64 + -192;
	// 8304A3E4: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304A3E8: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 8304A3EC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8304A3F0: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8304A3F4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8304A3F8: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 8304A3FC: 48049D85  bl 0x83094180
	ctx.lr = 0x8304A400;
	sub_83094180(ctx, base);
	// 8304A400: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8304A404: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8304A408: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 8304A40C: 48049D75  bl 0x83094180
	ctx.lr = 0x8304A410;
	sub_83094180(ctx, base);
	// 8304A410: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304A414: 38BF0050  addi r5, r31, 0x50
	ctx.r[5].s64 = ctx.r[31].s64 + 80;
	// 8304A418: 389F0070  addi r4, r31, 0x70
	ctx.r[4].s64 = ctx.r[31].s64 + 112;
	// 8304A41C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304A420: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 8304A424: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8304A428: 4E800421  bctrl
	ctx.lr = 0x8304A42C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8304A42C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8304A430: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 8304A434: 48049F2D  bl 0x83094360
	ctx.lr = 0x8304A438;
	sub_83094360(ctx, base);
	// 8304A438: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 8304A43C: 48049F25  bl 0x83094360
	ctx.lr = 0x8304A440;
	sub_83094360(ctx, base);
	// 8304A440: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304A444: 383F00C0  addi r1, r31, 0xc0
	ctx.r[1].s64 = ctx.r[31].s64 + 192;
	// 8304A448: 4815DD70  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304A44C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304A44C size=40
    let mut pc: u32 = 0x8304A44C;
    'dispatch: loop {
        match pc {
            0x8304A44C => {
    //   block [0x8304A44C..0x8304A474)
	// 8304A44C: 3BECFF40  addi r31, r12, -0xc0
	ctx.r[31].s64 = ctx.r[12].s64 + -192;
	// 8304A450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304A454: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304A458: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304A45C: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 8304A460: 48049F01  bl 0x83094360
	ctx.lr = 0x8304A464;
	sub_83094360(ctx, base);
	// 8304A464: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8304A468: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304A46C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304A470: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304A474(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304A474 size=40
    let mut pc: u32 = 0x8304A474;
    'dispatch: loop {
        match pc {
            0x8304A474 => {
    //   block [0x8304A474..0x8304A49C)
	// 8304A474: 3BECFF40  addi r31, r12, -0xc0
	ctx.r[31].s64 = ctx.r[12].s64 + -192;
	// 8304A478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304A47C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304A480: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304A484: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 8304A488: 48049ED9  bl 0x83094360
	ctx.lr = 0x8304A48C;
	sub_83094360(ctx, base);
	// 8304A48C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8304A490: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304A494: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304A498: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304A4A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8304A4A0 size=8
    let mut pc: u32 = 0x8304A4A0;
    'dispatch: loop {
        match pc {
            0x8304A4A0 => {
    //   block [0x8304A4A0..0x8304A4A8)
	// 8304A4A0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304A4A4: 82161F80  lwz r16, 0x1f80(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(8064 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304A4A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304A4A8 size=104
    let mut pc: u32 = 0x8304A4A8;
    'dispatch: loop {
        match pc {
            0x8304A4A8 => {
    //   block [0x8304A4A8..0x8304A510)
	// 8304A4A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304A4AC: 4815DCB5  bl 0x831a8160
	ctx.lr = 0x8304A4B0;
	sub_831A8130(ctx, base);
	// 8304A4B0: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 8304A4B4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304A4B8: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 8304A4BC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8304A4C0: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8304A4C4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8304A4C8: 38600068  li r3, 0x68
	ctx.r[3].s64 = 104;
	// 8304A4CC: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 8304A4D0: 93DF00C4  stw r30, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[30].u32 ) };
	// 8304A4D4: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 8304A4D8: 4BF8DDC1  bl 0x82fd8298
	ctx.lr = 0x8304A4DC;
	sub_82FD8298(ctx, base);
	// 8304A4DC: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8304A4E0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8304A4E4: 41820020  beq 0x8304a504
	if ctx.cr[0].eq {
	pc = 0x8304A504; continue 'dispatch;
	}
	// 8304A4E8: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 8304A4EC: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 8304A4F0: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 8304A4F4: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8304A4F8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8304A4FC: 4BFFFE4D  bl 0x8304a348
	ctx.lr = 0x8304A500;
	sub_8304A348(ctx, base);
	// 8304A500: 48000008  b 0x8304a508
	pc = 0x8304A508; continue 'dispatch;
	// 8304A504: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8304A508: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 8304A50C: 4815DCA4  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304A510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304A510 size=44
    let mut pc: u32 = 0x8304A510;
    'dispatch: loop {
        match pc {
            0x8304A510 => {
    //   block [0x8304A510..0x8304A53C)
	// 8304A510: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 8304A514: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304A518: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304A51C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304A520: 809F00C4  lwz r4, 0xc4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 8304A524: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8304A528: 4BF8DDB9  bl 0x82fd82e0
	ctx.lr = 0x8304A52C;
	sub_82FD82E0(ctx, base);
	// 8304A52C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8304A530: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304A534: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304A538: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304A540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8304A540 size=8
    let mut pc: u32 = 0x8304A540;
    'dispatch: loop {
        match pc {
            0x8304A540 => {
    //   block [0x8304A540..0x8304A548)
	// 8304A540: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304A544: 82162068  lwz r16, 0x2068(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(8296 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304A548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304A548 size=364
    let mut pc: u32 = 0x8304A548;
    'dispatch: loop {
        match pc {
            0x8304A548 => {
    //   block [0x8304A548..0x8304A6B4)
	// 8304A548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304A54C: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 8304A550: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 8304A554: 4815DC11  bl 0x831a8164
	ctx.lr = 0x8304A558;
	sub_831A8130(ctx, base);
	// 8304A558: 3BE1FED0  addi r31, r1, -0x130
	ctx.r[31].s64 = ctx.r[1].s64 + -304;
	// 8304A55C: 9421FED0  stwu r1, -0x130(r1)
	ea = ctx.r[1].u32.wrapping_add(-304 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304A560: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8304A564: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 8304A568: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8304A56C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8304A570: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8304A574: 388BCF94  addi r4, r11, -0x306c
	ctx.r[4].s64 = ctx.r[11].s64 + -12396;
	// 8304A578: 939F0154  stw r28, 0x154(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(340 as u32), ctx.r[28].u32 ) };
	// 8304A57C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304A580: 93DF015C  stw r30, 0x15c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(348 as u32), ctx.r[30].u32 ) };
	// 8304A584: 4BF896BD  bl 0x82fd3c40
	ctx.lr = 0x8304A588;
	sub_82FD3C40(ctx, base);
	// 8304A588: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304A58C: 41820068  beq 0x8304a5f4
	if ctx.cr[0].eq {
	pc = 0x8304A5F4; continue 'dispatch;
	}
	// 8304A590: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8304A594: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8304A598: 4BF89031  bl 0x82fd35c8
	ctx.lr = 0x8304A59C;
	sub_82FD35C8(ctx, base);
	// 8304A59C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8304A5A0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8304A5A4: 41990040  bgt cr6, 0x8304a5e4
	if ctx.cr[6].gt {
	pc = 0x8304A5E4; continue 'dispatch;
	}
	// 8304A5A8: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8304A5AC: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 8304A5B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8304A5B4: 388B1FB0  addi r4, r11, 0x1fb0
	ctx.r[4].s64 = ctx.r[11].s64 + 8112;
	// 8304A5B8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8304A5BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8304A5C0: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 8304A5C4: 38C000BF  li r6, 0xbf
	ctx.r[6].s64 = 191;
	// 8304A5C8: 38A00109  li r5, 0x109
	ctx.r[5].s64 = 265;
	// 8304A5CC: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 8304A5D0: 4BFF7549  bl 0x83041b18
	ctx.lr = 0x8304A5D4;
	sub_83041B18(ctx, base);
	// 8304A5D4: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8304A5D8: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 8304A5DC: 388BC990  addi r4, r11, -0x3670
	ctx.r[4].s64 = ctx.r[11].s64 + -13936;
	// 8304A5E0: 48166649  bl 0x831b0c28
	ctx.lr = 0x8304A5E4;
	sub_831B0C28(ctx, base);
	// 8304A5E4: 817B0010  lwz r11, 0x10(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(16 as u32) ) } as u64;
	// 8304A5E8: 907B0060  stw r3, 0x60(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 8304A5EC: 616B0200  ori r11, r11, 0x200
	ctx.r[11].u64 = ctx.r[11].u64 | 512;
	// 8304A5F0: 4800007C  b 0x8304a66c
	pc = 0x8304A66C; continue 'dispatch;
	// 8304A5F4: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8304A5F8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304A5FC: 388BCFAC  addi r4, r11, -0x3054
	ctx.r[4].s64 = ctx.r[11].s64 + -12372;
	// 8304A600: 4BF89641  bl 0x82fd3c40
	ctx.lr = 0x8304A604;
	sub_82FD3C40(ctx, base);
	// 8304A604: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304A608: 41820070  beq 0x8304a678
	if ctx.cr[0].eq {
	pc = 0x8304A678; continue 'dispatch;
	}
	// 8304A60C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8304A610: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8304A614: 4BF88FB5  bl 0x82fd35c8
	ctx.lr = 0x8304A618;
	sub_82FD35C8(ctx, base);
	// 8304A618: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8304A61C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8304A620: 40980040  bge cr6, 0x8304a660
	if !ctx.cr[6].lt {
	pc = 0x8304A660; continue 'dispatch;
	}
	// 8304A624: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8304A628: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 8304A62C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8304A630: 388B1FB0  addi r4, r11, 0x1fb0
	ctx.r[4].s64 = ctx.r[11].s64 + 8112;
	// 8304A634: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8304A638: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8304A63C: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 8304A640: 38C000C0  li r6, 0xc0
	ctx.r[6].s64 = 192;
	// 8304A644: 38A0011C  li r5, 0x11c
	ctx.r[5].s64 = 284;
	// 8304A648: 387F00A0  addi r3, r31, 0xa0
	ctx.r[3].s64 = ctx.r[31].s64 + 160;
	// 8304A64C: 4BFF74CD  bl 0x83041b18
	ctx.lr = 0x8304A650;
	sub_83041B18(ctx, base);
	// 8304A650: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8304A654: 387F00A0  addi r3, r31, 0xa0
	ctx.r[3].s64 = ctx.r[31].s64 + 160;
	// 8304A658: 388BC990  addi r4, r11, -0x3670
	ctx.r[4].s64 = ctx.r[11].s64 + -13936;
	// 8304A65C: 481665CD  bl 0x831b0c28
	ctx.lr = 0x8304A660;
	sub_831B0C28(ctx, base);
	// 8304A660: 817B0010  lwz r11, 0x10(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(16 as u32) ) } as u64;
	// 8304A664: 907B0064  stw r3, 0x64(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(100 as u32), ctx.r[3].u32 ) };
	// 8304A668: 616B0400  ori r11, r11, 0x400
	ctx.r[11].u64 = ctx.r[11].u64 | 1024;
	// 8304A66C: 917B0010  stw r11, 0x10(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8304A670: 383F0130  addi r1, r31, 0x130
	ctx.r[1].s64 = ctx.r[31].s64 + 304;
	// 8304A674: 4815DB40  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 8304A678: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8304A67C: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 8304A680: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8304A684: 388B1FB0  addi r4, r11, 0x1fb0
	ctx.r[4].s64 = ctx.r[11].s64 + 8112;
	// 8304A688: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8304A68C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8304A690: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 8304A694: 38C000AB  li r6, 0xab
	ctx.r[6].s64 = 171;
	// 8304A698: 38A00126  li r5, 0x126
	ctx.r[5].s64 = 294;
	// 8304A69C: 387F00E0  addi r3, r31, 0xe0
	ctx.r[3].s64 = ctx.r[31].s64 + 224;
	// 8304A6A0: 4BFF7479  bl 0x83041b18
	ctx.lr = 0x8304A6A4;
	sub_83041B18(ctx, base);
	// 8304A6A4: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8304A6A8: 387F00E0  addi r3, r31, 0xe0
	ctx.r[3].s64 = ctx.r[31].s64 + 224;
	// 8304A6AC: 388BC990  addi r4, r11, -0x3670
	ctx.r[4].s64 = ctx.r[11].s64 + -13936;
	// 8304A6B0: 48166579  bl 0x831b0c28
	ctx.lr = 0x8304A6B4;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304A6BC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304A6BC size=80
    let mut pc: u32 = 0x8304A6BC;
    'dispatch: loop {
        match pc {
            0x8304A6BC => {
    //   block [0x8304A6BC..0x8304A70C)
	// 8304A6BC: 3BECFED0  addi r31, r12, -0x130
	ctx.r[31].s64 = ctx.r[12].s64 + -304;
	// 8304A6C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304A6C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304A6C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304A6CC: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8304A6D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8304A6D4: 80FF0154  lwz r7, 0x154(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(340 as u32) ) } as u64;
	// 8304A6D8: 388B1FB0  addi r4, r11, 0x1fb0
	ctx.r[4].s64 = ctx.r[11].s64 + 8112;
	// 8304A6DC: 817F015C  lwz r11, 0x15c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(348 as u32) ) } as u64;
	// 8304A6E0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8304A6E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8304A6E8: 38C000BD  li r6, 0xbd
	ctx.r[6].s64 = 189;
	// 8304A6EC: 38A00104  li r5, 0x104
	ctx.r[5].s64 = 260;
	// 8304A6F0: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 8304A6F4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8304A6F8: 4BFF7421  bl 0x83041b18
	ctx.lr = 0x8304A6FC;
	sub_83041B18(ctx, base);
	// 8304A6FC: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8304A700: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 8304A704: 388BC990  addi r4, r11, -0x3670
	ctx.r[4].s64 = ctx.r[11].s64 + -13936;
	// 8304A708: 48166521  bl 0x831b0c28
	ctx.lr = 0x8304A70C;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304A714(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304A714 size=80
    let mut pc: u32 = 0x8304A714;
    'dispatch: loop {
        match pc {
            0x8304A714 => {
    //   block [0x8304A714..0x8304A764)
	// 8304A714: 3BECFED0  addi r31, r12, -0x130
	ctx.r[31].s64 = ctx.r[12].s64 + -304;
	// 8304A718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304A71C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304A720: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304A724: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8304A728: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8304A72C: 80FF0154  lwz r7, 0x154(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(340 as u32) ) } as u64;
	// 8304A730: 388B1FB0  addi r4, r11, 0x1fb0
	ctx.r[4].s64 = ctx.r[11].s64 + 8112;
	// 8304A734: 817F015C  lwz r11, 0x15c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(348 as u32) ) } as u64;
	// 8304A738: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8304A73C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8304A740: 38C000BE  li r6, 0xbe
	ctx.r[6].s64 = 190;
	// 8304A744: 38A00117  li r5, 0x117
	ctx.r[5].s64 = 279;
	// 8304A748: 387F00C0  addi r3, r31, 0xc0
	ctx.r[3].s64 = ctx.r[31].s64 + 192;
	// 8304A74C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8304A750: 4BFF73C9  bl 0x83041b18
	ctx.lr = 0x8304A754;
	sub_83041B18(ctx, base);
	// 8304A754: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8304A758: 387F00C0  addi r3, r31, 0xc0
	ctx.r[3].s64 = ctx.r[31].s64 + 192;
	// 8304A75C: 388BC990  addi r4, r11, -0x3670
	ctx.r[4].s64 = ctx.r[11].s64 + -13936;
	// 8304A760: 481664C9  bl 0x831b0c28
	ctx.lr = 0x8304A764;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304A768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8304A768 size=12
    let mut pc: u32 = 0x8304A768;
    'dispatch: loop {
        match pc {
            0x8304A768 => {
    //   block [0x8304A768..0x8304A774)
	// 8304A768: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 8304A76C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8304A770: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304A774(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8304A774 size=48
    let mut pc: u32 = 0x8304A774;
    'dispatch: loop {
        match pc {
            0x8304A774 => {
    //   block [0x8304A774..0x8304A7A4)
	// 8304A774: 812B0010  lwz r9, 0x10(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8304A778: 81430010  lwz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 8304A77C: 552805AD  rlwinm. r8, r9, 0, 0x16, 0x16
	ctx.r[8].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 8304A780: 4182001C  beq 0x8304a79c
	if ctx.cr[0].eq {
	pc = 0x8304A79C; continue 'dispatch;
	}
	// 8304A784: 554805AD  rlwinm. r8, r10, 0, 0x16, 0x16
	ctx.r[8].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 8304A788: 40820014  bne 0x8304a79c
	if !ctx.cr[0].eq {
	pc = 0x8304A79C; continue 'dispatch;
	}
	// 8304A78C: 810B0060  lwz r8, 0x60(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(96 as u32) ) } as u64;
	// 8304A790: 61470200  ori r7, r10, 0x200
	ctx.r[7].u64 = ctx.r[10].u64 | 512;
	// 8304A794: 91030060  stw r8, 0x60(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(96 as u32), ctx.r[8].u32 ) };
	// 8304A798: 90E30010  stw r7, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[7].u32 ) };
	// 8304A79C: 5529056B  rlwinm. r9, r9, 0, 0x15, 0x15
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8304A7A0: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304A7A4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8304A7A4 size=8
    let mut pc: u32 = 0x8304A7A4;
    'dispatch: loop {
        match pc {
            0x8304A7A4 => {
    //   block [0x8304A7A4..0x8304A7AC)
	// 8304A7A4: 554A056B  rlwinm. r10, r10, 0, 0x15, 0x15
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8304A7A8: 4C820020  bnelr
	if !ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304A7AC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8304A7AC size=24
    let mut pc: u32 = 0x8304A7AC;
    'dispatch: loop {
        match pc {
            0x8304A7AC => {
    //   block [0x8304A7AC..0x8304A7C4)
	// 8304A7AC: 81430010  lwz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 8304A7B0: 816B0064  lwz r11, 0x64(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(100 as u32) ) } as u64;
	// 8304A7B4: 614A0400  ori r10, r10, 0x400
	ctx.r[10].u64 = ctx.r[10].u64 | 1024;
	// 8304A7B8: 91630064  stw r11, 0x64(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8304A7BC: 91430010  stw r10, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 8304A7C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304A7C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304A7C8 size=192
    let mut pc: u32 = 0x8304A7C8;
    'dispatch: loop {
        match pc {
            0x8304A7C8 => {
    //   block [0x8304A7C8..0x8304A888)
	// 8304A7C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304A7CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304A7D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8304A7D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8304A7D8: 9421FE50  stwu r1, -0x1b0(r1)
	ea = ctx.r[1].u32.wrapping_add(-432 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304A7DC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8304A7E0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8304A7E4: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 8304A7E8: 556A056B  rlwinm. r10, r11, 0, 0x15, 0x15
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8304A7EC: 41820084  beq 0x8304a870
	if ctx.cr[0].eq {
	pc = 0x8304A870; continue 'dispatch;
	}
	// 8304A7F0: 556B05AD  rlwinm. r11, r11, 0, 0x16, 0x16
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304A7F4: 4182007C  beq 0x8304a870
	if ctx.cr[0].eq {
	pc = 0x8304A870; continue 'dispatch;
	}
	// 8304A7F8: 807E0064  lwz r3, 0x64(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(100 as u32) ) } as u64;
	// 8304A7FC: 817E0060  lwz r11, 0x60(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(96 as u32) ) } as u64;
	// 8304A800: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8304A804: 4099006C  ble cr6, 0x8304a870
	if !ctx.cr[6].gt {
	pc = 0x8304A870; continue 'dispatch;
	}
	// 8304A808: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 8304A80C: 38C0000A  li r6, 0xa
	ctx.r[6].s64 = 10;
	// 8304A810: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 8304A814: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 8304A818: 4BF87051  bl 0x82fd1868
	ctx.lr = 0x8304A81C;
	sub_82FD1868(ctx, base);
	// 8304A81C: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 8304A820: 38C0000A  li r6, 0xa
	ctx.r[6].s64 = 10;
	// 8304A824: 807E0060  lwz r3, 0x60(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(96 as u32) ) } as u64;
	// 8304A828: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 8304A82C: 38810110  addi r4, r1, 0x110
	ctx.r[4].s64 = ctx.r[1].s64 + 272;
	// 8304A830: 4BF87039  bl 0x82fd1868
	ctx.lr = 0x8304A834;
	sub_82FD1868(ctx, base);
	// 8304A834: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8304A838: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8304A83C: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 8304A840: 388B1FB0  addi r4, r11, 0x1fb0
	ctx.r[4].s64 = ctx.r[11].s64 + 8112;
	// 8304A844: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8304A848: 39010080  addi r8, r1, 0x80
	ctx.r[8].s64 = ctx.r[1].s64 + 128;
	// 8304A84C: 38E10110  addi r7, r1, 0x110
	ctx.r[7].s64 = ctx.r[1].s64 + 272;
	// 8304A850: 38C000C7  li r6, 0xc7
	ctx.r[6].s64 = 199;
	// 8304A854: 38A00158  li r5, 0x158
	ctx.r[5].s64 = 344;
	// 8304A858: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8304A85C: 4BFF72BD  bl 0x83041b18
	ctx.lr = 0x8304A860;
	sub_83041B18(ctx, base);
	// 8304A860: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8304A864: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8304A868: 388BC990  addi r4, r11, -0x3670
	ctx.r[4].s64 = ctx.r[11].s64 + -13936;
	// 8304A86C: 481663BD  bl 0x831b0c28
	ctx.lr = 0x8304A870;
	sub_831B0C28(ctx, base);
	// 8304A870: 382101B0  addi r1, r1, 0x1b0
	ctx.r[1].s64 = ctx.r[1].s64 + 432;
	// 8304A874: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304A878: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304A87C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8304A880: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8304A884: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304A888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304A888 size=752
    let mut pc: u32 = 0x8304A888;
    'dispatch: loop {
        match pc {
            0x8304A888 => {
    //   block [0x8304A888..0x8304AB78)
	// 8304A888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304A88C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304A890: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8304A894: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8304A898: 9421FE50  stwu r1, -0x1b0(r1)
	ea = ctx.r[1].u32.wrapping_add(-432 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304A89C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8304A8A0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8304A8A4: 83CB001C  lwz r30, 0x1c(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 8304A8A8: 281E0000  cmplwi r30, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8304A8AC: 418202B4  beq 0x8304ab60
	if ctx.cr[0].eq {
	pc = 0x8304AB60; continue 'dispatch;
	}
	// 8304A8B0: 812B0010  lwz r9, 0x10(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8304A8B4: 811E0010  lwz r8, 0x10(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 8304A8B8: 552A05AD  rlwinm. r10, r9, 0, 0x16, 0x16
	ctx.r[10].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8304A8BC: 41820110  beq 0x8304a9cc
	if ctx.cr[0].eq {
	pc = 0x8304A9CC; continue 'dispatch;
	}
	// 8304A8C0: 550A05AD  rlwinm. r10, r8, 0, 0x16, 0x16
	ctx.r[10].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8304A8C4: 4182007C  beq 0x8304a940
	if ctx.cr[0].eq {
	pc = 0x8304A940; continue 'dispatch;
	}
	// 8304A8C8: 806B0060  lwz r3, 0x60(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(96 as u32) ) } as u64;
	// 8304A8CC: 80FE0060  lwz r7, 0x60(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(96 as u32) ) } as u64;
	// 8304A8D0: 7F033840  cmplw cr6, r3, r7
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[7].u32, &mut ctx.xer);
	// 8304A8D4: 4099006C  ble cr6, 0x8304a940
	if !ctx.cr[6].gt {
	pc = 0x8304A940; continue 'dispatch;
	}
	// 8304A8D8: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 8304A8DC: 38C0000A  li r6, 0xa
	ctx.r[6].s64 = 10;
	// 8304A8E0: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 8304A8E4: 38810110  addi r4, r1, 0x110
	ctx.r[4].s64 = ctx.r[1].s64 + 272;
	// 8304A8E8: 4BF86F81  bl 0x82fd1868
	ctx.lr = 0x8304A8EC;
	sub_82FD1868(ctx, base);
	// 8304A8EC: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 8304A8F0: 38C0000A  li r6, 0xa
	ctx.r[6].s64 = 10;
	// 8304A8F4: 807E0060  lwz r3, 0x60(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(96 as u32) ) } as u64;
	// 8304A8F8: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 8304A8FC: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 8304A900: 4BF86F69  bl 0x82fd1868
	ctx.lr = 0x8304A904;
	sub_82FD1868(ctx, base);
	// 8304A904: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8304A908: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8304A90C: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 8304A910: 388B1FB0  addi r4, r11, 0x1fb0
	ctx.r[4].s64 = ctx.r[11].s64 + 8112;
	// 8304A914: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8304A918: 39010080  addi r8, r1, 0x80
	ctx.r[8].s64 = ctx.r[1].s64 + 128;
	// 8304A91C: 38E10110  addi r7, r1, 0x110
	ctx.r[7].s64 = ctx.r[1].s64 + 272;
	// 8304A920: 38C000DC  li r6, 0xdc
	ctx.r[6].s64 = 220;
	// 8304A924: 38A00178  li r5, 0x178
	ctx.r[5].s64 = 376;
	// 8304A928: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8304A92C: 4BFF71ED  bl 0x83041b18
	ctx.lr = 0x8304A930;
	sub_83041B18(ctx, base);
	// 8304A930: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8304A934: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8304A938: 388BC990  addi r4, r11, -0x3670
	ctx.r[4].s64 = ctx.r[11].s64 + -13936;
	// 8304A93C: 481662ED  bl 0x831b0c28
	ctx.lr = 0x8304A940;
	sub_831B0C28(ctx, base);
	// 8304A940: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8304A944: 419A0088  beq cr6, 0x8304a9cc
	if ctx.cr[6].eq {
	pc = 0x8304A9CC; continue 'dispatch;
	}
	// 8304A948: 815E0014  lwz r10, 0x14(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 8304A94C: 554A05AD  rlwinm. r10, r10, 0, 0x16, 0x16
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8304A950: 4182007C  beq 0x8304a9cc
	if ctx.cr[0].eq {
	pc = 0x8304A9CC; continue 'dispatch;
	}
	// 8304A954: 806B0060  lwz r3, 0x60(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(96 as u32) ) } as u64;
	// 8304A958: 815E0060  lwz r10, 0x60(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(96 as u32) ) } as u64;
	// 8304A95C: 7F035040  cmplw cr6, r3, r10
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8304A960: 419A006C  beq cr6, 0x8304a9cc
	if ctx.cr[6].eq {
	pc = 0x8304A9CC; continue 'dispatch;
	}
	// 8304A964: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 8304A968: 38C0000A  li r6, 0xa
	ctx.r[6].s64 = 10;
	// 8304A96C: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 8304A970: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 8304A974: 4BF86EF5  bl 0x82fd1868
	ctx.lr = 0x8304A978;
	sub_82FD1868(ctx, base);
	// 8304A978: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 8304A97C: 38C0000A  li r6, 0xa
	ctx.r[6].s64 = 10;
	// 8304A980: 807E0060  lwz r3, 0x60(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(96 as u32) ) } as u64;
	// 8304A984: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 8304A988: 38810110  addi r4, r1, 0x110
	ctx.r[4].s64 = ctx.r[1].s64 + 272;
	// 8304A98C: 4BF86EDD  bl 0x82fd1868
	ctx.lr = 0x8304A990;
	sub_82FD1868(ctx, base);
	// 8304A990: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8304A994: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8304A998: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 8304A99C: 388B1FB0  addi r4, r11, 0x1fb0
	ctx.r[4].s64 = ctx.r[11].s64 + 8112;
	// 8304A9A0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8304A9A4: 39010110  addi r8, r1, 0x110
	ctx.r[8].s64 = ctx.r[1].s64 + 272;
	// 8304A9A8: 38E10080  addi r7, r1, 0x80
	ctx.r[7].s64 = ctx.r[1].s64 + 128;
	// 8304A9AC: 38C000E3  li r6, 0xe3
	ctx.r[6].s64 = 227;
	// 8304A9B0: 38A00187  li r5, 0x187
	ctx.r[5].s64 = 391;
	// 8304A9B4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8304A9B8: 4BFF7161  bl 0x83041b18
	ctx.lr = 0x8304A9BC;
	sub_83041B18(ctx, base);
	// 8304A9BC: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8304A9C0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8304A9C4: 388BC990  addi r4, r11, -0x3670
	ctx.r[4].s64 = ctx.r[11].s64 + -13936;
	// 8304A9C8: 48166261  bl 0x831b0c28
	ctx.lr = 0x8304A9CC;
	sub_831B0C28(ctx, base);
	// 8304A9CC: 552A056B  rlwinm. r10, r9, 0, 0x15, 0x15
	ctx.r[10].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8304A9D0: 41820190  beq 0x8304ab60
	if ctx.cr[0].eq {
	pc = 0x8304AB60; continue 'dispatch;
	}
	// 8304A9D4: 550A056B  rlwinm. r10, r8, 0, 0x15, 0x15
	ctx.r[10].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8304A9D8: 4182007C  beq 0x8304aa54
	if ctx.cr[0].eq {
	pc = 0x8304AA54; continue 'dispatch;
	}
	// 8304A9DC: 806B0064  lwz r3, 0x64(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(100 as u32) ) } as u64;
	// 8304A9E0: 813E0064  lwz r9, 0x64(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(100 as u32) ) } as u64;
	// 8304A9E4: 7F034840  cmplw cr6, r3, r9
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8304A9E8: 4099006C  ble cr6, 0x8304aa54
	if !ctx.cr[6].gt {
	pc = 0x8304AA54; continue 'dispatch;
	}
	// 8304A9EC: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 8304A9F0: 38C0000A  li r6, 0xa
	ctx.r[6].s64 = 10;
	// 8304A9F4: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 8304A9F8: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 8304A9FC: 4BF86E6D  bl 0x82fd1868
	ctx.lr = 0x8304AA00;
	sub_82FD1868(ctx, base);
	// 8304AA00: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 8304AA04: 38C0000A  li r6, 0xa
	ctx.r[6].s64 = 10;
	// 8304AA08: 807E0064  lwz r3, 0x64(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(100 as u32) ) } as u64;
	// 8304AA0C: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 8304AA10: 38810110  addi r4, r1, 0x110
	ctx.r[4].s64 = ctx.r[1].s64 + 272;
	// 8304AA14: 4BF86E55  bl 0x82fd1868
	ctx.lr = 0x8304AA18;
	sub_82FD1868(ctx, base);
	// 8304AA18: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8304AA1C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8304AA20: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 8304AA24: 388B1FB0  addi r4, r11, 0x1fb0
	ctx.r[4].s64 = ctx.r[11].s64 + 8112;
	// 8304AA28: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8304AA2C: 39010110  addi r8, r1, 0x110
	ctx.r[8].s64 = ctx.r[1].s64 + 272;
	// 8304AA30: 38E10080  addi r7, r1, 0x80
	ctx.r[7].s64 = ctx.r[1].s64 + 128;
	// 8304AA34: 38C000DE  li r6, 0xde
	ctx.r[6].s64 = 222;
	// 8304AA38: 38A00199  li r5, 0x199
	ctx.r[5].s64 = 409;
	// 8304AA3C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8304AA40: 4BFF70D9  bl 0x83041b18
	ctx.lr = 0x8304AA44;
	sub_83041B18(ctx, base);
	// 8304AA44: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8304AA48: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8304AA4C: 388BC990  addi r4, r11, -0x3670
	ctx.r[4].s64 = ctx.r[11].s64 + -13936;
	// 8304AA50: 481661D9  bl 0x831b0c28
	ctx.lr = 0x8304AA54;
	sub_831B0C28(ctx, base);
	// 8304AA54: 550905AD  rlwinm. r9, r8, 0, 0x16, 0x16
	ctx.r[9].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8304AA58: 4182007C  beq 0x8304aad4
	if ctx.cr[0].eq {
	pc = 0x8304AAD4; continue 'dispatch;
	}
	// 8304AA5C: 806B0064  lwz r3, 0x64(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(100 as u32) ) } as u64;
	// 8304AA60: 813E0060  lwz r9, 0x60(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(96 as u32) ) } as u64;
	// 8304AA64: 7F034840  cmplw cr6, r3, r9
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8304AA68: 4099006C  ble cr6, 0x8304aad4
	if !ctx.cr[6].gt {
	pc = 0x8304AAD4; continue 'dispatch;
	}
	// 8304AA6C: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 8304AA70: 38C0000A  li r6, 0xa
	ctx.r[6].s64 = 10;
	// 8304AA74: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 8304AA78: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 8304AA7C: 4BF86DED  bl 0x82fd1868
	ctx.lr = 0x8304AA80;
	sub_82FD1868(ctx, base);
	// 8304AA80: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 8304AA84: 38C0000A  li r6, 0xa
	ctx.r[6].s64 = 10;
	// 8304AA88: 807E0060  lwz r3, 0x60(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(96 as u32) ) } as u64;
	// 8304AA8C: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 8304AA90: 38810110  addi r4, r1, 0x110
	ctx.r[4].s64 = ctx.r[1].s64 + 272;
	// 8304AA94: 4BF86DD5  bl 0x82fd1868
	ctx.lr = 0x8304AA98;
	sub_82FD1868(ctx, base);
	// 8304AA98: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8304AA9C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8304AAA0: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 8304AAA4: 388B1FB0  addi r4, r11, 0x1fb0
	ctx.r[4].s64 = ctx.r[11].s64 + 8112;
	// 8304AAA8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8304AAAC: 39010110  addi r8, r1, 0x110
	ctx.r[8].s64 = ctx.r[1].s64 + 272;
	// 8304AAB0: 38E10080  addi r7, r1, 0x80
	ctx.r[7].s64 = ctx.r[1].s64 + 128;
	// 8304AAB4: 38C000DD  li r6, 0xdd
	ctx.r[6].s64 = 221;
	// 8304AAB8: 38A001A8  li r5, 0x1a8
	ctx.r[5].s64 = 424;
	// 8304AABC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8304AAC0: 4BFF7059  bl 0x83041b18
	ctx.lr = 0x8304AAC4;
	sub_83041B18(ctx, base);
	// 8304AAC4: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8304AAC8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8304AACC: 388BC990  addi r4, r11, -0x3670
	ctx.r[4].s64 = ctx.r[11].s64 + -13936;
	// 8304AAD0: 48166159  bl 0x831b0c28
	ctx.lr = 0x8304AAD4;
	sub_831B0C28(ctx, base);
	// 8304AAD4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8304AAD8: 419A0088  beq cr6, 0x8304ab60
	if ctx.cr[6].eq {
	pc = 0x8304AB60; continue 'dispatch;
	}
	// 8304AADC: 815E0014  lwz r10, 0x14(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 8304AAE0: 554A056B  rlwinm. r10, r10, 0, 0x15, 0x15
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8304AAE4: 4182007C  beq 0x8304ab60
	if ctx.cr[0].eq {
	pc = 0x8304AB60; continue 'dispatch;
	}
	// 8304AAE8: 806B0064  lwz r3, 0x64(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(100 as u32) ) } as u64;
	// 8304AAEC: 817E0064  lwz r11, 0x64(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(100 as u32) ) } as u64;
	// 8304AAF0: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8304AAF4: 419A006C  beq cr6, 0x8304ab60
	if ctx.cr[6].eq {
	pc = 0x8304AB60; continue 'dispatch;
	}
	// 8304AAF8: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 8304AAFC: 38C0000A  li r6, 0xa
	ctx.r[6].s64 = 10;
	// 8304AB00: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 8304AB04: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 8304AB08: 4BF86D61  bl 0x82fd1868
	ctx.lr = 0x8304AB0C;
	sub_82FD1868(ctx, base);
	// 8304AB0C: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 8304AB10: 38C0000A  li r6, 0xa
	ctx.r[6].s64 = 10;
	// 8304AB14: 807E0064  lwz r3, 0x64(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(100 as u32) ) } as u64;
	// 8304AB18: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 8304AB1C: 38810110  addi r4, r1, 0x110
	ctx.r[4].s64 = ctx.r[1].s64 + 272;
	// 8304AB20: 4BF86D49  bl 0x82fd1868
	ctx.lr = 0x8304AB24;
	sub_82FD1868(ctx, base);
	// 8304AB24: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8304AB28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8304AB2C: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 8304AB30: 388B1FB0  addi r4, r11, 0x1fb0
	ctx.r[4].s64 = ctx.r[11].s64 + 8112;
	// 8304AB34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8304AB38: 39010110  addi r8, r1, 0x110
	ctx.r[8].s64 = ctx.r[1].s64 + 272;
	// 8304AB3C: 38E10080  addi r7, r1, 0x80
	ctx.r[7].s64 = ctx.r[1].s64 + 128;
	// 8304AB40: 38C000E4  li r6, 0xe4
	ctx.r[6].s64 = 228;
	// 8304AB44: 38A001B8  li r5, 0x1b8
	ctx.r[5].s64 = 440;
	// 8304AB48: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8304AB4C: 4BFF6FCD  bl 0x83041b18
	ctx.lr = 0x8304AB50;
	sub_83041B18(ctx, base);
	// 8304AB50: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8304AB54: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8304AB58: 388BC990  addi r4, r11, -0x3670
	ctx.r[4].s64 = ctx.r[11].s64 + -13936;
	// 8304AB5C: 481660CD  bl 0x831b0c28
	ctx.lr = 0x8304AB60;
	sub_831B0C28(ctx, base);
	// 8304AB60: 382101B0  addi r1, r1, 0x1b0
	ctx.r[1].s64 = ctx.r[1].s64 + 432;
	// 8304AB64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304AB68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304AB6C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8304AB70: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8304AB74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304AB78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8304AB78 size=16
    let mut pc: u32 = 0x8304AB78;
    'dispatch: loop {
        match pc {
            0x8304AB78 => {
    //   block [0x8304AB78..0x8304AB88)
	// 8304AB78: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8304AB7C: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 8304AB80: 80A3001C  lwz r5, 0x1c(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 8304AB84: 48049874  b 0x830943f8
	sub_830943F8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304AB88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8304AB88 size=8
    let mut pc: u32 = 0x8304AB88;
    'dispatch: loop {
        match pc {
            0x8304AB88 => {
    //   block [0x8304AB88..0x8304AB90)
	// 8304AB88: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304AB8C: 821620E8  lwz r16, 0x20e8(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(8424 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304AB90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304AB90 size=84
    let mut pc: u32 = 0x8304AB90;
    'dispatch: loop {
        match pc {
            0x8304AB90 => {
    //   block [0x8304AB90..0x8304ABE4)
	// 8304AB90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304AB94: 4815D5D9  bl 0x831a816c
	ctx.lr = 0x8304AB98;
	sub_831A8130(ctx, base);
	// 8304AB98: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 8304AB9C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304ABA0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8304ABA4: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 8304ABA8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8304ABAC: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8304ABB0: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 8304ABB4: 4BF8D6E5  bl 0x82fd8298
	ctx.lr = 0x8304ABB8;
	sub_82FD8298(ctx, base);
	// 8304ABB8: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8304ABBC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8304ABC0: 41820014  beq 0x8304abd4
	if ctx.cr[0].eq {
	pc = 0x8304ABD4; continue 'dispatch;
	}
	// 8304ABC4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8304ABC8: 80BE0004  lwz r5, 4(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8304ABCC: 480495B5  bl 0x83094180
	ctx.lr = 0x8304ABD0;
	sub_83094180(ctx, base);
	// 8304ABD0: 48000008  b 0x8304abd8
	pc = 0x8304ABD8; continue 'dispatch;
	// 8304ABD4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8304ABD8: 907E0048  stw r3, 0x48(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(72 as u32), ctx.r[3].u32 ) };
	// 8304ABDC: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 8304ABE0: 4815D5DC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304ABE4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304ABE4 size=48
    let mut pc: u32 = 0x8304ABE4;
    'dispatch: loop {
        match pc {
            0x8304ABE4 => {
    //   block [0x8304ABE4..0x8304AC14)
	// 8304ABE4: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 8304ABE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304ABEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304ABF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304ABF4: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 8304ABF8: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8304ABFC: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8304AC00: 4BF8D6E1  bl 0x82fd82e0
	ctx.lr = 0x8304AC04;
	sub_82FD82E0(ctx, base);
	// 8304AC04: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8304AC08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304AC0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304AC10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304AC18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8304AC18 size=8
    let mut pc: u32 = 0x8304AC18;
    'dispatch: loop {
        match pc {
            0x8304AC18 => {
    //   block [0x8304AC18..0x8304AC20)
	// 8304AC18: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304AC1C: 82162120  lwz r16, 0x2120(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(8480 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304AC20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304AC20 size=84
    let mut pc: u32 = 0x8304AC20;
    'dispatch: loop {
        match pc {
            0x8304AC20 => {
    //   block [0x8304AC20..0x8304AC74)
	// 8304AC20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304AC24: 4815D549  bl 0x831a816c
	ctx.lr = 0x8304AC28;
	sub_831A8130(ctx, base);
	// 8304AC28: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 8304AC2C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304AC30: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8304AC34: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 8304AC38: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8304AC3C: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8304AC40: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 8304AC44: 4BF8D655  bl 0x82fd8298
	ctx.lr = 0x8304AC48;
	sub_82FD8298(ctx, base);
	// 8304AC48: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8304AC4C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8304AC50: 41820014  beq 0x8304ac64
	if ctx.cr[0].eq {
	pc = 0x8304AC64; continue 'dispatch;
	}
	// 8304AC54: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8304AC58: 80BE0004  lwz r5, 4(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8304AC5C: 48049525  bl 0x83094180
	ctx.lr = 0x8304AC60;
	sub_83094180(ctx, base);
	// 8304AC60: 48000008  b 0x8304ac68
	pc = 0x8304AC68; continue 'dispatch;
	// 8304AC64: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8304AC68: 907E004C  stw r3, 0x4c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(76 as u32), ctx.r[3].u32 ) };
	// 8304AC6C: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 8304AC70: 4815D54C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304AC74(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304AC74 size=48
    let mut pc: u32 = 0x8304AC74;
    'dispatch: loop {
        match pc {
            0x8304AC74 => {
    //   block [0x8304AC74..0x8304ACA4)
	// 8304AC74: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 8304AC78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304AC7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304AC80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304AC84: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 8304AC88: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8304AC8C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8304AC90: 4BF8D651  bl 0x82fd82e0
	ctx.lr = 0x8304AC94;
	sub_82FD82E0(ctx, base);
	// 8304AC94: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8304AC98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304AC9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304ACA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304ACA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8304ACA8 size=8
    let mut pc: u32 = 0x8304ACA8;
    'dispatch: loop {
        match pc {
            0x8304ACA8 => {
    //   block [0x8304ACA8..0x8304ACB0)
	// 8304ACA8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304ACAC: 82162158  lwz r16, 0x2158(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(8536 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304ACB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304ACB0 size=84
    let mut pc: u32 = 0x8304ACB0;
    'dispatch: loop {
        match pc {
            0x8304ACB0 => {
    //   block [0x8304ACB0..0x8304AD04)
	// 8304ACB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304ACB4: 4815D4B9  bl 0x831a816c
	ctx.lr = 0x8304ACB8;
	sub_831A8130(ctx, base);
	// 8304ACB8: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 8304ACBC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304ACC0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8304ACC4: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 8304ACC8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8304ACCC: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8304ACD0: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 8304ACD4: 4BF8D5C5  bl 0x82fd8298
	ctx.lr = 0x8304ACD8;
	sub_82FD8298(ctx, base);
	// 8304ACD8: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8304ACDC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8304ACE0: 41820014  beq 0x8304acf4
	if ctx.cr[0].eq {
	pc = 0x8304ACF4; continue 'dispatch;
	}
	// 8304ACE4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8304ACE8: 80BE0004  lwz r5, 4(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8304ACEC: 48049495  bl 0x83094180
	ctx.lr = 0x8304ACF0;
	sub_83094180(ctx, base);
	// 8304ACF0: 48000008  b 0x8304acf8
	pc = 0x8304ACF8; continue 'dispatch;
	// 8304ACF4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8304ACF8: 907E0050  stw r3, 0x50(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8304ACFC: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 8304AD00: 4815D4BC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304AD04(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304AD04 size=48
    let mut pc: u32 = 0x8304AD04;
    'dispatch: loop {
        match pc {
            0x8304AD04 => {
    //   block [0x8304AD04..0x8304AD34)
	// 8304AD04: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 8304AD08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304AD0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304AD10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304AD14: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 8304AD18: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8304AD1C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8304AD20: 4BF8D5C1  bl 0x82fd82e0
	ctx.lr = 0x8304AD24;
	sub_82FD82E0(ctx, base);
	// 8304AD24: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8304AD28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304AD2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304AD30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304AD38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8304AD38 size=8
    let mut pc: u32 = 0x8304AD38;
    'dispatch: loop {
        match pc {
            0x8304AD38 => {
    //   block [0x8304AD38..0x8304AD40)
	// 8304AD38: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304AD3C: 82162190  lwz r16, 0x2190(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(8592 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304AD40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304AD40 size=84
    let mut pc: u32 = 0x8304AD40;
    'dispatch: loop {
        match pc {
            0x8304AD40 => {
    //   block [0x8304AD40..0x8304AD94)
	// 8304AD40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304AD44: 4815D429  bl 0x831a816c
	ctx.lr = 0x8304AD48;
	sub_831A8130(ctx, base);
	// 8304AD48: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 8304AD4C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304AD50: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8304AD54: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 8304AD58: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8304AD5C: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8304AD60: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 8304AD64: 4BF8D535  bl 0x82fd8298
	ctx.lr = 0x8304AD68;
	sub_82FD8298(ctx, base);
	// 8304AD68: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8304AD6C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8304AD70: 41820014  beq 0x8304ad84
	if ctx.cr[0].eq {
	pc = 0x8304AD84; continue 'dispatch;
	}
	// 8304AD74: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8304AD78: 80BE0004  lwz r5, 4(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8304AD7C: 48049405  bl 0x83094180
	ctx.lr = 0x8304AD80;
	sub_83094180(ctx, base);
	// 8304AD80: 48000008  b 0x8304ad88
	pc = 0x8304AD88; continue 'dispatch;
	// 8304AD84: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8304AD88: 907E0054  stw r3, 0x54(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 8304AD8C: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 8304AD90: 4815D42C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304AD94(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304AD94 size=48
    let mut pc: u32 = 0x8304AD94;
    'dispatch: loop {
        match pc {
            0x8304AD94 => {
    //   block [0x8304AD94..0x8304ADC4)
	// 8304AD94: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 8304AD98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304AD9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304ADA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304ADA4: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 8304ADA8: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8304ADAC: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8304ADB0: 4BF8D531  bl 0x82fd82e0
	ctx.lr = 0x8304ADB4;
	sub_82FD82E0(ctx, base);
	// 8304ADB4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8304ADB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304ADBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304ADC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304ADC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8304ADC8 size=8
    let mut pc: u32 = 0x8304ADC8;
    'dispatch: loop {
        match pc {
            0x8304ADC8 => {
    //   block [0x8304ADC8..0x8304ADD0)
	// 8304ADC8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304ADCC: 821621F4  lwz r16, 0x21f4(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(8692 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304ADD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304ADD0 size=200
    let mut pc: u32 = 0x8304ADD0;
    'dispatch: loop {
        match pc {
            0x8304ADD0 => {
    //   block [0x8304ADD0..0x8304AE98)
	// 8304ADD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304ADD4: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 8304ADD8: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 8304ADDC: 4815D38D  bl 0x831a8168
	ctx.lr = 0x8304ADE0;
	sub_831A8130(ctx, base);
	// 8304ADE0: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 8304ADE4: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304ADE8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8304ADEC: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8304ADF0: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 8304ADF4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8304ADF8: 409A0008  bne cr6, 0x8304ae00
	if !ctx.cr[6].eq {
	pc = 0x8304AE00; continue 'dispatch;
	}
	// 8304ADFC: 83BE0004  lwz r29, 4(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8304AE00: 54CB063F  clrlwi. r11, r6, 0x18
	ctx.r[11].u64 = ctx.r[6].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304AE04: 4182002C  beq 0x8304ae30
	if ctx.cr[0].eq {
	pc = 0x8304AE30; continue 'dispatch;
	}
	// 8304AE08: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304AE0C: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 8304AE10: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8304AE14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8304AE18: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8304AE1C: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 8304AE20: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304AE24: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8304AE28: 4E800421  bctrl
	ctx.lr = 0x8304AE2C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8304AE2C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8304AE30: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304AE34: 4BFC5675  bl 0x830104a8
	ctx.lr = 0x8304AE38;
	sub_830104A8(ctx, base);
	// 8304AE38: 2F030005  cmpwi cr6, r3, 5
	ctx.cr[6].compare_i32(ctx.r[3].s32, 5, &mut ctx.xer);
	// 8304AE3C: 419A003C  beq cr6, 0x8304ae78
	if ctx.cr[6].eq {
	pc = 0x8304AE78; continue 'dispatch;
	}
	// 8304AE40: 2F030006  cmpwi cr6, r3, 6
	ctx.cr[6].compare_i32(ctx.r[3].s32, 6, &mut ctx.xer);
	// 8304AE44: 419A0034  beq cr6, 0x8304ae78
	if ctx.cr[6].eq {
	pc = 0x8304AE78; continue 'dispatch;
	}
	// 8304AE48: 2F030007  cmpwi cr6, r3, 7
	ctx.cr[6].compare_i32(ctx.r[3].s32, 7, &mut ctx.xer);
	// 8304AE4C: 419A002C  beq cr6, 0x8304ae78
	if ctx.cr[6].eq {
	pc = 0x8304AE78; continue 'dispatch;
	}
	// 8304AE50: 2F030004  cmpwi cr6, r3, 4
	ctx.cr[6].compare_i32(ctx.r[3].s32, 4, &mut ctx.xer);
	// 8304AE54: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8304AE58: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8304AE5C: 409A0014  bne cr6, 0x8304ae70
	if !ctx.cr[6].eq {
	pc = 0x8304AE70; continue 'dispatch;
	}
	// 8304AE60: 480496E9  bl 0x83094548
	ctx.lr = 0x8304AE64;
	sub_83094548(ctx, base);
	// 8304AE64: 4800002C  b 0x8304ae90
	pc = 0x8304AE90; continue 'dispatch;
	// 8304AE68: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8304AE6C: 48000024  b 0x8304ae90
	pc = 0x8304AE90; continue 'dispatch;
	// 8304AE70: 4BF85D11  bl 0x82fd0b80
	ctx.lr = 0x8304AE74;
	sub_82FD0B80(ctx, base);
	// 8304AE74: 4800001C  b 0x8304ae90
	pc = 0x8304AE90; continue 'dispatch;
	// 8304AE78: 3963FFF9  addi r11, r3, -7
	ctx.r[11].s64 = ctx.r[3].s64 + -7;
	// 8304AE7C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8304AE80: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8304AE84: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8304AE88: 5565DFFE  rlwinm r5, r11, 0x1b, 0x1f, 0x1f
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8304AE8C: 48049DBD  bl 0x83094c48
	ctx.lr = 0x8304AE90;
	sub_83094C48(ctx, base);
	// 8304AE90: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 8304AE94: 4815D324  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304AE98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8304AE98 size=8
    let mut pc: u32 = 0x8304AE98;
    'dispatch: loop {
        match pc {
            0x8304AE98 => {
    //   block [0x8304AE98..0x8304AEA0)
	// 8304AE98: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304AE9C: 821621F4  lwz r16, 0x21f4(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(8692 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304AEA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304AEA0 size=20
    let mut pc: u32 = 0x8304AEA0;
    'dispatch: loop {
        match pc {
            0x8304AEA0 => {
    //   block [0x8304AEA0..0x8304AEB4)
	// 8304AEA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304AEA4: 3C608305  lis r3, -0x7cfb
	ctx.r[3].s64 = -2096824320;
	// 8304AEA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8304AEAC: 3863AE68  addi r3, r3, -0x5198
	ctx.r[3].s64 = ctx.r[3].s64 + -20888;
	// 8304AEB0: 4815D308  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304AEB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8304AEB8 size=8
    let mut pc: u32 = 0x8304AEB8;
    'dispatch: loop {
        match pc {
            0x8304AEB8 => {
    //   block [0x8304AEB8..0x8304AEC0)
	// 8304AEB8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304AEBC: 82162258  lwz r16, 0x2258(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(8792 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304AEC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304AEC0 size=96
    let mut pc: u32 = 0x8304AEC0;
    'dispatch: loop {
        match pc {
            0x8304AEC0 => {
    //   block [0x8304AEC0..0x8304AF20)
	// 8304AEC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304AEC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304AEC8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8304AECC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8304AED0: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 8304AED4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304AED8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8304AEDC: 38600068  li r3, 0x68
	ctx.r[3].s64 = 104;
	// 8304AEE0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8304AEE4: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 8304AEE8: 4BF8D3B1  bl 0x82fd8298
	ctx.lr = 0x8304AEEC;
	sub_82FD8298(ctx, base);
	// 8304AEEC: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8304AEF0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8304AEF4: 41820010  beq 0x8304af04
	if ctx.cr[0].eq {
	pc = 0x8304AF04; continue 'dispatch;
	}
	// 8304AEF8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8304AEFC: 4BFFF3D5  bl 0x8304a2d0
	ctx.lr = 0x8304AF00;
	sub_8304A2D0(ctx, base);
	// 8304AF00: 48000008  b 0x8304af08
	pc = 0x8304AF08; continue 'dispatch;
	// 8304AF04: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8304AF08: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 8304AF0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304AF10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304AF14: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8304AF18: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8304AF1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304AF20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304AF20 size=44
    let mut pc: u32 = 0x8304AF20;
    'dispatch: loop {
        match pc {
            0x8304AF20 => {
    //   block [0x8304AF20..0x8304AF4C)
	// 8304AF20: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 8304AF24: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304AF28: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304AF2C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304AF30: 809F0084  lwz r4, 0x84(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 8304AF34: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8304AF38: 4BF8D3A9  bl 0x82fd82e0
	ctx.lr = 0x8304AF3C;
	sub_82FD82E0(ctx, base);
	// 8304AF3C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8304AF40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304AF44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304AF48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304AF50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8304AF50 size=12
    let mut pc: u32 = 0x8304AF50;
    'dispatch: loop {
        match pc {
            0x8304AF50 => {
    //   block [0x8304AF50..0x8304AF5C)
	// 8304AF50: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8304AF54: 386B33DC  addi r3, r11, 0x33dc
	ctx.r[3].s64 = ctx.r[11].s64 + 13276;
	// 8304AF58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304AF60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304AF60 size=156
    let mut pc: u32 = 0x8304AF60;
    'dispatch: loop {
        match pc {
            0x8304AF60 => {
    //   block [0x8304AF60..0x8304AFFC)
	// 8304AF60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304AF64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304AF68: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8304AF6C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8304AF70: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304AF74: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8304AF78: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8304AF7C: A97F0000  lha r11, 0(r31)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as i16) as i64;
	// 8304AF80: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8304AF84: 556BDFFF  rlwinm. r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304AF88: 41820010  beq 0x8304af98
	if ctx.cr[0].eq {
	pc = 0x8304AF98; continue 'dispatch;
	}
	// 8304AF8C: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 8304AF90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8304AF94: 4BFAE365  bl 0x82ff92f8
	ctx.lr = 0x8304AF98;
	sub_82FF92F8(ctx, base);
	// 8304AF98: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8304AF9C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304AFA0: 48047B91  bl 0x83092b30
	ctx.lr = 0x8304AFA4;
	sub_83092B30(ctx, base);
	// 8304AFA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8304AFA8: A97F0000  lha r11, 0(r31)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as i16) as i64;
	// 8304AFAC: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8304AFB0: 556BDFFF  rlwinm. r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304AFB4: 4182001C  beq 0x8304afd0
	if ctx.cr[0].eq {
	pc = 0x8304AFD0; continue 'dispatch;
	}
	// 8304AFB8: 809E0060  lwz r4, 0x60(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(96 as u32) ) } as u64;
	// 8304AFBC: 4BFAE33D  bl 0x82ff92f8
	ctx.lr = 0x8304AFC0;
	sub_82FF92F8(ctx, base);
	// 8304AFC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8304AFC4: 809E0064  lwz r4, 0x64(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(100 as u32) ) } as u64;
	// 8304AFC8: 4BFAE331  bl 0x82ff92f8
	ctx.lr = 0x8304AFCC;
	sub_82FF92F8(ctx, base);
	// 8304AFCC: 48000018  b 0x8304afe4
	pc = 0x8304AFE4; continue 'dispatch;
	// 8304AFD0: 389E0060  addi r4, r30, 0x60
	ctx.r[4].s64 = ctx.r[30].s64 + 96;
	// 8304AFD4: 4BFAE5A5  bl 0x82ff9578
	ctx.lr = 0x8304AFD8;
	sub_82FF9578(ctx, base);
	// 8304AFD8: 389E0064  addi r4, r30, 0x64
	ctx.r[4].s64 = ctx.r[30].s64 + 100;
	// 8304AFDC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8304AFE0: 4BFAE599  bl 0x82ff9578
	ctx.lr = 0x8304AFE4;
	sub_82FF9578(ctx, base);
	// 8304AFE4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8304AFE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304AFEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304AFF0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8304AFF4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8304AFF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304B000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304B000 size=88
    let mut pc: u32 = 0x8304B000;
    'dispatch: loop {
        match pc {
            0x8304B000 => {
    //   block [0x8304B000..0x8304B058)
	// 8304B000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304B004: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304B008: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8304B00C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8304B010: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304B014: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8304B018: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8304B01C: 396B1E88  addi r11, r11, 0x1e88
	ctx.r[11].s64 = ctx.r[11].s64 + 7816;
	// 8304B020: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8304B024: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8304B028: 48047671  bl 0x83092698
	ctx.lr = 0x8304B02C;
	sub_83092698(ctx, base);
	// 8304B02C: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304B030: 4182000C  beq 0x8304b03c
	if ctx.cr[0].eq {
	pc = 0x8304B03C; continue 'dispatch;
	}
	// 8304B034: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8304B038: 4BF8D2A9  bl 0x82fd82e0
	ctx.lr = 0x8304B03C;
	sub_82FD82E0(ctx, base);
	// 8304B03C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8304B040: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8304B044: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304B048: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304B04C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8304B050: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8304B054: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304B058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8304B058 size=8
    let mut pc: u32 = 0x8304B058;
    'dispatch: loop {
        match pc {
            0x8304B058 => {
    //   block [0x8304B058..0x8304B060)
	// 8304B058: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304B05C: 82162308  lwz r16, 0x2308(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(8968 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304B060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304B060 size=1076
    let mut pc: u32 = 0x8304B060;
    'dispatch: loop {
        match pc {
            0x8304B060 => {
    //   block [0x8304B060..0x8304B494)
	// 8304B060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304B064: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 8304B068: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 8304B06C: 4815D0E9  bl 0x831a8154
	ctx.lr = 0x8304B070;
	sub_831A8130(ctx, base);
	// 8304B070: 3BE1FD20  addi r31, r1, -0x2e0
	ctx.r[31].s64 = ctx.r[1].s64 + -736;
	// 8304B074: 9421FD20  stwu r1, -0x2e0(r1)
	ea = ctx.r[1].u32.wrapping_add(-736 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304B078: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8304B07C: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 8304B080: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 8304B084: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 8304B088: 807E001C  lwz r3, 0x1c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 8304B08C: 93DF02F4  stw r30, 0x2f4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(756 as u32), ctx.r[30].u32 ) };
	// 8304B090: 937F0314  stw r27, 0x314(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(788 as u32), ctx.r[27].u32 ) };
	// 8304B094: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8304B098: 41820018  beq 0x8304b0b0
	if ctx.cr[0].eq {
	pc = 0x8304B0B0; continue 'dispatch;
	}
	// 8304B09C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304B0A0: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 8304B0A4: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 8304B0A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8304B0AC: 4E800421  bctrl
	ctx.lr = 0x8304B0B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8304B0B0: 82FE0010  lwz r23, 0x10(r30)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 8304B0B4: 56EB0739  rlwinm. r11, r23, 0, 0x1c, 0x1c
	ctx.r[11].u64 = ctx.r[23].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304B0B8: 418200A4  beq 0x8304b15c
	if ctx.cr[0].eq {
	pc = 0x8304B15C; continue 'dispatch;
	}
	// 8304B0BC: 817E0028  lwz r11, 0x28(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 8304B0C0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8304B0C4: 40820044  bne 0x8304b108
	if !ctx.cr[0].eq {
	pc = 0x8304B108; continue 'dispatch;
	}
	// 8304B0C8: 38600040  li r3, 0x40
	ctx.r[3].s64 = 64;
	// 8304B0CC: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8304B0D0: 4BF8D1C9  bl 0x82fd8298
	ctx.lr = 0x8304B0D4;
	sub_82FD8298(ctx, base);
	// 8304B0D4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8304B0D8: 907F0060  stw r3, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 8304B0DC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8304B0E0: 41820020  beq 0x8304b100
	if ctx.cr[0].eq {
	pc = 0x8304B100; continue 'dispatch;
	}
	// 8304B0E4: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8304B0E8: 809E0024  lwz r4, 0x24(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 8304B0EC: 80DE0004  lwz r6, 4(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8304B0F0: 38ABD918  addi r5, r11, -0x26e8
	ctx.r[5].s64 = ctx.r[11].s64 + -9960;
	// 8304B0F4: 4803EFF5  bl 0x8308a0e8
	ctx.lr = 0x8304B0F8;
	sub_8308A0E8(ctx, base);
	// 8304B0F8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8304B0FC: 48000008  b 0x8304b104
	pc = 0x8304B104; continue 'dispatch;
	// 8304B100: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8304B104: 907E0028  stw r3, 0x28(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), ctx.r[3].u32 ) };
	// 8304B108: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 8304B10C: 807E0028  lwz r3, 0x28(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 8304B110: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 8304B114: 4803F5FD  bl 0x8308a710
	ctx.lr = 0x8304B118;
	sub_8308A710(ctx, base);
	// 8304B118: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304B11C: 40820040  bne 0x8304b15c
	if !ctx.cr[0].eq {
	pc = 0x8304B15C; continue 'dispatch;
	}
	// 8304B120: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8304B124: 811E0024  lwz r8, 0x24(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 8304B128: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8304B12C: 93610054  stw r27, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[27].u32 ) };
	// 8304B130: 388B1FB0  addi r4, r11, 0x1fb0
	ctx.r[4].s64 = ctx.r[11].s64 + 8112;
	// 8304B134: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8304B138: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 8304B13C: 38C000EE  li r6, 0xee
	ctx.r[6].s64 = 238;
	// 8304B140: 38A0022F  li r5, 0x22f
	ctx.r[5].s64 = 559;
	// 8304B144: 387F0110  addi r3, r31, 0x110
	ctx.r[3].s64 = ctx.r[31].s64 + 272;
	// 8304B148: 4BFCAA41  bl 0x83015b88
	ctx.lr = 0x8304B14C;
	sub_83015B88(ctx, base);
	// 8304B14C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8304B150: 387F0110  addi r3, r31, 0x110
	ctx.r[3].s64 = ctx.r[31].s64 + 272;
	// 8304B154: 388BC8B0  addi r4, r11, -0x3750
	ctx.r[4].s64 = ctx.r[11].s64 + -14160;
	// 8304B158: 48165AD1  bl 0x831b0c28
	ctx.lr = 0x8304B15C;
	sub_831B0C28(ctx, base);
	// 8304B15C: 57AB063F  clrlwi. r11, r29, 0x18
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304B160: 4082032C  bne 0x8304b48c
	if !ctx.cr[0].eq {
	pc = 0x8304B48C; continue 'dispatch;
	}
	// 8304B164: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 8304B168: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 8304B16C: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 8304B170: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 8304B174: 4804900D  bl 0x83094180
	ctx.lr = 0x8304B178;
	sub_83094180(ctx, base);
	// 8304B178: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8304B17C: 817E0058  lwz r11, 0x58(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(88 as u32) ) } as u64;
	// 8304B180: 3B9F0070  addi r28, r31, 0x70
	ctx.r[28].s64 = ctx.r[31].s64 + 112;
	// 8304B184: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8304B188: 418200A0  beq 0x8304b228
	if ctx.cr[0].eq {
	pc = 0x8304B228; continue 'dispatch;
	}
	// 8304B18C: 834B0008  lwz r26, 8(r11)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8304B190: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8304B194: 7F1DD000  cmpw cr6, r29, r26
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[26].s32, &mut ctx.xer);
	// 8304B198: 40980044  bge cr6, 0x8304b1dc
	if !ctx.cr[6].lt {
	pc = 0x8304B1DC; continue 'dispatch;
	}
	// 8304B19C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8304B1A0: 807E0058  lwz r3, 0x58(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(88 as u32) ) } as u64;
	// 8304B1A4: 4BFE16CD  bl 0x8302c870
	ctx.lr = 0x8304B1A8;
	sub_8302C870(ctx, base);
	// 8304B1A8: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8304B1AC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8304B1B0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304B1B4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8304B1B8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304B1BC: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 8304B1C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8304B1C4: 4E800421  bctrl
	ctx.lr = 0x8304B1C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8304B1C8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8304B1CC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8304B1D0: 419A000C  beq cr6, 0x8304b1dc
	if ctx.cr[6].eq {
	pc = 0x8304B1DC; continue 'dispatch;
	}
	// 8304B1D4: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 8304B1D8: 4BFFFFBC  b 0x8304b194
	pc = 0x8304B194; continue 'dispatch;
	// 8304B1DC: 7F1DD000  cmpw cr6, r29, r26
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[26].s32, &mut ctx.xer);
	// 8304B1E0: 409A0048  bne cr6, 0x8304b228
	if !ctx.cr[6].eq {
	pc = 0x8304B228; continue 'dispatch;
	}
	// 8304B1E4: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8304B1E8: 93610054  stw r27, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[27].u32 ) };
	// 8304B1EC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8304B1F0: 388B1FB0  addi r4, r11, 0x1fb0
	ctx.r[4].s64 = ctx.r[11].s64 + 8112;
	// 8304B1F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8304B1F8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8304B1FC: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 8304B200: 38C000F4  li r6, 0xf4
	ctx.r[6].s64 = 244;
	// 8304B204: 38A00247  li r5, 0x247
	ctx.r[5].s64 = 583;
	// 8304B208: 387F0130  addi r3, r31, 0x130
	ctx.r[3].s64 = ctx.r[31].s64 + 304;
	// 8304B20C: 4BFCA97D  bl 0x83015b88
	ctx.lr = 0x8304B210;
	sub_83015B88(ctx, base);
	// 8304B210: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8304B214: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8304B218: 387F0130  addi r3, r31, 0x130
	ctx.r[3].s64 = ctx.r[31].s64 + 304;
	// 8304B21C: 388BC8B0  addi r4, r11, -0x3750
	ctx.r[4].s64 = ctx.r[11].s64 + -14160;
	// 8304B220: 48165A09  bl 0x831b0c28
	ctx.lr = 0x8304B224;
	sub_831B0C28(ctx, base);
	// 8304B224: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8304B228: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 8304B22C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8304B230: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304B234: 4804752D  bl 0x83092760
	ctx.lr = 0x8304B238;
	sub_83092760(ctx, base);
	// 8304B238: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8304B23C: 56EB056B  rlwinm. r11, r23, 0, 0x15, 0x15
	ctx.r[11].u64 = ctx.r[23].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304B240: 418200A4  beq 0x8304b2e4
	if ctx.cr[0].eq {
	pc = 0x8304B2E4; continue 'dispatch;
	}
	// 8304B244: 807C000C  lwz r3, 0xc(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(12 as u32) ) } as u64;
	// 8304B248: 817E0064  lwz r11, 0x64(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(100 as u32) ) } as u64;
	// 8304B24C: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8304B250: 40990094  ble cr6, 0x8304b2e4
	if !ctx.cr[6].gt {
	pc = 0x8304B2E4; continue 'dispatch;
	}
	// 8304B254: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 8304B258: 38C0000A  li r6, 0xa
	ctx.r[6].s64 = 10;
	// 8304B25C: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 8304B260: 389F0200  addi r4, r31, 0x200
	ctx.r[4].s64 = ctx.r[31].s64 + 512;
	// 8304B264: 4BF86605  bl 0x82fd1868
	ctx.lr = 0x8304B268;
	sub_82FD1868(ctx, base);
	// 8304B268: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8304B26C: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 8304B270: 807E0064  lwz r3, 0x64(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(100 as u32) ) } as u64;
	// 8304B274: 38C0000A  li r6, 0xa
	ctx.r[6].s64 = 10;
	// 8304B278: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 8304B27C: 389F0170  addi r4, r31, 0x170
	ctx.r[4].s64 = ctx.r[31].s64 + 368;
	// 8304B280: 4BF865E9  bl 0x82fd1868
	ctx.lr = 0x8304B284;
	sub_82FD1868(ctx, base);
	// 8304B284: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8304B288: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304B28C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8304B290: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8304B294: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8304B298: 4E800421  bctrl
	ctx.lr = 0x8304B29C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8304B29C: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 8304B2A0: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8304B2A4: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8304B2A8: 93610054  stw r27, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[27].u32 ) };
	// 8304B2AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8304B2B0: 388B1FB0  addi r4, r11, 0x1fb0
	ctx.r[4].s64 = ctx.r[11].s64 + 8112;
	// 8304B2B4: 393F0170  addi r9, r31, 0x170
	ctx.r[9].s64 = ctx.r[31].s64 + 368;
	// 8304B2B8: 391F0200  addi r8, r31, 0x200
	ctx.r[8].s64 = ctx.r[31].s64 + 512;
	// 8304B2BC: 38C000F6  li r6, 0xf6
	ctx.r[6].s64 = 246;
	// 8304B2C0: 38A00259  li r5, 0x259
	ctx.r[5].s64 = 601;
	// 8304B2C4: 387F0090  addi r3, r31, 0x90
	ctx.r[3].s64 = ctx.r[31].s64 + 144;
	// 8304B2C8: 4BFF6851  bl 0x83041b18
	ctx.lr = 0x8304B2CC;
	sub_83041B18(ctx, base);
	// 8304B2CC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8304B2D0: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8304B2D4: 387F0090  addi r3, r31, 0x90
	ctx.r[3].s64 = ctx.r[31].s64 + 144;
	// 8304B2D8: 388BC990  addi r4, r11, -0x3670
	ctx.r[4].s64 = ctx.r[11].s64 + -13936;
	// 8304B2DC: 4816594D  bl 0x831b0c28
	ctx.lr = 0x8304B2E0;
	sub_831B0C28(ctx, base);
	// 8304B2E0: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8304B2E4: 56EB05AD  rlwinm. r11, r23, 0, 0x16, 0x16
	ctx.r[11].u64 = ctx.r[23].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304B2E8: 41820140  beq 0x8304b428
	if ctx.cr[0].eq {
	pc = 0x8304B428; continue 'dispatch;
	}
	// 8304B2EC: 807C0008  lwz r3, 8(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 8304B2F0: 817E0060  lwz r11, 0x60(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(96 as u32) ) } as u64;
	// 8304B2F4: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8304B2F8: 40990094  ble cr6, 0x8304b38c
	if !ctx.cr[6].gt {
	pc = 0x8304B38C; continue 'dispatch;
	}
	// 8304B2FC: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 8304B300: 38C0000A  li r6, 0xa
	ctx.r[6].s64 = 10;
	// 8304B304: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 8304B308: 389F0170  addi r4, r31, 0x170
	ctx.r[4].s64 = ctx.r[31].s64 + 368;
	// 8304B30C: 4BF8655D  bl 0x82fd1868
	ctx.lr = 0x8304B310;
	sub_82FD1868(ctx, base);
	// 8304B310: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8304B314: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 8304B318: 807E0060  lwz r3, 0x60(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(96 as u32) ) } as u64;
	// 8304B31C: 38C0000A  li r6, 0xa
	ctx.r[6].s64 = 10;
	// 8304B320: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 8304B324: 389F0200  addi r4, r31, 0x200
	ctx.r[4].s64 = ctx.r[31].s64 + 512;
	// 8304B328: 4BF86541  bl 0x82fd1868
	ctx.lr = 0x8304B32C;
	sub_82FD1868(ctx, base);
	// 8304B32C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8304B330: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304B334: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8304B338: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8304B33C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8304B340: 4E800421  bctrl
	ctx.lr = 0x8304B344;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8304B344: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 8304B348: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8304B34C: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8304B350: 93610054  stw r27, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[27].u32 ) };
	// 8304B354: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8304B358: 388B1FB0  addi r4, r11, 0x1fb0
	ctx.r[4].s64 = ctx.r[11].s64 + 8112;
	// 8304B35C: 393F0200  addi r9, r31, 0x200
	ctx.r[9].s64 = ctx.r[31].s64 + 512;
	// 8304B360: 391F0170  addi r8, r31, 0x170
	ctx.r[8].s64 = ctx.r[31].s64 + 368;
	// 8304B364: 38C000F5  li r6, 0xf5
	ctx.r[6].s64 = 245;
	// 8304B368: 38A0026A  li r5, 0x26a
	ctx.r[5].s64 = 618;
	// 8304B36C: 387F0150  addi r3, r31, 0x150
	ctx.r[3].s64 = ctx.r[31].s64 + 336;
	// 8304B370: 4BFF67A9  bl 0x83041b18
	ctx.lr = 0x8304B374;
	sub_83041B18(ctx, base);
	// 8304B374: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8304B378: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8304B37C: 387F0150  addi r3, r31, 0x150
	ctx.r[3].s64 = ctx.r[31].s64 + 336;
	// 8304B380: 388BC990  addi r4, r11, -0x3670
	ctx.r[4].s64 = ctx.r[11].s64 + -13936;
	// 8304B384: 481658A5  bl 0x831b0c28
	ctx.lr = 0x8304B388;
	sub_831B0C28(ctx, base);
	// 8304B388: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8304B38C: 807C000C  lwz r3, 0xc(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(12 as u32) ) } as u64;
	// 8304B390: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8304B394: 40990094  ble cr6, 0x8304b428
	if !ctx.cr[6].gt {
	pc = 0x8304B428; continue 'dispatch;
	}
	// 8304B398: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 8304B39C: 38C0000A  li r6, 0xa
	ctx.r[6].s64 = 10;
	// 8304B3A0: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 8304B3A4: 389F0170  addi r4, r31, 0x170
	ctx.r[4].s64 = ctx.r[31].s64 + 368;
	// 8304B3A8: 4BF864C1  bl 0x82fd1868
	ctx.lr = 0x8304B3AC;
	sub_82FD1868(ctx, base);
	// 8304B3AC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8304B3B0: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 8304B3B4: 807E0060  lwz r3, 0x60(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(96 as u32) ) } as u64;
	// 8304B3B8: 38C0000A  li r6, 0xa
	ctx.r[6].s64 = 10;
	// 8304B3BC: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 8304B3C0: 389F0200  addi r4, r31, 0x200
	ctx.r[4].s64 = ctx.r[31].s64 + 512;
	// 8304B3C4: 4BF864A5  bl 0x82fd1868
	ctx.lr = 0x8304B3C8;
	sub_82FD1868(ctx, base);
	// 8304B3C8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8304B3CC: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304B3D0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8304B3D4: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8304B3D8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8304B3DC: 4E800421  bctrl
	ctx.lr = 0x8304B3E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8304B3E0: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 8304B3E4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8304B3E8: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8304B3EC: 93610054  stw r27, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[27].u32 ) };
	// 8304B3F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8304B3F4: 388B1FB0  addi r4, r11, 0x1fb0
	ctx.r[4].s64 = ctx.r[11].s64 + 8112;
	// 8304B3F8: 393F0200  addi r9, r31, 0x200
	ctx.r[9].s64 = ctx.r[31].s64 + 512;
	// 8304B3FC: 391F0170  addi r8, r31, 0x170
	ctx.r[8].s64 = ctx.r[31].s64 + 368;
	// 8304B400: 38C000F5  li r6, 0xf5
	ctx.r[6].s64 = 245;
	// 8304B404: 38A0027F  li r5, 0x27f
	ctx.r[5].s64 = 639;
	// 8304B408: 387F00B0  addi r3, r31, 0xb0
	ctx.r[3].s64 = ctx.r[31].s64 + 176;
	// 8304B40C: 4BFF670D  bl 0x83041b18
	ctx.lr = 0x8304B410;
	sub_83041B18(ctx, base);
	// 8304B410: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8304B414: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8304B418: 387F00B0  addi r3, r31, 0xb0
	ctx.r[3].s64 = ctx.r[31].s64 + 176;
	// 8304B41C: 388BC990  addi r4, r11, -0x3670
	ctx.r[4].s64 = ctx.r[11].s64 + -13936;
	// 8304B420: 48165809  bl 0x831b0c28
	ctx.lr = 0x8304B424;
	sub_831B0C28(ctx, base);
	// 8304B424: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8304B428: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 8304B42C: 48048F35  bl 0x83094360
	ctx.lr = 0x8304B430;
	sub_83094360(ctx, base);
	// 8304B430: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8304B434: 4800000C  b 0x8304b440
	pc = 0x8304B440; continue 'dispatch;
	// 8304B438: 837F0314  lwz r27, 0x314(r31)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(788 as u32) ) } as u64;
	// 8304B43C: 831F0060  lwz r24, 0x60(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 8304B440: 2B180000  cmplwi cr6, r24, 0
	ctx.cr[6].compare_u32(ctx.r[24].u32, 0 as u32, &mut ctx.xer);
	// 8304B444: 419A0048  beq cr6, 0x8304b48c
	if ctx.cr[6].eq {
	pc = 0x8304B48C; continue 'dispatch;
	}
	// 8304B448: 931F0060  stw r24, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[24].u32 ) };
	// 8304B44C: 937F0064  stw r27, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[27].u32 ) };
	// 8304B450: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8304B454: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8304B458: 93610054  stw r27, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[27].u32 ) };
	// 8304B45C: 388B1FB0  addi r4, r11, 0x1fb0
	ctx.r[4].s64 = ctx.r[11].s64 + 8112;
	// 8304B460: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8304B464: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8304B468: 7F07C378  mr r7, r24
	ctx.r[7].u64 = ctx.r[24].u64;
	// 8304B46C: 38C0012B  li r6, 0x12b
	ctx.r[6].s64 = 299;
	// 8304B470: 38A0028A  li r5, 0x28a
	ctx.r[5].s64 = 650;
	// 8304B474: 387F00D0  addi r3, r31, 0xd0
	ctx.r[3].s64 = ctx.r[31].s64 + 208;
	// 8304B478: 4BFF66A1  bl 0x83041b18
	ctx.lr = 0x8304B47C;
	sub_83041B18(ctx, base);
	// 8304B47C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8304B480: 387F00D0  addi r3, r31, 0xd0
	ctx.r[3].s64 = ctx.r[31].s64 + 208;
	// 8304B484: 388BC990  addi r4, r11, -0x3670
	ctx.r[4].s64 = ctx.r[11].s64 + -13936;
	// 8304B488: 481657A1  bl 0x831b0c28
	ctx.lr = 0x8304B48C;
	sub_831B0C28(ctx, base);
	// 8304B48C: 383F02E0  addi r1, r31, 0x2e0
	ctx.r[1].s64 = ctx.r[31].s64 + 736;
	// 8304B490: 4815CD14  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304B494(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8304B494 size=8
    let mut pc: u32 = 0x8304B494;
    'dispatch: loop {
        match pc {
            0x8304B494 => {
    //   block [0x8304B494..0x8304B49C)
	// 8304B494: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304B498: 82162308  lwz r16, 0x2308(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(8968 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304B49C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304B49C size=84
    let mut pc: u32 = 0x8304B49C;
    'dispatch: loop {
        match pc {
            0x8304B49C => {
    //   block [0x8304B49C..0x8304B4F0)
	// 8304B49C: 3BECFD20  addi r31, r12, -0x2e0
	ctx.r[31].s64 = ctx.r[12].s64 + -736;
	// 8304B4A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304B4A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304B4A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304B4AC: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8304B4B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8304B4B4: 388B1FB0  addi r4, r11, 0x1fb0
	ctx.r[4].s64 = ctx.r[11].s64 + 8112;
	// 8304B4B8: 817F0068  lwz r11, 0x68(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 8304B4BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8304B4C0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8304B4C4: 38C0012B  li r6, 0x12b
	ctx.r[6].s64 = 299;
	// 8304B4C8: 38A00225  li r5, 0x225
	ctx.r[5].s64 = 549;
	// 8304B4CC: 80EB0010  lwz r7, 0x10(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8304B4D0: 387F00F0  addi r3, r31, 0xf0
	ctx.r[3].s64 = ctx.r[31].s64 + 240;
	// 8304B4D4: 817F0314  lwz r11, 0x314(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(788 as u32) ) } as u64;
	// 8304B4D8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8304B4DC: 4BFCA6AD  bl 0x83015b88
	ctx.lr = 0x8304B4E0;
	sub_83015B88(ctx, base);
	// 8304B4E0: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8304B4E4: 387F00F0  addi r3, r31, 0xf0
	ctx.r[3].s64 = ctx.r[31].s64 + 240;
	// 8304B4E8: 388BC8B0  addi r4, r11, -0x3750
	ctx.r[4].s64 = ctx.r[11].s64 + -14160;
	// 8304B4EC: 4816573D  bl 0x831b0c28
	ctx.lr = 0x8304B4F0;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304B4F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304B4F8 size=60
    let mut pc: u32 = 0x8304B4F8;
    'dispatch: loop {
        match pc {
            0x8304B4F8 => {
    //   block [0x8304B4F8..0x8304B534)
	// 8304B4F8: 3BECFD20  addi r31, r12, -0x2e0
	ctx.r[31].s64 = ctx.r[12].s64 + -736;
	// 8304B4FC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304B500: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304B504: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304B508: 817F006C  lwz r11, 0x6c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 8304B50C: 809F0314  lwz r4, 0x314(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(788 as u32) ) } as u64;
	// 8304B510: 806B0010  lwz r3, 0x10(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8304B514: 4BF8566D  bl 0x82fd0b80
	ctx.lr = 0x8304B518;
	sub_82FD0B80(ctx, base);
	// 8304B518: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8304B51C: 907F0060  stw r3, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 8304B520: 3C608305  lis r3, -0x7cfb
	ctx.r[3].s64 = -2096824320;
	// 8304B524: 3863B438  addi r3, r3, -0x4bc8
	ctx.r[3].s64 = ctx.r[3].s64 + -19400;
	// 8304B528: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304B52C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304B530: 4815CC74  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304B534(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304B534 size=48
    let mut pc: u32 = 0x8304B534;
    'dispatch: loop {
        match pc {
            0x8304B534 => {
    //   block [0x8304B534..0x8304B564)
	// 8304B534: 3BECFD20  addi r31, r12, -0x2e0
	ctx.r[31].s64 = ctx.r[12].s64 + -736;
	// 8304B538: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304B53C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304B540: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304B544: 817F02F4  lwz r11, 0x2f4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(756 as u32) ) } as u64;
	// 8304B548: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8304B54C: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 8304B550: 4BF8CD91  bl 0x82fd82e0
	ctx.lr = 0x8304B554;
	sub_82FD82E0(ctx, base);
	// 8304B554: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8304B558: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304B55C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304B560: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304B564(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304B564 size=40
    let mut pc: u32 = 0x8304B564;
    'dispatch: loop {
        match pc {
            0x8304B564 => {
    //   block [0x8304B564..0x8304B58C)
	// 8304B564: 3BECFD20  addi r31, r12, -0x2e0
	ctx.r[31].s64 = ctx.r[12].s64 + -736;
	// 8304B568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304B56C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304B570: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304B574: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 8304B578: 48048DE9  bl 0x83094360
	ctx.lr = 0x8304B57C;
	sub_83094360(ctx, base);
	// 8304B57C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8304B580: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304B584: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304B588: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304B58C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304B58C size=40
    let mut pc: u32 = 0x8304B58C;
    'dispatch: loop {
        match pc {
            0x8304B58C => {
    //   block [0x8304B58C..0x8304B5B4)
	// 8304B58C: 3BECFD20  addi r31, r12, -0x2e0
	ctx.r[31].s64 = ctx.r[12].s64 + -736;
	// 8304B590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304B594: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304B598: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304B59C: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 8304B5A0: 4BF878B9  bl 0x82fd2e58
	ctx.lr = 0x8304B5A4;
	sub_82FD2E58(ctx, base);
	// 8304B5A4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8304B5A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304B5AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304B5B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304B5B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8304B5B8 size=8
    let mut pc: u32 = 0x8304B5B8;
    'dispatch: loop {
        match pc {
            0x8304B5B8 => {
    //   block [0x8304B5B8..0x8304B5C0)
	// 8304B5B8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304B5BC: 82162544  lwz r16, 0x2544(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(9540 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304B5C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304B5C0 size=412
    let mut pc: u32 = 0x8304B5C0;
    'dispatch: loop {
        match pc {
            0x8304B5C0 => {
    //   block [0x8304B5C0..0x8304B75C)
	// 8304B5C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304B5C4: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 8304B5C8: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 8304B5CC: 4815CB95  bl 0x831a8160
	ctx.lr = 0x8304B5D0;
	sub_831A8130(ctx, base);
	// 8304B5D0: 3BE1FF40  addi r31, r1, -0xc0
	ctx.r[31].s64 = ctx.r[1].s64 + -192;
	// 8304B5D4: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304B5D8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8304B5DC: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 8304B5E0: 817D005C  lwz r11, 0x5c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(92 as u32) ) } as u64;
	// 8304B5E4: 93BF00D4  stw r29, 0xd4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(212 as u32), ctx.r[29].u32 ) };
	// 8304B5E8: 935F00DC  stw r26, 0xdc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(220 as u32), ctx.r[26].u32 ) };
	// 8304B5EC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8304B5F0: 41820164  beq 0x8304b754
	if ctx.cr[0].eq {
	pc = 0x8304B754; continue 'dispatch;
	}
	// 8304B5F4: 839D001C  lwz r28, 0x1c(r29)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 8304B5F8: 836B0008  lwz r27, 8(r11)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8304B5FC: 281C0000  cmplwi r28, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8304B600: 41820058  beq 0x8304b658
	if ctx.cr[0].eq {
	pc = 0x8304B658; continue 'dispatch;
	}
	// 8304B604: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8304B608: 93DF0060  stw r30, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[30].u32 ) };
	// 8304B60C: 7F1ED800  cmpw cr6, r30, r27
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[27].s32, &mut ctx.xer);
	// 8304B610: 40980048  bge cr6, 0x8304b658
	if !ctx.cr[6].lt {
	pc = 0x8304B658; continue 'dispatch;
	}
	// 8304B614: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8304B618: 807D005C  lwz r3, 0x5c(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(92 as u32) ) } as u64;
	// 8304B61C: 4BFE1255  bl 0x8302c870
	ctx.lr = 0x8304B620;
	sub_8302C870(ctx, base);
	// 8304B620: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8304B624: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8304B628: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304B62C: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 8304B630: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8304B634: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8304B638: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8304B63C: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 8304B640: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8304B644: 4E800421  bctrl
	ctx.lr = 0x8304B648;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8304B648: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8304B64C: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 8304B650: 93DF0060  stw r30, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[30].u32 ) };
	// 8304B654: 4BFFFFB8  b 0x8304b60c
	pc = 0x8304B60C; continue 'dispatch;
	// 8304B658: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8304B65C: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 8304B660: 40990040  ble cr6, 0x8304b6a0
	if !ctx.cr[6].gt {
	pc = 0x8304B6A0; continue 'dispatch;
	}
	// 8304B664: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8304B668: 839D0000  lwz r28, 0(r29)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304B66C: 807D005C  lwz r3, 0x5c(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(92 as u32) ) } as u64;
	// 8304B670: 4BFE1201  bl 0x8302c870
	ctx.lr = 0x8304B674;
	sub_8302C870(ctx, base);
	// 8304B674: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8304B678: 817C0040  lwz r11, 0x40(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(64 as u32) ) } as u64;
	// 8304B67C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304B680: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8304B684: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8304B688: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 8304B68C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8304B690: 4E800421  bctrl
	ctx.lr = 0x8304B694;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8304B694: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 8304B698: 7F1ED800  cmpw cr6, r30, r27
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[27].s32, &mut ctx.xer);
	// 8304B69C: 4198FFC8  blt cr6, 0x8304b664
	if ctx.cr[6].lt {
	pc = 0x8304B664; continue 'dispatch;
	}
	// 8304B6A0: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8304B6A4: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 8304B6A8: 4BF8CBF1  bl 0x82fd8298
	ctx.lr = 0x8304B6AC;
	sub_82FD8298(ctx, base);
	// 8304B6AC: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8304B6B0: 93DF0060  stw r30, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[30].u32 ) };
	// 8304B6B4: 4182002C  beq 0x8304b6e0
	if ctx.cr[0].eq {
	pc = 0x8304B6E0; continue 'dispatch;
	}
	// 8304B6B8: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 8304B6BC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8304B6C0: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8304B6C4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304B6C8: 48001129  bl 0x8304c7f0
	ctx.lr = 0x8304B6CC;
	sub_8304C7F0(ctx, base);
	// 8304B6CC: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8304B6D0: 394B2990  addi r10, r11, 0x2990
	ctx.r[10].s64 = ctx.r[11].s64 + 10640;
	// 8304B6D4: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 8304B6D8: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8304B6DC: 48000008  b 0x8304b6e4
	pc = 0x8304B6E4; continue 'dispatch;
	// 8304B6E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8304B6E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8304B6E8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8304B6EC: 917D0058  stw r11, 0x58(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 8304B6F0: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 8304B6F4: 995D0044  stb r10, 0x44(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(68 as u32), ctx.r[10].u8 ) };
	// 8304B6F8: 4099005C  ble cr6, 0x8304b754
	if !ctx.cr[6].gt {
	pc = 0x8304B754; continue 'dispatch;
	}
	// 8304B6FC: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8304B700: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 8304B704: 4BF8CB95  bl 0x82fd8298
	ctx.lr = 0x8304B708;
	sub_82FD8298(ctx, base);
	// 8304B708: 7C7C1B79  or. r28, r3, r3
	ctx.r[28].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 8304B70C: 939F0060  stw r28, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[28].u32 ) };
	// 8304B710: 41820028  beq 0x8304b738
	if ctx.cr[0].eq {
	pc = 0x8304B738; continue 'dispatch;
	}
	// 8304B714: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8304B718: 807D005C  lwz r3, 0x5c(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(92 as u32) ) } as u64;
	// 8304B71C: 4BFE1155  bl 0x8302c870
	ctx.lr = 0x8304B720;
	sub_8302C870(ctx, base);
	// 8304B720: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8304B724: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8304B728: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 8304B72C: 48048A55  bl 0x83094180
	ctx.lr = 0x8304B730;
	sub_83094180(ctx, base);
	// 8304B730: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8304B734: 48000008  b 0x8304b73c
	pc = 0x8304B73C; continue 'dispatch;
	// 8304B738: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8304B73C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8304B740: 807D0058  lwz r3, 0x58(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(88 as u32) ) } as u64;
	// 8304B744: 4BFFD66D  bl 0x83048db0
	ctx.lr = 0x8304B748;
	sub_83048DB0(ctx, base);
	// 8304B748: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 8304B74C: 7F1ED800  cmpw cr6, r30, r27
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[27].s32, &mut ctx.xer);
	// 8304B750: 4198FFAC  blt cr6, 0x8304b6fc
	if ctx.cr[6].lt {
	pc = 0x8304B6FC; continue 'dispatch;
	}
	// 8304B754: 383F00C0  addi r1, r31, 0xc0
	ctx.r[1].s64 = ctx.r[31].s64 + 192;
	// 8304B758: 4815CA58  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304B75C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8304B75C size=8
    let mut pc: u32 = 0x8304B75C;
    'dispatch: loop {
        match pc {
            0x8304B75C => {
    //   block [0x8304B75C..0x8304B764)
	// 8304B75C: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304B760: 82162544  lwz r16, 0x2544(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(9540 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304B764(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304B764 size=96
    let mut pc: u32 = 0x8304B764;
    'dispatch: loop {
        match pc {
            0x8304B764 => {
    //   block [0x8304B764..0x8304B7C4)
	// 8304B764: 3BECFF40  addi r31, r12, -0xc0
	ctx.r[31].s64 = ctx.r[12].s64 + -192;
	// 8304B768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304B76C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304B770: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304B774: 817F00D4  lwz r11, 0xd4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(212 as u32) ) } as u64;
	// 8304B778: 809F0060  lwz r4, 0x60(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 8304B77C: 83DF00DC  lwz r30, 0xdc(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(220 as u32) ) } as u64;
	// 8304B780: 806B005C  lwz r3, 0x5c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 8304B784: 4BFE10ED  bl 0x8302c870
	ctx.lr = 0x8304B788;
	sub_8302C870(ctx, base);
	// 8304B788: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8304B78C: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 8304B790: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 8304B794: 388B1FB0  addi r4, r11, 0x1fb0
	ctx.r[4].s64 = ctx.r[11].s64 + 8112;
	// 8304B798: 38C000B5  li r6, 0xb5
	ctx.r[6].s64 = 181;
	// 8304B79C: 38A001F5  li r5, 0x1f5
	ctx.r[5].s64 = 501;
	// 8304B7A0: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 8304B7A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8304B7A8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8304B7AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8304B7B0: 4BFF6369  bl 0x83041b18
	ctx.lr = 0x8304B7B4;
	sub_83041B18(ctx, base);
	// 8304B7B4: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8304B7B8: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 8304B7BC: 388BC990  addi r4, r11, -0x3670
	ctx.r[4].s64 = ctx.r[11].s64 + -13936;
	// 8304B7C0: 48165469  bl 0x831b0c28
	ctx.lr = 0x8304B7C4;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304B7C4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304B7C4 size=44
    let mut pc: u32 = 0x8304B7C4;
    'dispatch: loop {
        match pc {
            0x8304B7C4 => {
    //   block [0x8304B7C4..0x8304B7F0)
	// 8304B7C4: 3BECFF40  addi r31, r12, -0xc0
	ctx.r[31].s64 = ctx.r[12].s64 + -192;
	// 8304B7C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304B7CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304B7D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304B7D4: 809F00DC  lwz r4, 0xdc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(220 as u32) ) } as u64;
	// 8304B7D8: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 8304B7DC: 4BF8CB05  bl 0x82fd82e0
	ctx.lr = 0x8304B7E0;
	sub_82FD82E0(ctx, base);
	// 8304B7E0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8304B7E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304B7E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304B7EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304B7F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304B7F0 size=44
    let mut pc: u32 = 0x8304B7F0;
    'dispatch: loop {
        match pc {
            0x8304B7F0 => {
    //   block [0x8304B7F0..0x8304B81C)
	// 8304B7F0: 3BECFF40  addi r31, r12, -0xc0
	ctx.r[31].s64 = ctx.r[12].s64 + -192;
	// 8304B7F4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304B7F8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304B7FC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304B800: 809F00DC  lwz r4, 0xdc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(220 as u32) ) } as u64;
	// 8304B804: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 8304B808: 4BF8CAD9  bl 0x82fd82e0
	ctx.lr = 0x8304B80C;
	sub_82FD82E0(ctx, base);
	// 8304B80C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8304B810: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304B814: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304B818: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304B820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304B820 size=96
    let mut pc: u32 = 0x8304B820;
    'dispatch: loop {
        match pc {
            0x8304B820 => {
    //   block [0x8304B820..0x8304B880)
	// 8304B820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304B824: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304B828: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8304B82C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304B830: 7C882378  mr r8, r4
	ctx.r[8].u64 = ctx.r[4].u64;
	// 8304B834: 38E0001A  li r7, 0x1a
	ctx.r[7].s64 = 26;
	// 8304B838: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 8304B83C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8304B840: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8304B844: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8304B848: 4BFF5649  bl 0x83040e90
	ctx.lr = 0x8304B84C;
	sub_83040E90(ctx, base);
	// 8304B84C: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8304B850: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8304B854: 396B25D0  addi r11, r11, 0x25d0
	ctx.r[11].s64 = ctx.r[11].s64 + 9680;
	// 8304B858: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8304B85C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8304B860: B15F000A  sth r10, 0xa(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(10 as u32), ctx.r[10].u16 ) };
	// 8304B864: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8304B868: 993F003C  stb r9, 0x3c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[9].u8 ) };
	// 8304B86C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8304B870: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304B874: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304B878: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8304B87C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304B880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8304B880 size=8
    let mut pc: u32 = 0x8304B880;
    'dispatch: loop {
        match pc {
            0x8304B880 => {
    //   block [0x8304B880..0x8304B888)
	// 8304B880: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304B884: 82162608  lwz r16, 0x2608(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(9736 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304B888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304B888 size=96
    let mut pc: u32 = 0x8304B888;
    'dispatch: loop {
        match pc {
            0x8304B888 => {
    //   block [0x8304B888..0x8304B8E8)
	// 8304B888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304B88C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304B890: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8304B894: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8304B898: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 8304B89C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304B8A0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8304B8A4: 38600040  li r3, 0x40
	ctx.r[3].s64 = 64;
	// 8304B8A8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8304B8AC: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 8304B8B0: 4BF8C9E9  bl 0x82fd8298
	ctx.lr = 0x8304B8B4;
	sub_82FD8298(ctx, base);
	// 8304B8B4: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8304B8B8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8304B8BC: 41820010  beq 0x8304b8cc
	if ctx.cr[0].eq {
	pc = 0x8304B8CC; continue 'dispatch;
	}
	// 8304B8C0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8304B8C4: 4BFFFF5D  bl 0x8304b820
	ctx.lr = 0x8304B8C8;
	sub_8304B820(ctx, base);
	// 8304B8C8: 48000008  b 0x8304b8d0
	pc = 0x8304B8D0; continue 'dispatch;
	// 8304B8CC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8304B8D0: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 8304B8D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304B8D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304B8DC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8304B8E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8304B8E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304B8E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304B8E8 size=44
    let mut pc: u32 = 0x8304B8E8;
    'dispatch: loop {
        match pc {
            0x8304B8E8 => {
    //   block [0x8304B8E8..0x8304B914)
	// 8304B8E8: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 8304B8EC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304B8F0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304B8F4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304B8F8: 809F0084  lwz r4, 0x84(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 8304B8FC: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8304B900: 4BF8C9E1  bl 0x82fd82e0
	ctx.lr = 0x8304B904;
	sub_82FD82E0(ctx, base);
	// 8304B904: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8304B908: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304B90C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304B910: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304B918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8304B918 size=12
    let mut pc: u32 = 0x8304B918;
    'dispatch: loop {
        match pc {
            0x8304B918 => {
    //   block [0x8304B918..0x8304B924)
	// 8304B918: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8304B91C: 386B33E4  addi r3, r11, 0x33e4
	ctx.r[3].s64 = ctx.r[11].s64 + 13284;
	// 8304B920: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304B928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304B928 size=88
    let mut pc: u32 = 0x8304B928;
    'dispatch: loop {
        match pc {
            0x8304B928 => {
    //   block [0x8304B928..0x8304B980)
	// 8304B928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304B92C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304B930: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8304B934: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8304B938: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304B93C: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8304B940: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8304B944: 396B25D0  addi r11, r11, 0x25d0
	ctx.r[11].s64 = ctx.r[11].s64 + 9680;
	// 8304B948: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8304B94C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8304B950: 4BFF6089  bl 0x830419d8
	ctx.lr = 0x8304B954;
	sub_830419D8(ctx, base);
	// 8304B954: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304B958: 4182000C  beq 0x8304b964
	if ctx.cr[0].eq {
	pc = 0x8304B964; continue 'dispatch;
	}
	// 8304B95C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8304B960: 4BF8C981  bl 0x82fd82e0
	ctx.lr = 0x8304B964;
	sub_82FD82E0(ctx, base);
	// 8304B964: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8304B968: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8304B96C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304B970: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304B974: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8304B978: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8304B97C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304B980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304B980 size=124
    let mut pc: u32 = 0x8304B980;
    'dispatch: loop {
        match pc {
            0x8304B980 => {
    //   block [0x8304B980..0x8304B9FC)
	// 8304B980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304B984: 4815C7E9  bl 0x831a816c
	ctx.lr = 0x8304B988;
	sub_831A8130(ctx, base);
	// 8304B988: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304B98C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8304B990: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8304B994: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 8304B998: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8304B99C: 419A0014  beq cr6, 0x8304b9b0
	if ctx.cr[6].eq {
	pc = 0x8304B9B0; continue 'dispatch;
	}
	// 8304B9A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8304B9A4: 4BF9B2C5  bl 0x82fe6c68
	ctx.lr = 0x8304B9A8;
	sub_82FE6C68(ctx, base);
	// 8304B9A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8304B9AC: 4BF8C935  bl 0x82fd82e0
	ctx.lr = 0x8304B9B0;
	sub_82FD82E0(ctx, base);
	// 8304B9B0: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8304B9B4: 419A001C  beq cr6, 0x8304b9d0
	if ctx.cr[6].eq {
	pc = 0x8304B9D0; continue 'dispatch;
	}
	// 8304B9B8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304B9BC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8304B9C0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304B9C4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304B9C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8304B9CC: 4E800421  bctrl
	ctx.lr = 0x8304B9D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8304B9D0: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8304B9D4: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 8304B9D8: 388B2638  addi r4, r11, 0x2638
	ctx.r[4].s64 = ctx.r[11].s64 + 9784;
	// 8304B9DC: 38C0012D  li r6, 0x12d
	ctx.r[6].s64 = 301;
	// 8304B9E0: 38A00064  li r5, 0x64
	ctx.r[5].s64 = 100;
	// 8304B9E4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8304B9E8: 4BF85671  bl 0x82fd1058
	ctx.lr = 0x8304B9EC;
	sub_82FD1058(ctx, base);
	// 8304B9EC: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8304B9F0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8304B9F4: 388BC3EC  addi r4, r11, -0x3c14
	ctx.r[4].s64 = ctx.r[11].s64 + -15380;
	// 8304B9F8: 48165231  bl 0x831b0c28
	ctx.lr = 0x8304B9FC;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304BA00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304BA00 size=80
    let mut pc: u32 = 0x8304BA00;
    'dispatch: loop {
        match pc {
            0x8304BA00 => {
    //   block [0x8304BA00..0x8304BA50)
	// 8304BA00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304BA04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304BA08: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8304BA0C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304BA10: 7C882378  mr r8, r4
	ctx.r[8].u64 = ctx.r[4].u64;
	// 8304BA14: 38E00017  li r7, 0x17
	ctx.r[7].s64 = 23;
	// 8304BA18: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8304BA1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8304BA20: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8304BA24: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8304BA28: 4803EEA1  bl 0x8308a8c8
	ctx.lr = 0x8304BA2C;
	sub_8308A8C8(ctx, base);
	// 8304BA2C: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8304BA30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8304BA34: 396B26C0  addi r11, r11, 0x26c0
	ctx.r[11].s64 = ctx.r[11].s64 + 9920;
	// 8304BA38: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8304BA3C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8304BA40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304BA44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304BA48: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8304BA4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304BA50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8304BA50 size=8
    let mut pc: u32 = 0x8304BA50;
    'dispatch: loop {
        match pc {
            0x8304BA50 => {
    //   block [0x8304BA50..0x8304BA58)
	// 8304BA50: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304BA54: 82162720  lwz r16, 0x2720(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(10016 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304BA58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304BA58 size=84
    let mut pc: u32 = 0x8304BA58;
    'dispatch: loop {
        match pc {
            0x8304BA58 => {
    //   block [0x8304BA58..0x8304BAAC)
	// 8304BA58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304BA5C: 4815C70D  bl 0x831a8168
	ctx.lr = 0x8304BA60;
	sub_831A8130(ctx, base);
	// 8304BA60: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 8304BA64: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304BA68: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8304BA6C: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8304BA70: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 8304BA74: 38E00017  li r7, 0x17
	ctx.r[7].s64 = 23;
	// 8304BA78: 7D1D4378  mr r29, r8
	ctx.r[29].u64 = ctx.r[8].u64;
	// 8304BA7C: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 8304BA80: 4803EE49  bl 0x8308a8c8
	ctx.lr = 0x8304BA84;
	sub_8308A8C8(ctx, base);
	// 8304BA84: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8304BA88: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8304BA8C: 396B26C0  addi r11, r11, 0x26c0
	ctx.r[11].s64 = ctx.r[11].s64 + 9920;
	// 8304BA90: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8304BA94: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304BA98: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8304BA9C: 480401BD  bl 0x8308bc58
	ctx.lr = 0x8304BAA0;
	sub_8308BC58(ctx, base);
	// 8304BAA0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304BAA4: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 8304BAA8: 4815C710  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304BAAC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304BAAC size=40
    let mut pc: u32 = 0x8304BAAC;
    'dispatch: loop {
        match pc {
            0x8304BAAC => {
    //   block [0x8304BAAC..0x8304BAD4)
	// 8304BAAC: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 8304BAB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304BAB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304BAB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304BABC: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 8304BAC0: 4803ED11  bl 0x8308a7d0
	ctx.lr = 0x8304BAC4;
	sub_8308A7D0(ctx, base);
	// 8304BAC4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8304BAC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304BACC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304BAD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304BAD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8304BAD8 size=8
    let mut pc: u32 = 0x8304BAD8;
    'dispatch: loop {
        match pc {
            0x8304BAD8 => {
    //   block [0x8304BAD8..0x8304BAE0)
	// 8304BAD8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304BADC: 82162758  lwz r16, 0x2758(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(10072 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304BAE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304BAE0 size=104
    let mut pc: u32 = 0x8304BAE0;
    'dispatch: loop {
        match pc {
            0x8304BAE0 => {
    //   block [0x8304BAE0..0x8304BB48)
	// 8304BAE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304BAE4: 4815C67D  bl 0x831a8160
	ctx.lr = 0x8304BAE8;
	sub_831A8130(ctx, base);
	// 8304BAE8: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 8304BAEC: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304BAF0: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 8304BAF4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8304BAF8: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8304BAFC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8304BB00: 38600054  li r3, 0x54
	ctx.r[3].s64 = 84;
	// 8304BB04: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 8304BB08: 93DF00C4  stw r30, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[30].u32 ) };
	// 8304BB0C: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 8304BB10: 4BF8C789  bl 0x82fd8298
	ctx.lr = 0x8304BB14;
	sub_82FD8298(ctx, base);
	// 8304BB14: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8304BB18: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8304BB1C: 41820020  beq 0x8304bb3c
	if ctx.cr[0].eq {
	pc = 0x8304BB3C; continue 'dispatch;
	}
	// 8304BB20: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 8304BB24: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 8304BB28: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 8304BB2C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8304BB30: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8304BB34: 4BFFFF25  bl 0x8304ba58
	ctx.lr = 0x8304BB38;
	sub_8304BA58(ctx, base);
	// 8304BB38: 48000008  b 0x8304bb40
	pc = 0x8304BB40; continue 'dispatch;
	// 8304BB3C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8304BB40: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 8304BB44: 4815C66C  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304BB48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304BB48 size=44
    let mut pc: u32 = 0x8304BB48;
    'dispatch: loop {
        match pc {
            0x8304BB48 => {
    //   block [0x8304BB48..0x8304BB74)
	// 8304BB48: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 8304BB4C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304BB50: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304BB54: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304BB58: 809F00C4  lwz r4, 0xc4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 8304BB5C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8304BB60: 4BF8C781  bl 0x82fd82e0
	ctx.lr = 0x8304BB64;
	sub_82FD82E0(ctx, base);
	// 8304BB64: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8304BB68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304BB6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304BB70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304BB78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304BB78 size=132
    let mut pc: u32 = 0x8304BB78;
    'dispatch: loop {
        match pc {
            0x8304BB78 => {
    //   block [0x8304BB78..0x8304BBFC)
	// 8304BB78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304BB7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304BB80: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8304BB84: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8304BB88: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304BB8C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8304BB90: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8304BB94: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8304BB98: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8304BB9C: 4BF8701D  bl 0x82fd2bb8
	ctx.lr = 0x8304BBA0;
	sub_82FD2BB8(ctx, base);
	// 8304BBA0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304BBA4: 40820040  bne 0x8304bbe4
	if !ctx.cr[0].eq {
	pc = 0x8304BBE4; continue 'dispatch;
	}
	// 8304BBA8: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8304BBAC: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 8304BBB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8304BBB4: 388B2788  addi r4, r11, 0x2788
	ctx.r[4].s64 = ctx.r[11].s64 + 10120;
	// 8304BBB8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8304BBBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8304BBC0: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 8304BBC4: 38C00102  li r6, 0x102
	ctx.r[6].s64 = 258;
	// 8304BBC8: 38A00088  li r5, 0x88
	ctx.r[5].s64 = 136;
	// 8304BBCC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8304BBD0: 4BFC9FB9  bl 0x83015b88
	ctx.lr = 0x8304BBD4;
	sub_83015B88(ctx, base);
	// 8304BBD4: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8304BBD8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8304BBDC: 388BC8B0  addi r4, r11, -0x3750
	ctx.r[4].s64 = ctx.r[11].s64 + -14160;
	// 8304BBE0: 48165049  bl 0x831b0c28
	ctx.lr = 0x8304BBE4;
	sub_831B0C28(ctx, base);
	// 8304BBE4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8304BBE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304BBEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304BBF0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8304BBF4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8304BBF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304BC00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8304BC00 size=8
    let mut pc: u32 = 0x8304BC00;
    'dispatch: loop {
        match pc {
            0x8304BC00 => {
    //   block [0x8304BC00..0x8304BC08)
	// 8304BC00: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304BC04: 821627E0  lwz r16, 0x27e0(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(10208 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304BC08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304BC08 size=96
    let mut pc: u32 = 0x8304BC08;
    'dispatch: loop {
        match pc {
            0x8304BC08 => {
    //   block [0x8304BC08..0x8304BC68)
	// 8304BC08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304BC0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304BC10: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8304BC14: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8304BC18: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 8304BC1C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304BC20: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8304BC24: 38600054  li r3, 0x54
	ctx.r[3].s64 = 84;
	// 8304BC28: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8304BC2C: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 8304BC30: 4BF8C669  bl 0x82fd8298
	ctx.lr = 0x8304BC34;
	sub_82FD8298(ctx, base);
	// 8304BC34: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8304BC38: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8304BC3C: 41820010  beq 0x8304bc4c
	if ctx.cr[0].eq {
	pc = 0x8304BC4C; continue 'dispatch;
	}
	// 8304BC40: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8304BC44: 4BFFFDBD  bl 0x8304ba00
	ctx.lr = 0x8304BC48;
	sub_8304BA00(ctx, base);
	// 8304BC48: 48000008  b 0x8304bc50
	pc = 0x8304BC50; continue 'dispatch;
	// 8304BC4C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8304BC50: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 8304BC54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304BC58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304BC5C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8304BC60: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8304BC64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304BC68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304BC68 size=44
    let mut pc: u32 = 0x8304BC68;
    'dispatch: loop {
        match pc {
            0x8304BC68 => {
    //   block [0x8304BC68..0x8304BC94)
	// 8304BC68: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 8304BC6C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304BC70: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304BC74: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304BC78: 809F0084  lwz r4, 0x84(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 8304BC7C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8304BC80: 4BF8C661  bl 0x82fd82e0
	ctx.lr = 0x8304BC84;
	sub_82FD82E0(ctx, base);
	// 8304BC84: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8304BC88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304BC8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304BC90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304BC98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8304BC98 size=12
    let mut pc: u32 = 0x8304BC98;
    'dispatch: loop {
        match pc {
            0x8304BC98 => {
    //   block [0x8304BC98..0x8304BCA4)
	// 8304BC98: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8304BC9C: 386B33EC  addi r3, r11, 0x33ec
	ctx.r[3].s64 = ctx.r[11].s64 + 13292;
	// 8304BCA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304BCA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304BCA8 size=88
    let mut pc: u32 = 0x8304BCA8;
    'dispatch: loop {
        match pc {
            0x8304BCA8 => {
    //   block [0x8304BCA8..0x8304BD00)
	// 8304BCA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304BCAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304BCB0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8304BCB4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8304BCB8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304BCBC: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8304BCC0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8304BCC4: 396B26C0  addi r11, r11, 0x26c0
	ctx.r[11].s64 = ctx.r[11].s64 + 9920;
	// 8304BCC8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8304BCCC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8304BCD0: 4803EB01  bl 0x8308a7d0
	ctx.lr = 0x8304BCD4;
	sub_8308A7D0(ctx, base);
	// 8304BCD4: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304BCD8: 4182000C  beq 0x8304bce4
	if ctx.cr[0].eq {
	pc = 0x8304BCE4; continue 'dispatch;
	}
	// 8304BCDC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8304BCE0: 4BF8C601  bl 0x82fd82e0
	ctx.lr = 0x8304BCE4;
	sub_82FD82E0(ctx, base);
	// 8304BCE4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8304BCE8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8304BCEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304BCF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304BCF4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8304BCF8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8304BCFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304BD00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304BD00 size=88
    let mut pc: u32 = 0x8304BD00;
    'dispatch: loop {
        match pc {
            0x8304BD00 => {
    //   block [0x8304BD00..0x8304BD58)
	// 8304BD00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304BD04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304BD08: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8304BD0C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304BD10: 7C882378  mr r8, r4
	ctx.r[8].u64 = ctx.r[4].u64;
	// 8304BD14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8304BD18: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8304BD1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8304BD20: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8304BD24: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8304BD28: 4803EBA1  bl 0x8308a8c8
	ctx.lr = 0x8304BD2C;
	sub_8308A8C8(ctx, base);
	// 8304BD2C: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8304BD30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8304BD34: 396B2828  addi r11, r11, 0x2828
	ctx.r[11].s64 = ctx.r[11].s64 + 10280;
	// 8304BD38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8304BD3C: B15F000A  sth r10, 0xa(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(10 as u32), ctx.r[10].u16 ) };
	// 8304BD40: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8304BD44: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8304BD48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304BD4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304BD50: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8304BD54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304BD58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8304BD58 size=8
    let mut pc: u32 = 0x8304BD58;
    'dispatch: loop {
        match pc {
            0x8304BD58 => {
    //   block [0x8304BD58..0x8304BD60)
	// 8304BD58: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304BD5C: 82162888  lwz r16, 0x2888(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(10376 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304BD60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304BD60 size=92
    let mut pc: u32 = 0x8304BD60;
    'dispatch: loop {
        match pc {
            0x8304BD60 => {
    //   block [0x8304BD60..0x8304BDBC)
	// 8304BD60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304BD64: 4815C405  bl 0x831a8168
	ctx.lr = 0x8304BD68;
	sub_831A8130(ctx, base);
	// 8304BD68: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 8304BD6C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304BD70: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8304BD74: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8304BD78: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 8304BD7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8304BD80: 7D1D4378  mr r29, r8
	ctx.r[29].u64 = ctx.r[8].u64;
	// 8304BD84: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 8304BD88: 4803EB41  bl 0x8308a8c8
	ctx.lr = 0x8304BD8C;
	sub_8308A8C8(ctx, base);
	// 8304BD8C: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8304BD90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8304BD94: 396B2828  addi r11, r11, 0x2828
	ctx.r[11].s64 = ctx.r[11].s64 + 10280;
	// 8304BD98: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8304BD9C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8304BDA0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304BDA4: B15E000A  sth r10, 0xa(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(10 as u32), ctx.r[10].u16 ) };
	// 8304BDA8: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8304BDAC: 4803FEAD  bl 0x8308bc58
	ctx.lr = 0x8304BDB0;
	sub_8308BC58(ctx, base);
	// 8304BDB0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304BDB4: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 8304BDB8: 4815C400  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304BDBC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304BDBC size=40
    let mut pc: u32 = 0x8304BDBC;
    'dispatch: loop {
        match pc {
            0x8304BDBC => {
    //   block [0x8304BDBC..0x8304BDE4)
	// 8304BDBC: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 8304BDC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304BDC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304BDC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304BDCC: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 8304BDD0: 4803EA01  bl 0x8308a7d0
	ctx.lr = 0x8304BDD4;
	sub_8308A7D0(ctx, base);
	// 8304BDD4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8304BDD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304BDDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304BDE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304BDE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8304BDE8 size=16
    let mut pc: u32 = 0x8304BDE8;
    'dispatch: loop {
        match pc {
            0x8304BDE8 => {
    //   block [0x8304BDE8..0x8304BDF8)
	// 8304BDE8: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8304BDEC: 396B2828  addi r11, r11, 0x2828
	ctx.r[11].s64 = ctx.r[11].s64 + 10280;
	// 8304BDF0: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8304BDF4: 4803E9DC  b 0x8308a7d0
	sub_8308A7D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304BDF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8304BDF8 size=8
    let mut pc: u32 = 0x8304BDF8;
    'dispatch: loop {
        match pc {
            0x8304BDF8 => {
    //   block [0x8304BDF8..0x8304BE00)
	// 8304BDF8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304BDFC: 821628C0  lwz r16, 0x28c0(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(10432 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304BE00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304BE00 size=104
    let mut pc: u32 = 0x8304BE00;
    'dispatch: loop {
        match pc {
            0x8304BE00 => {
    //   block [0x8304BE00..0x8304BE68)
	// 8304BE00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304BE04: 4815C35D  bl 0x831a8160
	ctx.lr = 0x8304BE08;
	sub_831A8130(ctx, base);
	// 8304BE08: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 8304BE0C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304BE10: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 8304BE14: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8304BE18: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8304BE1C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8304BE20: 38600054  li r3, 0x54
	ctx.r[3].s64 = 84;
	// 8304BE24: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 8304BE28: 93DF00C4  stw r30, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[30].u32 ) };
	// 8304BE2C: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 8304BE30: 4BF8C469  bl 0x82fd8298
	ctx.lr = 0x8304BE34;
	sub_82FD8298(ctx, base);
	// 8304BE34: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8304BE38: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8304BE3C: 41820020  beq 0x8304be5c
	if ctx.cr[0].eq {
	pc = 0x8304BE5C; continue 'dispatch;
	}
	// 8304BE40: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 8304BE44: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 8304BE48: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 8304BE4C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8304BE50: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8304BE54: 4BFFFF0D  bl 0x8304bd60
	ctx.lr = 0x8304BE58;
	sub_8304BD60(ctx, base);
	// 8304BE58: 48000008  b 0x8304be60
	pc = 0x8304BE60; continue 'dispatch;
	// 8304BE5C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8304BE60: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 8304BE64: 4815C34C  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304BE68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304BE68 size=44
    let mut pc: u32 = 0x8304BE68;
    'dispatch: loop {
        match pc {
            0x8304BE68 => {
    //   block [0x8304BE68..0x8304BE94)
	// 8304BE68: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 8304BE6C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304BE70: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304BE74: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304BE78: 809F00C4  lwz r4, 0xc4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 8304BE7C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8304BE80: 4BF8C461  bl 0x82fd82e0
	ctx.lr = 0x8304BE84;
	sub_82FD82E0(ctx, base);
	// 8304BE84: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8304BE88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304BE8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304BE90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304BE98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304BE98 size=68
    let mut pc: u32 = 0x8304BE98;
    'dispatch: loop {
        match pc {
            0x8304BE98 => {
    //   block [0x8304BE98..0x8304BEDC)
	// 8304BE98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304BE9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304BEA0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8304BEA4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304BEA8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8304BEAC: 4803EA1D  bl 0x8308a8c8
	ctx.lr = 0x8304BEB0;
	sub_8308A8C8(ctx, base);
	// 8304BEB0: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8304BEB4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8304BEB8: 396B2828  addi r11, r11, 0x2828
	ctx.r[11].s64 = ctx.r[11].s64 + 10280;
	// 8304BEBC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8304BEC0: B15F000A  sth r10, 0xa(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(10 as u32), ctx.r[10].u16 ) };
	// 8304BEC4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8304BEC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8304BECC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304BED0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304BED4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8304BED8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304BEE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304BEE0 size=288
    let mut pc: u32 = 0x8304BEE0;
    'dispatch: loop {
        match pc {
            0x8304BEE0 => {
    //   block [0x8304BEE0..0x8304C000)
	// 8304BEE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304BEE4: 4815C285  bl 0x831a8168
	ctx.lr = 0x8304BEE8;
	sub_831A8130(ctx, base);
	// 8304BEE8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304BEEC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8304BEF0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8304BEF4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8304BEF8: 388BCE48  addi r4, r11, -0x31b8
	ctx.r[4].s64 = ctx.r[11].s64 + -12728;
	// 8304BEFC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304BF00: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 8304BF04: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 8304BF08: 4BF87D39  bl 0x82fd3c40
	ctx.lr = 0x8304BF0C;
	sub_82FD3C40(ctx, base);
	// 8304BF0C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304BF10: 418200B4  beq 0x8304bfc4
	if ctx.cr[0].eq {
	pc = 0x8304BFC4; continue 'dispatch;
	}
	// 8304BF14: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8304BF18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8304BF1C: 388BD5E4  addi r4, r11, -0x2a1c
	ctx.r[4].s64 = ctx.r[11].s64 + -10780;
	// 8304BF20: 4BF87D21  bl 0x82fd3c40
	ctx.lr = 0x8304BF24;
	sub_82FD3C40(ctx, base);
	// 8304BF24: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304BF28: 4182000C  beq 0x8304bf34
	if ctx.cr[0].eq {
	pc = 0x8304BF34; continue 'dispatch;
	}
	// 8304BF2C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8304BF30: 48000040  b 0x8304bf70
	pc = 0x8304BF70; continue 'dispatch;
	// 8304BF34: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8304BF38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8304BF3C: 388BD60C  addi r4, r11, -0x29f4
	ctx.r[4].s64 = ctx.r[11].s64 + -10740;
	// 8304BF40: 4BF87D01  bl 0x82fd3c40
	ctx.lr = 0x8304BF44;
	sub_82FD3C40(ctx, base);
	// 8304BF44: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304BF48: 4182000C  beq 0x8304bf54
	if ctx.cr[0].eq {
	pc = 0x8304BF54; continue 'dispatch;
	}
	// 8304BF4C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8304BF50: 48000020  b 0x8304bf70
	pc = 0x8304BF70; continue 'dispatch;
	// 8304BF54: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8304BF58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8304BF5C: 388BD5F8  addi r4, r11, -0x2a08
	ctx.r[4].s64 = ctx.r[11].s64 + -10760;
	// 8304BF60: 4BF87CE1  bl 0x82fd3c40
	ctx.lr = 0x8304BF64;
	sub_82FD3C40(ctx, base);
	// 8304BF64: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304BF68: 41820020  beq 0x8304bf88
	if ctx.cr[0].eq {
	pc = 0x8304BF88; continue 'dispatch;
	}
	// 8304BF6C: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 8304BF70: B17C000A  sth r11, 0xa(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(10 as u32), ctx.r[11].u16 ) };
	// 8304BF74: 817C0010  lwz r11, 0x10(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(16 as u32) ) } as u64;
	// 8304BF78: 616B4000  ori r11, r11, 0x4000
	ctx.r[11].u64 = ctx.r[11].u64 | 16384;
	// 8304BF7C: 917C0010  stw r11, 0x10(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8304BF80: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8304BF84: 4815C234  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 8304BF88: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8304BF8C: 93A10054  stw r29, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 8304BF90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8304BF94: 388B28F0  addi r4, r11, 0x28f0
	ctx.r[4].s64 = ctx.r[11].s64 + 10480;
	// 8304BF98: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8304BF9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8304BFA0: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 8304BFA4: 38C000B6  li r6, 0xb6
	ctx.r[6].s64 = 182;
	// 8304BFA8: 38A000B7  li r5, 0xb7
	ctx.r[5].s64 = 183;
	// 8304BFAC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8304BFB0: 4BFF5B69  bl 0x83041b18
	ctx.lr = 0x8304BFB4;
	sub_83041B18(ctx, base);
	// 8304BFB4: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8304BFB8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8304BFBC: 388BC990  addi r4, r11, -0x3670
	ctx.r[4].s64 = ctx.r[11].s64 + -13936;
	// 8304BFC0: 48164C69  bl 0x831b0c28
	ctx.lr = 0x8304BFC4;
	sub_831B0C28(ctx, base);
	// 8304BFC4: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8304BFC8: 93A10054  stw r29, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 8304BFCC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8304BFD0: 388B28F0  addi r4, r11, 0x28f0
	ctx.r[4].s64 = ctx.r[11].s64 + 10480;
	// 8304BFD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8304BFD8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8304BFDC: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 8304BFE0: 38C000AB  li r6, 0xab
	ctx.r[6].s64 = 171;
	// 8304BFE4: 38A000C1  li r5, 0xc1
	ctx.r[5].s64 = 193;
	// 8304BFE8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8304BFEC: 4BFF5B2D  bl 0x83041b18
	ctx.lr = 0x8304BFF0;
	sub_83041B18(ctx, base);
	// 8304BFF0: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8304BFF4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8304BFF8: 388BC990  addi r4, r11, -0x3670
	ctx.r[4].s64 = ctx.r[11].s64 + -13936;
	// 8304BFFC: 48164C2D  bl 0x831b0c28
	ctx.lr = 0x8304C000;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304C000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8304C000 size=12
    let mut pc: u32 = 0x8304C000;
    'dispatch: loop {
        match pc {
            0x8304C000 => {
    //   block [0x8304C000..0x8304C00C)
	// 8304C000: 8143001C  lwz r10, 0x1c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 8304C004: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8304C008: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304C00C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8304C00C size=12
    let mut pc: u32 = 0x8304C00C;
    'dispatch: loop {
        match pc {
            0x8304C00C => {
    //   block [0x8304C00C..0x8304C018)
	// 8304C00C: 816A0010  lwz r11, 0x10(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 8304C010: 556B0463  rlwinm. r11, r11, 0, 0x11, 0x11
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304C014: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304C018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8304C018 size=12
    let mut pc: u32 = 0x8304C018;
    'dispatch: loop {
        match pc {
            0x8304C018 => {
    //   block [0x8304C018..0x8304C024)
	// 8304C018: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 8304C01C: 55690463  rlwinm. r9, r11, 0, 0x11, 0x11
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8304C020: 4C820020  bnelr
	if !ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304C024(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8304C024 size=20
    let mut pc: u32 = 0x8304C024;
    'dispatch: loop {
        match pc {
            0x8304C024 => {
    //   block [0x8304C024..0x8304C038)
	// 8304C024: A14A000A  lhz r10, 0xa(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(10 as u32) ) } as u64;
	// 8304C028: 616B4000  ori r11, r11, 0x4000
	ctx.r[11].u64 = ctx.r[11].u64 | 16384;
	// 8304C02C: B143000A  sth r10, 0xa(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(10 as u32), ctx.r[10].u16 ) };
	// 8304C030: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8304C034: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304C038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304C038 size=312
    let mut pc: u32 = 0x8304C038;
    'dispatch: loop {
        match pc {
            0x8304C038 => {
    //   block [0x8304C038..0x8304C170)
	// 8304C038: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304C03C: 4815C12D  bl 0x831a8168
	ctx.lr = 0x8304C040;
	sub_831A8130(ctx, base);
	// 8304C040: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304C044: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8304C048: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8304C04C: 815F001C  lwz r10, 0x1c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 8304C050: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8304C054: 41820114  beq 0x8304c168
	if ctx.cr[0].eq {
	pc = 0x8304C168; continue 'dispatch;
	}
	// 8304C058: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8304C05C: A3DF000A  lhz r30, 0xa(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(10 as u32) ) } as u64;
	// 8304C060: A08A000A  lhz r4, 0xa(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(10 as u32) ) } as u64;
	// 8304C064: 556B0463  rlwinm. r11, r11, 0, 0x11, 0x11
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304C068: 41820100  beq 0x8304c168
	if ctx.cr[0].eq {
	pc = 0x8304C168; continue 'dispatch;
	}
	// 8304C06C: 816A0010  lwz r11, 0x10(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 8304C070: 556B0463  rlwinm. r11, r11, 0, 0x11, 0x11
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304C074: 418200F4  beq 0x8304c168
	if ctx.cr[0].eq {
	pc = 0x8304C168; continue 'dispatch;
	}
	// 8304C078: 7C890734  extsh r9, r4
	ctx.r[9].s64 = ctx.r[4].s16 as i64;
	// 8304C07C: 2F090002  cmpwi cr6, r9, 2
	ctx.cr[6].compare_i32(ctx.r[9].s32, 2, &mut ctx.xer);
	// 8304C080: 409A0040  bne cr6, 0x8304c0c0
	if !ctx.cr[6].eq {
	pc = 0x8304C0C0; continue 'dispatch;
	}
	// 8304C084: 7FCB0735  extsh. r11, r30
	ctx.r[11].s64 = ctx.r[30].s16 as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304C088: 4182000C  beq 0x8304c094
	if ctx.cr[0].eq {
	pc = 0x8304C094; continue 'dispatch;
	}
	// 8304C08C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8304C090: 409A0030  bne cr6, 0x8304c0c0
	if !ctx.cr[6].eq {
	pc = 0x8304C0C0; continue 'dispatch;
	}
	// 8304C094: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8304C098: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 8304C09C: 388B28F0  addi r4, r11, 0x28f0
	ctx.r[4].s64 = ctx.r[11].s64 + 10480;
	// 8304C0A0: 38C000B7  li r6, 0xb7
	ctx.r[6].s64 = 183;
	// 8304C0A4: 38A000E7  li r5, 0xe7
	ctx.r[5].s64 = 231;
	// 8304C0A8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8304C0AC: 4BFF64C5  bl 0x83042570
	ctx.lr = 0x8304C0B0;
	sub_83042570(ctx, base);
	// 8304C0B0: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8304C0B4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8304C0B8: 388BC990  addi r4, r11, -0x3670
	ctx.r[4].s64 = ctx.r[11].s64 + -13936;
	// 8304C0BC: 48164B6D  bl 0x831b0c28
	ctx.lr = 0x8304C0C0;
	sub_831B0C28(ctx, base);
	// 8304C0C0: 2F090001  cmpwi cr6, r9, 1
	ctx.cr[6].compare_i32(ctx.r[9].s32, 1, &mut ctx.xer);
	// 8304C0C4: 409A0038  bne cr6, 0x8304c0fc
	if !ctx.cr[6].eq {
	pc = 0x8304C0FC; continue 'dispatch;
	}
	// 8304C0C8: 7FCB0735  extsh. r11, r30
	ctx.r[11].s64 = ctx.r[30].s16 as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304C0CC: 40820030  bne 0x8304c0fc
	if !ctx.cr[0].eq {
	pc = 0x8304C0FC; continue 'dispatch;
	}
	// 8304C0D0: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8304C0D4: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 8304C0D8: 388B28F0  addi r4, r11, 0x28f0
	ctx.r[4].s64 = ctx.r[11].s64 + 10480;
	// 8304C0DC: 38C000B8  li r6, 0xb8
	ctx.r[6].s64 = 184;
	// 8304C0E0: 38A000EB  li r5, 0xeb
	ctx.r[5].s64 = 235;
	// 8304C0E4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8304C0E8: 4BFF6489  bl 0x83042570
	ctx.lr = 0x8304C0EC;
	sub_83042570(ctx, base);
	// 8304C0EC: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8304C0F0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8304C0F4: 388BC990  addi r4, r11, -0x3670
	ctx.r[4].s64 = ctx.r[11].s64 + -13936;
	// 8304C0F8: 48164B31  bl 0x831b0c28
	ctx.lr = 0x8304C0FC;
	sub_831B0C28(ctx, base);
	// 8304C0FC: 816A0014  lwz r11, 0x14(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 8304C100: 556B0463  rlwinm. r11, r11, 0, 0x11, 0x11
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304C104: 41820064  beq 0x8304c168
	if ctx.cr[0].eq {
	pc = 0x8304C168; continue 'dispatch;
	}
	// 8304C108: 7FCB0734  extsh r11, r30
	ctx.r[11].s64 = ctx.r[30].s16 as i64;
	// 8304C10C: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 8304C110: 419A0058  beq cr6, 0x8304c168
	if ctx.cr[6].eq {
	pc = 0x8304C168; continue 'dispatch;
	}
	// 8304C114: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8304C118: 4BFF4DF9  bl 0x83040f10
	ctx.lr = 0x8304C11C;
	sub_83040F10(ctx, base);
	// 8304C11C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8304C120: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8304C124: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8304C128: 4BFF4DE9  bl 0x83040f10
	ctx.lr = 0x8304C12C;
	sub_83040F10(ctx, base);
	// 8304C12C: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8304C130: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 8304C134: 93A10054  stw r29, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 8304C138: 388B28F0  addi r4, r11, 0x28f0
	ctx.r[4].s64 = ctx.r[11].s64 + 10480;
	// 8304C13C: 38C000E8  li r6, 0xe8
	ctx.r[6].s64 = 232;
	// 8304C140: 38A000F4  li r5, 0xf4
	ctx.r[5].s64 = 244;
	// 8304C144: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8304C148: 7F88E378  mr r8, r28
	ctx.r[8].u64 = ctx.r[28].u64;
	// 8304C14C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8304C150: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8304C154: 4BFF59C5  bl 0x83041b18
	ctx.lr = 0x8304C158;
	sub_83041B18(ctx, base);
	// 8304C158: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8304C15C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8304C160: 388BC990  addi r4, r11, -0x3670
	ctx.r[4].s64 = ctx.r[11].s64 + -13936;
	// 8304C164: 48164AC5  bl 0x831b0c28
	ctx.lr = 0x8304C168;
	sub_831B0C28(ctx, base);
	// 8304C168: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8304C16C: 4815C04C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304C170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304C170 size=236
    let mut pc: u32 = 0x8304C170;
    'dispatch: loop {
        match pc {
            0x8304C170 => {
    //   block [0x8304C170..0x8304C25C)
	// 8304C170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304C174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304C178: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8304C17C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8304C180: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304C184: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 8304C188: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8304C18C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8304C190: 556B0463  rlwinm. r11, r11, 0, 0x11, 0x11
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304C194: 418200B0  beq 0x8304c244
	if ctx.cr[0].eq {
	pc = 0x8304C244; continue 'dispatch;
	}
	// 8304C198: A963000A  lha r11, 0xa(r3)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(10 as u32) ) } as i16) as i64;
	// 8304C19C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8304C1A0: 409A0050  bne cr6, 0x8304c1f0
	if !ctx.cr[6].eq {
	pc = 0x8304C1F0; continue 'dispatch;
	}
	// 8304C1A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8304C1A8: 4BF860C1  bl 0x82fd2268
	ctx.lr = 0x8304C1AC;
	sub_82FD2268(ctx, base);
	// 8304C1AC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304C1B0: 40820094  bne 0x8304c244
	if !ctx.cr[0].eq {
	pc = 0x8304C244; continue 'dispatch;
	}
	// 8304C1B4: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8304C1B8: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 8304C1BC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8304C1C0: 388B28F0  addi r4, r11, 0x28f0
	ctx.r[4].s64 = ctx.r[11].s64 + 10480;
	// 8304C1C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8304C1C8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8304C1CC: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 8304C1D0: 38C000FB  li r6, 0xfb
	ctx.r[6].s64 = 251;
	// 8304C1D4: 38A00105  li r5, 0x105
	ctx.r[5].s64 = 261;
	// 8304C1D8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8304C1DC: 4BFC99AD  bl 0x83015b88
	ctx.lr = 0x8304C1E0;
	sub_83015B88(ctx, base);
	// 8304C1E0: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8304C1E4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8304C1E8: 388BC8B0  addi r4, r11, -0x3750
	ctx.r[4].s64 = ctx.r[11].s64 + -14160;
	// 8304C1EC: 48164A3D  bl 0x831b0c28
	ctx.lr = 0x8304C1F0;
	sub_831B0C28(ctx, base);
	// 8304C1F0: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8304C1F4: 409A0050  bne cr6, 0x8304c244
	if !ctx.cr[6].eq {
	pc = 0x8304C244; continue 'dispatch;
	}
	// 8304C1F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8304C1FC: 4BF860B5  bl 0x82fd22b0
	ctx.lr = 0x8304C200;
	sub_82FD22B0(ctx, base);
	// 8304C200: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304C204: 40820040  bne 0x8304c244
	if !ctx.cr[0].eq {
	pc = 0x8304C244; continue 'dispatch;
	}
	// 8304C208: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8304C20C: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 8304C210: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8304C214: 388B28F0  addi r4, r11, 0x28f0
	ctx.r[4].s64 = ctx.r[11].s64 + 10480;
	// 8304C218: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8304C21C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8304C220: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 8304C224: 38C000FC  li r6, 0xfc
	ctx.r[6].s64 = 252;
	// 8304C228: 38A0010A  li r5, 0x10a
	ctx.r[5].s64 = 266;
	// 8304C22C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8304C230: 4BFC9959  bl 0x83015b88
	ctx.lr = 0x8304C234;
	sub_83015B88(ctx, base);
	// 8304C234: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8304C238: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8304C23C: 388BC8B0  addi r4, r11, -0x3750
	ctx.r[4].s64 = ctx.r[11].s64 + -14160;
	// 8304C240: 481649E9  bl 0x831b0c28
	ctx.lr = 0x8304C244;
	sub_831B0C28(ctx, base);
	// 8304C244: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8304C248: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304C24C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304C250: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8304C254: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8304C258: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304C260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8304C260 size=8
    let mut pc: u32 = 0x8304C260;
    'dispatch: loop {
        match pc {
            0x8304C260 => {
    //   block [0x8304C260..0x8304C268)
	// 8304C260: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304C264: 82162948  lwz r16, 0x2948(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(10568 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304C268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304C268 size=96
    let mut pc: u32 = 0x8304C268;
    'dispatch: loop {
        match pc {
            0x8304C268 => {
    //   block [0x8304C268..0x8304C2C8)
	// 8304C268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304C26C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304C270: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8304C274: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8304C278: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 8304C27C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304C280: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8304C284: 38600054  li r3, 0x54
	ctx.r[3].s64 = 84;
	// 8304C288: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8304C28C: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 8304C290: 4BF8C009  bl 0x82fd8298
	ctx.lr = 0x8304C294;
	sub_82FD8298(ctx, base);
	// 8304C294: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8304C298: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8304C29C: 41820010  beq 0x8304c2ac
	if ctx.cr[0].eq {
	pc = 0x8304C2AC; continue 'dispatch;
	}
	// 8304C2A0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8304C2A4: 4BFFFA5D  bl 0x8304bd00
	ctx.lr = 0x8304C2A8;
	sub_8304BD00(ctx, base);
	// 8304C2A8: 48000008  b 0x8304c2b0
	pc = 0x8304C2B0; continue 'dispatch;
	// 8304C2AC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8304C2B0: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 8304C2B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304C2B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304C2BC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8304C2C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8304C2C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304C2C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304C2C8 size=44
    let mut pc: u32 = 0x8304C2C8;
    'dispatch: loop {
        match pc {
            0x8304C2C8 => {
    //   block [0x8304C2C8..0x8304C2F4)
	// 8304C2C8: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 8304C2CC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304C2D0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304C2D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304C2D8: 809F0084  lwz r4, 0x84(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 8304C2DC: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8304C2E0: 4BF8C001  bl 0x82fd82e0
	ctx.lr = 0x8304C2E4;
	sub_82FD82E0(ctx, base);
	// 8304C2E4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8304C2E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304C2EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304C2F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304C2F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8304C2F8 size=12
    let mut pc: u32 = 0x8304C2F8;
    'dispatch: loop {
        match pc {
            0x8304C2F8 => {
    //   block [0x8304C2F8..0x8304C304)
	// 8304C2F8: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8304C2FC: 386B33F4  addi r3, r11, 0x33f4
	ctx.r[3].s64 = ctx.r[11].s64 + 13300;
	// 8304C300: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304C308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304C308 size=88
    let mut pc: u32 = 0x8304C308;
    'dispatch: loop {
        match pc {
            0x8304C308 => {
    //   block [0x8304C308..0x8304C360)
	// 8304C308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304C30C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304C310: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8304C314: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8304C318: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304C31C: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8304C320: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8304C324: 396B2828  addi r11, r11, 0x2828
	ctx.r[11].s64 = ctx.r[11].s64 + 10280;
	// 8304C328: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8304C32C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8304C330: 4803E4A1  bl 0x8308a7d0
	ctx.lr = 0x8304C334;
	sub_8308A7D0(ctx, base);
	// 8304C334: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304C338: 4182000C  beq 0x8304c344
	if ctx.cr[0].eq {
	pc = 0x8304C344; continue 'dispatch;
	}
	// 8304C33C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8304C340: 4BF8BFA1  bl 0x82fd82e0
	ctx.lr = 0x8304C344;
	sub_82FD82E0(ctx, base);
	// 8304C344: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8304C348: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8304C34C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304C350: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304C354: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8304C358: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8304C35C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304C360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304C360 size=136
    let mut pc: u32 = 0x8304C360;
    'dispatch: loop {
        match pc {
            0x8304C360 => {
    //   block [0x8304C360..0x8304C3E8)
	// 8304C360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304C364: 4815BE09  bl 0x831a816c
	ctx.lr = 0x8304C368;
	sub_831A8130(ctx, base);
	// 8304C368: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304C36C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8304C370: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8304C374: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8304C378: 41820050  beq 0x8304c3c8
	if ctx.cr[0].eq {
	pc = 0x8304C3C8; continue 'dispatch;
	}
	// 8304C37C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8304C380: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8304C384: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8304C388: 40990040  ble cr6, 0x8304c3c8
	if !ctx.cr[6].gt {
	pc = 0x8304C3C8; continue 'dispatch;
	}
	// 8304C38C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8304C390: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8304C394: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 8304C398: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8304C39C: 41820018  beq 0x8304c3b4
	if ctx.cr[0].eq {
	pc = 0x8304C3B4; continue 'dispatch;
	}
	// 8304C3A0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304C3A4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8304C3A8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304C3AC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8304C3B0: 4E800421  bctrl
	ctx.lr = 0x8304C3B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8304C3B4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8304C3B8: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 8304C3BC: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 8304C3C0: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8304C3C4: 4198FFCC  blt cr6, 0x8304c390
	if ctx.cr[6].lt {
	pc = 0x8304C390; continue 'dispatch;
	}
	// 8304C3C8: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8304C3CC: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8304C3D0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304C3D4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8304C3D8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8304C3DC: 4E800421  bctrl
	ctx.lr = 0x8304C3E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8304C3E0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8304C3E4: 4815BDD8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304C3E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304C3E8 size=128
    let mut pc: u32 = 0x8304C3E8;
    'dispatch: loop {
        match pc {
            0x8304C3E8 => {
    //   block [0x8304C3E8..0x8304C468)
	// 8304C3E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304C3EC: 4815BD7D  bl 0x831a8168
	ctx.lr = 0x8304C3F0;
	sub_831A8130(ctx, base);
	// 8304C3F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304C3F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8304C3F8: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8304C3FC: 7F9DE378  mr r29, r28
	ctx.r[29].u64 = ctx.r[28].u64;
	// 8304C400: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8304C404: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8304C408: 40990054  ble cr6, 0x8304c45c
	if !ctx.cr[6].gt {
	pc = 0x8304C45C; continue 'dispatch;
	}
	// 8304C40C: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 8304C410: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8304C414: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8304C418: 41820028  beq 0x8304c440
	if ctx.cr[0].eq {
	pc = 0x8304C440; continue 'dispatch;
	}
	// 8304C41C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8304C420: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 8304C424: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8304C428: 41820018  beq 0x8304c440
	if ctx.cr[0].eq {
	pc = 0x8304C440; continue 'dispatch;
	}
	// 8304C42C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304C430: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8304C434: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304C438: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8304C43C: 4E800421  bctrl
	ctx.lr = 0x8304C440;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8304C440: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8304C444: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 8304C448: 7F8BF12E  stwx r28, r11, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32), ctx.r[28].u32) };
	// 8304C44C: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 8304C450: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8304C454: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8304C458: 4198FFB8  blt cr6, 0x8304c410
	if ctx.cr[6].lt {
	pc = 0x8304C410; continue 'dispatch;
	}
	// 8304C45C: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 8304C460: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8304C464: 4815BD54  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304C468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8304C468 size=16
    let mut pc: u32 = 0x8304C468;
    'dispatch: loop {
        match pc {
            0x8304C468 => {
    //   block [0x8304C468..0x8304C478)
	// 8304C468: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8304C46C: 396B2978  addi r11, r11, 0x2978
	ctx.r[11].s64 = ctx.r[11].s64 + 10616;
	// 8304C470: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8304C474: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304C478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304C478 size=152
    let mut pc: u32 = 0x8304C478;
    'dispatch: loop {
        match pc {
            0x8304C478 => {
    //   block [0x8304C478..0x8304C510)
	// 8304C478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304C47C: 4815BCF1  bl 0x831a816c
	ctx.lr = 0x8304C480;
	sub_831A8130(ctx, base);
	// 8304C480: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304C484: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8304C488: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8304C48C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8304C490: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8304C494: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8304C498: 41980030  blt cr6, 0x8304c4c8
	if ctx.cr[6].lt {
	pc = 0x8304C4C8; continue 'dispatch;
	}
	// 8304C49C: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 8304C4A0: 80FF0014  lwz r7, 0x14(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8304C4A4: 38C00074  li r6, 0x74
	ctx.r[6].s64 = 116;
	// 8304C4A8: 388B6B80  addi r4, r11, 0x6b80
	ctx.r[4].s64 = ctx.r[11].s64 + 27520;
	// 8304C4AC: 38A00043  li r5, 0x43
	ctx.r[5].s64 = 67;
	// 8304C4B0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8304C4B4: 4BF844A5  bl 0x82fd0958
	ctx.lr = 0x8304C4B8;
	sub_82FD0958(ctx, base);
	// 8304C4B8: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8304C4BC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8304C4C0: 388BC49C  addi r4, r11, -0x3b64
	ctx.r[4].s64 = ctx.r[11].s64 + -15204;
	// 8304C4C4: 48164765  bl 0x831b0c28
	ctx.lr = 0x8304C4C8;
	sub_831B0C28(ctx, base);
	// 8304C4C8: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8304C4CC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8304C4D0: 4182002C  beq 0x8304c4fc
	if ctx.cr[0].eq {
	pc = 0x8304C4FC; continue 'dispatch;
	}
	// 8304C4D4: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8304C4D8: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8304C4DC: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8304C4E0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8304C4E4: 41820018  beq 0x8304c4fc
	if ctx.cr[0].eq {
	pc = 0x8304C4FC; continue 'dispatch;
	}
	// 8304C4E8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304C4EC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8304C4F0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304C4F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8304C4F8: 4E800421  bctrl
	ctx.lr = 0x8304C4FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8304C4FC: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8304C500: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8304C504: 7FAA592E  stwx r29, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[29].u32) };
	// 8304C508: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8304C50C: 4815BCB0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304C510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304C510 size=288
    let mut pc: u32 = 0x8304C510;
    'dispatch: loop {
        match pc {
            0x8304C510 => {
    //   block [0x8304C510..0x8304C630)
	// 8304C510: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304C514: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304C518: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8304C51C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8304C520: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304C524: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8304C528: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8304C52C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8304C530: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8304C534: 41980030  blt cr6, 0x8304c564
	if ctx.cr[6].lt {
	pc = 0x8304C564; continue 'dispatch;
	}
	// 8304C538: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 8304C53C: 80FF0014  lwz r7, 0x14(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8304C540: 38C00074  li r6, 0x74
	ctx.r[6].s64 = 116;
	// 8304C544: 388B6B80  addi r4, r11, 0x6b80
	ctx.r[4].s64 = ctx.r[11].s64 + 27520;
	// 8304C548: 38A00090  li r5, 0x90
	ctx.r[5].s64 = 144;
	// 8304C54C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8304C550: 4BF84409  bl 0x82fd0958
	ctx.lr = 0x8304C554;
	sub_82FD0958(ctx, base);
	// 8304C554: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8304C558: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8304C55C: 388BC49C  addi r4, r11, -0x3b64
	ctx.r[4].s64 = ctx.r[11].s64 + -15204;
	// 8304C560: 481646C9  bl 0x831b0c28
	ctx.lr = 0x8304C564;
	sub_831B0C28(ctx, base);
	// 8304C564: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8304C568: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8304C56C: 4182002C  beq 0x8304c598
	if ctx.cr[0].eq {
	pc = 0x8304C598; continue 'dispatch;
	}
	// 8304C570: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8304C574: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8304C578: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8304C57C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8304C580: 41820018  beq 0x8304c598
	if ctx.cr[0].eq {
	pc = 0x8304C598; continue 'dispatch;
	}
	// 8304C584: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304C588: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8304C58C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304C590: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8304C594: 4E800421  bctrl
	ctx.lr = 0x8304C598;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8304C598: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8304C59C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8304C5A0: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8304C5A4: 409A0018  bne cr6, 0x8304c5bc
	if !ctx.cr[6].eq {
	pc = 0x8304C5BC; continue 'dispatch;
	}
	// 8304C5A8: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8304C5AC: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8304C5B0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8304C5B4: 7D2A592E  stwx r9, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u32) };
	// 8304C5B8: 48000054  b 0x8304c60c
	pc = 0x8304C60C; continue 'dispatch;
	// 8304C5BC: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 8304C5C0: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8304C5C4: 40980030  bge cr6, 0x8304c5f4
	if !ctx.cr[6].lt {
	pc = 0x8304C5F4; continue 'dispatch;
	}
	// 8304C5C8: 57CB103A  slwi r11, r30, 2
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8304C5CC: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8304C5D0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8304C5D4: 7D295A14  add r9, r9, r11
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 8304C5D8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8304C5DC: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 8304C5E0: 91090000  stw r8, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8304C5E4: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8304C5E8: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 8304C5EC: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8304C5F0: 4198FFDC  blt cr6, 0x8304c5cc
	if ctx.cr[6].lt {
	pc = 0x8304C5CC; continue 'dispatch;
	}
	// 8304C5F4: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8304C5F8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8304C5FC: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8304C600: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8304C604: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8304C608: 912BFFFC  stw r9, -4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), ctx.r[9].u32 ) };
	// 8304C60C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8304C610: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8304C614: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8304C618: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8304C61C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304C620: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304C624: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8304C628: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8304C62C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304C630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304C630 size=268
    let mut pc: u32 = 0x8304C630;
    'dispatch: loop {
        match pc {
            0x8304C630 => {
    //   block [0x8304C630..0x8304C73C)
	// 8304C630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304C634: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304C638: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8304C63C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8304C640: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304C644: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8304C648: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8304C64C: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 8304C650: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 8304C654: 90BF000C  stw r5, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[5].u32 ) };
	// 8304C658: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 8304C65C: 90FF0000  stw r7, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 8304C660: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 8304C664: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 8304C668: 909F0014  stw r4, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[4].u32 ) };
	// 8304C66C: 409A002C  bne cr6, 0x8304c698
	if !ctx.cr[6].eq {
	pc = 0x8304C698; continue 'dispatch;
	}
	// 8304C670: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8304C674: 38C00044  li r6, 0x44
	ctx.r[6].s64 = 68;
	// 8304C678: 388B23A0  addi r4, r11, 0x23a0
	ctx.r[4].s64 = ctx.r[11].s64 + 9120;
	// 8304C67C: 38A0006C  li r5, 0x6c
	ctx.r[5].s64 = 108;
	// 8304C680: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8304C684: 4BF84645  bl 0x82fd0cc8
	ctx.lr = 0x8304C688;
	sub_82FD0CC8(ctx, base);
	// 8304C688: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8304C68C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8304C690: 388BC3FC  addi r4, r11, -0x3c04
	ctx.r[4].s64 = ctx.r[11].s64 + -15364;
	// 8304C694: 48164595  bl 0x831b0c28
	ctx.lr = 0x8304C698;
	sub_831B0C28(ctx, base);
	// 8304C698: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304C69C: 5484103A  slwi r4, r4, 2
	ctx.r[4].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 8304C6A0: 7CE33B78  mr r3, r7
	ctx.r[3].u64 = ctx.r[7].u64;
	// 8304C6A4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8304C6A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8304C6AC: 4E800421  bctrl
	ctx.lr = 0x8304C6B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8304C6B0: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8304C6B4: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 8304C6B8: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 8304C6BC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8304C6C0: 40990024  ble cr6, 0x8304c6e4
	if !ctx.cr[6].gt {
	pc = 0x8304C6E4; continue 'dispatch;
	}
	// 8304C6C4: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 8304C6C8: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8304C6CC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8304C6D0: 7FC9512E  stwx r30, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 8304C6D4: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8304C6D8: 813F0014  lwz r9, 0x14(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8304C6DC: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8304C6E0: 4198FFE8  blt cr6, 0x8304c6c8
	if ctx.cr[6].lt {
	pc = 0x8304C6C8; continue 'dispatch;
	}
	// 8304C6E4: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8304C6E8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8304C6EC: 409A000C  bne cr6, 0x8304c6f8
	if !ctx.cr[6].eq {
	pc = 0x8304C6F8; continue 'dispatch;
	}
	// 8304C6F0: 39600100  li r11, 0x100
	ctx.r[11].s64 = 256;
	// 8304C6F4: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8304C6F8: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304C6FC: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8304C700: 5564103A  slwi r4, r11, 2
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 8304C704: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304C708: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8304C70C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8304C710: 4E800421  bctrl
	ctx.lr = 0x8304C714;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8304C714: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8304C718: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8304C71C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8304C720: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 8304C724: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8304C728: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304C72C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304C730: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8304C734: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8304C738: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304C740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8304C740 size=28
    let mut pc: u32 = 0x8304C740;
    'dispatch: loop {
        match pc {
            0x8304C740 => {
    //   block [0x8304C740..0x8304C75C)
	// 8304C740: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8304C744: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8304C748: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8304C74C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8304C750: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 8304C754: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8304C758: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304C760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8304C760 size=16
    let mut pc: u32 = 0x8304C760;
    'dispatch: loop {
        match pc {
            0x8304C760 => {
    //   block [0x8304C760..0x8304C770)
	// 8304C760: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8304C764: 396BA93C  addi r11, r11, -0x56c4
	ctx.r[11].s64 = ctx.r[11].s64 + -22212;
	// 8304C768: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8304C76C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304C770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304C770 size=124
    let mut pc: u32 = 0x8304C770;
    'dispatch: loop {
        match pc {
            0x8304C770 => {
    //   block [0x8304C770..0x8304C7EC)
	// 8304C770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304C774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304C778: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304C77C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8304C780: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8304C784: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8304C788: 41820038  beq 0x8304c7c0
	if ctx.cr[0].eq {
	pc = 0x8304C7C0; continue 'dispatch;
	}
	// 8304C78C: 812B0008  lwz r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8304C790: 81090010  lwz r8, 0x10(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(16 as u32) ) } as u64;
	// 8304C794: 7F0A4040  cmplw cr6, r10, r8
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[8].u32, &mut ctx.xer);
	// 8304C798: 41990028  bgt cr6, 0x8304c7c0
	if ctx.cr[6].gt {
	pc = 0x8304C7C0; continue 'dispatch;
	}
	// 8304C79C: 81290008  lwz r9, 8(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 8304C7A0: 5548103A  slwi r8, r10, 2
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8304C7A4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8304C7A8: 7C69402E  lwzx r3, r9, r8
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 8304C7AC: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8304C7B0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8304C7B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304C7B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304C7BC: 4E800020  blr
	return;
	// 8304C7C0: 3D408214  lis r10, -0x7dec
	ctx.r[10].s64 = -2112618496;
	// 8304C7C4: 80EB000C  lwz r7, 0xc(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8304C7C8: 38C0001E  li r6, 0x1e
	ctx.r[6].s64 = 30;
	// 8304C7CC: 388A23A0  addi r4, r10, 0x23a0
	ctx.r[4].s64 = ctx.r[10].s64 + 9120;
	// 8304C7D0: 38A00191  li r5, 0x191
	ctx.r[5].s64 = 401;
	// 8304C7D4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8304C7D8: 4BF94551  bl 0x82fe0d28
	ctx.lr = 0x8304C7DC;
	sub_82FE0D28(ctx, base);
	// 8304C7DC: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8304C7E0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8304C7E4: 388BC690  addi r4, r11, -0x3970
	ctx.r[4].s64 = ctx.r[11].s64 + -14704;
	// 8304C7E8: 48164441  bl 0x831b0c28
	ctx.lr = 0x8304C7EC;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304C7F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304C7F0 size=128
    let mut pc: u32 = 0x8304C7F0;
    'dispatch: loop {
        match pc {
            0x8304C7F0 => {
    //   block [0x8304C7F0..0x8304C870)
	// 8304C7F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304C7F4: 4815B979  bl 0x831a816c
	ctx.lr = 0x8304C7F8;
	sub_831A8130(ctx, base);
	// 8304C7F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304C7FC: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8304C800: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8304C804: 396B2978  addi r11, r11, 0x2978
	ctx.r[11].s64 = ctx.r[11].s64 + 10616;
	// 8304C808: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8304C80C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8304C810: 57C4103A  slwi r4, r30, 2
	ctx.r[4].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 8304C814: 98BF0004  stb r5, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[5].u8 ) };
	// 8304C818: 7CC33378  mr r3, r6
	ctx.r[3].u64 = ctx.r[6].u64;
	// 8304C81C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8304C820: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 8304C824: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 8304C828: 90DF0014  stw r6, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[6].u32 ) };
	// 8304C82C: 93BF0010  stw r29, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[29].u32 ) };
	// 8304C830: 81660000  lwz r11, 0(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304C834: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8304C838: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8304C83C: 4E800421  bctrl
	ctx.lr = 0x8304C840;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8304C840: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8304C844: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 8304C848: 419A001C  beq cr6, 0x8304c864
	if ctx.cr[6].eq {
	pc = 0x8304C864; continue 'dispatch;
	}
	// 8304C84C: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 8304C850: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8304C854: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8304C858: 7FAA592E  stwx r29, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[29].u32) };
	// 8304C85C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8304C860: 4082FFF0  bne 0x8304c850
	if !ctx.cr[0].eq {
	pc = 0x8304C850; continue 'dispatch;
	}
	// 8304C864: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8304C868: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8304C86C: 4815B950  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304C870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304C870 size=176
    let mut pc: u32 = 0x8304C870;
    'dispatch: loop {
        match pc {
            0x8304C870 => {
    //   block [0x8304C870..0x8304C920)
	// 8304C870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304C874: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304C878: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8304C87C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304C880: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8304C884: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 8304C888: 409A0030  bne cr6, 0x8304c8b8
	if !ctx.cr[6].eq {
	pc = 0x8304C8B8; continue 'dispatch;
	}
	// 8304C88C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8304C890: 80FF0000  lwz r7, 0(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304C894: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 8304C898: 388BB9A0  addi r4, r11, -0x4660
	ctx.r[4].s64 = ctx.r[11].s64 + -18016;
	// 8304C89C: 38A000B6  li r5, 0xb6
	ctx.r[5].s64 = 182;
	// 8304C8A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8304C8A4: 4BF84425  bl 0x82fd0cc8
	ctx.lr = 0x8304C8A8;
	sub_82FD0CC8(ctx, base);
	// 8304C8A8: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8304C8AC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8304C8B0: 388BC3FC  addi r4, r11, -0x3c04
	ctx.r[4].s64 = ctx.r[11].s64 + -15364;
	// 8304C8B4: 48164375  bl 0x831b0c28
	ctx.lr = 0x8304C8B8;
	sub_831B0C28(ctx, base);
	// 8304C8B8: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304C8BC: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8304C8C0: 5544103A  slwi r4, r10, 2
	ctx.r[4].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 8304C8C4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304C8C8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8304C8CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8304C8D0: 4E800421  bctrl
	ctx.lr = 0x8304C8D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8304C8D4: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8304C8D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8304C8DC: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 8304C8E0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8304C8E4: 40990028  ble cr6, 0x8304c90c
	if !ctx.cr[6].gt {
	pc = 0x8304C90C; continue 'dispatch;
	}
	// 8304C8E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8304C8EC: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8304C8F0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8304C8F4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8304C8F8: 7D0A492E  stwx r8, r10, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32), ctx.r[8].u32) };
	// 8304C8FC: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8304C900: 813F000C  lwz r9, 0xc(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8304C904: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8304C908: 4198FFE4  blt cr6, 0x8304c8ec
	if ctx.cr[6].lt {
	pc = 0x8304C8EC; continue 'dispatch;
	}
	// 8304C90C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8304C910: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304C914: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304C918: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8304C91C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304C920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304C920 size=172
    let mut pc: u32 = 0x8304C920;
    'dispatch: loop {
        match pc {
            0x8304C920 => {
    //   block [0x8304C920..0x8304C9CC)
	// 8304C920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304C924: 4815B849  bl 0x831a816c
	ctx.lr = 0x8304C928;
	sub_831A8130(ctx, base);
	// 8304C928: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304C92C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8304C930: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8304C934: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8304C938: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304C93C: 80BF0000  lwz r5, 0(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304C940: 809F0014  lwz r4, 0x14(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8304C944: 4BF852BD  bl 0x82fd1c00
	ctx.lr = 0x8304C948;
	sub_82FD1C00(ctx, base);
	// 8304C948: 907E0000  stw r3, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8304C94C: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8304C950: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8304C954: 40990030  ble cr6, 0x8304c984
	if !ctx.cr[6].gt {
	pc = 0x8304C984; continue 'dispatch;
	}
	// 8304C958: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8304C95C: 80FF0000  lwz r7, 0(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304C960: 38C00042  li r6, 0x42
	ctx.r[6].s64 = 66;
	// 8304C964: 388B23A0  addi r4, r11, 0x23a0
	ctx.r[4].s64 = ctx.r[11].s64 + 9120;
	// 8304C968: 38A00130  li r5, 0x130
	ctx.r[5].s64 = 304;
	// 8304C96C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8304C970: 4BF846E9  bl 0x82fd1058
	ctx.lr = 0x8304C974;
	sub_82FD1058(ctx, base);
	// 8304C974: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8304C978: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8304C97C: 388BC3EC  addi r4, r11, -0x3c14
	ctx.r[4].s64 = ctx.r[11].s64 + -15380;
	// 8304C980: 481642A9  bl 0x831b0c28
	ctx.lr = 0x8304C984;
	sub_831B0C28(ctx, base);
	// 8304C984: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8304C988: 546A103A  slwi r10, r3, 2
	ctx.r[10].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8304C98C: 7FEA582E  lwzx r31, r10, r11
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8304C990: 48000020  b 0x8304c9b0
	pc = 0x8304C9B0; continue 'dispatch;
	// 8304C994: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304C998: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304C99C: 808B0008  lwz r4, 8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8304C9A0: 4BF872A1  bl 0x82fd3c40
	ctx.lr = 0x8304C9A4;
	sub_82FD3C40(ctx, base);
	// 8304C9A4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304C9A8: 4082001C  bne 0x8304c9c4
	if !ctx.cr[0].eq {
	pc = 0x8304C9C4; continue 'dispatch;
	}
	// 8304C9AC: 83FF0004  lwz r31, 4(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8304C9B0: 281F0000  cmplwi r31, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8304C9B4: 4082FFE0  bne 0x8304c994
	if !ctx.cr[0].eq {
	pc = 0x8304C994; continue 'dispatch;
	}
	// 8304C9B8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8304C9BC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8304C9C0: 4815B7FC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 8304C9C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8304C9C8: 4BFFFFF4  b 0x8304c9bc
	pc = 0x8304C9BC; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304C9D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304C9D0 size=116
    let mut pc: u32 = 0x8304C9D0;
    'dispatch: loop {
        match pc {
            0x8304C9D0 => {
    //   block [0x8304C9D0..0x8304CA44)
	// 8304C9D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304C9D4: 4815B795  bl 0x831a8168
	ctx.lr = 0x8304C9D8;
	sub_831A8130(ctx, base);
	// 8304C9D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304C9DC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8304C9E0: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8304C9E4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8304C9E8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8304C9EC: 4BFACCAD  bl 0x82ff9698
	ctx.lr = 0x8304C9F0;
	sub_82FF9698(ctx, base);
	// 8304C9F0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304C9F4: 41820048  beq 0x8304ca3c
	if ctx.cr[0].eq {
	pc = 0x8304CA3C; continue 'dispatch;
	}
	// 8304C9F8: 83DD0004  lwz r30, 4(r29)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 8304C9FC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8304CA00: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8304CA04: 4BFAC8F5  bl 0x82ff92f8
	ctx.lr = 0x8304CA08;
	sub_82FF92F8(ctx, base);
	// 8304CA08: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8304CA0C: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8304CA10: 4099002C  ble cr6, 0x8304ca3c
	if !ctx.cr[6].gt {
	pc = 0x8304CA3C; continue 'dispatch;
	}
	// 8304CA14: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8304CA18: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304CA1C: 4BFAB6B5  bl 0x82ff80d0
	ctx.lr = 0x8304CA20;
	sub_82FF80D0(ctx, base);
	// 8304CA20: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8304CA24: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8304CA28: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304CA2C: 4BFAD1D5  bl 0x82ff9c00
	ctx.lr = 0x8304CA30;
	sub_82FF9C00(ctx, base);
	// 8304CA30: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 8304CA34: 7F1FF000  cmpw cr6, r31, r30
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[30].s32, &mut ctx.xer);
	// 8304CA38: 4198FFDC  blt cr6, 0x8304ca14
	if ctx.cr[6].lt {
	pc = 0x8304CA14; continue 'dispatch;
	}
	// 8304CA3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8304CA40: 4815B778  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304CA48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304CA48 size=116
    let mut pc: u32 = 0x8304CA48;
    'dispatch: loop {
        match pc {
            0x8304CA48 => {
    //   block [0x8304CA48..0x8304CABC)
	// 8304CA48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304CA4C: 4815B71D  bl 0x831a8168
	ctx.lr = 0x8304CA50;
	sub_831A8130(ctx, base);
	// 8304CA50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304CA54: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8304CA58: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8304CA5C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8304CA60: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8304CA64: 4BFACC35  bl 0x82ff9698
	ctx.lr = 0x8304CA68;
	sub_82FF9698(ctx, base);
	// 8304CA68: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304CA6C: 41820048  beq 0x8304cab4
	if ctx.cr[0].eq {
	pc = 0x8304CAB4; continue 'dispatch;
	}
	// 8304CA70: 83DD0004  lwz r30, 4(r29)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 8304CA74: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8304CA78: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8304CA7C: 4BFAC87D  bl 0x82ff92f8
	ctx.lr = 0x8304CA80;
	sub_82FF92F8(ctx, base);
	// 8304CA80: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8304CA84: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8304CA88: 4099002C  ble cr6, 0x8304cab4
	if !ctx.cr[6].gt {
	pc = 0x8304CAB4; continue 'dispatch;
	}
	// 8304CA8C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8304CA90: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304CA94: 4BFAB63D  bl 0x82ff80d0
	ctx.lr = 0x8304CA98;
	sub_82FF80D0(ctx, base);
	// 8304CA98: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8304CA9C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8304CAA0: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304CAA4: 4BFAC855  bl 0x82ff92f8
	ctx.lr = 0x8304CAA8;
	sub_82FF92F8(ctx, base);
	// 8304CAA8: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 8304CAAC: 7F1FF000  cmpw cr6, r31, r30
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[30].s32, &mut ctx.xer);
	// 8304CAB0: 4198FFDC  blt cr6, 0x8304ca8c
	if ctx.cr[6].lt {
	pc = 0x8304CA8C; continue 'dispatch;
	}
	// 8304CAB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8304CAB8: 4815B700  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304CAC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304CAC0 size=120
    let mut pc: u32 = 0x8304CAC0;
    'dispatch: loop {
        match pc {
            0x8304CAC0 => {
    //   block [0x8304CAC0..0x8304CB38)
	// 8304CAC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304CAC4: 4815B6A5  bl 0x831a8168
	ctx.lr = 0x8304CAC8;
	sub_831A8130(ctx, base);
	// 8304CAC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304CACC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8304CAD0: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8304CAD4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8304CAD8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8304CADC: 4BFACBBD  bl 0x82ff9698
	ctx.lr = 0x8304CAE0;
	sub_82FF9698(ctx, base);
	// 8304CAE0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304CAE4: 4182004C  beq 0x8304cb30
	if ctx.cr[0].eq {
	pc = 0x8304CB30; continue 'dispatch;
	}
	// 8304CAE8: 83DD0008  lwz r30, 8(r29)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 8304CAEC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8304CAF0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8304CAF4: 4BFAC805  bl 0x82ff92f8
	ctx.lr = 0x8304CAF8;
	sub_82FF92F8(ctx, base);
	// 8304CAF8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8304CAFC: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8304CB00: 40990030  ble cr6, 0x8304cb30
	if !ctx.cr[6].gt {
	pc = 0x8304CB30; continue 'dispatch;
	}
	// 8304CB04: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8304CB08: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304CB0C: 4BFDFD65  bl 0x8302c870
	ctx.lr = 0x8304CB10;
	sub_8302C870(ctx, base);
	// 8304CB10: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8304CB14: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8304CB18: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8304CB1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8304CB20: 4BFACDE1  bl 0x82ff9900
	ctx.lr = 0x8304CB24;
	sub_82FF9900(ctx, base);
	// 8304CB24: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 8304CB28: 7F1FF000  cmpw cr6, r31, r30
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[30].s32, &mut ctx.xer);
	// 8304CB2C: 4198FFD8  blt cr6, 0x8304cb04
	if ctx.cr[6].lt {
	pc = 0x8304CB04; continue 'dispatch;
	}
	// 8304CB30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8304CB34: 4815B684  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304CB38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304CB38 size=112
    let mut pc: u32 = 0x8304CB38;
    'dispatch: loop {
        match pc {
            0x8304CB38 => {
    //   block [0x8304CB38..0x8304CBA8)
	// 8304CB38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304CB3C: 4815B62D  bl 0x831a8168
	ctx.lr = 0x8304CB40;
	sub_831A8130(ctx, base);
	// 8304CB40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304CB44: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8304CB48: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8304CB4C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8304CB50: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8304CB54: 4BFACB45  bl 0x82ff9698
	ctx.lr = 0x8304CB58;
	sub_82FF9698(ctx, base);
	// 8304CB58: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304CB5C: 41820044  beq 0x8304cba0
	if ctx.cr[0].eq {
	pc = 0x8304CBA0; continue 'dispatch;
	}
	// 8304CB60: 83DD0008  lwz r30, 8(r29)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 8304CB64: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8304CB68: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8304CB6C: 4BFAC78D  bl 0x82ff92f8
	ctx.lr = 0x8304CB70;
	sub_82FF92F8(ctx, base);
	// 8304CB70: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8304CB74: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8304CB78: 40990028  ble cr6, 0x8304cba0
	if !ctx.cr[6].gt {
	pc = 0x8304CBA0; continue 'dispatch;
	}
	// 8304CB7C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8304CB80: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304CB84: 4BFDFCED  bl 0x8302c870
	ctx.lr = 0x8304CB88;
	sub_8302C870(ctx, base);
	// 8304CB88: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8304CB8C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8304CB90: 4BFAD071  bl 0x82ff9c00
	ctx.lr = 0x8304CB94;
	sub_82FF9C00(ctx, base);
	// 8304CB94: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 8304CB98: 7F1FF000  cmpw cr6, r31, r30
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[30].s32, &mut ctx.xer);
	// 8304CB9C: 4198FFE0  blt cr6, 0x8304cb7c
	if ctx.cr[6].lt {
	pc = 0x8304CB7C; continue 'dispatch;
	}
	// 8304CBA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8304CBA4: 4815B614  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304CBA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304CBA8 size=112
    let mut pc: u32 = 0x8304CBA8;
    'dispatch: loop {
        match pc {
            0x8304CBA8 => {
    //   block [0x8304CBA8..0x8304CC18)
	// 8304CBA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304CBAC: 4815B5BD  bl 0x831a8168
	ctx.lr = 0x8304CBB0;
	sub_831A8130(ctx, base);
	// 8304CBB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304CBB4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8304CBB8: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8304CBBC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8304CBC0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8304CBC4: 4BFACAD5  bl 0x82ff9698
	ctx.lr = 0x8304CBC8;
	sub_82FF9698(ctx, base);
	// 8304CBC8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304CBCC: 41820044  beq 0x8304cc10
	if ctx.cr[0].eq {
	pc = 0x8304CC10; continue 'dispatch;
	}
	// 8304CBD0: 83DD0008  lwz r30, 8(r29)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 8304CBD4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8304CBD8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8304CBDC: 4BFAC71D  bl 0x82ff92f8
	ctx.lr = 0x8304CBE0;
	sub_82FF92F8(ctx, base);
	// 8304CBE0: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8304CBE4: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8304CBE8: 40990028  ble cr6, 0x8304cc10
	if !ctx.cr[6].gt {
	pc = 0x8304CC10; continue 'dispatch;
	}
	// 8304CBEC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8304CBF0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304CBF4: 4BFDFC7D  bl 0x8302c870
	ctx.lr = 0x8304CBF8;
	sub_8302C870(ctx, base);
	// 8304CBF8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8304CBFC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8304CC00: 4BFF4669  bl 0x83041268
	ctx.lr = 0x8304CC04;
	sub_83041268(ctx, base);
	// 8304CC04: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 8304CC08: 7F1FF000  cmpw cr6, r31, r30
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[30].s32, &mut ctx.xer);
	// 8304CC0C: 4198FFE0  blt cr6, 0x8304cbec
	if ctx.cr[6].lt {
	pc = 0x8304CBEC; continue 'dispatch;
	}
	// 8304CC10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8304CC14: 4815B5A4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304CC18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304CC18 size=112
    let mut pc: u32 = 0x8304CC18;
    'dispatch: loop {
        match pc {
            0x8304CC18 => {
    //   block [0x8304CC18..0x8304CC88)
	// 8304CC18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304CC1C: 4815B54D  bl 0x831a8168
	ctx.lr = 0x8304CC20;
	sub_831A8130(ctx, base);
	// 8304CC20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304CC24: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8304CC28: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8304CC2C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8304CC30: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8304CC34: 4BFACA65  bl 0x82ff9698
	ctx.lr = 0x8304CC38;
	sub_82FF9698(ctx, base);
	// 8304CC38: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304CC3C: 41820044  beq 0x8304cc80
	if ctx.cr[0].eq {
	pc = 0x8304CC80; continue 'dispatch;
	}
	// 8304CC40: 83DD0008  lwz r30, 8(r29)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 8304CC44: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8304CC48: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8304CC4C: 4BFAC6AD  bl 0x82ff92f8
	ctx.lr = 0x8304CC50;
	sub_82FF92F8(ctx, base);
	// 8304CC50: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8304CC54: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8304CC58: 40990028  ble cr6, 0x8304cc80
	if !ctx.cr[6].gt {
	pc = 0x8304CC80; continue 'dispatch;
	}
	// 8304CC5C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8304CC60: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304CC64: 4BFDFC0D  bl 0x8302c870
	ctx.lr = 0x8304CC68;
	sub_8302C870(ctx, base);
	// 8304CC68: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8304CC6C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8304CC70: 4804C431  bl 0x830990a0
	ctx.lr = 0x8304CC74;
	sub_830990A0(ctx, base);
	// 8304CC74: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 8304CC78: 7F1FF000  cmpw cr6, r31, r30
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[30].s32, &mut ctx.xer);
	// 8304CC7C: 4198FFE0  blt cr6, 0x8304cc5c
	if ctx.cr[6].lt {
	pc = 0x8304CC5C; continue 'dispatch;
	}
	// 8304CC80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8304CC84: 4815B534  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304CC88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8304CC88 size=348
    let mut pc: u32 = 0x8304CC88;
    'dispatch: loop {
        match pc {
            0x8304CC88 => {
    //   block [0x8304CC88..0x8304CDE4)
	// 8304CC88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304CC8C: 4815B4DD  bl 0x831a8168
	ctx.lr = 0x8304CC90;
	sub_831A8130(ctx, base);
	// 8304CC90: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304CC94: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8304CC98: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 8304CC9C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8304CCA0: 809C0010  lwz r4, 0x10(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(16 as u32) ) } as u64;
	// 8304CCA4: 4BFD2545  bl 0x8301f1e8
	ctx.lr = 0x8304CCA8;
	sub_8301F1E8(ctx, base);
	// 8304CCA8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8304CCAC: 41820044  beq 0x8304ccf0
	if ctx.cr[0].eq {
	pc = 0x8304CCF0; continue 'dispatch;
	}
	// 8304CCB0: 80BF0000  lwz r5, 0(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304CCB4: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8304CCB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8304CCBC: 80FC0010  lwz r7, 0x10(r28)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(16 as u32) ) } as u64;
	// 8304CCC0: 388B23A0  addi r4, r11, 0x23a0
	ctx.r[4].s64 = ctx.r[11].s64 + 9120;
	// 8304CCC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8304CCC8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8304CCCC: 90A10054  stw r5, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[5].u32 ) };
	// 8304CCD0: 38C00041  li r6, 0x41
	ctx.r[6].s64 = 65;
	// 8304CCD4: 38A000FA  li r5, 0xfa
	ctx.r[5].s64 = 250;
	// 8304CCD8: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8304CCDC: 4BFCAA0D  bl 0x830176e8
	ctx.lr = 0x8304CCE0;
	sub_830176E8(ctx, base);
	// 8304CCE0: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8304CCE4: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8304CCE8: 388BC3FC  addi r4, r11, -0x3c04
	ctx.r[4].s64 = ctx.r[11].s64 + -15364;
	// 8304CCEC: 48163F3D  bl 0x831b0c28
	ctx.lr = 0x8304CCF0;
	sub_831B0C28(ctx, base);
	// 8304CCF0: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 8304CCF4: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304CCF8: 4BF8B5A1  bl 0x82fd8298
	ctx.lr = 0x8304CCFC;
	sub_82FD8298(ctx, base);
	// 8304CCFC: 81210060  lwz r9, 0x60(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 8304CD00: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8304CD04: 41820020  beq 0x8304cd24
	if ctx.cr[0].eq {
	pc = 0x8304CD24; continue 'dispatch;
	}
	// 8304CD08: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8304CD0C: 5528103A  slwi r8, r9, 2
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8304CD10: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8304CD14: 7D48502E  lwzx r10, r8, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8304CD18: 93830000  stw r28, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 8304CD1C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8304CD20: 48000008  b 0x8304cd28
	pc = 0x8304CD28; continue 'dispatch;
	// 8304CD24: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8304CD28: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8304CD2C: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8304CD30: 7D69512E  stwx r11, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[11].u32) };
	// 8304CD34: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8304CD38: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8304CD3C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8304CD40: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8304CD44: 409A007C  bne cr6, 0x8304cdc0
	if !ctx.cr[6].eq {
	pc = 0x8304CDC0; continue 'dispatch;
	}
	// 8304CD48: 796B0020  clrldi r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64 & 0x00000000FFFFFFFFu64;
	// 8304CD4C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304CD50: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 8304CD54: F9610060  std r11, 0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u64 ) };
	// 8304CD58: C8010060  lfd f0, 0x60(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 8304CD5C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304CD60: FDA0069C  fcfid f13, f0
	ctx.f[13].f64 = (ctx.f[0].s64 as f64);
	// 8304CD64: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8304CD68: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8304CD6C: C80BA8D0  lfd f0, -0x5730(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-22320 as u32) ) };
	// 8304CD70: FC0D0032  fmul f0, f13, f0
	ctx.f[0].f64 = ctx.f[13].f64 * ctx.f[0].f64;
	// 8304CD74: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 8304CD78: 7C0057AE  stfiwx f0, 0, r10
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32, tmp.u32) };
	// 8304CD7C: 83A10060  lwz r29, 0x60(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 8304CD80: 57A4103A  slwi r4, r29, 2
	ctx.r[4].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 8304CD84: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 8304CD88: 4E800421  bctrl
	ctx.lr = 0x8304CD8C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8304CD8C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8304CD90: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8304CD94: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8304CD98: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 8304CD9C: 4815B775  bl 0x831a8510
	ctx.lr = 0x8304CDA0;
	sub_831A8510(ctx, base);
	// 8304CDA0: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304CDA4: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8304CDA8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304CDAC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8304CDB0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8304CDB4: 4E800421  bctrl
	ctx.lr = 0x8304CDB8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8304CDB8: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 8304CDBC: 93BF000C  stw r29, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 8304CDC0: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8304CDC4: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8304CDC8: 386B0001  addi r3, r11, 1
	ctx.r[3].s64 = ctx.r[11].s64 + 1;
	// 8304CDCC: 546B103A  slwi r11, r3, 2
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8304CDD0: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 8304CDD4: 7F8B512E  stwx r28, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[28].u32) };
	// 8304CDD8: 907C0004  stw r3, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 8304CDDC: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 8304CDE0: 4815B3D8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304CDE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8304CDE8 size=8
    let mut pc: u32 = 0x8304CDE8;
    'dispatch: loop {
        match pc {
            0x8304CDE8 => {
    //   block [0x8304CDE8..0x8304CDF0)
	// 8304CDE8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304CDEC: 821629B0  lwz r16, 0x29b0(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(10672 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304CDF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304CDF0 size=168
    let mut pc: u32 = 0x8304CDF0;
    'dispatch: loop {
        match pc {
            0x8304CDF0 => {
    //   block [0x8304CDF0..0x8304CE98)
	// 8304CDF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304CDF4: 4815B375  bl 0x831a8168
	ctx.lr = 0x8304CDF8;
	sub_831A8130(ctx, base);
	// 8304CDF8: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 8304CDFC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304CE00: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8304CE04: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8304CE08: 396B2990  addi r11, r11, 0x2990
	ctx.r[11].s64 = ctx.r[11].s64 + 10640;
	// 8304CE0C: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 8304CE10: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8304CE14: 897E0004  lbz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8304CE18: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8304CE1C: 41820050  beq 0x8304ce6c
	if ctx.cr[0].eq {
	pc = 0x8304CE6C; continue 'dispatch;
	}
	// 8304CE20: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8304CE24: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8304CE28: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8304CE2C: 40990040  ble cr6, 0x8304ce6c
	if !ctx.cr[6].gt {
	pc = 0x8304CE6C; continue 'dispatch;
	}
	// 8304CE30: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8304CE34: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 8304CE38: 7C6BE82E  lwzx r3, r11, r29
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 8304CE3C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8304CE40: 41820018  beq 0x8304ce58
	if ctx.cr[0].eq {
	pc = 0x8304CE58; continue 'dispatch;
	}
	// 8304CE44: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304CE48: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8304CE4C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304CE50: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8304CE54: 4E800421  bctrl
	ctx.lr = 0x8304CE58;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8304CE58: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8304CE5C: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 8304CE60: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 8304CE64: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8304CE68: 4198FFCC  blt cr6, 0x8304ce34
	if ctx.cr[6].lt {
	pc = 0x8304CE34; continue 'dispatch;
	}
	// 8304CE6C: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 8304CE70: 809E0010  lwz r4, 0x10(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 8304CE74: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304CE78: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8304CE7C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8304CE80: 4E800421  bctrl
	ctx.lr = 0x8304CE84;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8304CE84: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8304CE88: 396B2978  addi r11, r11, 0x2978
	ctx.r[11].s64 = ctx.r[11].s64 + 10616;
	// 8304CE8C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8304CE90: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 8304CE94: 4815B324  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304CE98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304CE98 size=40
    let mut pc: u32 = 0x8304CE98;
    'dispatch: loop {
        match pc {
            0x8304CE98 => {
    //   block [0x8304CE98..0x8304CEC0)
	// 8304CE98: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 8304CE9C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304CEA0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304CEA4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304CEA8: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 8304CEAC: 4BFFF5BD  bl 0x8304c468
	ctx.lr = 0x8304CEB0;
	sub_8304C468(ctx, base);
	// 8304CEB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8304CEB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304CEB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304CEBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304CEC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304CEC0 size=152
    let mut pc: u32 = 0x8304CEC0;
    'dispatch: loop {
        match pc {
            0x8304CEC0 => {
    //   block [0x8304CEC0..0x8304CF58)
	// 8304CEC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304CEC4: 4815B2A5  bl 0x831a8168
	ctx.lr = 0x8304CEC8;
	sub_831A8130(ctx, base);
	// 8304CEC8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304CECC: 83E30010  lwz r31, 0x10(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 8304CED0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8304CED4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8304CED8: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8304CEDC: 281F0000  cmplwi r31, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8304CEE0: 4082001C  bne 0x8304cefc
	if !ctx.cr[0].eq {
	pc = 0x8304CEFC; continue 'dispatch;
	}
	// 8304CEE4: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 8304CEE8: 81430014  lwz r10, 0x14(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 8304CEEC: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8304CEF0: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8304CEF4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8304CEF8: 419A0008  beq cr6, 0x8304cf00
	if ctx.cr[6].eq {
	pc = 0x8304CF00; continue 'dispatch;
	}
	// 8304CEFC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8304CF00: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304CF04: 40820030  bne 0x8304cf34
	if !ctx.cr[0].eq {
	pc = 0x8304CF34; continue 'dispatch;
	}
	// 8304CF08: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8304CF0C: 80E30018  lwz r7, 0x18(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 8304CF10: 38C0001E  li r6, 0x1e
	ctx.r[6].s64 = 30;
	// 8304CF14: 388B2368  addi r4, r11, 0x2368
	ctx.r[4].s64 = ctx.r[11].s64 + 9064;
	// 8304CF18: 38A0020B  li r5, 0x20b
	ctx.r[5].s64 = 523;
	// 8304CF1C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8304CF20: 4BF93E09  bl 0x82fe0d28
	ctx.lr = 0x8304CF24;
	sub_82FE0D28(ctx, base);
	// 8304CF24: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8304CF28: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8304CF2C: 388BC690  addi r4, r11, -0x3970
	ctx.r[4].s64 = ctx.r[11].s64 + -14704;
	// 8304CF30: 48163CF9  bl 0x831b0c28
	ctx.lr = 0x8304CF34;
	sub_831B0C28(ctx, base);
	// 8304CF34: 4BFC03DD  bl 0x8300d310
	ctx.lr = 0x8304CF38;
	sub_8300D310(ctx, base);
	// 8304CF38: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8304CF3C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8304CF40: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8304CF44: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8304CF48: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8304CF4C: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8304CF50: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8304CF54: 4815B264  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304CF58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8304CF58 size=8
    let mut pc: u32 = 0x8304CF58;
    'dispatch: loop {
        match pc {
            0x8304CF58 => {
    //   block [0x8304CF58..0x8304CF60)
	// 8304CF58: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304CF5C: 821629E8  lwz r16, 0x29e8(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(10728 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304CF60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304CF60 size=108
    let mut pc: u32 = 0x8304CF60;
    'dispatch: loop {
        match pc {
            0x8304CF60 => {
    //   block [0x8304CF60..0x8304CFCC)
	// 8304CF60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304CF64: 4815B209  bl 0x831a816c
	ctx.lr = 0x8304CF68;
	sub_831A8130(ctx, base);
	// 8304CF68: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 8304CF6C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304CF70: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8304CF74: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8304CF78: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 8304CF7C: 90DE0000  stw r6, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 8304CF80: 98BE0004  stb r5, 4(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[5].u8 ) };
	// 8304CF84: 93BE0008  stw r29, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 8304CF88: 909E000C  stw r4, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 8304CF8C: 909E0010  stw r4, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[4].u32 ) };
	// 8304CF90: 93BE0014  stw r29, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[29].u32 ) };
	// 8304CF94: 93BE0018  stw r29, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[29].u32 ) };
	// 8304CF98: 4BFFF8D9  bl 0x8304c870
	ctx.lr = 0x8304CF9C;
	sub_8304C870(ctx, base);
	// 8304CF9C: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 8304CFA0: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304CFA4: 4BF8B2F5  bl 0x82fd8298
	ctx.lr = 0x8304CFA8;
	sub_82FD8298(ctx, base);
	// 8304CFA8: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8304CFAC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8304CFB0: 4182000C  beq 0x8304cfbc
	if ctx.cr[0].eq {
	pc = 0x8304CFBC; continue 'dispatch;
	}
	// 8304CFB4: 4BFC29AD  bl 0x8300f960
	ctx.lr = 0x8304CFB8;
	sub_8300F960(ctx, base);
	// 8304CFB8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8304CFBC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304CFC0: 93BE0018  stw r29, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[29].u32 ) };
	// 8304CFC4: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 8304CFC8: 4815B1F4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304CFCC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304CFCC size=48
    let mut pc: u32 = 0x8304CFCC;
    'dispatch: loop {
        match pc {
            0x8304CFCC => {
    //   block [0x8304CFCC..0x8304CFFC)
	// 8304CFCC: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 8304CFD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304CFD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304CFD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304CFDC: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 8304CFE0: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304CFE4: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8304CFE8: 4BF8B2F9  bl 0x82fd82e0
	ctx.lr = 0x8304CFEC;
	sub_82FD82E0(ctx, base);
	// 8304CFEC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8304CFF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304CFF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304CFF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304D000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8304D000 size=348
    let mut pc: u32 = 0x8304D000;
    'dispatch: loop {
        match pc {
            0x8304D000 => {
    //   block [0x8304D000..0x8304D15C)
	// 8304D000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304D004: 4815B165  bl 0x831a8168
	ctx.lr = 0x8304D008;
	sub_831A8130(ctx, base);
	// 8304D008: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304D00C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8304D010: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 8304D014: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8304D018: 809C0008  lwz r4, 8(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 8304D01C: 4BFFF905  bl 0x8304c920
	ctx.lr = 0x8304D020;
	sub_8304C920(ctx, base);
	// 8304D020: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8304D024: 41820044  beq 0x8304d068
	if ctx.cr[0].eq {
	pc = 0x8304D068; continue 'dispatch;
	}
	// 8304D028: 80BF0000  lwz r5, 0(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304D02C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8304D030: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8304D034: 80FC0008  lwz r7, 8(r28)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 8304D038: 388B23A0  addi r4, r11, 0x23a0
	ctx.r[4].s64 = ctx.r[11].s64 + 9120;
	// 8304D03C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8304D040: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8304D044: 90A10054  stw r5, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[5].u32 ) };
	// 8304D048: 38C00041  li r6, 0x41
	ctx.r[6].s64 = 65;
	// 8304D04C: 38A000FA  li r5, 0xfa
	ctx.r[5].s64 = 250;
	// 8304D050: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8304D054: 4BFCA695  bl 0x830176e8
	ctx.lr = 0x8304D058;
	sub_830176E8(ctx, base);
	// 8304D058: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8304D05C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8304D060: 388BC3FC  addi r4, r11, -0x3c04
	ctx.r[4].s64 = ctx.r[11].s64 + -15364;
	// 8304D064: 48163BC5  bl 0x831b0c28
	ctx.lr = 0x8304D068;
	sub_831B0C28(ctx, base);
	// 8304D068: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 8304D06C: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304D070: 4BF8B229  bl 0x82fd8298
	ctx.lr = 0x8304D074;
	sub_82FD8298(ctx, base);
	// 8304D074: 81210060  lwz r9, 0x60(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 8304D078: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8304D07C: 41820020  beq 0x8304d09c
	if ctx.cr[0].eq {
	pc = 0x8304D09C; continue 'dispatch;
	}
	// 8304D080: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8304D084: 5528103A  slwi r8, r9, 2
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8304D088: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8304D08C: 7D48502E  lwzx r10, r8, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8304D090: 93830000  stw r28, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 8304D094: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8304D098: 48000008  b 0x8304d0a0
	pc = 0x8304D0A0; continue 'dispatch;
	// 8304D09C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8304D0A0: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8304D0A4: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8304D0A8: 7D69512E  stwx r11, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[11].u32) };
	// 8304D0AC: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8304D0B0: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8304D0B4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8304D0B8: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8304D0BC: 409A007C  bne cr6, 0x8304d138
	if !ctx.cr[6].eq {
	pc = 0x8304D138; continue 'dispatch;
	}
	// 8304D0C0: 796B0020  clrldi r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64 & 0x00000000FFFFFFFFu64;
	// 8304D0C4: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304D0C8: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 8304D0CC: F9610060  std r11, 0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u64 ) };
	// 8304D0D0: C8010060  lfd f0, 0x60(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 8304D0D4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304D0D8: FDA0069C  fcfid f13, f0
	ctx.f[13].f64 = (ctx.f[0].s64 as f64);
	// 8304D0DC: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8304D0E0: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8304D0E4: C80BA8D0  lfd f0, -0x5730(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-22320 as u32) ) };
	// 8304D0E8: FC0D0032  fmul f0, f13, f0
	ctx.f[0].f64 = ctx.f[13].f64 * ctx.f[0].f64;
	// 8304D0EC: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 8304D0F0: 7C0057AE  stfiwx f0, 0, r10
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32, tmp.u32) };
	// 8304D0F4: 83A10060  lwz r29, 0x60(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 8304D0F8: 57A4103A  slwi r4, r29, 2
	ctx.r[4].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 8304D0FC: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 8304D100: 4E800421  bctrl
	ctx.lr = 0x8304D104;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8304D104: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8304D108: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8304D10C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8304D110: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 8304D114: 4815B3FD  bl 0x831a8510
	ctx.lr = 0x8304D118;
	sub_831A8510(ctx, base);
	// 8304D118: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304D11C: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8304D120: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304D124: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8304D128: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8304D12C: 4E800421  bctrl
	ctx.lr = 0x8304D130;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8304D130: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 8304D134: 93BF000C  stw r29, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 8304D138: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8304D13C: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8304D140: 386B0001  addi r3, r11, 1
	ctx.r[3].s64 = ctx.r[11].s64 + 1;
	// 8304D144: 546B103A  slwi r11, r3, 2
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8304D148: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 8304D14C: 7F8B512E  stwx r28, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[28].u32) };
	// 8304D150: 907C0004  stw r3, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 8304D154: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 8304D158: 4815B060  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304D160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304D160 size=124
    let mut pc: u32 = 0x8304D160;
    'dispatch: loop {
        match pc {
            0x8304D160 => {
    //   block [0x8304D160..0x8304D1DC)
	// 8304D160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304D164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304D168: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8304D16C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304D170: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8304D174: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304D178: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8304D17C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8304D180: 4E800421  bctrl
	ctx.lr = 0x8304D184;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8304D184: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304D188: 40820030  bne 0x8304d1b8
	if !ctx.cr[0].eq {
	pc = 0x8304D1B8; continue 'dispatch;
	}
	// 8304D18C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8304D190: 80FF0014  lwz r7, 0x14(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8304D194: 38C0001E  li r6, 0x1e
	ctx.r[6].s64 = 30;
	// 8304D198: 388BB9A0  addi r4, r11, -0x4660
	ctx.r[4].s64 = ctx.r[11].s64 + -18016;
	// 8304D19C: 38A00297  li r5, 0x297
	ctx.r[5].s64 = 663;
	// 8304D1A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8304D1A4: 4BF93B85  bl 0x82fe0d28
	ctx.lr = 0x8304D1A8;
	sub_82FE0D28(ctx, base);
	// 8304D1A8: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8304D1AC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8304D1B0: 388BC690  addi r4, r11, -0x3970
	ctx.r[4].s64 = ctx.r[11].s64 + -14704;
	// 8304D1B4: 48163A75  bl 0x831b0c28
	ctx.lr = 0x8304D1B8;
	sub_831B0C28(ctx, base);
	// 8304D1B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8304D1BC: 83FF0008  lwz r31, 8(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8304D1C0: 4BFAD529  bl 0x82ffa6e8
	ctx.lr = 0x8304D1C4;
	sub_82FFA6E8(ctx, base);
	// 8304D1C4: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8304D1C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8304D1CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304D1D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304D1D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8304D1D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304D1E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8304D1E0 size=8
    let mut pc: u32 = 0x8304D1E0;
    'dispatch: loop {
        match pc {
            0x8304D1E0 => {
    //   block [0x8304D1E0..0x8304D1E8)
	// 8304D1E0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304D1E4: 82162A30  lwz r16, 0x2a30(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(10800 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304D1E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304D1E8 size=164
    let mut pc: u32 = 0x8304D1E8;
    'dispatch: loop {
        match pc {
            0x8304D1E8 => {
    //   block [0x8304D1E8..0x8304D28C)
	// 8304D1E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304D1EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304D1F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8304D1F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8304D1F8: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 8304D1FC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304D200: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8304D204: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 8304D208: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 8304D20C: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8304D210: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8304D214: 98BE0004  stb r5, 4(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[5].u8 ) };
	// 8304D218: 396B2A18  addi r11, r11, 0x2a18
	ctx.r[11].s64 = ctx.r[11].s64 + 10776;
	// 8304D21C: 909E0010  stw r4, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[4].u32 ) };
	// 8304D220: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 8304D224: 90FE0014  stw r7, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[7].u32 ) };
	// 8304D228: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 8304D22C: 915E0008  stw r10, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8304D230: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8304D234: 913E000C  stw r9, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 8304D238: 915E0018  stw r10, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 8304D23C: 409A002C  bne cr6, 0x8304d268
	if !ctx.cr[6].eq {
	pc = 0x8304D268; continue 'dispatch;
	}
	// 8304D240: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8304D244: 38C0000F  li r6, 0xf
	ctx.r[6].s64 = 15;
	// 8304D248: 388BAB5C  addi r4, r11, -0x54a4
	ctx.r[4].s64 = ctx.r[11].s64 + -21668;
	// 8304D24C: 38A001B8  li r5, 0x1b8
	ctx.r[5].s64 = 440;
	// 8304D250: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 8304D254: 4BF93C9D  bl 0x82fe0ef0
	ctx.lr = 0x8304D258;
	sub_82FE0EF0(ctx, base);
	// 8304D258: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8304D25C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 8304D260: 388BC654  addi r4, r11, -0x39ac
	ctx.r[4].s64 = ctx.r[11].s64 + -14764;
	// 8304D264: 481639C5  bl 0x831b0c28
	ctx.lr = 0x8304D268;
	sub_831B0C28(ctx, base);
	// 8304D268: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304D26C: 4BF9514D  bl 0x82fe23b8
	ctx.lr = 0x8304D270;
	sub_82FE23B8(ctx, base);
	// 8304D270: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304D274: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 8304D278: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304D27C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304D280: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8304D284: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8304D288: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304D28C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304D28C size=40
    let mut pc: u32 = 0x8304D28C;
    'dispatch: loop {
        match pc {
            0x8304D28C => {
    //   block [0x8304D28C..0x8304D2B4)
	// 8304D28C: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 8304D290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304D294: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304D298: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304D29C: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 8304D2A0: 4BFFF4C1  bl 0x8304c760
	ctx.lr = 0x8304D2A4;
	sub_8304C760(ctx, base);
	// 8304D2A4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8304D2A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304D2AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304D2B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304D2B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304D2B8 size=76
    let mut pc: u32 = 0x8304D2B8;
    'dispatch: loop {
        match pc {
            0x8304D2B8 => {
    //   block [0x8304D2B8..0x8304D304)
	// 8304D2B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304D2BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304D2C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8304D2C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8304D2C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304D2CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8304D2D0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8304D2D4: 4BFFFB1D  bl 0x8304cdf0
	ctx.lr = 0x8304D2D8;
	sub_8304CDF0(ctx, base);
	// 8304D2D8: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304D2DC: 4182000C  beq 0x8304d2e8
	if ctx.cr[0].eq {
	pc = 0x8304D2E8; continue 'dispatch;
	}
	// 8304D2E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8304D2E4: 4BF8AFFD  bl 0x82fd82e0
	ctx.lr = 0x8304D2E8;
	sub_82FD82E0(ctx, base);
	// 8304D2E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8304D2EC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8304D2F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304D2F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304D2F8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8304D2FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8304D300: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304D308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8304D308 size=8
    let mut pc: u32 = 0x8304D308;
    'dispatch: loop {
        match pc {
            0x8304D308 => {
    //   block [0x8304D308..0x8304D310)
	// 8304D308: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304D30C: 82162A78  lwz r16, 0x2a78(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(10872 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304D310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304D310 size=260
    let mut pc: u32 = 0x8304D310;
    'dispatch: loop {
        match pc {
            0x8304D310 => {
    //   block [0x8304D310..0x8304D414)
	// 8304D310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304D314: 4815AE4D  bl 0x831a8160
	ctx.lr = 0x8304D318;
	sub_831A8130(ctx, base);
	// 8304D318: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 8304D31C: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304D320: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8304D324: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 8304D328: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8304D32C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8304D330: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304D334: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 8304D338: 4BFAC3F9  bl 0x82ff9730
	ctx.lr = 0x8304D33C;
	sub_82FF9730(ctx, base);
	// 8304D33C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304D340: 418200CC  beq 0x8304d40c
	if ctx.cr[0].eq {
	pc = 0x8304D40C; continue 'dispatch;
	}
	// 8304D344: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304D348: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8304D34C: 409A005C  bne cr6, 0x8304d3a8
	if !ctx.cr[6].eq {
	pc = 0x8304D3A8; continue 'dispatch;
	}
	// 8304D350: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 8304D354: 40980008  bge cr6, 0x8304d35c
	if !ctx.cr[6].lt {
	pc = 0x8304D35C; continue 'dispatch;
	}
	// 8304D358: 3B800010  li r28, 0x10
	ctx.r[28].s64 = 16;
	// 8304D35C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304D360: 4BFAAD51  bl 0x82ff80b0
	ctx.lr = 0x8304D364;
	sub_82FF80B0(ctx, base);
	// 8304D364: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8304D368: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 8304D36C: 909F0054  stw r4, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[4].u32 ) };
	// 8304D370: 4BF8AF29  bl 0x82fd8298
	ctx.lr = 0x8304D374;
	sub_82FD8298(ctx, base);
	// 8304D374: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8304D378: 93BF0058  stw r29, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[29].u32 ) };
	// 8304D37C: 41820024  beq 0x8304d3a0
	if ctx.cr[0].eq {
	pc = 0x8304D3A0; continue 'dispatch;
	}
	// 8304D380: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304D384: 4BFAAD2D  bl 0x82ff80b0
	ctx.lr = 0x8304D388;
	sub_82FF80B0(ctx, base);
	// 8304D388: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8304D38C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8304D390: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304D394: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 8304D398: 4801B409  bl 0x830687a0
	ctx.lr = 0x8304D39C;
	sub_830687A0(ctx, base);
	// 8304D39C: 48000008  b 0x8304d3a4
	pc = 0x8304D3A4; continue 'dispatch;
	// 8304D3A0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8304D3A4: 907B0000  stw r3, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8304D3A8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304D3AC: 809B0000  lwz r4, 0(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304D3B0: 4BFABAC1  bl 0x82ff8e70
	ctx.lr = 0x8304D3B4;
	sub_82FF8E70(ctx, base);
	// 8304D3B4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8304D3B8: 389F0050  addi r4, r31, 0x50
	ctx.r[4].s64 = ctx.r[31].s64 + 80;
	// 8304D3BC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304D3C0: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8304D3C4: 4BFAC1B5  bl 0x82ff9578
	ctx.lr = 0x8304D3C8;
	sub_82FF9578(ctx, base);
	// 8304D3C8: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8304D3CC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8304D3D0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304D3D4: 40990038  ble cr6, 0x8304d40c
	if !ctx.cr[6].gt {
	pc = 0x8304D40C; continue 'dispatch;
	}
	// 8304D3D8: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8304D3DC: 3B8B343C  addi r28, r11, 0x343c
	ctx.r[28].s64 = ctx.r[11].s64 + 13372;
	// 8304D3E0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304D3E4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8304D3E8: 4BFAC8D9  bl 0x82ff9cc0
	ctx.lr = 0x8304D3EC;
	sub_82FF9CC0(ctx, base);
	// 8304D3EC: 389F0054  addi r4, r31, 0x54
	ctx.r[4].s64 = ctx.r[31].s64 + 84;
	// 8304D3F0: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 8304D3F4: 807B0000  lwz r3, 0(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304D3F8: 4BF99811  bl 0x82fe6c08
	ctx.lr = 0x8304D3FC;
	sub_82FE6C08(ctx, base);
	// 8304D3FC: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8304D400: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 8304D404: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8304D408: 4198FFD8  blt cr6, 0x8304d3e0
	if ctx.cr[6].lt {
	pc = 0x8304D3E0; continue 'dispatch;
	}
	// 8304D40C: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 8304D410: 4815ADA0  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304D414(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304D414 size=44
    let mut pc: u32 = 0x8304D414;
    'dispatch: loop {
        match pc {
            0x8304D414 => {
    //   block [0x8304D414..0x8304D440)
	// 8304D414: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 8304D418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304D41C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304D420: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304D424: 809F0054  lwz r4, 0x54(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8304D428: 807F0058  lwz r3, 0x58(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 8304D42C: 4BF8AEB5  bl 0x82fd82e0
	ctx.lr = 0x8304D430;
	sub_82FD82E0(ctx, base);
	// 8304D430: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8304D434: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304D438: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304D43C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304D440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8304D440 size=8
    let mut pc: u32 = 0x8304D440;
    'dispatch: loop {
        match pc {
            0x8304D440 => {
    //   block [0x8304D440..0x8304D448)
	// 8304D440: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304D444: 82162AC0  lwz r16, 0x2ac0(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(10944 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304D448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304D448 size=248
    let mut pc: u32 = 0x8304D448;
    'dispatch: loop {
        match pc {
            0x8304D448 => {
    //   block [0x8304D448..0x8304D540)
	// 8304D448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304D44C: 4815AD15  bl 0x831a8160
	ctx.lr = 0x8304D450;
	sub_831A8130(ctx, base);
	// 8304D450: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 8304D454: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304D458: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8304D45C: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 8304D460: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 8304D464: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8304D468: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304D46C: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 8304D470: 4BFAC2C1  bl 0x82ff9730
	ctx.lr = 0x8304D474;
	sub_82FF9730(ctx, base);
	// 8304D474: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304D478: 418200C0  beq 0x8304d538
	if ctx.cr[0].eq {
	pc = 0x8304D538; continue 'dispatch;
	}
	// 8304D47C: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304D480: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8304D484: 409A005C  bne cr6, 0x8304d4e0
	if !ctx.cr[6].eq {
	pc = 0x8304D4E0; continue 'dispatch;
	}
	// 8304D488: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 8304D48C: 40980008  bge cr6, 0x8304d494
	if !ctx.cr[6].lt {
	pc = 0x8304D494; continue 'dispatch;
	}
	// 8304D490: 3B600010  li r27, 0x10
	ctx.r[27].s64 = 16;
	// 8304D494: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304D498: 4BFAAC19  bl 0x82ff80b0
	ctx.lr = 0x8304D49C;
	sub_82FF80B0(ctx, base);
	// 8304D49C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8304D4A0: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 8304D4A4: 909F0054  stw r4, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[4].u32 ) };
	// 8304D4A8: 4BF8ADF1  bl 0x82fd8298
	ctx.lr = 0x8304D4AC;
	sub_82FD8298(ctx, base);
	// 8304D4AC: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8304D4B0: 93BF0058  stw r29, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[29].u32 ) };
	// 8304D4B4: 41820024  beq 0x8304d4d8
	if ctx.cr[0].eq {
	pc = 0x8304D4D8; continue 'dispatch;
	}
	// 8304D4B8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304D4BC: 4BFAABF5  bl 0x82ff80b0
	ctx.lr = 0x8304D4C0;
	sub_82FF80B0(ctx, base);
	// 8304D4C0: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8304D4C4: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8304D4C8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304D4CC: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 8304D4D0: 4801B2D1  bl 0x830687a0
	ctx.lr = 0x8304D4D4;
	sub_830687A0(ctx, base);
	// 8304D4D4: 48000008  b 0x8304d4dc
	pc = 0x8304D4DC; continue 'dispatch;
	// 8304D4D8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8304D4DC: 907C0000  stw r3, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8304D4E0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304D4E4: 809C0000  lwz r4, 0(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304D4E8: 4BFAB989  bl 0x82ff8e70
	ctx.lr = 0x8304D4EC;
	sub_82FF8E70(ctx, base);
	// 8304D4EC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8304D4F0: 389F0050  addi r4, r31, 0x50
	ctx.r[4].s64 = ctx.r[31].s64 + 80;
	// 8304D4F4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304D4F8: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8304D4FC: 4BFAC07D  bl 0x82ff9578
	ctx.lr = 0x8304D500;
	sub_82FF9578(ctx, base);
	// 8304D500: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8304D504: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8304D508: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304D50C: 4099002C  ble cr6, 0x8304d538
	if !ctx.cr[6].gt {
	pc = 0x8304D538; continue 'dispatch;
	}
	// 8304D510: 389F0054  addi r4, r31, 0x54
	ctx.r[4].s64 = ctx.r[31].s64 + 84;
	// 8304D514: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304D518: 4BFAC061  bl 0x82ff9578
	ctx.lr = 0x8304D51C;
	sub_82FF9578(ctx, base);
	// 8304D51C: 389F0054  addi r4, r31, 0x54
	ctx.r[4].s64 = ctx.r[31].s64 + 84;
	// 8304D520: 807C0000  lwz r3, 0(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304D524: 4BF996E5  bl 0x82fe6c08
	ctx.lr = 0x8304D528;
	sub_82FE6C08(ctx, base);
	// 8304D528: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8304D52C: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 8304D530: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8304D534: 4198FFDC  blt cr6, 0x8304d510
	if ctx.cr[6].lt {
	pc = 0x8304D510; continue 'dispatch;
	}
	// 8304D538: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 8304D53C: 4815AC74  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304D540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304D540 size=44
    let mut pc: u32 = 0x8304D540;
    'dispatch: loop {
        match pc {
            0x8304D540 => {
    //   block [0x8304D540..0x8304D56C)
	// 8304D540: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 8304D544: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304D548: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304D54C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304D550: 809F0054  lwz r4, 0x54(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8304D554: 807F0058  lwz r3, 0x58(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 8304D558: 4BF8AD89  bl 0x82fd82e0
	ctx.lr = 0x8304D55C;
	sub_82FD82E0(ctx, base);
	// 8304D55C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8304D560: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304D564: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304D568: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304D570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8304D570 size=8
    let mut pc: u32 = 0x8304D570;
    'dispatch: loop {
        match pc {
            0x8304D570 => {
    //   block [0x8304D570..0x8304D578)
	// 8304D570: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304D574: 82162B08  lwz r16, 0x2b08(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(11016 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304D578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304D578 size=276
    let mut pc: u32 = 0x8304D578;
    'dispatch: loop {
        match pc {
            0x8304D578 => {
    //   block [0x8304D578..0x8304D68C)
	// 8304D578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304D57C: 4815ABE5  bl 0x831a8160
	ctx.lr = 0x8304D580;
	sub_831A8130(ctx, base);
	// 8304D580: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 8304D584: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304D588: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 8304D58C: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 8304D590: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8304D594: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8304D598: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304D59C: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 8304D5A0: 4BFAC191  bl 0x82ff9730
	ctx.lr = 0x8304D5A4;
	sub_82FF9730(ctx, base);
	// 8304D5A4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304D5A8: 418200DC  beq 0x8304d684
	if ctx.cr[0].eq {
	pc = 0x8304D684; continue 'dispatch;
	}
	// 8304D5AC: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304D5B0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8304D5B4: 409A006C  bne cr6, 0x8304d620
	if !ctx.cr[6].eq {
	pc = 0x8304D620; continue 'dispatch;
	}
	// 8304D5B8: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 8304D5BC: 40980008  bge cr6, 0x8304d5c4
	if !ctx.cr[6].lt {
	pc = 0x8304D5C4; continue 'dispatch;
	}
	// 8304D5C0: 3B800010  li r28, 0x10
	ctx.r[28].s64 = 16;
	// 8304D5C4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304D5C8: 4BFAAAE9  bl 0x82ff80b0
	ctx.lr = 0x8304D5CC;
	sub_82FF80B0(ctx, base);
	// 8304D5CC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8304D5D0: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 8304D5D4: 909F0058  stw r4, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[4].u32 ) };
	// 8304D5D8: 4BF8ACC1  bl 0x82fd8298
	ctx.lr = 0x8304D5DC;
	sub_82FD8298(ctx, base);
	// 8304D5DC: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8304D5E0: 93DF005C  stw r30, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 8304D5E4: 41820034  beq 0x8304d618
	if ctx.cr[0].eq {
	pc = 0x8304D618; continue 'dispatch;
	}
	// 8304D5E8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304D5EC: 4BFAAAC5  bl 0x82ff80b0
	ctx.lr = 0x8304D5F0;
	sub_82FF80B0(ctx, base);
	// 8304D5F0: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 8304D5F4: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 8304D5F8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8304D5FC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304D600: 4BF85441  bl 0x82fd2a40
	ctx.lr = 0x8304D604;
	sub_82FD2A40(ctx, base);
	// 8304D604: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 8304D608: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 8304D60C: 396B6C68  addi r11, r11, 0x6c68
	ctx.r[11].s64 = ctx.r[11].s64 + 27752;
	// 8304D610: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8304D614: 48000008  b 0x8304d61c
	pc = 0x8304D61C; continue 'dispatch;
	// 8304D618: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8304D61C: 915A0000  stw r10, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8304D620: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304D624: 809A0000  lwz r4, 0(r26)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304D628: 4BFAB849  bl 0x82ff8e70
	ctx.lr = 0x8304D62C;
	sub_82FF8E70(ctx, base);
	// 8304D62C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8304D630: 389F0050  addi r4, r31, 0x50
	ctx.r[4].s64 = ctx.r[31].s64 + 80;
	// 8304D634: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304D638: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8304D63C: 4BFABF3D  bl 0x82ff9578
	ctx.lr = 0x8304D640;
	sub_82FF9578(ctx, base);
	// 8304D640: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8304D644: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8304D648: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304D64C: 40990038  ble cr6, 0x8304d684
	if !ctx.cr[6].gt {
	pc = 0x8304D684; continue 'dispatch;
	}
	// 8304D650: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8304D654: 38DF005C  addi r6, r31, 0x5c
	ctx.r[6].s64 = ctx.r[31].s64 + 92;
	// 8304D658: 38BF0058  addi r5, r31, 0x58
	ctx.r[5].s64 = ctx.r[31].s64 + 88;
	// 8304D65C: 389F0054  addi r4, r31, 0x54
	ctx.r[4].s64 = ctx.r[31].s64 + 84;
	// 8304D660: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304D664: 4BFAC4C5  bl 0x82ff9b28
	ctx.lr = 0x8304D668;
	sub_82FF9B28(ctx, base);
	// 8304D668: 809F0054  lwz r4, 0x54(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8304D66C: 807A0000  lwz r3, 0(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304D670: 4BFEDAE1  bl 0x8303b150
	ctx.lr = 0x8304D674;
	sub_8303B150(ctx, base);
	// 8304D674: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8304D678: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 8304D67C: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8304D680: 4198FFD0  blt cr6, 0x8304d650
	if ctx.cr[6].lt {
	pc = 0x8304D650; continue 'dispatch;
	}
	// 8304D684: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 8304D688: 4815AB28  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304D68C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304D68C size=44
    let mut pc: u32 = 0x8304D68C;
    'dispatch: loop {
        match pc {
            0x8304D68C => {
    //   block [0x8304D68C..0x8304D6B8)
	// 8304D68C: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 8304D690: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304D694: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304D698: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304D69C: 809F0058  lwz r4, 0x58(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 8304D6A0: 807F005C  lwz r3, 0x5c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 8304D6A4: 4BF8AC3D  bl 0x82fd82e0
	ctx.lr = 0x8304D6A8;
	sub_82FD82E0(ctx, base);
	// 8304D6A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8304D6AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304D6B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304D6B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304D6B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8304D6B8 size=8
    let mut pc: u32 = 0x8304D6B8;
    'dispatch: loop {
        match pc {
            0x8304D6B8 => {
    //   block [0x8304D6B8..0x8304D6C0)
	// 8304D6B8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304D6BC: 82162B50  lwz r16, 0x2b50(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(11088 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304D6C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304D6C0 size=272
    let mut pc: u32 = 0x8304D6C0;
    'dispatch: loop {
        match pc {
            0x8304D6C0 => {
    //   block [0x8304D6C0..0x8304D7D0)
	// 8304D6C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304D6C4: 4815AA9D  bl 0x831a8160
	ctx.lr = 0x8304D6C8;
	sub_831A8130(ctx, base);
	// 8304D6C8: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 8304D6CC: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304D6D0: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 8304D6D4: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 8304D6D8: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8304D6DC: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8304D6E0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304D6E4: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 8304D6E8: 4BFAC049  bl 0x82ff9730
	ctx.lr = 0x8304D6EC;
	sub_82FF9730(ctx, base);
	// 8304D6EC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304D6F0: 418200D8  beq 0x8304d7c8
	if ctx.cr[0].eq {
	pc = 0x8304D7C8; continue 'dispatch;
	}
	// 8304D6F4: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304D6F8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8304D6FC: 409A006C  bne cr6, 0x8304d768
	if !ctx.cr[6].eq {
	pc = 0x8304D768; continue 'dispatch;
	}
	// 8304D700: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 8304D704: 40980008  bge cr6, 0x8304d70c
	if !ctx.cr[6].lt {
	pc = 0x8304D70C; continue 'dispatch;
	}
	// 8304D708: 3B800010  li r28, 0x10
	ctx.r[28].s64 = 16;
	// 8304D70C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304D710: 4BFAA9A1  bl 0x82ff80b0
	ctx.lr = 0x8304D714;
	sub_82FF80B0(ctx, base);
	// 8304D714: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8304D718: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 8304D71C: 909F0054  stw r4, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[4].u32 ) };
	// 8304D720: 4BF8AB79  bl 0x82fd8298
	ctx.lr = 0x8304D724;
	sub_82FD8298(ctx, base);
	// 8304D724: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8304D728: 93DF0058  stw r30, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[30].u32 ) };
	// 8304D72C: 41820034  beq 0x8304d760
	if ctx.cr[0].eq {
	pc = 0x8304D760; continue 'dispatch;
	}
	// 8304D730: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304D734: 4BFAA97D  bl 0x82ff80b0
	ctx.lr = 0x8304D738;
	sub_82FF80B0(ctx, base);
	// 8304D738: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 8304D73C: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 8304D740: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8304D744: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304D748: 4BFFF0A9  bl 0x8304c7f0
	ctx.lr = 0x8304D74C;
	sub_8304C7F0(ctx, base);
	// 8304D74C: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8304D750: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 8304D754: 396B2990  addi r11, r11, 0x2990
	ctx.r[11].s64 = ctx.r[11].s64 + 10640;
	// 8304D758: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8304D75C: 48000008  b 0x8304d764
	pc = 0x8304D764; continue 'dispatch;
	// 8304D760: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8304D764: 915A0000  stw r10, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8304D768: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304D76C: 809A0000  lwz r4, 0(r26)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304D770: 4BFAB701  bl 0x82ff8e70
	ctx.lr = 0x8304D774;
	sub_82FF8E70(ctx, base);
	// 8304D774: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8304D778: 389F0050  addi r4, r31, 0x50
	ctx.r[4].s64 = ctx.r[31].s64 + 80;
	// 8304D77C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304D780: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8304D784: 4BFABDF5  bl 0x82ff9578
	ctx.lr = 0x8304D788;
	sub_82FF9578(ctx, base);
	// 8304D788: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8304D78C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8304D790: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304D794: 40990034  ble cr6, 0x8304d7c8
	if !ctx.cr[6].gt {
	pc = 0x8304D7C8; continue 'dispatch;
	}
	// 8304D798: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8304D79C: 3B8B34C0  addi r28, r11, 0x34c0
	ctx.r[28].s64 = ctx.r[11].s64 + 13504;
	// 8304D7A0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304D7A4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8304D7A8: 4BFAC519  bl 0x82ff9cc0
	ctx.lr = 0x8304D7AC;
	sub_82FF9CC0(ctx, base);
	// 8304D7AC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8304D7B0: 807A0000  lwz r3, 0(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304D7B4: 4BFED99D  bl 0x8303b150
	ctx.lr = 0x8304D7B8;
	sub_8303B150(ctx, base);
	// 8304D7B8: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8304D7BC: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 8304D7C0: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8304D7C4: 4198FFDC  blt cr6, 0x8304d7a0
	if ctx.cr[6].lt {
	pc = 0x8304D7A0; continue 'dispatch;
	}
	// 8304D7C8: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 8304D7CC: 4815A9E4  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304D7D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304D7D0 size=44
    let mut pc: u32 = 0x8304D7D0;
    'dispatch: loop {
        match pc {
            0x8304D7D0 => {
    //   block [0x8304D7D0..0x8304D7FC)
	// 8304D7D0: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 8304D7D4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304D7D8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304D7DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304D7E0: 809F0054  lwz r4, 0x54(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8304D7E4: 807F0058  lwz r3, 0x58(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 8304D7E8: 4BF8AAF9  bl 0x82fd82e0
	ctx.lr = 0x8304D7EC;
	sub_82FD82E0(ctx, base);
	// 8304D7EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8304D7F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304D7F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304D7F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304D800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8304D800 size=8
    let mut pc: u32 = 0x8304D800;
    'dispatch: loop {
        match pc {
            0x8304D800 => {
    //   block [0x8304D800..0x8304D808)
	// 8304D800: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304D804: 82162B98  lwz r16, 0x2b98(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(11160 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304D808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304D808 size=272
    let mut pc: u32 = 0x8304D808;
    'dispatch: loop {
        match pc {
            0x8304D808 => {
    //   block [0x8304D808..0x8304D918)
	// 8304D808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304D80C: 4815A955  bl 0x831a8160
	ctx.lr = 0x8304D810;
	sub_831A8130(ctx, base);
	// 8304D810: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 8304D814: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304D818: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 8304D81C: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 8304D820: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8304D824: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8304D828: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304D82C: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 8304D830: 4BFABF01  bl 0x82ff9730
	ctx.lr = 0x8304D834;
	sub_82FF9730(ctx, base);
	// 8304D834: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304D838: 418200D8  beq 0x8304d910
	if ctx.cr[0].eq {
	pc = 0x8304D910; continue 'dispatch;
	}
	// 8304D83C: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304D840: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8304D844: 409A006C  bne cr6, 0x8304d8b0
	if !ctx.cr[6].eq {
	pc = 0x8304D8B0; continue 'dispatch;
	}
	// 8304D848: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 8304D84C: 40980008  bge cr6, 0x8304d854
	if !ctx.cr[6].lt {
	pc = 0x8304D854; continue 'dispatch;
	}
	// 8304D850: 3B800010  li r28, 0x10
	ctx.r[28].s64 = 16;
	// 8304D854: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304D858: 4BFAA859  bl 0x82ff80b0
	ctx.lr = 0x8304D85C;
	sub_82FF80B0(ctx, base);
	// 8304D85C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8304D860: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 8304D864: 909F0054  stw r4, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[4].u32 ) };
	// 8304D868: 4BF8AA31  bl 0x82fd8298
	ctx.lr = 0x8304D86C;
	sub_82FD8298(ctx, base);
	// 8304D86C: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8304D870: 93DF0058  stw r30, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[30].u32 ) };
	// 8304D874: 41820034  beq 0x8304d8a8
	if ctx.cr[0].eq {
	pc = 0x8304D8A8; continue 'dispatch;
	}
	// 8304D878: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304D87C: 4BFAA835  bl 0x82ff80b0
	ctx.lr = 0x8304D880;
	sub_82FF80B0(ctx, base);
	// 8304D880: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 8304D884: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 8304D888: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8304D88C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304D890: 4BFFEF61  bl 0x8304c7f0
	ctx.lr = 0x8304D894;
	sub_8304C7F0(ctx, base);
	// 8304D894: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8304D898: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 8304D89C: 396B2990  addi r11, r11, 0x2990
	ctx.r[11].s64 = ctx.r[11].s64 + 10640;
	// 8304D8A0: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8304D8A4: 48000008  b 0x8304d8ac
	pc = 0x8304D8AC; continue 'dispatch;
	// 8304D8A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8304D8AC: 915A0000  stw r10, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8304D8B0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304D8B4: 809A0000  lwz r4, 0(r26)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304D8B8: 4BFAB5B9  bl 0x82ff8e70
	ctx.lr = 0x8304D8BC;
	sub_82FF8E70(ctx, base);
	// 8304D8BC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8304D8C0: 389F0050  addi r4, r31, 0x50
	ctx.r[4].s64 = ctx.r[31].s64 + 80;
	// 8304D8C4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304D8C8: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8304D8CC: 4BFABCAD  bl 0x82ff9578
	ctx.lr = 0x8304D8D0;
	sub_82FF9578(ctx, base);
	// 8304D8D0: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8304D8D4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8304D8D8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304D8DC: 40990034  ble cr6, 0x8304d910
	if !ctx.cr[6].gt {
	pc = 0x8304D910; continue 'dispatch;
	}
	// 8304D8E0: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8304D8E4: 3B8B343C  addi r28, r11, 0x343c
	ctx.r[28].s64 = ctx.r[11].s64 + 13372;
	// 8304D8E8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304D8EC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8304D8F0: 4BFAC3D1  bl 0x82ff9cc0
	ctx.lr = 0x8304D8F4;
	sub_82FF9CC0(ctx, base);
	// 8304D8F4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8304D8F8: 807A0000  lwz r3, 0(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304D8FC: 4BFED855  bl 0x8303b150
	ctx.lr = 0x8304D900;
	sub_8303B150(ctx, base);
	// 8304D900: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8304D904: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 8304D908: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8304D90C: 4198FFDC  blt cr6, 0x8304d8e8
	if ctx.cr[6].lt {
	pc = 0x8304D8E8; continue 'dispatch;
	}
	// 8304D910: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 8304D914: 4815A89C  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304D918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304D918 size=44
    let mut pc: u32 = 0x8304D918;
    'dispatch: loop {
        match pc {
            0x8304D918 => {
    //   block [0x8304D918..0x8304D944)
	// 8304D918: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 8304D91C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304D920: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304D924: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304D928: 809F0054  lwz r4, 0x54(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8304D92C: 807F0058  lwz r3, 0x58(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 8304D930: 4BF8A9B1  bl 0x82fd82e0
	ctx.lr = 0x8304D934;
	sub_82FD82E0(ctx, base);
	// 8304D934: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8304D938: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304D93C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304D940: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304D948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8304D948 size=8
    let mut pc: u32 = 0x8304D948;
    'dispatch: loop {
        match pc {
            0x8304D948 => {
    //   block [0x8304D948..0x8304D950)
	// 8304D948: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304D94C: 82162BE0  lwz r16, 0x2be0(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(11232 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304D950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304D950 size=272
    let mut pc: u32 = 0x8304D950;
    'dispatch: loop {
        match pc {
            0x8304D950 => {
    //   block [0x8304D950..0x8304DA60)
	// 8304D950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304D954: 4815A80D  bl 0x831a8160
	ctx.lr = 0x8304D958;
	sub_831A8130(ctx, base);
	// 8304D958: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 8304D95C: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304D960: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 8304D964: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 8304D968: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8304D96C: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8304D970: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304D974: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 8304D978: 4BFABDB9  bl 0x82ff9730
	ctx.lr = 0x8304D97C;
	sub_82FF9730(ctx, base);
	// 8304D97C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304D980: 418200D8  beq 0x8304da58
	if ctx.cr[0].eq {
	pc = 0x8304DA58; continue 'dispatch;
	}
	// 8304D984: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304D988: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8304D98C: 409A006C  bne cr6, 0x8304d9f8
	if !ctx.cr[6].eq {
	pc = 0x8304D9F8; continue 'dispatch;
	}
	// 8304D990: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 8304D994: 40980008  bge cr6, 0x8304d99c
	if !ctx.cr[6].lt {
	pc = 0x8304D99C; continue 'dispatch;
	}
	// 8304D998: 3B800010  li r28, 0x10
	ctx.r[28].s64 = 16;
	// 8304D99C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304D9A0: 4BFAA711  bl 0x82ff80b0
	ctx.lr = 0x8304D9A4;
	sub_82FF80B0(ctx, base);
	// 8304D9A4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8304D9A8: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 8304D9AC: 909F0054  stw r4, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[4].u32 ) };
	// 8304D9B0: 4BF8A8E9  bl 0x82fd8298
	ctx.lr = 0x8304D9B4;
	sub_82FD8298(ctx, base);
	// 8304D9B4: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8304D9B8: 93DF0058  stw r30, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[30].u32 ) };
	// 8304D9BC: 41820034  beq 0x8304d9f0
	if ctx.cr[0].eq {
	pc = 0x8304D9F0; continue 'dispatch;
	}
	// 8304D9C0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304D9C4: 4BFAA6ED  bl 0x82ff80b0
	ctx.lr = 0x8304D9C8;
	sub_82FF80B0(ctx, base);
	// 8304D9C8: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 8304D9CC: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 8304D9D0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8304D9D4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304D9D8: 4BFFEE19  bl 0x8304c7f0
	ctx.lr = 0x8304D9DC;
	sub_8304C7F0(ctx, base);
	// 8304D9DC: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8304D9E0: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 8304D9E4: 396B2990  addi r11, r11, 0x2990
	ctx.r[11].s64 = ctx.r[11].s64 + 10640;
	// 8304D9E8: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8304D9EC: 48000008  b 0x8304d9f4
	pc = 0x8304D9F4; continue 'dispatch;
	// 8304D9F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8304D9F4: 915A0000  stw r10, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8304D9F8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304D9FC: 809A0000  lwz r4, 0(r26)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304DA00: 4BFAB471  bl 0x82ff8e70
	ctx.lr = 0x8304DA04;
	sub_82FF8E70(ctx, base);
	// 8304DA04: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8304DA08: 389F0050  addi r4, r31, 0x50
	ctx.r[4].s64 = ctx.r[31].s64 + 80;
	// 8304DA0C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304DA10: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8304DA14: 4BFABB65  bl 0x82ff9578
	ctx.lr = 0x8304DA18;
	sub_82FF9578(ctx, base);
	// 8304DA18: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8304DA1C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8304DA20: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304DA24: 40990034  ble cr6, 0x8304da58
	if !ctx.cr[6].gt {
	pc = 0x8304DA58; continue 'dispatch;
	}
	// 8304DA28: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8304DA2C: 3B8B3590  addi r28, r11, 0x3590
	ctx.r[28].s64 = ctx.r[11].s64 + 13712;
	// 8304DA30: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304DA34: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8304DA38: 4BFAC289  bl 0x82ff9cc0
	ctx.lr = 0x8304DA3C;
	sub_82FF9CC0(ctx, base);
	// 8304DA3C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8304DA40: 807A0000  lwz r3, 0(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304DA44: 4BFED70D  bl 0x8303b150
	ctx.lr = 0x8304DA48;
	sub_8303B150(ctx, base);
	// 8304DA48: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8304DA4C: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 8304DA50: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8304DA54: 4198FFDC  blt cr6, 0x8304da30
	if ctx.cr[6].lt {
	pc = 0x8304DA30; continue 'dispatch;
	}
	// 8304DA58: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 8304DA5C: 4815A754  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304DA60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304DA60 size=44
    let mut pc: u32 = 0x8304DA60;
    'dispatch: loop {
        match pc {
            0x8304DA60 => {
    //   block [0x8304DA60..0x8304DA8C)
	// 8304DA60: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 8304DA64: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304DA68: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304DA6C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304DA70: 809F0054  lwz r4, 0x54(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8304DA74: 807F0058  lwz r3, 0x58(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 8304DA78: 4BF8A869  bl 0x82fd82e0
	ctx.lr = 0x8304DA7C;
	sub_82FD82E0(ctx, base);
	// 8304DA7C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8304DA80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304DA84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304DA88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304DA90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8304DA90 size=8
    let mut pc: u32 = 0x8304DA90;
    'dispatch: loop {
        match pc {
            0x8304DA90 => {
    //   block [0x8304DA90..0x8304DA98)
	// 8304DA90: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304DA94: 82162C28  lwz r16, 0x2c28(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(11304 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304DA98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304DA98 size=260
    let mut pc: u32 = 0x8304DA98;
    'dispatch: loop {
        match pc {
            0x8304DA98 => {
    //   block [0x8304DA98..0x8304DB9C)
	// 8304DA98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304DA9C: 4815A6C5  bl 0x831a8160
	ctx.lr = 0x8304DAA0;
	sub_831A8130(ctx, base);
	// 8304DAA0: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 8304DAA4: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304DAA8: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8304DAAC: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 8304DAB0: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8304DAB4: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8304DAB8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304DABC: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 8304DAC0: 4BFABC71  bl 0x82ff9730
	ctx.lr = 0x8304DAC4;
	sub_82FF9730(ctx, base);
	// 8304DAC4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304DAC8: 418200CC  beq 0x8304db94
	if ctx.cr[0].eq {
	pc = 0x8304DB94; continue 'dispatch;
	}
	// 8304DACC: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304DAD0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8304DAD4: 409A006C  bne cr6, 0x8304db40
	if !ctx.cr[6].eq {
	pc = 0x8304DB40; continue 'dispatch;
	}
	// 8304DAD8: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 8304DADC: 40980008  bge cr6, 0x8304dae4
	if !ctx.cr[6].lt {
	pc = 0x8304DAE4; continue 'dispatch;
	}
	// 8304DAE0: 3B800010  li r28, 0x10
	ctx.r[28].s64 = 16;
	// 8304DAE4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304DAE8: 4BFAA5C9  bl 0x82ff80b0
	ctx.lr = 0x8304DAEC;
	sub_82FF80B0(ctx, base);
	// 8304DAEC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8304DAF0: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 8304DAF4: 909F0054  stw r4, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[4].u32 ) };
	// 8304DAF8: 4BF8A7A1  bl 0x82fd8298
	ctx.lr = 0x8304DAFC;
	sub_82FD8298(ctx, base);
	// 8304DAFC: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8304DB00: 93DF0058  stw r30, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[30].u32 ) };
	// 8304DB04: 41820034  beq 0x8304db38
	if ctx.cr[0].eq {
	pc = 0x8304DB38; continue 'dispatch;
	}
	// 8304DB08: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304DB0C: 4BFAA5A5  bl 0x82ff80b0
	ctx.lr = 0x8304DB10;
	sub_82FF80B0(ctx, base);
	// 8304DB10: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 8304DB14: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 8304DB18: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8304DB1C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304DB20: 4BFFECD1  bl 0x8304c7f0
	ctx.lr = 0x8304DB24;
	sub_8304C7F0(ctx, base);
	// 8304DB24: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8304DB28: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 8304DB2C: 396B2990  addi r11, r11, 0x2990
	ctx.r[11].s64 = ctx.r[11].s64 + 10640;
	// 8304DB30: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8304DB34: 48000008  b 0x8304db3c
	pc = 0x8304DB3C; continue 'dispatch;
	// 8304DB38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8304DB3C: 915B0000  stw r10, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8304DB40: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304DB44: 809B0000  lwz r4, 0(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304DB48: 4BFAB329  bl 0x82ff8e70
	ctx.lr = 0x8304DB4C;
	sub_82FF8E70(ctx, base);
	// 8304DB4C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8304DB50: 389F0050  addi r4, r31, 0x50
	ctx.r[4].s64 = ctx.r[31].s64 + 80;
	// 8304DB54: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304DB58: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8304DB5C: 4BFABA1D  bl 0x82ff9578
	ctx.lr = 0x8304DB60;
	sub_82FF9578(ctx, base);
	// 8304DB60: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8304DB64: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8304DB68: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304DB6C: 40990028  ble cr6, 0x8304db94
	if !ctx.cr[6].gt {
	pc = 0x8304DB94; continue 'dispatch;
	}
	// 8304DB70: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304DB74: 4BFF37BD  bl 0x83041330
	ctx.lr = 0x8304DB78;
	sub_83041330(ctx, base);
	// 8304DB78: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8304DB7C: 807B0000  lwz r3, 0(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304DB80: 4BFED5D1  bl 0x8303b150
	ctx.lr = 0x8304DB84;
	sub_8303B150(ctx, base);
	// 8304DB84: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8304DB88: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 8304DB8C: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8304DB90: 4198FFE0  blt cr6, 0x8304db70
	if ctx.cr[6].lt {
	pc = 0x8304DB70; continue 'dispatch;
	}
	// 8304DB94: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 8304DB98: 4815A618  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304DB9C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304DB9C size=44
    let mut pc: u32 = 0x8304DB9C;
    'dispatch: loop {
        match pc {
            0x8304DB9C => {
    //   block [0x8304DB9C..0x8304DBC8)
	// 8304DB9C: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 8304DBA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304DBA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304DBA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304DBAC: 809F0054  lwz r4, 0x54(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8304DBB0: 807F0058  lwz r3, 0x58(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 8304DBB4: 4BF8A72D  bl 0x82fd82e0
	ctx.lr = 0x8304DBB8;
	sub_82FD82E0(ctx, base);
	// 8304DBB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8304DBBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304DBC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304DBC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304DBC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8304DBC8 size=8
    let mut pc: u32 = 0x8304DBC8;
    'dispatch: loop {
        match pc {
            0x8304DBC8 => {
    //   block [0x8304DBC8..0x8304DBD0)
	// 8304DBC8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304DBCC: 82162C70  lwz r16, 0x2c70(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(11376 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304DBD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304DBD0 size=260
    let mut pc: u32 = 0x8304DBD0;
    'dispatch: loop {
        match pc {
            0x8304DBD0 => {
    //   block [0x8304DBD0..0x8304DCD4)
	// 8304DBD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304DBD4: 4815A58D  bl 0x831a8160
	ctx.lr = 0x8304DBD8;
	sub_831A8130(ctx, base);
	// 8304DBD8: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 8304DBDC: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304DBE0: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8304DBE4: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 8304DBE8: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8304DBEC: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8304DBF0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304DBF4: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 8304DBF8: 4BFABB39  bl 0x82ff9730
	ctx.lr = 0x8304DBFC;
	sub_82FF9730(ctx, base);
	// 8304DBFC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304DC00: 418200CC  beq 0x8304dccc
	if ctx.cr[0].eq {
	pc = 0x8304DCCC; continue 'dispatch;
	}
	// 8304DC04: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304DC08: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8304DC0C: 409A006C  bne cr6, 0x8304dc78
	if !ctx.cr[6].eq {
	pc = 0x8304DC78; continue 'dispatch;
	}
	// 8304DC10: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 8304DC14: 40980008  bge cr6, 0x8304dc1c
	if !ctx.cr[6].lt {
	pc = 0x8304DC1C; continue 'dispatch;
	}
	// 8304DC18: 3B800010  li r28, 0x10
	ctx.r[28].s64 = 16;
	// 8304DC1C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304DC20: 4BFAA491  bl 0x82ff80b0
	ctx.lr = 0x8304DC24;
	sub_82FF80B0(ctx, base);
	// 8304DC24: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8304DC28: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 8304DC2C: 909F0054  stw r4, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[4].u32 ) };
	// 8304DC30: 4BF8A669  bl 0x82fd8298
	ctx.lr = 0x8304DC34;
	sub_82FD8298(ctx, base);
	// 8304DC34: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8304DC38: 93DF0058  stw r30, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[30].u32 ) };
	// 8304DC3C: 41820034  beq 0x8304dc70
	if ctx.cr[0].eq {
	pc = 0x8304DC70; continue 'dispatch;
	}
	// 8304DC40: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304DC44: 4BFAA46D  bl 0x82ff80b0
	ctx.lr = 0x8304DC48;
	sub_82FF80B0(ctx, base);
	// 8304DC48: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 8304DC4C: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 8304DC50: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8304DC54: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304DC58: 4BFFEB99  bl 0x8304c7f0
	ctx.lr = 0x8304DC5C;
	sub_8304C7F0(ctx, base);
	// 8304DC5C: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8304DC60: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 8304DC64: 396B2990  addi r11, r11, 0x2990
	ctx.r[11].s64 = ctx.r[11].s64 + 10640;
	// 8304DC68: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8304DC6C: 48000008  b 0x8304dc74
	pc = 0x8304DC74; continue 'dispatch;
	// 8304DC70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8304DC74: 915B0000  stw r10, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8304DC78: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304DC7C: 809B0000  lwz r4, 0(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304DC80: 4BFAB1F1  bl 0x82ff8e70
	ctx.lr = 0x8304DC84;
	sub_82FF8E70(ctx, base);
	// 8304DC84: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8304DC88: 389F0050  addi r4, r31, 0x50
	ctx.r[4].s64 = ctx.r[31].s64 + 80;
	// 8304DC8C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304DC90: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8304DC94: 4BFAB8E5  bl 0x82ff9578
	ctx.lr = 0x8304DC98;
	sub_82FF9578(ctx, base);
	// 8304DC98: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8304DC9C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8304DCA0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304DCA4: 40990028  ble cr6, 0x8304dccc
	if !ctx.cr[6].gt {
	pc = 0x8304DCCC; continue 'dispatch;
	}
	// 8304DCA8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304DCAC: 4804B475  bl 0x83099120
	ctx.lr = 0x8304DCB0;
	sub_83099120(ctx, base);
	// 8304DCB0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8304DCB4: 807B0000  lwz r3, 0(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304DCB8: 4BFED499  bl 0x8303b150
	ctx.lr = 0x8304DCBC;
	sub_8303B150(ctx, base);
	// 8304DCBC: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8304DCC0: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 8304DCC4: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8304DCC8: 4198FFE0  blt cr6, 0x8304dca8
	if ctx.cr[6].lt {
	pc = 0x8304DCA8; continue 'dispatch;
	}
	// 8304DCCC: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 8304DCD0: 4815A4E0  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304DCD4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304DCD4 size=44
    let mut pc: u32 = 0x8304DCD4;
    'dispatch: loop {
        match pc {
            0x8304DCD4 => {
    //   block [0x8304DCD4..0x8304DD00)
	// 8304DCD4: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 8304DCD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304DCDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304DCE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304DCE4: 809F0054  lwz r4, 0x54(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8304DCE8: 807F0058  lwz r3, 0x58(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 8304DCEC: 4BF8A5F5  bl 0x82fd82e0
	ctx.lr = 0x8304DCF0;
	sub_82FD82E0(ctx, base);
	// 8304DCF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8304DCF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304DCF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304DCFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304DD00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8304DD00 size=8
    let mut pc: u32 = 0x8304DD00;
    'dispatch: loop {
        match pc {
            0x8304DD00 => {
    //   block [0x8304DD00..0x8304DD08)
	// 8304DD00: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304DD04: 82162CB8  lwz r16, 0x2cb8(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(11448 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304DD08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304DD08 size=268
    let mut pc: u32 = 0x8304DD08;
    'dispatch: loop {
        match pc {
            0x8304DD08 => {
    //   block [0x8304DD08..0x8304DE14)
	// 8304DD08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304DD0C: 4815A451  bl 0x831a815c
	ctx.lr = 0x8304DD10;
	sub_831A8130(ctx, base);
	// 8304DD10: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 8304DD14: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304DD18: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8304DD1C: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 8304DD20: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8304DD24: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8304DD28: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304DD2C: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 8304DD30: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 8304DD34: 4BFAB9FD  bl 0x82ff9730
	ctx.lr = 0x8304DD38;
	sub_82FF9730(ctx, base);
	// 8304DD38: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304DD3C: 418200D0  beq 0x8304de0c
	if ctx.cr[0].eq {
	pc = 0x8304DE0C; continue 'dispatch;
	}
	// 8304DD40: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304DD44: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8304DD48: 409A006C  bne cr6, 0x8304ddb4
	if !ctx.cr[6].eq {
	pc = 0x8304DDB4; continue 'dispatch;
	}
	// 8304DD4C: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 8304DD50: 40980008  bge cr6, 0x8304dd58
	if !ctx.cr[6].lt {
	pc = 0x8304DD58; continue 'dispatch;
	}
	// 8304DD54: 3B800010  li r28, 0x10
	ctx.r[28].s64 = 16;
	// 8304DD58: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304DD5C: 4BFAA355  bl 0x82ff80b0
	ctx.lr = 0x8304DD60;
	sub_82FF80B0(ctx, base);
	// 8304DD60: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8304DD64: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 8304DD68: 909F0054  stw r4, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[4].u32 ) };
	// 8304DD6C: 4BF8A52D  bl 0x82fd8298
	ctx.lr = 0x8304DD70;
	sub_82FD8298(ctx, base);
	// 8304DD70: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8304DD74: 93DF0058  stw r30, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[30].u32 ) };
	// 8304DD78: 41820034  beq 0x8304ddac
	if ctx.cr[0].eq {
	pc = 0x8304DDAC; continue 'dispatch;
	}
	// 8304DD7C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304DD80: 4BFAA331  bl 0x82ff80b0
	ctx.lr = 0x8304DD84;
	sub_82FF80B0(ctx, base);
	// 8304DD84: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 8304DD88: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 8304DD8C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8304DD90: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304DD94: 4BFFEA5D  bl 0x8304c7f0
	ctx.lr = 0x8304DD98;
	sub_8304C7F0(ctx, base);
	// 8304DD98: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8304DD9C: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 8304DDA0: 396B2990  addi r11, r11, 0x2990
	ctx.r[11].s64 = ctx.r[11].s64 + 10640;
	// 8304DDA4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8304DDA8: 48000008  b 0x8304ddb0
	pc = 0x8304DDB0; continue 'dispatch;
	// 8304DDAC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8304DDB0: 915B0000  stw r10, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8304DDB4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304DDB8: 809B0000  lwz r4, 0(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304DDBC: 4BFAB0B5  bl 0x82ff8e70
	ctx.lr = 0x8304DDC0;
	sub_82FF8E70(ctx, base);
	// 8304DDC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8304DDC4: 389F0050  addi r4, r31, 0x50
	ctx.r[4].s64 = ctx.r[31].s64 + 80;
	// 8304DDC8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304DDCC: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8304DDD0: 4BFAB7A9  bl 0x82ff9578
	ctx.lr = 0x8304DDD4;
	sub_82FF9578(ctx, base);
	// 8304DDD4: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8304DDD8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8304DDDC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304DDE0: 4099002C  ble cr6, 0x8304de0c
	if !ctx.cr[6].gt {
	pc = 0x8304DE0C; continue 'dispatch;
	}
	// 8304DDE4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8304DDE8: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 8304DDEC: 4804B69D  bl 0x83099488
	ctx.lr = 0x8304DDF0;
	sub_83099488(ctx, base);
	// 8304DDF0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8304DDF4: 807B0000  lwz r3, 0(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304DDF8: 4BFED359  bl 0x8303b150
	ctx.lr = 0x8304DDFC;
	sub_8303B150(ctx, base);
	// 8304DDFC: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8304DE00: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 8304DE04: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8304DE08: 4198FFDC  blt cr6, 0x8304dde4
	if ctx.cr[6].lt {
	pc = 0x8304DDE4; continue 'dispatch;
	}
	// 8304DE0C: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 8304DE10: 4815A39C  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304DE14(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304DE14 size=44
    let mut pc: u32 = 0x8304DE14;
    'dispatch: loop {
        match pc {
            0x8304DE14 => {
    //   block [0x8304DE14..0x8304DE40)
	// 8304DE14: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 8304DE18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304DE1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304DE20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304DE24: 809F0054  lwz r4, 0x54(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8304DE28: 807F0058  lwz r3, 0x58(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 8304DE2C: 4BF8A4B5  bl 0x82fd82e0
	ctx.lr = 0x8304DE30;
	sub_82FD82E0(ctx, base);
	// 8304DE30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8304DE34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304DE38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304DE3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304DE40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8304DE40 size=8
    let mut pc: u32 = 0x8304DE40;
    'dispatch: loop {
        match pc {
            0x8304DE40 => {
    //   block [0x8304DE40..0x8304DE48)
	// 8304DE40: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304DE44: 82162D00  lwz r16, 0x2d00(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(11520 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304DE48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304DE48 size=272
    let mut pc: u32 = 0x8304DE48;
    'dispatch: loop {
        match pc {
            0x8304DE48 => {
    //   block [0x8304DE48..0x8304DF58)
	// 8304DE48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304DE4C: 4815A315  bl 0x831a8160
	ctx.lr = 0x8304DE50;
	sub_831A8130(ctx, base);
	// 8304DE50: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 8304DE54: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304DE58: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 8304DE5C: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 8304DE60: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8304DE64: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8304DE68: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304DE6C: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 8304DE70: 4BFAB8C1  bl 0x82ff9730
	ctx.lr = 0x8304DE74;
	sub_82FF9730(ctx, base);
	// 8304DE74: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304DE78: 418200D8  beq 0x8304df50
	if ctx.cr[0].eq {
	pc = 0x8304DF50; continue 'dispatch;
	}
	// 8304DE7C: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304DE80: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8304DE84: 409A006C  bne cr6, 0x8304def0
	if !ctx.cr[6].eq {
	pc = 0x8304DEF0; continue 'dispatch;
	}
	// 8304DE88: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 8304DE8C: 40980008  bge cr6, 0x8304de94
	if !ctx.cr[6].lt {
	pc = 0x8304DE94; continue 'dispatch;
	}
	// 8304DE90: 3B800010  li r28, 0x10
	ctx.r[28].s64 = 16;
	// 8304DE94: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304DE98: 4BFAA219  bl 0x82ff80b0
	ctx.lr = 0x8304DE9C;
	sub_82FF80B0(ctx, base);
	// 8304DE9C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8304DEA0: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 8304DEA4: 909F0054  stw r4, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[4].u32 ) };
	// 8304DEA8: 4BF8A3F1  bl 0x82fd8298
	ctx.lr = 0x8304DEAC;
	sub_82FD8298(ctx, base);
	// 8304DEAC: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8304DEB0: 93DF0058  stw r30, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[30].u32 ) };
	// 8304DEB4: 41820034  beq 0x8304dee8
	if ctx.cr[0].eq {
	pc = 0x8304DEE8; continue 'dispatch;
	}
	// 8304DEB8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304DEBC: 4BFAA1F5  bl 0x82ff80b0
	ctx.lr = 0x8304DEC0;
	sub_82FF80B0(ctx, base);
	// 8304DEC0: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 8304DEC4: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 8304DEC8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8304DECC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304DED0: 4BFFE921  bl 0x8304c7f0
	ctx.lr = 0x8304DED4;
	sub_8304C7F0(ctx, base);
	// 8304DED4: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8304DED8: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 8304DEDC: 396B2990  addi r11, r11, 0x2990
	ctx.r[11].s64 = ctx.r[11].s64 + 10640;
	// 8304DEE0: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8304DEE4: 48000008  b 0x8304deec
	pc = 0x8304DEEC; continue 'dispatch;
	// 8304DEE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8304DEEC: 915A0000  stw r10, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8304DEF0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304DEF4: 809A0000  lwz r4, 0(r26)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304DEF8: 4BFAAF79  bl 0x82ff8e70
	ctx.lr = 0x8304DEFC;
	sub_82FF8E70(ctx, base);
	// 8304DEFC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8304DF00: 389F0050  addi r4, r31, 0x50
	ctx.r[4].s64 = ctx.r[31].s64 + 80;
	// 8304DF04: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304DF08: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8304DF0C: 4BFAB66D  bl 0x82ff9578
	ctx.lr = 0x8304DF10;
	sub_82FF9578(ctx, base);
	// 8304DF10: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8304DF14: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8304DF18: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304DF1C: 40990034  ble cr6, 0x8304df50
	if !ctx.cr[6].gt {
	pc = 0x8304DF50; continue 'dispatch;
	}
	// 8304DF20: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8304DF24: 3B8B35B8  addi r28, r11, 0x35b8
	ctx.r[28].s64 = ctx.r[11].s64 + 13752;
	// 8304DF28: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304DF2C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8304DF30: 4BFABD91  bl 0x82ff9cc0
	ctx.lr = 0x8304DF34;
	sub_82FF9CC0(ctx, base);
	// 8304DF34: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8304DF38: 807A0000  lwz r3, 0(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304DF3C: 4BFED215  bl 0x8303b150
	ctx.lr = 0x8304DF40;
	sub_8303B150(ctx, base);
	// 8304DF40: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8304DF44: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 8304DF48: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8304DF4C: 4198FFDC  blt cr6, 0x8304df28
	if ctx.cr[6].lt {
	pc = 0x8304DF28; continue 'dispatch;
	}
	// 8304DF50: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 8304DF54: 4815A25C  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304DF58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304DF58 size=44
    let mut pc: u32 = 0x8304DF58;
    'dispatch: loop {
        match pc {
            0x8304DF58 => {
    //   block [0x8304DF58..0x8304DF84)
	// 8304DF58: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 8304DF5C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304DF60: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304DF64: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304DF68: 809F0054  lwz r4, 0x54(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8304DF6C: 807F0058  lwz r3, 0x58(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 8304DF70: 4BF8A371  bl 0x82fd82e0
	ctx.lr = 0x8304DF74;
	sub_82FD82E0(ctx, base);
	// 8304DF74: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8304DF78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304DF7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304DF80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304DF88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8304DF88 size=8
    let mut pc: u32 = 0x8304DF88;
    'dispatch: loop {
        match pc {
            0x8304DF88 => {
    //   block [0x8304DF88..0x8304DF90)
	// 8304DF88: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304DF8C: 82162D48  lwz r16, 0x2d48(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(11592 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304DF90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304DF90 size=272
    let mut pc: u32 = 0x8304DF90;
    'dispatch: loop {
        match pc {
            0x8304DF90 => {
    //   block [0x8304DF90..0x8304E0A0)
	// 8304DF90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304DF94: 4815A1CD  bl 0x831a8160
	ctx.lr = 0x8304DF98;
	sub_831A8130(ctx, base);
	// 8304DF98: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 8304DF9C: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304DFA0: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 8304DFA4: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 8304DFA8: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8304DFAC: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8304DFB0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304DFB4: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 8304DFB8: 4BFAB779  bl 0x82ff9730
	ctx.lr = 0x8304DFBC;
	sub_82FF9730(ctx, base);
	// 8304DFBC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304DFC0: 418200D8  beq 0x8304e098
	if ctx.cr[0].eq {
	pc = 0x8304E098; continue 'dispatch;
	}
	// 8304DFC4: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304DFC8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8304DFCC: 409A006C  bne cr6, 0x8304e038
	if !ctx.cr[6].eq {
	pc = 0x8304E038; continue 'dispatch;
	}
	// 8304DFD0: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 8304DFD4: 40980008  bge cr6, 0x8304dfdc
	if !ctx.cr[6].lt {
	pc = 0x8304DFDC; continue 'dispatch;
	}
	// 8304DFD8: 3B800010  li r28, 0x10
	ctx.r[28].s64 = 16;
	// 8304DFDC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304DFE0: 4BFAA0D1  bl 0x82ff80b0
	ctx.lr = 0x8304DFE4;
	sub_82FF80B0(ctx, base);
	// 8304DFE4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8304DFE8: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 8304DFEC: 909F0054  stw r4, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[4].u32 ) };
	// 8304DFF0: 4BF8A2A9  bl 0x82fd8298
	ctx.lr = 0x8304DFF4;
	sub_82FD8298(ctx, base);
	// 8304DFF4: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8304DFF8: 93DF0058  stw r30, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[30].u32 ) };
	// 8304DFFC: 41820034  beq 0x8304e030
	if ctx.cr[0].eq {
	pc = 0x8304E030; continue 'dispatch;
	}
	// 8304E000: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304E004: 4BFAA0AD  bl 0x82ff80b0
	ctx.lr = 0x8304E008;
	sub_82FF80B0(ctx, base);
	// 8304E008: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 8304E00C: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 8304E010: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8304E014: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304E018: 4BFFE7D9  bl 0x8304c7f0
	ctx.lr = 0x8304E01C;
	sub_8304C7F0(ctx, base);
	// 8304E01C: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8304E020: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 8304E024: 396B2990  addi r11, r11, 0x2990
	ctx.r[11].s64 = ctx.r[11].s64 + 10640;
	// 8304E028: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8304E02C: 48000008  b 0x8304e034
	pc = 0x8304E034; continue 'dispatch;
	// 8304E030: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8304E034: 915A0000  stw r10, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8304E038: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304E03C: 809A0000  lwz r4, 0(r26)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304E040: 4BFAAE31  bl 0x82ff8e70
	ctx.lr = 0x8304E044;
	sub_82FF8E70(ctx, base);
	// 8304E044: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8304E048: 389F0050  addi r4, r31, 0x50
	ctx.r[4].s64 = ctx.r[31].s64 + 80;
	// 8304E04C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304E050: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8304E054: 4BFAB525  bl 0x82ff9578
	ctx.lr = 0x8304E058;
	sub_82FF9578(ctx, base);
	// 8304E058: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8304E05C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8304E060: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304E064: 40990034  ble cr6, 0x8304e098
	if !ctx.cr[6].gt {
	pc = 0x8304E098; continue 'dispatch;
	}
	// 8304E068: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8304E06C: 3B8B35B0  addi r28, r11, 0x35b0
	ctx.r[28].s64 = ctx.r[11].s64 + 13744;
	// 8304E070: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304E074: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8304E078: 4BFABC49  bl 0x82ff9cc0
	ctx.lr = 0x8304E07C;
	sub_82FF9CC0(ctx, base);
	// 8304E07C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8304E080: 807A0000  lwz r3, 0(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304E084: 4BFED0CD  bl 0x8303b150
	ctx.lr = 0x8304E088;
	sub_8303B150(ctx, base);
	// 8304E088: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8304E08C: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 8304E090: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8304E094: 4198FFDC  blt cr6, 0x8304e070
	if ctx.cr[6].lt {
	pc = 0x8304E070; continue 'dispatch;
	}
	// 8304E098: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 8304E09C: 4815A114  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304E0A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304E0A0 size=44
    let mut pc: u32 = 0x8304E0A0;
    'dispatch: loop {
        match pc {
            0x8304E0A0 => {
    //   block [0x8304E0A0..0x8304E0CC)
	// 8304E0A0: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 8304E0A4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304E0A8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304E0AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304E0B0: 809F0054  lwz r4, 0x54(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8304E0B4: 807F0058  lwz r3, 0x58(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 8304E0B8: 4BF8A229  bl 0x82fd82e0
	ctx.lr = 0x8304E0BC;
	sub_82FD82E0(ctx, base);
	// 8304E0BC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8304E0C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304E0C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304E0C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304E0D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8304E0D0 size=8
    let mut pc: u32 = 0x8304E0D0;
    'dispatch: loop {
        match pc {
            0x8304E0D0 => {
    //   block [0x8304E0D0..0x8304E0D8)
	// 8304E0D0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304E0D4: 82162D90  lwz r16, 0x2d90(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(11664 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304E0D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304E0D8 size=256
    let mut pc: u32 = 0x8304E0D8;
    'dispatch: loop {
        match pc {
            0x8304E0D8 => {
    //   block [0x8304E0D8..0x8304E1D8)
	// 8304E0D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304E0DC: 4815A089  bl 0x831a8164
	ctx.lr = 0x8304E0E0;
	sub_831A8130(ctx, base);
	// 8304E0E0: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 8304E0E4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304E0E8: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8304E0EC: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 8304E0F0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8304E0F4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304E0F8: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 8304E0FC: 4BFAB635  bl 0x82ff9730
	ctx.lr = 0x8304E100;
	sub_82FF9730(ctx, base);
	// 8304E100: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304E104: 418200CC  beq 0x8304e1d0
	if ctx.cr[0].eq {
	pc = 0x8304E1D0; continue 'dispatch;
	}
	// 8304E108: 389F0054  addi r4, r31, 0x54
	ctx.r[4].s64 = ctx.r[31].s64 + 84;
	// 8304E10C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304E110: 4BFAB469  bl 0x82ff9578
	ctx.lr = 0x8304E114;
	sub_82FF9578(ctx, base);
	// 8304E114: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304E118: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8304E11C: 409A0050  bne cr6, 0x8304e16c
	if !ctx.cr[6].eq {
	pc = 0x8304E16C; continue 'dispatch;
	}
	// 8304E120: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304E124: 4BFA9F8D  bl 0x82ff80b0
	ctx.lr = 0x8304E128;
	sub_82FF80B0(ctx, base);
	// 8304E128: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8304E12C: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 8304E130: 909F0058  stw r4, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[4].u32 ) };
	// 8304E134: 4BF8A165  bl 0x82fd8298
	ctx.lr = 0x8304E138;
	sub_82FD8298(ctx, base);
	// 8304E138: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8304E13C: 93BF005C  stw r29, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[29].u32 ) };
	// 8304E140: 41820024  beq 0x8304e164
	if ctx.cr[0].eq {
	pc = 0x8304E164; continue 'dispatch;
	}
	// 8304E144: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304E148: 4BFA9F69  bl 0x82ff80b0
	ctx.lr = 0x8304E14C;
	sub_82FF80B0(ctx, base);
	// 8304E14C: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 8304E150: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 8304E154: 809F0054  lwz r4, 0x54(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8304E158: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304E15C: 4BFFEE05  bl 0x8304cf60
	ctx.lr = 0x8304E160;
	sub_8304CF60(ctx, base);
	// 8304E160: 48000008  b 0x8304e168
	pc = 0x8304E168; continue 'dispatch;
	// 8304E164: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8304E168: 907C0000  stw r3, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8304E16C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304E170: 809C0000  lwz r4, 0(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304E174: 4BFAACFD  bl 0x82ff8e70
	ctx.lr = 0x8304E178;
	sub_82FF8E70(ctx, base);
	// 8304E178: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8304E17C: 389F0050  addi r4, r31, 0x50
	ctx.r[4].s64 = ctx.r[31].s64 + 80;
	// 8304E180: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304E184: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8304E188: 4BFAB3F1  bl 0x82ff9578
	ctx.lr = 0x8304E18C;
	sub_82FF9578(ctx, base);
	// 8304E18C: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8304E190: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8304E194: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304E198: 40990038  ble cr6, 0x8304e1d0
	if !ctx.cr[6].gt {
	pc = 0x8304E1D0; continue 'dispatch;
	}
	// 8304E19C: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8304E1A0: 3B6B335C  addi r27, r11, 0x335c
	ctx.r[27].s64 = ctx.r[11].s64 + 13148;
	// 8304E1A4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304E1A8: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8304E1AC: 4BFABB15  bl 0x82ff9cc0
	ctx.lr = 0x8304E1B0;
	sub_82FF9CC0(ctx, base);
	// 8304E1B0: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8304E1B4: 807C0000  lwz r3, 0(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304E1B8: 80850008  lwz r4, 8(r5)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) } as u64;
	// 8304E1BC: 4BFDEC8D  bl 0x8302ce48
	ctx.lr = 0x8304E1C0;
	sub_8302CE48(ctx, base);
	// 8304E1C0: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8304E1C4: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 8304E1C8: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8304E1CC: 4198FFD8  blt cr6, 0x8304e1a4
	if ctx.cr[6].lt {
	pc = 0x8304E1A4; continue 'dispatch;
	}
	// 8304E1D0: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 8304E1D4: 48159FE0  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304E1D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304E1D8 size=44
    let mut pc: u32 = 0x8304E1D8;
    'dispatch: loop {
        match pc {
            0x8304E1D8 => {
    //   block [0x8304E1D8..0x8304E204)
	// 8304E1D8: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 8304E1DC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304E1E0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304E1E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304E1E8: 809F0058  lwz r4, 0x58(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 8304E1EC: 807F005C  lwz r3, 0x5c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 8304E1F0: 4BF8A0F1  bl 0x82fd82e0
	ctx.lr = 0x8304E1F4;
	sub_82FD82E0(ctx, base);
	// 8304E1F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8304E1F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304E1FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304E200: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304E208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8304E208 size=8
    let mut pc: u32 = 0x8304E208;
    'dispatch: loop {
        match pc {
            0x8304E208 => {
    //   block [0x8304E208..0x8304E210)
	// 8304E208: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304E20C: 82162DD8  lwz r16, 0x2dd8(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(11736 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304E210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304E210 size=260
    let mut pc: u32 = 0x8304E210;
    'dispatch: loop {
        match pc {
            0x8304E210 => {
    //   block [0x8304E210..0x8304E314)
	// 8304E210: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304E214: 48159F51  bl 0x831a8164
	ctx.lr = 0x8304E218;
	sub_831A8130(ctx, base);
	// 8304E218: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 8304E21C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304E220: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8304E224: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 8304E228: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8304E22C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304E230: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 8304E234: 4BFAB4FD  bl 0x82ff9730
	ctx.lr = 0x8304E238;
	sub_82FF9730(ctx, base);
	// 8304E238: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304E23C: 418200D0  beq 0x8304e30c
	if ctx.cr[0].eq {
	pc = 0x8304E30C; continue 'dispatch;
	}
	// 8304E240: 389F0054  addi r4, r31, 0x54
	ctx.r[4].s64 = ctx.r[31].s64 + 84;
	// 8304E244: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304E248: 4BFAB331  bl 0x82ff9578
	ctx.lr = 0x8304E24C;
	sub_82FF9578(ctx, base);
	// 8304E24C: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304E250: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8304E254: 409A0050  bne cr6, 0x8304e2a4
	if !ctx.cr[6].eq {
	pc = 0x8304E2A4; continue 'dispatch;
	}
	// 8304E258: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304E25C: 4BFA9E55  bl 0x82ff80b0
	ctx.lr = 0x8304E260;
	sub_82FF80B0(ctx, base);
	// 8304E260: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8304E264: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 8304E268: 909F0058  stw r4, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[4].u32 ) };
	// 8304E26C: 4BF8A02D  bl 0x82fd8298
	ctx.lr = 0x8304E270;
	sub_82FD8298(ctx, base);
	// 8304E270: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8304E274: 93BF005C  stw r29, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[29].u32 ) };
	// 8304E278: 41820024  beq 0x8304e29c
	if ctx.cr[0].eq {
	pc = 0x8304E29C; continue 'dispatch;
	}
	// 8304E27C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304E280: 4BFA9E31  bl 0x82ff80b0
	ctx.lr = 0x8304E284;
	sub_82FF80B0(ctx, base);
	// 8304E284: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 8304E288: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 8304E28C: 809F0054  lwz r4, 0x54(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8304E290: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304E294: 4BFFECCD  bl 0x8304cf60
	ctx.lr = 0x8304E298;
	sub_8304CF60(ctx, base);
	// 8304E298: 48000008  b 0x8304e2a0
	pc = 0x8304E2A0; continue 'dispatch;
	// 8304E29C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8304E2A0: 907C0000  stw r3, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8304E2A4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304E2A8: 809C0000  lwz r4, 0(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304E2AC: 4BFAABC5  bl 0x82ff8e70
	ctx.lr = 0x8304E2B0;
	sub_82FF8E70(ctx, base);
	// 8304E2B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8304E2B4: 389F0050  addi r4, r31, 0x50
	ctx.r[4].s64 = ctx.r[31].s64 + 80;
	// 8304E2B8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304E2BC: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8304E2C0: 4BFAB2B9  bl 0x82ff9578
	ctx.lr = 0x8304E2C4;
	sub_82FF9578(ctx, base);
	// 8304E2C4: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8304E2C8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8304E2CC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304E2D0: 4099003C  ble cr6, 0x8304e30c
	if !ctx.cr[6].gt {
	pc = 0x8304E30C; continue 'dispatch;
	}
	// 8304E2D4: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8304E2D8: 3B6B34C0  addi r27, r11, 0x34c0
	ctx.r[27].s64 = ctx.r[11].s64 + 13504;
	// 8304E2DC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304E2E0: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8304E2E4: 4BFAB9DD  bl 0x82ff9cc0
	ctx.lr = 0x8304E2E8;
	sub_82FF9CC0(ctx, base);
	// 8304E2E8: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8304E2EC: 807C0000  lwz r3, 0(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304E2F0: 81650028  lwz r11, 0x28(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(40 as u32) ) } as u64;
	// 8304E2F4: 808B0010  lwz r4, 0x10(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8304E2F8: 4BFDEB51  bl 0x8302ce48
	ctx.lr = 0x8304E2FC;
	sub_8302CE48(ctx, base);
	// 8304E2FC: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8304E300: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 8304E304: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8304E308: 4198FFD4  blt cr6, 0x8304e2dc
	if ctx.cr[6].lt {
	pc = 0x8304E2DC; continue 'dispatch;
	}
	// 8304E30C: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 8304E310: 48159EA4  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304E314(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304E314 size=44
    let mut pc: u32 = 0x8304E314;
    'dispatch: loop {
        match pc {
            0x8304E314 => {
    //   block [0x8304E314..0x8304E340)
	// 8304E314: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 8304E318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304E31C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304E320: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304E324: 809F0058  lwz r4, 0x58(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 8304E328: 807F005C  lwz r3, 0x5c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 8304E32C: 4BF89FB5  bl 0x82fd82e0
	ctx.lr = 0x8304E330;
	sub_82FD82E0(ctx, base);
	// 8304E330: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8304E334: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304E338: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304E33C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304E340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8304E340 size=8
    let mut pc: u32 = 0x8304E340;
    'dispatch: loop {
        match pc {
            0x8304E340 => {
    //   block [0x8304E340..0x8304E348)
	// 8304E340: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304E344: 82162E20  lwz r16, 0x2e20(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(11808 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304E348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304E348 size=276
    let mut pc: u32 = 0x8304E348;
    'dispatch: loop {
        match pc {
            0x8304E348 => {
    //   block [0x8304E348..0x8304E45C)
	// 8304E348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304E34C: 48159E15  bl 0x831a8160
	ctx.lr = 0x8304E350;
	sub_831A8130(ctx, base);
	// 8304E350: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 8304E354: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304E358: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8304E35C: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 8304E360: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8304E364: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304E368: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8304E36C: 4BFAB3C5  bl 0x82ff9730
	ctx.lr = 0x8304E370;
	sub_82FF9730(ctx, base);
	// 8304E370: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304E374: 418200E0  beq 0x8304e454
	if ctx.cr[0].eq {
	pc = 0x8304E454; continue 'dispatch;
	}
	// 8304E378: 389F0054  addi r4, r31, 0x54
	ctx.r[4].s64 = ctx.r[31].s64 + 84;
	// 8304E37C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304E380: 4BFAB1F9  bl 0x82ff9578
	ctx.lr = 0x8304E384;
	sub_82FF9578(ctx, base);
	// 8304E384: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304E388: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8304E38C: 409A0050  bne cr6, 0x8304e3dc
	if !ctx.cr[6].eq {
	pc = 0x8304E3DC; continue 'dispatch;
	}
	// 8304E390: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304E394: 4BFA9D1D  bl 0x82ff80b0
	ctx.lr = 0x8304E398;
	sub_82FF80B0(ctx, base);
	// 8304E398: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8304E39C: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 8304E3A0: 909F0058  stw r4, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[4].u32 ) };
	// 8304E3A4: 4BF89EF5  bl 0x82fd8298
	ctx.lr = 0x8304E3A8;
	sub_82FD8298(ctx, base);
	// 8304E3A8: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8304E3AC: 93BF005C  stw r29, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[29].u32 ) };
	// 8304E3B0: 41820024  beq 0x8304e3d4
	if ctx.cr[0].eq {
	pc = 0x8304E3D4; continue 'dispatch;
	}
	// 8304E3B4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304E3B8: 4BFA9CF9  bl 0x82ff80b0
	ctx.lr = 0x8304E3BC;
	sub_82FF80B0(ctx, base);
	// 8304E3BC: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 8304E3C0: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8304E3C4: 809F0054  lwz r4, 0x54(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8304E3C8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304E3CC: 4BFFEB95  bl 0x8304cf60
	ctx.lr = 0x8304E3D0;
	sub_8304CF60(ctx, base);
	// 8304E3D0: 48000008  b 0x8304e3d8
	pc = 0x8304E3D8; continue 'dispatch;
	// 8304E3D4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8304E3D8: 907B0000  stw r3, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8304E3DC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304E3E0: 809B0000  lwz r4, 0(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304E3E4: 4BFAAA8D  bl 0x82ff8e70
	ctx.lr = 0x8304E3E8;
	sub_82FF8E70(ctx, base);
	// 8304E3E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8304E3EC: 389F0050  addi r4, r31, 0x50
	ctx.r[4].s64 = ctx.r[31].s64 + 80;
	// 8304E3F0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304E3F4: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8304E3F8: 4BFAB181  bl 0x82ff9578
	ctx.lr = 0x8304E3FC;
	sub_82FF9578(ctx, base);
	// 8304E3FC: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8304E400: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8304E404: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304E408: 4099004C  ble cr6, 0x8304e454
	if !ctx.cr[6].gt {
	pc = 0x8304E454; continue 'dispatch;
	}
	// 8304E40C: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8304E410: 3B4B3588  addi r26, r11, 0x3588
	ctx.r[26].s64 = ctx.r[11].s64 + 13704;
	// 8304E414: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304E418: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8304E41C: 4BFAB8A5  bl 0x82ff9cc0
	ctx.lr = 0x8304E420;
	sub_82FF9CC0(ctx, base);
	// 8304E420: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8304E424: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304E428: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8304E42C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8304E430: 4E800421  bctrl
	ctx.lr = 0x8304E434;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8304E434: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8304E438: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8304E43C: 807B0000  lwz r3, 0(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304E440: 4BFDEA09  bl 0x8302ce48
	ctx.lr = 0x8304E444;
	sub_8302CE48(ctx, base);
	// 8304E444: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8304E448: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 8304E44C: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8304E450: 4198FFC4  blt cr6, 0x8304e414
	if ctx.cr[6].lt {
	pc = 0x8304E414; continue 'dispatch;
	}
	// 8304E454: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 8304E458: 48159D58  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304E45C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304E45C size=44
    let mut pc: u32 = 0x8304E45C;
    'dispatch: loop {
        match pc {
            0x8304E45C => {
    //   block [0x8304E45C..0x8304E488)
	// 8304E45C: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 8304E460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304E464: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304E468: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304E46C: 809F0058  lwz r4, 0x58(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 8304E470: 807F005C  lwz r3, 0x5c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 8304E474: 4BF89E6D  bl 0x82fd82e0
	ctx.lr = 0x8304E478;
	sub_82FD82E0(ctx, base);
	// 8304E478: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8304E47C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304E480: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304E484: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304E488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8304E488 size=8
    let mut pc: u32 = 0x8304E488;
    'dispatch: loop {
        match pc {
            0x8304E488 => {
    //   block [0x8304E488..0x8304E490)
	// 8304E488: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304E48C: 82162E68  lwz r16, 0x2e68(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(11880 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304E490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304E490 size=256
    let mut pc: u32 = 0x8304E490;
    'dispatch: loop {
        match pc {
            0x8304E490 => {
    //   block [0x8304E490..0x8304E590)
	// 8304E490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304E494: 48159CD1  bl 0x831a8164
	ctx.lr = 0x8304E498;
	sub_831A8130(ctx, base);
	// 8304E498: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 8304E49C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304E4A0: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8304E4A4: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 8304E4A8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8304E4AC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304E4B0: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 8304E4B4: 4BFAB27D  bl 0x82ff9730
	ctx.lr = 0x8304E4B8;
	sub_82FF9730(ctx, base);
	// 8304E4B8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304E4BC: 418200CC  beq 0x8304e588
	if ctx.cr[0].eq {
	pc = 0x8304E588; continue 'dispatch;
	}
	// 8304E4C0: 389F0054  addi r4, r31, 0x54
	ctx.r[4].s64 = ctx.r[31].s64 + 84;
	// 8304E4C4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304E4C8: 4BFAB0B1  bl 0x82ff9578
	ctx.lr = 0x8304E4CC;
	sub_82FF9578(ctx, base);
	// 8304E4CC: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304E4D0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8304E4D4: 409A0050  bne cr6, 0x8304e524
	if !ctx.cr[6].eq {
	pc = 0x8304E524; continue 'dispatch;
	}
	// 8304E4D8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304E4DC: 4BFA9BD5  bl 0x82ff80b0
	ctx.lr = 0x8304E4E0;
	sub_82FF80B0(ctx, base);
	// 8304E4E0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8304E4E4: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 8304E4E8: 909F0058  stw r4, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[4].u32 ) };
	// 8304E4EC: 4BF89DAD  bl 0x82fd8298
	ctx.lr = 0x8304E4F0;
	sub_82FD8298(ctx, base);
	// 8304E4F0: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8304E4F4: 93BF005C  stw r29, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[29].u32 ) };
	// 8304E4F8: 41820024  beq 0x8304e51c
	if ctx.cr[0].eq {
	pc = 0x8304E51C; continue 'dispatch;
	}
	// 8304E4FC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304E500: 4BFA9BB1  bl 0x82ff80b0
	ctx.lr = 0x8304E504;
	sub_82FF80B0(ctx, base);
	// 8304E504: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 8304E508: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 8304E50C: 809F0054  lwz r4, 0x54(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8304E510: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304E514: 4BFFEA4D  bl 0x8304cf60
	ctx.lr = 0x8304E518;
	sub_8304CF60(ctx, base);
	// 8304E518: 48000008  b 0x8304e520
	pc = 0x8304E520; continue 'dispatch;
	// 8304E51C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8304E520: 907C0000  stw r3, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8304E524: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304E528: 809C0000  lwz r4, 0(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304E52C: 4BFAA945  bl 0x82ff8e70
	ctx.lr = 0x8304E530;
	sub_82FF8E70(ctx, base);
	// 8304E530: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8304E534: 389F0050  addi r4, r31, 0x50
	ctx.r[4].s64 = ctx.r[31].s64 + 80;
	// 8304E538: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304E53C: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8304E540: 4BFAB039  bl 0x82ff9578
	ctx.lr = 0x8304E544;
	sub_82FF9578(ctx, base);
	// 8304E544: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8304E548: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8304E54C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304E550: 40990038  ble cr6, 0x8304e588
	if !ctx.cr[6].gt {
	pc = 0x8304E588; continue 'dispatch;
	}
	// 8304E554: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8304E558: 3B6B32CC  addi r27, r11, 0x32cc
	ctx.r[27].s64 = ctx.r[11].s64 + 13004;
	// 8304E55C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304E560: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8304E564: 4BFAB75D  bl 0x82ff9cc0
	ctx.lr = 0x8304E568;
	sub_82FF9CC0(ctx, base);
	// 8304E568: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8304E56C: 807C0000  lwz r3, 0(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304E570: 80850024  lwz r4, 0x24(r5)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(36 as u32) ) } as u64;
	// 8304E574: 4BFDE8D5  bl 0x8302ce48
	ctx.lr = 0x8304E578;
	sub_8302CE48(ctx, base);
	// 8304E578: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8304E57C: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 8304E580: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8304E584: 4198FFD8  blt cr6, 0x8304e55c
	if ctx.cr[6].lt {
	pc = 0x8304E55C; continue 'dispatch;
	}
	// 8304E588: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 8304E58C: 48159C28  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304E590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304E590 size=44
    let mut pc: u32 = 0x8304E590;
    'dispatch: loop {
        match pc {
            0x8304E590 => {
    //   block [0x8304E590..0x8304E5BC)
	// 8304E590: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 8304E594: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304E598: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304E59C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304E5A0: 809F0058  lwz r4, 0x58(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 8304E5A4: 807F005C  lwz r3, 0x5c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 8304E5A8: 4BF89D39  bl 0x82fd82e0
	ctx.lr = 0x8304E5AC;
	sub_82FD82E0(ctx, base);
	// 8304E5AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8304E5B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304E5B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304E5B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304E5C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8304E5C0 size=8
    let mut pc: u32 = 0x8304E5C0;
    'dispatch: loop {
        match pc {
            0x8304E5C0 => {
    //   block [0x8304E5C0..0x8304E5C8)
	// 8304E5C0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304E5C4: 82162EB0  lwz r16, 0x2eb0(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(11952 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304E5C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304E5C8 size=300
    let mut pc: u32 = 0x8304E5C8;
    'dispatch: loop {
        match pc {
            0x8304E5C8 => {
    //   block [0x8304E5C8..0x8304E6F4)
	// 8304E5C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304E5CC: 48159B95  bl 0x831a8160
	ctx.lr = 0x8304E5D0;
	sub_831A8130(ctx, base);
	// 8304E5D0: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 8304E5D4: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304E5D8: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8304E5DC: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 8304E5E0: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8304E5E4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304E5E8: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8304E5EC: 4BFAB145  bl 0x82ff9730
	ctx.lr = 0x8304E5F0;
	sub_82FF9730(ctx, base);
	// 8304E5F0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304E5F4: 418200F8  beq 0x8304e6ec
	if ctx.cr[0].eq {
	pc = 0x8304E6EC; continue 'dispatch;
	}
	// 8304E5F8: 389F0054  addi r4, r31, 0x54
	ctx.r[4].s64 = ctx.r[31].s64 + 84;
	// 8304E5FC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304E600: 4BFAAF79  bl 0x82ff9578
	ctx.lr = 0x8304E604;
	sub_82FF9578(ctx, base);
	// 8304E604: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304E608: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8304E60C: 409A0050  bne cr6, 0x8304e65c
	if !ctx.cr[6].eq {
	pc = 0x8304E65C; continue 'dispatch;
	}
	// 8304E610: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304E614: 4BFA9A9D  bl 0x82ff80b0
	ctx.lr = 0x8304E618;
	sub_82FF80B0(ctx, base);
	// 8304E618: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8304E61C: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 8304E620: 909F0058  stw r4, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[4].u32 ) };
	// 8304E624: 4BF89C75  bl 0x82fd8298
	ctx.lr = 0x8304E628;
	sub_82FD8298(ctx, base);
	// 8304E628: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8304E62C: 93BF005C  stw r29, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[29].u32 ) };
	// 8304E630: 41820024  beq 0x8304e654
	if ctx.cr[0].eq {
	pc = 0x8304E654; continue 'dispatch;
	}
	// 8304E634: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304E638: 4BFA9A79  bl 0x82ff80b0
	ctx.lr = 0x8304E63C;
	sub_82FF80B0(ctx, base);
	// 8304E63C: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 8304E640: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8304E644: 809F0054  lwz r4, 0x54(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8304E648: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304E64C: 4BFFE915  bl 0x8304cf60
	ctx.lr = 0x8304E650;
	sub_8304CF60(ctx, base);
	// 8304E650: 48000008  b 0x8304e658
	pc = 0x8304E658; continue 'dispatch;
	// 8304E654: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8304E658: 907B0000  stw r3, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8304E65C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304E660: 809B0000  lwz r4, 0(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304E664: 4BFAA80D  bl 0x82ff8e70
	ctx.lr = 0x8304E668;
	sub_82FF8E70(ctx, base);
	// 8304E668: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8304E66C: 389F0050  addi r4, r31, 0x50
	ctx.r[4].s64 = ctx.r[31].s64 + 80;
	// 8304E670: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304E674: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8304E678: 4BFAAF01  bl 0x82ff9578
	ctx.lr = 0x8304E67C;
	sub_82FF9578(ctx, base);
	// 8304E67C: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8304E680: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8304E684: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304E688: 40990064  ble cr6, 0x8304e6ec
	if !ctx.cr[6].gt {
	pc = 0x8304E6EC; continue 'dispatch;
	}
	// 8304E68C: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8304E690: 3B4B3598  addi r26, r11, 0x3598
	ctx.r[26].s64 = ctx.r[11].s64 + 13720;
	// 8304E694: 389F0058  addi r4, r31, 0x58
	ctx.r[4].s64 = ctx.r[31].s64 + 88;
	// 8304E698: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304E69C: 4BFAAEDD  bl 0x82ff9578
	ctx.lr = 0x8304E6A0;
	sub_82FF9578(ctx, base);
	// 8304E6A0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304E6A4: 4BFA99F5  bl 0x82ff8098
	ctx.lr = 0x8304E6A8;
	sub_82FF8098(ctx, base);
	// 8304E6A8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304E6AC: 809F0058  lwz r4, 0x58(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 8304E6B0: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8304E6B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8304E6B8: 4E800421  bctrl
	ctx.lr = 0x8304E6BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8304E6BC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8304E6C0: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8304E6C4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304E6C8: 4BFAB5F9  bl 0x82ff9cc0
	ctx.lr = 0x8304E6CC;
	sub_82FF9CC0(ctx, base);
	// 8304E6CC: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8304E6D0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8304E6D4: 807B0000  lwz r3, 0(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304E6D8: 4BFDE771  bl 0x8302ce48
	ctx.lr = 0x8304E6DC;
	sub_8302CE48(ctx, base);
	// 8304E6DC: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8304E6E0: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 8304E6E4: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8304E6E8: 4198FFAC  blt cr6, 0x8304e694
	if ctx.cr[6].lt {
	pc = 0x8304E694; continue 'dispatch;
	}
	// 8304E6EC: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 8304E6F0: 48159AC0  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304E6F4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304E6F4 size=44
    let mut pc: u32 = 0x8304E6F4;
    'dispatch: loop {
        match pc {
            0x8304E6F4 => {
    //   block [0x8304E6F4..0x8304E720)
	// 8304E6F4: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 8304E6F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304E6FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304E700: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304E704: 809F0058  lwz r4, 0x58(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 8304E708: 807F005C  lwz r3, 0x5c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 8304E70C: 4BF89BD5  bl 0x82fd82e0
	ctx.lr = 0x8304E710;
	sub_82FD82E0(ctx, base);
	// 8304E710: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8304E714: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304E718: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304E71C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304E720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8304E720 size=8
    let mut pc: u32 = 0x8304E720;
    'dispatch: loop {
        match pc {
            0x8304E720 => {
    //   block [0x8304E720..0x8304E728)
	// 8304E720: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304E724: 82162EF8  lwz r16, 0x2ef8(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(12024 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304E728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304E728 size=288
    let mut pc: u32 = 0x8304E728;
    'dispatch: loop {
        match pc {
            0x8304E728 => {
    //   block [0x8304E728..0x8304E848)
	// 8304E728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304E72C: 48159A35  bl 0x831a8160
	ctx.lr = 0x8304E730;
	sub_831A8130(ctx, base);
	// 8304E730: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 8304E734: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304E738: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8304E73C: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 8304E740: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8304E744: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304E748: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8304E74C: 4BFAAFE5  bl 0x82ff9730
	ctx.lr = 0x8304E750;
	sub_82FF9730(ctx, base);
	// 8304E750: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304E754: 418200EC  beq 0x8304e840
	if ctx.cr[0].eq {
	pc = 0x8304E840; continue 'dispatch;
	}
	// 8304E758: 389F0054  addi r4, r31, 0x54
	ctx.r[4].s64 = ctx.r[31].s64 + 84;
	// 8304E75C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304E760: 4BFAAE19  bl 0x82ff9578
	ctx.lr = 0x8304E764;
	sub_82FF9578(ctx, base);
	// 8304E764: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304E768: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8304E76C: 409A0050  bne cr6, 0x8304e7bc
	if !ctx.cr[6].eq {
	pc = 0x8304E7BC; continue 'dispatch;
	}
	// 8304E770: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304E774: 4BFA993D  bl 0x82ff80b0
	ctx.lr = 0x8304E778;
	sub_82FF80B0(ctx, base);
	// 8304E778: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8304E77C: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 8304E780: 909F0058  stw r4, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[4].u32 ) };
	// 8304E784: 4BF89B15  bl 0x82fd8298
	ctx.lr = 0x8304E788;
	sub_82FD8298(ctx, base);
	// 8304E788: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8304E78C: 93BF005C  stw r29, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[29].u32 ) };
	// 8304E790: 41820024  beq 0x8304e7b4
	if ctx.cr[0].eq {
	pc = 0x8304E7B4; continue 'dispatch;
	}
	// 8304E794: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304E798: 4BFA9919  bl 0x82ff80b0
	ctx.lr = 0x8304E79C;
	sub_82FF80B0(ctx, base);
	// 8304E79C: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 8304E7A0: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8304E7A4: 809F0054  lwz r4, 0x54(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8304E7A8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304E7AC: 4BFFE7B5  bl 0x8304cf60
	ctx.lr = 0x8304E7B0;
	sub_8304CF60(ctx, base);
	// 8304E7B0: 48000008  b 0x8304e7b8
	pc = 0x8304E7B8; continue 'dispatch;
	// 8304E7B4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8304E7B8: 907B0000  stw r3, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8304E7BC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304E7C0: 809B0000  lwz r4, 0(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304E7C4: 4BFAA6AD  bl 0x82ff8e70
	ctx.lr = 0x8304E7C8;
	sub_82FF8E70(ctx, base);
	// 8304E7C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8304E7CC: 389F0050  addi r4, r31, 0x50
	ctx.r[4].s64 = ctx.r[31].s64 + 80;
	// 8304E7D0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304E7D4: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8304E7D8: 4BFAADA1  bl 0x82ff9578
	ctx.lr = 0x8304E7DC;
	sub_82FF9578(ctx, base);
	// 8304E7DC: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8304E7E0: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8304E7E4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304E7E8: 40990058  ble cr6, 0x8304e840
	if !ctx.cr[6].gt {
	pc = 0x8304E840; continue 'dispatch;
	}
	// 8304E7EC: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8304E7F0: 3B4B35A0  addi r26, r11, 0x35a0
	ctx.r[26].s64 = ctx.r[11].s64 + 13728;
	// 8304E7F4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304E7F8: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8304E7FC: 4BFAB4C5  bl 0x82ff9cc0
	ctx.lr = 0x8304E800;
	sub_82FF9CC0(ctx, base);
	// 8304E800: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8304E804: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304E808: 4BFA9891  bl 0x82ff8098
	ctx.lr = 0x8304E80C;
	sub_82FF8098(ctx, base);
	// 8304E80C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304E810: 809C0008  lwz r4, 8(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 8304E814: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8304E818: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8304E81C: 4E800421  bctrl
	ctx.lr = 0x8304E820;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8304E820: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8304E824: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8304E828: 807B0000  lwz r3, 0(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304E82C: 4BFDE61D  bl 0x8302ce48
	ctx.lr = 0x8304E830;
	sub_8302CE48(ctx, base);
	// 8304E830: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8304E834: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 8304E838: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8304E83C: 4198FFB8  blt cr6, 0x8304e7f4
	if ctx.cr[6].lt {
	pc = 0x8304E7F4; continue 'dispatch;
	}
	// 8304E840: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 8304E844: 4815996C  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304E848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304E848 size=44
    let mut pc: u32 = 0x8304E848;
    'dispatch: loop {
        match pc {
            0x8304E848 => {
    //   block [0x8304E848..0x8304E874)
	// 8304E848: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 8304E84C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304E850: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304E854: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304E858: 809F0058  lwz r4, 0x58(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 8304E85C: 807F005C  lwz r3, 0x5c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 8304E860: 4BF89A81  bl 0x82fd82e0
	ctx.lr = 0x8304E864;
	sub_82FD82E0(ctx, base);
	// 8304E864: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8304E868: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304E86C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304E870: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304E878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8304E878 size=8
    let mut pc: u32 = 0x8304E878;
    'dispatch: loop {
        match pc {
            0x8304E878 => {
    //   block [0x8304E878..0x8304E880)
	// 8304E878: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304E87C: 82162F48  lwz r16, 0x2f48(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(12104 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304E880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304E880 size=564
    let mut pc: u32 = 0x8304E880;
    'dispatch: loop {
        match pc {
            0x8304E880 => {
    //   block [0x8304E880..0x8304EAB4)
	// 8304E880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304E884: 481598C5  bl 0x831a8148
	ctx.lr = 0x8304E888;
	sub_831A8130(ctx, base);
	// 8304E888: 3BE1FF30  addi r31, r1, -0xd0
	ctx.r[31].s64 = ctx.r[1].s64 + -208;
	// 8304E88C: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304E890: 7C751B78  mr r21, r3
	ctx.r[21].u64 = ctx.r[3].u64;
	// 8304E894: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 8304E898: 7EA4AB78  mr r4, r21
	ctx.r[4].u64 = ctx.r[21].u64;
	// 8304E89C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304E8A0: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8304E8A4: 4BFAAE8D  bl 0x82ff9730
	ctx.lr = 0x8304E8A8;
	sub_82FF9730(ctx, base);
	// 8304E8A8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304E8AC: 41820200  beq 0x8304eaac
	if ctx.cr[0].eq {
	pc = 0x8304EAAC; continue 'dispatch;
	}
	// 8304E8B0: 389F0054  addi r4, r31, 0x54
	ctx.r[4].s64 = ctx.r[31].s64 + 84;
	// 8304E8B4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304E8B8: 4BFAACC1  bl 0x82ff9578
	ctx.lr = 0x8304E8BC;
	sub_82FF9578(ctx, base);
	// 8304E8BC: 81750000  lwz r11, 0(r21)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304E8C0: 3A800000  li r20, 0
	ctx.r[20].s64 = 0;
	// 8304E8C4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8304E8C8: 409A0050  bne cr6, 0x8304e918
	if !ctx.cr[6].eq {
	pc = 0x8304E918; continue 'dispatch;
	}
	// 8304E8CC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304E8D0: 4BFA97E1  bl 0x82ff80b0
	ctx.lr = 0x8304E8D4;
	sub_82FF80B0(ctx, base);
	// 8304E8D4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8304E8D8: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 8304E8DC: 909F0058  stw r4, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[4].u32 ) };
	// 8304E8E0: 4BF899B9  bl 0x82fd8298
	ctx.lr = 0x8304E8E4;
	sub_82FD8298(ctx, base);
	// 8304E8E4: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8304E8E8: 93DF0060  stw r30, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[30].u32 ) };
	// 8304E8EC: 41820024  beq 0x8304e910
	if ctx.cr[0].eq {
	pc = 0x8304E910; continue 'dispatch;
	}
	// 8304E8F0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304E8F4: 4BFA97BD  bl 0x82ff80b0
	ctx.lr = 0x8304E8F8;
	sub_82FF80B0(ctx, base);
	// 8304E8F8: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 8304E8FC: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8304E900: 809F0054  lwz r4, 0x54(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8304E904: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304E908: 4BFFE659  bl 0x8304cf60
	ctx.lr = 0x8304E90C;
	sub_8304CF60(ctx, base);
	// 8304E90C: 48000008  b 0x8304e914
	pc = 0x8304E914; continue 'dispatch;
	// 8304E910: 7E83A378  mr r3, r20
	ctx.r[3].u64 = ctx.r[20].u64;
	// 8304E914: 90750000  stw r3, 0(r21)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[21].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8304E918: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304E91C: 80950000  lwz r4, 0(r21)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304E920: 4BFAA551  bl 0x82ff8e70
	ctx.lr = 0x8304E924;
	sub_82FF8E70(ctx, base);
	// 8304E924: 389F0050  addi r4, r31, 0x50
	ctx.r[4].s64 = ctx.r[31].s64 + 80;
	// 8304E928: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304E92C: 929F0050  stw r20, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[20].u32 ) };
	// 8304E930: 4BFAAC49  bl 0x82ff9578
	ctx.lr = 0x8304E934;
	sub_82FF9578(ctx, base);
	// 8304E934: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8304E938: 7E99A378  mr r25, r20
	ctx.r[25].u64 = ctx.r[20].u64;
	// 8304E93C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304E940: 4099016C  ble cr6, 0x8304eaac
	if !ctx.cr[6].gt {
	pc = 0x8304EAAC; continue 'dispatch;
	}
	// 8304E944: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304E948: 4BFF29E9  bl 0x83041330
	ctx.lr = 0x8304E94C;
	sub_83041330(ctx, base);
	// 8304E94C: 7C761B78  mr r22, r3
	ctx.r[22].u64 = ctx.r[3].u64;
	// 8304E950: 83160034  lwz r24, 0x34(r22)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(52 as u32) ) } as u64;
	// 8304E954: 82F60030  lwz r23, 0x30(r22)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(48 as u32) ) } as u64;
	// 8304E958: 28180000  cmplwi r24, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8304E95C: 41820034  beq 0x8304e990
	if ctx.cr[0].eq {
	pc = 0x8304E990; continue 'dispatch;
	}
	// 8304E960: A1780000  lhz r11, 0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304E964: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8304E968: 41820028  beq 0x8304e990
	if ctx.cr[0].eq {
	pc = 0x8304E990; continue 'dispatch;
	}
	// 8304E96C: 39780002  addi r11, r24, 2
	ctx.r[11].s64 = ctx.r[24].s64 + 2;
	// 8304E970: 48000008  b 0x8304e978
	pc = 0x8304E978; continue 'dispatch;
	// 8304E974: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 8304E978: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304E97C: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8304E980: 4082FFF4  bne 0x8304e974
	if !ctx.cr[0].eq {
	pc = 0x8304E974; continue 'dispatch;
	}
	// 8304E984: 7D785850  subf r11, r24, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[24].s64;
	// 8304E988: 7D7B0E70  srawi r27, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[27].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 8304E98C: 48000008  b 0x8304e994
	pc = 0x8304E994; continue 'dispatch;
	// 8304E990: 7E9BA378  mr r27, r20
	ctx.r[27].u64 = ctx.r[20].u64;
	// 8304E994: 2B170000  cmplwi cr6, r23, 0
	ctx.cr[6].compare_u32(ctx.r[23].u32, 0 as u32, &mut ctx.xer);
	// 8304E998: 419A0034  beq cr6, 0x8304e9cc
	if ctx.cr[6].eq {
	pc = 0x8304E9CC; continue 'dispatch;
	}
	// 8304E99C: A1770000  lhz r11, 0(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304E9A0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8304E9A4: 41820028  beq 0x8304e9cc
	if ctx.cr[0].eq {
	pc = 0x8304E9CC; continue 'dispatch;
	}
	// 8304E9A8: 39770002  addi r11, r23, 2
	ctx.r[11].s64 = ctx.r[23].s64 + 2;
	// 8304E9AC: 48000008  b 0x8304e9b4
	pc = 0x8304E9B4; continue 'dispatch;
	// 8304E9B0: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 8304E9B4: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304E9B8: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8304E9BC: 4082FFF4  bne 0x8304e9b0
	if !ctx.cr[0].eq {
	pc = 0x8304E9B0; continue 'dispatch;
	}
	// 8304E9C0: 7D775850  subf r11, r23, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[23].s64;
	// 8304E9C4: 7D7A0E70  srawi r26, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[26].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 8304E9C8: 48000008  b 0x8304e9d0
	pc = 0x8304E9D0; continue 'dispatch;
	// 8304E9CC: 7E9AA378  mr r26, r20
	ctx.r[26].u64 = ctx.r[20].u64;
	// 8304E9D0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304E9D4: 4BFA96DD  bl 0x82ff80b0
	ctx.lr = 0x8304E9D8;
	sub_82FF80B0(ctx, base);
	// 8304E9D8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304E9DC: 7F9ADA14  add r28, r26, r27
	ctx.r[28].u64 = ctx.r[26].u64 + ctx.r[27].u64;
	// 8304E9E0: 395C0002  addi r10, r28, 2
	ctx.r[10].s64 = ctx.r[28].s64 + 2;
	// 8304E9E4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8304E9E8: 5544083C  slwi r4, r10, 1
	ctx.r[4].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 8304E9EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8304E9F0: 4E800421  bctrl
	ctx.lr = 0x8304E9F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8304E9F4: 577B083C  slwi r27, r27, 1
	ctx.r[27].u32 = ctx.r[27].u32.wrapping_shl(1);
	ctx.r[27].u64 = ctx.r[27].u32 as u64;
	// 8304E9F8: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 8304E9FC: 38BB0002  addi r5, r27, 2
	ctx.r[5].s64 = ctx.r[27].s64 + 2;
	// 8304EA00: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8304EA04: 48159B0D  bl 0x831a8510
	ctx.lr = 0x8304EA08;
	sub_831A8510(ctx, base);
	// 8304EA08: 7D7BF214  add r11, r27, r30
	ctx.r[11].u64 = ctx.r[27].u64 + ctx.r[30].u64;
	// 8304EA0C: 3940002C  li r10, 0x2c
	ctx.r[10].s64 = 44;
	// 8304EA10: 393A0001  addi r9, r26, 1
	ctx.r[9].s64 = ctx.r[26].s64 + 1;
	// 8304EA14: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 8304EA18: 5525083C  slwi r5, r9, 1
	ctx.r[5].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 8304EA1C: 386B0002  addi r3, r11, 2
	ctx.r[3].s64 = ctx.r[11].s64 + 2;
	// 8304EA20: B14B0000  sth r10, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 8304EA24: 48159AED  bl 0x831a8510
	ctx.lr = 0x8304EA28;
	sub_831A8510(ctx, base);
	// 8304EA28: 397C0001  addi r11, r28, 1
	ctx.r[11].s64 = ctx.r[28].s64 + 1;
	// 8304EA2C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304EA30: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8304EA34: 7E8BF32E  sthx r20, r11, r30
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32), ctx.r[20].u16) };
	// 8304EA38: 4BFA9679  bl 0x82ff80b0
	ctx.lr = 0x8304EA3C;
	sub_82FF80B0(ctx, base);
	// 8304EA3C: 93DF0060  stw r30, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[30].u32 ) };
	// 8304EA40: 907F0064  stw r3, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[3].u32 ) };
	// 8304EA44: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304EA48: 4BFA9651  bl 0x82ff8098
	ctx.lr = 0x8304EA4C;
	sub_82FF8098(ctx, base);
	// 8304EA4C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304EA50: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8304EA54: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8304EA58: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8304EA5C: 4E800421  bctrl
	ctx.lr = 0x8304EA60;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8304EA60: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8304EA64: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304EA68: 4BFA9631  bl 0x82ff8098
	ctx.lr = 0x8304EA6C;
	sub_82FF8098(ctx, base);
	// 8304EA6C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304EA70: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8304EA74: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8304EA78: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8304EA7C: 4E800421  bctrl
	ctx.lr = 0x8304EA80;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8304EA80: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8304EA84: 7EC5B378  mr r5, r22
	ctx.r[5].u64 = ctx.r[22].u64;
	// 8304EA88: 80750000  lwz r3, 0(r21)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304EA8C: 4BFDE3BD  bl 0x8302ce48
	ctx.lr = 0x8304EA90;
	sub_8302CE48(ctx, base);
	// 8304EA90: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8304EA94: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 8304EA98: 4BF84029  bl 0x82fd2ac0
	ctx.lr = 0x8304EA9C;
	sub_82FD2AC0(ctx, base);
	// 8304EA9C: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8304EAA0: 3B390001  addi r25, r25, 1
	ctx.r[25].s64 = ctx.r[25].s64 + 1;
	// 8304EAA4: 7F195800  cmpw cr6, r25, r11
	ctx.cr[6].compare_i32(ctx.r[25].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8304EAA8: 4198FE9C  blt cr6, 0x8304e944
	if ctx.cr[6].lt {
	pc = 0x8304E944; continue 'dispatch;
	}
	// 8304EAAC: 383F00D0  addi r1, r31, 0xd0
	ctx.r[1].s64 = ctx.r[31].s64 + 208;
	// 8304EAB0: 481596E8  b 0x831a8198
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304EAB4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304EAB4 size=44
    let mut pc: u32 = 0x8304EAB4;
    'dispatch: loop {
        match pc {
            0x8304EAB4 => {
    //   block [0x8304EAB4..0x8304EAE0)
	// 8304EAB4: 3BECFF30  addi r31, r12, -0xd0
	ctx.r[31].s64 = ctx.r[12].s64 + -208;
	// 8304EAB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304EABC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304EAC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304EAC4: 809F0058  lwz r4, 0x58(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 8304EAC8: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 8304EACC: 4BF89815  bl 0x82fd82e0
	ctx.lr = 0x8304EAD0;
	sub_82FD82E0(ctx, base);
	// 8304EAD0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8304EAD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304EAD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304EADC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304EAE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304EAE0 size=40
    let mut pc: u32 = 0x8304EAE0;
    'dispatch: loop {
        match pc {
            0x8304EAE0 => {
    //   block [0x8304EAE0..0x8304EB08)
	// 8304EAE0: 3BECFF30  addi r31, r12, -0xd0
	ctx.r[31].s64 = ctx.r[12].s64 + -208;
	// 8304EAE4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304EAE8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304EAEC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304EAF0: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 8304EAF4: 4BF84365  bl 0x82fd2e58
	ctx.lr = 0x8304EAF8;
	sub_82FD2E58(ctx, base);
	// 8304EAF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8304EAFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304EB00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304EB04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304EB08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8304EB08 size=8
    let mut pc: u32 = 0x8304EB08;
    'dispatch: loop {
        match pc {
            0x8304EB08 => {
    //   block [0x8304EB08..0x8304EB10)
	// 8304EB08: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304EB0C: 82162FC0  lwz r16, 0x2fc0(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(12224 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304EB10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304EB10 size=280
    let mut pc: u32 = 0x8304EB10;
    'dispatch: loop {
        match pc {
            0x8304EB10 => {
    //   block [0x8304EB10..0x8304EC28)
	// 8304EB10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304EB14: 48159651  bl 0x831a8164
	ctx.lr = 0x8304EB18;
	sub_831A8130(ctx, base);
	// 8304EB18: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 8304EB1C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304EB20: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8304EB24: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 8304EB28: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8304EB2C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304EB30: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8304EB34: 4BFAABFD  bl 0x82ff9730
	ctx.lr = 0x8304EB38;
	sub_82FF9730(ctx, base);
	// 8304EB38: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304EB3C: 418200E4  beq 0x8304ec20
	if ctx.cr[0].eq {
	pc = 0x8304EC20; continue 'dispatch;
	}
	// 8304EB40: 389F0054  addi r4, r31, 0x54
	ctx.r[4].s64 = ctx.r[31].s64 + 84;
	// 8304EB44: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304EB48: 4BFAAA31  bl 0x82ff9578
	ctx.lr = 0x8304EB4C;
	sub_82FF9578(ctx, base);
	// 8304EB4C: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304EB50: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8304EB54: 409A0050  bne cr6, 0x8304eba4
	if !ctx.cr[6].eq {
	pc = 0x8304EBA4; continue 'dispatch;
	}
	// 8304EB58: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304EB5C: 4BFA9555  bl 0x82ff80b0
	ctx.lr = 0x8304EB60;
	sub_82FF80B0(ctx, base);
	// 8304EB60: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8304EB64: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 8304EB68: 909F0058  stw r4, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[4].u32 ) };
	// 8304EB6C: 4BF8972D  bl 0x82fd8298
	ctx.lr = 0x8304EB70;
	sub_82FD8298(ctx, base);
	// 8304EB70: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8304EB74: 93BF005C  stw r29, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[29].u32 ) };
	// 8304EB78: 41820024  beq 0x8304eb9c
	if ctx.cr[0].eq {
	pc = 0x8304EB9C; continue 'dispatch;
	}
	// 8304EB7C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304EB80: 4BFA9531  bl 0x82ff80b0
	ctx.lr = 0x8304EB84;
	sub_82FF80B0(ctx, base);
	// 8304EB84: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 8304EB88: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8304EB8C: 809F0054  lwz r4, 0x54(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8304EB90: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304EB94: 4BFFE3CD  bl 0x8304cf60
	ctx.lr = 0x8304EB98;
	sub_8304CF60(ctx, base);
	// 8304EB98: 48000008  b 0x8304eba0
	pc = 0x8304EBA0; continue 'dispatch;
	// 8304EB9C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8304EBA0: 907B0000  stw r3, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8304EBA4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304EBA8: 809B0000  lwz r4, 0(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304EBAC: 4BFAA2C5  bl 0x82ff8e70
	ctx.lr = 0x8304EBB0;
	sub_82FF8E70(ctx, base);
	// 8304EBB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8304EBB4: 389F0050  addi r4, r31, 0x50
	ctx.r[4].s64 = ctx.r[31].s64 + 80;
	// 8304EBB8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304EBBC: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8304EBC0: 4BFAA9B9  bl 0x82ff9578
	ctx.lr = 0x8304EBC4;
	sub_82FF9578(ctx, base);
	// 8304EBC4: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8304EBC8: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8304EBCC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304EBD0: 40990050  ble cr6, 0x8304ec20
	if !ctx.cr[6].gt {
	pc = 0x8304EC20; continue 'dispatch;
	}
	// 8304EBD4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304EBD8: 4804A9D1  bl 0x830995a8
	ctx.lr = 0x8304EBDC;
	sub_830995A8(ctx, base);
	// 8304EBDC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8304EBE0: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304EBE4: 816B0054  lwz r11, 0x54(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 8304EBE8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8304EBEC: 4E800421  bctrl
	ctx.lr = 0x8304EBF0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8304EBF0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304EBF4: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8304EBF8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8304EBFC: 4E800421  bctrl
	ctx.lr = 0x8304EC00;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8304EC00: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8304EC04: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8304EC08: 807B0000  lwz r3, 0(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304EC0C: 4BFDE23D  bl 0x8302ce48
	ctx.lr = 0x8304EC10;
	sub_8302CE48(ctx, base);
	// 8304EC10: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8304EC14: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 8304EC18: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8304EC1C: 4198FFB8  blt cr6, 0x8304ebd4
	if ctx.cr[6].lt {
	pc = 0x8304EBD4; continue 'dispatch;
	}
	// 8304EC20: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 8304EC24: 48159590  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304EC28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304EC28 size=44
    let mut pc: u32 = 0x8304EC28;
    'dispatch: loop {
        match pc {
            0x8304EC28 => {
    //   block [0x8304EC28..0x8304EC54)
	// 8304EC28: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 8304EC2C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304EC30: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304EC34: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304EC38: 809F0058  lwz r4, 0x58(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 8304EC3C: 807F005C  lwz r3, 0x5c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 8304EC40: 4BF896A1  bl 0x82fd82e0
	ctx.lr = 0x8304EC44;
	sub_82FD82E0(ctx, base);
	// 8304EC44: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8304EC48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304EC4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304EC50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304EC58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8304EC58 size=8
    let mut pc: u32 = 0x8304EC58;
    'dispatch: loop {
        match pc {
            0x8304EC58 => {
    //   block [0x8304EC58..0x8304EC60)
	// 8304EC58: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304EC5C: 82163010  lwz r16, 0x3010(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(12304 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304EC60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304EC60 size=344
    let mut pc: u32 = 0x8304EC60;
    'dispatch: loop {
        match pc {
            0x8304EC60 => {
    //   block [0x8304EC60..0x8304EDB8)
	// 8304EC60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304EC64: 481594F9  bl 0x831a815c
	ctx.lr = 0x8304EC68;
	sub_831A8130(ctx, base);
	// 8304EC68: 3BE1FF50  addi r31, r1, -0xb0
	ctx.r[31].s64 = ctx.r[1].s64 + -176;
	// 8304EC6C: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304EC70: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 8304EC74: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 8304EC78: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8304EC7C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304EC80: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 8304EC84: 4BFAAAAD  bl 0x82ff9730
	ctx.lr = 0x8304EC88;
	sub_82FF9730(ctx, base);
	// 8304EC88: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304EC8C: 41820124  beq 0x8304edb0
	if ctx.cr[0].eq {
	pc = 0x8304EDB0; continue 'dispatch;
	}
	// 8304EC90: 389F0058  addi r4, r31, 0x58
	ctx.r[4].s64 = ctx.r[31].s64 + 88;
	// 8304EC94: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304EC98: 4BFAA8E1  bl 0x82ff9578
	ctx.lr = 0x8304EC9C;
	sub_82FF9578(ctx, base);
	// 8304EC9C: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304ECA0: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 8304ECA4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8304ECA8: 409A0088  bne cr6, 0x8304ed30
	if !ctx.cr[6].eq {
	pc = 0x8304ED30; continue 'dispatch;
	}
	// 8304ECAC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304ECB0: 4BFA9401  bl 0x82ff80b0
	ctx.lr = 0x8304ECB4;
	sub_82FF80B0(ctx, base);
	// 8304ECB4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8304ECB8: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 8304ECBC: 909F0054  stw r4, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[4].u32 ) };
	// 8304ECC0: 4BF895D9  bl 0x82fd8298
	ctx.lr = 0x8304ECC4;
	sub_82FD8298(ctx, base);
	// 8304ECC4: 7C7C1B79  or. r28, r3, r3
	ctx.r[28].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 8304ECC8: 939F005C  stw r28, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[28].u32 ) };
	// 8304ECCC: 4182005C  beq 0x8304ed28
	if ctx.cr[0].eq {
	pc = 0x8304ED28; continue 'dispatch;
	}
	// 8304ECD0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304ECD4: 4BFA93DD  bl 0x82ff80b0
	ctx.lr = 0x8304ECD8;
	sub_82FF80B0(ctx, base);
	// 8304ECD8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8304ECDC: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 8304ECE0: 909F0060  stw r4, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[4].u32 ) };
	// 8304ECE4: 4BF895B5  bl 0x82fd8298
	ctx.lr = 0x8304ECE8;
	sub_82FD8298(ctx, base);
	// 8304ECE8: 907F0064  stw r3, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[3].u32 ) };
	// 8304ECEC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8304ECF0: 41820010  beq 0x8304ed00
	if ctx.cr[0].eq {
	pc = 0x8304ED00; continue 'dispatch;
	}
	// 8304ECF4: 4BFB089D  bl 0x82fff590
	ctx.lr = 0x8304ECF8;
	sub_82FFF590(ctx, base);
	// 8304ECF8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8304ECFC: 48000008  b 0x8304ed04
	pc = 0x8304ED04; continue 'dispatch;
	// 8304ED00: 7F3DCB78  mr r29, r25
	ctx.r[29].u64 = ctx.r[25].u64;
	// 8304ED04: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304ED08: 4BFA93A9  bl 0x82ff80b0
	ctx.lr = 0x8304ED0C;
	sub_82FF80B0(ctx, base);
	// 8304ED0C: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 8304ED10: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 8304ED14: 809F0058  lwz r4, 0x58(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 8304ED18: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 8304ED1C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8304ED20: 4BFEC3D1  bl 0x8303b0f0
	ctx.lr = 0x8304ED24;
	sub_8303B0F0(ctx, base);
	// 8304ED24: 48000008  b 0x8304ed2c
	pc = 0x8304ED2C; continue 'dispatch;
	// 8304ED28: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 8304ED2C: 907A0000  stw r3, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8304ED30: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304ED34: 809A0000  lwz r4, 0(r26)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304ED38: 4BFAA139  bl 0x82ff8e70
	ctx.lr = 0x8304ED3C;
	sub_82FF8E70(ctx, base);
	// 8304ED3C: 389F0050  addi r4, r31, 0x50
	ctx.r[4].s64 = ctx.r[31].s64 + 80;
	// 8304ED40: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304ED44: 933F0050  stw r25, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[25].u32 ) };
	// 8304ED48: 4BFAA831  bl 0x82ff9578
	ctx.lr = 0x8304ED4C;
	sub_82FF9578(ctx, base);
	// 8304ED4C: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8304ED50: 7F3DCB78  mr r29, r25
	ctx.r[29].u64 = ctx.r[25].u64;
	// 8304ED54: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304ED58: 40990058  ble cr6, 0x8304edb0
	if !ctx.cr[6].gt {
	pc = 0x8304EDB0; continue 'dispatch;
	}
	// 8304ED5C: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8304ED60: 3B6B32D4  addi r27, r11, 0x32d4
	ctx.r[27].s64 = ctx.r[11].s64 + 13012;
	// 8304ED64: 389F0054  addi r4, r31, 0x54
	ctx.r[4].s64 = ctx.r[31].s64 + 84;
	// 8304ED68: 933F0054  stw r25, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[25].u32 ) };
	// 8304ED6C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304ED70: 4BFAA809  bl 0x82ff9578
	ctx.lr = 0x8304ED74;
	sub_82FF9578(ctx, base);
	// 8304ED74: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304ED78: 809F0054  lwz r4, 0x54(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8304ED7C: 4BFA941D  bl 0x82ff8198
	ctx.lr = 0x8304ED80;
	sub_82FF8198(ctx, base);
	// 8304ED80: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8304ED84: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8304ED88: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304ED8C: 4BFAAF35  bl 0x82ff9cc0
	ctx.lr = 0x8304ED90;
	sub_82FF9CC0(ctx, base);
	// 8304ED90: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8304ED94: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8304ED98: 807A0000  lwz r3, 0(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304ED9C: 4BFDE0AD  bl 0x8302ce48
	ctx.lr = 0x8304EDA0;
	sub_8302CE48(ctx, base);
	// 8304EDA0: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8304EDA4: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 8304EDA8: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8304EDAC: 4198FFB8  blt cr6, 0x8304ed64
	if ctx.cr[6].lt {
	pc = 0x8304ED64; continue 'dispatch;
	}
	// 8304EDB0: 383F00B0  addi r1, r31, 0xb0
	ctx.r[1].s64 = ctx.r[31].s64 + 176;
	// 8304EDB4: 481593F8  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304EDB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304EDB8 size=44
    let mut pc: u32 = 0x8304EDB8;
    'dispatch: loop {
        match pc {
            0x8304EDB8 => {
    //   block [0x8304EDB8..0x8304EDE4)
	// 8304EDB8: 3BECFF50  addi r31, r12, -0xb0
	ctx.r[31].s64 = ctx.r[12].s64 + -176;
	// 8304EDBC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304EDC0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304EDC4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304EDC8: 809F0054  lwz r4, 0x54(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8304EDCC: 807F005C  lwz r3, 0x5c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 8304EDD0: 4BF89511  bl 0x82fd82e0
	ctx.lr = 0x8304EDD4;
	sub_82FD82E0(ctx, base);
	// 8304EDD4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8304EDD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304EDDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304EDE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304EDE4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304EDE4 size=44
    let mut pc: u32 = 0x8304EDE4;
    'dispatch: loop {
        match pc {
            0x8304EDE4 => {
    //   block [0x8304EDE4..0x8304EE10)
	// 8304EDE4: 3BECFF50  addi r31, r12, -0xb0
	ctx.r[31].s64 = ctx.r[12].s64 + -176;
	// 8304EDE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304EDEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304EDF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304EDF4: 809F0060  lwz r4, 0x60(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 8304EDF8: 807F0064  lwz r3, 0x64(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 8304EDFC: 4BF894E5  bl 0x82fd82e0
	ctx.lr = 0x8304EE00;
	sub_82FD82E0(ctx, base);
	// 8304EE00: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8304EE04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304EE08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304EE0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304EE10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8304EE10 size=8
    let mut pc: u32 = 0x8304EE10;
    'dispatch: loop {
        match pc {
            0x8304EE10 => {
    //   block [0x8304EE10..0x8304EE18)
	// 8304EE10: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304EE14: 82163068  lwz r16, 0x3068(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(12392 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304EE18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304EE18 size=264
    let mut pc: u32 = 0x8304EE18;
    'dispatch: loop {
        match pc {
            0x8304EE18 => {
    //   block [0x8304EE18..0x8304EF20)
	// 8304EE18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304EE1C: 48159349  bl 0x831a8164
	ctx.lr = 0x8304EE20;
	sub_831A8130(ctx, base);
	// 8304EE20: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 8304EE24: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304EE28: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8304EE2C: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 8304EE30: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8304EE34: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304EE38: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 8304EE3C: 4BFAA8F5  bl 0x82ff9730
	ctx.lr = 0x8304EE40;
	sub_82FF9730(ctx, base);
	// 8304EE40: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304EE44: 418200D4  beq 0x8304ef18
	if ctx.cr[0].eq {
	pc = 0x8304EF18; continue 'dispatch;
	}
	// 8304EE48: 389F0054  addi r4, r31, 0x54
	ctx.r[4].s64 = ctx.r[31].s64 + 84;
	// 8304EE4C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304EE50: 4BFAA729  bl 0x82ff9578
	ctx.lr = 0x8304EE54;
	sub_82FF9578(ctx, base);
	// 8304EE54: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304EE58: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8304EE5C: 409A0050  bne cr6, 0x8304eeac
	if !ctx.cr[6].eq {
	pc = 0x8304EEAC; continue 'dispatch;
	}
	// 8304EE60: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304EE64: 4BFA924D  bl 0x82ff80b0
	ctx.lr = 0x8304EE68;
	sub_82FF80B0(ctx, base);
	// 8304EE68: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8304EE6C: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 8304EE70: 909F0058  stw r4, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[4].u32 ) };
	// 8304EE74: 4BF89425  bl 0x82fd8298
	ctx.lr = 0x8304EE78;
	sub_82FD8298(ctx, base);
	// 8304EE78: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8304EE7C: 93BF005C  stw r29, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[29].u32 ) };
	// 8304EE80: 41820024  beq 0x8304eea4
	if ctx.cr[0].eq {
	pc = 0x8304EEA4; continue 'dispatch;
	}
	// 8304EE84: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304EE88: 4BFA9229  bl 0x82ff80b0
	ctx.lr = 0x8304EE8C;
	sub_82FF80B0(ctx, base);
	// 8304EE8C: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 8304EE90: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 8304EE94: 809F0054  lwz r4, 0x54(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8304EE98: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304EE9C: 4BFEC315  bl 0x8303b1b0
	ctx.lr = 0x8304EEA0;
	sub_8303B1B0(ctx, base);
	// 8304EEA0: 48000008  b 0x8304eea8
	pc = 0x8304EEA8; continue 'dispatch;
	// 8304EEA4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8304EEA8: 907C0000  stw r3, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8304EEAC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304EEB0: 809C0000  lwz r4, 0(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304EEB4: 4BFA9FBD  bl 0x82ff8e70
	ctx.lr = 0x8304EEB8;
	sub_82FF8E70(ctx, base);
	// 8304EEB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8304EEBC: 389F0050  addi r4, r31, 0x50
	ctx.r[4].s64 = ctx.r[31].s64 + 80;
	// 8304EEC0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304EEC4: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8304EEC8: 4BFAA6B1  bl 0x82ff9578
	ctx.lr = 0x8304EECC;
	sub_82FF9578(ctx, base);
	// 8304EECC: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8304EED0: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8304EED4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304EED8: 40990040  ble cr6, 0x8304ef18
	if !ctx.cr[6].gt {
	pc = 0x8304EF18; continue 'dispatch;
	}
	// 8304EEDC: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8304EEE0: 3B6B34C0  addi r27, r11, 0x34c0
	ctx.r[27].s64 = ctx.r[11].s64 + 13504;
	// 8304EEE4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304EEE8: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8304EEEC: 4BFAADD5  bl 0x82ff9cc0
	ctx.lr = 0x8304EEF0;
	sub_82FF9CC0(ctx, base);
	// 8304EEF0: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 8304EEF4: 807C0000  lwz r3, 0(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304EEF8: 81660028  lwz r11, 0x28(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(40 as u32) ) } as u64;
	// 8304EEFC: 80AB0020  lwz r5, 0x20(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8304EF00: 808B0010  lwz r4, 0x10(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8304EF04: 4BFF0DFD  bl 0x8303fd00
	ctx.lr = 0x8304EF08;
	sub_8303FD00(ctx, base);
	// 8304EF08: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8304EF0C: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 8304EF10: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8304EF14: 4198FFD0  blt cr6, 0x8304eee4
	if ctx.cr[6].lt {
	pc = 0x8304EEE4; continue 'dispatch;
	}
	// 8304EF18: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 8304EF1C: 48159298  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304EF20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304EF20 size=44
    let mut pc: u32 = 0x8304EF20;
    'dispatch: loop {
        match pc {
            0x8304EF20 => {
    //   block [0x8304EF20..0x8304EF4C)
	// 8304EF20: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 8304EF24: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304EF28: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304EF2C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304EF30: 809F0058  lwz r4, 0x58(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 8304EF34: 807F005C  lwz r3, 0x5c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 8304EF38: 4BF893A9  bl 0x82fd82e0
	ctx.lr = 0x8304EF3C;
	sub_82FD82E0(ctx, base);
	// 8304EF3C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8304EF40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304EF44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304EF48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304EF50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8304EF50 size=8
    let mut pc: u32 = 0x8304EF50;
    'dispatch: loop {
        match pc {
            0x8304EF50 => {
    //   block [0x8304EF50..0x8304EF58)
	// 8304EF50: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304EF54: 821630B0  lwz r16, 0x30b0(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(12464 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304EF58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304EF58 size=276
    let mut pc: u32 = 0x8304EF58;
    'dispatch: loop {
        match pc {
            0x8304EF58 => {
    //   block [0x8304EF58..0x8304F06C)
	// 8304EF58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304EF5C: 48159205  bl 0x831a8160
	ctx.lr = 0x8304EF60;
	sub_831A8130(ctx, base);
	// 8304EF60: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 8304EF64: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304EF68: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8304EF6C: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 8304EF70: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8304EF74: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304EF78: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 8304EF7C: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 8304EF80: 4BFAA7B1  bl 0x82ff9730
	ctx.lr = 0x8304EF84;
	sub_82FF9730(ctx, base);
	// 8304EF84: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304EF88: 418200DC  beq 0x8304f064
	if ctx.cr[0].eq {
	pc = 0x8304F064; continue 'dispatch;
	}
	// 8304EF8C: 389F0054  addi r4, r31, 0x54
	ctx.r[4].s64 = ctx.r[31].s64 + 84;
	// 8304EF90: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304EF94: 4BFAA5E5  bl 0x82ff9578
	ctx.lr = 0x8304EF98;
	sub_82FF9578(ctx, base);
	// 8304EF98: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304EF9C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8304EFA0: 409A0054  bne cr6, 0x8304eff4
	if !ctx.cr[6].eq {
	pc = 0x8304EFF4; continue 'dispatch;
	}
	// 8304EFA4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304EFA8: 4BFA9109  bl 0x82ff80b0
	ctx.lr = 0x8304EFAC;
	sub_82FF80B0(ctx, base);
	// 8304EFAC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8304EFB0: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 8304EFB4: 909F0058  stw r4, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[4].u32 ) };
	// 8304EFB8: 4BF892E1  bl 0x82fd8298
	ctx.lr = 0x8304EFBC;
	sub_82FD8298(ctx, base);
	// 8304EFBC: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8304EFC0: 93BF005C  stw r29, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[29].u32 ) };
	// 8304EFC4: 41820028  beq 0x8304efec
	if ctx.cr[0].eq {
	pc = 0x8304EFEC; continue 'dispatch;
	}
	// 8304EFC8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304EFCC: 4BFA90E5  bl 0x82ff80b0
	ctx.lr = 0x8304EFD0;
	sub_82FF80B0(ctx, base);
	// 8304EFD0: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 8304EFD4: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 8304EFD8: 809F0054  lwz r4, 0x54(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8304EFDC: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 8304EFE0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304EFE4: 4BFC9465  bl 0x83018448
	ctx.lr = 0x8304EFE8;
	sub_83018448(ctx, base);
	// 8304EFE8: 48000008  b 0x8304eff0
	pc = 0x8304EFF0; continue 'dispatch;
	// 8304EFEC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8304EFF0: 907C0000  stw r3, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8304EFF4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304EFF8: 809C0000  lwz r4, 0(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304EFFC: 4BFA9E75  bl 0x82ff8e70
	ctx.lr = 0x8304F000;
	sub_82FF8E70(ctx, base);
	// 8304F000: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8304F004: 389F0050  addi r4, r31, 0x50
	ctx.r[4].s64 = ctx.r[31].s64 + 80;
	// 8304F008: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304F00C: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8304F010: 4BFAA569  bl 0x82ff9578
	ctx.lr = 0x8304F014;
	sub_82FF9578(ctx, base);
	// 8304F014: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8304F018: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8304F01C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304F020: 40990044  ble cr6, 0x8304f064
	if !ctx.cr[6].gt {
	pc = 0x8304F064; continue 'dispatch;
	}
	// 8304F024: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8304F028: 3B6B343C  addi r27, r11, 0x343c
	ctx.r[27].s64 = ctx.r[11].s64 + 13372;
	// 8304F02C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304F030: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8304F034: 4BFAAC8D  bl 0x82ff9cc0
	ctx.lr = 0x8304F038;
	sub_82FF9CC0(ctx, base);
	// 8304F038: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 8304F03C: 807C0000  lwz r3, 0(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304F040: 81670008  lwz r11, 8(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(8 as u32) ) } as u64;
	// 8304F044: 80C70024  lwz r6, 0x24(r7)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(36 as u32) ) } as u64;
	// 8304F048: 80AB0020  lwz r5, 0x20(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8304F04C: 808B0010  lwz r4, 0x10(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8304F050: 4BFC94D9  bl 0x83018528
	ctx.lr = 0x8304F054;
	sub_83018528(ctx, base);
	// 8304F054: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8304F058: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 8304F05C: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8304F060: 4198FFCC  blt cr6, 0x8304f02c
	if ctx.cr[6].lt {
	pc = 0x8304F02C; continue 'dispatch;
	}
	// 8304F064: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 8304F068: 48159148  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304F06C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304F06C size=44
    let mut pc: u32 = 0x8304F06C;
    'dispatch: loop {
        match pc {
            0x8304F06C => {
    //   block [0x8304F06C..0x8304F098)
	// 8304F06C: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 8304F070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304F074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304F078: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304F07C: 809F0058  lwz r4, 0x58(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 8304F080: 807F005C  lwz r3, 0x5c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 8304F084: 4BF8925D  bl 0x82fd82e0
	ctx.lr = 0x8304F088;
	sub_82FD82E0(ctx, base);
	// 8304F088: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8304F08C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304F090: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304F094: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304F098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8304F098 size=8
    let mut pc: u32 = 0x8304F098;
    'dispatch: loop {
        match pc {
            0x8304F098 => {
    //   block [0x8304F098..0x8304F0A0)
	// 8304F098: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304F09C: 821630F8  lwz r16, 0x30f8(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(12536 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304F0A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304F0A0 size=268
    let mut pc: u32 = 0x8304F0A0;
    'dispatch: loop {
        match pc {
            0x8304F0A0 => {
    //   block [0x8304F0A0..0x8304F1AC)
	// 8304F0A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304F0A4: 481590C5  bl 0x831a8168
	ctx.lr = 0x8304F0A8;
	sub_831A8130(ctx, base);
	// 8304F0A8: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 8304F0AC: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304F0B0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8304F0B4: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8304F0B8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8304F0BC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8304F0C0: 4BFAA5D9  bl 0x82ff9698
	ctx.lr = 0x8304F0C4;
	sub_82FF9698(ctx, base);
	// 8304F0C4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304F0C8: 418200DC  beq 0x8304f1a4
	if ctx.cr[0].eq {
	pc = 0x8304F1A4; continue 'dispatch;
	}
	// 8304F0CC: 3D408216  lis r10, -0x7dea
	ctx.r[10].s64 = -2112487424;
	// 8304F0D0: 813E0010  lwz r9, 0x10(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 8304F0D4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304F0D8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8304F0DC: 394A2A60  addi r10, r10, 0x2a60
	ctx.r[10].s64 = ctx.r[10].s64 + 10848;
	// 8304F0E0: 93DF0058  stw r30, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[30].u32 ) };
	// 8304F0E4: 7D290034  cntlzw r9, r9
	ctx.r[9].u64 = if ctx.r[9].u32 == 0 { 32 } else { ctx.r[9].u32.leading_zeros() as u64 };
	// 8304F0E8: 917F005C  stw r11, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8304F0EC: 915F0050  stw r10, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8304F0F0: 552ADFFE  rlwinm r10, r9, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[9].u32 as u64 & 0x0000001Fu64;
	// 8304F0F4: 694A0001  xori r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u64 ^ 1;
	// 8304F0F8: 915F0054  stw r10, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 8304F0FC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8304F100: 419A0014  beq cr6, 0x8304f114
	if ctx.cr[6].eq {
	pc = 0x8304F114; continue 'dispatch;
	}
	// 8304F104: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 8304F108: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8304F10C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8304F110: 40990008  ble cr6, 0x8304f118
	if !ctx.cr[6].gt {
	pc = 0x8304F118; continue 'dispatch;
	}
	// 8304F114: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8304F118: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304F11C: 4182001C  beq 0x8304f138
	if ctx.cr[0].eq {
	pc = 0x8304F138; continue 'dispatch;
	}
	// 8304F120: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 8304F124: 4BFFD64D  bl 0x8304c770
	ctx.lr = 0x8304F128;
	sub_8304C770(ctx, base);
	// 8304F128: 83DF0058  lwz r30, 0x58(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 8304F12C: 815F0054  lwz r10, 0x54(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8304F130: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 8304F134: 4BFFFFC8  b 0x8304f0fc
	pc = 0x8304F0FC; continue 'dispatch;
	// 8304F138: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8304F13C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8304F140: 4BFAA1B9  bl 0x82ff92f8
	ctx.lr = 0x8304F144;
	sub_82FF92F8(ctx, base);
	// 8304F144: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 8304F148: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8304F14C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8304F150: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 8304F154: 917F0054  stw r11, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8304F158: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8304F15C: 419A0014  beq cr6, 0x8304f170
	if ctx.cr[6].eq {
	pc = 0x8304F170; continue 'dispatch;
	}
	// 8304F160: 815E0010  lwz r10, 0x10(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 8304F164: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8304F168: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8304F16C: 40990008  ble cr6, 0x8304f174
	if !ctx.cr[6].gt {
	pc = 0x8304F174; continue 'dispatch;
	}
	// 8304F170: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8304F174: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304F178: 4182002C  beq 0x8304f1a4
	if ctx.cr[0].eq {
	pc = 0x8304F1A4; continue 'dispatch;
	}
	// 8304F17C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 8304F180: 4BFFD5F1  bl 0x8304c770
	ctx.lr = 0x8304F184;
	sub_8304C770(ctx, base);
	// 8304F184: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304F188: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8304F18C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8304F190: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8304F194: 4E800421  bctrl
	ctx.lr = 0x8304F198;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8304F198: 83DF0058  lwz r30, 0x58(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 8304F19C: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8304F1A0: 4BFFFFB8  b 0x8304f158
	pc = 0x8304F158; continue 'dispatch;
	// 8304F1A4: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 8304F1A8: 48159010  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304F1AC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304F1AC size=40
    let mut pc: u32 = 0x8304F1AC;
    'dispatch: loop {
        match pc {
            0x8304F1AC => {
    //   block [0x8304F1AC..0x8304F1D4)
	// 8304F1AC: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 8304F1B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304F1B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304F1B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304F1BC: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 8304F1C0: 4BFFD5A1  bl 0x8304c760
	ctx.lr = 0x8304F1C4;
	sub_8304C760(ctx, base);
	// 8304F1C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8304F1C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304F1CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304F1D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304F1D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8304F1D8 size=8
    let mut pc: u32 = 0x8304F1D8;
    'dispatch: loop {
        match pc {
            0x8304F1D8 => {
    //   block [0x8304F1D8..0x8304F1E0)
	// 8304F1D8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304F1DC: 82163138  lwz r16, 0x3138(r22)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(12600 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304F1E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304F1E0 size=328
    let mut pc: u32 = 0x8304F1E0;
    'dispatch: loop {
        match pc {
            0x8304F1E0 => {
    //   block [0x8304F1E0..0x8304F328)
	// 8304F1E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304F1E4: 48158F7D  bl 0x831a8160
	ctx.lr = 0x8304F1E8;
	sub_831A8130(ctx, base);
	// 8304F1E8: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 8304F1EC: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304F1F0: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 8304F1F4: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 8304F1F8: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8304F1FC: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8304F200: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304F204: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 8304F208: 4BFAA529  bl 0x82ff9730
	ctx.lr = 0x8304F20C;
	sub_82FF9730(ctx, base);
	// 8304F20C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304F210: 41820110  beq 0x8304f320
	if ctx.cr[0].eq {
	pc = 0x8304F320; continue 'dispatch;
	}
	// 8304F214: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304F218: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8304F21C: 409A005C  bne cr6, 0x8304f278
	if !ctx.cr[6].eq {
	pc = 0x8304F278; continue 'dispatch;
	}
	// 8304F220: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 8304F224: 40980008  bge cr6, 0x8304f22c
	if !ctx.cr[6].lt {
	pc = 0x8304F22C; continue 'dispatch;
	}
	// 8304F228: 3B800010  li r28, 0x10
	ctx.r[28].s64 = 16;
	// 8304F22C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304F230: 4BFA8E81  bl 0x82ff80b0
	ctx.lr = 0x8304F234;
	sub_82FF80B0(ctx, base);
	// 8304F234: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8304F238: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 8304F23C: 909F0054  stw r4, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[4].u32 ) };
	// 8304F240: 4BF89059  bl 0x82fd8298
	ctx.lr = 0x8304F244;
	sub_82FD8298(ctx, base);
	// 8304F244: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8304F248: 93BF0058  stw r29, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[29].u32 ) };
	// 8304F24C: 41820024  beq 0x8304f270
	if ctx.cr[0].eq {
	pc = 0x8304F270; continue 'dispatch;
	}
	// 8304F250: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304F254: 4BFA8E5D  bl 0x82ff80b0
	ctx.lr = 0x8304F258;
	sub_82FF80B0(ctx, base);
	// 8304F258: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 8304F25C: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 8304F260: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8304F264: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304F268: 4BFFD3C9  bl 0x8304c630
	ctx.lr = 0x8304F26C;
	sub_8304C630(ctx, base);
	// 8304F26C: 48000008  b 0x8304f274
	pc = 0x8304F274; continue 'dispatch;
	// 8304F270: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8304F274: 907A0000  stw r3, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8304F278: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304F27C: 809A0000  lwz r4, 0(r26)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304F280: 4BFA9BF1  bl 0x82ff8e70
	ctx.lr = 0x8304F284;
	sub_82FF8E70(ctx, base);
	// 8304F284: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8304F288: 389F0050  addi r4, r31, 0x50
	ctx.r[4].s64 = ctx.r[31].s64 + 80;
	// 8304F28C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304F290: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8304F294: 4BFAA2E5  bl 0x82ff9578
	ctx.lr = 0x8304F298;
	sub_82FF9578(ctx, base);
	// 8304F298: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8304F29C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8304F2A0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304F2A4: 4099007C  ble cr6, 0x8304f320
	if !ctx.cr[6].gt {
	pc = 0x8304F320; continue 'dispatch;
	}
	// 8304F2A8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304F2AC: 4BFA8E05  bl 0x82ff80b0
	ctx.lr = 0x8304F2B0;
	sub_82FF80B0(ctx, base);
	// 8304F2B0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8304F2B4: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 8304F2B8: 909F0058  stw r4, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[4].u32 ) };
	// 8304F2BC: 4BF88FDD  bl 0x82fd8298
	ctx.lr = 0x8304F2C0;
	sub_82FD8298(ctx, base);
	// 8304F2C0: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8304F2C4: 93BF0054  stw r29, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 8304F2C8: 41820020  beq 0x8304f2e8
	if ctx.cr[0].eq {
	pc = 0x8304F2E8; continue 'dispatch;
	}
	// 8304F2CC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304F2D0: 4BFA8DE1  bl 0x82ff80b0
	ctx.lr = 0x8304F2D4;
	sub_82FF80B0(ctx, base);
	// 8304F2D4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8304F2D8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304F2DC: 4801542D  bl 0x83064708
	ctx.lr = 0x8304F2E0;
	sub_83064708(ctx, base);
	// 8304F2E0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8304F2E4: 48000008  b 0x8304f2ec
	pc = 0x8304F2EC; continue 'dispatch;
	// 8304F2E8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8304F2EC: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304F2F0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8304F2F4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304F2F8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8304F2FC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8304F300: 4E800421  bctrl
	ctx.lr = 0x8304F304;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8304F304: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8304F308: 807A0000  lwz r3, 0(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304F30C: 4BFC8FC5  bl 0x830182d0
	ctx.lr = 0x8304F310;
	sub_830182D0(ctx, base);
	// 8304F310: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8304F314: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 8304F318: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8304F31C: 4198FF8C  blt cr6, 0x8304f2a8
	if ctx.cr[6].lt {
	pc = 0x8304F2A8; continue 'dispatch;
	}
	// 8304F320: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 8304F324: 48158E8C  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304F328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304F328 size=44
    let mut pc: u32 = 0x8304F328;
    'dispatch: loop {
        match pc {
            0x8304F328 => {
    //   block [0x8304F328..0x8304F354)
	// 8304F328: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 8304F32C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304F330: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304F334: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304F338: 809F0054  lwz r4, 0x54(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8304F33C: 807F0058  lwz r3, 0x58(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 8304F340: 4BF88FA1  bl 0x82fd82e0
	ctx.lr = 0x8304F344;
	sub_82FD82E0(ctx, base);
	// 8304F344: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8304F348: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304F34C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304F350: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304F354(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304F354 size=44
    let mut pc: u32 = 0x8304F354;
    'dispatch: loop {
        match pc {
            0x8304F354 => {
    //   block [0x8304F354..0x8304F380)
	// 8304F354: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 8304F358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304F35C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304F360: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304F364: 809F0058  lwz r4, 0x58(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 8304F368: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8304F36C: 4BF88F75  bl 0x82fd82e0
	ctx.lr = 0x8304F370;
	sub_82FD82E0(ctx, base);
	// 8304F370: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8304F374: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304F378: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304F37C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


