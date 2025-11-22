pub fn sub_823004F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823004F0 size=8
    let mut pc: u32 = 0x823004F0;
    'dispatch: loop {
        match pc {
            0x823004F0 => {
    //   block [0x823004F0..0x823004F8)
	// 823004F0: 98830264  stb r4, 0x264(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(612 as u32), ctx.r[4].u8 ) };
	// 823004F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823004F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823004F8 size=20
    let mut pc: u32 = 0x823004F8;
    'dispatch: loop {
        match pc {
            0x823004F8 => {
    //   block [0x823004F8..0x8230050C)
	// 823004F8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 823004FC: 88840018  lbz r4, 0x18(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 82300500: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82300504: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82300508: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82300510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82300510 size=12
    let mut pc: u32 = 0x82300510;
    'dispatch: loop {
        match pc {
            0x82300510 => {
    //   block [0x82300510..0x8230051C)
	// 82300510: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82300514: 99630256  stb r11, 0x256(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(598 as u32), ctx.r[11].u8 ) };
	// 82300518: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82300520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82300520 size=12
    let mut pc: u32 = 0x82300520;
    'dispatch: loop {
        match pc {
            0x82300520 => {
    //   block [0x82300520..0x8230052C)
	// 82300520: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82300524: 99630256  stb r11, 0x256(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(598 as u32), ctx.r[11].u8 ) };
	// 82300528: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82300530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82300530 size=72
    let mut pc: u32 = 0x82300530;
    'dispatch: loop {
        match pc {
            0x82300530 => {
    //   block [0x82300530..0x82300578)
	// 82300530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82300534: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82300538: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8230053C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82300540: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82300544: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82300548: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8230054C: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82300550: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82300554: 4E800421  bctrl
	ctx.lr = 0x82300558;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82300558: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8230055C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82300560: 48B55E69  bl 0x82e563c8
	ctx.lr = 0x82300564;
	sub_82E563C8(ctx, base);
	// 82300564: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82300568: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8230056C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82300570: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82300574: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82300578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82300578 size=72
    let mut pc: u32 = 0x82300578;
    'dispatch: loop {
        match pc {
            0x82300578 => {
    //   block [0x82300578..0x823005C0)
	// 82300578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8230057C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82300580: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82300584: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82300588: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8230058C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82300590: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82300594: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82300598: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8230059C: 4E800421  bctrl
	ctx.lr = 0x823005A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823005A0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 823005A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823005A8: 48B55E21  bl 0x82e563c8
	ctx.lr = 0x823005AC;
	sub_82E563C8(ctx, base);
	// 823005AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 823005B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823005B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823005B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823005BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823005C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823005C0 size=136
    let mut pc: u32 = 0x823005C0;
    'dispatch: loop {
        match pc {
            0x823005C0 => {
    //   block [0x823005C0..0x82300648)
	// 823005C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823005C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823005C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823005CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823005D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823005D4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823005D8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823005DC: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 823005E0: 409A0020  bne cr6, 0x82300600
	if !ctx.cr[6].eq {
	pc = 0x82300600; continue 'dispatch;
	}
	// 823005E4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823005E8: 419A0048  beq cr6, 0x82300630
	if ctx.cr[6].eq {
	pc = 0x82300630; continue 'dispatch;
	}
	// 823005EC: E97E0000  ld r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	// 823005F0: F97F0000  std r11, 0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 823005F4: E97E0008  ld r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) };
	// 823005F8: F97F0008  std r11, 8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 823005FC: 48000034  b 0x82300630
	pc = 0x82300630; continue 'dispatch;
	// 82300600: 2F050001  cmpwi cr6, r5, 1
	ctx.cr[6].compare_i32(ctx.r[5].s32, 1, &mut ctx.xer);
	// 82300604: 419A002C  beq cr6, 0x82300630
	if ctx.cr[6].eq {
	pc = 0x82300630; continue 'dispatch;
	}
	// 82300608: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 8230060C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82300610: 388B5718  addi r4, r11, 0x5718
	ctx.r[4].s64 = ctx.r[11].s64 + 22296;
	// 82300614: 48EA7AE5  bl 0x831a80f8
	ctx.lr = 0x82300618;
	sub_831A80F8(ctx, base);
	// 82300618: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8230061C: 4182000C  beq 0x82300628
	if ctx.cr[0].eq {
	pc = 0x82300628; continue 'dispatch;
	}
	// 82300620: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82300624: 4800000C  b 0x82300630
	pc = 0x82300630; continue 'dispatch;
	// 82300628: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8230062C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82300630: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82300634: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82300638: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8230063C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82300640: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82300644: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82300648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82300648 size=76
    let mut pc: u32 = 0x82300648;
    'dispatch: loop {
        match pc {
            0x82300648 => {
    //   block [0x82300648..0x82300694)
	// 82300648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8230064C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82300650: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82300654: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82300658: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8230065C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82300660: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82300664: 488BD025  bl 0x82bbd688
	ctx.lr = 0x82300668;
	sub_82BBD688(ctx, base);
	// 82300668: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8230066C: 4182000C  beq 0x82300678
	if ctx.cr[0].eq {
	pc = 0x82300678; continue 'dispatch;
	}
	// 82300670: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82300674: 48AF1D65  bl 0x82df23d8
	ctx.lr = 0x82300678;
	sub_82DF23D8(ctx, base);
	// 82300678: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8230067C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82300680: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82300684: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82300688: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8230068C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82300690: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82300698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82300698 size=276
    let mut pc: u32 = 0x82300698;
    'dispatch: loop {
        match pc {
            0x82300698 => {
    //   block [0x82300698..0x823007AC)
	// 82300698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8230069C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823006A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823006A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823006A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823006AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823006B0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 823006B4: 807F00C4  lwz r3, 0xc4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 823006B8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 823006BC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 823006C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823006C4: 4E800421  bctrl
	ctx.lr = 0x823006C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823006C8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 823006CC: 387F012C  addi r3, r31, 0x12c
	ctx.r[3].s64 = ctx.r[31].s64 + 300;
	// 823006D0: 48B5F7A9  bl 0x82e5fe78
	ctx.lr = 0x823006D4;
	sub_82E5FE78(ctx, base);
	// 823006D4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 823006D8: 387F00CC  addi r3, r31, 0xcc
	ctx.r[3].s64 = ctx.r[31].s64 + 204;
	// 823006DC: 48B5C79D  bl 0x82e5ce78
	ctx.lr = 0x823006E0;
	sub_82E5CE78(ctx, base);
	// 823006E0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 823006E4: 387F0134  addi r3, r31, 0x134
	ctx.r[3].s64 = ctx.r[31].s64 + 308;
	// 823006E8: 48B5F791  bl 0x82e5fe78
	ctx.lr = 0x823006EC;
	sub_82E5FE78(ctx, base);
	// 823006EC: 807F00C4  lwz r3, 0xc4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 823006F0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 823006F4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 823006F8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823006FC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82300700: 4E800421  bctrl
	ctx.lr = 0x82300704;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82300704: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82300708: 807F00C4  lwz r3, 0xc4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 8230070C: 388B6910  addi r4, r11, 0x6910
	ctx.r[4].s64 = ctx.r[11].s64 + 26896;
	// 82300710: 487130C9  bl 0x82a137d8
	ctx.lr = 0x82300714;
	sub_82A137D8(ctx, base);
	// 82300714: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82300718: 387F01B0  addi r3, r31, 0x1b0
	ctx.r[3].s64 = ctx.r[31].s64 + 432;
	// 8230071C: 48B5C75D  bl 0x82e5ce78
	ctx.lr = 0x82300720;
	sub_82E5CE78(ctx, base);
	// 82300720: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82300724: 387F0224  addi r3, r31, 0x224
	ctx.r[3].s64 = ctx.r[31].s64 + 548;
	// 82300728: 48B5F751  bl 0x82e5fe78
	ctx.lr = 0x8230072C;
	sub_82E5FE78(ctx, base);
	// 8230072C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82300730: 387F013C  addi r3, r31, 0x13c
	ctx.r[3].s64 = ctx.r[31].s64 + 316;
	// 82300734: 48B5C745  bl 0x82e5ce78
	ctx.lr = 0x82300738;
	sub_82E5CE78(ctx, base);
	// 82300738: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8230073C: 387F022C  addi r3, r31, 0x22c
	ctx.r[3].s64 = ctx.r[31].s64 + 556;
	// 82300740: 48B5F739  bl 0x82e5fe78
	ctx.lr = 0x82300744;
	sub_82E5FE78(ctx, base);
	// 82300744: 807F00C4  lwz r3, 0xc4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 82300748: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8230074C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82300750: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82300754: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82300758: 4E800421  bctrl
	ctx.lr = 0x8230075C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8230075C: 807F023C  lwz r3, 0x23c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(572 as u32) ) } as u64;
	// 82300760: 488AE429  bl 0x82baeb88
	ctx.lr = 0x82300764;
	sub_82BAEB88(ctx, base);
	// 82300764: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82300768: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8230076C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82300770: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82300774: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82300778: 4E800421  bctrl
	ctx.lr = 0x8230077C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8230077C: 807F00C4  lwz r3, 0xc4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 82300780: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82300784: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82300788: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8230078C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82300790: 4E800421  bctrl
	ctx.lr = 0x82300794;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82300794: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82300798: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8230079C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823007A0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823007A4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823007A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823007B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x823007B0 size=88
    let mut pc: u32 = 0x823007B0;
    'dispatch: loop {
        match pc {
            0x823007B0 => {
    //   block [0x823007B0..0x82300808)
	// 823007B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823007B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823007B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823007BC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823007C0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823007C4: 808300C4  lwz r4, 0xc4(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(196 as u32) ) } as u64;
	// 823007C8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823007CC: 480052ED  bl 0x82305ab8
	ctx.lr = 0x823007D0;
	sub_82305AB8(ctx, base);
	// 823007D0: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 823007D4: C0030000  lfs f0, 0(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823007D8: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 823007DC: C0030004  lfs f0, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823007E0: D00B0004  stfs f0, 4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 823007E4: C0030008  lfs f0, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823007E8: D00B0008  stfs f0, 8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 823007EC: C003000C  lfs f0, 0xc(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823007F0: D00B000C  stfs f0, 0xc(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 823007F4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823007F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823007FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82300800: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82300804: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82300808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82300808 size=8
    let mut pc: u32 = 0x82300808;
    'dispatch: loop {
        match pc {
            0x82300808 => {
    //   block [0x82300808..0x82300810)
	// 82300808: 8063017C  lwz r3, 0x17c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(380 as u32) ) } as u64;
	// 8230080C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82300810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82300810 size=8
    let mut pc: u32 = 0x82300810;
    'dispatch: loop {
        match pc {
            0x82300810 => {
    //   block [0x82300810..0x82300818)
	// 82300810: 8063023C  lwz r3, 0x23c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(572 as u32) ) } as u64;
	// 82300814: 488AD12C  b 0x82bad940
	sub_82BAD940(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82300818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82300818 size=192
    let mut pc: u32 = 0x82300818;
    'dispatch: loop {
        match pc {
            0x82300818 => {
    //   block [0x82300818..0x823008D8)
	// 82300818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8230081C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82300820: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82300824: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82300828: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8230082C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82300830: 3BDF00CC  addi r30, r31, 0xcc
	ctx.r[30].s64 = ctx.r[31].s64 + 204;
	// 82300834: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82300838: 817F00CC  lwz r11, 0xcc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 8230083C: 809F00C4  lwz r4, 0xc4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 82300840: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82300844: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82300848: 4E800421  bctrl
	ctx.lr = 0x8230084C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8230084C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82300850: 387F012C  addi r3, r31, 0x12c
	ctx.r[3].s64 = ctx.r[31].s64 + 300;
	// 82300854: 809F00C4  lwz r4, 0xc4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 82300858: 48B5E2A9  bl 0x82e5eb00
	ctx.lr = 0x8230085C;
	sub_82E5EB00(ctx, base);
	// 8230085C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82300860: 387F0134  addi r3, r31, 0x134
	ctx.r[3].s64 = ctx.r[31].s64 + 308;
	// 82300864: 809F00C4  lwz r4, 0xc4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 82300868: 48B5E299  bl 0x82e5eb00
	ctx.lr = 0x8230086C;
	sub_82E5EB00(ctx, base);
	// 8230086C: 817F013C  lwz r11, 0x13c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(316 as u32) ) } as u64;
	// 82300870: 3BDF013C  addi r30, r31, 0x13c
	ctx.r[30].s64 = ctx.r[31].s64 + 316;
	// 82300874: 809F00C4  lwz r4, 0xc4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 82300878: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8230087C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82300880: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82300884: 4E800421  bctrl
	ctx.lr = 0x82300888;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82300888: 817F01B0  lwz r11, 0x1b0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(432 as u32) ) } as u64;
	// 8230088C: 809F00C4  lwz r4, 0xc4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 82300890: 387F01B0  addi r3, r31, 0x1b0
	ctx.r[3].s64 = ctx.r[31].s64 + 432;
	// 82300894: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82300898: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8230089C: 4E800421  bctrl
	ctx.lr = 0x823008A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823008A0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 823008A4: 387F0224  addi r3, r31, 0x224
	ctx.r[3].s64 = ctx.r[31].s64 + 548;
	// 823008A8: 809F00C4  lwz r4, 0xc4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 823008AC: 48B5E255  bl 0x82e5eb00
	ctx.lr = 0x823008B0;
	sub_82E5EB00(ctx, base);
	// 823008B0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 823008B4: 387F022C  addi r3, r31, 0x22c
	ctx.r[3].s64 = ctx.r[31].s64 + 556;
	// 823008B8: 809F00C4  lwz r4, 0xc4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 823008BC: 48B5E245  bl 0x82e5eb00
	ctx.lr = 0x823008C0;
	sub_82E5EB00(ctx, base);
	// 823008C0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823008C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823008C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823008CC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823008D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823008D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823008D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823008D8 size=24
    let mut pc: u32 = 0x823008D8;
    'dispatch: loop {
        match pc {
            0x823008D8 => {
    //   block [0x823008D8..0x823008F0)
	// 823008D8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823008DC: 806300C4  lwz r3, 0xc4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(196 as u32) ) } as u64;
	// 823008E0: 38C003EE  li r6, 0x3ee
	ctx.r[6].s64 = 1006;
	// 823008E4: 38ABCB60  addi r5, r11, -0x34a0
	ctx.r[5].s64 = ctx.r[11].s64 + -13472;
	// 823008E8: 38840020  addi r4, r4, 0x20
	ctx.r[4].s64 = ctx.r[4].s64 + 32;
	// 823008EC: 4800517C  b 0x82305a68
	sub_82305A68(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823008F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x823008F0 size=348
    let mut pc: u32 = 0x823008F0;
    'dispatch: loop {
        match pc {
            0x823008F0 => {
    //   block [0x823008F0..0x82300A4C)
	// 823008F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823008F4: 48EA7875  bl 0x831a8168
	ctx.lr = 0x823008F8;
	sub_831A8130(ctx, base);
	// 823008F8: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 823008FC: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82300900: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82300904: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82300908: 817F0234  lwz r11, 0x234(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(564 as u32) ) } as u64;
	// 8230090C: 806B0008  lwz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82300910: 48AFDED1  bl 0x82dfe7e0
	ctx.lr = 0x82300914;
	sub_82DFE7E0(ctx, base);
	// 82300914: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82300918: 41820128  beq 0x82300a40
	if ctx.cr[0].eq {
	pc = 0x82300A40; continue 'dispatch;
	}
	// 8230091C: 807F023C  lwz r3, 0x23c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(572 as u32) ) } as u64;
	// 82300920: 488AD329  bl 0x82badc48
	ctx.lr = 0x82300924;
	sub_82BADC48(ctx, base);
	// 82300924: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82300928: 4182010C  beq 0x82300a34
	if ctx.cr[0].eq {
	pc = 0x82300A34; continue 'dispatch;
	}
	// 8230092C: 897F0264  lbz r11, 0x264(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(612 as u32) ) } as u64;
	// 82300930: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82300934: 41820100  beq 0x82300a34
	if ctx.cr[0].eq {
	pc = 0x82300A34; continue 'dispatch;
	}
	// 82300938: 809F00C4  lwz r4, 0xc4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 8230093C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82300940: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82300944: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82300948: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8230094C: 4E800421  bctrl
	ctx.lr = 0x82300950;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82300950: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82300954: 39410070  addi r10, r1, 0x70
	ctx.r[10].s64 = ctx.r[1].s64 + 112;
	// 82300958: 807F00C4  lwz r3, 0xc4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 8230095C: 396B6910  addi r11, r11, 0x6910
	ctx.r[11].s64 = ctx.r[11].s64 + 26896;
	// 82300960: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82300A50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82300A50 size=64
    let mut pc: u32 = 0x82300A50;
    'dispatch: loop {
        match pc {
            0x82300A50 => {
    //   block [0x82300A50..0x82300A90)
	// 82300A50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82300A54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82300A58: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82300A5C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82300A60: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82300A64: 808300C4  lwz r4, 0xc4(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(196 as u32) ) } as u64;
	// 82300A68: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82300A6C: 48005005  bl 0x82305a70
	ctx.lr = 0x82300A70;
	sub_82305A70(ctx, base);
	// 82300A70: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82300A74: 13E018C7  vcmpequd (lvx128) v31, v0, v3
	tmp.u32 = ctx.r[3].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82300A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82300A90 size=12
    let mut pc: u32 = 0x82300A90;
    'dispatch: loop {
        match pc {
            0x82300A90 => {
    //   block [0x82300A90..0x82300A9C)
	// 82300A90: 806300C4  lwz r3, 0xc4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(196 as u32) ) } as u64;
	// 82300A94: 38840020  addi r4, r4, 0x20
	ctx.r[4].s64 = ctx.r[4].s64 + 32;
	// 82300A98: 48008340  b 0x82308dd8
	sub_82308DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82300AA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82300AA0 size=144
    let mut pc: u32 = 0x82300AA0;
    'dispatch: loop {
        match pc {
            0x82300AA0 => {
    //   block [0x82300AA0..0x82300B30)
	// 82300AA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82300AA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82300AA8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82300AAC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82300AB0: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82300AB4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82300AB8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82300ABC: 807E00C4  lwz r3, 0xc4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(196 as u32) ) } as u64;
	// 82300AC0: 48004969  bl 0x82305428
	ctx.lr = 0x82300AC4;
	sub_82305428(ctx, base);
	// 82300AC4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82300AC8: 41820018  beq 0x82300ae0
	if ctx.cr[0].eq {
	pc = 0x82300AE0; continue 'dispatch;
	}
	// 82300ACC: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82300AD0: 396B6910  addi r11, r11, 0x6910
	ctx.r[11].s64 = ctx.r[11].s64 + 26896;
	// 82300AD4: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82300B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82300B30 size=20
    let mut pc: u32 = 0x82300B30;
    'dispatch: loop {
        match pc {
            0x82300B30 => {
    //   block [0x82300B30..0x82300B44)
	// 82300B30: 806300C4  lwz r3, 0xc4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(196 as u32) ) } as u64;
	// 82300B34: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82300B38: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 82300B3C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82300B40: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82300B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82300B48 size=164
    let mut pc: u32 = 0x82300B48;
    'dispatch: loop {
        match pc {
            0x82300B48 => {
    //   block [0x82300B48..0x82300BEC)
	// 82300B48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82300B4C: 48EA7621  bl 0x831a816c
	ctx.lr = 0x82300B50;
	sub_831A8130(ctx, base);
	// 82300B50: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82300B54: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82300B58: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82300B5C: 57AB063E  clrlwi r11, r29, 0x18
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	// 82300B60: 895F0254  lbz r10, 0x254(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(596 as u32) ) } as u64;
	// 82300B64: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82300B68: 419A007C  beq cr6, 0x82300be4
	if ctx.cr[6].eq {
	pc = 0x82300BE4; continue 'dispatch;
	}
	// 82300B6C: 895F0255  lbz r10, 0x255(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(597 as u32) ) } as u64;
	// 82300B70: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82300B74: 41820060  beq 0x82300bd4
	if ctx.cr[0].eq {
	pc = 0x82300BD4; continue 'dispatch;
	}
	// 82300B78: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82300B7C: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82300B80: 3BDF0234  addi r30, r31, 0x234
	ctx.r[30].s64 = ctx.r[31].s64 + 564;
	// 82300B84: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82300B88: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82300B8C: 808B7074  lwz r4, 0x7074(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28788 as u32) ) } as u64;
	// 82300B90: 419A0024  beq cr6, 0x82300bb4
	if ctx.cr[6].eq {
	pc = 0x82300BB4; continue 'dispatch;
	}
	// 82300B94: 4820FFC5  bl 0x82510b58
	ctx.lr = 0x82300B98;
	sub_82510B58(ctx, base);
	// 82300B98: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82300B9C: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82300BA0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82300BA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82300BA8: 808B70BC  lwz r4, 0x70bc(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28860 as u32) ) } as u64;
	// 82300BAC: 4820FFAD  bl 0x82510b58
	ctx.lr = 0x82300BB0;
	sub_82510B58(ctx, base);
	// 82300BB0: 48000030  b 0x82300be0
	pc = 0x82300BE0; continue 'dispatch;
	// 82300BB4: 4820F63D  bl 0x825101f0
	ctx.lr = 0x82300BB8;
	sub_825101F0(ctx, base);
	// 82300BB8: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82300BBC: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82300BC0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82300BC4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82300BC8: 808B70BC  lwz r4, 0x70bc(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28860 as u32) ) } as u64;
	// 82300BCC: 4820F625  bl 0x825101f0
	ctx.lr = 0x82300BD0;
	sub_825101F0(ctx, base);
	// 82300BD0: 48000010  b 0x82300be0
	pc = 0x82300BE0; continue 'dispatch;
	// 82300BD4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82300BD8: 807F0234  lwz r3, 0x234(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(564 as u32) ) } as u64;
	// 82300BDC: 48B11345  bl 0x82e11f20
	ctx.lr = 0x82300BE0;
	sub_82E11F20(ctx, base);
	// 82300BE0: 9BBF0254  stb r29, 0x254(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(596 as u32), ctx.r[29].u8 ) };
	// 82300BE4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82300BE8: 48EA75D4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82300BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82300BF0 size=72
    let mut pc: u32 = 0x82300BF0;
    'dispatch: loop {
        match pc {
            0x82300BF0 => {
    //   block [0x82300BF0..0x82300C38)
	// 82300BF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82300BF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82300BF8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82300BFC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82300C00: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82300C04: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82300C08: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82300C0C: 807F0268  lwz r3, 0x268(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(616 as u32) ) } as u64;
	// 82300C10: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82300C14: 419A0008  beq cr6, 0x82300c1c
	if ctx.cr[6].eq {
	pc = 0x82300C1C; continue 'dispatch;
	}
	// 82300C18: 4818C3F1  bl 0x8248d008
	ctx.lr = 0x82300C1C;
	sub_8248D008(ctx, base);
	// 82300C1C: 93DF0258  stw r30, 0x258(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(600 as u32), ctx.r[30].u32 ) };
	// 82300C20: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82300C24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82300C28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82300C2C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82300C30: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82300C34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82300C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82300C38 size=12
    let mut pc: u32 = 0x82300C38;
    'dispatch: loop {
        match pc {
            0x82300C38 => {
    //   block [0x82300C38..0x82300C44)
	// 82300C38: 81630258  lwz r11, 0x258(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(600 as u32) ) } as u64;
	// 82300C3C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82300C40: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82300C44(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82300C44 size=12
    let mut pc: u32 = 0x82300C44;
    'dispatch: loop {
        match pc {
            0x82300C44 => {
    //   block [0x82300C44..0x82300C50)
	// 82300C44: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82300C48: 91630258  stw r11, 0x258(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(600 as u32), ctx.r[11].u32 ) };
	// 82300C4C: 4C820020  bnelr
	if !ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82300C50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82300C50 size=8
    let mut pc: u32 = 0x82300C50;
    'dispatch: loop {
        match pc {
            0x82300C50 => {
    //   block [0x82300C50..0x82300C58)
	// 82300C50: 80630268  lwz r3, 0x268(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(616 as u32) ) } as u64;
	// 82300C54: 4818C45C  b 0x8248d0b0
	sub_8248D0B0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82300C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82300C58 size=4
    let mut pc: u32 = 0x82300C58;
    'dispatch: loop {
        match pc {
            0x82300C58 => {
    //   block [0x82300C58..0x82300C5C)
	// 82300C58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82300C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82300C60 size=20
    let mut pc: u32 = 0x82300C60;
    'dispatch: loop {
        match pc {
            0x82300C60 => {
    //   block [0x82300C60..0x82300C74)
	// 82300C60: 806300C4  lwz r3, 0xc4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(196 as u32) ) } as u64;
	// 82300C64: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82300C68: 816B0048  lwz r11, 0x48(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 82300C6C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82300C70: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82300C78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82300C78 size=104
    let mut pc: u32 = 0x82300C78;
    'dispatch: loop {
        match pc {
            0x82300C78 => {
    //   block [0x82300C78..0x82300CE0)
	// 82300C78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82300C7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82300C80: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82300C84: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82300C88: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82300C8C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82300C90: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82300C94: 807F0268  lwz r3, 0x268(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(616 as u32) ) } as u64;
	// 82300C98: 897E0018  lbz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82300C9C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82300CA0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82300CA4: 419A0014  beq cr6, 0x82300cb8
	if ctx.cr[6].eq {
	pc = 0x82300CB8; continue 'dispatch;
	}
	// 82300CA8: 4182000C  beq 0x82300cb4
	if ctx.cr[0].eq {
	pc = 0x82300CB4; continue 'dispatch;
	}
	// 82300CAC: 4818C35D  bl 0x8248d008
	ctx.lr = 0x82300CB0;
	sub_8248D008(ctx, base);
	// 82300CB0: 48000008  b 0x82300cb8
	pc = 0x82300CB8; continue 'dispatch;
	// 82300CB4: 4818C3FD  bl 0x8248d0b0
	ctx.lr = 0x82300CB8;
	sub_8248D0B0(ctx, base);
	// 82300CB8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82300CBC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82300CC0: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 82300CC4: 482108D5  bl 0x82511598
	ctx.lr = 0x82300CC8;
	sub_82511598(ctx, base);
	// 82300CC8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82300CCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82300CD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82300CD4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82300CD8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82300CDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82300CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82300CE0 size=12
    let mut pc: u32 = 0x82300CE0;
    'dispatch: loop {
        match pc {
            0x82300CE0 => {
    //   block [0x82300CE0..0x82300CEC)
	// 82300CE0: 806300C4  lwz r3, 0xc4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(196 as u32) ) } as u64;
	// 82300CE4: 38840020  addi r4, r4, 0x20
	ctx.r[4].s64 = ctx.r[4].s64 + 32;
	// 82300CE8: 480073D0  b 0x823080b8
	sub_823080B8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82300CF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82300CF0 size=92
    let mut pc: u32 = 0x82300CF0;
    'dispatch: loop {
        match pc {
            0x82300CF0 => {
    //   block [0x82300CF0..0x82300D4C)
	// 82300CF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82300CF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82300CF8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82300CFC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82300D00: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82300D04: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82300D08: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82300D0C: 83DF00C4  lwz r30, 0xc4(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 82300D10: 48D07419  bl 0x83008128
	ctx.lr = 0x82300D14;
	sub_83008128(ctx, base);
	// 82300D14: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82300D18: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82300D1C: 485148DD  bl 0x828155f8
	ctx.lr = 0x82300D20;
	sub_828155F8(ctx, base);
	// 82300D20: 807F00C4  lwz r3, 0xc4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 82300D24: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82300D28: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 82300D2C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82300D30: 4E800421  bctrl
	ctx.lr = 0x82300D34;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82300D34: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82300D38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82300D3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82300D40: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82300D44: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82300D48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82300D50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82300D50 size=120
    let mut pc: u32 = 0x82300D50;
    'dispatch: loop {
        match pc {
            0x82300D50 => {
    //   block [0x82300D50..0x82300DC8)
	// 82300D50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82300D54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82300D58: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82300D5C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82300D60: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82300D64: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82300D68: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82300D6C: 38A40020  addi r5, r4, 0x20
	ctx.r[5].s64 = ctx.r[4].s64 + 32;
	// 82300D70: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82300D74: C1AB08A8  lfs f13, 0x8a8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82300D78: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82300D7C: C00A08A4  lfs f0, 0x8a4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82300D80: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82300D84: D1A10050  stfs f13, 0x50(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82300D88: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82300D8C: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82300D90: D001005C  stfs f0, 0x5c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 82300D94: D0010060  stfs f0, 0x60(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 82300D98: D0010064  stfs f0, 0x64(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 82300D9C: D1A10068  stfs f13, 0x68(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 82300DA0: D001006C  stfs f0, 0x6c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 82300DA4: 48B7C2BD  bl 0x82e7d060
	ctx.lr = 0x82300DA8;
	sub_82E7D060(ctx, base);
	// 82300DA8: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82300DAC: 807F00C4  lwz r3, 0xc4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 82300DB0: 48004D01  bl 0x82305ab0
	ctx.lr = 0x82300DB4;
	sub_82305AB0(ctx, base);
	// 82300DB4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82300DB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82300DBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82300DC0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82300DC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82300DC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82300DC8 size=52
    let mut pc: u32 = 0x82300DC8;
    'dispatch: loop {
        match pc {
            0x82300DC8 => {
    //   block [0x82300DC8..0x82300DFC)
	// 82300DC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82300DCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82300DD0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82300DD4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82300DD8: 806300C4  lwz r3, 0xc4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(196 as u32) ) } as u64;
	// 82300DDC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82300DE0: 48E0E0F9  bl 0x8310eed8
	ctx.lr = 0x82300DE4;
	sub_8310EED8(ctx, base);
	// 82300DE4: 907F0018  stw r3, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[3].u32 ) };
	// 82300DE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82300DEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82300DF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82300DF4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82300DF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82300E00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82300E00 size=196
    let mut pc: u32 = 0x82300E00;
    'dispatch: loop {
        match pc {
            0x82300E00 => {
    //   block [0x82300E00..0x82300EC4)
	// 82300E00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82300E04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82300E08: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82300E0C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82300E10: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82300E14: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82300E18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82300E1C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82300E20: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82300E24: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82300E28: 4BFBFB11  bl 0x822c0938
	ctx.lr = 0x82300E2C;
	sub_822C0938(ctx, base);
	// 82300E2C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82300E30: 41820028  beq 0x82300e58
	if ctx.cr[0].eq {
	pc = 0x82300E58; continue 'dispatch;
	}
	// 82300E34: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82300E38: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82300E3C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82300E40: 392BCB4C  addi r9, r11, -0x34b4
	ctx.r[9].s64 = ctx.r[11].s64 + -13492;
	// 82300E44: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82300E48: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82300E4C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82300E50: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82300E54: 48000008  b 0x82300e5c
	pc = 0x82300E5C; continue 'dispatch;
	// 82300E58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82300E5C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82300E60: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82300E64: 409A0044  bne cr6, 0x82300ea8
	if !ctx.cr[6].eq {
	pc = 0x82300EA8; continue 'dispatch;
	}
	// 82300E68: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82300E6C: 419A001C  beq cr6, 0x82300e88
	if ctx.cr[6].eq {
	pc = 0x82300E88; continue 'dispatch;
	}
	// 82300E70: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82300E74: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82300E78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82300E7C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82300E80: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82300E84: 4E800421  bctrl
	ctx.lr = 0x82300E88;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82300E88: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 82300E8C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82300E90: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82300E94: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82300E98: 816B51A4  lwz r11, 0x51a4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20900 as u32) ) } as u64;
	// 82300E9C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82300EA0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82300EA4: 4BFBF15D  bl 0x822c0000
	ctx.lr = 0x82300EA8;
	sub_822C0000(ctx, base);
	// 82300EA8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82300EAC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82300EB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82300EB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82300EB8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82300EBC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82300EC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82300EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82300EC8 size=252
    let mut pc: u32 = 0x82300EC8;
    'dispatch: loop {
        match pc {
            0x82300EC8 => {
    //   block [0x82300EC8..0x82300FC4)
	// 82300EC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82300ECC: 48EA7295  bl 0x831a8160
	ctx.lr = 0x82300ED0;
	sub_831A8130(ctx, base);
	// 82300ED0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82300ED4: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82300ED8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82300EDC: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82300EE0: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82300EE4: 80FD0000  lwz r7, 0(r29)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82300EE8: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82300EEC: 7F075840  cmplw cr6, r7, r11
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82300EF0: 409A0018  bne cr6, 0x82300f08
	if !ctx.cr[6].eq {
	pc = 0x82300F08; continue 'dispatch;
	}
	// 82300EF4: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82300EF8: 815B0004  lwz r10, 4(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82300EFC: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82300F00: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82300F04: 419A0008  beq cr6, 0x82300f0c
	if ctx.cr[6].eq {
	pc = 0x82300F0C; continue 'dispatch;
	}
	// 82300F08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82300F0C: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82300F10: 40820098  bne 0x82300fa8
	if !ctx.cr[0].eq {
	pc = 0x82300FA8; continue 'dispatch;
	}
	// 82300F14: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82300F18: 81670008  lwz r11, 8(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(8 as u32) ) } as u64;
	// 82300F1C: 5549F87E  srwi r9, r10, 1
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shr(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82300F20: 554507FE  clrlwi r5, r10, 0x1f
	ctx.r[5].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 82300F24: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82300F28: 41990008  bgt cr6, 0x82300f30
	if ctx.cr[6].gt {
	pc = 0x82300F30; continue 'dispatch;
	}
	// 82300F2C: 7D2B4850  subf r9, r11, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 82300F30: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82300F34: 811C0004  lwz r8, 4(r28)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82300F38: 550BF87E  srwi r11, r8, 1
	ctx.r[11].u32 = ctx.r[8].u32.wrapping_shr(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82300F3C: 550807FE  clrlwi r8, r8, 0x1f
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0x00000001u64;
	// 82300F40: 80CA0008  lwz r6, 8(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82300F44: 7F065840  cmplw cr6, r6, r11
	ctx.cr[6].compare_u32(ctx.r[6].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82300F48: 41990008  bgt cr6, 0x82300f50
	if ctx.cr[6].gt {
	pc = 0x82300F50; continue 'dispatch;
	}
	// 82300F4C: 7D665850  subf r11, r6, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[6].s64;
	// 82300F50: 80CA0004  lwz r6, 4(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82300F54: 5524103A  slwi r4, r9, 2
	ctx.r[4].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82300F58: 80E70004  lwz r7, 4(r7)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) } as u64;
	// 82300F5C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82300F60: 550A1838  slwi r10, r8, 3
	ctx.r[10].u32 = ctx.r[8].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82300F64: 54A91838  slwi r9, r5, 3
	ctx.r[9].u32 = ctx.r[5].u32.wrapping_shl(3);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82300F68: 7D07202E  lwzx r8, r7, r4
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 82300F6C: 7D66582E  lwzx r11, r6, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[6].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82300F70: 7FE84A14  add r31, r8, r9
	ctx.r[31].u64 = ctx.r[8].u64 + ctx.r[9].u64;
	// 82300F74: 7FCB5214  add r30, r11, r10
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82300F78: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82300F7C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82300F80: 48AF2C51  bl 0x82df3bd0
	ctx.lr = 0x82300F84;
	sub_82DF3BD0(ctx, base);
	// 82300F84: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82300F88: 997E0004  stb r11, 4(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u8 ) };
	// 82300F8C: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82300F90: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82300F94: 917C0004  stw r11, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82300F98: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82300F9C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82300FA0: 917D0004  stw r11, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82300FA4: 4BFFFF40  b 0x82300ee4
	pc = 0x82300EE4; continue 'dispatch;
	// 82300FA8: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82300FAC: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82300FB0: 815C0004  lwz r10, 4(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82300FB4: 917A0000  stw r11, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82300FB8: 915A0004  stw r10, 4(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82300FBC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82300FC0: 48EA71F0  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82300FC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82300FC8 size=196
    let mut pc: u32 = 0x82300FC8;
    'dispatch: loop {
        match pc {
            0x82300FC8 => {
    //   block [0x82300FC8..0x8230108C)
	// 82300FC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82300FCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82300FD0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82300FD4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82300FD8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82300FDC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82300FE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82300FE4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82300FE8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82300FEC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82300FF0: 4BFBF949  bl 0x822c0938
	ctx.lr = 0x82300FF4;
	sub_822C0938(ctx, base);
	// 82300FF4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82300FF8: 41820028  beq 0x82301020
	if ctx.cr[0].eq {
	pc = 0x82301020; continue 'dispatch;
	}
	// 82300FFC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82301000: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82301004: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82301008: 392BCA6C  addi r9, r11, -0x3594
	ctx.r[9].s64 = ctx.r[11].s64 + -13716;
	// 8230100C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82301010: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82301014: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82301018: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8230101C: 48000008  b 0x82301024
	pc = 0x82301024; continue 'dispatch;
	// 82301020: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82301024: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82301028: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8230102C: 409A0044  bne cr6, 0x82301070
	if !ctx.cr[6].eq {
	pc = 0x82301070; continue 'dispatch;
	}
	// 82301030: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82301034: 419A001C  beq cr6, 0x82301050
	if ctx.cr[6].eq {
	pc = 0x82301050; continue 'dispatch;
	}
	// 82301038: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8230103C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82301040: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82301044: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82301048: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8230104C: 4E800421  bctrl
	ctx.lr = 0x82301050;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82301050: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 82301054: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82301058: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8230105C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82301060: 816B51A4  lwz r11, 0x51a4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20900 as u32) ) } as u64;
	// 82301064: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82301068: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8230106C: 4BFBEF95  bl 0x822c0000
	ctx.lr = 0x82301070;
	sub_822C0000(ctx, base);
	// 82301070: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82301074: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82301078: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8230107C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82301080: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82301084: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82301088: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82301090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82301090 size=196
    let mut pc: u32 = 0x82301090;
    'dispatch: loop {
        match pc {
            0x82301090 => {
    //   block [0x82301090..0x82301154)
	// 82301090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82301094: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82301098: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8230109C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823010A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823010A4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823010A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823010AC: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823010B0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823010B4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823010B8: 4BFBF881  bl 0x822c0938
	ctx.lr = 0x823010BC;
	sub_822C0938(ctx, base);
	// 823010BC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823010C0: 41820028  beq 0x823010e8
	if ctx.cr[0].eq {
	pc = 0x823010E8; continue 'dispatch;
	}
	// 823010C4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823010C8: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823010CC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823010D0: 392BCA80  addi r9, r11, -0x3580
	ctx.r[9].s64 = ctx.r[11].s64 + -13696;
	// 823010D4: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823010D8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823010DC: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823010E0: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823010E4: 48000008  b 0x823010ec
	pc = 0x823010EC; continue 'dispatch;
	// 823010E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823010EC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823010F0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823010F4: 409A0044  bne cr6, 0x82301138
	if !ctx.cr[6].eq {
	pc = 0x82301138; continue 'dispatch;
	}
	// 823010F8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823010FC: 419A001C  beq cr6, 0x82301118
	if ctx.cr[6].eq {
	pc = 0x82301118; continue 'dispatch;
	}
	// 82301100: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82301104: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82301108: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8230110C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82301110: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82301114: 4E800421  bctrl
	ctx.lr = 0x82301118;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82301118: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 8230111C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82301120: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82301124: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82301128: 816B51A4  lwz r11, 0x51a4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20900 as u32) ) } as u64;
	// 8230112C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82301130: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82301134: 4BFBEECD  bl 0x822c0000
	ctx.lr = 0x82301138;
	sub_822C0000(ctx, base);
	// 82301138: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8230113C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82301140: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82301144: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82301148: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8230114C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82301150: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82301158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82301158 size=196
    let mut pc: u32 = 0x82301158;
    'dispatch: loop {
        match pc {
            0x82301158 => {
    //   block [0x82301158..0x8230121C)
	// 82301158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8230115C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82301160: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82301164: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82301168: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8230116C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82301170: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82301174: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82301178: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8230117C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82301180: 4BFBF7B9  bl 0x822c0938
	ctx.lr = 0x82301184;
	sub_822C0938(ctx, base);
	// 82301184: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82301188: 41820028  beq 0x823011b0
	if ctx.cr[0].eq {
	pc = 0x823011B0; continue 'dispatch;
	}
	// 8230118C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82301190: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82301194: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82301198: 392BCAA8  addi r9, r11, -0x3558
	ctx.r[9].s64 = ctx.r[11].s64 + -13656;
	// 8230119C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823011A0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823011A4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823011A8: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823011AC: 48000008  b 0x823011b4
	pc = 0x823011B4; continue 'dispatch;
	// 823011B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823011B4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823011B8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823011BC: 409A0044  bne cr6, 0x82301200
	if !ctx.cr[6].eq {
	pc = 0x82301200; continue 'dispatch;
	}
	// 823011C0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823011C4: 419A001C  beq cr6, 0x823011e0
	if ctx.cr[6].eq {
	pc = 0x823011E0; continue 'dispatch;
	}
	// 823011C8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823011CC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823011D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823011D4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 823011D8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823011DC: 4E800421  bctrl
	ctx.lr = 0x823011E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823011E0: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 823011E4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823011E8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823011EC: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823011F0: 816B51A4  lwz r11, 0x51a4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20900 as u32) ) } as u64;
	// 823011F4: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823011F8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823011FC: 4BFBEE05  bl 0x822c0000
	ctx.lr = 0x82301200;
	sub_822C0000(ctx, base);
	// 82301200: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82301204: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82301208: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8230120C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82301210: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82301214: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82301218: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82301220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82301220 size=196
    let mut pc: u32 = 0x82301220;
    'dispatch: loop {
        match pc {
            0x82301220 => {
    //   block [0x82301220..0x823012E4)
	// 82301220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82301224: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82301228: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8230122C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82301230: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82301234: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82301238: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8230123C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82301240: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82301244: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82301248: 4BFBF6F1  bl 0x822c0938
	ctx.lr = 0x8230124C;
	sub_822C0938(ctx, base);
	// 8230124C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82301250: 41820028  beq 0x82301278
	if ctx.cr[0].eq {
	pc = 0x82301278; continue 'dispatch;
	}
	// 82301254: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82301258: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8230125C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82301260: 392BCABC  addi r9, r11, -0x3544
	ctx.r[9].s64 = ctx.r[11].s64 + -13636;
	// 82301264: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82301268: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8230126C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82301270: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82301274: 48000008  b 0x8230127c
	pc = 0x8230127C; continue 'dispatch;
	// 82301278: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8230127C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82301280: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82301284: 409A0044  bne cr6, 0x823012c8
	if !ctx.cr[6].eq {
	pc = 0x823012C8; continue 'dispatch;
	}
	// 82301288: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8230128C: 419A001C  beq cr6, 0x823012a8
	if ctx.cr[6].eq {
	pc = 0x823012A8; continue 'dispatch;
	}
	// 82301290: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82301294: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82301298: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8230129C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 823012A0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823012A4: 4E800421  bctrl
	ctx.lr = 0x823012A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823012A8: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 823012AC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823012B0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823012B4: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823012B8: 816B51A4  lwz r11, 0x51a4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20900 as u32) ) } as u64;
	// 823012BC: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823012C0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823012C4: 4BFBED3D  bl 0x822c0000
	ctx.lr = 0x823012C8;
	sub_822C0000(ctx, base);
	// 823012C8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823012CC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823012D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823012D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823012D8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823012DC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823012E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823012E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823012E8 size=196
    let mut pc: u32 = 0x823012E8;
    'dispatch: loop {
        match pc {
            0x823012E8 => {
    //   block [0x823012E8..0x823013AC)
	// 823012E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823012EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823012F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823012F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823012F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823012FC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82301300: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82301304: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82301308: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8230130C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82301310: 4BFBF629  bl 0x822c0938
	ctx.lr = 0x82301314;
	sub_822C0938(ctx, base);
	// 82301314: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82301318: 41820028  beq 0x82301340
	if ctx.cr[0].eq {
	pc = 0x82301340; continue 'dispatch;
	}
	// 8230131C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82301320: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82301324: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82301328: 392BCAD0  addi r9, r11, -0x3530
	ctx.r[9].s64 = ctx.r[11].s64 + -13616;
	// 8230132C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82301330: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82301334: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82301338: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8230133C: 48000008  b 0x82301344
	pc = 0x82301344; continue 'dispatch;
	// 82301340: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82301344: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82301348: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8230134C: 409A0044  bne cr6, 0x82301390
	if !ctx.cr[6].eq {
	pc = 0x82301390; continue 'dispatch;
	}
	// 82301350: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82301354: 419A001C  beq cr6, 0x82301370
	if ctx.cr[6].eq {
	pc = 0x82301370; continue 'dispatch;
	}
	// 82301358: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8230135C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82301360: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82301364: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82301368: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8230136C: 4E800421  bctrl
	ctx.lr = 0x82301370;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82301370: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 82301374: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82301378: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8230137C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82301380: 816B51A4  lwz r11, 0x51a4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20900 as u32) ) } as u64;
	// 82301384: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82301388: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8230138C: 4BFBEC75  bl 0x822c0000
	ctx.lr = 0x82301390;
	sub_822C0000(ctx, base);
	// 82301390: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82301394: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82301398: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8230139C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823013A0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823013A4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823013A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823013B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823013B0 size=196
    let mut pc: u32 = 0x823013B0;
    'dispatch: loop {
        match pc {
            0x823013B0 => {
    //   block [0x823013B0..0x82301474)
	// 823013B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823013B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823013B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823013BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823013C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823013C4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823013C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823013CC: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823013D0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823013D4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823013D8: 4BFBF561  bl 0x822c0938
	ctx.lr = 0x823013DC;
	sub_822C0938(ctx, base);
	// 823013DC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823013E0: 41820028  beq 0x82301408
	if ctx.cr[0].eq {
	pc = 0x82301408; continue 'dispatch;
	}
	// 823013E4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823013E8: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823013EC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823013F0: 392BCAE4  addi r9, r11, -0x351c
	ctx.r[9].s64 = ctx.r[11].s64 + -13596;
	// 823013F4: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823013F8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823013FC: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82301400: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82301404: 48000008  b 0x8230140c
	pc = 0x8230140C; continue 'dispatch;
	// 82301408: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8230140C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82301410: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82301414: 409A0044  bne cr6, 0x82301458
	if !ctx.cr[6].eq {
	pc = 0x82301458; continue 'dispatch;
	}
	// 82301418: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8230141C: 419A001C  beq cr6, 0x82301438
	if ctx.cr[6].eq {
	pc = 0x82301438; continue 'dispatch;
	}
	// 82301420: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82301424: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82301428: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8230142C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82301430: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82301434: 4E800421  bctrl
	ctx.lr = 0x82301438;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82301438: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 8230143C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82301440: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82301444: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82301448: 816B51A4  lwz r11, 0x51a4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20900 as u32) ) } as u64;
	// 8230144C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82301450: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82301454: 4BFBEBAD  bl 0x822c0000
	ctx.lr = 0x82301458;
	sub_822C0000(ctx, base);
	// 82301458: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8230145C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82301460: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82301464: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82301468: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8230146C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82301470: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82301478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82301478 size=196
    let mut pc: u32 = 0x82301478;
    'dispatch: loop {
        match pc {
            0x82301478 => {
    //   block [0x82301478..0x8230153C)
	// 82301478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8230147C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82301480: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82301484: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82301488: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8230148C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82301490: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82301494: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82301498: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8230149C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823014A0: 4BFBF499  bl 0x822c0938
	ctx.lr = 0x823014A4;
	sub_822C0938(ctx, base);
	// 823014A4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823014A8: 41820028  beq 0x823014d0
	if ctx.cr[0].eq {
	pc = 0x823014D0; continue 'dispatch;
	}
	// 823014AC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823014B0: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823014B4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823014B8: 392BCAF8  addi r9, r11, -0x3508
	ctx.r[9].s64 = ctx.r[11].s64 + -13576;
	// 823014BC: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823014C0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823014C4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823014C8: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823014CC: 48000008  b 0x823014d4
	pc = 0x823014D4; continue 'dispatch;
	// 823014D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823014D4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823014D8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823014DC: 409A0044  bne cr6, 0x82301520
	if !ctx.cr[6].eq {
	pc = 0x82301520; continue 'dispatch;
	}
	// 823014E0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823014E4: 419A001C  beq cr6, 0x82301500
	if ctx.cr[6].eq {
	pc = 0x82301500; continue 'dispatch;
	}
	// 823014E8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823014EC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823014F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823014F4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 823014F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823014FC: 4E800421  bctrl
	ctx.lr = 0x82301500;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82301500: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 82301504: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82301508: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8230150C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82301510: 816B51A4  lwz r11, 0x51a4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20900 as u32) ) } as u64;
	// 82301514: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82301518: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8230151C: 4BFBEAE5  bl 0x822c0000
	ctx.lr = 0x82301520;
	sub_822C0000(ctx, base);
	// 82301520: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82301524: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82301528: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8230152C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82301530: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82301534: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82301538: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82301540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82301540 size=196
    let mut pc: u32 = 0x82301540;
    'dispatch: loop {
        match pc {
            0x82301540 => {
    //   block [0x82301540..0x82301604)
	// 82301540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82301544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82301548: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8230154C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82301550: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82301554: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82301558: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8230155C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82301560: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82301564: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82301568: 4BFBF3D1  bl 0x822c0938
	ctx.lr = 0x8230156C;
	sub_822C0938(ctx, base);
	// 8230156C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82301570: 41820028  beq 0x82301598
	if ctx.cr[0].eq {
	pc = 0x82301598; continue 'dispatch;
	}
	// 82301574: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82301578: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8230157C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82301580: 392BCB0C  addi r9, r11, -0x34f4
	ctx.r[9].s64 = ctx.r[11].s64 + -13556;
	// 82301584: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82301588: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8230158C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82301590: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82301594: 48000008  b 0x8230159c
	pc = 0x8230159C; continue 'dispatch;
	// 82301598: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8230159C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823015A0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823015A4: 409A0044  bne cr6, 0x823015e8
	if !ctx.cr[6].eq {
	pc = 0x823015E8; continue 'dispatch;
	}
	// 823015A8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823015AC: 419A001C  beq cr6, 0x823015c8
	if ctx.cr[6].eq {
	pc = 0x823015C8; continue 'dispatch;
	}
	// 823015B0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823015B4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823015B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823015BC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 823015C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823015C4: 4E800421  bctrl
	ctx.lr = 0x823015C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823015C8: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 823015CC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823015D0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823015D4: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823015D8: 816B51A4  lwz r11, 0x51a4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20900 as u32) ) } as u64;
	// 823015DC: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823015E0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823015E4: 4BFBEA1D  bl 0x822c0000
	ctx.lr = 0x823015E8;
	sub_822C0000(ctx, base);
	// 823015E8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823015EC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823015F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823015F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823015F8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823015FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82301600: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82301608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82301608 size=196
    let mut pc: u32 = 0x82301608;
    'dispatch: loop {
        match pc {
            0x82301608 => {
    //   block [0x82301608..0x823016CC)
	// 82301608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8230160C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82301610: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82301614: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82301618: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8230161C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82301620: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82301624: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82301628: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8230162C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82301630: 4BFBF309  bl 0x822c0938
	ctx.lr = 0x82301634;
	sub_822C0938(ctx, base);
	// 82301634: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82301638: 41820028  beq 0x82301660
	if ctx.cr[0].eq {
	pc = 0x82301660; continue 'dispatch;
	}
	// 8230163C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82301640: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82301644: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82301648: 392BCB20  addi r9, r11, -0x34e0
	ctx.r[9].s64 = ctx.r[11].s64 + -13536;
	// 8230164C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82301650: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82301654: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82301658: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8230165C: 48000008  b 0x82301664
	pc = 0x82301664; continue 'dispatch;
	// 82301660: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82301664: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82301668: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8230166C: 409A0044  bne cr6, 0x823016b0
	if !ctx.cr[6].eq {
	pc = 0x823016B0; continue 'dispatch;
	}
	// 82301670: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82301674: 419A001C  beq cr6, 0x82301690
	if ctx.cr[6].eq {
	pc = 0x82301690; continue 'dispatch;
	}
	// 82301678: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8230167C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82301680: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82301684: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82301688: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8230168C: 4E800421  bctrl
	ctx.lr = 0x82301690;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82301690: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 82301694: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82301698: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8230169C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823016A0: 816B51A4  lwz r11, 0x51a4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20900 as u32) ) } as u64;
	// 823016A4: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823016A8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823016AC: 4BFBE955  bl 0x822c0000
	ctx.lr = 0x823016B0;
	sub_822C0000(ctx, base);
	// 823016B0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823016B4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823016B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823016BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823016C0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823016C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823016C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823016D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x823016D0 size=256
    let mut pc: u32 = 0x823016D0;
    'dispatch: loop {
        match pc {
            0x823016D0 => {
    //   block [0x823016D0..0x823017D0)
	// 823016D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823016D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823016D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823016DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823016E0: DBA1FFD0  stfd f29, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[29].u64 ) };
	// 823016E4: DBC1FFD8  stfd f30, -0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[30].u64 ) };
	// 823016E8: DBE1FFE0  stfd f31, -0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 823016EC: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823016F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823016F4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 823016F8: 817F0258  lwz r11, 0x258(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(600 as u32) ) } as u64;
	// 823016FC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82301700: 40990018  ble cr6, 0x82301718
	if !ctx.cr[6].gt {
	pc = 0x82301718; continue 'dispatch;
	}
	// 82301704: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82301708: 917F0258  stw r11, 0x258(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(600 as u32), ctx.r[11].u32 ) };
	// 8230170C: 4082000C  bne 0x82301718
	if !ctx.cr[0].eq {
	pc = 0x82301718; continue 'dispatch;
	}
	// 82301710: 807F0268  lwz r3, 0x268(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(616 as u32) ) } as u64;
	// 82301714: 4818B99D  bl 0x8248d0b0
	ctx.lr = 0x82301718;
	sub_8248D0B0(ctx, base);
	// 82301718: 897F0256  lbz r11, 0x256(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(598 as u32) ) } as u64;
	// 8230171C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82301720: 41820078  beq 0x82301798
	if ctx.cr[0].eq {
	pc = 0x82301798; continue 'dispatch;
	}
	// 82301724: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82301728: C01E0000  lfs f0, 0(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8230172C: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82301730: 389E0008  addi r4, r30, 8
	ctx.r[4].s64 = ctx.r[30].s64 + 8;
	// 82301734: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82301738: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8230173C: 48AF24C5  bl 0x82df3c00
	ctx.lr = 0x82301740;
	sub_82DF3C00(ctx, base);
	// 82301740: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82301744: C3CB08A4  lfs f30, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 82301748: C3E10050  lfs f31, 0x50(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8230174C: FF1FF000  fcmpu cr6, f31, f30
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[30].f64);
	// 82301750: 4099003C  ble cr6, 0x8230178c
	if !ctx.cr[6].gt {
	pc = 0x8230178C; continue 'dispatch;
	}
	// 82301754: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82301758: C3ABC0AC  lfs f29, -0x3f54(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16212 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 8230175C: FF1FE800  fcmpu cr6, f31, f29
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[29].f64);
	// 82301760: 4098000C  bge cr6, 0x8230176c
	if !ctx.cr[6].lt {
	pc = 0x8230176C; continue 'dispatch;
	}
	// 82301764: D3E10050  stfs f31, 0x50(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82301768: 48000008  b 0x82301770
	pc = 0x82301770; continue 'dispatch;
	// 8230176C: D3A10050  stfs f29, 0x50(r1)
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82301770: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82301774: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82301778: 4BFFEF21  bl 0x82300698
	ctx.lr = 0x8230177C;
	sub_82300698(ctx, base);
	// 8230177C: C0010050  lfs f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82301780: EFFF0028  fsubs f31, f31, f0
	ctx.f[31].f64 = (((ctx.f[31].f64 - ctx.f[0].f64) as f32) as f64);
	// 82301784: FF1FF000  fcmpu cr6, f31, f30
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[30].f64);
	// 82301788: 4199FFD4  bgt cr6, 0x8230175c
	if ctx.cr[6].gt {
	pc = 0x8230175C; continue 'dispatch;
	}
	// 8230178C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82301790: 48AF1C99  bl 0x82df3428
	ctx.lr = 0x82301794;
	sub_82DF3428(ctx, base);
	// 82301794: 48000010  b 0x823017a4
	pc = 0x823017A4; continue 'dispatch;
	// 82301798: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8230179C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823017A0: 4BFFEEF9  bl 0x82300698
	ctx.lr = 0x823017A4;
	sub_82300698(ctx, base);
	// 823017A4: 807F00C4  lwz r3, 0xc4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 823017A8: 48767B71  bl 0x82a69318
	ctx.lr = 0x823017AC;
	sub_82A69318(ctx, base);
	// 823017AC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 823017B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823017B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823017B8: CBA1FFD0  lfd f29, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[29].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 823017BC: CBC1FFD8  lfd f30, -0x28(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 823017C0: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 823017C4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823017C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823017CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823017D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823017D0 size=180
    let mut pc: u32 = 0x823017D0;
    'dispatch: loop {
        match pc {
            0x823017D0 => {
    //   block [0x823017D0..0x82301884)
	// 823017D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823017D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823017D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823017DC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823017E0: 3BE300CC  addi r31, r3, 0xcc
	ctx.r[31].s64 = ctx.r[3].s64 + 204;
	// 823017E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823017E8: 48B57F39  bl 0x82e59720
	ctx.lr = 0x823017EC;
	sub_82E59720(ctx, base);
	// 823017EC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823017F0: 41820044  beq 0x82301834
	if ctx.cr[0].eq {
	pc = 0x82301834; continue 'dispatch;
	}
	// 823017F4: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 823017F8: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 823017FC: 3BEB2638  addi r31, r11, 0x2638
	ctx.r[31].s64 = ctx.r[11].s64 + 9784;
	// 82301800: 816A263C  lwz r11, 0x263c(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(9788 as u32) ) } as u64;
	// 82301804: 556907FF  clrlwi. r9, r11, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82301808: 40820064  bne 0x8230186c
	if !ctx.cr[0].eq {
	pc = 0x8230186C; continue 'dispatch;
	}
	// 8230180C: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 82301810: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82301814: 916A263C  stw r11, 0x263c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(9788 as u32), ctx.r[11].u32 ) };
	// 82301818: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8230181C: 38899BC9  addi r4, r9, -0x6437
	ctx.r[4].s64 = ctx.r[9].s64 + -25655;
	// 82301820: 48AF21E9  bl 0x82df3a08
	ctx.lr = 0x82301824;
	sub_82DF3A08(ctx, base);
	// 82301824: 3D608324  lis r11, -0x7cdc
	ctx.r[11].s64 = -2094792704;
	// 82301828: 386BEBF0  addi r3, r11, -0x1410
	ctx.r[3].s64 = ctx.r[11].s64 + -5136;
	// 8230182C: 48EA6CAD  bl 0x831a84d8
	ctx.lr = 0x82301830;
	sub_831A84D8(ctx, base);
	// 82301830: 4800003C  b 0x8230186c
	pc = 0x8230186C; continue 'dispatch;
	// 82301834: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82301838: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8230183C: 48B58CE5  bl 0x82e5a520
	ctx.lr = 0x82301840;
	sub_82E5A520(ctx, base);
	// 82301840: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82301844: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82301848: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8230184C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82301850: 4E800421  bctrl
	ctx.lr = 0x82301854;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82301854: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82301858: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8230185C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82301860: 419A000C  beq cr6, 0x8230186c
	if ctx.cr[6].eq {
	pc = 0x8230186C; continue 'dispatch;
	}
	// 82301864: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82301868: 4BFBF029  bl 0x822c0890
	ctx.lr = 0x8230186C;
	sub_822C0890(ctx, base);
	// 8230186C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82301870: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82301874: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82301878: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8230187C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82301880: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82301888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82301888 size=76
    let mut pc: u32 = 0x82301888;
    'dispatch: loop {
        match pc {
            0x82301888 => {
    //   block [0x82301888..0x823018D4)
	// 82301888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8230188C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82301890: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82301894: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82301898: 388300CC  addi r4, r3, 0xcc
	ctx.r[4].s64 = ctx.r[3].s64 + 204;
	// 8230189C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823018A0: 48B58C81  bl 0x82e5a520
	ctx.lr = 0x823018A4;
	sub_82E5A520(ctx, base);
	// 823018A4: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 823018A8: 83E30000  lwz r31, 0(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 823018AC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823018B0: 419A000C  beq cr6, 0x823018bc
	if ctx.cr[6].eq {
	pc = 0x823018BC; continue 'dispatch;
	}
	// 823018B4: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 823018B8: 4BFBEFD9  bl 0x822c0890
	ctx.lr = 0x823018BC;
	sub_822C0890(ctx, base);
	// 823018BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823018C0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823018C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823018C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823018CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823018D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823018D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x823018D8 size=116
    let mut pc: u32 = 0x823018D8;
    'dispatch: loop {
        match pc {
            0x823018D8 => {
    //   block [0x823018D8..0x8230194C)
	// 823018D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823018DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823018E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823018E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823018E8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823018EC: 806B00C4  lwz r3, 0xc4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(196 as u32) ) } as u64;
	// 823018F0: 83EB0234  lwz r31, 0x234(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(564 as u32) ) } as u64;
	// 823018F4: 4800415D  bl 0x82305a50
	ctx.lr = 0x823018F8;
	sub_82305A50(ctx, base);
	// 823018F8: 815F0084  lwz r10, 0x84(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 823018FC: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82301900: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 82301904: 39000020  li r8, 0x20
	ctx.r[8].s64 = 32;
	// 82301908: 39200030  li r9, 0x30
	ctx.r[9].s64 = 48;
	// 8230190C: 396A0010  addi r11, r10, 0x10
	ctx.r[11].s64 = ctx.r[10].s64 + 16;
	// 82301910: 13E03407  vcmpneb. (lvlx128) v31, v0, v6
	tmp.u32 = ctx.r[6].u32;
	// load shuffled into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82301914: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 82301918: 13C73407  vcmpneb. (lvlx128) v30, v7, v6
	tmp.u32 = ctx.r[7].u32 + ctx.r[6].u32;
	// load shuffled into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 8230191C: 13A83407  vcmpneb. (lvlx128) v29, v8, v6
	tmp.u32 = ctx.r[8].u32 + ctx.r[6].u32;
	// load shuffled into ctx.v[61] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82301920: 13893407  vcmpneb. (lvlx128) v28, v9, v6
	tmp.u32 = ctx.r[9].u32 + ctx.r[6].u32;
	// load shuffled into ctx.v[60] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82301950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82301950 size=72
    let mut pc: u32 = 0x82301950;
    'dispatch: loop {
        match pc {
            0x82301950 => {
    //   block [0x82301950..0x82301998)
	// 82301950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82301954: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82301958: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8230195C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82301960: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82301964: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82301968: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8230196C: 807F0244  lwz r3, 0x244(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(580 as u32) ) } as u64;
	// 82301970: 4BFFC379  bl 0x822fdce8
	ctx.lr = 0x82301974;
	sub_822FDCE8(ctx, base);
	// 82301974: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82301978: C03E0000  lfs f1, 0(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8230197C: 4BFFEF75  bl 0x823008f0
	ctx.lr = 0x82301980;
	sub_823008F0(ctx, base);
	// 82301980: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82301984: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82301988: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8230198C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82301990: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82301994: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82301998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82301998 size=132
    let mut pc: u32 = 0x82301998;
    'dispatch: loop {
        match pc {
            0x82301998 => {
    //   block [0x82301998..0x82301A1C)
	// 82301998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8230199C: 48EA67D1  bl 0x831a816c
	ctx.lr = 0x823019A0;
	sub_831A8130(ctx, base);
	// 823019A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823019A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823019A8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 823019AC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 823019B0: 817F00C4  lwz r11, 0xc4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 823019B4: 806B01FC  lwz r3, 0x1fc(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(508 as u32) ) } as u64;
	// 823019B8: 481593E9  bl 0x8245ada0
	ctx.lr = 0x823019BC;
	sub_8245ADA0(ctx, base);
	// 823019BC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823019C0: 41820054  beq 0x82301a14
	if ctx.cr[0].eq {
	pc = 0x82301A14; continue 'dispatch;
	}
	// 823019C4: 481F18D5  bl 0x824f3298
	ctx.lr = 0x823019C8;
	sub_824F3298(ctx, base);
	// 823019C8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823019CC: 41820048  beq 0x82301a14
	if ctx.cr[0].eq {
	pc = 0x82301A14; continue 'dispatch;
	}
	// 823019D0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823019D4: 809F00C4  lwz r4, 0xc4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 823019D8: 83BF024C  lwz r29, 0x24c(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(588 as u32) ) } as u64;
	// 823019DC: 48004095  bl 0x82305a70
	ctx.lr = 0x823019E0;
	sub_82305A70(ctx, base);
	// 823019E0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 823019E4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823019E8: 488451C1  bl 0x82b46ba8
	ctx.lr = 0x823019EC;
	sub_82B46BA8(ctx, base);
	// 823019EC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823019F0: 83DF024C  lwz r30, 0x24c(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(588 as u32) ) } as u64;
	// 823019F4: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 823019F8: 48AF17B9  bl 0x82df31b0
	ctx.lr = 0x823019FC;
	sub_82DF31B0(ctx, base);
	// 823019FC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82301A00: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82301A04: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82301A08: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82301A0C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82301A10: 4E800421  bctrl
	ctx.lr = 0x82301A14;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82301A14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82301A18: 48EA67A4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82301A20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82301A20 size=308
    let mut pc: u32 = 0x82301A20;
    'dispatch: loop {
        match pc {
            0x82301A20 => {
    //   block [0x82301A20..0x82301B54)
	// 82301A20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82301A24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82301A28: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82301A2C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82301A30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82301A34: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82301A38: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82301A3C: 48B54995  bl 0x82e563d0
	ctx.lr = 0x82301A40;
	sub_82E563D0(ctx, base);
	// 82301A40: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82301A44: 408200F8  bne 0x82301b3c
	if !ctx.cr[0].eq {
	pc = 0x82301B3C; continue 'dispatch;
	}
	// 82301A48: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82301A4C: 556B07BD  rlwinm. r11, r11, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82301A50: 41820020  beq 0x82301a70
	if ctx.cr[0].eq {
	pc = 0x82301A70; continue 'dispatch;
	}
	// 82301A54: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82301A58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82301A5C: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 82301A60: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82301A64: 4E800421  bctrl
	ctx.lr = 0x82301A68;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82301A68: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82301A6C: 418200D0  beq 0x82301b3c
	if ctx.cr[0].eq {
	pc = 0x82301B3C; continue 'dispatch;
	}
	// 82301A70: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82301A74: 807F00C4  lwz r3, 0xc4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 82301A78: 556A07FF  clrlwi. r10, r11, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82301A7C: 41820048  beq 0x82301ac4
	if ctx.cr[0].eq {
	pc = 0x82301AC4; continue 'dispatch;
	}
	// 82301A80: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82301A84: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82301A88: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82301A8C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82301A90: 4E800421  bctrl
	ctx.lr = 0x82301A94;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82301A94: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82301A98: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82301A9C: 4820DA2D  bl 0x8250f4c8
	ctx.lr = 0x82301AA0;
	sub_8250F4C8(ctx, base);
	// 82301AA0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82301AA4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82301AA8: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 82301AAC: 409A0008  bne cr6, 0x82301ab4
	if !ctx.cr[6].eq {
	pc = 0x82301AB4; continue 'dispatch;
	}
	// 82301AB0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82301AB4: 48206C25  bl 0x825086d8
	ctx.lr = 0x82301AB8;
	sub_825086D8(ctx, base);
	// 82301AB8: 4815A821  bl 0x8245c2d8
	ctx.lr = 0x82301ABC;
	sub_8245C2D8(ctx, base);
	// 82301ABC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82301AC0: 4800006C  b 0x82301b2c
	pc = 0x82301B2C; continue 'dispatch;
	// 82301AC4: 556B077B  rlwinm. r11, r11, 0, 0x1d, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82301AC8: 41820024  beq 0x82301aec
	if ctx.cr[0].eq {
	pc = 0x82301AEC; continue 'dispatch;
	}
	// 82301ACC: 48004D7D  bl 0x82306848
	ctx.lr = 0x82301AD0;
	sub_82306848(ctx, base);
	// 82301AD0: 807F00C4  lwz r3, 0xc4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 82301AD4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82301AD8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82301ADC: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82301AE0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82301AE4: 4E800421  bctrl
	ctx.lr = 0x82301AE8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82301AE8: 48000048  b 0x82301b30
	pc = 0x82301B30; continue 'dispatch;
	// 82301AEC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82301AF0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82301AF4: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82301AF8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82301AFC: 4E800421  bctrl
	ctx.lr = 0x82301B00;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82301B00: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82301B04: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82301B08: 4820D9C1  bl 0x8250f4c8
	ctx.lr = 0x82301B0C;
	sub_8250F4C8(ctx, base);
	// 82301B0C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82301B10: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82301B14: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 82301B18: 409A0008  bne cr6, 0x82301b20
	if !ctx.cr[6].eq {
	pc = 0x82301B20; continue 'dispatch;
	}
	// 82301B1C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82301B20: 48206BB9  bl 0x825086d8
	ctx.lr = 0x82301B24;
	sub_825086D8(ctx, base);
	// 82301B24: 4815A7B5  bl 0x8245c2d8
	ctx.lr = 0x82301B28;
	sub_8245C2D8(ctx, base);
	// 82301B28: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82301B2C: 48AF0165  bl 0x82df1c90
	ctx.lr = 0x82301B30;
	sub_82DF1C90(ctx, base);
	// 82301B30: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82301B34: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82301B38: 4BFFF0B9  bl 0x82300bf0
	ctx.lr = 0x82301B3C;
	sub_82300BF0(ctx, base);
	// 82301B3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82301B40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82301B44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82301B48: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82301B4C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82301B50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82301B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82301B58 size=52
    let mut pc: u32 = 0x82301B58;
    'dispatch: loop {
        match pc {
            0x82301B58 => {
    //   block [0x82301B58..0x82301B8C)
	// 82301B58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82301B5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82301B60: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82301B64: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82301B68: 80840234  lwz r4, 0x234(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(564 as u32) ) } as u64;
	// 82301B6C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82301B70: 48B134C1  bl 0x82e15030
	ctx.lr = 0x82301B74;
	sub_82E15030(ctx, base);
	// 82301B74: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82301B78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82301B7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82301B80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82301B84: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82301B88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82301B90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82301B90 size=220
    let mut pc: u32 = 0x82301B90;
    'dispatch: loop {
        match pc {
            0x82301B90 => {
    //   block [0x82301B90..0x82301C6C)
	// 82301B90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82301B94: 48EA65D9  bl 0x831a816c
	ctx.lr = 0x82301B98;
	sub_831A8130(ctx, base);
	// 82301B98: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82301B9C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82301BA0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82301BA4: 807E00C4  lwz r3, 0xc4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(196 as u32) ) } as u64;
	// 82301BA8: 48003881  bl 0x82305428
	ctx.lr = 0x82301BAC;
	sub_82305428(ctx, base);
	// 82301BAC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82301BB0: 41820030  beq 0x82301be0
	if ctx.cr[0].eq {
	pc = 0x82301BE0; continue 'dispatch;
	}
	// 82301BB4: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82301BB8: 394BBA80  addi r10, r11, -0x4580
	ctx.r[10].s64 = ctx.r[11].s64 + -17792;
	// 82301BBC: C00BBA80  lfs f0, -0x4580(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17792 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82301BC0: D01F0000  stfs f0, 0(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82301BC4: C00A0004  lfs f0, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82301BC8: C1AA0008  lfs f13, 8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82301BCC: C18A000C  lfs f12, 0xc(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82301BD0: D01F0004  stfs f0, 4(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82301BD4: D1BF0008  stfs f13, 8(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82301BD8: D19F000C  stfs f12, 0xc(r31)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82301BDC: 48000084  b 0x82301c60
	pc = 0x82301C60; continue 'dispatch;
	// 82301BE0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82301BE4: 809E023C  lwz r4, 0x23c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(572 as u32) ) } as u64;
	// 82301BE8: 83DE00C4  lwz r30, 0xc4(r30)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(196 as u32) ) } as u64;
	// 82301BEC: 488AC0A5  bl 0x82badc90
	ctx.lr = 0x82301BF0;
	sub_82BADC90(ctx, base);
	// 82301BF0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82301BF4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82301BF8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82301BFC: 48003EBD  bl 0x82305ab8
	ctx.lr = 0x82301C00;
	sub_82305AB8(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82301C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82301C70 size=132
    let mut pc: u32 = 0x82301C70;
    'dispatch: loop {
        match pc {
            0x82301C70 => {
    //   block [0x82301C70..0x82301CF4)
	// 82301C70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82301C74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82301C78: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82301C7C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82301C80: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82301C84: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82301C88: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82301C8C: 419A0054  beq cr6, 0x82301ce0
	if ctx.cr[6].eq {
	pc = 0x82301CE0; continue 'dispatch;
	}
	// 82301C90: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82301C94: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82301C98: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82301C9C: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 82301CA0: 554BF87E  srwi r11, r10, 1
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shr(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82301CA4: 7F095840  cmplw cr6, r9, r11
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82301CA8: 41990008  bgt cr6, 0x82301cb0
	if ctx.cr[6].gt {
	pc = 0x82301CB0; continue 'dispatch;
	}
	// 82301CAC: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 82301CB0: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82301CB4: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82301CB8: 554A1F38  rlwinm r10, r10, 3, 0x1c, 0x1c
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x1FFFFFFFu64;
	// 82301CBC: 7D6B482E  lwzx r11, r11, r9
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82301CC0: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82301CC4: 48AF1765  bl 0x82df3428
	ctx.lr = 0x82301CC8;
	sub_82DF3428(ctx, base);
	// 82301CC8: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82301CCC: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82301CD0: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82301CD4: 4082000C  bne 0x82301ce0
	if !ctx.cr[0].eq {
	pc = 0x82301CE0; continue 'dispatch;
	}
	// 82301CD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82301CDC: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82301CE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82301CE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82301CE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82301CEC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82301CF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82301CF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82301CF8 size=116
    let mut pc: u32 = 0x82301CF8;
    'dispatch: loop {
        match pc {
            0x82301CF8 => {
    //   block [0x82301CF8..0x82301D6C)
	// 82301CF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82301CFC: 48EA6471  bl 0x831a816c
	ctx.lr = 0x82301D00;
	sub_831A8130(ctx, base);
	// 82301D00: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82301D04: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82301D08: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82301D0C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82301D10: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82301D14: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82301D18: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 82301D1C: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82301D20: 388A08B0  addi r4, r10, 0x8b0
	ctx.r[4].s64 = ctx.r[10].s64 + 2224;
	// 82301D24: 38A0002D  li r5, 0x2d
	ctx.r[5].s64 = 45;
	// 82301D28: 48AF03A1  bl 0x82df20c8
	ctx.lr = 0x82301D2C;
	sub_82DF20C8(ctx, base);
	// 82301D2C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82301D30: 41820008  beq 0x82301d38
	if ctx.cr[0].eq {
	pc = 0x82301D38; continue 'dispatch;
	}
	// 82301D34: 93E30000  stw r31, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82301D38: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82301D3C: 41820008  beq 0x82301d44
	if ctx.cr[0].eq {
	pc = 0x82301D44; continue 'dispatch;
	}
	// 82301D40: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82301D44: 35630008  addic. r11, r3, 8
	ctx.xer.ca = (ctx.r[3].u32 > (!(8 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82301D48: 4182001C  beq 0x82301d64
	if ctx.cr[0].eq {
	pc = 0x82301D64; continue 'dispatch;
	}
	// 82301D4C: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82301D50: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82301D54: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82301D58: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82301D5C: 815D0008  lwz r10, 8(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82301D60: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82301D64: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82301D68: 48EA6454  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82301D70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82301D70 size=116
    let mut pc: u32 = 0x82301D70;
    'dispatch: loop {
        match pc {
            0x82301D70 => {
    //   block [0x82301D70..0x82301DE4)
	// 82301D70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82301D74: 48EA63F5  bl 0x831a8168
	ctx.lr = 0x82301D78;
	sub_831A8130(ctx, base);
	// 82301D78: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82301D7C: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82301D80: 81260004  lwz r9, 4(r6)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) } as u64;
	// 82301D84: 80E50000  lwz r7, 0(r5)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82301D88: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82301D8C: 81460000  lwz r10, 0(r6)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82301D90: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 82301D94: 83C50004  lwz r30, 4(r5)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 82301D98: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82301D9C: 83A40000  lwz r29, 0(r4)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82301DA0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82301DA4: 83840004  lwz r28, 4(r4)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82301DA8: 38810068  addi r4, r1, 0x68
	ctx.r[4].s64 = ctx.r[1].s64 + 104;
	// 82301DAC: 990B0000  stb r8, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u8 ) };
	// 82301DB0: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82301DB4: 90E10060  stw r7, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[7].u32 ) };
	// 82301DB8: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82301DBC: 93C10064  stw r30, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[30].u32 ) };
	// 82301DC0: 93A10068  stw r29, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[29].u32 ) };
	// 82301DC4: 9381006C  stw r28, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[28].u32 ) };
	// 82301DC8: 89010050  lbz r8, 0x50(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82301DCC: 88E10050  lbz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82301DD0: 89210050  lbz r9, 0x50(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82301DD4: 4BFFF0F5  bl 0x82300ec8
	ctx.lr = 0x82301DD8;
	sub_82300EC8(ctx, base);
	// 82301DD8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82301DDC: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82301DE0: 48EA63D8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82301DE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82301DE8 size=244
    let mut pc: u32 = 0x82301DE8;
    'dispatch: loop {
        match pc {
            0x82301DE8 => {
    //   block [0x82301DE8..0x82301EDC)
	// 82301DE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82301DEC: 48EA6375  bl 0x831a8160
	ctx.lr = 0x82301DF0;
	sub_831A8130(ctx, base);
	// 82301DF0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82301DF4: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82301DF8: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82301DFC: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82301E00: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82301E04: 813C0000  lwz r9, 0(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82301E08: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82301E0C: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82301E10: 409A0018  bne cr6, 0x82301e28
	if !ctx.cr[6].eq {
	pc = 0x82301E28; continue 'dispatch;
	}
	// 82301E14: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82301E18: 815C0004  lwz r10, 4(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82301E1C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82301E20: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82301E24: 419A0008  beq cr6, 0x82301e2c
	if ctx.cr[6].eq {
	pc = 0x82301E2C; continue 'dispatch;
	}
	// 82301E28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82301E2C: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82301E30: 40820090  bne 0x82301ec0
	if !ctx.cr[0].eq {
	pc = 0x82301EC0; continue 'dispatch;
	}
	// 82301E34: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82301E38: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 82301E3C: 554BF87E  srwi r11, r10, 1
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shr(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82301E40: 915C0004  stw r10, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82301E44: 81090008  lwz r8, 8(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 82301E48: 554A07FE  clrlwi r10, r10, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 82301E4C: 7F085840  cmplw cr6, r8, r11
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82301E50: 41990008  bgt cr6, 0x82301e58
	if ctx.cr[6].gt {
	pc = 0x82301E58; continue 'dispatch;
	}
	// 82301E54: 7D685850  subf r11, r8, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[8].s64;
	// 82301E58: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82301E5C: 80C90004  lwz r6, 4(r9)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82301E60: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82301E64: 55471838  slwi r7, r10, 3
	ctx.r[7].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82301E68: 813D0000  lwz r9, 0(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82301E6C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82301E70: 7D06402E  lwzx r8, r6, r8
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[6].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82301E74: 556AF87E  srwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shr(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82301E78: 917D0004  stw r11, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82301E7C: 556607FE  clrlwi r6, r11, 0x1f
	ctx.r[6].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 82301E80: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 82301E84: 7FE83A14  add r31, r8, r7
	ctx.r[31].u64 = ctx.r[8].u64 + ctx.r[7].u64;
	// 82301E88: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82301E8C: 41990008  bgt cr6, 0x82301e94
	if ctx.cr[6].gt {
	pc = 0x82301E94; continue 'dispatch;
	}
	// 82301E90: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82301E94: 81690004  lwz r11, 4(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82301E98: 5549103A  slwi r9, r10, 2
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82301E9C: 54CA1838  slwi r10, r6, 3
	ctx.r[10].u32 = ctx.r[6].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82301EA0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82301EA4: 7D6B482E  lwzx r11, r11, r9
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82301EA8: 7FCB5214  add r30, r11, r10
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82301EAC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82301EB0: 48AF1D21  bl 0x82df3bd0
	ctx.lr = 0x82301EB4;
	sub_82DF3BD0(ctx, base);
	// 82301EB4: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82301EB8: 997E0004  stb r11, 4(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u8 ) };
	// 82301EBC: 4BFFFF48  b 0x82301e04
	pc = 0x82301E04; continue 'dispatch;
	// 82301EC0: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82301EC4: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82301EC8: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82301ECC: 917A0000  stw r11, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82301ED0: 915A0004  stw r10, 4(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82301ED4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82301ED8: 48EA62D8  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82301EE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82301EE0 size=72
    let mut pc: u32 = 0x82301EE0;
    'dispatch: loop {
        match pc {
            0x82301EE0 => {
    //   block [0x82301EE0..0x82301F28)
	// 82301EE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82301EE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82301EE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82301EEC: 2F050003  cmpwi cr6, r5, 3
	ctx.cr[6].compare_i32(ctx.r[5].s32, 3, &mut ctx.xer);
	// 82301EF0: 419A001C  beq cr6, 0x82301f0c
	if ctx.cr[6].eq {
	pc = 0x82301F0C; continue 'dispatch;
	}
	// 82301EF4: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82301EF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82301EFC: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82301F00: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82301F04: 4BFFE6BD  bl 0x823005c0
	ctx.lr = 0x82301F08;
	sub_823005C0(ctx, base);
	// 82301F08: 48000010  b 0x82301f18
	pc = 0x82301F18; continue 'dispatch;
	// 82301F0C: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 82301F10: 396B5718  addi r11, r11, 0x5718
	ctx.r[11].s64 = ctx.r[11].s64 + 22296;
	// 82301F14: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82301F18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82301F1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82301F20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82301F24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82301F28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82301F28 size=236
    let mut pc: u32 = 0x82301F28;
    'dispatch: loop {
        match pc {
            0x82301F28 => {
    //   block [0x82301F28..0x82302014)
	// 82301F28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82301F2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82301F30: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82301F34: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82301F38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82301F3C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82301F40: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82301F44: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82301F48: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82301F4C: 388BCB60  addi r4, r11, -0x34a0
	ctx.r[4].s64 = ctx.r[11].s64 + -13472;
	// 82301F50: 38A00057  li r5, 0x57
	ctx.r[5].s64 = 87;
	// 82301F54: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 82301F58: 4BFBE481  bl 0x822c03d8
	ctx.lr = 0x82301F5C;
	sub_822C03D8(ctx, base);
	// 82301F5C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82301F60: 41820014  beq 0x82301f74
	if ctx.cr[0].eq {
	pc = 0x82301F74; continue 'dispatch;
	}
	// 82301F64: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82301F68: 4815B341  bl 0x8245d2a8
	ctx.lr = 0x82301F6C;
	sub_8245D2A8(ctx, base);
	// 82301F6C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82301F70: 48000008  b 0x82301f78
	pc = 0x82301F78; continue 'dispatch;
	// 82301F74: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82301F78: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82301F7C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82301F80: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82301F84: 4BFFEE7D  bl 0x82300e00
	ctx.lr = 0x82301F88;
	sub_82300E00(ctx, base);
	// 82301F88: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82301F8C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82301F90: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82301F94: 4BFBE06D  bl 0x822c0000
	ctx.lr = 0x82301F98;
	sub_822C0000(ctx, base);
	// 82301F98: 83E10054  lwz r31, 0x54(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82301F9C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82301FA0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82301FA4: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 82301FA8: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82301FAC: 419A0024  beq cr6, 0x82301fd0
	if ctx.cr[6].eq {
	pc = 0x82301FD0; continue 'dispatch;
	}
	// 82301FB0: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 82301FB4: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82301FB8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82301FBC: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82301FC0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82301FC4: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82301FC8: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82301FCC: 4082FFE8  bne 0x82301fb4
	if !ctx.cr[0].eq {
	pc = 0x82301FB4; continue 'dispatch;
	}
	// 82301FD0: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82301FD4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82301FD8: 488BAF89  bl 0x82bbcf60
	ctx.lr = 0x82301FDC;
	sub_82BBCF60(ctx, base);
	// 82301FDC: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82301FE0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82301FE4: 419A0008  beq cr6, 0x82301fec
	if ctx.cr[6].eq {
	pc = 0x82301FEC; continue 'dispatch;
	}
	// 82301FE8: 4BFBE8A9  bl 0x822c0890
	ctx.lr = 0x82301FEC;
	sub_822C0890(ctx, base);
	// 82301FEC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82301FF0: 419A000C  beq cr6, 0x82301ffc
	if ctx.cr[6].eq {
	pc = 0x82301FFC; continue 'dispatch;
	}
	// 82301FF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82301FF8: 4BFBE899  bl 0x822c0890
	ctx.lr = 0x82301FFC;
	sub_822C0890(ctx, base);
	// 82301FFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82302000: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82302004: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82302008: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8230200C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82302010: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82302018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82302018 size=24
    let mut pc: u32 = 0x82302018;
    'dispatch: loop {
        match pc {
            0x82302018 => {
    //   block [0x82302018..0x82302030)
	// 82302018: 8164023C  lwz r11, 0x23c(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(572 as u32) ) } as u64;
	// 8230201C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82302020: 81640240  lwz r11, 0x240(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(576 as u32) ) } as u64;
	// 82302024: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82302028: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8230202C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82302030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82302030 size=36
    let mut pc: u32 = 0x82302030;
    'dispatch: loop {
        match pc {
            0x82302030 => {
    //   block [0x82302030..0x82302054)
	// 82302030: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82302034: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82302038: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8230203C: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82302040: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82302044: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82302048: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8230204C: 4082FFE8  bne 0x82302034
	if !ctx.cr[0].eq {
	pc = 0x82302034; continue 'dispatch;
	}
	// 82302050: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82302058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82302058 size=108
    let mut pc: u32 = 0x82302058;
    'dispatch: loop {
        match pc {
            0x82302058 => {
    //   block [0x82302058..0x823020C4)
	// 82302058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8230205C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82302060: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82302064: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82302068: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8230206C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82302070: 48AF1B91  bl 0x82df3c00
	ctx.lr = 0x82302074;
	sub_82DF3C00(ctx, base);
	// 82302074: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82302078: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8230207C: 389F00CC  addi r4, r31, 0xcc
	ctx.r[4].s64 = ctx.r[31].s64 + 204;
	// 82302080: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82302084: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82302088: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8230208C: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82302090: 48B5C8D1  bl 0x82e5e960
	ctx.lr = 0x82302094;
	sub_82E5E960(ctx, base);
	// 82302094: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82302098: 83E30000  lwz r31, 0(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8230209C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823020A0: 419A000C  beq cr6, 0x823020ac
	if ctx.cr[6].eq {
	pc = 0x823020AC; continue 'dispatch;
	}
	// 823020A4: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 823020A8: 4BFBE7E9  bl 0x822c0890
	ctx.lr = 0x823020AC;
	sub_822C0890(ctx, base);
	// 823020AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823020B0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823020B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823020B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823020BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823020C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823020C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x823020C8 size=196
    let mut pc: u32 = 0x823020C8;
    'dispatch: loop {
        match pc {
            0x823020C8 => {
    //   block [0x823020C8..0x8230218C)
	// 823020C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823020CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823020D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823020D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823020D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823020DC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823020E0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 823020E4: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823020E8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823020EC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823020F0: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823020F4: 419A0024  beq cr6, 0x82302118
	if ctx.cr[6].eq {
	pc = 0x82302118; continue 'dispatch;
	}
	// 823020F8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 823020FC: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82302100: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82302104: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82302108: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8230210C: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82302110: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82302114: 4082FFE8  bne 0x823020fc
	if !ctx.cr[0].eq {
	pc = 0x823020FC; continue 'dispatch;
	}
	// 82302118: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8230211C: 388300CC  addi r4, r3, 0xcc
	ctx.r[4].s64 = ctx.r[3].s64 + 204;
	// 82302120: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82302124: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82302128: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8230212C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82302130: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82302134: 48B5C575  bl 0x82e5e6a8
	ctx.lr = 0x82302138;
	sub_82E5E6A8(ctx, base);
	// 82302138: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8230213C: 83C30000  lwz r30, 0(r3)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82302140: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82302144: 419A000C  beq cr6, 0x82302150
	if ctx.cr[6].eq {
	pc = 0x82302150; continue 'dispatch;
	}
	// 82302148: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8230214C: 4BFBE745  bl 0x822c0890
	ctx.lr = 0x82302150;
	sub_822C0890(ctx, base);
	// 82302150: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82302154: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82302158: 419A0008  beq cr6, 0x82302160
	if ctx.cr[6].eq {
	pc = 0x82302160; continue 'dispatch;
	}
	// 8230215C: 4BFBE735  bl 0x822c0890
	ctx.lr = 0x82302160;
	sub_822C0890(ctx, base);
	// 82302160: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82302164: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82302168: 419A0008  beq cr6, 0x82302170
	if ctx.cr[6].eq {
	pc = 0x82302170; continue 'dispatch;
	}
	// 8230216C: 4BFBE725  bl 0x822c0890
	ctx.lr = 0x82302170;
	sub_822C0890(ctx, base);
	// 82302170: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82302174: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82302178: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8230217C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82302180: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82302184: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82302188: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82302190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82302190 size=156
    let mut pc: u32 = 0x82302190;
    'dispatch: loop {
        match pc {
            0x82302190 => {
    //   block [0x82302190..0x8230222C)
	// 82302190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82302194: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82302198: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8230219C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823021A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823021A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823021A8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 823021AC: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 823021B0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823021B4: 48AF1A4D  bl 0x82df3c00
	ctx.lr = 0x823021B8;
	sub_82DF3C00(ctx, base);
	// 823021B8: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 823021BC: 389E00CC  addi r4, r30, 0xcc
	ctx.r[4].s64 = ctx.r[30].s64 + 204;
	// 823021C0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 823021C4: 48B590DD  bl 0x82e5b2a0
	ctx.lr = 0x823021C8;
	sub_82E5B2A0(ctx, base);
	// 823021C8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 823021CC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823021D0: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 823021D4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823021D8: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 823021DC: 419A0024  beq cr6, 0x82302200
	if ctx.cr[6].eq {
	pc = 0x82302200; continue 'dispatch;
	}
	// 823021E0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 823021E4: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 823021E8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 823021EC: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 823021F0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 823021F4: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 823021F8: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 823021FC: 4082FFE8  bne 0x823021e4
	if !ctx.cr[0].eq {
	pc = 0x823021E4; continue 'dispatch;
	}
	// 82302200: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82302204: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82302208: 419A0008  beq cr6, 0x82302210
	if ctx.cr[6].eq {
	pc = 0x82302210; continue 'dispatch;
	}
	// 8230220C: 4BFBE685  bl 0x822c0890
	ctx.lr = 0x82302210;
	sub_822C0890(ctx, base);
	// 82302210: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82302214: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82302218: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8230221C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82302220: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82302224: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82302228: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82302230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82302230 size=120
    let mut pc: u32 = 0x82302230;
    'dispatch: loop {
        match pc {
            0x82302230 => {
    //   block [0x82302230..0x823022A8)
	// 82302230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82302234: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82302238: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8230223C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82302240: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82302244: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82302248: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8230224C: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82302250: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82302254: 3883012C  addi r4, r3, 0x12c
	ctx.r[4].s64 = ctx.r[3].s64 + 300;
	// 82302258: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 8230225C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82302260: C02A08A4  lfs f1, 0x8a4(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82302264: 48B5E475  bl 0x82e606d8
	ctx.lr = 0x82302268;
	sub_82E606D8(ctx, base);
	// 82302268: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8230226C: 83E30000  lwz r31, 0(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82302270: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82302274: 419A000C  beq cr6, 0x82302280
	if ctx.cr[6].eq {
	pc = 0x82302280; continue 'dispatch;
	}
	// 82302278: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8230227C: 4BFBE615  bl 0x822c0890
	ctx.lr = 0x82302280;
	sub_822C0890(ctx, base);
	// 82302280: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82302284: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82302288: 419A0008  beq cr6, 0x82302290
	if ctx.cr[6].eq {
	pc = 0x82302290; continue 'dispatch;
	}
	// 8230228C: 4BFBE605  bl 0x822c0890
	ctx.lr = 0x82302290;
	sub_822C0890(ctx, base);
	// 82302290: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82302294: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82302298: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8230229C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823022A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823022A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823022A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x823022A8 size=76
    let mut pc: u32 = 0x823022A8;
    'dispatch: loop {
        match pc {
            0x823022A8 => {
    //   block [0x823022A8..0x823022F4)
	// 823022A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823022AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823022B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823022B4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823022B8: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823022BC: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 823022C0: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 823022C4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823022C8: 3863012C  addi r3, r3, 0x12c
	ctx.r[3].s64 = ctx.r[3].s64 + 300;
	// 823022CC: C02A08A4  lfs f1, 0x8a4(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 823022D0: 48B5DF91  bl 0x82e60260
	ctx.lr = 0x823022D4;
	sub_82E60260(ctx, base);
	// 823022D4: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 823022D8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823022DC: 419A0008  beq cr6, 0x823022e4
	if ctx.cr[6].eq {
	pc = 0x823022E4; continue 'dispatch;
	}
	// 823022E0: 4BFBE5B1  bl 0x822c0890
	ctx.lr = 0x823022E4;
	sub_822C0890(ctx, base);
	// 823022E4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 823022E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823022EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823022F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823022F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823022F8 size=128
    let mut pc: u32 = 0x823022F8;
    'dispatch: loop {
        match pc {
            0x823022F8 => {
    //   block [0x823022F8..0x82302378)
	// 823022F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823022FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82302300: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82302304: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82302308: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8230230C: 3884013C  addi r4, r4, 0x13c
	ctx.r[4].s64 = ctx.r[4].s64 + 316;
	// 82302310: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82302314: 48B5820D  bl 0x82e5a520
	ctx.lr = 0x82302318;
	sub_82E5A520(ctx, base);
	// 82302318: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8230231C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82302320: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82302324: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82302328: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8230232C: 419A0024  beq cr6, 0x82302350
	if ctx.cr[6].eq {
	pc = 0x82302350; continue 'dispatch;
	}
	// 82302330: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82302334: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82302338: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8230233C: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82302340: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82302344: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82302348: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8230234C: 4082FFE8  bne 0x82302334
	if !ctx.cr[0].eq {
	pc = 0x82302334; continue 'dispatch;
	}
	// 82302350: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82302354: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82302358: 419A0008  beq cr6, 0x82302360
	if ctx.cr[6].eq {
	pc = 0x82302360; continue 'dispatch;
	}
	// 8230235C: 4BFBE535  bl 0x822c0890
	ctx.lr = 0x82302360;
	sub_822C0890(ctx, base);
	// 82302360: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82302364: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82302368: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8230236C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82302370: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82302374: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82302378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82302378 size=168
    let mut pc: u32 = 0x82302378;
    'dispatch: loop {
        match pc {
            0x82302378 => {
    //   block [0x82302378..0x82302420)
	// 82302378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8230237C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82302380: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82302384: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82302388: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8230238C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82302390: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82302394: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82302398: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8230239C: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 823023A0: 3884022C  addi r4, r4, 0x22c
	ctx.r[4].s64 = ctx.r[4].s64 + 556;
	// 823023A4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 823023A8: C02A08A4  lfs f1, 0x8a4(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 823023AC: 48B5E32D  bl 0x82e606d8
	ctx.lr = 0x823023B0;
	sub_82E606D8(ctx, base);
	// 823023B0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 823023B4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823023B8: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 823023BC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823023C0: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 823023C4: 419A0024  beq cr6, 0x823023e8
	if ctx.cr[6].eq {
	pc = 0x823023E8; continue 'dispatch;
	}
	// 823023C8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 823023CC: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 823023D0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 823023D4: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 823023D8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 823023DC: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 823023E0: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 823023E4: 4082FFE8  bne 0x823023cc
	if !ctx.cr[0].eq {
	pc = 0x823023CC; continue 'dispatch;
	}
	// 823023E8: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 823023EC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823023F0: 419A0008  beq cr6, 0x823023f8
	if ctx.cr[6].eq {
	pc = 0x823023F8; continue 'dispatch;
	}
	// 823023F4: 4BFBE49D  bl 0x822c0890
	ctx.lr = 0x823023F8;
	sub_822C0890(ctx, base);
	// 823023F8: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 823023FC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82302400: 419A0008  beq cr6, 0x82302408
	if ctx.cr[6].eq {
	pc = 0x82302408; continue 'dispatch;
	}
	// 82302404: 4BFBE48D  bl 0x822c0890
	ctx.lr = 0x82302408;
	sub_822C0890(ctx, base);
	// 82302408: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8230240C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82302410: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82302414: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82302418: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8230241C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82302420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82302420 size=76
    let mut pc: u32 = 0x82302420;
    'dispatch: loop {
        match pc {
            0x82302420 => {
    //   block [0x82302420..0x8230246C)
	// 82302420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82302424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82302428: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8230242C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82302430: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82302434: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82302438: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8230243C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82302440: 3863022C  addi r3, r3, 0x22c
	ctx.r[3].s64 = ctx.r[3].s64 + 556;
	// 82302444: C02A08A4  lfs f1, 0x8a4(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82302448: 48B5DE19  bl 0x82e60260
	ctx.lr = 0x8230244C;
	sub_82E60260(ctx, base);
	// 8230244C: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82302450: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82302454: 419A0008  beq cr6, 0x8230245c
	if ctx.cr[6].eq {
	pc = 0x8230245C; continue 'dispatch;
	}
	// 82302458: 4BFBE439  bl 0x822c0890
	ctx.lr = 0x8230245C;
	sub_822C0890(ctx, base);
	// 8230245C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82302460: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82302464: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82302468: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82302470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82302470 size=168
    let mut pc: u32 = 0x82302470;
    'dispatch: loop {
        match pc {
            0x82302470 => {
    //   block [0x82302470..0x82302518)
	// 82302470: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82302474: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82302478: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8230247C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82302480: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82302484: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82302488: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8230248C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82302490: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82302494: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82302498: 38840224  addi r4, r4, 0x224
	ctx.r[4].s64 = ctx.r[4].s64 + 548;
	// 8230249C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 823024A0: C02A08A4  lfs f1, 0x8a4(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 823024A4: 48B5E235  bl 0x82e606d8
	ctx.lr = 0x823024A8;
	sub_82E606D8(ctx, base);
	// 823024A8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 823024AC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823024B0: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 823024B4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823024B8: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 823024BC: 419A0024  beq cr6, 0x823024e0
	if ctx.cr[6].eq {
	pc = 0x823024E0; continue 'dispatch;
	}
	// 823024C0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 823024C4: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 823024C8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 823024CC: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 823024D0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 823024D4: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 823024D8: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 823024DC: 4082FFE8  bne 0x823024c4
	if !ctx.cr[0].eq {
	pc = 0x823024C4; continue 'dispatch;
	}
	// 823024E0: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 823024E4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823024E8: 419A0008  beq cr6, 0x823024f0
	if ctx.cr[6].eq {
	pc = 0x823024F0; continue 'dispatch;
	}
	// 823024EC: 4BFBE3A5  bl 0x822c0890
	ctx.lr = 0x823024F0;
	sub_822C0890(ctx, base);
	// 823024F0: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 823024F4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823024F8: 419A0008  beq cr6, 0x82302500
	if ctx.cr[6].eq {
	pc = 0x82302500; continue 'dispatch;
	}
	// 823024FC: 4BFBE395  bl 0x822c0890
	ctx.lr = 0x82302500;
	sub_822C0890(ctx, base);
	// 82302500: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82302504: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82302508: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8230250C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82302510: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82302514: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82302518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82302518 size=76
    let mut pc: u32 = 0x82302518;
    'dispatch: loop {
        match pc {
            0x82302518 => {
    //   block [0x82302518..0x82302564)
	// 82302518: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8230251C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82302520: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82302524: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82302528: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8230252C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82302530: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82302534: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82302538: 38630224  addi r3, r3, 0x224
	ctx.r[3].s64 = ctx.r[3].s64 + 548;
	// 8230253C: C02A08A4  lfs f1, 0x8a4(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82302540: 48B5DD21  bl 0x82e60260
	ctx.lr = 0x82302544;
	sub_82E60260(ctx, base);
	// 82302544: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82302548: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8230254C: 419A0008  beq cr6, 0x82302554
	if ctx.cr[6].eq {
	pc = 0x82302554; continue 'dispatch;
	}
	// 82302550: 4BFBE341  bl 0x822c0890
	ctx.lr = 0x82302554;
	sub_822C0890(ctx, base);
	// 82302554: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82302558: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8230255C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82302560: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82302568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82302568 size=136
    let mut pc: u32 = 0x82302568;
    'dispatch: loop {
        match pc {
            0x82302568 => {
    //   block [0x82302568..0x823025F0)
	// 82302568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8230256C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82302570: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82302574: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82302578: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8230257C: 81240000  lwz r9, 0(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82302580: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82302584: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82302588: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 8230258C: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82302590: 91210058  stw r9, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[9].u32 ) };
	// 82302594: 419A0024  beq cr6, 0x823025b8
	if ctx.cr[6].eq {
	pc = 0x823025B8; continue 'dispatch;
	}
	// 82302598: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8230259C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 823025A0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 823025A4: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 823025A8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 823025AC: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 823025B0: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 823025B4: 4082FFE8  bne 0x8230259c
	if !ctx.cr[0].eq {
	pc = 0x8230259C; continue 'dispatch;
	}
	// 823025B8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 823025BC: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 823025C0: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 823025C4: 38630224  addi r3, r3, 0x224
	ctx.r[3].s64 = ctx.r[3].s64 + 548;
	// 823025C8: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 823025CC: 48B5D675  bl 0x82e5fc40
	ctx.lr = 0x823025D0;
	sub_82E5FC40(ctx, base);
	// 823025D0: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 823025D4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823025D8: 419A0008  beq cr6, 0x823025e0
	if ctx.cr[6].eq {
	pc = 0x823025E0; continue 'dispatch;
	}
	// 823025DC: 4BFBE2B5  bl 0x822c0890
	ctx.lr = 0x823025E0;
	sub_822C0890(ctx, base);
	// 823025E0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823025E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823025E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823025EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823025F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823025F0 size=124
    let mut pc: u32 = 0x823025F0;
    'dispatch: loop {
        match pc {
            0x823025F0 => {
    //   block [0x823025F0..0x8230266C)
	// 823025F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823025F4: 48EA5B79  bl 0x831a816c
	ctx.lr = 0x823025F8;
	sub_831A8130(ctx, base);
	// 823025F8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823025FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82302600: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82302604: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82302608: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8230260C: 4820CEBD  bl 0x8250f4c8
	ctx.lr = 0x82302610;
	sub_8250F4C8(ctx, base);
	// 82302610: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82302614: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82302618: 3BCBFFFC  addi r30, r11, -4
	ctx.r[30].s64 = ctx.r[11].s64 + -4;
	// 8230261C: 409A0008  bne cr6, 0x82302624
	if !ctx.cr[6].eq {
	pc = 0x82302624; continue 'dispatch;
	}
	// 82302620: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82302624: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82302628: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8230262C: 4818A43D  bl 0x8248ca68
	ctx.lr = 0x82302630;
	sub_8248CA68(ctx, base);
	// 82302630: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82302634: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82302638: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8230263C: 4820CEDD  bl 0x8250f518
	ctx.lr = 0x82302640;
	sub_8250F518(ctx, base);
	// 82302640: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82302644: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82302648: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8230264C: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82302650: 4820AEB9  bl 0x8250d508
	ctx.lr = 0x82302654;
	sub_8250D508(ctx, base);
	// 82302654: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82302658: 48AEF639  bl 0x82df1c90
	ctx.lr = 0x8230265C;
	sub_82DF1C90(ctx, base);
	// 8230265C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82302660: 48AEF631  bl 0x82df1c90
	ctx.lr = 0x82302664;
	sub_82DF1C90(ctx, base);
	// 82302664: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82302668: 48EA5B54  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82302670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82302670 size=52
    let mut pc: u32 = 0x82302670;
    'dispatch: loop {
        match pc {
            0x82302670 => {
    //   block [0x82302670..0x823026A4)
	// 82302670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82302674: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82302678: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8230267C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82302680: 80840268  lwz r4, 0x268(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(616 as u32) ) } as u64;
	// 82302684: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82302688: 4818A7B9  bl 0x8248ce40
	ctx.lr = 0x8230268C;
	sub_8248CE40(ctx, base);
	// 8230268C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82302690: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82302694: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82302698: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8230269C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823026A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823026A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823026A8 size=160
    let mut pc: u32 = 0x823026A8;
    'dispatch: loop {
        match pc {
            0x823026A8 => {
    //   block [0x823026A8..0x82302748)
	// 823026A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823026AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823026B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823026B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823026B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823026BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823026C0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 823026C4: 387F00C0  addi r3, r31, 0xc0
	ctx.r[3].s64 = ctx.r[31].s64 + 192;
	// 823026C8: 817F00C0  lwz r11, 0xc0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(192 as u32) ) } as u64;
	// 823026CC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 823026D0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823026D4: 4E800421  bctrl
	ctx.lr = 0x823026D8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823026D8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 823026DC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 823026E0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 823026E4: 488B04E5  bl 0x82bb2bc8
	ctx.lr = 0x823026E8;
	sub_82BB2BC8(ctx, base);
	// 823026E8: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 823026EC: 83C30000  lwz r30, 0(r3)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 823026F0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823026F4: 419A000C  beq cr6, 0x82302700
	if ctx.cr[6].eq {
	pc = 0x82302700; continue 'dispatch;
	}
	// 823026F8: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 823026FC: 4BFBE195  bl 0x822c0890
	ctx.lr = 0x82302700;
	sub_822C0890(ctx, base);
	// 82302700: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82302704: 419A002C  beq cr6, 0x82302730
	if ctx.cr[6].eq {
	pc = 0x82302730; continue 'dispatch;
	}
	// 82302708: 817F00C4  lwz r11, 0xc4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 8230270C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82302710: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82302714: 394ACB34  addi r10, r10, -0x34cc
	ctx.r[10].s64 = ctx.r[10].s64 + -13516;
	// 82302718: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8230271C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82302720: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82302724: 488B4435  bl 0x82bb6b58
	ctx.lr = 0x82302728;
	sub_82BB6B58(ctx, base);
	// 82302728: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8230272C: 488BAF5D  bl 0x82bbd688
	ctx.lr = 0x82302730;
	sub_82BBD688(ctx, base);
	// 82302730: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82302734: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82302738: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8230273C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82302740: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82302744: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82302748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82302748 size=128
    let mut pc: u32 = 0x82302748;
    'dispatch: loop {
        match pc {
            0x82302748 => {
    //   block [0x82302748..0x823027C8)
	// 82302748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8230274C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82302750: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82302754: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82302758: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8230275C: 388401B0  addi r4, r4, 0x1b0
	ctx.r[4].s64 = ctx.r[4].s64 + 432;
	// 82302760: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82302764: 48B57DBD  bl 0x82e5a520
	ctx.lr = 0x82302768;
	sub_82E5A520(ctx, base);
	// 82302768: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8230276C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82302770: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82302774: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82302778: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8230277C: 419A0024  beq cr6, 0x823027a0
	if ctx.cr[6].eq {
	pc = 0x823027A0; continue 'dispatch;
	}
	// 82302780: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82302784: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82302788: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8230278C: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82302790: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82302794: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82302798: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8230279C: 4082FFE8  bne 0x82302784
	if !ctx.cr[0].eq {
	pc = 0x82302784; continue 'dispatch;
	}
	// 823027A0: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 823027A4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823027A8: 419A0008  beq cr6, 0x823027b0
	if ctx.cr[6].eq {
	pc = 0x823027B0; continue 'dispatch;
	}
	// 823027AC: 4BFBE0E5  bl 0x822c0890
	ctx.lr = 0x823027B0;
	sub_822C0890(ctx, base);
	// 823027B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823027B4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823027B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823027BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823027C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823027C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823027C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823027C8 size=136
    let mut pc: u32 = 0x823027C8;
    'dispatch: loop {
        match pc {
            0x823027C8 => {
    //   block [0x823027C8..0x82302850)
	// 823027C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823027CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823027D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823027D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823027D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823027DC: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 823027E0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823027E4: 419A0058  beq cr6, 0x8230283c
	if ctx.cr[6].eq {
	pc = 0x8230283C; continue 'dispatch;
	}
	// 823027E8: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 823027EC: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 823027F0: 5569083A  rlwinm r9, r11, 1, 0, 0x1d
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x7FFFFFFFu64;
	// 823027F4: 556B1F38  rlwinm r11, r11, 3, 0x1c, 0x1c
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x1FFFFFFFu64;
	// 823027F8: 7D49502E  lwzx r10, r9, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 823027FC: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82302800: 48AF0C29  bl 0x82df3428
	ctx.lr = 0x82302804;
	sub_82DF3428(ctx, base);
	// 82302804: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82302808: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8230280C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82302810: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82302814: 5529083C  slwi r9, r9, 1
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82302818: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8230281C: 7F095840  cmplw cr6, r9, r11
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82302820: 41990008  bgt cr6, 0x82302828
	if ctx.cr[6].gt {
	pc = 0x82302828; continue 'dispatch;
	}
	// 82302824: 915F000C  stw r10, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82302828: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8230282C: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82302830: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82302834: 40820008  bne 0x8230283c
	if !ctx.cr[0].eq {
	pc = 0x8230283C; continue 'dispatch;
	}
	// 82302838: 915F000C  stw r10, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8230283C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82302840: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82302844: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82302848: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8230284C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82302850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82302850 size=124
    let mut pc: u32 = 0x82302850;
    'dispatch: loop {
        match pc {
            0x82302850 => {
    //   block [0x82302850..0x823028CC)
	// 82302850: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82302854: 48EA5919  bl 0x831a816c
	ctx.lr = 0x82302858;
	sub_831A8130(ctx, base);
	// 82302858: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8230285C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82302860: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82302864: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82302868: 388BA66C  addi r4, r11, -0x5994
	ctx.r[4].s64 = ctx.r[11].s64 + -22932;
	// 8230286C: 38A00073  li r5, 0x73
	ctx.r[5].s64 = 115;
	// 82302870: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 82302874: 48AEFB75  bl 0x82df23e8
	ctx.lr = 0x82302878;
	sub_82DF23E8(ctx, base);
	// 82302878: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8230287C: 4182001C  beq 0x82302898
	if ctx.cr[0].eq {
	pc = 0x82302898; continue 'dispatch;
	}
	// 82302880: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82302884: 48B56DFD  bl 0x82e59680
	ctx.lr = 0x82302888;
	sub_82E59680(ctx, base);
	// 82302888: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8230288C: 396BCA3C  addi r11, r11, -0x35c4
	ctx.r[11].s64 = ctx.r[11].s64 + -13764;
	// 82302890: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82302894: 48000008  b 0x8230289c
	pc = 0x8230289C; continue 'dispatch;
	// 82302898: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8230289C: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 823028A0: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 823028A4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823028A8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823028AC: 4BFFE71D  bl 0x82300fc8
	ctx.lr = 0x823028B0;
	sub_82300FC8(ctx, base);
	// 823028B0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 823028B4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823028B8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823028BC: 4BFBD745  bl 0x822c0000
	ctx.lr = 0x823028C0;
	sub_822C0000(ctx, base);
	// 823028C0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823028C4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823028C8: 48EA58F4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823028D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823028D0 size=124
    let mut pc: u32 = 0x823028D0;
    'dispatch: loop {
        match pc {
            0x823028D0 => {
    //   block [0x823028D0..0x8230294C)
	// 823028D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823028D4: 48EA5899  bl 0x831a816c
	ctx.lr = 0x823028D8;
	sub_831A8130(ctx, base);
	// 823028D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823028DC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823028E0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823028E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823028E8: 388BA66C  addi r4, r11, -0x5994
	ctx.r[4].s64 = ctx.r[11].s64 + -22932;
	// 823028EC: 38A00073  li r5, 0x73
	ctx.r[5].s64 = 115;
	// 823028F0: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 823028F4: 48AEFAF5  bl 0x82df23e8
	ctx.lr = 0x823028F8;
	sub_82DF23E8(ctx, base);
	// 823028F8: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 823028FC: 4182001C  beq 0x82302918
	if ctx.cr[0].eq {
	pc = 0x82302918; continue 'dispatch;
	}
	// 82302900: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82302904: 48B56D7D  bl 0x82e59680
	ctx.lr = 0x82302908;
	sub_82E59680(ctx, base);
	// 82302908: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8230290C: 396BCA44  addi r11, r11, -0x35bc
	ctx.r[11].s64 = ctx.r[11].s64 + -13756;
	// 82302910: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82302914: 48000008  b 0x8230291c
	pc = 0x8230291C; continue 'dispatch;
	// 82302918: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8230291C: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82302920: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 82302924: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82302928: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8230292C: 4BFFE765  bl 0x82301090
	ctx.lr = 0x82302930;
	sub_82301090(ctx, base);
	// 82302930: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82302934: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82302938: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8230293C: 4BFBD6C5  bl 0x822c0000
	ctx.lr = 0x82302940;
	sub_822C0000(ctx, base);
	// 82302940: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82302944: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82302948: 48EA5874  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82302950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82302950 size=148
    let mut pc: u32 = 0x82302950;
    'dispatch: loop {
        match pc {
            0x82302950 => {
    //   block [0x82302950..0x823029E4)
	// 82302950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82302954: 48EA5811  bl 0x831a8164
	ctx.lr = 0x82302958;
	sub_831A8130(ctx, base);
	// 82302958: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8230295C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82302960: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82302964: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82302968: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8230296C: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82302970: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82302974: 388BA66C  addi r4, r11, -0x5994
	ctx.r[4].s64 = ctx.r[11].s64 + -22932;
	// 82302978: 38A00089  li r5, 0x89
	ctx.r[5].s64 = 137;
	// 8230297C: 38600038  li r3, 0x38
	ctx.r[3].s64 = 56;
	// 82302980: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 82302984: 48AEFA65  bl 0x82df23e8
	ctx.lr = 0x82302988;
	sub_82DF23E8(ctx, base);
	// 82302988: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8230298C: 41820024  beq 0x823029b0
	if ctx.cr[0].eq {
	pc = 0x823029B0; continue 'dispatch;
	}
	// 82302990: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82302994: 80FC0000  lwz r7, 0(r28)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82302998: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8230299C: 88DD0000  lbz r6, 0(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 823029A0: C03E0000  lfs f1, 0(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 823029A4: 4884ED3D  bl 0x82b516e0
	ctx.lr = 0x823029A8;
	sub_82B516E0(ctx, base);
	// 823029A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823029AC: 48000008  b 0x823029b4
	pc = 0x823029B4; continue 'dispatch;
	// 823029B0: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823029B4: 93FB0000  stw r31, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 823029B8: 3BDB0004  addi r30, r27, 4
	ctx.r[30].s64 = ctx.r[27].s64 + 4;
	// 823029BC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823029C0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823029C4: 4BFFE795  bl 0x82301158
	ctx.lr = 0x823029C8;
	sub_82301158(ctx, base);
	// 823029C8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 823029CC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823029D0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823029D4: 4BFBD62D  bl 0x822c0000
	ctx.lr = 0x823029D8;
	sub_822C0000(ctx, base);
	// 823029D8: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 823029DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 823029E0: 48EA57D4  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823029E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823029E8 size=124
    let mut pc: u32 = 0x823029E8;
    'dispatch: loop {
        match pc {
            0x823029E8 => {
    //   block [0x823029E8..0x82302A64)
	// 823029E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823029EC: 48EA5781  bl 0x831a816c
	ctx.lr = 0x823029F0;
	sub_831A8130(ctx, base);
	// 823029F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823029F4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823029F8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823029FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82302A00: 388BA66C  addi r4, r11, -0x5994
	ctx.r[4].s64 = ctx.r[11].s64 + -22932;
	// 82302A04: 38A00073  li r5, 0x73
	ctx.r[5].s64 = 115;
	// 82302A08: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82302A0C: 48AEF9DD  bl 0x82df23e8
	ctx.lr = 0x82302A10;
	sub_82DF23E8(ctx, base);
	// 82302A10: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82302A14: 4182001C  beq 0x82302a30
	if ctx.cr[0].eq {
	pc = 0x82302A30; continue 'dispatch;
	}
	// 82302A18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82302A1C: 48B56C65  bl 0x82e59680
	ctx.lr = 0x82302A20;
	sub_82E59680(ctx, base);
	// 82302A20: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82302A24: 396BCA4C  addi r11, r11, -0x35b4
	ctx.r[11].s64 = ctx.r[11].s64 + -13748;
	// 82302A28: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82302A2C: 48000008  b 0x82302a34
	pc = 0x82302A34; continue 'dispatch;
	// 82302A30: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82302A34: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82302A38: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 82302A3C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82302A40: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82302A44: 4BFFE96D  bl 0x823013b0
	ctx.lr = 0x82302A48;
	sub_823013B0(ctx, base);
	// 82302A48: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82302A4C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82302A50: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82302A54: 4BFBD5AD  bl 0x822c0000
	ctx.lr = 0x82302A58;
	sub_822C0000(ctx, base);
	// 82302A58: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82302A5C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82302A60: 48EA575C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82302A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82302A68 size=124
    let mut pc: u32 = 0x82302A68;
    'dispatch: loop {
        match pc {
            0x82302A68 => {
    //   block [0x82302A68..0x82302AE4)
	// 82302A68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82302A6C: 48EA5701  bl 0x831a816c
	ctx.lr = 0x82302A70;
	sub_831A8130(ctx, base);
	// 82302A70: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82302A74: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82302A78: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82302A7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82302A80: 388BA66C  addi r4, r11, -0x5994
	ctx.r[4].s64 = ctx.r[11].s64 + -22932;
	// 82302A84: 38A00073  li r5, 0x73
	ctx.r[5].s64 = 115;
	// 82302A88: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82302A8C: 48AEF95D  bl 0x82df23e8
	ctx.lr = 0x82302A90;
	sub_82DF23E8(ctx, base);
	// 82302A90: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82302A94: 4182001C  beq 0x82302ab0
	if ctx.cr[0].eq {
	pc = 0x82302AB0; continue 'dispatch;
	}
	// 82302A98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82302A9C: 48B56BE5  bl 0x82e59680
	ctx.lr = 0x82302AA0;
	sub_82E59680(ctx, base);
	// 82302AA0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82302AA4: 396BCA54  addi r11, r11, -0x35ac
	ctx.r[11].s64 = ctx.r[11].s64 + -13740;
	// 82302AA8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82302AAC: 48000008  b 0x82302ab4
	pc = 0x82302AB4; continue 'dispatch;
	// 82302AB0: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82302AB4: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82302AB8: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 82302ABC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82302AC0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82302AC4: 4BFFE9B5  bl 0x82301478
	ctx.lr = 0x82302AC8;
	sub_82301478(ctx, base);
	// 82302AC8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82302ACC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82302AD0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82302AD4: 4BFBD52D  bl 0x822c0000
	ctx.lr = 0x82302AD8;
	sub_822C0000(ctx, base);
	// 82302AD8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82302ADC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82302AE0: 48EA56DC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82302AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82302AE8 size=124
    let mut pc: u32 = 0x82302AE8;
    'dispatch: loop {
        match pc {
            0x82302AE8 => {
    //   block [0x82302AE8..0x82302B64)
	// 82302AE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82302AEC: 48EA5681  bl 0x831a816c
	ctx.lr = 0x82302AF0;
	sub_831A8130(ctx, base);
	// 82302AF0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82302AF4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82302AF8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82302AFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82302B00: 388BA66C  addi r4, r11, -0x5994
	ctx.r[4].s64 = ctx.r[11].s64 + -22932;
	// 82302B04: 38A00073  li r5, 0x73
	ctx.r[5].s64 = 115;
	// 82302B08: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82302B0C: 48AEF8DD  bl 0x82df23e8
	ctx.lr = 0x82302B10;
	sub_82DF23E8(ctx, base);
	// 82302B10: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82302B14: 4182001C  beq 0x82302b30
	if ctx.cr[0].eq {
	pc = 0x82302B30; continue 'dispatch;
	}
	// 82302B18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82302B1C: 48B56B65  bl 0x82e59680
	ctx.lr = 0x82302B20;
	sub_82E59680(ctx, base);
	// 82302B20: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82302B24: 396BCA5C  addi r11, r11, -0x35a4
	ctx.r[11].s64 = ctx.r[11].s64 + -13732;
	// 82302B28: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82302B2C: 48000008  b 0x82302b34
	pc = 0x82302B34; continue 'dispatch;
	// 82302B30: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82302B34: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82302B38: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 82302B3C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82302B40: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82302B44: 4BFFE9FD  bl 0x82301540
	ctx.lr = 0x82302B48;
	sub_82301540(ctx, base);
	// 82302B48: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82302B4C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82302B50: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82302B54: 4BFBD4AD  bl 0x822c0000
	ctx.lr = 0x82302B58;
	sub_822C0000(ctx, base);
	// 82302B58: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82302B5C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82302B60: 48EA565C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82302B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82302B68 size=124
    let mut pc: u32 = 0x82302B68;
    'dispatch: loop {
        match pc {
            0x82302B68 => {
    //   block [0x82302B68..0x82302BE4)
	// 82302B68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82302B6C: 48EA5601  bl 0x831a816c
	ctx.lr = 0x82302B70;
	sub_831A8130(ctx, base);
	// 82302B70: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82302B74: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82302B78: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82302B7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82302B80: 388BA66C  addi r4, r11, -0x5994
	ctx.r[4].s64 = ctx.r[11].s64 + -22932;
	// 82302B84: 38A00073  li r5, 0x73
	ctx.r[5].s64 = 115;
	// 82302B88: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82302B8C: 48AEF85D  bl 0x82df23e8
	ctx.lr = 0x82302B90;
	sub_82DF23E8(ctx, base);
	// 82302B90: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82302B94: 4182001C  beq 0x82302bb0
	if ctx.cr[0].eq {
	pc = 0x82302BB0; continue 'dispatch;
	}
	// 82302B98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82302B9C: 48B56AE5  bl 0x82e59680
	ctx.lr = 0x82302BA0;
	sub_82E59680(ctx, base);
	// 82302BA0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82302BA4: 396BCA64  addi r11, r11, -0x359c
	ctx.r[11].s64 = ctx.r[11].s64 + -13724;
	// 82302BA8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82302BAC: 48000008  b 0x82302bb4
	pc = 0x82302BB4; continue 'dispatch;
	// 82302BB0: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82302BB4: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82302BB8: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 82302BBC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82302BC0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82302BC4: 4BFFEA45  bl 0x82301608
	ctx.lr = 0x82302BC8;
	sub_82301608(ctx, base);
	// 82302BC8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82302BCC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82302BD0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82302BD4: 4BFBD42D  bl 0x822c0000
	ctx.lr = 0x82302BD8;
	sub_822C0000(ctx, base);
	// 82302BD8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82302BDC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82302BE0: 48EA55DC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82302BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82302BE8 size=116
    let mut pc: u32 = 0x82302BE8;
    'dispatch: loop {
        match pc {
            0x82302BE8 => {
    //   block [0x82302BE8..0x82302C5C)
	// 82302BE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82302BEC: 48EA557D  bl 0x831a8168
	ctx.lr = 0x82302BF0;
	sub_831A8130(ctx, base);
	// 82302BF0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82302BF4: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82302BF8: 81260004  lwz r9, 4(r6)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) } as u64;
	// 82302BFC: 80E50000  lwz r7, 0(r5)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82302C00: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82302C04: 81460000  lwz r10, 0(r6)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82302C08: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 82302C0C: 83C50004  lwz r30, 4(r5)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 82302C10: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82302C14: 83A40000  lwz r29, 0(r4)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82302C18: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82302C1C: 83840004  lwz r28, 4(r4)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82302C20: 38810068  addi r4, r1, 0x68
	ctx.r[4].s64 = ctx.r[1].s64 + 104;
	// 82302C24: 990B0000  stb r8, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u8 ) };
	// 82302C28: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82302C2C: 90E10060  stw r7, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[7].u32 ) };
	// 82302C30: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82302C34: 93C10064  stw r30, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[30].u32 ) };
	// 82302C38: 93A10068  stw r29, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[29].u32 ) };
	// 82302C3C: 9381006C  stw r28, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[28].u32 ) };
	// 82302C40: 89010050  lbz r8, 0x50(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82302C44: 88E10050  lbz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82302C48: 89210050  lbz r9, 0x50(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82302C4C: 4BFFF19D  bl 0x82301de8
	ctx.lr = 0x82302C50;
	sub_82301DE8(ctx, base);
	// 82302C50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82302C54: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82302C58: 48EA5560  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82302C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82302C60 size=676
    let mut pc: u32 = 0x82302C60;
    'dispatch: loop {
        match pc {
            0x82302C60 => {
    //   block [0x82302C60..0x82302F04)
	// 82302C60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82302C64: 48EA5501  bl 0x831a8164
	ctx.lr = 0x82302C68;
	sub_831A8130(ctx, base);
	// 82302C68: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82302C6C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82302C70: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82302C74: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82302C78: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82302C7C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82302C80: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82302C84: 482883CD  bl 0x8258b050
	ctx.lr = 0x82302C88;
	sub_8258B050(ctx, base);
	// 82302C88: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82302C8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82302C90: 4820C729  bl 0x8250f3b8
	ctx.lr = 0x82302C94;
	sub_8250F3B8(ctx, base);
	// 82302C94: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82302C98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82302C9C: 4820C6D5  bl 0x8250f370
	ctx.lr = 0x82302CA0;
	sub_8250F370(ctx, base);
	// 82302CA0: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 82302CA4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82302CA8: 808BE250  lwz r4, -0x1db0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-7600 as u32) ) } as u64;
	// 82302CAC: 48AF0D5D  bl 0x82df3a08
	ctx.lr = 0x82302CB0;
	sub_82DF3A08(ctx, base);
	// 82302CB0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82302CB4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82302CB8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82302CBC: 48205AC5  bl 0x82508780
	ctx.lr = 0x82302CC0;
	sub_82508780(ctx, base);
	// 82302CC0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82302CC4: 48AF0765  bl 0x82df3428
	ctx.lr = 0x82302CC8;
	sub_82DF3428(ctx, base);
	// 82302CC8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82302CCC: 389F0028  addi r4, r31, 0x28
	ctx.r[4].s64 = ctx.r[31].s64 + 40;
	// 82302CD0: 409A0008  bne cr6, 0x82302cd8
	if !ctx.cr[6].eq {
	pc = 0x82302CD8; continue 'dispatch;
	}
	// 82302CD4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82302CD8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82302CDC: 48205AC5  bl 0x825087a0
	ctx.lr = 0x82302CE0;
	sub_825087A0(ctx, base);
	// 82302CE0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82302CE4: 3BBF0028  addi r29, r31, 0x28
	ctx.r[29].s64 = ctx.r[31].s64 + 40;
	// 82302CE8: 409A0008  bne cr6, 0x82302cf0
	if !ctx.cr[6].eq {
	pc = 0x82302CF0; continue 'dispatch;
	}
	// 82302CEC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82302CF0: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 82302CF4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82302CF8: 808BD070  lwz r4, -0x2f90(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12176 as u32) ) } as u64;
	// 82302CFC: 48AF0D0D  bl 0x82df3a08
	ctx.lr = 0x82302D00;
	sub_82DF3A08(ctx, base);
	// 82302D00: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82302D04: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82302D08: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82302D0C: 48205AE5  bl 0x825087f0
	ctx.lr = 0x82302D10;
	sub_825087F0(ctx, base);
	// 82302D10: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82302D14: 48AF0715  bl 0x82df3428
	ctx.lr = 0x82302D18;
	sub_82DF3428(ctx, base);
	// 82302D18: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82302D1C: 80BC0000  lwz r5, 0(r28)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82302D20: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82302D24: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82302D28: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82302D2C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82302D30: 4E800421  bctrl
	ctx.lr = 0x82302D34;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82302D34: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82302D38: 809F00C4  lwz r4, 0xc4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 82302D3C: 4800605D  bl 0x82308d98
	ctx.lr = 0x82302D40;
	sub_82308D98(ctx, base);
	// 82302D40: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82302D44: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82302D48: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82302D4C: 80AB0000  lwz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82302D50: 48205E51  bl 0x82508ba0
	ctx.lr = 0x82302D54;
	sub_82508BA0(ctx, base);
	// 82302D54: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82302D58: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82302D5C: 419A0008  beq cr6, 0x82302d64
	if ctx.cr[6].eq {
	pc = 0x82302D64; continue 'dispatch;
	}
	// 82302D60: 4BFBDB31  bl 0x822c0890
	ctx.lr = 0x82302D64;
	sub_822C0890(ctx, base);
	// 82302D64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82302D68: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82302D6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82302D70: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82302D74: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82302D78: 48844129  bl 0x82b46ea0
	ctx.lr = 0x82302D7C;
	sub_82B46EA0(ctx, base);
	// 82302D7C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82302D80: 395F024C  addi r10, r31, 0x24c
	ctx.r[10].s64 = ctx.r[31].s64 + 588;
	// 82302D84: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 82302D88: 386A0004  addi r3, r10, 4
	ctx.r[3].s64 = ctx.r[10].s64 + 4;
	// 82302D8C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82302D90: 917F024C  stw r11, 0x24c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(588 as u32), ctx.r[11].u32 ) };
	// 82302D94: 4BFC16CD  bl 0x822c4460
	ctx.lr = 0x82302D98;
	sub_822C4460(ctx, base);
	// 82302D98: 8061006C  lwz r3, 0x6c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 82302D9C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82302DA0: 419A0008  beq cr6, 0x82302da8
	if ctx.cr[6].eq {
	pc = 0x82302DA8; continue 'dispatch;
	}
	// 82302DA4: 4BFBDAED  bl 0x822c0890
	ctx.lr = 0x82302DA8;
	sub_822C0890(ctx, base);
	// 82302DA8: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82302DAC: 4BFFFAA5  bl 0x82302850
	ctx.lr = 0x82302DB0;
	sub_82302850(ctx, base);
	// 82302DB0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82302DB4: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82302DB8: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82302DBC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82302DC0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82302DC4: 419A0024  beq cr6, 0x82302de8
	if ctx.cr[6].eq {
	pc = 0x82302DE8; continue 'dispatch;
	}
	// 82302DC8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82302DCC: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82302DD0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82302DD4: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82302DD8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82302DDC: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82302DE0: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82302DE4: 4082FFE8  bne 0x82302dcc
	if !ctx.cr[0].eq {
	pc = 0x82302DCC; continue 'dispatch;
	}
	// 82302DE8: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82302DEC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82302DF0: 386BFF6C  addi r3, r11, -0x94
	ctx.r[3].s64 = ctx.r[11].s64 + -148;
	// 82302DF4: 409A0008  bne cr6, 0x82302dfc
	if !ctx.cr[6].eq {
	pc = 0x82302DFC; continue 'dispatch;
	}
	// 82302DF8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82302DFC: 3BA10058  addi r29, r1, 0x58
	ctx.r[29].s64 = ctx.r[1].s64 + 88;
	// 82302E00: 48225251  bl 0x82528050
	ctx.lr = 0x82302E04;
	sub_82528050(ctx, base);
	// 82302E04: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82302E08: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82302E0C: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82302E10: 388ACB60  addi r4, r10, -0x34a0
	ctx.r[4].s64 = ctx.r[10].s64 + -13472;
	// 82302E14: 38A00099  li r5, 0x99
	ctx.r[5].s64 = 153;
	// 82302E18: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 82302E1C: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82302E20: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 82302E24: 48B55C1D  bl 0x82e58a40
	ctx.lr = 0x82302E28;
	sub_82E58A40(ctx, base);
	// 82302E28: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82302E2C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82302E30: 419A0008  beq cr6, 0x82302e38
	if ctx.cr[6].eq {
	pc = 0x82302E38; continue 'dispatch;
	}
	// 82302E34: 4BFBDA5D  bl 0x822c0890
	ctx.lr = 0x82302E38;
	sub_822C0890(ctx, base);
	// 82302E38: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82302E3C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82302E40: 419A0008  beq cr6, 0x82302e48
	if ctx.cr[6].eq {
	pc = 0x82302E48; continue 'dispatch;
	}
	// 82302E44: 4BFBDA4D  bl 0x822c0890
	ctx.lr = 0x82302E48;
	sub_822C0890(ctx, base);
	// 82302E48: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82302E4C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82302E50: 388BCBA4  addi r4, r11, -0x345c
	ctx.r[4].s64 = ctx.r[11].s64 + -13404;
	// 82302E54: 48AF0BB5  bl 0x82df3a08
	ctx.lr = 0x82302E58;
	sub_82DF3A08(ctx, base);
	// 82302E58: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82302E5C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82302E60: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82302E64: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 82302E68: 48207641  bl 0x8250a4a8
	ctx.lr = 0x82302E6C;
	sub_8250A4A8(ctx, base);
	// 82302E6C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82302E70: 3BDF025C  addi r30, r31, 0x25c
	ctx.r[30].s64 = ctx.r[31].s64 + 604;
	// 82302E74: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 82302E78: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 82302E7C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82302E80: 917F025C  stw r11, 0x25c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(604 as u32), ctx.r[11].u32 ) };
	// 82302E84: 4BFC15DD  bl 0x822c4460
	ctx.lr = 0x82302E88;
	sub_822C4460(ctx, base);
	// 82302E88: 8061007C  lwz r3, 0x7c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82302E8C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82302E90: 419A0008  beq cr6, 0x82302e98
	if ctx.cr[6].eq {
	pc = 0x82302E98; continue 'dispatch;
	}
	// 82302E94: 4BFBD9FD  bl 0x822c0890
	ctx.lr = 0x82302E98;
	sub_822C0890(ctx, base);
	// 82302E98: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82302E9C: 48AF058D  bl 0x82df3428
	ctx.lr = 0x82302EA0;
	sub_82DF3428(ctx, base);
	// 82302EA0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82302EA4: 38610088  addi r3, r1, 0x88
	ctx.r[3].s64 = ctx.r[1].s64 + 136;
	// 82302EA8: 83BE0000  lwz r29, 0(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82302EAC: 4820C485  bl 0x8250f330
	ctx.lr = 0x82302EB0;
	sub_8250F330(ctx, base);
	// 82302EB0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82302EB4: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82302EB8: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82302EBC: 481E8405  bl 0x824eb2c0
	ctx.lr = 0x82302EC0;
	sub_824EB2C0(ctx, base);
	// 82302EC0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82302EC4: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82302EC8: 48214FD9  bl 0x82517ea0
	ctx.lr = 0x82302ECC;
	sub_82517EA0(ctx, base);
	// 82302ECC: 80610084  lwz r3, 0x84(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82302ED0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82302ED4: 419A0008  beq cr6, 0x82302edc
	if ctx.cr[6].eq {
	pc = 0x82302EDC; continue 'dispatch;
	}
	// 82302ED8: 4BFBD9B9  bl 0x822c0890
	ctx.lr = 0x82302EDC;
	sub_822C0890(ctx, base);
	// 82302EDC: 38610088  addi r3, r1, 0x88
	ctx.r[3].s64 = ctx.r[1].s64 + 136;
	// 82302EE0: 48AEEDB1  bl 0x82df1c90
	ctx.lr = 0x82302EE4;
	sub_82DF1C90(ctx, base);
	// 82302EE4: 807F00C4  lwz r3, 0xc4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 82302EE8: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82302EEC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82302EF0: 816B0058  lwz r11, 0x58(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(88 as u32) ) } as u64;
	// 82302EF4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82302EF8: 4E800421  bctrl
	ctx.lr = 0x82302EFC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82302EFC: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82302F00: 48EA52B4  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82302F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82302F08 size=260
    let mut pc: u32 = 0x82302F08;
    'dispatch: loop {
        match pc {
            0x82302F08 => {
    //   block [0x82302F08..0x8230300C)
	// 82302F08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82302F0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82302F10: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82302F14: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82302F18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82302F1C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82302F20: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82302F24: 397F00C4  addi r11, r31, 0xc4
	ctx.r[11].s64 = ctx.r[31].s64 + 196;
	// 82302F28: 93DF00C4  stw r30, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[30].u32 ) };
	// 82302F2C: 807F00C8  lwz r3, 0xc8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(200 as u32) ) } as u64;
	// 82302F30: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82302F34: 93DF00C8  stw r30, 0xc8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(200 as u32), ctx.r[30].u32 ) };
	// 82302F38: 419A0008  beq cr6, 0x82302f40
	if ctx.cr[6].eq {
	pc = 0x82302F40; continue 'dispatch;
	}
	// 82302F3C: 4BFBD955  bl 0x822c0890
	ctx.lr = 0x82302F40;
	sub_822C0890(ctx, base);
	// 82302F40: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82302F44: 4BFFF98D  bl 0x823028d0
	ctx.lr = 0x82302F48;
	sub_823028D0(ctx, base);
	// 82302F48: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82302F4C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82302F50: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82302F54: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82302F58: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82302F5C: 419A0024  beq cr6, 0x82302f80
	if ctx.cr[6].eq {
	pc = 0x82302F80; continue 'dispatch;
	}
	// 82302F60: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82302F64: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82302F68: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82302F6C: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82302F70: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82302F74: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82302F78: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82302F7C: 4082FFE8  bne 0x82302f64
	if !ctx.cr[0].eq {
	pc = 0x82302F64; continue 'dispatch;
	}
	// 82302F80: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82302F84: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82302F88: 4820C591  bl 0x8250f518
	ctx.lr = 0x82302F8C;
	sub_8250F518(ctx, base);
	// 82302F8C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82302F90: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82302F94: 386BFF6C  addi r3, r11, -0x94
	ctx.r[3].s64 = ctx.r[11].s64 + -148;
	// 82302F98: 409A0008  bne cr6, 0x82302fa0
	if !ctx.cr[6].eq {
	pc = 0x82302FA0; continue 'dispatch;
	}
	// 82302F9C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82302FA0: 3BC10050  addi r30, r1, 0x50
	ctx.r[30].s64 = ctx.r[1].s64 + 80;
	// 82302FA4: 482250AD  bl 0x82528050
	ctx.lr = 0x82302FA8;
	sub_82528050(ctx, base);
	// 82302FA8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82302FAC: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82302FB0: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82302FB4: 388ACB60  addi r4, r10, -0x34a0
	ctx.r[4].s64 = ctx.r[10].s64 + -13472;
	// 82302FB8: 38A000AD  li r5, 0xad
	ctx.r[5].s64 = 173;
	// 82302FBC: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 82302FC0: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82302FC4: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 82302FC8: 48B55A79  bl 0x82e58a40
	ctx.lr = 0x82302FCC;
	sub_82E58A40(ctx, base);
	// 82302FCC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82302FD0: 48AEECC1  bl 0x82df1c90
	ctx.lr = 0x82302FD4;
	sub_82DF1C90(ctx, base);
	// 82302FD4: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82302FD8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82302FDC: 419A0008  beq cr6, 0x82302fe4
	if ctx.cr[6].eq {
	pc = 0x82302FE4; continue 'dispatch;
	}
	// 82302FE0: 4BFBD8B1  bl 0x822c0890
	ctx.lr = 0x82302FE4;
	sub_822C0890(ctx, base);
	// 82302FE4: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82302FE8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82302FEC: 419A0008  beq cr6, 0x82302ff4
	if ctx.cr[6].eq {
	pc = 0x82302FF4; continue 'dispatch;
	}
	// 82302FF0: 4BFBD8A1  bl 0x822c0890
	ctx.lr = 0x82302FF4;
	sub_822C0890(ctx, base);
	// 82302FF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82302FF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82302FFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82303000: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82303004: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82303008: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82303010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82303010 size=272
    let mut pc: u32 = 0x82303010;
    'dispatch: loop {
        match pc {
            0x82303010 => {
    //   block [0x82303010..0x82303120)
	// 82303010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82303014: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82303018: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8230301C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82303020: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82303024: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82303028: 817E01AC  lwz r11, 0x1ac(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(428 as u32) ) } as u64;
	// 8230302C: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82303030: 409900D8  ble cr6, 0x82303108
	if !ctx.cr[6].gt {
	pc = 0x82303108; continue 'dispatch;
	}
	// 82303034: 3BFE019C  addi r31, r30, 0x19c
	ctx.r[31].s64 = ctx.r[30].s64 + 412;
	// 82303038: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8230303C: 4BFFF78D  bl 0x823027c8
	ctx.lr = 0x82303040;
	sub_823027C8(ctx, base);
	// 82303040: 817E01AC  lwz r11, 0x1ac(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(428 as u32) ) } as u64;
	// 82303044: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82303048: 419A00AC  beq cr6, 0x823030f4
	if ctx.cr[6].eq {
	pc = 0x823030F4; continue 'dispatch;
	}
	// 8230304C: 813F000C  lwz r9, 0xc(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82303050: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82303054: 552BF87E  srwi r11, r9, 1
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shr(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82303058: 552907FE  clrlwi r9, r9, 0x1f
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x00000001u64;
	// 8230305C: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82303060: 41990008  bgt cr6, 0x82303068
	if ctx.cr[6].gt {
	pc = 0x82303068; continue 'dispatch;
	}
	// 82303064: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82303068: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8230306C: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82303070: 552B1838  slwi r11, r9, 3
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82303074: 7D48502E  lwzx r10, r8, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82303078: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8230307C: 896B0004  lbz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82303080: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82303084: 41820070  beq 0x823030f4
	if ctx.cr[0].eq {
	pc = 0x823030F4; continue 'dispatch;
	}
	// 82303088: 813F000C  lwz r9, 0xc(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8230308C: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82303090: 552BF87E  srwi r11, r9, 1
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shr(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82303094: 552907FE  clrlwi r9, r9, 0x1f
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x00000001u64;
	// 82303098: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8230309C: 41990008  bgt cr6, 0x823030a4
	if ctx.cr[6].gt {
	pc = 0x823030A4; continue 'dispatch;
	}
	// 823030A0: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 823030A4: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 823030A8: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 823030AC: 552B1838  slwi r11, r9, 3
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823030B0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823030B4: 7D48502E  lwzx r10, r8, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 823030B8: 7C8A5A14  add r4, r10, r11
	ctx.r[4].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 823030BC: 48AF0B45  bl 0x82df3c00
	ctx.lr = 0x823030C0;
	sub_82DF3C00(ctx, base);
	// 823030C0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 823030C4: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 823030C8: 389E013C  addi r4, r30, 0x13c
	ctx.r[4].s64 = ctx.r[30].s64 + 316;
	// 823030CC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 823030D0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823030D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 823030D8: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 823030DC: 48B5B885  bl 0x82e5e960
	ctx.lr = 0x823030E0;
	sub_82E5E960(ctx, base);
	// 823030E0: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 823030E4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823030E8: 419A0020  beq cr6, 0x82303108
	if ctx.cr[6].eq {
	pc = 0x82303108; continue 'dispatch;
	}
	// 823030EC: 4BFBD7A5  bl 0x822c0890
	ctx.lr = 0x823030F0;
	sub_822C0890(ctx, base);
	// 823030F0: 48000018  b 0x82303108
	pc = 0x82303108; continue 'dispatch;
	// 823030F4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 823030F8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 823030FC: 387E013C  addi r3, r30, 0x13c
	ctx.r[3].s64 = ctx.r[30].s64 + 316;
	// 82303100: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82303104: 48B594F5  bl 0x82e5c5f8
	ctx.lr = 0x82303108;
	sub_82E5C5F8(ctx, base);
	// 82303108: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8230310C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82303110: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82303114: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82303118: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8230311C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82303120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82303120 size=424
    let mut pc: u32 = 0x82303120;
    'dispatch: loop {
        match pc {
            0x82303120 => {
    //   block [0x82303120..0x823032C8)
	// 82303120: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82303124: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82303128: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8230312C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82303130: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82303134: 98C100CF  stb r6, 0xcf(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(207 as u32), ctx.r[6].u8 ) };
	// 82303138: 54EB063F  clrlwi. r11, r7, 0x18
	ctx.r[11].u64 = ctx.r[7].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8230313C: D02100C4  stfs f1, 0xc4(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(196 as u32), tmp.u32 ) };
	// 82303140: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82303144: 910100DC  stw r8, 0xdc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(220 as u32), ctx.r[8].u32 ) };
	// 82303148: 38E100DC  addi r7, r1, 0xdc
	ctx.r[7].s64 = ctx.r[1].s64 + 220;
	// 8230314C: 38C100CF  addi r6, r1, 0xcf
	ctx.r[6].s64 = ctx.r[1].s64 + 207;
	// 82303150: 38A100C4  addi r5, r1, 0xc4
	ctx.r[5].s64 = ctx.r[1].s64 + 196;
	// 82303154: 418200B0  beq 0x82303204
	if ctx.cr[0].eq {
	pc = 0x82303204; continue 'dispatch;
	}
	// 82303158: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8230315C: 4BFFF7F5  bl 0x82302950
	ctx.lr = 0x82303160;
	sub_82302950(ctx, base);
	// 82303160: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82303164: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82303168: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8230316C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82303170: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82303174: 419A0024  beq cr6, 0x82303198
	if ctx.cr[6].eq {
	pc = 0x82303198; continue 'dispatch;
	}
	// 82303178: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8230317C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82303180: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82303184: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82303188: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8230318C: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82303190: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82303194: 4082FFE8  bne 0x8230317c
	if !ctx.cr[0].eq {
	pc = 0x8230317C; continue 'dispatch;
	}
	// 82303198: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8230319C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 823031A0: 4820C379  bl 0x8250f518
	ctx.lr = 0x823031A4;
	sub_8250F518(ctx, base);
	// 823031A4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 823031A8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823031AC: 386BFF6C  addi r3, r11, -0x94
	ctx.r[3].s64 = ctx.r[11].s64 + -148;
	// 823031B0: 409A0008  bne cr6, 0x823031b8
	if !ctx.cr[6].eq {
	pc = 0x823031B8; continue 'dispatch;
	}
	// 823031B4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 823031B8: 3BC10050  addi r30, r1, 0x50
	ctx.r[30].s64 = ctx.r[1].s64 + 80;
	// 823031BC: 48224E95  bl 0x82528050
	ctx.lr = 0x823031C0;
	sub_82528050(ctx, base);
	// 823031C0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 823031C4: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 823031C8: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 823031CC: 388ACB60  addi r4, r10, -0x34a0
	ctx.r[4].s64 = ctx.r[10].s64 + -13472;
	// 823031D0: 38A003FC  li r5, 0x3fc
	ctx.r[5].s64 = 1020;
	// 823031D4: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 823031D8: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 823031DC: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 823031E0: 48B55861  bl 0x82e58a40
	ctx.lr = 0x823031E4;
	sub_82E58A40(ctx, base);
	// 823031E4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 823031E8: 48AEEAA9  bl 0x82df1c90
	ctx.lr = 0x823031EC;
	sub_82DF1C90(ctx, base);
	// 823031EC: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 823031F0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823031F4: 419A0008  beq cr6, 0x823031fc
	if ctx.cr[6].eq {
	pc = 0x823031FC; continue 'dispatch;
	}
	// 823031F8: 4BFBD699  bl 0x822c0890
	ctx.lr = 0x823031FC;
	sub_822C0890(ctx, base);
	// 823031FC: 8061006C  lwz r3, 0x6c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 82303200: 480000A4  b 0x823032a4
	pc = 0x823032A4; continue 'dispatch;
	// 82303204: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 82303208: 4BFFF749  bl 0x82302950
	ctx.lr = 0x8230320C;
	sub_82302950(ctx, base);
	// 8230320C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82303210: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82303214: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82303218: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8230321C: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82303220: 419A0024  beq cr6, 0x82303244
	if ctx.cr[6].eq {
	pc = 0x82303244; continue 'dispatch;
	}
	// 82303224: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82303228: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8230322C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82303230: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82303234: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82303238: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8230323C: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82303240: 4082FFE8  bne 0x82303228
	if !ctx.cr[0].eq {
	pc = 0x82303228; continue 'dispatch;
	}
	// 82303244: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82303248: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8230324C: 4820C2CD  bl 0x8250f518
	ctx.lr = 0x82303250;
	sub_8250F518(ctx, base);
	// 82303250: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82303254: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82303258: 386BFF6C  addi r3, r11, -0x94
	ctx.r[3].s64 = ctx.r[11].s64 + -148;
	// 8230325C: 409A0008  bne cr6, 0x82303264
	if !ctx.cr[6].eq {
	pc = 0x82303264; continue 'dispatch;
	}
	// 82303260: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82303264: 3BC10058  addi r30, r1, 0x58
	ctx.r[30].s64 = ctx.r[1].s64 + 88;
	// 82303268: 48224DE9  bl 0x82528050
	ctx.lr = 0x8230326C;
	sub_82528050(ctx, base);
	// 8230326C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82303270: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82303274: 388BCB60  addi r4, r11, -0x34a0
	ctx.r[4].s64 = ctx.r[11].s64 + -13472;
	// 82303278: 38A003FF  li r5, 0x3ff
	ctx.r[5].s64 = 1023;
	// 8230327C: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 82303280: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 82303284: 48B53D65  bl 0x82e56fe8
	ctx.lr = 0x82303288;
	sub_82E56FE8(ctx, base);
	// 82303288: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8230328C: 48AEEA05  bl 0x82df1c90
	ctx.lr = 0x82303290;
	sub_82DF1C90(ctx, base);
	// 82303290: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82303294: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82303298: 419A0008  beq cr6, 0x823032a0
	if ctx.cr[6].eq {
	pc = 0x823032A0; continue 'dispatch;
	}
	// 8230329C: 4BFBD5F5  bl 0x822c0890
	ctx.lr = 0x823032A0;
	sub_822C0890(ctx, base);
	// 823032A0: 8061007C  lwz r3, 0x7c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 823032A4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823032A8: 419A0008  beq cr6, 0x823032b0
	if ctx.cr[6].eq {
	pc = 0x823032B0; continue 'dispatch;
	}
	// 823032AC: 4BFBD5E5  bl 0x822c0890
	ctx.lr = 0x823032B0;
	sub_822C0890(ctx, base);
	// 823032B0: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 823032B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823032B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823032BC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823032C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823032C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823032C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x823032C8 size=416
    let mut pc: u32 = 0x823032C8;
    'dispatch: loop {
        match pc {
            0x823032C8 => {
    //   block [0x823032C8..0x82303468)
	// 823032C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823032CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823032D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823032D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823032D8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823032DC: 54CB063F  clrlwi. r11, r6, 0x18
	ctx.r[11].u64 = ctx.r[6].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823032E0: D02100C4  stfs f1, 0xc4(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(196 as u32), tmp.u32 ) };
	// 823032E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823032E8: 90E100D4  stw r7, 0xd4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(212 as u32), ctx.r[7].u32 ) };
	// 823032EC: 38C100D4  addi r6, r1, 0xd4
	ctx.r[6].s64 = ctx.r[1].s64 + 212;
	// 823032F0: 38A100C4  addi r5, r1, 0xc4
	ctx.r[5].s64 = ctx.r[1].s64 + 196;
	// 823032F4: 418200B0  beq 0x823033a4
	if ctx.cr[0].eq {
	pc = 0x823033A4; continue 'dispatch;
	}
	// 823032F8: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 823032FC: 4836645D  bl 0x82669758
	ctx.lr = 0x82303300;
	sub_82669758(ctx, base);
	// 82303300: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82303304: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82303308: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8230330C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82303310: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82303314: 419A0024  beq cr6, 0x82303338
	if ctx.cr[6].eq {
	pc = 0x82303338; continue 'dispatch;
	}
	// 82303318: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8230331C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82303320: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82303324: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82303328: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8230332C: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82303330: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82303334: 4082FFE8  bne 0x8230331c
	if !ctx.cr[0].eq {
	pc = 0x8230331C; continue 'dispatch;
	}
	// 82303338: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8230333C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82303340: 4820C1D9  bl 0x8250f518
	ctx.lr = 0x82303344;
	sub_8250F518(ctx, base);
	// 82303344: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82303348: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8230334C: 386BFF6C  addi r3, r11, -0x94
	ctx.r[3].s64 = ctx.r[11].s64 + -148;
	// 82303350: 409A0008  bne cr6, 0x82303358
	if !ctx.cr[6].eq {
	pc = 0x82303358; continue 'dispatch;
	}
	// 82303354: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82303358: 3BC10050  addi r30, r1, 0x50
	ctx.r[30].s64 = ctx.r[1].s64 + 80;
	// 8230335C: 48224CF5  bl 0x82528050
	ctx.lr = 0x82303360;
	sub_82528050(ctx, base);
	// 82303360: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82303364: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82303368: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 8230336C: 388ACB60  addi r4, r10, -0x34a0
	ctx.r[4].s64 = ctx.r[10].s64 + -13472;
	// 82303370: 38A00420  li r5, 0x420
	ctx.r[5].s64 = 1056;
	// 82303374: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 82303378: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8230337C: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 82303380: 48B556C1  bl 0x82e58a40
	ctx.lr = 0x82303384;
	sub_82E58A40(ctx, base);
	// 82303384: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82303388: 48AEE909  bl 0x82df1c90
	ctx.lr = 0x8230338C;
	sub_82DF1C90(ctx, base);
	// 8230338C: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82303390: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82303394: 419A0008  beq cr6, 0x8230339c
	if ctx.cr[6].eq {
	pc = 0x8230339C; continue 'dispatch;
	}
	// 82303398: 4BFBD4F9  bl 0x822c0890
	ctx.lr = 0x8230339C;
	sub_822C0890(ctx, base);
	// 8230339C: 8061006C  lwz r3, 0x6c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 823033A0: 480000A4  b 0x82303444
	pc = 0x82303444; continue 'dispatch;
	// 823033A4: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 823033A8: 483663B1  bl 0x82669758
	ctx.lr = 0x823033AC;
	sub_82669758(ctx, base);
	// 823033AC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 823033B0: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 823033B4: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 823033B8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823033BC: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 823033C0: 419A0024  beq cr6, 0x823033e4
	if ctx.cr[6].eq {
	pc = 0x823033E4; continue 'dispatch;
	}
	// 823033C4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 823033C8: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 823033CC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 823033D0: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 823033D4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 823033D8: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 823033DC: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 823033E0: 4082FFE8  bne 0x823033c8
	if !ctx.cr[0].eq {
	pc = 0x823033C8; continue 'dispatch;
	}
	// 823033E4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823033E8: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 823033EC: 4820C12D  bl 0x8250f518
	ctx.lr = 0x823033F0;
	sub_8250F518(ctx, base);
	// 823033F0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 823033F4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823033F8: 386BFF6C  addi r3, r11, -0x94
	ctx.r[3].s64 = ctx.r[11].s64 + -148;
	// 823033FC: 409A0008  bne cr6, 0x82303404
	if !ctx.cr[6].eq {
	pc = 0x82303404; continue 'dispatch;
	}
	// 82303400: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82303404: 3BC10058  addi r30, r1, 0x58
	ctx.r[30].s64 = ctx.r[1].s64 + 88;
	// 82303408: 48224C49  bl 0x82528050
	ctx.lr = 0x8230340C;
	sub_82528050(ctx, base);
	// 8230340C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82303410: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82303414: 388BCB60  addi r4, r11, -0x34a0
	ctx.r[4].s64 = ctx.r[11].s64 + -13472;
	// 82303418: 38A00423  li r5, 0x423
	ctx.r[5].s64 = 1059;
	// 8230341C: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 82303420: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 82303424: 48B53BC5  bl 0x82e56fe8
	ctx.lr = 0x82303428;
	sub_82E56FE8(ctx, base);
	// 82303428: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8230342C: 48AEE865  bl 0x82df1c90
	ctx.lr = 0x82303430;
	sub_82DF1C90(ctx, base);
	// 82303430: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82303434: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82303438: 419A0008  beq cr6, 0x82303440
	if ctx.cr[6].eq {
	pc = 0x82303440; continue 'dispatch;
	}
	// 8230343C: 4BFBD455  bl 0x822c0890
	ctx.lr = 0x82303440;
	sub_822C0890(ctx, base);
	// 82303440: 8061007C  lwz r3, 0x7c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82303444: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82303448: 419A0008  beq cr6, 0x82303450
	if ctx.cr[6].eq {
	pc = 0x82303450; continue 'dispatch;
	}
	// 8230344C: 4BFBD445  bl 0x822c0890
	ctx.lr = 0x82303450;
	sub_822C0890(ctx, base);
	// 82303450: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82303454: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82303458: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8230345C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82303460: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82303464: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82303468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82303468 size=420
    let mut pc: u32 = 0x82303468;
    'dispatch: loop {
        match pc {
            0x82303468 => {
    //   block [0x82303468..0x8230360C)
	// 82303468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8230346C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82303470: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82303474: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82303478: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8230347C: 54CB063F  clrlwi. r11, r6, 0x18
	ctx.r[11].u64 = ctx.r[6].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82303480: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82303484: D02100C4  stfs f1, 0xc4(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(196 as u32), tmp.u32 ) };
	// 82303488: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8230348C: 38A100C4  addi r5, r1, 0xc4
	ctx.r[5].s64 = ctx.r[1].s64 + 196;
	// 82303490: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82303494: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82303498: 418200B0  beq 0x82303548
	if ctx.cr[0].eq {
	pc = 0x82303548; continue 'dispatch;
	}
	// 8230349C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 823034A0: 48543171  bl 0x82846610
	ctx.lr = 0x823034A4;
	sub_82846610(ctx, base);
	// 823034A4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 823034A8: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 823034AC: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 823034B0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823034B4: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 823034B8: 419A0024  beq cr6, 0x823034dc
	if ctx.cr[6].eq {
	pc = 0x823034DC; continue 'dispatch;
	}
	// 823034BC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 823034C0: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 823034C4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 823034C8: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 823034CC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 823034D0: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 823034D4: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 823034D8: 4082FFE8  bne 0x823034c0
	if !ctx.cr[0].eq {
	pc = 0x823034C0; continue 'dispatch;
	}
	// 823034DC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823034E0: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 823034E4: 4820C035  bl 0x8250f518
	ctx.lr = 0x823034E8;
	sub_8250F518(ctx, base);
	// 823034E8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 823034EC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823034F0: 386BFF6C  addi r3, r11, -0x94
	ctx.r[3].s64 = ctx.r[11].s64 + -148;
	// 823034F4: 409A0008  bne cr6, 0x823034fc
	if !ctx.cr[6].eq {
	pc = 0x823034FC; continue 'dispatch;
	}
	// 823034F8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 823034FC: 3BC10058  addi r30, r1, 0x58
	ctx.r[30].s64 = ctx.r[1].s64 + 88;
	// 82303500: 48224B51  bl 0x82528050
	ctx.lr = 0x82303504;
	sub_82528050(ctx, base);
	// 82303504: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82303508: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8230350C: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82303510: 388ACB60  addi r4, r10, -0x34a0
	ctx.r[4].s64 = ctx.r[10].s64 + -13472;
	// 82303514: 38A00432  li r5, 0x432
	ctx.r[5].s64 = 1074;
	// 82303518: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 8230351C: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82303520: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 82303524: 48B5551D  bl 0x82e58a40
	ctx.lr = 0x82303528;
	sub_82E58A40(ctx, base);
	// 82303528: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8230352C: 48AEE765  bl 0x82df1c90
	ctx.lr = 0x82303530;
	sub_82DF1C90(ctx, base);
	// 82303530: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82303534: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82303538: 419A0008  beq cr6, 0x82303540
	if ctx.cr[6].eq {
	pc = 0x82303540; continue 'dispatch;
	}
	// 8230353C: 4BFBD355  bl 0x822c0890
	ctx.lr = 0x82303540;
	sub_822C0890(ctx, base);
	// 82303540: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82303544: 480000A4  b 0x823035e8
	pc = 0x823035E8; continue 'dispatch;
	// 82303548: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 8230354C: 485430C5  bl 0x82846610
	ctx.lr = 0x82303550;
	sub_82846610(ctx, base);
	// 82303550: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82303554: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82303558: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8230355C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82303560: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82303564: 419A0024  beq cr6, 0x82303588
	if ctx.cr[6].eq {
	pc = 0x82303588; continue 'dispatch;
	}
	// 82303568: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8230356C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82303570: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82303574: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82303578: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8230357C: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82303580: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82303584: 4082FFE8  bne 0x8230356c
	if !ctx.cr[0].eq {
	pc = 0x8230356C; continue 'dispatch;
	}
	// 82303588: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8230358C: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 82303590: 4820BF89  bl 0x8250f518
	ctx.lr = 0x82303594;
	sub_8250F518(ctx, base);
	// 82303594: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82303598: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8230359C: 386BFF6C  addi r3, r11, -0x94
	ctx.r[3].s64 = ctx.r[11].s64 + -148;
	// 823035A0: 409A0008  bne cr6, 0x823035a8
	if !ctx.cr[6].eq {
	pc = 0x823035A8; continue 'dispatch;
	}
	// 823035A4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 823035A8: 3BC10060  addi r30, r1, 0x60
	ctx.r[30].s64 = ctx.r[1].s64 + 96;
	// 823035AC: 48224AA5  bl 0x82528050
	ctx.lr = 0x823035B0;
	sub_82528050(ctx, base);
	// 823035B0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823035B4: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 823035B8: 388BCB60  addi r4, r11, -0x34a0
	ctx.r[4].s64 = ctx.r[11].s64 + -13472;
	// 823035BC: 38A00435  li r5, 0x435
	ctx.r[5].s64 = 1077;
	// 823035C0: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 823035C4: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 823035C8: 48B53A21  bl 0x82e56fe8
	ctx.lr = 0x823035CC;
	sub_82E56FE8(ctx, base);
	// 823035CC: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 823035D0: 48AEE6C1  bl 0x82df1c90
	ctx.lr = 0x823035D4;
	sub_82DF1C90(ctx, base);
	// 823035D4: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 823035D8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823035DC: 419A0008  beq cr6, 0x823035e4
	if ctx.cr[6].eq {
	pc = 0x823035E4; continue 'dispatch;
	}
	// 823035E0: 4BFBD2B1  bl 0x822c0890
	ctx.lr = 0x823035E4;
	sub_822C0890(ctx, base);
	// 823035E4: 80610084  lwz r3, 0x84(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 823035E8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823035EC: 419A0008  beq cr6, 0x823035f4
	if ctx.cr[6].eq {
	pc = 0x823035F4; continue 'dispatch;
	}
	// 823035F0: 4BFBD2A1  bl 0x822c0890
	ctx.lr = 0x823035F4;
	sub_822C0890(ctx, base);
	// 823035F4: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 823035F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823035FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82303600: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82303604: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82303608: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82303610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82303610 size=464
    let mut pc: u32 = 0x82303610;
    'dispatch: loop {
        match pc {
            0x82303610 => {
    //   block [0x82303610..0x823037E0)
	// 82303610: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82303614: 48EA4B55  bl 0x831a8168
	ctx.lr = 0x82303618;
	sub_831A8130(ctx, base);
	// 82303618: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 8230361C: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82303620: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82303624: 38840020  addi r4, r4, 0x20
	ctx.r[4].s64 = ctx.r[4].s64 + 32;
	// 82303628: 807F00C4  lwz r3, 0xc4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 8230362C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82303630: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82303634: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82303638: 4E800421  bctrl
	ctx.lr = 0x8230363C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8230363C: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82303640: 4BFFF3A9  bl 0x823029e8
	ctx.lr = 0x82303644;
	sub_823029E8(ctx, base);
	// 82303644: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82303648: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8230364C: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82303650: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82303654: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82303658: 419A0024  beq cr6, 0x8230367c
	if ctx.cr[6].eq {
	pc = 0x8230367C; continue 'dispatch;
	}
	// 8230365C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82303660: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82303664: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82303668: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8230366C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82303670: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82303674: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82303678: 4082FFE8  bne 0x82303660
	if !ctx.cr[0].eq {
	pc = 0x82303660; continue 'dispatch;
	}
	// 8230367C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82303680: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82303684: 4820BE45  bl 0x8250f4c8
	ctx.lr = 0x82303688;
	sub_8250F4C8(ctx, base);
	// 82303688: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8230368C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82303690: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 82303694: 409A0008  bne cr6, 0x8230369c
	if !ctx.cr[6].eq {
	pc = 0x8230369C; continue 'dispatch;
	}
	// 82303698: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8230369C: 3BDF0028  addi r30, r31, 0x28
	ctx.r[30].s64 = ctx.r[31].s64 + 40;
	// 823036A0: 3B810050  addi r28, r1, 0x50
	ctx.r[28].s64 = ctx.r[1].s64 + 80;
	// 823036A4: 48204EC5  bl 0x82508568
	ctx.lr = 0x823036A8;
	sub_82508568(ctx, base);
	// 823036A8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823036AC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823036B0: 3BABCB60  addi r29, r11, -0x34a0
	ctx.r[29].s64 = ctx.r[11].s64 + -13472;
	// 823036B4: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 823036B8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 823036BC: 38A004C4  li r5, 0x4c4
	ctx.r[5].s64 = 1220;
	// 823036C0: C3EA08A4  lfs f31, 0x8a4(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 823036C4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823036C8: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 823036CC: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 823036D0: 48B55371  bl 0x82e58a40
	ctx.lr = 0x823036D4;
	sub_82E58A40(ctx, base);
	// 823036D4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 823036D8: 48AEE5B9  bl 0x82df1c90
	ctx.lr = 0x823036DC;
	sub_82DF1C90(ctx, base);
	// 823036DC: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 823036E0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823036E4: 419A0008  beq cr6, 0x823036ec
	if ctx.cr[6].eq {
	pc = 0x823036EC; continue 'dispatch;
	}
	// 823036E8: 4BFBD1A9  bl 0x822c0890
	ctx.lr = 0x823036EC;
	sub_822C0890(ctx, base);
	// 823036EC: 8061006C  lwz r3, 0x6c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 823036F0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823036F4: 419A0008  beq cr6, 0x823036fc
	if ctx.cr[6].eq {
	pc = 0x823036FC; continue 'dispatch;
	}
	// 823036F8: 4BFBD199  bl 0x822c0890
	ctx.lr = 0x823036FC;
	sub_822C0890(ctx, base);
	// 823036FC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82303700: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82303704: 4820BDC5  bl 0x8250f4c8
	ctx.lr = 0x82303708;
	sub_8250F4C8(ctx, base);
	// 82303708: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8230370C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82303710: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 82303714: 409A0008  bne cr6, 0x8230371c
	if !ctx.cr[6].eq {
	pc = 0x8230371C; continue 'dispatch;
	}
	// 82303718: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8230371C: 4820ABCD  bl 0x8250e2e8
	ctx.lr = 0x82303720;
	sub_8250E2E8(ctx, base);
	// 82303720: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82303724: 48AEE56D  bl 0x82df1c90
	ctx.lr = 0x82303728;
	sub_82DF1C90(ctx, base);
	// 82303728: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 8230372C: 4BFFF33D  bl 0x82302a68
	ctx.lr = 0x82303730;
	sub_82302A68(ctx, base);
	// 82303730: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82303734: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82303738: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8230373C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82303740: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82303744: 419A0024  beq cr6, 0x82303768
	if ctx.cr[6].eq {
	pc = 0x82303768; continue 'dispatch;
	}
	// 82303748: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8230374C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82303750: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82303754: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82303758: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8230375C: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82303760: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82303764: 4082FFE8  bne 0x8230374c
	if !ctx.cr[0].eq {
	pc = 0x8230374C; continue 'dispatch;
	}
	// 82303768: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8230376C: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 82303770: 4820BD59  bl 0x8250f4c8
	ctx.lr = 0x82303774;
	sub_8250F4C8(ctx, base);
	// 82303774: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82303778: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8230377C: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 82303780: 409A0008  bne cr6, 0x82303788
	if !ctx.cr[6].eq {
	pc = 0x82303788; continue 'dispatch;
	}
	// 82303784: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82303788: 3BE10058  addi r31, r1, 0x58
	ctx.r[31].s64 = ctx.r[1].s64 + 88;
	// 8230378C: 48204D25  bl 0x825084b0
	ctx.lr = 0x82303790;
	sub_825084B0(ctx, base);
	// 82303790: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82303794: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82303798: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8230379C: 38A004C8  li r5, 0x4c8
	ctx.r[5].s64 = 1224;
	// 823037A0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823037A4: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 823037A8: 48B55299  bl 0x82e58a40
	ctx.lr = 0x823037AC;
	sub_82E58A40(ctx, base);
	// 823037AC: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 823037B0: 48AEE4E1  bl 0x82df1c90
	ctx.lr = 0x823037B4;
	sub_82DF1C90(ctx, base);
	// 823037B4: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 823037B8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823037BC: 419A0008  beq cr6, 0x823037c4
	if ctx.cr[6].eq {
	pc = 0x823037C4; continue 'dispatch;
	}
	// 823037C0: 4BFBD0D1  bl 0x822c0890
	ctx.lr = 0x823037C4;
	sub_822C0890(ctx, base);
	// 823037C4: 80610084  lwz r3, 0x84(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 823037C8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823037CC: 419A0008  beq cr6, 0x823037d4
	if ctx.cr[6].eq {
	pc = 0x823037D4; continue 'dispatch;
	}
	// 823037D0: 4BFBD0C1  bl 0x822c0890
	ctx.lr = 0x823037D4;
	sub_822C0890(ctx, base);
	// 823037D4: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 823037D8: CBE1FFD0  lfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 823037DC: 48EA49DC  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823037E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x823037E0 size=228
    let mut pc: u32 = 0x823037E0;
    'dispatch: loop {
        match pc {
            0x823037E0 => {
    //   block [0x823037E0..0x823038C4)
	// 823037E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823037E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823037E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823037EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823037F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823037F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823037F8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 823037FC: 4BFFF2ED  bl 0x82302ae8
	ctx.lr = 0x82303800;
	sub_82302AE8(ctx, base);
	// 82303800: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82303804: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82303808: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8230380C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82303810: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82303814: 419A0024  beq cr6, 0x82303838
	if ctx.cr[6].eq {
	pc = 0x82303838; continue 'dispatch;
	}
	// 82303818: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8230381C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82303820: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82303824: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82303828: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8230382C: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82303830: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82303834: 4082FFE8  bne 0x8230381c
	if !ctx.cr[0].eq {
	pc = 0x8230381C; continue 'dispatch;
	}
	// 82303838: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8230383C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82303840: 4820BCD9  bl 0x8250f518
	ctx.lr = 0x82303844;
	sub_8250F518(ctx, base);
	// 82303844: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82303848: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8230384C: 386BFF6C  addi r3, r11, -0x94
	ctx.r[3].s64 = ctx.r[11].s64 + -148;
	// 82303850: 409A0008  bne cr6, 0x82303858
	if !ctx.cr[6].eq {
	pc = 0x82303858; continue 'dispatch;
	}
	// 82303854: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82303858: 3BC10050  addi r30, r1, 0x50
	ctx.r[30].s64 = ctx.r[1].s64 + 80;
	// 8230385C: 482247F5  bl 0x82528050
	ctx.lr = 0x82303860;
	sub_82528050(ctx, base);
	// 82303860: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82303864: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82303868: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 8230386C: 388ACB60  addi r4, r10, -0x34a0
	ctx.r[4].s64 = ctx.r[10].s64 + -13472;
	// 82303870: 38A004F9  li r5, 0x4f9
	ctx.r[5].s64 = 1273;
	// 82303874: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 82303878: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8230387C: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 82303880: 48B551C1  bl 0x82e58a40
	ctx.lr = 0x82303884;
	sub_82E58A40(ctx, base);
	// 82303884: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82303888: 48AEE409  bl 0x82df1c90
	ctx.lr = 0x8230388C;
	sub_82DF1C90(ctx, base);
	// 8230388C: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82303890: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82303894: 419A0008  beq cr6, 0x8230389c
	if ctx.cr[6].eq {
	pc = 0x8230389C; continue 'dispatch;
	}
	// 82303898: 4BFBCFF9  bl 0x822c0890
	ctx.lr = 0x8230389C;
	sub_822C0890(ctx, base);
	// 8230389C: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 823038A0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823038A4: 419A0008  beq cr6, 0x823038ac
	if ctx.cr[6].eq {
	pc = 0x823038AC; continue 'dispatch;
	}
	// 823038A8: 4BFBCFE9  bl 0x822c0890
	ctx.lr = 0x823038AC;
	sub_822C0890(ctx, base);
	// 823038AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 823038B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823038B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823038B8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823038BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823038C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823038C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x823038C8 size=400
    let mut pc: u32 = 0x823038C8;
    'dispatch: loop {
        match pc {
            0x823038C8 => {
    //   block [0x823038C8..0x82303A58)
	// 823038C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823038CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823038D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823038D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823038D8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823038DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823038E0: 548B063F  clrlwi. r11, r4, 0x18
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823038E4: 418200B0  beq 0x82303994
	if ctx.cr[0].eq {
	pc = 0x82303994; continue 'dispatch;
	}
	// 823038E8: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 823038EC: 4BFFF27D  bl 0x82302b68
	ctx.lr = 0x823038F0;
	sub_82302B68(ctx, base);
	// 823038F0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 823038F4: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 823038F8: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 823038FC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82303900: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82303904: 419A0024  beq cr6, 0x82303928
	if ctx.cr[6].eq {
	pc = 0x82303928; continue 'dispatch;
	}
	// 82303908: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8230390C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82303910: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82303914: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82303918: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8230391C: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82303920: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82303924: 4082FFE8  bne 0x8230390c
	if !ctx.cr[0].eq {
	pc = 0x8230390C; continue 'dispatch;
	}
	// 82303928: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8230392C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82303930: 4820BBE9  bl 0x8250f518
	ctx.lr = 0x82303934;
	sub_8250F518(ctx, base);
	// 82303934: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82303938: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8230393C: 386BFF6C  addi r3, r11, -0x94
	ctx.r[3].s64 = ctx.r[11].s64 + -148;
	// 82303940: 409A0008  bne cr6, 0x82303948
	if !ctx.cr[6].eq {
	pc = 0x82303948; continue 'dispatch;
	}
	// 82303944: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82303948: 3BC10050  addi r30, r1, 0x50
	ctx.r[30].s64 = ctx.r[1].s64 + 80;
	// 8230394C: 48224705  bl 0x82528050
	ctx.lr = 0x82303950;
	sub_82528050(ctx, base);
	// 82303950: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82303954: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82303958: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 8230395C: 388ACB60  addi r4, r10, -0x34a0
	ctx.r[4].s64 = ctx.r[10].s64 + -13472;
	// 82303960: 38A006AF  li r5, 0x6af
	ctx.r[5].s64 = 1711;
	// 82303964: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 82303968: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8230396C: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 82303970: 48B550D1  bl 0x82e58a40
	ctx.lr = 0x82303974;
	sub_82E58A40(ctx, base);
	// 82303974: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82303978: 48AEE319  bl 0x82df1c90
	ctx.lr = 0x8230397C;
	sub_82DF1C90(ctx, base);
	// 8230397C: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82303980: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82303984: 419A0008  beq cr6, 0x8230398c
	if ctx.cr[6].eq {
	pc = 0x8230398C; continue 'dispatch;
	}
	// 82303988: 4BFBCF09  bl 0x822c0890
	ctx.lr = 0x8230398C;
	sub_822C0890(ctx, base);
	// 8230398C: 8061006C  lwz r3, 0x6c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 82303990: 480000A4  b 0x82303a34
	pc = 0x82303A34; continue 'dispatch;
	// 82303994: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 82303998: 4BFFF1D1  bl 0x82302b68
	ctx.lr = 0x8230399C;
	sub_82302B68(ctx, base);
	// 8230399C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 823039A0: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 823039A4: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 823039A8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823039AC: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 823039B0: 419A0024  beq cr6, 0x823039d4
	if ctx.cr[6].eq {
	pc = 0x823039D4; continue 'dispatch;
	}
	// 823039B4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 823039B8: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 823039BC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 823039C0: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 823039C4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 823039C8: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 823039CC: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 823039D0: 4082FFE8  bne 0x823039b8
	if !ctx.cr[0].eq {
	pc = 0x823039B8; continue 'dispatch;
	}
	// 823039D4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823039D8: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 823039DC: 4820BB3D  bl 0x8250f518
	ctx.lr = 0x823039E0;
	sub_8250F518(ctx, base);
	// 823039E0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 823039E4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823039E8: 386BFF6C  addi r3, r11, -0x94
	ctx.r[3].s64 = ctx.r[11].s64 + -148;
	// 823039EC: 409A0008  bne cr6, 0x823039f4
	if !ctx.cr[6].eq {
	pc = 0x823039F4; continue 'dispatch;
	}
	// 823039F0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 823039F4: 3BC10058  addi r30, r1, 0x58
	ctx.r[30].s64 = ctx.r[1].s64 + 88;
	// 823039F8: 48224659  bl 0x82528050
	ctx.lr = 0x823039FC;
	sub_82528050(ctx, base);
	// 823039FC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82303A00: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82303A04: 388BCB60  addi r4, r11, -0x34a0
	ctx.r[4].s64 = ctx.r[11].s64 + -13472;
	// 82303A08: 38A006B2  li r5, 0x6b2
	ctx.r[5].s64 = 1714;
	// 82303A0C: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 82303A10: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 82303A14: 48B535D5  bl 0x82e56fe8
	ctx.lr = 0x82303A18;
	sub_82E56FE8(ctx, base);
	// 82303A18: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82303A1C: 48AEE275  bl 0x82df1c90
	ctx.lr = 0x82303A20;
	sub_82DF1C90(ctx, base);
	// 82303A20: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82303A24: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82303A28: 419A0008  beq cr6, 0x82303a30
	if ctx.cr[6].eq {
	pc = 0x82303A30; continue 'dispatch;
	}
	// 82303A2C: 4BFBCE65  bl 0x822c0890
	ctx.lr = 0x82303A30;
	sub_822C0890(ctx, base);
	// 82303A30: 8061007C  lwz r3, 0x7c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82303A34: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82303A38: 419A0008  beq cr6, 0x82303a40
	if ctx.cr[6].eq {
	pc = 0x82303A40; continue 'dispatch;
	}
	// 82303A3C: 4BFBCE55  bl 0x822c0890
	ctx.lr = 0x82303A40;
	sub_822C0890(ctx, base);
	// 82303A40: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82303A44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82303A48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82303A4C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82303A50: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82303A54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82303A58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82303A58 size=244
    let mut pc: u32 = 0x82303A58;
    'dispatch: loop {
        match pc {
            0x82303A58 => {
    //   block [0x82303A58..0x82303B4C)
	// 82303A58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82303A5C: 48EA470D  bl 0x831a8168
	ctx.lr = 0x82303A60;
	sub_831A8130(ctx, base);
	// 82303A60: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82303A64: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82303A68: 81450004  lwz r10, 4(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 82303A6C: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 82303A70: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82303A74: 93E10060  stw r31, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[31].u32 ) };
	// 82303A78: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82303A7C: 811F000C  lwz r8, 0xc(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82303A80: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82303A84: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82303A88: 7F085040  cmplw cr6, r8, r10
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82303A8C: 7D294214  add r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 82303A90: 9141005C  stw r10, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 82303A94: 7FA85050  subf r29, r8, r10
	ctx.r[29].s64 = ctx.r[10].s64 - ctx.r[8].s64;
	// 82303A98: 7CEB4850  subf r7, r11, r9
	ctx.r[7].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 82303A9C: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82303AA0: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82303AA4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82303AA8: 7FCA5850  subf r30, r10, r11
	ctx.r[30].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82303AAC: 7F1D3840  cmplw cr6, r29, r7
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82303AB0: 40980044  bge cr6, 0x82303af4
	if !ctx.cr[6].lt {
	pc = 0x82303AF4; continue 'dispatch;
	}
	// 82303AB4: 81240000  lwz r9, 0(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82303AB8: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82303ABC: 80E50000  lwz r7, 0(r5)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82303AC0: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 82303AC4: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 82303AC8: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82303ACC: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82303AD0: 90E10058  stw r7, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[7].u32 ) };
	// 82303AD4: 4BFFF115  bl 0x82302be8
	ctx.lr = 0x82303AD8;
	sub_82302BE8(ctx, base);
	// 82303AD8: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82303ADC: 419A0054  beq cr6, 0x82303b30
	if ctx.cr[6].eq {
	pc = 0x82303B30; continue 'dispatch;
	}
	// 82303AE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82303AE4: 4BFFECE5  bl 0x823027c8
	ctx.lr = 0x82303AE8;
	sub_823027C8(ctx, base);
	// 82303AE8: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82303AEC: 4082FFF4  bne 0x82303ae0
	if !ctx.cr[0].eq {
	pc = 0x82303AE0; continue 'dispatch;
	}
	// 82303AF0: 48000040  b 0x82303b30
	pc = 0x82303B30; continue 'dispatch;
	// 82303AF4: 81050000  lwz r8, 0(r5)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82303AF8: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 82303AFC: 80E40000  lwz r7, 0(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82303B00: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82303B04: 91210064  stw r9, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[9].u32 ) };
	// 82303B08: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82303B0C: 91010058  stw r8, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[8].u32 ) };
	// 82303B10: 90E10050  stw r7, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[7].u32 ) };
	// 82303B14: 4BFFE25D  bl 0x82301d70
	ctx.lr = 0x82303B18;
	sub_82301D70(ctx, base);
	// 82303B18: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82303B1C: 419A0014  beq cr6, 0x82303b30
	if ctx.cr[6].eq {
	pc = 0x82303B30; continue 'dispatch;
	}
	// 82303B20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82303B24: 4BFFE14D  bl 0x82301c70
	ctx.lr = 0x82303B28;
	sub_82301C70(ctx, base);
	// 82303B28: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82303B2C: 4082FFF4  bne 0x82303b20
	if !ctx.cr[0].eq {
	pc = 0x82303B20; continue 'dispatch;
	}
	// 82303B30: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82303B34: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82303B38: 93FC0000  stw r31, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82303B3C: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82303B40: 917C0004  stw r11, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82303B44: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82303B48: 48EA4670  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82303B50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82303B50 size=268
    let mut pc: u32 = 0x82303B50;
    'dispatch: loop {
        match pc {
            0x82303B50 => {
    //   block [0x82303B50..0x82303C5C)
	// 82303B50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82303B54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82303B58: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82303B5C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82303B60: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82303B64: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82303B68: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82303B6C: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82303B70: 396BCBDC  addi r11, r11, -0x3424
	ctx.r[11].s64 = ctx.r[11].s64 + -13348;
	// 82303B74: 394ACBC8  addi r10, r10, -0x3438
	ctx.r[10].s64 = ctx.r[10].s64 + -13368;
	// 82303B78: 3929CBB8  addi r9, r9, -0x3448
	ctx.r[9].s64 = ctx.r[9].s64 + -13384;
	// 82303B7C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82303B80: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 82303B84: 913F00C0  stw r9, 0xc0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(192 as u32), ctx.r[9].u32 ) };
	// 82303B88: 807F026C  lwz r3, 0x26c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(620 as u32) ) } as u64;
	// 82303B8C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82303B90: 419A0008  beq cr6, 0x82303b98
	if ctx.cr[6].eq {
	pc = 0x82303B98; continue 'dispatch;
	}
	// 82303B94: 4BFBCCFD  bl 0x822c0890
	ctx.lr = 0x82303B98;
	sub_822C0890(ctx, base);
	// 82303B98: 807F0260  lwz r3, 0x260(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(608 as u32) ) } as u64;
	// 82303B9C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82303BA0: 419A0008  beq cr6, 0x82303ba8
	if ctx.cr[6].eq {
	pc = 0x82303BA8; continue 'dispatch;
	}
	// 82303BA4: 4BFBCCED  bl 0x822c0890
	ctx.lr = 0x82303BA8;
	sub_822C0890(ctx, base);
	// 82303BA8: 807F0250  lwz r3, 0x250(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(592 as u32) ) } as u64;
	// 82303BAC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82303BB0: 419A0008  beq cr6, 0x82303bb8
	if ctx.cr[6].eq {
	pc = 0x82303BB8; continue 'dispatch;
	}
	// 82303BB4: 4BFBCCDD  bl 0x822c0890
	ctx.lr = 0x82303BB8;
	sub_822C0890(ctx, base);
	// 82303BB8: 807F0248  lwz r3, 0x248(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(584 as u32) ) } as u64;
	// 82303BBC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82303BC0: 419A0008  beq cr6, 0x82303bc8
	if ctx.cr[6].eq {
	pc = 0x82303BC8; continue 'dispatch;
	}
	// 82303BC4: 4BFBCCCD  bl 0x822c0890
	ctx.lr = 0x82303BC8;
	sub_822C0890(ctx, base);
	// 82303BC8: 807F0240  lwz r3, 0x240(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(576 as u32) ) } as u64;
	// 82303BCC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82303BD0: 419A0008  beq cr6, 0x82303bd8
	if ctx.cr[6].eq {
	pc = 0x82303BD8; continue 'dispatch;
	}
	// 82303BD4: 4BFBCCBD  bl 0x822c0890
	ctx.lr = 0x82303BD8;
	sub_822C0890(ctx, base);
	// 82303BD8: 807F0238  lwz r3, 0x238(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(568 as u32) ) } as u64;
	// 82303BDC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82303BE0: 419A0008  beq cr6, 0x82303be8
	if ctx.cr[6].eq {
	pc = 0x82303BE8; continue 'dispatch;
	}
	// 82303BE4: 4BFBCCAD  bl 0x822c0890
	ctx.lr = 0x82303BE8;
	sub_822C0890(ctx, base);
	// 82303BE8: 387F022C  addi r3, r31, 0x22c
	ctx.r[3].s64 = ctx.r[31].s64 + 556;
	// 82303BEC: 48B5CD85  bl 0x82e60970
	ctx.lr = 0x82303BF0;
	sub_82E60970(ctx, base);
	// 82303BF0: 387F0224  addi r3, r31, 0x224
	ctx.r[3].s64 = ctx.r[31].s64 + 548;
	// 82303BF4: 48B5CD7D  bl 0x82e60970
	ctx.lr = 0x82303BF8;
	sub_82E60970(ctx, base);
	// 82303BF8: 387F0210  addi r3, r31, 0x210
	ctx.r[3].s64 = ctx.r[31].s64 + 528;
	// 82303BFC: 48517425  bl 0x8281b020
	ctx.lr = 0x82303C00;
	sub_8281B020(ctx, base);
	// 82303C00: 387F01B0  addi r3, r31, 0x1b0
	ctx.r[3].s64 = ctx.r[31].s64 + 432;
	// 82303C04: 48B59C45  bl 0x82e5d848
	ctx.lr = 0x82303C08;
	sub_82E5D848(ctx, base);
	// 82303C08: 387F019C  addi r3, r31, 0x19c
	ctx.r[3].s64 = ctx.r[31].s64 + 412;
	// 82303C0C: 48517415  bl 0x8281b020
	ctx.lr = 0x82303C10;
	sub_8281B020(ctx, base);
	// 82303C10: 387F013C  addi r3, r31, 0x13c
	ctx.r[3].s64 = ctx.r[31].s64 + 316;
	// 82303C14: 48B59C35  bl 0x82e5d848
	ctx.lr = 0x82303C18;
	sub_82E5D848(ctx, base);
	// 82303C18: 387F0134  addi r3, r31, 0x134
	ctx.r[3].s64 = ctx.r[31].s64 + 308;
	// 82303C1C: 48B5CD55  bl 0x82e60970
	ctx.lr = 0x82303C20;
	sub_82E60970(ctx, base);
	// 82303C20: 387F012C  addi r3, r31, 0x12c
	ctx.r[3].s64 = ctx.r[31].s64 + 300;
	// 82303C24: 48B5CD4D  bl 0x82e60970
	ctx.lr = 0x82303C28;
	sub_82E60970(ctx, base);
	// 82303C28: 387F00CC  addi r3, r31, 0xcc
	ctx.r[3].s64 = ctx.r[31].s64 + 204;
	// 82303C2C: 48B59C1D  bl 0x82e5d848
	ctx.lr = 0x82303C30;
	sub_82E5D848(ctx, base);
	// 82303C30: 807F00C8  lwz r3, 0xc8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(200 as u32) ) } as u64;
	// 82303C34: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82303C38: 419A0008  beq cr6, 0x82303c40
	if ctx.cr[6].eq {
	pc = 0x82303C40; continue 'dispatch;
	}
	// 82303C3C: 4BFBCC55  bl 0x822c0890
	ctx.lr = 0x82303C40;
	sub_822C0890(ctx, base);
	// 82303C40: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82303C44: 4820D555  bl 0x82511198
	ctx.lr = 0x82303C48;
	sub_82511198(ctx, base);
	// 82303C48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82303C4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82303C50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82303C54: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82303C58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82303C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82303C60 size=8
    let mut pc: u32 = 0x82303C60;
    'dispatch: loop {
        match pc {
            0x82303C60 => {
    //   block [0x82303C60..0x82303C68)
	// 82303C60: 3863FFD8  addi r3, r3, -0x28
	ctx.r[3].s64 = ctx.r[3].s64 + -40;
	// 82303C64: 480001BC  b 0x82303e20
	sub_82303E20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82303C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82303C68 size=56
    let mut pc: u32 = 0x82303C68;
    'dispatch: loop {
        match pc {
            0x82303C68 => {
    //   block [0x82303C68..0x82303CA0)
	// 82303C68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82303C6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82303C70: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82303C74: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82303C78: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82303C7C: 387F013C  addi r3, r31, 0x13c
	ctx.r[3].s64 = ctx.r[31].s64 + 316;
	// 82303C80: 48B58A19  bl 0x82e5c698
	ctx.lr = 0x82303C84;
	sub_82E5C698(ctx, base);
	// 82303C84: 387F019C  addi r3, r31, 0x19c
	ctx.r[3].s64 = ctx.r[31].s64 + 412;
	// 82303C88: 48517399  bl 0x8281b020
	ctx.lr = 0x82303C8C;
	sub_8281B020(ctx, base);
	// 82303C8C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82303C90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82303C94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82303C98: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82303C9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82303CA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82303CA0 size=84
    let mut pc: u32 = 0x82303CA0;
    'dispatch: loop {
        match pc {
            0x82303CA0 => {
    //   block [0x82303CA0..0x82303CF4)
	// 82303CA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82303CA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82303CA8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82303CAC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82303CB0: 81650004  lwz r11, 4(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 82303CB4: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 82303CB8: 81250000  lwz r9, 0(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82303CBC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82303CC0: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 82303CC4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82303CC8: 9141005C  stw r10, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 82303CCC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82303CD0: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82303CD4: 91210058  stw r9, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[9].u32 ) };
	// 82303CD8: 4BFFFD81  bl 0x82303a58
	ctx.lr = 0x82303CDC;
	sub_82303A58(ctx, base);
	// 82303CDC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82303CE0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82303CE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82303CE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82303CEC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82303CF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82303CF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82303CF8 size=296
    let mut pc: u32 = 0x82303CF8;
    'dispatch: loop {
        match pc {
            0x82303CF8 => {
    //   block [0x82303CF8..0x82303E20)
	// 82303CF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82303CFC: 48EA446D  bl 0x831a8168
	ctx.lr = 0x82303D00;
	sub_831A8130(ctx, base);
	// 82303D00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82303D04: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82303D08: 4820D3E9  bl 0x825110f0
	ctx.lr = 0x82303D0C;
	sub_825110F0(ctx, base);
	// 82303D0C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82303D10: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 82303D14: 396BCA10  addi r11, r11, -0x35f0
	ctx.r[11].s64 = ctx.r[11].s64 + -13808;
	// 82303D18: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82303D1C: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82303D20: 917F00C0  stw r11, 0xc0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(192 as u32), ctx.r[11].u32 ) };
	// 82303D24: 394ACBDC  addi r10, r10, -0x3424
	ctx.r[10].s64 = ctx.r[10].s64 + -13348;
	// 82303D28: 3929CBC8  addi r9, r9, -0x3438
	ctx.r[9].s64 = ctx.r[9].s64 + -13368;
	// 82303D2C: 3968CBB8  addi r11, r8, -0x3448
	ctx.r[11].s64 = ctx.r[8].s64 + -13384;
	// 82303D30: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82303D34: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82303D38: 913F0028  stw r9, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[9].u32 ) };
	// 82303D3C: 917F00C0  stw r11, 0xc0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(192 as u32), ctx.r[11].u32 ) };
	// 82303D40: 387F00CC  addi r3, r31, 0xcc
	ctx.r[3].s64 = ctx.r[31].s64 + 204;
	// 82303D44: 93DF00C4  stw r30, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[30].u32 ) };
	// 82303D48: 93DF00C8  stw r30, 0xc8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(200 as u32), ctx.r[30].u32 ) };
	// 82303D4C: 48B59B85  bl 0x82e5d8d0
	ctx.lr = 0x82303D50;
	sub_82E5D8D0(ctx, base);
	// 82303D50: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82303D54: 387F012C  addi r3, r31, 0x12c
	ctx.r[3].s64 = ctx.r[31].s64 + 300;
	// 82303D58: 3BABCA20  addi r29, r11, -0x35e0
	ctx.r[29].s64 = ctx.r[11].s64 + -13792;
	// 82303D5C: 93BF00CC  stw r29, 0xcc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(204 as u32), ctx.r[29].u32 ) };
	// 82303D60: 48B5CBA1  bl 0x82e60900
	ctx.lr = 0x82303D64;
	sub_82E60900(ctx, base);
	// 82303D64: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82303D68: 387F0134  addi r3, r31, 0x134
	ctx.r[3].s64 = ctx.r[31].s64 + 308;
	// 82303D6C: 3B8BCA30  addi r28, r11, -0x35d0
	ctx.r[28].s64 = ctx.r[11].s64 + -13776;
	// 82303D70: 939F012C  stw r28, 0x12c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(300 as u32), ctx.r[28].u32 ) };
	// 82303D74: 48B5CB8D  bl 0x82e60900
	ctx.lr = 0x82303D78;
	sub_82E60900(ctx, base);
	// 82303D78: 939F0134  stw r28, 0x134(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(308 as u32), ctx.r[28].u32 ) };
	// 82303D7C: 387F013C  addi r3, r31, 0x13c
	ctx.r[3].s64 = ctx.r[31].s64 + 316;
	// 82303D80: 48B59B51  bl 0x82e5d8d0
	ctx.lr = 0x82303D84;
	sub_82E5D8D0(ctx, base);
	// 82303D84: 93BF013C  stw r29, 0x13c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(316 as u32), ctx.r[29].u32 ) };
	// 82303D88: 93DF01A0  stw r30, 0x1a0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(416 as u32), ctx.r[30].u32 ) };
	// 82303D8C: 387F01B0  addi r3, r31, 0x1b0
	ctx.r[3].s64 = ctx.r[31].s64 + 432;
	// 82303D90: 93DF01A4  stw r30, 0x1a4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(420 as u32), ctx.r[30].u32 ) };
	// 82303D94: 93DF01A8  stw r30, 0x1a8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(424 as u32), ctx.r[30].u32 ) };
	// 82303D98: 93DF01AC  stw r30, 0x1ac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(428 as u32), ctx.r[30].u32 ) };
	// 82303D9C: 48B59B35  bl 0x82e5d8d0
	ctx.lr = 0x82303DA0;
	sub_82E5D8D0(ctx, base);
	// 82303DA0: 93BF01B0  stw r29, 0x1b0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(432 as u32), ctx.r[29].u32 ) };
	// 82303DA4: 93DF0214  stw r30, 0x214(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(532 as u32), ctx.r[30].u32 ) };
	// 82303DA8: 387F0224  addi r3, r31, 0x224
	ctx.r[3].s64 = ctx.r[31].s64 + 548;
	// 82303DAC: 93DF0218  stw r30, 0x218(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(536 as u32), ctx.r[30].u32 ) };
	// 82303DB0: 93DF021C  stw r30, 0x21c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(540 as u32), ctx.r[30].u32 ) };
	// 82303DB4: 93DF0220  stw r30, 0x220(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(544 as u32), ctx.r[30].u32 ) };
	// 82303DB8: 48B5CB49  bl 0x82e60900
	ctx.lr = 0x82303DBC;
	sub_82E60900(ctx, base);
	// 82303DBC: 939F0224  stw r28, 0x224(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(548 as u32), ctx.r[28].u32 ) };
	// 82303DC0: 387F022C  addi r3, r31, 0x22c
	ctx.r[3].s64 = ctx.r[31].s64 + 556;
	// 82303DC4: 48B5CB3D  bl 0x82e60900
	ctx.lr = 0x82303DC8;
	sub_82E60900(ctx, base);
	// 82303DC8: 939F022C  stw r28, 0x22c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(556 as u32), ctx.r[28].u32 ) };
	// 82303DCC: 93DF0234  stw r30, 0x234(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(564 as u32), ctx.r[30].u32 ) };
	// 82303DD0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82303DD4: 93DF0238  stw r30, 0x238(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(568 as u32), ctx.r[30].u32 ) };
	// 82303DD8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82303DDC: 93DF023C  stw r30, 0x23c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(572 as u32), ctx.r[30].u32 ) };
	// 82303DE0: 93DF0240  stw r30, 0x240(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(576 as u32), ctx.r[30].u32 ) };
	// 82303DE4: 93DF0244  stw r30, 0x244(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(580 as u32), ctx.r[30].u32 ) };
	// 82303DE8: 93DF0248  stw r30, 0x248(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(584 as u32), ctx.r[30].u32 ) };
	// 82303DEC: 93DF024C  stw r30, 0x24c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(588 as u32), ctx.r[30].u32 ) };
	// 82303DF0: 93DF0250  stw r30, 0x250(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(592 as u32), ctx.r[30].u32 ) };
	// 82303DF4: 997F0254  stb r11, 0x254(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(596 as u32), ctx.r[11].u8 ) };
	// 82303DF8: 997F0255  stb r11, 0x255(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(597 as u32), ctx.r[11].u8 ) };
	// 82303DFC: 9BDF0256  stb r30, 0x256(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(598 as u32), ctx.r[30].u8 ) };
	// 82303E00: 93DF0258  stw r30, 0x258(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(600 as u32), ctx.r[30].u32 ) };
	// 82303E04: 93DF025C  stw r30, 0x25c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(604 as u32), ctx.r[30].u32 ) };
	// 82303E08: 93DF0260  stw r30, 0x260(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(608 as u32), ctx.r[30].u32 ) };
	// 82303E0C: 997F0264  stb r11, 0x264(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(612 as u32), ctx.r[11].u8 ) };
	// 82303E10: 93DF0268  stw r30, 0x268(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(616 as u32), ctx.r[30].u32 ) };
	// 82303E14: 93DF026C  stw r30, 0x26c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(620 as u32), ctx.r[30].u32 ) };
	// 82303E18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82303E1C: 48EA439C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82303E20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82303E20 size=76
    let mut pc: u32 = 0x82303E20;
    'dispatch: loop {
        match pc {
            0x82303E20 => {
    //   block [0x82303E20..0x82303E6C)
	// 82303E20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82303E24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82303E28: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82303E2C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82303E30: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82303E34: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82303E38: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82303E3C: 4BFFFD15  bl 0x82303b50
	ctx.lr = 0x82303E40;
	sub_82303B50(ctx, base);
	// 82303E40: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82303E44: 4182000C  beq 0x82303e50
	if ctx.cr[0].eq {
	pc = 0x82303E50; continue 'dispatch;
	}
	// 82303E48: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82303E4C: 48AEE58D  bl 0x82df23d8
	ctx.lr = 0x82303E50;
	sub_82DF23D8(ctx, base);
	// 82303E50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82303E54: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82303E58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82303E5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82303E60: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82303E64: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82303E68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82303E70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82303E70 size=600
    let mut pc: u32 = 0x82303E70;
    'dispatch: loop {
        match pc {
            0x82303E70 => {
    //   block [0x82303E70..0x823040C8)
	// 82303E70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82303E74: 48EA42F1  bl 0x831a8164
	ctx.lr = 0x82303E78;
	sub_831A8130(ctx, base);
	// 82303E78: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82303E7C: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82303E80: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82303E84: 817B01AC  lwz r11, 0x1ac(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(428 as u32) ) } as u64;
	// 82303E88: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82303E8C: 40990234  ble cr6, 0x823040c0
	if !ctx.cr[6].gt {
	pc = 0x823040C0; continue 'dispatch;
	}
	// 82303E90: 813B01A8  lwz r9, 0x1a8(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(424 as u32) ) } as u64;
	// 82303E94: 3BFB019C  addi r31, r27, 0x19c
	ctx.r[31].s64 = ctx.r[27].s64 + 412;
	// 82303E98: 815B01A4  lwz r10, 0x1a4(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(420 as u32) ) } as u64;
	// 82303E9C: 552BF87E  srwi r11, r9, 1
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shr(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82303EA0: 552907FE  clrlwi r9, r9, 0x1f
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x00000001u64;
	// 82303EA4: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82303EA8: 41990008  bgt cr6, 0x82303eb0
	if ctx.cr[6].gt {
	pc = 0x82303EB0; continue 'dispatch;
	}
	// 82303EAC: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82303EB0: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82303EB4: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82303EB8: 552B1838  slwi r11, r9, 3
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82303EBC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82303EC0: 7D48502E  lwzx r10, r8, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82303EC4: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82303EC8: 48AEF441  bl 0x82df3308
	ctx.lr = 0x82303ECC;
	sub_82DF3308(ctx, base);
	// 82303ECC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82303ED0: 418200E8  beq 0x82303fb8
	if ctx.cr[0].eq {
	pc = 0x82303FB8; continue 'dispatch;
	}
	// 82303ED4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82303ED8: 4BFFE8F1  bl 0x823027c8
	ctx.lr = 0x82303EDC;
	sub_823027C8(ctx, base);
	// 82303EDC: 817B01AC  lwz r11, 0x1ac(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(428 as u32) ) } as u64;
	// 82303EE0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82303EE4: 419A00AC  beq cr6, 0x82303f90
	if ctx.cr[6].eq {
	pc = 0x82303F90; continue 'dispatch;
	}
	// 82303EE8: 813F000C  lwz r9, 0xc(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82303EEC: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82303EF0: 552BF87E  srwi r11, r9, 1
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shr(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82303EF4: 552907FE  clrlwi r9, r9, 0x1f
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x00000001u64;
	// 82303EF8: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82303EFC: 41990008  bgt cr6, 0x82303f04
	if ctx.cr[6].gt {
	pc = 0x82303F04; continue 'dispatch;
	}
	// 82303F00: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82303F04: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82303F08: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82303F0C: 552B1838  slwi r11, r9, 3
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82303F10: 7D48502E  lwzx r10, r8, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82303F14: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82303F18: 896B0004  lbz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82303F1C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82303F20: 41820070  beq 0x82303f90
	if ctx.cr[0].eq {
	pc = 0x82303F90; continue 'dispatch;
	}
	// 82303F24: 813F000C  lwz r9, 0xc(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82303F28: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82303F2C: 552BF87E  srwi r11, r9, 1
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shr(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82303F30: 552907FE  clrlwi r9, r9, 0x1f
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x00000001u64;
	// 82303F34: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82303F38: 41990008  bgt cr6, 0x82303f40
	if ctx.cr[6].gt {
	pc = 0x82303F40; continue 'dispatch;
	}
	// 82303F3C: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82303F40: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82303F44: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82303F48: 552B1838  slwi r11, r9, 3
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82303F4C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82303F50: 7D48502E  lwzx r10, r8, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82303F54: 7C8A5A14  add r4, r10, r11
	ctx.r[4].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82303F58: 48AEFCA9  bl 0x82df3c00
	ctx.lr = 0x82303F5C;
	sub_82DF3C00(ctx, base);
	// 82303F5C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82303F60: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82303F64: 389B013C  addi r4, r27, 0x13c
	ctx.r[4].s64 = ctx.r[27].s64 + 316;
	// 82303F68: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82303F6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82303F70: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82303F74: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82303F78: 48B5A9E9  bl 0x82e5e960
	ctx.lr = 0x82303F7C;
	sub_82E5E960(ctx, base);
	// 82303F7C: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82303F80: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82303F84: 419A013C  beq cr6, 0x823040c0
	if ctx.cr[6].eq {
	pc = 0x823040C0; continue 'dispatch;
	}
	// 82303F88: 4BFBC909  bl 0x822c0890
	ctx.lr = 0x82303F8C;
	sub_822C0890(ctx, base);
	// 82303F8C: 48000134  b 0x823040c0
	pc = 0x823040C0; continue 'dispatch;
	// 82303F90: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82303F94: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82303F98: 48AEFC69  bl 0x82df3c00
	ctx.lr = 0x82303F9C;
	sub_82DF3C00(ctx, base);
	// 82303F9C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82303FA0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82303FA4: 387B013C  addi r3, r27, 0x13c
	ctx.r[3].s64 = ctx.r[27].s64 + 316;
	// 82303FA8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82303FAC: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82303FB0: 48B58681  bl 0x82e5c630
	ctx.lr = 0x82303FB4;
	sub_82E5C630(ctx, base);
	// 82303FB4: 4800010C  b 0x823040c0
	pc = 0x823040C0; continue 'dispatch;
	// 82303FB8: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82303FBC: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82303FC0: 7D5E5378  mr r30, r10
	ctx.r[30].u64 = ctx.r[10].u64;
	// 82303FC4: 7FAB5214  add r29, r11, r10
	ctx.r[29].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82303FC8: 7F1FF840  cmplw cr6, r31, r31
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82303FCC: 409A0010  bne cr6, 0x82303fdc
	if !ctx.cr[6].eq {
	pc = 0x82303FDC; continue 'dispatch;
	}
	// 82303FD0: 7F1EE840  cmplw cr6, r30, r29
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82303FD4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82303FD8: 419A0008  beq cr6, 0x82303fe0
	if ctx.cr[6].eq {
	pc = 0x82303FE0; continue 'dispatch;
	}
	// 82303FDC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82303FE0: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82303FE4: 408200DC  bne 0x823040c0
	if !ctx.cr[0].eq {
	pc = 0x823040C0; continue 'dispatch;
	}
	// 82303FE8: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82303FEC: 57CBF87E  srwi r11, r30, 1
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shr(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82303FF0: 57C907FE  clrlwi r9, r30, 0x1f
	ctx.r[9].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82303FF4: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82303FF8: 41990008  bgt cr6, 0x82304000
	if ctx.cr[6].gt {
	pc = 0x82304000; continue 'dispatch;
	}
	// 82303FFC: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82304000: 811F0004  lwz r8, 4(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82304004: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82304008: 552A1838  slwi r10, r9, 3
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8230400C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82304010: 7D68582E  lwzx r11, r8, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82304014: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82304018: 48AEF2F1  bl 0x82df3308
	ctx.lr = 0x8230401C;
	sub_82DF3308(ctx, base);
	// 8230401C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82304020: 4082000C  bne 0x8230402c
	if !ctx.cr[0].eq {
	pc = 0x8230402C; continue 'dispatch;
	}
	// 82304024: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82304028: 4BFFFFA0  b 0x82303fc8
	pc = 0x82303FC8; continue 'dispatch;
	// 8230402C: 811F0008  lwz r8, 8(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82304030: 57CAF87E  srwi r10, r30, 1
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shr(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82304034: 57C707FE  clrlwi r7, r30, 0x1f
	ctx.r[7].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82304038: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 8230403C: 7F085040  cmplw cr6, r8, r10
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82304040: 41990008  bgt cr6, 0x82304048
	if ctx.cr[6].gt {
	pc = 0x82304048; continue 'dispatch;
	}
	// 82304044: 7D685850  subf r11, r8, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[8].s64;
	// 82304048: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8230404C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82304050: 54E61838  slwi r6, r7, 3
	ctx.r[6].u32 = ctx.r[7].u32.wrapping_shl(3);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82304054: 7D6B482E  lwzx r11, r11, r9
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82304058: 7D6B3214  add r11, r11, r6
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[6].u64;
	// 8230405C: 896B0004  lbz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82304060: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82304064: 40820044  bne 0x823040a8
	if !ctx.cr[0].eq {
	pc = 0x823040A8; continue 'dispatch;
	}
	// 82304068: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 8230406C: 7F085040  cmplw cr6, r8, r10
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82304070: 41990008  bgt cr6, 0x82304078
	if ctx.cr[6].gt {
	pc = 0x82304078; continue 'dispatch;
	}
	// 82304074: 7D685850  subf r11, r8, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[8].s64;
	// 82304078: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8230407C: 54EA1838  slwi r10, r7, 3
	ctx.r[10].u32 = ctx.r[7].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82304080: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82304084: 7D6B482E  lwzx r11, r11, r9
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82304088: 7C8B5214  add r4, r11, r10
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8230408C: 48AEFB75  bl 0x82df3c00
	ctx.lr = 0x82304090;
	sub_82DF3C00(ctx, base);
	// 82304090: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82304094: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82304098: 387B013C  addi r3, r27, 0x13c
	ctx.r[3].s64 = ctx.r[27].s64 + 316;
	// 8230409C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823040A0: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 823040A4: 48B5858D  bl 0x82e5c630
	ctx.lr = 0x823040A8;
	sub_82E5C630(ctx, base);
	// 823040A8: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 823040AC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 823040B0: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 823040B4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823040B8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 823040BC: 4BFFFBE5  bl 0x82303ca0
	ctx.lr = 0x823040C0;
	sub_82303CA0(ctx, base);
	// 823040C0: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 823040C4: 48EA40F0  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823040C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x823040C8 size=600
    let mut pc: u32 = 0x823040C8;
    'dispatch: loop {
        match pc {
            0x823040C8 => {
    //   block [0x823040C8..0x82304320)
	// 823040C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823040CC: 48EA4099  bl 0x831a8164
	ctx.lr = 0x823040D0;
	sub_831A8130(ctx, base);
	// 823040D0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823040D4: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 823040D8: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 823040DC: 817B0220  lwz r11, 0x220(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(544 as u32) ) } as u64;
	// 823040E0: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 823040E4: 40990234  ble cr6, 0x82304318
	if !ctx.cr[6].gt {
	pc = 0x82304318; continue 'dispatch;
	}
	// 823040E8: 813B021C  lwz r9, 0x21c(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(540 as u32) ) } as u64;
	// 823040EC: 3BFB0210  addi r31, r27, 0x210
	ctx.r[31].s64 = ctx.r[27].s64 + 528;
	// 823040F0: 815B0218  lwz r10, 0x218(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(536 as u32) ) } as u64;
	// 823040F4: 552BF87E  srwi r11, r9, 1
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shr(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823040F8: 552907FE  clrlwi r9, r9, 0x1f
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x00000001u64;
	// 823040FC: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82304100: 41990008  bgt cr6, 0x82304108
	if ctx.cr[6].gt {
	pc = 0x82304108; continue 'dispatch;
	}
	// 82304104: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82304108: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8230410C: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82304110: 552B1838  slwi r11, r9, 3
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82304114: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82304118: 7D48502E  lwzx r10, r8, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8230411C: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82304120: 48AEF1E9  bl 0x82df3308
	ctx.lr = 0x82304124;
	sub_82DF3308(ctx, base);
	// 82304124: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82304128: 418200E8  beq 0x82304210
	if ctx.cr[0].eq {
	pc = 0x82304210; continue 'dispatch;
	}
	// 8230412C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82304130: 4BFFE699  bl 0x823027c8
	ctx.lr = 0x82304134;
	sub_823027C8(ctx, base);
	// 82304134: 817B0220  lwz r11, 0x220(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(544 as u32) ) } as u64;
	// 82304138: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8230413C: 419A00AC  beq cr6, 0x823041e8
	if ctx.cr[6].eq {
	pc = 0x823041E8; continue 'dispatch;
	}
	// 82304140: 813F000C  lwz r9, 0xc(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82304144: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82304148: 552BF87E  srwi r11, r9, 1
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shr(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8230414C: 552907FE  clrlwi r9, r9, 0x1f
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x00000001u64;
	// 82304150: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82304154: 41990008  bgt cr6, 0x8230415c
	if ctx.cr[6].gt {
	pc = 0x8230415C; continue 'dispatch;
	}
	// 82304158: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8230415C: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82304160: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82304164: 552B1838  slwi r11, r9, 3
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82304168: 7D48502E  lwzx r10, r8, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8230416C: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82304170: 896B0004  lbz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82304174: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82304178: 41820070  beq 0x823041e8
	if ctx.cr[0].eq {
	pc = 0x823041E8; continue 'dispatch;
	}
	// 8230417C: 813F000C  lwz r9, 0xc(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82304180: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82304184: 552BF87E  srwi r11, r9, 1
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shr(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82304188: 552907FE  clrlwi r9, r9, 0x1f
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x00000001u64;
	// 8230418C: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82304190: 41990008  bgt cr6, 0x82304198
	if ctx.cr[6].gt {
	pc = 0x82304198; continue 'dispatch;
	}
	// 82304194: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82304198: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8230419C: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 823041A0: 552B1838  slwi r11, r9, 3
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823041A4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823041A8: 7D48502E  lwzx r10, r8, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 823041AC: 7C8A5A14  add r4, r10, r11
	ctx.r[4].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 823041B0: 48AEFA51  bl 0x82df3c00
	ctx.lr = 0x823041B4;
	sub_82DF3C00(ctx, base);
	// 823041B4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 823041B8: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 823041BC: 389B01B0  addi r4, r27, 0x1b0
	ctx.r[4].s64 = ctx.r[27].s64 + 432;
	// 823041C0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 823041C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823041C8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 823041CC: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 823041D0: 48B5A791  bl 0x82e5e960
	ctx.lr = 0x823041D4;
	sub_82E5E960(ctx, base);
	// 823041D4: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 823041D8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823041DC: 419A013C  beq cr6, 0x82304318
	if ctx.cr[6].eq {
	pc = 0x82304318; continue 'dispatch;
	}
	// 823041E0: 4BFBC6B1  bl 0x822c0890
	ctx.lr = 0x823041E4;
	sub_822C0890(ctx, base);
	// 823041E4: 48000134  b 0x82304318
	pc = 0x82304318; continue 'dispatch;
	// 823041E8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 823041EC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823041F0: 48AEFA11  bl 0x82df3c00
	ctx.lr = 0x823041F4;
	sub_82DF3C00(ctx, base);
	// 823041F4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 823041F8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 823041FC: 387B01B0  addi r3, r27, 0x1b0
	ctx.r[3].s64 = ctx.r[27].s64 + 432;
	// 82304200: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82304204: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82304208: 48B58429  bl 0x82e5c630
	ctx.lr = 0x8230420C;
	sub_82E5C630(ctx, base);
	// 8230420C: 4800010C  b 0x82304318
	pc = 0x82304318; continue 'dispatch;
	// 82304210: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82304214: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82304218: 7D5E5378  mr r30, r10
	ctx.r[30].u64 = ctx.r[10].u64;
	// 8230421C: 7FAB5214  add r29, r11, r10
	ctx.r[29].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82304220: 7F1FF840  cmplw cr6, r31, r31
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82304224: 409A0010  bne cr6, 0x82304234
	if !ctx.cr[6].eq {
	pc = 0x82304234; continue 'dispatch;
	}
	// 82304228: 7F1EE840  cmplw cr6, r30, r29
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[29].u32, &mut ctx.xer);
	// 8230422C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82304230: 419A0008  beq cr6, 0x82304238
	if ctx.cr[6].eq {
	pc = 0x82304238; continue 'dispatch;
	}
	// 82304234: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82304238: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8230423C: 408200DC  bne 0x82304318
	if !ctx.cr[0].eq {
	pc = 0x82304318; continue 'dispatch;
	}
	// 82304240: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82304244: 57CBF87E  srwi r11, r30, 1
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shr(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82304248: 57C907FE  clrlwi r9, r30, 0x1f
	ctx.r[9].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 8230424C: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82304250: 41990008  bgt cr6, 0x82304258
	if ctx.cr[6].gt {
	pc = 0x82304258; continue 'dispatch;
	}
	// 82304254: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82304258: 811F0004  lwz r8, 4(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8230425C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82304260: 552A1838  slwi r10, r9, 3
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82304264: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82304268: 7D68582E  lwzx r11, r8, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8230426C: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82304270: 48AEF099  bl 0x82df3308
	ctx.lr = 0x82304274;
	sub_82DF3308(ctx, base);
	// 82304274: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82304278: 4082000C  bne 0x82304284
	if !ctx.cr[0].eq {
	pc = 0x82304284; continue 'dispatch;
	}
	// 8230427C: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82304280: 4BFFFFA0  b 0x82304220
	pc = 0x82304220; continue 'dispatch;
	// 82304284: 811F0008  lwz r8, 8(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82304288: 57CAF87E  srwi r10, r30, 1
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shr(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8230428C: 57C707FE  clrlwi r7, r30, 0x1f
	ctx.r[7].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82304290: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82304294: 7F085040  cmplw cr6, r8, r10
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82304298: 41990008  bgt cr6, 0x823042a0
	if ctx.cr[6].gt {
	pc = 0x823042A0; continue 'dispatch;
	}
	// 8230429C: 7D685850  subf r11, r8, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[8].s64;
	// 823042A0: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 823042A4: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823042A8: 54E61838  slwi r6, r7, 3
	ctx.r[6].u32 = ctx.r[7].u32.wrapping_shl(3);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 823042AC: 7D6B482E  lwzx r11, r11, r9
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 823042B0: 7D6B3214  add r11, r11, r6
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[6].u64;
	// 823042B4: 896B0004  lbz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823042B8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823042BC: 40820044  bne 0x82304300
	if !ctx.cr[0].eq {
	pc = 0x82304300; continue 'dispatch;
	}
	// 823042C0: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 823042C4: 7F085040  cmplw cr6, r8, r10
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[10].u32, &mut ctx.xer);
	// 823042C8: 41990008  bgt cr6, 0x823042d0
	if ctx.cr[6].gt {
	pc = 0x823042D0; continue 'dispatch;
	}
	// 823042CC: 7D685850  subf r11, r8, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[8].s64;
	// 823042D0: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823042D4: 54EA1838  slwi r10, r7, 3
	ctx.r[10].u32 = ctx.r[7].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 823042D8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823042DC: 7D6B482E  lwzx r11, r11, r9
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 823042E0: 7C8B5214  add r4, r11, r10
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 823042E4: 48AEF91D  bl 0x82df3c00
	ctx.lr = 0x823042E8;
	sub_82DF3C00(ctx, base);
	// 823042E8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 823042EC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 823042F0: 387B01B0  addi r3, r27, 0x1b0
	ctx.r[3].s64 = ctx.r[27].s64 + 432;
	// 823042F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823042F8: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 823042FC: 48B58335  bl 0x82e5c630
	ctx.lr = 0x82304300;
	sub_82E5C630(ctx, base);
	// 82304300: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82304304: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82304308: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 8230430C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82304310: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82304314: 4BFFF98D  bl 0x82303ca0
	ctx.lr = 0x82304318;
	sub_82303CA0(ctx, base);
	// 82304318: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8230431C: 48EA3E98  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82304320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82304320 size=128
    let mut pc: u32 = 0x82304320;
    'dispatch: loop {
        match pc {
            0x82304320 => {
    //   block [0x82304320..0x823043A0)
	// 82304320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82304324: 48EA3E49  bl 0x831a816c
	ctx.lr = 0x82304328;
	sub_831A8130(ctx, base);
	// 82304328: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8230432C: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 82304330: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82304334: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82304338: 3BEB2640  addi r31, r11, 0x2640
	ctx.r[31].s64 = ctx.r[11].s64 + 9792;
	// 8230433C: 816A2648  lwz r11, 0x2648(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(9800 as u32) ) } as u64;
	// 82304340: 556907FF  clrlwi. r9, r11, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82304344: 40820024  bne 0x82304368
	if !ctx.cr[0].eq {
	pc = 0x82304368; continue 'dispatch;
	}
	// 82304348: 3D208290  lis r9, -0x7d70
	ctx.r[9].s64 = -2104492032;
	// 8230434C: 3D008230  lis r8, -0x7dd0
	ctx.r[8].s64 = -2110783488;
	// 82304350: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 82304354: 3929DD90  addi r9, r9, -0x2270
	ctx.r[9].s64 = ctx.r[9].s64 + -8816;
	// 82304358: 39081EE0  addi r8, r8, 0x1ee0
	ctx.r[8].s64 = ctx.r[8].s64 + 7904;
	// 8230435C: 916A2648  stw r11, 0x2648(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(9800 as u32), ctx.r[11].u32 ) };
	// 82304360: 913F0004  stw r9, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82304364: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82304368: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 8230436C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82304370: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82304374: 38DE0008  addi r6, r30, 8
	ctx.r[6].s64 = ctx.r[30].s64 + 8;
	// 82304378: 9BAB0000  stb r29, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u8 ) };
	// 8230437C: 88E10050  lbz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82304380: 48350241  bl 0x826545c0
	ctx.lr = 0x82304384;
	sub_826545C0(ctx, base);
	// 82304384: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82304388: 4182000C  beq 0x82304394
	if ctx.cr[0].eq {
	pc = 0x82304394; continue 'dispatch;
	}
	// 8230438C: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82304390: 48000008  b 0x82304398
	pc = 0x82304398; continue 'dispatch;
	// 82304394: 93BE0000  stw r29, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82304398: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8230439C: 48EA3E20  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823043A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823043A0 size=208
    let mut pc: u32 = 0x823043A0;
    'dispatch: loop {
        match pc {
            0x823043A0 => {
    //   block [0x823043A0..0x82304470)
	// 823043A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823043A4: 48EA3DC5  bl 0x831a8168
	ctx.lr = 0x823043A8;
	sub_831A8130(ctx, base);
	// 823043A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823043AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823043B0: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 823043B4: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 823043B8: 556B07FF  clrlwi. r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823043BC: 40820024  bne 0x823043e0
	if !ctx.cr[0].eq {
	pc = 0x823043E0; continue 'dispatch;
	}
	// 823043C0: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 823043C4: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 823043C8: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 823043CC: 556BF87E  srwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shr(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823043D0: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 823043D4: 4199000C  bgt cr6, 0x823043e0
	if ctx.cr[6].gt {
	pc = 0x823043E0; continue 'dispatch;
	}
	// 823043D8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823043DC: 48516F75  bl 0x8281b350
	ctx.lr = 0x823043E0;
	sub_8281B350(ctx, base);
	// 823043E0: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 823043E4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823043E8: 409A000C  bne cr6, 0x823043f4
	if !ctx.cr[6].eq {
	pc = 0x823043F4; continue 'dispatch;
	}
	// 823043EC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 823043F0: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823043F4: 3BABFFFF  addi r29, r11, -1
	ctx.r[29].s64 = ctx.r[11].s64 + -1;
	// 823043F8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 823043FC: 57BE083A  rlwinm r30, r29, 1, 0, 0x1d
	ctx.r[30].u64 = ctx.r[29].u32 as u64 & 0x7FFFFFFFu64;
	// 82304400: 7D6BF02E  lwzx r11, r11, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82304404: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82304408: 409A0028  bne cr6, 0x82304430
	if !ctx.cr[6].eq {
	pc = 0x82304430; continue 'dispatch;
	}
	// 8230440C: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82304410: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82304414: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 82304418: 388A08B0  addi r4, r10, 0x8b0
	ctx.r[4].s64 = ctx.r[10].s64 + 2224;
	// 8230441C: 38A0002D  li r5, 0x2d
	ctx.r[5].s64 = 45;
	// 82304420: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82304424: 48AEDCA5  bl 0x82df20c8
	ctx.lr = 0x82304428;
	sub_82DF20C8(ctx, base);
	// 82304428: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8230442C: 7C6BF12E  stwx r3, r11, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32), ctx.r[3].u32) };
	// 82304430: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82304434: 57AB1F38  rlwinm r11, r29, 3, 0x1c, 0x1c
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x1FFFFFFFu64;
	// 82304438: 7D4AF02E  lwzx r10, r10, r30
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 8230443C: 7FCA5A15  add. r30, r10, r11
	ctx.r[30].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82304440: 41820018  beq 0x82304458
	if ctx.cr[0].eq {
	pc = 0x82304458; continue 'dispatch;
	}
	// 82304444: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82304448: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8230444C: 48AEF7B5  bl 0x82df3c00
	ctx.lr = 0x82304450;
	sub_82DF3C00(ctx, base);
	// 82304450: 897C0004  lbz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82304454: 997E0004  stb r11, 4(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u8 ) };
	// 82304458: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8230445C: 93BF000C  stw r29, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 82304460: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82304464: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82304468: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8230446C: 48EA3D4C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82304470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82304470 size=260
    let mut pc: u32 = 0x82304470;
    'dispatch: loop {
        match pc {
            0x82304470 => {
    //   block [0x82304470..0x82304574)
	// 82304470: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82304474: 48EA3CF9  bl 0x831a816c
	ctx.lr = 0x82304478;
	sub_831A8130(ctx, base);
	// 82304478: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8230447C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82304480: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82304484: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82304488: 817F01AC  lwz r11, 0x1ac(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(428 as u32) ) } as u64;
	// 8230448C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82304490: 409A0028  bne cr6, 0x823044b8
	if !ctx.cr[6].eq {
	pc = 0x823044B8; continue 'dispatch;
	}
	// 82304494: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82304498: 48AEF769  bl 0x82df3c00
	ctx.lr = 0x8230449C;
	sub_82DF3C00(ctx, base);
	// 8230449C: 9BA1005C  stb r29, 0x5c(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[29].u8 ) };
	// 823044A0: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 823044A4: 387F019C  addi r3, r31, 0x19c
	ctx.r[3].s64 = ctx.r[31].s64 + 412;
	// 823044A8: 4BFFFEF9  bl 0x823043a0
	ctx.lr = 0x823044AC;
	sub_823043A0(ctx, base);
	// 823044AC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 823044B0: 48AEEF79  bl 0x82df3428
	ctx.lr = 0x823044B4;
	sub_82DF3428(ctx, base);
	// 823044B4: 48000070  b 0x82304524
	pc = 0x82304524; continue 'dispatch;
	// 823044B8: 815F01A8  lwz r10, 0x1a8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(424 as u32) ) } as u64;
	// 823044BC: 813F01A4  lwz r9, 0x1a4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(420 as u32) ) } as u64;
	// 823044C0: 554BF87E  srwi r11, r10, 1
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shr(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823044C4: 554A07FE  clrlwi r10, r10, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 823044C8: 7F095840  cmplw cr6, r9, r11
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[11].u32, &mut ctx.xer);
	// 823044CC: 41990008  bgt cr6, 0x823044d4
	if ctx.cr[6].gt {
	pc = 0x823044D4; continue 'dispatch;
	}
	// 823044D0: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 823044D4: 813F01A0  lwz r9, 0x1a0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(416 as u32) ) } as u64;
	// 823044D8: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 823044DC: 554B1838  slwi r11, r10, 3
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823044E0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 823044E4: 7D48482E  lwzx r10, r8, r9
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 823044E8: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 823044EC: 48AEF6E5  bl 0x82df3bd0
	ctx.lr = 0x823044F0;
	sub_82DF3BD0(ctx, base);
	// 823044F0: 813F01A8  lwz r9, 0x1a8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(424 as u32) ) } as u64;
	// 823044F4: 815F01A4  lwz r10, 0x1a4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(420 as u32) ) } as u64;
	// 823044F8: 552BF87E  srwi r11, r9, 1
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shr(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823044FC: 552907FE  clrlwi r9, r9, 0x1f
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x00000001u64;
	// 82304500: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82304504: 41990008  bgt cr6, 0x8230450c
	if ctx.cr[6].gt {
	pc = 0x8230450C; continue 'dispatch;
	}
	// 82304508: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8230450C: 815F01A0  lwz r10, 0x1a0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(416 as u32) ) } as u64;
	// 82304510: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82304514: 552B1838  slwi r11, r9, 3
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82304518: 7D48502E  lwzx r10, r8, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8230451C: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82304520: 9BAB0004  stb r29, 4(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[29].u8 ) };
	// 82304524: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82304528: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8230452C: 48AEF6D5  bl 0x82df3c00
	ctx.lr = 0x82304530;
	sub_82DF3C00(ctx, base);
	// 82304530: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82304534: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82304538: 389F013C  addi r4, r31, 0x13c
	ctx.r[4].s64 = ctx.r[31].s64 + 316;
	// 8230453C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82304540: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82304544: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82304548: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8230454C: 48B5A415  bl 0x82e5e960
	ctx.lr = 0x82304550;
	sub_82E5E960(ctx, base);
	// 82304550: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82304554: 83E30000  lwz r31, 0(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82304558: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8230455C: 419A000C  beq cr6, 0x82304568
	if ctx.cr[6].eq {
	pc = 0x82304568; continue 'dispatch;
	}
	// 82304560: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82304564: 4BFBC32D  bl 0x822c0890
	ctx.lr = 0x82304568;
	sub_822C0890(ctx, base);
	// 82304568: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8230456C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82304570: 48EA3C4C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82304578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82304578 size=260
    let mut pc: u32 = 0x82304578;
    'dispatch: loop {
        match pc {
            0x82304578 => {
    //   block [0x82304578..0x8230467C)
	// 82304578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8230457C: 48EA3BED  bl 0x831a8168
	ctx.lr = 0x82304580;
	sub_831A8130(ctx, base);
	// 82304580: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82304584: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82304588: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8230458C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82304590: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82304594: 817F01AC  lwz r11, 0x1ac(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(428 as u32) ) } as u64;
	// 82304598: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8230459C: 419A0038  beq cr6, 0x823045d4
	if ctx.cr[6].eq {
	pc = 0x823045D4; continue 'dispatch;
	}
	// 823045A0: 813F01A8  lwz r9, 0x1a8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(424 as u32) ) } as u64;
	// 823045A4: 815F01A4  lwz r10, 0x1a4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(420 as u32) ) } as u64;
	// 823045A8: 552BF87E  srwi r11, r9, 1
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shr(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823045AC: 552907FE  clrlwi r9, r9, 0x1f
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x00000001u64;
	// 823045B0: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 823045B4: 41990008  bgt cr6, 0x823045bc
	if ctx.cr[6].gt {
	pc = 0x823045BC; continue 'dispatch;
	}
	// 823045B8: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 823045BC: 815F01A0  lwz r10, 0x1a0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(416 as u32) ) } as u64;
	// 823045C0: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 823045C4: 552B1838  slwi r11, r9, 3
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823045C8: 7D48502E  lwzx r10, r8, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 823045CC: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 823045D0: 8BCB0004  lbz r30, 4(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823045D4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 823045D8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 823045DC: 48AEF625  bl 0x82df3c00
	ctx.lr = 0x823045E0;
	sub_82DF3C00(ctx, base);
	// 823045E0: 9B81005C  stb r28, 0x5c(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[28].u8 ) };
	// 823045E4: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 823045E8: 387F019C  addi r3, r31, 0x19c
	ctx.r[3].s64 = ctx.r[31].s64 + 412;
	// 823045EC: 4BFFFDB5  bl 0x823043a0
	ctx.lr = 0x823045F0;
	sub_823043A0(ctx, base);
	// 823045F0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 823045F4: 48AEEE35  bl 0x82df3428
	ctx.lr = 0x823045F8;
	sub_82DF3428(ctx, base);
	// 823045F8: 57CB063F  clrlwi. r11, r30, 0x18
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823045FC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82304600: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82304604: 41820030  beq 0x82304634
	if ctx.cr[0].eq {
	pc = 0x82304634; continue 'dispatch;
	}
	// 82304608: 48AEF5F9  bl 0x82df3c00
	ctx.lr = 0x8230460C;
	sub_82DF3C00(ctx, base);
	// 8230460C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82304610: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82304614: 389F013C  addi r4, r31, 0x13c
	ctx.r[4].s64 = ctx.r[31].s64 + 316;
	// 82304618: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8230461C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82304620: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82304624: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82304628: 48B5A339  bl 0x82e5e960
	ctx.lr = 0x8230462C;
	sub_82E5E960(ctx, base);
	// 8230462C: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82304630: 4800002C  b 0x8230465c
	pc = 0x8230465C; continue 'dispatch;
	// 82304634: 48AEF5CD  bl 0x82df3c00
	ctx.lr = 0x82304638;
	sub_82DF3C00(ctx, base);
	// 82304638: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8230463C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82304640: 389F013C  addi r4, r31, 0x13c
	ctx.r[4].s64 = ctx.r[31].s64 + 316;
	// 82304644: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82304648: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8230464C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82304650: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82304654: 48B5A3A5  bl 0x82e5e9f8
	ctx.lr = 0x82304658;
	sub_82E5E9F8(ctx, base);
	// 82304658: 8161006C  lwz r11, 0x6c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 8230465C: 83E30000  lwz r31, 0(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82304660: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82304664: 419A000C  beq cr6, 0x82304670
	if ctx.cr[6].eq {
	pc = 0x82304670; continue 'dispatch;
	}
	// 82304668: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8230466C: 4BFBC225  bl 0x822c0890
	ctx.lr = 0x82304670;
	sub_822C0890(ctx, base);
	// 82304670: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82304674: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82304678: 48EA3B40  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82304680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82304680 size=268
    let mut pc: u32 = 0x82304680;
    'dispatch: loop {
        match pc {
            0x82304680 => {
    //   block [0x82304680..0x8230478C)
	// 82304680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82304684: 48EA3AE9  bl 0x831a816c
	ctx.lr = 0x82304688;
	sub_831A8130(ctx, base);
	// 82304688: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 8230468C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82304690: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82304694: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82304698: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8230469C: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 823046A0: 817F0220  lwz r11, 0x220(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(544 as u32) ) } as u64;
	// 823046A4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823046A8: 409A0028  bne cr6, 0x823046d0
	if !ctx.cr[6].eq {
	pc = 0x823046D0; continue 'dispatch;
	}
	// 823046AC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 823046B0: 48AEF551  bl 0x82df3c00
	ctx.lr = 0x823046B4;
	sub_82DF3C00(ctx, base);
	// 823046B4: 9BA1005C  stb r29, 0x5c(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[29].u8 ) };
	// 823046B8: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 823046BC: 387F0210  addi r3, r31, 0x210
	ctx.r[3].s64 = ctx.r[31].s64 + 528;
	// 823046C0: 4BFFFCE1  bl 0x823043a0
	ctx.lr = 0x823046C4;
	sub_823043A0(ctx, base);
	// 823046C4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 823046C8: 48AEED61  bl 0x82df3428
	ctx.lr = 0x823046CC;
	sub_82DF3428(ctx, base);
	// 823046CC: 48000070  b 0x8230473c
	pc = 0x8230473C; continue 'dispatch;
	// 823046D0: 815F021C  lwz r10, 0x21c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(540 as u32) ) } as u64;
	// 823046D4: 813F0218  lwz r9, 0x218(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(536 as u32) ) } as u64;
	// 823046D8: 554BF87E  srwi r11, r10, 1
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shr(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823046DC: 554A07FE  clrlwi r10, r10, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 823046E0: 7F095840  cmplw cr6, r9, r11
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[11].u32, &mut ctx.xer);
	// 823046E4: 41990008  bgt cr6, 0x823046ec
	if ctx.cr[6].gt {
	pc = 0x823046EC; continue 'dispatch;
	}
	// 823046E8: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 823046EC: 813F0214  lwz r9, 0x214(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(532 as u32) ) } as u64;
	// 823046F0: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 823046F4: 554B1838  slwi r11, r10, 3
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823046F8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 823046FC: 7D48482E  lwzx r10, r8, r9
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82304700: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82304704: 48AEF4CD  bl 0x82df3bd0
	ctx.lr = 0x82304708;
	sub_82DF3BD0(ctx, base);
	// 82304708: 813F021C  lwz r9, 0x21c(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(540 as u32) ) } as u64;
	// 8230470C: 815F0218  lwz r10, 0x218(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(536 as u32) ) } as u64;
	// 82304710: 552BF87E  srwi r11, r9, 1
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shr(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82304714: 552907FE  clrlwi r9, r9, 0x1f
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x00000001u64;
	// 82304718: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8230471C: 41990008  bgt cr6, 0x82304724
	if ctx.cr[6].gt {
	pc = 0x82304724; continue 'dispatch;
	}
	// 82304720: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82304724: 811F0214  lwz r8, 0x214(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(532 as u32) ) } as u64;
	// 82304728: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8230472C: 552A1838  slwi r10, r9, 3
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82304730: 7D6B402E  lwzx r11, r11, r8
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82304734: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82304738: 9BAB0004  stb r29, 4(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[29].u8 ) };
	// 8230473C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82304740: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82304744: 48AEF4BD  bl 0x82df3c00
	ctx.lr = 0x82304748;
	sub_82DF3C00(ctx, base);
	// 82304748: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8230474C: 389F01B0  addi r4, r31, 0x1b0
	ctx.r[4].s64 = ctx.r[31].s64 + 432;
	// 82304750: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82304754: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82304758: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8230475C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82304760: 48B5A201  bl 0x82e5e960
	ctx.lr = 0x82304764;
	sub_82E5E960(ctx, base);
	// 82304764: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82304768: 83E30000  lwz r31, 0(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8230476C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82304770: 419A000C  beq cr6, 0x8230477c
	if ctx.cr[6].eq {
	pc = 0x8230477C; continue 'dispatch;
	}
	// 82304774: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82304778: 4BFBC119  bl 0x822c0890
	ctx.lr = 0x8230477C;
	sub_822C0890(ctx, base);
	// 8230477C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82304780: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82304784: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 82304788: 48EA3A34  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82304790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82304790 size=264
    let mut pc: u32 = 0x82304790;
    'dispatch: loop {
        match pc {
            0x82304790 => {
    //   block [0x82304790..0x82304898)
	// 82304790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82304794: 48EA39D5  bl 0x831a8168
	ctx.lr = 0x82304798;
	sub_831A8130(ctx, base);
	// 82304798: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 8230479C: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823047A0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823047A4: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 823047A8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 823047AC: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 823047B0: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 823047B4: 817F0220  lwz r11, 0x220(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(544 as u32) ) } as u64;
	// 823047B8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823047BC: 419A0038  beq cr6, 0x823047f4
	if ctx.cr[6].eq {
	pc = 0x823047F4; continue 'dispatch;
	}
	// 823047C0: 813F021C  lwz r9, 0x21c(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(540 as u32) ) } as u64;
	// 823047C4: 815F0218  lwz r10, 0x218(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(536 as u32) ) } as u64;
	// 823047C8: 552BF87E  srwi r11, r9, 1
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shr(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823047CC: 552907FE  clrlwi r9, r9, 0x1f
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x00000001u64;
	// 823047D0: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 823047D4: 41990008  bgt cr6, 0x823047dc
	if ctx.cr[6].gt {
	pc = 0x823047DC; continue 'dispatch;
	}
	// 823047D8: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 823047DC: 815F0214  lwz r10, 0x214(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(532 as u32) ) } as u64;
	// 823047E0: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 823047E4: 552B1838  slwi r11, r9, 3
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823047E8: 7D48502E  lwzx r10, r8, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 823047EC: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 823047F0: 8BCB0004  lbz r30, 4(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823047F4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 823047F8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 823047FC: 48AEF405  bl 0x82df3c00
	ctx.lr = 0x82304800;
	sub_82DF3C00(ctx, base);
	// 82304800: 9B81005C  stb r28, 0x5c(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[28].u8 ) };
	// 82304804: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82304808: 387F0210  addi r3, r31, 0x210
	ctx.r[3].s64 = ctx.r[31].s64 + 528;
	// 8230480C: 4BFFFB95  bl 0x823043a0
	ctx.lr = 0x82304810;
	sub_823043A0(ctx, base);
	// 82304810: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82304814: 48AEEC15  bl 0x82df3428
	ctx.lr = 0x82304818;
	sub_82DF3428(ctx, base);
	// 82304818: 57CB063F  clrlwi. r11, r30, 0x18
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8230481C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82304820: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82304824: 4182002C  beq 0x82304850
	if ctx.cr[0].eq {
	pc = 0x82304850; continue 'dispatch;
	}
	// 82304828: 48AEF3D9  bl 0x82df3c00
	ctx.lr = 0x8230482C;
	sub_82DF3C00(ctx, base);
	// 8230482C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82304830: 389F01B0  addi r4, r31, 0x1b0
	ctx.r[4].s64 = ctx.r[31].s64 + 432;
	// 82304834: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82304838: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8230483C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82304840: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82304844: 48B5A11D  bl 0x82e5e960
	ctx.lr = 0x82304848;
	sub_82E5E960(ctx, base);
	// 82304848: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 8230484C: 48000028  b 0x82304874
	pc = 0x82304874; continue 'dispatch;
	// 82304850: 48AEF3B1  bl 0x82df3c00
	ctx.lr = 0x82304854;
	sub_82DF3C00(ctx, base);
	// 82304854: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82304858: 389F01B0  addi r4, r31, 0x1b0
	ctx.r[4].s64 = ctx.r[31].s64 + 432;
	// 8230485C: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82304860: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82304864: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82304868: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8230486C: 48B5A18D  bl 0x82e5e9f8
	ctx.lr = 0x82304870;
	sub_82E5E9F8(ctx, base);
	// 82304870: 8161006C  lwz r11, 0x6c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 82304874: 83E30000  lwz r31, 0(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82304878: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8230487C: 419A000C  beq cr6, 0x82304888
	if ctx.cr[6].eq {
	pc = 0x82304888; continue 'dispatch;
	}
	// 82304880: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82304884: 4BFBC00D  bl 0x822c0890
	ctx.lr = 0x82304888;
	sub_822C0890(ctx, base);
	// 82304888: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8230488C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82304890: CBE1FFD0  lfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 82304894: 48EA3924  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82304898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82304898 size=136
    let mut pc: u32 = 0x82304898;
    'dispatch: loop {
        match pc {
            0x82304898 => {
    //   block [0x82304898..0x82304920)
	// 82304898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8230489C: 48EA38D1  bl 0x831a816c
	ctx.lr = 0x823048A0;
	sub_831A8130(ctx, base);
	// 823048A0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823048A4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823048A8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 823048AC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823048B0: 808B00C4  lwz r4, 0xc4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(196 as u32) ) } as u64;
	// 823048B4: 480011BD  bl 0x82305a70
	ctx.lr = 0x823048B8;
	sub_82305A70(ctx, base);
	// 823048B8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 823048BC: C1A10064  lfs f13, 0x64(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 823048C0: C1810060  lfs f12, 0x60(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 823048C4: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 823048C8: D1810050  stfs f12, 0x50(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 823048CC: C00B08A8  lfs f0, 0x8a8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823048D0: EC0D002A  fadds f0, f13, f0
	ctx.f[0].f64 = ((ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64;
	// 823048D4: D0010064  stfs f0, 0x64(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 823048D8: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 823048DC: C1A10068  lfs f13, 0x68(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 823048E0: 83FF0018  lwz r31, 0x18(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 823048E4: D1A10058  stfs f13, 0x58(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 823048E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823048EC: 83DF0004  lwz r30, 4(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 823048F0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 823048F4: 80BE0004  lwz r5, 4(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 823048F8: 4BFFD401  bl 0x82301cf8
	ctx.lr = 0x823048FC;
	sub_82301CF8(ctx, base);
	// 823048FC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82304900: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82304904: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82304908: 48124C29  bl 0x82429530
	ctx.lr = 0x8230490C;
	sub_82429530(ctx, base);
	// 8230490C: 93BE0004  stw r29, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82304910: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82304914: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82304918: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8230491C: 48EA38A0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82304920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82304920 size=192
    let mut pc: u32 = 0x82304920;
    'dispatch: loop {
        match pc {
            0x82304920 => {
    //   block [0x82304920..0x823049E0)
	// 82304920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82304924: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82304928: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8230492C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82304930: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82304934: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82304938: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8230493C: 897E001C  lbz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82304940: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82304944: 4182005C  beq 0x823049a0
	if ctx.cr[0].eq {
	pc = 0x823049A0; continue 'dispatch;
	}
	// 82304948: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 8230494C: 556B07FF  clrlwi. r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82304950: 41820050  beq 0x823049a0
	if ctx.cr[0].eq {
	pc = 0x823049A0; continue 'dispatch;
	}
	// 82304954: 3D408230  lis r10, -0x7dd0
	ctx.r[10].s64 = -2110783488;
	// 82304958: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 8230495C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82304960: E8A10058  ld r5, 0x58(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 82304964: 394A25F0  addi r10, r10, 0x25f0
	ctx.r[10].s64 = ctx.r[10].s64 + 9712;
	// 82304968: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8230496C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82304970: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82304974: E8810050  ld r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82304978: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8230497C: 4BFFF9A5  bl 0x82304320
	ctx.lr = 0x82304980;
	sub_82304320(ctx, base);
	// 82304980: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82304984: 807F0268  lwz r3, 0x268(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(616 as u32) ) } as u64;
	// 82304988: 481889B9  bl 0x8248d340
	ctx.lr = 0x8230498C;
	sub_8248D340(ctx, base);
	// 8230498C: 807F00C4  lwz r3, 0xc4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 82304990: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82304994: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82304998: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8230499C: 4E800421  bctrl
	ctx.lr = 0x823049A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823049A0: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 823049A4: 556B077B  rlwinm. r11, r11, 0, 0x1d, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823049A8: 41820010  beq 0x823049b8
	if ctx.cr[0].eq {
	pc = 0x823049B8; continue 'dispatch;
	}
	// 823049AC: 889E001C  lbz r4, 0x1c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 823049B0: 807F00C4  lwz r3, 0xc4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 823049B4: 48000BF5  bl 0x823055a8
	ctx.lr = 0x823049B8;
	sub_823055A8(ctx, base);
	// 823049B8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 823049BC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 823049C0: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 823049C4: 4820CBD5  bl 0x82511598
	ctx.lr = 0x823049C8;
	sub_82511598(ctx, base);
	// 823049C8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 823049CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823049D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823049D4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823049D8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823049DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823049E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823049E0 size=1172
    let mut pc: u32 = 0x823049E0;
    'dispatch: loop {
        match pc {
            0x823049E0 => {
    //   block [0x823049E0..0x82304E74)
	// 823049E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823049E4: 48EA3785  bl 0x831a8168
	ctx.lr = 0x823049E8;
	sub_831A8130(ctx, base);
	// 823049E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823049EC: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 823049F0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823049F4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823049F8: 579D063F  clrlwi. r29, r28, 0x18
	ctx.r[29].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 823049FC: 41820038  beq 0x82304a34
	if ctx.cr[0].eq {
	pc = 0x82304A34; continue 'dispatch;
	}
	// 82304A00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82304A04: 48EA4F85  bl 0x831a9988
	ctx.lr = 0x82304A08;
	sub_831A9988(ctx, base);
	// 82304A08: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 82304A0C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82304A10: 386B5C00  addi r3, r11, 0x5c00
	ctx.r[3].s64 = ctx.r[11].s64 + 23552;
	// 82304A14: 48EA36E5  bl 0x831a80f8
	ctx.lr = 0x82304A18;
	sub_831A80F8(ctx, base);
	// 82304A18: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82304A1C: 41820018  beq 0x82304a34
	if ctx.cr[0].eq {
	pc = 0x82304A34; continue 'dispatch;
	}
	// 82304A20: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82304A24: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 82304A28: 4BFFC029  bl 0x82300a50
	ctx.lr = 0x82304A2C;
	sub_82300A50(ctx, base);
	// 82304A2C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82304A30: 4800043C  b 0x82304e6c
	pc = 0x82304E6C; continue 'dispatch;
	// 82304A34: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82304A38: 419A018C  beq cr6, 0x82304bc4
	if ctx.cr[6].eq {
	pc = 0x82304BC4; continue 'dispatch;
	}
	// 82304A3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82304A40: 48EA4F49  bl 0x831a9988
	ctx.lr = 0x82304A44;
	sub_831A9988(ctx, base);
	// 82304A44: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 82304A48: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82304A4C: 386B5BD4  addi r3, r11, 0x5bd4
	ctx.r[3].s64 = ctx.r[11].s64 + 23508;
	// 82304A50: 48EA36A9  bl 0x831a80f8
	ctx.lr = 0x82304A54;
	sub_831A80F8(ctx, base);
	// 82304A54: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82304A58: 41820014  beq 0x82304a6c
	if ctx.cr[0].eq {
	pc = 0x82304A6C; continue 'dispatch;
	}
	// 82304A5C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82304A60: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 82304A64: 4BFFBD4D  bl 0x823007b0
	ctx.lr = 0x82304A68;
	sub_823007B0(ctx, base);
	// 82304A68: 4BFFFFC4  b 0x82304a2c
	pc = 0x82304A2C; continue 'dispatch;
	// 82304A6C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82304A70: 419A0154  beq cr6, 0x82304bc4
	if ctx.cr[6].eq {
	pc = 0x82304BC4; continue 'dispatch;
	}
	// 82304A74: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82304A78: 48EA4F11  bl 0x831a9988
	ctx.lr = 0x82304A7C;
	sub_831A9988(ctx, base);
	// 82304A7C: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 82304A80: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82304A84: 386B5BA8  addi r3, r11, 0x5ba8
	ctx.r[3].s64 = ctx.r[11].s64 + 23464;
	// 82304A88: 48EA3671  bl 0x831a80f8
	ctx.lr = 0x82304A8C;
	sub_831A80F8(ctx, base);
	// 82304A8C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82304A90: 41820014  beq 0x82304aa4
	if ctx.cr[0].eq {
	pc = 0x82304AA4; continue 'dispatch;
	}
	// 82304A94: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82304A98: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 82304A9C: 4BFFBE3D  bl 0x823008d8
	ctx.lr = 0x82304AA0;
	sub_823008D8(ctx, base);
	// 82304AA0: 4BFFFF8C  b 0x82304a2c
	pc = 0x82304A2C; continue 'dispatch;
	// 82304AA4: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82304AA8: 419A011C  beq cr6, 0x82304bc4
	if ctx.cr[6].eq {
	pc = 0x82304BC4; continue 'dispatch;
	}
	// 82304AAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82304AB0: 48EA4ED9  bl 0x831a9988
	ctx.lr = 0x82304AB4;
	sub_831A9988(ctx, base);
	// 82304AB4: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 82304AB8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82304ABC: 386B5B78  addi r3, r11, 0x5b78
	ctx.r[3].s64 = ctx.r[11].s64 + 23416;
	// 82304AC0: 48EA3639  bl 0x831a80f8
	ctx.lr = 0x82304AC4;
	sub_831A80F8(ctx, base);
	// 82304AC4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82304AC8: 41820014  beq 0x82304adc
	if ctx.cr[0].eq {
	pc = 0x82304ADC; continue 'dispatch;
	}
	// 82304ACC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82304AD0: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 82304AD4: 4BFFBFBD  bl 0x82300a90
	ctx.lr = 0x82304AD8;
	sub_82300A90(ctx, base);
	// 82304AD8: 4BFFFF54  b 0x82304a2c
	pc = 0x82304A2C; continue 'dispatch;
	// 82304ADC: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82304AE0: 419A00E4  beq cr6, 0x82304bc4
	if ctx.cr[6].eq {
	pc = 0x82304BC4; continue 'dispatch;
	}
	// 82304AE4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82304AE8: 48EA4EA1  bl 0x831a9988
	ctx.lr = 0x82304AEC;
	sub_831A9988(ctx, base);
	// 82304AEC: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 82304AF0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82304AF4: 386B5B4C  addi r3, r11, 0x5b4c
	ctx.r[3].s64 = ctx.r[11].s64 + 23372;
	// 82304AF8: 48EA3601  bl 0x831a80f8
	ctx.lr = 0x82304AFC;
	sub_831A80F8(ctx, base);
	// 82304AFC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82304B00: 41820014  beq 0x82304b14
	if ctx.cr[0].eq {
	pc = 0x82304B14; continue 'dispatch;
	}
	// 82304B04: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82304B08: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 82304B0C: 4BFFEB05  bl 0x82303610
	ctx.lr = 0x82304B10;
	sub_82303610(ctx, base);
	// 82304B10: 4BFFFF1C  b 0x82304a2c
	pc = 0x82304A2C; continue 'dispatch;
	// 82304B14: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82304B18: 419A00AC  beq cr6, 0x82304bc4
	if ctx.cr[6].eq {
	pc = 0x82304BC4; continue 'dispatch;
	}
	// 82304B1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82304B20: 48EA4E69  bl 0x831a9988
	ctx.lr = 0x82304B24;
	sub_831A9988(ctx, base);
	// 82304B24: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 82304B28: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82304B2C: 386B5B1C  addi r3, r11, 0x5b1c
	ctx.r[3].s64 = ctx.r[11].s64 + 23324;
	// 82304B30: 48EA35C9  bl 0x831a80f8
	ctx.lr = 0x82304B34;
	sub_831A80F8(ctx, base);
	// 82304B34: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82304B38: 41820014  beq 0x82304b4c
	if ctx.cr[0].eq {
	pc = 0x82304B4C; continue 'dispatch;
	}
	// 82304B3C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82304B40: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 82304B44: 4BFFCD95  bl 0x823018d8
	ctx.lr = 0x82304B48;
	sub_823018D8(ctx, base);
	// 82304B48: 4BFFFEE4  b 0x82304a2c
	pc = 0x82304A2C; continue 'dispatch;
	// 82304B4C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82304B50: 419A0074  beq cr6, 0x82304bc4
	if ctx.cr[6].eq {
	pc = 0x82304BC4; continue 'dispatch;
	}
	// 82304B54: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82304B58: 48EA4E31  bl 0x831a9988
	ctx.lr = 0x82304B5C;
	sub_831A9988(ctx, base);
	// 82304B5C: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 82304B60: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82304B64: 386B5AEC  addi r3, r11, 0x5aec
	ctx.r[3].s64 = ctx.r[11].s64 + 23276;
	// 82304B68: 48EA3591  bl 0x831a80f8
	ctx.lr = 0x82304B6C;
	sub_831A80F8(ctx, base);
	// 82304B6C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82304B70: 41820014  beq 0x82304b84
	if ctx.cr[0].eq {
	pc = 0x82304B84; continue 'dispatch;
	}
	// 82304B74: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82304B78: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 82304B7C: 4BFFFD1D  bl 0x82304898
	ctx.lr = 0x82304B80;
	sub_82304898(ctx, base);
	// 82304B80: 4BFFFEAC  b 0x82304a2c
	pc = 0x82304A2C; continue 'dispatch;
	// 82304B84: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82304B88: 419A003C  beq cr6, 0x82304bc4
	if ctx.cr[6].eq {
	pc = 0x82304BC4; continue 'dispatch;
	}
	// 82304B8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82304B90: 48EA4DF9  bl 0x831a9988
	ctx.lr = 0x82304B94;
	sub_831A9988(ctx, base);
	// 82304B94: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 82304B98: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82304B9C: 386B5AB4  addi r3, r11, 0x5ab4
	ctx.r[3].s64 = ctx.r[11].s64 + 23220;
	// 82304BA0: 48EA3559  bl 0x831a80f8
	ctx.lr = 0x82304BA4;
	sub_831A80F8(ctx, base);
	// 82304BA4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82304BA8: 41820014  beq 0x82304bbc
	if ctx.cr[0].eq {
	pc = 0x82304BBC; continue 'dispatch;
	}
	// 82304BAC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82304BB0: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 82304BB4: 4BFFBE9D  bl 0x82300a50
	ctx.lr = 0x82304BB8;
	sub_82300A50(ctx, base);
	// 82304BB8: 4BFFFE74  b 0x82304a2c
	pc = 0x82304A2C; continue 'dispatch;
	// 82304BBC: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82304BC0: 409A003C  bne cr6, 0x82304bfc
	if !ctx.cr[6].eq {
	pc = 0x82304BFC; continue 'dispatch;
	}
	// 82304BC4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82304BC8: 48EA4DC1  bl 0x831a9988
	ctx.lr = 0x82304BCC;
	sub_831A9988(ctx, base);
	// 82304BCC: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 82304BD0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82304BD4: 386B5A88  addi r3, r11, 0x5a88
	ctx.r[3].s64 = ctx.r[11].s64 + 23176;
	// 82304BD8: 48EA3521  bl 0x831a80f8
	ctx.lr = 0x82304BDC;
	sub_831A80F8(ctx, base);
	// 82304BDC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82304BE0: 41820014  beq 0x82304bf4
	if ctx.cr[0].eq {
	pc = 0x82304BF4; continue 'dispatch;
	}
	// 82304BE4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82304BE8: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 82304BEC: 4BFFCE35  bl 0x82301a20
	ctx.lr = 0x82304BF0;
	sub_82301A20(ctx, base);
	// 82304BF0: 4BFFFE3C  b 0x82304a2c
	pc = 0x82304A2C; continue 'dispatch;
	// 82304BF4: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82304BF8: 419A0264  beq cr6, 0x82304e5c
	if ctx.cr[6].eq {
	pc = 0x82304E5C; continue 'dispatch;
	}
	// 82304BFC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82304C00: 48EA4D89  bl 0x831a9988
	ctx.lr = 0x82304C04;
	sub_831A9988(ctx, base);
	// 82304C04: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 82304C08: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82304C0C: 386B5A60  addi r3, r11, 0x5a60
	ctx.r[3].s64 = ctx.r[11].s64 + 23136;
	// 82304C10: 48EA34E9  bl 0x831a80f8
	ctx.lr = 0x82304C14;
	sub_831A80F8(ctx, base);
	// 82304C14: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82304C18: 41820014  beq 0x82304c2c
	if ctx.cr[0].eq {
	pc = 0x82304C2C; continue 'dispatch;
	}
	// 82304C1C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82304C20: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 82304C24: 4BFFFCFD  bl 0x82304920
	ctx.lr = 0x82304C28;
	sub_82304920(ctx, base);
	// 82304C28: 4BFFFE04  b 0x82304a2c
	pc = 0x82304A2C; continue 'dispatch;
	// 82304C2C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82304C30: 419A022C  beq cr6, 0x82304e5c
	if ctx.cr[6].eq {
	pc = 0x82304E5C; continue 'dispatch;
	}
	// 82304C34: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82304C38: 48EA4D51  bl 0x831a9988
	ctx.lr = 0x82304C3C;
	sub_831A9988(ctx, base);
	// 82304C3C: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 82304C40: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82304C44: 386B5A30  addi r3, r11, 0x5a30
	ctx.r[3].s64 = ctx.r[11].s64 + 23088;
	// 82304C48: 48EA34B1  bl 0x831a80f8
	ctx.lr = 0x82304C4C;
	sub_831A80F8(ctx, base);
	// 82304C4C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82304C50: 41820014  beq 0x82304c64
	if ctx.cr[0].eq {
	pc = 0x82304C64; continue 'dispatch;
	}
	// 82304C54: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82304C58: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 82304C5C: 4BFFBED5  bl 0x82300b30
	ctx.lr = 0x82304C60;
	sub_82300B30(ctx, base);
	// 82304C60: 4BFFFDCC  b 0x82304a2c
	pc = 0x82304A2C; continue 'dispatch;
	// 82304C64: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82304C68: 419A01F4  beq cr6, 0x82304e5c
	if ctx.cr[6].eq {
	pc = 0x82304E5C; continue 'dispatch;
	}
	// 82304C6C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82304C70: 48EA4D19  bl 0x831a9988
	ctx.lr = 0x82304C74;
	sub_831A9988(ctx, base);
	// 82304C74: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 82304C78: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82304C7C: 386B5A00  addi r3, r11, 0x5a00
	ctx.r[3].s64 = ctx.r[11].s64 + 23040;
	// 82304C80: 48EA3479  bl 0x831a80f8
	ctx.lr = 0x82304C84;
	sub_831A80F8(ctx, base);
	// 82304C84: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82304C88: 41820014  beq 0x82304c9c
	if ctx.cr[0].eq {
	pc = 0x82304C9C; continue 'dispatch;
	}
	// 82304C8C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82304C90: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 82304C94: 4BFFBFCD  bl 0x82300c60
	ctx.lr = 0x82304C98;
	sub_82300C60(ctx, base);
	// 82304C98: 4BFFFD94  b 0x82304a2c
	pc = 0x82304A2C; continue 'dispatch;
	// 82304C9C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82304CA0: 419A01BC  beq cr6, 0x82304e5c
	if ctx.cr[6].eq {
	pc = 0x82304E5C; continue 'dispatch;
	}
	// 82304CA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82304CA8: 48EA4CE1  bl 0x831a9988
	ctx.lr = 0x82304CAC;
	sub_831A9988(ctx, base);
	// 82304CAC: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 82304CB0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82304CB4: 386B59D8  addi r3, r11, 0x59d8
	ctx.r[3].s64 = ctx.r[11].s64 + 23000;
	// 82304CB8: 48EA3441  bl 0x831a80f8
	ctx.lr = 0x82304CBC;
	sub_831A80F8(ctx, base);
	// 82304CBC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82304CC0: 41820014  beq 0x82304cd4
	if ctx.cr[0].eq {
	pc = 0x82304CD4; continue 'dispatch;
	}
	// 82304CC4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82304CC8: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 82304CCC: 4BFFB82D  bl 0x823004f8
	ctx.lr = 0x82304CD0;
	sub_823004F8(ctx, base);
	// 82304CD0: 4BFFFD5C  b 0x82304a2c
	pc = 0x82304A2C; continue 'dispatch;
	// 82304CD4: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82304CD8: 419A0184  beq cr6, 0x82304e5c
	if ctx.cr[6].eq {
	pc = 0x82304E5C; continue 'dispatch;
	}
	// 82304CDC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82304CE0: 48EA4CA9  bl 0x831a9988
	ctx.lr = 0x82304CE4;
	sub_831A9988(ctx, base);
	// 82304CE4: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 82304CE8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82304CEC: 386B59B0  addi r3, r11, 0x59b0
	ctx.r[3].s64 = ctx.r[11].s64 + 22960;
	// 82304CF0: 48EA3409  bl 0x831a80f8
	ctx.lr = 0x82304CF4;
	sub_831A80F8(ctx, base);
	// 82304CF4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82304CF8: 41820014  beq 0x82304d0c
	if ctx.cr[0].eq {
	pc = 0x82304D0C; continue 'dispatch;
	}
	// 82304CFC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82304D00: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 82304D04: 4BFFB82D  bl 0x82300530
	ctx.lr = 0x82304D08;
	sub_82300530(ctx, base);
	// 82304D08: 4BFFFD24  b 0x82304a2c
	pc = 0x82304A2C; continue 'dispatch;
	// 82304D0C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82304D10: 419A014C  beq cr6, 0x82304e5c
	if ctx.cr[6].eq {
	pc = 0x82304E5C; continue 'dispatch;
	}
	// 82304D14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82304D18: 48EA4C71  bl 0x831a9988
	ctx.lr = 0x82304D1C;
	sub_831A9988(ctx, base);
	// 82304D1C: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 82304D20: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82304D24: 386B5984  addi r3, r11, 0x5984
	ctx.r[3].s64 = ctx.r[11].s64 + 22916;
	// 82304D28: 48EA33D1  bl 0x831a80f8
	ctx.lr = 0x82304D2C;
	sub_831A80F8(ctx, base);
	// 82304D2C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82304D30: 41820014  beq 0x82304d44
	if ctx.cr[0].eq {
	pc = 0x82304D44; continue 'dispatch;
	}
	// 82304D34: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82304D38: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 82304D3C: 4BFFB83D  bl 0x82300578
	ctx.lr = 0x82304D40;
	sub_82300578(ctx, base);
	// 82304D40: 4BFFFCEC  b 0x82304a2c
	pc = 0x82304A2C; continue 'dispatch;
	// 82304D44: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82304D48: 419A0114  beq cr6, 0x82304e5c
	if ctx.cr[6].eq {
	pc = 0x82304E5C; continue 'dispatch;
	}
	// 82304D4C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82304D50: 48EA4C39  bl 0x831a9988
	ctx.lr = 0x82304D54;
	sub_831A9988(ctx, base);
	// 82304D54: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 82304D58: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82304D5C: 386B595C  addi r3, r11, 0x595c
	ctx.r[3].s64 = ctx.r[11].s64 + 22876;
	// 82304D60: 48EA3399  bl 0x831a80f8
	ctx.lr = 0x82304D64;
	sub_831A80F8(ctx, base);
	// 82304D64: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82304D68: 41820014  beq 0x82304d7c
	if ctx.cr[0].eq {
	pc = 0x82304D7C; continue 'dispatch;
	}
	// 82304D6C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82304D70: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 82304D74: 4BFFBF05  bl 0x82300c78
	ctx.lr = 0x82304D78;
	sub_82300C78(ctx, base);
	// 82304D78: 4BFFFCB4  b 0x82304a2c
	pc = 0x82304A2C; continue 'dispatch;
	// 82304D7C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82304D80: 419A00DC  beq cr6, 0x82304e5c
	if ctx.cr[6].eq {
	pc = 0x82304E5C; continue 'dispatch;
	}
	// 82304D84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82304D88: 48EA4C01  bl 0x831a9988
	ctx.lr = 0x82304D8C;
	sub_831A9988(ctx, base);
	// 82304D8C: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 82304D90: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82304D94: 386B592C  addi r3, r11, 0x592c
	ctx.r[3].s64 = ctx.r[11].s64 + 22828;
	// 82304D98: 48EA3361  bl 0x831a80f8
	ctx.lr = 0x82304D9C;
	sub_831A80F8(ctx, base);
	// 82304D9C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82304DA0: 41820014  beq 0x82304db4
	if ctx.cr[0].eq {
	pc = 0x82304DB4; continue 'dispatch;
	}
	// 82304DA4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82304DA8: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 82304DAC: 4BFFBF35  bl 0x82300ce0
	ctx.lr = 0x82304DB0;
	sub_82300CE0(ctx, base);
	// 82304DB0: 4BFFFC7C  b 0x82304a2c
	pc = 0x82304A2C; continue 'dispatch;
	// 82304DB4: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82304DB8: 419A00A4  beq cr6, 0x82304e5c
	if ctx.cr[6].eq {
	pc = 0x82304E5C; continue 'dispatch;
	}
	// 82304DBC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82304DC0: 48EA4BC9  bl 0x831a9988
	ctx.lr = 0x82304DC4;
	sub_831A9988(ctx, base);
	// 82304DC4: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 82304DC8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82304DCC: 386B5900  addi r3, r11, 0x5900
	ctx.r[3].s64 = ctx.r[11].s64 + 22784;
	// 82304DD0: 48EA3329  bl 0x831a80f8
	ctx.lr = 0x82304DD4;
	sub_831A80F8(ctx, base);
	// 82304DD4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82304DD8: 41820014  beq 0x82304dec
	if ctx.cr[0].eq {
	pc = 0x82304DEC; continue 'dispatch;
	}
	// 82304DDC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82304DE0: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 82304DE4: 4BFFBF0D  bl 0x82300cf0
	ctx.lr = 0x82304DE8;
	sub_82300CF0(ctx, base);
	// 82304DE8: 4BFFFC44  b 0x82304a2c
	pc = 0x82304A2C; continue 'dispatch;
	// 82304DEC: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82304DF0: 419A006C  beq cr6, 0x82304e5c
	if ctx.cr[6].eq {
	pc = 0x82304E5C; continue 'dispatch;
	}
	// 82304DF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82304DF8: 48EA4B91  bl 0x831a9988
	ctx.lr = 0x82304DFC;
	sub_831A9988(ctx, base);
	// 82304DFC: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 82304E00: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82304E04: 386B58D0  addi r3, r11, 0x58d0
	ctx.r[3].s64 = ctx.r[11].s64 + 22736;
	// 82304E08: 48EA32F1  bl 0x831a80f8
	ctx.lr = 0x82304E0C;
	sub_831A80F8(ctx, base);
	// 82304E0C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82304E10: 41820014  beq 0x82304e24
	if ctx.cr[0].eq {
	pc = 0x82304E24; continue 'dispatch;
	}
	// 82304E14: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82304E18: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 82304E1C: 4BFFBF35  bl 0x82300d50
	ctx.lr = 0x82304E20;
	sub_82300D50(ctx, base);
	// 82304E20: 4BFFFC0C  b 0x82304a2c
	pc = 0x82304A2C; continue 'dispatch;
	// 82304E24: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82304E28: 419A0034  beq cr6, 0x82304e5c
	if ctx.cr[6].eq {
	pc = 0x82304E5C; continue 'dispatch;
	}
	// 82304E2C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82304E30: 48EA4B59  bl 0x831a9988
	ctx.lr = 0x82304E34;
	sub_831A9988(ctx, base);
	// 82304E34: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 82304E38: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82304E3C: 386B58A0  addi r3, r11, 0x58a0
	ctx.r[3].s64 = ctx.r[11].s64 + 22688;
	// 82304E40: 48EA32B9  bl 0x831a80f8
	ctx.lr = 0x82304E44;
	sub_831A80F8(ctx, base);
	// 82304E44: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82304E48: 41820014  beq 0x82304e5c
	if ctx.cr[0].eq {
	pc = 0x82304E5C; continue 'dispatch;
	}
	// 82304E4C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82304E50: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 82304E54: 4BFFBF75  bl 0x82300dc8
	ctx.lr = 0x82304E58;
	sub_82300DC8(ctx, base);
	// 82304E58: 4BFFFBD4  b 0x82304a2c
	pc = 0x82304A2C; continue 'dispatch;
	// 82304E5C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82304E60: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82304E64: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82304E68: 4820C731  bl 0x82511598
	ctx.lr = 0x82304E6C;
	sub_82511598(ctx, base);
	// 82304E6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82304E70: 48EA3348  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82304E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82304E78 size=188
    let mut pc: u32 = 0x82304E78;
    'dispatch: loop {
        match pc {
            0x82304E78 => {
    //   block [0x82304E78..0x82304F34)
	// 82304E78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82304E7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82304E80: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82304E84: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82304E88: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82304E8C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82304E90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82304E94: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82304E98: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82304E9C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82304EA0: 4BFBBA99  bl 0x822c0938
	ctx.lr = 0x82304EA4;
	sub_822C0938(ctx, base);
	// 82304EA4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82304EA8: 41820028  beq 0x82304ed0
	if ctx.cr[0].eq {
	pc = 0x82304ED0; continue 'dispatch;
	}
	// 82304EAC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82304EB0: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82304EB4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82304EB8: 392BCA94  addi r9, r11, -0x356c
	ctx.r[9].s64 = ctx.r[11].s64 + -13676;
	// 82304EBC: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82304EC0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82304EC4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82304EC8: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82304ECC: 48000008  b 0x82304ed4
	pc = 0x82304ED4; continue 'dispatch;
	// 82304ED0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82304ED4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82304ED8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82304EDC: 409A003C  bne cr6, 0x82304f18
	if !ctx.cr[6].eq {
	pc = 0x82304F18; continue 'dispatch;
	}
	// 82304EE0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82304EE4: 419A0014  beq cr6, 0x82304ef8
	if ctx.cr[6].eq {
	pc = 0x82304EF8; continue 'dispatch;
	}
	// 82304EE8: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82304EEC: 487E840D  bl 0x82aed2f8
	ctx.lr = 0x82304EF0;
	sub_82AED2F8(ctx, base);
	// 82304EF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82304EF4: 48AED4E5  bl 0x82df23d8
	ctx.lr = 0x82304EF8;
	sub_82DF23D8(ctx, base);
	// 82304EF8: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 82304EFC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82304F00: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82304F04: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82304F08: 816B51A4  lwz r11, 0x51a4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20900 as u32) ) } as u64;
	// 82304F0C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82304F10: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82304F14: 4BFBB0ED  bl 0x822c0000
	ctx.lr = 0x82304F18;
	sub_822C0000(ctx, base);
	// 82304F18: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82304F1C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82304F20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82304F24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82304F28: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82304F2C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82304F30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82304F38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82304F38 size=64
    let mut pc: u32 = 0x82304F38;
    'dispatch: loop {
        match pc {
            0x82304F38 => {
    //   block [0x82304F38..0x82304F78)
	// 82304F38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82304F3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82304F40: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82304F44: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82304F48: 83E3000C  lwz r31, 0xc(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82304F4C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82304F50: 419A0014  beq cr6, 0x82304f64
	if ctx.cr[6].eq {
	pc = 0x82304F64; continue 'dispatch;
	}
	// 82304F54: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82304F58: 487E83A1  bl 0x82aed2f8
	ctx.lr = 0x82304F5C;
	sub_82AED2F8(ctx, base);
	// 82304F5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82304F60: 48AED479  bl 0x82df23d8
	ctx.lr = 0x82304F64;
	sub_82DF23D8(ctx, base);
	// 82304F64: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82304F68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82304F6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82304F70: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82304F74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82304F78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82304F78 size=112
    let mut pc: u32 = 0x82304F78;
    'dispatch: loop {
        match pc {
            0x82304F78 => {
    //   block [0x82304F78..0x82304FE8)
	// 82304F78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82304F7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82304F80: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82304F84: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82304F88: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82304F8C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82304F90: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82304F94: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82304F98: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82304F9C: 4BFFFEDD  bl 0x82304e78
	ctx.lr = 0x82304FA0;
	sub_82304E78(ctx, base);
	// 82304FA0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82304FA4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82304FA8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82304FAC: 4BFBB055  bl 0x822c0000
	ctx.lr = 0x82304FB0;
	sub_822C0000(ctx, base);
	// 82304FB0: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82304FB4: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82304FB8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82304FBC: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82304FC0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82304FC4: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82304FC8: 419A0008  beq cr6, 0x82304fd0
	if ctx.cr[6].eq {
	pc = 0x82304FD0; continue 'dispatch;
	}
	// 82304FCC: 4BFBB8C5  bl 0x822c0890
	ctx.lr = 0x82304FD0;
	sub_822C0890(ctx, base);
	// 82304FD0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82304FD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82304FD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82304FDC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82304FE0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82304FE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82304FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82304FE8 size=112
    let mut pc: u32 = 0x82304FE8;
    'dispatch: loop {
        match pc {
            0x82304FE8 => {
    //   block [0x82304FE8..0x82305058)
	// 82304FE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82304FEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82304FF0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82304FF4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82304FF8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82304FFC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82305000: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82305004: 388BCB60  addi r4, r11, -0x34a0
	ctx.r[4].s64 = ctx.r[11].s64 + -13472;
	// 82305008: 38A00396  li r5, 0x396
	ctx.r[5].s64 = 918;
	// 8230500C: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 82305010: 48AED3D9  bl 0x82df23e8
	ctx.lr = 0x82305014;
	sub_82DF23E8(ctx, base);
	// 82305014: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82305018: 41820020  beq 0x82305038
	if ctx.cr[0].eq {
	pc = 0x82305038; continue 'dispatch;
	}
	// 8230501C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82305020: 389F0028  addi r4, r31, 0x28
	ctx.r[4].s64 = ctx.r[31].s64 + 40;
	// 82305024: 409A0008  bne cr6, 0x8230502c
	if !ctx.cr[6].eq {
	pc = 0x8230502C; continue 'dispatch;
	}
	// 82305028: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8230502C: 48188505  bl 0x8248d530
	ctx.lr = 0x82305030;
	sub_8248D530(ctx, base);
	// 82305030: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82305034: 48000008  b 0x8230503c
	pc = 0x8230503C; continue 'dispatch;
	// 82305038: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8230503C: 387F0268  addi r3, r31, 0x268
	ctx.r[3].s64 = ctx.r[31].s64 + 616;
	// 82305040: 4BFFFF39  bl 0x82304f78
	ctx.lr = 0x82305044;
	sub_82304F78(ctx, base);
	// 82305044: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82305048: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8230504C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82305050: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82305054: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82305058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82305058 size=212
    let mut pc: u32 = 0x82305058;
    'dispatch: loop {
        match pc {
            0x82305058 => {
    //   block [0x82305058..0x8230512C)
	// 82305058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8230505C: 48EA3105  bl 0x831a8160
	ctx.lr = 0x82305060;
	sub_831A8130(ctx, base);
	// 82305060: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82305064: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82305068: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8230506C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82305070: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82305074: 817F0268  lwz r11, 0x268(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(616 as u32) ) } as u64;
	// 82305078: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8230507C: 409A0008  bne cr6, 0x82305084
	if !ctx.cr[6].eq {
	pc = 0x82305084; continue 'dispatch;
	}
	// 82305080: 4BFFFF69  bl 0x82304fe8
	ctx.lr = 0x82305084;
	sub_82304FE8(ctx, base);
	// 82305084: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82305088: 809F00C4  lwz r4, 0xc4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 8230508C: 48003D0D  bl 0x82308d98
	ctx.lr = 0x82305090;
	sub_82308D98(ctx, base);
	// 82305090: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82305094: 837F0268  lwz r27, 0x268(r31)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(616 as u32) ) } as u64;
	// 82305098: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8230509C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 823050A0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823050A4: 4E800421  bctrl
	ctx.lr = 0x823050A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823050A8: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 823050AC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823050B0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823050B4: 4820A465  bl 0x8250f518
	ctx.lr = 0x823050B8;
	sub_8250F518(ctx, base);
	// 823050B8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 823050BC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 823050C0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 823050C4: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 823050C8: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 823050CC: 7F48D378  mr r8, r26
	ctx.r[8].u64 = ctx.r[26].u64;
	// 823050D0: 48188451  bl 0x8248d520
	ctx.lr = 0x823050D4;
	sub_8248D520(ctx, base);
	// 823050D4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823050D8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823050DC: 48AECBB5  bl 0x82df1c90
	ctx.lr = 0x823050E0;
	sub_82DF1C90(ctx, base);
	// 823050E0: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 823050E4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823050E8: 419A0008  beq cr6, 0x823050f0
	if ctx.cr[6].eq {
	pc = 0x823050F0; continue 'dispatch;
	}
	// 823050EC: 4BFBB7A5  bl 0x822c0890
	ctx.lr = 0x823050F0;
	sub_822C0890(ctx, base);
	// 823050F0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 823050F4: 809F00C4  lwz r4, 0xc4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 823050F8: 83FF0268  lwz r31, 0x268(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(616 as u32) ) } as u64;
	// 823050FC: 48003C9D  bl 0x82308d98
	ctx.lr = 0x82305100;
	sub_82308D98(ctx, base);
	// 82305100: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82305104: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82305108: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8230510C: 48187B85  bl 0x8248cc90
	ctx.lr = 0x82305110;
	sub_8248CC90(ctx, base);
	// 82305110: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82305114: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82305118: 419A0008  beq cr6, 0x82305120
	if ctx.cr[6].eq {
	pc = 0x82305120; continue 'dispatch;
	}
	// 8230511C: 4BFBB775  bl 0x822c0890
	ctx.lr = 0x82305120;
	sub_822C0890(ctx, base);
	// 82305120: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82305124: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82305128: 48EA3088  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82305130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82305130 size=300
    let mut pc: u32 = 0x82305130;
    'dispatch: loop {
        match pc {
            0x82305130 => {
    //   block [0x82305130..0x8230525C)
	// 82305130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82305134: 48EA302D  bl 0x831a8160
	ctx.lr = 0x82305138;
	sub_831A8130(ctx, base);
	// 82305138: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8230513C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82305140: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82305144: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82305148: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 8230514C: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82305150: 817F0268  lwz r11, 0x268(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(616 as u32) ) } as u64;
	// 82305154: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82305158: 409A0008  bne cr6, 0x82305160
	if !ctx.cr[6].eq {
	pc = 0x82305160; continue 'dispatch;
	}
	// 8230515C: 4BFFFE8D  bl 0x82304fe8
	ctx.lr = 0x82305160;
	sub_82304FE8(ctx, base);
	// 82305160: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82305164: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82305168: 388BCB60  addi r4, r11, -0x34a0
	ctx.r[4].s64 = ctx.r[11].s64 + -13472;
	// 8230516C: 38A003B9  li r5, 0x3b9
	ctx.r[5].s64 = 953;
	// 82305170: 386000E0  li r3, 0xe0
	ctx.r[3].s64 = 224;
	// 82305174: 48AED275  bl 0x82df23e8
	ctx.lr = 0x82305178;
	sub_82DF23E8(ctx, base);
	// 82305178: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8230517C: 41820014  beq 0x82305190
	if ctx.cr[0].eq {
	pc = 0x82305190; continue 'dispatch;
	}
	// 82305180: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82305184: 48B0DF6D  bl 0x82e130f0
	ctx.lr = 0x82305188;
	sub_82E130F0(ctx, base);
	// 82305188: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8230518C: 48000008  b 0x82305194
	pc = 0x82305194; continue 'dispatch;
	// 82305190: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82305194: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82305198: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8230519C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 823051A0: 4BFDBE79  bl 0x822e1018
	ctx.lr = 0x823051A4;
	sub_822E1018(ctx, base);
	// 823051A4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 823051A8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 823051AC: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 823051B0: 4BFBAE51  bl 0x822c0000
	ctx.lr = 0x823051B4;
	sub_822C0000(ctx, base);
	// 823051B4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 823051B8: 809F00C4  lwz r4, 0xc4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 823051BC: 83C10050  lwz r30, 0x50(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 823051C0: 48003BD9  bl 0x82308d98
	ctx.lr = 0x823051C4;
	sub_82308D98(ctx, base);
	// 823051C4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 823051C8: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 823051CC: 48B0DC65  bl 0x82e12e30
	ctx.lr = 0x823051D0;
	sub_82E12E30(ctx, base);
	// 823051D0: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 823051D4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823051D8: 419A0008  beq cr6, 0x823051e0
	if ctx.cr[6].eq {
	pc = 0x823051E0; continue 'dispatch;
	}
	// 823051DC: 4BFBB6B5  bl 0x822c0890
	ctx.lr = 0x823051E0;
	sub_822C0890(ctx, base);
	// 823051E0: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 823051E4: 83DF0268  lwz r30, 0x268(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(616 as u32) ) } as u64;
	// 823051E8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 823051EC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 823051F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823051F4: 4E800421  bctrl
	ctx.lr = 0x823051F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823051F8: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 823051FC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82305200: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82305204: 4820A315  bl 0x8250f518
	ctx.lr = 0x82305208;
	sub_8250F518(ctx, base);
	// 82305208: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8230520C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82305210: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82305214: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82305218: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 8230521C: 7F48D378  mr r8, r26
	ctx.r[8].u64 = ctx.r[26].u64;
	// 82305220: 48188301  bl 0x8248d520
	ctx.lr = 0x82305224;
	sub_8248D520(ctx, base);
	// 82305224: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82305228: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8230522C: 48AECA65  bl 0x82df1c90
	ctx.lr = 0x82305230;
	sub_82DF1C90(ctx, base);
	// 82305230: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82305234: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82305238: 807F0268  lwz r3, 0x268(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(616 as u32) ) } as u64;
	// 8230523C: 48187A55  bl 0x8248cc90
	ctx.lr = 0x82305240;
	sub_8248CC90(ctx, base);
	// 82305240: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82305244: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82305248: 419A0008  beq cr6, 0x82305250
	if ctx.cr[6].eq {
	pc = 0x82305250; continue 'dispatch;
	}
	// 8230524C: 4BFBB645  bl 0x822c0890
	ctx.lr = 0x82305250;
	sub_822C0890(ctx, base);
	// 82305250: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82305254: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82305258: 48EA2F58  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82305260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82305260 size=156
    let mut pc: u32 = 0x82305260;
    'dispatch: loop {
        match pc {
            0x82305260 => {
    //   block [0x82305260..0x823052FC)
	// 82305260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82305264: 48EA2EF9  bl 0x831a815c
	ctx.lr = 0x82305268;
	sub_831A8130(ctx, base);
	// 82305268: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8230526C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82305270: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82305274: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82305278: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 8230527C: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82305280: 817F0268  lwz r11, 0x268(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(616 as u32) ) } as u64;
	// 82305284: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82305288: 409A0008  bne cr6, 0x82305290
	if !ctx.cr[6].eq {
	pc = 0x82305290; continue 'dispatch;
	}
	// 8230528C: 4BFFFD5D  bl 0x82304fe8
	ctx.lr = 0x82305290;
	sub_82304FE8(ctx, base);
	// 82305290: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82305294: 835F0268  lwz r26, 0x268(r31)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(616 as u32) ) } as u64;
	// 82305298: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8230529C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 823052A0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823052A4: 4E800421  bctrl
	ctx.lr = 0x823052A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823052A8: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 823052AC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823052B0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823052B4: 4820A265  bl 0x8250f518
	ctx.lr = 0x823052B8;
	sub_8250F518(ctx, base);
	// 823052B8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 823052BC: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 823052C0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 823052C4: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 823052C8: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 823052CC: 7F28CB78  mr r8, r25
	ctx.r[8].u64 = ctx.r[25].u64;
	// 823052D0: 48188251  bl 0x8248d520
	ctx.lr = 0x823052D4;
	sub_8248D520(ctx, base);
	// 823052D4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823052D8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823052DC: 48AEC9B5  bl 0x82df1c90
	ctx.lr = 0x823052E0;
	sub_82DF1C90(ctx, base);
	// 823052E0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 823052E4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 823052E8: 807F0268  lwz r3, 0x268(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(616 as u32) ) } as u64;
	// 823052EC: 481879A5  bl 0x8248cc90
	ctx.lr = 0x823052F0;
	sub_8248CC90(ctx, base);
	// 823052F0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823052F4: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 823052F8: 48EA2EB4  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82305300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82305300 size=92
    let mut pc: u32 = 0x82305300;
    'dispatch: loop {
        match pc {
            0x82305300 => {
    //   block [0x82305300..0x8230535C)
	// 82305300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82305304: 48EA2E69  bl 0x831a816c
	ctx.lr = 0x82305308;
	sub_831A8130(ctx, base);
	// 82305308: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8230530C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82305310: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82305314: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82305318: 817F0268  lwz r11, 0x268(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(616 as u32) ) } as u64;
	// 8230531C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82305320: 409A0008  bne cr6, 0x82305328
	if !ctx.cr[6].eq {
	pc = 0x82305328; continue 'dispatch;
	}
	// 82305324: 4BFFFCC5  bl 0x82304fe8
	ctx.lr = 0x82305328;
	sub_82304FE8(ctx, base);
	// 82305328: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8230532C: 83FF0268  lwz r31, 0x268(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(616 as u32) ) } as u64;
	// 82305330: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82305334: 4820A1E5  bl 0x8250f518
	ctx.lr = 0x82305338;
	sub_8250F518(ctx, base);
	// 82305338: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8230533C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82305340: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82305344: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82305348: 481880D1  bl 0x8248d418
	ctx.lr = 0x8230534C;
	sub_8248D418(ctx, base);
	// 8230534C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82305350: 48AEC941  bl 0x82df1c90
	ctx.lr = 0x82305354;
	sub_82DF1C90(ctx, base);
	// 82305354: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82305358: 48EA2E64  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82305360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82305360 size=64
    let mut pc: u32 = 0x82305360;
    'dispatch: loop {
        match pc {
            0x82305360 => {
    //   block [0x82305360..0x823053A0)
	// 82305360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82305364: 48EA2E09  bl 0x831a816c
	ctx.lr = 0x82305368;
	sub_831A8130(ctx, base);
	// 82305368: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8230536C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82305370: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82305374: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82305378: 817F0268  lwz r11, 0x268(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(616 as u32) ) } as u64;
	// 8230537C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82305380: 409A0008  bne cr6, 0x82305388
	if !ctx.cr[6].eq {
	pc = 0x82305388; continue 'dispatch;
	}
	// 82305384: 4BFFFC65  bl 0x82304fe8
	ctx.lr = 0x82305388;
	sub_82304FE8(ctx, base);
	// 82305388: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8230538C: 807F0268  lwz r3, 0x268(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(616 as u32) ) } as u64;
	// 82305390: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82305394: 48187A45  bl 0x8248cdd8
	ctx.lr = 0x82305398;
	sub_8248CDD8(ctx, base);
	// 82305398: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8230539C: 48EA2E20  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823053A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823053A0 size=68
    let mut pc: u32 = 0x823053A0;
    'dispatch: loop {
        match pc {
            0x823053A0 => {
    //   block [0x823053A0..0x823053E4)
	// 823053A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823053A4: 48EA2DC9  bl 0x831a816c
	ctx.lr = 0x823053A8;
	sub_831A8130(ctx, base);
	// 823053A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823053AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823053B0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 823053B4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 823053B8: 817F0268  lwz r11, 0x268(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(616 as u32) ) } as u64;
	// 823053BC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823053C0: 409A0008  bne cr6, 0x823053c8
	if !ctx.cr[6].eq {
	pc = 0x823053C8; continue 'dispatch;
	}
	// 823053C4: 4BFFFC25  bl 0x82304fe8
	ctx.lr = 0x823053C8;
	sub_82304FE8(ctx, base);
	// 823053C8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823053CC: 807F0268  lwz r3, 0x268(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(616 as u32) ) } as u64;
	// 823053D0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 823053D4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 823053D8: 48187D91  bl 0x8248d168
	ctx.lr = 0x823053DC;
	sub_8248D168(ctx, base);
	// 823053DC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823053E0: 48EA2DDC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823053E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823053E8 size=24
    let mut pc: u32 = 0x823053E8;
    'dispatch: loop {
        match pc {
            0x823053E8 => {
    //   block [0x823053E8..0x82305400)
	// 823053E8: 81630100  lwz r11, 0x100(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(256 as u32) ) } as u64;
	// 823053EC: 386B00C0  addi r3, r11, 0xc0
	ctx.r[3].s64 = ctx.r[11].s64 + 192;
	// 823053F0: 816B00C0  lwz r11, 0xc0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(192 as u32) ) } as u64;
	// 823053F4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 823053F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823053FC: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82305400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82305400 size=40
    let mut pc: u32 = 0x82305400;
    'dispatch: loop {
        match pc {
            0x82305400 => {
    //   block [0x82305400..0x82305428)
	// 82305400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82305404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82305408: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8230540C: 80630100  lwz r3, 0x100(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(256 as u32) ) } as u64;
	// 82305410: 486039D9  bl 0x82908de8
	ctx.lr = 0x82305414;
	sub_82908DE8(ctx, base);
	// 82305414: 4BFF8E45  bl 0x822fe258
	ctx.lr = 0x82305418;
	sub_822FE258(ctx, base);
	// 82305418: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8230541C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82305420: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82305424: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82305428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82305428 size=40
    let mut pc: u32 = 0x82305428;
    'dispatch: loop {
        match pc {
            0x82305428 => {
    //   block [0x82305428..0x82305450)
	// 82305428: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8230542C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82305430: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82305434: 80630100  lwz r3, 0x100(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(256 as u32) ) } as u64;
	// 82305438: 486039B1  bl 0x82908de8
	ctx.lr = 0x8230543C;
	sub_82908DE8(ctx, base);
	// 8230543C: 4BFF8B95  bl 0x822fdfd0
	ctx.lr = 0x82305440;
	sub_822FDFD0(ctx, base);
	// 82305440: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82305444: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82305448: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8230544C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82305450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82305450 size=40
    let mut pc: u32 = 0x82305450;
    'dispatch: loop {
        match pc {
            0x82305450 => {
    //   block [0x82305450..0x82305478)
	// 82305450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82305454: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82305458: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8230545C: 80630100  lwz r3, 0x100(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(256 as u32) ) } as u64;
	// 82305460: 48603989  bl 0x82908de8
	ctx.lr = 0x82305464;
	sub_82908DE8(ctx, base);
	// 82305464: 4BFF8BFD  bl 0x822fe060
	ctx.lr = 0x82305468;
	sub_822FE060(ctx, base);
	// 82305468: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8230546C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82305470: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82305474: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82305478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82305478 size=56
    let mut pc: u32 = 0x82305478;
    'dispatch: loop {
        match pc {
            0x82305478 => {
    //   block [0x82305478..0x823054B0)
	// 82305478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8230547C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82305480: DBE1FFF0  stfd f31, -0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[31].u64 ) };
	// 82305484: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82305488: 80630100  lwz r3, 0x100(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(256 as u32) ) } as u64;
	// 8230548C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82305490: 48603959  bl 0x82908de8
	ctx.lr = 0x82305494;
	sub_82908DE8(ctx, base);
	// 82305494: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82305498: 4BFF8E71  bl 0x822fe308
	ctx.lr = 0x8230549C;
	sub_822FE308(ctx, base);
	// 8230549C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 823054A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823054A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823054A8: CBE1FFF0  lfd f31, -0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823054AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823054B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823054B0 size=40
    let mut pc: u32 = 0x823054B0;
    'dispatch: loop {
        match pc {
            0x823054B0 => {
    //   block [0x823054B0..0x823054D8)
	// 823054B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823054B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823054B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823054BC: 80630100  lwz r3, 0x100(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(256 as u32) ) } as u64;
	// 823054C0: 48603929  bl 0x82908de8
	ctx.lr = 0x823054C4;
	sub_82908DE8(ctx, base);
	// 823054C4: 4BFF8EAD  bl 0x822fe370
	ctx.lr = 0x823054C8;
	sub_822FE370(ctx, base);
	// 823054C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 823054CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823054D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823054D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823054D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823054D8 size=40
    let mut pc: u32 = 0x823054D8;
    'dispatch: loop {
        match pc {
            0x823054D8 => {
    //   block [0x823054D8..0x82305500)
	// 823054D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823054DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823054E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823054E4: 80630100  lwz r3, 0x100(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(256 as u32) ) } as u64;
	// 823054E8: 48603901  bl 0x82908de8
	ctx.lr = 0x823054EC;
	sub_82908DE8(ctx, base);
	// 823054EC: 4BFF8EFD  bl 0x822fe3e8
	ctx.lr = 0x823054F0;
	sub_822FE3E8(ctx, base);
	// 823054F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 823054F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823054F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823054FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82305500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82305500 size=40
    let mut pc: u32 = 0x82305500;
    'dispatch: loop {
        match pc {
            0x82305500 => {
    //   block [0x82305500..0x82305528)
	// 82305500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82305504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82305508: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8230550C: 80630100  lwz r3, 0x100(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(256 as u32) ) } as u64;
	// 82305510: 486038D9  bl 0x82908de8
	ctx.lr = 0x82305514;
	sub_82908DE8(ctx, base);
	// 82305514: 4BFF8F4D  bl 0x822fe460
	ctx.lr = 0x82305518;
	sub_822FE460(ctx, base);
	// 82305518: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8230551C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82305520: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82305524: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82305528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82305528 size=8
    let mut pc: u32 = 0x82305528;
    'dispatch: loop {
        match pc {
            0x82305528 => {
    //   block [0x82305528..0x82305530)
	// 82305528: 80630100  lwz r3, 0x100(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(256 as u32) ) } as u64;
	// 8230552C: 4BFFCD04  b 0x82302230
	sub_82302230(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82305530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82305530 size=8
    let mut pc: u32 = 0x82305530;
    'dispatch: loop {
        match pc {
            0x82305530 => {
    //   block [0x82305530..0x82305538)
	// 82305530: 80630100  lwz r3, 0x100(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(256 as u32) ) } as u64;
	// 82305534: 4BFFCD74  b 0x823022a8
	sub_823022A8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82305538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82305538 size=8
    let mut pc: u32 = 0x82305538;
    'dispatch: loop {
        match pc {
            0x82305538 => {
    //   block [0x82305538..0x82305540)
	// 82305538: 80630100  lwz r3, 0x100(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(256 as u32) ) } as u64;
	// 8230553C: 4BFFEF34  b 0x82304470
	sub_82304470(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82305540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82305540 size=8
    let mut pc: u32 = 0x82305540;
    'dispatch: loop {
        match pc {
            0x82305540 => {
    //   block [0x82305540..0x82305548)
	// 82305540: 80630100  lwz r3, 0x100(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(256 as u32) ) } as u64;
	// 82305544: 4BFFF034  b 0x82304578
	sub_82304578(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82305548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82305548 size=8
    let mut pc: u32 = 0x82305548;
    'dispatch: loop {
        match pc {
            0x82305548 => {
    //   block [0x82305548..0x82305550)
	// 82305548: 80630100  lwz r3, 0x100(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(256 as u32) ) } as u64;
	// 8230554C: 4BFFE924  b 0x82303e70
	sub_82303E70(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82305550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82305550 size=8
    let mut pc: u32 = 0x82305550;
    'dispatch: loop {
        match pc {
            0x82305550 => {
    //   block [0x82305550..0x82305558)
	// 82305550: 80630100  lwz r3, 0x100(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(256 as u32) ) } as u64;
	// 82305554: 4BFFDABC  b 0x82303010
	sub_82303010(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82305558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82305558 size=8
    let mut pc: u32 = 0x82305558;
    'dispatch: loop {
        match pc {
            0x82305558 => {
    //   block [0x82305558..0x82305560)
	// 82305558: 80630100  lwz r3, 0x100(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(256 as u32) ) } as u64;
	// 8230555C: 4BFFF124  b 0x82304680
	sub_82304680(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82305560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82305560 size=8
    let mut pc: u32 = 0x82305560;
    'dispatch: loop {
        match pc {
            0x82305560 => {
    //   block [0x82305560..0x82305568)
	// 82305560: 80630100  lwz r3, 0x100(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(256 as u32) ) } as u64;
	// 82305564: 4BFFF22C  b 0x82304790
	sub_82304790(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82305568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82305568 size=8
    let mut pc: u32 = 0x82305568;
    'dispatch: loop {
        match pc {
            0x82305568 => {
    //   block [0x82305568..0x82305570)
	// 82305568: 80630100  lwz r3, 0x100(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(256 as u32) ) } as u64;
	// 8230556C: 4BFFEB5C  b 0x823040c8
	sub_823040C8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82305570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82305570 size=8
    let mut pc: u32 = 0x82305570;
    'dispatch: loop {
        match pc {
            0x82305570 => {
    //   block [0x82305570..0x82305578)
	// 82305570: 80630100  lwz r3, 0x100(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(256 as u32) ) } as u64;
	// 82305574: 4BFFCEAC  b 0x82302420
	sub_82302420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82305578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82305578 size=8
    let mut pc: u32 = 0x82305578;
    'dispatch: loop {
        match pc {
            0x82305578 => {
    //   block [0x82305578..0x82305580)
	// 82305578: 80630100  lwz r3, 0x100(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(256 as u32) ) } as u64;
	// 8230557C: 4BFFCF9C  b 0x82302518
	sub_82302518(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82305580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82305580 size=8
    let mut pc: u32 = 0x82305580;
    'dispatch: loop {
        match pc {
            0x82305580 => {
    //   block [0x82305580..0x82305588)
	// 82305580: 80630100  lwz r3, 0x100(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(256 as u32) ) } as u64;
	// 82305584: 4BFFCFE4  b 0x82302568
	sub_82302568(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82305588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82305588 size=8
    let mut pc: u32 = 0x82305588;
    'dispatch: loop {
        match pc {
            0x82305588 => {
    //   block [0x82305588..0x82305590)
	// 82305588: 80630100  lwz r3, 0x100(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(256 as u32) ) } as u64;
	// 8230558C: 4BFFAF64  b 0x823004f0
	sub_823004F0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82305590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82305590 size=8
    let mut pc: u32 = 0x82305590;
    'dispatch: loop {
        match pc {
            0x82305590 => {
    //   block [0x82305590..0x82305598)
	// 82305590: 80630100  lwz r3, 0x100(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(256 as u32) ) } as u64;
	// 82305594: 4BFFCAC4  b 0x82302058
	sub_82302058(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82305598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82305598 size=8
    let mut pc: u32 = 0x82305598;
    'dispatch: loop {
        match pc {
            0x82305598 => {
    //   block [0x82305598..0x823055A0)
	// 82305598: 80630100  lwz r3, 0x100(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(256 as u32) ) } as u64;
	// 8230559C: 4BFFC234  b 0x823017d0
	sub_823017D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823055A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823055A0 size=8
    let mut pc: u32 = 0x823055A0;
    'dispatch: loop {
        match pc {
            0x823055A0 => {
    //   block [0x823055A0..0x823055A8)
	// 823055A0: 80630100  lwz r3, 0x100(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(256 as u32) ) } as u64;
	// 823055A4: 4BFFC2E4  b 0x82301888
	sub_82301888(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823055A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823055A8 size=8
    let mut pc: u32 = 0x823055A8;
    'dispatch: loop {
        match pc {
            0x823055A8 => {
    //   block [0x823055A8..0x823055B0)
	// 823055A8: 98830130  stb r4, 0x130(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(304 as u32), ctx.r[4].u8 ) };
	// 823055AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823055B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823055B0 size=16
    let mut pc: u32 = 0x823055B0;
    'dispatch: loop {
        match pc {
            0x823055B0 => {
    //   block [0x823055B0..0x823055C0)
	// 823055B0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823055B4: 80630100  lwz r3, 0x100(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(256 as u32) ) } as u64;
	// 823055B8: C02BCC2C  lfs f1, -0x33d4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-13268 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 823055BC: 4BFFB334  b 0x823008f0
	sub_823008F0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823055C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823055C0 size=52
    let mut pc: u32 = 0x823055C0;
    'dispatch: loop {
        match pc {
            0x823055C0 => {
    //   block [0x823055C0..0x823055F4)
	// 823055C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823055C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823055C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823055CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823055D0: 80840100  lwz r4, 0x100(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(256 as u32) ) } as u64;
	// 823055D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823055D8: 4BFFB4C9  bl 0x82300aa0
	ctx.lr = 0x823055DC;
	sub_82300AA0(ctx, base);
	// 823055DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823055E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 823055E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823055E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823055EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823055F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823055F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823055F8 size=52
    let mut pc: u32 = 0x823055F8;
    'dispatch: loop {
        match pc {
            0x823055F8 => {
    //   block [0x823055F8..0x8230562C)
	// 823055F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823055FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82305600: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82305604: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82305608: 80840100  lwz r4, 0x100(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(256 as u32) ) } as u64;
	// 8230560C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82305610: 4BFFC581  bl 0x82301b90
	ctx.lr = 0x82305614;
	sub_82301B90(ctx, base);
	// 82305614: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82305618: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8230561C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82305620: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82305624: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82305628: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82305630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82305630 size=8
    let mut pc: u32 = 0x82305630;
    'dispatch: loop {
        match pc {
            0x82305630 => {
    //   block [0x82305630..0x82305638)
	// 82305630: 386301FC  addi r3, r3, 0x1fc
	ctx.r[3].s64 = ctx.r[3].s64 + 508;
	// 82305634: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82305638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82305638 size=12
    let mut pc: u32 = 0x82305638;
    'dispatch: loop {
        match pc {
            0x82305638 => {
    //   block [0x82305638..0x82305644)
	// 82305638: 81630138  lwz r11, 0x138(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(312 as u32) ) } as u64;
	// 8230563C: 7F0B2000  cmpw cr6, r11, r4
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[4].s32, &mut ctx.xer);
	// 82305640: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82305644(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82305644 size=20
    let mut pc: u32 = 0x82305644;
    'dispatch: loop {
        match pc {
            0x82305644 => {
    //   block [0x82305644..0x82305658)
	// 82305644: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82305648: 90830138  stw r4, 0x138(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(312 as u32), ctx.r[4].u32 ) };
	// 8230564C: 816B004C  lwz r11, 0x4c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 82305650: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82305654: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82305658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82305658 size=4
    let mut pc: u32 = 0x82305658;
    'dispatch: loop {
        match pc {
            0x82305658 => {
    //   block [0x82305658..0x8230565C)
	// 82305658: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82305660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82305660 size=124
    let mut pc: u32 = 0x82305660;
    'dispatch: loop {
        match pc {
            0x82305660 => {
    //   block [0x82305660..0x823056DC)
	// 82305660: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82305664: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82305668: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8230566C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82305670: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82305674: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82305678: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8230567C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82305680: 80830138  lwz r4, 0x138(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(312 as u32) ) } as u64;
	// 82305684: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82305688: 99410050  stb r10, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u8 ) };
	// 8230568C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82305690: 816B0050  lwz r11, 0x50(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 82305694: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82305698: 4E800421  bctrl
	ctx.lr = 0x8230569C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8230569C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 823056A0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 823056A4: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 823056A8: 48AEE651  bl 0x82df3cf8
	ctx.lr = 0x823056AC;
	sub_82DF3CF8(ctx, base);
	// 823056AC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 823056B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823056B4: 48AEE51D  bl 0x82df3bd0
	ctx.lr = 0x823056B8;
	sub_82DF3BD0(ctx, base);
	// 823056B8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 823056BC: 48AEDD6D  bl 0x82df3428
	ctx.lr = 0x823056C0;
	sub_82DF3428(ctx, base);
	// 823056C0: 88610050  lbz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 823056C4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823056C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823056CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823056D0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823056D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823056D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823056E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823056E0 size=8
    let mut pc: u32 = 0x823056E0;
    'dispatch: loop {
        match pc {
            0x823056E0 => {
    //   block [0x823056E0..0x823056E8)
	// 823056E0: 80630100  lwz r3, 0x100(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(256 as u32) ) } as u64;
	// 823056E4: 4854A6E4  b 0x8284fdc8
	sub_8284FDC8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823056E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823056E8 size=56
    let mut pc: u32 = 0x823056E8;
    'dispatch: loop {
        match pc {
            0x823056E8 => {
    //   block [0x823056E8..0x82305720)
	// 823056E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823056EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823056F0: DBE1FFF0  stfd f31, -0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[31].u64 ) };
	// 823056F4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823056F8: 80630100  lwz r3, 0x100(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(256 as u32) ) } as u64;
	// 823056FC: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82305700: 486036E9  bl 0x82908de8
	ctx.lr = 0x82305704;
	sub_82908DE8(ctx, base);
	// 82305704: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82305708: 4BFF8511  bl 0x822fdc18
	ctx.lr = 0x8230570C;
	sub_822FDC18(ctx, base);
	// 8230570C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82305710: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82305714: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82305718: CBE1FFF0  lfd f31, -0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8230571C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82305720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82305720 size=56
    let mut pc: u32 = 0x82305720;
    'dispatch: loop {
        match pc {
            0x82305720 => {
    //   block [0x82305720..0x82305758)
	// 82305720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82305724: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82305728: DBE1FFF0  stfd f31, -0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[31].u64 ) };
	// 8230572C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82305730: 80630100  lwz r3, 0x100(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(256 as u32) ) } as u64;
	// 82305734: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82305738: 486036B1  bl 0x82908de8
	ctx.lr = 0x8230573C;
	sub_82908DE8(ctx, base);
	// 8230573C: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82305740: 4BFF8541  bl 0x822fdc80
	ctx.lr = 0x82305744;
	sub_822FDC80(ctx, base);
	// 82305744: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82305748: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8230574C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82305750: CBE1FFF0  lfd f31, -0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82305754: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82305758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82305758 size=20
    let mut pc: u32 = 0x82305758;
    'dispatch: loop {
        match pc {
            0x82305758 => {
    //   block [0x82305758..0x8230576C)
	// 82305758: 80630100  lwz r3, 0x100(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(256 as u32) ) } as u64;
	// 8230575C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82305760: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82305764: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82305768: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82305770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82305770 size=8
    let mut pc: u32 = 0x82305770;
    'dispatch: loop {
        match pc {
            0x82305770 => {
    //   block [0x82305770..0x82305778)
	// 82305770: 80630100  lwz r3, 0x100(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(256 as u32) ) } as u64;
	// 82305774: 4BFFAD9C  b 0x82300510
	sub_82300510(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82305778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82305778 size=8
    let mut pc: u32 = 0x82305778;
    'dispatch: loop {
        match pc {
            0x82305778 => {
    //   block [0x82305778..0x82305780)
	// 82305778: 80630100  lwz r3, 0x100(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(256 as u32) ) } as u64;
	// 8230577C: 4BFFADA4  b 0x82300520
	sub_82300520(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82305780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82305780 size=12
    let mut pc: u32 = 0x82305780;
    'dispatch: loop {
        match pc {
            0x82305780 => {
    //   block [0x82305780..0x8230578C)
	// 82305780: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82305784: 9963013C  stb r11, 0x13c(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(316 as u32), ctx.r[11].u8 ) };
	// 82305788: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82305790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82305790 size=12
    let mut pc: u32 = 0x82305790;
    'dispatch: loop {
        match pc {
            0x82305790 => {
    //   block [0x82305790..0x8230579C)
	// 82305790: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82305794: 9963013C  stb r11, 0x13c(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(316 as u32), ctx.r[11].u8 ) };
	// 82305798: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823057A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823057A0 size=76
    let mut pc: u32 = 0x823057A0;
    'dispatch: loop {
        match pc {
            0x823057A0 => {
    //   block [0x823057A0..0x823057EC)
	// 823057A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823057A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823057A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823057AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823057B0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823057B4: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 823057B8: 48AED939  bl 0x82df30f0
	ctx.lr = 0x823057BC;
	sub_82DF30F0(ctx, base);
	// 823057BC: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 823057C0: 48AED931  bl 0x82df30f0
	ctx.lr = 0x823057C4;
	sub_82DF30F0(ctx, base);
	// 823057C4: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 823057C8: 48AED929  bl 0x82df30f0
	ctx.lr = 0x823057CC;
	sub_82DF30F0(ctx, base);
	// 823057CC: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 823057D0: 48AED921  bl 0x82df30f0
	ctx.lr = 0x823057D4;
	sub_82DF30F0(ctx, base);
	// 823057D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823057D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 823057DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823057E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823057E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823057E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823057F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823057F0 size=88
    let mut pc: u32 = 0x823057F0;
    'dispatch: loop {
        match pc {
            0x823057F0 => {
    //   block [0x823057F0..0x82305848)
	// 823057F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823057F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823057F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823057FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82305800: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82305804: 81630100  lwz r11, 0x100(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(256 as u32) ) } as u64;
	// 82305808: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8230580C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82305810: 386B00C0  addi r3, r11, 0xc0
	ctx.r[3].s64 = ctx.r[11].s64 + 192;
	// 82305814: 816B00C0  lwz r11, 0xc0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(192 as u32) ) } as u64;
	// 82305818: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8230581C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82305820: 4E800421  bctrl
	ctx.lr = 0x82305824;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82305824: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82305828: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8230582C: 488AC145  bl 0x82bb1970
	ctx.lr = 0x82305830;
	sub_82BB1970(ctx, base);
	// 82305830: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82305834: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82305838: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8230583C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82305840: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82305844: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82305848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82305848 size=8
    let mut pc: u32 = 0x82305848;
    'dispatch: loop {
        match pc {
            0x82305848 => {
    //   block [0x82305848..0x82305850)
	// 82305848: 80630100  lwz r3, 0x100(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(256 as u32) ) } as u64;
	// 8230584C: 4BFFDA7C  b 0x823032c8
	sub_823032C8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82305850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82305850 size=8
    let mut pc: u32 = 0x82305850;
    'dispatch: loop {
        match pc {
            0x82305850 => {
    //   block [0x82305850..0x82305858)
	// 82305850: 80630100  lwz r3, 0x100(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(256 as u32) ) } as u64;
	// 82305854: 4BFFDC14  b 0x82303468
	sub_82303468(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82305858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82305858 size=8
    let mut pc: u32 = 0x82305858;
    'dispatch: loop {
        match pc {
            0x82305858 => {
    //   block [0x82305858..0x82305860)
	// 82305858: 98830151  stb r4, 0x151(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(337 as u32), ctx.r[4].u8 ) };
	// 8230585C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82305860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82305860 size=8
    let mut pc: u32 = 0x82305860;
    'dispatch: loop {
        match pc {
            0x82305860 => {
    //   block [0x82305860..0x82305868)
	// 82305860: 80630100  lwz r3, 0x100(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(256 as u32) ) } as u64;
	// 82305864: 4BFFE064  b 0x823038c8
	sub_823038C8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82305868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82305868 size=8
    let mut pc: u32 = 0x82305868;
    'dispatch: loop {
        match pc {
            0x82305868 => {
    //   block [0x82305868..0x82305870)
	// 82305868: 886301F0  lbz r3, 0x1f0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(496 as u32) ) } as u64;
	// 8230586C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82305870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82305870 size=60
    let mut pc: u32 = 0x82305870;
    'dispatch: loop {
        match pc {
            0x82305870 => {
    //   block [0x82305870..0x823058AC)
	// 82305870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82305874: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82305878: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8230587C: 548B063E  clrlwi r11, r4, 0x18
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 82305880: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82305884: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 82305888: C8010050  lfd f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8230588C: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 82305890: FC200018  frsp f1, f0
	ctx.f[1].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82305894: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82305898: 4E800421  bctrl
	ctx.lr = 0x8230589C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8230589C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 823058A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823058A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823058A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823058B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823058B0 size=12
    let mut pc: u32 = 0x823058B0;
    'dispatch: loop {
        match pc {
            0x823058B0 => {
    //   block [0x823058B0..0x823058BC)
	// 823058B0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 823058B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823058B8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823058C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823058C0 size=116
    let mut pc: u32 = 0x823058C0;
    'dispatch: loop {
        match pc {
            0x823058C0 => {
    //   block [0x823058C0..0x82305934)
	// 823058C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823058C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823058C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823058CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823058D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823058D4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823058D8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823058DC: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 823058E0: 409A000C  bne cr6, 0x823058ec
	if !ctx.cr[6].eq {
	pc = 0x823058EC; continue 'dispatch;
	}
	// 823058E4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 823058E8: 48000030  b 0x82305918
	pc = 0x82305918; continue 'dispatch;
	// 823058EC: 2F050001  cmpwi cr6, r5, 1
	ctx.cr[6].compare_i32(ctx.r[5].s32, 1, &mut ctx.xer);
	// 823058F0: 419A0024  beq cr6, 0x82305914
	if ctx.cr[6].eq {
	pc = 0x82305914; continue 'dispatch;
	}
	// 823058F4: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 823058F8: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823058FC: 388B6334  addi r4, r11, 0x6334
	ctx.r[4].s64 = ctx.r[11].s64 + 25396;
	// 82305900: 48EA27F9  bl 0x831a80f8
	ctx.lr = 0x82305904;
	sub_831A80F8(ctx, base);
	// 82305904: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82305908: 4182000C  beq 0x82305914
	if ctx.cr[0].eq {
	pc = 0x82305914; continue 'dispatch;
	}
	// 8230590C: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82305910: 4800000C  b 0x8230591c
	pc = 0x8230591C; continue 'dispatch;
	// 82305914: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82305918: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8230591C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82305920: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82305924: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82305928: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8230592C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82305930: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82305938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82305938 size=136
    let mut pc: u32 = 0x82305938;
    'dispatch: loop {
        match pc {
            0x82305938 => {
    //   block [0x82305938..0x823059C0)
	// 82305938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8230593C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82305940: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82305944: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82305948: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8230594C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82305950: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82305954: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82305958: 409A0020  bne cr6, 0x82305978
	if !ctx.cr[6].eq {
	pc = 0x82305978; continue 'dispatch;
	}
	// 8230595C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82305960: 419A0048  beq cr6, 0x823059a8
	if ctx.cr[6].eq {
	pc = 0x823059A8; continue 'dispatch;
	}
	// 82305964: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82305968: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8230596C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82305970: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82305974: 48000034  b 0x823059a8
	pc = 0x823059A8; continue 'dispatch;
	// 82305978: 2F050001  cmpwi cr6, r5, 1
	ctx.cr[6].compare_i32(ctx.r[5].s32, 1, &mut ctx.xer);
	// 8230597C: 419A002C  beq cr6, 0x823059a8
	if ctx.cr[6].eq {
	pc = 0x823059A8; continue 'dispatch;
	}
	// 82305980: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 82305984: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82305988: 388B6348  addi r4, r11, 0x6348
	ctx.r[4].s64 = ctx.r[11].s64 + 25416;
	// 8230598C: 48EA276D  bl 0x831a80f8
	ctx.lr = 0x82305990;
	sub_831A80F8(ctx, base);
	// 82305990: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82305994: 4182000C  beq 0x823059a0
	if ctx.cr[0].eq {
	pc = 0x823059A0; continue 'dispatch;
	}
	// 82305998: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 8230599C: 4800000C  b 0x823059a8
	pc = 0x823059A8; continue 'dispatch;
	// 823059A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823059A4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823059A8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823059AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823059B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823059B4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823059B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823059BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823059C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823059C0 size=144
    let mut pc: u32 = 0x823059C0;
    'dispatch: loop {
        match pc {
            0x823059C0 => {
    //   block [0x823059C0..0x82305A50)
	// 823059C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823059C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823059C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823059CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823059D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823059D4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823059D8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823059DC: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 823059E0: 409A0028  bne cr6, 0x82305a08
	if !ctx.cr[6].eq {
	pc = 0x82305A08; continue 'dispatch;
	}
	// 823059E4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823059E8: 419A0050  beq cr6, 0x82305a38
	if ctx.cr[6].eq {
	pc = 0x82305A38; continue 'dispatch;
	}
	// 823059EC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 823059F0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823059F4: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 823059F8: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 823059FC: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82305A00: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82305A04: 48000034  b 0x82305a38
	pc = 0x82305A38; continue 'dispatch;
	// 82305A08: 2F050001  cmpwi cr6, r5, 1
	ctx.cr[6].compare_i32(ctx.r[5].s32, 1, &mut ctx.xer);
	// 82305A0C: 419A002C  beq cr6, 0x82305a38
	if ctx.cr[6].eq {
	pc = 0x82305A38; continue 'dispatch;
	}
	// 82305A10: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 82305A14: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82305A18: 388B63F0  addi r4, r11, 0x63f0
	ctx.r[4].s64 = ctx.r[11].s64 + 25584;
	// 82305A1C: 48EA26DD  bl 0x831a80f8
	ctx.lr = 0x82305A20;
	sub_831A80F8(ctx, base);
	// 82305A20: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82305A24: 4182000C  beq 0x82305a30
	if ctx.cr[0].eq {
	pc = 0x82305A30; continue 'dispatch;
	}
	// 82305A28: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82305A2C: 4800000C  b 0x82305a38
	pc = 0x82305A38; continue 'dispatch;
	// 82305A30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82305A34: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82305A38: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82305A3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82305A40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82305A44: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82305A48: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82305A4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82305A50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82305A50 size=20
    let mut pc: u32 = 0x82305A50;
    'dispatch: loop {
        match pc {
            0x82305A50 => {
    //   block [0x82305A50..0x82305A64)
	// 82305A50: 80630010  lwz r3, 0x10(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82305A54: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82305A58: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82305A5C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82305A60: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82305A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82305A68 size=8
    let mut pc: u32 = 0x82305A68;
    'dispatch: loop {
        match pc {
            0x82305A68 => {
    //   block [0x82305A68..0x82305A70)
	// 82305A68: 80630010  lwz r3, 0x10(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82305A6C: 482853FC  b 0x8258ae68
	sub_8258AE68(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82305A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82305A70 size=60
    let mut pc: u32 = 0x82305A70;
    'dispatch: loop {
        match pc {
            0x82305A70 => {
    //   block [0x82305A70..0x82305AAC)
	// 82305A70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82305A74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82305A78: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82305A7C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82305A80: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82305A84: 80640010  lwz r3, 0x10(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 82305A88: 484EF949  bl 0x827f53d0
	ctx.lr = 0x82305A8C;
	sub_827F53D0(ctx, base);
	// 82305A8C: 13E018C7  vcmpequd (lvx128) v31, v0, v3
	tmp.u32 = ctx.r[3].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82305AB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82305AB0 size=8
    let mut pc: u32 = 0x82305AB0;
    'dispatch: loop {
        match pc {
            0x82305AB0 => {
    //   block [0x82305AB0..0x82305AB8)
	// 82305AB0: 80630010  lwz r3, 0x10(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82305AB4: 482853FC  b 0x8258aeb0
	sub_8258AEB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82305AB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82305AB8 size=84
    let mut pc: u32 = 0x82305AB8;
    'dispatch: loop {
        match pc {
            0x82305AB8 => {
    //   block [0x82305AB8..0x82305B0C)
	// 82305AB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82305ABC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82305AC0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82305AC4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82305AC8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82305ACC: 80640010  lwz r3, 0x10(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 82305AD0: 48180FE9  bl 0x82486ab8
	ctx.lr = 0x82305AD4;
	sub_82486AB8(ctx, base);
	// 82305AD4: C0030000  lfs f0, 0(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82305AD8: C1A30004  lfs f13, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82305ADC: C1830008  lfs f12, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82305AE0: C163000C  lfs f11, 0xc(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82305AE4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82305AE8: D01F0000  stfs f0, 0(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82305AEC: D1BF0004  stfs f13, 4(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82305AF0: D19F0008  stfs f12, 8(r31)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82305AF4: D17F000C  stfs f11, 0xc(r31)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82305AF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82305AFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82305B00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82305B04: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82305B08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82305B10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82305B10 size=20
    let mut pc: u32 = 0x82305B10;
    'dispatch: loop {
        match pc {
            0x82305B10 => {
    //   block [0x82305B10..0x82305B24)
	// 82305B10: 8063010C  lwz r3, 0x10c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(268 as u32) ) } as u64;
	// 82305B14: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82305B18: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82305B1C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82305B20: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82305B28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82305B28 size=20
    let mut pc: u32 = 0x82305B28;
    'dispatch: loop {
        match pc {
            0x82305B28 => {
    //   block [0x82305B28..0x82305B3C)
	// 82305B28: 8063010C  lwz r3, 0x10c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(268 as u32) ) } as u64;
	// 82305B2C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82305B30: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82305B34: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82305B38: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82305B40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82305B40 size=20
    let mut pc: u32 = 0x82305B40;
    'dispatch: loop {
        match pc {
            0x82305B40 => {
    //   block [0x82305B40..0x82305B54)
	// 82305B40: 8063010C  lwz r3, 0x10c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(268 as u32) ) } as u64;
	// 82305B44: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82305B48: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82305B4C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82305B50: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82305B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82305B58 size=8
    let mut pc: u32 = 0x82305B58;
    'dispatch: loop {
        match pc {
            0x82305B58 => {
    //   block [0x82305B58..0x82305B60)
	// 82305B58: 80630104  lwz r3, 0x104(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(260 as u32) ) } as u64;
	// 82305B5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82305B60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82305B60 size=64
    let mut pc: u32 = 0x82305B60;
    'dispatch: loop {
        match pc {
            0x82305B60 => {
    //   block [0x82305B60..0x82305BA0)
	// 82305B60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82305B64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82305B68: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82305B6C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82305B70: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82305B74: 807F0100  lwz r3, 0x100(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(256 as u32) ) } as u64;
	// 82305B78: 4BFFAC99  bl 0x82300810
	ctx.lr = 0x82305B7C;
	sub_82300810(ctx, base);
	// 82305B7C: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82305B80: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82305B84: 388B6880  addi r4, r11, 0x6880
	ctx.r[4].s64 = ctx.r[11].s64 + 26752;
	// 82305B88: 48B0D6D9  bl 0x82e13260
	ctx.lr = 0x82305B8C;
	sub_82E13260(ctx, base);
	// 82305B8C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82305B90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82305B94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82305B98: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82305B9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82305BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82305BA0 size=144
    let mut pc: u32 = 0x82305BA0;
    'dispatch: loop {
        match pc {
            0x82305BA0 => {
    //   block [0x82305BA0..0x82305C30)
	// 82305BA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82305BA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82305BA8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82305BAC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82305BB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82305BB4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82305BB8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82305BBC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82305BC0: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 82305BC4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82305BC8: 4E800421  bctrl
	ctx.lr = 0x82305BCC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82305BCC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82305BD0: 40820038  bne 0x82305c08
	if !ctx.cr[0].eq {
	pc = 0x82305C08; continue 'dispatch;
	}
	// 82305BD4: 39600120  li r11, 0x120
	ctx.r[11].s64 = 288;
	// 82305BD8: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82305BDC: 3880000C  li r4, 0xc
	ctx.r[4].s64 = 12;
	// 82305BE0: 13FF58C7  vcmpequd (lvx128) v31, v31, v11
	tmp.u32 = ctx.r[31].u32 + ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82305C30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82305C30 size=92
    let mut pc: u32 = 0x82305C30;
    'dispatch: loop {
        match pc {
            0x82305C30 => {
    //   block [0x82305C30..0x82305C8C)
	// 82305C30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82305C34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82305C38: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82305C3C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82305C40: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82305C44: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82305C48: 808B0100  lwz r4, 0x100(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(256 as u32) ) } as u64;
	// 82305C4C: 482098CD  bl 0x8250f518
	ctx.lr = 0x82305C50;
	sub_8250F518(ctx, base);
	// 82305C50: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82305C54: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82305C58: 386BFF6C  addi r3, r11, -0x94
	ctx.r[3].s64 = ctx.r[11].s64 + -148;
	// 82305C5C: 409A0008  bne cr6, 0x82305c64
	if !ctx.cr[6].eq {
	pc = 0x82305C64; continue 'dispatch;
	}
	// 82305C60: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82305C64: 482223ED  bl 0x82528050
	ctx.lr = 0x82305C68;
	sub_82528050(ctx, base);
	// 82305C68: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82305C6C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82305C70: 48AEC021  bl 0x82df1c90
	ctx.lr = 0x82305C74;
	sub_82DF1C90(ctx, base);
	// 82305C74: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82305C78: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82305C7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82305C80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82305C84: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82305C88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82305C90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82305C90 size=76
    let mut pc: u32 = 0x82305C90;
    'dispatch: loop {
        match pc {
            0x82305C90 => {
    //   block [0x82305C90..0x82305CDC)
	// 82305C90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82305C94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82305C98: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82305C9C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82305CA0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82305CA4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82305CA8: 808B0100  lwz r4, 0x100(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(256 as u32) ) } as u64;
	// 82305CAC: 48209685  bl 0x8250f330
	ctx.lr = 0x82305CB0;
	sub_8250F330(ctx, base);
	// 82305CB0: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82305CB4: 481E3BBD  bl 0x824e9870
	ctx.lr = 0x82305CB8;
	sub_824E9870(ctx, base);
	// 82305CB8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82305CBC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82305CC0: 48AEBFD1  bl 0x82df1c90
	ctx.lr = 0x82305CC4;
	sub_82DF1C90(ctx, base);
	// 82305CC4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82305CC8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82305CCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82305CD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82305CD4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82305CD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82305CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82305CE0 size=84
    let mut pc: u32 = 0x82305CE0;
    'dispatch: loop {
        match pc {
            0x82305CE0 => {
    //   block [0x82305CE0..0x82305D34)
	// 82305CE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82305CE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82305CE8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82305CEC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82305CF0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82305CF4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82305CF8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82305CFC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82305D00: 807F01FC  lwz r3, 0x1fc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(508 as u32) ) } as u64;
	// 82305D04: 4815509D  bl 0x8245ada0
	ctx.lr = 0x82305D08;
	sub_8245ADA0(ctx, base);
	// 82305D08: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82305D0C: 41820010  beq 0x82305d1c
	if ctx.cr[0].eq {
	pc = 0x82305D1C; continue 'dispatch;
	}
	// 82305D10: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82305D14: 807F0100  lwz r3, 0x100(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(256 as u32) ) } as u64;
	// 82305D18: 4BFFBC81  bl 0x82301998
	ctx.lr = 0x82305D1C;
	sub_82301998(ctx, base);
	// 82305D1C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82305D20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82305D24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82305D28: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82305D2C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82305D30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82305D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82305D38 size=12
    let mut pc: u32 = 0x82305D38;
    'dispatch: loop {
        match pc {
            0x82305D38 => {
    //   block [0x82305D38..0x82305D44)
	// 82305D38: 80640000  lwz r3, 0(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82305D3C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82305D40: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82305D44(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82305D44 size=8
    let mut pc: u32 = 0x82305D44;
    'dispatch: loop {
        match pc {
            0x82305D44 => {
    //   block [0x82305D44..0x82305D4C)
	// 82305D44: 48840984  b 0x82b466c8
	sub_82B466C8(ctx, base);
	return;
	// 82305D48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82305D50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82305D50 size=8
    let mut pc: u32 = 0x82305D50;
    'dispatch: loop {
        match pc {
            0x82305D50 => {
    //   block [0x82305D50..0x82305D58)
	// 82305D50: 80630018  lwz r3, 0x18(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82305D54: 48B0D50C  b 0x82e13260
	sub_82E13260(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82305D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82305D58 size=92
    let mut pc: u32 = 0x82305D58;
    'dispatch: loop {
        match pc {
            0x82305D58 => {
    //   block [0x82305D58..0x82305DB4)
	// 82305D58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82305D5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82305D60: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82305D64: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82305D68: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82305D6C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82305D70: 808B0100  lwz r4, 0x100(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(256 as u32) ) } as u64;
	// 82305D74: 48209755  bl 0x8250f4c8
	ctx.lr = 0x82305D78;
	sub_8250F4C8(ctx, base);
	// 82305D78: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82305D7C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82305D80: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 82305D84: 409A0008  bne cr6, 0x82305d8c
	if !ctx.cr[6].eq {
	pc = 0x82305D8C; continue 'dispatch;
	}
	// 82305D88: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82305D8C: 48202CF5  bl 0x82508a80
	ctx.lr = 0x82305D90;
	sub_82508A80(ctx, base);
	// 82305D90: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82305D94: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82305D98: 48AEBEF9  bl 0x82df1c90
	ctx.lr = 0x82305D9C;
	sub_82DF1C90(ctx, base);
	// 82305D9C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82305DA0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82305DA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82305DA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82305DAC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82305DB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82305DB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82305DB8 size=152
    let mut pc: u32 = 0x82305DB8;
    'dispatch: loop {
        match pc {
            0x82305DB8 => {
    //   block [0x82305DB8..0x82305E50)
	// 82305DB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82305DBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82305DC0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82305DC4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82305DC8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82305DCC: 548B063F  clrlwi. r11, r4, 0x18
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82305DD0: 4182002C  beq 0x82305dfc
	if ctx.cr[0].eq {
	pc = 0x82305DFC; continue 'dispatch;
	}
	// 82305DD4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82305DD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82305DDC: 915F014C  stw r10, 0x14c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(332 as u32), ctx.r[10].u32 ) };
	// 82305DE0: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82305DE4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82305DE8: 4E800421  bctrl
	ctx.lr = 0x82305DEC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82305DEC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82305DF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82305DF4: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82305DF8: 48000018  b 0x82305e10
	pc = 0x82305E10; continue 'dispatch;
	// 82305DFC: 817F014C  lwz r11, 0x14c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(332 as u32) ) } as u64;
	// 82305E00: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82305E04: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82305E08: 917F014C  stw r11, 0x14c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(332 as u32), ctx.r[11].u32 ) };
	// 82305E0C: 816A0030  lwz r11, 0x30(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(48 as u32) ) } as u64;
	// 82305E10: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82305E14: 4E800421  bctrl
	ctx.lr = 0x82305E18;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82305E18: 83FF0010  lwz r31, 0x10(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82305E1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82305E20: 48D4C929  bl 0x83052748
	ctx.lr = 0x82305E24;
	sub_83052748(ctx, base);
	// 82305E24: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82305E28: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82305E2C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82305E30: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82305E34: C02B9528  lfs f1, -0x6ad8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27352 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82305E38: 48B0D1A9  bl 0x82e12fe0
	ctx.lr = 0x82305E3C;
	sub_82E12FE0(ctx, base);
	// 82305E3C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82305E40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82305E44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82305E48: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82305E4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82305E50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82305E50 size=8
    let mut pc: u32 = 0x82305E50;
    'dispatch: loop {
        match pc {
            0x82305E50 => {
    //   block [0x82305E50..0x82305E58)
	// 82305E50: 80630028  lwz r3, 0x28(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82305E54: 48B0D40C  b 0x82e13260
	sub_82E13260(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82305E58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82305E58 size=76
    let mut pc: u32 = 0x82305E58;
    'dispatch: loop {
        match pc {
            0x82305E58 => {
    //   block [0x82305E58..0x82305EA4)
	// 82305E58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82305E5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82305E60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82305E64: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82305E68: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82305E6C: 808B0100  lwz r4, 0x100(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(256 as u32) ) } as u64;
	// 82305E70: 48209659  bl 0x8250f4c8
	ctx.lr = 0x82305E74;
	sub_8250F4C8(ctx, base);
	// 82305E74: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82305E78: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82305E7C: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 82305E80: 409A0008  bne cr6, 0x82305e88
	if !ctx.cr[6].eq {
	pc = 0x82305E88; continue 'dispatch;
	}
	// 82305E84: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82305E88: 48202E29  bl 0x82508cb0
	ctx.lr = 0x82305E8C;
	sub_82508CB0(ctx, base);
	// 82305E8C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82305E90: 48AEBE01  bl 0x82df1c90
	ctx.lr = 0x82305E94;
	sub_82DF1C90(ctx, base);
	// 82305E94: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82305E98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82305E9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82305EA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82305EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82305EA8 size=92
    let mut pc: u32 = 0x82305EA8;
    'dispatch: loop {
        match pc {
            0x82305EA8 => {
    //   block [0x82305EA8..0x82305F04)
	// 82305EA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82305EAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82305EB0: DBE1FFF0  stfd f31, -0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[31].u64 ) };
	// 82305EB4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82305EB8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82305EBC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82305EC0: 808B0100  lwz r4, 0x100(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(256 as u32) ) } as u64;
	// 82305EC4: 48209605  bl 0x8250f4c8
	ctx.lr = 0x82305EC8;
	sub_8250F4C8(ctx, base);
	// 82305EC8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82305ECC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82305ED0: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 82305ED4: 409A0008  bne cr6, 0x82305edc
	if !ctx.cr[6].eq {
	pc = 0x82305EDC; continue 'dispatch;
	}
	// 82305ED8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82305EDC: 48202CAD  bl 0x82508b88
	ctx.lr = 0x82305EE0;
	sub_82508B88(ctx, base);
	// 82305EE0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82305EE4: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82305EE8: 48AEBDA9  bl 0x82df1c90
	ctx.lr = 0x82305EEC;
	sub_82DF1C90(ctx, base);
	// 82305EEC: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82305EF0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82305EF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82305EF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82305EFC: CBE1FFF0  lfd f31, -0x10(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82305F00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82305F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82305F08 size=84
    let mut pc: u32 = 0x82305F08;
    'dispatch: loop {
        match pc {
            0x82305F08 => {
    //   block [0x82305F08..0x82305F5C)
	// 82305F08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82305F0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82305F10: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82305F14: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82305F18: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82305F1C: 389F00A0  addi r4, r31, 0xa0
	ctx.r[4].s64 = ctx.r[31].s64 + 160;
	// 82305F20: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82305F24: 48284FE5  bl 0x8258af08
	ctx.lr = 0x82305F28;
	sub_8258AF08(ctx, base);
	// 82305F28: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82305F2C: 39400120  li r10, 0x120
	ctx.r[10].s64 = 288;
	// 82305F30: 392B6910  addi r9, r11, 0x6910
	ctx.r[9].s64 = ctx.r[11].s64 + 26896;
	// 82305F34: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82305F38: 997F013C  stb r11, 0x13c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(316 as u32), ctx.r[11].u8 ) };
	// 82305F3C: 13E048C7  vcmpequd (lvx128) v31, v0, v9
	tmp.u32 = ctx.r[9].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82305F60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82305F60 size=24
    let mut pc: u32 = 0x82305F60;
    'dispatch: loop {
        match pc {
            0x82305F60 => {
    //   block [0x82305F60..0x82305F78)
	// 82305F60: 8063010C  lwz r3, 0x10c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(268 as u32) ) } as u64;
	// 82305F64: C0240000  lfs f1, 0(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82305F68: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82305F6C: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82305F70: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82305F74: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82305F78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82305F78 size=48
    let mut pc: u32 = 0x82305F78;
    'dispatch: loop {
        match pc {
            0x82305F78 => {
    //   block [0x82305F78..0x82305FA8)
	// 82305F78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82305F7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82305F80: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82305F84: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82305F88: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82305F8C: 4BFFFB2D  bl 0x82305ab8
	ctx.lr = 0x82305F90;
	sub_82305AB8(ctx, base);
	// 82305F90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82305F94: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82305F98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82305F9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82305FA0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82305FA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82305FA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82305FA8 size=92
    let mut pc: u32 = 0x82305FA8;
    'dispatch: loop {
        match pc {
            0x82305FA8 => {
    //   block [0x82305FA8..0x82306004)
	// 82305FA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82305FAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82305FB0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82305FB4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82305FB8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82305FBC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82305FC0: 808B0100  lwz r4, 0x100(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(256 as u32) ) } as u64;
	// 82305FC4: 48209505  bl 0x8250f4c8
	ctx.lr = 0x82305FC8;
	sub_8250F4C8(ctx, base);
	// 82305FC8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82305FCC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82305FD0: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 82305FD4: 409A0008  bne cr6, 0x82305fdc
	if !ctx.cr[6].eq {
	pc = 0x82305FDC; continue 'dispatch;
	}
	// 82305FD8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82305FDC: 4820271D  bl 0x825086f8
	ctx.lr = 0x82305FE0;
	sub_825086F8(ctx, base);
	// 82305FE0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82305FE4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82305FE8: 48AEBCA9  bl 0x82df1c90
	ctx.lr = 0x82305FEC;
	sub_82DF1C90(ctx, base);
	// 82305FEC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82305FF0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82305FF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82305FF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82305FFC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82306000: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82306008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82306008 size=64
    let mut pc: u32 = 0x82306008;
    'dispatch: loop {
        match pc {
            0x82306008 => {
    //   block [0x82306008..0x82306048)
	// 82306008: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8230600C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82306010: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82306014: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82306018: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8230601C: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 82306020: 807F01FC  lwz r3, 0x1fc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(508 as u32) ) } as u64;
	// 82306024: 48154E85  bl 0x8245aea8
	ctx.lr = 0x82306028;
	sub_8245AEA8(ctx, base);
	// 82306028: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8230602C: 40820008  bne 0x82306034
	if !ctx.cr[0].eq {
	pc = 0x82306034; continue 'dispatch;
	}
	// 82306030: 807F0138  lwz r3, 0x138(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(312 as u32) ) } as u64;
	// 82306034: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82306038: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8230603C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82306040: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82306044: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82306048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82306048 size=248
    //   switch @ 0x823060A8: r11 with 15 label(s)
    //       case  0 → 0x82306120
    //       case  1 → 0x823060B8
    //       case  2 → 0x82306120
    //       case  3 → 0x823060C4
    //       case  4 → 0x823060DC
    //       case  5 → 0x823060E8
    //       case  6 → 0x823060F4
    //       case  7 → 0x82306100
    //       case  8 → 0x823060D0
    //       case  9 → 0x82306120
    //       case 10 → 0x823060B8
    //       case 11 → 0x82306120
    //       case 12 → 0x82306120
    //       case 13 → 0x823060B8
    //       case 14 → 0x8230610C
    let mut pc: u32 = 0x82306048;
    'dispatch: loop {
        match pc {
            0x82306048 => {
    //   block [0x82306048..0x823060B8)
	// 82306048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8230604C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82306050: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82306054: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82306058: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8230605C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82306060: 806301FC  lwz r3, 0x1fc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(508 as u32) ) } as u64;
	// 82306064: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 82306068: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8230606C: 48154E3D  bl 0x8245aea8
	ctx.lr = 0x82306070;
	sub_8245AEA8(ctx, base);
	// 82306070: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82306074: 40820008  bne 0x8230607c
	if !ctx.cr[0].eq {
	pc = 0x8230607C; continue 'dispatch;
	}
	// 82306078: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8230607C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82306080: 3963FFFF  addi r11, r3, -1
	ctx.r[11].s64 = ctx.r[3].s64 + -1;
	// 82306084: 995E0000  stb r10, 0(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82306088: 2B0B000E  cmplwi cr6, r11, 0xe
	ctx.cr[6].compare_u32(ctx.r[11].u32, 14 as u32, &mut ctx.xer);
	// 8230608C: 41990094  bgt cr6, 0x82306120
	if ctx.cr[6].gt {
	pc = 0x82306120; continue 'dispatch;
	}
	// 82306090: 3D808201  lis r12, -0x7dff
	ctx.r[12].s64 = -2113863680;
	// 82306094: 398CCDD8  addi r12, r12, -0x3228
	ctx.r[12].s64 = ctx.r[12].s64 + -12840;
	// 82306098: 7C0C58AE  lbzx r0, r12, r11
	ctx.r[0].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8230609C: 3D808230  lis r12, -0x7dd0
	ctx.r[12].s64 = -2110783488;
	// 823060A0: 398C60B8  addi r12, r12, 0x60b8
	ctx.r[12].s64 = ctx.r[12].s64 + 24760;
	// 823060A4: 7D8C0214  add r12, r12, r0
	ctx.r[12].u64 = ctx.r[12].u64 + ctx.r[0].u64;
	// 823060A8: 7D8903A6  mtctr r12
	ctx.ctr.u64 = ctx.r[12].u64;
	// 823060AC: 60000000  nop
	// 823060B0: 60000000  nop
	// 823060B4: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            0x823060B8 => {
    //   block [0x823060B8..0x823060C4)
	// 823060B8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823060BC: 386BCE48  addi r3, r11, -0x31b8
	ctx.r[3].s64 = ctx.r[11].s64 + -12728;
	// 823060C0: 48000068  b 0x82306128
	pc = 0x82306128; continue 'dispatch;
            }
            0x823060C4 => {
    //   block [0x823060C4..0x823060D0)
	// 823060C4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823060C8: 386BCE40  addi r3, r11, -0x31c0
	ctx.r[3].s64 = ctx.r[11].s64 + -12736;
	// 823060CC: 4800005C  b 0x82306128
	pc = 0x82306128; continue 'dispatch;
            }
            0x823060D0 => {
    //   block [0x823060D0..0x823060DC)
	// 823060D0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823060D4: 386BCE38  addi r3, r11, -0x31c8
	ctx.r[3].s64 = ctx.r[11].s64 + -12744;
	// 823060D8: 48000050  b 0x82306128
	pc = 0x82306128; continue 'dispatch;
            }
            0x823060DC => {
    //   block [0x823060DC..0x823060E8)
	// 823060DC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823060E0: 386BCE30  addi r3, r11, -0x31d0
	ctx.r[3].s64 = ctx.r[11].s64 + -12752;
	// 823060E4: 48000044  b 0x82306128
	pc = 0x82306128; continue 'dispatch;
            }
            0x823060E8 => {
    //   block [0x823060E8..0x823060F4)
	// 823060E8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823060EC: 386BCE28  addi r3, r11, -0x31d8
	ctx.r[3].s64 = ctx.r[11].s64 + -12760;
	// 823060F0: 48000038  b 0x82306128
	pc = 0x82306128; continue 'dispatch;
            }
            0x823060F4 => {
    //   block [0x823060F4..0x82306100)
	// 823060F4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823060F8: 386BCE20  addi r3, r11, -0x31e0
	ctx.r[3].s64 = ctx.r[11].s64 + -12768;
	// 823060FC: 4800002C  b 0x82306128
	pc = 0x82306128; continue 'dispatch;
            }
            0x82306100 => {
    //   block [0x82306100..0x8230610C)
	// 82306100: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82306104: 386BCE18  addi r3, r11, -0x31e8
	ctx.r[3].s64 = ctx.r[11].s64 + -12776;
	// 82306108: 48000020  b 0x82306128
	pc = 0x82306128; continue 'dispatch;
            }
            0x8230610C => {
    //   block [0x8230610C..0x82306120)
	// 8230610C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82306110: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82306114: 997E0000  stb r11, 0(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82306118: 386ACE10  addi r3, r10, -0x31f0
	ctx.r[3].s64 = ctx.r[10].s64 + -12784;
	// 8230611C: 4800000C  b 0x82306128
	pc = 0x82306128; continue 'dispatch;
            }
            0x82306120 => {
    //   block [0x82306120..0x82306140)
	// 82306120: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82306124: 386BCE08  addi r3, r11, -0x31f8
	ctx.r[3].s64 = ctx.r[11].s64 + -12792;
	// 82306128: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8230612C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82306130: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82306134: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82306138: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8230613C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82306140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82306140 size=448
    //   switch @ 0x823061A4: r11 with 15 label(s)
    //       case  0 → 0x823061B4
    //       case  1 → 0x823061CC
    //       case  2 → 0x823061F0
    //       case  3 → 0x823061FC
    //       case  4 → 0x82306214
    //       case  5 → 0x82306220
    //       case  6 → 0x8230622C
    //       case  7 → 0x82306238
    //       case  8 → 0x82306208
    //       case  9 → 0x823061C0
    //       case 10 → 0x823061D8
    //       case 11 → 0x82306258
    //       case 12 → 0x82306258
    //       case 13 → 0x823061E4
    //       case 14 → 0x82306244
    let mut pc: u32 = 0x82306140;
    'dispatch: loop {
        match pc {
            0x82306140 => {
    //   block [0x82306140..0x823061B4)
	// 82306140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82306144: 48EA2021  bl 0x831a8164
	ctx.lr = 0x82306148;
	sub_831A8130(ctx, base);
	// 82306148: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8230614C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82306150: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 82306154: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82306158: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8230615C: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82306160: 807F01FC  lwz r3, 0x1fc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(508 as u32) ) } as u64;
	// 82306164: 48154D45  bl 0x8245aea8
	ctx.lr = 0x82306168;
	sub_8245AEA8(ctx, base);
	// 82306168: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8230616C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82306170: 40820008  bne 0x82306178
	if !ctx.cr[0].eq {
	pc = 0x82306178; continue 'dispatch;
	}
	// 82306174: 7F9DE378  mr r29, r28
	ctx.r[29].u64 = ctx.r[28].u64;
	// 82306178: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8230617C: 48AECF75  bl 0x82df30f0
	ctx.lr = 0x82306180;
	sub_82DF30F0(ctx, base);
	// 82306180: 397DFFFF  addi r11, r29, -1
	ctx.r[11].s64 = ctx.r[29].s64 + -1;
	// 82306184: 2B0B000E  cmplwi cr6, r11, 0xe
	ctx.cr[6].compare_u32(ctx.r[11].u32, 14 as u32, &mut ctx.xer);
	// 82306188: 419900D0  bgt cr6, 0x82306258
	if ctx.cr[6].gt {
	pc = 0x82306258; continue 'dispatch;
	}
	// 8230618C: 3D808201  lis r12, -0x7dff
	ctx.r[12].s64 = -2113863680;
	// 82306190: 398CCDE8  addi r12, r12, -0x3218
	ctx.r[12].s64 = ctx.r[12].s64 + -12824;
	// 82306194: 7C0C58AE  lbzx r0, r12, r11
	ctx.r[0].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82306198: 3D808230  lis r12, -0x7dd0
	ctx.r[12].s64 = -2110783488;
	// 8230619C: 398C61B4  addi r12, r12, 0x61b4
	ctx.r[12].s64 = ctx.r[12].s64 + 25012;
	// 823061A0: 7D8C0214  add r12, r12, r0
	ctx.r[12].u64 = ctx.r[12].u64 + ctx.r[0].u64;
	// 823061A4: 7D8903A6  mtctr r12
	ctx.ctr.u64 = ctx.r[12].u64;
	// 823061A8: 60000000  nop
	// 823061AC: 60000000  nop
	// 823061B0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            0x823061B4 => {
    //   block [0x823061B4..0x823061C0)
	// 823061B4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823061B8: 388BCEB4  addi r4, r11, -0x314c
	ctx.r[4].s64 = ctx.r[11].s64 + -12620;
	// 823061BC: 480000A4  b 0x82306260
	pc = 0x82306260; continue 'dispatch;
            }
            0x823061C0 => {
    //   block [0x823061C0..0x823061CC)
	// 823061C0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823061C4: 388BCEB0  addi r4, r11, -0x3150
	ctx.r[4].s64 = ctx.r[11].s64 + -12624;
	// 823061C8: 48000098  b 0x82306260
	pc = 0x82306260; continue 'dispatch;
            }
            0x823061CC => {
    //   block [0x823061CC..0x823061D8)
	// 823061CC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823061D0: 388BCEA4  addi r4, r11, -0x315c
	ctx.r[4].s64 = ctx.r[11].s64 + -12636;
	// 823061D4: 4800008C  b 0x82306260
	pc = 0x82306260; continue 'dispatch;
            }
            0x823061D8 => {
    //   block [0x823061D8..0x823061E4)
	// 823061D8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823061DC: 388BCE94  addi r4, r11, -0x316c
	ctx.r[4].s64 = ctx.r[11].s64 + -12652;
	// 823061E0: 48000080  b 0x82306260
	pc = 0x82306260; continue 'dispatch;
            }
            0x823061E4 => {
    //   block [0x823061E4..0x823061F0)
	// 823061E4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823061E8: 388BCE84  addi r4, r11, -0x317c
	ctx.r[4].s64 = ctx.r[11].s64 + -12668;
	// 823061EC: 48000074  b 0x82306260
	pc = 0x82306260; continue 'dispatch;
            }
            0x823061F0 => {
    //   block [0x823061F0..0x823061FC)
	// 823061F0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823061F4: 388BCE7C  addi r4, r11, -0x3184
	ctx.r[4].s64 = ctx.r[11].s64 + -12676;
	// 823061F8: 48000068  b 0x82306260
	pc = 0x82306260; continue 'dispatch;
            }
            0x823061FC => {
    //   block [0x823061FC..0x82306208)
	// 823061FC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82306200: 388BCE40  addi r4, r11, -0x31c0
	ctx.r[4].s64 = ctx.r[11].s64 + -12736;
	// 82306204: 4800005C  b 0x82306260
	pc = 0x82306260; continue 'dispatch;
            }
            0x82306208 => {
    //   block [0x82306208..0x82306214)
	// 82306208: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8230620C: 388BCE38  addi r4, r11, -0x31c8
	ctx.r[4].s64 = ctx.r[11].s64 + -12744;
	// 82306210: 48000050  b 0x82306260
	pc = 0x82306260; continue 'dispatch;
            }
            0x82306214 => {
    //   block [0x82306214..0x82306220)
	// 82306214: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82306218: 388BCE30  addi r4, r11, -0x31d0
	ctx.r[4].s64 = ctx.r[11].s64 + -12752;
	// 8230621C: 48000044  b 0x82306260
	pc = 0x82306260; continue 'dispatch;
            }
            0x82306220 => {
    //   block [0x82306220..0x8230622C)
	// 82306220: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82306224: 388BCE74  addi r4, r11, -0x318c
	ctx.r[4].s64 = ctx.r[11].s64 + -12684;
	// 82306228: 48000038  b 0x82306260
	pc = 0x82306260; continue 'dispatch;
            }
            0x8230622C => {
    //   block [0x8230622C..0x82306238)
	// 8230622C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82306230: 388BCE20  addi r4, r11, -0x31e0
	ctx.r[4].s64 = ctx.r[11].s64 + -12768;
	// 82306234: 4800002C  b 0x82306260
	pc = 0x82306260; continue 'dispatch;
            }
            0x82306238 => {
    //   block [0x82306238..0x82306244)
	// 82306238: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8230623C: 388BCE6C  addi r4, r11, -0x3194
	ctx.r[4].s64 = ctx.r[11].s64 + -12692;
	// 82306240: 48000020  b 0x82306260
	pc = 0x82306260; continue 'dispatch;
            }
            0x82306244 => {
    //   block [0x82306244..0x82306258)
	// 82306244: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82306248: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8230624C: 388BCE60  addi r4, r11, -0x31a0
	ctx.r[4].s64 = ctx.r[11].s64 + -12704;
	// 82306250: 48AED7B9  bl 0x82df3a08
	ctx.lr = 0x82306254;
	sub_82DF3A08(ctx, base);
	// 82306254: 48000098  b 0x823062ec
	pc = 0x823062EC; continue 'dispatch;
            }
            0x82306258 => {
    //   block [0x82306258..0x82306300)
	// 82306258: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8230625C: 388BCE58  addi r4, r11, -0x31a8
	ctx.r[4].s64 = ctx.r[11].s64 + -12712;
	// 82306260: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82306264: 48AED615  bl 0x82df3878
	ctx.lr = 0x82306268;
	sub_82DF3878(ctx, base);
	// 82306268: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8230626C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82306270: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82306274: 3BAACE54  addi r29, r10, -0x31ac
	ctx.r[29].s64 = ctx.r[10].s64 + -12716;
	// 82306278: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8230627C: 816B0050  lwz r11, 0x50(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 82306280: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82306284: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82306288: 4E800421  bctrl
	ctx.lr = 0x8230628C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8230628C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82306290: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 82306294: 48AED775  bl 0x82df3a08
	ctx.lr = 0x82306298;
	sub_82DF3A08(ctx, base);
	// 82306298: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8230629C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 823062A0: 388BCE50  addi r4, r11, -0x31b0
	ctx.r[4].s64 = ctx.r[11].s64 + -12720;
	// 823062A4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 823062A8: 48AED9F9  bl 0x82df3ca0
	ctx.lr = 0x823062AC;
	sub_82DF3CA0(ctx, base);
	// 823062AC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 823062B0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 823062B4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 823062B8: 48AEDA41  bl 0x82df3cf8
	ctx.lr = 0x823062BC;
	sub_82DF3CF8(ctx, base);
	// 823062BC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 823062C0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823062C4: 48AED5ED  bl 0x82df38b0
	ctx.lr = 0x823062C8;
	sub_82DF38B0(ctx, base);
	// 823062C8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 823062CC: 48AED15D  bl 0x82df3428
	ctx.lr = 0x823062D0;
	sub_82DF3428(ctx, base);
	// 823062D0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 823062D4: 48AED155  bl 0x82df3428
	ctx.lr = 0x823062D8;
	sub_82DF3428(ctx, base);
	// 823062D8: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 823062DC: 48AED14D  bl 0x82df3428
	ctx.lr = 0x823062E0;
	sub_82DF3428(ctx, base);
	// 823062E0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 823062E4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823062E8: 48AED919  bl 0x82df3c00
	ctx.lr = 0x823062EC;
	sub_82DF3C00(ctx, base);
	// 823062EC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823062F0: 48AED139  bl 0x82df3428
	ctx.lr = 0x823062F4;
	sub_82DF3428(ctx, base);
	// 823062F4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823062F8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 823062FC: 48EA1EB8  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82306300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82306300 size=60
    let mut pc: u32 = 0x82306300;
    'dispatch: loop {
        match pc {
            0x82306300 => {
    //   block [0x82306300..0x8230633C)
	// 82306300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82306304: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82306308: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8230630C: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82306310: 83E30020  lwz r31, 0x20(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82306314: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82306318: 48B75CC1  bl 0x82e7bfd8
	ctx.lr = 0x8230631C;
	sub_82E7BFD8(ctx, base);
	// 8230631C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82306320: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82306324: 48B0CF3D  bl 0x82e13260
	ctx.lr = 0x82306328;
	sub_82E13260(ctx, base);
	// 82306328: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8230632C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82306330: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82306334: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82306338: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82306340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82306340 size=136
    let mut pc: u32 = 0x82306340;
    'dispatch: loop {
        match pc {
            0x82306340 => {
    //   block [0x82306340..0x823063C8)
	// 82306340: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82306344: 48EA1E25  bl 0x831a8168
	ctx.lr = 0x82306348;
	sub_831A8130(ctx, base);
	// 82306348: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8230634C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82306350: 80840100  lwz r4, 0x100(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(256 as u32) ) } as u64;
	// 82306354: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82306358: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8230635C: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82306360: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 82306364: 48209165  bl 0x8250f4c8
	ctx.lr = 0x82306368;
	sub_8250F4C8(ctx, base);
	// 82306368: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8230636C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82306370: 388BFFFC  addi r4, r11, -4
	ctx.r[4].s64 = ctx.r[11].s64 + -4;
	// 82306374: 409A0008  bne cr6, 0x8230637c
	if !ctx.cr[6].eq {
	pc = 0x8230637C; continue 'dispatch;
	}
	// 82306378: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8230637C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82306380: 482045D1  bl 0x8250a950
	ctx.lr = 0x82306384;
	sub_8250A950(ctx, base);
	// 82306384: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82306388: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8230638C: 388BFF40  addi r4, r11, -0xc0
	ctx.r[4].s64 = ctx.r[11].s64 + -192;
	// 82306390: 409A0008  bne cr6, 0x82306398
	if !ctx.cr[6].eq {
	pc = 0x82306398; continue 'dispatch;
	}
	// 82306394: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82306398: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 8230639C: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 823063A0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 823063A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823063A8: 484CEA71  bl 0x827d4e18
	ctx.lr = 0x823063AC;
	sub_827D4E18(ctx, base);
	// 823063AC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823063B0: 48AEB8E1  bl 0x82df1c90
	ctx.lr = 0x823063B4;
	sub_82DF1C90(ctx, base);
	// 823063B4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 823063B8: 48AEB8D9  bl 0x82df1c90
	ctx.lr = 0x823063BC;
	sub_82DF1C90(ctx, base);
	// 823063BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823063C0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 823063C4: 48EA1DF4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823063C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x823063C8 size=120
    let mut pc: u32 = 0x823063C8;
    'dispatch: loop {
        match pc {
            0x823063C8 => {
    //   block [0x823063C8..0x82306440)
	// 823063C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823063CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823063D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823063D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823063D8: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 823063DC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823063E0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 823063E4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823063E8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823063EC: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 823063F0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 823063F4: C3EB08A4  lfs f31, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 823063F8: D3E10050  stfs f31, 0x50(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 823063FC: 48AECCF5  bl 0x82df30f0
	ctx.lr = 0x82306400;
	sub_82DF30F0(ctx, base);
	// 82306400: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 82306404: D3E10050  stfs f31, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82306408: 807E0100  lwz r3, 0x100(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(256 as u32) ) } as u64;
	// 8230640C: 3BE10050  addi r31, r1, 0x50
	ctx.r[31].s64 = ctx.r[1].s64 + 80;
	// 82306410: 486029D9  bl 0x82908de8
	ctx.lr = 0x82306414;
	sub_82908DE8(ctx, base);
	// 82306414: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82306418: 4BFF78D1  bl 0x822fdce8
	ctx.lr = 0x8230641C;
	sub_822FDCE8(ctx, base);
	// 8230641C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82306420: 48AED009  bl 0x82df3428
	ctx.lr = 0x82306424;
	sub_82DF3428(ctx, base);
	// 82306424: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82306428: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8230642C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82306430: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82306434: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82306438: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8230643C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82306440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82306440 size=144
    let mut pc: u32 = 0x82306440;
    'dispatch: loop {
        match pc {
            0x82306440 => {
    //   block [0x82306440..0x823064D0)
	// 82306440: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82306444: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82306448: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8230644C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82306450: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82306454: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82306458: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8230645C: 808B0100  lwz r4, 0x100(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(256 as u32) ) } as u64;
	// 82306460: 48209069  bl 0x8250f4c8
	ctx.lr = 0x82306464;
	sub_8250F4C8(ctx, base);
	// 82306464: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82306468: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8230646C: 388BFFFC  addi r4, r11, -4
	ctx.r[4].s64 = ctx.r[11].s64 + -4;
	// 82306470: 409A0008  bne cr6, 0x82306478
	if !ctx.cr[6].eq {
	pc = 0x82306478; continue 'dispatch;
	}
	// 82306474: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82306478: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8230647C: 482044D5  bl 0x8250a950
	ctx.lr = 0x82306480;
	sub_8250A950(ctx, base);
	// 82306480: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82306484: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82306488: 386BFF40  addi r3, r11, -0xc0
	ctx.r[3].s64 = ctx.r[11].s64 + -192;
	// 8230648C: 409A0008  bne cr6, 0x82306494
	if !ctx.cr[6].eq {
	pc = 0x82306494; continue 'dispatch;
	}
	// 82306490: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82306494: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82306498: 484CD4D9  bl 0x827d3970
	ctx.lr = 0x8230649C;
	sub_827D3970(ctx, base);
	// 8230649C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823064A0: 48AEB7F1  bl 0x82df1c90
	ctx.lr = 0x823064A4;
	sub_82DF1C90(ctx, base);
	// 823064A4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 823064A8: 48AEB7E9  bl 0x82df1c90
	ctx.lr = 0x823064AC;
	sub_82DF1C90(ctx, base);
	// 823064AC: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 823064B0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823064B4: 419A0008  beq cr6, 0x823064bc
	if ctx.cr[6].eq {
	pc = 0x823064BC; continue 'dispatch;
	}
	// 823064B8: 4BFBA3D9  bl 0x822c0890
	ctx.lr = 0x823064BC;
	sub_822C0890(ctx, base);
	// 823064BC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823064C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823064C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823064C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823064CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823064D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823064D0 size=144
    let mut pc: u32 = 0x823064D0;
    'dispatch: loop {
        match pc {
            0x823064D0 => {
    //   block [0x823064D0..0x82306560)
	// 823064D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823064D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823064D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823064DC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823064E0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823064E4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 823064E8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823064EC: 808B0100  lwz r4, 0x100(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(256 as u32) ) } as u64;
	// 823064F0: 48208FD9  bl 0x8250f4c8
	ctx.lr = 0x823064F4;
	sub_8250F4C8(ctx, base);
	// 823064F4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 823064F8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823064FC: 388BFFFC  addi r4, r11, -4
	ctx.r[4].s64 = ctx.r[11].s64 + -4;
	// 82306500: 409A0008  bne cr6, 0x82306508
	if !ctx.cr[6].eq {
	pc = 0x82306508; continue 'dispatch;
	}
	// 82306504: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82306508: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8230650C: 48204445  bl 0x8250a950
	ctx.lr = 0x82306510;
	sub_8250A950(ctx, base);
	// 82306510: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82306514: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82306518: 386BFF40  addi r3, r11, -0xc0
	ctx.r[3].s64 = ctx.r[11].s64 + -192;
	// 8230651C: 409A0008  bne cr6, 0x82306524
	if !ctx.cr[6].eq {
	pc = 0x82306524; continue 'dispatch;
	}
	// 82306520: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82306524: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82306528: 484CD4D9  bl 0x827d3a00
	ctx.lr = 0x8230652C;
	sub_827D3A00(ctx, base);
	// 8230652C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82306530: 48AEB761  bl 0x82df1c90
	ctx.lr = 0x82306534;
	sub_82DF1C90(ctx, base);
	// 82306534: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82306538: 48AEB759  bl 0x82df1c90
	ctx.lr = 0x8230653C;
	sub_82DF1C90(ctx, base);
	// 8230653C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82306540: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82306544: 419A0008  beq cr6, 0x8230654c
	if ctx.cr[6].eq {
	pc = 0x8230654C; continue 'dispatch;
	}
	// 82306548: 4BFBA349  bl 0x822c0890
	ctx.lr = 0x8230654C;
	sub_822C0890(ctx, base);
	// 8230654C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82306550: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82306554: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82306558: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8230655C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82306560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82306560 size=232
    //   switch @ 0x823065C0: r11 with 15 label(s)
    //       case  0 → 0x82306628
    //       case  1 → 0x823065D0
    //       case  2 → 0x82306628
    //       case  3 → 0x823065DC
    //       case  4 → 0x823065E8
    //       case  5 → 0x823065F4
    //       case  6 → 0x82306600
    //       case  7 → 0x8230660C
    //       case  8 → 0x823065DC
    //       case  9 → 0x82306600
    //       case 10 → 0x823065D0
    //       case 11 → 0x82306628
    //       case 12 → 0x82306628
    //       case 13 → 0x823065D0
    //       case 14 → 0x82306618
    let mut pc: u32 = 0x82306560;
    'dispatch: loop {
        match pc {
            0x82306560 => {
    //   block [0x82306560..0x823065D0)
	// 82306560: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82306564: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82306568: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8230656C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82306570: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82306574: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82306578: 806301FC  lwz r3, 0x1fc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(508 as u32) ) } as u64;
	// 8230657C: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 82306580: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82306584: 48154925  bl 0x8245aea8
	ctx.lr = 0x82306588;
	sub_8245AEA8(ctx, base);
	// 82306588: 7C6B1B79  or. r11, r3, r3
	ctx.r[11].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8230658C: 40820008  bne 0x82306594
	if !ctx.cr[0].eq {
	pc = 0x82306594; continue 'dispatch;
	}
	// 82306590: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82306594: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82306598: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8230659C: 995E0000  stb r10, 0(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 823065A0: 2B0B000E  cmplwi cr6, r11, 0xe
	ctx.cr[6].compare_u32(ctx.r[11].u32, 14 as u32, &mut ctx.xer);
	// 823065A4: 41990084  bgt cr6, 0x82306628
	if ctx.cr[6].gt {
	pc = 0x82306628; continue 'dispatch;
	}
	// 823065A8: 3D808201  lis r12, -0x7dff
	ctx.r[12].s64 = -2113863680;
	// 823065AC: 398CCDF8  addi r12, r12, -0x3208
	ctx.r[12].s64 = ctx.r[12].s64 + -12808;
	// 823065B0: 7C0C58AE  lbzx r0, r12, r11
	ctx.r[0].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 823065B4: 3D808230  lis r12, -0x7dd0
	ctx.r[12].s64 = -2110783488;
	// 823065B8: 398C65D0  addi r12, r12, 0x65d0
	ctx.r[12].s64 = ctx.r[12].s64 + 26064;
	// 823065BC: 7D8C0214  add r12, r12, r0
	ctx.r[12].u64 = ctx.r[12].u64 + ctx.r[0].u64;
	// 823065C0: 7D8903A6  mtctr r12
	ctx.ctr.u64 = ctx.r[12].u64;
	// 823065C4: 60000000  nop
	// 823065C8: 60000000  nop
	// 823065CC: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            0x823065D0 => {
    //   block [0x823065D0..0x823065DC)
	// 823065D0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823065D4: 386BCED4  addi r3, r11, -0x312c
	ctx.r[3].s64 = ctx.r[11].s64 + -12588;
	// 823065D8: 48000058  b 0x82306630
	pc = 0x82306630; continue 'dispatch;
            }
            0x823065DC => {
    //   block [0x823065DC..0x823065E8)
	// 823065DC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823065E0: 386BCECC  addi r3, r11, -0x3134
	ctx.r[3].s64 = ctx.r[11].s64 + -12596;
	// 823065E4: 4800004C  b 0x82306630
	pc = 0x82306630; continue 'dispatch;
            }
            0x823065E8 => {
    //   block [0x823065E8..0x823065F4)
	// 823065E8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823065EC: 386BCE30  addi r3, r11, -0x31d0
	ctx.r[3].s64 = ctx.r[11].s64 + -12752;
	// 823065F0: 48000040  b 0x82306630
	pc = 0x82306630; continue 'dispatch;
            }
            0x823065F4 => {
    //   block [0x823065F4..0x82306600)
	// 823065F4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823065F8: 386BCE74  addi r3, r11, -0x318c
	ctx.r[3].s64 = ctx.r[11].s64 + -12684;
	// 823065FC: 48000034  b 0x82306630
	pc = 0x82306630; continue 'dispatch;
            }
            0x82306600 => {
    //   block [0x82306600..0x8230660C)
	// 82306600: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82306604: 386BCE20  addi r3, r11, -0x31e0
	ctx.r[3].s64 = ctx.r[11].s64 + -12768;
	// 82306608: 48000028  b 0x82306630
	pc = 0x82306630; continue 'dispatch;
            }
            0x8230660C => {
    //   block [0x8230660C..0x82306618)
	// 8230660C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82306610: 386BCEC4  addi r3, r11, -0x313c
	ctx.r[3].s64 = ctx.r[11].s64 + -12604;
	// 82306614: 4800001C  b 0x82306630
	pc = 0x82306630; continue 'dispatch;
            }
            0x82306618 => {
    //   block [0x82306618..0x82306628)
	// 82306618: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8230661C: 995E0000  stb r10, 0(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82306620: 386BCE10  addi r3, r11, -0x31f0
	ctx.r[3].s64 = ctx.r[11].s64 + -12784;
	// 82306624: 4800000C  b 0x82306630
	pc = 0x82306630; continue 'dispatch;
            }
            0x82306628 => {
    //   block [0x82306628..0x82306648)
	// 82306628: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8230662C: 386BCEBC  addi r3, r11, -0x3144
	ctx.r[3].s64 = ctx.r[11].s64 + -12612;
	// 82306630: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82306634: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82306638: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8230663C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82306640: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82306644: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82306648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82306648 size=104
    let mut pc: u32 = 0x82306648;
    'dispatch: loop {
        match pc {
            0x82306648 => {
    //   block [0x82306648..0x823066B0)
	// 82306648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8230664C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82306650: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82306654: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82306658: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8230665C: 80830100  lwz r4, 0x100(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(256 as u32) ) } as u64;
	// 82306660: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82306664: 48208CCD  bl 0x8250f330
	ctx.lr = 0x82306668;
	sub_8250F330(ctx, base);
	// 82306668: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8230666C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82306670: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82306674: 481E4C4D  bl 0x824eb2c0
	ctx.lr = 0x82306678;
	sub_824EB2C0(ctx, base);
	// 82306678: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8230667C: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82306680: 48210759  bl 0x82516dd8
	ctx.lr = 0x82306684;
	sub_82516DD8(ctx, base);
	// 82306684: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82306688: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8230668C: 419A0008  beq cr6, 0x82306694
	if ctx.cr[6].eq {
	pc = 0x82306694; continue 'dispatch;
	}
	// 82306690: 4BFBA201  bl 0x822c0890
	ctx.lr = 0x82306694;
	sub_822C0890(ctx, base);
	// 82306694: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82306698: 48AEB5F9  bl 0x82df1c90
	ctx.lr = 0x8230669C;
	sub_82DF1C90(ctx, base);
	// 8230669C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823066A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823066A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823066A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823066AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823066B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823066B0 size=92
    let mut pc: u32 = 0x823066B0;
    'dispatch: loop {
        match pc {
            0x823066B0 => {
    //   block [0x823066B0..0x8230670C)
	// 823066B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823066B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823066B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823066BC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823066C0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823066C4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823066C8: 808B0100  lwz r4, 0x100(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(256 as u32) ) } as u64;
	// 823066CC: 48208DFD  bl 0x8250f4c8
	ctx.lr = 0x823066D0;
	sub_8250F4C8(ctx, base);
	// 823066D0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 823066D4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823066D8: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 823066DC: 409A0008  bne cr6, 0x823066e4
	if !ctx.cr[6].eq {
	pc = 0x823066E4; continue 'dispatch;
	}
	// 823066E0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 823066E4: 48201FF5  bl 0x825086d8
	ctx.lr = 0x823066E8;
	sub_825086D8(ctx, base);
	// 823066E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823066EC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823066F0: 48AEB5A1  bl 0x82df1c90
	ctx.lr = 0x823066F4;
	sub_82DF1C90(ctx, base);
	// 823066F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823066F8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823066FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82306700: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82306704: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82306708: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82306710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82306710 size=100
    let mut pc: u32 = 0x82306710;
    'dispatch: loop {
        match pc {
            0x82306710 => {
    //   block [0x82306710..0x82306774)
	// 82306710: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82306714: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82306718: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8230671C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82306720: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82306724: 80840100  lwz r4, 0x100(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(256 as u32) ) } as u64;
	// 82306728: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8230672C: 48208DED  bl 0x8250f518
	ctx.lr = 0x82306730;
	sub_8250F518(ctx, base);
	// 82306730: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82306734: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82306738: 386BFF6C  addi r3, r11, -0x94
	ctx.r[3].s64 = ctx.r[11].s64 + -148;
	// 8230673C: 409A0008  bne cr6, 0x82306744
	if !ctx.cr[6].eq {
	pc = 0x82306744; continue 'dispatch;
	}
	// 82306740: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82306744: 48221E3D  bl 0x82528580
	ctx.lr = 0x82306748;
	sub_82528580(ctx, base);
	// 82306748: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8230674C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82306750: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82306778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82306778 size=204
    let mut pc: u32 = 0x82306778;
    'dispatch: loop {
        match pc {
            0x82306778 => {
    //   block [0x82306778..0x82306844)
	// 82306778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8230677C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82306780: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82306784: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82306788: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8230678C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82306790: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82306794: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82306798: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 8230679C: 809F0100  lwz r4, 0x100(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(256 as u32) ) } as u64;
	// 823067A0: 48208D29  bl 0x8250f4c8
	ctx.lr = 0x823067A4;
	sub_8250F4C8(ctx, base);
	// 823067A4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 823067A8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823067AC: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 823067B0: 409A0008  bne cr6, 0x823067b8
	if !ctx.cr[6].eq {
	pc = 0x823067B8; continue 'dispatch;
	}
	// 823067B4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 823067B8: 48201F21  bl 0x825086d8
	ctx.lr = 0x823067BC;
	sub_825086D8(ctx, base);
	// 823067BC: 484D37F5  bl 0x827d9fb0
	ctx.lr = 0x823067C0;
	sub_827D9FB0(ctx, base);
	// 823067C0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823067C4: 41820038  beq 0x823067fc
	if ctx.cr[0].eq {
	pc = 0x823067FC; continue 'dispatch;
	}
	// 823067C8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823067CC: 809F0100  lwz r4, 0x100(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(256 as u32) ) } as u64;
	// 823067D0: 3BC00003  li r30, 3
	ctx.r[30].s64 = 3;
	// 823067D4: 48208CF5  bl 0x8250f4c8
	ctx.lr = 0x823067D8;
	sub_8250F4C8(ctx, base);
	// 823067D8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 823067DC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823067E0: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 823067E4: 409A0008  bne cr6, 0x823067ec
	if !ctx.cr[6].eq {
	pc = 0x823067EC; continue 'dispatch;
	}
	// 823067E8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 823067EC: 48201F2D  bl 0x82508718
	ctx.lr = 0x823067F0;
	sub_82508718(ctx, base);
	// 823067F0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823067F4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823067F8: 40820008  bne 0x82306800
	if !ctx.cr[0].eq {
	pc = 0x82306800; continue 'dispatch;
	}
	// 823067FC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82306800: 57CA07BD  rlwinm. r10, r30, 0, 0x1e, 0x1e
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82306804: 557F063E  clrlwi r31, r11, 0x18
	ctx.r[31].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82306808: 41820010  beq 0x82306818
	if ctx.cr[0].eq {
	pc = 0x82306818; continue 'dispatch;
	}
	// 8230680C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82306810: 57DE07FA  rlwinm r30, r30, 0, 0x1f, 0x1d
	ctx.r[30].u64 = ctx.r[30].u32 as u64 & 0xFFFFFFFFu64;
	// 82306814: 48AEB47D  bl 0x82df1c90
	ctx.lr = 0x82306818;
	sub_82DF1C90(ctx, base);
	// 82306818: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8230681C: 4182000C  beq 0x82306828
	if ctx.cr[0].eq {
	pc = 0x82306828; continue 'dispatch;
	}
	// 82306820: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82306824: 48AEB46D  bl 0x82df1c90
	ctx.lr = 0x82306828;
	sub_82DF1C90(ctx, base);
	// 82306828: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8230682C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82306830: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82306834: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82306838: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8230683C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82306840: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82306848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82306848 size=84
    let mut pc: u32 = 0x82306848;
    'dispatch: loop {
        match pc {
            0x82306848 => {
    //   block [0x82306848..0x8230689C)
	// 82306848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8230684C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82306850: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82306854: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82306858: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8230685C: 808B0100  lwz r4, 0x100(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(256 as u32) ) } as u64;
	// 82306860: 48208C69  bl 0x8250f4c8
	ctx.lr = 0x82306864;
	sub_8250F4C8(ctx, base);
	// 82306864: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82306868: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8230686C: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 82306870: 409A0008  bne cr6, 0x82306878
	if !ctx.cr[6].eq {
	pc = 0x82306878; continue 'dispatch;
	}
	// 82306874: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82306878: 48201E61  bl 0x825086d8
	ctx.lr = 0x8230687C;
	sub_825086D8(ctx, base);
	// 8230687C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82306880: 48155AB1  bl 0x8245c330
	ctx.lr = 0x82306884;
	sub_8245C330(ctx, base);
	// 82306884: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82306888: 48AEB409  bl 0x82df1c90
	ctx.lr = 0x8230688C;
	sub_82DF1C90(ctx, base);
	// 8230688C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82306890: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82306894: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82306898: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823068A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823068A0 size=40
    let mut pc: u32 = 0x823068A0;
    'dispatch: loop {
        match pc {
            0x823068A0 => {
    //   block [0x823068A0..0x823068C8)
	// 823068A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823068A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823068A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823068AC: 4BFFF6FD  bl 0x82305fa8
	ctx.lr = 0x823068B0;
	sub_82305FA8(ctx, base);
	// 823068B0: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 823068B4: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 823068B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 823068BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823068C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823068C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823068C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823068C8 size=196
    let mut pc: u32 = 0x823068C8;
    'dispatch: loop {
        match pc {
            0x823068C8 => {
    //   block [0x823068C8..0x8230698C)
	// 823068C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823068CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823068D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823068D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823068D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823068DC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823068E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823068E4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823068E8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823068EC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823068F0: 4BFBA049  bl 0x822c0938
	ctx.lr = 0x823068F4;
	sub_822C0938(ctx, base);
	// 823068F4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823068F8: 41820028  beq 0x82306920
	if ctx.cr[0].eq {
	pc = 0x82306920; continue 'dispatch;
	}
	// 823068FC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82306900: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82306904: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82306908: 392BCC4C  addi r9, r11, -0x33b4
	ctx.r[9].s64 = ctx.r[11].s64 + -13236;
	// 8230690C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82306910: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82306914: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82306918: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8230691C: 48000008  b 0x82306924
	pc = 0x82306924; continue 'dispatch;
	// 82306920: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82306924: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82306928: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8230692C: 409A0044  bne cr6, 0x82306970
	if !ctx.cr[6].eq {
	pc = 0x82306970; continue 'dispatch;
	}
	// 82306930: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82306934: 419A001C  beq cr6, 0x82306950
	if ctx.cr[6].eq {
	pc = 0x82306950; continue 'dispatch;
	}
	// 82306938: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8230693C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82306940: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82306944: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82306948: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8230694C: 4E800421  bctrl
	ctx.lr = 0x82306950;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82306950: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 82306954: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82306958: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8230695C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82306960: 816B5C38  lwz r11, 0x5c38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(23608 as u32) ) } as u64;
	// 82306964: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82306968: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8230696C: 4BFB9695  bl 0x822c0000
	ctx.lr = 0x82306970;
	sub_822C0000(ctx, base);
	// 82306970: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82306974: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82306978: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8230697C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82306980: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82306984: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82306988: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82306990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82306990 size=196
    let mut pc: u32 = 0x82306990;
    'dispatch: loop {
        match pc {
            0x82306990 => {
    //   block [0x82306990..0x82306A54)
	// 82306990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82306994: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82306998: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8230699C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823069A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823069A4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823069A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823069AC: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823069B0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823069B4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823069B8: 4BFB9F81  bl 0x822c0938
	ctx.lr = 0x823069BC;
	sub_822C0938(ctx, base);
	// 823069BC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823069C0: 41820028  beq 0x823069e8
	if ctx.cr[0].eq {
	pc = 0x823069E8; continue 'dispatch;
	}
	// 823069C4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823069C8: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823069CC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823069D0: 392BCC60  addi r9, r11, -0x33a0
	ctx.r[9].s64 = ctx.r[11].s64 + -13216;
	// 823069D4: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823069D8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823069DC: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823069E0: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823069E4: 48000008  b 0x823069ec
	pc = 0x823069EC; continue 'dispatch;
	// 823069E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823069EC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823069F0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823069F4: 409A0044  bne cr6, 0x82306a38
	if !ctx.cr[6].eq {
	pc = 0x82306A38; continue 'dispatch;
	}
	// 823069F8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823069FC: 419A001C  beq cr6, 0x82306a18
	if ctx.cr[6].eq {
	pc = 0x82306A18; continue 'dispatch;
	}
	// 82306A00: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82306A04: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82306A08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82306A0C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82306A10: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82306A14: 4E800421  bctrl
	ctx.lr = 0x82306A18;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82306A18: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 82306A1C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82306A20: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82306A24: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82306A28: 816B5C38  lwz r11, 0x5c38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(23608 as u32) ) } as u64;
	// 82306A2C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82306A30: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82306A34: 4BFB95CD  bl 0x822c0000
	ctx.lr = 0x82306A38;
	sub_822C0000(ctx, base);
	// 82306A38: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82306A3C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82306A40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82306A44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82306A48: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82306A4C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82306A50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82306A58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82306A58 size=196
    let mut pc: u32 = 0x82306A58;
    'dispatch: loop {
        match pc {
            0x82306A58 => {
    //   block [0x82306A58..0x82306B1C)
	// 82306A58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82306A5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82306A60: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82306A64: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82306A68: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82306A6C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82306A70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82306A74: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82306A78: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82306A7C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82306A80: 4BFB9EB9  bl 0x822c0938
	ctx.lr = 0x82306A84;
	sub_822C0938(ctx, base);
	// 82306A84: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82306A88: 41820028  beq 0x82306ab0
	if ctx.cr[0].eq {
	pc = 0x82306AB0; continue 'dispatch;
	}
	// 82306A8C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82306A90: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82306A94: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82306A98: 392BCC88  addi r9, r11, -0x3378
	ctx.r[9].s64 = ctx.r[11].s64 + -13176;
	// 82306A9C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82306AA0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82306AA4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82306AA8: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82306AAC: 48000008  b 0x82306ab4
	pc = 0x82306AB4; continue 'dispatch;
	// 82306AB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82306AB4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82306AB8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82306ABC: 409A0044  bne cr6, 0x82306b00
	if !ctx.cr[6].eq {
	pc = 0x82306B00; continue 'dispatch;
	}
	// 82306AC0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82306AC4: 419A001C  beq cr6, 0x82306ae0
	if ctx.cr[6].eq {
	pc = 0x82306AE0; continue 'dispatch;
	}
	// 82306AC8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82306ACC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82306AD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82306AD4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82306AD8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82306ADC: 4E800421  bctrl
	ctx.lr = 0x82306AE0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82306AE0: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 82306AE4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82306AE8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82306AEC: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82306AF0: 816B5C38  lwz r11, 0x5c38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(23608 as u32) ) } as u64;
	// 82306AF4: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82306AF8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82306AFC: 4BFB9505  bl 0x822c0000
	ctx.lr = 0x82306B00;
	sub_822C0000(ctx, base);
	// 82306B00: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82306B04: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82306B08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82306B0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82306B10: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82306B14: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82306B18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82306B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82306B20 size=196
    let mut pc: u32 = 0x82306B20;
    'dispatch: loop {
        match pc {
            0x82306B20 => {
    //   block [0x82306B20..0x82306BE4)
	// 82306B20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82306B24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82306B28: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82306B2C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82306B30: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82306B34: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82306B38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82306B3C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82306B40: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82306B44: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82306B48: 4BFB9DF1  bl 0x822c0938
	ctx.lr = 0x82306B4C;
	sub_822C0938(ctx, base);
	// 82306B4C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82306B50: 41820028  beq 0x82306b78
	if ctx.cr[0].eq {
	pc = 0x82306B78; continue 'dispatch;
	}
	// 82306B54: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82306B58: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82306B5C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82306B60: 392BCC9C  addi r9, r11, -0x3364
	ctx.r[9].s64 = ctx.r[11].s64 + -13156;
	// 82306B64: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82306B68: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82306B6C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82306B70: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82306B74: 48000008  b 0x82306b7c
	pc = 0x82306B7C; continue 'dispatch;
	// 82306B78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82306B7C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82306B80: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82306B84: 409A0044  bne cr6, 0x82306bc8
	if !ctx.cr[6].eq {
	pc = 0x82306BC8; continue 'dispatch;
	}
	// 82306B88: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82306B8C: 419A001C  beq cr6, 0x82306ba8
	if ctx.cr[6].eq {
	pc = 0x82306BA8; continue 'dispatch;
	}
	// 82306B90: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82306B94: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82306B98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82306B9C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82306BA0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82306BA4: 4E800421  bctrl
	ctx.lr = 0x82306BA8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82306BA8: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 82306BAC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82306BB0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82306BB4: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82306BB8: 816B5C38  lwz r11, 0x5c38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(23608 as u32) ) } as u64;
	// 82306BBC: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82306BC0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82306BC4: 4BFB943D  bl 0x822c0000
	ctx.lr = 0x82306BC8;
	sub_822C0000(ctx, base);
	// 82306BC8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82306BCC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82306BD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82306BD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82306BD8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82306BDC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82306BE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82306BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82306BE8 size=172
    let mut pc: u32 = 0x82306BE8;
    'dispatch: loop {
        match pc {
            0x82306BE8 => {
    //   block [0x82306BE8..0x82306C94)
	// 82306BE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82306BEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82306BF0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82306BF4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82306BF8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82306BFC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82306C00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82306C04: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82306C08: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82306C0C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82306C10: 4BFB9D29  bl 0x822c0938
	ctx.lr = 0x82306C14;
	sub_822C0938(ctx, base);
	// 82306C14: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82306C18: 41820028  beq 0x82306c40
	if ctx.cr[0].eq {
	pc = 0x82306C40; continue 'dispatch;
	}
	// 82306C1C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82306C20: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82306C24: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82306C28: 392BCCC4  addi r9, r11, -0x333c
	ctx.r[9].s64 = ctx.r[11].s64 + -13116;
	// 82306C2C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82306C30: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82306C34: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82306C38: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82306C3C: 48000008  b 0x82306c44
	pc = 0x82306C44; continue 'dispatch;
	// 82306C40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82306C44: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82306C48: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82306C4C: 409A002C  bne cr6, 0x82306c78
	if !ctx.cr[6].eq {
	pc = 0x82306C78; continue 'dispatch;
	}
	// 82306C50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82306C54: 4BFB9615  bl 0x822c0268
	ctx.lr = 0x82306C58;
	sub_822C0268(ctx, base);
	// 82306C58: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 82306C5C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82306C60: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82306C64: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82306C68: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82306C6C: 816B5C38  lwz r11, 0x5c38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(23608 as u32) ) } as u64;
	// 82306C70: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82306C74: 4BFB938D  bl 0x822c0000
	ctx.lr = 0x82306C78;
	sub_822C0000(ctx, base);
	// 82306C78: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82306C7C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82306C80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82306C84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82306C88: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82306C8C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82306C90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82306C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82306C98 size=196
    let mut pc: u32 = 0x82306C98;
    'dispatch: loop {
        match pc {
            0x82306C98 => {
    //   block [0x82306C98..0x82306D5C)
	// 82306C98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82306C9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82306CA0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82306CA4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82306CA8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82306CAC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82306CB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82306CB4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82306CB8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82306CBC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82306CC0: 4BFB9C79  bl 0x822c0938
	ctx.lr = 0x82306CC4;
	sub_822C0938(ctx, base);
	// 82306CC4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82306CC8: 41820028  beq 0x82306cf0
	if ctx.cr[0].eq {
	pc = 0x82306CF0; continue 'dispatch;
	}
	// 82306CCC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82306CD0: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82306CD4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82306CD8: 392BCCD8  addi r9, r11, -0x3328
	ctx.r[9].s64 = ctx.r[11].s64 + -13096;
	// 82306CDC: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82306CE0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82306CE4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82306CE8: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82306CEC: 48000008  b 0x82306cf4
	pc = 0x82306CF4; continue 'dispatch;
	// 82306CF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82306CF4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82306CF8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82306CFC: 409A0044  bne cr6, 0x82306d40
	if !ctx.cr[6].eq {
	pc = 0x82306D40; continue 'dispatch;
	}
	// 82306D00: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82306D04: 419A001C  beq cr6, 0x82306d20
	if ctx.cr[6].eq {
	pc = 0x82306D20; continue 'dispatch;
	}
	// 82306D08: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82306D0C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82306D10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82306D14: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82306D18: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82306D1C: 4E800421  bctrl
	ctx.lr = 0x82306D20;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82306D20: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 82306D24: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82306D28: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82306D2C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82306D30: 816B5C38  lwz r11, 0x5c38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(23608 as u32) ) } as u64;
	// 82306D34: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82306D38: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82306D3C: 4BFB92C5  bl 0x822c0000
	ctx.lr = 0x82306D40;
	sub_822C0000(ctx, base);
	// 82306D40: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82306D44: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82306D48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82306D4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82306D50: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82306D54: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82306D58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82306D60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82306D60 size=196
    let mut pc: u32 = 0x82306D60;
    'dispatch: loop {
        match pc {
            0x82306D60 => {
    //   block [0x82306D60..0x82306E24)
	// 82306D60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82306D64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82306D68: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82306D6C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82306D70: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82306D74: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82306D78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82306D7C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82306D80: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82306D84: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82306D88: 4BFB9BB1  bl 0x822c0938
	ctx.lr = 0x82306D8C;
	sub_822C0938(ctx, base);
	// 82306D8C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82306D90: 41820028  beq 0x82306db8
	if ctx.cr[0].eq {
	pc = 0x82306DB8; continue 'dispatch;
	}
	// 82306D94: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82306D98: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82306D9C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82306DA0: 392BCCEC  addi r9, r11, -0x3314
	ctx.r[9].s64 = ctx.r[11].s64 + -13076;
	// 82306DA4: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82306DA8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82306DAC: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82306DB0: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82306DB4: 48000008  b 0x82306dbc
	pc = 0x82306DBC; continue 'dispatch;
	// 82306DB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82306DBC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82306DC0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82306DC4: 409A0044  bne cr6, 0x82306e08
	if !ctx.cr[6].eq {
	pc = 0x82306E08; continue 'dispatch;
	}
	// 82306DC8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82306DCC: 419A001C  beq cr6, 0x82306de8
	if ctx.cr[6].eq {
	pc = 0x82306DE8; continue 'dispatch;
	}
	// 82306DD0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82306DD4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82306DD8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82306DDC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82306DE0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82306DE4: 4E800421  bctrl
	ctx.lr = 0x82306DE8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82306DE8: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 82306DEC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82306DF0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82306DF4: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82306DF8: 816B5C38  lwz r11, 0x5c38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(23608 as u32) ) } as u64;
	// 82306DFC: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82306E00: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82306E04: 4BFB91FD  bl 0x822c0000
	ctx.lr = 0x82306E08;
	sub_822C0000(ctx, base);
	// 82306E08: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82306E0C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82306E10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82306E14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82306E18: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82306E1C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82306E20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82306E28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82306E28 size=196
    let mut pc: u32 = 0x82306E28;
    'dispatch: loop {
        match pc {
            0x82306E28 => {
    //   block [0x82306E28..0x82306EEC)
	// 82306E28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82306E2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82306E30: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82306E34: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82306E38: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82306E3C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82306E40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82306E44: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82306E48: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82306E4C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82306E50: 4BFB9AE9  bl 0x822c0938
	ctx.lr = 0x82306E54;
	sub_822C0938(ctx, base);
	// 82306E54: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82306E58: 41820028  beq 0x82306e80
	if ctx.cr[0].eq {
	pc = 0x82306E80; continue 'dispatch;
	}
	// 82306E5C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82306E60: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82306E64: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82306E68: 392BCD00  addi r9, r11, -0x3300
	ctx.r[9].s64 = ctx.r[11].s64 + -13056;
	// 82306E6C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82306E70: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82306E74: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82306E78: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82306E7C: 48000008  b 0x82306e84
	pc = 0x82306E84; continue 'dispatch;
	// 82306E80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82306E84: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82306E88: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82306E8C: 409A0044  bne cr6, 0x82306ed0
	if !ctx.cr[6].eq {
	pc = 0x82306ED0; continue 'dispatch;
	}
	// 82306E90: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82306E94: 419A001C  beq cr6, 0x82306eb0
	if ctx.cr[6].eq {
	pc = 0x82306EB0; continue 'dispatch;
	}
	// 82306E98: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82306E9C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82306EA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82306EA4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82306EA8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82306EAC: 4E800421  bctrl
	ctx.lr = 0x82306EB0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82306EB0: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 82306EB4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82306EB8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82306EBC: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82306EC0: 816B5C38  lwz r11, 0x5c38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(23608 as u32) ) } as u64;
	// 82306EC4: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82306EC8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82306ECC: 4BFB9135  bl 0x822c0000
	ctx.lr = 0x82306ED0;
	sub_822C0000(ctx, base);
	// 82306ED0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82306ED4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82306ED8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82306EDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82306EE0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82306EE4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82306EE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82306EF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82306EF0 size=196
    let mut pc: u32 = 0x82306EF0;
    'dispatch: loop {
        match pc {
            0x82306EF0 => {
    //   block [0x82306EF0..0x82306FB4)
	// 82306EF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82306EF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82306EF8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82306EFC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82306F00: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82306F04: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82306F08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82306F0C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82306F10: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82306F14: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82306F18: 4BFB9A21  bl 0x822c0938
	ctx.lr = 0x82306F1C;
	sub_822C0938(ctx, base);
	// 82306F1C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82306F20: 41820028  beq 0x82306f48
	if ctx.cr[0].eq {
	pc = 0x82306F48; continue 'dispatch;
	}
	// 82306F24: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82306F28: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82306F2C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82306F30: 392BCD14  addi r9, r11, -0x32ec
	ctx.r[9].s64 = ctx.r[11].s64 + -13036;
	// 82306F34: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82306F38: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82306F3C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82306F40: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82306F44: 48000008  b 0x82306f4c
	pc = 0x82306F4C; continue 'dispatch;
	// 82306F48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82306F4C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82306F50: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82306F54: 409A0044  bne cr6, 0x82306f98
	if !ctx.cr[6].eq {
	pc = 0x82306F98; continue 'dispatch;
	}
	// 82306F58: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82306F5C: 419A001C  beq cr6, 0x82306f78
	if ctx.cr[6].eq {
	pc = 0x82306F78; continue 'dispatch;
	}
	// 82306F60: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82306F64: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82306F68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82306F6C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82306F70: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82306F74: 4E800421  bctrl
	ctx.lr = 0x82306F78;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82306F78: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 82306F7C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82306F80: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82306F84: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82306F88: 816B5C38  lwz r11, 0x5c38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(23608 as u32) ) } as u64;
	// 82306F8C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82306F90: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82306F94: 4BFB906D  bl 0x822c0000
	ctx.lr = 0x82306F98;
	sub_822C0000(ctx, base);
	// 82306F98: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82306F9C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82306FA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82306FA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82306FA8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82306FAC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82306FB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82306FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82306FB8 size=196
    let mut pc: u32 = 0x82306FB8;
    'dispatch: loop {
        match pc {
            0x82306FB8 => {
    //   block [0x82306FB8..0x8230707C)
	// 82306FB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82306FBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82306FC0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82306FC4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82306FC8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82306FCC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82306FD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82306FD4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82306FD8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82306FDC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82306FE0: 4BFB9959  bl 0x822c0938
	ctx.lr = 0x82306FE4;
	sub_822C0938(ctx, base);
	// 82306FE4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82306FE8: 41820028  beq 0x82307010
	if ctx.cr[0].eq {
	pc = 0x82307010; continue 'dispatch;
	}
	// 82306FEC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82306FF0: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82306FF4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82306FF8: 392BCD28  addi r9, r11, -0x32d8
	ctx.r[9].s64 = ctx.r[11].s64 + -13016;
	// 82306FFC: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82307000: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82307004: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82307008: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8230700C: 48000008  b 0x82307014
	pc = 0x82307014; continue 'dispatch;
	// 82307010: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82307014: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82307018: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8230701C: 409A0044  bne cr6, 0x82307060
	if !ctx.cr[6].eq {
	pc = 0x82307060; continue 'dispatch;
	}
	// 82307020: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82307024: 419A001C  beq cr6, 0x82307040
	if ctx.cr[6].eq {
	pc = 0x82307040; continue 'dispatch;
	}
	// 82307028: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8230702C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82307030: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82307034: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82307038: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8230703C: 4E800421  bctrl
	ctx.lr = 0x82307040;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82307040: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 82307044: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82307048: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8230704C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82307050: 816B5C38  lwz r11, 0x5c38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(23608 as u32) ) } as u64;
	// 82307054: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82307058: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8230705C: 4BFB8FA5  bl 0x822c0000
	ctx.lr = 0x82307060;
	sub_822C0000(ctx, base);
	// 82307060: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82307064: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82307068: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8230706C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82307070: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82307074: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82307078: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82307080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82307080 size=196
    let mut pc: u32 = 0x82307080;
    'dispatch: loop {
        match pc {
            0x82307080 => {
    //   block [0x82307080..0x82307144)
	// 82307080: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82307084: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82307088: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8230708C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82307090: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82307094: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82307098: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8230709C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823070A0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823070A4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823070A8: 4BFB9891  bl 0x822c0938
	ctx.lr = 0x823070AC;
	sub_822C0938(ctx, base);
	// 823070AC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823070B0: 41820028  beq 0x823070d8
	if ctx.cr[0].eq {
	pc = 0x823070D8; continue 'dispatch;
	}
	// 823070B4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823070B8: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823070BC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823070C0: 392BCD3C  addi r9, r11, -0x32c4
	ctx.r[9].s64 = ctx.r[11].s64 + -12996;
	// 823070C4: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823070C8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823070CC: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823070D0: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823070D4: 48000008  b 0x823070dc
	pc = 0x823070DC; continue 'dispatch;
	// 823070D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823070DC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823070E0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823070E4: 409A0044  bne cr6, 0x82307128
	if !ctx.cr[6].eq {
	pc = 0x82307128; continue 'dispatch;
	}
	// 823070E8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823070EC: 419A001C  beq cr6, 0x82307108
	if ctx.cr[6].eq {
	pc = 0x82307108; continue 'dispatch;
	}
	// 823070F0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823070F4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823070F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823070FC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82307100: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82307104: 4E800421  bctrl
	ctx.lr = 0x82307108;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82307108: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 8230710C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82307110: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82307114: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82307118: 816B5C38  lwz r11, 0x5c38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(23608 as u32) ) } as u64;
	// 8230711C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82307120: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82307124: 4BFB8EDD  bl 0x822c0000
	ctx.lr = 0x82307128;
	sub_822C0000(ctx, base);
	// 82307128: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8230712C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82307130: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82307134: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82307138: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8230713C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82307140: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82307148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82307148 size=196
    let mut pc: u32 = 0x82307148;
    'dispatch: loop {
        match pc {
            0x82307148 => {
    //   block [0x82307148..0x8230720C)
	// 82307148: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8230714C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82307150: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82307154: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82307158: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8230715C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82307160: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82307164: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82307168: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8230716C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82307170: 4BFB97C9  bl 0x822c0938
	ctx.lr = 0x82307174;
	sub_822C0938(ctx, base);
	// 82307174: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82307178: 41820028  beq 0x823071a0
	if ctx.cr[0].eq {
	pc = 0x823071A0; continue 'dispatch;
	}
	// 8230717C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82307180: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82307184: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82307188: 392BCD50  addi r9, r11, -0x32b0
	ctx.r[9].s64 = ctx.r[11].s64 + -12976;
	// 8230718C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82307190: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82307194: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82307198: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8230719C: 48000008  b 0x823071a4
	pc = 0x823071A4; continue 'dispatch;
	// 823071A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823071A4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823071A8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823071AC: 409A0044  bne cr6, 0x823071f0
	if !ctx.cr[6].eq {
	pc = 0x823071F0; continue 'dispatch;
	}
	// 823071B0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823071B4: 419A001C  beq cr6, 0x823071d0
	if ctx.cr[6].eq {
	pc = 0x823071D0; continue 'dispatch;
	}
	// 823071B8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823071BC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823071C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823071C4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 823071C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823071CC: 4E800421  bctrl
	ctx.lr = 0x823071D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823071D0: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 823071D4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823071D8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823071DC: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823071E0: 816B5C38  lwz r11, 0x5c38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(23608 as u32) ) } as u64;
	// 823071E4: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823071E8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823071EC: 4BFB8E15  bl 0x822c0000
	ctx.lr = 0x823071F0;
	sub_822C0000(ctx, base);
	// 823071F0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823071F4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823071F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823071FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82307200: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82307204: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82307208: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82307210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82307210 size=196
    let mut pc: u32 = 0x82307210;
    'dispatch: loop {
        match pc {
            0x82307210 => {
    //   block [0x82307210..0x823072D4)
	// 82307210: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82307214: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82307218: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8230721C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82307220: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82307224: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82307228: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8230722C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82307230: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82307234: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82307238: 4BFB9701  bl 0x822c0938
	ctx.lr = 0x8230723C;
	sub_822C0938(ctx, base);
	// 8230723C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82307240: 41820028  beq 0x82307268
	if ctx.cr[0].eq {
	pc = 0x82307268; continue 'dispatch;
	}
	// 82307244: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82307248: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8230724C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82307250: 392BCD64  addi r9, r11, -0x329c
	ctx.r[9].s64 = ctx.r[11].s64 + -12956;
	// 82307254: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82307258: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8230725C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82307260: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82307264: 48000008  b 0x8230726c
	pc = 0x8230726C; continue 'dispatch;
	// 82307268: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8230726C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82307270: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82307274: 409A0044  bne cr6, 0x823072b8
	if !ctx.cr[6].eq {
	pc = 0x823072B8; continue 'dispatch;
	}
	// 82307278: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8230727C: 419A001C  beq cr6, 0x82307298
	if ctx.cr[6].eq {
	pc = 0x82307298; continue 'dispatch;
	}
	// 82307280: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82307284: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82307288: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8230728C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82307290: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82307294: 4E800421  bctrl
	ctx.lr = 0x82307298;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82307298: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 8230729C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823072A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823072A4: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823072A8: 816B5C38  lwz r11, 0x5c38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(23608 as u32) ) } as u64;
	// 823072AC: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823072B0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823072B4: 4BFB8D4D  bl 0x822c0000
	ctx.lr = 0x823072B8;
	sub_822C0000(ctx, base);
	// 823072B8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823072BC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823072C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823072C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823072C8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823072CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823072D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823072D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823072D8 size=196
    let mut pc: u32 = 0x823072D8;
    'dispatch: loop {
        match pc {
            0x823072D8 => {
    //   block [0x823072D8..0x8230739C)
	// 823072D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823072DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823072E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823072E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823072E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823072EC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823072F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823072F4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823072F8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823072FC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82307300: 4BFB9639  bl 0x822c0938
	ctx.lr = 0x82307304;
	sub_822C0938(ctx, base);
	// 82307304: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82307308: 41820028  beq 0x82307330
	if ctx.cr[0].eq {
	pc = 0x82307330; continue 'dispatch;
	}
	// 8230730C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82307310: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82307314: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82307318: 392BCD78  addi r9, r11, -0x3288
	ctx.r[9].s64 = ctx.r[11].s64 + -12936;
	// 8230731C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82307320: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82307324: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82307328: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8230732C: 48000008  b 0x82307334
	pc = 0x82307334; continue 'dispatch;
	// 82307330: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82307334: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82307338: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8230733C: 409A0044  bne cr6, 0x82307380
	if !ctx.cr[6].eq {
	pc = 0x82307380; continue 'dispatch;
	}
	// 82307340: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82307344: 419A001C  beq cr6, 0x82307360
	if ctx.cr[6].eq {
	pc = 0x82307360; continue 'dispatch;
	}
	// 82307348: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8230734C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82307350: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82307354: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82307358: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8230735C: 4E800421  bctrl
	ctx.lr = 0x82307360;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82307360: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 82307364: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82307368: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8230736C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82307370: 816B5C38  lwz r11, 0x5c38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(23608 as u32) ) } as u64;
	// 82307374: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82307378: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8230737C: 4BFB8C85  bl 0x822c0000
	ctx.lr = 0x82307380;
	sub_822C0000(ctx, base);
	// 82307380: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82307384: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82307388: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8230738C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82307390: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82307394: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82307398: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823073A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823073A0 size=196
    let mut pc: u32 = 0x823073A0;
    'dispatch: loop {
        match pc {
            0x823073A0 => {
    //   block [0x823073A0..0x82307464)
	// 823073A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823073A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823073A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823073AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823073B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823073B4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823073B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823073BC: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823073C0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823073C4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823073C8: 4BFB9571  bl 0x822c0938
	ctx.lr = 0x823073CC;
	sub_822C0938(ctx, base);
	// 823073CC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823073D0: 41820028  beq 0x823073f8
	if ctx.cr[0].eq {
	pc = 0x823073F8; continue 'dispatch;
	}
	// 823073D4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823073D8: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823073DC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823073E0: 392BCD8C  addi r9, r11, -0x3274
	ctx.r[9].s64 = ctx.r[11].s64 + -12916;
	// 823073E4: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823073E8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823073EC: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823073F0: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823073F4: 48000008  b 0x823073fc
	pc = 0x823073FC; continue 'dispatch;
	// 823073F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823073FC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82307400: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82307404: 409A0044  bne cr6, 0x82307448
	if !ctx.cr[6].eq {
	pc = 0x82307448; continue 'dispatch;
	}
	// 82307408: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8230740C: 419A001C  beq cr6, 0x82307428
	if ctx.cr[6].eq {
	pc = 0x82307428; continue 'dispatch;
	}
	// 82307410: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82307414: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82307418: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8230741C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82307420: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82307424: 4E800421  bctrl
	ctx.lr = 0x82307428;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82307428: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 8230742C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82307430: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82307434: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82307438: 816B5C38  lwz r11, 0x5c38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(23608 as u32) ) } as u64;
	// 8230743C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82307440: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82307444: 4BFB8BBD  bl 0x822c0000
	ctx.lr = 0x82307448;
	sub_822C0000(ctx, base);
	// 82307448: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8230744C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82307450: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82307454: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82307458: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8230745C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82307460: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82307468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82307468 size=196
    let mut pc: u32 = 0x82307468;
    'dispatch: loop {
        match pc {
            0x82307468 => {
    //   block [0x82307468..0x8230752C)
	// 82307468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8230746C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82307470: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82307474: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82307478: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8230747C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82307480: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82307484: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82307488: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8230748C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82307490: 4BFB94A9  bl 0x822c0938
	ctx.lr = 0x82307494;
	sub_822C0938(ctx, base);
	// 82307494: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82307498: 41820028  beq 0x823074c0
	if ctx.cr[0].eq {
	pc = 0x823074C0; continue 'dispatch;
	}
	// 8230749C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823074A0: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823074A4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823074A8: 392BCDA0  addi r9, r11, -0x3260
	ctx.r[9].s64 = ctx.r[11].s64 + -12896;
	// 823074AC: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823074B0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823074B4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823074B8: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823074BC: 48000008  b 0x823074c4
	pc = 0x823074C4; continue 'dispatch;
	// 823074C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823074C4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823074C8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823074CC: 409A0044  bne cr6, 0x82307510
	if !ctx.cr[6].eq {
	pc = 0x82307510; continue 'dispatch;
	}
	// 823074D0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823074D4: 419A001C  beq cr6, 0x823074f0
	if ctx.cr[6].eq {
	pc = 0x823074F0; continue 'dispatch;
	}
	// 823074D8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823074DC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823074E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823074E4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 823074E8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823074EC: 4E800421  bctrl
	ctx.lr = 0x823074F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823074F0: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 823074F4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823074F8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823074FC: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82307500: 816B5C38  lwz r11, 0x5c38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(23608 as u32) ) } as u64;
	// 82307504: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82307508: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8230750C: 4BFB8AF5  bl 0x822c0000
	ctx.lr = 0x82307510;
	sub_822C0000(ctx, base);
	// 82307510: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82307514: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82307518: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8230751C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82307520: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82307524: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82307528: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82307530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82307530 size=196
    let mut pc: u32 = 0x82307530;
    'dispatch: loop {
        match pc {
            0x82307530 => {
    //   block [0x82307530..0x823075F4)
	// 82307530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82307534: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82307538: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8230753C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82307540: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82307544: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82307548: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8230754C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82307550: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82307554: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82307558: 4BFB93E1  bl 0x822c0938
	ctx.lr = 0x8230755C;
	sub_822C0938(ctx, base);
	// 8230755C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82307560: 41820028  beq 0x82307588
	if ctx.cr[0].eq {
	pc = 0x82307588; continue 'dispatch;
	}
	// 82307564: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82307568: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8230756C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82307570: 392BCDB4  addi r9, r11, -0x324c
	ctx.r[9].s64 = ctx.r[11].s64 + -12876;
	// 82307574: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82307578: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8230757C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82307580: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82307584: 48000008  b 0x8230758c
	pc = 0x8230758C; continue 'dispatch;
	// 82307588: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8230758C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82307590: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82307594: 409A0044  bne cr6, 0x823075d8
	if !ctx.cr[6].eq {
	pc = 0x823075D8; continue 'dispatch;
	}
	// 82307598: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8230759C: 419A001C  beq cr6, 0x823075b8
	if ctx.cr[6].eq {
	pc = 0x823075B8; continue 'dispatch;
	}
	// 823075A0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823075A4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823075A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823075AC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 823075B0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823075B4: 4E800421  bctrl
	ctx.lr = 0x823075B8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823075B8: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 823075BC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823075C0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823075C4: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823075C8: 816B5C38  lwz r11, 0x5c38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(23608 as u32) ) } as u64;
	// 823075CC: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823075D0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823075D4: 4BFB8A2D  bl 0x822c0000
	ctx.lr = 0x823075D8;
	sub_822C0000(ctx, base);
	// 823075D8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823075DC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823075E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823075E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823075E8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823075EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823075F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823075F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823075F8 size=196
    let mut pc: u32 = 0x823075F8;
    'dispatch: loop {
        match pc {
            0x823075F8 => {
    //   block [0x823075F8..0x823076BC)
	// 823075F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823075FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82307600: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82307604: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82307608: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8230760C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82307610: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82307614: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82307618: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8230761C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82307620: 4BFB9319  bl 0x822c0938
	ctx.lr = 0x82307624;
	sub_822C0938(ctx, base);
	// 82307624: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82307628: 41820028  beq 0x82307650
	if ctx.cr[0].eq {
	pc = 0x82307650; continue 'dispatch;
	}
	// 8230762C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82307630: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82307634: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82307638: 392BCDC8  addi r9, r11, -0x3238
	ctx.r[9].s64 = ctx.r[11].s64 + -12856;
	// 8230763C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82307640: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82307644: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82307648: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8230764C: 48000008  b 0x82307654
	pc = 0x82307654; continue 'dispatch;
	// 82307650: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82307654: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82307658: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8230765C: 409A0044  bne cr6, 0x823076a0
	if !ctx.cr[6].eq {
	pc = 0x823076A0; continue 'dispatch;
	}
	// 82307660: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82307664: 419A001C  beq cr6, 0x82307680
	if ctx.cr[6].eq {
	pc = 0x82307680; continue 'dispatch;
	}
	// 82307668: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8230766C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82307670: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82307674: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82307678: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8230767C: 4E800421  bctrl
	ctx.lr = 0x82307680;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82307680: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 82307684: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82307688: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8230768C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82307690: 816B5C38  lwz r11, 0x5c38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(23608 as u32) ) } as u64;
	// 82307694: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82307698: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8230769C: 4BFB8965  bl 0x822c0000
	ctx.lr = 0x823076A0;
	sub_822C0000(ctx, base);
	// 823076A0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823076A4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823076A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823076AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823076B0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823076B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823076B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823076C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823076C0 size=108
    let mut pc: u32 = 0x823076C0;
    'dispatch: loop {
        match pc {
            0x823076C0 => {
    //   block [0x823076C0..0x8230772C)
	// 823076C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823076C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823076C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823076CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823076D0: F8810080  std r4, 0x80(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[4].u64 ) };
	// 823076D4: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 823076D8: F8A10088  std r5, 0x88(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[5].u64 ) };
	// 823076DC: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 823076E0: 484C7AF9  bl 0x827cf1d8
	ctx.lr = 0x823076E4;
	sub_827CF1D8(ctx, base);
	// 823076E4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823076E8: 4082002C  bne 0x82307714
	if !ctx.cr[0].eq {
	pc = 0x82307714; continue 'dispatch;
	}
	// 823076EC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823076F0: 419A001C  beq cr6, 0x8230770c
	if ctx.cr[6].eq {
	pc = 0x8230770C; continue 'dispatch;
	}
	// 823076F4: 81610080  lwz r11, 0x80(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 823076F8: 81410084  lwz r10, 0x84(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 823076FC: 81210088  lwz r9, 0x88(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 82307700: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82307704: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82307708: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 8230770C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82307710: 48000008  b 0x82307718
	pc = 0x82307718; continue 'dispatch;
	// 82307714: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82307718: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8230771C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82307720: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82307724: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82307728: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82307730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82307730 size=72
    let mut pc: u32 = 0x82307730;
    'dispatch: loop {
        match pc {
            0x82307730 => {
    //   block [0x82307730..0x82307778)
	// 82307730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82307734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82307738: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8230773C: 2F050003  cmpwi cr6, r5, 3
	ctx.cr[6].compare_i32(ctx.r[5].s32, 3, &mut ctx.xer);
	// 82307740: 419A001C  beq cr6, 0x8230775c
	if ctx.cr[6].eq {
	pc = 0x8230775C; continue 'dispatch;
	}
	// 82307744: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82307748: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8230774C: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82307750: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82307754: 4BFFE16D  bl 0x823058c0
	ctx.lr = 0x82307758;
	sub_823058C0(ctx, base);
	// 82307758: 48000010  b 0x82307768
	pc = 0x82307768; continue 'dispatch;
	// 8230775C: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 82307760: 396B6334  addi r11, r11, 0x6334
	ctx.r[11].s64 = ctx.r[11].s64 + 25396;
	// 82307764: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82307768: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8230776C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82307770: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82307774: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82307778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82307778 size=92
    let mut pc: u32 = 0x82307778;
    'dispatch: loop {
        match pc {
            0x82307778 => {
    //   block [0x82307778..0x823077D4)
	// 82307778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8230777C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82307780: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82307784: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82307788: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8230778C: C0040000  lfs f0, 0(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82307790: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
	// 82307794: D01F0000  stfs f0, 0(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82307798: C0040004  lfs f0, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8230779C: D01F0004  stfs f0, 4(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 823077A0: C0040008  lfs f0, 8(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823077A4: D01F0008  stfs f0, 8(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 823077A8: C004000C  lfs f0, 0xc(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823077AC: D01F000C  stfs f0, 0xc(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 823077B0: 13E028C7  vcmpequd (lvx128) v31, v0, v5
	tmp.u32 = ctx.r[5].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823077D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x823077D8 size=104
    let mut pc: u32 = 0x823077D8;
    'dispatch: loop {
        match pc {
            0x823077D8 => {
    //   block [0x823077D8..0x82307840)
	// 823077D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823077DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823077E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823077E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823077E8: 80630010  lwz r3, 0x10(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 823077EC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823077F0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 823077F4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 823077F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823077FC: 4E800421  bctrl
	ctx.lr = 0x82307800;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82307800: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82307804: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 82307808: 13E01C07  vcmpneb. (lvlx128) v31, v0, v3
	tmp.u32 = ctx.r[3].u32;
	// load shuffled into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 8230780C: 39600030  li r11, 0x30
	ctx.r[11].s64 = 48;
	// 82307810: 13C91C07  vcmpneb. (lvlx128) v30, v9, v3
	tmp.u32 = ctx.r[9].u32 + ctx.r[3].u32;
	// load shuffled into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82307814: 13AA1C07  vcmpneb. (lvlx128) v29, v10, v3
	tmp.u32 = ctx.r[10].u32 + ctx.r[3].u32;
	// load shuffled into ctx.v[61] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82307818: 138B1C07  vcmpneb. (lvlx128) v28, v11, v3
	tmp.u32 = ctx.r[11].u32 + ctx.r[3].u32;
	// load shuffled into ctx.v[60] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82307840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82307840 size=96
    let mut pc: u32 = 0x82307840;
    'dispatch: loop {
        match pc {
            0x82307840 => {
    //   block [0x82307840..0x823078A0)
	// 82307840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82307844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82307848: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8230784C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82307850: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82307854: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82307858: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8230785C: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82307860: 484EDB71  bl 0x827f53d0
	ctx.lr = 0x82307864;
	sub_827F53D0(ctx, base);
	// 82307864: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82307868: 13E0F0C7  vcmpequd (lvx128) v31, v0, v30
	tmp.u32 = ctx.r[30].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 8230786C: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82307870: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82307874: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82307878: 13C058C7  vcmpequd (lvx128) v30, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823078A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823078A0 size=52
    let mut pc: u32 = 0x823078A0;
    'dispatch: loop {
        match pc {
            0x823078A0 => {
    //   block [0x823078A0..0x823078D4)
	// 823078A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823078A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823078A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823078AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823078B0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823078B4: 4BFFFF8D  bl 0x82307840
	ctx.lr = 0x823078B8;
	sub_82307840(ctx, base);
	// 823078B8: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 823078BC: 484EDB15  bl 0x827f53d0
	ctx.lr = 0x823078C0;
	sub_827F53D0(ctx, base);
	// 823078C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 823078C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823078C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823078CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823078D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823078D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823078D8 size=92
    let mut pc: u32 = 0x823078D8;
    'dispatch: loop {
        match pc {
            0x823078D8 => {
    //   block [0x823078D8..0x82307934)
	// 823078D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823078DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823078E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823078E4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823078E8: 8963013C  lbz r11, 0x13c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(316 as u32) ) } as u64;
	// 823078EC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823078F0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823078F4: 4082002C  bne 0x82307920
	if !ctx.cr[0].eq {
	pc = 0x82307920; continue 'dispatch;
	}
	// 823078F8: 80630100  lwz r3, 0x100(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(256 as u32) ) } as u64;
	// 823078FC: 486014ED  bl 0x82908de8
	ctx.lr = 0x82307900;
	sub_82908DE8(ctx, base);
	// 82307900: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82307904: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82307908: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8230790C: 4BFF6585  bl 0x822fde90
	ctx.lr = 0x82307910;
	sub_822FDE90(ctx, base);
	// 82307910: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82307914: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82307918: 419A0008  beq cr6, 0x82307920
	if ctx.cr[6].eq {
	pc = 0x82307920; continue 'dispatch;
	}
	// 8230791C: 4BFB8F75  bl 0x822c0890
	ctx.lr = 0x82307920;
	sub_822C0890(ctx, base);
	// 82307920: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82307924: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82307928: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8230792C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82307930: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82307938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82307938 size=108
    let mut pc: u32 = 0x82307938;
    'dispatch: loop {
        match pc {
            0x82307938 => {
    //   block [0x82307938..0x823079A4)
	// 82307938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8230793C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82307940: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82307944: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82307948: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8230794C: 8963013C  lbz r11, 0x13c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(316 as u32) ) } as u64;
	// 82307950: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82307954: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82307958: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8230795C: 40820030  bne 0x8230798c
	if !ctx.cr[0].eq {
	pc = 0x8230798C; continue 'dispatch;
	}
	// 82307960: 80630100  lwz r3, 0x100(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(256 as u32) ) } as u64;
	// 82307964: 48601485  bl 0x82908de8
	ctx.lr = 0x82307968;
	sub_82908DE8(ctx, base);
	// 82307968: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8230796C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82307970: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82307974: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82307978: 4BFF6621  bl 0x822fdf98
	ctx.lr = 0x8230797C;
	sub_822FDF98(ctx, base);
	// 8230797C: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82307980: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82307984: 419A0008  beq cr6, 0x8230798c
	if ctx.cr[6].eq {
	pc = 0x8230798C; continue 'dispatch;
	}
	// 82307988: 4BFB8F09  bl 0x822c0890
	ctx.lr = 0x8230798C;
	sub_822C0890(ctx, base);
	// 8230798C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82307990: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82307994: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82307998: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8230799C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823079A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823079A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823079A8 size=76
    let mut pc: u32 = 0x823079A8;
    'dispatch: loop {
        match pc {
            0x823079A8 => {
    //   block [0x823079A8..0x823079F4)
	// 823079A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823079AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823079B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823079B4: 8963013C  lbz r11, 0x13c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(316 as u32) ) } as u64;
	// 823079B8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823079BC: 40820028  bne 0x823079e4
	if !ctx.cr[0].eq {
	pc = 0x823079E4; continue 'dispatch;
	}
	// 823079C0: 80630100  lwz r3, 0x100(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(256 as u32) ) } as u64;
	// 823079C4: 48601425  bl 0x82908de8
	ctx.lr = 0x823079C8;
	sub_82908DE8(ctx, base);
	// 823079C8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 823079CC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823079D0: 4BFF6D61  bl 0x822fe730
	ctx.lr = 0x823079D4;
	sub_822FE730(ctx, base);
	// 823079D4: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 823079D8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823079DC: 419A0008  beq cr6, 0x823079e4
	if ctx.cr[6].eq {
	pc = 0x823079E4; continue 'dispatch;
	}
	// 823079E0: 4BFB8EB1  bl 0x822c0890
	ctx.lr = 0x823079E4;
	sub_822C0890(ctx, base);
	// 823079E4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 823079E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823079EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823079F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823079F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823079F8 size=92
    let mut pc: u32 = 0x823079F8;
    'dispatch: loop {
        match pc {
            0x823079F8 => {
    //   block [0x823079F8..0x82307A54)
	// 823079F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823079FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82307A00: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82307A04: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82307A08: 8963013C  lbz r11, 0x13c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(316 as u32) ) } as u64;
	// 82307A0C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82307A10: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82307A14: 4082002C  bne 0x82307a40
	if !ctx.cr[0].eq {
	pc = 0x82307A40; continue 'dispatch;
	}
	// 82307A18: 80630100  lwz r3, 0x100(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(256 as u32) ) } as u64;
	// 82307A1C: 486013CD  bl 0x82908de8
	ctx.lr = 0x82307A20;
	sub_82908DE8(ctx, base);
	// 82307A20: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82307A24: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82307A28: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82307A2C: 4BFF6C3D  bl 0x822fe668
	ctx.lr = 0x82307A30;
	sub_822FE668(ctx, base);
	// 82307A30: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82307A34: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82307A38: 419A0008  beq cr6, 0x82307a40
	if ctx.cr[6].eq {
	pc = 0x82307A40; continue 'dispatch;
	}
	// 82307A3C: 4BFB8E55  bl 0x822c0890
	ctx.lr = 0x82307A40;
	sub_822C0890(ctx, base);
	// 82307A40: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82307A44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82307A48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82307A4C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82307A50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82307A58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82307A58 size=108
    let mut pc: u32 = 0x82307A58;
    'dispatch: loop {
        match pc {
            0x82307A58 => {
    //   block [0x82307A58..0x82307AC4)
	// 82307A58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82307A5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82307A60: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82307A64: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 82307A68: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82307A6C: 8963013C  lbz r11, 0x13c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(316 as u32) ) } as u64;
	// 82307A70: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82307A74: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82307A78: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82307A7C: 40820030  bne 0x82307aac
	if !ctx.cr[0].eq {
	pc = 0x82307AAC; continue 'dispatch;
	}
	// 82307A80: 80630100  lwz r3, 0x100(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(256 as u32) ) } as u64;
	// 82307A84: 48601365  bl 0x82908de8
	ctx.lr = 0x82307A88;
	sub_82908DE8(ctx, base);
	// 82307A88: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82307A8C: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82307A90: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82307A94: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82307A98: 4BFF6B31  bl 0x822fe5c8
	ctx.lr = 0x82307A9C;
	sub_822FE5C8(ctx, base);
	// 82307A9C: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82307AA0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82307AA4: 419A0008  beq cr6, 0x82307aac
	if ctx.cr[6].eq {
	pc = 0x82307AAC; continue 'dispatch;
	}
	// 82307AA8: 4BFB8DE9  bl 0x822c0890
	ctx.lr = 0x82307AAC;
	sub_822C0890(ctx, base);
	// 82307AAC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82307AB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82307AB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82307AB8: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82307ABC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82307AC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82307AC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82307AC8 size=92
    let mut pc: u32 = 0x82307AC8;
    'dispatch: loop {
        match pc {
            0x82307AC8 => {
    //   block [0x82307AC8..0x82307B24)
	// 82307AC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82307ACC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82307AD0: DBE1FFF0  stfd f31, -0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[31].u64 ) };
	// 82307AD4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82307AD8: 8963013C  lbz r11, 0x13c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(316 as u32) ) } as u64;
	// 82307ADC: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82307AE0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82307AE4: 4082002C  bne 0x82307b10
	if !ctx.cr[0].eq {
	pc = 0x82307B10; continue 'dispatch;
	}
	// 82307AE8: 80630100  lwz r3, 0x100(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(256 as u32) ) } as u64;
	// 82307AEC: 486012FD  bl 0x82908de8
	ctx.lr = 0x82307AF0;
	sub_82908DE8(ctx, base);
	// 82307AF0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82307AF4: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82307AF8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82307AFC: 4BFF6BA5  bl 0x822fe6a0
	ctx.lr = 0x82307B00;
	sub_822FE6A0(ctx, base);
	// 82307B00: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82307B04: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82307B08: 419A0008  beq cr6, 0x82307b10
	if ctx.cr[6].eq {
	pc = 0x82307B10; continue 'dispatch;
	}
	// 82307B0C: 4BFB8D85  bl 0x822c0890
	ctx.lr = 0x82307B10;
	sub_822C0890(ctx, base);
	// 82307B10: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82307B14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82307B18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82307B1C: CBE1FFF0  lfd f31, -0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82307B20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82307B28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82307B28 size=76
    let mut pc: u32 = 0x82307B28;
    'dispatch: loop {
        match pc {
            0x82307B28 => {
    //   block [0x82307B28..0x82307B74)
	// 82307B28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82307B2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82307B30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82307B34: 8963013C  lbz r11, 0x13c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(316 as u32) ) } as u64;
	// 82307B38: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82307B3C: 40820028  bne 0x82307b64
	if !ctx.cr[0].eq {
	pc = 0x82307B64; continue 'dispatch;
	}
	// 82307B40: 80630100  lwz r3, 0x100(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(256 as u32) ) } as u64;
	// 82307B44: 486012A5  bl 0x82908de8
	ctx.lr = 0x82307B48;
	sub_82908DE8(ctx, base);
	// 82307B48: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82307B4C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82307B50: 4BFF6581  bl 0x822fe0d0
	ctx.lr = 0x82307B54;
	sub_822FE0D0(ctx, base);
	// 82307B54: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82307B58: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82307B5C: 419A0008  beq cr6, 0x82307b64
	if ctx.cr[6].eq {
	pc = 0x82307B64; continue 'dispatch;
	}
	// 82307B60: 4BFB8D31  bl 0x822c0890
	ctx.lr = 0x82307B64;
	sub_822C0890(ctx, base);
	// 82307B64: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82307B68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82307B6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82307B70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82307B78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82307B78 size=92
    let mut pc: u32 = 0x82307B78;
    'dispatch: loop {
        match pc {
            0x82307B78 => {
    //   block [0x82307B78..0x82307BD4)
	// 82307B78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82307B7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82307B80: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82307B84: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82307B88: 8963013C  lbz r11, 0x13c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(316 as u32) ) } as u64;
	// 82307B8C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82307B90: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82307B94: 4082002C  bne 0x82307bc0
	if !ctx.cr[0].eq {
	pc = 0x82307BC0; continue 'dispatch;
	}
	// 82307B98: 80630100  lwz r3, 0x100(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(256 as u32) ) } as u64;
	// 82307B9C: 4860124D  bl 0x82908de8
	ctx.lr = 0x82307BA0;
	sub_82908DE8(ctx, base);
	// 82307BA0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82307BA4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82307BA8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82307BAC: 4BFF65E5  bl 0x822fe190
	ctx.lr = 0x82307BB0;
	sub_822FE190(ctx, base);
	// 82307BB0: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82307BB4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82307BB8: 419A0008  beq cr6, 0x82307bc0
	if ctx.cr[6].eq {
	pc = 0x82307BC0; continue 'dispatch;
	}
	// 82307BBC: 4BFB8CD5  bl 0x822c0890
	ctx.lr = 0x82307BC0;
	sub_822C0890(ctx, base);
	// 82307BC0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82307BC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82307BC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82307BCC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82307BD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82307BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82307BD8 size=52
    let mut pc: u32 = 0x82307BD8;
    'dispatch: loop {
        match pc {
            0x82307BD8 => {
    //   block [0x82307BD8..0x82307C0C)
	// 82307BD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82307BDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82307BE0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82307BE4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82307BE8: 80840100  lwz r4, 0x100(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(256 as u32) ) } as u64;
	// 82307BEC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82307BF0: 4BFFA881  bl 0x82302470
	ctx.lr = 0x82307BF4;
	sub_82302470(ctx, base);
	// 82307BF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82307BF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82307BFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82307C00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82307C04: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82307C08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82307C10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82307C10 size=172
    let mut pc: u32 = 0x82307C10;
    'dispatch: loop {
        match pc {
            0x82307C10 => {
    //   block [0x82307C10..0x82307CBC)
	// 82307C10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82307C14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82307C18: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82307C1C: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82307C20: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82307C24: 80640010  lwz r3, 0x10(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 82307C28: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82307C2C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82307C30: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82307C34: 4E800421  bctrl
	ctx.lr = 0x82307C38;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82307C38: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
	// 82307C3C: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 82307C40: 13E01C07  vcmpneb. (lvlx128) v31, v0, v3
	tmp.u32 = ctx.r[3].u32;
	// load shuffled into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82307C44: 39200030  li r9, 0x30
	ctx.r[9].s64 = 48;
	// 82307C48: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 82307C4C: 3CE08200  lis r7, -0x7e00
	ctx.r[7].s64 = -2113929216;
	// 82307C50: 13CB1C07  vcmpneb. (lvlx128) v30, v11, v3
	tmp.u32 = ctx.r[11].u32 + ctx.r[3].u32;
	// load shuffled into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82307C54: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 82307C58: 13AA1C07  vcmpneb. (lvlx128) v29, v10, v3
	tmp.u32 = ctx.r[10].u32 + ctx.r[3].u32;
	// load shuffled into ctx.v[61] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82307C5C: 39610070  addi r11, r1, 0x70
	ctx.r[11].s64 = ctx.r[1].s64 + 112;
	// 82307C60: 13891C07  vcmpneb. (lvlx128) v28, v9, v3
	tmp.u32 = ctx.r[9].u32 + ctx.r[3].u32;
	// load shuffled into ctx.v[60] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82307C64: 39410080  addi r10, r1, 0x80
	ctx.r[10].s64 = ctx.r[1].s64 + 128;
	// 82307C68: 39210090  addi r9, r1, 0x90
	ctx.r[9].s64 = ctx.r[1].s64 + 144;
	// 82307C6C: C1A89534  lfs f13, -0x6acc(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-27340 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82307C70: C00708A4  lfs f0, 0x8a4(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82307C74: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82307C78: D1A10050  stfs f13, 0x50(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82307C7C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82307C80: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82307C84: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82307C88: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82307C8C: D001005C  stfs f0, 0x5c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82307CC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82307CC0 size=172
    let mut pc: u32 = 0x82307CC0;
    'dispatch: loop {
        match pc {
            0x82307CC0 => {
    //   block [0x82307CC0..0x82307D6C)
	// 82307CC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82307CC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82307CC8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82307CCC: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82307CD0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82307CD4: 80640010  lwz r3, 0x10(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 82307CD8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82307CDC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82307CE0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82307CE4: 4E800421  bctrl
	ctx.lr = 0x82307CE8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82307CE8: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
	// 82307CEC: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 82307CF0: 13E01C07  vcmpneb. (lvlx128) v31, v0, v3
	tmp.u32 = ctx.r[3].u32;
	// load shuffled into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82307CF4: 39200030  li r9, 0x30
	ctx.r[9].s64 = 48;
	// 82307CF8: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82307CFC: 3CE08200  lis r7, -0x7e00
	ctx.r[7].s64 = -2113929216;
	// 82307D00: 13CB1C07  vcmpneb. (lvlx128) v30, v11, v3
	tmp.u32 = ctx.r[11].u32 + ctx.r[3].u32;
	// load shuffled into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82307D04: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 82307D08: 13AA1C07  vcmpneb. (lvlx128) v29, v10, v3
	tmp.u32 = ctx.r[10].u32 + ctx.r[3].u32;
	// load shuffled into ctx.v[61] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82307D0C: 39610070  addi r11, r1, 0x70
	ctx.r[11].s64 = ctx.r[1].s64 + 112;
	// 82307D10: 13891C07  vcmpneb. (lvlx128) v28, v9, v3
	tmp.u32 = ctx.r[9].u32 + ctx.r[3].u32;
	// load shuffled into ctx.v[60] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82307D14: 39410080  addi r10, r1, 0x80
	ctx.r[10].s64 = ctx.r[1].s64 + 128;
	// 82307D18: 39210090  addi r9, r1, 0x90
	ctx.r[9].s64 = ctx.r[1].s64 + 144;
	// 82307D1C: C00808A4  lfs f0, 0x8a4(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82307D20: C1A708A8  lfs f13, 0x8a8(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(2216 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82307D24: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82307D28: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82307D2C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82307D30: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82307D34: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82307D38: D1A10058  stfs f13, 0x58(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82307D3C: D001005C  stfs f0, 0x5c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82307D70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82307D70 size=172
    let mut pc: u32 = 0x82307D70;
    'dispatch: loop {
        match pc {
            0x82307D70 => {
    //   block [0x82307D70..0x82307E1C)
	// 82307D70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82307D74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82307D78: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82307D7C: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82307D80: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82307D84: 80640010  lwz r3, 0x10(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 82307D88: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82307D8C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82307D90: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82307D94: 4E800421  bctrl
	ctx.lr = 0x82307D98;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82307D98: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
	// 82307D9C: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 82307DA0: 13E01C07  vcmpneb. (lvlx128) v31, v0, v3
	tmp.u32 = ctx.r[3].u32;
	// load shuffled into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82307DA4: 39200030  li r9, 0x30
	ctx.r[9].s64 = 48;
	// 82307DA8: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82307DAC: 3CE08200  lis r7, -0x7e00
	ctx.r[7].s64 = -2113929216;
	// 82307DB0: 13CB1C07  vcmpneb. (lvlx128) v30, v11, v3
	tmp.u32 = ctx.r[11].u32 + ctx.r[3].u32;
	// load shuffled into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82307DB4: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 82307DB8: 13AA1C07  vcmpneb. (lvlx128) v29, v10, v3
	tmp.u32 = ctx.r[10].u32 + ctx.r[3].u32;
	// load shuffled into ctx.v[61] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82307DBC: 39610070  addi r11, r1, 0x70
	ctx.r[11].s64 = ctx.r[1].s64 + 112;
	// 82307DC0: 13891C07  vcmpneb. (lvlx128) v28, v9, v3
	tmp.u32 = ctx.r[9].u32 + ctx.r[3].u32;
	// load shuffled into ctx.v[60] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82307DC4: 39410080  addi r10, r1, 0x80
	ctx.r[10].s64 = ctx.r[1].s64 + 128;
	// 82307DC8: 39210090  addi r9, r1, 0x90
	ctx.r[9].s64 = ctx.r[1].s64 + 144;
	// 82307DCC: C00808A4  lfs f0, 0x8a4(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82307DD0: C1A708A8  lfs f13, 0x8a8(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(2216 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82307DD4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82307DD8: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82307DDC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82307DE0: D1A10054  stfs f13, 0x54(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82307DE4: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82307DE8: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82307DEC: D001005C  stfs f0, 0x5c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


