pub fn sub_83008748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83008748 size=536
    let mut pc: u32 = 0x83008748;
    'dispatch: loop {
        match pc {
            0x83008748 => {
    //   block [0x83008748..0x83008960)
	// 83008748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300874C: 4819FA15  bl 0x831a8160
	ctx.lr = 0x83008750;
	sub_831A8130(ctx, base);
	// 83008750: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 83008754: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83008758: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8300875C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83008760: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 83008764: 93DF00A4  stw r30, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[30].u32 ) };
	// 83008768: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8300876C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83008770: 396B1040  addi r11, r11, 0x1040
	ctx.r[11].s64 = ctx.r[11].s64 + 4160;
	// 83008774: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83008778: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300877C: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83008780: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83008784: 4E800421  bctrl
	ctx.lr = 0x83008788;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83008788: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 8300878C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83008790: 3883000C  addi r4, r3, 0xc
	ctx.r[4].s64 = ctx.r[3].s64 + 12;
	// 83008794: 40820008  bne 0x8300879c
	if !ctx.cr[0].eq {
	pc = 0x8300879C; continue 'dispatch;
	}
	// 83008798: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8300879C: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 830087A0: 4BFD6A79  bl 0x82fdf218
	ctx.lr = 0x830087A4;
	sub_82FDF218(ctx, base);
	// 830087A4: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830087A8: 3B7E000C  addi r27, r30, 0xc
	ctx.r[27].s64 = ctx.r[30].s64 + 12;
	// 830087AC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830087B0: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830087B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830087B8: 4E800421  bctrl
	ctx.lr = 0x830087BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830087BC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830087C0: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 830087C4: 4BFF3CD5  bl 0x82ffc498
	ctx.lr = 0x830087C8;
	sub_82FFC498(ctx, base);
	// 830087C8: 387E001C  addi r3, r30, 0x1c
	ctx.r[3].s64 = ctx.r[30].s64 + 28;
	// 830087CC: 481853E5  bl 0x8318dbb0
	ctx.lr = 0x830087D0;
	sub_8318DBB0(ctx, base);
	// 830087D0: 578B063F  clrlwi. r11, r28, 0x18
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830087D4: 935E0024  stw r26, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[26].u32 ) };
	// 830087D8: 935E0028  stw r26, 0x28(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), ctx.r[26].u32 ) };
	// 830087DC: 817D002C  lwz r11, 0x2c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(44 as u32) ) } as u64;
	// 830087E0: 917E002C  stw r11, 0x2c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 830087E4: 41820010  beq 0x830087f4
	if ctx.cr[0].eq {
	pc = 0x830087F4; continue 'dispatch;
	}
	// 830087E8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830087EC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 830087F0: 4BFF4BA9  bl 0x82ffd398
	ctx.lr = 0x830087F4;
	sub_82FFD398(ctx, base);
	// 830087F4: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830087F8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830087FC: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 83008800: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83008804: 4E800421  bctrl
	ctx.lr = 0x83008808;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83008808: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300880C: 41820030  beq 0x8300883c
	if ctx.cr[0].eq {
	pc = 0x8300883C; continue 'dispatch;
	}
	// 83008810: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83008814: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83008818: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 8300881C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83008820: 4E800421  bctrl
	ctx.lr = 0x83008824;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83008824: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83008828: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8300882C: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 83008830: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83008834: 4E800421  bctrl
	ctx.lr = 0x83008838;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83008838: 907E0024  stw r3, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[3].u32 ) };
	// 8300883C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83008840: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83008844: 816B00F8  lwz r11, 0xf8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(248 as u32) ) } as u64;
	// 83008848: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300884C: 4E800421  bctrl
	ctx.lr = 0x83008850;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83008850: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83008854: 41820030  beq 0x83008884
	if ctx.cr[0].eq {
	pc = 0x83008884; continue 'dispatch;
	}
	// 83008858: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300885C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83008860: 816B00F8  lwz r11, 0xf8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(248 as u32) ) } as u64;
	// 83008864: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83008868: 4E800421  bctrl
	ctx.lr = 0x8300886C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300886C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83008870: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83008874: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 83008878: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300887C: 4E800421  bctrl
	ctx.lr = 0x83008880;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83008880: 907E0028  stw r3, 0x28(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), ctx.r[3].u32 ) };
	// 83008884: 817E0028  lwz r11, 0x28(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 83008888: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8300888C: 409A000C  bne cr6, 0x83008898
	if !ctx.cr[6].eq {
	pc = 0x83008898; continue 'dispatch;
	}
	// 83008890: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83008894: 4BFFF09D  bl 0x83007930
	ctx.lr = 0x83008898;
	sub_83007930(ctx, base);
	// 83008898: 817E0028  lwz r11, 0x28(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 8300889C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830088A0: 409A0034  bne cr6, 0x830088d4
	if !ctx.cr[6].eq {
	pc = 0x830088D4; continue 'dispatch;
	}
	// 830088A4: 807B0000  lwz r3, 0(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 830088A8: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 830088AC: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 830088B0: 4BFD9141  bl 0x82fe19f0
	ctx.lr = 0x830088B4;
	sub_82FE19F0(ctx, base);
	// 830088B4: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 830088B8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830088BC: 41820010  beq 0x830088cc
	if ctx.cr[0].eq {
	pc = 0x830088CC; continue 'dispatch;
	}
	// 830088C0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830088C4: 48030FFD  bl 0x830398c0
	ctx.lr = 0x830088C8;
	sub_830398C0(ctx, base);
	// 830088C8: 48000008  b 0x830088d0
	pc = 0x830088D0; continue 'dispatch;
	// 830088CC: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 830088D0: 907E0028  stw r3, 0x28(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), ctx.r[3].u32 ) };
	// 830088D4: 817E0024  lwz r11, 0x24(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 830088D8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830088DC: 409A0078  bne cr6, 0x83008954
	if !ctx.cr[6].eq {
	pc = 0x83008954; continue 'dispatch;
	}
	// 830088E0: 817E0028  lwz r11, 0x28(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 830088E4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830088E8: 409A0038  bne cr6, 0x83008920
	if !ctx.cr[6].eq {
	pc = 0x83008920; continue 'dispatch;
	}
	// 830088EC: 807B0000  lwz r3, 0(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 830088F0: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 830088F4: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 830088F8: 4BFD90F9  bl 0x82fe19f0
	ctx.lr = 0x830088FC;
	sub_82FE19F0(ctx, base);
	// 830088FC: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83008900: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83008904: 41820010  beq 0x83008914
	if ctx.cr[0].eq {
	pc = 0x83008914; continue 'dispatch;
	}
	// 83008908: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8300890C: 48030FB5  bl 0x830398c0
	ctx.lr = 0x83008910;
	sub_830398C0(ctx, base);
	// 83008910: 48000008  b 0x83008918
	pc = 0x83008918; continue 'dispatch;
	// 83008914: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 83008918: 907E0024  stw r3, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[3].u32 ) };
	// 8300891C: 48000038  b 0x83008954
	pc = 0x83008954; continue 'dispatch;
	// 83008920: 807B0000  lwz r3, 0(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 83008924: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 83008928: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 8300892C: 4BFD90C5  bl 0x82fe19f0
	ctx.lr = 0x83008930;
	sub_82FE19F0(ctx, base);
	// 83008930: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83008934: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83008938: 41820014  beq 0x8300894c
	if ctx.cr[0].eq {
	pc = 0x8300894C; continue 'dispatch;
	}
	// 8300893C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83008940: 80BE0028  lwz r5, 0x28(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 83008944: 4803256D  bl 0x8303aeb0
	ctx.lr = 0x83008948;
	sub_8303AEB0(ctx, base);
	// 83008948: 48000008  b 0x83008950
	pc = 0x83008950; continue 'dispatch;
	// 8300894C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 83008950: 907E0024  stw r3, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[3].u32 ) };
	// 83008954: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83008958: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 8300895C: 4819F854  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83008960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83008960 size=40
    let mut pc: u32 = 0x83008960;
    'dispatch: loop {
        match pc {
            0x83008960 => {
    //   block [0x83008960..0x83008988)
	// 83008960: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 83008964: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83008968: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300896C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83008970: 807F00A4  lwz r3, 0xa4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 83008974: 4BFFD20D  bl 0x83005b80
	ctx.lr = 0x83008978;
	sub_83005B80(ctx, base);
	// 83008978: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300897C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83008980: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83008984: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83008988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83008988 size=44
    let mut pc: u32 = 0x83008988;
    'dispatch: loop {
        match pc {
            0x83008988 => {
    //   block [0x83008988..0x830089B4)
	// 83008988: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 8300898C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83008990: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83008994: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83008998: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 8300899C: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 830089A0: 480BF141  bl 0x830c7ae0
	ctx.lr = 0x830089A4;
	sub_830C7AE0(ctx, base);
	// 830089A4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830089A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830089AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830089B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830089B4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830089B4 size=44
    let mut pc: u32 = 0x830089B4;
    'dispatch: loop {
        match pc {
            0x830089B4 => {
    //   block [0x830089B4..0x830089E0)
	// 830089B4: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 830089B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830089BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830089C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830089C4: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 830089C8: 386B000C  addi r3, r11, 0xc
	ctx.r[3].s64 = ctx.r[11].s64 + 12;
	// 830089CC: 4BFD885D  bl 0x82fe1228
	ctx.lr = 0x830089D0;
	sub_82FE1228(ctx, base);
	// 830089D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830089D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830089D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830089DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830089E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830089E0 size=44
    let mut pc: u32 = 0x830089E0;
    'dispatch: loop {
        match pc {
            0x830089E0 => {
    //   block [0x830089E0..0x83008A0C)
	// 830089E0: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 830089E4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830089E8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830089EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830089F0: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 830089F4: 386B001C  addi r3, r11, 0x1c
	ctx.r[3].s64 = ctx.r[11].s64 + 28;
	// 830089F8: 480BF0E9  bl 0x830c7ae0
	ctx.lr = 0x830089FC;
	sub_830C7AE0(ctx, base);
	// 830089FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83008A00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83008A04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83008A08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83008A0C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83008A0C size=44
    let mut pc: u32 = 0x83008A0C;
    'dispatch: loop {
        match pc {
            0x83008A0C => {
    //   block [0x83008A0C..0x83008A38)
	// 83008A0C: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 83008A10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83008A14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83008A18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83008A1C: 809F0050  lwz r4, 0x50(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83008A20: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 83008A24: 480BF0BD  bl 0x830c7ae0
	ctx.lr = 0x83008A28;
	sub_830C7AE0(ctx, base);
	// 83008A28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83008A2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83008A30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83008A34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83008A38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83008A38 size=44
    let mut pc: u32 = 0x83008A38;
    'dispatch: loop {
        match pc {
            0x83008A38 => {
    //   block [0x83008A38..0x83008A64)
	// 83008A38: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 83008A3C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83008A40: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83008A44: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83008A48: 809F0054  lwz r4, 0x54(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 83008A4C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83008A50: 480BF091  bl 0x830c7ae0
	ctx.lr = 0x83008A54;
	sub_830C7AE0(ctx, base);
	// 83008A54: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83008A58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83008A5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83008A60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83008A64(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83008A64 size=44
    let mut pc: u32 = 0x83008A64;
    'dispatch: loop {
        match pc {
            0x83008A64 => {
    //   block [0x83008A64..0x83008A90)
	// 83008A64: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 83008A68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83008A6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83008A70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83008A74: 809F0054  lwz r4, 0x54(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 83008A78: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83008A7C: 480BF065  bl 0x830c7ae0
	ctx.lr = 0x83008A80;
	sub_830C7AE0(ctx, base);
	// 83008A80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83008A84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83008A88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83008A8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83008A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83008A90 size=8
    let mut pc: u32 = 0x83008A90;
    'dispatch: loop {
        match pc {
            0x83008A90 => {
    //   block [0x83008A90..0x83008A98)
	// 83008A90: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83008A94: 82141450  lwz r16, 0x1450(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(5200 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83008A98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83008A98 size=124
    let mut pc: u32 = 0x83008A98;
    'dispatch: loop {
        match pc {
            0x83008A98 => {
    //   block [0x83008A98..0x83008B14)
	// 83008A98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83008A9C: 4819F6D1  bl 0x831a816c
	ctx.lr = 0x83008AA0;
	sub_831A8130(ctx, base);
	// 83008AA0: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 83008AA4: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83008AA8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83008AAC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83008AB0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83008AB4: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83008AB8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83008ABC: 4E800421  bctrl
	ctx.lr = 0x83008AC0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83008AC0: 38A00006  li r5, 6
	ctx.r[5].s64 = 6;
	// 83008AC4: 38800030  li r4, 0x30
	ctx.r[4].s64 = 48;
	// 83008AC8: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83008ACC: 4BFDBDED  bl 0x82fe48b8
	ctx.lr = 0x83008AD0;
	sub_82FE48B8(ctx, base);
	// 83008AD0: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 83008AD4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83008AD8: 41820018  beq 0x83008af0
	if ctx.cr[0].eq {
	pc = 0x83008AF0; continue 'dispatch;
	}
	// 83008ADC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83008AE0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83008AE4: 4BFFFC65  bl 0x83008748
	ctx.lr = 0x83008AE8;
	sub_83008748(ctx, base);
	// 83008AE8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83008AEC: 48000008  b 0x83008af4
	pc = 0x83008AF4; continue 'dispatch;
	// 83008AF0: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83008AF4: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 83008AF8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83008AFC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83008B00: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 83008B04: 4BFD6C75  bl 0x82fdf778
	ctx.lr = 0x83008B08;
	sub_82FDF778(ctx, base);
	// 83008B08: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83008B0C: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 83008B10: 4819F6AC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83008B14(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83008B14 size=48
    let mut pc: u32 = 0x83008B14;
    'dispatch: loop {
        match pc {
            0x83008B14 => {
    //   block [0x83008B14..0x83008B44)
	// 83008B14: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83008B18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83008B1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83008B20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83008B24: 38A00006  li r5, 6
	ctx.r[5].s64 = 6;
	// 83008B28: 809F0050  lwz r4, 0x50(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83008B2C: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 83008B30: 480BEFB1  bl 0x830c7ae0
	ctx.lr = 0x83008B34;
	sub_830C7AE0(ctx, base);
	// 83008B34: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83008B38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83008B3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83008B40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83008B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83008B48 size=8
    let mut pc: u32 = 0x83008B48;
    'dispatch: loop {
        match pc {
            0x83008B48 => {
    //   block [0x83008B48..0x83008B50)
	// 83008B48: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83008B4C: 82141580  lwz r16, 0x1580(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(5504 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83008B50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83008B50 size=204
    let mut pc: u32 = 0x83008B50;
    'dispatch: loop {
        match pc {
            0x83008B50 => {
    //   block [0x83008B50..0x83008C1C)
	// 83008B50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83008B54: 4819F60D  bl 0x831a8160
	ctx.lr = 0x83008B58;
	sub_831A8130(ctx, base);
	// 83008B58: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 83008B5C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83008B60: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83008B64: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83008B68: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 83008B6C: 93DF00A4  stw r30, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[30].u32 ) };
	// 83008B70: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83008B74: 3B5E0004  addi r26, r30, 4
	ctx.r[26].s64 = ctx.r[30].s64 + 4;
	// 83008B78: 396B1490  addi r11, r11, 0x1490
	ctx.r[11].s64 = ctx.r[11].s64 + 5264;
	// 83008B7C: 389D0004  addi r4, r29, 4
	ctx.r[4].s64 = ctx.r[29].s64 + 4;
	// 83008B80: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 83008B84: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83008B88: 4BFD7A71  bl 0x82fe05f8
	ctx.lr = 0x83008B8C;
	sub_82FE05F8(ctx, base);
	// 83008B8C: 3B9E000C  addi r28, r30, 0xc
	ctx.r[28].s64 = ctx.r[30].s64 + 12;
	// 83008B90: 389D000C  addi r4, r29, 0xc
	ctx.r[4].s64 = ctx.r[29].s64 + 12;
	// 83008B94: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83008B98: 4BFF3949  bl 0x82ffc4e0
	ctx.lr = 0x83008B9C;
	sub_82FFC4E0(ctx, base);
	// 83008B9C: 817D001C  lwz r11, 0x1c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 83008BA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83008BA4: 5769063F  clrlwi. r9, r27, 0x18
	ctx.r[9].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 83008BA8: 917E001C  stw r11, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 83008BAC: 817D0020  lwz r11, 0x20(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32 as u32) ) } as u64;
	// 83008BB0: 917E0020  stw r11, 0x20(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 83008BB4: 817D0024  lwz r11, 0x24(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 83008BB8: 917E0024  stw r11, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 83008BBC: 817D0028  lwz r11, 0x28(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(40 as u32) ) } as u64;
	// 83008BC0: 917E0028  stw r11, 0x28(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 83008BC4: 817D002C  lwz r11, 0x2c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(44 as u32) ) } as u64;
	// 83008BC8: 917E002C  stw r11, 0x2c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 83008BCC: 817D0030  lwz r11, 0x30(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(48 as u32) ) } as u64;
	// 83008BD0: 917E0030  stw r11, 0x30(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 83008BD4: 817D0034  lwz r11, 0x34(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(52 as u32) ) } as u64;
	// 83008BD8: 917E0034  stw r11, 0x34(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 83008BDC: 817D0038  lwz r11, 0x38(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(56 as u32) ) } as u64;
	// 83008BE0: 917E0038  stw r11, 0x38(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 83008BE4: 817D003C  lwz r11, 0x3c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(60 as u32) ) } as u64;
	// 83008BE8: 995E0040  stb r10, 0x40(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(64 as u32), ctx.r[10].u8 ) };
	// 83008BEC: 917E003C  stw r11, 0x3c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 83008BF0: 41820010  beq 0x83008c00
	if ctx.cr[0].eq {
	pc = 0x83008C00; continue 'dispatch;
	}
	// 83008BF4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83008BF8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83008BFC: 4BFF479D  bl 0x82ffd398
	ctx.lr = 0x83008C00;
	sub_82FFD398(ctx, base);
	// 83008C00: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83008C04: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83008C08: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 83008C0C: 4BFD6785  bl 0x82fdf390
	ctx.lr = 0x83008C10;
	sub_82FDF390(ctx, base);
	// 83008C10: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83008C14: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 83008C18: 4819F598  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83008C1C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83008C1C size=40
    let mut pc: u32 = 0x83008C1C;
    'dispatch: loop {
        match pc {
            0x83008C1C => {
    //   block [0x83008C1C..0x83008C44)
	// 83008C1C: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 83008C20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83008C24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83008C28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83008C2C: 807F00A4  lwz r3, 0xa4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 83008C30: 4BFFCF51  bl 0x83005b80
	ctx.lr = 0x83008C34;
	sub_83005B80(ctx, base);
	// 83008C34: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83008C38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83008C3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83008C40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83008C44(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83008C44 size=44
    let mut pc: u32 = 0x83008C44;
    'dispatch: loop {
        match pc {
            0x83008C44 => {
    //   block [0x83008C44..0x83008C70)
	// 83008C44: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 83008C48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83008C4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83008C50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83008C54: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 83008C58: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 83008C5C: 480BEE85  bl 0x830c7ae0
	ctx.lr = 0x83008C60;
	sub_830C7AE0(ctx, base);
	// 83008C60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83008C64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83008C68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83008C6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83008C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83008C70 size=44
    let mut pc: u32 = 0x83008C70;
    'dispatch: loop {
        match pc {
            0x83008C70 => {
    //   block [0x83008C70..0x83008C9C)
	// 83008C70: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 83008C74: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83008C78: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83008C7C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83008C80: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 83008C84: 386B000C  addi r3, r11, 0xc
	ctx.r[3].s64 = ctx.r[11].s64 + 12;
	// 83008C88: 4BFD85A1  bl 0x82fe1228
	ctx.lr = 0x83008C8C;
	sub_82FE1228(ctx, base);
	// 83008C8C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83008C90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83008C94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83008C98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83008CA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83008CA0 size=8
    let mut pc: u32 = 0x83008CA0;
    'dispatch: loop {
        match pc {
            0x83008CA0 => {
    //   block [0x83008CA0..0x83008CA8)
	// 83008CA0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83008CA4: 821415D8  lwz r16, 0x15d8(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(5592 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83008CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83008CA8 size=100
    let mut pc: u32 = 0x83008CA8;
    'dispatch: loop {
        match pc {
            0x83008CA8 => {
    //   block [0x83008CA8..0x83008D0C)
	// 83008CA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83008CAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83008CB0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83008CB4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83008CB8: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 83008CBC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83008CC0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83008CC4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83008CC8: 396B1490  addi r11, r11, 0x1490
	ctx.r[11].s64 = ctx.r[11].s64 + 5264;
	// 83008CCC: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 83008CD0: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83008CD4: 397E000C  addi r11, r30, 0xc
	ctx.r[11].s64 = ctx.r[30].s64 + 12;
	// 83008CD8: 386B0008  addi r3, r11, 8
	ctx.r[3].s64 = ctx.r[11].s64 + 8;
	// 83008CDC: 4BFF132D  bl 0x82ffa008
	ctx.lr = 0x83008CE0;
	sub_82FFA008(ctx, base);
	// 83008CE0: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 83008CE4: 480BEDFD  bl 0x830c7ae0
	ctx.lr = 0x83008CE8;
	sub_830C7AE0(ctx, base);
	// 83008CE8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83008CEC: 396BA8A0  addi r11, r11, -0x5760
	ctx.r[11].s64 = ctx.r[11].s64 + -22368;
	// 83008CF0: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83008CF4: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 83008CF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83008CFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83008D00: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83008D04: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83008D08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83008D0C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83008D0C size=40
    let mut pc: u32 = 0x83008D0C;
    'dispatch: loop {
        match pc {
            0x83008D0C => {
    //   block [0x83008D0C..0x83008D34)
	// 83008D0C: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 83008D10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83008D14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83008D18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83008D1C: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83008D20: 4BFFCE61  bl 0x83005b80
	ctx.lr = 0x83008D24;
	sub_83005B80(ctx, base);
	// 83008D24: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83008D28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83008D2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83008D30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83008D34(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83008D34 size=44
    let mut pc: u32 = 0x83008D34;
    'dispatch: loop {
        match pc {
            0x83008D34 => {
    //   block [0x83008D34..0x83008D60)
	// 83008D34: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 83008D38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83008D3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83008D40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83008D44: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83008D48: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 83008D4C: 480BED95  bl 0x830c7ae0
	ctx.lr = 0x83008D50;
	sub_830C7AE0(ctx, base);
	// 83008D50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83008D54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83008D58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83008D5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83008D60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83008D60 size=8
    let mut pc: u32 = 0x83008D60;
    'dispatch: loop {
        match pc {
            0x83008D60 => {
    //   block [0x83008D60..0x83008D68)
	// 83008D60: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83008D64: 82141618  lwz r16, 0x1618(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(5656 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83008D68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83008D68 size=124
    let mut pc: u32 = 0x83008D68;
    'dispatch: loop {
        match pc {
            0x83008D68 => {
    //   block [0x83008D68..0x83008DE4)
	// 83008D68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83008D6C: 4819F401  bl 0x831a816c
	ctx.lr = 0x83008D70;
	sub_831A8130(ctx, base);
	// 83008D70: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 83008D74: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83008D78: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83008D7C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83008D80: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83008D84: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83008D88: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83008D8C: 4E800421  bctrl
	ctx.lr = 0x83008D90;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83008D90: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 83008D94: 38800044  li r4, 0x44
	ctx.r[4].s64 = 68;
	// 83008D98: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83008D9C: 4BFDBB1D  bl 0x82fe48b8
	ctx.lr = 0x83008DA0;
	sub_82FE48B8(ctx, base);
	// 83008DA0: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 83008DA4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83008DA8: 41820018  beq 0x83008dc0
	if ctx.cr[0].eq {
	pc = 0x83008DC0; continue 'dispatch;
	}
	// 83008DAC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83008DB0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83008DB4: 4BFFFD9D  bl 0x83008b50
	ctx.lr = 0x83008DB8;
	sub_83008B50(ctx, base);
	// 83008DB8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83008DBC: 48000008  b 0x83008dc4
	pc = 0x83008DC4; continue 'dispatch;
	// 83008DC0: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83008DC4: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 83008DC8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83008DCC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83008DD0: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 83008DD4: 4BFD69A5  bl 0x82fdf778
	ctx.lr = 0x83008DD8;
	sub_82FDF778(ctx, base);
	// 83008DD8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83008DDC: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 83008DE0: 4819F3DC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83008DE4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83008DE4 size=48
    let mut pc: u32 = 0x83008DE4;
    'dispatch: loop {
        match pc {
            0x83008DE4 => {
    //   block [0x83008DE4..0x83008E14)
	// 83008DE4: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83008DE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83008DEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83008DF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83008DF4: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 83008DF8: 809F0050  lwz r4, 0x50(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83008DFC: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 83008E00: 480BECE1  bl 0x830c7ae0
	ctx.lr = 0x83008E04;
	sub_830C7AE0(ctx, base);
	// 83008E04: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83008E08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83008E0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83008E10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83008E18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83008E18 size=8
    let mut pc: u32 = 0x83008E18;
    'dispatch: loop {
        match pc {
            0x83008E18 => {
    //   block [0x83008E18..0x83008E20)
	// 83008E18: 38600006  li r3, 6
	ctx.r[3].s64 = 6;
	// 83008E1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83008E20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83008E20 size=8
    let mut pc: u32 = 0x83008E20;
    'dispatch: loop {
        match pc {
            0x83008E20 => {
    //   block [0x83008E20..0x83008E28)
	// 83008E20: 80630020  lwz r3, 0x20(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 83008E24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83008E28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83008E28 size=8
    let mut pc: u32 = 0x83008E28;
    'dispatch: loop {
        match pc {
            0x83008E28 => {
    //   block [0x83008E28..0x83008E30)
	// 83008E28: 8063003C  lwz r3, 0x3c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(60 as u32) ) } as u64;
	// 83008E2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83008E30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83008E30 size=80
    let mut pc: u32 = 0x83008E30;
    'dispatch: loop {
        match pc {
            0x83008E30 => {
    //   block [0x83008E30..0x83008E80)
	// 83008E30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83008E34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83008E38: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83008E3C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83008E40: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83008E44: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83008E48: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83008E4C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83008E50: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83008E54: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83008E58: 4E800421  bctrl
	ctx.lr = 0x83008E5C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83008E5C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83008E60: 4BFD9DA1  bl 0x82fe2c00
	ctx.lr = 0x83008E64;
	sub_82FE2C00(ctx, base);
	// 83008E64: 907F0028  stw r3, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[3].u32 ) };
	// 83008E68: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83008E6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83008E70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83008E74: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83008E78: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83008E7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83008E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83008E80 size=80
    let mut pc: u32 = 0x83008E80;
    'dispatch: loop {
        match pc {
            0x83008E80 => {
    //   block [0x83008E80..0x83008ED0)
	// 83008E80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83008E84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83008E88: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83008E8C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83008E90: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83008E94: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83008E98: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83008E9C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83008EA0: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83008EA4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83008EA8: 4E800421  bctrl
	ctx.lr = 0x83008EAC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83008EAC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83008EB0: 4BFD9D51  bl 0x82fe2c00
	ctx.lr = 0x83008EB4;
	sub_82FE2C00(ctx, base);
	// 83008EB4: 907F0020  stw r3, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[3].u32 ) };
	// 83008EB8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83008EBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83008EC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83008EC4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83008EC8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83008ECC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83008ED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83008ED0 size=80
    let mut pc: u32 = 0x83008ED0;
    'dispatch: loop {
        match pc {
            0x83008ED0 => {
    //   block [0x83008ED0..0x83008F20)
	// 83008ED0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83008ED4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83008ED8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83008EDC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83008EE0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83008EE4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83008EE8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83008EEC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83008EF0: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83008EF4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83008EF8: 4E800421  bctrl
	ctx.lr = 0x83008EFC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83008EFC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83008F00: 4BFD9D01  bl 0x82fe2c00
	ctx.lr = 0x83008F04;
	sub_82FE2C00(ctx, base);
	// 83008F04: 907F0024  stw r3, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[3].u32 ) };
	// 83008F08: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83008F0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83008F10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83008F14: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83008F18: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83008F1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83008F20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83008F20 size=144
    let mut pc: u32 = 0x83008F20;
    'dispatch: loop {
        match pc {
            0x83008F20 => {
    //   block [0x83008F20..0x83008FB0)
	// 83008F20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83008F24: 4819F249  bl 0x831a816c
	ctx.lr = 0x83008F28;
	sub_831A8130(ctx, base);
	// 83008F28: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83008F2C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83008F30: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83008F34: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83008F38: 419A0068  beq cr6, 0x83008fa0
	if ctx.cr[6].eq {
	pc = 0x83008FA0; continue 'dispatch;
	}
	// 83008F3C: A17F0000  lhz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83008F40: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83008F44: 4182005C  beq 0x83008fa0
	if ctx.cr[0].eq {
	pc = 0x83008FA0; continue 'dispatch;
	}
	// 83008F48: 397F0002  addi r11, r31, 2
	ctx.r[11].s64 = ctx.r[31].s64 + 2;
	// 83008F4C: 48000008  b 0x83008f54
	pc = 0x83008F54; continue 'dispatch;
	// 83008F50: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 83008F54: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83008F58: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83008F5C: 4082FFF4  bne 0x83008f50
	if !ctx.cr[0].eq {
	pc = 0x83008F50; continue 'dispatch;
	}
	// 83008F60: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83008F64: 7D7F5850  subf r11, r31, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[31].s64;
	// 83008F68: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83008F6C: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 83008F70: 3BCB0009  addi r30, r11, 9
	ctx.r[30].s64 = ctx.r[11].s64 + 9;
	// 83008F74: 814A002C  lwz r10, 0x2c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(44 as u32) ) } as u64;
	// 83008F78: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 83008F7C: 4E800421  bctrl
	ctx.lr = 0x83008F80;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83008F80: 57C4083C  slwi r4, r30, 1
	ctx.r[4].u32 = ctx.r[30].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 83008F84: 4BFD8A6D  bl 0x82fe19f0
	ctx.lr = 0x83008F88;
	sub_82FE19F0(ctx, base);
	// 83008F88: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83008F8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83008F90: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83008F94: 4BFC9575  bl 0x82fd2508
	ctx.lr = 0x83008F98;
	sub_82FD2508(ctx, base);
	// 83008F98: 93DD003C  stw r30, 0x3c(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(60 as u32), ctx.r[30].u32 ) };
	// 83008F9C: 4800000C  b 0x83008fa8
	pc = 0x83008FA8; continue 'dispatch;
	// 83008FA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83008FA4: 917D003C  stw r11, 0x3c(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 83008FA8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83008FAC: 4819F210  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83008FB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83008FB0 size=8
    let mut pc: u32 = 0x83008FB0;
    'dispatch: loop {
        match pc {
            0x83008FB0 => {
    //   block [0x83008FB0..0x83008FB8)
	// 83008FB0: 9083002C  stw r4, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[4].u32 ) };
	// 83008FB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83008FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83008FB8 size=8
    let mut pc: u32 = 0x83008FB8;
    'dispatch: loop {
        match pc {
            0x83008FB8 => {
    //   block [0x83008FB8..0x83008FC0)
	// 83008FB8: 8063002C  lwz r3, 0x2c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 83008FBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83008FC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83008FC0 size=140
    let mut pc: u32 = 0x83008FC0;
    'dispatch: loop {
        match pc {
            0x83008FC0 => {
    //   block [0x83008FC0..0x8300904C)
	// 83008FC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83008FC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83008FC8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83008FCC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83008FD0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83008FD4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83008FD8: 897F0040  lbz r11, 0x40(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 83008FDC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83008FE0: 40820054  bne 0x83009034
	if !ctx.cr[0].eq {
	pc = 0x83009034; continue 'dispatch;
	}
	// 83008FE4: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83008FE8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83008FEC: 409A0048  bne cr6, 0x83009034
	if !ctx.cr[6].eq {
	pc = 0x83009034; continue 'dispatch;
	}
	// 83008FF0: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 83008FF4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83008FF8: 419A003C  beq cr6, 0x83009034
	if ctx.cr[6].eq {
	pc = 0x83009034; continue 'dispatch;
	}
	// 83008FFC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83009000: 3BDF0004  addi r30, r31, 4
	ctx.r[30].s64 = ctx.r[31].s64 + 4;
	// 83009004: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83009008: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8300900C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83009010: 997F0040  stb r11, 0x40(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[11].u8 ) };
	// 83009014: 4BFD637D  bl 0x82fdf390
	ctx.lr = 0x83009018;
	sub_82FDF390(ctx, base);
	// 83009018: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 8300901C: 809F002C  lwz r4, 0x2c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 83009020: 4BFF4379  bl 0x82ffd398
	ctx.lr = 0x83009024;
	sub_82FFD398(ctx, base);
	// 83009024: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83009028: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8300902C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83009030: 4BFD6361  bl 0x82fdf390
	ctx.lr = 0x83009034;
	sub_82FDF390(ctx, base);
	// 83009034: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83009038: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300903C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83009040: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83009044: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83009048: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83009050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83009050 size=48
    let mut pc: u32 = 0x83009050;
    'dispatch: loop {
        match pc {
            0x83009050 => {
    //   block [0x83009050..0x83009080)
	// 83009050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83009054: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83009058: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8300905C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83009060: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83009064: 4BFFFF5D  bl 0x83008fc0
	ctx.lr = 0x83009068;
	sub_83008FC0(ctx, base);
	// 83009068: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8300906C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83009070: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83009074: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83009078: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8300907C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83009080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83009080 size=52
    let mut pc: u32 = 0x83009080;
    'dispatch: loop {
        match pc {
            0x83009080 => {
    //   block [0x83009080..0x830090B4)
	// 83009080: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83009084: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83009088: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8300908C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83009090: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83009094: 4BFFFF2D  bl 0x83008fc0
	ctx.lr = 0x83009098;
	sub_83008FC0(ctx, base);
	// 83009098: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 8300909C: 4BFF36A5  bl 0x82ffc740
	ctx.lr = 0x830090A0;
	sub_82FFC740(ctx, base);
	// 830090A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830090A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830090A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830090AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830090B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830090B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830090B8 size=52
    let mut pc: u32 = 0x830090B8;
    'dispatch: loop {
        match pc {
            0x830090B8 => {
    //   block [0x830090B8..0x830090EC)
	// 830090B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830090BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830090C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830090C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830090C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830090CC: 4BFFFEF5  bl 0x83008fc0
	ctx.lr = 0x830090D0;
	sub_83008FC0(ctx, base);
	// 830090D0: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 830090D4: 4BFF3465  bl 0x82ffc538
	ctx.lr = 0x830090D8;
	sub_82FFC538(ctx, base);
	// 830090D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830090DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830090E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830090E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830090E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830090F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830090F0 size=64
    let mut pc: u32 = 0x830090F0;
    'dispatch: loop {
        match pc {
            0x830090F0 => {
    //   block [0x830090F0..0x83009130)
	// 830090F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830090F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830090F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830090FC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83009100: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83009104: 4BFFFEBD  bl 0x83008fc0
	ctx.lr = 0x83009108;
	sub_83008FC0(ctx, base);
	// 83009108: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8300910C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83009110: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83009114: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 83009118: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 8300911C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83009120: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83009124: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83009128: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8300912C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83009130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83009130 size=376
    let mut pc: u32 = 0x83009130;
    'dispatch: loop {
        match pc {
            0x83009130 => {
    //   block [0x83009130..0x830092A8)
	// 83009130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83009134: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83009138: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8300913C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83009140: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83009144: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83009148: 3D408214  lis r10, -0x7dec
	ctx.r[10].s64 = -2112618496;
	// 8300914C: A17F0008  lhz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83009150: A14AA6A4  lhz r10, -0x595c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(-22876 as u32) ) } as u64;
	// 83009154: 7D4A5838  and r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 83009158: 7D4A0034  cntlzw r10, r10
	ctx.r[10].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 8300915C: 554ADFFE  rlwinm r10, r10, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 83009160: 694A0001  xori r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u64 ^ 1;
	// 83009164: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83009168: 41820080  beq 0x830091e8
	if ctx.cr[0].eq {
	pc = 0x830091E8; continue 'dispatch;
	}
	// 8300916C: 3D408214  lis r10, -0x7dec
	ctx.r[10].s64 = -2112618496;
	// 83009170: A14AA6C8  lhz r10, -0x5938(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(-22840 as u32) ) } as u64;
	// 83009174: 7D4B5838  and r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 83009178: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8300917C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83009180: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 83009184: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83009188: 40820060  bne 0x830091e8
	if !ctx.cr[0].eq {
	pc = 0x830091E8; continue 'dispatch;
	}
	// 8300918C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83009190: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83009194: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83009198: 4E800421  bctrl
	ctx.lr = 0x8300919C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300919C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830091A0: 41820020  beq 0x830091c0
	if ctx.cr[0].eq {
	pc = 0x830091C0; continue 'dispatch;
	}
	// 830091A4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830091A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830091AC: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830091B0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830091B4: 4E800421  bctrl
	ctx.lr = 0x830091B8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830091B8: 80C30090  lwz r6, 0x90(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 830091BC: 4800000C  b 0x830091c8
	pc = 0x830091C8; continue 'dispatch;
	// 830091C0: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830091C4: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 830091C8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830091CC: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 830091D0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830091D4: 4BFF0CFD  bl 0x82ff9ed0
	ctx.lr = 0x830091D8;
	sub_82FF9ED0(ctx, base);
	// 830091D8: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 830091DC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830091E0: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 830091E4: 481A7A45  bl 0x831b0c28
	ctx.lr = 0x830091E8;
	sub_831B0C28(ctx, base);
	// 830091E8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830091EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830091F0: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830091F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830091F8: 4E800421  bctrl
	ctx.lr = 0x830091FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830091FC: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83009200: 41820048  beq 0x83009248
	if ctx.cr[0].eq {
	pc = 0x83009248; continue 'dispatch;
	}
	// 83009204: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83009208: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8300920C: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 83009210: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 83009214: 4BFD6565  bl 0x82fdf778
	ctx.lr = 0x83009218;
	sub_82FDF778(ctx, base);
	// 83009218: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 8300921C: 4BFF346D  bl 0x82ffc688
	ctx.lr = 0x83009220;
	sub_82FFC688(ctx, base);
	// 83009220: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 83009224: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83009228: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8300922C: 4BFDC4B5  bl 0x82fe56e0
	ctx.lr = 0x83009230;
	sub_82FE56E0(ctx, base);
	// 83009230: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83009234: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83009238: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300923C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83009240: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83009244: 4E800020  blr
	return;
	// 83009248: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300924C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83009250: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83009254: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83009258: 4E800421  bctrl
	ctx.lr = 0x8300925C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300925C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83009260: 41820020  beq 0x83009280
	if ctx.cr[0].eq {
	pc = 0x83009280; continue 'dispatch;
	}
	// 83009264: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83009268: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300926C: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83009270: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83009274: 4E800421  bctrl
	ctx.lr = 0x83009278;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83009278: 80C30090  lwz r6, 0x90(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 8300927C: 4800000C  b 0x83009288
	pc = 0x83009288; continue 'dispatch;
	// 83009280: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83009284: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83009288: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8300928C: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 83009290: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83009294: 4BFF0C3D  bl 0x82ff9ed0
	ctx.lr = 0x83009298;
	sub_82FF9ED0(ctx, base);
	// 83009298: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8300929C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830092A0: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 830092A4: 481A7985  bl 0x831b0c28
	ctx.lr = 0x830092A8;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830092A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830092A8 size=68
    let mut pc: u32 = 0x830092A8;
    'dispatch: loop {
        match pc {
            0x830092A8 => {
    //   block [0x830092A8..0x830092EC)
	// 830092A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830092AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830092B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830092B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830092B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830092BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830092C0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830092C4: 4BFFFCFD  bl 0x83008fc0
	ctx.lr = 0x830092C8;
	sub_83008FC0(ctx, base);
	// 830092C8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830092CC: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 830092D0: 4BFF40C1  bl 0x82ffd390
	ctx.lr = 0x830092D4;
	sub_82FFD390(ctx, base);
	// 830092D4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830092D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830092DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830092E0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830092E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830092E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830092F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830092F0 size=52
    let mut pc: u32 = 0x830092F0;
    'dispatch: loop {
        match pc {
            0x830092F0 => {
    //   block [0x830092F0..0x83009324)
	// 830092F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830092F4: 4819EE79  bl 0x831a816c
	ctx.lr = 0x830092F8;
	sub_831A8130(ctx, base);
	// 830092F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830092FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83009300: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83009304: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83009308: 4BFFFCB9  bl 0x83008fc0
	ctx.lr = 0x8300930C;
	sub_83008FC0(ctx, base);
	// 8300930C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83009310: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83009314: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 83009318: 4BFF3431  bl 0x82ffc748
	ctx.lr = 0x8300931C;
	sub_82FFC748(ctx, base);
	// 8300931C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83009320: 4819EE9C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83009328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83009328 size=52
    let mut pc: u32 = 0x83009328;
    'dispatch: loop {
        match pc {
            0x83009328 => {
    //   block [0x83009328..0x8300935C)
	// 83009328: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300932C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83009330: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83009334: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83009338: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8300933C: 4BFFFC85  bl 0x83008fc0
	ctx.lr = 0x83009340;
	sub_83008FC0(ctx, base);
	// 83009340: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 83009344: 4BFF3F2D  bl 0x82ffd270
	ctx.lr = 0x83009348;
	sub_82FFD270(ctx, base);
	// 83009348: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300934C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83009350: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83009354: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83009358: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83009360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83009360 size=68
    let mut pc: u32 = 0x83009360;
    'dispatch: loop {
        match pc {
            0x83009360 => {
    //   block [0x83009360..0x830093A4)
	// 83009360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83009364: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83009368: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8300936C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83009370: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83009374: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83009378: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8300937C: 4BFFFC45  bl 0x83008fc0
	ctx.lr = 0x83009380;
	sub_83008FC0(ctx, base);
	// 83009380: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83009384: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 83009388: 4BFF3AC1  bl 0x82ffce48
	ctx.lr = 0x8300938C;
	sub_82FFCE48(ctx, base);
	// 8300938C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83009390: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83009394: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83009398: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8300939C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830093A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830093A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830093A8 size=52
    let mut pc: u32 = 0x830093A8;
    'dispatch: loop {
        match pc {
            0x830093A8 => {
    //   block [0x830093A8..0x830093DC)
	// 830093A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830093AC: 4819EDC1  bl 0x831a816c
	ctx.lr = 0x830093B0;
	sub_831A8130(ctx, base);
	// 830093B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830093B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830093B8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830093BC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 830093C0: 4BFFFC01  bl 0x83008fc0
	ctx.lr = 0x830093C4;
	sub_83008FC0(ctx, base);
	// 830093C4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830093C8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830093CC: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 830093D0: 4BFF3E59  bl 0x82ffd228
	ctx.lr = 0x830093D4;
	sub_82FFD228(ctx, base);
	// 830093D4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830093D8: 4819EDE4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830093E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830093E0 size=68
    let mut pc: u32 = 0x830093E0;
    'dispatch: loop {
        match pc {
            0x830093E0 => {
    //   block [0x830093E0..0x83009424)
	// 830093E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830093E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830093E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830093EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830093F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830093F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830093F8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830093FC: 4BFFFBC5  bl 0x83008fc0
	ctx.lr = 0x83009400;
	sub_83008FC0(ctx, base);
	// 83009400: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83009404: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 83009408: 4BFF31A1  bl 0x82ffc5a8
	ctx.lr = 0x8300940C;
	sub_82FFC5A8(ctx, base);
	// 8300940C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83009410: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83009414: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83009418: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8300941C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83009420: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83009428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83009428 size=8
    let mut pc: u32 = 0x83009428;
    'dispatch: loop {
        match pc {
            0x83009428 => {
    //   block [0x83009428..0x83009430)
	// 83009428: 38630004  addi r3, r3, 4
	ctx.r[3].s64 = ctx.r[3].s64 + 4;
	// 8300942C: 4BFD6D84  b 0x82fe01b0
	sub_82FE01B0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83009430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83009430 size=80
    let mut pc: u32 = 0x83009430;
    'dispatch: loop {
        match pc {
            0x83009430 => {
    //   block [0x83009430..0x83009480)
	// 83009430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83009434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83009438: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8300943C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83009440: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83009444: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83009448: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8300944C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83009450: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83009454: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83009458: 4E800421  bctrl
	ctx.lr = 0x8300945C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300945C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83009460: 4BFD97A1  bl 0x82fe2c00
	ctx.lr = 0x83009464;
	sub_82FE2C00(ctx, base);
	// 83009464: 907F0030  stw r3, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[3].u32 ) };
	// 83009468: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8300946C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83009470: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83009474: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83009478: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8300947C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83009480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83009480 size=8
    let mut pc: u32 = 0x83009480;
    'dispatch: loop {
        match pc {
            0x83009480 => {
    //   block [0x83009480..0x83009488)
	// 83009480: 80630034  lwz r3, 0x34(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 83009484: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83009488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83009488 size=80
    let mut pc: u32 = 0x83009488;
    'dispatch: loop {
        match pc {
            0x83009488 => {
    //   block [0x83009488..0x830094D8)
	// 83009488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300948C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83009490: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83009494: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83009498: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300949C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830094A0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830094A4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830094A8: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830094AC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830094B0: 4E800421  bctrl
	ctx.lr = 0x830094B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830094B4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830094B8: 4BFD9749  bl 0x82fe2c00
	ctx.lr = 0x830094BC;
	sub_82FE2C00(ctx, base);
	// 830094BC: 907F0034  stw r3, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[3].u32 ) };
	// 830094C0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830094C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830094C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830094CC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830094D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830094D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830094D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830094D8 size=8
    let mut pc: u32 = 0x830094D8;
    'dispatch: loop {
        match pc {
            0x830094D8 => {
    //   block [0x830094D8..0x830094E0)
	// 830094D8: 80630038  lwz r3, 0x38(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) } as u64;
	// 830094DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830094E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830094E0 size=80
    let mut pc: u32 = 0x830094E0;
    'dispatch: loop {
        match pc {
            0x830094E0 => {
    //   block [0x830094E0..0x83009530)
	// 830094E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830094E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830094E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830094EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830094F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830094F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830094F8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830094FC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83009500: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83009504: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83009508: 4E800421  bctrl
	ctx.lr = 0x8300950C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300950C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83009510: 4BFD96F1  bl 0x82fe2c00
	ctx.lr = 0x83009514;
	sub_82FE2C00(ctx, base);
	// 83009514: 907F0038  stw r3, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[3].u32 ) };
	// 83009518: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8300951C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83009520: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83009524: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83009528: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8300952C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83009530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83009530 size=8
    let mut pc: u32 = 0x83009530;
    'dispatch: loop {
        match pc {
            0x83009530 => {
    //   block [0x83009530..0x83009538)
	// 83009530: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83009534: 82141670  lwz r16, 0x1670(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(5744 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83009538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83009538 size=172
    let mut pc: u32 = 0x83009538;
    'dispatch: loop {
        match pc {
            0x83009538 => {
    //   block [0x83009538..0x830095E4)
	// 83009538: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300953C: 4819EC25  bl 0x831a8160
	ctx.lr = 0x83009540;
	sub_831A8130(ctx, base);
	// 83009540: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 83009544: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83009548: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8300954C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 83009550: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 83009554: 93DF00A4  stw r30, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[30].u32 ) };
	// 83009558: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8300955C: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 83009560: 396B1490  addi r11, r11, 0x1490
	ctx.r[11].s64 = ctx.r[11].s64 + 5264;
	// 83009564: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83009568: 389C000C  addi r4, r28, 0xc
	ctx.r[4].s64 = ctx.r[28].s64 + 12;
	// 8300956C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83009570: 409A0008  bne cr6, 0x83009578
	if !ctx.cr[6].eq {
	pc = 0x83009578; continue 'dispatch;
	}
	// 83009574: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83009578: 3B7E0004  addi r27, r30, 4
	ctx.r[27].s64 = ctx.r[30].s64 + 4;
	// 8300957C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83009580: 4BFD5C99  bl 0x82fdf218
	ctx.lr = 0x83009584;
	sub_82FDF218(ctx, base);
	// 83009584: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83009588: 387E000C  addi r3, r30, 0xc
	ctx.r[3].s64 = ctx.r[30].s64 + 12;
	// 8300958C: 4BFF2F0D  bl 0x82ffc498
	ctx.lr = 0x83009590;
	sub_82FFC498(ctx, base);
	// 83009590: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 83009594: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83009598: 93BE0020  stw r29, 0x20(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[29].u32 ) };
	// 8300959C: 93BE0024  stw r29, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[29].u32 ) };
	// 830095A0: 93BE0028  stw r29, 0x28(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), ctx.r[29].u32 ) };
	// 830095A4: 93BE002C  stw r29, 0x2c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(44 as u32), ctx.r[29].u32 ) };
	// 830095A8: 93BE0030  stw r29, 0x30(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(48 as u32), ctx.r[29].u32 ) };
	// 830095AC: 93BE0034  stw r29, 0x34(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(52 as u32), ctx.r[29].u32 ) };
	// 830095B0: 93BE0038  stw r29, 0x38(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(56 as u32), ctx.r[29].u32 ) };
	// 830095B4: 93BE003C  stw r29, 0x3c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(60 as u32), ctx.r[29].u32 ) };
	// 830095B8: 9BBE0040  stb r29, 0x40(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(64 as u32), ctx.r[29].u8 ) };
	// 830095BC: 4BFD841D  bl 0x82fe19d8
	ctx.lr = 0x830095C0;
	sub_82FE19D8(ctx, base);
	// 830095C0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 830095C4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 830095C8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830095CC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 830095D0: 917E001C  stw r11, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 830095D4: 4BFD5DBD  bl 0x82fdf390
	ctx.lr = 0x830095D8;
	sub_82FDF390(ctx, base);
	// 830095D8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830095DC: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 830095E0: 4819EBD0  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830095E4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830095E4 size=40
    let mut pc: u32 = 0x830095E4;
    'dispatch: loop {
        match pc {
            0x830095E4 => {
    //   block [0x830095E4..0x8300960C)
	// 830095E4: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 830095E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830095EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830095F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830095F4: 807F00A4  lwz r3, 0xa4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 830095F8: 4BFFC589  bl 0x83005b80
	ctx.lr = 0x830095FC;
	sub_83005B80(ctx, base);
	// 830095FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83009600: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83009604: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83009608: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300960C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300960C size=44
    let mut pc: u32 = 0x8300960C;
    'dispatch: loop {
        match pc {
            0x8300960C => {
    //   block [0x8300960C..0x83009638)
	// 8300960C: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 83009610: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83009614: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83009618: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300961C: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 83009620: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 83009624: 480BE4BD  bl 0x830c7ae0
	ctx.lr = 0x83009628;
	sub_830C7AE0(ctx, base);
	// 83009628: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300962C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83009630: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83009634: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83009638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83009638 size=44
    let mut pc: u32 = 0x83009638;
    'dispatch: loop {
        match pc {
            0x83009638 => {
    //   block [0x83009638..0x83009664)
	// 83009638: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 8300963C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83009640: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83009644: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83009648: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 8300964C: 386B000C  addi r3, r11, 0xc
	ctx.r[3].s64 = ctx.r[11].s64 + 12;
	// 83009650: 4BFD7BD9  bl 0x82fe1228
	ctx.lr = 0x83009654;
	sub_82FE1228(ctx, base);
	// 83009654: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83009658: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300965C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83009660: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83009668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83009668 size=76
    let mut pc: u32 = 0x83009668;
    'dispatch: loop {
        match pc {
            0x83009668 => {
    //   block [0x83009668..0x830096B4)
	// 83009668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300966C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83009670: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83009674: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83009678: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300967C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83009680: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83009684: 4BFFF625  bl 0x83008ca8
	ctx.lr = 0x83009688;
	sub_83008CA8(ctx, base);
	// 83009688: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8300968C: 4182000C  beq 0x83009698
	if ctx.cr[0].eq {
	pc = 0x83009698; continue 'dispatch;
	}
	// 83009690: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83009694: 4B2B6BD5  bl 0x822c0268
	ctx.lr = 0x83009698;
	sub_822C0268(ctx, base);
	// 83009698: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300969C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830096A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830096A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830096A8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830096AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830096B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830096B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830096B8 size=8
    let mut pc: u32 = 0x830096B8;
    'dispatch: loop {
        match pc {
            0x830096B8 => {
    //   block [0x830096B8..0x830096C0)
	// 830096B8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830096BC: 82141778  lwz r16, 0x1778(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(6008 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830096C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830096C0 size=152
    let mut pc: u32 = 0x830096C0;
    'dispatch: loop {
        match pc {
            0x830096C0 => {
    //   block [0x830096C0..0x83009758)
	// 830096C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830096C4: 4819EA9D  bl 0x831a8160
	ctx.lr = 0x830096C8;
	sub_831A8130(ctx, base);
	// 830096C8: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 830096CC: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830096D0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830096D4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 830096D8: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 830096DC: 93DF00A4  stw r30, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[30].u32 ) };
	// 830096E0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830096E4: 3B5E0004  addi r26, r30, 4
	ctx.r[26].s64 = ctx.r[30].s64 + 4;
	// 830096E8: 396B16B8  addi r11, r11, 0x16b8
	ctx.r[11].s64 = ctx.r[11].s64 + 5816;
	// 830096EC: 389D0004  addi r4, r29, 4
	ctx.r[4].s64 = ctx.r[29].s64 + 4;
	// 830096F0: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 830096F4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830096F8: 4BFD6F01  bl 0x82fe05f8
	ctx.lr = 0x830096FC;
	sub_82FE05F8(ctx, base);
	// 830096FC: 3B9E000C  addi r28, r30, 0xc
	ctx.r[28].s64 = ctx.r[30].s64 + 12;
	// 83009700: 389D000C  addi r4, r29, 0xc
	ctx.r[4].s64 = ctx.r[29].s64 + 12;
	// 83009704: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83009708: 4BFF2DD9  bl 0x82ffc4e0
	ctx.lr = 0x8300970C;
	sub_82FFC4E0(ctx, base);
	// 8300970C: 389D001C  addi r4, r29, 0x1c
	ctx.r[4].s64 = ctx.r[29].s64 + 28;
	// 83009710: 387E001C  addi r3, r30, 0x1c
	ctx.r[3].s64 = ctx.r[30].s64 + 28;
	// 83009714: 4818449D  bl 0x8318dbb0
	ctx.lr = 0x83009718;
	sub_8318DBB0(ctx, base);
	// 83009718: 817D0024  lwz r11, 0x24(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 8300971C: 576A063F  clrlwi. r10, r27, 0x18
	ctx.r[10].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83009720: 917E0024  stw r11, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 83009724: 817D0028  lwz r11, 0x28(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(40 as u32) ) } as u64;
	// 83009728: 917E0028  stw r11, 0x28(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 8300972C: 41820010  beq 0x8300973c
	if ctx.cr[0].eq {
	pc = 0x8300973C; continue 'dispatch;
	}
	// 83009730: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83009734: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83009738: 4BFF3C61  bl 0x82ffd398
	ctx.lr = 0x8300973C;
	sub_82FFD398(ctx, base);
	// 8300973C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83009740: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83009744: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 83009748: 4BFD5C49  bl 0x82fdf390
	ctx.lr = 0x8300974C;
	sub_82FDF390(ctx, base);
	// 8300974C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83009750: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 83009754: 4819EA5C  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83009758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83009758 size=40
    let mut pc: u32 = 0x83009758;
    'dispatch: loop {
        match pc {
            0x83009758 => {
    //   block [0x83009758..0x83009780)
	// 83009758: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 8300975C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83009760: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83009764: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83009768: 807F00A4  lwz r3, 0xa4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 8300976C: 4BFFC415  bl 0x83005b80
	ctx.lr = 0x83009770;
	sub_83005B80(ctx, base);
	// 83009770: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83009774: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83009778: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300977C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83009780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83009780 size=44
    let mut pc: u32 = 0x83009780;
    'dispatch: loop {
        match pc {
            0x83009780 => {
    //   block [0x83009780..0x830097AC)
	// 83009780: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 83009784: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83009788: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300978C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83009790: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 83009794: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 83009798: 480BE349  bl 0x830c7ae0
	ctx.lr = 0x8300979C;
	sub_830C7AE0(ctx, base);
	// 8300979C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830097A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830097A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830097A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830097AC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830097AC size=44
    let mut pc: u32 = 0x830097AC;
    'dispatch: loop {
        match pc {
            0x830097AC => {
    //   block [0x830097AC..0x830097D8)
	// 830097AC: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 830097B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830097B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830097B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830097BC: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 830097C0: 386B000C  addi r3, r11, 0xc
	ctx.r[3].s64 = ctx.r[11].s64 + 12;
	// 830097C4: 4BFD7A65  bl 0x82fe1228
	ctx.lr = 0x830097C8;
	sub_82FE1228(ctx, base);
	// 830097C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830097CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830097D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830097D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830097D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830097D8 size=44
    let mut pc: u32 = 0x830097D8;
    'dispatch: loop {
        match pc {
            0x830097D8 => {
    //   block [0x830097D8..0x83009804)
	// 830097D8: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 830097DC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830097E0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830097E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830097E8: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 830097EC: 386B001C  addi r3, r11, 0x1c
	ctx.r[3].s64 = ctx.r[11].s64 + 28;
	// 830097F0: 480BE2F1  bl 0x830c7ae0
	ctx.lr = 0x830097F4;
	sub_830C7AE0(ctx, base);
	// 830097F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830097F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830097FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83009800: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83009808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83009808 size=8
    let mut pc: u32 = 0x83009808;
    'dispatch: loop {
        match pc {
            0x83009808 => {
    //   block [0x83009808..0x83009810)
	// 83009808: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8300980C: 821417E0  lwz r16, 0x17e0(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(6112 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83009810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83009810 size=108
    let mut pc: u32 = 0x83009810;
    'dispatch: loop {
        match pc {
            0x83009810 => {
    //   block [0x83009810..0x8300987C)
	// 83009810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83009814: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83009818: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8300981C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83009820: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 83009824: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83009828: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8300982C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83009830: 396B16B8  addi r11, r11, 0x16b8
	ctx.r[11].s64 = ctx.r[11].s64 + 5816;
	// 83009834: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 83009838: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8300983C: 387E001C  addi r3, r30, 0x1c
	ctx.r[3].s64 = ctx.r[30].s64 + 28;
	// 83009840: 480BE2A1  bl 0x830c7ae0
	ctx.lr = 0x83009844;
	sub_830C7AE0(ctx, base);
	// 83009844: 397E000C  addi r11, r30, 0xc
	ctx.r[11].s64 = ctx.r[30].s64 + 12;
	// 83009848: 386B0008  addi r3, r11, 8
	ctx.r[3].s64 = ctx.r[11].s64 + 8;
	// 8300984C: 4BFF07BD  bl 0x82ffa008
	ctx.lr = 0x83009850;
	sub_82FFA008(ctx, base);
	// 83009850: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 83009854: 480BE28D  bl 0x830c7ae0
	ctx.lr = 0x83009858;
	sub_830C7AE0(ctx, base);
	// 83009858: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8300985C: 396BA8A0  addi r11, r11, -0x5760
	ctx.r[11].s64 = ctx.r[11].s64 + -22368;
	// 83009860: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83009864: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 83009868: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300986C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83009870: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83009874: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83009878: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300987C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300987C size=40
    let mut pc: u32 = 0x8300987C;
    'dispatch: loop {
        match pc {
            0x8300987C => {
    //   block [0x8300987C..0x830098A4)
	// 8300987C: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 83009880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83009884: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83009888: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300988C: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83009890: 4BFFC2F1  bl 0x83005b80
	ctx.lr = 0x83009894;
	sub_83005B80(ctx, base);
	// 83009894: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83009898: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300989C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830098A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830098A4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830098A4 size=44
    let mut pc: u32 = 0x830098A4;
    'dispatch: loop {
        match pc {
            0x830098A4 => {
    //   block [0x830098A4..0x830098D0)
	// 830098A4: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 830098A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830098AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830098B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830098B4: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 830098B8: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 830098BC: 480BE225  bl 0x830c7ae0
	ctx.lr = 0x830098C0;
	sub_830C7AE0(ctx, base);
	// 830098C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830098C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830098C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830098CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830098D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830098D0 size=44
    let mut pc: u32 = 0x830098D0;
    'dispatch: loop {
        match pc {
            0x830098D0 => {
    //   block [0x830098D0..0x830098FC)
	// 830098D0: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 830098D4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830098D8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830098DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830098E0: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 830098E4: 386B000C  addi r3, r11, 0xc
	ctx.r[3].s64 = ctx.r[11].s64 + 12;
	// 830098E8: 4BFD7941  bl 0x82fe1228
	ctx.lr = 0x830098EC;
	sub_82FE1228(ctx, base);
	// 830098EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830098F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830098F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830098F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83009900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83009900 size=8
    let mut pc: u32 = 0x83009900;
    'dispatch: loop {
        match pc {
            0x83009900 => {
    //   block [0x83009900..0x83009908)
	// 83009900: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83009904: 82141828  lwz r16, 0x1828(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(6184 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83009908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83009908 size=124
    let mut pc: u32 = 0x83009908;
    'dispatch: loop {
        match pc {
            0x83009908 => {
    //   block [0x83009908..0x83009984)
	// 83009908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300990C: 4819E861  bl 0x831a816c
	ctx.lr = 0x83009910;
	sub_831A8130(ctx, base);
	// 83009910: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 83009914: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83009918: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8300991C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83009920: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83009924: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83009928: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300992C: 4E800421  bctrl
	ctx.lr = 0x83009930;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83009930: 38A00009  li r5, 9
	ctx.r[5].s64 = 9;
	// 83009934: 3880002C  li r4, 0x2c
	ctx.r[4].s64 = 44;
	// 83009938: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8300993C: 4BFDAF7D  bl 0x82fe48b8
	ctx.lr = 0x83009940;
	sub_82FE48B8(ctx, base);
	// 83009940: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 83009944: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83009948: 41820018  beq 0x83009960
	if ctx.cr[0].eq {
	pc = 0x83009960; continue 'dispatch;
	}
	// 8300994C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83009950: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83009954: 4BFFFD6D  bl 0x830096c0
	ctx.lr = 0x83009958;
	sub_830096C0(ctx, base);
	// 83009958: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8300995C: 48000008  b 0x83009964
	pc = 0x83009964; continue 'dispatch;
	// 83009960: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83009964: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 83009968: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8300996C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83009970: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 83009974: 4BFD5E05  bl 0x82fdf778
	ctx.lr = 0x83009978;
	sub_82FDF778(ctx, base);
	// 83009978: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8300997C: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 83009980: 4819E83C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83009984(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83009984 size=48
    let mut pc: u32 = 0x83009984;
    'dispatch: loop {
        match pc {
            0x83009984 => {
    //   block [0x83009984..0x830099B4)
	// 83009984: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83009988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300998C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83009990: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83009994: 38A00009  li r5, 9
	ctx.r[5].s64 = 9;
	// 83009998: 809F0050  lwz r4, 0x50(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8300999C: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 830099A0: 480BE141  bl 0x830c7ae0
	ctx.lr = 0x830099A4;
	sub_830C7AE0(ctx, base);
	// 830099A4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830099A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830099AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830099B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830099B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830099B8 size=8
    let mut pc: u32 = 0x830099B8;
    'dispatch: loop {
        match pc {
            0x830099B8 => {
    //   block [0x830099B8..0x830099C0)
	// 830099B8: 38600005  li r3, 5
	ctx.r[3].s64 = 5;
	// 830099BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830099C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830099C0 size=180
    let mut pc: u32 = 0x830099C0;
    'dispatch: loop {
        match pc {
            0x830099C0 => {
    //   block [0x830099C0..0x83009A74)
	// 830099C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830099C4: 4819E7A9  bl 0x831a816c
	ctx.lr = 0x830099C8;
	sub_831A8130(ctx, base);
	// 830099C8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830099CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830099D0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830099D4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 830099D8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830099DC: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830099E0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830099E4: 4E800421  bctrl
	ctx.lr = 0x830099E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830099E8: 89630098  lbz r11, 0x98(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(152 as u32) ) } as u64;
	// 830099EC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830099F0: 4182006C  beq 0x83009a5c
	if ctx.cr[0].eq {
	pc = 0x83009A5C; continue 'dispatch;
	}
	// 830099F4: 57CB063F  clrlwi. r11, r30, 0x18
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830099F8: 40820064  bne 0x83009a5c
	if !ctx.cr[0].eq {
	pc = 0x83009A5C; continue 'dispatch;
	}
	// 830099FC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83009A00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83009A04: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83009A08: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83009A0C: 4E800421  bctrl
	ctx.lr = 0x83009A10;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83009A10: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83009A14: 41820020  beq 0x83009a34
	if ctx.cr[0].eq {
	pc = 0x83009A34; continue 'dispatch;
	}
	// 83009A18: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83009A1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83009A20: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83009A24: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83009A28: 4E800421  bctrl
	ctx.lr = 0x83009A2C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83009A2C: 80C30090  lwz r6, 0x90(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 83009A30: 4800000C  b 0x83009a3c
	pc = 0x83009A3C; continue 'dispatch;
	// 83009A34: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83009A38: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83009A3C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83009A40: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 83009A44: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83009A48: 4BFF0489  bl 0x82ff9ed0
	ctx.lr = 0x83009A4C;
	sub_82FF9ED0(ctx, base);
	// 83009A4C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83009A50: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83009A54: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 83009A58: 481A71D1  bl 0x831b0c28
	ctx.lr = 0x83009A5C;
	sub_831B0C28(ctx, base);
	// 83009A5C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83009A60: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83009A64: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 83009A68: 4BFD5929  bl 0x82fdf390
	ctx.lr = 0x83009A6C;
	sub_82FDF390(ctx, base);
	// 83009A6C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83009A70: 4819E74C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83009A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83009A78 size=376
    let mut pc: u32 = 0x83009A78;
    'dispatch: loop {
        match pc {
            0x83009A78 => {
    //   block [0x83009A78..0x83009BF0)
	// 83009A78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83009A7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83009A80: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83009A84: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83009A88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83009A8C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83009A90: 3D408214  lis r10, -0x7dec
	ctx.r[10].s64 = -2112618496;
	// 83009A94: A17F0008  lhz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83009A98: A14AA6A4  lhz r10, -0x595c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(-22876 as u32) ) } as u64;
	// 83009A9C: 7D4A5838  and r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 83009AA0: 7D4A0034  cntlzw r10, r10
	ctx.r[10].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 83009AA4: 554ADFFE  rlwinm r10, r10, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 83009AA8: 694A0001  xori r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u64 ^ 1;
	// 83009AAC: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83009AB0: 41820080  beq 0x83009b30
	if ctx.cr[0].eq {
	pc = 0x83009B30; continue 'dispatch;
	}
	// 83009AB4: 3D408214  lis r10, -0x7dec
	ctx.r[10].s64 = -2112618496;
	// 83009AB8: A14AA6C8  lhz r10, -0x5938(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(-22840 as u32) ) } as u64;
	// 83009ABC: 7D4B5838  and r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 83009AC0: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83009AC4: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83009AC8: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 83009ACC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83009AD0: 40820060  bne 0x83009b30
	if !ctx.cr[0].eq {
	pc = 0x83009B30; continue 'dispatch;
	}
	// 83009AD4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83009AD8: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83009ADC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83009AE0: 4E800421  bctrl
	ctx.lr = 0x83009AE4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83009AE4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83009AE8: 41820020  beq 0x83009b08
	if ctx.cr[0].eq {
	pc = 0x83009B08; continue 'dispatch;
	}
	// 83009AEC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83009AF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83009AF4: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83009AF8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83009AFC: 4E800421  bctrl
	ctx.lr = 0x83009B00;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83009B00: 80C30090  lwz r6, 0x90(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 83009B04: 4800000C  b 0x83009b10
	pc = 0x83009B10; continue 'dispatch;
	// 83009B08: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83009B0C: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83009B10: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83009B14: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 83009B18: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83009B1C: 4BFF03B5  bl 0x82ff9ed0
	ctx.lr = 0x83009B20;
	sub_82FF9ED0(ctx, base);
	// 83009B20: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83009B24: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83009B28: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 83009B2C: 481A70FD  bl 0x831b0c28
	ctx.lr = 0x83009B30;
	sub_831B0C28(ctx, base);
	// 83009B30: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83009B34: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83009B38: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83009B3C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83009B40: 4E800421  bctrl
	ctx.lr = 0x83009B44;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83009B44: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83009B48: 41820048  beq 0x83009b90
	if ctx.cr[0].eq {
	pc = 0x83009B90; continue 'dispatch;
	}
	// 83009B4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83009B50: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83009B54: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 83009B58: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 83009B5C: 4BFD5C1D  bl 0x82fdf778
	ctx.lr = 0x83009B60;
	sub_82FDF778(ctx, base);
	// 83009B60: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 83009B64: 4BFF2B25  bl 0x82ffc688
	ctx.lr = 0x83009B68;
	sub_82FFC688(ctx, base);
	// 83009B68: 38A00009  li r5, 9
	ctx.r[5].s64 = 9;
	// 83009B6C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83009B70: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83009B74: 4BFDBB6D  bl 0x82fe56e0
	ctx.lr = 0x83009B78;
	sub_82FE56E0(ctx, base);
	// 83009B78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83009B7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83009B80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83009B84: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83009B88: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83009B8C: 4E800020  blr
	return;
	// 83009B90: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83009B94: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83009B98: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83009B9C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83009BA0: 4E800421  bctrl
	ctx.lr = 0x83009BA4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83009BA4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83009BA8: 41820020  beq 0x83009bc8
	if ctx.cr[0].eq {
	pc = 0x83009BC8; continue 'dispatch;
	}
	// 83009BAC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83009BB0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83009BB4: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83009BB8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83009BBC: 4E800421  bctrl
	ctx.lr = 0x83009BC0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83009BC0: 80C30090  lwz r6, 0x90(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 83009BC4: 4800000C  b 0x83009bd0
	pc = 0x83009BD0; continue 'dispatch;
	// 83009BC8: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83009BCC: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 83009BD0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83009BD4: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 83009BD8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83009BDC: 4BFF02F5  bl 0x82ff9ed0
	ctx.lr = 0x83009BE0;
	sub_82FF9ED0(ctx, base);
	// 83009BE0: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 83009BE4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83009BE8: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 83009BEC: 481A703D  bl 0x831b0c28
	ctx.lr = 0x83009BF0;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83009BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83009BF0 size=12
    let mut pc: u32 = 0x83009BF0;
    'dispatch: loop {
        match pc {
            0x83009BF0 => {
    //   block [0x83009BF0..0x83009BFC)
	// 83009BF0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83009BF4: 3864001C  addi r3, r4, 0x1c
	ctx.r[3].s64 = ctx.r[4].s64 + 28;
	// 83009BF8: 4802ACE0  b 0x830348d8
	sub_830348D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83009C00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83009C00 size=8
    let mut pc: u32 = 0x83009C00;
    'dispatch: loop {
        match pc {
            0x83009C00 => {
    //   block [0x83009C00..0x83009C08)
	// 83009C00: 38630004  addi r3, r3, 4
	ctx.r[3].s64 = ctx.r[3].s64 + 4;
	// 83009C04: 4BFD5AA4  b 0x82fdf6a8
	sub_82FDF6A8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83009C08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83009C08 size=8
    let mut pc: u32 = 0x83009C08;
    'dispatch: loop {
        match pc {
            0x83009C08 => {
    //   block [0x83009C08..0x83009C10)
	// 83009C08: 38630004  addi r3, r3, 4
	ctx.r[3].s64 = ctx.r[3].s64 + 4;
	// 83009C0C: 4BFD690C  b 0x82fe0518
	sub_82FE0518(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83009C10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83009C10 size=8
    let mut pc: u32 = 0x83009C10;
    'dispatch: loop {
        match pc {
            0x83009C10 => {
    //   block [0x83009C10..0x83009C18)
	// 83009C10: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83009C14: 82141888  lwz r16, 0x1888(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(6280 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83009C18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83009C18 size=356
    let mut pc: u32 = 0x83009C18;
    'dispatch: loop {
        match pc {
            0x83009C18 => {
    //   block [0x83009C18..0x83009D7C)
	// 83009C18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83009C1C: 4819E545  bl 0x831a8160
	ctx.lr = 0x83009C20;
	sub_831A8130(ctx, base);
	// 83009C20: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 83009C24: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83009C28: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83009C2C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83009C30: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 83009C34: 93BF00A4  stw r29, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[29].u32 ) };
	// 83009C38: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83009C3C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83009C40: 396B16B8  addi r11, r11, 0x16b8
	ctx.r[11].s64 = ctx.r[11].s64 + 5816;
	// 83009C44: 389E000C  addi r4, r30, 0xc
	ctx.r[4].s64 = ctx.r[30].s64 + 12;
	// 83009C48: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83009C4C: 409A0008  bne cr6, 0x83009c54
	if !ctx.cr[6].eq {
	pc = 0x83009C54; continue 'dispatch;
	}
	// 83009C50: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83009C54: 3B5D0004  addi r26, r29, 4
	ctx.r[26].s64 = ctx.r[29].s64 + 4;
	// 83009C58: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 83009C5C: 4BFD55BD  bl 0x82fdf218
	ctx.lr = 0x83009C60;
	sub_82FDF218(ctx, base);
	// 83009C60: 3B9D000C  addi r28, r29, 0xc
	ctx.r[28].s64 = ctx.r[29].s64 + 12;
	// 83009C64: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83009C68: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83009C6C: 4BFF282D  bl 0x82ffc498
	ctx.lr = 0x83009C70;
	sub_82FFC498(ctx, base);
	// 83009C70: 387D001C  addi r3, r29, 0x1c
	ctx.r[3].s64 = ctx.r[29].s64 + 28;
	// 83009C74: 48183F3D  bl 0x8318dbb0
	ctx.lr = 0x83009C78;
	sub_8318DBB0(ctx, base);
	// 83009C78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83009C7C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83009C80: 807C0000  lwz r3, 0(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 83009C84: 917D0028  stw r11, 0x28(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 83009C88: 4BFD7D51  bl 0x82fe19d8
	ctx.lr = 0x83009C8C;
	sub_82FE19D8(ctx, base);
	// 83009C8C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83009C90: 907D0024  stw r3, 0x24(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(36 as u32), ctx.r[3].u32 ) };
	// 83009C94: 419A00CC  beq cr6, 0x83009d60
	if ctx.cr[6].eq {
	pc = 0x83009D60; continue 'dispatch;
	}
	// 83009C98: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83009C9C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83009CA0: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 83009CA4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83009CA8: 4E800421  bctrl
	ctx.lr = 0x83009CAC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83009CAC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83009CB0: 418200B0  beq 0x83009d60
	if ctx.cr[0].eq {
	pc = 0x83009D60; continue 'dispatch;
	}
	// 83009CB4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83009CB8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83009CBC: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 83009CC0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83009CC4: 4E800421  bctrl
	ctx.lr = 0x83009CC8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83009CC8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83009CCC: 816B00A0  lwz r11, 0xa0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(160 as u32) ) } as u64;
	// 83009CD0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83009CD4: 4E800421  bctrl
	ctx.lr = 0x83009CD8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83009CD8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83009CDC: 41820084  beq 0x83009d60
	if ctx.cr[0].eq {
	pc = 0x83009D60; continue 'dispatch;
	}
	// 83009CE0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83009CE4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83009CE8: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 83009CEC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83009CF0: 4E800421  bctrl
	ctx.lr = 0x83009CF4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83009CF4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83009CF8: 816B00A0  lwz r11, 0xa0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(160 as u32) ) } as u64;
	// 83009CFC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83009D00: 4E800421  bctrl
	ctx.lr = 0x83009D04;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83009D04: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83009D08: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83009D0C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83009D10: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83009D14: 4E800421  bctrl
	ctx.lr = 0x83009D18;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83009D18: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83009D1C: 41820044  beq 0x83009d60
	if ctx.cr[0].eq {
	pc = 0x83009D60; continue 'dispatch;
	}
	// 83009D20: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83009D24: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83009D28: 816B0078  lwz r11, 0x78(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(120 as u32) ) } as u64;
	// 83009D2C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83009D30: 4E800421  bctrl
	ctx.lr = 0x83009D34;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83009D34: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83009D38: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83009D3C: 917D0028  stw r11, 0x28(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 83009D40: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83009D44: 816B00D0  lwz r11, 0xd0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(208 as u32) ) } as u64;
	// 83009D48: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83009D4C: 4E800421  bctrl
	ctx.lr = 0x83009D50;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83009D50: 7C641B79  or. r4, r3, r3
	ctx.r[4].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 83009D54: 4182000C  beq 0x83009d60
	if ctx.cr[0].eq {
	pc = 0x83009D60; continue 'dispatch;
	}
	// 83009D58: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83009D5C: 4BFF363D  bl 0x82ffd398
	ctx.lr = 0x83009D60;
	sub_82FFD398(ctx, base);
	// 83009D60: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83009D64: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83009D68: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 83009D6C: 4BFD5625  bl 0x82fdf390
	ctx.lr = 0x83009D70;
	sub_82FDF390(ctx, base);
	// 83009D70: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83009D74: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 83009D78: 4819E438  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83009D7C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83009D7C size=40
    let mut pc: u32 = 0x83009D7C;
    'dispatch: loop {
        match pc {
            0x83009D7C => {
    //   block [0x83009D7C..0x83009DA4)
	// 83009D7C: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 83009D80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83009D84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83009D88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83009D8C: 807F00A4  lwz r3, 0xa4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 83009D90: 4BFFBDF1  bl 0x83005b80
	ctx.lr = 0x83009D94;
	sub_83005B80(ctx, base);
	// 83009D94: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83009D98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83009D9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83009DA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83009DA4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83009DA4 size=44
    let mut pc: u32 = 0x83009DA4;
    'dispatch: loop {
        match pc {
            0x83009DA4 => {
    //   block [0x83009DA4..0x83009DD0)
	// 83009DA4: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 83009DA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83009DAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83009DB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83009DB4: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 83009DB8: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 83009DBC: 480BDD25  bl 0x830c7ae0
	ctx.lr = 0x83009DC0;
	sub_830C7AE0(ctx, base);
	// 83009DC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83009DC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83009DC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83009DCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83009DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83009DD0 size=44
    let mut pc: u32 = 0x83009DD0;
    'dispatch: loop {
        match pc {
            0x83009DD0 => {
    //   block [0x83009DD0..0x83009DFC)
	// 83009DD0: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 83009DD4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83009DD8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83009DDC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83009DE0: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 83009DE4: 386B000C  addi r3, r11, 0xc
	ctx.r[3].s64 = ctx.r[11].s64 + 12;
	// 83009DE8: 4BFD7441  bl 0x82fe1228
	ctx.lr = 0x83009DEC;
	sub_82FE1228(ctx, base);
	// 83009DEC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83009DF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83009DF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83009DF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83009DFC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83009DFC size=44
    let mut pc: u32 = 0x83009DFC;
    'dispatch: loop {
        match pc {
            0x83009DFC => {
    //   block [0x83009DFC..0x83009E28)
	// 83009DFC: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 83009E00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83009E04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83009E08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83009E0C: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 83009E10: 386B001C  addi r3, r11, 0x1c
	ctx.r[3].s64 = ctx.r[11].s64 + 28;
	// 83009E14: 480BDCCD  bl 0x830c7ae0
	ctx.lr = 0x83009E18;
	sub_830C7AE0(ctx, base);
	// 83009E18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83009E1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83009E20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83009E24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83009E28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83009E28 size=76
    let mut pc: u32 = 0x83009E28;
    'dispatch: loop {
        match pc {
            0x83009E28 => {
    //   block [0x83009E28..0x83009E74)
	// 83009E28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83009E2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83009E30: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83009E34: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83009E38: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83009E3C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83009E40: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83009E44: 4BFFF9CD  bl 0x83009810
	ctx.lr = 0x83009E48;
	sub_83009810(ctx, base);
	// 83009E48: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83009E4C: 4182000C  beq 0x83009e58
	if ctx.cr[0].eq {
	pc = 0x83009E58; continue 'dispatch;
	}
	// 83009E50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83009E54: 4B2B6415  bl 0x822c0268
	ctx.lr = 0x83009E58;
	sub_822C0268(ctx, base);
	// 83009E58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83009E5C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83009E60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83009E64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83009E68: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83009E6C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83009E70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83009E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83009E78 size=8
    let mut pc: u32 = 0x83009E78;
    'dispatch: loop {
        match pc {
            0x83009E78 => {
    //   block [0x83009E78..0x83009E80)
	// 83009E78: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83009E7C: 821418F8  lwz r16, 0x18f8(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(6392 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83009E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83009E80 size=364
    let mut pc: u32 = 0x83009E80;
    'dispatch: loop {
        match pc {
            0x83009E80 => {
    //   block [0x83009E80..0x83009FEC)
	// 83009E80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83009E84: 4819E2D9  bl 0x831a815c
	ctx.lr = 0x83009E88;
	sub_831A8130(ctx, base);
	// 83009E88: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 83009E8C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83009E90: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83009E94: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83009E98: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 83009E9C: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 83009EA0: 93BF00A4  stw r29, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[29].u32 ) };
	// 83009EA4: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83009EA8: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83009EAC: 396B16B8  addi r11, r11, 0x16b8
	ctx.r[11].s64 = ctx.r[11].s64 + 5816;
	// 83009EB0: 389E000C  addi r4, r30, 0xc
	ctx.r[4].s64 = ctx.r[30].s64 + 12;
	// 83009EB4: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83009EB8: 409A0008  bne cr6, 0x83009ec0
	if !ctx.cr[6].eq {
	pc = 0x83009EC0; continue 'dispatch;
	}
	// 83009EBC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83009EC0: 3B5D0004  addi r26, r29, 4
	ctx.r[26].s64 = ctx.r[29].s64 + 4;
	// 83009EC4: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 83009EC8: 4BFD5351  bl 0x82fdf218
	ctx.lr = 0x83009ECC;
	sub_82FDF218(ctx, base);
	// 83009ECC: 3B9D000C  addi r28, r29, 0xc
	ctx.r[28].s64 = ctx.r[29].s64 + 12;
	// 83009ED0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83009ED4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83009ED8: 4BFF25C1  bl 0x82ffc498
	ctx.lr = 0x83009EDC;
	sub_82FFC498(ctx, base);
	// 83009EDC: 387D001C  addi r3, r29, 0x1c
	ctx.r[3].s64 = ctx.r[29].s64 + 28;
	// 83009EE0: 48183CD1  bl 0x8318dbb0
	ctx.lr = 0x83009EE4;
	sub_8318DBB0(ctx, base);
	// 83009EE4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83009EE8: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83009EEC: 807C0000  lwz r3, 0(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 83009EF0: 917D0028  stw r11, 0x28(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 83009EF4: 4BFD7AE5  bl 0x82fe19d8
	ctx.lr = 0x83009EF8;
	sub_82FE19D8(ctx, base);
	// 83009EF8: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83009EFC: 907D0024  stw r3, 0x24(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(36 as u32), ctx.r[3].u32 ) };
	// 83009F00: 419A00D0  beq cr6, 0x83009fd0
	if ctx.cr[6].eq {
	pc = 0x83009FD0; continue 'dispatch;
	}
	// 83009F04: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83009F08: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83009F0C: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 83009F10: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83009F14: 4E800421  bctrl
	ctx.lr = 0x83009F18;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83009F18: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83009F1C: 418200B4  beq 0x83009fd0
	if ctx.cr[0].eq {
	pc = 0x83009FD0; continue 'dispatch;
	}
	// 83009F20: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83009F24: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83009F28: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 83009F2C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83009F30: 4E800421  bctrl
	ctx.lr = 0x83009F34;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83009F34: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83009F38: 816B00A0  lwz r11, 0xa0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(160 as u32) ) } as u64;
	// 83009F3C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83009F40: 4E800421  bctrl
	ctx.lr = 0x83009F44;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83009F44: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83009F48: 41820088  beq 0x83009fd0
	if ctx.cr[0].eq {
	pc = 0x83009FD0; continue 'dispatch;
	}
	// 83009F4C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83009F50: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83009F54: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 83009F58: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83009F5C: 4E800421  bctrl
	ctx.lr = 0x83009F60;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83009F60: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83009F64: 816B00A0  lwz r11, 0xa0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(160 as u32) ) } as u64;
	// 83009F68: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83009F6C: 4E800421  bctrl
	ctx.lr = 0x83009F70;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83009F70: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83009F74: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83009F78: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83009F7C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83009F80: 4E800421  bctrl
	ctx.lr = 0x83009F84;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83009F84: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83009F88: 41820048  beq 0x83009fd0
	if ctx.cr[0].eq {
	pc = 0x83009FD0; continue 'dispatch;
	}
	// 83009F8C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83009F90: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83009F94: 816B0078  lwz r11, 0x78(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(120 as u32) ) } as u64;
	// 83009F98: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83009F9C: 4E800421  bctrl
	ctx.lr = 0x83009FA0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83009FA0: 907D0028  stw r3, 0x28(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(40 as u32), ctx.r[3].u32 ) };
	// 83009FA4: 572B063F  clrlwi. r11, r25, 0x18
	ctx.r[11].u64 = ctx.r[25].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83009FA8: 41820028  beq 0x83009fd0
	if ctx.cr[0].eq {
	pc = 0x83009FD0; continue 'dispatch;
	}
	// 83009FAC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83009FB0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83009FB4: 816B00D0  lwz r11, 0xd0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(208 as u32) ) } as u64;
	// 83009FB8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83009FBC: 4E800421  bctrl
	ctx.lr = 0x83009FC0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83009FC0: 7C641B79  or. r4, r3, r3
	ctx.r[4].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 83009FC4: 4182000C  beq 0x83009fd0
	if ctx.cr[0].eq {
	pc = 0x83009FD0; continue 'dispatch;
	}
	// 83009FC8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83009FCC: 4BFF33CD  bl 0x82ffd398
	ctx.lr = 0x83009FD0;
	sub_82FFD398(ctx, base);
	// 83009FD0: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83009FD4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83009FD8: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 83009FDC: 4BFD53B5  bl 0x82fdf390
	ctx.lr = 0x83009FE0;
	sub_82FDF390(ctx, base);
	// 83009FE0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83009FE4: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 83009FE8: 4819E1C4  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83009FEC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83009FEC size=40
    let mut pc: u32 = 0x83009FEC;
    'dispatch: loop {
        match pc {
            0x83009FEC => {
    //   block [0x83009FEC..0x8300A014)
	// 83009FEC: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 83009FF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83009FF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83009FF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83009FFC: 807F00A4  lwz r3, 0xa4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 8300A000: 4BFFBB81  bl 0x83005b80
	ctx.lr = 0x8300A004;
	sub_83005B80(ctx, base);
	// 8300A004: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300A008: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300A00C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300A010: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300A014(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300A014 size=44
    let mut pc: u32 = 0x8300A014;
    'dispatch: loop {
        match pc {
            0x8300A014 => {
    //   block [0x8300A014..0x8300A040)
	// 8300A014: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 8300A018: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300A01C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300A020: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300A024: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 8300A028: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 8300A02C: 480BDAB5  bl 0x830c7ae0
	ctx.lr = 0x8300A030;
	sub_830C7AE0(ctx, base);
	// 8300A030: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300A034: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300A038: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300A03C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300A040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300A040 size=44
    let mut pc: u32 = 0x8300A040;
    'dispatch: loop {
        match pc {
            0x8300A040 => {
    //   block [0x8300A040..0x8300A06C)
	// 8300A040: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 8300A044: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300A048: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300A04C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300A050: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 8300A054: 386B000C  addi r3, r11, 0xc
	ctx.r[3].s64 = ctx.r[11].s64 + 12;
	// 8300A058: 4BFD71D1  bl 0x82fe1228
	ctx.lr = 0x8300A05C;
	sub_82FE1228(ctx, base);
	// 8300A05C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300A060: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300A064: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300A068: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300A06C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300A06C size=44
    let mut pc: u32 = 0x8300A06C;
    'dispatch: loop {
        match pc {
            0x8300A06C => {
    //   block [0x8300A06C..0x8300A098)
	// 8300A06C: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 8300A070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300A074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300A078: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300A07C: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 8300A080: 386B001C  addi r3, r11, 0x1c
	ctx.r[3].s64 = ctx.r[11].s64 + 28;
	// 8300A084: 480BDA5D  bl 0x830c7ae0
	ctx.lr = 0x8300A088;
	sub_830C7AE0(ctx, base);
	// 8300A088: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300A08C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300A090: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300A094: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300A098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300A098 size=8
    let mut pc: u32 = 0x8300A098;
    'dispatch: loop {
        match pc {
            0x8300A098 => {
    //   block [0x8300A098..0x8300A0A0)
	// 8300A098: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8300A09C: 82141A08  lwz r16, 0x1a08(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(6664 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300A0A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300A0A0 size=140
    let mut pc: u32 = 0x8300A0A0;
    'dispatch: loop {
        match pc {
            0x8300A0A0 => {
    //   block [0x8300A0A0..0x8300A12C)
	// 8300A0A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300A0A4: 4819E0BD  bl 0x831a8160
	ctx.lr = 0x8300A0A8;
	sub_831A8130(ctx, base);
	// 8300A0A8: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 8300A0AC: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300A0B0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8300A0B4: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 8300A0B8: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 8300A0BC: 93DF00A4  stw r30, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[30].u32 ) };
	// 8300A0C0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8300A0C4: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 8300A0C8: 396B1948  addi r11, r11, 0x1948
	ctx.r[11].s64 = ctx.r[11].s64 + 6472;
	// 8300A0CC: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8300A0D0: 389B000C  addi r4, r27, 0xc
	ctx.r[4].s64 = ctx.r[27].s64 + 12;
	// 8300A0D4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8300A0D8: 409A0008  bne cr6, 0x8300a0e0
	if !ctx.cr[6].eq {
	pc = 0x8300A0E0; continue 'dispatch;
	}
	// 8300A0DC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8300A0E0: 3BBE0004  addi r29, r30, 4
	ctx.r[29].s64 = ctx.r[30].s64 + 4;
	// 8300A0E4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8300A0E8: 4BFD5131  bl 0x82fdf218
	ctx.lr = 0x8300A0EC;
	sub_82FDF218(ctx, base);
	// 8300A0EC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8300A0F0: 939E0010  stw r28, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[28].u32 ) };
	// 8300A0F4: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8300A0F8: 939E0014  stw r28, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[28].u32 ) };
	// 8300A0FC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8300A100: 939E0018  stw r28, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[28].u32 ) };
	// 8300A104: 939E000C  stw r28, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[28].u32 ) };
	// 8300A108: A16BA6C0  lhz r11, -0x5940(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(-22848 as u32) ) } as u64;
	// 8300A10C: A15D0004  lhz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 8300A110: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 8300A114: B17D0004  sth r11, 4(r29)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 8300A118: 4BFD78C1  bl 0x82fe19d8
	ctx.lr = 0x8300A11C;
	sub_82FE19D8(ctx, base);
	// 8300A11C: 907E000C  stw r3, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 8300A120: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8300A124: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 8300A128: 4819E088  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300A12C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300A12C size=40
    let mut pc: u32 = 0x8300A12C;
    'dispatch: loop {
        match pc {
            0x8300A12C => {
    //   block [0x8300A12C..0x8300A154)
	// 8300A12C: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 8300A130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300A134: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300A138: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300A13C: 807F00A4  lwz r3, 0xa4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 8300A140: 4BFFBA41  bl 0x83005b80
	ctx.lr = 0x8300A144;
	sub_83005B80(ctx, base);
	// 8300A144: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300A148: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300A14C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300A150: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300A154(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300A154 size=44
    let mut pc: u32 = 0x8300A154;
    'dispatch: loop {
        match pc {
            0x8300A154 => {
    //   block [0x8300A154..0x8300A180)
	// 8300A154: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 8300A158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300A15C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300A160: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300A164: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 8300A168: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 8300A16C: 480BD975  bl 0x830c7ae0
	ctx.lr = 0x8300A170;
	sub_830C7AE0(ctx, base);
	// 8300A170: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300A174: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300A178: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300A17C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300A180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300A180 size=8
    let mut pc: u32 = 0x8300A180;
    'dispatch: loop {
        match pc {
            0x8300A180 => {
    //   block [0x8300A180..0x8300A188)
	// 8300A180: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8300A184: 82141A50  lwz r16, 0x1a50(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(6736 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300A188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300A188 size=120
    let mut pc: u32 = 0x8300A188;
    'dispatch: loop {
        match pc {
            0x8300A188 => {
    //   block [0x8300A188..0x8300A200)
	// 8300A188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300A18C: 4819DFDD  bl 0x831a8168
	ctx.lr = 0x8300A190;
	sub_831A8130(ctx, base);
	// 8300A190: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 8300A194: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300A198: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8300A19C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8300A1A0: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 8300A1A4: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8300A1A8: 3B9E0004  addi r28, r30, 4
	ctx.r[28].s64 = ctx.r[30].s64 + 4;
	// 8300A1AC: 396B1948  addi r11, r11, 0x1948
	ctx.r[11].s64 = ctx.r[11].s64 + 6472;
	// 8300A1B0: 389D0004  addi r4, r29, 4
	ctx.r[4].s64 = ctx.r[29].s64 + 4;
	// 8300A1B4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8300A1B8: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8300A1BC: 4BFD643D  bl 0x82fe05f8
	ctx.lr = 0x8300A1C0;
	sub_82FE05F8(ctx, base);
	// 8300A1C0: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 8300A1C4: 917E000C  stw r11, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8300A1C8: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 8300A1CC: 917E0010  stw r11, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8300A1D0: 817D0014  lwz r11, 0x14(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) } as u64;
	// 8300A1D4: 917E0014  stw r11, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8300A1D8: 817D0018  lwz r11, 0x18(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 8300A1DC: 917E0018  stw r11, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 8300A1E0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8300A1E4: A15C0004  lhz r10, 4(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 8300A1E8: A16BA6C0  lhz r11, -0x5940(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(-22848 as u32) ) } as u64;
	// 8300A1EC: 7D4B5B78  or r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 8300A1F0: B17C0004  sth r11, 4(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 8300A1F4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8300A1F8: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 8300A1FC: 4819DFBC  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300A200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300A200 size=40
    let mut pc: u32 = 0x8300A200;
    'dispatch: loop {
        match pc {
            0x8300A200 => {
    //   block [0x8300A200..0x8300A228)
	// 8300A200: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 8300A204: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300A208: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300A20C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300A210: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 8300A214: 4BFFB96D  bl 0x83005b80
	ctx.lr = 0x8300A218;
	sub_83005B80(ctx, base);
	// 8300A218: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300A21C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300A220: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300A224: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300A228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300A228 size=8
    let mut pc: u32 = 0x8300A228;
    'dispatch: loop {
        match pc {
            0x8300A228 => {
    //   block [0x8300A228..0x8300A230)
	// 8300A228: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8300A22C: 82141A88  lwz r16, 0x1a88(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(6792 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300A230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300A230 size=88
    let mut pc: u32 = 0x8300A230;
    'dispatch: loop {
        match pc {
            0x8300A230 => {
    //   block [0x8300A230..0x8300A288)
	// 8300A230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300A234: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300A238: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8300A23C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8300A240: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 8300A244: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300A248: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8300A24C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8300A250: 396B1948  addi r11, r11, 0x1948
	ctx.r[11].s64 = ctx.r[11].s64 + 6472;
	// 8300A254: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 8300A258: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8300A25C: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 8300A260: 480BD881  bl 0x830c7ae0
	ctx.lr = 0x8300A264;
	sub_830C7AE0(ctx, base);
	// 8300A264: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8300A268: 396BA8A0  addi r11, r11, -0x5760
	ctx.r[11].s64 = ctx.r[11].s64 + -22368;
	// 8300A26C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8300A270: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 8300A274: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300A278: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300A27C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8300A280: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8300A284: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300A288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300A288 size=40
    let mut pc: u32 = 0x8300A288;
    'dispatch: loop {
        match pc {
            0x8300A288 => {
    //   block [0x8300A288..0x8300A2B0)
	// 8300A288: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 8300A28C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300A290: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300A294: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300A298: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 8300A29C: 4BFFB8E5  bl 0x83005b80
	ctx.lr = 0x8300A2A0;
	sub_83005B80(ctx, base);
	// 8300A2A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300A2A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300A2A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300A2AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300A2B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300A2B0 size=8
    let mut pc: u32 = 0x8300A2B0;
    'dispatch: loop {
        match pc {
            0x8300A2B0 => {
    //   block [0x8300A2B0..0x8300A2B8)
	// 8300A2B0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8300A2B4: 82141AC0  lwz r16, 0x1ac0(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(6848 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300A2B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300A2B8 size=124
    let mut pc: u32 = 0x8300A2B8;
    'dispatch: loop {
        match pc {
            0x8300A2B8 => {
    //   block [0x8300A2B8..0x8300A334)
	// 8300A2B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300A2BC: 4819DEB1  bl 0x831a816c
	ctx.lr = 0x8300A2C0;
	sub_831A8130(ctx, base);
	// 8300A2C0: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 8300A2C4: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300A2C8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8300A2CC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8300A2D0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300A2D4: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300A2D8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300A2DC: 4E800421  bctrl
	ctx.lr = 0x8300A2E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300A2E0: 38A0000A  li r5, 0xa
	ctx.r[5].s64 = 10;
	// 8300A2E4: 3880001C  li r4, 0x1c
	ctx.r[4].s64 = 28;
	// 8300A2E8: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8300A2EC: 4BFDA5CD  bl 0x82fe48b8
	ctx.lr = 0x8300A2F0;
	sub_82FE48B8(ctx, base);
	// 8300A2F0: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 8300A2F4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300A2F8: 41820018  beq 0x8300a310
	if ctx.cr[0].eq {
	pc = 0x8300A310; continue 'dispatch;
	}
	// 8300A2FC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8300A300: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8300A304: 4BFFFE85  bl 0x8300a188
	ctx.lr = 0x8300A308;
	sub_8300A188(ctx, base);
	// 8300A308: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8300A30C: 48000008  b 0x8300a314
	pc = 0x8300A314; continue 'dispatch;
	// 8300A310: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8300A314: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 8300A318: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8300A31C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8300A320: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 8300A324: 4BFD5455  bl 0x82fdf778
	ctx.lr = 0x8300A328;
	sub_82FDF778(ctx, base);
	// 8300A328: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8300A32C: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 8300A330: 4819DE8C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300A334(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300A334 size=48
    let mut pc: u32 = 0x8300A334;
    'dispatch: loop {
        match pc {
            0x8300A334 => {
    //   block [0x8300A334..0x8300A364)
	// 8300A334: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 8300A338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300A33C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300A340: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300A344: 38A0000A  li r5, 0xa
	ctx.r[5].s64 = 10;
	// 8300A348: 809F0050  lwz r4, 0x50(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8300A34C: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8300A350: 480BD791  bl 0x830c7ae0
	ctx.lr = 0x8300A354;
	sub_830C7AE0(ctx, base);
	// 8300A354: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300A358: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300A35C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300A360: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300A368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300A368 size=8
    let mut pc: u32 = 0x8300A368;
    'dispatch: loop {
        match pc {
            0x8300A368 => {
    //   block [0x8300A368..0x8300A370)
	// 8300A368: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 8300A36C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300A370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300A370 size=196
    let mut pc: u32 = 0x8300A370;
    'dispatch: loop {
        match pc {
            0x8300A370 => {
    //   block [0x8300A370..0x8300A434)
	// 8300A370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300A374: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300A378: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8300A37C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8300A380: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300A384: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8300A388: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8300A38C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8300A390: A15F0008  lhz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300A394: A16BA698  lhz r11, -0x5968(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(-22888 as u32) ) } as u64;
	// 8300A398: 7D4B5838  and r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 8300A39C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8300A3A0: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8300A3A4: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 8300A3A8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300A3AC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300A3B0: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300A3B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300A3B8: 41820054  beq 0x8300a40c
	if ctx.cr[0].eq {
	pc = 0x8300A40C; continue 'dispatch;
	}
	// 8300A3BC: 4E800421  bctrl
	ctx.lr = 0x8300A3C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300A3C0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300A3C4: 41820020  beq 0x8300a3e4
	if ctx.cr[0].eq {
	pc = 0x8300A3E4; continue 'dispatch;
	}
	// 8300A3C8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300A3CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300A3D0: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300A3D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300A3D8: 4E800421  bctrl
	ctx.lr = 0x8300A3DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300A3DC: 80C30090  lwz r6, 0x90(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 8300A3E0: 4800000C  b 0x8300a3ec
	pc = 0x8300A3EC; continue 'dispatch;
	// 8300A3E4: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8300A3E8: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 8300A3EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8300A3F0: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 8300A3F4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8300A3F8: 4BFEFAD9  bl 0x82ff9ed0
	ctx.lr = 0x8300A3FC;
	sub_82FF9ED0(ctx, base);
	// 8300A3FC: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8300A400: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8300A404: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 8300A408: 481A6821  bl 0x831b0c28
	ctx.lr = 0x8300A40C;
	sub_831B0C28(ctx, base);
	// 8300A40C: 4E800421  bctrl
	ctx.lr = 0x8300A410;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300A410: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8300A414: 4BFD87ED  bl 0x82fe2c00
	ctx.lr = 0x8300A418;
	sub_82FE2C00(ctx, base);
	// 8300A418: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 8300A41C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8300A420: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300A424: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300A428: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8300A42C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8300A430: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300A438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300A438 size=196
    let mut pc: u32 = 0x8300A438;
    'dispatch: loop {
        match pc {
            0x8300A438 => {
    //   block [0x8300A438..0x8300A4FC)
	// 8300A438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300A43C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300A440: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8300A444: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8300A448: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300A44C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8300A450: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8300A454: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8300A458: A15F0008  lhz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300A45C: A16BA698  lhz r11, -0x5968(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(-22888 as u32) ) } as u64;
	// 8300A460: 7D4B5838  and r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 8300A464: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8300A468: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8300A46C: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 8300A470: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300A474: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300A478: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300A47C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300A480: 41820054  beq 0x8300a4d4
	if ctx.cr[0].eq {
	pc = 0x8300A4D4; continue 'dispatch;
	}
	// 8300A484: 4E800421  bctrl
	ctx.lr = 0x8300A488;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300A488: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300A48C: 41820020  beq 0x8300a4ac
	if ctx.cr[0].eq {
	pc = 0x8300A4AC; continue 'dispatch;
	}
	// 8300A490: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300A494: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300A498: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300A49C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300A4A0: 4E800421  bctrl
	ctx.lr = 0x8300A4A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300A4A4: 80C30090  lwz r6, 0x90(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 8300A4A8: 4800000C  b 0x8300a4b4
	pc = 0x8300A4B4; continue 'dispatch;
	// 8300A4AC: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8300A4B0: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 8300A4B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8300A4B8: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 8300A4BC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8300A4C0: 4BFEFA11  bl 0x82ff9ed0
	ctx.lr = 0x8300A4C4;
	sub_82FF9ED0(ctx, base);
	// 8300A4C4: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8300A4C8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8300A4CC: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 8300A4D0: 481A6759  bl 0x831b0c28
	ctx.lr = 0x8300A4D4;
	sub_831B0C28(ctx, base);
	// 8300A4D4: 4E800421  bctrl
	ctx.lr = 0x8300A4D8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300A4D8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8300A4DC: 4BFD8725  bl 0x82fe2c00
	ctx.lr = 0x8300A4E0;
	sub_82FE2C00(ctx, base);
	// 8300A4E0: 907F0014  stw r3, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 8300A4E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8300A4E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300A4EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300A4F0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8300A4F4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8300A4F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300A500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300A500 size=368
    let mut pc: u32 = 0x8300A500;
    'dispatch: loop {
        match pc {
            0x8300A500 => {
    //   block [0x8300A500..0x8300A670)
	// 8300A500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300A504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300A508: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8300A50C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8300A510: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300A514: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8300A518: 3D408214  lis r10, -0x7dec
	ctx.r[10].s64 = -2112618496;
	// 8300A51C: A17F0008  lhz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300A520: A14AA6A4  lhz r10, -0x595c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(-22876 as u32) ) } as u64;
	// 8300A524: 7D4A5838  and r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 8300A528: 7D4A0034  cntlzw r10, r10
	ctx.r[10].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 8300A52C: 554ADFFE  rlwinm r10, r10, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 8300A530: 694A0001  xori r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u64 ^ 1;
	// 8300A534: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300A538: 41820080  beq 0x8300a5b8
	if ctx.cr[0].eq {
	pc = 0x8300A5B8; continue 'dispatch;
	}
	// 8300A53C: 3D408214  lis r10, -0x7dec
	ctx.r[10].s64 = -2112618496;
	// 8300A540: A14AA6C8  lhz r10, -0x5938(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(-22840 as u32) ) } as u64;
	// 8300A544: 7D4B5838  and r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 8300A548: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8300A54C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8300A550: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 8300A554: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300A558: 40820060  bne 0x8300a5b8
	if !ctx.cr[0].eq {
	pc = 0x8300A5B8; continue 'dispatch;
	}
	// 8300A55C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300A560: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300A564: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300A568: 4E800421  bctrl
	ctx.lr = 0x8300A56C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300A56C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300A570: 41820020  beq 0x8300a590
	if ctx.cr[0].eq {
	pc = 0x8300A590; continue 'dispatch;
	}
	// 8300A574: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300A578: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300A57C: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300A580: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300A584: 4E800421  bctrl
	ctx.lr = 0x8300A588;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300A588: 80C30090  lwz r6, 0x90(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 8300A58C: 4800000C  b 0x8300a598
	pc = 0x8300A598; continue 'dispatch;
	// 8300A590: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8300A594: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 8300A598: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8300A59C: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 8300A5A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8300A5A4: 4BFEF92D  bl 0x82ff9ed0
	ctx.lr = 0x8300A5A8;
	sub_82FF9ED0(ctx, base);
	// 8300A5A8: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8300A5AC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8300A5B0: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 8300A5B4: 481A6675  bl 0x831b0c28
	ctx.lr = 0x8300A5B8;
	sub_831B0C28(ctx, base);
	// 8300A5B8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300A5BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300A5C0: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300A5C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300A5C8: 4E800421  bctrl
	ctx.lr = 0x8300A5CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300A5CC: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8300A5D0: 41820040  beq 0x8300a610
	if ctx.cr[0].eq {
	pc = 0x8300A610; continue 'dispatch;
	}
	// 8300A5D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8300A5D8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8300A5DC: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 8300A5E0: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 8300A5E4: 4BFD5195  bl 0x82fdf778
	ctx.lr = 0x8300A5E8;
	sub_82FDF778(ctx, base);
	// 8300A5E8: 38A0000A  li r5, 0xa
	ctx.r[5].s64 = 10;
	// 8300A5EC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8300A5F0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8300A5F4: 4BFDB0ED  bl 0x82fe56e0
	ctx.lr = 0x8300A5F8;
	sub_82FE56E0(ctx, base);
	// 8300A5F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8300A5FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300A600: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300A604: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8300A608: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8300A60C: 4E800020  blr
	return;
	// 8300A610: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300A614: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300A618: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300A61C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300A620: 4E800421  bctrl
	ctx.lr = 0x8300A624;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300A624: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300A628: 41820020  beq 0x8300a648
	if ctx.cr[0].eq {
	pc = 0x8300A648; continue 'dispatch;
	}
	// 8300A62C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300A630: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300A634: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300A638: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300A63C: 4E800421  bctrl
	ctx.lr = 0x8300A640;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300A640: 80C30090  lwz r6, 0x90(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 8300A644: 4800000C  b 0x8300a650
	pc = 0x8300A650; continue 'dispatch;
	// 8300A648: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8300A64C: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 8300A650: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8300A654: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 8300A658: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8300A65C: 4BFEF875  bl 0x82ff9ed0
	ctx.lr = 0x8300A660;
	sub_82FF9ED0(ctx, base);
	// 8300A660: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8300A664: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8300A668: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 8300A66C: 481A65BD  bl 0x831b0c28
	ctx.lr = 0x8300A670;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300A670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300A670 size=144
    let mut pc: u32 = 0x8300A670;
    'dispatch: loop {
        match pc {
            0x8300A670 => {
    //   block [0x8300A670..0x8300A700)
	// 8300A670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300A674: 4819DAF9  bl 0x831a816c
	ctx.lr = 0x8300A678;
	sub_831A8130(ctx, base);
	// 8300A678: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300A67C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8300A680: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8300A684: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8300A688: 419A0068  beq cr6, 0x8300a6f0
	if ctx.cr[6].eq {
	pc = 0x8300A6F0; continue 'dispatch;
	}
	// 8300A68C: A17F0000  lhz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300A690: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300A694: 4182005C  beq 0x8300a6f0
	if ctx.cr[0].eq {
	pc = 0x8300A6F0; continue 'dispatch;
	}
	// 8300A698: 397F0002  addi r11, r31, 2
	ctx.r[11].s64 = ctx.r[31].s64 + 2;
	// 8300A69C: 48000008  b 0x8300a6a4
	pc = 0x8300A6A4; continue 'dispatch;
	// 8300A6A0: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 8300A6A4: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300A6A8: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300A6AC: 4082FFF4  bne 0x8300a6a0
	if !ctx.cr[0].eq {
	pc = 0x8300A6A0; continue 'dispatch;
	}
	// 8300A6B0: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300A6B4: 7D7F5850  subf r11, r31, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[31].s64;
	// 8300A6B8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8300A6BC: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 8300A6C0: 3BCB0009  addi r30, r11, 9
	ctx.r[30].s64 = ctx.r[11].s64 + 9;
	// 8300A6C4: 814A002C  lwz r10, 0x2c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300A6C8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8300A6CC: 4E800421  bctrl
	ctx.lr = 0x8300A6D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300A6D0: 57C4083C  slwi r4, r30, 1
	ctx.r[4].u32 = ctx.r[30].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 8300A6D4: 4BFD731D  bl 0x82fe19f0
	ctx.lr = 0x8300A6D8;
	sub_82FE19F0(ctx, base);
	// 8300A6D8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8300A6DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300A6E0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8300A6E4: 4BFC7E25  bl 0x82fd2508
	ctx.lr = 0x8300A6E8;
	sub_82FD2508(ctx, base);
	// 8300A6E8: 93DD0018  stw r30, 0x18(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 8300A6EC: 4800000C  b 0x8300a6f8
	pc = 0x8300A6F8; continue 'dispatch;
	// 8300A6F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8300A6F4: 917D0018  stw r11, 0x18(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 8300A6F8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8300A6FC: 4819DAC0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300A700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300A700 size=8
    let mut pc: u32 = 0x8300A700;
    'dispatch: loop {
        match pc {
            0x8300A700 => {
    //   block [0x8300A700..0x8300A708)
	// 8300A700: 38630004  addi r3, r3, 4
	ctx.r[3].s64 = ctx.r[3].s64 + 4;
	// 8300A704: 4BFD4C2C  b 0x82fdf330
	sub_82FDF330(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300A708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300A708 size=8
    let mut pc: u32 = 0x8300A708;
    'dispatch: loop {
        match pc {
            0x8300A708 => {
    //   block [0x8300A708..0x8300A710)
	// 8300A708: 38630004  addi r3, r3, 4
	ctx.r[3].s64 = ctx.r[3].s64 + 4;
	// 8300A70C: 4BFD5C94  b 0x82fe03a0
	sub_82FE03A0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300A710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300A710 size=76
    let mut pc: u32 = 0x8300A710;
    'dispatch: loop {
        match pc {
            0x8300A710 => {
    //   block [0x8300A710..0x8300A75C)
	// 8300A710: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300A714: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300A718: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8300A71C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8300A720: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300A724: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8300A728: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8300A72C: 4BFFFB05  bl 0x8300a230
	ctx.lr = 0x8300A730;
	sub_8300A230(ctx, base);
	// 8300A730: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8300A734: 4182000C  beq 0x8300a740
	if ctx.cr[0].eq {
	pc = 0x8300A740; continue 'dispatch;
	}
	// 8300A738: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300A73C: 4B2B5B2D  bl 0x822c0268
	ctx.lr = 0x8300A740;
	sub_822C0268(ctx, base);
	// 8300A740: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300A744: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8300A748: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300A74C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300A750: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8300A754: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8300A758: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300A760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300A760 size=8
    let mut pc: u32 = 0x8300A760;
    'dispatch: loop {
        match pc {
            0x8300A760 => {
    //   block [0x8300A760..0x8300A768)
	// 8300A760: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8300A764: 82141BD0  lwz r16, 0x1bd0(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(7120 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300A768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300A768 size=156
    let mut pc: u32 = 0x8300A768;
    'dispatch: loop {
        match pc {
            0x8300A768 => {
    //   block [0x8300A768..0x8300A804)
	// 8300A768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300A76C: 4819D9F5  bl 0x831a8160
	ctx.lr = 0x8300A770;
	sub_831A8130(ctx, base);
	// 8300A770: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 8300A774: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300A778: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8300A77C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8300A780: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 8300A784: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 8300A788: 93DF00A4  stw r30, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[30].u32 ) };
	// 8300A78C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8300A790: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 8300A794: 396B1B00  addi r11, r11, 0x1b00
	ctx.r[11].s64 = ctx.r[11].s64 + 6912;
	// 8300A798: 389C000C  addi r4, r28, 0xc
	ctx.r[4].s64 = ctx.r[28].s64 + 12;
	// 8300A79C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8300A7A0: 409A0008  bne cr6, 0x8300a7a8
	if !ctx.cr[6].eq {
	pc = 0x8300A7A8; continue 'dispatch;
	}
	// 8300A7A4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8300A7A8: 3BBE0004  addi r29, r30, 4
	ctx.r[29].s64 = ctx.r[30].s64 + 4;
	// 8300A7AC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8300A7B0: 4BFD4A69  bl 0x82fdf218
	ctx.lr = 0x8300A7B4;
	sub_82FDF218(ctx, base);
	// 8300A7B4: 387E000C  addi r3, r30, 0xc
	ctx.r[3].s64 = ctx.r[30].s64 + 12;
	// 8300A7B8: 481833F9  bl 0x8318dbb0
	ctx.lr = 0x8300A7BC;
	sub_8318DBB0(ctx, base);
	// 8300A7BC: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 8300A7C0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8300A7C4: 387E0014  addi r3, r30, 0x14
	ctx.r[3].s64 = ctx.r[30].s64 + 20;
	// 8300A7C8: 4802E6D1  bl 0x83038e98
	ctx.lr = 0x8300A7CC;
	sub_83038E98(ctx, base);
	// 8300A7CC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8300A7D0: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8300A7D4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8300A7D8: 917E0020  stw r11, 0x20(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8300A7DC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8300A7E0: A15D0004  lhz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 8300A7E4: A16BA6C0  lhz r11, -0x5940(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(-22848 as u32) ) } as u64;
	// 8300A7E8: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 8300A7EC: B17D0004  sth r11, 4(r29)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 8300A7F0: 4BFD8411  bl 0x82fe2c00
	ctx.lr = 0x8300A7F4;
	sub_82FE2C00(ctx, base);
	// 8300A7F4: 907E001C  stw r3, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[3].u32 ) };
	// 8300A7F8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8300A7FC: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 8300A800: 4819D9B0  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300A804(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300A804 size=40
    let mut pc: u32 = 0x8300A804;
    'dispatch: loop {
        match pc {
            0x8300A804 => {
    //   block [0x8300A804..0x8300A82C)
	// 8300A804: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 8300A808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300A80C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300A810: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300A814: 807F00A4  lwz r3, 0xa4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 8300A818: 4BFFB369  bl 0x83005b80
	ctx.lr = 0x8300A81C;
	sub_83005B80(ctx, base);
	// 8300A81C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300A820: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300A824: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300A828: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300A82C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300A82C size=44
    let mut pc: u32 = 0x8300A82C;
    'dispatch: loop {
        match pc {
            0x8300A82C => {
    //   block [0x8300A82C..0x8300A858)
	// 8300A82C: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 8300A830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300A834: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300A838: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300A83C: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 8300A840: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 8300A844: 480BD29D  bl 0x830c7ae0
	ctx.lr = 0x8300A848;
	sub_830C7AE0(ctx, base);
	// 8300A848: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300A84C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300A850: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300A854: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300A858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300A858 size=44
    let mut pc: u32 = 0x8300A858;
    'dispatch: loop {
        match pc {
            0x8300A858 => {
    //   block [0x8300A858..0x8300A884)
	// 8300A858: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 8300A85C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300A860: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300A864: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300A868: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 8300A86C: 386B000C  addi r3, r11, 0xc
	ctx.r[3].s64 = ctx.r[11].s64 + 12;
	// 8300A870: 480BD271  bl 0x830c7ae0
	ctx.lr = 0x8300A874;
	sub_830C7AE0(ctx, base);
	// 8300A874: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300A878: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300A87C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300A880: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300A884(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300A884 size=44
    let mut pc: u32 = 0x8300A884;
    'dispatch: loop {
        match pc {
            0x8300A884 => {
    //   block [0x8300A884..0x8300A8B0)
	// 8300A884: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 8300A888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300A88C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300A890: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300A894: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 8300A898: 386B0014  addi r3, r11, 0x14
	ctx.r[3].s64 = ctx.r[11].s64 + 20;
	// 8300A89C: 480BD245  bl 0x830c7ae0
	ctx.lr = 0x8300A8A0;
	sub_830C7AE0(ctx, base);
	// 8300A8A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300A8A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300A8A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300A8AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300A8B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300A8B0 size=8
    let mut pc: u32 = 0x8300A8B0;
    'dispatch: loop {
        match pc {
            0x8300A8B0 => {
    //   block [0x8300A8B0..0x8300A8B8)
	// 8300A8B0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8300A8B4: 82141C38  lwz r16, 0x1c38(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(7224 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300A8B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300A8B8 size=128
    let mut pc: u32 = 0x8300A8B8;
    'dispatch: loop {
        match pc {
            0x8300A8B8 => {
    //   block [0x8300A8B8..0x8300A938)
	// 8300A8B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300A8BC: 4819D8AD  bl 0x831a8168
	ctx.lr = 0x8300A8C0;
	sub_831A8130(ctx, base);
	// 8300A8C0: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 8300A8C4: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300A8C8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8300A8CC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8300A8D0: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 8300A8D4: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8300A8D8: 3B9E0004  addi r28, r30, 4
	ctx.r[28].s64 = ctx.r[30].s64 + 4;
	// 8300A8DC: 396B1B00  addi r11, r11, 0x1b00
	ctx.r[11].s64 = ctx.r[11].s64 + 6912;
	// 8300A8E0: 389D0004  addi r4, r29, 4
	ctx.r[4].s64 = ctx.r[29].s64 + 4;
	// 8300A8E4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8300A8E8: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8300A8EC: 4BFD5D0D  bl 0x82fe05f8
	ctx.lr = 0x8300A8F0;
	sub_82FE05F8(ctx, base);
	// 8300A8F0: 389D000C  addi r4, r29, 0xc
	ctx.r[4].s64 = ctx.r[29].s64 + 12;
	// 8300A8F4: 387E000C  addi r3, r30, 0xc
	ctx.r[3].s64 = ctx.r[30].s64 + 12;
	// 8300A8F8: 481832B9  bl 0x8318dbb0
	ctx.lr = 0x8300A8FC;
	sub_8318DBB0(ctx, base);
	// 8300A8FC: 389D0014  addi r4, r29, 0x14
	ctx.r[4].s64 = ctx.r[29].s64 + 20;
	// 8300A900: 387E0014  addi r3, r30, 0x14
	ctx.r[3].s64 = ctx.r[30].s64 + 20;
	// 8300A904: 4802E655  bl 0x83038f58
	ctx.lr = 0x8300A908;
	sub_83038F58(ctx, base);
	// 8300A908: 817D001C  lwz r11, 0x1c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 8300A90C: 917E001C  stw r11, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 8300A910: 817D0020  lwz r11, 0x20(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32 as u32) ) } as u64;
	// 8300A914: 917E0020  stw r11, 0x20(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8300A918: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8300A91C: A15C0004  lhz r10, 4(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 8300A920: A16BA6C0  lhz r11, -0x5940(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(-22848 as u32) ) } as u64;
	// 8300A924: 7D4B5B78  or r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 8300A928: B17C0004  sth r11, 4(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 8300A92C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8300A930: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 8300A934: 4819D884  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300A938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300A938 size=40
    let mut pc: u32 = 0x8300A938;
    'dispatch: loop {
        match pc {
            0x8300A938 => {
    //   block [0x8300A938..0x8300A960)
	// 8300A938: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 8300A93C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300A940: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300A944: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300A948: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 8300A94C: 4BFFB235  bl 0x83005b80
	ctx.lr = 0x8300A950;
	sub_83005B80(ctx, base);
	// 8300A950: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300A954: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300A958: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300A95C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300A960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300A960 size=44
    let mut pc: u32 = 0x8300A960;
    'dispatch: loop {
        match pc {
            0x8300A960 => {
    //   block [0x8300A960..0x8300A98C)
	// 8300A960: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 8300A964: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300A968: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300A96C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300A970: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 8300A974: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 8300A978: 480BD169  bl 0x830c7ae0
	ctx.lr = 0x8300A97C;
	sub_830C7AE0(ctx, base);
	// 8300A97C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300A980: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300A984: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300A988: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300A98C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300A98C size=44
    let mut pc: u32 = 0x8300A98C;
    'dispatch: loop {
        match pc {
            0x8300A98C => {
    //   block [0x8300A98C..0x8300A9B8)
	// 8300A98C: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 8300A990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300A994: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300A998: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300A99C: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 8300A9A0: 386B000C  addi r3, r11, 0xc
	ctx.r[3].s64 = ctx.r[11].s64 + 12;
	// 8300A9A4: 480BD13D  bl 0x830c7ae0
	ctx.lr = 0x8300A9A8;
	sub_830C7AE0(ctx, base);
	// 8300A9A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300A9AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300A9B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300A9B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300A9B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300A9B8 size=8
    let mut pc: u32 = 0x8300A9B8;
    'dispatch: loop {
        match pc {
            0x8300A9B8 => {
    //   block [0x8300A9B8..0x8300A9C0)
	// 8300A9B8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8300A9BC: 82141C98  lwz r16, 0x1c98(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(7320 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300A9C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300A9C0 size=104
    let mut pc: u32 = 0x8300A9C0;
    'dispatch: loop {
        match pc {
            0x8300A9C0 => {
    //   block [0x8300A9C0..0x8300AA28)
	// 8300A9C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300A9C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300A9C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8300A9CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8300A9D0: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 8300A9D4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300A9D8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8300A9DC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8300A9E0: 396B1B00  addi r11, r11, 0x1b00
	ctx.r[11].s64 = ctx.r[11].s64 + 6912;
	// 8300A9E4: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 8300A9E8: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8300A9EC: 387E0014  addi r3, r30, 0x14
	ctx.r[3].s64 = ctx.r[30].s64 + 20;
	// 8300A9F0: 480BD0F1  bl 0x830c7ae0
	ctx.lr = 0x8300A9F4;
	sub_830C7AE0(ctx, base);
	// 8300A9F4: 387E000C  addi r3, r30, 0xc
	ctx.r[3].s64 = ctx.r[30].s64 + 12;
	// 8300A9F8: 480BD0E9  bl 0x830c7ae0
	ctx.lr = 0x8300A9FC;
	sub_830C7AE0(ctx, base);
	// 8300A9FC: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 8300AA00: 480BD0E1  bl 0x830c7ae0
	ctx.lr = 0x8300AA04;
	sub_830C7AE0(ctx, base);
	// 8300AA04: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8300AA08: 396BA8A0  addi r11, r11, -0x5760
	ctx.r[11].s64 = ctx.r[11].s64 + -22368;
	// 8300AA0C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8300AA10: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 8300AA14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300AA18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300AA1C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8300AA20: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8300AA24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300AA28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300AA28 size=40
    let mut pc: u32 = 0x8300AA28;
    'dispatch: loop {
        match pc {
            0x8300AA28 => {
    //   block [0x8300AA28..0x8300AA50)
	// 8300AA28: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 8300AA2C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300AA30: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300AA34: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300AA38: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 8300AA3C: 4BFFB145  bl 0x83005b80
	ctx.lr = 0x8300AA40;
	sub_83005B80(ctx, base);
	// 8300AA40: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300AA44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300AA48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300AA4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300AA50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300AA50 size=44
    let mut pc: u32 = 0x8300AA50;
    'dispatch: loop {
        match pc {
            0x8300AA50 => {
    //   block [0x8300AA50..0x8300AA7C)
	// 8300AA50: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 8300AA54: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300AA58: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300AA5C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300AA60: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 8300AA64: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 8300AA68: 480BD079  bl 0x830c7ae0
	ctx.lr = 0x8300AA6C;
	sub_830C7AE0(ctx, base);
	// 8300AA6C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300AA70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300AA74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300AA78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300AA7C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300AA7C size=44
    let mut pc: u32 = 0x8300AA7C;
    'dispatch: loop {
        match pc {
            0x8300AA7C => {
    //   block [0x8300AA7C..0x8300AAA8)
	// 8300AA7C: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 8300AA80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300AA84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300AA88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300AA8C: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 8300AA90: 386B000C  addi r3, r11, 0xc
	ctx.r[3].s64 = ctx.r[11].s64 + 12;
	// 8300AA94: 480BD04D  bl 0x830c7ae0
	ctx.lr = 0x8300AA98;
	sub_830C7AE0(ctx, base);
	// 8300AA98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300AA9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300AAA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300AAA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300AAA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300AAA8 size=8
    let mut pc: u32 = 0x8300AAA8;
    'dispatch: loop {
        match pc {
            0x8300AAA8 => {
    //   block [0x8300AAA8..0x8300AAB0)
	// 8300AAA8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8300AAAC: 82141CE0  lwz r16, 0x1ce0(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(7392 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300AAB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300AAB0 size=124
    let mut pc: u32 = 0x8300AAB0;
    'dispatch: loop {
        match pc {
            0x8300AAB0 => {
    //   block [0x8300AAB0..0x8300AB2C)
	// 8300AAB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300AAB4: 4819D6B9  bl 0x831a816c
	ctx.lr = 0x8300AAB8;
	sub_831A8130(ctx, base);
	// 8300AAB8: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 8300AABC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300AAC0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8300AAC4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8300AAC8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300AACC: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300AAD0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300AAD4: 4E800421  bctrl
	ctx.lr = 0x8300AAD8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300AAD8: 38A0000B  li r5, 0xb
	ctx.r[5].s64 = 11;
	// 8300AADC: 38800024  li r4, 0x24
	ctx.r[4].s64 = 36;
	// 8300AAE0: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8300AAE4: 4BFD9DD5  bl 0x82fe48b8
	ctx.lr = 0x8300AAE8;
	sub_82FE48B8(ctx, base);
	// 8300AAE8: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 8300AAEC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300AAF0: 41820018  beq 0x8300ab08
	if ctx.cr[0].eq {
	pc = 0x8300AB08; continue 'dispatch;
	}
	// 8300AAF4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8300AAF8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8300AAFC: 4BFFFDBD  bl 0x8300a8b8
	ctx.lr = 0x8300AB00;
	sub_8300A8B8(ctx, base);
	// 8300AB00: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8300AB04: 48000008  b 0x8300ab0c
	pc = 0x8300AB0C; continue 'dispatch;
	// 8300AB08: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8300AB0C: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 8300AB10: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8300AB14: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8300AB18: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 8300AB1C: 4BFD4C5D  bl 0x82fdf778
	ctx.lr = 0x8300AB20;
	sub_82FDF778(ctx, base);
	// 8300AB20: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8300AB24: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 8300AB28: 4819D694  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300AB2C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300AB2C size=48
    let mut pc: u32 = 0x8300AB2C;
    'dispatch: loop {
        match pc {
            0x8300AB2C => {
    //   block [0x8300AB2C..0x8300AB5C)
	// 8300AB2C: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 8300AB30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300AB34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300AB38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300AB3C: 38A0000B  li r5, 0xb
	ctx.r[5].s64 = 11;
	// 8300AB40: 809F0050  lwz r4, 0x50(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8300AB44: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8300AB48: 480BCF99  bl 0x830c7ae0
	ctx.lr = 0x8300AB4C;
	sub_830C7AE0(ctx, base);
	// 8300AB4C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300AB50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300AB54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300AB58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300AB60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300AB60 size=8
    let mut pc: u32 = 0x8300AB60;
    'dispatch: loop {
        match pc {
            0x8300AB60 => {
    //   block [0x8300AB60..0x8300AB68)
	// 8300AB60: 38600007  li r3, 7
	ctx.r[3].s64 = 7;
	// 8300AB64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300AB68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300AB68 size=376
    let mut pc: u32 = 0x8300AB68;
    'dispatch: loop {
        match pc {
            0x8300AB68 => {
    //   block [0x8300AB68..0x8300ACE0)
	// 8300AB68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300AB6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300AB70: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8300AB74: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8300AB78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300AB7C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8300AB80: 3D408214  lis r10, -0x7dec
	ctx.r[10].s64 = -2112618496;
	// 8300AB84: A17F0008  lhz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300AB88: A14AA6A4  lhz r10, -0x595c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(-22876 as u32) ) } as u64;
	// 8300AB8C: 7D4A5838  and r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 8300AB90: 7D4A0034  cntlzw r10, r10
	ctx.r[10].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 8300AB94: 554ADFFE  rlwinm r10, r10, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 8300AB98: 694A0001  xori r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u64 ^ 1;
	// 8300AB9C: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300ABA0: 41820080  beq 0x8300ac20
	if ctx.cr[0].eq {
	pc = 0x8300AC20; continue 'dispatch;
	}
	// 8300ABA4: 3D408214  lis r10, -0x7dec
	ctx.r[10].s64 = -2112618496;
	// 8300ABA8: A14AA6C8  lhz r10, -0x5938(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(-22840 as u32) ) } as u64;
	// 8300ABAC: 7D4B5838  and r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 8300ABB0: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8300ABB4: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8300ABB8: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 8300ABBC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300ABC0: 40820060  bne 0x8300ac20
	if !ctx.cr[0].eq {
	pc = 0x8300AC20; continue 'dispatch;
	}
	// 8300ABC4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300ABC8: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300ABCC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300ABD0: 4E800421  bctrl
	ctx.lr = 0x8300ABD4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300ABD4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300ABD8: 41820020  beq 0x8300abf8
	if ctx.cr[0].eq {
	pc = 0x8300ABF8; continue 'dispatch;
	}
	// 8300ABDC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300ABE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300ABE4: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300ABE8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300ABEC: 4E800421  bctrl
	ctx.lr = 0x8300ABF0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300ABF0: 80C30090  lwz r6, 0x90(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 8300ABF4: 4800000C  b 0x8300ac00
	pc = 0x8300AC00; continue 'dispatch;
	// 8300ABF8: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8300ABFC: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 8300AC00: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8300AC04: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 8300AC08: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8300AC0C: 4BFEF2C5  bl 0x82ff9ed0
	ctx.lr = 0x8300AC10;
	sub_82FF9ED0(ctx, base);
	// 8300AC10: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8300AC14: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8300AC18: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 8300AC1C: 481A600D  bl 0x831b0c28
	ctx.lr = 0x8300AC20;
	sub_831B0C28(ctx, base);
	// 8300AC20: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300AC24: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300AC28: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300AC2C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300AC30: 4E800421  bctrl
	ctx.lr = 0x8300AC34;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300AC34: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8300AC38: 41820048  beq 0x8300ac80
	if ctx.cr[0].eq {
	pc = 0x8300AC80; continue 'dispatch;
	}
	// 8300AC3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8300AC40: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8300AC44: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 8300AC48: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 8300AC4C: 4BFD4B2D  bl 0x82fdf778
	ctx.lr = 0x8300AC50;
	sub_82FDF778(ctx, base);
	// 8300AC50: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 8300AC54: 4802E5F5  bl 0x83039248
	ctx.lr = 0x8300AC58;
	sub_83039248(ctx, base);
	// 8300AC58: 38A0000B  li r5, 0xb
	ctx.r[5].s64 = 11;
	// 8300AC5C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8300AC60: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8300AC64: 4BFDAA7D  bl 0x82fe56e0
	ctx.lr = 0x8300AC68;
	sub_82FE56E0(ctx, base);
	// 8300AC68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8300AC6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300AC70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300AC74: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8300AC78: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8300AC7C: 4E800020  blr
	return;
	// 8300AC80: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300AC84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300AC88: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300AC8C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300AC90: 4E800421  bctrl
	ctx.lr = 0x8300AC94;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300AC94: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300AC98: 41820020  beq 0x8300acb8
	if ctx.cr[0].eq {
	pc = 0x8300ACB8; continue 'dispatch;
	}
	// 8300AC9C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300ACA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300ACA4: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300ACA8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300ACAC: 4E800421  bctrl
	ctx.lr = 0x8300ACB0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300ACB0: 80C30090  lwz r6, 0x90(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 8300ACB4: 4800000C  b 0x8300acc0
	pc = 0x8300ACC0; continue 'dispatch;
	// 8300ACB8: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8300ACBC: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 8300ACC0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8300ACC4: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 8300ACC8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8300ACCC: 4BFEF205  bl 0x82ff9ed0
	ctx.lr = 0x8300ACD0;
	sub_82FF9ED0(ctx, base);
	// 8300ACD0: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8300ACD4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8300ACD8: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 8300ACDC: 481A5F4D  bl 0x831b0c28
	ctx.lr = 0x8300ACE0;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300ACE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300ACE0 size=20
    let mut pc: u32 = 0x8300ACE0;
    'dispatch: loop {
        match pc {
            0x8300ACE0 => {
    //   block [0x8300ACE0..0x8300ACF4)
	// 8300ACE0: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 8300ACE4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300ACE8: 4182000C  beq 0x8300acf4
	if ctx.cr[0].eq {
		sub_8300ACF4(ctx, base);
		return;
	}
	// 8300ACEC: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8300ACF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300ACF4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300ACF4 size=20
    let mut pc: u32 = 0x8300ACF4;
    'dispatch: loop {
        match pc {
            0x8300ACF4 => {
    //   block [0x8300ACF4..0x8300AD08)
	// 8300ACF4: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8300ACF8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300ACFC: 816B0078  lwz r11, 0x78(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(120 as u32) ) } as u64;
	// 8300AD00: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300AD04: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300AD08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300AD08 size=8
    let mut pc: u32 = 0x8300AD08;
    'dispatch: loop {
        match pc {
            0x8300AD08 => {
    //   block [0x8300AD08..0x8300AD10)
	// 8300AD08: 38630004  addi r3, r3, 4
	ctx.r[3].s64 = ctx.r[3].s64 + 4;
	// 8300AD0C: 4BFD5954  b 0x82fe0660
	sub_82FE0660(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300AD10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300AD10 size=12
    let mut pc: u32 = 0x8300AD10;
    'dispatch: loop {
        match pc {
            0x8300AD10 => {
    //   block [0x8300AD10..0x8300AD1C)
	// 8300AD10: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8300AD14: 3864000C  addi r3, r4, 0xc
	ctx.r[3].s64 = ctx.r[4].s64 + 12;
	// 8300AD18: 48029BF8  b 0x83034910
	sub_83034910(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300AD20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300AD20 size=8
    let mut pc: u32 = 0x8300AD20;
    'dispatch: loop {
        match pc {
            0x8300AD20 => {
    //   block [0x8300AD20..0x8300AD28)
	// 8300AD20: 38630004  addi r3, r3, 4
	ctx.r[3].s64 = ctx.r[3].s64 + 4;
	// 8300AD24: 4BFD4AB4  b 0x82fdf7d8
	sub_82FDF7D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300AD28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300AD28 size=8
    let mut pc: u32 = 0x8300AD28;
    'dispatch: loop {
        match pc {
            0x8300AD28 => {
    //   block [0x8300AD28..0x8300AD30)
	// 8300AD28: 38630004  addi r3, r3, 4
	ctx.r[3].s64 = ctx.r[3].s64 + 4;
	// 8300AD2C: 4BFD49EC  b 0x82fdf718
	sub_82FDF718(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300AD30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300AD30 size=20
    let mut pc: u32 = 0x8300AD30;
    'dispatch: loop {
        match pc {
            0x8300AD30 => {
    //   block [0x8300AD30..0x8300AD44)
	// 8300AD30: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 8300AD34: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 8300AD38: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8300AD3C: 38630014  addi r3, r3, 0x14
	ctx.r[3].s64 = ctx.r[3].s64 + 20;
	// 8300AD40: 4802E620  b 0x83039360
	sub_83039360(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300AD48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300AD48 size=76
    let mut pc: u32 = 0x8300AD48;
    'dispatch: loop {
        match pc {
            0x8300AD48 => {
    //   block [0x8300AD48..0x8300AD94)
	// 8300AD48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300AD4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300AD50: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8300AD54: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8300AD58: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300AD5C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8300AD60: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8300AD64: 4BFFFC5D  bl 0x8300a9c0
	ctx.lr = 0x8300AD68;
	sub_8300A9C0(ctx, base);
	// 8300AD68: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8300AD6C: 4182000C  beq 0x8300ad78
	if ctx.cr[0].eq {
	pc = 0x8300AD78; continue 'dispatch;
	}
	// 8300AD70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300AD74: 4B2B54F5  bl 0x822c0268
	ctx.lr = 0x8300AD78;
	sub_822C0268(ctx, base);
	// 8300AD78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300AD7C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8300AD80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300AD84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300AD88: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8300AD8C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8300AD90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300AD98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300AD98 size=564
    let mut pc: u32 = 0x8300AD98;
    'dispatch: loop {
        match pc {
            0x8300AD98 => {
    //   block [0x8300AD98..0x8300AFCC)
	// 8300AD98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300AD9C: 4819D3C5  bl 0x831a8160
	ctx.lr = 0x8300ADA0;
	sub_831A8130(ctx, base);
	// 8300ADA0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300ADA4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8300ADA8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8300ADAC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8300ADB0: A15F0008  lhz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300ADB4: A16BA698  lhz r11, -0x5968(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(-22888 as u32) ) } as u64;
	// 8300ADB8: 7D4B5838  and r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 8300ADBC: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8300ADC0: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8300ADC4: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 8300ADC8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300ADCC: 41820060  beq 0x8300ae2c
	if ctx.cr[0].eq {
	pc = 0x8300AE2C; continue 'dispatch;
	}
	// 8300ADD0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300ADD4: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300ADD8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300ADDC: 4E800421  bctrl
	ctx.lr = 0x8300ADE0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300ADE0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300ADE4: 41820020  beq 0x8300ae04
	if ctx.cr[0].eq {
	pc = 0x8300AE04; continue 'dispatch;
	}
	// 8300ADE8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300ADEC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300ADF0: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300ADF4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300ADF8: 4E800421  bctrl
	ctx.lr = 0x8300ADFC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300ADFC: 80C30090  lwz r6, 0x90(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 8300AE00: 4800000C  b 0x8300ae0c
	pc = 0x8300AE0C; continue 'dispatch;
	// 8300AE04: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8300AE08: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 8300AE0C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8300AE10: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 8300AE14: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8300AE18: 4BFEF0B9  bl 0x82ff9ed0
	ctx.lr = 0x8300AE1C;
	sub_82FF9ED0(ctx, base);
	// 8300AE1C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8300AE20: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8300AE24: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 8300AE28: 481A5E01  bl 0x831b0c28
	ctx.lr = 0x8300AE2C;
	sub_831B0C28(ctx, base);
	// 8300AE2C: 3B9F0014  addi r28, r31, 0x14
	ctx.r[28].s64 = ctx.r[31].s64 + 20;
	// 8300AE30: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300AE34: 836B0004  lwz r27, 4(r11)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8300AE38: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300AE3C: 7F1ED840  cmplw cr6, r30, r27
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8300AE40: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300AE44: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300AE48: 41990134  bgt cr6, 0x8300af7c
	if ctx.cr[6].gt {
	pc = 0x8300AF7C; continue 'dispatch;
	}
	// 8300AE4C: 4E800421  bctrl
	ctx.lr = 0x8300AE50;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300AE50: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8300AE54: 7CDED850  subf r6, r30, r27
	ctx.r[6].s64 = ctx.r[27].s64 - ctx.r[30].s64;
	// 8300AE58: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8300AE5C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8300AE60: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8300AE64: 4802E29D  bl 0x83039100
	ctx.lr = 0x8300AE68;
	sub_83039100(ctx, base);
	// 8300AE68: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300AE6C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8300AE70: 809F001C  lwz r4, 0x1c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 8300AE74: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8300AE78: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 8300AE7C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300AE80: 4E800421  bctrl
	ctx.lr = 0x8300AE84;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300AE84: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300AE88: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 8300AE8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300AE90: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8300AE94: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300AE98: 4E800421  bctrl
	ctx.lr = 0x8300AE9C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300AE9C: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8300AEA0: 41820034  beq 0x8300aed4
	if ctx.cr[0].eq {
	pc = 0x8300AED4; continue 'dispatch;
	}
	// 8300AEA4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300AEA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300AEAC: 837D0000  lwz r27, 0(r29)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300AEB0: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8300AEB4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300AEB8: 4E800421  bctrl
	ctx.lr = 0x8300AEBC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300AEBC: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8300AEC0: 817B0034  lwz r11, 0x34(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(52 as u32) ) } as u64;
	// 8300AEC4: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8300AEC8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8300AECC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300AED0: 4E800421  bctrl
	ctx.lr = 0x8300AED4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300AED4: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300AED8: 57CA083C  slwi r10, r30, 1
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8300AEDC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8300AEE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300AEE4: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300AEE8: 7FAA4B2E  sthx r29, r10, r9
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32), ctx.r[29].u16) };
	// 8300AEEC: 93CB0004  stw r30, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 8300AEF0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300AEF4: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300AEF8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300AEFC: 4E800421  bctrl
	ctx.lr = 0x8300AF00;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300AF00: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300AF04: 4182006C  beq 0x8300af70
	if ctx.cr[0].eq {
	pc = 0x8300AF70; continue 'dispatch;
	}
	// 8300AF08: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300AF0C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300AF10: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300AF14: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300AF18: 4E800421  bctrl
	ctx.lr = 0x8300AF1C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300AF1C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300AF20: 816B00A4  lwz r11, 0xa4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(164 as u32) ) } as u64;
	// 8300AF24: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300AF28: 4E800421  bctrl
	ctx.lr = 0x8300AF2C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300AF2C: 7C7B1B79  or. r27, r3, r3
	ctx.r[27].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 8300AF30: 41820040  beq 0x8300af70
	if ctx.cr[0].eq {
	pc = 0x8300AF70; continue 'dispatch;
	}
	// 8300AF34: 839B0008  lwz r28, 8(r27)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300AF38: 281C0000  cmplwi r28, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300AF3C: 41820034  beq 0x8300af70
	if ctx.cr[0].eq {
	pc = 0x8300AF70; continue 'dispatch;
	}
	// 8300AF40: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 8300AF44: 419A002C  beq cr6, 0x8300af70
	if ctx.cr[6].eq {
	pc = 0x8300AF70; continue 'dispatch;
	}
	// 8300AF48: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8300AF4C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8300AF50: 48021921  bl 0x8302c870
	ctx.lr = 0x8300AF54;
	sub_8302C870(ctx, base);
	// 8300AF54: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8300AF58: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 8300AF5C: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8300AF60: 4BFF5ED9  bl 0x83000e38
	ctx.lr = 0x8300AF64;
	sub_83000E38(ctx, base);
	// 8300AF64: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 8300AF68: 7F1DE040  cmplw cr6, r29, r28
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[28].u32, &mut ctx.xer);
	// 8300AF6C: 4198FFDC  blt cr6, 0x8300af48
	if ctx.cr[6].lt {
	pc = 0x8300AF48; continue 'dispatch;
	}
	// 8300AF70: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8300AF74: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8300AF78: 4819D238  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
	// 8300AF7C: 4E800421  bctrl
	ctx.lr = 0x8300AF80;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300AF80: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300AF84: 41820020  beq 0x8300afa4
	if ctx.cr[0].eq {
	pc = 0x8300AFA4; continue 'dispatch;
	}
	// 8300AF88: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300AF8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300AF90: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300AF94: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300AF98: 4E800421  bctrl
	ctx.lr = 0x8300AF9C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300AF9C: 80C30090  lwz r6, 0x90(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 8300AFA0: 4800000C  b 0x8300afac
	pc = 0x8300AFAC; continue 'dispatch;
	// 8300AFA4: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8300AFA8: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 8300AFAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8300AFB0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8300AFB4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8300AFB8: 4BFEEF19  bl 0x82ff9ed0
	ctx.lr = 0x8300AFBC;
	sub_82FF9ED0(ctx, base);
	// 8300AFBC: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8300AFC0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8300AFC4: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 8300AFC8: 481A5C61  bl 0x831b0c28
	ctx.lr = 0x8300AFCC;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300AFD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300AFD0 size=8
    let mut pc: u32 = 0x8300AFD0;
    'dispatch: loop {
        match pc {
            0x8300AFD0 => {
    //   block [0x8300AFD0..0x8300AFD8)
	// 8300AFD0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8300AFD4: 82141E10  lwz r16, 0x1e10(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(7696 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300AFD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300AFD8 size=128
    let mut pc: u32 = 0x8300AFD8;
    'dispatch: loop {
        match pc {
            0x8300AFD8 => {
    //   block [0x8300AFD8..0x8300B058)
	// 8300AFD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300AFDC: 4819D189  bl 0x831a8164
	ctx.lr = 0x8300AFE0;
	sub_831A8130(ctx, base);
	// 8300AFE0: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 8300AFE4: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300AFE8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8300AFEC: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8300AFF0: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 8300AFF4: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 8300AFF8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8300AFFC: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 8300B000: 396B1D20  addi r11, r11, 0x1d20
	ctx.r[11].s64 = ctx.r[11].s64 + 7456;
	// 8300B004: 389C000C  addi r4, r28, 0xc
	ctx.r[4].s64 = ctx.r[28].s64 + 12;
	// 8300B008: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8300B00C: 409A0008  bne cr6, 0x8300b014
	if !ctx.cr[6].eq {
	pc = 0x8300B014; continue 'dispatch;
	}
	// 8300B010: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8300B014: 3BBE0004  addi r29, r30, 4
	ctx.r[29].s64 = ctx.r[30].s64 + 4;
	// 8300B018: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8300B01C: 4BFD41FD  bl 0x82fdf218
	ctx.lr = 0x8300B020;
	sub_82FDF218(ctx, base);
	// 8300B020: 387E000C  addi r3, r30, 0xc
	ctx.r[3].s64 = ctx.r[30].s64 + 12;
	// 8300B024: 48182B8D  bl 0x8318dbb0
	ctx.lr = 0x8300B028;
	sub_8318DBB0(ctx, base);
	// 8300B028: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 8300B02C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8300B030: 387E0014  addi r3, r30, 0x14
	ctx.r[3].s64 = ctx.r[30].s64 + 20;
	// 8300B034: 4802DE65  bl 0x83038e98
	ctx.lr = 0x8300B038;
	sub_83038E98(ctx, base);
	// 8300B038: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8300B03C: A15D0004  lhz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 8300B040: A16BA6C0  lhz r11, -0x5940(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(-22848 as u32) ) } as u64;
	// 8300B044: 7D4B5B78  or r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 8300B048: B17D0004  sth r11, 4(r29)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 8300B04C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8300B050: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 8300B054: 4819D160  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300B058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300B058 size=40
    let mut pc: u32 = 0x8300B058;
    'dispatch: loop {
        match pc {
            0x8300B058 => {
    //   block [0x8300B058..0x8300B080)
	// 8300B058: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 8300B05C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300B060: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300B064: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300B068: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 8300B06C: 4BFFAB15  bl 0x83005b80
	ctx.lr = 0x8300B070;
	sub_83005B80(ctx, base);
	// 8300B070: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300B074: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300B078: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300B07C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300B080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300B080 size=44
    let mut pc: u32 = 0x8300B080;
    'dispatch: loop {
        match pc {
            0x8300B080 => {
    //   block [0x8300B080..0x8300B0AC)
	// 8300B080: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 8300B084: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300B088: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300B08C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300B090: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 8300B094: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 8300B098: 480BCA49  bl 0x830c7ae0
	ctx.lr = 0x8300B09C;
	sub_830C7AE0(ctx, base);
	// 8300B09C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300B0A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300B0A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300B0A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300B0AC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300B0AC size=44
    let mut pc: u32 = 0x8300B0AC;
    'dispatch: loop {
        match pc {
            0x8300B0AC => {
    //   block [0x8300B0AC..0x8300B0D8)
	// 8300B0AC: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 8300B0B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300B0B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300B0B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300B0BC: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 8300B0C0: 386B000C  addi r3, r11, 0xc
	ctx.r[3].s64 = ctx.r[11].s64 + 12;
	// 8300B0C4: 480BCA1D  bl 0x830c7ae0
	ctx.lr = 0x8300B0C8;
	sub_830C7AE0(ctx, base);
	// 8300B0C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300B0CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300B0D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300B0D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300B0D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300B0D8 size=8
    let mut pc: u32 = 0x8300B0D8;
    'dispatch: loop {
        match pc {
            0x8300B0D8 => {
    //   block [0x8300B0D8..0x8300B0E0)
	// 8300B0D8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8300B0DC: 82141E70  lwz r16, 0x1e70(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(7792 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300B0E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300B0E0 size=108
    let mut pc: u32 = 0x8300B0E0;
    'dispatch: loop {
        match pc {
            0x8300B0E0 => {
    //   block [0x8300B0E0..0x8300B14C)
	// 8300B0E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300B0E4: 4819D085  bl 0x831a8168
	ctx.lr = 0x8300B0E8;
	sub_831A8130(ctx, base);
	// 8300B0E8: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 8300B0EC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300B0F0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8300B0F4: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8300B0F8: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 8300B0FC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8300B100: 3BBE0004  addi r29, r30, 4
	ctx.r[29].s64 = ctx.r[30].s64 + 4;
	// 8300B104: 396B1D20  addi r11, r11, 0x1d20
	ctx.r[11].s64 = ctx.r[11].s64 + 7456;
	// 8300B108: 389C0004  addi r4, r28, 4
	ctx.r[4].s64 = ctx.r[28].s64 + 4;
	// 8300B10C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8300B110: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8300B114: 4BFD54E5  bl 0x82fe05f8
	ctx.lr = 0x8300B118;
	sub_82FE05F8(ctx, base);
	// 8300B118: 387E000C  addi r3, r30, 0xc
	ctx.r[3].s64 = ctx.r[30].s64 + 12;
	// 8300B11C: 48182A95  bl 0x8318dbb0
	ctx.lr = 0x8300B120;
	sub_8318DBB0(ctx, base);
	// 8300B120: 389C0014  addi r4, r28, 0x14
	ctx.r[4].s64 = ctx.r[28].s64 + 20;
	// 8300B124: 387E0014  addi r3, r30, 0x14
	ctx.r[3].s64 = ctx.r[30].s64 + 20;
	// 8300B128: 4802DE31  bl 0x83038f58
	ctx.lr = 0x8300B12C;
	sub_83038F58(ctx, base);
	// 8300B12C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8300B130: A15D0004  lhz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 8300B134: A16BA6C0  lhz r11, -0x5940(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(-22848 as u32) ) } as u64;
	// 8300B138: 7D4B5B78  or r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 8300B13C: B17D0004  sth r11, 4(r29)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 8300B140: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8300B144: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 8300B148: 4819D070  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300B14C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300B14C size=40
    let mut pc: u32 = 0x8300B14C;
    'dispatch: loop {
        match pc {
            0x8300B14C => {
    //   block [0x8300B14C..0x8300B174)
	// 8300B14C: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 8300B150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300B154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300B158: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300B15C: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 8300B160: 4BFFAA21  bl 0x83005b80
	ctx.lr = 0x8300B164;
	sub_83005B80(ctx, base);
	// 8300B164: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300B168: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300B16C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300B170: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300B174(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300B174 size=44
    let mut pc: u32 = 0x8300B174;
    'dispatch: loop {
        match pc {
            0x8300B174 => {
    //   block [0x8300B174..0x8300B1A0)
	// 8300B174: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 8300B178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300B17C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300B180: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300B184: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 8300B188: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 8300B18C: 480BC955  bl 0x830c7ae0
	ctx.lr = 0x8300B190;
	sub_830C7AE0(ctx, base);
	// 8300B190: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300B194: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300B198: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300B19C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300B1A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300B1A0 size=44
    let mut pc: u32 = 0x8300B1A0;
    'dispatch: loop {
        match pc {
            0x8300B1A0 => {
    //   block [0x8300B1A0..0x8300B1CC)
	// 8300B1A0: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 8300B1A4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300B1A8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300B1AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300B1B0: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 8300B1B4: 386B000C  addi r3, r11, 0xc
	ctx.r[3].s64 = ctx.r[11].s64 + 12;
	// 8300B1B8: 480BC929  bl 0x830c7ae0
	ctx.lr = 0x8300B1BC;
	sub_830C7AE0(ctx, base);
	// 8300B1BC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300B1C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300B1C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300B1C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300B1D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300B1D0 size=8
    let mut pc: u32 = 0x8300B1D0;
    'dispatch: loop {
        match pc {
            0x8300B1D0 => {
    //   block [0x8300B1D0..0x8300B1D8)
	// 8300B1D0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8300B1D4: 82141ED0  lwz r16, 0x1ed0(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(7888 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300B1D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300B1D8 size=104
    let mut pc: u32 = 0x8300B1D8;
    'dispatch: loop {
        match pc {
            0x8300B1D8 => {
    //   block [0x8300B1D8..0x8300B240)
	// 8300B1D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300B1DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300B1E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8300B1E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8300B1E8: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 8300B1EC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300B1F0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8300B1F4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8300B1F8: 396B1D20  addi r11, r11, 0x1d20
	ctx.r[11].s64 = ctx.r[11].s64 + 7456;
	// 8300B1FC: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 8300B200: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8300B204: 387E0014  addi r3, r30, 0x14
	ctx.r[3].s64 = ctx.r[30].s64 + 20;
	// 8300B208: 480BC8D9  bl 0x830c7ae0
	ctx.lr = 0x8300B20C;
	sub_830C7AE0(ctx, base);
	// 8300B20C: 387E000C  addi r3, r30, 0xc
	ctx.r[3].s64 = ctx.r[30].s64 + 12;
	// 8300B210: 480BC8D1  bl 0x830c7ae0
	ctx.lr = 0x8300B214;
	sub_830C7AE0(ctx, base);
	// 8300B214: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 8300B218: 480BC8C9  bl 0x830c7ae0
	ctx.lr = 0x8300B21C;
	sub_830C7AE0(ctx, base);
	// 8300B21C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8300B220: 396BA8A0  addi r11, r11, -0x5760
	ctx.r[11].s64 = ctx.r[11].s64 + -22368;
	// 8300B224: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8300B228: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 8300B22C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300B230: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300B234: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8300B238: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8300B23C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300B240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300B240 size=40
    let mut pc: u32 = 0x8300B240;
    'dispatch: loop {
        match pc {
            0x8300B240 => {
    //   block [0x8300B240..0x8300B268)
	// 8300B240: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 8300B244: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300B248: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300B24C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300B250: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 8300B254: 4BFFA92D  bl 0x83005b80
	ctx.lr = 0x8300B258;
	sub_83005B80(ctx, base);
	// 8300B258: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300B25C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300B260: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300B264: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300B268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300B268 size=44
    let mut pc: u32 = 0x8300B268;
    'dispatch: loop {
        match pc {
            0x8300B268 => {
    //   block [0x8300B268..0x8300B294)
	// 8300B268: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 8300B26C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300B270: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300B274: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300B278: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 8300B27C: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 8300B280: 480BC861  bl 0x830c7ae0
	ctx.lr = 0x8300B284;
	sub_830C7AE0(ctx, base);
	// 8300B284: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300B288: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300B28C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300B290: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300B294(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300B294 size=44
    let mut pc: u32 = 0x8300B294;
    'dispatch: loop {
        match pc {
            0x8300B294 => {
    //   block [0x8300B294..0x8300B2C0)
	// 8300B294: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 8300B298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300B29C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300B2A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300B2A4: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 8300B2A8: 386B000C  addi r3, r11, 0xc
	ctx.r[3].s64 = ctx.r[11].s64 + 12;
	// 8300B2AC: 480BC835  bl 0x830c7ae0
	ctx.lr = 0x8300B2B0;
	sub_830C7AE0(ctx, base);
	// 8300B2B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300B2B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300B2B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300B2BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300B2C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300B2C0 size=8
    let mut pc: u32 = 0x8300B2C0;
    'dispatch: loop {
        match pc {
            0x8300B2C0 => {
    //   block [0x8300B2C0..0x8300B2C8)
	// 8300B2C0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8300B2C4: 82141F18  lwz r16, 0x1f18(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(7960 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300B2C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300B2C8 size=124
    let mut pc: u32 = 0x8300B2C8;
    'dispatch: loop {
        match pc {
            0x8300B2C8 => {
    //   block [0x8300B2C8..0x8300B344)
	// 8300B2C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300B2CC: 4819CEA1  bl 0x831a816c
	ctx.lr = 0x8300B2D0;
	sub_831A8130(ctx, base);
	// 8300B2D0: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 8300B2D4: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300B2D8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8300B2DC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8300B2E0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300B2E4: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300B2E8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300B2EC: 4E800421  bctrl
	ctx.lr = 0x8300B2F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300B2F0: 38A0000C  li r5, 0xc
	ctx.r[5].s64 = 12;
	// 8300B2F4: 3880001C  li r4, 0x1c
	ctx.r[4].s64 = 28;
	// 8300B2F8: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8300B2FC: 4BFD95BD  bl 0x82fe48b8
	ctx.lr = 0x8300B300;
	sub_82FE48B8(ctx, base);
	// 8300B300: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 8300B304: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300B308: 41820018  beq 0x8300b320
	if ctx.cr[0].eq {
	pc = 0x8300B320; continue 'dispatch;
	}
	// 8300B30C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8300B310: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8300B314: 4BFFFDCD  bl 0x8300b0e0
	ctx.lr = 0x8300B318;
	sub_8300B0E0(ctx, base);
	// 8300B318: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8300B31C: 48000008  b 0x8300b324
	pc = 0x8300B324; continue 'dispatch;
	// 8300B320: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8300B324: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 8300B328: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8300B32C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8300B330: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 8300B334: 4BFD4445  bl 0x82fdf778
	ctx.lr = 0x8300B338;
	sub_82FDF778(ctx, base);
	// 8300B338: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8300B33C: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 8300B340: 4819CE7C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300B344(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300B344 size=48
    let mut pc: u32 = 0x8300B344;
    'dispatch: loop {
        match pc {
            0x8300B344 => {
    //   block [0x8300B344..0x8300B374)
	// 8300B344: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 8300B348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300B34C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300B350: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300B354: 38A0000C  li r5, 0xc
	ctx.r[5].s64 = 12;
	// 8300B358: 809F0050  lwz r4, 0x50(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8300B35C: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8300B360: 480BC781  bl 0x830c7ae0
	ctx.lr = 0x8300B364;
	sub_830C7AE0(ctx, base);
	// 8300B364: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300B368: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300B36C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300B370: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300B378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300B378 size=12
    let mut pc: u32 = 0x8300B378;
    'dispatch: loop {
        match pc {
            0x8300B378 => {
    //   block [0x8300B378..0x8300B384)
	// 8300B378: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8300B37C: 386B1F58  addi r3, r11, 0x1f58
	ctx.r[3].s64 = ctx.r[11].s64 + 8024;
	// 8300B380: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300B388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300B388 size=8
    let mut pc: u32 = 0x8300B388;
    'dispatch: loop {
        match pc {
            0x8300B388 => {
    //   block [0x8300B388..0x8300B390)
	// 8300B388: 38600003  li r3, 3
	ctx.r[3].s64 = 3;
	// 8300B38C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300B390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300B390 size=32
    let mut pc: u32 = 0x8300B390;
    'dispatch: loop {
        match pc {
            0x8300B390 => {
    //   block [0x8300B390..0x8300B3B0)
	// 8300B390: 548A063F  clrlwi. r10, r4, 0x18
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8300B394: 39630004  addi r11, r3, 4
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	// 8300B398: 3D408214  lis r10, -0x7dec
	ctx.r[10].s64 = -2112618496;
	// 8300B39C: A12B0004  lhz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8300B3A0: A14AA6B0  lhz r10, -0x5950(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(-22864 as u32) ) } as u64;
	// 8300B3A4: 4182000C  beq 0x8300b3b0
	if ctx.cr[0].eq {
		sub_8300B3B0(ctx, base);
		return;
	}
	// 8300B3A8: 7D2A5378  or r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 | ctx.r[10].u64;
	// 8300B3AC: 48000008  b 0x8300b3b4
	sub_8300B3B0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300B3B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300B3B0 size=12
    let mut pc: u32 = 0x8300B3B0;
    'dispatch: loop {
        match pc {
            0x8300B3B0 => {
    //   block [0x8300B3B0..0x8300B3BC)
	// 8300B3B0: 7D2A5078  andc r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 & !ctx.r[10].u64;
	// 8300B3B4: B14B0004  sth r10, 4(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 8300B3B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300B3C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300B3C0 size=16
    let mut pc: u32 = 0x8300B3C0;
    'dispatch: loop {
        match pc {
            0x8300B3C0 => {
    //   block [0x8300B3C0..0x8300B3D0)
	// 8300B3C0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300B3C4: 816B00CC  lwz r11, 0xcc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(204 as u32) ) } as u64;
	// 8300B3C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300B3CC: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300B3D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300B3D0 size=376
    let mut pc: u32 = 0x8300B3D0;
    'dispatch: loop {
        match pc {
            0x8300B3D0 => {
    //   block [0x8300B3D0..0x8300B548)
	// 8300B3D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300B3D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300B3D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8300B3DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8300B3E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300B3E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8300B3E8: 3D408214  lis r10, -0x7dec
	ctx.r[10].s64 = -2112618496;
	// 8300B3EC: A17F0008  lhz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300B3F0: A14AA6A4  lhz r10, -0x595c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(-22876 as u32) ) } as u64;
	// 8300B3F4: 7D4A5838  and r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 8300B3F8: 7D4A0034  cntlzw r10, r10
	ctx.r[10].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 8300B3FC: 554ADFFE  rlwinm r10, r10, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 8300B400: 694A0001  xori r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u64 ^ 1;
	// 8300B404: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300B408: 41820080  beq 0x8300b488
	if ctx.cr[0].eq {
	pc = 0x8300B488; continue 'dispatch;
	}
	// 8300B40C: 3D408214  lis r10, -0x7dec
	ctx.r[10].s64 = -2112618496;
	// 8300B410: A14AA6C8  lhz r10, -0x5938(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(-22840 as u32) ) } as u64;
	// 8300B414: 7D4B5838  and r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 8300B418: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8300B41C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8300B420: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 8300B424: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300B428: 40820060  bne 0x8300b488
	if !ctx.cr[0].eq {
	pc = 0x8300B488; continue 'dispatch;
	}
	// 8300B42C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300B430: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300B434: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300B438: 4E800421  bctrl
	ctx.lr = 0x8300B43C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300B43C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300B440: 41820020  beq 0x8300b460
	if ctx.cr[0].eq {
	pc = 0x8300B460; continue 'dispatch;
	}
	// 8300B444: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300B448: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300B44C: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300B450: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300B454: 4E800421  bctrl
	ctx.lr = 0x8300B458;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300B458: 80C30090  lwz r6, 0x90(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 8300B45C: 4800000C  b 0x8300b468
	pc = 0x8300B468; continue 'dispatch;
	// 8300B460: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8300B464: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 8300B468: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8300B46C: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 8300B470: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8300B474: 4BFEEA5D  bl 0x82ff9ed0
	ctx.lr = 0x8300B478;
	sub_82FF9ED0(ctx, base);
	// 8300B478: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8300B47C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8300B480: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 8300B484: 481A57A5  bl 0x831b0c28
	ctx.lr = 0x8300B488;
	sub_831B0C28(ctx, base);
	// 8300B488: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300B48C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300B490: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300B494: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300B498: 4E800421  bctrl
	ctx.lr = 0x8300B49C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300B49C: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8300B4A0: 41820048  beq 0x8300b4e8
	if ctx.cr[0].eq {
	pc = 0x8300B4E8; continue 'dispatch;
	}
	// 8300B4A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8300B4A8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8300B4AC: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 8300B4B0: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 8300B4B4: 4BFD42C5  bl 0x82fdf778
	ctx.lr = 0x8300B4B8;
	sub_82FDF778(ctx, base);
	// 8300B4B8: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 8300B4BC: 4802DD8D  bl 0x83039248
	ctx.lr = 0x8300B4C0;
	sub_83039248(ctx, base);
	// 8300B4C0: 38A0000C  li r5, 0xc
	ctx.r[5].s64 = 12;
	// 8300B4C4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8300B4C8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8300B4CC: 4BFDA215  bl 0x82fe56e0
	ctx.lr = 0x8300B4D0;
	sub_82FE56E0(ctx, base);
	// 8300B4D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8300B4D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300B4D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300B4DC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8300B4E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8300B4E4: 4E800020  blr
	return;
	// 8300B4E8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300B4EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300B4F0: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300B4F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300B4F8: 4E800421  bctrl
	ctx.lr = 0x8300B4FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300B4FC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300B500: 41820020  beq 0x8300b520
	if ctx.cr[0].eq {
	pc = 0x8300B520; continue 'dispatch;
	}
	// 8300B504: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300B508: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300B50C: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300B510: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300B514: 4E800421  bctrl
	ctx.lr = 0x8300B518;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300B518: 80C30090  lwz r6, 0x90(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 8300B51C: 4800000C  b 0x8300b528
	pc = 0x8300B528; continue 'dispatch;
	// 8300B520: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8300B524: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 8300B528: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8300B52C: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 8300B530: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8300B534: 4BFEE99D  bl 0x82ff9ed0
	ctx.lr = 0x8300B538;
	sub_82FF9ED0(ctx, base);
	// 8300B538: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8300B53C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8300B540: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 8300B544: 481A56E5  bl 0x831b0c28
	ctx.lr = 0x8300B548;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300B548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300B548 size=8
    let mut pc: u32 = 0x8300B548;
    'dispatch: loop {
        match pc {
            0x8300B548 => {
    //   block [0x8300B548..0x8300B550)
	// 8300B548: 38630014  addi r3, r3, 0x14
	ctx.r[3].s64 = ctx.r[3].s64 + 20;
	// 8300B54C: 4802DB84  b 0x830390d0
	sub_830390D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300B550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300B550 size=12
    let mut pc: u32 = 0x8300B550;
    'dispatch: loop {
        match pc {
            0x8300B550 => {
    //   block [0x8300B550..0x8300B55C)
	// 8300B550: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8300B554: 3864000C  addi r3, r4, 0xc
	ctx.r[3].s64 = ctx.r[4].s64 + 12;
	// 8300B558: 48029380  b 0x830348d8
	sub_830348D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300B560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300B560 size=8
    let mut pc: u32 = 0x8300B560;
    'dispatch: loop {
        match pc {
            0x8300B560 => {
    //   block [0x8300B560..0x8300B568)
	// 8300B560: 38630014  addi r3, r3, 0x14
	ctx.r[3].s64 = ctx.r[3].s64 + 20;
	// 8300B564: 4802DB8C  b 0x830390f0
	sub_830390F0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300B568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300B568 size=20
    let mut pc: u32 = 0x8300B568;
    'dispatch: loop {
        match pc {
            0x8300B568 => {
    //   block [0x8300B568..0x8300B57C)
	// 8300B568: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 8300B56C: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 8300B570: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8300B574: 38630014  addi r3, r3, 0x14
	ctx.r[3].s64 = ctx.r[3].s64 + 20;
	// 8300B578: 4802E020  b 0x83039598
	sub_83039598(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300B580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300B580 size=24
    let mut pc: u32 = 0x8300B580;
    'dispatch: loop {
        match pc {
            0x8300B580 => {
    //   block [0x8300B580..0x8300B598)
	// 8300B580: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 8300B584: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 8300B588: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 8300B58C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8300B590: 38630014  addi r3, r3, 0x14
	ctx.r[3].s64 = ctx.r[3].s64 + 20;
	// 8300B594: 4802E274  b 0x83039808
	sub_83039808(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300B598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300B598 size=16
    let mut pc: u32 = 0x8300B598;
    'dispatch: loop {
        match pc {
            0x8300B598 => {
    //   block [0x8300B598..0x8300B5A8)
	// 8300B598: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 8300B59C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8300B5A0: 38630014  addi r3, r3, 0x14
	ctx.r[3].s64 = ctx.r[3].s64 + 20;
	// 8300B5A4: 4802DCB4  b 0x83039258
	sub_83039258(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300B5A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300B5A8 size=76
    let mut pc: u32 = 0x8300B5A8;
    'dispatch: loop {
        match pc {
            0x8300B5A8 => {
    //   block [0x8300B5A8..0x8300B5F4)
	// 8300B5A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300B5AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300B5B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8300B5B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8300B5B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300B5BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8300B5C0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8300B5C4: 4BFFFC15  bl 0x8300b1d8
	ctx.lr = 0x8300B5C8;
	sub_8300B1D8(ctx, base);
	// 8300B5C8: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8300B5CC: 4182000C  beq 0x8300b5d8
	if ctx.cr[0].eq {
	pc = 0x8300B5D8; continue 'dispatch;
	}
	// 8300B5D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300B5D4: 4B2B4C95  bl 0x822c0268
	ctx.lr = 0x8300B5D8;
	sub_822C0268(ctx, base);
	// 8300B5D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300B5DC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8300B5E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300B5E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300B5E8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8300B5EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8300B5F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300B5F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300B5F8 size=564
    let mut pc: u32 = 0x8300B5F8;
    'dispatch: loop {
        match pc {
            0x8300B5F8 => {
    //   block [0x8300B5F8..0x8300B82C)
	// 8300B5F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300B5FC: 4819CB65  bl 0x831a8160
	ctx.lr = 0x8300B600;
	sub_831A8130(ctx, base);
	// 8300B600: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300B604: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8300B608: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8300B60C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8300B610: A15F0008  lhz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300B614: A16BA698  lhz r11, -0x5968(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(-22888 as u32) ) } as u64;
	// 8300B618: 7D4B5838  and r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 8300B61C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8300B620: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8300B624: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 8300B628: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300B62C: 41820060  beq 0x8300b68c
	if ctx.cr[0].eq {
	pc = 0x8300B68C; continue 'dispatch;
	}
	// 8300B630: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300B634: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300B638: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300B63C: 4E800421  bctrl
	ctx.lr = 0x8300B640;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300B640: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300B644: 41820020  beq 0x8300b664
	if ctx.cr[0].eq {
	pc = 0x8300B664; continue 'dispatch;
	}
	// 8300B648: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300B64C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300B650: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300B654: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300B658: 4E800421  bctrl
	ctx.lr = 0x8300B65C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300B65C: 80C30090  lwz r6, 0x90(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 8300B660: 4800000C  b 0x8300b66c
	pc = 0x8300B66C; continue 'dispatch;
	// 8300B664: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8300B668: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 8300B66C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8300B670: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 8300B674: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8300B678: 4BFEE859  bl 0x82ff9ed0
	ctx.lr = 0x8300B67C;
	sub_82FF9ED0(ctx, base);
	// 8300B67C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8300B680: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8300B684: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 8300B688: 481A55A1  bl 0x831b0c28
	ctx.lr = 0x8300B68C;
	sub_831B0C28(ctx, base);
	// 8300B68C: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8300B690: 838B0004  lwz r28, 4(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8300B694: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300B698: 7F1DE040  cmplw cr6, r29, r28
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[28].u32, &mut ctx.xer);
	// 8300B69C: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300B6A0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300B6A4: 41990138  bgt cr6, 0x8300b7dc
	if ctx.cr[6].gt {
	pc = 0x8300B7DC; continue 'dispatch;
	}
	// 8300B6A8: 4E800421  bctrl
	ctx.lr = 0x8300B6AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300B6AC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300B6B0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8300B6B4: 7CBDE050  subf r5, r29, r28
	ctx.r[5].s64 = ctx.r[28].s64 - ctx.r[29].s64;
	// 8300B6B8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8300B6BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300B6C0: 816B00A4  lwz r11, 0xa4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(164 as u32) ) } as u64;
	// 8300B6C4: 839E0000  lwz r28, 0(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300B6C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300B6CC: 4E800421  bctrl
	ctx.lr = 0x8300B6D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300B6D0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8300B6D4: 817C0010  lwz r11, 0x10(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(16 as u32) ) } as u64;
	// 8300B6D8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8300B6DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300B6E0: 4E800421  bctrl
	ctx.lr = 0x8300B6E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300B6E4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300B6E8: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 8300B6EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300B6F0: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8300B6F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300B6F8: 4E800421  bctrl
	ctx.lr = 0x8300B6FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300B6FC: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8300B700: 41820034  beq 0x8300b734
	if ctx.cr[0].eq {
	pc = 0x8300B734; continue 'dispatch;
	}
	// 8300B704: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300B708: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300B70C: 839E0000  lwz r28, 0(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300B710: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8300B714: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300B718: 4E800421  bctrl
	ctx.lr = 0x8300B71C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300B71C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8300B720: 817C0034  lwz r11, 0x34(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(52 as u32) ) } as u64;
	// 8300B724: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8300B728: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8300B72C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300B730: 4E800421  bctrl
	ctx.lr = 0x8300B734;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300B734: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8300B738: 57AA083C  slwi r10, r29, 1
	ctx.r[10].u32 = ctx.r[29].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8300B73C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8300B740: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300B744: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300B748: 7FCA4B2E  sthx r30, r10, r9
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32), ctx.r[30].u16) };
	// 8300B74C: 93AB0004  stw r29, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 8300B750: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300B754: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300B758: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300B75C: 4E800421  bctrl
	ctx.lr = 0x8300B760;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300B760: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300B764: 4182006C  beq 0x8300b7d0
	if ctx.cr[0].eq {
	pc = 0x8300B7D0; continue 'dispatch;
	}
	// 8300B768: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300B76C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300B770: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300B774: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300B778: 4E800421  bctrl
	ctx.lr = 0x8300B77C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300B77C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300B780: 816B00A4  lwz r11, 0xa4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(164 as u32) ) } as u64;
	// 8300B784: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300B788: 4E800421  bctrl
	ctx.lr = 0x8300B78C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300B78C: 7C7B1B79  or. r27, r3, r3
	ctx.r[27].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 8300B790: 41820040  beq 0x8300b7d0
	if ctx.cr[0].eq {
	pc = 0x8300B7D0; continue 'dispatch;
	}
	// 8300B794: 839B0008  lwz r28, 8(r27)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300B798: 281C0000  cmplwi r28, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300B79C: 41820034  beq 0x8300b7d0
	if ctx.cr[0].eq {
	pc = 0x8300B7D0; continue 'dispatch;
	}
	// 8300B7A0: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 8300B7A4: 419A002C  beq cr6, 0x8300b7d0
	if ctx.cr[6].eq {
	pc = 0x8300B7D0; continue 'dispatch;
	}
	// 8300B7A8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8300B7AC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8300B7B0: 480210C1  bl 0x8302c870
	ctx.lr = 0x8300B7B4;
	sub_8302C870(ctx, base);
	// 8300B7B4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8300B7B8: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 8300B7BC: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 8300B7C0: 4BFF5679  bl 0x83000e38
	ctx.lr = 0x8300B7C4;
	sub_83000E38(ctx, base);
	// 8300B7C4: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 8300B7C8: 7F1EE040  cmplw cr6, r30, r28
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[28].u32, &mut ctx.xer);
	// 8300B7CC: 4198FFDC  blt cr6, 0x8300b7a8
	if ctx.cr[6].lt {
	pc = 0x8300B7A8; continue 'dispatch;
	}
	// 8300B7D0: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8300B7D4: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8300B7D8: 4819C9D8  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
	// 8300B7DC: 4E800421  bctrl
	ctx.lr = 0x8300B7E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300B7E0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300B7E4: 41820020  beq 0x8300b804
	if ctx.cr[0].eq {
	pc = 0x8300B804; continue 'dispatch;
	}
	// 8300B7E8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300B7EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300B7F0: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300B7F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300B7F8: 4E800421  bctrl
	ctx.lr = 0x8300B7FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300B7FC: 80C30090  lwz r6, 0x90(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 8300B800: 4800000C  b 0x8300b80c
	pc = 0x8300B80C; continue 'dispatch;
	// 8300B804: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8300B808: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 8300B80C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8300B810: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8300B814: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8300B818: 4BFEE6B9  bl 0x82ff9ed0
	ctx.lr = 0x8300B81C;
	sub_82FF9ED0(ctx, base);
	// 8300B81C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8300B820: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8300B824: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 8300B828: 481A5401  bl 0x831b0c28
	ctx.lr = 0x8300B82C;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300B830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300B830 size=104
    let mut pc: u32 = 0x8300B830;
    'dispatch: loop {
        match pc {
            0x8300B830 => {
    //   block [0x8300B830..0x8300B898)
	// 8300B830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300B834: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300B838: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8300B83C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8300B840: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300B844: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8300B848: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8300B84C: 4BFFCEFD  bl 0x83008748
	ctx.lr = 0x8300B850;
	sub_83008748(ctx, base);
	// 8300B850: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8300B854: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300B858: 396B1F80  addi r11, r11, 0x1f80
	ctx.r[11].s64 = ctx.r[11].s64 + 8064;
	// 8300B85C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8300B860: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 8300B864: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 8300B868: 817E0034  lwz r11, 0x34(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 8300B86C: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 8300B870: 817E0038  lwz r11, 0x38(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(56 as u32) ) } as u64;
	// 8300B874: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 8300B878: 817E003C  lwz r11, 0x3c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(60 as u32) ) } as u64;
	// 8300B87C: 917F003C  stw r11, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 8300B880: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8300B884: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300B888: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300B88C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8300B890: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8300B894: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300B898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300B898 size=8
    let mut pc: u32 = 0x8300B898;
    'dispatch: loop {
        match pc {
            0x8300B898 => {
    //   block [0x8300B898..0x8300B8A0)
	// 8300B898: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8300B89C: 82142090  lwz r16, 0x2090(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(8336 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300B8A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300B8A0 size=124
    let mut pc: u32 = 0x8300B8A0;
    'dispatch: loop {
        match pc {
            0x8300B8A0 => {
    //   block [0x8300B8A0..0x8300B91C)
	// 8300B8A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300B8A4: 4819C8C9  bl 0x831a816c
	ctx.lr = 0x8300B8A8;
	sub_831A8130(ctx, base);
	// 8300B8A8: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 8300B8AC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300B8B0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8300B8B4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8300B8B8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300B8BC: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300B8C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300B8C4: 4E800421  bctrl
	ctx.lr = 0x8300B8C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300B8C8: 38A00007  li r5, 7
	ctx.r[5].s64 = 7;
	// 8300B8CC: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 8300B8D0: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8300B8D4: 4BFD8FE5  bl 0x82fe48b8
	ctx.lr = 0x8300B8D8;
	sub_82FE48B8(ctx, base);
	// 8300B8D8: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 8300B8DC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300B8E0: 41820018  beq 0x8300b8f8
	if ctx.cr[0].eq {
	pc = 0x8300B8F8; continue 'dispatch;
	}
	// 8300B8E4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8300B8E8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8300B8EC: 4BFFFF45  bl 0x8300b830
	ctx.lr = 0x8300B8F0;
	sub_8300B830(ctx, base);
	// 8300B8F0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8300B8F4: 48000008  b 0x8300b8fc
	pc = 0x8300B8FC; continue 'dispatch;
	// 8300B8F8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8300B8FC: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 8300B900: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8300B904: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8300B908: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 8300B90C: 4BFD3E6D  bl 0x82fdf778
	ctx.lr = 0x8300B910;
	sub_82FDF778(ctx, base);
	// 8300B910: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8300B914: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 8300B918: 4819C8A4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300B91C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300B91C size=48
    let mut pc: u32 = 0x8300B91C;
    'dispatch: loop {
        match pc {
            0x8300B91C => {
    //   block [0x8300B91C..0x8300B94C)
	// 8300B91C: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 8300B920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300B924: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300B928: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300B92C: 38A00007  li r5, 7
	ctx.r[5].s64 = 7;
	// 8300B930: 809F0050  lwz r4, 0x50(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8300B934: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8300B938: 480BC1A9  bl 0x830c7ae0
	ctx.lr = 0x8300B93C;
	sub_830C7AE0(ctx, base);
	// 8300B93C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300B940: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300B944: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300B948: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300B950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300B950 size=8
    let mut pc: u32 = 0x8300B950;
    'dispatch: loop {
        match pc {
            0x8300B950 => {
    //   block [0x8300B950..0x8300B958)
	// 8300B950: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8300B954: 82142124  lwz r16, 0x2124(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(8484 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300B958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300B958 size=436
    let mut pc: u32 = 0x8300B958;
    'dispatch: loop {
        match pc {
            0x8300B958 => {
    //   block [0x8300B958..0x8300BB0C)
	// 8300B958: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300B95C: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 8300B960: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 8300B964: 4819C801  bl 0x831a8164
	ctx.lr = 0x8300B968;
	sub_831A8130(ctx, base);
	// 8300B968: 3BE1FF10  addi r31, r1, -0xf0
	ctx.r[31].s64 = ctx.r[1].s64 + -240;
	// 8300B96C: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300B970: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8300B974: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8300B978: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300B97C: 816B0078  lwz r11, 0x78(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(120 as u32) ) } as u64;
	// 8300B980: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300B984: 4E800421  bctrl
	ctx.lr = 0x8300B988;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300B988: 817E0024  lwz r11, 0x24(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 8300B98C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8300B990: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8300B994: 419A016C  beq cr6, 0x8300bb00
	if ctx.cr[6].eq {
	pc = 0x8300BB00; continue 'dispatch;
	}
	// 8300B998: 39400062  li r10, 0x62
	ctx.r[10].s64 = 98;
	// 8300B99C: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8300B9A0: 3B7F0050  addi r27, r31, 0x50
	ctx.r[27].s64 = ctx.r[31].s64 + 80;
	// 8300B9A4: B15F0050  sth r10, 0x50(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[10].u16 ) };
	// 8300B9A8: 39400061  li r10, 0x61
	ctx.r[10].s64 = 97;
	// 8300B9AC: B15F0052  sth r10, 0x52(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(82 as u32), ctx.r[10].u16 ) };
	// 8300B9B0: 39400073  li r10, 0x73
	ctx.r[10].s64 = 115;
	// 8300B9B4: B15F0054  sth r10, 0x54(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[10].u16 ) };
	// 8300B9B8: 39400065  li r10, 0x65
	ctx.r[10].s64 = 101;
	// 8300B9BC: B15F0056  sth r10, 0x56(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(86 as u32), ctx.r[10].u16 ) };
	// 8300B9C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8300B9C4: B15F0058  sth r10, 0x58(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[10].u16 ) };
	// 8300B9C8: 83AB0000  lwz r29, 0(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300B9CC: 4BFD3B75  bl 0x82fdf540
	ctx.lr = 0x8300B9D0;
	sub_82FDF540(ctx, base);
	// 8300B9D0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8300B9D4: 817D0018  lwz r11, 0x18(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 8300B9D8: 807E0024  lwz r3, 0x24(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 8300B9DC: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 8300B9E0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300B9E4: 4E800421  bctrl
	ctx.lr = 0x8300B9E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300B9E8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300B9EC: 41820114  beq 0x8300bb00
	if ctx.cr[0].eq {
	pc = 0x8300BB00; continue 'dispatch;
	}
	// 8300B9F0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300B9F4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300B9F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300B9FC: 4E800421  bctrl
	ctx.lr = 0x8300BA00;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300BA00: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8300BA04: 418200FC  beq 0x8300bb00
	if ctx.cr[0].eq {
	pc = 0x8300BB00; continue 'dispatch;
	}
	// 8300BA08: A17D0000  lhz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300BA0C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300BA10: 418200F0  beq 0x8300bb00
	if ctx.cr[0].eq {
	pc = 0x8300BB00; continue 'dispatch;
	}
	// 8300BA14: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300BA18: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8300BA1C: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300BA20: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300BA24: 4E800421  bctrl
	ctx.lr = 0x8300BA28;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300BA28: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8300BA2C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8300BA30: 80A30090  lwz r5, 0x90(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 8300BA34: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8300BA38: 387F0090  addi r3, r31, 0x90
	ctx.r[3].s64 = ctx.r[31].s64 + 144;
	// 8300BA3C: 4BFD2035  bl 0x82fdda70
	ctx.lr = 0x8300BA40;
	sub_82FDDA70(ctx, base);
	// 8300BA40: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8300BA44: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8300BA48: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300BA4C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8300BA50: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300BA54: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300BA58: 4E800421  bctrl
	ctx.lr = 0x8300BA5C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300BA5C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8300BA60: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8300BA64: 80C30090  lwz r6, 0x90(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 8300BA68: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8300BA6C: 389F0090  addi r4, r31, 0x90
	ctx.r[4].s64 = ctx.r[31].s64 + 144;
	// 8300BA70: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 8300BA74: 4BFD210D  bl 0x82fddb80
	ctx.lr = 0x8300BA78;
	sub_82FDDB80(ctx, base);
	// 8300BA78: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8300BA7C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8300BA80: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 8300BA84: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8300BA88: 409A0014  bne cr6, 0x8300ba9c
	if !ctx.cr[6].eq {
	pc = 0x8300BA9C; continue 'dispatch;
	}
	// 8300BA8C: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 8300BA90: 4BFCF361  bl 0x82fdadf0
	ctx.lr = 0x8300BA94;
	sub_82FDADF0(ctx, base);
	// 8300BA94: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8300BA98: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8300BA9C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300BAA0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8300BAA4: 83BF0084  lwz r29, 0x84(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 8300BAA8: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300BAAC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300BAB0: 4E800421  bctrl
	ctx.lr = 0x8300BAB4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300BAB4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8300BAB8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8300BABC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8300BAC0: 4BFD7141  bl 0x82fe2c00
	ctx.lr = 0x8300BAC4;
	sub_82FE2C00(ctx, base);
	// 8300BAC4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8300BAC8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8300BACC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8300BAD0: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 8300BAD4: 4BFCFD65  bl 0x82fdb838
	ctx.lr = 0x8300BAD8;
	sub_82FDB838(ctx, base);
	// 8300BAD8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8300BADC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8300BAE0: 387F0090  addi r3, r31, 0x90
	ctx.r[3].s64 = ctx.r[31].s64 + 144;
	// 8300BAE4: 4BFCFD55  bl 0x82fdb838
	ctx.lr = 0x8300BAE8;
	sub_82FDB838(ctx, base);
	// 8300BAE8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8300BAEC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8300BAF0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8300BAF4: 48000010  b 0x8300bb04
	pc = 0x8300BB04; continue 'dispatch;
	// 8300BAF8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8300BAFC: 48000008  b 0x8300bb04
	pc = 0x8300BB04; continue 'dispatch;
	// 8300BB00: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8300BB04: 383F00F0  addi r1, r31, 0xf0
	ctx.r[1].s64 = ctx.r[31].s64 + 240;
	// 8300BB08: 4819C6AC  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300BB0C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300BB0C size=8
    let mut pc: u32 = 0x8300BB0C;
    'dispatch: loop {
        match pc {
            0x8300BB0C => {
    //   block [0x8300BB0C..0x8300BB14)
	// 8300BB0C: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8300BB10: 82142124  lwz r16, 0x2124(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(8484 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300BB14(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300BB14 size=24
    let mut pc: u32 = 0x8300BB14;
    'dispatch: loop {
        match pc {
            0x8300BB14 => {
    //   block [0x8300BB14..0x8300BB2C)
	// 8300BB14: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300BB18: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300BB1C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300BB20: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8300BB24: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8300BB28: 481A5101  bl 0x831b0c28
	ctx.lr = 0x8300BB2C;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300BB34(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300BB34 size=20
    let mut pc: u32 = 0x8300BB34;
    'dispatch: loop {
        match pc {
            0x8300BB34 => {
    //   block [0x8300BB34..0x8300BB48)
	// 8300BB34: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300BB38: 3C608301  lis r3, -0x7cff
	ctx.r[3].s64 = -2097086464;
	// 8300BB3C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300BB40: 3863BAF8  addi r3, r3, -0x4508
	ctx.r[3].s64 = ctx.r[3].s64 + -17672;
	// 8300BB44: 4819C670  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300BB48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300BB48 size=40
    let mut pc: u32 = 0x8300BB48;
    'dispatch: loop {
        match pc {
            0x8300BB48 => {
    //   block [0x8300BB48..0x8300BB70)
	// 8300BB48: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 8300BB4C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300BB50: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300BB54: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300BB58: 387F0090  addi r3, r31, 0x90
	ctx.r[3].s64 = ctx.r[31].s64 + 144;
	// 8300BB5C: 4BFCFCDD  bl 0x82fdb838
	ctx.lr = 0x8300BB60;
	sub_82FDB838(ctx, base);
	// 8300BB60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300BB64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300BB68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300BB6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300BB70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300BB70 size=40
    let mut pc: u32 = 0x8300BB70;
    'dispatch: loop {
        match pc {
            0x8300BB70 => {
    //   block [0x8300BB70..0x8300BB98)
	// 8300BB70: 3BECFF10  addi r31, r12, -0xf0
	ctx.r[31].s64 = ctx.r[12].s64 + -240;
	// 8300BB74: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300BB78: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300BB7C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300BB80: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 8300BB84: 4BFCFCB5  bl 0x82fdb838
	ctx.lr = 0x8300BB88;
	sub_82FDB838(ctx, base);
	// 8300BB88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300BB8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300BB90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300BB94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300BB98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300BB98 size=1048
    let mut pc: u32 = 0x8300BB98;
    'dispatch: loop {
        match pc {
            0x8300BB98 => {
    //   block [0x8300BB98..0x8300BFB0)
	// 8300BB98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300BB9C: 4819C5C9  bl 0x831a8164
	ctx.lr = 0x8300BBA0;
	sub_831A8130(ctx, base);
	// 8300BBA0: E981F000  ld r12, -0x1000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-4096 as u32) ) };
	// 8300BBA4: 9421E020  stwu r1, -0x1fe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-8160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300BBA8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8300BBAC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8300BBB0: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8300BBB4: A15F0008  lhz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300BBB8: A16BA698  lhz r11, -0x5968(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(-22888 as u32) ) } as u64;
	// 8300BBBC: 7D4B5838  and r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 8300BBC0: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8300BBC4: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8300BBC8: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 8300BBCC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300BBD0: 41820060  beq 0x8300bc30
	if ctx.cr[0].eq {
	pc = 0x8300BC30; continue 'dispatch;
	}
	// 8300BBD4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300BBD8: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300BBDC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300BBE0: 4E800421  bctrl
	ctx.lr = 0x8300BBE4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300BBE4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300BBE8: 41820020  beq 0x8300bc08
	if ctx.cr[0].eq {
	pc = 0x8300BC08; continue 'dispatch;
	}
	// 8300BBEC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300BBF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300BBF4: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300BBF8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300BBFC: 4E800421  bctrl
	ctx.lr = 0x8300BC00;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300BC00: 80C30090  lwz r6, 0x90(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 8300BC04: 4800000C  b 0x8300bc10
	pc = 0x8300BC10; continue 'dispatch;
	// 8300BC08: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8300BC0C: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 8300BC10: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8300BC14: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 8300BC18: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8300BC1C: 4BFEE2B5  bl 0x82ff9ed0
	ctx.lr = 0x8300BC20;
	sub_82FF9ED0(ctx, base);
	// 8300BC20: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8300BC24: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8300BC28: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 8300BC2C: 481A4FFD  bl 0x831b0c28
	ctx.lr = 0x8300BC30;
	sub_831B0C28(ctx, base);
	// 8300BC30: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 8300BC34: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300BC38: 41820318  beq 0x8300bf50
	if ctx.cr[0].eq {
	pc = 0x8300BF50; continue 'dispatch;
	}
	// 8300BC3C: A16B0000  lhz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300BC40: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300BC44: 4182030C  beq 0x8300bf50
	if ctx.cr[0].eq {
	pc = 0x8300BF50; continue 'dispatch;
	}
	// 8300BC48: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 8300BC4C: 419A02F0  beq cr6, 0x8300bf3c
	if ctx.cr[6].eq {
	pc = 0x8300BF3C; continue 'dispatch;
	}
	// 8300BC50: A17C0000  lhz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300BC54: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300BC58: 418202E4  beq 0x8300bf3c
	if ctx.cr[0].eq {
	pc = 0x8300BF3C; continue 'dispatch;
	}
	// 8300BC5C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300BC60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300BC64: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300BC68: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300BC6C: 4E800421  bctrl
	ctx.lr = 0x8300BC70;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300BC70: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8300BC74: 4BFD59CD  bl 0x82fe1640
	ctx.lr = 0x8300BC78;
	sub_82FE1640(ctx, base);
	// 8300BC78: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8300BC7C: 40820064  bne 0x8300bce0
	if !ctx.cr[0].eq {
	pc = 0x8300BCE0; continue 'dispatch;
	}
	// 8300BC80: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300BC84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300BC88: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300BC8C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300BC90: 4E800421  bctrl
	ctx.lr = 0x8300BC94;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300BC94: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300BC98: 41820020  beq 0x8300bcb8
	if ctx.cr[0].eq {
	pc = 0x8300BCB8; continue 'dispatch;
	}
	// 8300BC9C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300BCA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300BCA4: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300BCA8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300BCAC: 4E800421  bctrl
	ctx.lr = 0x8300BCB0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300BCB0: 80C30090  lwz r6, 0x90(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 8300BCB4: 4800000C  b 0x8300bcc0
	pc = 0x8300BCC0; continue 'dispatch;
	// 8300BCB8: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8300BCBC: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 8300BCC0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8300BCC4: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 8300BCC8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8300BCCC: 4BFEE205  bl 0x82ff9ed0
	ctx.lr = 0x8300BCD0;
	sub_82FF9ED0(ctx, base);
	// 8300BCD0: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8300BCD4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8300BCD8: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 8300BCDC: 481A4F4D  bl 0x831b0c28
	ctx.lr = 0x8300BCE0;
	sub_831B0C28(ctx, base);
	// 8300BCE0: 4BFD3851  bl 0x82fdf530
	ctx.lr = 0x8300BCE4;
	sub_82FDF530(ctx, base);
	// 8300BCE4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8300BCE8: 4BFD3859  bl 0x82fdf540
	ctx.lr = 0x8300BCEC;
	sub_82FDF540(ctx, base);
	// 8300BCEC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8300BCF0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8300BCF4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8300BCF8: 4BFC7F49  bl 0x82fd3c40
	ctx.lr = 0x8300BCFC;
	sub_82FD3C40(ctx, base);
	// 8300BCFC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8300BD00: 41820078  beq 0x8300bd78
	if ctx.cr[0].eq {
	pc = 0x8300BD78; continue 'dispatch;
	}
	// 8300BD04: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8300BD08: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 8300BD0C: 4BFC7F35  bl 0x82fd3c40
	ctx.lr = 0x8300BD10;
	sub_82FD3C40(ctx, base);
	// 8300BD10: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8300BD14: 40820064  bne 0x8300bd78
	if !ctx.cr[0].eq {
	pc = 0x8300BD78; continue 'dispatch;
	}
	// 8300BD18: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300BD1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300BD20: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300BD24: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300BD28: 4E800421  bctrl
	ctx.lr = 0x8300BD2C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300BD2C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300BD30: 41820020  beq 0x8300bd50
	if ctx.cr[0].eq {
	pc = 0x8300BD50; continue 'dispatch;
	}
	// 8300BD34: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300BD38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300BD3C: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300BD40: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300BD44: 4E800421  bctrl
	ctx.lr = 0x8300BD48;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300BD48: 80C30090  lwz r6, 0x90(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 8300BD4C: 4800000C  b 0x8300bd58
	pc = 0x8300BD58; continue 'dispatch;
	// 8300BD50: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8300BD54: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 8300BD58: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8300BD5C: 3880000E  li r4, 0xe
	ctx.r[4].s64 = 14;
	// 8300BD60: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8300BD64: 4BFEE16D  bl 0x82ff9ed0
	ctx.lr = 0x8300BD68;
	sub_82FF9ED0(ctx, base);
	// 8300BD68: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8300BD6C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8300BD70: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 8300BD74: 481A4EB5  bl 0x831b0c28
	ctx.lr = 0x8300BD78;
	sub_831B0C28(ctx, base);
	// 8300BD78: 3880003A  li r4, 0x3a
	ctx.r[4].s64 = 58;
	// 8300BD7C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8300BD80: 4BFC6031  bl 0x82fd1db0
	ctx.lr = 0x8300BD84;
	sub_82FD1DB0(ctx, base);
	// 8300BD84: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300BD88: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 8300BD8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300BD90: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300BD94: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300BD98: 419A0054  beq cr6, 0x8300bdec
	if ctx.cr[6].eq {
	pc = 0x8300BDEC; continue 'dispatch;
	}
	// 8300BD9C: 4E800421  bctrl
	ctx.lr = 0x8300BDA0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300BDA0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300BDA4: 41820020  beq 0x8300bdc4
	if ctx.cr[0].eq {
	pc = 0x8300BDC4; continue 'dispatch;
	}
	// 8300BDA8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300BDAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300BDB0: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300BDB4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300BDB8: 4E800421  bctrl
	ctx.lr = 0x8300BDBC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300BDBC: 80C30090  lwz r6, 0x90(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 8300BDC0: 4800000C  b 0x8300bdcc
	pc = 0x8300BDCC; continue 'dispatch;
	// 8300BDC4: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8300BDC8: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 8300BDCC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8300BDD0: 3880000E  li r4, 0xe
	ctx.r[4].s64 = 14;
	// 8300BDD4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8300BDD8: 4BFEE0F9  bl 0x82ff9ed0
	ctx.lr = 0x8300BDDC;
	sub_82FF9ED0(ctx, base);
	// 8300BDDC: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8300BDE0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8300BDE4: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 8300BDE8: 481A4E41  bl 0x831b0c28
	ctx.lr = 0x8300BDEC;
	sub_831B0C28(ctx, base);
	// 8300BDEC: 4E800421  bctrl
	ctx.lr = 0x8300BDF0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300BDF0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8300BDF4: 4BFD5BE5  bl 0x82fe19d8
	ctx.lr = 0x8300BDF8;
	sub_82FE19D8(ctx, base);
	// 8300BDF8: 907F0038  stw r3, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[3].u32 ) };
	// 8300BDFC: A17C0000  lhz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300BE00: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300BE04: 41820028  beq 0x8300be2c
	if ctx.cr[0].eq {
	pc = 0x8300BE2C; continue 'dispatch;
	}
	// 8300BE08: 397C0002  addi r11, r28, 2
	ctx.r[11].s64 = ctx.r[28].s64 + 2;
	// 8300BE0C: 48000008  b 0x8300be14
	pc = 0x8300BE14; continue 'dispatch;
	// 8300BE10: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 8300BE14: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300BE18: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300BE1C: 4082FFF4  bne 0x8300be10
	if !ctx.cr[0].eq {
	pc = 0x8300BE10; continue 'dispatch;
	}
	// 8300BE20: 7D7C5850  subf r11, r28, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[28].s64;
	// 8300BE24: 7D7B0E70  srawi r27, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[27].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 8300BE28: 48000008  b 0x8300be30
	pc = 0x8300BE30; continue 'dispatch;
	// 8300BE2C: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8300BE30: 815F0034  lwz r10, 0x34(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 8300BE34: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300BE38: 41820034  beq 0x8300be6c
	if ctx.cr[0].eq {
	pc = 0x8300BE6C; continue 'dispatch;
	}
	// 8300BE3C: A16A0000  lhz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300BE40: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300BE44: 41820028  beq 0x8300be6c
	if ctx.cr[0].eq {
	pc = 0x8300BE6C; continue 'dispatch;
	}
	// 8300BE48: 396A0002  addi r11, r10, 2
	ctx.r[11].s64 = ctx.r[10].s64 + 2;
	// 8300BE4C: 48000008  b 0x8300be54
	pc = 0x8300BE54; continue 'dispatch;
	// 8300BE50: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 8300BE54: A12B0000  lhz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300BE58: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300BE5C: 4082FFF4  bne 0x8300be50
	if !ctx.cr[0].eq {
	pc = 0x8300BE50; continue 'dispatch;
	}
	// 8300BE60: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8300BE64: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 8300BE68: 48000008  b 0x8300be70
	pc = 0x8300BE70; continue 'dispatch;
	// 8300BE6C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8300BE70: 7D6BDA14  add r11, r11, r27
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 8300BE74: 3BAB0001  addi r29, r11, 1
	ctx.r[29].s64 = ctx.r[11].s64 + 1;
	// 8300BE78: 2F1D0F9F  cmpwi cr6, r29, 0xf9f
	ctx.cr[6].compare_i32(ctx.r[29].s32, 3999, &mut ctx.xer);
	// 8300BE7C: 41980038  blt cr6, 0x8300beb4
	if ctx.cr[6].lt {
	pc = 0x8300BEB4; continue 'dispatch;
	}
	// 8300BE80: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300BE84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300BE88: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300BE8C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300BE90: 4E800421  bctrl
	ctx.lr = 0x8300BE94;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300BE94: 80630090  lwz r3, 0x90(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 8300BE98: 57A4083C  slwi r4, r29, 1
	ctx.r[4].u32 = ctx.r[29].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 8300BE9C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300BEA0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8300BEA4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300BEA8: 4E800421  bctrl
	ctx.lr = 0x8300BEAC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300BEAC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8300BEB0: 48000008  b 0x8300beb8
	pc = 0x8300BEB8; continue 'dispatch;
	// 8300BEB4: 3BC10070  addi r30, r1, 0x70
	ctx.r[30].s64 = ctx.r[1].s64 + 112;
	// 8300BEB8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8300BEBC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8300BEC0: 4BFC5CA9  bl 0x82fd1b68
	ctx.lr = 0x8300BEC4;
	sub_82FD1B68(ctx, base);
	// 8300BEC4: 576B083C  slwi r11, r27, 1
	ctx.r[11].u32 = ctx.r[27].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8300BEC8: 3940003A  li r10, 0x3a
	ctx.r[10].s64 = 58;
	// 8300BECC: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 8300BED0: 386B0002  addi r3, r11, 2
	ctx.r[3].s64 = ctx.r[11].s64 + 2;
	// 8300BED4: B14B0000  sth r10, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 8300BED8: 809F0034  lwz r4, 0x34(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 8300BEDC: 4BFC5C8D  bl 0x82fd1b68
	ctx.lr = 0x8300BEE0;
	sub_82FD1B68(ctx, base);
	// 8300BEE0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300BEE4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300BEE8: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300BEEC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300BEF0: 4E800421  bctrl
	ctx.lr = 0x8300BEF4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300BEF4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8300BEF8: 4BFD5AE1  bl 0x82fe19d8
	ctx.lr = 0x8300BEFC;
	sub_82FE19D8(ctx, base);
	// 8300BEFC: 2F1D0F9F  cmpwi cr6, r29, 0xf9f
	ctx.cr[6].compare_i32(ctx.r[29].s32, 3999, &mut ctx.xer);
	// 8300BF00: 907F002C  stw r3, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[3].u32 ) };
	// 8300BF04: 41980030  blt cr6, 0x8300bf34
	if ctx.cr[6].lt {
	pc = 0x8300BF34; continue 'dispatch;
	}
	// 8300BF08: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300BF0C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300BF10: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300BF14: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300BF18: 4E800421  bctrl
	ctx.lr = 0x8300BF1C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300BF1C: 80630090  lwz r3, 0x90(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 8300BF20: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8300BF24: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300BF28: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300BF2C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300BF30: 4E800421  bctrl
	ctx.lr = 0x8300BF34;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300BF34: 38211FE0  addi r1, r1, 0x1fe0
	ctx.r[1].s64 = ctx.r[1].s64 + 8160;
	// 8300BF38: 4819C27C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 8300BF3C: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 8300BF40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8300BF44: 915F0038  stw r10, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[10].u32 ) };
	// 8300BF48: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 8300BF4C: 4BFFFFE8  b 0x8300bf34
	pc = 0x8300BF34; continue 'dispatch;
	// 8300BF50: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300BF54: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300BF58: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300BF5C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300BF60: 4E800421  bctrl
	ctx.lr = 0x8300BF64;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300BF64: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300BF68: 41820020  beq 0x8300bf88
	if ctx.cr[0].eq {
	pc = 0x8300BF88; continue 'dispatch;
	}
	// 8300BF6C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300BF70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300BF74: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300BF78: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300BF7C: 4E800421  bctrl
	ctx.lr = 0x8300BF80;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300BF80: 80C30090  lwz r6, 0x90(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 8300BF84: 4800000C  b 0x8300bf90
	pc = 0x8300BF90; continue 'dispatch;
	// 8300BF88: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8300BF8C: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 8300BF90: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8300BF94: 3880000E  li r4, 0xe
	ctx.r[4].s64 = 14;
	// 8300BF98: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8300BF9C: 4BFEDF35  bl 0x82ff9ed0
	ctx.lr = 0x8300BFA0;
	sub_82FF9ED0(ctx, base);
	// 8300BFA0: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8300BFA4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8300BFA8: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 8300BFAC: 481A4C7D  bl 0x831b0c28
	ctx.lr = 0x8300BFB0;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300BFB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300BFB0 size=376
    let mut pc: u32 = 0x8300BFB0;
    'dispatch: loop {
        match pc {
            0x8300BFB0 => {
    //   block [0x8300BFB0..0x8300C128)
	// 8300BFB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300BFB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300BFB8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8300BFBC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8300BFC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300BFC4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8300BFC8: 3D408214  lis r10, -0x7dec
	ctx.r[10].s64 = -2112618496;
	// 8300BFCC: A17F0008  lhz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300BFD0: A14AA6A4  lhz r10, -0x595c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(-22876 as u32) ) } as u64;
	// 8300BFD4: 7D4A5838  and r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 8300BFD8: 7D4A0034  cntlzw r10, r10
	ctx.r[10].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 8300BFDC: 554ADFFE  rlwinm r10, r10, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 8300BFE0: 694A0001  xori r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u64 ^ 1;
	// 8300BFE4: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300BFE8: 41820080  beq 0x8300c068
	if ctx.cr[0].eq {
	pc = 0x8300C068; continue 'dispatch;
	}
	// 8300BFEC: 3D408214  lis r10, -0x7dec
	ctx.r[10].s64 = -2112618496;
	// 8300BFF0: A14AA6C8  lhz r10, -0x5938(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(-22840 as u32) ) } as u64;
	// 8300BFF4: 7D4B5838  and r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 8300BFF8: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8300BFFC: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8300C000: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 8300C004: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300C008: 40820060  bne 0x8300c068
	if !ctx.cr[0].eq {
	pc = 0x8300C068; continue 'dispatch;
	}
	// 8300C00C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300C010: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300C014: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300C018: 4E800421  bctrl
	ctx.lr = 0x8300C01C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300C01C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300C020: 41820020  beq 0x8300c040
	if ctx.cr[0].eq {
	pc = 0x8300C040; continue 'dispatch;
	}
	// 8300C024: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300C028: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300C02C: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300C030: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300C034: 4E800421  bctrl
	ctx.lr = 0x8300C038;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300C038: 80C30090  lwz r6, 0x90(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 8300C03C: 4800000C  b 0x8300c048
	pc = 0x8300C048; continue 'dispatch;
	// 8300C040: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8300C044: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 8300C048: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8300C04C: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 8300C050: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8300C054: 4BFEDE7D  bl 0x82ff9ed0
	ctx.lr = 0x8300C058;
	sub_82FF9ED0(ctx, base);
	// 8300C058: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8300C05C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8300C060: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 8300C064: 481A4BC5  bl 0x831b0c28
	ctx.lr = 0x8300C068;
	sub_831B0C28(ctx, base);
	// 8300C068: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300C06C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300C070: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300C074: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300C078: 4E800421  bctrl
	ctx.lr = 0x8300C07C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300C07C: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8300C080: 41820048  beq 0x8300c0c8
	if ctx.cr[0].eq {
	pc = 0x8300C0C8; continue 'dispatch;
	}
	// 8300C084: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8300C088: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8300C08C: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 8300C090: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 8300C094: 4BFD36E5  bl 0x82fdf778
	ctx.lr = 0x8300C098;
	sub_82FDF778(ctx, base);
	// 8300C098: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 8300C09C: 4BFF05ED  bl 0x82ffc688
	ctx.lr = 0x8300C0A0;
	sub_82FFC688(ctx, base);
	// 8300C0A0: 38A00007  li r5, 7
	ctx.r[5].s64 = 7;
	// 8300C0A4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8300C0A8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8300C0AC: 4BFD9635  bl 0x82fe56e0
	ctx.lr = 0x8300C0B0;
	sub_82FE56E0(ctx, base);
	// 8300C0B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8300C0B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300C0B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300C0BC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8300C0C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8300C0C4: 4E800020  blr
	return;
	// 8300C0C8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300C0CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300C0D0: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300C0D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300C0D8: 4E800421  bctrl
	ctx.lr = 0x8300C0DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300C0DC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300C0E0: 41820020  beq 0x8300c100
	if ctx.cr[0].eq {
	pc = 0x8300C100; continue 'dispatch;
	}
	// 8300C0E4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300C0E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300C0EC: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300C0F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300C0F4: 4E800421  bctrl
	ctx.lr = 0x8300C0F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300C0F8: 80C30090  lwz r6, 0x90(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 8300C0FC: 4800000C  b 0x8300c108
	pc = 0x8300C108; continue 'dispatch;
	// 8300C100: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8300C104: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 8300C108: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8300C10C: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 8300C110: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8300C114: 4BFEDDBD  bl 0x82ff9ed0
	ctx.lr = 0x8300C118;
	sub_82FF9ED0(ctx, base);
	// 8300C118: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8300C11C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8300C120: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 8300C124: 481A4B05  bl 0x831b0c28
	ctx.lr = 0x8300C128;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300C128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300C128 size=668
    let mut pc: u32 = 0x8300C128;
    'dispatch: loop {
        match pc {
            0x8300C128 => {
    //   block [0x8300C128..0x8300C3C4)
	// 8300C128: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300C12C: 4819C035  bl 0x831a8160
	ctx.lr = 0x8300C130;
	sub_831A8130(ctx, base);
	// 8300C130: E981F000  ld r12, -0x1000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-4096 as u32) ) };
	// 8300C134: 9421E010  stwu r1, -0x1ff0(r1)
	ea = ctx.r[1].u32.wrapping_add(-8176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300C138: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8300C13C: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 8300C140: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8300C144: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300C148: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300C14C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300C150: 4E800421  bctrl
	ctx.lr = 0x8300C154;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300C154: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8300C158: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8300C15C: 4BFD587D  bl 0x82fe19d8
	ctx.lr = 0x8300C160;
	sub_82FE19D8(ctx, base);
	// 8300C160: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8300C164: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8300C168: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 8300C16C: 4BFD56A5  bl 0x82fe1810
	ctx.lr = 0x8300C170;
	sub_82FE1810(ctx, base);
	// 8300C170: 7C7C1B79  or. r28, r3, r3
	ctx.r[28].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 8300C174: 40800064  bge 0x8300c1d8
	if !ctx.cr[0].lt {
	pc = 0x8300C1D8; continue 'dispatch;
	}
	// 8300C178: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300C17C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300C180: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300C184: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300C188: 4E800421  bctrl
	ctx.lr = 0x8300C18C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300C18C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300C190: 41820020  beq 0x8300c1b0
	if ctx.cr[0].eq {
	pc = 0x8300C1B0; continue 'dispatch;
	}
	// 8300C194: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300C198: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300C19C: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300C1A0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300C1A4: 4E800421  bctrl
	ctx.lr = 0x8300C1A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300C1A8: 80C30090  lwz r6, 0x90(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 8300C1AC: 4800000C  b 0x8300c1b8
	pc = 0x8300C1B8; continue 'dispatch;
	// 8300C1B0: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8300C1B4: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 8300C1B8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8300C1BC: 3880000E  li r4, 0xe
	ctx.r[4].s64 = 14;
	// 8300C1C0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8300C1C4: 4BFEDD0D  bl 0x82ff9ed0
	ctx.lr = 0x8300C1C8;
	sub_82FF9ED0(ctx, base);
	// 8300C1C8: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8300C1CC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8300C1D0: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 8300C1D4: 481A4A55  bl 0x831b0c28
	ctx.lr = 0x8300C1D8;
	sub_831B0C28(ctx, base);
	// 8300C1D8: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 8300C1DC: 409A0018  bne cr6, 0x8300c1f4
	if !ctx.cr[6].eq {
	pc = 0x8300C1F4; continue 'dispatch;
	}
	// 8300C1E0: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300C1E4: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 8300C1E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8300C1EC: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 8300C1F0: 48000128  b 0x8300c318
	pc = 0x8300C318; continue 'dispatch;
	// 8300C1F4: 2F1C0F9F  cmpwi cr6, r28, 0xf9f
	ctx.cr[6].compare_i32(ctx.r[28].s32, 3999, &mut ctx.xer);
	// 8300C1F8: 41980064  blt cr6, 0x8300c25c
	if ctx.cr[6].lt {
	pc = 0x8300C25C; continue 'dispatch;
	}
	// 8300C1FC: 807B0090  lwz r3, 0x90(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(144 as u32) ) } as u64;
	// 8300C200: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8300C204: 419A0034  beq cr6, 0x8300c238
	if ctx.cr[6].eq {
	pc = 0x8300C238; continue 'dispatch;
	}
	// 8300C208: A17E0000  lhz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300C20C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300C210: 41820028  beq 0x8300c238
	if ctx.cr[0].eq {
	pc = 0x8300C238; continue 'dispatch;
	}
	// 8300C214: 397E0002  addi r11, r30, 2
	ctx.r[11].s64 = ctx.r[30].s64 + 2;
	// 8300C218: 48000008  b 0x8300c220
	pc = 0x8300C220; continue 'dispatch;
	// 8300C21C: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 8300C220: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300C224: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300C228: 4082FFF4  bne 0x8300c21c
	if !ctx.cr[0].eq {
	pc = 0x8300C21C; continue 'dispatch;
	}
	// 8300C22C: 7D7E5850  subf r11, r30, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	// 8300C230: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 8300C234: 48000008  b 0x8300c23c
	pc = 0x8300C23C; continue 'dispatch;
	// 8300C238: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8300C23C: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300C240: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8300C244: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 8300C248: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8300C24C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300C250: 4E800421  bctrl
	ctx.lr = 0x8300C254;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300C254: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8300C258: 48000008  b 0x8300c260
	pc = 0x8300C260; continue 'dispatch;
	// 8300C25C: 3BC10070  addi r30, r1, 0x70
	ctx.r[30].s64 = ctx.r[1].s64 + 112;
	// 8300C260: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8300C264: 809F002C  lwz r4, 0x2c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300C268: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8300C26C: 4BFC593D  bl 0x82fd1ba8
	ctx.lr = 0x8300C270;
	sub_82FD1BA8(ctx, base);
	// 8300C270: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8300C274: 579D083C  slwi r29, r28, 1
	ctx.r[29].u32 = ctx.r[28].u32.wrapping_shl(1);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 8300C278: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8300C27C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8300C280: 7D7DF32E  sthx r11, r29, r30
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[29].u32.wrapping_add(ctx.r[30].u32), ctx.r[11].u16) };
	// 8300C284: 4BFD5755  bl 0x82fe19d8
	ctx.lr = 0x8300C288;
	sub_82FE19D8(ctx, base);
	// 8300C288: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300C28C: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8300C290: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8300C294: 7D7D5A14  add r11, r29, r11
	ctx.r[11].u64 = ctx.r[29].u64 + ctx.r[11].u64;
	// 8300C298: 388B0002  addi r4, r11, 2
	ctx.r[4].s64 = ctx.r[11].s64 + 2;
	// 8300C29C: 915F0038  stw r10, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[10].u32 ) };
	// 8300C2A0: 4BFD5739  bl 0x82fe19d8
	ctx.lr = 0x8300C2A4;
	sub_82FE19D8(ctx, base);
	// 8300C2A4: 2F1C0F9F  cmpwi cr6, r28, 0xf9f
	ctx.cr[6].compare_i32(ctx.r[28].s32, 3999, &mut ctx.xer);
	// 8300C2A8: 907F0034  stw r3, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[3].u32 ) };
	// 8300C2AC: 4198001C  blt cr6, 0x8300c2c8
	if ctx.cr[6].lt {
	pc = 0x8300C2C8; continue 'dispatch;
	}
	// 8300C2B0: 807B0090  lwz r3, 0x90(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(144 as u32) ) } as u64;
	// 8300C2B4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8300C2B8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300C2BC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300C2C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300C2C4: 4E800421  bctrl
	ctx.lr = 0x8300C2C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300C2C8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300C2CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300C2D0: 83DF0038  lwz r30, 0x38(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 8300C2D4: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300C2D8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300C2DC: 4E800421  bctrl
	ctx.lr = 0x8300C2E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300C2E0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8300C2E4: 4BFD535D  bl 0x82fe1640
	ctx.lr = 0x8300C2E8;
	sub_82FE1640(ctx, base);
	// 8300C2E8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8300C2EC: 41820078  beq 0x8300c364
	if ctx.cr[0].eq {
	pc = 0x8300C364; continue 'dispatch;
	}
	// 8300C2F0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300C2F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300C2F8: 83DF0034  lwz r30, 0x34(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 8300C2FC: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300C300: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300C304: 4E800421  bctrl
	ctx.lr = 0x8300C308;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300C308: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8300C30C: 4BFD5335  bl 0x82fe1640
	ctx.lr = 0x8300C310;
	sub_82FE1640(ctx, base);
	// 8300C310: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8300C314: 41820050  beq 0x8300c364
	if ctx.cr[0].eq {
	pc = 0x8300C364; continue 'dispatch;
	}
	// 8300C318: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 8300C31C: 419A0014  beq cr6, 0x8300c330
	if ctx.cr[6].eq {
	pc = 0x8300C330; continue 'dispatch;
	}
	// 8300C320: A17A0000  lhz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300C324: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8300C328: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300C32C: 40820008  bne 0x8300c334
	if !ctx.cr[0].eq {
	pc = 0x8300C334; continue 'dispatch;
	}
	// 8300C330: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8300C334: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8300C338: 807F0038  lwz r3, 0x38(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 8300C33C: 4BFD3235  bl 0x82fdf570
	ctx.lr = 0x8300C340;
	sub_82FDF570(ctx, base);
	// 8300C340: 7C641B79  or. r4, r3, r3
	ctx.r[4].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8300C344: 4082000C  bne 0x8300c350
	if !ctx.cr[0].eq {
	pc = 0x8300C350; continue 'dispatch;
	}
	// 8300C348: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8300C34C: 4800000C  b 0x8300c358
	pc = 0x8300C358; continue 'dispatch;
	// 8300C350: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8300C354: 4BFD5685  bl 0x82fe19d8
	ctx.lr = 0x8300C358;
	sub_82FE19D8(ctx, base);
	// 8300C358: 907F0030  stw r3, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[3].u32 ) };
	// 8300C35C: 38211FF0  addi r1, r1, 0x1ff0
	ctx.r[1].s64 = ctx.r[1].s64 + 8176;
	// 8300C360: 4819BE50  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
	// 8300C364: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300C368: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300C36C: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300C370: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300C374: 4E800421  bctrl
	ctx.lr = 0x8300C378;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300C378: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300C37C: 41820020  beq 0x8300c39c
	if ctx.cr[0].eq {
	pc = 0x8300C39C; continue 'dispatch;
	}
	// 8300C380: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300C384: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300C388: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300C38C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300C390: 4E800421  bctrl
	ctx.lr = 0x8300C394;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300C394: 80C30090  lwz r6, 0x90(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 8300C398: 4800000C  b 0x8300c3a4
	pc = 0x8300C3A4; continue 'dispatch;
	// 8300C39C: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8300C3A0: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 8300C3A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8300C3A8: 3880000E  li r4, 0xe
	ctx.r[4].s64 = 14;
	// 8300C3AC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8300C3B0: 4BFEDB21  bl 0x82ff9ed0
	ctx.lr = 0x8300C3B4;
	sub_82FF9ED0(ctx, base);
	// 8300C3B4: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8300C3B8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8300C3BC: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 8300C3C0: 481A4869  bl 0x831b0c28
	ctx.lr = 0x8300C3C4;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300C3C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300C3C8 size=12
    let mut pc: u32 = 0x8300C3C8;
    'dispatch: loop {
        match pc {
            0x8300C3C8 => {
    //   block [0x8300C3C8..0x8300C3D4)
	// 8300C3C8: 8063003C  lwz r3, 0x3c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(60 as u32) ) } as u64;
	// 8300C3CC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300C3D0: 4C820020  bnelr
	if !ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300C3D4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300C3D4 size=12
    let mut pc: u32 = 0x8300C3D4;
    'dispatch: loop {
        match pc {
            0x8300C3D4 => {
    //   block [0x8300C3D4..0x8300C3E0)
	// 8300C3D4: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8300C3D8: 386B2F60  addi r3, r11, 0x2f60
	ctx.r[3].s64 = ctx.r[11].s64 + 12128;
	// 8300C3DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300C3E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300C3E0 size=112
    let mut pc: u32 = 0x8300C3E0;
    'dispatch: loop {
        match pc {
            0x8300C3E0 => {
    //   block [0x8300C3E0..0x8300C450)
	// 8300C3E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300C3E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300C3E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8300C3EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8300C3F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300C3F4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8300C3F8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8300C3FC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8300C400: 388B9468  addi r4, r11, -0x6b98
	ctx.r[4].s64 = ctx.r[11].s64 + -27544;
	// 8300C404: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300C408: 4BFC7839  bl 0x82fd3c40
	ctx.lr = 0x8300C40C;
	sub_82FD3C40(ctx, base);
	// 8300C40C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8300C410: 4182001C  beq 0x8300c42c
	if ctx.cr[0].eq {
	pc = 0x8300C42C; continue 'dispatch;
	}
	// 8300C414: 817E003C  lwz r11, 0x3c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(60 as u32) ) } as u64;
	// 8300C418: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300C41C: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 8300C420: 40820018  bne 0x8300c438
	if !ctx.cr[0].eq {
	pc = 0x8300C438; continue 'dispatch;
	}
	// 8300C424: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8300C428: 48000010  b 0x8300c438
	pc = 0x8300C438; continue 'dispatch;
	// 8300C42C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8300C430: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8300C434: 4BFFD7D5  bl 0x83009c08
	ctx.lr = 0x8300C438;
	sub_83009C08(ctx, base);
	// 8300C438: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8300C43C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300C440: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300C444: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8300C448: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8300C44C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300C450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300C450 size=8
    let mut pc: u32 = 0x8300C450;
    'dispatch: loop {
        match pc {
            0x8300C450 => {
    //   block [0x8300C450..0x8300C458)
	// 8300C450: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8300C454: 821421E8  lwz r16, 0x21e8(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(8680 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300C458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300C458 size=88
    let mut pc: u32 = 0x8300C458;
    'dispatch: loop {
        match pc {
            0x8300C458 => {
    //   block [0x8300C458..0x8300C4B0)
	// 8300C458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300C45C: 4819BD0D  bl 0x831a8168
	ctx.lr = 0x8300C460;
	sub_831A8130(ctx, base);
	// 8300C460: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 8300C464: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300C468: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8300C46C: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 8300C470: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8300C474: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8300C478: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 8300C47C: 4BFFC05D  bl 0x830084d8
	ctx.lr = 0x8300C480;
	sub_830084D8(ctx, base);
	// 8300C480: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8300C484: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8300C488: 396B1F80  addi r11, r11, 0x1f80
	ctx.r[11].s64 = ctx.r[11].s64 + 8064;
	// 8300C48C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8300C490: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8300C494: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8300C498: 4BFFFC91  bl 0x8300c128
	ctx.lr = 0x8300C49C;
	sub_8300C128(ctx, base);
	// 8300C49C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8300C4A0: 917E003C  stw r11, 0x3c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 8300C4A4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8300C4A8: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 8300C4AC: 4819BD0C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300C4B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300C4B0 size=40
    let mut pc: u32 = 0x8300C4B0;
    'dispatch: loop {
        match pc {
            0x8300C4B0 => {
    //   block [0x8300C4B0..0x8300C4D8)
	// 8300C4B0: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 8300C4B4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300C4B8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300C4BC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300C4C0: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 8300C4C4: 4BFFA3FD  bl 0x830068c0
	ctx.lr = 0x8300C4C8;
	sub_830068C0(ctx, base);
	// 8300C4C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300C4CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300C4D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300C4D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300C4D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300C4D8 size=80
    let mut pc: u32 = 0x8300C4D8;
    'dispatch: loop {
        match pc {
            0x8300C4D8 => {
    //   block [0x8300C4D8..0x8300C528)
	// 8300C4D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300C4DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300C4E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8300C4E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300C4E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8300C4EC: 4BFFFC3D  bl 0x8300c128
	ctx.lr = 0x8300C4F0;
	sub_8300C128(ctx, base);
	// 8300C4F0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300C4F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300C4F8: 816B00F8  lwz r11, 0xf8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(248 as u32) ) } as u64;
	// 8300C4FC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300C500: 4E800421  bctrl
	ctx.lr = 0x8300C504;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300C504: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8300C508: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 8300C50C: 4802E6ED  bl 0x8303abf8
	ctx.lr = 0x8300C510;
	sub_8303ABF8(ctx, base);
	// 8300C510: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300C514: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300C518: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300C51C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300C520: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8300C524: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300C528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300C528 size=96
    let mut pc: u32 = 0x8300C528;
    'dispatch: loop {
        match pc {
            0x8300C528 => {
    //   block [0x8300C528..0x8300C588)
	// 8300C528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300C52C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300C530: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8300C534: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8300C538: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300C53C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8300C540: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8300C544: 4BFF88DD  bl 0x83004e20
	ctx.lr = 0x8300C548;
	sub_83004E20(ctx, base);
	// 8300C548: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8300C54C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300C550: 396B2218  addi r11, r11, 0x2218
	ctx.r[11].s64 = ctx.r[11].s64 + 8728;
	// 8300C554: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8300C558: 817E0024  lwz r11, 0x24(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 8300C55C: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 8300C560: 817E0028  lwz r11, 0x28(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 8300C564: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 8300C568: 817E002C  lwz r11, 0x2c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300C56C: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 8300C570: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8300C574: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300C578: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300C57C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8300C580: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8300C584: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300C588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300C588 size=8
    let mut pc: u32 = 0x8300C588;
    'dispatch: loop {
        match pc {
            0x8300C588 => {
    //   block [0x8300C588..0x8300C590)
	// 8300C588: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8300C58C: 821422F0  lwz r16, 0x22f0(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(8944 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300C590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300C590 size=124
    let mut pc: u32 = 0x8300C590;
    'dispatch: loop {
        match pc {
            0x8300C590 => {
    //   block [0x8300C590..0x8300C60C)
	// 8300C590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300C594: 4819BBD9  bl 0x831a816c
	ctx.lr = 0x8300C598;
	sub_831A8130(ctx, base);
	// 8300C598: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 8300C59C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300C5A0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8300C5A4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8300C5A8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300C5AC: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300C5B0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300C5B4: 4E800421  bctrl
	ctx.lr = 0x8300C5B8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300C5B8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8300C5BC: 38800030  li r4, 0x30
	ctx.r[4].s64 = 48;
	// 8300C5C0: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8300C5C4: 4BFD82F5  bl 0x82fe48b8
	ctx.lr = 0x8300C5C8;
	sub_82FE48B8(ctx, base);
	// 8300C5C8: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 8300C5CC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300C5D0: 41820018  beq 0x8300c5e8
	if ctx.cr[0].eq {
	pc = 0x8300C5E8; continue 'dispatch;
	}
	// 8300C5D4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8300C5D8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8300C5DC: 4BFFFF4D  bl 0x8300c528
	ctx.lr = 0x8300C5E0;
	sub_8300C528(ctx, base);
	// 8300C5E0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8300C5E4: 48000008  b 0x8300c5ec
	pc = 0x8300C5EC; continue 'dispatch;
	// 8300C5E8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8300C5EC: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 8300C5F0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8300C5F4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8300C5F8: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 8300C5FC: 4BFD317D  bl 0x82fdf778
	ctx.lr = 0x8300C600;
	sub_82FDF778(ctx, base);
	// 8300C600: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8300C604: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 8300C608: 4819BBB4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300C60C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300C60C size=48
    let mut pc: u32 = 0x8300C60C;
    'dispatch: loop {
        match pc {
            0x8300C60C => {
    //   block [0x8300C60C..0x8300C63C)
	// 8300C60C: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 8300C610: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300C614: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300C618: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300C61C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8300C620: 809F0050  lwz r4, 0x50(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8300C624: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8300C628: 480BB4B9  bl 0x830c7ae0
	ctx.lr = 0x8300C62C;
	sub_830C7AE0(ctx, base);
	// 8300C62C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300C630: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300C634: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300C638: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300C640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300C640 size=1128
    let mut pc: u32 = 0x8300C640;
    'dispatch: loop {
        match pc {
            0x8300C640 => {
    //   block [0x8300C640..0x8300CAA8)
	// 8300C640: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300C644: 4819BB1D  bl 0x831a8160
	ctx.lr = 0x8300C648;
	sub_831A8130(ctx, base);
	// 8300C648: E981F000  ld r12, -0x1000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-4096 as u32) ) };
	// 8300C64C: 9421E010  stwu r1, -0x1ff0(r1)
	ea = ctx.r[1].u32.wrapping_add(-8176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300C650: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8300C654: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 8300C658: 4BFD2EF9  bl 0x82fdf550
	ctx.lr = 0x8300C65C;
	sub_82FDF550(ctx, base);
	// 8300C65C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8300C660: A15F0008  lhz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300C664: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 8300C668: A16BA698  lhz r11, -0x5968(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(-22888 as u32) ) } as u64;
	// 8300C66C: 7D4B5838  and r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 8300C670: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8300C674: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8300C678: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 8300C67C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300C680: 41820064  beq 0x8300c6e4
	if ctx.cr[0].eq {
	pc = 0x8300C6E4; continue 'dispatch;
	}
	// 8300C684: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300C688: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300C68C: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300C690: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300C694: 4E800421  bctrl
	ctx.lr = 0x8300C698;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300C698: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300C69C: 41820020  beq 0x8300c6bc
	if ctx.cr[0].eq {
	pc = 0x8300C6BC; continue 'dispatch;
	}
	// 8300C6A0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300C6A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300C6A8: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300C6AC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300C6B0: 4E800421  bctrl
	ctx.lr = 0x8300C6B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300C6B4: 80C30090  lwz r6, 0x90(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 8300C6B8: 4800000C  b 0x8300c6c4
	pc = 0x8300C6C4; continue 'dispatch;
	// 8300C6BC: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8300C6C0: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 8300C6C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8300C6C8: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 8300C6CC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8300C6D0: 4BFED801  bl 0x82ff9ed0
	ctx.lr = 0x8300C6D4;
	sub_82FF9ED0(ctx, base);
	// 8300C6D4: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8300C6D8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8300C6DC: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 8300C6E0: 481A4549  bl 0x831b0c28
	ctx.lr = 0x8300C6E4;
	sub_831B0C28(ctx, base);
	// 8300C6E4: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 8300C6E8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300C6EC: 4182035C  beq 0x8300ca48
	if ctx.cr[0].eq {
	pc = 0x8300CA48; continue 'dispatch;
	}
	// 8300C6F0: A16B0000  lhz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300C6F4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300C6F8: 41820350  beq 0x8300ca48
	if ctx.cr[0].eq {
	pc = 0x8300CA48; continue 'dispatch;
	}
	// 8300C6FC: 83DF0028  lwz r30, 0x28(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8300C700: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8300C704: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8300C708: 4BFC7539  bl 0x82fd3c40
	ctx.lr = 0x8300C70C;
	sub_82FD3C40(ctx, base);
	// 8300C70C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8300C710: 40820338  bne 0x8300ca48
	if !ctx.cr[0].eq {
	pc = 0x8300CA48; continue 'dispatch;
	}
	// 8300C714: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 8300C718: 419A0320  beq cr6, 0x8300ca38
	if ctx.cr[6].eq {
	pc = 0x8300CA38; continue 'dispatch;
	}
	// 8300C71C: A17B0000  lhz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300C720: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300C724: 41820314  beq 0x8300ca38
	if ctx.cr[0].eq {
	pc = 0x8300CA38; continue 'dispatch;
	}
	// 8300C728: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300C72C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300C730: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300C734: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300C738: 4E800421  bctrl
	ctx.lr = 0x8300C73C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300C73C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8300C740: 4BFD4F01  bl 0x82fe1640
	ctx.lr = 0x8300C744;
	sub_82FE1640(ctx, base);
	// 8300C744: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8300C748: 40820064  bne 0x8300c7ac
	if !ctx.cr[0].eq {
	pc = 0x8300C7AC; continue 'dispatch;
	}
	// 8300C74C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300C750: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300C754: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300C758: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300C75C: 4E800421  bctrl
	ctx.lr = 0x8300C760;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300C760: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300C764: 41820020  beq 0x8300c784
	if ctx.cr[0].eq {
	pc = 0x8300C784; continue 'dispatch;
	}
	// 8300C768: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300C76C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300C770: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300C774: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300C778: 4E800421  bctrl
	ctx.lr = 0x8300C77C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300C77C: 80C30090  lwz r6, 0x90(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 8300C780: 4800000C  b 0x8300c78c
	pc = 0x8300C78C; continue 'dispatch;
	// 8300C784: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8300C788: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 8300C78C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8300C790: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 8300C794: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8300C798: 4BFED739  bl 0x82ff9ed0
	ctx.lr = 0x8300C79C;
	sub_82FF9ED0(ctx, base);
	// 8300C79C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8300C7A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8300C7A4: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 8300C7A8: 481A4481  bl 0x831b0c28
	ctx.lr = 0x8300C7AC;
	sub_831B0C28(ctx, base);
	// 8300C7AC: 4BFD2D85  bl 0x82fdf530
	ctx.lr = 0x8300C7B0;
	sub_82FDF530(ctx, base);
	// 8300C7B0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8300C7B4: 4BFD2D8D  bl 0x82fdf540
	ctx.lr = 0x8300C7B8;
	sub_82FDF540(ctx, base);
	// 8300C7B8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8300C7BC: 4BFD2DA5  bl 0x82fdf560
	ctx.lr = 0x8300C7C0;
	sub_82FDF560(ctx, base);
	// 8300C7C0: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8300C7C4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8300C7C8: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8300C7CC: 4BFC7475  bl 0x82fd3c40
	ctx.lr = 0x8300C7D0;
	sub_82FD3C40(ctx, base);
	// 8300C7D0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8300C7D4: 41820018  beq 0x8300c7ec
	if ctx.cr[0].eq {
	pc = 0x8300C7EC; continue 'dispatch;
	}
	// 8300C7D8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8300C7DC: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 8300C7E0: 4BFC7461  bl 0x82fd3c40
	ctx.lr = 0x8300C7E4;
	sub_82FD3C40(ctx, base);
	// 8300C7E4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8300C7E8: 4182002C  beq 0x8300c814
	if ctx.cr[0].eq {
	pc = 0x8300C814; continue 'dispatch;
	}
	// 8300C7EC: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8300C7F0: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8300C7F4: 4BFC744D  bl 0x82fd3c40
	ctx.lr = 0x8300C7F8;
	sub_82FD3C40(ctx, base);
	// 8300C7F8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8300C7FC: 41820078  beq 0x8300c874
	if ctx.cr[0].eq {
	pc = 0x8300C874; continue 'dispatch;
	}
	// 8300C800: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8300C804: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 8300C808: 4BFC7439  bl 0x82fd3c40
	ctx.lr = 0x8300C80C;
	sub_82FD3C40(ctx, base);
	// 8300C80C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8300C810: 40820064  bne 0x8300c874
	if !ctx.cr[0].eq {
	pc = 0x8300C874; continue 'dispatch;
	}
	// 8300C814: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300C818: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300C81C: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300C820: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300C824: 4E800421  bctrl
	ctx.lr = 0x8300C828;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300C828: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300C82C: 41820020  beq 0x8300c84c
	if ctx.cr[0].eq {
	pc = 0x8300C84C; continue 'dispatch;
	}
	// 8300C830: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300C834: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300C838: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300C83C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300C840: 4E800421  bctrl
	ctx.lr = 0x8300C844;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300C844: 80C30090  lwz r6, 0x90(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 8300C848: 4800000C  b 0x8300c854
	pc = 0x8300C854; continue 'dispatch;
	// 8300C84C: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8300C850: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 8300C854: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8300C858: 3880000E  li r4, 0xe
	ctx.r[4].s64 = 14;
	// 8300C85C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8300C860: 4BFED671  bl 0x82ff9ed0
	ctx.lr = 0x8300C864;
	sub_82FF9ED0(ctx, base);
	// 8300C864: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8300C868: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8300C86C: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 8300C870: 481A43B9  bl 0x831b0c28
	ctx.lr = 0x8300C874;
	sub_831B0C28(ctx, base);
	// 8300C874: 3880003A  li r4, 0x3a
	ctx.r[4].s64 = 58;
	// 8300C878: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8300C87C: 4BFC5535  bl 0x82fd1db0
	ctx.lr = 0x8300C880;
	sub_82FD1DB0(ctx, base);
	// 8300C880: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300C884: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 8300C888: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300C88C: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300C890: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300C894: 419A0054  beq cr6, 0x8300c8e8
	if ctx.cr[6].eq {
	pc = 0x8300C8E8; continue 'dispatch;
	}
	// 8300C898: 4E800421  bctrl
	ctx.lr = 0x8300C89C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300C89C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300C8A0: 41820020  beq 0x8300c8c0
	if ctx.cr[0].eq {
	pc = 0x8300C8C0; continue 'dispatch;
	}
	// 8300C8A4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300C8A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300C8AC: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300C8B0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300C8B4: 4E800421  bctrl
	ctx.lr = 0x8300C8B8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300C8B8: 80C30090  lwz r6, 0x90(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 8300C8BC: 4800000C  b 0x8300c8c8
	pc = 0x8300C8C8; continue 'dispatch;
	// 8300C8C0: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8300C8C4: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 8300C8C8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8300C8CC: 3880000E  li r4, 0xe
	ctx.r[4].s64 = 14;
	// 8300C8D0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8300C8D4: 4BFED5FD  bl 0x82ff9ed0
	ctx.lr = 0x8300C8D8;
	sub_82FF9ED0(ctx, base);
	// 8300C8D8: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8300C8DC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8300C8E0: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 8300C8E4: 481A4345  bl 0x831b0c28
	ctx.lr = 0x8300C8E8;
	sub_831B0C28(ctx, base);
	// 8300C8E8: 4E800421  bctrl
	ctx.lr = 0x8300C8EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300C8EC: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8300C8F0: 4BFD50E9  bl 0x82fe19d8
	ctx.lr = 0x8300C8F4;
	sub_82FE19D8(ctx, base);
	// 8300C8F4: 907F002C  stw r3, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[3].u32 ) };
	// 8300C8F8: A17B0000  lhz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300C8FC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300C900: 41820028  beq 0x8300c928
	if ctx.cr[0].eq {
	pc = 0x8300C928; continue 'dispatch;
	}
	// 8300C904: 397B0002  addi r11, r27, 2
	ctx.r[11].s64 = ctx.r[27].s64 + 2;
	// 8300C908: 48000008  b 0x8300c910
	pc = 0x8300C910; continue 'dispatch;
	// 8300C90C: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 8300C910: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300C914: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300C918: 4082FFF4  bne 0x8300c90c
	if !ctx.cr[0].eq {
	pc = 0x8300C90C; continue 'dispatch;
	}
	// 8300C91C: 7D7B5850  subf r11, r27, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[27].s64;
	// 8300C920: 7D7C0E70  srawi r28, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[28].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 8300C924: 48000008  b 0x8300c92c
	pc = 0x8300C92C; continue 'dispatch;
	// 8300C928: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8300C92C: 815F0028  lwz r10, 0x28(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8300C930: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300C934: 41820034  beq 0x8300c968
	if ctx.cr[0].eq {
	pc = 0x8300C968; continue 'dispatch;
	}
	// 8300C938: A16A0000  lhz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300C93C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300C940: 41820028  beq 0x8300c968
	if ctx.cr[0].eq {
	pc = 0x8300C968; continue 'dispatch;
	}
	// 8300C944: 396A0002  addi r11, r10, 2
	ctx.r[11].s64 = ctx.r[10].s64 + 2;
	// 8300C948: 48000008  b 0x8300c950
	pc = 0x8300C950; continue 'dispatch;
	// 8300C94C: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 8300C950: A12B0000  lhz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300C954: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300C958: 4082FFF4  bne 0x8300c94c
	if !ctx.cr[0].eq {
	pc = 0x8300C94C; continue 'dispatch;
	}
	// 8300C95C: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8300C960: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 8300C964: 48000008  b 0x8300c96c
	pc = 0x8300C96C; continue 'dispatch;
	// 8300C968: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8300C96C: 7D6BE214  add r11, r11, r28
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 8300C970: 3BAB0001  addi r29, r11, 1
	ctx.r[29].s64 = ctx.r[11].s64 + 1;
	// 8300C974: 2F1D0F9F  cmpwi cr6, r29, 0xf9f
	ctx.cr[6].compare_i32(ctx.r[29].s32, 3999, &mut ctx.xer);
	// 8300C978: 41980038  blt cr6, 0x8300c9b0
	if ctx.cr[6].lt {
	pc = 0x8300C9B0; continue 'dispatch;
	}
	// 8300C97C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300C980: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300C984: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300C988: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300C98C: 4E800421  bctrl
	ctx.lr = 0x8300C990;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300C990: 80630090  lwz r3, 0x90(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 8300C994: 57A4083C  slwi r4, r29, 1
	ctx.r[4].u32 = ctx.r[29].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 8300C998: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300C99C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8300C9A0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300C9A4: 4E800421  bctrl
	ctx.lr = 0x8300C9A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300C9A8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8300C9AC: 48000008  b 0x8300c9b4
	pc = 0x8300C9B4; continue 'dispatch;
	// 8300C9B0: 3BC10070  addi r30, r1, 0x70
	ctx.r[30].s64 = ctx.r[1].s64 + 112;
	// 8300C9B4: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8300C9B8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8300C9BC: 4BFC51AD  bl 0x82fd1b68
	ctx.lr = 0x8300C9C0;
	sub_82FD1B68(ctx, base);
	// 8300C9C0: 578B083C  slwi r11, r28, 1
	ctx.r[11].u32 = ctx.r[28].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8300C9C4: 3940003A  li r10, 0x3a
	ctx.r[10].s64 = 58;
	// 8300C9C8: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 8300C9CC: 386B0002  addi r3, r11, 2
	ctx.r[3].s64 = ctx.r[11].s64 + 2;
	// 8300C9D0: B14B0000  sth r10, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 8300C9D4: 809F0028  lwz r4, 0x28(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8300C9D8: 4BFC5191  bl 0x82fd1b68
	ctx.lr = 0x8300C9DC;
	sub_82FD1B68(ctx, base);
	// 8300C9DC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300C9E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300C9E4: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300C9E8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300C9EC: 4E800421  bctrl
	ctx.lr = 0x8300C9F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300C9F0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8300C9F4: 4BFD4FE5  bl 0x82fe19d8
	ctx.lr = 0x8300C9F8;
	sub_82FE19D8(ctx, base);
	// 8300C9F8: 2F1D0F9F  cmpwi cr6, r29, 0xf9f
	ctx.cr[6].compare_i32(ctx.r[29].s32, 3999, &mut ctx.xer);
	// 8300C9FC: 907F001C  stw r3, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[3].u32 ) };
	// 8300CA00: 41980030  blt cr6, 0x8300ca30
	if ctx.cr[6].lt {
	pc = 0x8300CA30; continue 'dispatch;
	}
	// 8300CA04: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300CA08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300CA0C: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300CA10: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300CA14: 4E800421  bctrl
	ctx.lr = 0x8300CA18;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300CA18: 80630090  lwz r3, 0x90(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 8300CA1C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8300CA20: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300CA24: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300CA28: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300CA2C: 4E800421  bctrl
	ctx.lr = 0x8300CA30;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300CA30: 38211FF0  addi r1, r1, 0x1ff0
	ctx.r[1].s64 = ctx.r[1].s64 + 8176;
	// 8300CA34: 4819B77C  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
	// 8300CA38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8300CA3C: 93DF001C  stw r30, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u32 ) };
	// 8300CA40: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 8300CA44: 4BFFFFEC  b 0x8300ca30
	pc = 0x8300CA30; continue 'dispatch;
	// 8300CA48: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300CA4C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300CA50: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300CA54: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300CA58: 4E800421  bctrl
	ctx.lr = 0x8300CA5C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300CA5C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300CA60: 41820020  beq 0x8300ca80
	if ctx.cr[0].eq {
	pc = 0x8300CA80; continue 'dispatch;
	}
	// 8300CA64: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300CA68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300CA6C: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300CA70: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300CA74: 4E800421  bctrl
	ctx.lr = 0x8300CA78;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300CA78: 80C30090  lwz r6, 0x90(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 8300CA7C: 4800000C  b 0x8300ca88
	pc = 0x8300CA88; continue 'dispatch;
	// 8300CA80: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8300CA84: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 8300CA88: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8300CA8C: 3880000E  li r4, 0xe
	ctx.r[4].s64 = 14;
	// 8300CA90: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8300CA94: 4BFED43D  bl 0x82ff9ed0
	ctx.lr = 0x8300CA98;
	sub_82FF9ED0(ctx, base);
	// 8300CA98: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8300CA9C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8300CAA0: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 8300CAA4: 481A4185  bl 0x831b0c28
	ctx.lr = 0x8300CAA8;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300CAA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300CAA8 size=376
    let mut pc: u32 = 0x8300CAA8;
    'dispatch: loop {
        match pc {
            0x8300CAA8 => {
    //   block [0x8300CAA8..0x8300CC20)
	// 8300CAA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300CAAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300CAB0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8300CAB4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8300CAB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300CABC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8300CAC0: 3D408214  lis r10, -0x7dec
	ctx.r[10].s64 = -2112618496;
	// 8300CAC4: A17F0008  lhz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300CAC8: A14AA6A4  lhz r10, -0x595c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(-22876 as u32) ) } as u64;
	// 8300CACC: 7D4A5838  and r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 8300CAD0: 7D4A0034  cntlzw r10, r10
	ctx.r[10].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 8300CAD4: 554ADFFE  rlwinm r10, r10, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 8300CAD8: 694A0001  xori r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u64 ^ 1;
	// 8300CADC: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300CAE0: 41820080  beq 0x8300cb60
	if ctx.cr[0].eq {
	pc = 0x8300CB60; continue 'dispatch;
	}
	// 8300CAE4: 3D408214  lis r10, -0x7dec
	ctx.r[10].s64 = -2112618496;
	// 8300CAE8: A14AA6C8  lhz r10, -0x5938(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(-22840 as u32) ) } as u64;
	// 8300CAEC: 7D4B5838  and r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 8300CAF0: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8300CAF4: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8300CAF8: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 8300CAFC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300CB00: 40820060  bne 0x8300cb60
	if !ctx.cr[0].eq {
	pc = 0x8300CB60; continue 'dispatch;
	}
	// 8300CB04: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300CB08: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300CB0C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300CB10: 4E800421  bctrl
	ctx.lr = 0x8300CB14;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300CB14: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300CB18: 41820020  beq 0x8300cb38
	if ctx.cr[0].eq {
	pc = 0x8300CB38; continue 'dispatch;
	}
	// 8300CB1C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300CB20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300CB24: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300CB28: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300CB2C: 4E800421  bctrl
	ctx.lr = 0x8300CB30;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300CB30: 80C30090  lwz r6, 0x90(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 8300CB34: 4800000C  b 0x8300cb40
	pc = 0x8300CB40; continue 'dispatch;
	// 8300CB38: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8300CB3C: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 8300CB40: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8300CB44: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 8300CB48: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8300CB4C: 4BFED385  bl 0x82ff9ed0
	ctx.lr = 0x8300CB50;
	sub_82FF9ED0(ctx, base);
	// 8300CB50: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8300CB54: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8300CB58: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 8300CB5C: 481A40CD  bl 0x831b0c28
	ctx.lr = 0x8300CB60;
	sub_831B0C28(ctx, base);
	// 8300CB60: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300CB64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300CB68: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300CB6C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300CB70: 4E800421  bctrl
	ctx.lr = 0x8300CB74;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300CB74: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8300CB78: 41820048  beq 0x8300cbc0
	if ctx.cr[0].eq {
	pc = 0x8300CBC0; continue 'dispatch;
	}
	// 8300CB7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8300CB80: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8300CB84: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 8300CB88: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 8300CB8C: 4BFD2BED  bl 0x82fdf778
	ctx.lr = 0x8300CB90;
	sub_82FDF778(ctx, base);
	// 8300CB90: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 8300CB94: 4BFEFAF5  bl 0x82ffc688
	ctx.lr = 0x8300CB98;
	sub_82FFC688(ctx, base);
	// 8300CB98: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8300CB9C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8300CBA0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8300CBA4: 4BFD8B3D  bl 0x82fe56e0
	ctx.lr = 0x8300CBA8;
	sub_82FE56E0(ctx, base);
	// 8300CBA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8300CBAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300CBB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300CBB4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8300CBB8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8300CBBC: 4E800020  blr
	return;
	// 8300CBC0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300CBC4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300CBC8: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300CBCC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300CBD0: 4E800421  bctrl
	ctx.lr = 0x8300CBD4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300CBD4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300CBD8: 41820020  beq 0x8300cbf8
	if ctx.cr[0].eq {
	pc = 0x8300CBF8; continue 'dispatch;
	}
	// 8300CBDC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300CBE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300CBE4: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300CBE8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300CBEC: 4E800421  bctrl
	ctx.lr = 0x8300CBF0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300CBF0: 80C30090  lwz r6, 0x90(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 8300CBF4: 4800000C  b 0x8300cc00
	pc = 0x8300CC00; continue 'dispatch;
	// 8300CBF8: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8300CBFC: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 8300CC00: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8300CC04: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 8300CC08: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8300CC0C: 4BFED2C5  bl 0x82ff9ed0
	ctx.lr = 0x8300CC10;
	sub_82FF9ED0(ctx, base);
	// 8300CC10: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8300CC14: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8300CC18: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 8300CC1C: 481A400D  bl 0x831b0c28
	ctx.lr = 0x8300CC20;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300CC20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300CC20 size=888
    let mut pc: u32 = 0x8300CC20;
    'dispatch: loop {
        match pc {
            0x8300CC20 => {
    //   block [0x8300CC20..0x8300CF98)
	// 8300CC20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300CC24: 4819B531  bl 0x831a8154
	ctx.lr = 0x8300CC28;
	sub_831A8130(ctx, base);
	// 8300CC28: E981F000  ld r12, -0x1000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-4096 as u32) ) };
	// 8300CC2C: 9421E000  stwu r1, -0x2000(r1)
	ea = ctx.r[1].u32.wrapping_add(-8192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300CC30: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8300CC34: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 8300CC38: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8300CC3C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300CC40: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300CC44: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300CC48: 4E800421  bctrl
	ctx.lr = 0x8300CC4C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300CC4C: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8300CC50: 4BFD2901  bl 0x82fdf550
	ctx.lr = 0x8300CC54;
	sub_82FDF550(ctx, base);
	// 8300CC54: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8300CC58: 4BFD2909  bl 0x82fdf560
	ctx.lr = 0x8300CC5C;
	sub_82FDF560(ctx, base);
	// 8300CC5C: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 8300CC60: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8300CC64: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8300CC68: 4BFD4D71  bl 0x82fe19d8
	ctx.lr = 0x8300CC6C;
	sub_82FE19D8(ctx, base);
	// 8300CC6C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8300CC70: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8300CC74: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 8300CC78: 4BFD4B99  bl 0x82fe1810
	ctx.lr = 0x8300CC7C;
	sub_82FE1810(ctx, base);
	// 8300CC7C: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8300CC80: 40800064  bge 0x8300cce4
	if !ctx.cr[0].lt {
	pc = 0x8300CCE4; continue 'dispatch;
	}
	// 8300CC84: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300CC88: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300CC8C: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300CC90: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300CC94: 4E800421  bctrl
	ctx.lr = 0x8300CC98;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300CC98: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300CC9C: 41820020  beq 0x8300ccbc
	if ctx.cr[0].eq {
	pc = 0x8300CCBC; continue 'dispatch;
	}
	// 8300CCA0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300CCA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300CCA8: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300CCAC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300CCB0: 4E800421  bctrl
	ctx.lr = 0x8300CCB4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300CCB4: 80C30090  lwz r6, 0x90(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 8300CCB8: 4800000C  b 0x8300ccc4
	pc = 0x8300CCC4; continue 'dispatch;
	// 8300CCBC: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8300CCC0: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 8300CCC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8300CCC8: 3880000E  li r4, 0xe
	ctx.r[4].s64 = 14;
	// 8300CCCC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8300CCD0: 4BFED201  bl 0x82ff9ed0
	ctx.lr = 0x8300CCD4;
	sub_82FF9ED0(ctx, base);
	// 8300CCD4: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8300CCD8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8300CCDC: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 8300CCE0: 481A3F49  bl 0x831b0c28
	ctx.lr = 0x8300CCE4;
	sub_831B0C28(ctx, base);
	// 8300CCE4: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 8300CCE8: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8300CCEC: 7F57D378  mr r23, r26
	ctx.r[23].u64 = ctx.r[26].u64;
	// 8300CCF0: 409A00A0  bne cr6, 0x8300cd90
	if !ctx.cr[6].eq {
	pc = 0x8300CD90; continue 'dispatch;
	}
	// 8300CCF4: 83DF001C  lwz r30, 0x1c(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 8300CCF8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8300CCFC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8300CD00: 4BFC6F41  bl 0x82fd3c40
	ctx.lr = 0x8300CD04;
	sub_82FD3C40(ctx, base);
	// 8300CD04: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8300CD08: 4182007C  beq 0x8300cd84
	if ctx.cr[0].eq {
	pc = 0x8300CD84; continue 'dispatch;
	}
	// 8300CD0C: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 8300CD10: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 8300CD14: 4BFC6F2D  bl 0x82fd3c40
	ctx.lr = 0x8300CD18;
	sub_82FD3C40(ctx, base);
	// 8300CD18: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8300CD1C: 40820064  bne 0x8300cd80
	if !ctx.cr[0].eq {
	pc = 0x8300CD80; continue 'dispatch;
	}
	// 8300CD20: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300CD24: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300CD28: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300CD2C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300CD30: 4E800421  bctrl
	ctx.lr = 0x8300CD34;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300CD34: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300CD38: 41820020  beq 0x8300cd58
	if ctx.cr[0].eq {
	pc = 0x8300CD58; continue 'dispatch;
	}
	// 8300CD3C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300CD40: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300CD44: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300CD48: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300CD4C: 4E800421  bctrl
	ctx.lr = 0x8300CD50;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300CD50: 80C30090  lwz r6, 0x90(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 8300CD54: 4800000C  b 0x8300cd60
	pc = 0x8300CD60; continue 'dispatch;
	// 8300CD58: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8300CD5C: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 8300CD60: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8300CD64: 3880000E  li r4, 0xe
	ctx.r[4].s64 = 14;
	// 8300CD68: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8300CD6C: 4BFED165  bl 0x82ff9ed0
	ctx.lr = 0x8300CD70;
	sub_82FF9ED0(ctx, base);
	// 8300CD70: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8300CD74: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8300CD78: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 8300CD7C: 481A3EAD  bl 0x831b0c28
	ctx.lr = 0x8300CD80;
	sub_831B0C28(ctx, base);
	// 8300CD80: 3AE00001  li r23, 1
	ctx.r[23].s64 = 1;
	// 8300CD84: 93DF0028  stw r30, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[30].u32 ) };
	// 8300CD88: 935F002C  stw r26, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[26].u32 ) };
	// 8300CD8C: 4800014C  b 0x8300ced8
	pc = 0x8300CED8; continue 'dispatch;
	// 8300CD90: 2F1D0F9F  cmpwi cr6, r29, 0xf9f
	ctx.cr[6].compare_i32(ctx.r[29].s32, 3999, &mut ctx.xer);
	// 8300CD94: 41980078  blt cr6, 0x8300ce0c
	if ctx.cr[6].lt {
	pc = 0x8300CE0C; continue 'dispatch;
	}
	// 8300CD98: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300CD9C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300CDA0: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300CDA4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300CDA8: 4E800421  bctrl
	ctx.lr = 0x8300CDAC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300CDAC: 80630090  lwz r3, 0x90(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 8300CDB0: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8300CDB4: 419A0034  beq cr6, 0x8300cde8
	if ctx.cr[6].eq {
	pc = 0x8300CDE8; continue 'dispatch;
	}
	// 8300CDB8: A17E0000  lhz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300CDBC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300CDC0: 41820028  beq 0x8300cde8
	if ctx.cr[0].eq {
	pc = 0x8300CDE8; continue 'dispatch;
	}
	// 8300CDC4: 397E0002  addi r11, r30, 2
	ctx.r[11].s64 = ctx.r[30].s64 + 2;
	// 8300CDC8: 48000008  b 0x8300cdd0
	pc = 0x8300CDD0; continue 'dispatch;
	// 8300CDCC: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 8300CDD0: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300CDD4: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300CDD8: 4082FFF4  bne 0x8300cdcc
	if !ctx.cr[0].eq {
	pc = 0x8300CDCC; continue 'dispatch;
	}
	// 8300CDDC: 7D7E5850  subf r11, r30, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	// 8300CDE0: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 8300CDE4: 48000008  b 0x8300cdec
	pc = 0x8300CDEC; continue 'dispatch;
	// 8300CDE8: 7F4BD378  mr r11, r26
	ctx.r[11].u64 = ctx.r[26].u64;
	// 8300CDEC: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300CDF0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8300CDF4: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 8300CDF8: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8300CDFC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300CE00: 4E800421  bctrl
	ctx.lr = 0x8300CE04;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300CE04: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8300CE08: 48000008  b 0x8300ce10
	pc = 0x8300CE10; continue 'dispatch;
	// 8300CE0C: 3BC10070  addi r30, r1, 0x70
	ctx.r[30].s64 = ctx.r[1].s64 + 112;
	// 8300CE10: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8300CE14: 809F001C  lwz r4, 0x1c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 8300CE18: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8300CE1C: 4BFC4D8D  bl 0x82fd1ba8
	ctx.lr = 0x8300CE20;
	sub_82FD1BA8(ctx, base);
	// 8300CE20: 57BC083C  slwi r28, r29, 1
	ctx.r[28].u32 = ctx.r[29].u32.wrapping_shl(1);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 8300CE24: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8300CE28: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8300CE2C: 7F5CF32E  sthx r26, r28, r30
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[28].u32.wrapping_add(ctx.r[30].u32), ctx.r[26].u16) };
	// 8300CE30: 4BFD4BA9  bl 0x82fe19d8
	ctx.lr = 0x8300CE34;
	sub_82FE19D8(ctx, base);
	// 8300CE34: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 8300CE38: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8300CE3C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8300CE40: 7D7C5A14  add r11, r28, r11
	ctx.r[11].u64 = ctx.r[28].u64 + ctx.r[11].u64;
	// 8300CE44: 388B0002  addi r4, r11, 2
	ctx.r[4].s64 = ctx.r[11].s64 + 2;
	// 8300CE48: 915F002C  stw r10, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[10].u32 ) };
	// 8300CE4C: 4BFD4B8D  bl 0x82fe19d8
	ctx.lr = 0x8300CE50;
	sub_82FE19D8(ctx, base);
	// 8300CE50: 2F1D0F9F  cmpwi cr6, r29, 0xf9f
	ctx.cr[6].compare_i32(ctx.r[29].s32, 3999, &mut ctx.xer);
	// 8300CE54: 907F0028  stw r3, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[3].u32 ) };
	// 8300CE58: 41980030  blt cr6, 0x8300ce88
	if ctx.cr[6].lt {
	pc = 0x8300CE88; continue 'dispatch;
	}
	// 8300CE5C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300CE60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300CE64: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300CE68: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300CE6C: 4E800421  bctrl
	ctx.lr = 0x8300CE70;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300CE70: 80630090  lwz r3, 0x90(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 8300CE74: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8300CE78: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300CE7C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300CE80: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300CE84: 4E800421  bctrl
	ctx.lr = 0x8300CE88;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300CE88: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300CE8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300CE90: 83DF002C  lwz r30, 0x2c(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300CE94: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300CE98: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300CE9C: 4E800421  bctrl
	ctx.lr = 0x8300CEA0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300CEA0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8300CEA4: 4BFD479D  bl 0x82fe1640
	ctx.lr = 0x8300CEA8;
	sub_82FE1640(ctx, base);
	// 8300CEA8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8300CEAC: 4182008C  beq 0x8300cf38
	if ctx.cr[0].eq {
	pc = 0x8300CF38; continue 'dispatch;
	}
	// 8300CEB0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300CEB4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300CEB8: 83DF0028  lwz r30, 0x28(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8300CEBC: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300CEC0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300CEC4: 4E800421  bctrl
	ctx.lr = 0x8300CEC8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300CEC8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8300CECC: 4BFD4775  bl 0x82fe1640
	ctx.lr = 0x8300CED0;
	sub_82FE1640(ctx, base);
	// 8300CED0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8300CED4: 41820064  beq 0x8300cf38
	if ctx.cr[0].eq {
	pc = 0x8300CF38; continue 'dispatch;
	}
	// 8300CED8: 56EB063F  clrlwi. r11, r23, 0x18
	ctx.r[11].u64 = ctx.r[23].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8300CEDC: 4182000C  beq 0x8300cee8
	if ctx.cr[0].eq {
	pc = 0x8300CEE8; continue 'dispatch;
	}
	// 8300CEE0: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 8300CEE4: 48000030  b 0x8300cf14
	pc = 0x8300CF14; continue 'dispatch;
	// 8300CEE8: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 8300CEEC: 419A0014  beq cr6, 0x8300cf00
	if ctx.cr[6].eq {
	pc = 0x8300CF00; continue 'dispatch;
	}
	// 8300CEF0: A1790000  lhz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300CEF4: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 8300CEF8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300CEFC: 40820008  bne 0x8300cf04
	if !ctx.cr[0].eq {
	pc = 0x8300CF04; continue 'dispatch;
	}
	// 8300CF00: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8300CF04: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 8300CF08: 807F002C  lwz r3, 0x2c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300CF0C: 4BFD2665  bl 0x82fdf570
	ctx.lr = 0x8300CF10;
	sub_82FDF570(ctx, base);
	// 8300CF10: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8300CF14: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 8300CF18: 409A000C  bne cr6, 0x8300cf24
	if !ctx.cr[6].eq {
	pc = 0x8300CF24; continue 'dispatch;
	}
	// 8300CF1C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8300CF20: 4800000C  b 0x8300cf2c
	pc = 0x8300CF2C; continue 'dispatch;
	// 8300CF24: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8300CF28: 4BFD4AB1  bl 0x82fe19d8
	ctx.lr = 0x8300CF2C;
	sub_82FE19D8(ctx, base);
	// 8300CF2C: 907F0024  stw r3, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[3].u32 ) };
	// 8300CF30: 38212000  addi r1, r1, 0x2000
	ctx.r[1].s64 = ctx.r[1].s64 + 8192;
	// 8300CF34: 4819B270  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
	// 8300CF38: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300CF3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300CF40: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300CF44: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300CF48: 4E800421  bctrl
	ctx.lr = 0x8300CF4C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300CF4C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300CF50: 41820020  beq 0x8300cf70
	if ctx.cr[0].eq {
	pc = 0x8300CF70; continue 'dispatch;
	}
	// 8300CF54: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300CF58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300CF5C: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8300CF60: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300CF64: 4E800421  bctrl
	ctx.lr = 0x8300CF68;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300CF68: 80C30090  lwz r6, 0x90(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 8300CF6C: 4800000C  b 0x8300cf78
	pc = 0x8300CF78; continue 'dispatch;
	// 8300CF70: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8300CF74: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 8300CF78: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8300CF7C: 3880000E  li r4, 0xe
	ctx.r[4].s64 = 14;
	// 8300CF80: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8300CF84: 4BFECF4D  bl 0x82ff9ed0
	ctx.lr = 0x8300CF88;
	sub_82FF9ED0(ctx, base);
	// 8300CF88: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8300CF8C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8300CF90: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 8300CF94: 481A3C95  bl 0x831b0c28
	ctx.lr = 0x8300CF98;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300CF98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300CF98 size=76
    let mut pc: u32 = 0x8300CF98;
    'dispatch: loop {
        match pc {
            0x8300CF98 => {
    //   block [0x8300CF98..0x8300CFE4)
	// 8300CF98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300CF9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300CFA0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8300CFA4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8300CFA8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300CFAC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8300CFB0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8300CFB4: 4BFF7435  bl 0x830043e8
	ctx.lr = 0x8300CFB8;
	sub_830043E8(ctx, base);
	// 8300CFB8: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8300CFBC: 4182000C  beq 0x8300cfc8
	if ctx.cr[0].eq {
	pc = 0x8300CFC8; continue 'dispatch;
	}
	// 8300CFC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300CFC4: 4B2B32A5  bl 0x822c0268
	ctx.lr = 0x8300CFC8;
	sub_822C0268(ctx, base);
	// 8300CFC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300CFCC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8300CFD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300CFD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300CFD8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8300CFDC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8300CFE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300CFE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300CFE8 size=8
    let mut pc: u32 = 0x8300CFE8;
    'dispatch: loop {
        match pc {
            0x8300CFE8 => {
    //   block [0x8300CFE8..0x8300CFF0)
	// 8300CFE8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8300CFEC: 82142338  lwz r16, 0x2338(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(9016 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300CFF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300CFF0 size=80
    let mut pc: u32 = 0x8300CFF0;
    'dispatch: loop {
        match pc {
            0x8300CFF0 => {
    //   block [0x8300CFF0..0x8300D040)
	// 8300CFF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300CFF4: 4819B175  bl 0x831a8168
	ctx.lr = 0x8300CFF8;
	sub_831A8130(ctx, base);
	// 8300CFF8: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 8300CFFC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300D000: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8300D004: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 8300D008: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8300D00C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8300D010: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 8300D014: 4BFF7CF5  bl 0x83004d08
	ctx.lr = 0x8300D018;
	sub_83004D08(ctx, base);
	// 8300D018: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8300D01C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8300D020: 396B2218  addi r11, r11, 0x2218
	ctx.r[11].s64 = ctx.r[11].s64 + 8728;
	// 8300D024: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8300D028: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8300D02C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8300D030: 4BFFFBF1  bl 0x8300cc20
	ctx.lr = 0x8300D034;
	sub_8300CC20(ctx, base);
	// 8300D034: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8300D038: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 8300D03C: 4819B17C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300D040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300D040 size=40
    let mut pc: u32 = 0x8300D040;
    'dispatch: loop {
        match pc {
            0x8300D040 => {
    //   block [0x8300D040..0x8300D068)
	// 8300D040: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 8300D044: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300D048: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300D04C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300D050: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 8300D054: 4BFF7395  bl 0x830043e8
	ctx.lr = 0x8300D058;
	sub_830043E8(ctx, base);
	// 8300D058: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300D05C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300D060: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300D064: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300D068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300D068 size=132
    let mut pc: u32 = 0x8300D068;
    'dispatch: loop {
        match pc {
            0x8300D068 => {
    //   block [0x8300D068..0x8300D0EC)
	// 8300D068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300D06C: 4819B0FD  bl 0x831a8168
	ctx.lr = 0x8300D070;
	sub_831A8130(ctx, base);
	// 8300D070: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300D074: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8300D078: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8300D07C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8300D080: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300D084: 816B00AC  lwz r11, 0xac(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(172 as u32) ) } as u64;
	// 8300D088: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300D08C: 4E800421  bctrl
	ctx.lr = 0x8300D090;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300D090: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8300D094: 4182001C  beq 0x8300d0b0
	if ctx.cr[0].eq {
	pc = 0x8300D0B0; continue 'dispatch;
	}
	// 8300D098: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300D09C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8300D0A0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8300D0A4: 816B00B4  lwz r11, 0xb4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(180 as u32) ) } as u64;
	// 8300D0A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300D0AC: 4E800421  bctrl
	ctx.lr = 0x8300D0B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300D0B0: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8300D0B4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8300D0B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300D0BC: 4BFFFB65  bl 0x8300cc20
	ctx.lr = 0x8300D0C0;
	sub_8300CC20(ctx, base);
	// 8300D0C0: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8300D0C4: 419A001C  beq cr6, 0x8300d0e0
	if ctx.cr[6].eq {
	pc = 0x8300D0E0; continue 'dispatch;
	}
	// 8300D0C8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300D0CC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8300D0D0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8300D0D4: 816B00CC  lwz r11, 0xcc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(204 as u32) ) } as u64;
	// 8300D0D8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300D0DC: 4E800421  bctrl
	ctx.lr = 0x8300D0E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300D0E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300D0E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8300D0E8: 4819B0D0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300D0F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300D0F0 size=32
    let mut pc: u32 = 0x8300D0F0;
    'dispatch: loop {
        match pc {
            0x8300D0F0 => {
    //   block [0x8300D0F0..0x8300D110)
	// 8300D0F0: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300D0F4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300D0F8: 41820018  beq 0x8300d110
	if ctx.cr[0].eq {
		sub_8300D110(ctx, base);
		return;
	}
	// 8300D0FC: 8143000C  lwz r10, 0xc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 8300D100: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8300D104: 814A001C  lwz r10, 0x1c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 8300D108: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8300D10C: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300D110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300D110 size=8
    let mut pc: u32 = 0x8300D110;
    'dispatch: loop {
        match pc {
            0x8300D110 => {
    //   block [0x8300D110..0x8300D118)
	// 8300D110: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8300D114: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300D118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300D118 size=124
    let mut pc: u32 = 0x8300D118;
    'dispatch: loop {
        match pc {
            0x8300D118 => {
    //   block [0x8300D118..0x8300D194)
	// 8300D118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300D11C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300D120: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300D124: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8300D128: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300D12C: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300D130: 41820038  beq 0x8300d168
	if ctx.cr[0].eq {
	pc = 0x8300D168; continue 'dispatch;
	}
	// 8300D134: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8300D138: 8109001C  lwz r8, 0x1c(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(28 as u32) ) } as u64;
	// 8300D13C: 7F0A4040  cmplw cr6, r10, r8
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[8].u32, &mut ctx.xer);
	// 8300D140: 41990028  bgt cr6, 0x8300d168
	if ctx.cr[6].gt {
	pc = 0x8300D168; continue 'dispatch;
	}
	// 8300D144: 81290014  lwz r9, 0x14(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(20 as u32) ) } as u64;
	// 8300D148: 5548103A  slwi r8, r10, 2
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8300D14C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8300D150: 7C69402E  lwzx r3, r9, r8
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 8300D154: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8300D158: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8300D15C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300D160: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300D164: 4E800020  blr
	return;
	// 8300D168: 3D408214  lis r10, -0x7dec
	ctx.r[10].s64 = -2112618496;
	// 8300D16C: 80EB0018  lwz r7, 0x18(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8300D170: 38C0001E  li r6, 0x1e
	ctx.r[6].s64 = 30;
	// 8300D174: 388A2368  addi r4, r10, 0x2368
	ctx.r[4].s64 = ctx.r[10].s64 + 9064;
	// 8300D178: 38A001DD  li r5, 0x1dd
	ctx.r[5].s64 = 477;
	// 8300D17C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8300D180: 4BFD3BA9  bl 0x82fe0d28
	ctx.lr = 0x8300D184;
	sub_82FE0D28(ctx, base);
	// 8300D184: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8300D188: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8300D18C: 388BC690  addi r4, r11, -0x3970
	ctx.r[4].s64 = ctx.r[11].s64 + -14704;
	// 8300D190: 481A3A99  bl 0x831b0c28
	ctx.lr = 0x8300D194;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300D198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300D198 size=28
    let mut pc: u32 = 0x8300D198;
    'dispatch: loop {
        match pc {
            0x8300D198 => {
    //   block [0x8300D198..0x8300D1B4)
	// 8300D198: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 8300D19C: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 8300D1A0: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8300D1A4: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8300D1A8: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 8300D1AC: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8300D1B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300D1B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300D1B8 size=32
    let mut pc: u32 = 0x8300D1B8;
    'dispatch: loop {
        match pc {
            0x8300D1B8 => {
    //   block [0x8300D1B8..0x8300D1D8)
	// 8300D1B8: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8300D1BC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300D1C0: 41820018  beq 0x8300d1d8
	if ctx.cr[0].eq {
		sub_8300D1D8(ctx, base);
		return;
	}
	// 8300D1C4: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300D1C8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8300D1CC: 814A0010  lwz r10, 0x10(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 8300D1D0: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8300D1D4: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300D1D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300D1D8 size=8
    let mut pc: u32 = 0x8300D1D8;
    'dispatch: loop {
        match pc {
            0x8300D1D8 => {
    //   block [0x8300D1D8..0x8300D1E0)
	// 8300D1D8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8300D1DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300D1E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300D1E0 size=16
    let mut pc: u32 = 0x8300D1E0;
    'dispatch: loop {
        match pc {
            0x8300D1E0 => {
    //   block [0x8300D1E0..0x8300D1F0)
	// 8300D1E0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8300D1E4: 396B23D0  addi r11, r11, 0x23d0
	ctx.r[11].s64 = ctx.r[11].s64 + 9168;
	// 8300D1E8: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8300D1EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300D1F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300D1F0 size=160
    let mut pc: u32 = 0x8300D1F0;
    'dispatch: loop {
        match pc {
            0x8300D1F0 => {
    //   block [0x8300D1F0..0x8300D290)
	// 8300D1F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300D1F4: 4819AF69  bl 0x831a815c
	ctx.lr = 0x8300D1F8;
	sub_831A8130(ctx, base);
	// 8300D1F8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300D1FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8300D200: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 8300D204: 7F3ACB78  mr r26, r25
	ctx.r[26].u64 = ctx.r[25].u64;
	// 8300D208: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8300D20C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8300D210: 40990074  ble cr6, 0x8300d284
	if !ctx.cr[6].gt {
	pc = 0x8300D284; continue 'dispatch;
	}
	// 8300D214: 7F3DCB78  mr r29, r25
	ctx.r[29].u64 = ctx.r[25].u64;
	// 8300D218: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300D21C: 7FCBE82E  lwzx r30, r11, r29
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 8300D220: 281E0000  cmplwi r30, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300D224: 41820044  beq 0x8300d268
	if ctx.cr[0].eq {
	pc = 0x8300D268; continue 'dispatch;
	}
	// 8300D228: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8300D22C: 837E0004  lwz r27, 4(r30)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8300D230: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300D234: 41820020  beq 0x8300d254
	if ctx.cr[0].eq {
	pc = 0x8300D254; continue 'dispatch;
	}
	// 8300D238: 839E0000  lwz r28, 0(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300D23C: 281C0000  cmplwi r28, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300D240: 41820014  beq 0x8300d254
	if ctx.cr[0].eq {
	pc = 0x8300D254; continue 'dispatch;
	}
	// 8300D244: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8300D248: 48030969  bl 0x8303dbb0
	ctx.lr = 0x8300D24C;
	sub_8303DBB0(ctx, base);
	// 8300D24C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8300D250: 4BFCB091  bl 0x82fd82e0
	ctx.lr = 0x8300D254;
	sub_82FD82E0(ctx, base);
	// 8300D254: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8300D258: 4BFCB089  bl 0x82fd82e0
	ctx.lr = 0x8300D25C;
	sub_82FD82E0(ctx, base);
	// 8300D25C: 7F7EDB78  mr r30, r27
	ctx.r[30].u64 = ctx.r[27].u64;
	// 8300D260: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 8300D264: 409AFFC4  bne cr6, 0x8300d228
	if !ctx.cr[6].eq {
	pc = 0x8300D228; continue 'dispatch;
	}
	// 8300D268: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300D26C: 3B5A0001  addi r26, r26, 1
	ctx.r[26].s64 = ctx.r[26].s64 + 1;
	// 8300D270: 7F2BE92E  stwx r25, r11, r29
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32), ctx.r[25].u32) };
	// 8300D274: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 8300D278: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8300D27C: 7F1A5840  cmplw cr6, r26, r11
	ctx.cr[6].compare_u32(ctx.r[26].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8300D280: 4198FF98  blt cr6, 0x8300d218
	if ctx.cr[6].lt {
	pc = 0x8300D218; continue 'dispatch;
	}
	// 8300D284: 933F0014  stw r25, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[25].u32 ) };
	// 8300D288: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8300D28C: 4819AF20  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300D290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300D290 size=128
    let mut pc: u32 = 0x8300D290;
    'dispatch: loop {
        match pc {
            0x8300D290 => {
    //   block [0x8300D290..0x8300D310)
	// 8300D290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300D294: 4819AED9  bl 0x831a816c
	ctx.lr = 0x8300D298;
	sub_831A8130(ctx, base);
	// 8300D298: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300D29C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8300D2A0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8300D2A4: 396B23D0  addi r11, r11, 0x23d0
	ctx.r[11].s64 = ctx.r[11].s64 + 9168;
	// 8300D2A8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8300D2AC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8300D2B0: 57C4103A  slwi r4, r30, 2
	ctx.r[4].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 8300D2B4: 98BF0004  stb r5, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[5].u8 ) };
	// 8300D2B8: 7CC33378  mr r3, r6
	ctx.r[3].u64 = ctx.r[6].u64;
	// 8300D2BC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8300D2C0: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 8300D2C4: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 8300D2C8: 90DF0014  stw r6, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[6].u32 ) };
	// 8300D2CC: 93BF0010  stw r29, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[29].u32 ) };
	// 8300D2D0: 81660000  lwz r11, 0(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300D2D4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8300D2D8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300D2DC: 4E800421  bctrl
	ctx.lr = 0x8300D2E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300D2E0: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8300D2E4: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 8300D2E8: 419A001C  beq cr6, 0x8300d304
	if ctx.cr[6].eq {
	pc = 0x8300D304; continue 'dispatch;
	}
	// 8300D2EC: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 8300D2F0: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8300D2F4: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8300D2F8: 7FAA592E  stwx r29, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[29].u32) };
	// 8300D2FC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8300D300: 4082FFF0  bne 0x8300d2f0
	if !ctx.cr[0].eq {
	pc = 0x8300D2F0; continue 'dispatch;
	}
	// 8300D304: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300D308: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8300D30C: 4819AEB0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300D310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300D310 size=32
    let mut pc: u32 = 0x8300D310;
    'dispatch: loop {
        match pc {
            0x8300D310 => {
    //   block [0x8300D310..0x8300D330)
	// 8300D310: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 8300D314: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300D318: 4182000C  beq 0x8300d324
	if ctx.cr[0].eq {
	pc = 0x8300D324; continue 'dispatch;
	}
	// 8300D31C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8300D320: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8300D324: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 8300D328: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8300D32C: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300D330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300D330 size=28
    let mut pc: u32 = 0x8300D330;
    'dispatch: loop {
        match pc {
            0x8300D330 => {
    //   block [0x8300D330..0x8300D34C)
	// 8300D330: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 8300D334: 8143000C  lwz r10, 0xc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 8300D338: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8300D33C: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8300D340: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 8300D344: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8300D348: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300D34C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300D34C size=12
    let mut pc: u32 = 0x8300D34C;
    'dispatch: loop {
        match pc {
            0x8300D34C => {
    //   block [0x8300D34C..0x8300D358)
	// 8300D34C: 810A0008  lwz r8, 8(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300D350: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8300D354: 48000028  b 0x8300d37c
	sub_8300D36C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300D358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300D358 size=20
    let mut pc: u32 = 0x8300D358;
    'dispatch: loop {
        match pc {
            0x8300D358 => {
    //   block [0x8300D358..0x8300D36C)
	// 8300D358: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8300D35C: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8300D360: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 8300D364: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8300D368: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300D36C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300D36C size=52
    let mut pc: u32 = 0x8300D36C;
    'dispatch: loop {
        match pc {
            0x8300D36C => {
    //   block [0x8300D36C..0x8300D3A0)
	// 8300D36C: 8123000C  lwz r9, 0xc(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 8300D370: 5568003E  slwi r8, r11, 0
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8300D374: 5508103A  slwi r8, r8, 2
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8300D378: 81290008  lwz r9, 8(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300D37C: 7D29402E  lwzx r9, r9, r8
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 8300D380: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8300D384: 419AFFD4  beq cr6, 0x8300d358
	if ctx.cr[6].eq {
		sub_8300D358(ctx, base);
		return;
	}
	// 8300D388: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 8300D38C: 814A0008  lwz r10, 8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300D390: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8300D394: 7D6B502E  lwzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8300D398: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8300D39C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300D3A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300D3A0 size=164
    let mut pc: u32 = 0x8300D3A0;
    'dispatch: loop {
        match pc {
            0x8300D3A0 => {
    //   block [0x8300D3A0..0x8300D444)
	// 8300D3A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300D3A4: 4819ADBD  bl 0x831a8160
	ctx.lr = 0x8300D3A8;
	sub_831A8130(ctx, base);
	// 8300D3A8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300D3AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8300D3B0: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 8300D3B4: 7F5BD378  mr r27, r26
	ctx.r[27].u64 = ctx.r[26].u64;
	// 8300D3B8: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8300D3BC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8300D3C0: 40990078  ble cr6, 0x8300d438
	if !ctx.cr[6].gt {
	pc = 0x8300D438; continue 'dispatch;
	}
	// 8300D3C4: 7F5DD378  mr r29, r26
	ctx.r[29].u64 = ctx.r[26].u64;
	// 8300D3C8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300D3CC: 7FCBE82E  lwzx r30, r11, r29
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 8300D3D0: 281E0000  cmplwi r30, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300D3D4: 41820048  beq 0x8300d41c
	if ctx.cr[0].eq {
	pc = 0x8300D41C; continue 'dispatch;
	}
	// 8300D3D8: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8300D3DC: 839E0004  lwz r28, 4(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8300D3E0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300D3E4: 41820024  beq 0x8300d408
	if ctx.cr[0].eq {
	pc = 0x8300D408; continue 'dispatch;
	}
	// 8300D3E8: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300D3EC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300D3F0: 41820018  beq 0x8300d408
	if ctx.cr[0].eq {
	pc = 0x8300D408; continue 'dispatch;
	}
	// 8300D3F4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300D3F8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8300D3FC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300D400: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300D404: 4E800421  bctrl
	ctx.lr = 0x8300D408;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300D408: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8300D40C: 4BFCAED5  bl 0x82fd82e0
	ctx.lr = 0x8300D410;
	sub_82FD82E0(ctx, base);
	// 8300D410: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 8300D414: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 8300D418: 409AFFC0  bne cr6, 0x8300d3d8
	if !ctx.cr[6].eq {
	pc = 0x8300D3D8; continue 'dispatch;
	}
	// 8300D41C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300D420: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 8300D424: 7F4BE92E  stwx r26, r11, r29
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32), ctx.r[26].u32) };
	// 8300D428: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 8300D42C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8300D430: 7F1B5840  cmplw cr6, r27, r11
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8300D434: 4198FF94  blt cr6, 0x8300d3c8
	if ctx.cr[6].lt {
	pc = 0x8300D3C8; continue 'dispatch;
	}
	// 8300D438: 935F001C  stw r26, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[26].u32 ) };
	// 8300D43C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8300D440: 4819AD70  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300D448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300D448 size=8
    let mut pc: u32 = 0x8300D448;
    'dispatch: loop {
        match pc {
            0x8300D448 => {
    //   block [0x8300D448..0x8300D450)
	// 8300D448: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8300D44C: 82142408  lwz r16, 0x2408(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(9224 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300D450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300D450 size=164
    let mut pc: u32 = 0x8300D450;
    'dispatch: loop {
        match pc {
            0x8300D450 => {
    //   block [0x8300D450..0x8300D4F4)
	// 8300D450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300D454: 4819AD11  bl 0x831a8164
	ctx.lr = 0x8300D458;
	sub_831A8130(ctx, base);
	// 8300D458: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 8300D45C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300D460: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8300D464: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8300D468: 396B23E8  addi r11, r11, 0x23e8
	ctx.r[11].s64 = ctx.r[11].s64 + 9192;
	// 8300D46C: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 8300D470: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8300D474: 897E0004  lbz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8300D478: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300D47C: 4182004C  beq 0x8300d4c8
	if ctx.cr[0].eq {
	pc = 0x8300D4C8; continue 'dispatch;
	}
	// 8300D480: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300D484: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8300D488: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8300D48C: 4099003C  ble cr6, 0x8300d4c8
	if !ctx.cr[6].gt {
	pc = 0x8300D4C8; continue 'dispatch;
	}
	// 8300D490: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8300D494: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 8300D498: 7F8BE82E  lwzx r28, r11, r29
	ctx.r[28].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 8300D49C: 281C0000  cmplwi r28, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300D4A0: 41820014  beq 0x8300d4b4
	if ctx.cr[0].eq {
	pc = 0x8300D4B4; continue 'dispatch;
	}
	// 8300D4A4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8300D4A8: 48030709  bl 0x8303dbb0
	ctx.lr = 0x8300D4AC;
	sub_8303DBB0(ctx, base);
	// 8300D4AC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8300D4B0: 4BFCAE31  bl 0x82fd82e0
	ctx.lr = 0x8300D4B4;
	sub_82FD82E0(ctx, base);
	// 8300D4B4: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300D4B8: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 8300D4BC: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 8300D4C0: 7F1B5840  cmplw cr6, r27, r11
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8300D4C4: 4198FFD0  blt cr6, 0x8300d494
	if ctx.cr[6].lt {
	pc = 0x8300D494; continue 'dispatch;
	}
	// 8300D4C8: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 8300D4CC: 809E0010  lwz r4, 0x10(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 8300D4D0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300D4D4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300D4D8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300D4DC: 4E800421  bctrl
	ctx.lr = 0x8300D4E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300D4E0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8300D4E4: 396B23D0  addi r11, r11, 0x23d0
	ctx.r[11].s64 = ctx.r[11].s64 + 9168;
	// 8300D4E8: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8300D4EC: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 8300D4F0: 4819ACC4  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300D4F4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300D4F4 size=40
    let mut pc: u32 = 0x8300D4F4;
    'dispatch: loop {
        match pc {
            0x8300D4F4 => {
    //   block [0x8300D4F4..0x8300D51C)
	// 8300D4F4: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 8300D4F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300D4FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300D500: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300D504: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 8300D508: 4BFFFCD9  bl 0x8300d1e0
	ctx.lr = 0x8300D50C;
	sub_8300D1E0(ctx, base);
	// 8300D50C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300D510: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300D514: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300D518: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300D520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300D520 size=100
    let mut pc: u32 = 0x8300D520;
    'dispatch: loop {
        match pc {
            0x8300D520 => {
    //   block [0x8300D520..0x8300D584)
	// 8300D520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300D524: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300D528: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8300D52C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300D530: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8300D534: 4BFFFCBD  bl 0x8300d1f0
	ctx.lr = 0x8300D538;
	sub_8300D1F0(ctx, base);
	// 8300D538: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300D53C: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300D540: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300D544: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300D548: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300D54C: 4E800421  bctrl
	ctx.lr = 0x8300D550;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300D550: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8300D554: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300D558: 41820018  beq 0x8300d570
	if ctx.cr[0].eq {
	pc = 0x8300D570; continue 'dispatch;
	}
	// 8300D55C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300D560: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8300D564: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300D568: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300D56C: 4E800421  bctrl
	ctx.lr = 0x8300D570;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300D570: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300D574: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300D578: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300D57C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8300D580: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300D588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300D588 size=220
    let mut pc: u32 = 0x8300D588;
    'dispatch: loop {
        match pc {
            0x8300D588 => {
    //   block [0x8300D588..0x8300D664)
	// 8300D588: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300D58C: 4819ABDD  bl 0x831a8168
	ctx.lr = 0x8300D590;
	sub_831A8130(ctx, base);
	// 8300D590: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300D594: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8300D598: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8300D59C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8300D5A0: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8300D5A4: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8300D5A8: 1D6B0003  mulli r11, r11, 3
	ctx.r[11].s64 = ctx.r[11].s64 * 3;
	// 8300D5AC: 556BF0BE  srwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shr(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8300D5B0: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8300D5B4: 41980008  blt cr6, 0x8300d5bc
	if ctx.cr[6].lt {
	pc = 0x8300D5BC; continue 'dispatch;
	}
	// 8300D5B8: 4805B431  bl 0x830689e8
	ctx.lr = 0x8300D5BC;
	sub_830689E8(ctx, base);
	// 8300D5BC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8300D5C0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8300D5C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300D5C8: 4BFECE99  bl 0x82ffa460
	ctx.lr = 0x8300D5CC;
	sub_82FFA460(ctx, base);
	// 8300D5CC: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8300D5D0: 41820038  beq 0x8300d608
	if ctx.cr[0].eq {
	pc = 0x8300D608; continue 'dispatch;
	}
	// 8300D5D4: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8300D5D8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300D5DC: 41820020  beq 0x8300d5fc
	if ctx.cr[0].eq {
	pc = 0x8300D5FC; continue 'dispatch;
	}
	// 8300D5E0: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300D5E4: 281F0000  cmplwi r31, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300D5E8: 41820014  beq 0x8300d5fc
	if ctx.cr[0].eq {
	pc = 0x8300D5FC; continue 'dispatch;
	}
	// 8300D5EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300D5F0: 480305C1  bl 0x8303dbb0
	ctx.lr = 0x8300D5F4;
	sub_8303DBB0(ctx, base);
	// 8300D5F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300D5F8: 4BFCACE9  bl 0x82fd82e0
	ctx.lr = 0x8300D5FC;
	sub_82FD82E0(ctx, base);
	// 8300D5FC: 939E0000  stw r28, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 8300D600: 93BE0008  stw r29, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 8300D604: 48000058  b 0x8300d65c
	pc = 0x8300D65C; continue 'dispatch;
	// 8300D608: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 8300D60C: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300D610: 4BFCAC89  bl 0x82fd8298
	ctx.lr = 0x8300D614;
	sub_82FD8298(ctx, base);
	// 8300D614: 81210050  lwz r9, 0x50(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8300D618: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300D61C: 41820024  beq 0x8300d640
	if ctx.cr[0].eq {
	pc = 0x8300D640; continue 'dispatch;
	}
	// 8300D620: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300D624: 5528103A  slwi r8, r9, 2
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8300D628: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8300D62C: 7D68582E  lwzx r11, r8, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8300D630: 93830000  stw r28, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 8300D634: 93A30008  stw r29, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 8300D638: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8300D63C: 48000008  b 0x8300d644
	pc = 0x8300D644; continue 'dispatch;
	// 8300D640: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8300D644: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300D648: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8300D64C: 7D49592E  stwx r10, r9, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u32) };
	// 8300D650: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8300D654: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8300D658: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8300D65C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8300D660: 4819AB58  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300D668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300D668 size=148
    let mut pc: u32 = 0x8300D668;
    'dispatch: loop {
        match pc {
            0x8300D668 => {
    //   block [0x8300D668..0x8300D6FC)
	// 8300D668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300D66C: 4819AAFD  bl 0x831a8168
	ctx.lr = 0x8300D670;
	sub_831A8130(ctx, base);
	// 8300D670: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300D674: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8300D678: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8300D67C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8300D680: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300D684: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8300D688: 41980030  blt cr6, 0x8300d6b8
	if ctx.cr[6].lt {
	pc = 0x8300D6B8; continue 'dispatch;
	}
	// 8300D68C: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 8300D690: 80FF0014  lwz r7, 0x14(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8300D694: 38C00074  li r6, 0x74
	ctx.r[6].s64 = 116;
	// 8300D698: 388B6B80  addi r4, r11, 0x6b80
	ctx.r[4].s64 = ctx.r[11].s64 + 27520;
	// 8300D69C: 38A00043  li r5, 0x43
	ctx.r[5].s64 = 67;
	// 8300D6A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8300D6A4: 4BFC32B5  bl 0x82fd0958
	ctx.lr = 0x8300D6A8;
	sub_82FD0958(ctx, base);
	// 8300D6A8: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8300D6AC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8300D6B0: 388BC49C  addi r4, r11, -0x3b64
	ctx.r[4].s64 = ctx.r[11].s64 + -15204;
	// 8300D6B4: 481A3575  bl 0x831b0c28
	ctx.lr = 0x8300D6B8;
	sub_831B0C28(ctx, base);
	// 8300D6B8: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8300D6BC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300D6C0: 41820028  beq 0x8300d6e8
	if ctx.cr[0].eq {
	pc = 0x8300D6E8; continue 'dispatch;
	}
	// 8300D6C4: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8300D6C8: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8300D6CC: 7FAA582E  lwzx r29, r10, r11
	ctx.r[29].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8300D6D0: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300D6D4: 41820014  beq 0x8300d6e8
	if ctx.cr[0].eq {
	pc = 0x8300D6E8; continue 'dispatch;
	}
	// 8300D6D8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8300D6DC: 480304D5  bl 0x8303dbb0
	ctx.lr = 0x8300D6E0;
	sub_8303DBB0(ctx, base);
	// 8300D6E0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8300D6E4: 4BFCABFD  bl 0x82fd82e0
	ctx.lr = 0x8300D6E8;
	sub_82FD82E0(ctx, base);
	// 8300D6E8: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8300D6EC: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8300D6F0: 7F8A592E  stwx r28, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[28].u32) };
	// 8300D6F4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8300D6F8: 4819AAC0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300D700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300D700 size=124
    let mut pc: u32 = 0x8300D700;
    'dispatch: loop {
        match pc {
            0x8300D700 => {
    //   block [0x8300D700..0x8300D77C)
	// 8300D700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300D704: 4819AA61  bl 0x831a8164
	ctx.lr = 0x8300D708;
	sub_831A8130(ctx, base);
	// 8300D708: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300D70C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8300D710: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8300D714: 7F7DDB78  mr r29, r27
	ctx.r[29].u64 = ctx.r[27].u64;
	// 8300D718: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300D71C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8300D720: 40990050  ble cr6, 0x8300d770
	if !ctx.cr[6].gt {
	pc = 0x8300D770; continue 'dispatch;
	}
	// 8300D724: 7F7EDB78  mr r30, r27
	ctx.r[30].u64 = ctx.r[27].u64;
	// 8300D728: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8300D72C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300D730: 41820024  beq 0x8300d754
	if ctx.cr[0].eq {
	pc = 0x8300D754; continue 'dispatch;
	}
	// 8300D734: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8300D738: 7F8BF02E  lwzx r28, r11, r30
	ctx.r[28].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 8300D73C: 281C0000  cmplwi r28, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300D740: 41820014  beq 0x8300d754
	if ctx.cr[0].eq {
	pc = 0x8300D754; continue 'dispatch;
	}
	// 8300D744: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8300D748: 48030469  bl 0x8303dbb0
	ctx.lr = 0x8300D74C;
	sub_8303DBB0(ctx, base);
	// 8300D74C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8300D750: 4BFCAB91  bl 0x82fd82e0
	ctx.lr = 0x8300D754;
	sub_82FD82E0(ctx, base);
	// 8300D754: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8300D758: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 8300D75C: 7F6BF12E  stwx r27, r11, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32), ctx.r[27].u32) };
	// 8300D760: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 8300D764: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300D768: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8300D76C: 4198FFBC  blt cr6, 0x8300d728
	if ctx.cr[6].lt {
	pc = 0x8300D728; continue 'dispatch;
	}
	// 8300D770: 937F0008  stw r27, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[27].u32 ) };
	// 8300D774: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8300D778: 4819AA3C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300D780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300D780 size=260
    let mut pc: u32 = 0x8300D780;
    'dispatch: loop {
        match pc {
            0x8300D780 => {
    //   block [0x8300D780..0x8300D884)
	// 8300D780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300D784: 4819A9E9  bl 0x831a816c
	ctx.lr = 0x8300D788;
	sub_831A8130(ctx, base);
	// 8300D788: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300D78C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8300D790: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8300D794: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300D798: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8300D79C: 41980030  blt cr6, 0x8300d7cc
	if ctx.cr[6].lt {
	pc = 0x8300D7CC; continue 'dispatch;
	}
	// 8300D7A0: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 8300D7A4: 80FF0014  lwz r7, 0x14(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8300D7A8: 38C00074  li r6, 0x74
	ctx.r[6].s64 = 116;
	// 8300D7AC: 388B6B80  addi r4, r11, 0x6b80
	ctx.r[4].s64 = ctx.r[11].s64 + 27520;
	// 8300D7B0: 38A00090  li r5, 0x90
	ctx.r[5].s64 = 144;
	// 8300D7B4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8300D7B8: 4BFC31A1  bl 0x82fd0958
	ctx.lr = 0x8300D7BC;
	sub_82FD0958(ctx, base);
	// 8300D7BC: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8300D7C0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8300D7C4: 388BC49C  addi r4, r11, -0x3b64
	ctx.r[4].s64 = ctx.r[11].s64 + -15204;
	// 8300D7C8: 481A3461  bl 0x831b0c28
	ctx.lr = 0x8300D7CC;
	sub_831B0C28(ctx, base);
	// 8300D7CC: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8300D7D0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300D7D4: 41820028  beq 0x8300d7fc
	if ctx.cr[0].eq {
	pc = 0x8300D7FC; continue 'dispatch;
	}
	// 8300D7D8: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8300D7DC: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8300D7E0: 7FAA582E  lwzx r29, r10, r11
	ctx.r[29].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8300D7E4: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300D7E8: 41820014  beq 0x8300d7fc
	if ctx.cr[0].eq {
	pc = 0x8300D7FC; continue 'dispatch;
	}
	// 8300D7EC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8300D7F0: 480303C1  bl 0x8303dbb0
	ctx.lr = 0x8300D7F4;
	sub_8303DBB0(ctx, base);
	// 8300D7F4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8300D7F8: 4BFCAAE9  bl 0x82fd82e0
	ctx.lr = 0x8300D7FC;
	sub_82FD82E0(ctx, base);
	// 8300D7FC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300D800: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8300D804: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8300D808: 409A0018  bne cr6, 0x8300d820
	if !ctx.cr[6].eq {
	pc = 0x8300D820; continue 'dispatch;
	}
	// 8300D80C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8300D810: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8300D814: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8300D818: 7D2A592E  stwx r9, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u32) };
	// 8300D81C: 48000054  b 0x8300d870
	pc = 0x8300D870; continue 'dispatch;
	// 8300D820: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 8300D824: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8300D828: 40980030  bge cr6, 0x8300d858
	if !ctx.cr[6].lt {
	pc = 0x8300D858; continue 'dispatch;
	}
	// 8300D82C: 57CB103A  slwi r11, r30, 2
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8300D830: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8300D834: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8300D838: 7D295A14  add r9, r9, r11
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 8300D83C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8300D840: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 8300D844: 91090000  stw r8, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8300D848: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300D84C: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 8300D850: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8300D854: 4198FFDC  blt cr6, 0x8300d830
	if ctx.cr[6].lt {
	pc = 0x8300D830; continue 'dispatch;
	}
	// 8300D858: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300D85C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8300D860: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8300D864: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8300D868: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8300D86C: 912BFFFC  stw r9, -4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), ctx.r[9].u32 ) };
	// 8300D870: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300D874: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8300D878: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8300D87C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8300D880: 4819A93C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300D888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300D888 size=104
    let mut pc: u32 = 0x8300D888;
    'dispatch: loop {
        match pc {
            0x8300D888 => {
    //   block [0x8300D888..0x8300D8F0)
	// 8300D888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300D88C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300D890: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8300D894: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300D898: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300D89C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300D8A0: 4182003C  beq 0x8300d8dc
	if ctx.cr[0].eq {
	pc = 0x8300D8DC; continue 'dispatch;
	}
	// 8300D8A4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8300D8A8: 89430004  lbz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8300D8AC: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300D8B0: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8300D8B4: 41820028  beq 0x8300d8dc
	if ctx.cr[0].eq {
	pc = 0x8300D8DC; continue 'dispatch;
	}
	// 8300D8B8: 81430010  lwz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 8300D8BC: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8300D8C0: 7FEB502E  lwzx r31, r11, r10
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8300D8C4: 281F0000  cmplwi r31, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300D8C8: 41820014  beq 0x8300d8dc
	if ctx.cr[0].eq {
	pc = 0x8300D8DC; continue 'dispatch;
	}
	// 8300D8CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300D8D0: 480302E1  bl 0x8303dbb0
	ctx.lr = 0x8300D8D4;
	sub_8303DBB0(ctx, base);
	// 8300D8D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300D8D8: 4BFCAA09  bl 0x82fd82e0
	ctx.lr = 0x8300D8DC;
	sub_82FD82E0(ctx, base);
	// 8300D8DC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300D8E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300D8E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300D8E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8300D8EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300D8F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300D8F0 size=132
    let mut pc: u32 = 0x8300D8F0;
    'dispatch: loop {
        match pc {
            0x8300D8F0 => {
    //   block [0x8300D8F0..0x8300D974)
	// 8300D8F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300D8F4: 4819A875  bl 0x831a8168
	ctx.lr = 0x8300D8F8;
	sub_831A8130(ctx, base);
	// 8300D8F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300D8FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8300D900: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8300D904: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300D908: 4182004C  beq 0x8300d954
	if ctx.cr[0].eq {
	pc = 0x8300D954; continue 'dispatch;
	}
	// 8300D90C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300D910: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8300D914: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8300D918: 4099003C  ble cr6, 0x8300d954
	if !ctx.cr[6].gt {
	pc = 0x8300D954; continue 'dispatch;
	}
	// 8300D91C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8300D920: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8300D924: 7FABF02E  lwzx r29, r11, r30
	ctx.r[29].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 8300D928: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300D92C: 41820014  beq 0x8300d940
	if ctx.cr[0].eq {
	pc = 0x8300D940; continue 'dispatch;
	}
	// 8300D930: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8300D934: 4803027D  bl 0x8303dbb0
	ctx.lr = 0x8300D938;
	sub_8303DBB0(ctx, base);
	// 8300D938: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8300D93C: 4BFCA9A5  bl 0x82fd82e0
	ctx.lr = 0x8300D940;
	sub_82FD82E0(ctx, base);
	// 8300D940: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300D944: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 8300D948: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 8300D94C: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8300D950: 4198FFD0  blt cr6, 0x8300d920
	if ctx.cr[6].lt {
	pc = 0x8300D920; continue 'dispatch;
	}
	// 8300D954: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8300D958: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8300D95C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300D960: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300D964: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300D968: 4E800421  bctrl
	ctx.lr = 0x8300D96C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300D96C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8300D970: 4819A848  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300D978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300D978 size=124
    let mut pc: u32 = 0x8300D978;
    'dispatch: loop {
        match pc {
            0x8300D978 => {
    //   block [0x8300D978..0x8300D9F4)
	// 8300D978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300D97C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300D980: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8300D984: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300D988: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8300D98C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300D990: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8300D994: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300D998: 4E800421  bctrl
	ctx.lr = 0x8300D99C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300D99C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8300D9A0: 40820030  bne 0x8300d9d0
	if !ctx.cr[0].eq {
	pc = 0x8300D9D0; continue 'dispatch;
	}
	// 8300D9A4: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8300D9A8: 80FF0014  lwz r7, 0x14(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8300D9AC: 38C0001E  li r6, 0x1e
	ctx.r[6].s64 = 30;
	// 8300D9B0: 388BB9A0  addi r4, r11, -0x4660
	ctx.r[4].s64 = ctx.r[11].s64 + -18016;
	// 8300D9B4: 38A00287  li r5, 0x287
	ctx.r[5].s64 = 647;
	// 8300D9B8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8300D9BC: 4BFD336D  bl 0x82fe0d28
	ctx.lr = 0x8300D9C0;
	sub_82FE0D28(ctx, base);
	// 8300D9C0: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8300D9C4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8300D9C8: 388BC690  addi r4, r11, -0x3970
	ctx.r[4].s64 = ctx.r[11].s64 + -14704;
	// 8300D9CC: 481A325D  bl 0x831b0c28
	ctx.lr = 0x8300D9D0;
	sub_831B0C28(ctx, base);
	// 8300D9D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300D9D4: 83FF0008  lwz r31, 8(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300D9D8: 4BFECD11  bl 0x82ffa6e8
	ctx.lr = 0x8300D9DC;
	sub_82FFA6E8(ctx, base);
	// 8300D9DC: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300D9E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8300D9E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300D9E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300D9EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8300D9F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300D9F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300D9F8 size=76
    let mut pc: u32 = 0x8300D9F8;
    'dispatch: loop {
        match pc {
            0x8300D9F8 => {
    //   block [0x8300D9F8..0x8300DA44)
	// 8300D9F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300D9FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300DA00: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8300DA04: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8300DA08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300DA0C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8300DA10: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8300DA14: 4BFFFA3D  bl 0x8300d450
	ctx.lr = 0x8300DA18;
	sub_8300D450(ctx, base);
	// 8300DA18: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8300DA1C: 4182000C  beq 0x8300da28
	if ctx.cr[0].eq {
	pc = 0x8300DA28; continue 'dispatch;
	}
	// 8300DA20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300DA24: 4BFCA8BD  bl 0x82fd82e0
	ctx.lr = 0x8300DA28;
	sub_82FD82E0(ctx, base);
	// 8300DA28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300DA2C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8300DA30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300DA34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300DA38: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8300DA3C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8300DA40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300DA48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300DA48 size=124
    let mut pc: u32 = 0x8300DA48;
    'dispatch: loop {
        match pc {
            0x8300DA48 => {
    //   block [0x8300DA48..0x8300DAC4)
	// 8300DA48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300DA4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300DA50: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8300DA54: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300DA58: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8300DA5C: 4BFFF945  bl 0x8300d3a0
	ctx.lr = 0x8300DA60;
	sub_8300D3A0(ctx, base);
	// 8300DA60: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300DA64: 809F0014  lwz r4, 0x14(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8300DA68: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300DA6C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300DA70: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300DA74: 4E800421  bctrl
	ctx.lr = 0x8300DA78;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300DA78: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300DA7C: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300DA80: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300DA84: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300DA88: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300DA8C: 4E800421  bctrl
	ctx.lr = 0x8300DA90;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300DA90: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8300DA94: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300DA98: 41820018  beq 0x8300dab0
	if ctx.cr[0].eq {
	pc = 0x8300DAB0; continue 'dispatch;
	}
	// 8300DA9C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300DAA0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8300DAA4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300DAA8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300DAAC: 4E800421  bctrl
	ctx.lr = 0x8300DAB0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300DAB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300DAB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300DAB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300DABC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8300DAC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300DAC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300DAC8 size=60
    let mut pc: u32 = 0x8300DAC8;
    'dispatch: loop {
        match pc {
            0x8300DAC8 => {
    //   block [0x8300DAC8..0x8300DB04)
	// 8300DAC8: 3D408216  lis r10, -0x7dea
	ctx.r[10].s64 = -2112487424;
	// 8300DACC: 81640014  lwz r11, 0x14(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) } as u64;
	// 8300DAD0: 81240034  lwz r9, 0x34(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(52 as u32) ) } as u64;
	// 8300DAD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8300DAD8: 394A2A60  addi r10, r10, 0x2a60
	ctx.r[10].s64 = ctx.r[10].s64 + 10848;
	// 8300DADC: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8300DAE0: 9101FFF0  stw r8, -0x10(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[8].u32 ) };
	// 8300DAE4: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8300DAE8: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8300DAEC: 9123000C  stw r9, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 8300DAF0: 7D4A0034  cntlzw r10, r10
	ctx.r[10].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 8300DAF4: 554ADFFE  rlwinm r10, r10, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 8300DAF8: 694B0001  xori r11, r10, 1
	ctx.r[11].u64 = ctx.r[10].u64 ^ 1;
	// 8300DAFC: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8300DB00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300DB08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300DB08 size=64
    let mut pc: u32 = 0x8300DB08;
    'dispatch: loop {
        match pc {
            0x8300DB08 => {
    //   block [0x8300DB08..0x8300DB48)
	// 8300DB08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300DB0C: 4819A661  bl 0x831a816c
	ctx.lr = 0x8300DB10;
	sub_831A8130(ctx, base);
	// 8300DB10: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300DB14: 39650003  addi r11, r5, 3
	ctx.r[11].s64 = ctx.r[5].s64 + 3;
	// 8300DB18: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8300DB1C: 557D103A  slwi r29, r11, 2
	ctx.r[29].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 8300DB20: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8300DB24: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8300DB28: 7D7DF82E  lwzx r11, r29, r31
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 8300DB2C: 808B0008  lwz r4, 8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300DB30: 4BDF8B21  bl 0x82e06650
	ctx.lr = 0x8300DB34;
	sub_82E06650(ctx, base);
	// 8300DB34: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8300DB38: 7C7DF82E  lwzx r3, r29, r31
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 8300DB3C: 4802D615  bl 0x8303b150
	ctx.lr = 0x8300DB40;
	sub_8303B150(ctx, base);
	// 8300DB40: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8300DB44: 4819A678  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300DB48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300DB48 size=104
    let mut pc: u32 = 0x8300DB48;
    'dispatch: loop {
        match pc {
            0x8300DB48 => {
    //   block [0x8300DB48..0x8300DBB0)
	// 8300DB48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300DB4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300DB50: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8300DB54: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8300DB58: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300DB5C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8300DB60: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8300DB64: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8300DB68: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 8300DB6C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8300DB70: 4BFEC8F1  bl 0x82ffa460
	ctx.lr = 0x8300DB74;
	sub_82FFA460(ctx, base);
	// 8300DB74: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300DB78: 41820010  beq 0x8300db88
	if ctx.cr[0].eq {
	pc = 0x8300DB88; continue 'dispatch;
	}
	// 8300DB7C: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300DB80: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300DB84: 40820014  bne 0x8300db98
	if !ctx.cr[0].eq {
	pc = 0x8300DB98; continue 'dispatch;
	}
	// 8300DB88: 83FF0090  lwz r31, 0x90(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(144 as u32) ) } as u64;
	// 8300DB8C: 281F0000  cmplwi r31, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300DB90: 4082FFD4  bne 0x8300db64
	if !ctx.cr[0].eq {
	pc = 0x8300DB64; continue 'dispatch;
	}
	// 8300DB94: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8300DB98: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8300DB9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300DBA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300DBA4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8300DBA8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8300DBAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300DBB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300DBB0 size=120
    let mut pc: u32 = 0x8300DBB0;
    'dispatch: loop {
        match pc {
            0x8300DBB0 => {
    //   block [0x8300DBB0..0x8300DC28)
	// 8300DBB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300DBB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300DBB8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8300DBBC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8300DBC0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300DBC4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8300DBC8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8300DBCC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8300DBD0: 817F0088  lwz r11, 0x88(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(136 as u32) ) } as u64;
	// 8300DBD4: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8300DBD8: 4BFEC889  bl 0x82ffa460
	ctx.lr = 0x8300DBDC;
	sub_82FFA460(ctx, base);
	// 8300DBDC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300DBE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8300DBE4: 41820008  beq 0x8300dbec
	if ctx.cr[0].eq {
	pc = 0x8300DBEC; continue 'dispatch;
	}
	// 8300DBE8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300DBEC: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8300DBF0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8300DBF4: 409A001C  bne cr6, 0x8300dc10
	if !ctx.cr[6].eq {
	pc = 0x8300DC10; continue 'dispatch;
	}
	// 8300DBF8: 817F0090  lwz r11, 0x90(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(144 as u32) ) } as u64;
	// 8300DBFC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300DC00: 41820010  beq 0x8300dc10
	if ctx.cr[0].eq {
	pc = 0x8300DC10; continue 'dispatch;
	}
	// 8300DC04: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8300DC08: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8300DC0C: 4BFFFFA5  bl 0x8300dbb0
	ctx.lr = 0x8300DC10;
	sub_8300DBB0(ctx, base);
	// 8300DC10: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8300DC14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300DC18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300DC1C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8300DC20: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8300DC24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300DC28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300DC28 size=8
    let mut pc: u32 = 0x8300DC28;
    'dispatch: loop {
        match pc {
            0x8300DC28 => {
    //   block [0x8300DC28..0x8300DC30)
	// 8300DC28: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8300DC2C: 82142458  lwz r16, 0x2458(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(9304 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300DC30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300DC30 size=188
    let mut pc: u32 = 0x8300DC30;
    'dispatch: loop {
        match pc {
            0x8300DC30 => {
    //   block [0x8300DC30..0x8300DCEC)
	// 8300DC30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300DC34: 4819A529  bl 0x831a815c
	ctx.lr = 0x8300DC38;
	sub_831A8130(ctx, base);
	// 8300DC38: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 8300DC3C: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300DC40: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8300DC44: 7D1E4378  mr r30, r8
	ctx.r[30].u64 = ctx.r[8].u64;
	// 8300DC48: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 8300DC4C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8300DC50: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 8300DC54: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 8300DC58: 90DC0004  stw r6, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 8300DC5C: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 8300DC60: 93DF00DC  stw r30, 0xdc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(220 as u32), ctx.r[30].u32 ) };
	// 8300DC64: 93DC0000  stw r30, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 8300DC68: 4BFCA631  bl 0x82fd8298
	ctx.lr = 0x8300DC6C;
	sub_82FD8298(ctx, base);
	// 8300DC6C: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8300DC70: 93BF0050  stw r29, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 8300DC74: 4182002C  beq 0x8300dca0
	if ctx.cr[0].eq {
	pc = 0x8300DCA0; continue 'dispatch;
	}
	// 8300DC78: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8300DC7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8300DC80: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8300DC84: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8300DC88: 4803EB69  bl 0x8304c7f0
	ctx.lr = 0x8300DC8C;
	sub_8304C7F0(ctx, base);
	// 8300DC8C: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8300DC90: 394B2990  addi r10, r11, 0x2990
	ctx.r[10].s64 = ctx.r[11].s64 + 10640;
	// 8300DC94: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 8300DC98: 915D0000  stw r10, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8300DC9C: 48000008  b 0x8300dca4
	pc = 0x8300DCA4; continue 'dispatch;
	// 8300DCA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8300DCA4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8300DCA8: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 8300DCAC: 917C0008  stw r11, 8(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8300DCB0: 4BFCA5E9  bl 0x82fd8298
	ctx.lr = 0x8300DCB4;
	sub_82FD8298(ctx, base);
	// 8300DCB4: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8300DCB8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300DCBC: 4182001C  beq 0x8300dcd8
	if ctx.cr[0].eq {
	pc = 0x8300DCD8; continue 'dispatch;
	}
	// 8300DCC0: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8300DCC4: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 8300DCC8: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8300DCCC: 4802D4E5  bl 0x8303b1b0
	ctx.lr = 0x8300DCD0;
	sub_8303B1B0(ctx, base);
	// 8300DCD0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8300DCD4: 48000008  b 0x8300dcdc
	pc = 0x8300DCDC; continue 'dispatch;
	// 8300DCD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8300DCDC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8300DCE0: 917C000C  stw r11, 0xc(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8300DCE4: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 8300DCE8: 4819A4C4  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300DCEC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300DCEC size=44
    let mut pc: u32 = 0x8300DCEC;
    'dispatch: loop {
        match pc {
            0x8300DCEC => {
    //   block [0x8300DCEC..0x8300DD18)
	// 8300DCEC: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 8300DCF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300DCF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300DCF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300DCFC: 809F00DC  lwz r4, 0xdc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(220 as u32) ) } as u64;
	// 8300DD00: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8300DD04: 4BFCA5DD  bl 0x82fd82e0
	ctx.lr = 0x8300DD08;
	sub_82FD82E0(ctx, base);
	// 8300DD08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300DD0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300DD10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300DD14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300DD18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300DD18 size=44
    let mut pc: u32 = 0x8300DD18;
    'dispatch: loop {
        match pc {
            0x8300DD18 => {
    //   block [0x8300DD18..0x8300DD44)
	// 8300DD18: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 8300DD1C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300DD20: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300DD24: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300DD28: 809F00DC  lwz r4, 0xdc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(220 as u32) ) } as u64;
	// 8300DD2C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8300DD30: 4BFCA5B1  bl 0x82fd82e0
	ctx.lr = 0x8300DD34;
	sub_82FD82E0(ctx, base);
	// 8300DD34: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300DD38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300DD3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300DD40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300DD48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300DD48 size=88
    let mut pc: u32 = 0x8300DD48;
    'dispatch: loop {
        match pc {
            0x8300DD48 => {
    //   block [0x8300DD48..0x8300DDA0)
	// 8300DD48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300DD4C: 4819A41D  bl 0x831a8168
	ctx.lr = 0x8300DD50;
	sub_831A8130(ctx, base);
	// 8300DD50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300DD54: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8300DD58: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8300DD5C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8300DD60: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8300DD64: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8300DD68: 4802D3E9  bl 0x8303b150
	ctx.lr = 0x8300DD6C;
	sub_8303B150(ctx, base);
	// 8300DD6C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8300DD70: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8300DD74: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300DD78: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8300DD7C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300DD80: 4E800421  bctrl
	ctx.lr = 0x8300DD84;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300DD84: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8300DD88: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8300DD8C: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8300DD90: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8300DD94: 48031F6D  bl 0x8303fd00
	ctx.lr = 0x8300DD98;
	sub_8303FD00(ctx, base);
	// 8300DD98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8300DD9C: 4819A41C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300DDA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300DDA0 size=8
    let mut pc: u32 = 0x8300DDA0;
    'dispatch: loop {
        match pc {
            0x8300DDA0 => {
    //   block [0x8300DDA0..0x8300DDA8)
	// 8300DDA0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8300DDA4: 821424B8  lwz r16, 0x24b8(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(9400 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300DDA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300DDA8 size=184
    let mut pc: u32 = 0x8300DDA8;
    'dispatch: loop {
        match pc {
            0x8300DDA8 => {
    //   block [0x8300DDA8..0x8300DE60)
	// 8300DDA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300DDAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300DDB0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8300DDB4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8300DDB8: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 8300DDBC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300DDC0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8300DDC4: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 8300DDC8: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 8300DDCC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8300DDD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8300DDD4: 98BE0004  stb r5, 4(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[5].u8 ) };
	// 8300DDD8: 396B24A0  addi r11, r11, 0x24a0
	ctx.r[11].s64 = ctx.r[11].s64 + 9376;
	// 8300DDDC: 909E000C  stw r4, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 8300DDE0: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 8300DDE4: 90FE0018  stw r7, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[7].u32 ) };
	// 8300DDE8: 915E0008  stw r10, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8300DDEC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8300DDF0: 409A002C  bne cr6, 0x8300de1c
	if !ctx.cr[6].eq {
	pc = 0x8300DE1C; continue 'dispatch;
	}
	// 8300DDF4: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8300DDF8: 38C0000F  li r6, 0xf
	ctx.r[6].s64 = 15;
	// 8300DDFC: 388B2368  addi r4, r11, 0x2368
	ctx.r[4].s64 = ctx.r[11].s64 + 9064;
	// 8300DE00: 38A001B7  li r5, 0x1b7
	ctx.r[5].s64 = 439;
	// 8300DE04: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 8300DE08: 4BFD30E9  bl 0x82fe0ef0
	ctx.lr = 0x8300DE0C;
	sub_82FE0EF0(ctx, base);
	// 8300DE0C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8300DE10: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 8300DE14: 388BC654  addi r4, r11, -0x39ac
	ctx.r[4].s64 = ctx.r[11].s64 + -14764;
	// 8300DE18: 481A2E11  bl 0x831b0c28
	ctx.lr = 0x8300DE1C;
	sub_831B0C28(ctx, base);
	// 8300DE1C: 8164001C  lwz r11, 0x1c(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(28 as u32) ) } as u64;
	// 8300DE20: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 8300DE24: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8300DE28: 915E0010  stw r10, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 8300DE2C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8300DE30: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8300DE34: 913E0014  stw r9, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 8300DE38: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 8300DE3C: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8300DE40: 4BFFF4D1  bl 0x8300d310
	ctx.lr = 0x8300DE44;
	sub_8300D310(ctx, base);
	// 8300DE44: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8300DE48: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 8300DE4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300DE50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300DE54: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8300DE58: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8300DE5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300DE60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300DE60 size=40
    let mut pc: u32 = 0x8300DE60;
    'dispatch: loop {
        match pc {
            0x8300DE60 => {
    //   block [0x8300DE60..0x8300DE88)
	// 8300DE60: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 8300DE64: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300DE68: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300DE6C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300DE70: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 8300DE74: 4803E8ED  bl 0x8304c760
	ctx.lr = 0x8300DE78;
	sub_8304C760(ctx, base);
	// 8300DE78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300DE7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300DE80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300DE84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300DE88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300DE88 size=196
    let mut pc: u32 = 0x8300DE88;
    'dispatch: loop {
        match pc {
            0x8300DE88 => {
    //   block [0x8300DE88..0x8300DF4C)
	// 8300DE88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300DE8C: 4819A2D5  bl 0x831a8160
	ctx.lr = 0x8300DE90;
	sub_831A8130(ctx, base);
	// 8300DE90: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300DE94: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 8300DE98: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8300DE9C: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 8300DEA0: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 8300DEA4: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 8300DEA8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300DEAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300DEB0: 837E0080  lwz r27, 0x80(r30)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(128 as u32) ) } as u64;
	// 8300DEB4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8300DEB8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300DEBC: 4E800421  bctrl
	ctx.lr = 0x8300DEC0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300DEC0: 397D0003  addi r11, r29, 3
	ctx.r[11].s64 = ctx.r[29].s64 + 3;
	// 8300DEC4: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8300DEC8: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8300DECC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8300DED0: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 8300DED4: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 8300DED8: 4BFFFE71  bl 0x8300dd48
	ctx.lr = 0x8300DEDC;
	sub_8300DD48(ctx, base);
	// 8300DEDC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300DEE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300DEE4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8300DEE8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300DEEC: 4E800421  bctrl
	ctx.lr = 0x8300DEF0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300DEF0: 397D0012  addi r11, r29, 0x12
	ctx.r[11].s64 = ctx.r[29].s64 + 18;
	// 8300DEF4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8300DEF8: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8300DEFC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8300DF00: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 8300DF04: 4801EF45  bl 0x8302ce48
	ctx.lr = 0x8300DF08;
	sub_8302CE48(ctx, base);
	// 8300DF08: 578B063F  clrlwi. r11, r28, 0x18
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8300DF0C: 41820038  beq 0x8300df44
	if ctx.cr[0].eq {
	pc = 0x8300DF44; continue 'dispatch;
	}
	// 8300DF10: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300DF14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8300DF18: 83DE0080  lwz r30, 0x80(r30)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(128 as u32) ) } as u64;
	// 8300DF1C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8300DF20: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300DF24: 4E800421  bctrl
	ctx.lr = 0x8300DF28;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300DF28: 397D0011  addi r11, r29, 0x11
	ctx.r[11].s64 = ctx.r[29].s64 + 17;
	// 8300DF2C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8300DF30: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8300DF34: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8300DF38: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8300DF3C: 7C6BD02E  lwzx r3, r11, r26
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 8300DF40: 4BFFFE09  bl 0x8300dd48
	ctx.lr = 0x8300DF44;
	sub_8300DD48(ctx, base);
	// 8300DF44: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8300DF48: 4819A268  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300DF50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8300DF50 size=8
    let mut pc: u32 = 0x8300DF50;
    'dispatch: loop {
        match pc {
            0x8300DF50 => {
    //   block [0x8300DF50..0x8300DF58)
	// 8300DF50: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8300DF54: 821424F0  lwz r16, 0x24f0(r20)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(9456 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300DF58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300DF58 size=336
    let mut pc: u32 = 0x8300DF58;
    'dispatch: loop {
        match pc {
            0x8300DF58 => {
    //   block [0x8300DF58..0x8300E0A8)
	// 8300DF58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300DF5C: 4819A209  bl 0x831a8164
	ctx.lr = 0x8300DF60;
	sub_831A8130(ctx, base);
	// 8300DF60: 3BE1FF50  addi r31, r1, -0xb0
	ctx.r[31].s64 = ctx.r[1].s64 + -176;
	// 8300DF64: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300DF68: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8300DF6C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8300DF70: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8300DF74: 388B8158  addi r4, r11, -0x7ea8
	ctx.r[4].s64 = ctx.r[11].s64 + -32424;
	// 8300DF78: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8300DF7C: 807E007C  lwz r3, 0x7c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(124 as u32) ) } as u64;
	// 8300DF80: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300DF84: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8300DF88: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8300DF8C: 4E800421  bctrl
	ctx.lr = 0x8300DF90;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8300DF90: 480316D9  bl 0x8303f668
	ctx.lr = 0x8300DF94;
	sub_8303F668(ctx, base);
	// 8300DF94: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8300DF98: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8300DF9C: 807E0088  lwz r3, 0x88(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(136 as u32) ) } as u64;
	// 8300DFA0: 4802E8F1  bl 0x8303c890
	ctx.lr = 0x8300DFA4;
	sub_8303C890(ctx, base);
	// 8300DFA4: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8300DFA8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8300DFAC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8300DFB0: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 8300DFB4: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8300DFB8: 4BFFFED1  bl 0x8300de88
	ctx.lr = 0x8300DFBC;
	sub_8300DE88(ctx, base);
	// 8300DFBC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8300DFC0: 38BF0050  addi r5, r31, 0x50
	ctx.r[5].s64 = ctx.r[31].s64 + 80;
	// 8300DFC4: 388BD8FC  addi r4, r11, -0x2704
	ctx.r[4].s64 = ctx.r[11].s64 + -9988;
	// 8300DFC8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8300DFCC: 4BFEC495  bl 0x82ffa460
	ctx.lr = 0x8300DFD0;
	sub_82FFA460(ctx, base);
	// 8300DFD0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8300DFD4: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8300DFD8: 41820008  beq 0x8300dfe0
	if ctx.cr[0].eq {
	pc = 0x8300DFE0; continue 'dispatch;
	}
	// 8300DFDC: 83630000  lwz r27, 0(r3)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300DFE0: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 8300DFE4: 807E0088  lwz r3, 0x88(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(136 as u32) ) } as u64;
	// 8300DFE8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8300DFEC: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8300DFF0: 4802E411  bl 0x8303c400
	ctx.lr = 0x8300DFF4;
	sub_8303C400(ctx, base);
	// 8300DFF4: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8300DFF8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8300DFFC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8300E000: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 8300E004: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8300E008: 4BFFFE81  bl 0x8300de88
	ctx.lr = 0x8300E00C;
	sub_8300DE88(ctx, base);
	// 8300E00C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8300E010: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8300E014: 80DE0000  lwz r6, 0(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8300E018: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 8300E01C: 4BFD8B25  bl 0x82fe6b40
	ctx.lr = 0x8300E020;
	sub_82FE6B40(ctx, base);
	// 8300E020: 817F0068  lwz r11, 0x68(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 8300E024: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8300E028: 409A001C  bne cr6, 0x8300e044
	if !ctx.cr[6].eq {
	pc = 0x8300E044; continue 'dispatch;
	}
	// 8300E02C: 817F0070  lwz r11, 0x70(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 8300E030: 815F006C  lwz r10, 0x6c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 8300E034: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8300E038: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8300E03C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8300E040: 419A0008  beq cr6, 0x8300e048
	if ctx.cr[6].eq {
	pc = 0x8300E048; continue 'dispatch;
	}
	// 8300E044: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8300E048: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8300E04C: 41820044  beq 0x8300e090
	if ctx.cr[0].eq {
	pc = 0x8300E090; continue 'dispatch;
	}
	// 8300E050: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 8300E054: 4BFFF925  bl 0x8300d978
	ctx.lr = 0x8300E058;
	sub_8300D978(ctx, base);
	// 8300E058: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8300E05C: 7F04D840  cmplw cr6, r4, r27
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8300E060: 419AFFC0  beq cr6, 0x8300e020
	if ctx.cr[6].eq {
	pc = 0x8300E020; continue 'dispatch;
	}
	// 8300E064: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8300E068: 807E0088  lwz r3, 0x88(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(136 as u32) ) } as u64;
	// 8300E06C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8300E070: 4802E391  bl 0x8303c400
	ctx.lr = 0x8300E074;
	sub_8303C400(ctx, base);
	// 8300E074: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8300E078: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8300E07C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8300E080: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 8300E084: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8300E088: 4BFFFE01  bl 0x8300de88
	ctx.lr = 0x8300E08C;
	sub_8300DE88(ctx, base);
	// 8300E08C: 4BFFFF94  b 0x8300e020
	pc = 0x8300E020; continue 'dispatch;
	// 8300E090: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8300E094: 997E0095  stb r11, 0x95(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(149 as u32), ctx.r[11].u8 ) };
	// 8300E098: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 8300E09C: 4800012D  bl 0x8300e1c8
	ctx.lr = 0x8300E0A0;
	sub_8300E1C8(ctx, base);
	// 8300E0A0: 383F00B0  addi r1, r31, 0xb0
	ctx.r[1].s64 = ctx.r[31].s64 + 176;
	// 8300E0A4: 4819A110  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300E0A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300E0A8 size=40
    let mut pc: u32 = 0x8300E0A8;
    'dispatch: loop {
        match pc {
            0x8300E0A8 => {
    //   block [0x8300E0A8..0x8300E0D0)
	// 8300E0A8: 3BECFF50  addi r31, r12, -0xb0
	ctx.r[31].s64 = ctx.r[12].s64 + -176;
	// 8300E0AC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300E0B0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300E0B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300E0B8: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 8300E0BC: 4800010D  bl 0x8300e1c8
	ctx.lr = 0x8300E0C0;
	sub_8300E1C8(ctx, base);
	// 8300E0C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300E0C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300E0C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300E0CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8300E0D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8300E0D0 size=92
    let mut pc: u32 = 0x8300E0D0;
    'dispatch: loop {
        match pc {
            0x8300E0D0 => {
    //   block [0x8300E0D0..0x8300E12C)
	// 8300E0D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8300E0D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8300E0D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8300E0DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8300E0E0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8300E0E4: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 8300E0E8: 419A000C  beq cr6, 0x8300e0f4
	if ctx.cr[6].eq {
	pc = 0x8300E0F4; continue 'dispatch;
	}
	// 8300E0EC: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 8300E0F0: 4800000C  b 0x8300e0fc
	pc = 0x8300E0FC; continue 'dispatch;
	// 8300E0F4: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8300E0F8: 388B8158  addi r4, r11, -0x7ea8
	ctx.r[4].s64 = ctx.r[11].s64 + -32424;
	// 8300E0FC: 4BFFFA4D  bl 0x8300db48
	ctx.lr = 0x8300E100;
	sub_8300DB48(ctx, base);
	// 8300E100: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8300E104: 419A0010  beq cr6, 0x8300e114
	if ctx.cr[6].eq {
	pc = 0x8300E114; continue 'dispatch;
	}
	// 8300E108: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8300E10C: 4802F675  bl 0x8303d780
	ctx.lr = 0x8300E110;
	sub_8303D780(ctx, base);
	// 8300E110: 48000008  b 0x8300e118
	pc = 0x8300E118; continue 'dispatch;
	// 8300E114: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8300E118: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8300E11C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8300E120: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8300E124: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8300E128: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


