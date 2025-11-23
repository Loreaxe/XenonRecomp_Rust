pub fn sub_832B3528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B3528 size=132
    let mut pc: u32 = 0x832B3528;
    'dispatch: loop {
        match pc {
            0x832B3528 => {
    //   block [0x832B3528..0x832B3568)
	// 832B3528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B352C: 4B9F5EDD  bl 0x82ca9408
	ctx.lr = 0x832B3530;
	sub_82CA93D0(ctx, base);
	// 832B3530: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B3534: 3F80834A  lis r28, -0x7cb6
	ctx.r[28].s64 = -2092302336;
	// 832B3538: 83DCF9C4  lwz r30, -0x63c(r28)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-1596 as u32) ) } as u64;
	// 832B353C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 832B3540: 419A005C  beq cr6, 0x832b359c
	if ctx.cr[6].eq {
	pc = 0x832B359C; continue 'dispatch;
	}
	// 832B3544: 809E0028  lwz r4, 0x28(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 832B3548: 3BFE0024  addi r31, r30, 0x24
	ctx.r[31].s64 = ctx.r[30].s64 + 36;
	// 832B354C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 832B3550: 419A0018  beq cr6, 0x832b3568
	if ctx.cr[6].eq {
	pc = 0x832B3568; continue 'dispatch;
	}
	// 832B3554: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B3558: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 832B355C: 4B7EE4AD  bl 0x82aa1a08
	ctx.lr = 0x832B3560;
	sub_82AA1A08(ctx, base);
	// 832B3560: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B3564: 4AF687D5  bl 0x8221bd38
	ctx.lr = 0x832B3568;
	sub_8221BD38(ctx, base);
	pc = 0x832B3568; continue 'dispatch;
            }
            0x832B3568 => {
    //   block [0x832B3568..0x832B359C)
	// 832B3568: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 832B356C: 387E0010  addi r3, r30, 0x10
	ctx.r[3].s64 = ctx.r[30].s64 + 16;
	// 832B3570: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 832B3574: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 832B3578: 93BF000C  stw r29, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 832B357C: 4B7EE0DD  bl 0x82aa1658
	ctx.lr = 0x832B3580;
	sub_82AA1658(ctx, base);
	// 832B3580: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 832B3584: 4B7EDF15  bl 0x82aa1498
	ctx.lr = 0x832B3588;
	sub_82AA1498(ctx, base);
	// 832B3588: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832B358C: 4AF687AD  bl 0x8221bd38
	ctx.lr = 0x832B3590;
	sub_8221BD38(ctx, base);
	// 832B3590: 93BCF9C4  stw r29, -0x63c(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(-1596 as u32), ctx.r[29].u32 ) };
	// 832B3594: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832B3598: 4B9F5EC0  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            0x832B359C => {
    //   block [0x832B359C..0x832B35AC)
	// 832B359C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B35A0: 917CF9C4  stw r11, -0x63c(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(-1596 as u32), ctx.r[11].u32 ) };
	// 832B35A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832B35A8: 4B9F5EB0  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B35B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B35B0 size=124
    let mut pc: u32 = 0x832B35B0;
    'dispatch: loop {
        match pc {
            0x832B35B0 => {
    //   block [0x832B35B0..0x832B35E4)
	// 832B35B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B35B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B35B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B35BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B35C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B35C4: 3FC0834A  lis r30, -0x7cb6
	ctx.r[30].s64 = -2092302336;
	// 832B35C8: 83FEF9C8  lwz r31, -0x638(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-1592 as u32) ) } as u64;
	// 832B35CC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 832B35D0: 419A003C  beq cr6, 0x832b360c
	if ctx.cr[6].eq {
	pc = 0x832B360C; continue 'dispatch;
	}
	// 832B35D4: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 832B35D8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B35DC: 419A0008  beq cr6, 0x832b35e4
	if ctx.cr[6].eq {
	pc = 0x832B35E4; continue 'dispatch;
	}
	// 832B35E0: 4B88A1E9  bl 0x82b3d7c8
	ctx.lr = 0x832B35E4;
	sub_82B3D7C8(ctx, base);
	pc = 0x832B35E4; continue 'dispatch;
            }
            0x832B35E4 => {
    //   block [0x832B35E4..0x832B360C)
	// 832B35E4: 387F0040  addi r3, r31, 0x40
	ctx.r[3].s64 = ctx.r[31].s64 + 64;
	// 832B35E8: 4B3870A1  bl 0x8263a688
	ctx.lr = 0x832B35EC;
	sub_8263A688(ctx, base);
	// 832B35EC: 387F0034  addi r3, r31, 0x34
	ctx.r[3].s64 = ctx.r[31].s64 + 52;
	// 832B35F0: 4B387099  bl 0x8263a688
	ctx.lr = 0x832B35F4;
	sub_8263A688(ctx, base);
	// 832B35F4: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 832B35F8: 4B7EE061  bl 0x82aa1658
	ctx.lr = 0x832B35FC;
	sub_82AA1658(ctx, base);
	// 832B35FC: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 832B3600: 4B7EE059  bl 0x82aa1658
	ctx.lr = 0x832B3604;
	sub_82AA1658(ctx, base);
	// 832B3604: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B3608: 4AF68731  bl 0x8221bd38
	ctx.lr = 0x832B360C;
	sub_8221BD38(ctx, base);
	pc = 0x832B360C; continue 'dispatch;
            }
            0x832B360C => {
    //   block [0x832B360C..0x832B362C)
	// 832B360C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B3610: 917EF9C8  stw r11, -0x638(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-1592 as u32), ctx.r[11].u32 ) };
	// 832B3614: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B3618: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B361C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B3620: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B3624: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B3628: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B3650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B3650 size=132
    let mut pc: u32 = 0x832B3650;
    'dispatch: loop {
        match pc {
            0x832B3650 => {
    //   block [0x832B3650..0x832B3690)
	// 832B3650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B3654: 4B9F5DB5  bl 0x82ca9408
	ctx.lr = 0x832B3658;
	sub_82CA93D0(ctx, base);
	// 832B3658: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B365C: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B3660: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 832B3664: 3BCBF9D8  addi r30, r11, -0x628
	ctx.r[30].s64 = ctx.r[11].s64 + -1576;
	// 832B3668: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 832B366C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B3670: 83EB0000  lwz r31, 0(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B3674: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 832B3678: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B367C: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B3680: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B3684: 7F1F1840  cmplw cr6, r31, r3
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[3].u32, &mut ctx.xer);
	// 832B3688: 915E0008  stw r10, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B368C: 419A0034  beq cr6, 0x832b36c0
	if ctx.cr[6].eq {
	pc = 0x832B36C0; continue 'dispatch;
	}
	pc = 0x832B3690; continue 'dispatch;
            }
            0x832B3690 => {
    //   block [0x832B3690..0x832B36A8)
	// 832B3690: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 832B3694: 83BF0000  lwz r29, 0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B3698: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B369C: 419A000C  beq cr6, 0x832b36a8
	if ctx.cr[6].eq {
	pc = 0x832B36A8; continue 'dispatch;
	}
	// 832B36A0: 4B8D3F29  bl 0x82b875c8
	ctx.lr = 0x832B36A4;
	sub_82B875C8(ctx, base);
	// 832B36A4: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	pc = 0x832B36A8; continue 'dispatch;
            }
            0x832B36A8 => {
    //   block [0x832B36A8..0x832B36C0)
	// 832B36A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B36AC: 4AF6868D  bl 0x8221bd38
	ctx.lr = 0x832B36B0;
	sub_8221BD38(ctx, base);
	// 832B36B0: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B36B4: 7FBFEB78  mr r31, r29
	ctx.r[31].u64 = ctx.r[29].u64;
	// 832B36B8: 7F1D1840  cmplw cr6, r29, r3
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[3].u32, &mut ctx.xer);
	// 832B36BC: 409AFFD4  bne cr6, 0x832b3690
	if !ctx.cr[6].eq {
	pc = 0x832B3690; continue 'dispatch;
	}
	pc = 0x832B36C0; continue 'dispatch;
            }
            0x832B36C0 => {
    //   block [0x832B36C0..0x832B36D4)
	// 832B36C0: 4AF68679  bl 0x8221bd38
	ctx.lr = 0x832B36C4;
	sub_8221BD38(ctx, base);
	// 832B36C4: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 832B36C8: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B36CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832B36D0: 4B9F5D88  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B36E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B36E8 size=132
    let mut pc: u32 = 0x832B36E8;
    'dispatch: loop {
        match pc {
            0x832B36E8 => {
    //   block [0x832B36E8..0x832B372C)
	// 832B36E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B36EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B36F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B36F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B36F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B36FC: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B3700: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B3704: 3BEBF9F0  addi r31, r11, -0x610
	ctx.r[31].s64 = ctx.r[11].s64 + -1552;
	// 832B3708: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B370C: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B3710: 914A0000  stw r10, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 832B3714: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B3718: 914A0004  stw r10, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 832B371C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B3720: 7F0B1840  cmplw cr6, r11, r3
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[3].u32, &mut ctx.xer);
	// 832B3724: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 832B3728: 419A0020  beq cr6, 0x832b3748
	if ctx.cr[6].eq {
	pc = 0x832B3748; continue 'dispatch;
	}
	pc = 0x832B372C; continue 'dispatch;
            }
            0x832B372C => {
    //   block [0x832B372C..0x832B3748)
	// 832B372C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 832B3730: 83CB0000  lwz r30, 0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B3734: 4AF68605  bl 0x8221bd38
	ctx.lr = 0x832B3738;
	sub_8221BD38(ctx, base);
	// 832B3738: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B373C: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 832B3740: 7F1E1840  cmplw cr6, r30, r3
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[3].u32, &mut ctx.xer);
	// 832B3744: 409AFFE8  bne cr6, 0x832b372c
	if !ctx.cr[6].eq {
	pc = 0x832B372C; continue 'dispatch;
	}
	pc = 0x832B3748; continue 'dispatch;
            }
            0x832B3748 => {
    //   block [0x832B3748..0x832B376C)
	// 832B3748: 4AF685F1  bl 0x8221bd38
	ctx.lr = 0x832B374C;
	sub_8221BD38(ctx, base);
	// 832B374C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B3750: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B3754: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B3758: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B375C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B3760: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B3764: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B3768: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B3780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B3780 size=84
    let mut pc: u32 = 0x832B3780;
    'dispatch: loop {
        match pc {
            0x832B3780 => {
    //   block [0x832B3780..0x832B37A8)
	// 832B3780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B3784: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B3788: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B378C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B3790: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B3794: 3BEBF9FC  addi r31, r11, -0x604
	ctx.r[31].s64 = ctx.r[11].s64 + -1540;
	// 832B3798: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B379C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B37A0: 419A0008  beq cr6, 0x832b37a8
	if ctx.cr[6].eq {
	pc = 0x832B37A8; continue 'dispatch;
	}
	// 832B37A4: 4AF68595  bl 0x8221bd38
	ctx.lr = 0x832B37A8;
	sub_8221BD38(ctx, base);
	pc = 0x832B37A8; continue 'dispatch;
            }
            0x832B37A8 => {
    //   block [0x832B37A8..0x832B37D4)
	// 832B37A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B37AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B37B0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B37B4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B37B8: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B37BC: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832B37C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B37C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B37C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B37CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B37D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B37D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B37D8 size=80
    let mut pc: u32 = 0x832B37D8;
    'dispatch: loop {
        match pc {
            0x832B37D8 => {
    //   block [0x832B37D8..0x832B380C)
	// 832B37D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B37DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B37E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B37E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B37E8: 3FE0834A  lis r31, -0x7cb6
	ctx.r[31].s64 = -2092302336;
	// 832B37EC: 807FFA0C  lwz r3, -0x5f4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-1524 as u32) ) } as u64;
	// 832B37F0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B37F4: 419A0018  beq cr6, 0x832b380c
	if ctx.cr[6].eq {
	pc = 0x832B380C; continue 'dispatch;
	}
	// 832B37F8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B37FC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 832B3800: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B3804: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 832B3808: 4E800421  bctrl
	ctx.lr = 0x832B380C;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x832B380C => {
    //   block [0x832B380C..0x832B3828)
	// 832B380C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B3810: 917FFA0C  stw r11, -0x5f4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-1524 as u32), ctx.r[11].u32 ) };
	// 832B3814: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B3818: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B381C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B3820: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B3824: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B3828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B3828 size=80
    let mut pc: u32 = 0x832B3828;
    'dispatch: loop {
        match pc {
            0x832B3828 => {
    //   block [0x832B3828..0x832B384C)
	// 832B3828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B382C: 4B9F5BDD  bl 0x82ca9408
	ctx.lr = 0x832B3830;
	sub_82CA93D0(ctx, base);
	// 832B3830: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B3834: 3F80834A  lis r28, -0x7cb6
	ctx.r[28].s64 = -2092302336;
	// 832B3838: 83BCFA10  lwz r29, -0x5f0(r28)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-1520 as u32) ) } as u64;
	// 832B383C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 832B3840: 419A0028  beq cr6, 0x832b3868
	if ctx.cr[6].eq {
	pc = 0x832B3868; continue 'dispatch;
	}
	// 832B3844: 3BFD0100  addi r31, r29, 0x100
	ctx.r[31].s64 = ctx.r[29].s64 + 256;
	// 832B3848: 3BC0001F  li r30, 0x1f
	ctx.r[30].s64 = 31;
	pc = 0x832B384C; continue 'dispatch;
            }
            0x832B384C => {
    //   block [0x832B384C..0x832B3868)
	// 832B384C: 3BFFFFF8  addi r31, r31, -8
	ctx.r[31].s64 = ctx.r[31].s64 + -8;
	// 832B3850: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B3854: 4AF042C5  bl 0x821b7b18
	ctx.lr = 0x832B3858;
	sub_821B7B18(ctx, base);
	// 832B3858: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832B385C: 4080FFF0  bge 0x832b384c
	if !ctx.cr[0].lt {
	pc = 0x832B384C; continue 'dispatch;
	}
	// 832B3860: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 832B3864: 4AF684D5  bl 0x8221bd38
	ctx.lr = 0x832B3868;
	sub_8221BD38(ctx, base);
	pc = 0x832B3868; continue 'dispatch;
            }
            0x832B3868 => {
    //   block [0x832B3868..0x832B3878)
	// 832B3868: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B386C: 917CFA10  stw r11, -0x5f0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(-1520 as u32), ctx.r[11].u32 ) };
	// 832B3870: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832B3874: 4B9F5BE4  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B38A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B38A0 size=164
    let mut pc: u32 = 0x832B38A0;
    'dispatch: loop {
        match pc {
            0x832B38A0 => {
    //   block [0x832B38A0..0x832B38C8)
	// 832B38A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B38A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B38A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B38AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B38B0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B38B4: 3BEBFA18  addi r31, r11, -0x5e8
	ctx.r[31].s64 = ctx.r[11].s64 + -1512;
	// 832B38B8: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 832B38BC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B38C0: 419A0008  beq cr6, 0x832b38c8
	if ctx.cr[6].eq {
	pc = 0x832B38C8; continue 'dispatch;
	}
	// 832B38C4: 4AF68475  bl 0x8221bd38
	ctx.lr = 0x832B38C8;
	sub_8221BD38(ctx, base);
	pc = 0x832B38C8; continue 'dispatch;
            }
            0x832B38C8 => {
    //   block [0x832B38C8..0x832B38F0)
	// 832B38C8: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 832B38CC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B38D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B38D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B38D8: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 832B38DC: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 832B38E0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B38E4: 913F002C  stw r9, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[9].u32 ) };
	// 832B38E8: 419A0008  beq cr6, 0x832b38f0
	if ctx.cr[6].eq {
	pc = 0x832B38F0; continue 'dispatch;
	}
	// 832B38EC: 4AF6844D  bl 0x8221bd38
	ctx.lr = 0x832B38F0;
	sub_8221BD38(ctx, base);
	pc = 0x832B38F0; continue 'dispatch;
            }
            0x832B38F0 => {
    //   block [0x832B38F0..0x832B3918)
	// 832B38F0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B38F4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B38F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B38FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B3900: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 832B3904: 915F0018  stw r10, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 832B3908: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B390C: 913F001C  stw r9, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[9].u32 ) };
	// 832B3910: 419A0008  beq cr6, 0x832b3918
	if ctx.cr[6].eq {
	pc = 0x832B3918; continue 'dispatch;
	}
	// 832B3914: 4AF68425  bl 0x8221bd38
	ctx.lr = 0x832B3918;
	sub_8221BD38(ctx, base);
	pc = 0x832B3918; continue 'dispatch;
            }
            0x832B3918 => {
    //   block [0x832B3918..0x832B3944)
	// 832B3918: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B391C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B3920: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B3924: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B3928: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B392C: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832B3930: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B3934: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B3938: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B393C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B3940: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B3948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B3948 size=4
    let mut pc: u32 = 0x832B3948;
    'dispatch: loop {
        match pc {
            0x832B3948 => {
    //   block [0x832B3948..0x832B394C)
	// 832B3948: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B3950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B3950 size=12
    let mut pc: u32 = 0x832B3950;
    'dispatch: loop {
        match pc {
            0x832B3950 => {
    //   block [0x832B3950..0x832B395C)
	// 832B3950: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B3954: 386BFA4C  addi r3, r11, -0x5b4
	ctx.r[3].s64 = ctx.r[11].s64 + -1460;
	// 832B3958: 4AF61480  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B3960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B3960 size=12
    let mut pc: u32 = 0x832B3960;
    'dispatch: loop {
        match pc {
            0x832B3960 => {
    //   block [0x832B3960..0x832B396C)
	// 832B3960: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B3964: 386BFA50  addi r3, r11, -0x5b0
	ctx.r[3].s64 = ctx.r[11].s64 + -1456;
	// 832B3968: 4AF61470  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B3970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B3970 size=12
    let mut pc: u32 = 0x832B3970;
    'dispatch: loop {
        match pc {
            0x832B3970 => {
    //   block [0x832B3970..0x832B397C)
	// 832B3970: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B3974: 386BFA54  addi r3, r11, -0x5ac
	ctx.r[3].s64 = ctx.r[11].s64 + -1452;
	// 832B3978: 4AF61460  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B3980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B3980 size=12
    let mut pc: u32 = 0x832B3980;
    'dispatch: loop {
        match pc {
            0x832B3980 => {
    //   block [0x832B3980..0x832B398C)
	// 832B3980: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B3984: 386BFA58  addi r3, r11, -0x5a8
	ctx.r[3].s64 = ctx.r[11].s64 + -1448;
	// 832B3988: 4AF61450  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B3990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B3990 size=12
    let mut pc: u32 = 0x832B3990;
    'dispatch: loop {
        match pc {
            0x832B3990 => {
    //   block [0x832B3990..0x832B399C)
	// 832B3990: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B3994: 386BFA5C  addi r3, r11, -0x5a4
	ctx.r[3].s64 = ctx.r[11].s64 + -1444;
	// 832B3998: 4AF61440  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B39A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B39A0 size=12
    let mut pc: u32 = 0x832B39A0;
    'dispatch: loop {
        match pc {
            0x832B39A0 => {
    //   block [0x832B39A0..0x832B39AC)
	// 832B39A0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B39A4: 386BFA70  addi r3, r11, -0x590
	ctx.r[3].s64 = ctx.r[11].s64 + -1424;
	// 832B39A8: 4AF61430  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B39B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B39B0 size=12
    let mut pc: u32 = 0x832B39B0;
    'dispatch: loop {
        match pc {
            0x832B39B0 => {
    //   block [0x832B39B0..0x832B39BC)
	// 832B39B0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B39B4: 386BFA74  addi r3, r11, -0x58c
	ctx.r[3].s64 = ctx.r[11].s64 + -1420;
	// 832B39B8: 4AF61420  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B39C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B39C0 size=12
    let mut pc: u32 = 0x832B39C0;
    'dispatch: loop {
        match pc {
            0x832B39C0 => {
    //   block [0x832B39C0..0x832B39CC)
	// 832B39C0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B39C4: 386BFA78  addi r3, r11, -0x588
	ctx.r[3].s64 = ctx.r[11].s64 + -1416;
	// 832B39C8: 4AF61410  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B39D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B39D0 size=12
    let mut pc: u32 = 0x832B39D0;
    'dispatch: loop {
        match pc {
            0x832B39D0 => {
    //   block [0x832B39D0..0x832B39DC)
	// 832B39D0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B39D4: 386BFA7C  addi r3, r11, -0x584
	ctx.r[3].s64 = ctx.r[11].s64 + -1412;
	// 832B39D8: 4AF61400  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B39E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B39E0 size=84
    let mut pc: u32 = 0x832B39E0;
    'dispatch: loop {
        match pc {
            0x832B39E0 => {
    //   block [0x832B39E0..0x832B3A08)
	// 832B39E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B39E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B39E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B39EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B39F0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B39F4: 3BEBFAB0  addi r31, r11, -0x550
	ctx.r[31].s64 = ctx.r[11].s64 + -1360;
	// 832B39F8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B39FC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B3A00: 419A0008  beq cr6, 0x832b3a08
	if ctx.cr[6].eq {
	pc = 0x832B3A08; continue 'dispatch;
	}
	// 832B3A04: 4AF68335  bl 0x8221bd38
	ctx.lr = 0x832B3A08;
	sub_8221BD38(ctx, base);
	pc = 0x832B3A08; continue 'dispatch;
            }
            0x832B3A08 => {
    //   block [0x832B3A08..0x832B3A34)
	// 832B3A08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B3A0C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B3A10: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B3A14: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B3A18: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B3A1C: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832B3A20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B3A24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B3A28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B3A2C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B3A30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B3A38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B3A38 size=84
    let mut pc: u32 = 0x832B3A38;
    'dispatch: loop {
        match pc {
            0x832B3A38 => {
    //   block [0x832B3A38..0x832B3A60)
	// 832B3A38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B3A3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B3A40: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B3A44: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B3A48: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B3A4C: 3BEBFAC0  addi r31, r11, -0x540
	ctx.r[31].s64 = ctx.r[11].s64 + -1344;
	// 832B3A50: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B3A54: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B3A58: 419A0008  beq cr6, 0x832b3a60
	if ctx.cr[6].eq {
	pc = 0x832B3A60; continue 'dispatch;
	}
	// 832B3A5C: 4AF682DD  bl 0x8221bd38
	ctx.lr = 0x832B3A60;
	sub_8221BD38(ctx, base);
	pc = 0x832B3A60; continue 'dispatch;
            }
            0x832B3A60 => {
    //   block [0x832B3A60..0x832B3A8C)
	// 832B3A60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B3A64: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B3A68: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B3A6C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B3A70: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B3A74: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832B3A78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B3A7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B3A80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B3A84: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B3A88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B3A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B3A90 size=84
    let mut pc: u32 = 0x832B3A90;
    'dispatch: loop {
        match pc {
            0x832B3A90 => {
    //   block [0x832B3A90..0x832B3AB8)
	// 832B3A90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B3A94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B3A98: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B3A9C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B3AA0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B3AA4: 3BEBFAD0  addi r31, r11, -0x530
	ctx.r[31].s64 = ctx.r[11].s64 + -1328;
	// 832B3AA8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B3AAC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B3AB0: 419A0008  beq cr6, 0x832b3ab8
	if ctx.cr[6].eq {
	pc = 0x832B3AB8; continue 'dispatch;
	}
	// 832B3AB4: 4AF68285  bl 0x8221bd38
	ctx.lr = 0x832B3AB8;
	sub_8221BD38(ctx, base);
	pc = 0x832B3AB8; continue 'dispatch;
            }
            0x832B3AB8 => {
    //   block [0x832B3AB8..0x832B3AE4)
	// 832B3AB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B3ABC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B3AC0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B3AC4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B3AC8: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B3ACC: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832B3AD0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B3AD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B3AD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B3ADC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B3AE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B3AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B3AE8 size=84
    let mut pc: u32 = 0x832B3AE8;
    'dispatch: loop {
        match pc {
            0x832B3AE8 => {
    //   block [0x832B3AE8..0x832B3B10)
	// 832B3AE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B3AEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B3AF0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B3AF4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B3AF8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B3AFC: 3BEBFAE0  addi r31, r11, -0x520
	ctx.r[31].s64 = ctx.r[11].s64 + -1312;
	// 832B3B00: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B3B04: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B3B08: 419A0008  beq cr6, 0x832b3b10
	if ctx.cr[6].eq {
	pc = 0x832B3B10; continue 'dispatch;
	}
	// 832B3B0C: 4AF6822D  bl 0x8221bd38
	ctx.lr = 0x832B3B10;
	sub_8221BD38(ctx, base);
	pc = 0x832B3B10; continue 'dispatch;
            }
            0x832B3B10 => {
    //   block [0x832B3B10..0x832B3B3C)
	// 832B3B10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B3B14: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B3B18: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B3B1C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B3B20: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B3B24: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832B3B28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B3B2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B3B30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B3B34: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B3B38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B3B40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B3B40 size=84
    let mut pc: u32 = 0x832B3B40;
    'dispatch: loop {
        match pc {
            0x832B3B40 => {
    //   block [0x832B3B40..0x832B3B68)
	// 832B3B40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B3B44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B3B48: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B3B4C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B3B50: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B3B54: 3BEBFAF0  addi r31, r11, -0x510
	ctx.r[31].s64 = ctx.r[11].s64 + -1296;
	// 832B3B58: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B3B5C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B3B60: 419A0008  beq cr6, 0x832b3b68
	if ctx.cr[6].eq {
	pc = 0x832B3B68; continue 'dispatch;
	}
	// 832B3B64: 4AF681D5  bl 0x8221bd38
	ctx.lr = 0x832B3B68;
	sub_8221BD38(ctx, base);
	pc = 0x832B3B68; continue 'dispatch;
            }
            0x832B3B68 => {
    //   block [0x832B3B68..0x832B3B94)
	// 832B3B68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B3B6C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B3B70: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B3B74: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B3B78: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B3B7C: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832B3B80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B3B84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B3B88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B3B8C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B3B90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B3B98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B3B98 size=84
    let mut pc: u32 = 0x832B3B98;
    'dispatch: loop {
        match pc {
            0x832B3B98 => {
    //   block [0x832B3B98..0x832B3BC0)
	// 832B3B98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B3B9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B3BA0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B3BA4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B3BA8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B3BAC: 3BEBFB04  addi r31, r11, -0x4fc
	ctx.r[31].s64 = ctx.r[11].s64 + -1276;
	// 832B3BB0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B3BB4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B3BB8: 419A0008  beq cr6, 0x832b3bc0
	if ctx.cr[6].eq {
	pc = 0x832B3BC0; continue 'dispatch;
	}
	// 832B3BBC: 4AF6817D  bl 0x8221bd38
	ctx.lr = 0x832B3BC0;
	sub_8221BD38(ctx, base);
	pc = 0x832B3BC0; continue 'dispatch;
            }
            0x832B3BC0 => {
    //   block [0x832B3BC0..0x832B3BEC)
	// 832B3BC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B3BC4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B3BC8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B3BCC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B3BD0: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B3BD4: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832B3BD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B3BDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B3BE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B3BE4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B3BE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B3BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B3BF0 size=84
    let mut pc: u32 = 0x832B3BF0;
    'dispatch: loop {
        match pc {
            0x832B3BF0 => {
    //   block [0x832B3BF0..0x832B3C18)
	// 832B3BF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B3BF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B3BF8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B3BFC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B3C00: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B3C04: 3BEBFB14  addi r31, r11, -0x4ec
	ctx.r[31].s64 = ctx.r[11].s64 + -1260;
	// 832B3C08: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B3C0C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B3C10: 419A0008  beq cr6, 0x832b3c18
	if ctx.cr[6].eq {
	pc = 0x832B3C18; continue 'dispatch;
	}
	// 832B3C14: 4AF68125  bl 0x8221bd38
	ctx.lr = 0x832B3C18;
	sub_8221BD38(ctx, base);
	pc = 0x832B3C18; continue 'dispatch;
            }
            0x832B3C18 => {
    //   block [0x832B3C18..0x832B3C44)
	// 832B3C18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B3C1C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B3C20: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B3C24: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B3C28: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B3C2C: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832B3C30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B3C34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B3C38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B3C3C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B3C40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B3C78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B3C78 size=84
    let mut pc: u32 = 0x832B3C78;
    'dispatch: loop {
        match pc {
            0x832B3C78 => {
    //   block [0x832B3C78..0x832B3CA0)
	// 832B3C78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B3C7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B3C80: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B3C84: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B3C88: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B3C8C: 3BEBFB30  addi r31, r11, -0x4d0
	ctx.r[31].s64 = ctx.r[11].s64 + -1232;
	// 832B3C90: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B3C94: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B3C98: 419A0008  beq cr6, 0x832b3ca0
	if ctx.cr[6].eq {
	pc = 0x832B3CA0; continue 'dispatch;
	}
	// 832B3C9C: 4AF6809D  bl 0x8221bd38
	ctx.lr = 0x832B3CA0;
	sub_8221BD38(ctx, base);
	pc = 0x832B3CA0; continue 'dispatch;
            }
            0x832B3CA0 => {
    //   block [0x832B3CA0..0x832B3CCC)
	// 832B3CA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B3CA4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B3CA8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B3CAC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B3CB0: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B3CB4: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832B3CB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B3CBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B3CC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B3CC4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B3CC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B3D00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B3D00 size=120
    let mut pc: u32 = 0x832B3D00;
    'dispatch: loop {
        match pc {
            0x832B3D00 => {
    //   block [0x832B3D00..0x832B3D3C)
	// 832B3D00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B3D04: 4B9F5709  bl 0x82ca940c
	ctx.lr = 0x832B3D08;
	sub_82CA93D0(ctx, base);
	// 832B3D08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B3D0C: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B3D10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B3D14: 3BEBFB54  addi r31, r11, -0x4ac
	ctx.r[31].s64 = ctx.r[11].s64 + -1196;
	// 832B3D18: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B3D1C: 83CB0000  lwz r30, 0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B3D20: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 832B3D24: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B3D28: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B3D2C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B3D30: 7F1E1840  cmplw cr6, r30, r3
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[3].u32, &mut ctx.xer);
	// 832B3D34: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B3D38: 419A002C  beq cr6, 0x832b3d64
	if ctx.cr[6].eq {
	pc = 0x832B3D64; continue 'dispatch;
	}
	pc = 0x832B3D3C; continue 'dispatch;
            }
            0x832B3D3C => {
    //   block [0x832B3D3C..0x832B3D64)
	// 832B3D3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B3D40: 83BE0000  lwz r29, 0(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B3D44: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 832B3D48: 4B2B2139  bl 0x82565e80
	ctx.lr = 0x832B3D4C;
	sub_82565E80(ctx, base);
	// 832B3D4C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832B3D50: 4AF67FE9  bl 0x8221bd38
	ctx.lr = 0x832B3D54;
	sub_8221BD38(ctx, base);
	// 832B3D54: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B3D58: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	// 832B3D5C: 7F1D1840  cmplw cr6, r29, r3
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[3].u32, &mut ctx.xer);
	// 832B3D60: 409AFFDC  bne cr6, 0x832b3d3c
	if !ctx.cr[6].eq {
	pc = 0x832B3D3C; continue 'dispatch;
	}
	pc = 0x832B3D64; continue 'dispatch;
            }
            0x832B3D64 => {
    //   block [0x832B3D64..0x832B3D78)
	// 832B3D64: 4AF67FD5  bl 0x8221bd38
	ctx.lr = 0x832B3D68;
	sub_8221BD38(ctx, base);
	// 832B3D68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B3D6C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B3D70: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B3D74: 4B9F56E8  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B3D78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B3D78 size=12
    let mut pc: u32 = 0x832B3D78;
    'dispatch: loop {
        match pc {
            0x832B3D78 => {
    //   block [0x832B3D78..0x832B3D84)
	// 832B3D78: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B3D7C: 386BFB60  addi r3, r11, -0x4a0
	ctx.r[3].s64 = ctx.r[11].s64 + -1184;
	// 832B3D80: 4B7AA6B8  b 0x82a5e438
	sub_82A5E438(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B3D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B3D88 size=12
    let mut pc: u32 = 0x832B3D88;
    'dispatch: loop {
        match pc {
            0x832B3D88 => {
    //   block [0x832B3D88..0x832B3D94)
	// 832B3D88: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B3D8C: 386BFB70  addi r3, r11, -0x490
	ctx.r[3].s64 = ctx.r[11].s64 + -1168;
	// 832B3D90: 4AF61048  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B3D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B3D98 size=12
    let mut pc: u32 = 0x832B3D98;
    'dispatch: loop {
        match pc {
            0x832B3D98 => {
    //   block [0x832B3D98..0x832B3DA4)
	// 832B3D98: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B3D9C: 386BFB74  addi r3, r11, -0x48c
	ctx.r[3].s64 = ctx.r[11].s64 + -1164;
	// 832B3DA0: 4AF61038  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B3DA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B3DA8 size=12
    let mut pc: u32 = 0x832B3DA8;
    'dispatch: loop {
        match pc {
            0x832B3DA8 => {
    //   block [0x832B3DA8..0x832B3DB4)
	// 832B3DA8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B3DAC: 386BFB78  addi r3, r11, -0x488
	ctx.r[3].s64 = ctx.r[11].s64 + -1160;
	// 832B3DB0: 4AF61028  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B3DB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B3DB8 size=12
    let mut pc: u32 = 0x832B3DB8;
    'dispatch: loop {
        match pc {
            0x832B3DB8 => {
    //   block [0x832B3DB8..0x832B3DC4)
	// 832B3DB8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B3DBC: 386BFB7C  addi r3, r11, -0x484
	ctx.r[3].s64 = ctx.r[11].s64 + -1156;
	// 832B3DC0: 4AF61018  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B3DC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B3DC8 size=20
    let mut pc: u32 = 0x832B3DC8;
    'dispatch: loop {
        match pc {
            0x832B3DC8 => {
    //   block [0x832B3DC8..0x832B3DDC)
	// 832B3DC8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B3DCC: 806BFB80  lwz r3, -0x480(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-1152 as u32) ) } as u64;
	// 832B3DD0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B3DD4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 832B3DD8: 4AF48548  b 0x821fc320
	sub_821FC320(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B3DDC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B3DDC size=4
    let mut pc: u32 = 0x832B3DDC;
    'dispatch: loop {
        match pc {
            0x832B3DDC => {
    //   block [0x832B3DDC..0x832B3DE0)
	// 832B3DDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B3DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B3DE0 size=12
    let mut pc: u32 = 0x832B3DE0;
    'dispatch: loop {
        match pc {
            0x832B3DE0 => {
    //   block [0x832B3DE0..0x832B3DEC)
	// 832B3DE0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B3DE4: 386BFB94  addi r3, r11, -0x46c
	ctx.r[3].s64 = ctx.r[11].s64 + -1132;
	// 832B3DE8: 4AF60FF0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B3DF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B3DF0 size=12
    let mut pc: u32 = 0x832B3DF0;
    'dispatch: loop {
        match pc {
            0x832B3DF0 => {
    //   block [0x832B3DF0..0x832B3DFC)
	// 832B3DF0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B3DF4: 386BFB98  addi r3, r11, -0x468
	ctx.r[3].s64 = ctx.r[11].s64 + -1128;
	// 832B3DF8: 4AF60FE0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B3E00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B3E00 size=68
    let mut pc: u32 = 0x832B3E00;
    'dispatch: loop {
        match pc {
            0x832B3E00 => {
    //   block [0x832B3E00..0x832B3E44)
	// 832B3E00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B3E04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B3E08: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B3E0C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B3E10: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B3E14: 3BEBFB9C  addi r31, r11, -0x464
	ctx.r[31].s64 = ctx.r[11].s64 + -1124;
	// 832B3E18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B3E1C: 4B7AFD95  bl 0x82a63bb0
	ctx.lr = 0x832B3E20;
	sub_82A63BB0(ctx, base);
	// 832B3E20: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B3E24: 4AF67F15  bl 0x8221bd38
	ctx.lr = 0x832B3E28;
	sub_8221BD38(ctx, base);
	// 832B3E28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B3E2C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B3E30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B3E34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B3E38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B3E3C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B3E40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B3E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B3E48 size=80
    let mut pc: u32 = 0x832B3E48;
    'dispatch: loop {
        match pc {
            0x832B3E48 => {
    //   block [0x832B3E48..0x832B3E6C)
	// 832B3E48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B3E4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B3E50: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B3E54: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B3E58: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B3E5C: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B3E60: 3BC00031  li r30, 0x31
	ctx.r[30].s64 = 49;
	// 832B3E64: 396BFBD0  addi r11, r11, -0x430
	ctx.r[11].s64 = ctx.r[11].s64 + -1072;
	// 832B3E68: 3BEB12C0  addi r31, r11, 0x12c0
	ctx.r[31].s64 = ctx.r[11].s64 + 4800;
	pc = 0x832B3E6C; continue 'dispatch;
            }
            0x832B3E6C => {
    //   block [0x832B3E6C..0x832B3E98)
	// 832B3E6C: 3BFFFFA0  addi r31, r31, -0x60
	ctx.r[31].s64 = ctx.r[31].s64 + -96;
	// 832B3E70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B3E74: 4B7AE645  bl 0x82a624b8
	ctx.lr = 0x832B3E78;
	sub_82A624B8(ctx, base);
	// 832B3E78: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832B3E7C: 4080FFF0  bge 0x832b3e6c
	if !ctx.cr[0].lt {
	pc = 0x832B3E6C; continue 'dispatch;
	}
	// 832B3E80: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B3E84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B3E88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B3E8C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B3E90: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B3E94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B3E98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B3E98 size=64
    let mut pc: u32 = 0x832B3E98;
    'dispatch: loop {
        match pc {
            0x832B3E98 => {
    //   block [0x832B3E98..0x832B3EC4)
	// 832B3E98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B3E9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B3EA0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B3EA4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B3EA8: 3FE0834A  lis r31, -0x7cb6
	ctx.r[31].s64 = -2092302336;
	// 832B3EAC: 807F0E90  lwz r3, 0xe90(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3728 as u32) ) } as u64;
	// 832B3EB0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B3EB4: 419A0010  beq cr6, 0x832b3ec4
	if ctx.cr[6].eq {
	pc = 0x832B3EC4; continue 'dispatch;
	}
	// 832B3EB8: 4B8D3711  bl 0x82b875c8
	ctx.lr = 0x832B3EBC;
	sub_82B875C8(ctx, base);
	// 832B3EBC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B3EC0: 917F0E90  stw r11, 0xe90(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3728 as u32), ctx.r[11].u32 ) };
	pc = 0x832B3EC4; continue 'dispatch;
            }
            0x832B3EC4 => {
    //   block [0x832B3EC4..0x832B3ED8)
	// 832B3EC4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B3EC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B3ECC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B3ED0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B3ED4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B3ED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B3ED8 size=64
    let mut pc: u32 = 0x832B3ED8;
    'dispatch: loop {
        match pc {
            0x832B3ED8 => {
    //   block [0x832B3ED8..0x832B3F04)
	// 832B3ED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B3EDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B3EE0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B3EE4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B3EE8: 3FE08332  lis r31, -0x7cce
	ctx.r[31].s64 = -2093875200;
	// 832B3EEC: 807FAB0C  lwz r3, -0x54f4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-21748 as u32) ) } as u64;
	// 832B3EF0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B3EF4: 419A0010  beq cr6, 0x832b3f04
	if ctx.cr[6].eq {
	pc = 0x832B3F04; continue 'dispatch;
	}
	// 832B3EF8: 4B8D36D1  bl 0x82b875c8
	ctx.lr = 0x832B3EFC;
	sub_82B875C8(ctx, base);
	// 832B3EFC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B3F00: 917FAB0C  stw r11, -0x54f4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-21748 as u32), ctx.r[11].u32 ) };
	pc = 0x832B3F04; continue 'dispatch;
            }
            0x832B3F04 => {
    //   block [0x832B3F04..0x832B3F18)
	// 832B3F04: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B3F08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B3F0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B3F10: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B3F14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B3F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B3F18 size=20
    let mut pc: u32 = 0x832B3F18;
    'dispatch: loop {
        match pc {
            0x832B3F18 => {
    //   block [0x832B3F18..0x832B3F2C)
	// 832B3F18: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B3F1C: 806BFBA8  lwz r3, -0x458(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-1112 as u32) ) } as u64;
	// 832B3F20: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B3F24: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 832B3F28: 4AF483F8  b 0x821fc320
	sub_821FC320(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B3F2C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B3F2C size=4
    let mut pc: u32 = 0x832B3F2C;
    'dispatch: loop {
        match pc {
            0x832B3F2C => {
    //   block [0x832B3F2C..0x832B3F30)
	// 832B3F2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B3F30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B3F30 size=20
    let mut pc: u32 = 0x832B3F30;
    'dispatch: loop {
        match pc {
            0x832B3F30 => {
    //   block [0x832B3F30..0x832B3F44)
	// 832B3F30: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B3F34: 806BFBBC  lwz r3, -0x444(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-1092 as u32) ) } as u64;
	// 832B3F38: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B3F3C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 832B3F40: 4AF483E0  b 0x821fc320
	sub_821FC320(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B3F44(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B3F44 size=4
    let mut pc: u32 = 0x832B3F44;
    'dispatch: loop {
        match pc {
            0x832B3F44 => {
    //   block [0x832B3F44..0x832B3F48)
	// 832B3F44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B3F48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B3F48 size=12
    let mut pc: u32 = 0x832B3F48;
    'dispatch: loop {
        match pc {
            0x832B3F48 => {
    //   block [0x832B3F48..0x832B3F54)
	// 832B3F48: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B3F4C: 386B0E9C  addi r3, r11, 0xe9c
	ctx.r[3].s64 = ctx.r[11].s64 + 3740;
	// 832B3F50: 4AF60E88  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B3F58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B3F58 size=12
    let mut pc: u32 = 0x832B3F58;
    'dispatch: loop {
        match pc {
            0x832B3F58 => {
    //   block [0x832B3F58..0x832B3F64)
	// 832B3F58: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B3F5C: 386B0EA0  addi r3, r11, 0xea0
	ctx.r[3].s64 = ctx.r[11].s64 + 3744;
	// 832B3F60: 4AF60E78  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B3F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B3F68 size=88
    let mut pc: u32 = 0x832B3F68;
    'dispatch: loop {
        match pc {
            0x832B3F68 => {
    //   block [0x832B3F68..0x832B3F94)
	// 832B3F68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B3F6C: 4B9F549D  bl 0x82ca9408
	ctx.lr = 0x832B3F70;
	sub_82CA93D0(ctx, base);
	// 832B3F70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B3F74: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B3F78: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 832B3F7C: 396B0EA4  addi r11, r11, 0xea4
	ctx.r[11].s64 = ctx.r[11].s64 + 3748;
	// 832B3F80: 3BC00002  li r30, 2
	ctx.r[30].s64 = 2;
	// 832B3F84: 3BEB0018  addi r31, r11, 0x18
	ctx.r[31].s64 = ctx.r[11].s64 + 24;
	// 832B3F88: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832B3F8C: 3BAA2A40  addi r29, r10, 0x2a40
	ctx.r[29].s64 = ctx.r[10].s64 + 10816;
	// 832B3F90: 3B8B2A30  addi r28, r11, 0x2a30
	ctx.r[28].s64 = ctx.r[11].s64 + 10800;
	pc = 0x832B3F94; continue 'dispatch;
            }
            0x832B3F94 => {
    //   block [0x832B3F94..0x832B3FC0)
	// 832B3F94: 3BFFFFF8  addi r31, r31, -8
	ctx.r[31].s64 = ctx.r[31].s64 + -8;
	// 832B3F98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B3F9C: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832B3FA0: 4AF48069  bl 0x821fc008
	ctx.lr = 0x832B3FA4;
	sub_821FC008(ctx, base);
	// 832B3FA4: 93BF0000  stw r29, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 832B3FA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B3FAC: 4AF4805D  bl 0x821fc008
	ctx.lr = 0x832B3FB0;
	sub_821FC008(ctx, base);
	// 832B3FB0: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832B3FB4: 4080FFE0  bge 0x832b3f94
	if !ctx.cr[0].lt {
	pc = 0x832B3F94; continue 'dispatch;
	}
	// 832B3FB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832B3FBC: 4B9F549C  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B3FC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B3FC0 size=88
    let mut pc: u32 = 0x832B3FC0;
    'dispatch: loop {
        match pc {
            0x832B3FC0 => {
    //   block [0x832B3FC0..0x832B3FEC)
	// 832B3FC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B3FC4: 4B9F5445  bl 0x82ca9408
	ctx.lr = 0x832B3FC8;
	sub_82CA93D0(ctx, base);
	// 832B3FC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B3FCC: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B3FD0: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 832B3FD4: 396B0F60  addi r11, r11, 0xf60
	ctx.r[11].s64 = ctx.r[11].s64 + 3936;
	// 832B3FD8: 3BC00002  li r30, 2
	ctx.r[30].s64 = 2;
	// 832B3FDC: 3BEB0018  addi r31, r11, 0x18
	ctx.r[31].s64 = ctx.r[11].s64 + 24;
	// 832B3FE0: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832B3FE4: 3BAA2A40  addi r29, r10, 0x2a40
	ctx.r[29].s64 = ctx.r[10].s64 + 10816;
	// 832B3FE8: 3B8B2A30  addi r28, r11, 0x2a30
	ctx.r[28].s64 = ctx.r[11].s64 + 10800;
	pc = 0x832B3FEC; continue 'dispatch;
            }
            0x832B3FEC => {
    //   block [0x832B3FEC..0x832B4018)
	// 832B3FEC: 3BFFFFF8  addi r31, r31, -8
	ctx.r[31].s64 = ctx.r[31].s64 + -8;
	// 832B3FF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B3FF4: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832B3FF8: 4AF48011  bl 0x821fc008
	ctx.lr = 0x832B3FFC;
	sub_821FC008(ctx, base);
	// 832B3FFC: 93BF0000  stw r29, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 832B4000: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B4004: 4AF48005  bl 0x821fc008
	ctx.lr = 0x832B4008;
	sub_821FC008(ctx, base);
	// 832B4008: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832B400C: 4080FFE0  bge 0x832b3fec
	if !ctx.cr[0].lt {
	pc = 0x832B3FEC; continue 'dispatch;
	}
	// 832B4010: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832B4014: 4B9F5444  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4018 size=12
    let mut pc: u32 = 0x832B4018;
    'dispatch: loop {
        match pc {
            0x832B4018 => {
    //   block [0x832B4018..0x832B4024)
	// 832B4018: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B401C: 386B0F78  addi r3, r11, 0xf78
	ctx.r[3].s64 = ctx.r[11].s64 + 3960;
	// 832B4020: 4AF60DB8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4028 size=12
    let mut pc: u32 = 0x832B4028;
    'dispatch: loop {
        match pc {
            0x832B4028 => {
    //   block [0x832B4028..0x832B4034)
	// 832B4028: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B402C: 386B0F7C  addi r3, r11, 0xf7c
	ctx.r[3].s64 = ctx.r[11].s64 + 3964;
	// 832B4030: 4AF60DA8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4038 size=12
    let mut pc: u32 = 0x832B4038;
    'dispatch: loop {
        match pc {
            0x832B4038 => {
    //   block [0x832B4038..0x832B4044)
	// 832B4038: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B403C: 386B0F80  addi r3, r11, 0xf80
	ctx.r[3].s64 = ctx.r[11].s64 + 3968;
	// 832B4040: 4AF60D98  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4048 size=12
    let mut pc: u32 = 0x832B4048;
    'dispatch: loop {
        match pc {
            0x832B4048 => {
    //   block [0x832B4048..0x832B4054)
	// 832B4048: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B404C: 386B0F84  addi r3, r11, 0xf84
	ctx.r[3].s64 = ctx.r[11].s64 + 3972;
	// 832B4050: 4AF60D88  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4058 size=12
    let mut pc: u32 = 0x832B4058;
    'dispatch: loop {
        match pc {
            0x832B4058 => {
    //   block [0x832B4058..0x832B4064)
	// 832B4058: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B405C: 386B0F88  addi r3, r11, 0xf88
	ctx.r[3].s64 = ctx.r[11].s64 + 3976;
	// 832B4060: 4AF60D78  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4068 size=12
    let mut pc: u32 = 0x832B4068;
    'dispatch: loop {
        match pc {
            0x832B4068 => {
    //   block [0x832B4068..0x832B4074)
	// 832B4068: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B406C: 386B0F8C  addi r3, r11, 0xf8c
	ctx.r[3].s64 = ctx.r[11].s64 + 3980;
	// 832B4070: 4AF60D68  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4078 size=12
    let mut pc: u32 = 0x832B4078;
    'dispatch: loop {
        match pc {
            0x832B4078 => {
    //   block [0x832B4078..0x832B4084)
	// 832B4078: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B407C: 386B0F90  addi r3, r11, 0xf90
	ctx.r[3].s64 = ctx.r[11].s64 + 3984;
	// 832B4080: 4AF60D58  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4088 size=12
    let mut pc: u32 = 0x832B4088;
    'dispatch: loop {
        match pc {
            0x832B4088 => {
    //   block [0x832B4088..0x832B4094)
	// 832B4088: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B408C: 386B0F94  addi r3, r11, 0xf94
	ctx.r[3].s64 = ctx.r[11].s64 + 3988;
	// 832B4090: 4AF60D48  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4098 size=12
    let mut pc: u32 = 0x832B4098;
    'dispatch: loop {
        match pc {
            0x832B4098 => {
    //   block [0x832B4098..0x832B40A4)
	// 832B4098: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B409C: 386B0F98  addi r3, r11, 0xf98
	ctx.r[3].s64 = ctx.r[11].s64 + 3992;
	// 832B40A0: 4AF60D38  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B40A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B40A8 size=12
    let mut pc: u32 = 0x832B40A8;
    'dispatch: loop {
        match pc {
            0x832B40A8 => {
    //   block [0x832B40A8..0x832B40B4)
	// 832B40A8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B40AC: 386B0F9C  addi r3, r11, 0xf9c
	ctx.r[3].s64 = ctx.r[11].s64 + 3996;
	// 832B40B0: 4AF60D28  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B40B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B40B8 size=12
    let mut pc: u32 = 0x832B40B8;
    'dispatch: loop {
        match pc {
            0x832B40B8 => {
    //   block [0x832B40B8..0x832B40C4)
	// 832B40B8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B40BC: 386B0FA0  addi r3, r11, 0xfa0
	ctx.r[3].s64 = ctx.r[11].s64 + 4000;
	// 832B40C0: 4AF60D18  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B40C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B40C8 size=12
    let mut pc: u32 = 0x832B40C8;
    'dispatch: loop {
        match pc {
            0x832B40C8 => {
    //   block [0x832B40C8..0x832B40D4)
	// 832B40C8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B40CC: 386B0FA4  addi r3, r11, 0xfa4
	ctx.r[3].s64 = ctx.r[11].s64 + 4004;
	// 832B40D0: 4AF60D08  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B40D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B40D8 size=12
    let mut pc: u32 = 0x832B40D8;
    'dispatch: loop {
        match pc {
            0x832B40D8 => {
    //   block [0x832B40D8..0x832B40E4)
	// 832B40D8: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 832B40DC: 386BAB2C  addi r3, r11, -0x54d4
	ctx.r[3].s64 = ctx.r[11].s64 + -21716;
	// 832B40E0: 4AF47EB0  b 0x821fbf90
	sub_821FBF90(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B40E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B40E8 size=88
    let mut pc: u32 = 0x832B40E8;
    'dispatch: loop {
        match pc {
            0x832B40E8 => {
    //   block [0x832B40E8..0x832B412C)
	// 832B40E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B40EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B40F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B40F4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B40F8: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 832B40FC: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 832B4100: 3BEBAB50  addi r31, r11, -0x54b0
	ctx.r[31].s64 = ctx.r[11].s64 + -21680;
	// 832B4104: 396A2A2C  addi r11, r10, 0x2a2c
	ctx.r[11].s64 = ctx.r[10].s64 + 10796;
	// 832B4108: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B410C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 832B4110: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B4114: 419A0018  beq cr6, 0x832b412c
	if ctx.cr[6].eq {
	pc = 0x832B412C; continue 'dispatch;
	}
	// 832B4118: 4AF48209  bl 0x821fc320
	ctx.lr = 0x832B411C;
	sub_821FC320(ctx, base);
	// 832B411C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B4120: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B4124: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B4128: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	pc = 0x832B412C; continue 'dispatch;
            }
            0x832B412C => {
    //   block [0x832B412C..0x832B4140)
	// 832B412C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B4130: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B4134: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B4138: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B413C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B4140 size=68
    let mut pc: u32 = 0x832B4140;
    'dispatch: loop {
        match pc {
            0x832B4140 => {
    //   block [0x832B4140..0x832B4184)
	// 832B4140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B4144: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B4148: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B414C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B4150: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B4154: 3BEB0FF0  addi r31, r11, 0xff0
	ctx.r[31].s64 = ctx.r[11].s64 + 4080;
	// 832B4158: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B415C: 4AEC8D1D  bl 0x8217ce78
	ctx.lr = 0x832B4160;
	sub_8217CE78(ctx, base);
	// 832B4160: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B4164: 4AF67BD5  bl 0x8221bd38
	ctx.lr = 0x832B4168;
	sub_8221BD38(ctx, base);
	// 832B4168: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B416C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B4170: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B4174: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B4178: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B417C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B4180: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4188 size=12
    let mut pc: u32 = 0x832B4188;
    'dispatch: loop {
        match pc {
            0x832B4188 => {
    //   block [0x832B4188..0x832B4194)
	// 832B4188: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B418C: 386B0FFC  addi r3, r11, 0xffc
	ctx.r[3].s64 = ctx.r[11].s64 + 4092;
	// 832B4190: 4AFE9908  b 0x8229da98
	sub_8229DA98(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4198 size=12
    let mut pc: u32 = 0x832B4198;
    'dispatch: loop {
        match pc {
            0x832B4198 => {
    //   block [0x832B4198..0x832B41A4)
	// 832B4198: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B419C: 386B1048  addi r3, r11, 0x1048
	ctx.r[3].s64 = ctx.r[11].s64 + 4168;
	// 832B41A0: 4AFE98F8  b 0x8229da98
	sub_8229DA98(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B41A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B41A8 size=12
    let mut pc: u32 = 0x832B41A8;
    'dispatch: loop {
        match pc {
            0x832B41A8 => {
    //   block [0x832B41A8..0x832B41B4)
	// 832B41A8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B41AC: 386B1094  addi r3, r11, 0x1094
	ctx.r[3].s64 = ctx.r[11].s64 + 4244;
	// 832B41B0: 4AF60C28  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B41B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B41B8 size=12
    let mut pc: u32 = 0x832B41B8;
    'dispatch: loop {
        match pc {
            0x832B41B8 => {
    //   block [0x832B41B8..0x832B41C4)
	// 832B41B8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B41BC: 386B1098  addi r3, r11, 0x1098
	ctx.r[3].s64 = ctx.r[11].s64 + 4248;
	// 832B41C0: 4AF60C18  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B41C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B41C8 size=12
    let mut pc: u32 = 0x832B41C8;
    'dispatch: loop {
        match pc {
            0x832B41C8 => {
    //   block [0x832B41C8..0x832B41D4)
	// 832B41C8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B41CC: 386B109C  addi r3, r11, 0x109c
	ctx.r[3].s64 = ctx.r[11].s64 + 4252;
	// 832B41D0: 4AF60C08  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B41D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B41D8 size=12
    let mut pc: u32 = 0x832B41D8;
    'dispatch: loop {
        match pc {
            0x832B41D8 => {
    //   block [0x832B41D8..0x832B41E4)
	// 832B41D8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B41DC: 386B10A0  addi r3, r11, 0x10a0
	ctx.r[3].s64 = ctx.r[11].s64 + 4256;
	// 832B41E0: 4AF60BF8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B41E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B41E8 size=12
    let mut pc: u32 = 0x832B41E8;
    'dispatch: loop {
        match pc {
            0x832B41E8 => {
    //   block [0x832B41E8..0x832B41F4)
	// 832B41E8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B41EC: 386B10A4  addi r3, r11, 0x10a4
	ctx.r[3].s64 = ctx.r[11].s64 + 4260;
	// 832B41F0: 4AF60BE8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B41F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B41F8 size=12
    let mut pc: u32 = 0x832B41F8;
    'dispatch: loop {
        match pc {
            0x832B41F8 => {
    //   block [0x832B41F8..0x832B4204)
	// 832B41F8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B41FC: 386B10A8  addi r3, r11, 0x10a8
	ctx.r[3].s64 = ctx.r[11].s64 + 4264;
	// 832B4200: 4AF60BD8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4208 size=20
    let mut pc: u32 = 0x832B4208;
    'dispatch: loop {
        match pc {
            0x832B4208 => {
    //   block [0x832B4208..0x832B421C)
	// 832B4208: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B420C: 806B10AC  lwz r3, 0x10ac(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4268 as u32) ) } as u64;
	// 832B4210: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B4214: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 832B4218: 4AF48108  b 0x821fc320
	sub_821FC320(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B421C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B421C size=4
    let mut pc: u32 = 0x832B421C;
    'dispatch: loop {
        match pc {
            0x832B421C => {
    //   block [0x832B421C..0x832B4220)
	// 832B421C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4220 size=20
    let mut pc: u32 = 0x832B4220;
    'dispatch: loop {
        match pc {
            0x832B4220 => {
    //   block [0x832B4220..0x832B4234)
	// 832B4220: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B4224: 806B10C0  lwz r3, 0x10c0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4288 as u32) ) } as u64;
	// 832B4228: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B422C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 832B4230: 4AF480F0  b 0x821fc320
	sub_821FC320(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4234(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4234 size=4
    let mut pc: u32 = 0x832B4234;
    'dispatch: loop {
        match pc {
            0x832B4234 => {
    //   block [0x832B4234..0x832B4238)
	// 832B4234: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4238 size=20
    let mut pc: u32 = 0x832B4238;
    'dispatch: loop {
        match pc {
            0x832B4238 => {
    //   block [0x832B4238..0x832B424C)
	// 832B4238: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B423C: 806B10D4  lwz r3, 0x10d4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4308 as u32) ) } as u64;
	// 832B4240: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B4244: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 832B4248: 4AF480D8  b 0x821fc320
	sub_821FC320(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B424C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B424C size=4
    let mut pc: u32 = 0x832B424C;
    'dispatch: loop {
        match pc {
            0x832B424C => {
    //   block [0x832B424C..0x832B4250)
	// 832B424C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4250 size=12
    let mut pc: u32 = 0x832B4250;
    'dispatch: loop {
        match pc {
            0x832B4250 => {
    //   block [0x832B4250..0x832B425C)
	// 832B4250: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B4254: 386B10E8  addi r3, r11, 0x10e8
	ctx.r[3].s64 = ctx.r[11].s64 + 4328;
	// 832B4258: 4AF60B80  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4260 size=12
    let mut pc: u32 = 0x832B4260;
    'dispatch: loop {
        match pc {
            0x832B4260 => {
    //   block [0x832B4260..0x832B426C)
	// 832B4260: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B4264: 386B10EC  addi r3, r11, 0x10ec
	ctx.r[3].s64 = ctx.r[11].s64 + 4332;
	// 832B4268: 4AF60B70  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B4270 size=88
    let mut pc: u32 = 0x832B4270;
    'dispatch: loop {
        match pc {
            0x832B4270 => {
    //   block [0x832B4270..0x832B4294)
	// 832B4270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B4274: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B4278: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B427C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B4280: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B4284: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B4288: 3BC00003  li r30, 3
	ctx.r[30].s64 = 3;
	// 832B428C: 396B10F0  addi r11, r11, 0x10f0
	ctx.r[11].s64 = ctx.r[11].s64 + 4336;
	// 832B4290: 3BEB0050  addi r31, r11, 0x50
	ctx.r[31].s64 = ctx.r[11].s64 + 80;
	pc = 0x832B4294; continue 'dispatch;
            }
            0x832B4294 => {
    //   block [0x832B4294..0x832B42A8)
	// 832B4294: 3BFFFFEC  addi r31, r31, -0x14
	ctx.r[31].s64 = ctx.r[31].s64 + -20;
	// 832B4298: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B429C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B42A0: 419A0008  beq cr6, 0x832b42a8
	if ctx.cr[6].eq {
	pc = 0x832B42A8; continue 'dispatch;
	}
	// 832B42A4: 4AF4807D  bl 0x821fc320
	ctx.lr = 0x832B42A8;
	sub_821FC320(ctx, base);
	pc = 0x832B42A8; continue 'dispatch;
            }
            0x832B42A8 => {
    //   block [0x832B42A8..0x832B42C8)
	// 832B42A8: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832B42AC: 4080FFE8  bge 0x832b4294
	if !ctx.cr[0].lt {
	pc = 0x832B4294; continue 'dispatch;
	}
	// 832B42B0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B42B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B42B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B42BC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B42C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B42C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B42C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B42C8 size=20
    let mut pc: u32 = 0x832B42C8;
    'dispatch: loop {
        match pc {
            0x832B42C8 => {
    //   block [0x832B42C8..0x832B42DC)
	// 832B42C8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B42CC: 806B1140  lwz r3, 0x1140(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4416 as u32) ) } as u64;
	// 832B42D0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B42D4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 832B42D8: 4AF48048  b 0x821fc320
	sub_821FC320(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B42DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B42DC size=4
    let mut pc: u32 = 0x832B42DC;
    'dispatch: loop {
        match pc {
            0x832B42DC => {
    //   block [0x832B42DC..0x832B42E0)
	// 832B42DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B42E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B42E0 size=20
    let mut pc: u32 = 0x832B42E0;
    'dispatch: loop {
        match pc {
            0x832B42E0 => {
    //   block [0x832B42E0..0x832B42F4)
	// 832B42E0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B42E4: 806B1154  lwz r3, 0x1154(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4436 as u32) ) } as u64;
	// 832B42E8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B42EC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 832B42F0: 4AF48030  b 0x821fc320
	sub_821FC320(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B42F4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B42F4 size=4
    let mut pc: u32 = 0x832B42F4;
    'dispatch: loop {
        match pc {
            0x832B42F4 => {
    //   block [0x832B42F4..0x832B42F8)
	// 832B42F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B42F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B42F8 size=12
    let mut pc: u32 = 0x832B42F8;
    'dispatch: loop {
        match pc {
            0x832B42F8 => {
    //   block [0x832B42F8..0x832B4304)
	// 832B42F8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B42FC: 386B1190  addi r3, r11, 0x1190
	ctx.r[3].s64 = ctx.r[11].s64 + 4496;
	// 832B4300: 4AF60AD8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4308 size=12
    let mut pc: u32 = 0x832B4308;
    'dispatch: loop {
        match pc {
            0x832B4308 => {
    //   block [0x832B4308..0x832B4314)
	// 832B4308: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B430C: 386B1194  addi r3, r11, 0x1194
	ctx.r[3].s64 = ctx.r[11].s64 + 4500;
	// 832B4310: 4AF60AC8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B4318 size=88
    let mut pc: u32 = 0x832B4318;
    'dispatch: loop {
        match pc {
            0x832B4318 => {
    //   block [0x832B4318..0x832B433C)
	// 832B4318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B431C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B4320: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B4324: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B4328: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B432C: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B4330: 3BC00003  li r30, 3
	ctx.r[30].s64 = 3;
	// 832B4334: 396B1198  addi r11, r11, 0x1198
	ctx.r[11].s64 = ctx.r[11].s64 + 4504;
	// 832B4338: 3BEB0050  addi r31, r11, 0x50
	ctx.r[31].s64 = ctx.r[11].s64 + 80;
	pc = 0x832B433C; continue 'dispatch;
            }
            0x832B433C => {
    //   block [0x832B433C..0x832B4350)
	// 832B433C: 3BFFFFEC  addi r31, r31, -0x14
	ctx.r[31].s64 = ctx.r[31].s64 + -20;
	// 832B4340: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B4344: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B4348: 419A0008  beq cr6, 0x832b4350
	if ctx.cr[6].eq {
	pc = 0x832B4350; continue 'dispatch;
	}
	// 832B434C: 4AF47FD5  bl 0x821fc320
	ctx.lr = 0x832B4350;
	sub_821FC320(ctx, base);
	pc = 0x832B4350; continue 'dispatch;
            }
            0x832B4350 => {
    //   block [0x832B4350..0x832B4370)
	// 832B4350: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832B4354: 4080FFE8  bge 0x832b433c
	if !ctx.cr[0].lt {
	pc = 0x832B433C; continue 'dispatch;
	}
	// 832B4358: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B435C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B4360: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B4364: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B4368: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B436C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4370 size=20
    let mut pc: u32 = 0x832B4370;
    'dispatch: loop {
        match pc {
            0x832B4370 => {
    //   block [0x832B4370..0x832B4384)
	// 832B4370: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B4374: 806B11E8  lwz r3, 0x11e8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4584 as u32) ) } as u64;
	// 832B4378: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B437C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 832B4380: 4AF47FA0  b 0x821fc320
	sub_821FC320(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4384(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4384 size=4
    let mut pc: u32 = 0x832B4384;
    'dispatch: loop {
        match pc {
            0x832B4384 => {
    //   block [0x832B4384..0x832B4388)
	// 832B4384: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4388 size=12
    let mut pc: u32 = 0x832B4388;
    'dispatch: loop {
        match pc {
            0x832B4388 => {
    //   block [0x832B4388..0x832B4394)
	// 832B4388: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B438C: 386B11FC  addi r3, r11, 0x11fc
	ctx.r[3].s64 = ctx.r[11].s64 + 4604;
	// 832B4390: 4AF60A48  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4398 size=12
    let mut pc: u32 = 0x832B4398;
    'dispatch: loop {
        match pc {
            0x832B4398 => {
    //   block [0x832B4398..0x832B43A4)
	// 832B4398: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B439C: 386B1200  addi r3, r11, 0x1200
	ctx.r[3].s64 = ctx.r[11].s64 + 4608;
	// 832B43A0: 4AF60A38  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B43A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B43A8 size=20
    let mut pc: u32 = 0x832B43A8;
    'dispatch: loop {
        match pc {
            0x832B43A8 => {
    //   block [0x832B43A8..0x832B43BC)
	// 832B43A8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B43AC: 806B1204  lwz r3, 0x1204(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4612 as u32) ) } as u64;
	// 832B43B0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B43B4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 832B43B8: 4AF47F68  b 0x821fc320
	sub_821FC320(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B43BC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B43BC size=4
    let mut pc: u32 = 0x832B43BC;
    'dispatch: loop {
        match pc {
            0x832B43BC => {
    //   block [0x832B43BC..0x832B43C0)
	// 832B43BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B43C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B43C0 size=12
    let mut pc: u32 = 0x832B43C0;
    'dispatch: loop {
        match pc {
            0x832B43C0 => {
    //   block [0x832B43C0..0x832B43CC)
	// 832B43C0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B43C4: 386B1280  addi r3, r11, 0x1280
	ctx.r[3].s64 = ctx.r[11].s64 + 4736;
	// 832B43C8: 4AF60A10  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B43D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B43D0 size=12
    let mut pc: u32 = 0x832B43D0;
    'dispatch: loop {
        match pc {
            0x832B43D0 => {
    //   block [0x832B43D0..0x832B43DC)
	// 832B43D0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B43D4: 386B1284  addi r3, r11, 0x1284
	ctx.r[3].s64 = ctx.r[11].s64 + 4740;
	// 832B43D8: 4AF60A00  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B43E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B43E0 size=12
    let mut pc: u32 = 0x832B43E0;
    'dispatch: loop {
        match pc {
            0x832B43E0 => {
    //   block [0x832B43E0..0x832B43EC)
	// 832B43E0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B43E4: 386B12A0  addi r3, r11, 0x12a0
	ctx.r[3].s64 = ctx.r[11].s64 + 4768;
	// 832B43E8: 4AF609F0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B43F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B43F0 size=12
    let mut pc: u32 = 0x832B43F0;
    'dispatch: loop {
        match pc {
            0x832B43F0 => {
    //   block [0x832B43F0..0x832B43FC)
	// 832B43F0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B43F4: 386B12A4  addi r3, r11, 0x12a4
	ctx.r[3].s64 = ctx.r[11].s64 + 4772;
	// 832B43F8: 4AF609E0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B4400 size=124
    let mut pc: u32 = 0x832B4400;
    'dispatch: loop {
        match pc {
            0x832B4400 => {
    //   block [0x832B4400..0x832B4428)
	// 832B4400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B4404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B4408: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B440C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B4410: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832B4414: 3BEB2300  addi r31, r11, 0x2300
	ctx.r[31].s64 = ctx.r[11].s64 + 8960;
	// 832B4418: 807F0068  lwz r3, 0x68(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 832B441C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B4420: 419A0008  beq cr6, 0x832b4428
	if ctx.cr[6].eq {
	pc = 0x832B4428; continue 'dispatch;
	}
	// 832B4424: 4AF67915  bl 0x8221bd38
	ctx.lr = 0x832B4428;
	sub_8221BD38(ctx, base);
	pc = 0x832B4428; continue 'dispatch;
            }
            0x832B4428 => {
    //   block [0x832B4428..0x832B4450)
	// 832B4428: 807F0058  lwz r3, 0x58(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 832B442C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B4430: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B4434: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B4438: 917F0068  stw r11, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 832B443C: 915F006C  stw r10, 0x6c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[10].u32 ) };
	// 832B4440: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B4444: 913F0070  stw r9, 0x70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[9].u32 ) };
	// 832B4448: 419A0008  beq cr6, 0x832b4450
	if ctx.cr[6].eq {
	pc = 0x832B4450; continue 'dispatch;
	}
	// 832B444C: 4AF678ED  bl 0x8221bd38
	ctx.lr = 0x832B4450;
	sub_8221BD38(ctx, base);
	pc = 0x832B4450; continue 'dispatch;
            }
            0x832B4450 => {
    //   block [0x832B4450..0x832B447C)
	// 832B4450: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B4454: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B4458: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B445C: 917F0058  stw r11, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 832B4460: 915F005C  stw r10, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 832B4464: 913F0060  stw r9, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[9].u32 ) };
	// 832B4468: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B446C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B4470: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B4474: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B4478: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B44A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B44A0 size=88
    let mut pc: u32 = 0x832B44A0;
    'dispatch: loop {
        match pc {
            0x832B44A0 => {
    //   block [0x832B44A0..0x832B44C4)
	// 832B44A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B44A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B44A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B44AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B44B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B44B4: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B44B8: 3BC00003  li r30, 3
	ctx.r[30].s64 = 3;
	// 832B44BC: 396B12B0  addi r11, r11, 0x12b0
	ctx.r[11].s64 = ctx.r[11].s64 + 4784;
	// 832B44C0: 3BEB0050  addi r31, r11, 0x50
	ctx.r[31].s64 = ctx.r[11].s64 + 80;
	pc = 0x832B44C4; continue 'dispatch;
            }
            0x832B44C4 => {
    //   block [0x832B44C4..0x832B44D8)
	// 832B44C4: 3BFFFFEC  addi r31, r31, -0x14
	ctx.r[31].s64 = ctx.r[31].s64 + -20;
	// 832B44C8: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B44CC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B44D0: 419A0008  beq cr6, 0x832b44d8
	if ctx.cr[6].eq {
	pc = 0x832B44D8; continue 'dispatch;
	}
	// 832B44D4: 4AF47E4D  bl 0x821fc320
	ctx.lr = 0x832B44D8;
	sub_821FC320(ctx, base);
	pc = 0x832B44D8; continue 'dispatch;
            }
            0x832B44D8 => {
    //   block [0x832B44D8..0x832B44F8)
	// 832B44D8: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832B44DC: 4080FFE8  bge 0x832b44c4
	if !ctx.cr[0].lt {
	pc = 0x832B44C4; continue 'dispatch;
	}
	// 832B44E0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B44E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B44E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B44EC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B44F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B44F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B44F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B44F8 size=12
    let mut pc: u32 = 0x832B44F8;
    'dispatch: loop {
        match pc {
            0x832B44F8 => {
    //   block [0x832B44F8..0x832B4504)
	// 832B44F8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B44FC: 386B1328  addi r3, r11, 0x1328
	ctx.r[3].s64 = ctx.r[11].s64 + 4904;
	// 832B4500: 4AF608D8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4508 size=12
    let mut pc: u32 = 0x832B4508;
    'dispatch: loop {
        match pc {
            0x832B4508 => {
    //   block [0x832B4508..0x832B4514)
	// 832B4508: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B450C: 386B132C  addi r3, r11, 0x132c
	ctx.r[3].s64 = ctx.r[11].s64 + 4908;
	// 832B4510: 4AF608C8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4518 size=12
    let mut pc: u32 = 0x832B4518;
    'dispatch: loop {
        match pc {
            0x832B4518 => {
    //   block [0x832B4518..0x832B4524)
	// 832B4518: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B451C: 386B4330  addi r3, r11, 0x4330
	ctx.r[3].s64 = ctx.r[11].s64 + 17200;
	// 832B4520: 4AF608B8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4528 size=12
    let mut pc: u32 = 0x832B4528;
    'dispatch: loop {
        match pc {
            0x832B4528 => {
    //   block [0x832B4528..0x832B4534)
	// 832B4528: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B452C: 386B4334  addi r3, r11, 0x4334
	ctx.r[3].s64 = ctx.r[11].s64 + 17204;
	// 832B4530: 4AF608A8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4538 size=12
    let mut pc: u32 = 0x832B4538;
    'dispatch: loop {
        match pc {
            0x832B4538 => {
    //   block [0x832B4538..0x832B4544)
	// 832B4538: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B453C: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 832B4540: 4AF60898  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4548 size=12
    let mut pc: u32 = 0x832B4548;
    'dispatch: loop {
        match pc {
            0x832B4548 => {
    //   block [0x832B4548..0x832B4554)
	// 832B4548: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B454C: 386B433C  addi r3, r11, 0x433c
	ctx.r[3].s64 = ctx.r[11].s64 + 17212;
	// 832B4550: 4AF60888  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B4558 size=116
    let mut pc: u32 = 0x832B4558;
    'dispatch: loop {
        match pc {
            0x832B4558 => {
    //   block [0x832B4558..0x832B457C)
	// 832B4558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B455C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B4560: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B4564: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B4568: 3FE0834A  lis r31, -0x7cb6
	ctx.r[31].s64 = -2092302336;
	// 832B456C: 807F4340  lwz r3, 0x4340(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(17216 as u32) ) } as u64;
	// 832B4570: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B4574: 419A0044  beq cr6, 0x832b45b8
	if ctx.cr[6].eq {
	pc = 0x832B45B8; continue 'dispatch;
	}
	// 832B4578: 39230004  addi r9, r3, 4
	ctx.r[9].s64 = ctx.r[3].s64 + 4;
	pc = 0x832B457C; continue 'dispatch;
            }
            0x832B457C => {
    //   block [0x832B457C..0x832B45B0)
	// 832B457C: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832B4580: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832B4584: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832B4588: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832B458C: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832B4590: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832B4594: 4082FFE8  bne 0x832b457c
	if !ctx.cr[0].eq {
	pc = 0x832B457C; continue 'dispatch;
	}
	// 832B4598: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 832B459C: 409A0014  bne cr6, 0x832b45b0
	if !ctx.cr[6].eq {
	pc = 0x832B45B0; continue 'dispatch;
	}
	// 832B45A0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B45A4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B45A8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 832B45AC: 4E800421  bctrl
	ctx.lr = 0x832B45B0;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x832B45B0 => {
    //   block [0x832B45B0..0x832B45B8)
	// 832B45B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B45B4: 917F4340  stw r11, 0x4340(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(17216 as u32), ctx.r[11].u32 ) };
	pc = 0x832B45B8; continue 'dispatch;
            }
            0x832B45B8 => {
    //   block [0x832B45B8..0x832B45CC)
	// 832B45B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B45BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B45C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B45C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B45C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B45D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B45D0 size=116
    let mut pc: u32 = 0x832B45D0;
    'dispatch: loop {
        match pc {
            0x832B45D0 => {
    //   block [0x832B45D0..0x832B45F4)
	// 832B45D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B45D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B45D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B45DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B45E0: 3FE0834A  lis r31, -0x7cb6
	ctx.r[31].s64 = -2092302336;
	// 832B45E4: 807F4344  lwz r3, 0x4344(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(17220 as u32) ) } as u64;
	// 832B45E8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B45EC: 419A0044  beq cr6, 0x832b4630
	if ctx.cr[6].eq {
	pc = 0x832B4630; continue 'dispatch;
	}
	// 832B45F0: 39230004  addi r9, r3, 4
	ctx.r[9].s64 = ctx.r[3].s64 + 4;
	pc = 0x832B45F4; continue 'dispatch;
            }
            0x832B45F4 => {
    //   block [0x832B45F4..0x832B4628)
	// 832B45F4: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832B45F8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832B45FC: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832B4600: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832B4604: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832B4608: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832B460C: 4082FFE8  bne 0x832b45f4
	if !ctx.cr[0].eq {
	pc = 0x832B45F4; continue 'dispatch;
	}
	// 832B4610: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 832B4614: 409A0014  bne cr6, 0x832b4628
	if !ctx.cr[6].eq {
	pc = 0x832B4628; continue 'dispatch;
	}
	// 832B4618: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B461C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B4620: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 832B4624: 4E800421  bctrl
	ctx.lr = 0x832B4628;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x832B4628 => {
    //   block [0x832B4628..0x832B4630)
	// 832B4628: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B462C: 917F4344  stw r11, 0x4344(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(17220 as u32), ctx.r[11].u32 ) };
	pc = 0x832B4630; continue 'dispatch;
            }
            0x832B4630 => {
    //   block [0x832B4630..0x832B4644)
	// 832B4630: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B4634: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B4638: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B463C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B4640: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B4648 size=116
    let mut pc: u32 = 0x832B4648;
    'dispatch: loop {
        match pc {
            0x832B4648 => {
    //   block [0x832B4648..0x832B466C)
	// 832B4648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B464C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B4650: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B4654: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B4658: 3FE0834A  lis r31, -0x7cb6
	ctx.r[31].s64 = -2092302336;
	// 832B465C: 807F4348  lwz r3, 0x4348(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(17224 as u32) ) } as u64;
	// 832B4660: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B4664: 419A0044  beq cr6, 0x832b46a8
	if ctx.cr[6].eq {
	pc = 0x832B46A8; continue 'dispatch;
	}
	// 832B4668: 39230004  addi r9, r3, 4
	ctx.r[9].s64 = ctx.r[3].s64 + 4;
	pc = 0x832B466C; continue 'dispatch;
            }
            0x832B466C => {
    //   block [0x832B466C..0x832B46A0)
	// 832B466C: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832B4670: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832B4674: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832B4678: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832B467C: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832B4680: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832B4684: 4082FFE8  bne 0x832b466c
	if !ctx.cr[0].eq {
	pc = 0x832B466C; continue 'dispatch;
	}
	// 832B4688: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 832B468C: 409A0014  bne cr6, 0x832b46a0
	if !ctx.cr[6].eq {
	pc = 0x832B46A0; continue 'dispatch;
	}
	// 832B4690: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B4694: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B4698: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 832B469C: 4E800421  bctrl
	ctx.lr = 0x832B46A0;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x832B46A0 => {
    //   block [0x832B46A0..0x832B46A8)
	// 832B46A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B46A4: 917F4348  stw r11, 0x4348(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(17224 as u32), ctx.r[11].u32 ) };
	pc = 0x832B46A8; continue 'dispatch;
            }
            0x832B46A8 => {
    //   block [0x832B46A8..0x832B46BC)
	// 832B46A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B46AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B46B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B46B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B46B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B46C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B46C0 size=116
    let mut pc: u32 = 0x832B46C0;
    'dispatch: loop {
        match pc {
            0x832B46C0 => {
    //   block [0x832B46C0..0x832B46E4)
	// 832B46C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B46C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B46C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B46CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B46D0: 3FE0834A  lis r31, -0x7cb6
	ctx.r[31].s64 = -2092302336;
	// 832B46D4: 807F434C  lwz r3, 0x434c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(17228 as u32) ) } as u64;
	// 832B46D8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B46DC: 419A0044  beq cr6, 0x832b4720
	if ctx.cr[6].eq {
	pc = 0x832B4720; continue 'dispatch;
	}
	// 832B46E0: 39230004  addi r9, r3, 4
	ctx.r[9].s64 = ctx.r[3].s64 + 4;
	pc = 0x832B46E4; continue 'dispatch;
            }
            0x832B46E4 => {
    //   block [0x832B46E4..0x832B4718)
	// 832B46E4: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832B46E8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832B46EC: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832B46F0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832B46F4: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832B46F8: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832B46FC: 4082FFE8  bne 0x832b46e4
	if !ctx.cr[0].eq {
	pc = 0x832B46E4; continue 'dispatch;
	}
	// 832B4700: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 832B4704: 409A0014  bne cr6, 0x832b4718
	if !ctx.cr[6].eq {
	pc = 0x832B4718; continue 'dispatch;
	}
	// 832B4708: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B470C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B4710: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 832B4714: 4E800421  bctrl
	ctx.lr = 0x832B4718;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x832B4718 => {
    //   block [0x832B4718..0x832B4720)
	// 832B4718: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B471C: 917F434C  stw r11, 0x434c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(17228 as u32), ctx.r[11].u32 ) };
	pc = 0x832B4720; continue 'dispatch;
            }
            0x832B4720 => {
    //   block [0x832B4720..0x832B4734)
	// 832B4720: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B4724: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B4728: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B472C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B4730: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B4738 size=116
    let mut pc: u32 = 0x832B4738;
    'dispatch: loop {
        match pc {
            0x832B4738 => {
    //   block [0x832B4738..0x832B475C)
	// 832B4738: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B473C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B4740: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B4744: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B4748: 3FE0834A  lis r31, -0x7cb6
	ctx.r[31].s64 = -2092302336;
	// 832B474C: 807F4350  lwz r3, 0x4350(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(17232 as u32) ) } as u64;
	// 832B4750: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B4754: 419A0044  beq cr6, 0x832b4798
	if ctx.cr[6].eq {
	pc = 0x832B4798; continue 'dispatch;
	}
	// 832B4758: 39230004  addi r9, r3, 4
	ctx.r[9].s64 = ctx.r[3].s64 + 4;
	pc = 0x832B475C; continue 'dispatch;
            }
            0x832B475C => {
    //   block [0x832B475C..0x832B4790)
	// 832B475C: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832B4760: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832B4764: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832B4768: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832B476C: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832B4770: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832B4774: 4082FFE8  bne 0x832b475c
	if !ctx.cr[0].eq {
	pc = 0x832B475C; continue 'dispatch;
	}
	// 832B4778: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 832B477C: 409A0014  bne cr6, 0x832b4790
	if !ctx.cr[6].eq {
	pc = 0x832B4790; continue 'dispatch;
	}
	// 832B4780: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B4784: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B4788: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 832B478C: 4E800421  bctrl
	ctx.lr = 0x832B4790;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x832B4790 => {
    //   block [0x832B4790..0x832B4798)
	// 832B4790: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B4794: 917F4350  stw r11, 0x4350(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(17232 as u32), ctx.r[11].u32 ) };
	pc = 0x832B4798; continue 'dispatch;
            }
            0x832B4798 => {
    //   block [0x832B4798..0x832B47AC)
	// 832B4798: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B479C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B47A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B47A4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B47A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B47B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B47B0 size=116
    let mut pc: u32 = 0x832B47B0;
    'dispatch: loop {
        match pc {
            0x832B47B0 => {
    //   block [0x832B47B0..0x832B47D4)
	// 832B47B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B47B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B47B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B47BC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B47C0: 3FE0834A  lis r31, -0x7cb6
	ctx.r[31].s64 = -2092302336;
	// 832B47C4: 807F4354  lwz r3, 0x4354(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(17236 as u32) ) } as u64;
	// 832B47C8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B47CC: 419A0044  beq cr6, 0x832b4810
	if ctx.cr[6].eq {
	pc = 0x832B4810; continue 'dispatch;
	}
	// 832B47D0: 39230004  addi r9, r3, 4
	ctx.r[9].s64 = ctx.r[3].s64 + 4;
	pc = 0x832B47D4; continue 'dispatch;
            }
            0x832B47D4 => {
    //   block [0x832B47D4..0x832B4808)
	// 832B47D4: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832B47D8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832B47DC: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832B47E0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832B47E4: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832B47E8: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832B47EC: 4082FFE8  bne 0x832b47d4
	if !ctx.cr[0].eq {
	pc = 0x832B47D4; continue 'dispatch;
	}
	// 832B47F0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 832B47F4: 409A0014  bne cr6, 0x832b4808
	if !ctx.cr[6].eq {
	pc = 0x832B4808; continue 'dispatch;
	}
	// 832B47F8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B47FC: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B4800: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 832B4804: 4E800421  bctrl
	ctx.lr = 0x832B4808;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x832B4808 => {
    //   block [0x832B4808..0x832B4810)
	// 832B4808: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B480C: 917F4354  stw r11, 0x4354(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(17236 as u32), ctx.r[11].u32 ) };
	pc = 0x832B4810; continue 'dispatch;
            }
            0x832B4810 => {
    //   block [0x832B4810..0x832B4824)
	// 832B4810: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B4814: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B4818: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B481C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B4820: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B4828 size=116
    let mut pc: u32 = 0x832B4828;
    'dispatch: loop {
        match pc {
            0x832B4828 => {
    //   block [0x832B4828..0x832B484C)
	// 832B4828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B482C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B4830: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B4834: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B4838: 3FE0834A  lis r31, -0x7cb6
	ctx.r[31].s64 = -2092302336;
	// 832B483C: 807F4358  lwz r3, 0x4358(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(17240 as u32) ) } as u64;
	// 832B4840: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B4844: 419A0044  beq cr6, 0x832b4888
	if ctx.cr[6].eq {
	pc = 0x832B4888; continue 'dispatch;
	}
	// 832B4848: 39230004  addi r9, r3, 4
	ctx.r[9].s64 = ctx.r[3].s64 + 4;
	pc = 0x832B484C; continue 'dispatch;
            }
            0x832B484C => {
    //   block [0x832B484C..0x832B4880)
	// 832B484C: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832B4850: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832B4854: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832B4858: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832B485C: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832B4860: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832B4864: 4082FFE8  bne 0x832b484c
	if !ctx.cr[0].eq {
	pc = 0x832B484C; continue 'dispatch;
	}
	// 832B4868: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 832B486C: 409A0014  bne cr6, 0x832b4880
	if !ctx.cr[6].eq {
	pc = 0x832B4880; continue 'dispatch;
	}
	// 832B4870: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B4874: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B4878: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 832B487C: 4E800421  bctrl
	ctx.lr = 0x832B4880;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x832B4880 => {
    //   block [0x832B4880..0x832B4888)
	// 832B4880: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B4884: 917F4358  stw r11, 0x4358(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(17240 as u32), ctx.r[11].u32 ) };
	pc = 0x832B4888; continue 'dispatch;
            }
            0x832B4888 => {
    //   block [0x832B4888..0x832B489C)
	// 832B4888: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B488C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B4890: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B4894: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B4898: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B48A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B48A0 size=116
    let mut pc: u32 = 0x832B48A0;
    'dispatch: loop {
        match pc {
            0x832B48A0 => {
    //   block [0x832B48A0..0x832B48C4)
	// 832B48A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B48A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B48A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B48AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B48B0: 3FE0834A  lis r31, -0x7cb6
	ctx.r[31].s64 = -2092302336;
	// 832B48B4: 807F435C  lwz r3, 0x435c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(17244 as u32) ) } as u64;
	// 832B48B8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B48BC: 419A0044  beq cr6, 0x832b4900
	if ctx.cr[6].eq {
	pc = 0x832B4900; continue 'dispatch;
	}
	// 832B48C0: 39230004  addi r9, r3, 4
	ctx.r[9].s64 = ctx.r[3].s64 + 4;
	pc = 0x832B48C4; continue 'dispatch;
            }
            0x832B48C4 => {
    //   block [0x832B48C4..0x832B48F8)
	// 832B48C4: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832B48C8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832B48CC: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832B48D0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832B48D4: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832B48D8: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832B48DC: 4082FFE8  bne 0x832b48c4
	if !ctx.cr[0].eq {
	pc = 0x832B48C4; continue 'dispatch;
	}
	// 832B48E0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 832B48E4: 409A0014  bne cr6, 0x832b48f8
	if !ctx.cr[6].eq {
	pc = 0x832B48F8; continue 'dispatch;
	}
	// 832B48E8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B48EC: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B48F0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 832B48F4: 4E800421  bctrl
	ctx.lr = 0x832B48F8;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x832B48F8 => {
    //   block [0x832B48F8..0x832B4900)
	// 832B48F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B48FC: 917F435C  stw r11, 0x435c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(17244 as u32), ctx.r[11].u32 ) };
	pc = 0x832B4900; continue 'dispatch;
            }
            0x832B4900 => {
    //   block [0x832B4900..0x832B4914)
	// 832B4900: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B4904: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B4908: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B490C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B4910: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B4918 size=116
    let mut pc: u32 = 0x832B4918;
    'dispatch: loop {
        match pc {
            0x832B4918 => {
    //   block [0x832B4918..0x832B493C)
	// 832B4918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B491C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B4920: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B4924: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B4928: 3FE0834A  lis r31, -0x7cb6
	ctx.r[31].s64 = -2092302336;
	// 832B492C: 807F4360  lwz r3, 0x4360(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(17248 as u32) ) } as u64;
	// 832B4930: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B4934: 419A0044  beq cr6, 0x832b4978
	if ctx.cr[6].eq {
	pc = 0x832B4978; continue 'dispatch;
	}
	// 832B4938: 39230004  addi r9, r3, 4
	ctx.r[9].s64 = ctx.r[3].s64 + 4;
	pc = 0x832B493C; continue 'dispatch;
            }
            0x832B493C => {
    //   block [0x832B493C..0x832B4970)
	// 832B493C: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832B4940: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832B4944: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832B4948: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832B494C: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832B4950: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832B4954: 4082FFE8  bne 0x832b493c
	if !ctx.cr[0].eq {
	pc = 0x832B493C; continue 'dispatch;
	}
	// 832B4958: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 832B495C: 409A0014  bne cr6, 0x832b4970
	if !ctx.cr[6].eq {
	pc = 0x832B4970; continue 'dispatch;
	}
	// 832B4960: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B4964: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B4968: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 832B496C: 4E800421  bctrl
	ctx.lr = 0x832B4970;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x832B4970 => {
    //   block [0x832B4970..0x832B4978)
	// 832B4970: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B4974: 917F4360  stw r11, 0x4360(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(17248 as u32), ctx.r[11].u32 ) };
	pc = 0x832B4978; continue 'dispatch;
            }
            0x832B4978 => {
    //   block [0x832B4978..0x832B498C)
	// 832B4978: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B497C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B4980: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B4984: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B4988: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B49A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B49A0 size=116
    let mut pc: u32 = 0x832B49A0;
    'dispatch: loop {
        match pc {
            0x832B49A0 => {
    //   block [0x832B49A0..0x832B49C4)
	// 832B49A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B49A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B49A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B49AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B49B0: 3FE0834A  lis r31, -0x7cb6
	ctx.r[31].s64 = -2092302336;
	// 832B49B4: 807F4364  lwz r3, 0x4364(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(17252 as u32) ) } as u64;
	// 832B49B8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B49BC: 419A0044  beq cr6, 0x832b4a00
	if ctx.cr[6].eq {
	pc = 0x832B4A00; continue 'dispatch;
	}
	// 832B49C0: 39230004  addi r9, r3, 4
	ctx.r[9].s64 = ctx.r[3].s64 + 4;
	pc = 0x832B49C4; continue 'dispatch;
            }
            0x832B49C4 => {
    //   block [0x832B49C4..0x832B49F8)
	// 832B49C4: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832B49C8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832B49CC: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832B49D0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832B49D4: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832B49D8: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832B49DC: 4082FFE8  bne 0x832b49c4
	if !ctx.cr[0].eq {
	pc = 0x832B49C4; continue 'dispatch;
	}
	// 832B49E0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 832B49E4: 409A0014  bne cr6, 0x832b49f8
	if !ctx.cr[6].eq {
	pc = 0x832B49F8; continue 'dispatch;
	}
	// 832B49E8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B49EC: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B49F0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 832B49F4: 4E800421  bctrl
	ctx.lr = 0x832B49F8;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x832B49F8 => {
    //   block [0x832B49F8..0x832B4A00)
	// 832B49F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B49FC: 917F4364  stw r11, 0x4364(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(17252 as u32), ctx.r[11].u32 ) };
	pc = 0x832B4A00; continue 'dispatch;
            }
            0x832B4A00 => {
    //   block [0x832B4A00..0x832B4A14)
	// 832B4A00: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B4A04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B4A08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B4A0C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B4A10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B4A48 size=92
    let mut pc: u32 = 0x832B4A48;
    'dispatch: loop {
        match pc {
            0x832B4A48 => {
    //   block [0x832B4A48..0x832B4AA4)
	// 832B4A48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B4A4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B4A50: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B4A54: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B4A58: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B4A5C: 3FE08332  lis r31, -0x7cce
	ctx.r[31].s64 = -2093875200;
	// 832B4A60: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832B4A64: 3BDFABB8  addi r30, r31, -0x5448
	ctx.r[30].s64 = ctx.r[31].s64 + -21576;
	// 832B4A68: 396B2A30  addi r11, r11, 0x2a30
	ctx.r[11].s64 = ctx.r[11].s64 + 10800;
	// 832B4A6C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832B4A70: 917FABB8  stw r11, -0x5448(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-21576 as u32), ctx.r[11].u32 ) };
	// 832B4A74: 4AF47595  bl 0x821fc008
	ctx.lr = 0x832B4A78;
	sub_821FC008(ctx, base);
	// 832B4A78: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832B4A7C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832B4A80: 396B2A40  addi r11, r11, 0x2a40
	ctx.r[11].s64 = ctx.r[11].s64 + 10816;
	// 832B4A84: 917FABB8  stw r11, -0x5448(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-21576 as u32), ctx.r[11].u32 ) };
	// 832B4A88: 4AF47581  bl 0x821fc008
	ctx.lr = 0x832B4A8C;
	sub_821FC008(ctx, base);
	// 832B4A8C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B4A90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B4A94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B4A98: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B4A9C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B4AA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B4AA8 size=92
    let mut pc: u32 = 0x832B4AA8;
    'dispatch: loop {
        match pc {
            0x832B4AA8 => {
    //   block [0x832B4AA8..0x832B4B04)
	// 832B4AA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B4AAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B4AB0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B4AB4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B4AB8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B4ABC: 3FE08332  lis r31, -0x7cce
	ctx.r[31].s64 = -2093875200;
	// 832B4AC0: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832B4AC4: 3BDFABC0  addi r30, r31, -0x5440
	ctx.r[30].s64 = ctx.r[31].s64 + -21568;
	// 832B4AC8: 396B2A30  addi r11, r11, 0x2a30
	ctx.r[11].s64 = ctx.r[11].s64 + 10800;
	// 832B4ACC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832B4AD0: 917FABC0  stw r11, -0x5440(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-21568 as u32), ctx.r[11].u32 ) };
	// 832B4AD4: 4AF47535  bl 0x821fc008
	ctx.lr = 0x832B4AD8;
	sub_821FC008(ctx, base);
	// 832B4AD8: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832B4ADC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832B4AE0: 396B2A40  addi r11, r11, 0x2a40
	ctx.r[11].s64 = ctx.r[11].s64 + 10816;
	// 832B4AE4: 917FABC0  stw r11, -0x5440(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-21568 as u32), ctx.r[11].u32 ) };
	// 832B4AE8: 4AF47521  bl 0x821fc008
	ctx.lr = 0x832B4AEC;
	sub_821FC008(ctx, base);
	// 832B4AEC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B4AF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B4AF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B4AF8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B4AFC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B4B00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4B08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4B08 size=12
    let mut pc: u32 = 0x832B4B08;
    'dispatch: loop {
        match pc {
            0x832B4B08 => {
    //   block [0x832B4B08..0x832B4B14)
	// 832B4B08: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B4B0C: 386B4390  addi r3, r11, 0x4390
	ctx.r[3].s64 = ctx.r[11].s64 + 17296;
	// 832B4B10: 4AF602C8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4B18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4B18 size=12
    let mut pc: u32 = 0x832B4B18;
    'dispatch: loop {
        match pc {
            0x832B4B18 => {
    //   block [0x832B4B18..0x832B4B24)
	// 832B4B18: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B4B1C: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 832B4B20: 4AF602B8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4B28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4B28 size=12
    let mut pc: u32 = 0x832B4B28;
    'dispatch: loop {
        match pc {
            0x832B4B28 => {
    //   block [0x832B4B28..0x832B4B34)
	// 832B4B28: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832B4B2C: 386B3530  addi r3, r11, 0x3530
	ctx.r[3].s64 = ctx.r[11].s64 + 13616;
	// 832B4B30: 4AF602A8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4B38 size=12
    let mut pc: u32 = 0x832B4B38;
    'dispatch: loop {
        match pc {
            0x832B4B38 => {
    //   block [0x832B4B38..0x832B4B44)
	// 832B4B38: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B4B3C: 386B4398  addi r3, r11, 0x4398
	ctx.r[3].s64 = ctx.r[11].s64 + 17304;
	// 832B4B40: 4AF60298  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4B48 size=12
    let mut pc: u32 = 0x832B4B48;
    'dispatch: loop {
        match pc {
            0x832B4B48 => {
    //   block [0x832B4B48..0x832B4B54)
	// 832B4B48: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B4B4C: 386B439C  addi r3, r11, 0x439c
	ctx.r[3].s64 = ctx.r[11].s64 + 17308;
	// 832B4B50: 4AF60288  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4B58 size=12
    let mut pc: u32 = 0x832B4B58;
    'dispatch: loop {
        match pc {
            0x832B4B58 => {
    //   block [0x832B4B58..0x832B4B64)
	// 832B4B58: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B4B5C: 386B43A0  addi r3, r11, 0x43a0
	ctx.r[3].s64 = ctx.r[11].s64 + 17312;
	// 832B4B60: 4AF60278  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4B68 size=12
    let mut pc: u32 = 0x832B4B68;
    'dispatch: loop {
        match pc {
            0x832B4B68 => {
    //   block [0x832B4B68..0x832B4B74)
	// 832B4B68: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B4B6C: 386B43A4  addi r3, r11, 0x43a4
	ctx.r[3].s64 = ctx.r[11].s64 + 17316;
	// 832B4B70: 4AF60268  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4B78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4B78 size=12
    let mut pc: u32 = 0x832B4B78;
    'dispatch: loop {
        match pc {
            0x832B4B78 => {
    //   block [0x832B4B78..0x832B4B84)
	// 832B4B78: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B4B7C: 386B43A8  addi r3, r11, 0x43a8
	ctx.r[3].s64 = ctx.r[11].s64 + 17320;
	// 832B4B80: 4AF60258  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4B88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4B88 size=12
    let mut pc: u32 = 0x832B4B88;
    'dispatch: loop {
        match pc {
            0x832B4B88 => {
    //   block [0x832B4B88..0x832B4B94)
	// 832B4B88: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B4B8C: 386B43AC  addi r3, r11, 0x43ac
	ctx.r[3].s64 = ctx.r[11].s64 + 17324;
	// 832B4B90: 4AF60248  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4B98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4B98 size=12
    let mut pc: u32 = 0x832B4B98;
    'dispatch: loop {
        match pc {
            0x832B4B98 => {
    //   block [0x832B4B98..0x832B4BA4)
	// 832B4B98: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B4B9C: 386B43B0  addi r3, r11, 0x43b0
	ctx.r[3].s64 = ctx.r[11].s64 + 17328;
	// 832B4BA0: 4AF60238  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4BA8 size=12
    let mut pc: u32 = 0x832B4BA8;
    'dispatch: loop {
        match pc {
            0x832B4BA8 => {
    //   block [0x832B4BA8..0x832B4BB4)
	// 832B4BA8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B4BAC: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 832B4BB0: 4AF60228  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4BB8 size=12
    let mut pc: u32 = 0x832B4BB8;
    'dispatch: loop {
        match pc {
            0x832B4BB8 => {
    //   block [0x832B4BB8..0x832B4BC4)
	// 832B4BB8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B4BBC: 386B43B8  addi r3, r11, 0x43b8
	ctx.r[3].s64 = ctx.r[11].s64 + 17336;
	// 832B4BC0: 4AF60218  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4BC8 size=12
    let mut pc: u32 = 0x832B4BC8;
    'dispatch: loop {
        match pc {
            0x832B4BC8 => {
    //   block [0x832B4BC8..0x832B4BD4)
	// 832B4BC8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B4BCC: 386B43BC  addi r3, r11, 0x43bc
	ctx.r[3].s64 = ctx.r[11].s64 + 17340;
	// 832B4BD0: 4AF60208  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4BD8 size=12
    let mut pc: u32 = 0x832B4BD8;
    'dispatch: loop {
        match pc {
            0x832B4BD8 => {
    //   block [0x832B4BD8..0x832B4BE4)
	// 832B4BD8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B4BDC: 386B43C0  addi r3, r11, 0x43c0
	ctx.r[3].s64 = ctx.r[11].s64 + 17344;
	// 832B4BE0: 4AF601F8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4BE8 size=12
    let mut pc: u32 = 0x832B4BE8;
    'dispatch: loop {
        match pc {
            0x832B4BE8 => {
    //   block [0x832B4BE8..0x832B4BF4)
	// 832B4BE8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B4BEC: 386B43C4  addi r3, r11, 0x43c4
	ctx.r[3].s64 = ctx.r[11].s64 + 17348;
	// 832B4BF0: 4AF601E8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4BF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4BF8 size=12
    let mut pc: u32 = 0x832B4BF8;
    'dispatch: loop {
        match pc {
            0x832B4BF8 => {
    //   block [0x832B4BF8..0x832B4C04)
	// 832B4BF8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B4BFC: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 832B4C00: 4AF601D8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4C08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4C08 size=12
    let mut pc: u32 = 0x832B4C08;
    'dispatch: loop {
        match pc {
            0x832B4C08 => {
    //   block [0x832B4C08..0x832B4C14)
	// 832B4C08: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B4C0C: 386B43CC  addi r3, r11, 0x43cc
	ctx.r[3].s64 = ctx.r[11].s64 + 17356;
	// 832B4C10: 4AF601C8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4C18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4C18 size=12
    let mut pc: u32 = 0x832B4C18;
    'dispatch: loop {
        match pc {
            0x832B4C18 => {
    //   block [0x832B4C18..0x832B4C24)
	// 832B4C18: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B4C1C: 386B43D0  addi r3, r11, 0x43d0
	ctx.r[3].s64 = ctx.r[11].s64 + 17360;
	// 832B4C20: 4AF601B8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4C28 size=12
    let mut pc: u32 = 0x832B4C28;
    'dispatch: loop {
        match pc {
            0x832B4C28 => {
    //   block [0x832B4C28..0x832B4C34)
	// 832B4C28: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B4C2C: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 832B4C30: 4AF601A8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4C38 size=20
    let mut pc: u32 = 0x832B4C38;
    'dispatch: loop {
        match pc {
            0x832B4C38 => {
    //   block [0x832B4C38..0x832B4C4C)
	// 832B4C38: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B4C3C: 806B43D8  lwz r3, 0x43d8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(17368 as u32) ) } as u64;
	// 832B4C40: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B4C44: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 832B4C48: 4AF476D8  b 0x821fc320
	sub_821FC320(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4C4C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4C4C size=4
    let mut pc: u32 = 0x832B4C4C;
    'dispatch: loop {
        match pc {
            0x832B4C4C => {
    //   block [0x832B4C4C..0x832B4C50)
	// 832B4C4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4C50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4C50 size=12
    let mut pc: u32 = 0x832B4C50;
    'dispatch: loop {
        match pc {
            0x832B4C50 => {
    //   block [0x832B4C50..0x832B4C5C)
	// 832B4C50: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B4C54: 386B43EC  addi r3, r11, 0x43ec
	ctx.r[3].s64 = ctx.r[11].s64 + 17388;
	// 832B4C58: 4AF60180  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4C60 size=12
    let mut pc: u32 = 0x832B4C60;
    'dispatch: loop {
        match pc {
            0x832B4C60 => {
    //   block [0x832B4C60..0x832B4C6C)
	// 832B4C60: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B4C64: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 832B4C68: 4AF60170  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B4C70 size=176
    let mut pc: u32 = 0x832B4C70;
    'dispatch: loop {
        match pc {
            0x832B4C70 => {
    //   block [0x832B4C70..0x832B4CB0)
	// 832B4C70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B4C74: 4B9F4785  bl 0x82ca93f8
	ctx.lr = 0x832B4C78;
	sub_82CA93D0(ctx, base);
	// 832B4C78: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B4C7C: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B4C80: 3D00820F  lis r8, -0x7df1
	ctx.r[8].s64 = -2112946176;
	// 832B4C84: 396B4440  addi r11, r11, 0x4440
	ctx.r[11].s64 = ctx.r[11].s64 + 17472;
	// 832B4C88: 3D20820F  lis r9, -0x7df1
	ctx.r[9].s64 = -2112946176;
	// 832B4C8C: 3BCB08A0  addi r30, r11, 0x8a0
	ctx.r[30].s64 = ctx.r[11].s64 + 2208;
	// 832B4C90: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 832B4C94: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 832B4C98: 3B800016  li r28, 0x16
	ctx.r[28].s64 = 22;
	// 832B4C9C: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 832B4CA0: 3B682A40  addi r27, r8, 0x2a40
	ctx.r[27].s64 = ctx.r[8].s64 + 10816;
	// 832B4CA4: 3B492A30  addi r26, r9, 0x2a30
	ctx.r[26].s64 = ctx.r[9].s64 + 10800;
	// 832B4CA8: 3B2A2A2C  addi r25, r10, 0x2a2c
	ctx.r[25].s64 = ctx.r[10].s64 + 10796;
	// 832B4CAC: 3BABB00C  addi r29, r11, -0x4ff4
	ctx.r[29].s64 = ctx.r[11].s64 + -20468;
	pc = 0x832B4CB0; continue 'dispatch;
            }
            0x832B4CB0 => {
    //   block [0x832B4CB0..0x832B4CC4)
	// 832B4CB0: 3BDEFFA0  addi r30, r30, -0x60
	ctx.r[30].s64 = ctx.r[30].s64 + -96;
	// 832B4CB4: 37FE0010  addic. r31, r30, 0x10
	ctx.xer.ca = (ctx.r[30].u32 > (!(16 as u32)));
	ctx.r[31].s64 = ctx.r[30].s64 + 16;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 832B4CB8: 397F000C  addi r11, r31, 0xc
	ctx.r[11].s64 = ctx.r[31].s64 + 12;
	// 832B4CBC: 40820008  bne 0x832b4cc4
	if !ctx.cr[0].eq {
	pc = 0x832B4CC4; continue 'dispatch;
	}
	// 832B4CC0: 7F0BC378  mr r11, r24
	ctx.r[11].u64 = ctx.r[24].u64;
	pc = 0x832B4CC4; continue 'dispatch;
            }
            0x832B4CC4 => {
    //   block [0x832B4CC4..0x832B4CE4)
	// 832B4CC4: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 832B4CC8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B4CCC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B4CD0: 933F0000  stw r25, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 832B4CD4: 419A0010  beq cr6, 0x832b4ce4
	if ctx.cr[6].eq {
	pc = 0x832B4CE4; continue 'dispatch;
	}
	// 832B4CD8: 4AF47649  bl 0x821fc320
	ctx.lr = 0x832B4CDC;
	sub_821FC320(ctx, base);
	// 832B4CDC: 931F0004  stw r24, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[24].u32 ) };
	// 832B4CE0: 931F0008  stw r24, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[24].u32 ) };
	pc = 0x832B4CE4; continue 'dispatch;
            }
            0x832B4CE4 => {
    //   block [0x832B4CE4..0x832B4CF4)
	// 832B4CE4: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 832B4CE8: 397E0008  addi r11, r30, 8
	ctx.r[11].s64 = ctx.r[30].s64 + 8;
	// 832B4CEC: 409A0008  bne cr6, 0x832b4cf4
	if !ctx.cr[6].eq {
	pc = 0x832B4CF4; continue 'dispatch;
	}
	// 832B4CF0: 7F0BC378  mr r11, r24
	ctx.r[11].u64 = ctx.r[24].u64;
	pc = 0x832B4CF4; continue 'dispatch;
            }
            0x832B4CF4 => {
    //   block [0x832B4CF4..0x832B4D20)
	// 832B4CF4: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 832B4CF8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832B4CFC: 935E0000  stw r26, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[26].u32 ) };
	// 832B4D00: 4AF47309  bl 0x821fc008
	ctx.lr = 0x832B4D04;
	sub_821FC008(ctx, base);
	// 832B4D04: 937E0000  stw r27, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 832B4D08: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832B4D0C: 4AF472FD  bl 0x821fc008
	ctx.lr = 0x832B4D10;
	sub_821FC008(ctx, base);
	// 832B4D10: 379CFFFF  addic. r28, r28, -1
	ctx.xer.ca = (ctx.r[28].u32 > (!(-1 as u32)));
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 832B4D14: 4080FF9C  bge 0x832b4cb0
	if !ctx.cr[0].lt {
	pc = 0x832B4CB0; continue 'dispatch;
	}
	// 832B4D18: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 832B4D1C: 4B9F472C  b 0x82ca9448
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B4D30 size=116
    let mut pc: u32 = 0x832B4D30;
    'dispatch: loop {
        match pc {
            0x832B4D30 => {
    //   block [0x832B4D30..0x832B4D54)
	// 832B4D30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B4D34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B4D38: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B4D3C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B4D40: 3FE0834A  lis r31, -0x7cb6
	ctx.r[31].s64 = -2092302336;
	// 832B4D44: 807F4CE0  lwz r3, 0x4ce0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(19680 as u32) ) } as u64;
	// 832B4D48: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B4D4C: 419A0044  beq cr6, 0x832b4d90
	if ctx.cr[6].eq {
	pc = 0x832B4D90; continue 'dispatch;
	}
	// 832B4D50: 39230004  addi r9, r3, 4
	ctx.r[9].s64 = ctx.r[3].s64 + 4;
	pc = 0x832B4D54; continue 'dispatch;
            }
            0x832B4D54 => {
    //   block [0x832B4D54..0x832B4D88)
	// 832B4D54: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832B4D58: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832B4D5C: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832B4D60: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832B4D64: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832B4D68: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832B4D6C: 4082FFE8  bne 0x832b4d54
	if !ctx.cr[0].eq {
	pc = 0x832B4D54; continue 'dispatch;
	}
	// 832B4D70: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 832B4D74: 409A0014  bne cr6, 0x832b4d88
	if !ctx.cr[6].eq {
	pc = 0x832B4D88; continue 'dispatch;
	}
	// 832B4D78: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B4D7C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B4D80: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 832B4D84: 4E800421  bctrl
	ctx.lr = 0x832B4D88;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x832B4D88 => {
    //   block [0x832B4D88..0x832B4D90)
	// 832B4D88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B4D8C: 917F4CE0  stw r11, 0x4ce0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(19680 as u32), ctx.r[11].u32 ) };
	pc = 0x832B4D90; continue 'dispatch;
            }
            0x832B4D90 => {
    //   block [0x832B4D90..0x832B4DA4)
	// 832B4D90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B4D94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B4D98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B4D9C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B4DA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B4DD8 size=92
    let mut pc: u32 = 0x832B4DD8;
    'dispatch: loop {
        match pc {
            0x832B4DD8 => {
    //   block [0x832B4DD8..0x832B4E34)
	// 832B4DD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B4DDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B4DE0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B4DE4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B4DE8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B4DEC: 3FE08332  lis r31, -0x7cce
	ctx.r[31].s64 = -2093875200;
	// 832B4DF0: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832B4DF4: 3BDFAC08  addi r30, r31, -0x53f8
	ctx.r[30].s64 = ctx.r[31].s64 + -21496;
	// 832B4DF8: 396B2A30  addi r11, r11, 0x2a30
	ctx.r[11].s64 = ctx.r[11].s64 + 10800;
	// 832B4DFC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832B4E00: 917FAC08  stw r11, -0x53f8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-21496 as u32), ctx.r[11].u32 ) };
	// 832B4E04: 4AF47205  bl 0x821fc008
	ctx.lr = 0x832B4E08;
	sub_821FC008(ctx, base);
	// 832B4E08: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832B4E0C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832B4E10: 396B2A40  addi r11, r11, 0x2a40
	ctx.r[11].s64 = ctx.r[11].s64 + 10816;
	// 832B4E14: 917FAC08  stw r11, -0x53f8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-21496 as u32), ctx.r[11].u32 ) };
	// 832B4E18: 4AF471F1  bl 0x821fc008
	ctx.lr = 0x832B4E1C;
	sub_821FC008(ctx, base);
	// 832B4E1C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B4E20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B4E24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B4E28: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B4E2C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B4E30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B4E38 size=92
    let mut pc: u32 = 0x832B4E38;
    'dispatch: loop {
        match pc {
            0x832B4E38 => {
    //   block [0x832B4E38..0x832B4E94)
	// 832B4E38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B4E3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B4E40: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B4E44: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B4E48: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B4E4C: 3FE08332  lis r31, -0x7cce
	ctx.r[31].s64 = -2093875200;
	// 832B4E50: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832B4E54: 3BDFAC10  addi r30, r31, -0x53f0
	ctx.r[30].s64 = ctx.r[31].s64 + -21488;
	// 832B4E58: 396B2A30  addi r11, r11, 0x2a30
	ctx.r[11].s64 = ctx.r[11].s64 + 10800;
	// 832B4E5C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832B4E60: 917FAC10  stw r11, -0x53f0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-21488 as u32), ctx.r[11].u32 ) };
	// 832B4E64: 4AF471A5  bl 0x821fc008
	ctx.lr = 0x832B4E68;
	sub_821FC008(ctx, base);
	// 832B4E68: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832B4E6C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832B4E70: 396B2A40  addi r11, r11, 0x2a40
	ctx.r[11].s64 = ctx.r[11].s64 + 10816;
	// 832B4E74: 917FAC10  stw r11, -0x53f0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-21488 as u32), ctx.r[11].u32 ) };
	// 832B4E78: 4AF47191  bl 0x821fc008
	ctx.lr = 0x832B4E7C;
	sub_821FC008(ctx, base);
	// 832B4E7C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B4E80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B4E84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B4E88: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B4E8C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B4E90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4E98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4E98 size=12
    let mut pc: u32 = 0x832B4E98;
    'dispatch: loop {
        match pc {
            0x832B4E98 => {
    //   block [0x832B4E98..0x832B4EA4)
	// 832B4E98: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B4E9C: 386B4D60  addi r3, r11, 0x4d60
	ctx.r[3].s64 = ctx.r[11].s64 + 19808;
	// 832B4EA0: 4B7D0EF8  b 0x82a85d98
	sub_82A85D98(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4EA8 size=12
    let mut pc: u32 = 0x832B4EA8;
    'dispatch: loop {
        match pc {
            0x832B4EA8 => {
    //   block [0x832B4EA8..0x832B4EB4)
	// 832B4EA8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B4EAC: 386B4DAC  addi r3, r11, 0x4dac
	ctx.r[3].s64 = ctx.r[11].s64 + 19884;
	// 832B4EB0: 4AF5FF28  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4EB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4EB8 size=12
    let mut pc: u32 = 0x832B4EB8;
    'dispatch: loop {
        match pc {
            0x832B4EB8 => {
    //   block [0x832B4EB8..0x832B4EC4)
	// 832B4EB8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B4EBC: 386B4DB0  addi r3, r11, 0x4db0
	ctx.r[3].s64 = ctx.r[11].s64 + 19888;
	// 832B4EC0: 4AF5FF18  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4EC8 size=12
    let mut pc: u32 = 0x832B4EC8;
    'dispatch: loop {
        match pc {
            0x832B4EC8 => {
    //   block [0x832B4EC8..0x832B4ED4)
	// 832B4EC8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B4ECC: 386B4DB4  addi r3, r11, 0x4db4
	ctx.r[3].s64 = ctx.r[11].s64 + 19892;
	// 832B4ED0: 4AF5FF08  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4ED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4ED8 size=12
    let mut pc: u32 = 0x832B4ED8;
    'dispatch: loop {
        match pc {
            0x832B4ED8 => {
    //   block [0x832B4ED8..0x832B4EE4)
	// 832B4ED8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B4EDC: 386B4DB8  addi r3, r11, 0x4db8
	ctx.r[3].s64 = ctx.r[11].s64 + 19896;
	// 832B4EE0: 4AF5FEF8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4EE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4EE8 size=20
    let mut pc: u32 = 0x832B4EE8;
    'dispatch: loop {
        match pc {
            0x832B4EE8 => {
    //   block [0x832B4EE8..0x832B4EFC)
	// 832B4EE8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B4EEC: 806B4DBC  lwz r3, 0x4dbc(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(19900 as u32) ) } as u64;
	// 832B4EF0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B4EF4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 832B4EF8: 4AF47428  b 0x821fc320
	sub_821FC320(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4EFC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4EFC size=4
    let mut pc: u32 = 0x832B4EFC;
    'dispatch: loop {
        match pc {
            0x832B4EFC => {
    //   block [0x832B4EFC..0x832B4F00)
	// 832B4EFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4F00 size=20
    let mut pc: u32 = 0x832B4F00;
    'dispatch: loop {
        match pc {
            0x832B4F00 => {
    //   block [0x832B4F00..0x832B4F14)
	// 832B4F00: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B4F04: 806B4DD0  lwz r3, 0x4dd0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(19920 as u32) ) } as u64;
	// 832B4F08: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B4F0C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 832B4F10: 4AF47410  b 0x821fc320
	sub_821FC320(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4F14(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4F14 size=4
    let mut pc: u32 = 0x832B4F14;
    'dispatch: loop {
        match pc {
            0x832B4F14 => {
    //   block [0x832B4F14..0x832B4F18)
	// 832B4F14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B4F18 size=84
    let mut pc: u32 = 0x832B4F18;
    'dispatch: loop {
        match pc {
            0x832B4F18 => {
    //   block [0x832B4F18..0x832B4F40)
	// 832B4F18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B4F1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B4F20: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B4F24: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B4F28: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B4F2C: 3BEB4DE4  addi r31, r11, 0x4de4
	ctx.r[31].s64 = ctx.r[11].s64 + 19940;
	// 832B4F30: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B4F34: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B4F38: 419A0008  beq cr6, 0x832b4f40
	if ctx.cr[6].eq {
	pc = 0x832B4F40; continue 'dispatch;
	}
	// 832B4F3C: 4AF66DFD  bl 0x8221bd38
	ctx.lr = 0x832B4F40;
	sub_8221BD38(ctx, base);
	pc = 0x832B4F40; continue 'dispatch;
            }
            0x832B4F40 => {
    //   block [0x832B4F40..0x832B4F6C)
	// 832B4F40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B4F44: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B4F48: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B4F4C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B4F50: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B4F54: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832B4F58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B4F5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B4F60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B4F64: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B4F68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4F70 size=12
    let mut pc: u32 = 0x832B4F70;
    'dispatch: loop {
        match pc {
            0x832B4F70 => {
    //   block [0x832B4F70..0x832B4F7C)
	// 832B4F70: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B4F74: 386B4DF4  addi r3, r11, 0x4df4
	ctx.r[3].s64 = ctx.r[11].s64 + 19956;
	// 832B4F78: 4AF5FE60  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4F80 size=12
    let mut pc: u32 = 0x832B4F80;
    'dispatch: loop {
        match pc {
            0x832B4F80 => {
    //   block [0x832B4F80..0x832B4F8C)
	// 832B4F80: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B4F84: 386B4DF8  addi r3, r11, 0x4df8
	ctx.r[3].s64 = ctx.r[11].s64 + 19960;
	// 832B4F88: 4AF5FE50  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4F90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4F90 size=12
    let mut pc: u32 = 0x832B4F90;
    'dispatch: loop {
        match pc {
            0x832B4F90 => {
    //   block [0x832B4F90..0x832B4F9C)
	// 832B4F90: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B4F94: 386B4DFC  addi r3, r11, 0x4dfc
	ctx.r[3].s64 = ctx.r[11].s64 + 19964;
	// 832B4F98: 4AF5FE40  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4FA0 size=12
    let mut pc: u32 = 0x832B4FA0;
    'dispatch: loop {
        match pc {
            0x832B4FA0 => {
    //   block [0x832B4FA0..0x832B4FAC)
	// 832B4FA0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B4FA4: 386B4E00  addi r3, r11, 0x4e00
	ctx.r[3].s64 = ctx.r[11].s64 + 19968;
	// 832B4FA8: 4AF5FE30  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4FB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4FB0 size=12
    let mut pc: u32 = 0x832B4FB0;
    'dispatch: loop {
        match pc {
            0x832B4FB0 => {
    //   block [0x832B4FB0..0x832B4FBC)
	// 832B4FB0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B4FB4: 386B4E04  addi r3, r11, 0x4e04
	ctx.r[3].s64 = ctx.r[11].s64 + 19972;
	// 832B4FB8: 4AF5FE20  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4FC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B4FC0 size=12
    let mut pc: u32 = 0x832B4FC0;
    'dispatch: loop {
        match pc {
            0x832B4FC0 => {
    //   block [0x832B4FC0..0x832B4FCC)
	// 832B4FC0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B4FC4: 386B4E08  addi r3, r11, 0x4e08
	ctx.r[3].s64 = ctx.r[11].s64 + 19976;
	// 832B4FC8: 4AF5FE10  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B4FD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B4FD0 size=80
    let mut pc: u32 = 0x832B4FD0;
    'dispatch: loop {
        match pc {
            0x832B4FD0 => {
    //   block [0x832B4FD0..0x832B4FF4)
	// 832B4FD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B4FD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B4FD8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B4FDC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B4FE0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B4FE4: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B4FE8: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 832B4FEC: 396B4E0C  addi r11, r11, 0x4e0c
	ctx.r[11].s64 = ctx.r[11].s64 + 19980;
	// 832B4FF0: 3BEB0010  addi r31, r11, 0x10
	ctx.r[31].s64 = ctx.r[11].s64 + 16;
	pc = 0x832B4FF4; continue 'dispatch;
            }
            0x832B4FF4 => {
    //   block [0x832B4FF4..0x832B5020)
	// 832B4FF4: 3BFFFFF8  addi r31, r31, -8
	ctx.r[31].s64 = ctx.r[31].s64 + -8;
	// 832B4FF8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B4FFC: 4AF02B1D  bl 0x821b7b18
	ctx.lr = 0x832B5000;
	sub_821B7B18(ctx, base);
	// 832B5000: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832B5004: 4080FFF0  bge 0x832b4ff4
	if !ctx.cr[0].lt {
	pc = 0x832B4FF4; continue 'dispatch;
	}
	// 832B5008: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B500C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B5010: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B5014: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B5018: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B501C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B5020 size=80
    let mut pc: u32 = 0x832B5020;
    'dispatch: loop {
        match pc {
            0x832B5020 => {
    //   block [0x832B5020..0x832B5044)
	// 832B5020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B5024: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B5028: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B502C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B5030: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B5034: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5038: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 832B503C: 396B4E1C  addi r11, r11, 0x4e1c
	ctx.r[11].s64 = ctx.r[11].s64 + 19996;
	// 832B5040: 3BEB0010  addi r31, r11, 0x10
	ctx.r[31].s64 = ctx.r[11].s64 + 16;
	pc = 0x832B5044; continue 'dispatch;
            }
            0x832B5044 => {
    //   block [0x832B5044..0x832B5070)
	// 832B5044: 3BFFFFF8  addi r31, r31, -8
	ctx.r[31].s64 = ctx.r[31].s64 + -8;
	// 832B5048: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B504C: 4AF02ACD  bl 0x821b7b18
	ctx.lr = 0x832B5050;
	sub_821B7B18(ctx, base);
	// 832B5050: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832B5054: 4080FFF0  bge 0x832b5044
	if !ctx.cr[0].lt {
	pc = 0x832B5044; continue 'dispatch;
	}
	// 832B5058: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B505C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B5060: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B5064: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B5068: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B506C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B5070 size=80
    let mut pc: u32 = 0x832B5070;
    'dispatch: loop {
        match pc {
            0x832B5070 => {
    //   block [0x832B5070..0x832B5094)
	// 832B5070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B5074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B5078: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B507C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B5080: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B5084: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5088: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 832B508C: 396B4E2C  addi r11, r11, 0x4e2c
	ctx.r[11].s64 = ctx.r[11].s64 + 20012;
	// 832B5090: 3BEB0010  addi r31, r11, 0x10
	ctx.r[31].s64 = ctx.r[11].s64 + 16;
	pc = 0x832B5094; continue 'dispatch;
            }
            0x832B5094 => {
    //   block [0x832B5094..0x832B50C0)
	// 832B5094: 3BFFFFF8  addi r31, r31, -8
	ctx.r[31].s64 = ctx.r[31].s64 + -8;
	// 832B5098: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B509C: 4AF02A7D  bl 0x821b7b18
	ctx.lr = 0x832B50A0;
	sub_821B7B18(ctx, base);
	// 832B50A0: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832B50A4: 4080FFF0  bge 0x832b5094
	if !ctx.cr[0].lt {
	pc = 0x832B5094; continue 'dispatch;
	}
	// 832B50A8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B50AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B50B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B50B4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B50B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B50BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B50C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B50C0 size=80
    let mut pc: u32 = 0x832B50C0;
    'dispatch: loop {
        match pc {
            0x832B50C0 => {
    //   block [0x832B50C0..0x832B50E4)
	// 832B50C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B50C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B50C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B50CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B50D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B50D4: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B50D8: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 832B50DC: 396B4E3C  addi r11, r11, 0x4e3c
	ctx.r[11].s64 = ctx.r[11].s64 + 20028;
	// 832B50E0: 3BEB0010  addi r31, r11, 0x10
	ctx.r[31].s64 = ctx.r[11].s64 + 16;
	pc = 0x832B50E4; continue 'dispatch;
            }
            0x832B50E4 => {
    //   block [0x832B50E4..0x832B5110)
	// 832B50E4: 3BFFFFF8  addi r31, r31, -8
	ctx.r[31].s64 = ctx.r[31].s64 + -8;
	// 832B50E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B50EC: 4AF02A2D  bl 0x821b7b18
	ctx.lr = 0x832B50F0;
	sub_821B7B18(ctx, base);
	// 832B50F0: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832B50F4: 4080FFF0  bge 0x832b50e4
	if !ctx.cr[0].lt {
	pc = 0x832B50E4; continue 'dispatch;
	}
	// 832B50F8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B50FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B5100: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B5104: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B5108: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B510C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B5110 size=80
    let mut pc: u32 = 0x832B5110;
    'dispatch: loop {
        match pc {
            0x832B5110 => {
    //   block [0x832B5110..0x832B5134)
	// 832B5110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B5114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B5118: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B511C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B5120: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B5124: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5128: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 832B512C: 396B4E4C  addi r11, r11, 0x4e4c
	ctx.r[11].s64 = ctx.r[11].s64 + 20044;
	// 832B5130: 3BEB0010  addi r31, r11, 0x10
	ctx.r[31].s64 = ctx.r[11].s64 + 16;
	pc = 0x832B5134; continue 'dispatch;
            }
            0x832B5134 => {
    //   block [0x832B5134..0x832B5160)
	// 832B5134: 3BFFFFF8  addi r31, r31, -8
	ctx.r[31].s64 = ctx.r[31].s64 + -8;
	// 832B5138: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B513C: 4AF029DD  bl 0x821b7b18
	ctx.lr = 0x832B5140;
	sub_821B7B18(ctx, base);
	// 832B5140: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832B5144: 4080FFF0  bge 0x832b5134
	if !ctx.cr[0].lt {
	pc = 0x832B5134; continue 'dispatch;
	}
	// 832B5148: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B514C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B5150: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B5154: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B5158: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B515C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B5160 size=80
    let mut pc: u32 = 0x832B5160;
    'dispatch: loop {
        match pc {
            0x832B5160 => {
    //   block [0x832B5160..0x832B5184)
	// 832B5160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B5164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B5168: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B516C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B5170: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B5174: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5178: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 832B517C: 396B4E5C  addi r11, r11, 0x4e5c
	ctx.r[11].s64 = ctx.r[11].s64 + 20060;
	// 832B5180: 3BEB0010  addi r31, r11, 0x10
	ctx.r[31].s64 = ctx.r[11].s64 + 16;
	pc = 0x832B5184; continue 'dispatch;
            }
            0x832B5184 => {
    //   block [0x832B5184..0x832B51B0)
	// 832B5184: 3BFFFFF8  addi r31, r31, -8
	ctx.r[31].s64 = ctx.r[31].s64 + -8;
	// 832B5188: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B518C: 4AF0298D  bl 0x821b7b18
	ctx.lr = 0x832B5190;
	sub_821B7B18(ctx, base);
	// 832B5190: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832B5194: 4080FFF0  bge 0x832b5184
	if !ctx.cr[0].lt {
	pc = 0x832B5184; continue 'dispatch;
	}
	// 832B5198: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B519C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B51A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B51A4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B51A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B51AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B51B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B51B0 size=92
    let mut pc: u32 = 0x832B51B0;
    'dispatch: loop {
        match pc {
            0x832B51B0 => {
    //   block [0x832B51B0..0x832B520C)
	// 832B51B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B51B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B51B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B51BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B51C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B51C4: 3FE08332  lis r31, -0x7cce
	ctx.r[31].s64 = -2093875200;
	// 832B51C8: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832B51CC: 3BDFAC18  addi r30, r31, -0x53e8
	ctx.r[30].s64 = ctx.r[31].s64 + -21480;
	// 832B51D0: 396B2A30  addi r11, r11, 0x2a30
	ctx.r[11].s64 = ctx.r[11].s64 + 10800;
	// 832B51D4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832B51D8: 917FAC18  stw r11, -0x53e8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-21480 as u32), ctx.r[11].u32 ) };
	// 832B51DC: 4AF46E2D  bl 0x821fc008
	ctx.lr = 0x832B51E0;
	sub_821FC008(ctx, base);
	// 832B51E0: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832B51E4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832B51E8: 396B2A40  addi r11, r11, 0x2a40
	ctx.r[11].s64 = ctx.r[11].s64 + 10816;
	// 832B51EC: 917FAC18  stw r11, -0x53e8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-21480 as u32), ctx.r[11].u32 ) };
	// 832B51F0: 4AF46E19  bl 0x821fc008
	ctx.lr = 0x832B51F4;
	sub_821FC008(ctx, base);
	// 832B51F4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B51F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B51FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B5200: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B5204: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B5208: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5210 size=12
    let mut pc: u32 = 0x832B5210;
    'dispatch: loop {
        match pc {
            0x832B5210 => {
    //   block [0x832B5210..0x832B521C)
	// 832B5210: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5214: 386B4E6C  addi r3, r11, 0x4e6c
	ctx.r[3].s64 = ctx.r[11].s64 + 20076;
	// 832B5218: 4B7D3CA8  b 0x82a88ec0
	sub_82A88EC0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5220 size=12
    let mut pc: u32 = 0x832B5220;
    'dispatch: loop {
        match pc {
            0x832B5220 => {
    //   block [0x832B5220..0x832B522C)
	// 832B5220: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832B5224: 386B3528  addi r3, r11, 0x3528
	ctx.r[3].s64 = ctx.r[11].s64 + 13608;
	// 832B5228: 4AF5FBB0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5230 size=12
    let mut pc: u32 = 0x832B5230;
    'dispatch: loop {
        match pc {
            0x832B5230 => {
    //   block [0x832B5230..0x832B523C)
	// 832B5230: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5234: 386B4E78  addi r3, r11, 0x4e78
	ctx.r[3].s64 = ctx.r[11].s64 + 20088;
	// 832B5238: 4AF5FBA0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5240 size=12
    let mut pc: u32 = 0x832B5240;
    'dispatch: loop {
        match pc {
            0x832B5240 => {
    //   block [0x832B5240..0x832B524C)
	// 832B5240: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5244: 386B4E7C  addi r3, r11, 0x4e7c
	ctx.r[3].s64 = ctx.r[11].s64 + 20092;
	// 832B5248: 4AF5FB90  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5250 size=12
    let mut pc: u32 = 0x832B5250;
    'dispatch: loop {
        match pc {
            0x832B5250 => {
    //   block [0x832B5250..0x832B525C)
	// 832B5250: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5254: 386B4E80  addi r3, r11, 0x4e80
	ctx.r[3].s64 = ctx.r[11].s64 + 20096;
	// 832B5258: 4B7D4D80  b 0x82a89fd8
	sub_82A89FD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5260 size=12
    let mut pc: u32 = 0x832B5260;
    'dispatch: loop {
        match pc {
            0x832B5260 => {
    //   block [0x832B5260..0x832B526C)
	// 832B5260: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5264: 386B4E8C  addi r3, r11, 0x4e8c
	ctx.r[3].s64 = ctx.r[11].s64 + 20108;
	// 832B5268: 4AF5FB70  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5270 size=12
    let mut pc: u32 = 0x832B5270;
    'dispatch: loop {
        match pc {
            0x832B5270 => {
    //   block [0x832B5270..0x832B527C)
	// 832B5270: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5274: 386B4E90  addi r3, r11, 0x4e90
	ctx.r[3].s64 = ctx.r[11].s64 + 20112;
	// 832B5278: 4AF5FB60  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B5280 size=84
    let mut pc: u32 = 0x832B5280;
    'dispatch: loop {
        match pc {
            0x832B5280 => {
    //   block [0x832B5280..0x832B52A8)
	// 832B5280: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B5284: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B5288: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B528C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B5290: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5294: 3BEB4E94  addi r31, r11, 0x4e94
	ctx.r[31].s64 = ctx.r[11].s64 + 20116;
	// 832B5298: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B529C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B52A0: 419A0008  beq cr6, 0x832b52a8
	if ctx.cr[6].eq {
	pc = 0x832B52A8; continue 'dispatch;
	}
	// 832B52A4: 4AF66A95  bl 0x8221bd38
	ctx.lr = 0x832B52A8;
	sub_8221BD38(ctx, base);
	pc = 0x832B52A8; continue 'dispatch;
            }
            0x832B52A8 => {
    //   block [0x832B52A8..0x832B52D4)
	// 832B52A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B52AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B52B0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B52B4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B52B8: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B52BC: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832B52C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B52C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B52C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B52CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B52D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B52F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B52F8 size=88
    let mut pc: u32 = 0x832B52F8;
    'dispatch: loop {
        match pc {
            0x832B52F8 => {
    //   block [0x832B52F8..0x832B531C)
	// 832B52F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B52FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B5300: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B5304: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B5308: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B530C: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5310: 3BC00003  li r30, 3
	ctx.r[30].s64 = 3;
	// 832B5314: 396B4EC0  addi r11, r11, 0x4ec0
	ctx.r[11].s64 = ctx.r[11].s64 + 20160;
	// 832B5318: 3BEB0050  addi r31, r11, 0x50
	ctx.r[31].s64 = ctx.r[11].s64 + 80;
	pc = 0x832B531C; continue 'dispatch;
            }
            0x832B531C => {
    //   block [0x832B531C..0x832B5330)
	// 832B531C: 3BFFFFEC  addi r31, r31, -0x14
	ctx.r[31].s64 = ctx.r[31].s64 + -20;
	// 832B5320: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B5324: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B5328: 419A0008  beq cr6, 0x832b5330
	if ctx.cr[6].eq {
	pc = 0x832B5330; continue 'dispatch;
	}
	// 832B532C: 4AF46FF5  bl 0x821fc320
	ctx.lr = 0x832B5330;
	sub_821FC320(ctx, base);
	pc = 0x832B5330; continue 'dispatch;
            }
            0x832B5330 => {
    //   block [0x832B5330..0x832B5350)
	// 832B5330: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832B5334: 4080FFE8  bge 0x832b531c
	if !ctx.cr[0].lt {
	pc = 0x832B531C; continue 'dispatch;
	}
	// 832B5338: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B533C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B5340: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B5344: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B5348: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B534C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5350 size=20
    let mut pc: u32 = 0x832B5350;
    'dispatch: loop {
        match pc {
            0x832B5350 => {
    //   block [0x832B5350..0x832B5364)
	// 832B5350: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5354: 806B4EAC  lwz r3, 0x4eac(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20140 as u32) ) } as u64;
	// 832B5358: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B535C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 832B5360: 4AF46FC0  b 0x821fc320
	sub_821FC320(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5364(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5364 size=4
    let mut pc: u32 = 0x832B5364;
    'dispatch: loop {
        match pc {
            0x832B5364 => {
    //   block [0x832B5364..0x832B5368)
	// 832B5364: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5368 size=20
    let mut pc: u32 = 0x832B5368;
    'dispatch: loop {
        match pc {
            0x832B5368 => {
    //   block [0x832B5368..0x832B537C)
	// 832B5368: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B536C: 806B4F10  lwz r3, 0x4f10(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20240 as u32) ) } as u64;
	// 832B5370: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B5374: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 832B5378: 4AF46FA8  b 0x821fc320
	sub_821FC320(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B537C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B537C size=4
    let mut pc: u32 = 0x832B537C;
    'dispatch: loop {
        match pc {
            0x832B537C => {
    //   block [0x832B537C..0x832B5380)
	// 832B537C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5380 size=12
    let mut pc: u32 = 0x832B5380;
    'dispatch: loop {
        match pc {
            0x832B5380 => {
    //   block [0x832B5380..0x832B538C)
	// 832B5380: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5384: 386B4F24  addi r3, r11, 0x4f24
	ctx.r[3].s64 = ctx.r[11].s64 + 20260;
	// 832B5388: 4AF5FA50  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5390 size=12
    let mut pc: u32 = 0x832B5390;
    'dispatch: loop {
        match pc {
            0x832B5390 => {
    //   block [0x832B5390..0x832B539C)
	// 832B5390: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5394: 386B4F28  addi r3, r11, 0x4f28
	ctx.r[3].s64 = ctx.r[11].s64 + 20264;
	// 832B5398: 4AF5FA40  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B53A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B53A0 size=88
    let mut pc: u32 = 0x832B53A0;
    'dispatch: loop {
        match pc {
            0x832B53A0 => {
    //   block [0x832B53A0..0x832B53C4)
	// 832B53A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B53A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B53A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B53AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B53B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B53B4: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B53B8: 3BC00003  li r30, 3
	ctx.r[30].s64 = 3;
	// 832B53BC: 396B4F30  addi r11, r11, 0x4f30
	ctx.r[11].s64 = ctx.r[11].s64 + 20272;
	// 832B53C0: 3BEB0050  addi r31, r11, 0x50
	ctx.r[31].s64 = ctx.r[11].s64 + 80;
	pc = 0x832B53C4; continue 'dispatch;
            }
            0x832B53C4 => {
    //   block [0x832B53C4..0x832B53D8)
	// 832B53C4: 3BFFFFEC  addi r31, r31, -0x14
	ctx.r[31].s64 = ctx.r[31].s64 + -20;
	// 832B53C8: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B53CC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B53D0: 419A0008  beq cr6, 0x832b53d8
	if ctx.cr[6].eq {
	pc = 0x832B53D8; continue 'dispatch;
	}
	// 832B53D4: 4AF46F4D  bl 0x821fc320
	ctx.lr = 0x832B53D8;
	sub_821FC320(ctx, base);
	pc = 0x832B53D8; continue 'dispatch;
            }
            0x832B53D8 => {
    //   block [0x832B53D8..0x832B53F8)
	// 832B53D8: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832B53DC: 4080FFE8  bge 0x832b53c4
	if !ctx.cr[0].lt {
	pc = 0x832B53C4; continue 'dispatch;
	}
	// 832B53E0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B53E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B53E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B53EC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B53F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B53F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B53F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B53F8 size=116
    let mut pc: u32 = 0x832B53F8;
    'dispatch: loop {
        match pc {
            0x832B53F8 => {
    //   block [0x832B53F8..0x832B541C)
	// 832B53F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B53FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B5400: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B5404: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B5408: 3FE0834A  lis r31, -0x7cb6
	ctx.r[31].s64 = -2092302336;
	// 832B540C: 807F4F80  lwz r3, 0x4f80(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20352 as u32) ) } as u64;
	// 832B5410: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B5414: 419A0044  beq cr6, 0x832b5458
	if ctx.cr[6].eq {
	pc = 0x832B5458; continue 'dispatch;
	}
	// 832B5418: 39230004  addi r9, r3, 4
	ctx.r[9].s64 = ctx.r[3].s64 + 4;
	pc = 0x832B541C; continue 'dispatch;
            }
            0x832B541C => {
    //   block [0x832B541C..0x832B5450)
	// 832B541C: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832B5420: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832B5424: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832B5428: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832B542C: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832B5430: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832B5434: 4082FFE8  bne 0x832b541c
	if !ctx.cr[0].eq {
	pc = 0x832B541C; continue 'dispatch;
	}
	// 832B5438: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 832B543C: 409A0014  bne cr6, 0x832b5450
	if !ctx.cr[6].eq {
	pc = 0x832B5450; continue 'dispatch;
	}
	// 832B5440: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B5444: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B5448: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 832B544C: 4E800421  bctrl
	ctx.lr = 0x832B5450;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x832B5450 => {
    //   block [0x832B5450..0x832B5458)
	// 832B5450: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B5454: 917F4F80  stw r11, 0x4f80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20352 as u32), ctx.r[11].u32 ) };
	pc = 0x832B5458; continue 'dispatch;
            }
            0x832B5458 => {
    //   block [0x832B5458..0x832B546C)
	// 832B5458: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B545C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B5460: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B5464: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B5468: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5470 size=12
    let mut pc: u32 = 0x832B5470;
    'dispatch: loop {
        match pc {
            0x832B5470 => {
    //   block [0x832B5470..0x832B547C)
	// 832B5470: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5474: 386B4F84  addi r3, r11, 0x4f84
	ctx.r[3].s64 = ctx.r[11].s64 + 20356;
	// 832B5478: 4AF5F960  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5480 size=12
    let mut pc: u32 = 0x832B5480;
    'dispatch: loop {
        match pc {
            0x832B5480 => {
    //   block [0x832B5480..0x832B548C)
	// 832B5480: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5484: 386B4F88  addi r3, r11, 0x4f88
	ctx.r[3].s64 = ctx.r[11].s64 + 20360;
	// 832B5488: 4AF5F950  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5490 size=20
    let mut pc: u32 = 0x832B5490;
    'dispatch: loop {
        match pc {
            0x832B5490 => {
    //   block [0x832B5490..0x832B54A4)
	// 832B5490: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5494: 806B4F8C  lwz r3, 0x4f8c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20364 as u32) ) } as u64;
	// 832B5498: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B549C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 832B54A0: 4AF46E80  b 0x821fc320
	sub_821FC320(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B54A4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B54A4 size=4
    let mut pc: u32 = 0x832B54A4;
    'dispatch: loop {
        match pc {
            0x832B54A4 => {
    //   block [0x832B54A4..0x832B54A8)
	// 832B54A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B54A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B54A8 size=20
    let mut pc: u32 = 0x832B54A8;
    'dispatch: loop {
        match pc {
            0x832B54A8 => {
    //   block [0x832B54A8..0x832B54BC)
	// 832B54A8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B54AC: 806B4FA0  lwz r3, 0x4fa0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20384 as u32) ) } as u64;
	// 832B54B0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B54B4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 832B54B8: 4AF46E68  b 0x821fc320
	sub_821FC320(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B54BC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B54BC size=4
    let mut pc: u32 = 0x832B54BC;
    'dispatch: loop {
        match pc {
            0x832B54BC => {
    //   block [0x832B54BC..0x832B54C0)
	// 832B54BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B54C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B54C0 size=20
    let mut pc: u32 = 0x832B54C0;
    'dispatch: loop {
        match pc {
            0x832B54C0 => {
    //   block [0x832B54C0..0x832B54D4)
	// 832B54C0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B54C4: 806B4FB4  lwz r3, 0x4fb4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20404 as u32) ) } as u64;
	// 832B54C8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B54CC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 832B54D0: 4AF46E50  b 0x821fc320
	sub_821FC320(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B54D4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B54D4 size=4
    let mut pc: u32 = 0x832B54D4;
    'dispatch: loop {
        match pc {
            0x832B54D4 => {
    //   block [0x832B54D4..0x832B54D8)
	// 832B54D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B54D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B54D8 size=20
    let mut pc: u32 = 0x832B54D8;
    'dispatch: loop {
        match pc {
            0x832B54D8 => {
    //   block [0x832B54D8..0x832B54EC)
	// 832B54D8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B54DC: 806B4FC8  lwz r3, 0x4fc8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20424 as u32) ) } as u64;
	// 832B54E0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B54E4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 832B54E8: 4AF46E38  b 0x821fc320
	sub_821FC320(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B54EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B54EC size=4
    let mut pc: u32 = 0x832B54EC;
    'dispatch: loop {
        match pc {
            0x832B54EC => {
    //   block [0x832B54EC..0x832B54F0)
	// 832B54EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B54F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B54F0 size=20
    let mut pc: u32 = 0x832B54F0;
    'dispatch: loop {
        match pc {
            0x832B54F0 => {
    //   block [0x832B54F0..0x832B5504)
	// 832B54F0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B54F4: 806B4FDC  lwz r3, 0x4fdc(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20444 as u32) ) } as u64;
	// 832B54F8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B54FC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 832B5500: 4AF46E20  b 0x821fc320
	sub_821FC320(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5504(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5504 size=4
    let mut pc: u32 = 0x832B5504;
    'dispatch: loop {
        match pc {
            0x832B5504 => {
    //   block [0x832B5504..0x832B5508)
	// 832B5504: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B5508 size=84
    let mut pc: u32 = 0x832B5508;
    'dispatch: loop {
        match pc {
            0x832B5508 => {
    //   block [0x832B5508..0x832B5530)
	// 832B5508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B550C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B5510: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B5514: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B5518: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832B551C: 3BEB1A50  addi r31, r11, 0x1a50
	ctx.r[31].s64 = ctx.r[11].s64 + 6736;
	// 832B5520: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B5524: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B5528: 419A0008  beq cr6, 0x832b5530
	if ctx.cr[6].eq {
	pc = 0x832B5530; continue 'dispatch;
	}
	// 832B552C: 4AF6680D  bl 0x8221bd38
	ctx.lr = 0x832B5530;
	sub_8221BD38(ctx, base);
	pc = 0x832B5530; continue 'dispatch;
            }
            0x832B5530 => {
    //   block [0x832B5530..0x832B555C)
	// 832B5530: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B5534: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B5538: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B553C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B5540: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B5544: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832B5548: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B554C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B5550: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B5554: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B5558: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B5580 size=84
    let mut pc: u32 = 0x832B5580;
    'dispatch: loop {
        match pc {
            0x832B5580 => {
    //   block [0x832B5580..0x832B55A8)
	// 832B5580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B5584: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B5588: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B558C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B5590: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5594: 3BEB4FF8  addi r31, r11, 0x4ff8
	ctx.r[31].s64 = ctx.r[11].s64 + 20472;
	// 832B5598: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B559C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B55A0: 419A0008  beq cr6, 0x832b55a8
	if ctx.cr[6].eq {
	pc = 0x832B55A8; continue 'dispatch;
	}
	// 832B55A4: 4AF66795  bl 0x8221bd38
	ctx.lr = 0x832B55A8;
	sub_8221BD38(ctx, base);
	pc = 0x832B55A8; continue 'dispatch;
            }
            0x832B55A8 => {
    //   block [0x832B55A8..0x832B55D4)
	// 832B55A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B55AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B55B0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B55B4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B55B8: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B55BC: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832B55C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B55C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B55C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B55CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B55D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B55D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B55D8 size=84
    let mut pc: u32 = 0x832B55D8;
    'dispatch: loop {
        match pc {
            0x832B55D8 => {
    //   block [0x832B55D8..0x832B5600)
	// 832B55D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B55DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B55E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B55E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B55E8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B55EC: 3BEB5008  addi r31, r11, 0x5008
	ctx.r[31].s64 = ctx.r[11].s64 + 20488;
	// 832B55F0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B55F4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B55F8: 419A0008  beq cr6, 0x832b5600
	if ctx.cr[6].eq {
	pc = 0x832B5600; continue 'dispatch;
	}
	// 832B55FC: 4AF6673D  bl 0x8221bd38
	ctx.lr = 0x832B5600;
	sub_8221BD38(ctx, base);
	pc = 0x832B5600; continue 'dispatch;
            }
            0x832B5600 => {
    //   block [0x832B5600..0x832B562C)
	// 832B5600: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B5604: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B5608: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B560C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B5610: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B5614: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832B5618: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B561C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B5620: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B5624: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B5628: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B5630 size=84
    let mut pc: u32 = 0x832B5630;
    'dispatch: loop {
        match pc {
            0x832B5630 => {
    //   block [0x832B5630..0x832B5658)
	// 832B5630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B5634: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B5638: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B563C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B5640: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5644: 3BEB5018  addi r31, r11, 0x5018
	ctx.r[31].s64 = ctx.r[11].s64 + 20504;
	// 832B5648: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B564C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B5650: 419A0008  beq cr6, 0x832b5658
	if ctx.cr[6].eq {
	pc = 0x832B5658; continue 'dispatch;
	}
	// 832B5654: 4AF666E5  bl 0x8221bd38
	ctx.lr = 0x832B5658;
	sub_8221BD38(ctx, base);
	pc = 0x832B5658; continue 'dispatch;
            }
            0x832B5658 => {
    //   block [0x832B5658..0x832B5684)
	// 832B5658: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B565C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B5660: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B5664: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B5668: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B566C: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832B5670: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B5674: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B5678: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B567C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B5680: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B56A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B56A8 size=64
    let mut pc: u32 = 0x832B56A8;
    'dispatch: loop {
        match pc {
            0x832B56A8 => {
    //   block [0x832B56A8..0x832B56CC)
	// 832B56A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B56AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B56B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B56B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B56B8: 3FE0834A  lis r31, -0x7cb6
	ctx.r[31].s64 = -2092302336;
	// 832B56BC: 807F5030  lwz r3, 0x5030(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20528 as u32) ) } as u64;
	// 832B56C0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B56C4: 419A0008  beq cr6, 0x832b56cc
	if ctx.cr[6].eq {
	pc = 0x832B56CC; continue 'dispatch;
	}
	// 832B56C8: 4AF66671  bl 0x8221bd38
	ctx.lr = 0x832B56CC;
	sub_8221BD38(ctx, base);
	pc = 0x832B56CC; continue 'dispatch;
            }
            0x832B56CC => {
    //   block [0x832B56CC..0x832B56E8)
	// 832B56CC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B56D0: 917F5030  stw r11, 0x5030(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20528 as u32), ctx.r[11].u32 ) };
	// 832B56D4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B56D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B56DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B56E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B56E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B56E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B56E8 size=68
    let mut pc: u32 = 0x832B56E8;
    'dispatch: loop {
        match pc {
            0x832B56E8 => {
    //   block [0x832B56E8..0x832B572C)
	// 832B56E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B56EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B56F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B56F4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B56F8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B56FC: 3BEB5034  addi r31, r11, 0x5034
	ctx.r[31].s64 = ctx.r[11].s64 + 20532;
	// 832B5700: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B5704: 4B75F68D  bl 0x82a14d90
	ctx.lr = 0x832B5708;
	sub_82A14D90(ctx, base);
	// 832B5708: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B570C: 4AF6662D  bl 0x8221bd38
	ctx.lr = 0x832B5710;
	sub_8221BD38(ctx, base);
	// 832B5710: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B5714: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B5718: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B571C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B5720: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B5724: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B5728: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B5730 size=68
    let mut pc: u32 = 0x832B5730;
    'dispatch: loop {
        match pc {
            0x832B5730 => {
    //   block [0x832B5730..0x832B5774)
	// 832B5730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B5734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B5738: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B573C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B5740: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5744: 3BEB5040  addi r31, r11, 0x5040
	ctx.r[31].s64 = ctx.r[11].s64 + 20544;
	// 832B5748: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B574C: 4B75F645  bl 0x82a14d90
	ctx.lr = 0x832B5750;
	sub_82A14D90(ctx, base);
	// 832B5750: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B5754: 4AF665E5  bl 0x8221bd38
	ctx.lr = 0x832B5758;
	sub_8221BD38(ctx, base);
	// 832B5758: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B575C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B5760: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B5764: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B5768: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B576C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B5770: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B5778 size=132
    let mut pc: u32 = 0x832B5778;
    'dispatch: loop {
        match pc {
            0x832B5778 => {
    //   block [0x832B5778..0x832B57BC)
	// 832B5778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B577C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B5780: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B5784: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B5788: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B578C: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5790: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B5794: 3BEB504C  addi r31, r11, 0x504c
	ctx.r[31].s64 = ctx.r[11].s64 + 20556;
	// 832B5798: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B579C: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B57A0: 914A0000  stw r10, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 832B57A4: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B57A8: 914A0004  stw r10, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 832B57AC: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B57B0: 7F0B1840  cmplw cr6, r11, r3
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[3].u32, &mut ctx.xer);
	// 832B57B4: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 832B57B8: 419A0020  beq cr6, 0x832b57d8
	if ctx.cr[6].eq {
	pc = 0x832B57D8; continue 'dispatch;
	}
	pc = 0x832B57BC; continue 'dispatch;
            }
            0x832B57BC => {
    //   block [0x832B57BC..0x832B57D8)
	// 832B57BC: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 832B57C0: 83CB0000  lwz r30, 0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B57C4: 4AF66575  bl 0x8221bd38
	ctx.lr = 0x832B57C8;
	sub_8221BD38(ctx, base);
	// 832B57C8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B57CC: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 832B57D0: 7F1E1840  cmplw cr6, r30, r3
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[3].u32, &mut ctx.xer);
	// 832B57D4: 409AFFE8  bne cr6, 0x832b57bc
	if !ctx.cr[6].eq {
	pc = 0x832B57BC; continue 'dispatch;
	}
	pc = 0x832B57D8; continue 'dispatch;
            }
            0x832B57D8 => {
    //   block [0x832B57D8..0x832B57FC)
	// 832B57D8: 4AF66561  bl 0x8221bd38
	ctx.lr = 0x832B57DC;
	sub_8221BD38(ctx, base);
	// 832B57DC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B57E0: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B57E4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B57E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B57EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B57F0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B57F4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B57F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B5800 size=68
    let mut pc: u32 = 0x832B5800;
    'dispatch: loop {
        match pc {
            0x832B5800 => {
    //   block [0x832B5800..0x832B5844)
	// 832B5800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B5804: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B5808: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B580C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B5810: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5814: 3BEB5058  addi r31, r11, 0x5058
	ctx.r[31].s64 = ctx.r[11].s64 + 20568;
	// 832B5818: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B581C: 4B7E18C5  bl 0x82a970e0
	ctx.lr = 0x832B5820;
	sub_82A970E0(ctx, base);
	// 832B5820: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B5824: 4AF66515  bl 0x8221bd38
	ctx.lr = 0x832B5828;
	sub_8221BD38(ctx, base);
	// 832B5828: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B582C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B5830: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B5834: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B5838: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B583C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B5840: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B5868 size=64
    let mut pc: u32 = 0x832B5868;
    'dispatch: loop {
        match pc {
            0x832B5868 => {
    //   block [0x832B5868..0x832B5894)
	// 832B5868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B586C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B5870: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B5874: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B5878: 3FE0834A  lis r31, -0x7cb6
	ctx.r[31].s64 = -2092302336;
	// 832B587C: 807F5120  lwz r3, 0x5120(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20768 as u32) ) } as u64;
	// 832B5880: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B5884: 419A0010  beq cr6, 0x832b5894
	if ctx.cr[6].eq {
	pc = 0x832B5894; continue 'dispatch;
	}
	// 832B5888: 4B8D1D41  bl 0x82b875c8
	ctx.lr = 0x832B588C;
	sub_82B875C8(ctx, base);
	// 832B588C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B5890: 917F5120  stw r11, 0x5120(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20768 as u32), ctx.r[11].u32 ) };
	pc = 0x832B5894; continue 'dispatch;
            }
            0x832B5894 => {
    //   block [0x832B5894..0x832B58A8)
	// 832B5894: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B5898: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B589C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B58A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B58A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B58A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B58A8 size=64
    let mut pc: u32 = 0x832B58A8;
    'dispatch: loop {
        match pc {
            0x832B58A8 => {
    //   block [0x832B58A8..0x832B58D4)
	// 832B58A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B58AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B58B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B58B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B58B8: 3FE08332  lis r31, -0x7cce
	ctx.r[31].s64 = -2093875200;
	// 832B58BC: 807FAD00  lwz r3, -0x5300(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-21248 as u32) ) } as u64;
	// 832B58C0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B58C4: 419A0010  beq cr6, 0x832b58d4
	if ctx.cr[6].eq {
	pc = 0x832B58D4; continue 'dispatch;
	}
	// 832B58C8: 4B8D1D01  bl 0x82b875c8
	ctx.lr = 0x832B58CC;
	sub_82B875C8(ctx, base);
	// 832B58CC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B58D0: 917FAD00  stw r11, -0x5300(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-21248 as u32), ctx.r[11].u32 ) };
	pc = 0x832B58D4; continue 'dispatch;
            }
            0x832B58D4 => {
    //   block [0x832B58D4..0x832B58E8)
	// 832B58D4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B58D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B58DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B58E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B58E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B58E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B58E8 size=64
    let mut pc: u32 = 0x832B58E8;
    'dispatch: loop {
        match pc {
            0x832B58E8 => {
    //   block [0x832B58E8..0x832B5914)
	// 832B58E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B58EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B58F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B58F4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B58F8: 3FE0834A  lis r31, -0x7cb6
	ctx.r[31].s64 = -2092302336;
	// 832B58FC: 807F512C  lwz r3, 0x512c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20780 as u32) ) } as u64;
	// 832B5900: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B5904: 419A0010  beq cr6, 0x832b5914
	if ctx.cr[6].eq {
	pc = 0x832B5914; continue 'dispatch;
	}
	// 832B5908: 4B8D1CC1  bl 0x82b875c8
	ctx.lr = 0x832B590C;
	sub_82B875C8(ctx, base);
	// 832B590C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B5910: 917F512C  stw r11, 0x512c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20780 as u32), ctx.r[11].u32 ) };
	pc = 0x832B5914; continue 'dispatch;
            }
            0x832B5914 => {
    //   block [0x832B5914..0x832B5928)
	// 832B5914: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B5918: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B591C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B5920: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B5924: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B5928 size=64
    let mut pc: u32 = 0x832B5928;
    'dispatch: loop {
        match pc {
            0x832B5928 => {
    //   block [0x832B5928..0x832B5954)
	// 832B5928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B592C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B5930: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B5934: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B5938: 3FE08332  lis r31, -0x7cce
	ctx.r[31].s64 = -2093875200;
	// 832B593C: 807FAD0C  lwz r3, -0x52f4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-21236 as u32) ) } as u64;
	// 832B5940: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B5944: 419A0010  beq cr6, 0x832b5954
	if ctx.cr[6].eq {
	pc = 0x832B5954; continue 'dispatch;
	}
	// 832B5948: 4B8D1C81  bl 0x82b875c8
	ctx.lr = 0x832B594C;
	sub_82B875C8(ctx, base);
	// 832B594C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B5950: 917FAD0C  stw r11, -0x52f4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-21236 as u32), ctx.r[11].u32 ) };
	pc = 0x832B5954; continue 'dispatch;
            }
            0x832B5954 => {
    //   block [0x832B5954..0x832B5968)
	// 832B5954: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B5958: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B595C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B5960: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B5964: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B5968 size=64
    let mut pc: u32 = 0x832B5968;
    'dispatch: loop {
        match pc {
            0x832B5968 => {
    //   block [0x832B5968..0x832B5994)
	// 832B5968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B596C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B5970: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B5974: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B5978: 3FE0834A  lis r31, -0x7cb6
	ctx.r[31].s64 = -2092302336;
	// 832B597C: 807F5138  lwz r3, 0x5138(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20792 as u32) ) } as u64;
	// 832B5980: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B5984: 419A0010  beq cr6, 0x832b5994
	if ctx.cr[6].eq {
	pc = 0x832B5994; continue 'dispatch;
	}
	// 832B5988: 4B8D1C41  bl 0x82b875c8
	ctx.lr = 0x832B598C;
	sub_82B875C8(ctx, base);
	// 832B598C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B5990: 917F5138  stw r11, 0x5138(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20792 as u32), ctx.r[11].u32 ) };
	pc = 0x832B5994; continue 'dispatch;
            }
            0x832B5994 => {
    //   block [0x832B5994..0x832B59A8)
	// 832B5994: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B5998: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B599C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B59A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B59A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B59C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B59C0 size=72
    let mut pc: u32 = 0x832B59C0;
    'dispatch: loop {
        match pc {
            0x832B59C0 => {
    //   block [0x832B59C0..0x832B5A08)
	// 832B59C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B59C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B59C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B59CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B59D0: 3FE08332  lis r31, -0x7cce
	ctx.r[31].s64 = -2093875200;
	// 832B59D4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832B59D8: 387FAD18  addi r3, r31, -0x52e8
	ctx.r[3].s64 = ctx.r[31].s64 + -21224;
	// 832B59DC: 396B7EFC  addi r11, r11, 0x7efc
	ctx.r[11].s64 = ctx.r[11].s64 + 32508;
	// 832B59E0: 917FAD18  stw r11, -0x52e8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-21224 as u32), ctx.r[11].u32 ) };
	// 832B59E4: 4B935955  bl 0x82beb338
	ctx.lr = 0x832B59E8;
	sub_82BEB338(ctx, base);
	// 832B59E8: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832B59EC: 396B2B90  addi r11, r11, 0x2b90
	ctx.r[11].s64 = ctx.r[11].s64 + 11152;
	// 832B59F0: 917FAD18  stw r11, -0x52e8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-21224 as u32), ctx.r[11].u32 ) };
	// 832B59F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B59F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B59FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B5A00: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B5A04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5A08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5A08 size=12
    let mut pc: u32 = 0x832B5A08;
    'dispatch: loop {
        match pc {
            0x832B5A08 => {
    //   block [0x832B5A08..0x832B5A14)
	// 832B5A08: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5A0C: 386B5158  addi r3, r11, 0x5158
	ctx.r[3].s64 = ctx.r[11].s64 + 20824;
	// 832B5A10: 4B7E38C8  b 0x82a992d8
	sub_82A992D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5A18 size=12
    let mut pc: u32 = 0x832B5A18;
    'dispatch: loop {
        match pc {
            0x832B5A18 => {
    //   block [0x832B5A18..0x832B5A24)
	// 832B5A18: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5A1C: 386B5164  addi r3, r11, 0x5164
	ctx.r[3].s64 = ctx.r[11].s64 + 20836;
	// 832B5A20: 4B7E3A68  b 0x82a99488
	sub_82A99488(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5A28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5A28 size=12
    let mut pc: u32 = 0x832B5A28;
    'dispatch: loop {
        match pc {
            0x832B5A28 => {
    //   block [0x832B5A28..0x832B5A34)
	// 832B5A28: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5A2C: 386B5170  addi r3, r11, 0x5170
	ctx.r[3].s64 = ctx.r[11].s64 + 20848;
	// 832B5A30: 4B7E3C08  b 0x82a99638
	sub_82A99638(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5A38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5A38 size=12
    let mut pc: u32 = 0x832B5A38;
    'dispatch: loop {
        match pc {
            0x832B5A38 => {
    //   block [0x832B5A38..0x832B5A44)
	// 832B5A38: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5A3C: 386B517C  addi r3, r11, 0x517c
	ctx.r[3].s64 = ctx.r[11].s64 + 20860;
	// 832B5A40: 4B7E3DA8  b 0x82a997e8
	sub_82A997E8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5A48 size=12
    let mut pc: u32 = 0x832B5A48;
    'dispatch: loop {
        match pc {
            0x832B5A48 => {
    //   block [0x832B5A48..0x832B5A54)
	// 832B5A48: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5A4C: 386B5188  addi r3, r11, 0x5188
	ctx.r[3].s64 = ctx.r[11].s64 + 20872;
	// 832B5A50: 4B7E3F48  b 0x82a99998
	sub_82A99998(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5A58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5A58 size=12
    let mut pc: u32 = 0x832B5A58;
    'dispatch: loop {
        match pc {
            0x832B5A58 => {
    //   block [0x832B5A58..0x832B5A64)
	// 832B5A58: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5A5C: 386B5194  addi r3, r11, 0x5194
	ctx.r[3].s64 = ctx.r[11].s64 + 20884;
	// 832B5A60: 4AF5F378  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5A68 size=12
    let mut pc: u32 = 0x832B5A68;
    'dispatch: loop {
        match pc {
            0x832B5A68 => {
    //   block [0x832B5A68..0x832B5A74)
	// 832B5A68: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5A6C: 386B5198  addi r3, r11, 0x5198
	ctx.r[3].s64 = ctx.r[11].s64 + 20888;
	// 832B5A70: 4AF5F368  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5A78 size=4
    let mut pc: u32 = 0x832B5A78;
    'dispatch: loop {
        match pc {
            0x832B5A78 => {
    //   block [0x832B5A78..0x832B5A7C)
	// 832B5A78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5A80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5A80 size=12
    let mut pc: u32 = 0x832B5A80;
    'dispatch: loop {
        match pc {
            0x832B5A80 => {
    //   block [0x832B5A80..0x832B5A8C)
	// 832B5A80: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5A84: 386B5290  addi r3, r11, 0x5290
	ctx.r[3].s64 = ctx.r[11].s64 + 21136;
	// 832B5A88: 4AF5F350  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5A90 size=12
    let mut pc: u32 = 0x832B5A90;
    'dispatch: loop {
        match pc {
            0x832B5A90 => {
    //   block [0x832B5A90..0x832B5A9C)
	// 832B5A90: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5A94: 386B5294  addi r3, r11, 0x5294
	ctx.r[3].s64 = ctx.r[11].s64 + 21140;
	// 832B5A98: 4AF5F340  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5AA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5AA0 size=12
    let mut pc: u32 = 0x832B5AA0;
    'dispatch: loop {
        match pc {
            0x832B5AA0 => {
    //   block [0x832B5AA0..0x832B5AAC)
	// 832B5AA0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5AA4: 386B5298  addi r3, r11, 0x5298
	ctx.r[3].s64 = ctx.r[11].s64 + 21144;
	// 832B5AA8: 4AFBF860  b 0x82275308
	sub_82275308(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5AB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5AB0 size=12
    let mut pc: u32 = 0x832B5AB0;
    'dispatch: loop {
        match pc {
            0x832B5AB0 => {
    //   block [0x832B5AB0..0x832B5ABC)
	// 832B5AB0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5AB4: 386B5654  addi r3, r11, 0x5654
	ctx.r[3].s64 = ctx.r[11].s64 + 22100;
	// 832B5AB8: 4AFE7FE0  b 0x8229da98
	sub_8229DA98(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5AC0 size=12
    let mut pc: u32 = 0x832B5AC0;
    'dispatch: loop {
        match pc {
            0x832B5AC0 => {
    //   block [0x832B5AC0..0x832B5ACC)
	// 832B5AC0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5AC4: 386B56A0  addi r3, r11, 0x56a0
	ctx.r[3].s64 = ctx.r[11].s64 + 22176;
	// 832B5AC8: 4AF5F310  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5AD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5AD0 size=12
    let mut pc: u32 = 0x832B5AD0;
    'dispatch: loop {
        match pc {
            0x832B5AD0 => {
    //   block [0x832B5AD0..0x832B5ADC)
	// 832B5AD0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5AD4: 386B56A4  addi r3, r11, 0x56a4
	ctx.r[3].s64 = ctx.r[11].s64 + 22180;
	// 832B5AD8: 4AF5F300  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5AE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5AE0 size=12
    let mut pc: u32 = 0x832B5AE0;
    'dispatch: loop {
        match pc {
            0x832B5AE0 => {
    //   block [0x832B5AE0..0x832B5AEC)
	// 832B5AE0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5AE4: 386B56A8  addi r3, r11, 0x56a8
	ctx.r[3].s64 = ctx.r[11].s64 + 22184;
	// 832B5AE8: 4AF5F2F0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5AF0 size=12
    let mut pc: u32 = 0x832B5AF0;
    'dispatch: loop {
        match pc {
            0x832B5AF0 => {
    //   block [0x832B5AF0..0x832B5AFC)
	// 832B5AF0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5AF4: 386B56AC  addi r3, r11, 0x56ac
	ctx.r[3].s64 = ctx.r[11].s64 + 22188;
	// 832B5AF8: 4AF5F2E0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5B00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5B00 size=12
    let mut pc: u32 = 0x832B5B00;
    'dispatch: loop {
        match pc {
            0x832B5B00 => {
    //   block [0x832B5B00..0x832B5B0C)
	// 832B5B00: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5B04: 386B56B0  addi r3, r11, 0x56b0
	ctx.r[3].s64 = ctx.r[11].s64 + 22192;
	// 832B5B08: 4AF5F2D0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5B10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5B10 size=12
    let mut pc: u32 = 0x832B5B10;
    'dispatch: loop {
        match pc {
            0x832B5B10 => {
    //   block [0x832B5B10..0x832B5B1C)
	// 832B5B10: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5B14: 386B56B4  addi r3, r11, 0x56b4
	ctx.r[3].s64 = ctx.r[11].s64 + 22196;
	// 832B5B18: 4AF5F2C0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5B20 size=12
    let mut pc: u32 = 0x832B5B20;
    'dispatch: loop {
        match pc {
            0x832B5B20 => {
    //   block [0x832B5B20..0x832B5B2C)
	// 832B5B20: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5B24: 386B56B8  addi r3, r11, 0x56b8
	ctx.r[3].s64 = ctx.r[11].s64 + 22200;
	// 832B5B28: 4AF5F2B0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5B30 size=12
    let mut pc: u32 = 0x832B5B30;
    'dispatch: loop {
        match pc {
            0x832B5B30 => {
    //   block [0x832B5B30..0x832B5B3C)
	// 832B5B30: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5B34: 386B56BC  addi r3, r11, 0x56bc
	ctx.r[3].s64 = ctx.r[11].s64 + 22204;
	// 832B5B38: 4AF5F2A0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5B40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B5B40 size=68
    let mut pc: u32 = 0x832B5B40;
    'dispatch: loop {
        match pc {
            0x832B5B40 => {
    //   block [0x832B5B40..0x832B5B84)
	// 832B5B40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B5B44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B5B48: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B5B4C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B5B50: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5B54: 3BEB56C0  addi r31, r11, 0x56c0
	ctx.r[31].s64 = ctx.r[11].s64 + 22208;
	// 832B5B58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B5B5C: 4AEC67DD  bl 0x8217c338
	ctx.lr = 0x832B5B60;
	sub_8217C338(ctx, base);
	// 832B5B60: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B5B64: 4AF661D5  bl 0x8221bd38
	ctx.lr = 0x832B5B68;
	sub_8221BD38(ctx, base);
	// 832B5B68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B5B6C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B5B70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B5B74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B5B78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B5B7C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B5B80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5B88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B5B88 size=68
    let mut pc: u32 = 0x832B5B88;
    'dispatch: loop {
        match pc {
            0x832B5B88 => {
    //   block [0x832B5B88..0x832B5BCC)
	// 832B5B88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B5B8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B5B90: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B5B94: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B5B98: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5B9C: 3BEB56CC  addi r31, r11, 0x56cc
	ctx.r[31].s64 = ctx.r[11].s64 + 22220;
	// 832B5BA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B5BA4: 4AEC69ED  bl 0x8217c590
	ctx.lr = 0x832B5BA8;
	sub_8217C590(ctx, base);
	// 832B5BA8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B5BAC: 4AF6618D  bl 0x8221bd38
	ctx.lr = 0x832B5BB0;
	sub_8221BD38(ctx, base);
	// 832B5BB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B5BB4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B5BB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B5BBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B5BC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B5BC4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B5BC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5BD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B5BD0 size=68
    let mut pc: u32 = 0x832B5BD0;
    'dispatch: loop {
        match pc {
            0x832B5BD0 => {
    //   block [0x832B5BD0..0x832B5C14)
	// 832B5BD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B5BD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B5BD8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B5BDC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B5BE0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5BE4: 3BEB56D8  addi r31, r11, 0x56d8
	ctx.r[31].s64 = ctx.r[11].s64 + 22232;
	// 832B5BE8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B5BEC: 4AEC683D  bl 0x8217c428
	ctx.lr = 0x832B5BF0;
	sub_8217C428(ctx, base);
	// 832B5BF0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B5BF4: 4AF66145  bl 0x8221bd38
	ctx.lr = 0x832B5BF8;
	sub_8221BD38(ctx, base);
	// 832B5BF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B5BFC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B5C00: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B5C04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B5C08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B5C0C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B5C10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5C18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B5C18 size=68
    let mut pc: u32 = 0x832B5C18;
    'dispatch: loop {
        match pc {
            0x832B5C18 => {
    //   block [0x832B5C18..0x832B5C5C)
	// 832B5C18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B5C1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B5C20: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B5C24: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B5C28: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5C2C: 3BEB56E4  addi r31, r11, 0x56e4
	ctx.r[31].s64 = ctx.r[11].s64 + 22244;
	// 832B5C30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B5C34: 4AEC68E5  bl 0x8217c518
	ctx.lr = 0x832B5C38;
	sub_8217C518(ctx, base);
	// 832B5C38: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B5C3C: 4AF660FD  bl 0x8221bd38
	ctx.lr = 0x832B5C40;
	sub_8221BD38(ctx, base);
	// 832B5C40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B5C44: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B5C48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B5C4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B5C50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B5C54: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B5C58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B5C60 size=68
    let mut pc: u32 = 0x832B5C60;
    'dispatch: loop {
        match pc {
            0x832B5C60 => {
    //   block [0x832B5C60..0x832B5CA4)
	// 832B5C60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B5C64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B5C68: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B5C6C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B5C70: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5C74: 3BEB56F0  addi r31, r11, 0x56f0
	ctx.r[31].s64 = ctx.r[11].s64 + 22256;
	// 832B5C78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B5C7C: 4AEC6735  bl 0x8217c3b0
	ctx.lr = 0x832B5C80;
	sub_8217C3B0(ctx, base);
	// 832B5C80: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B5C84: 4AF660B5  bl 0x8221bd38
	ctx.lr = 0x832B5C88;
	sub_8221BD38(ctx, base);
	// 832B5C88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B5C8C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B5C90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B5C94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B5C98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B5C9C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B5CA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B5CA8 size=68
    let mut pc: u32 = 0x832B5CA8;
    'dispatch: loop {
        match pc {
            0x832B5CA8 => {
    //   block [0x832B5CA8..0x832B5CEC)
	// 832B5CA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B5CAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B5CB0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B5CB4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B5CB8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5CBC: 3BEB56FC  addi r31, r11, 0x56fc
	ctx.r[31].s64 = ctx.r[11].s64 + 22268;
	// 832B5CC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B5CC4: 4AEC67DD  bl 0x8217c4a0
	ctx.lr = 0x832B5CC8;
	sub_8217C4A0(ctx, base);
	// 832B5CC8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B5CCC: 4AF6606D  bl 0x8221bd38
	ctx.lr = 0x832B5CD0;
	sub_8221BD38(ctx, base);
	// 832B5CD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B5CD4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B5CD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B5CDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B5CE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B5CE4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B5CE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5CF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B5CF0 size=84
    let mut pc: u32 = 0x832B5CF0;
    'dispatch: loop {
        match pc {
            0x832B5CF0 => {
    //   block [0x832B5CF0..0x832B5D24)
	// 832B5CF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B5CF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B5CF8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B5CFC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B5D00: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B5D04: 3FC0834A  lis r30, -0x7cb6
	ctx.r[30].s64 = -2092302336;
	// 832B5D08: 83FE5708  lwz r31, 0x5708(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(22280 as u32) ) } as u64;
	// 832B5D0C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 832B5D10: 419A0014  beq cr6, 0x832b5d24
	if ctx.cr[6].eq {
	pc = 0x832B5D24; continue 'dispatch;
	}
	// 832B5D14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B5D18: 4B8C2689  bl 0x82b783a0
	ctx.lr = 0x832B5D1C;
	sub_82B783A0(ctx, base);
	// 832B5D1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B5D20: 4AF66019  bl 0x8221bd38
	ctx.lr = 0x832B5D24;
	sub_8221BD38(ctx, base);
	pc = 0x832B5D24; continue 'dispatch;
            }
            0x832B5D24 => {
    //   block [0x832B5D24..0x832B5D44)
	// 832B5D24: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B5D28: 917E5708  stw r11, 0x5708(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(22280 as u32), ctx.r[11].u32 ) };
	// 832B5D2C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B5D30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B5D34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B5D38: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B5D3C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B5D40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5D48 size=12
    let mut pc: u32 = 0x832B5D48;
    'dispatch: loop {
        match pc {
            0x832B5D48 => {
    //   block [0x832B5D48..0x832B5D54)
	// 832B5D48: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5D4C: 386B570C  addi r3, r11, 0x570c
	ctx.r[3].s64 = ctx.r[11].s64 + 22284;
	// 832B5D50: 4AF7BF68  b 0x82231cb8
	sub_82231CB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5D58 size=12
    let mut pc: u32 = 0x832B5D58;
    'dispatch: loop {
        match pc {
            0x832B5D58 => {
    //   block [0x832B5D58..0x832B5D64)
	// 832B5D58: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5D5C: 386B5710  addi r3, r11, 0x5710
	ctx.r[3].s64 = ctx.r[11].s64 + 22288;
	// 832B5D60: 4AF7BF58  b 0x82231cb8
	sub_82231CB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5D68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5D68 size=12
    let mut pc: u32 = 0x832B5D68;
    'dispatch: loop {
        match pc {
            0x832B5D68 => {
    //   block [0x832B5D68..0x832B5D74)
	// 832B5D68: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5D6C: 386B5714  addi r3, r11, 0x5714
	ctx.r[3].s64 = ctx.r[11].s64 + 22292;
	// 832B5D70: 4AF5F068  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5D78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5D78 size=12
    let mut pc: u32 = 0x832B5D78;
    'dispatch: loop {
        match pc {
            0x832B5D78 => {
    //   block [0x832B5D78..0x832B5D84)
	// 832B5D78: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5D7C: 386B5718  addi r3, r11, 0x5718
	ctx.r[3].s64 = ctx.r[11].s64 + 22296;
	// 832B5D80: 4AF5F058  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5D88 size=12
    let mut pc: u32 = 0x832B5D88;
    'dispatch: loop {
        match pc {
            0x832B5D88 => {
    //   block [0x832B5D88..0x832B5D94)
	// 832B5D88: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5D8C: 386B571C  addi r3, r11, 0x571c
	ctx.r[3].s64 = ctx.r[11].s64 + 22300;
	// 832B5D90: 4B7F7F28  b 0x82aadcb8
	sub_82AADCB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5D98 size=12
    let mut pc: u32 = 0x832B5D98;
    'dispatch: loop {
        match pc {
            0x832B5D98 => {
    //   block [0x832B5D98..0x832B5DA4)
	// 832B5D98: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5D9C: 386B572C  addi r3, r11, 0x572c
	ctx.r[3].s64 = ctx.r[11].s64 + 22316;
	// 832B5DA0: 4AF7BF18  b 0x82231cb8
	sub_82231CB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5DA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5DA8 size=12
    let mut pc: u32 = 0x832B5DA8;
    'dispatch: loop {
        match pc {
            0x832B5DA8 => {
    //   block [0x832B5DA8..0x832B5DB4)
	// 832B5DA8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5DAC: 386B5730  addi r3, r11, 0x5730
	ctx.r[3].s64 = ctx.r[11].s64 + 22320;
	// 832B5DB0: 4AF7BF08  b 0x82231cb8
	sub_82231CB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5DB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5DB8 size=4
    let mut pc: u32 = 0x832B5DB8;
    'dispatch: loop {
        match pc {
            0x832B5DB8 => {
    //   block [0x832B5DB8..0x832B5DBC)
	// 832B5DB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5DC0 size=12
    let mut pc: u32 = 0x832B5DC0;
    'dispatch: loop {
        match pc {
            0x832B5DC0 => {
    //   block [0x832B5DC0..0x832B5DCC)
	// 832B5DC0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5DC4: 386B5758  addi r3, r11, 0x5758
	ctx.r[3].s64 = ctx.r[11].s64 + 22360;
	// 832B5DC8: 4AF5F010  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5DD0 size=12
    let mut pc: u32 = 0x832B5DD0;
    'dispatch: loop {
        match pc {
            0x832B5DD0 => {
    //   block [0x832B5DD0..0x832B5DDC)
	// 832B5DD0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5DD4: 386B575C  addi r3, r11, 0x575c
	ctx.r[3].s64 = ctx.r[11].s64 + 22364;
	// 832B5DD8: 4AF5F000  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B5DE0 size=80
    let mut pc: u32 = 0x832B5DE0;
    'dispatch: loop {
        match pc {
            0x832B5DE0 => {
    //   block [0x832B5DE0..0x832B5E14)
	// 832B5DE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B5DE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B5DE8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B5DEC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B5DF0: 3FE0834A  lis r31, -0x7cb6
	ctx.r[31].s64 = -2092302336;
	// 832B5DF4: 807F5760  lwz r3, 0x5760(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(22368 as u32) ) } as u64;
	// 832B5DF8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B5DFC: 419A0018  beq cr6, 0x832b5e14
	if ctx.cr[6].eq {
	pc = 0x832B5E14; continue 'dispatch;
	}
	// 832B5E00: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B5E04: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 832B5E08: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B5E0C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 832B5E10: 4E800421  bctrl
	ctx.lr = 0x832B5E14;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x832B5E14 => {
    //   block [0x832B5E14..0x832B5E30)
	// 832B5E14: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B5E18: 917F5760  stw r11, 0x5760(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(22368 as u32), ctx.r[11].u32 ) };
	// 832B5E1C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B5E20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B5E24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B5E28: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B5E2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B5E30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B5E30 size=12
    let mut pc: u32 = 0x832B5E30;
    'dispatch: loop {
        match pc {
            0x832B5E30 => {
    //   block [0x832B5E30..0x832B5E3C)
	// 832B5E30: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B5E34: 386B5764  addi r3, r11, 0x5764
	ctx.r[3].s64 = ctx.r[11].s64 + 22372;
	// 832B5E38: 4AF5EFA0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


