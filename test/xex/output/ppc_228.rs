pub fn sub_832974E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832974E0 size=52
    let mut pc: u32 = 0x832974E0;
    'dispatch: loop {
        match pc {
            0x832974E0 => {
    //   block [0x832974E0..0x83297514)
	// 832974E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832974E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832974E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832974EC: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832974F0: 386B4BC0  addi r3, r11, 0x4bc0
	ctx.r[3].s64 = ctx.r[11].s64 + 19392;
	// 832974F4: 4AF6030D  bl 0x821f7800
	ctx.lr = 0x832974F8;
	sub_821F7800(ctx, base);
	// 832974F8: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 832974FC: 386A7310  addi r3, r10, 0x7310
	ctx.r[3].s64 = ctx.r[10].s64 + 29456;
	// 83297500: 4BA12A21  bl 0x82ca9f20
	ctx.lr = 0x83297504;
	sub_82CA9F20(ctx, base);
	// 83297504: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83297508: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329750C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83297510: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83297518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83297518 size=52
    let mut pc: u32 = 0x83297518;
    'dispatch: loop {
        match pc {
            0x83297518 => {
    //   block [0x83297518..0x8329754C)
	// 83297518: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329751C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83297520: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83297524: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 83297528: 386B4BD8  addi r3, r11, 0x4bd8
	ctx.r[3].s64 = ctx.r[11].s64 + 19416;
	// 8329752C: 4B972685  bl 0x82c09bb0
	ctx.lr = 0x83297530;
	sub_82C09BB0(ctx, base);
	// 83297530: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 83297534: 386A73B8  addi r3, r10, 0x73b8
	ctx.r[3].s64 = ctx.r[10].s64 + 29624;
	// 83297538: 4BA129E9  bl 0x82ca9f20
	ctx.lr = 0x8329753C;
	sub_82CA9F20(ctx, base);
	// 8329753C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83297540: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83297544: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83297548: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83297550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83297550 size=84
    let mut pc: u32 = 0x83297550;
    'dispatch: loop {
        match pc {
            0x83297550 => {
    //   block [0x83297550..0x83297584)
	// 83297550: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83297554: 3D602323  lis r11, 0x2323
	ctx.r[11].s64 = 589496320;
	// 83297558: 390A4C38  addi r8, r10, 0x4c38
	ctx.r[8].s64 = ctx.r[10].s64 + 19512;
	// 8329755C: 616B2323  ori r11, r11, 0x2323
	ctx.r[11].u64 = ctx.r[11].u64 | 8995;
	// 83297560: 7D064378  mr r6, r8
	ctx.r[6].u64 = ctx.r[8].u64;
	// 83297564: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 83297568: 7D094378  mr r9, r8
	ctx.r[9].u64 = ctx.r[8].u64;
	// 8329756C: 394AA8DC  addi r10, r10, -0x5724
	ctx.r[10].s64 = ctx.r[10].s64 + -22308;
	// 83297570: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 83297574: 91660000  stw r11, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83297578: 91660004  stw r11, 4(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8329757C: 91660008  stw r11, 8(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83297580: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	pc = 0x83297584; continue 'dispatch;
            }
            0x83297584 => {
    //   block [0x83297584..0x832975A4)
	// 83297584: 896A0000  lbz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83297588: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8329758C: 99690000  stb r11, 0(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 83297590: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 83297594: 4200FFF0  bdnz 0x83297584
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x83297584; continue 'dispatch;
	}
	// 83297598: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8329759C: 9168000C  stw r11, 0xc(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 832975A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832975A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832975A8 size=96
    let mut pc: u32 = 0x832975A8;
    'dispatch: loop {
        match pc {
            0x832975A8 => {
    //   block [0x832975A8..0x83297608)
	// 832975A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832975AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832975B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832975B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832975B8: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 832975BC: 3D602323  lis r11, 0x2323
	ctx.r[11].s64 = 589496320;
	// 832975C0: 3BEA4C48  addi r31, r10, 0x4c48
	ctx.r[31].s64 = ctx.r[10].s64 + 19528;
	// 832975C4: 616B2323  ori r11, r11, 0x2323
	ctx.r[11].u64 = ctx.r[11].u64 | 8995;
	// 832975C8: 7FE9FB78  mr r9, r31
	ctx.r[9].u64 = ctx.r[31].u64;
	// 832975CC: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 832975D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832975D4: 3888A8E8  addi r4, r8, -0x5718
	ctx.r[4].s64 = ctx.r[8].s64 + -22296;
	// 832975D8: 38A0000C  li r5, 0xc
	ctx.r[5].s64 = 12;
	// 832975DC: 91690000  stw r11, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 832975E0: 91690004  stw r11, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832975E4: 91690008  stw r11, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 832975E8: 4BA11E99  bl 0x82ca9480
	ctx.lr = 0x832975EC;
	sub_82CA9480(ctx, base);
	// 832975EC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 832975F0: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 832975F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832975F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832975FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83297600: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83297604: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83297608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83297608 size=52
    let mut pc: u32 = 0x83297608;
    'dispatch: loop {
        match pc {
            0x83297608 => {
    //   block [0x83297608..0x8329763C)
	// 83297608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329760C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83297610: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83297614: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 83297618: 386B4C60  addi r3, r11, 0x4c60
	ctx.r[3].s64 = ctx.r[11].s64 + 19552;
	// 8329761C: 4B972595  bl 0x82c09bb0
	ctx.lr = 0x83297620;
	sub_82C09BB0(ctx, base);
	// 83297620: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 83297624: 386A7408  addi r3, r10, 0x7408
	ctx.r[3].s64 = ctx.r[10].s64 + 29704;
	// 83297628: 4BA128F9  bl 0x82ca9f20
	ctx.lr = 0x8329762C;
	sub_82CA9F20(ctx, base);
	// 8329762C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83297630: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83297634: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83297638: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83297640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83297640 size=64
    let mut pc: u32 = 0x83297640;
    'dispatch: loop {
        match pc {
            0x83297640 => {
    //   block [0x83297640..0x83297680)
	// 83297640: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83297644: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83297648: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329764C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83297650: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83297654: 388B4A24  addi r4, r11, 0x4a24
	ctx.r[4].s64 = ctx.r[11].s64 + 18980;
	// 83297658: 386A4CB8  addi r3, r10, 0x4cb8
	ctx.r[3].s64 = ctx.r[10].s64 + 19640;
	// 8329765C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83297660: 4AF95871  bl 0x8222ced0
	ctx.lr = 0x83297664;
	sub_8222CED0(ctx, base);
	// 83297664: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83297668: 38697418  addi r3, r9, 0x7418
	ctx.r[3].s64 = ctx.r[9].s64 + 29720;
	// 8329766C: 4BA128B5  bl 0x82ca9f20
	ctx.lr = 0x83297670;
	sub_82CA9F20(ctx, base);
	// 83297670: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83297674: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83297678: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329767C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83297680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83297680 size=52
    let mut pc: u32 = 0x83297680;
    'dispatch: loop {
        match pc {
            0x83297680 => {
    //   block [0x83297680..0x832976B4)
	// 83297680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83297684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83297688: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329768C: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 83297690: 386B4CBC  addi r3, r11, 0x4cbc
	ctx.r[3].s64 = ctx.r[11].s64 + 19644;
	// 83297694: 4B08A915  bl 0x82321fa8
	ctx.lr = 0x83297698;
	sub_82321FA8(ctx, base);
	// 83297698: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 8329769C: 386A7428  addi r3, r10, 0x7428
	ctx.r[3].s64 = ctx.r[10].s64 + 29736;
	// 832976A0: 4BA12881  bl 0x82ca9f20
	ctx.lr = 0x832976A4;
	sub_82CA9F20(ctx, base);
	// 832976A4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832976A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832976AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832976B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832976B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832976B8 size=52
    let mut pc: u32 = 0x832976B8;
    'dispatch: loop {
        match pc {
            0x832976B8 => {
    //   block [0x832976B8..0x832976EC)
	// 832976B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832976BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832976C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832976C4: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832976C8: 386B4CC8  addi r3, r11, 0x4cc8
	ctx.r[3].s64 = ctx.r[11].s64 + 19656;
	// 832976CC: 4B780855  bl 0x82a17f20
	ctx.lr = 0x832976D0;
	sub_82A17F20(ctx, base);
	// 832976D0: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 832976D4: 386A7498  addi r3, r10, 0x7498
	ctx.r[3].s64 = ctx.r[10].s64 + 29848;
	// 832976D8: 4BA12849  bl 0x82ca9f20
	ctx.lr = 0x832976DC;
	sub_82CA9F20(ctx, base);
	// 832976DC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832976E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832976E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832976E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832976F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832976F0 size=52
    let mut pc: u32 = 0x832976F0;
    'dispatch: loop {
        match pc {
            0x832976F0 => {
    //   block [0x832976F0..0x83297724)
	// 832976F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832976F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832976F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832976FC: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 83297700: 386B4CD4  addi r3, r11, 0x4cd4
	ctx.r[3].s64 = ctx.r[11].s64 + 19668;
	// 83297704: 4B9737D5  bl 0x82c0aed8
	ctx.lr = 0x83297708;
	sub_82C0AED8(ctx, base);
	// 83297708: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 8329770C: 386A7508  addi r3, r10, 0x7508
	ctx.r[3].s64 = ctx.r[10].s64 + 29960;
	// 83297710: 4BA12811  bl 0x82ca9f20
	ctx.lr = 0x83297714;
	sub_82CA9F20(ctx, base);
	// 83297714: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83297718: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329771C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83297720: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83297728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83297728 size=52
    let mut pc: u32 = 0x83297728;
    'dispatch: loop {
        match pc {
            0x83297728 => {
    //   block [0x83297728..0x8329775C)
	// 83297728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329772C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83297730: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83297734: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 83297738: 386B4CE0  addi r3, r11, 0x4ce0
	ctx.r[3].s64 = ctx.r[11].s64 + 19680;
	// 8329773C: 4B216F25  bl 0x824ae660
	ctx.lr = 0x83297740;
	sub_824AE660(ctx, base);
	// 83297740: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 83297744: 386A7578  addi r3, r10, 0x7578
	ctx.r[3].s64 = ctx.r[10].s64 + 30072;
	// 83297748: 4BA127D9  bl 0x82ca9f20
	ctx.lr = 0x8329774C;
	sub_82CA9F20(ctx, base);
	// 8329774C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83297750: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83297754: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83297758: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83297760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83297760 size=104
    let mut pc: u32 = 0x83297760;
    'dispatch: loop {
        match pc {
            0x83297760 => {
    //   block [0x83297760..0x83297780)
	// 83297760: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83297764: 4BA11CA9  bl 0x82ca940c
	ctx.lr = 0x83297768;
	sub_82CA93D0(ctx, base);
	// 83297768: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329776C: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 83297770: 3BC00004  li r30, 4
	ctx.r[30].s64 = 4;
	// 83297774: 396B4CF4  addi r11, r11, 0x4cf4
	ctx.r[11].s64 = ctx.r[11].s64 + 19700;
	// 83297778: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8329777C: 3BEB0008  addi r31, r11, 8
	ctx.r[31].s64 = ctx.r[11].s64 + 8;
	pc = 0x83297780; continue 'dispatch;
            }
            0x83297780 => {
    //   block [0x83297780..0x83297794)
	// 83297780: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 83297784: 4AF87AD5  bl 0x8221f258
	ctx.lr = 0x83297788;
	sub_8221F258(ctx, base);
	// 83297788: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8329778C: 419A0008  beq cr6, 0x83297794
	if ctx.cr[6].eq {
	pc = 0x83297794; continue 'dispatch;
	}
	// 83297790: 90630000  stw r3, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	pc = 0x83297794; continue 'dispatch;
            }
            0x83297794 => {
    //   block [0x83297794..0x832977A0)
	// 83297794: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83297798: 41820008  beq 0x832977a0
	if ctx.cr[0].eq {
	pc = 0x832977A0; continue 'dispatch;
	}
	// 8329779C: 906B0000  stw r3, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	pc = 0x832977A0; continue 'dispatch;
            }
            0x832977A0 => {
    //   block [0x832977A0..0x832977C8)
	// 832977A0: 907FFFFC  stw r3, -4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-4 as u32), ctx.r[3].u32 ) };
	// 832977A4: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832977A8: 93BF0000  stw r29, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 832977AC: 3BFF000C  addi r31, r31, 0xc
	ctx.r[31].s64 = ctx.r[31].s64 + 12;
	// 832977B0: 4080FFD0  bge 0x83297780
	if !ctx.cr[0].lt {
	pc = 0x83297780; continue 'dispatch;
	}
	// 832977B4: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 832977B8: 386B7688  addi r3, r11, 0x7688
	ctx.r[3].s64 = ctx.r[11].s64 + 30344;
	// 832977BC: 4BA12765  bl 0x82ca9f20
	ctx.lr = 0x832977C0;
	sub_82CA9F20(ctx, base);
	// 832977C0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832977C4: 4BA11C98  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832977C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832977C8 size=88
    let mut pc: u32 = 0x832977C8;
    'dispatch: loop {
        match pc {
            0x832977C8 => {
    //   block [0x832977C8..0x832977E8)
	// 832977C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832977CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832977D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832977D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832977D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832977DC: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832977E0: 3BE00004  li r31, 4
	ctx.r[31].s64 = 4;
	// 832977E4: 3BCB4D30  addi r30, r11, 0x4d30
	ctx.r[30].s64 = ctx.r[11].s64 + 19760;
	pc = 0x832977E8; continue 'dispatch;
            }
            0x832977E8 => {
    //   block [0x832977E8..0x83297820)
	// 832977E8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832977EC: 4B8A60C5  bl 0x82b3d8b0
	ctx.lr = 0x832977F0;
	sub_82B3D8B0(ctx, base);
	// 832977F0: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 832977F4: 3BDE001C  addi r30, r30, 0x1c
	ctx.r[30].s64 = ctx.r[30].s64 + 28;
	// 832977F8: 4080FFF0  bge 0x832977e8
	if !ctx.cr[0].lt {
	pc = 0x832977E8; continue 'dispatch;
	}
	// 832977FC: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83297800: 386B75E8  addi r3, r11, 0x75e8
	ctx.r[3].s64 = ctx.r[11].s64 + 30184;
	// 83297804: 4BA1271D  bl 0x82ca9f20
	ctx.lr = 0x83297808;
	sub_82CA9F20(ctx, base);
	// 83297808: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8329780C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83297810: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83297814: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83297818: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8329781C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83297820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83297820 size=108
    let mut pc: u32 = 0x83297820;
    'dispatch: loop {
        match pc {
            0x83297820 => {
    //   block [0x83297820..0x8329784C)
	// 83297820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83297824: 4BA11BE1  bl 0x82ca9404
	ctx.lr = 0x83297828;
	sub_82CA93D0(ctx, base);
	// 83297828: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329782C: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 83297830: 3BA00004  li r29, 4
	ctx.r[29].s64 = 4;
	// 83297834: 396B4DBC  addi r11, r11, 0x4dbc
	ctx.r[11].s64 = ctx.r[11].s64 + 19900;
	// 83297838: 3B600005  li r27, 5
	ctx.r[27].s64 = 5;
	// 8329783C: 3BEB0018  addi r31, r11, 0x18
	ctx.r[31].s64 = ctx.r[11].s64 + 24;
	// 83297840: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83297844: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83297848: 3B8BA954  addi r28, r11, -0x56ac
	ctx.r[28].s64 = ctx.r[11].s64 + -22188;
	pc = 0x8329784C; continue 'dispatch;
            }
            0x8329784C => {
    //   block [0x8329784C..0x8329788C)
	// 8329784C: 387FFFE8  addi r3, r31, -0x18
	ctx.r[3].s64 = ctx.r[31].s64 + -24;
	// 83297850: 3C800001  lis r4, 1
	ctx.r[4].s64 = 65536;
	// 83297854: 4B8A60E5  bl 0x82b3d938
	ctx.lr = 0x83297858;
	sub_82B3D938(ctx, base);
	// 83297858: 939FFFE8  stw r28, -0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-24 as u32), ctx.r[28].u32 ) };
	// 8329785C: 937FFFFC  stw r27, -4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-4 as u32), ctx.r[27].u32 ) };
	// 83297860: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 83297864: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 83297868: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 8329786C: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 83297870: 3BFF0024  addi r31, r31, 0x24
	ctx.r[31].s64 = ctx.r[31].s64 + 36;
	// 83297874: 4080FFD8  bge 0x8329784c
	if !ctx.cr[0].lt {
	pc = 0x8329784C; continue 'dispatch;
	}
	// 83297878: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8329787C: 386B7638  addi r3, r11, 0x7638
	ctx.r[3].s64 = ctx.r[11].s64 + 30264;
	// 83297880: 4BA126A1  bl 0x82ca9f20
	ctx.lr = 0x83297884;
	sub_82CA9F20(ctx, base);
	// 83297884: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83297888: 4BA11BCC  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83297890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83297890 size=80
    let mut pc: u32 = 0x83297890;
    'dispatch: loop {
        match pc {
            0x83297890 => {
    //   block [0x83297890..0x832978E0)
	// 83297890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83297894: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83297898: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8329789C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832978A0: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832978A4: 3BEB4E74  addi r31, r11, 0x4e74
	ctx.r[31].s64 = ctx.r[11].s64 + 20084;
	// 832978A8: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 832978AC: 4B08A6FD  bl 0x82321fa8
	ctx.lr = 0x832978B0;
	sub_82321FA8(ctx, base);
	// 832978B0: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 832978B4: 4B8A5FFD  bl 0x82b3d8b0
	ctx.lr = 0x832978B8;
	sub_82B3D8B0(ctx, base);
	// 832978B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832978BC: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 832978C0: 997F0038  stb r11, 0x38(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u8 ) };
	// 832978C4: 386A7708  addi r3, r10, 0x7708
	ctx.r[3].s64 = ctx.r[10].s64 + 30472;
	// 832978C8: 4BA12659  bl 0x82ca9f20
	ctx.lr = 0x832978CC;
	sub_82CA9F20(ctx, base);
	// 832978CC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832978D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832978D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832978D8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832978DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832978E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832978E0 size=80
    let mut pc: u32 = 0x832978E0;
    'dispatch: loop {
        match pc {
            0x832978E0 => {
    //   block [0x832978E0..0x83297930)
	// 832978E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832978E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832978E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832978EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832978F0: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832978F4: 3BEB4EB0  addi r31, r11, 0x4eb0
	ctx.r[31].s64 = ctx.r[11].s64 + 20144;
	// 832978F8: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 832978FC: 4B08A6AD  bl 0x82321fa8
	ctx.lr = 0x83297900;
	sub_82321FA8(ctx, base);
	// 83297900: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 83297904: 4B8A5FAD  bl 0x82b3d8b0
	ctx.lr = 0x83297908;
	sub_82B3D8B0(ctx, base);
	// 83297908: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329790C: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 83297910: 997F0038  stb r11, 0x38(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u8 ) };
	// 83297914: 386A7718  addi r3, r10, 0x7718
	ctx.r[3].s64 = ctx.r[10].s64 + 30488;
	// 83297918: 4BA12609  bl 0x82ca9f20
	ctx.lr = 0x8329791C;
	sub_82CA9F20(ctx, base);
	// 8329791C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83297920: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83297924: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83297928: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8329792C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83297930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83297930 size=64
    let mut pc: u32 = 0x83297930;
    'dispatch: loop {
        match pc {
            0x83297930 => {
    //   block [0x83297930..0x83297970)
	// 83297930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83297934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83297938: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329793C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83297940: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83297944: 388BB3D8  addi r4, r11, -0x4c28
	ctx.r[4].s64 = ctx.r[11].s64 + -19496;
	// 83297948: 386A4EF8  addi r3, r10, 0x4ef8
	ctx.r[3].s64 = ctx.r[10].s64 + 20216;
	// 8329794C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83297950: 4AF95581  bl 0x8222ced0
	ctx.lr = 0x83297954;
	sub_8222CED0(ctx, base);
	// 83297954: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83297958: 38697728  addi r3, r9, 0x7728
	ctx.r[3].s64 = ctx.r[9].s64 + 30504;
	// 8329795C: 4BA125C5  bl 0x82ca9f20
	ctx.lr = 0x83297960;
	sub_82CA9F20(ctx, base);
	// 83297960: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83297964: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83297968: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329796C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83297970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83297970 size=64
    let mut pc: u32 = 0x83297970;
    'dispatch: loop {
        match pc {
            0x83297970 => {
    //   block [0x83297970..0x832979B0)
	// 83297970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83297974: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83297978: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329797C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83297980: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83297984: 388BB3E8  addi r4, r11, -0x4c18
	ctx.r[4].s64 = ctx.r[11].s64 + -19480;
	// 83297988: 386A4EFC  addi r3, r10, 0x4efc
	ctx.r[3].s64 = ctx.r[10].s64 + 20220;
	// 8329798C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83297990: 4AF95541  bl 0x8222ced0
	ctx.lr = 0x83297994;
	sub_8222CED0(ctx, base);
	// 83297994: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83297998: 38697738  addi r3, r9, 0x7738
	ctx.r[3].s64 = ctx.r[9].s64 + 30520;
	// 8329799C: 4BA12585  bl 0x82ca9f20
	ctx.lr = 0x832979A0;
	sub_82CA9F20(ctx, base);
	// 832979A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832979A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832979A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832979AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832979B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832979B0 size=64
    let mut pc: u32 = 0x832979B0;
    'dispatch: loop {
        match pc {
            0x832979B0 => {
    //   block [0x832979B0..0x832979F0)
	// 832979B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832979B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832979B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832979BC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 832979C0: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 832979C4: 388BB534  addi r4, r11, -0x4acc
	ctx.r[4].s64 = ctx.r[11].s64 + -19148;
	// 832979C8: 386A4F00  addi r3, r10, 0x4f00
	ctx.r[3].s64 = ctx.r[10].s64 + 20224;
	// 832979CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832979D0: 4AF95501  bl 0x8222ced0
	ctx.lr = 0x832979D4;
	sub_8222CED0(ctx, base);
	// 832979D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832979D8: 38697748  addi r3, r9, 0x7748
	ctx.r[3].s64 = ctx.r[9].s64 + 30536;
	// 832979DC: 4BA12545  bl 0x82ca9f20
	ctx.lr = 0x832979E0;
	sub_82CA9F20(ctx, base);
	// 832979E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832979E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832979E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832979EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832979F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832979F0 size=64
    let mut pc: u32 = 0x832979F0;
    'dispatch: loop {
        match pc {
            0x832979F0 => {
    //   block [0x832979F0..0x83297A30)
	// 832979F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832979F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832979F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832979FC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83297A00: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83297A04: 388BB540  addi r4, r11, -0x4ac0
	ctx.r[4].s64 = ctx.r[11].s64 + -19136;
	// 83297A08: 386A4F04  addi r3, r10, 0x4f04
	ctx.r[3].s64 = ctx.r[10].s64 + 20228;
	// 83297A0C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83297A10: 4AF954C1  bl 0x8222ced0
	ctx.lr = 0x83297A14;
	sub_8222CED0(ctx, base);
	// 83297A14: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83297A18: 38697758  addi r3, r9, 0x7758
	ctx.r[3].s64 = ctx.r[9].s64 + 30552;
	// 83297A1C: 4BA12505  bl 0x82ca9f20
	ctx.lr = 0x83297A20;
	sub_82CA9F20(ctx, base);
	// 83297A20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83297A24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83297A28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83297A2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83297A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83297A30 size=64
    let mut pc: u32 = 0x83297A30;
    'dispatch: loop {
        match pc {
            0x83297A30 => {
    //   block [0x83297A30..0x83297A70)
	// 83297A30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83297A34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83297A38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83297A3C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83297A40: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83297A44: 388BB780  addi r4, r11, -0x4880
	ctx.r[4].s64 = ctx.r[11].s64 + -18560;
	// 83297A48: 386A4F08  addi r3, r10, 0x4f08
	ctx.r[3].s64 = ctx.r[10].s64 + 20232;
	// 83297A4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83297A50: 4AF95481  bl 0x8222ced0
	ctx.lr = 0x83297A54;
	sub_8222CED0(ctx, base);
	// 83297A54: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83297A58: 38697768  addi r3, r9, 0x7768
	ctx.r[3].s64 = ctx.r[9].s64 + 30568;
	// 83297A5C: 4BA124C5  bl 0x82ca9f20
	ctx.lr = 0x83297A60;
	sub_82CA9F20(ctx, base);
	// 83297A60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83297A64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83297A68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83297A6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83297A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83297A70 size=64
    let mut pc: u32 = 0x83297A70;
    'dispatch: loop {
        match pc {
            0x83297A70 => {
    //   block [0x83297A70..0x83297AB0)
	// 83297A70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83297A74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83297A78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83297A7C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83297A80: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83297A84: 388BB790  addi r4, r11, -0x4870
	ctx.r[4].s64 = ctx.r[11].s64 + -18544;
	// 83297A88: 386A4F0C  addi r3, r10, 0x4f0c
	ctx.r[3].s64 = ctx.r[10].s64 + 20236;
	// 83297A8C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83297A90: 4AF95441  bl 0x8222ced0
	ctx.lr = 0x83297A94;
	sub_8222CED0(ctx, base);
	// 83297A94: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83297A98: 38697778  addi r3, r9, 0x7778
	ctx.r[3].s64 = ctx.r[9].s64 + 30584;
	// 83297A9C: 4BA12485  bl 0x82ca9f20
	ctx.lr = 0x83297AA0;
	sub_82CA9F20(ctx, base);
	// 83297AA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83297AA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83297AA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83297AAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83297AB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83297AB0 size=64
    let mut pc: u32 = 0x83297AB0;
    'dispatch: loop {
        match pc {
            0x83297AB0 => {
    //   block [0x83297AB0..0x83297AF0)
	// 83297AB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83297AB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83297AB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83297ABC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83297AC0: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83297AC4: 388BB79C  addi r4, r11, -0x4864
	ctx.r[4].s64 = ctx.r[11].s64 + -18532;
	// 83297AC8: 386A4F10  addi r3, r10, 0x4f10
	ctx.r[3].s64 = ctx.r[10].s64 + 20240;
	// 83297ACC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83297AD0: 4AF95401  bl 0x8222ced0
	ctx.lr = 0x83297AD4;
	sub_8222CED0(ctx, base);
	// 83297AD4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83297AD8: 38697788  addi r3, r9, 0x7788
	ctx.r[3].s64 = ctx.r[9].s64 + 30600;
	// 83297ADC: 4BA12445  bl 0x82ca9f20
	ctx.lr = 0x83297AE0;
	sub_82CA9F20(ctx, base);
	// 83297AE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83297AE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83297AE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83297AEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83297AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83297AF0 size=64
    let mut pc: u32 = 0x83297AF0;
    'dispatch: loop {
        match pc {
            0x83297AF0 => {
    //   block [0x83297AF0..0x83297B30)
	// 83297AF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83297AF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83297AF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83297AFC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83297B00: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83297B04: 388BB884  addi r4, r11, -0x477c
	ctx.r[4].s64 = ctx.r[11].s64 + -18300;
	// 83297B08: 386A4F14  addi r3, r10, 0x4f14
	ctx.r[3].s64 = ctx.r[10].s64 + 20244;
	// 83297B0C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83297B10: 4AF953C1  bl 0x8222ced0
	ctx.lr = 0x83297B14;
	sub_8222CED0(ctx, base);
	// 83297B14: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83297B18: 38697798  addi r3, r9, 0x7798
	ctx.r[3].s64 = ctx.r[9].s64 + 30616;
	// 83297B1C: 4BA12405  bl 0x82ca9f20
	ctx.lr = 0x83297B20;
	sub_82CA9F20(ctx, base);
	// 83297B20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83297B24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83297B28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83297B2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83297B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83297B30 size=64
    let mut pc: u32 = 0x83297B30;
    'dispatch: loop {
        match pc {
            0x83297B30 => {
    //   block [0x83297B30..0x83297B70)
	// 83297B30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83297B34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83297B38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83297B3C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83297B40: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83297B44: 388BB89C  addi r4, r11, -0x4764
	ctx.r[4].s64 = ctx.r[11].s64 + -18276;
	// 83297B48: 386A4F18  addi r3, r10, 0x4f18
	ctx.r[3].s64 = ctx.r[10].s64 + 20248;
	// 83297B4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83297B50: 4AF95381  bl 0x8222ced0
	ctx.lr = 0x83297B54;
	sub_8222CED0(ctx, base);
	// 83297B54: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83297B58: 386977A8  addi r3, r9, 0x77a8
	ctx.r[3].s64 = ctx.r[9].s64 + 30632;
	// 83297B5C: 4BA123C5  bl 0x82ca9f20
	ctx.lr = 0x83297B60;
	sub_82CA9F20(ctx, base);
	// 83297B60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83297B64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83297B68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83297B6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83297B70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83297B70 size=64
    let mut pc: u32 = 0x83297B70;
    'dispatch: loop {
        match pc {
            0x83297B70 => {
    //   block [0x83297B70..0x83297BB0)
	// 83297B70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83297B74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83297B78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83297B7C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83297B80: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83297B84: 388BB8AC  addi r4, r11, -0x4754
	ctx.r[4].s64 = ctx.r[11].s64 + -18260;
	// 83297B88: 386A4F1C  addi r3, r10, 0x4f1c
	ctx.r[3].s64 = ctx.r[10].s64 + 20252;
	// 83297B8C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83297B90: 4AF95341  bl 0x8222ced0
	ctx.lr = 0x83297B94;
	sub_8222CED0(ctx, base);
	// 83297B94: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83297B98: 386977B8  addi r3, r9, 0x77b8
	ctx.r[3].s64 = ctx.r[9].s64 + 30648;
	// 83297B9C: 4BA12385  bl 0x82ca9f20
	ctx.lr = 0x83297BA0;
	sub_82CA9F20(ctx, base);
	// 83297BA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83297BA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83297BA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83297BAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83297BB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83297BB0 size=64
    let mut pc: u32 = 0x83297BB0;
    'dispatch: loop {
        match pc {
            0x83297BB0 => {
    //   block [0x83297BB0..0x83297BF0)
	// 83297BB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83297BB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83297BB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83297BBC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83297BC0: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83297BC4: 388BB9F8  addi r4, r11, -0x4608
	ctx.r[4].s64 = ctx.r[11].s64 + -17928;
	// 83297BC8: 386A4F20  addi r3, r10, 0x4f20
	ctx.r[3].s64 = ctx.r[10].s64 + 20256;
	// 83297BCC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83297BD0: 4AF95301  bl 0x8222ced0
	ctx.lr = 0x83297BD4;
	sub_8222CED0(ctx, base);
	// 83297BD4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83297BD8: 386977C8  addi r3, r9, 0x77c8
	ctx.r[3].s64 = ctx.r[9].s64 + 30664;
	// 83297BDC: 4BA12345  bl 0x82ca9f20
	ctx.lr = 0x83297BE0;
	sub_82CA9F20(ctx, base);
	// 83297BE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83297BE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83297BE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83297BEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83297BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83297BF0 size=64
    let mut pc: u32 = 0x83297BF0;
    'dispatch: loop {
        match pc {
            0x83297BF0 => {
    //   block [0x83297BF0..0x83297C30)
	// 83297BF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83297BF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83297BF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83297BFC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83297C00: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83297C04: 388BBA10  addi r4, r11, -0x45f0
	ctx.r[4].s64 = ctx.r[11].s64 + -17904;
	// 83297C08: 386A4F24  addi r3, r10, 0x4f24
	ctx.r[3].s64 = ctx.r[10].s64 + 20260;
	// 83297C0C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83297C10: 4AF952C1  bl 0x8222ced0
	ctx.lr = 0x83297C14;
	sub_8222CED0(ctx, base);
	// 83297C14: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83297C18: 386977D8  addi r3, r9, 0x77d8
	ctx.r[3].s64 = ctx.r[9].s64 + 30680;
	// 83297C1C: 4BA12305  bl 0x82ca9f20
	ctx.lr = 0x83297C20;
	sub_82CA9F20(ctx, base);
	// 83297C20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83297C24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83297C28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83297C2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83297C30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83297C30 size=64
    let mut pc: u32 = 0x83297C30;
    'dispatch: loop {
        match pc {
            0x83297C30 => {
    //   block [0x83297C30..0x83297C70)
	// 83297C30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83297C34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83297C38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83297C3C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83297C40: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83297C44: 388BBA20  addi r4, r11, -0x45e0
	ctx.r[4].s64 = ctx.r[11].s64 + -17888;
	// 83297C48: 386A4F28  addi r3, r10, 0x4f28
	ctx.r[3].s64 = ctx.r[10].s64 + 20264;
	// 83297C4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83297C50: 4AF95281  bl 0x8222ced0
	ctx.lr = 0x83297C54;
	sub_8222CED0(ctx, base);
	// 83297C54: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83297C58: 386977E8  addi r3, r9, 0x77e8
	ctx.r[3].s64 = ctx.r[9].s64 + 30696;
	// 83297C5C: 4BA122C5  bl 0x82ca9f20
	ctx.lr = 0x83297C60;
	sub_82CA9F20(ctx, base);
	// 83297C60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83297C64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83297C68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83297C6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83297C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83297C70 size=64
    let mut pc: u32 = 0x83297C70;
    'dispatch: loop {
        match pc {
            0x83297C70 => {
    //   block [0x83297C70..0x83297CB0)
	// 83297C70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83297C74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83297C78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83297C7C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83297C80: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83297C84: 388BBA6C  addi r4, r11, -0x4594
	ctx.r[4].s64 = ctx.r[11].s64 + -17812;
	// 83297C88: 386A4F2C  addi r3, r10, 0x4f2c
	ctx.r[3].s64 = ctx.r[10].s64 + 20268;
	// 83297C8C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83297C90: 4AF95241  bl 0x8222ced0
	ctx.lr = 0x83297C94;
	sub_8222CED0(ctx, base);
	// 83297C94: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83297C98: 386977F8  addi r3, r9, 0x77f8
	ctx.r[3].s64 = ctx.r[9].s64 + 30712;
	// 83297C9C: 4BA12285  bl 0x82ca9f20
	ctx.lr = 0x83297CA0;
	sub_82CA9F20(ctx, base);
	// 83297CA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83297CA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83297CA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83297CAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83297CB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83297CB0 size=64
    let mut pc: u32 = 0x83297CB0;
    'dispatch: loop {
        match pc {
            0x83297CB0 => {
    //   block [0x83297CB0..0x83297CF0)
	// 83297CB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83297CB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83297CB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83297CBC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83297CC0: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83297CC4: 388BBA84  addi r4, r11, -0x457c
	ctx.r[4].s64 = ctx.r[11].s64 + -17788;
	// 83297CC8: 386A4F30  addi r3, r10, 0x4f30
	ctx.r[3].s64 = ctx.r[10].s64 + 20272;
	// 83297CCC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83297CD0: 4AF95201  bl 0x8222ced0
	ctx.lr = 0x83297CD4;
	sub_8222CED0(ctx, base);
	// 83297CD4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83297CD8: 38697808  addi r3, r9, 0x7808
	ctx.r[3].s64 = ctx.r[9].s64 + 30728;
	// 83297CDC: 4BA12245  bl 0x82ca9f20
	ctx.lr = 0x83297CE0;
	sub_82CA9F20(ctx, base);
	// 83297CE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83297CE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83297CE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83297CEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83297CF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83297CF0 size=64
    let mut pc: u32 = 0x83297CF0;
    'dispatch: loop {
        match pc {
            0x83297CF0 => {
    //   block [0x83297CF0..0x83297D30)
	// 83297CF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83297CF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83297CF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83297CFC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83297D00: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83297D04: 388BBA98  addi r4, r11, -0x4568
	ctx.r[4].s64 = ctx.r[11].s64 + -17768;
	// 83297D08: 386A4F34  addi r3, r10, 0x4f34
	ctx.r[3].s64 = ctx.r[10].s64 + 20276;
	// 83297D0C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83297D10: 4AF951C1  bl 0x8222ced0
	ctx.lr = 0x83297D14;
	sub_8222CED0(ctx, base);
	// 83297D14: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83297D18: 38697818  addi r3, r9, 0x7818
	ctx.r[3].s64 = ctx.r[9].s64 + 30744;
	// 83297D1C: 4BA12205  bl 0x82ca9f20
	ctx.lr = 0x83297D20;
	sub_82CA9F20(ctx, base);
	// 83297D20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83297D24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83297D28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83297D2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83297D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83297D30 size=64
    let mut pc: u32 = 0x83297D30;
    'dispatch: loop {
        match pc {
            0x83297D30 => {
    //   block [0x83297D30..0x83297D70)
	// 83297D30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83297D34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83297D38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83297D3C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83297D40: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83297D44: 388BBB60  addi r4, r11, -0x44a0
	ctx.r[4].s64 = ctx.r[11].s64 + -17568;
	// 83297D48: 386A4F38  addi r3, r10, 0x4f38
	ctx.r[3].s64 = ctx.r[10].s64 + 20280;
	// 83297D4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83297D50: 4AF95181  bl 0x8222ced0
	ctx.lr = 0x83297D54;
	sub_8222CED0(ctx, base);
	// 83297D54: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83297D58: 38697828  addi r3, r9, 0x7828
	ctx.r[3].s64 = ctx.r[9].s64 + 30760;
	// 83297D5C: 4BA121C5  bl 0x82ca9f20
	ctx.lr = 0x83297D60;
	sub_82CA9F20(ctx, base);
	// 83297D60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83297D64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83297D68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83297D6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83297D70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83297D70 size=64
    let mut pc: u32 = 0x83297D70;
    'dispatch: loop {
        match pc {
            0x83297D70 => {
    //   block [0x83297D70..0x83297DB0)
	// 83297D70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83297D74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83297D78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83297D7C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83297D80: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83297D84: 388BBB70  addi r4, r11, -0x4490
	ctx.r[4].s64 = ctx.r[11].s64 + -17552;
	// 83297D88: 386A4F3C  addi r3, r10, 0x4f3c
	ctx.r[3].s64 = ctx.r[10].s64 + 20284;
	// 83297D8C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83297D90: 4AF95141  bl 0x8222ced0
	ctx.lr = 0x83297D94;
	sub_8222CED0(ctx, base);
	// 83297D94: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83297D98: 38697838  addi r3, r9, 0x7838
	ctx.r[3].s64 = ctx.r[9].s64 + 30776;
	// 83297D9C: 4BA12185  bl 0x82ca9f20
	ctx.lr = 0x83297DA0;
	sub_82CA9F20(ctx, base);
	// 83297DA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83297DA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83297DA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83297DAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83297DB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83297DB0 size=64
    let mut pc: u32 = 0x83297DB0;
    'dispatch: loop {
        match pc {
            0x83297DB0 => {
    //   block [0x83297DB0..0x83297DF0)
	// 83297DB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83297DB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83297DB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83297DBC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83297DC0: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83297DC4: 388BBB7C  addi r4, r11, -0x4484
	ctx.r[4].s64 = ctx.r[11].s64 + -17540;
	// 83297DC8: 386A4F40  addi r3, r10, 0x4f40
	ctx.r[3].s64 = ctx.r[10].s64 + 20288;
	// 83297DCC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83297DD0: 4AF95101  bl 0x8222ced0
	ctx.lr = 0x83297DD4;
	sub_8222CED0(ctx, base);
	// 83297DD4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83297DD8: 38697848  addi r3, r9, 0x7848
	ctx.r[3].s64 = ctx.r[9].s64 + 30792;
	// 83297DDC: 4BA12145  bl 0x82ca9f20
	ctx.lr = 0x83297DE0;
	sub_82CA9F20(ctx, base);
	// 83297DE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83297DE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83297DE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83297DEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83297DF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83297DF0 size=64
    let mut pc: u32 = 0x83297DF0;
    'dispatch: loop {
        match pc {
            0x83297DF0 => {
    //   block [0x83297DF0..0x83297E30)
	// 83297DF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83297DF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83297DF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83297DFC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83297E00: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83297E04: 388BBC24  addi r4, r11, -0x43dc
	ctx.r[4].s64 = ctx.r[11].s64 + -17372;
	// 83297E08: 386A4F44  addi r3, r10, 0x4f44
	ctx.r[3].s64 = ctx.r[10].s64 + 20292;
	// 83297E0C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83297E10: 4AF950C1  bl 0x8222ced0
	ctx.lr = 0x83297E14;
	sub_8222CED0(ctx, base);
	// 83297E14: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83297E18: 38697858  addi r3, r9, 0x7858
	ctx.r[3].s64 = ctx.r[9].s64 + 30808;
	// 83297E1C: 4BA12105  bl 0x82ca9f20
	ctx.lr = 0x83297E20;
	sub_82CA9F20(ctx, base);
	// 83297E20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83297E24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83297E28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83297E2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83297E30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83297E30 size=64
    let mut pc: u32 = 0x83297E30;
    'dispatch: loop {
        match pc {
            0x83297E30 => {
    //   block [0x83297E30..0x83297E70)
	// 83297E30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83297E34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83297E38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83297E3C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83297E40: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83297E44: 388BBC38  addi r4, r11, -0x43c8
	ctx.r[4].s64 = ctx.r[11].s64 + -17352;
	// 83297E48: 386A4F48  addi r3, r10, 0x4f48
	ctx.r[3].s64 = ctx.r[10].s64 + 20296;
	// 83297E4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83297E50: 4AF95081  bl 0x8222ced0
	ctx.lr = 0x83297E54;
	sub_8222CED0(ctx, base);
	// 83297E54: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83297E58: 38697868  addi r3, r9, 0x7868
	ctx.r[3].s64 = ctx.r[9].s64 + 30824;
	// 83297E5C: 4BA120C5  bl 0x82ca9f20
	ctx.lr = 0x83297E60;
	sub_82CA9F20(ctx, base);
	// 83297E60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83297E64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83297E68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83297E6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83297E70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83297E70 size=64
    let mut pc: u32 = 0x83297E70;
    'dispatch: loop {
        match pc {
            0x83297E70 => {
    //   block [0x83297E70..0x83297EB0)
	// 83297E70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83297E74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83297E78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83297E7C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83297E80: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83297E84: 388BBC48  addi r4, r11, -0x43b8
	ctx.r[4].s64 = ctx.r[11].s64 + -17336;
	// 83297E88: 386A4F4C  addi r3, r10, 0x4f4c
	ctx.r[3].s64 = ctx.r[10].s64 + 20300;
	// 83297E8C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83297E90: 4AF95041  bl 0x8222ced0
	ctx.lr = 0x83297E94;
	sub_8222CED0(ctx, base);
	// 83297E94: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83297E98: 38697878  addi r3, r9, 0x7878
	ctx.r[3].s64 = ctx.r[9].s64 + 30840;
	// 83297E9C: 4BA12085  bl 0x82ca9f20
	ctx.lr = 0x83297EA0;
	sub_82CA9F20(ctx, base);
	// 83297EA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83297EA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83297EA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83297EAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83297EB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83297EB0 size=64
    let mut pc: u32 = 0x83297EB0;
    'dispatch: loop {
        match pc {
            0x83297EB0 => {
    //   block [0x83297EB0..0x83297EF0)
	// 83297EB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83297EB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83297EB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83297EBC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83297EC0: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83297EC4: 388BBD4C  addi r4, r11, -0x42b4
	ctx.r[4].s64 = ctx.r[11].s64 + -17076;
	// 83297EC8: 386A4F50  addi r3, r10, 0x4f50
	ctx.r[3].s64 = ctx.r[10].s64 + 20304;
	// 83297ECC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83297ED0: 4AF95001  bl 0x8222ced0
	ctx.lr = 0x83297ED4;
	sub_8222CED0(ctx, base);
	// 83297ED4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83297ED8: 38697888  addi r3, r9, 0x7888
	ctx.r[3].s64 = ctx.r[9].s64 + 30856;
	// 83297EDC: 4BA12045  bl 0x82ca9f20
	ctx.lr = 0x83297EE0;
	sub_82CA9F20(ctx, base);
	// 83297EE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83297EE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83297EE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83297EEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83297EF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83297EF0 size=64
    let mut pc: u32 = 0x83297EF0;
    'dispatch: loop {
        match pc {
            0x83297EF0 => {
    //   block [0x83297EF0..0x83297F30)
	// 83297EF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83297EF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83297EF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83297EFC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83297F00: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83297F04: 388BBD64  addi r4, r11, -0x429c
	ctx.r[4].s64 = ctx.r[11].s64 + -17052;
	// 83297F08: 386A4F54  addi r3, r10, 0x4f54
	ctx.r[3].s64 = ctx.r[10].s64 + 20308;
	// 83297F0C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83297F10: 4AF94FC1  bl 0x8222ced0
	ctx.lr = 0x83297F14;
	sub_8222CED0(ctx, base);
	// 83297F14: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83297F18: 38697898  addi r3, r9, 0x7898
	ctx.r[3].s64 = ctx.r[9].s64 + 30872;
	// 83297F1C: 4BA12005  bl 0x82ca9f20
	ctx.lr = 0x83297F20;
	sub_82CA9F20(ctx, base);
	// 83297F20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83297F24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83297F28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83297F2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83297F30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83297F30 size=64
    let mut pc: u32 = 0x83297F30;
    'dispatch: loop {
        match pc {
            0x83297F30 => {
    //   block [0x83297F30..0x83297F70)
	// 83297F30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83297F34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83297F38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83297F3C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83297F40: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83297F44: 388BBD64  addi r4, r11, -0x429c
	ctx.r[4].s64 = ctx.r[11].s64 + -17052;
	// 83297F48: 386A4F58  addi r3, r10, 0x4f58
	ctx.r[3].s64 = ctx.r[10].s64 + 20312;
	// 83297F4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83297F50: 4AF94F81  bl 0x8222ced0
	ctx.lr = 0x83297F54;
	sub_8222CED0(ctx, base);
	// 83297F54: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83297F58: 386978A8  addi r3, r9, 0x78a8
	ctx.r[3].s64 = ctx.r[9].s64 + 30888;
	// 83297F5C: 4BA11FC5  bl 0x82ca9f20
	ctx.lr = 0x83297F60;
	sub_82CA9F20(ctx, base);
	// 83297F60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83297F64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83297F68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83297F6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83297F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83297F70 size=64
    let mut pc: u32 = 0x83297F70;
    'dispatch: loop {
        match pc {
            0x83297F70 => {
    //   block [0x83297F70..0x83297FB0)
	// 83297F70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83297F74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83297F78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83297F7C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83297F80: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83297F84: 388BBEC4  addi r4, r11, -0x413c
	ctx.r[4].s64 = ctx.r[11].s64 + -16700;
	// 83297F88: 386A4F5C  addi r3, r10, 0x4f5c
	ctx.r[3].s64 = ctx.r[10].s64 + 20316;
	// 83297F8C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83297F90: 4AF94F41  bl 0x8222ced0
	ctx.lr = 0x83297F94;
	sub_8222CED0(ctx, base);
	// 83297F94: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83297F98: 386978B8  addi r3, r9, 0x78b8
	ctx.r[3].s64 = ctx.r[9].s64 + 30904;
	// 83297F9C: 4BA11F85  bl 0x82ca9f20
	ctx.lr = 0x83297FA0;
	sub_82CA9F20(ctx, base);
	// 83297FA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83297FA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83297FA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83297FAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83297FB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83297FB0 size=64
    let mut pc: u32 = 0x83297FB0;
    'dispatch: loop {
        match pc {
            0x83297FB0 => {
    //   block [0x83297FB0..0x83297FF0)
	// 83297FB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83297FB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83297FB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83297FBC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83297FC0: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83297FC4: 388BBED4  addi r4, r11, -0x412c
	ctx.r[4].s64 = ctx.r[11].s64 + -16684;
	// 83297FC8: 386A4F60  addi r3, r10, 0x4f60
	ctx.r[3].s64 = ctx.r[10].s64 + 20320;
	// 83297FCC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83297FD0: 4AF94F01  bl 0x8222ced0
	ctx.lr = 0x83297FD4;
	sub_8222CED0(ctx, base);
	// 83297FD4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83297FD8: 386978C8  addi r3, r9, 0x78c8
	ctx.r[3].s64 = ctx.r[9].s64 + 30920;
	// 83297FDC: 4BA11F45  bl 0x82ca9f20
	ctx.lr = 0x83297FE0;
	sub_82CA9F20(ctx, base);
	// 83297FE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83297FE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83297FE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83297FEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83297FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83297FF0 size=64
    let mut pc: u32 = 0x83297FF0;
    'dispatch: loop {
        match pc {
            0x83297FF0 => {
    //   block [0x83297FF0..0x83298030)
	// 83297FF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83297FF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83297FF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83297FFC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83298000: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83298004: 388BC07C  addi r4, r11, -0x3f84
	ctx.r[4].s64 = ctx.r[11].s64 + -16260;
	// 83298008: 386A4F64  addi r3, r10, 0x4f64
	ctx.r[3].s64 = ctx.r[10].s64 + 20324;
	// 8329800C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83298010: 4AF94EC1  bl 0x8222ced0
	ctx.lr = 0x83298014;
	sub_8222CED0(ctx, base);
	// 83298014: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83298018: 386978D8  addi r3, r9, 0x78d8
	ctx.r[3].s64 = ctx.r[9].s64 + 30936;
	// 8329801C: 4BA11F05  bl 0x82ca9f20
	ctx.lr = 0x83298020;
	sub_82CA9F20(ctx, base);
	// 83298020: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83298024: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83298028: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329802C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83298030 size=64
    let mut pc: u32 = 0x83298030;
    'dispatch: loop {
        match pc {
            0x83298030 => {
    //   block [0x83298030..0x83298070)
	// 83298030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83298034: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83298038: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329803C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83298040: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83298044: 388BC094  addi r4, r11, -0x3f6c
	ctx.r[4].s64 = ctx.r[11].s64 + -16236;
	// 83298048: 386A4F68  addi r3, r10, 0x4f68
	ctx.r[3].s64 = ctx.r[10].s64 + 20328;
	// 8329804C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83298050: 4AF94E81  bl 0x8222ced0
	ctx.lr = 0x83298054;
	sub_8222CED0(ctx, base);
	// 83298054: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83298058: 386978E8  addi r3, r9, 0x78e8
	ctx.r[3].s64 = ctx.r[9].s64 + 30952;
	// 8329805C: 4BA11EC5  bl 0x82ca9f20
	ctx.lr = 0x83298060;
	sub_82CA9F20(ctx, base);
	// 83298060: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83298064: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83298068: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329806C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83298070 size=64
    let mut pc: u32 = 0x83298070;
    'dispatch: loop {
        match pc {
            0x83298070 => {
    //   block [0x83298070..0x832980B0)
	// 83298070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83298074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83298078: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329807C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83298080: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83298084: 388BC0A4  addi r4, r11, -0x3f5c
	ctx.r[4].s64 = ctx.r[11].s64 + -16220;
	// 83298088: 386A4F6C  addi r3, r10, 0x4f6c
	ctx.r[3].s64 = ctx.r[10].s64 + 20332;
	// 8329808C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83298090: 4AF94E41  bl 0x8222ced0
	ctx.lr = 0x83298094;
	sub_8222CED0(ctx, base);
	// 83298094: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83298098: 386978F8  addi r3, r9, 0x78f8
	ctx.r[3].s64 = ctx.r[9].s64 + 30968;
	// 8329809C: 4BA11E85  bl 0x82ca9f20
	ctx.lr = 0x832980A0;
	sub_82CA9F20(ctx, base);
	// 832980A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832980A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832980A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832980AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832980B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832980B0 size=64
    let mut pc: u32 = 0x832980B0;
    'dispatch: loop {
        match pc {
            0x832980B0 => {
    //   block [0x832980B0..0x832980F0)
	// 832980B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832980B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832980B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832980BC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 832980C0: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 832980C4: 388BC180  addi r4, r11, -0x3e80
	ctx.r[4].s64 = ctx.r[11].s64 + -16000;
	// 832980C8: 386A4F70  addi r3, r10, 0x4f70
	ctx.r[3].s64 = ctx.r[10].s64 + 20336;
	// 832980CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832980D0: 4AF94E01  bl 0x8222ced0
	ctx.lr = 0x832980D4;
	sub_8222CED0(ctx, base);
	// 832980D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832980D8: 38697908  addi r3, r9, 0x7908
	ctx.r[3].s64 = ctx.r[9].s64 + 30984;
	// 832980DC: 4BA11E45  bl 0x82ca9f20
	ctx.lr = 0x832980E0;
	sub_82CA9F20(ctx, base);
	// 832980E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832980E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832980E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832980EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832980F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832980F0 size=64
    let mut pc: u32 = 0x832980F0;
    'dispatch: loop {
        match pc {
            0x832980F0 => {
    //   block [0x832980F0..0x83298130)
	// 832980F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832980F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832980F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832980FC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83298100: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83298104: 388BC190  addi r4, r11, -0x3e70
	ctx.r[4].s64 = ctx.r[11].s64 + -15984;
	// 83298108: 386A4F74  addi r3, r10, 0x4f74
	ctx.r[3].s64 = ctx.r[10].s64 + 20340;
	// 8329810C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83298110: 4AF94DC1  bl 0x8222ced0
	ctx.lr = 0x83298114;
	sub_8222CED0(ctx, base);
	// 83298114: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83298118: 38697918  addi r3, r9, 0x7918
	ctx.r[3].s64 = ctx.r[9].s64 + 31000;
	// 8329811C: 4BA11E05  bl 0x82ca9f20
	ctx.lr = 0x83298120;
	sub_82CA9F20(ctx, base);
	// 83298120: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83298124: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83298128: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329812C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83298130 size=64
    let mut pc: u32 = 0x83298130;
    'dispatch: loop {
        match pc {
            0x83298130 => {
    //   block [0x83298130..0x83298170)
	// 83298130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83298134: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83298138: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329813C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83298140: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83298144: 388BC1A4  addi r4, r11, -0x3e5c
	ctx.r[4].s64 = ctx.r[11].s64 + -15964;
	// 83298148: 386A4F78  addi r3, r10, 0x4f78
	ctx.r[3].s64 = ctx.r[10].s64 + 20344;
	// 8329814C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83298150: 4AF94D81  bl 0x8222ced0
	ctx.lr = 0x83298154;
	sub_8222CED0(ctx, base);
	// 83298154: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83298158: 38697928  addi r3, r9, 0x7928
	ctx.r[3].s64 = ctx.r[9].s64 + 31016;
	// 8329815C: 4BA11DC5  bl 0x82ca9f20
	ctx.lr = 0x83298160;
	sub_82CA9F20(ctx, base);
	// 83298160: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83298164: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83298168: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329816C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83298170 size=64
    let mut pc: u32 = 0x83298170;
    'dispatch: loop {
        match pc {
            0x83298170 => {
    //   block [0x83298170..0x832981B0)
	// 83298170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83298174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83298178: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329817C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83298180: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83298184: 388BC24C  addi r4, r11, -0x3db4
	ctx.r[4].s64 = ctx.r[11].s64 + -15796;
	// 83298188: 386A4F7C  addi r3, r10, 0x4f7c
	ctx.r[3].s64 = ctx.r[10].s64 + 20348;
	// 8329818C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83298190: 4AF94D41  bl 0x8222ced0
	ctx.lr = 0x83298194;
	sub_8222CED0(ctx, base);
	// 83298194: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83298198: 38697938  addi r3, r9, 0x7938
	ctx.r[3].s64 = ctx.r[9].s64 + 31032;
	// 8329819C: 4BA11D85  bl 0x82ca9f20
	ctx.lr = 0x832981A0;
	sub_82CA9F20(ctx, base);
	// 832981A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832981A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832981A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832981AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832981B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832981B0 size=64
    let mut pc: u32 = 0x832981B0;
    'dispatch: loop {
        match pc {
            0x832981B0 => {
    //   block [0x832981B0..0x832981F0)
	// 832981B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832981B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832981B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832981BC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 832981C0: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 832981C4: 388BC258  addi r4, r11, -0x3da8
	ctx.r[4].s64 = ctx.r[11].s64 + -15784;
	// 832981C8: 386A4F80  addi r3, r10, 0x4f80
	ctx.r[3].s64 = ctx.r[10].s64 + 20352;
	// 832981CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832981D0: 4AF94D01  bl 0x8222ced0
	ctx.lr = 0x832981D4;
	sub_8222CED0(ctx, base);
	// 832981D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832981D8: 38697948  addi r3, r9, 0x7948
	ctx.r[3].s64 = ctx.r[9].s64 + 31048;
	// 832981DC: 4BA11D45  bl 0x82ca9f20
	ctx.lr = 0x832981E0;
	sub_82CA9F20(ctx, base);
	// 832981E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832981E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832981E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832981EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832981F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832981F0 size=64
    let mut pc: u32 = 0x832981F0;
    'dispatch: loop {
        match pc {
            0x832981F0 => {
    //   block [0x832981F0..0x83298230)
	// 832981F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832981F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832981F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832981FC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83298200: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83298204: 388BC26C  addi r4, r11, -0x3d94
	ctx.r[4].s64 = ctx.r[11].s64 + -15764;
	// 83298208: 386A4F84  addi r3, r10, 0x4f84
	ctx.r[3].s64 = ctx.r[10].s64 + 20356;
	// 8329820C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83298210: 4AF94CC1  bl 0x8222ced0
	ctx.lr = 0x83298214;
	sub_8222CED0(ctx, base);
	// 83298214: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83298218: 38697958  addi r3, r9, 0x7958
	ctx.r[3].s64 = ctx.r[9].s64 + 31064;
	// 8329821C: 4BA11D05  bl 0x82ca9f20
	ctx.lr = 0x83298220;
	sub_82CA9F20(ctx, base);
	// 83298220: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83298224: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83298228: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329822C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83298230 size=64
    let mut pc: u32 = 0x83298230;
    'dispatch: loop {
        match pc {
            0x83298230 => {
    //   block [0x83298230..0x83298270)
	// 83298230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83298234: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83298238: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329823C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83298240: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83298244: 388BC324  addi r4, r11, -0x3cdc
	ctx.r[4].s64 = ctx.r[11].s64 + -15580;
	// 83298248: 386A4F94  addi r3, r10, 0x4f94
	ctx.r[3].s64 = ctx.r[10].s64 + 20372;
	// 8329824C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83298250: 4AF94C81  bl 0x8222ced0
	ctx.lr = 0x83298254;
	sub_8222CED0(ctx, base);
	// 83298254: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83298258: 38697968  addi r3, r9, 0x7968
	ctx.r[3].s64 = ctx.r[9].s64 + 31080;
	// 8329825C: 4BA11CC5  bl 0x82ca9f20
	ctx.lr = 0x83298260;
	sub_82CA9F20(ctx, base);
	// 83298260: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83298264: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83298268: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329826C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83298270 size=64
    let mut pc: u32 = 0x83298270;
    'dispatch: loop {
        match pc {
            0x83298270 => {
    //   block [0x83298270..0x832982B0)
	// 83298270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83298274: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83298278: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329827C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83298280: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83298284: 388B8CC0  addi r4, r11, -0x7340
	ctx.r[4].s64 = ctx.r[11].s64 + -29504;
	// 83298288: 386A4F98  addi r3, r10, 0x4f98
	ctx.r[3].s64 = ctx.r[10].s64 + 20376;
	// 8329828C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83298290: 4AF94C41  bl 0x8222ced0
	ctx.lr = 0x83298294;
	sub_8222CED0(ctx, base);
	// 83298294: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83298298: 38697978  addi r3, r9, 0x7978
	ctx.r[3].s64 = ctx.r[9].s64 + 31096;
	// 8329829C: 4BA11C85  bl 0x82ca9f20
	ctx.lr = 0x832982A0;
	sub_82CA9F20(ctx, base);
	// 832982A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832982A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832982A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832982AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832982B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832982B0 size=64
    let mut pc: u32 = 0x832982B0;
    'dispatch: loop {
        match pc {
            0x832982B0 => {
    //   block [0x832982B0..0x832982F0)
	// 832982B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832982B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832982B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832982BC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 832982C0: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 832982C4: 388BC3B8  addi r4, r11, -0x3c48
	ctx.r[4].s64 = ctx.r[11].s64 + -15432;
	// 832982C8: 386A4F9C  addi r3, r10, 0x4f9c
	ctx.r[3].s64 = ctx.r[10].s64 + 20380;
	// 832982CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832982D0: 4AF94C01  bl 0x8222ced0
	ctx.lr = 0x832982D4;
	sub_8222CED0(ctx, base);
	// 832982D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832982D8: 38697988  addi r3, r9, 0x7988
	ctx.r[3].s64 = ctx.r[9].s64 + 31112;
	// 832982DC: 4BA11C45  bl 0x82ca9f20
	ctx.lr = 0x832982E0;
	sub_82CA9F20(ctx, base);
	// 832982E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832982E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832982E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832982EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832982F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832982F0 size=64
    let mut pc: u32 = 0x832982F0;
    'dispatch: loop {
        match pc {
            0x832982F0 => {
    //   block [0x832982F0..0x83298330)
	// 832982F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832982F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832982F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832982FC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83298300: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83298304: 388BC3C0  addi r4, r11, -0x3c40
	ctx.r[4].s64 = ctx.r[11].s64 + -15424;
	// 83298308: 386A4FA0  addi r3, r10, 0x4fa0
	ctx.r[3].s64 = ctx.r[10].s64 + 20384;
	// 8329830C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83298310: 4AF94BC1  bl 0x8222ced0
	ctx.lr = 0x83298314;
	sub_8222CED0(ctx, base);
	// 83298314: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83298318: 38697998  addi r3, r9, 0x7998
	ctx.r[3].s64 = ctx.r[9].s64 + 31128;
	// 8329831C: 4BA11C05  bl 0x82ca9f20
	ctx.lr = 0x83298320;
	sub_82CA9F20(ctx, base);
	// 83298320: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83298324: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83298328: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329832C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83298330 size=64
    let mut pc: u32 = 0x83298330;
    'dispatch: loop {
        match pc {
            0x83298330 => {
    //   block [0x83298330..0x83298370)
	// 83298330: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83298334: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83298338: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329833C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83298340: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83298344: 388BC3CC  addi r4, r11, -0x3c34
	ctx.r[4].s64 = ctx.r[11].s64 + -15412;
	// 83298348: 386A4FA4  addi r3, r10, 0x4fa4
	ctx.r[3].s64 = ctx.r[10].s64 + 20388;
	// 8329834C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83298350: 4AF94B81  bl 0x8222ced0
	ctx.lr = 0x83298354;
	sub_8222CED0(ctx, base);
	// 83298354: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83298358: 386979A8  addi r3, r9, 0x79a8
	ctx.r[3].s64 = ctx.r[9].s64 + 31144;
	// 8329835C: 4BA11BC5  bl 0x82ca9f20
	ctx.lr = 0x83298360;
	sub_82CA9F20(ctx, base);
	// 83298360: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83298364: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83298368: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329836C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83298370 size=64
    let mut pc: u32 = 0x83298370;
    'dispatch: loop {
        match pc {
            0x83298370 => {
    //   block [0x83298370..0x832983B0)
	// 83298370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83298374: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83298378: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329837C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83298380: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83298384: 388BC55C  addi r4, r11, -0x3aa4
	ctx.r[4].s64 = ctx.r[11].s64 + -15012;
	// 83298388: 386A4FA8  addi r3, r10, 0x4fa8
	ctx.r[3].s64 = ctx.r[10].s64 + 20392;
	// 8329838C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83298390: 4AF94B41  bl 0x8222ced0
	ctx.lr = 0x83298394;
	sub_8222CED0(ctx, base);
	// 83298394: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83298398: 386979B8  addi r3, r9, 0x79b8
	ctx.r[3].s64 = ctx.r[9].s64 + 31160;
	// 8329839C: 4BA11B85  bl 0x82ca9f20
	ctx.lr = 0x832983A0;
	sub_82CA9F20(ctx, base);
	// 832983A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832983A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832983A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832983AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832983B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832983B0 size=64
    let mut pc: u32 = 0x832983B0;
    'dispatch: loop {
        match pc {
            0x832983B0 => {
    //   block [0x832983B0..0x832983F0)
	// 832983B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832983B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832983B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832983BC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 832983C0: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 832983C4: 388BC574  addi r4, r11, -0x3a8c
	ctx.r[4].s64 = ctx.r[11].s64 + -14988;
	// 832983C8: 386A4FAC  addi r3, r10, 0x4fac
	ctx.r[3].s64 = ctx.r[10].s64 + 20396;
	// 832983CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832983D0: 4AF94B01  bl 0x8222ced0
	ctx.lr = 0x832983D4;
	sub_8222CED0(ctx, base);
	// 832983D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832983D8: 386979C8  addi r3, r9, 0x79c8
	ctx.r[3].s64 = ctx.r[9].s64 + 31176;
	// 832983DC: 4BA11B45  bl 0x82ca9f20
	ctx.lr = 0x832983E0;
	sub_82CA9F20(ctx, base);
	// 832983E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832983E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832983E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832983EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832983F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832983F0 size=64
    let mut pc: u32 = 0x832983F0;
    'dispatch: loop {
        match pc {
            0x832983F0 => {
    //   block [0x832983F0..0x83298430)
	// 832983F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832983F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832983F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832983FC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83298400: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83298404: 388BC674  addi r4, r11, -0x398c
	ctx.r[4].s64 = ctx.r[11].s64 + -14732;
	// 83298408: 386A4FB0  addi r3, r10, 0x4fb0
	ctx.r[3].s64 = ctx.r[10].s64 + 20400;
	// 8329840C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83298410: 4AF94AC1  bl 0x8222ced0
	ctx.lr = 0x83298414;
	sub_8222CED0(ctx, base);
	// 83298414: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83298418: 386979D8  addi r3, r9, 0x79d8
	ctx.r[3].s64 = ctx.r[9].s64 + 31192;
	// 8329841C: 4BA11B05  bl 0x82ca9f20
	ctx.lr = 0x83298420;
	sub_82CA9F20(ctx, base);
	// 83298420: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83298424: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83298428: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329842C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83298430 size=64
    let mut pc: u32 = 0x83298430;
    'dispatch: loop {
        match pc {
            0x83298430 => {
    //   block [0x83298430..0x83298470)
	// 83298430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83298434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83298438: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329843C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83298440: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83298444: 388BC684  addi r4, r11, -0x397c
	ctx.r[4].s64 = ctx.r[11].s64 + -14716;
	// 83298448: 386A4FB4  addi r3, r10, 0x4fb4
	ctx.r[3].s64 = ctx.r[10].s64 + 20404;
	// 8329844C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83298450: 4AF94A81  bl 0x8222ced0
	ctx.lr = 0x83298454;
	sub_8222CED0(ctx, base);
	// 83298454: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83298458: 386979E8  addi r3, r9, 0x79e8
	ctx.r[3].s64 = ctx.r[9].s64 + 31208;
	// 8329845C: 4BA11AC5  bl 0x82ca9f20
	ctx.lr = 0x83298460;
	sub_82CA9F20(ctx, base);
	// 83298460: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83298464: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83298468: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329846C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83298470 size=64
    let mut pc: u32 = 0x83298470;
    'dispatch: loop {
        match pc {
            0x83298470 => {
    //   block [0x83298470..0x832984B0)
	// 83298470: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83298474: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83298478: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329847C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83298480: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83298484: 388BC7D4  addi r4, r11, -0x382c
	ctx.r[4].s64 = ctx.r[11].s64 + -14380;
	// 83298488: 386A4FB8  addi r3, r10, 0x4fb8
	ctx.r[3].s64 = ctx.r[10].s64 + 20408;
	// 8329848C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83298490: 4AF94A41  bl 0x8222ced0
	ctx.lr = 0x83298494;
	sub_8222CED0(ctx, base);
	// 83298494: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83298498: 386979F8  addi r3, r9, 0x79f8
	ctx.r[3].s64 = ctx.r[9].s64 + 31224;
	// 8329849C: 4BA11A85  bl 0x82ca9f20
	ctx.lr = 0x832984A0;
	sub_82CA9F20(ctx, base);
	// 832984A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832984A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832984A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832984AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832984B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832984B0 size=64
    let mut pc: u32 = 0x832984B0;
    'dispatch: loop {
        match pc {
            0x832984B0 => {
    //   block [0x832984B0..0x832984F0)
	// 832984B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832984B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832984B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832984BC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 832984C0: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 832984C4: 388BC7E8  addi r4, r11, -0x3818
	ctx.r[4].s64 = ctx.r[11].s64 + -14360;
	// 832984C8: 386A4FBC  addi r3, r10, 0x4fbc
	ctx.r[3].s64 = ctx.r[10].s64 + 20412;
	// 832984CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832984D0: 4AF94A01  bl 0x8222ced0
	ctx.lr = 0x832984D4;
	sub_8222CED0(ctx, base);
	// 832984D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832984D8: 38697A08  addi r3, r9, 0x7a08
	ctx.r[3].s64 = ctx.r[9].s64 + 31240;
	// 832984DC: 4BA11A45  bl 0x82ca9f20
	ctx.lr = 0x832984E0;
	sub_82CA9F20(ctx, base);
	// 832984E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832984E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832984E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832984EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832984F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832984F0 size=64
    let mut pc: u32 = 0x832984F0;
    'dispatch: loop {
        match pc {
            0x832984F0 => {
    //   block [0x832984F0..0x83298530)
	// 832984F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832984F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832984F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832984FC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83298500: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83298504: 388BC8D8  addi r4, r11, -0x3728
	ctx.r[4].s64 = ctx.r[11].s64 + -14120;
	// 83298508: 386A4FC0  addi r3, r10, 0x4fc0
	ctx.r[3].s64 = ctx.r[10].s64 + 20416;
	// 8329850C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83298510: 4AF949C1  bl 0x8222ced0
	ctx.lr = 0x83298514;
	sub_8222CED0(ctx, base);
	// 83298514: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83298518: 38697A18  addi r3, r9, 0x7a18
	ctx.r[3].s64 = ctx.r[9].s64 + 31256;
	// 8329851C: 4BA11A05  bl 0x82ca9f20
	ctx.lr = 0x83298520;
	sub_82CA9F20(ctx, base);
	// 83298520: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83298524: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83298528: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329852C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83298530 size=64
    let mut pc: u32 = 0x83298530;
    'dispatch: loop {
        match pc {
            0x83298530 => {
    //   block [0x83298530..0x83298570)
	// 83298530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83298534: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83298538: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329853C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83298540: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83298544: 388BC8E8  addi r4, r11, -0x3718
	ctx.r[4].s64 = ctx.r[11].s64 + -14104;
	// 83298548: 386A4FC4  addi r3, r10, 0x4fc4
	ctx.r[3].s64 = ctx.r[10].s64 + 20420;
	// 8329854C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83298550: 4AF94981  bl 0x8222ced0
	ctx.lr = 0x83298554;
	sub_8222CED0(ctx, base);
	// 83298554: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83298558: 38697A28  addi r3, r9, 0x7a28
	ctx.r[3].s64 = ctx.r[9].s64 + 31272;
	// 8329855C: 4BA119C5  bl 0x82ca9f20
	ctx.lr = 0x83298560;
	sub_82CA9F20(ctx, base);
	// 83298560: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83298564: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83298568: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329856C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83298570 size=64
    let mut pc: u32 = 0x83298570;
    'dispatch: loop {
        match pc {
            0x83298570 => {
    //   block [0x83298570..0x832985B0)
	// 83298570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83298574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83298578: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329857C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83298580: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83298584: 388BC914  addi r4, r11, -0x36ec
	ctx.r[4].s64 = ctx.r[11].s64 + -14060;
	// 83298588: 386A4FC8  addi r3, r10, 0x4fc8
	ctx.r[3].s64 = ctx.r[10].s64 + 20424;
	// 8329858C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83298590: 4AF94941  bl 0x8222ced0
	ctx.lr = 0x83298594;
	sub_8222CED0(ctx, base);
	// 83298594: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83298598: 38697A38  addi r3, r9, 0x7a38
	ctx.r[3].s64 = ctx.r[9].s64 + 31288;
	// 8329859C: 4BA11985  bl 0x82ca9f20
	ctx.lr = 0x832985A0;
	sub_82CA9F20(ctx, base);
	// 832985A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832985A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832985A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832985AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832985B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832985B0 size=64
    let mut pc: u32 = 0x832985B0;
    'dispatch: loop {
        match pc {
            0x832985B0 => {
    //   block [0x832985B0..0x832985F0)
	// 832985B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832985B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832985B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832985BC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 832985C0: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 832985C4: 388BC9D0  addi r4, r11, -0x3630
	ctx.r[4].s64 = ctx.r[11].s64 + -13872;
	// 832985C8: 386A4FD8  addi r3, r10, 0x4fd8
	ctx.r[3].s64 = ctx.r[10].s64 + 20440;
	// 832985CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832985D0: 4AF94901  bl 0x8222ced0
	ctx.lr = 0x832985D4;
	sub_8222CED0(ctx, base);
	// 832985D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832985D8: 38697A48  addi r3, r9, 0x7a48
	ctx.r[3].s64 = ctx.r[9].s64 + 31304;
	// 832985DC: 4BA11945  bl 0x82ca9f20
	ctx.lr = 0x832985E0;
	sub_82CA9F20(ctx, base);
	// 832985E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832985E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832985E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832985EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832985F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832985F0 size=64
    let mut pc: u32 = 0x832985F0;
    'dispatch: loop {
        match pc {
            0x832985F0 => {
    //   block [0x832985F0..0x83298630)
	// 832985F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832985F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832985F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832985FC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83298600: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83298604: 388BA950  addi r4, r11, -0x56b0
	ctx.r[4].s64 = ctx.r[11].s64 + -22192;
	// 83298608: 386A4FDC  addi r3, r10, 0x4fdc
	ctx.r[3].s64 = ctx.r[10].s64 + 20444;
	// 8329860C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83298610: 4AF948C1  bl 0x8222ced0
	ctx.lr = 0x83298614;
	sub_8222CED0(ctx, base);
	// 83298614: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83298618: 38697A58  addi r3, r9, 0x7a58
	ctx.r[3].s64 = ctx.r[9].s64 + 31320;
	// 8329861C: 4BA11905  bl 0x82ca9f20
	ctx.lr = 0x83298620;
	sub_82CA9F20(ctx, base);
	// 83298620: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83298624: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83298628: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329862C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83298630 size=132
    let mut pc: u32 = 0x83298630;
    'dispatch: loop {
        match pc {
            0x83298630 => {
    //   block [0x83298630..0x83298688)
	// 83298630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83298634: 4BA10DD9  bl 0x82ca940c
	ctx.lr = 0x83298638;
	sub_82CA93D0(ctx, base);
	// 83298638: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329863C: 4B8BC6BD  bl 0x82b54cf8
	ctx.lr = 0x83298640;
	sub_82B54CF8(ctx, base);
	// 83298640: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83298644: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83298648: 3BEA4FE0  addi r31, r10, 0x4fe0
	ctx.r[31].s64 = ctx.r[10].s64 + 20448;
	// 8329864C: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 83298650: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	// 83298654: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83298658: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 8329865C: F97F0000  std r11, 0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 83298660: F95F0008  std r10, 8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u64 ) };
	// 83298664: 4BD68A8D  bl 0x830010f0
	ctx.lr = 0x83298668;
	sub_830010F0(ctx, base);
	// 83298668: 93DF0020  stw r30, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 8329866C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 83298670: 4AF86BE9  bl 0x8221f258
	ctx.lr = 0x83298674;
	sub_8221F258(ctx, base);
	// 83298674: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83298678: 419A0010  beq cr6, 0x83298688
	if ctx.cr[6].eq {
	pc = 0x83298688; continue 'dispatch;
	}
	// 8329867C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83298680: FBA30008  std r29, 8(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[29].u64 ) };
	// 83298684: 48000008  b 0x8329868c
	pc = 0x8329868C; continue 'dispatch;
            }
            0x83298688 => {
    //   block [0x83298688..0x8329868C)
	// 83298688: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	pc = 0x8329868C; continue 'dispatch;
            }
            0x8329868C => {
    //   block [0x8329868C..0x832986B4)
	// 8329868C: 93AB0008  stw r29, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 83298690: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 83298694: 93AB000C  stw r29, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 83298698: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8329869C: 386A7A68  addi r3, r10, 0x7a68
	ctx.r[3].s64 = ctx.r[10].s64 + 31336;
	// 832986A0: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 832986A4: 93BF0010  stw r29, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[29].u32 ) };
	// 832986A8: 4BA11879  bl 0x82ca9f20
	ctx.lr = 0x832986AC;
	sub_82CA9F20(ctx, base);
	// 832986AC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832986B0: 4BA10DAC  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832986B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832986B8 size=64
    let mut pc: u32 = 0x832986B8;
    'dispatch: loop {
        match pc {
            0x832986B8 => {
    //   block [0x832986B8..0x832986F8)
	// 832986B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832986BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832986C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832986C4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 832986C8: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 832986CC: 388BCA58  addi r4, r11, -0x35a8
	ctx.r[4].s64 = ctx.r[11].s64 + -13736;
	// 832986D0: 386A5008  addi r3, r10, 0x5008
	ctx.r[3].s64 = ctx.r[10].s64 + 20488;
	// 832986D4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832986D8: 4AF947F9  bl 0x8222ced0
	ctx.lr = 0x832986DC;
	sub_8222CED0(ctx, base);
	// 832986DC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832986E0: 38697A78  addi r3, r9, 0x7a78
	ctx.r[3].s64 = ctx.r[9].s64 + 31352;
	// 832986E4: 4BA1183D  bl 0x82ca9f20
	ctx.lr = 0x832986E8;
	sub_82CA9F20(ctx, base);
	// 832986E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832986EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832986F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832986F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832986F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832986F8 size=64
    let mut pc: u32 = 0x832986F8;
    'dispatch: loop {
        match pc {
            0x832986F8 => {
    //   block [0x832986F8..0x83298738)
	// 832986F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832986FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83298700: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83298704: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83298708: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 8329870C: 388BCA64  addi r4, r11, -0x359c
	ctx.r[4].s64 = ctx.r[11].s64 + -13724;
	// 83298710: 386A500C  addi r3, r10, 0x500c
	ctx.r[3].s64 = ctx.r[10].s64 + 20492;
	// 83298714: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83298718: 4AF947B9  bl 0x8222ced0
	ctx.lr = 0x8329871C;
	sub_8222CED0(ctx, base);
	// 8329871C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83298720: 38697A88  addi r3, r9, 0x7a88
	ctx.r[3].s64 = ctx.r[9].s64 + 31368;
	// 83298724: 4BA117FD  bl 0x82ca9f20
	ctx.lr = 0x83298728;
	sub_82CA9F20(ctx, base);
	// 83298728: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8329872C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83298730: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83298734: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83298738 size=52
    let mut pc: u32 = 0x83298738;
    'dispatch: loop {
        match pc {
            0x83298738 => {
    //   block [0x83298738..0x8329876C)
	// 83298738: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329873C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83298740: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83298744: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 83298748: 386B5010  addi r3, r11, 0x5010
	ctx.r[3].s64 = ctx.r[11].s64 + 20496;
	// 8329874C: 4B9B73BD  bl 0x82c4fb08
	ctx.lr = 0x83298750;
	sub_82C4FB08(ctx, base);
	// 83298750: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 83298754: 386A7A98  addi r3, r10, 0x7a98
	ctx.r[3].s64 = ctx.r[10].s64 + 31384;
	// 83298758: 4BA117C9  bl 0x82ca9f20
	ctx.lr = 0x8329875C;
	sub_82CA9F20(ctx, base);
	// 8329875C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83298760: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83298764: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83298768: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83298770 size=52
    let mut pc: u32 = 0x83298770;
    'dispatch: loop {
        match pc {
            0x83298770 => {
    //   block [0x83298770..0x832987A4)
	// 83298770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83298774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83298778: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329877C: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 83298780: 386B5088  addi r3, r11, 0x5088
	ctx.r[3].s64 = ctx.r[11].s64 + 20616;
	// 83298784: 4B9B7385  bl 0x82c4fb08
	ctx.lr = 0x83298788;
	sub_82C4FB08(ctx, base);
	// 83298788: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 8329878C: 386A7AF0  addi r3, r10, 0x7af0
	ctx.r[3].s64 = ctx.r[10].s64 + 31472;
	// 83298790: 4BA11791  bl 0x82ca9f20
	ctx.lr = 0x83298794;
	sub_82CA9F20(ctx, base);
	// 83298794: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83298798: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329879C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832987A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832987A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832987A8 size=52
    let mut pc: u32 = 0x832987A8;
    'dispatch: loop {
        match pc {
            0x832987A8 => {
    //   block [0x832987A8..0x832987DC)
	// 832987A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832987AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832987B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832987B4: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832987B8: 386B5108  addi r3, r11, 0x5108
	ctx.r[3].s64 = ctx.r[11].s64 + 20744;
	// 832987BC: 4B9713F5  bl 0x82c09bb0
	ctx.lr = 0x832987C0;
	sub_82C09BB0(ctx, base);
	// 832987C0: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 832987C4: 386A7B48  addi r3, r10, 0x7b48
	ctx.r[3].s64 = ctx.r[10].s64 + 31560;
	// 832987C8: 4BA11759  bl 0x82ca9f20
	ctx.lr = 0x832987CC;
	sub_82CA9F20(ctx, base);
	// 832987CC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832987D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832987D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832987D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832987E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832987E0 size=16
    let mut pc: u32 = 0x832987E0;
    'dispatch: loop {
        match pc {
            0x832987E0 => {
    //   block [0x832987E0..0x832987F0)
	// 832987E0: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832987E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832987E8: 914B515C  stw r10, 0x515c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20828 as u32), ctx.r[10].u32 ) };
	// 832987EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832987F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832987F0 size=16
    let mut pc: u32 = 0x832987F0;
    'dispatch: loop {
        match pc {
            0x832987F0 => {
    //   block [0x832987F0..0x83298800)
	// 832987F0: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832987F4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832987F8: 914B5160  stw r10, 0x5160(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20832 as u32), ctx.r[10].u32 ) };
	// 832987FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83298800 size=52
    let mut pc: u32 = 0x83298800;
    'dispatch: loop {
        match pc {
            0x83298800 => {
    //   block [0x83298800..0x83298834)
	// 83298800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83298804: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83298808: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329880C: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 83298810: 386B5184  addi r3, r11, 0x5184
	ctx.r[3].s64 = ctx.r[11].s64 + 20868;
	// 83298814: 4B215E4D  bl 0x824ae660
	ctx.lr = 0x83298818;
	sub_824AE660(ctx, base);
	// 83298818: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 8329881C: 386A7B58  addi r3, r10, 0x7b58
	ctx.r[3].s64 = ctx.r[10].s64 + 31576;
	// 83298820: 4BA11701  bl 0x82ca9f20
	ctx.lr = 0x83298824;
	sub_82CA9F20(ctx, base);
	// 83298824: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83298828: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329882C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83298830: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83298838 size=524
    let mut pc: u32 = 0x83298838;
    'dispatch: loop {
        match pc {
            0x83298838 => {
    //   block [0x83298838..0x832988E4)
	// 83298838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329883C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83298840: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83298844: DBC1FFE0  stfd f30, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[30].u64 ) };
	// 83298848: DBE1FFE8  stfd f31, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 8329884C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83298850: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 83298854: 38A00045  li r5, 0x45
	ctx.r[5].s64 = 69;
	// 83298858: 3BEB62D0  addi r31, r11, 0x62d0
	ctx.r[31].s64 = ctx.r[11].s64 + 25296;
	// 8329885C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83298860: 387F00CF  addi r3, r31, 0xcf
	ctx.r[3].s64 = ctx.r[31].s64 + 207;
	// 83298864: 4BA1114D  bl 0x82ca99b0
	ctx.lr = 0x83298868;
	sub_82CA99B0(ctx, base);
	// 83298868: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8329886C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 83298870: 392BD53C  addi r9, r11, -0x2ac4
	ctx.r[9].s64 = ctx.r[11].s64 + -10948;
	// 83298874: 38A00044  li r5, 0x44
	ctx.r[5].s64 = 68;
	// 83298878: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8329887C: 810BD53C  lwz r8, -0x2ac4(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10948 as u32) ) } as u64;
	// 83298880: 387F012C  addi r3, r31, 0x12c
	ctx.r[3].s64 = ctx.r[31].s64 + 300;
	// 83298884: C3EA0C14  lfs f31, 0xc14(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3092 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 83298888: 80E90004  lwz r7, 4(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 8329888C: D3FF0114  stfs f31, 0x114(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(276 as u32), tmp.u32 ) };
	// 83298890: 80C90008  lwz r6, 8(r9)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 83298894: D3FF0118  stfs f31, 0x118(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(280 as u32), tmp.u32 ) };
	// 83298898: D3FF011C  stfs f31, 0x11c(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(284 as u32), tmp.u32 ) };
	// 8329889C: 911F0120  stw r8, 0x120(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(288 as u32), ctx.r[8].u32 ) };
	// 832988A0: 90FF0124  stw r7, 0x124(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(292 as u32), ctx.r[7].u32 ) };
	// 832988A4: 90DF0128  stw r6, 0x128(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(296 as u32), ctx.r[6].u32 ) };
	// 832988A8: 4BA11109  bl 0x82ca99b0
	ctx.lr = 0x832988AC;
	sub_82CA99B0(ctx, base);
	// 832988AC: 3CA08201  lis r5, -0x7dff
	ctx.r[5].s64 = -2113863680;
	// 832988B0: 3C808201  lis r4, -0x7dff
	ctx.r[4].s64 = -2113863680;
	// 832988B4: 3C608201  lis r3, -0x7dff
	ctx.r[3].s64 = -2113863680;
	// 832988B8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 832988BC: 395F017C  addi r10, r31, 0x17c
	ctx.r[10].s64 = ctx.r[31].s64 + 380;
	// 832988C0: C005D308  lfs f0, -0x2cf8(r5)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(-11512 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832988C4: 396BD520  addi r11, r11, -0x2ae0
	ctx.r[11].s64 = ctx.r[11].s64 + -10976;
	// 832988C8: C1A4D300  lfs f13, -0x2d00(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(-11520 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 832988CC: 3920000D  li r9, 0xd
	ctx.r[9].s64 = 13;
	// 832988D0: C183D304  lfs f12, -0x2cfc(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-11516 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 832988D4: D01F0170  stfs f0, 0x170(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(368 as u32), tmp.u32 ) };
	// 832988D8: D1BF0174  stfs f13, 0x174(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(372 as u32), tmp.u32 ) };
	// 832988DC: D19F0178  stfs f12, 0x178(r31)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(376 as u32), tmp.u32 ) };
	// 832988E0: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	pc = 0x832988E4; continue 'dispatch;
            }
            0x832988E4 => {
    //   block [0x832988E4..0x8329893C)
	// 832988E4: A12B0000  lhz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 832988E8: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 832988EC: B12A0000  sth r9, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u16 ) };
	// 832988F0: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 832988F4: 4200FFF0  bdnz 0x832988e4
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x832988E4; continue 'dispatch;
	}
	// 832988F8: 38A00036  li r5, 0x36
	ctx.r[5].s64 = 54;
	// 832988FC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83298900: 387F0196  addi r3, r31, 0x196
	ctx.r[3].s64 = ctx.r[31].s64 + 406;
	// 83298904: 4BA110AD  bl 0x82ca99b0
	ctx.lr = 0x83298908;
	sub_82CA99B0(ctx, base);
	// 83298908: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8329890C: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 83298910: D3FF01CC  stfs f31, 0x1cc(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(460 as u32), tmp.u32 ) };
	// 83298914: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 83298918: 395F01D8  addi r10, r31, 0x1d8
	ctx.r[10].s64 = ctx.r[31].s64 + 472;
	// 8329891C: 38E8D508  addi r7, r8, -0x2af8
	ctx.r[7].s64 = ctx.r[8].s64 + -11000;
	// 83298920: C3CB0C18  lfs f30, 0xc18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 83298924: C0090A54  lfs f0, 0xa54(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2644 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83298928: 7CEB3B78  mr r11, r7
	ctx.r[11].u64 = ctx.r[7].u64;
	// 8329892C: D3DF01D0  stfs f30, 0x1d0(r31)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(464 as u32), tmp.u32 ) };
	// 83298930: 39200015  li r9, 0x15
	ctx.r[9].s64 = 21;
	// 83298934: D01F01D4  stfs f0, 0x1d4(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(468 as u32), tmp.u32 ) };
	// 83298938: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	pc = 0x8329893C; continue 'dispatch;
            }
            0x8329893C => {
    //   block [0x8329893C..0x832989A0)
	// 8329893C: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83298940: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83298944: 992A0000  stb r9, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 83298948: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8329894C: 4200FFF0  bdnz 0x8329893c
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8329893C; continue 'dispatch;
	}
	// 83298950: 38A0003B  li r5, 0x3b
	ctx.r[5].s64 = 59;
	// 83298954: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83298958: 387F01ED  addi r3, r31, 0x1ed
	ctx.r[3].s64 = ctx.r[31].s64 + 493;
	// 8329895C: 4BA11055  bl 0x82ca99b0
	ctx.lr = 0x83298960;
	sub_82CA99B0(ctx, base);
	// 83298960: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83298964: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 83298968: 3D208210  lis r9, -0x7df0
	ctx.r[9].s64 = -2112880640;
	// 8329896C: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 83298970: 38FF0234  addi r7, r31, 0x234
	ctx.r[7].s64 = ctx.r[31].s64 + 564;
	// 83298974: 38C8D4F4  addi r6, r8, -0x2b0c
	ctx.r[6].s64 = ctx.r[8].s64 + -11020;
	// 83298978: C00BD310  lfs f0, -0x2cf0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-11504 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8329897C: C1AA0BF8  lfs f13, 0xbf8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3064 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 83298980: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 83298984: C1899404  lfs f12, -0x6bfc(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-27644 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 83298988: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 8329898C: D01F0228  stfs f0, 0x228(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(552 as u32), tmp.u32 ) };
	// 83298990: 39200009  li r9, 9
	ctx.r[9].s64 = 9;
	// 83298994: D1BF022C  stfs f13, 0x22c(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(556 as u32), tmp.u32 ) };
	// 83298998: D19F0230  stfs f12, 0x230(r31)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(560 as u32), tmp.u32 ) };
	// 8329899C: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	pc = 0x832989A0; continue 'dispatch;
            }
            0x832989A0 => {
    //   block [0x832989A0..0x83298A0C)
	// 832989A0: A12B0000  lhz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 832989A4: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 832989A8: B12A0000  sth r9, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u16 ) };
	// 832989AC: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 832989B0: 4200FFF0  bdnz 0x832989a0
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x832989A0; continue 'dispatch;
	}
	// 832989B4: 38A0003E  li r5, 0x3e
	ctx.r[5].s64 = 62;
	// 832989B8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 832989BC: 387F0246  addi r3, r31, 0x246
	ctx.r[3].s64 = ctx.r[31].s64 + 582;
	// 832989C0: 4BA10FF1  bl 0x82ca99b0
	ctx.lr = 0x832989C4;
	sub_82CA99B0(ctx, base);
	// 832989C4: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 832989C8: D3DF0284  stfs f30, 0x284(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(644 as u32), tmp.u32 ) };
	// 832989CC: 391F0290  addi r8, r31, 0x290
	ctx.r[8].s64 = ctx.r[31].s64 + 656;
	// 832989D0: 38E9D4E4  addi r7, r9, -0x2b1c
	ctx.r[7].s64 = ctx.r[9].s64 + -11036;
	// 832989D4: D3DF0288  stfs f30, 0x288(r31)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(648 as u32), tmp.u32 ) };
	// 832989D8: D3FF028C  stfs f31, 0x28c(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(652 as u32), tmp.u32 ) };
	// 832989DC: 397F02A0  addi r11, r31, 0x2a0
	ctx.r[11].s64 = ctx.r[31].s64 + 672;
	// 832989E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832989E4: 80C9D4E4  lwz r6, -0x2b1c(r9)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-11036 as u32) ) } as u64;
	// 832989E8: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 832989EC: 80A70004  lwz r5, 4(r7)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) } as u64;
	// 832989F0: 80870008  lwz r4, 8(r7)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(8 as u32) ) } as u64;
	// 832989F4: 8067000C  lwz r3, 0xc(r7)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(12 as u32) ) } as u64;
	// 832989F8: 90DF0290  stw r6, 0x290(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(656 as u32), ctx.r[6].u32 ) };
	// 832989FC: 90BF0294  stw r5, 0x294(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(660 as u32), ctx.r[5].u32 ) };
	// 83298A00: 909F0298  stw r4, 0x298(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(664 as u32), ctx.r[4].u32 ) };
	// 83298A04: 907F029C  stw r3, 0x29c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(668 as u32), ctx.r[3].u32 ) };
	// 83298A08: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	pc = 0x83298A0C; continue 'dispatch;
            }
            0x83298A0C => {
    //   block [0x83298A0C..0x83298A44)
	// 83298A0C: F94B0000  std r10, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u64 ) };
	// 83298A10: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 83298A14: 4200FFF8  bdnz 0x83298a0c
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x83298A0C; continue 'dispatch;
	}
	// 83298A18: 38A001CC  li r5, 0x1cc
	ctx.r[5].s64 = 460;
	// 83298A1C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83298A20: 387F02E0  addi r3, r31, 0x2e0
	ctx.r[3].s64 = ctx.r[31].s64 + 736;
	// 83298A24: 4BA10F8D  bl 0x82ca99b0
	ctx.lr = 0x83298A28;
	sub_82CA99B0(ctx, base);
	// 83298A28: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83298A2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83298A30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83298A34: CBC1FFE0  lfd f30, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 83298A38: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83298A3C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83298A40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83298A48 size=44
    let mut pc: u32 = 0x83298A48;
    'dispatch: loop {
        match pc {
            0x83298A48 => {
    //   block [0x83298A48..0x83298A74)
	// 83298A48: 3D208333  lis r9, -0x7ccd
	ctx.r[9].s64 = -2093809664;
	// 83298A4C: 3D6082C6  lis r11, -0x7d3a
	ctx.r[11].s64 = -2100953088;
	// 83298A50: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 83298A54: 386951AC  addi r3, r9, 0x51ac
	ctx.r[3].s64 = ctx.r[9].s64 + 20908;
	// 83298A58: 388AD6F8  addi r4, r10, -0x2908
	ctx.r[4].s64 = ctx.r[10].s64 + -10504;
	// 83298A5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83298A60: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83298A64: 38EBFB28  addi r7, r11, -0x4d8
	ctx.r[7].s64 = ctx.r[11].s64 + -1240;
	// 83298A68: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83298A6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83298A70: 4B9CB448  b 0x82c63eb8
	sub_82C63EB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83298A78 size=44
    let mut pc: u32 = 0x83298A78;
    'dispatch: loop {
        match pc {
            0x83298A78 => {
    //   block [0x83298A78..0x83298AA4)
	// 83298A78: 3D208333  lis r9, -0x7ccd
	ctx.r[9].s64 = -2093809664;
	// 83298A7C: 3D6082C6  lis r11, -0x7d3a
	ctx.r[11].s64 = -2100953088;
	// 83298A80: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 83298A84: 38695190  addi r3, r9, 0x5190
	ctx.r[3].s64 = ctx.r[9].s64 + 20880;
	// 83298A88: 388AD700  addi r4, r10, -0x2900
	ctx.r[4].s64 = ctx.r[10].s64 + -10496;
	// 83298A8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83298A90: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83298A94: 38EBFB68  addi r7, r11, -0x498
	ctx.r[7].s64 = ctx.r[11].s64 + -1176;
	// 83298A98: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83298A9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83298AA0: 4B9CB418  b 0x82c63eb8
	sub_82C63EB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83298AA8 size=44
    let mut pc: u32 = 0x83298AA8;
    'dispatch: loop {
        match pc {
            0x83298AA8 => {
    //   block [0x83298AA8..0x83298AD4)
	// 83298AA8: 3D208333  lis r9, -0x7ccd
	ctx.r[9].s64 = -2093809664;
	// 83298AAC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83298AB0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 83298AB4: 38695858  addi r3, r9, 0x5858
	ctx.r[3].s64 = ctx.r[9].s64 + 22616;
	// 83298AB8: 38CBDA98  addi r6, r11, -0x2568
	ctx.r[6].s64 = ctx.r[11].s64 + -9576;
	// 83298ABC: 388ADBC0  addi r4, r10, -0x2440
	ctx.r[4].s64 = ctx.r[10].s64 + -9280;
	// 83298AC0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83298AC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83298AC8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83298ACC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83298AD0: 4B9CB3E8  b 0x82c63eb8
	sub_82C63EB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298AD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83298AD8 size=48
    let mut pc: u32 = 0x83298AD8;
    'dispatch: loop {
        match pc {
            0x83298AD8 => {
    //   block [0x83298AD8..0x83298B08)
	// 83298AD8: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 83298ADC: 3D008333  lis r8, -0x7ccd
	ctx.r[8].s64 = -2093809664;
	// 83298AE0: 3D6082C6  lis r11, -0x7d3a
	ctx.r[11].s64 = -2100953088;
	// 83298AE4: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 83298AE8: 3889DBC0  addi r4, r9, -0x2440
	ctx.r[4].s64 = ctx.r[9].s64 + -9280;
	// 83298AEC: 38685804  addi r3, r8, 0x5804
	ctx.r[3].s64 = ctx.r[8].s64 + 22532;
	// 83298AF0: 38CADA98  addi r6, r10, -0x2568
	ctx.r[6].s64 = ctx.r[10].s64 + -9576;
	// 83298AF4: 392B2A90  addi r9, r11, 0x2a90
	ctx.r[9].s64 = ctx.r[11].s64 + 10896;
	// 83298AF8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83298AFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83298B00: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83298B04: 4B9CB3B4  b 0x82c63eb8
	sub_82C63EB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298B08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83298B08 size=48
    let mut pc: u32 = 0x83298B08;
    'dispatch: loop {
        match pc {
            0x83298B08 => {
    //   block [0x83298B08..0x83298B38)
	// 83298B08: 3C808333  lis r4, -0x7ccd
	ctx.r[4].s64 = -2093809664;
	// 83298B0C: 3D6082C6  lis r11, -0x7d3a
	ctx.r[11].s64 = -2100953088;
	// 83298B10: 3D4082C6  lis r10, -0x7d3a
	ctx.r[10].s64 = -2100953088;
	// 83298B14: 3CA082C6  lis r5, -0x7d3a
	ctx.r[5].s64 = -2100953088;
	// 83298B18: 38645794  addi r3, r4, 0x5794
	ctx.r[3].s64 = ctx.r[4].s64 + 22420;
	// 83298B1C: 392B2868  addi r9, r11, 0x2868
	ctx.r[9].s64 = ctx.r[11].s64 + 10344;
	// 83298B20: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83298B24: 38EA2590  addi r7, r10, 0x2590
	ctx.r[7].s64 = ctx.r[10].s64 + 9616;
	// 83298B28: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83298B2C: 38A52560  addi r5, r5, 0x2560
	ctx.r[5].s64 = ctx.r[5].s64 + 9568;
	// 83298B30: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83298B34: 4B9CB384  b 0x82c63eb8
	sub_82C63EB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83298B38 size=48
    let mut pc: u32 = 0x83298B38;
    'dispatch: loop {
        match pc {
            0x83298B38 => {
    //   block [0x83298B38..0x83298B68)
	// 83298B38: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 83298B3C: 3D008333  lis r8, -0x7ccd
	ctx.r[8].s64 = -2093809664;
	// 83298B40: 3D6082C6  lis r11, -0x7d3a
	ctx.r[11].s64 = -2100953088;
	// 83298B44: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 83298B48: 3889DA94  addi r4, r9, -0x256c
	ctx.r[4].s64 = ctx.r[9].s64 + -9580;
	// 83298B4C: 38685900  addi r3, r8, 0x5900
	ctx.r[3].s64 = ctx.r[8].s64 + 22784;
	// 83298B50: 38CADBB8  addi r6, r10, -0x2448
	ctx.r[6].s64 = ctx.r[10].s64 + -9288;
	// 83298B54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83298B58: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83298B5C: 38EB2648  addi r7, r11, 0x2648
	ctx.r[7].s64 = ctx.r[11].s64 + 9800;
	// 83298B60: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83298B64: 4B9CB354  b 0x82c63eb8
	sub_82C63EB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83298B68 size=48
    let mut pc: u32 = 0x83298B68;
    'dispatch: loop {
        match pc {
            0x83298B68 => {
    //   block [0x83298B68..0x83298B98)
	// 83298B68: 3D208210  lis r9, -0x7df0
	ctx.r[9].s64 = -2112880640;
	// 83298B6C: 3D008333  lis r8, -0x7ccd
	ctx.r[8].s64 = -2093809664;
	// 83298B70: 3D6082C6  lis r11, -0x7d3a
	ctx.r[11].s64 = -2100953088;
	// 83298B74: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 83298B78: 3889EBB8  addi r4, r9, -0x1448
	ctx.r[4].s64 = ctx.r[9].s64 + -5192;
	// 83298B7C: 386858E4  addi r3, r8, 0x58e4
	ctx.r[3].s64 = ctx.r[8].s64 + 22756;
	// 83298B80: 38CADAA0  addi r6, r10, -0x2560
	ctx.r[6].s64 = ctx.r[10].s64 + -9568;
	// 83298B84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83298B88: 390B20E8  addi r8, r11, 0x20e8
	ctx.r[8].s64 = ctx.r[11].s64 + 8424;
	// 83298B8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83298B90: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83298B94: 4B9CB324  b 0x82c63eb8
	sub_82C63EB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298B98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83298B98 size=52
    let mut pc: u32 = 0x83298B98;
    'dispatch: loop {
        match pc {
            0x83298B98 => {
    //   block [0x83298B98..0x83298BCC)
	// 83298B98: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 83298B9C: 3D00820F  lis r8, -0x7df1
	ctx.r[8].s64 = -2112946176;
	// 83298BA0: 3CE08333  lis r7, -0x7ccd
	ctx.r[7].s64 = -2093809664;
	// 83298BA4: 3D6082C6  lis r11, -0x7d3a
	ctx.r[11].s64 = -2100953088;
	// 83298BA8: 3D4082C6  lis r10, -0x7d3a
	ctx.r[10].s64 = -2100953088;
	// 83298BAC: 38C9DAA8  addi r6, r9, -0x2558
	ctx.r[6].s64 = ctx.r[9].s64 + -9560;
	// 83298BB0: 38883D28  addi r4, r8, 0x3d28
	ctx.r[4].s64 = ctx.r[8].s64 + 15656;
	// 83298BB4: 38675890  addi r3, r7, 0x5890
	ctx.r[3].s64 = ctx.r[7].s64 + 22672;
	// 83298BB8: 392B21E0  addi r9, r11, 0x21e0
	ctx.r[9].s64 = ctx.r[11].s64 + 8672;
	// 83298BBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83298BC0: 38EA2170  addi r7, r10, 0x2170
	ctx.r[7].s64 = ctx.r[10].s64 + 8560;
	// 83298BC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83298BC8: 4B9CB2F0  b 0x82c63eb8
	sub_82C63EB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298BD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83298BD0 size=44
    let mut pc: u32 = 0x83298BD0;
    'dispatch: loop {
        match pc {
            0x83298BD0 => {
    //   block [0x83298BD0..0x83298BFC)
	// 83298BD0: 3D208333  lis r9, -0x7ccd
	ctx.r[9].s64 = -2093809664;
	// 83298BD4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83298BD8: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 83298BDC: 3869591C  addi r3, r9, 0x591c
	ctx.r[3].s64 = ctx.r[9].s64 + 22812;
	// 83298BE0: 38CBDAA8  addi r6, r11, -0x2558
	ctx.r[6].s64 = ctx.r[11].s64 + -9560;
	// 83298BE4: 388A3D28  addi r4, r10, 0x3d28
	ctx.r[4].s64 = ctx.r[10].s64 + 15656;
	// 83298BE8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83298BEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83298BF0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83298BF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83298BF8: 4B9CB2C0  b 0x82c63eb8
	sub_82C63EB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298C00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83298C00 size=44
    let mut pc: u32 = 0x83298C00;
    'dispatch: loop {
        match pc {
            0x83298C00 => {
    //   block [0x83298C00..0x83298C2C)
	// 83298C00: 3D208333  lis r9, -0x7ccd
	ctx.r[9].s64 = -2093809664;
	// 83298C04: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83298C08: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 83298C0C: 386957CC  addi r3, r9, 0x57cc
	ctx.r[3].s64 = ctx.r[9].s64 + 22476;
	// 83298C10: 38CBDA98  addi r6, r11, -0x2568
	ctx.r[6].s64 = ctx.r[11].s64 + -9576;
	// 83298C14: 388ADBB0  addi r4, r10, -0x2450
	ctx.r[4].s64 = ctx.r[10].s64 + -9296;
	// 83298C18: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83298C1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83298C20: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83298C24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83298C28: 4B9CB290  b 0x82c63eb8
	sub_82C63EB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298C30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83298C30 size=44
    let mut pc: u32 = 0x83298C30;
    'dispatch: loop {
        match pc {
            0x83298C30 => {
    //   block [0x83298C30..0x83298C5C)
	// 83298C30: 3D208333  lis r9, -0x7ccd
	ctx.r[9].s64 = -2093809664;
	// 83298C34: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83298C38: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 83298C3C: 38695874  addi r3, r9, 0x5874
	ctx.r[3].s64 = ctx.r[9].s64 + 22644;
	// 83298C40: 38CBDBB8  addi r6, r11, -0x2448
	ctx.r[6].s64 = ctx.r[11].s64 + -9288;
	// 83298C44: 388ADBAC  addi r4, r10, -0x2454
	ctx.r[4].s64 = ctx.r[10].s64 + -9300;
	// 83298C48: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83298C4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83298C50: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83298C54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83298C58: 4B9CB260  b 0x82c63eb8
	sub_82C63EB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83298C60 size=48
    let mut pc: u32 = 0x83298C60;
    'dispatch: loop {
        match pc {
            0x83298C60 => {
    //   block [0x83298C60..0x83298C90)
	// 83298C60: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 83298C64: 3D008333  lis r8, -0x7ccd
	ctx.r[8].s64 = -2093809664;
	// 83298C68: 3D6082C6  lis r11, -0x7d3a
	ctx.r[11].s64 = -2100953088;
	// 83298C6C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 83298C70: 3889DBA8  addi r4, r9, -0x2458
	ctx.r[4].s64 = ctx.r[9].s64 + -9304;
	// 83298C74: 38685778  addi r3, r8, 0x5778
	ctx.r[3].s64 = ctx.r[8].s64 + 22392;
	// 83298C78: 38CADAA8  addi r6, r10, -0x2558
	ctx.r[6].s64 = ctx.r[10].s64 + -9560;
	// 83298C7C: 392B23C8  addi r9, r11, 0x23c8
	ctx.r[9].s64 = ctx.r[11].s64 + 9160;
	// 83298C80: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83298C84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83298C88: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83298C8C: 4B9CB22C  b 0x82c63eb8
	sub_82C63EB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298C90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83298C90 size=52
    let mut pc: u32 = 0x83298C90;
    'dispatch: loop {
        match pc {
            0x83298C90 => {
    //   block [0x83298C90..0x83298CC4)
	// 83298C90: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 83298C94: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 83298C98: 3CE08333  lis r7, -0x7ccd
	ctx.r[7].s64 = -2093809664;
	// 83298C9C: 3D6082C6  lis r11, -0x7d3a
	ctx.r[11].s64 = -2100953088;
	// 83298CA0: 3D4082C6  lis r10, -0x7d3a
	ctx.r[10].s64 = -2100953088;
	// 83298CA4: 38C9DAA8  addi r6, r9, -0x2558
	ctx.r[6].s64 = ctx.r[9].s64 + -9560;
	// 83298CA8: 3888DBA8  addi r4, r8, -0x2458
	ctx.r[4].s64 = ctx.r[8].s64 + -9304;
	// 83298CAC: 38675954  addi r3, r7, 0x5954
	ctx.r[3].s64 = ctx.r[7].s64 + 22868;
	// 83298CB0: 392B23C8  addi r9, r11, 0x23c8
	ctx.r[9].s64 = ctx.r[11].s64 + 9160;
	// 83298CB4: 390A24B0  addi r8, r10, 0x24b0
	ctx.r[8].s64 = ctx.r[10].s64 + 9392;
	// 83298CB8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83298CBC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83298CC0: 4B9CB1F8  b 0x82c63eb8
	sub_82C63EB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298CC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83298CC8 size=44
    let mut pc: u32 = 0x83298CC8;
    'dispatch: loop {
        match pc {
            0x83298CC8 => {
    //   block [0x83298CC8..0x83298CF4)
	// 83298CC8: 3D208333  lis r9, -0x7ccd
	ctx.r[9].s64 = -2093809664;
	// 83298CCC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83298CD0: 3D408210  lis r10, -0x7df0
	ctx.r[10].s64 = -2112880640;
	// 83298CD4: 386957E8  addi r3, r9, 0x57e8
	ctx.r[3].s64 = ctx.r[9].s64 + 22504;
	// 83298CD8: 38CBDBB8  addi r6, r11, -0x2448
	ctx.r[6].s64 = ctx.r[11].s64 + -9288;
	// 83298CDC: 388AFCA8  addi r4, r10, -0x358
	ctx.r[4].s64 = ctx.r[10].s64 + -856;
	// 83298CE0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83298CE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83298CE8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83298CEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83298CF0: 4B9CB1C8  b 0x82c63eb8
	sub_82C63EB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298CF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83298CF8 size=44
    let mut pc: u32 = 0x83298CF8;
    'dispatch: loop {
        match pc {
            0x83298CF8 => {
    //   block [0x83298CF8..0x83298D24)
	// 83298CF8: 3D208333  lis r9, -0x7ccd
	ctx.r[9].s64 = -2093809664;
	// 83298CFC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83298D00: 3D408210  lis r10, -0x7df0
	ctx.r[10].s64 = -2112880640;
	// 83298D04: 386958AC  addi r3, r9, 0x58ac
	ctx.r[3].s64 = ctx.r[9].s64 + 22700;
	// 83298D08: 38CBDA98  addi r6, r11, -0x2568
	ctx.r[6].s64 = ctx.r[11].s64 + -9576;
	// 83298D0C: 388AFCA8  addi r4, r10, -0x358
	ctx.r[4].s64 = ctx.r[10].s64 + -856;
	// 83298D10: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83298D14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83298D18: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83298D1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83298D20: 4B9CB198  b 0x82c63eb8
	sub_82C63EB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83298D28 size=52
    let mut pc: u32 = 0x83298D28;
    'dispatch: loop {
        match pc {
            0x83298D28 => {
    //   block [0x83298D28..0x83298D5C)
	// 83298D28: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 83298D2C: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 83298D30: 3CE08333  lis r7, -0x7ccd
	ctx.r[7].s64 = -2093809664;
	// 83298D34: 3D6082C6  lis r11, -0x7d3a
	ctx.r[11].s64 = -2100953088;
	// 83298D38: 3D4082C6  lis r10, -0x7d3a
	ctx.r[10].s64 = -2100953088;
	// 83298D3C: 38C9DAA8  addi r6, r9, -0x2558
	ctx.r[6].s64 = ctx.r[9].s64 + -9560;
	// 83298D40: 3888DBA8  addi r4, r8, -0x2458
	ctx.r[4].s64 = ctx.r[8].s64 + -9304;
	// 83298D44: 3867583C  addi r3, r7, 0x583c
	ctx.r[3].s64 = ctx.r[7].s64 + 22588;
	// 83298D48: 392B2308  addi r9, r11, 0x2308
	ctx.r[9].s64 = ctx.r[11].s64 + 8968;
	// 83298D4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83298D50: 38EA2298  addi r7, r10, 0x2298
	ctx.r[7].s64 = ctx.r[10].s64 + 8856;
	// 83298D54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83298D58: 4B9CB160  b 0x82c63eb8
	sub_82C63EB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298D60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83298D60 size=44
    let mut pc: u32 = 0x83298D60;
    'dispatch: loop {
        match pc {
            0x83298D60 => {
    //   block [0x83298D60..0x83298D8C)
	// 83298D60: 3D208333  lis r9, -0x7ccd
	ctx.r[9].s64 = -2093809664;
	// 83298D64: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83298D68: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 83298D6C: 38695740  addi r3, r9, 0x5740
	ctx.r[3].s64 = ctx.r[9].s64 + 22336;
	// 83298D70: 38CBDBB8  addi r6, r11, -0x2448
	ctx.r[6].s64 = ctx.r[11].s64 + -9288;
	// 83298D74: 388A0CA0  addi r4, r10, 0xca0
	ctx.r[4].s64 = ctx.r[10].s64 + 3232;
	// 83298D78: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83298D7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83298D80: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83298D84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83298D88: 4B9CB130  b 0x82c63eb8
	sub_82C63EB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83298D90 size=48
    let mut pc: u32 = 0x83298D90;
    'dispatch: loop {
        match pc {
            0x83298D90 => {
    //   block [0x83298D90..0x83298DC0)
	// 83298D90: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83298D94: 3CE08333  lis r7, -0x7ccd
	ctx.r[7].s64 = -2093809664;
	// 83298D98: 3D4082C6  lis r10, -0x7d3a
	ctx.r[10].s64 = -2100953088;
	// 83298D9C: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83298DA0: 3D0082C6  lis r8, -0x7d3a
	ctx.r[8].s64 = -2100953088;
	// 83298DA4: 386757B0  addi r3, r7, 0x57b0
	ctx.r[3].s64 = ctx.r[7].s64 + 22448;
	// 83298DA8: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 83298DAC: 392A27D0  addi r9, r10, 0x27d0
	ctx.r[9].s64 = ctx.r[10].s64 + 10192;
	// 83298DB0: 390826F8  addi r8, r8, 0x26f8
	ctx.r[8].s64 = ctx.r[8].s64 + 9976;
	// 83298DB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83298DB8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83298DBC: 4B9CB0FC  b 0x82c63eb8
	sub_82C63EB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83298DC0 size=44
    let mut pc: u32 = 0x83298DC0;
    'dispatch: loop {
        match pc {
            0x83298DC0 => {
    //   block [0x83298DC0..0x83298DEC)
	// 83298DC0: 3D208333  lis r9, -0x7ccd
	ctx.r[9].s64 = -2093809664;
	// 83298DC4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83298DC8: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 83298DCC: 38695938  addi r3, r9, 0x5938
	ctx.r[3].s64 = ctx.r[9].s64 + 22840;
	// 83298DD0: 38CBDBA0  addi r6, r11, -0x2460
	ctx.r[6].s64 = ctx.r[11].s64 + -9312;
	// 83298DD4: 388A0CA0  addi r4, r10, 0xca0
	ctx.r[4].s64 = ctx.r[10].s64 + 3232;
	// 83298DD8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83298DDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83298DE0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83298DE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83298DE8: 4B9CB0D0  b 0x82c63eb8
	sub_82C63EB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298DF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83298DF0 size=48
    let mut pc: u32 = 0x83298DF0;
    'dispatch: loop {
        match pc {
            0x83298DF0 => {
    //   block [0x83298DF0..0x83298E20)
	// 83298DF0: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 83298DF4: 3D008333  lis r8, -0x7ccd
	ctx.r[8].s64 = -2093809664;
	// 83298DF8: 3D6082C6  lis r11, -0x7d3a
	ctx.r[11].s64 = -2100953088;
	// 83298DFC: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 83298E00: 38890CA0  addi r4, r9, 0xca0
	ctx.r[4].s64 = ctx.r[9].s64 + 3232;
	// 83298E04: 3868575C  addi r3, r8, 0x575c
	ctx.r[3].s64 = ctx.r[8].s64 + 22364;
	// 83298E08: 38CADB98  addi r6, r10, -0x2468
	ctx.r[6].s64 = ctx.r[10].s64 + -9320;
	// 83298E0C: 392B2980  addi r9, r11, 0x2980
	ctx.r[9].s64 = ctx.r[11].s64 + 10624;
	// 83298E10: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83298E14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83298E18: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83298E1C: 4B9CB09C  b 0x82c63eb8
	sub_82C63EB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298E20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83298E20 size=44
    let mut pc: u32 = 0x83298E20;
    'dispatch: loop {
        match pc {
            0x83298E20 => {
    //   block [0x83298E20..0x83298E4C)
	// 83298E20: 3D208333  lis r9, -0x7ccd
	ctx.r[9].s64 = -2093809664;
	// 83298E24: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83298E28: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 83298E2C: 386958C8  addi r3, r9, 0x58c8
	ctx.r[3].s64 = ctx.r[9].s64 + 22728;
	// 83298E30: 38CBDAA8  addi r6, r11, -0x2558
	ctx.r[6].s64 = ctx.r[11].s64 + -9560;
	// 83298E34: 388A34A8  addi r4, r10, 0x34a8
	ctx.r[4].s64 = ctx.r[10].s64 + 13480;
	// 83298E38: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83298E3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83298E40: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83298E44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83298E48: 4B9CB070  b 0x82c63eb8
	sub_82C63EB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298E50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83298E50 size=44
    let mut pc: u32 = 0x83298E50;
    'dispatch: loop {
        match pc {
            0x83298E50 => {
    //   block [0x83298E50..0x83298E7C)
	// 83298E50: 3D208333  lis r9, -0x7ccd
	ctx.r[9].s64 = -2093809664;
	// 83298E54: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83298E58: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 83298E5C: 38695820  addi r3, r9, 0x5820
	ctx.r[3].s64 = ctx.r[9].s64 + 22560;
	// 83298E60: 38CBDAA8  addi r6, r11, -0x2558
	ctx.r[6].s64 = ctx.r[11].s64 + -9560;
	// 83298E64: 388A0CA0  addi r4, r10, 0xca0
	ctx.r[4].s64 = ctx.r[10].s64 + 3232;
	// 83298E68: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83298E6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83298E70: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83298E74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83298E78: 4B9CB040  b 0x82c63eb8
	sub_82C63EB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83298E80 size=56
    let mut pc: u32 = 0x83298E80;
    'dispatch: loop {
        match pc {
            0x83298E80 => {
    //   block [0x83298E80..0x83298EB8)
	// 83298E80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83298E84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83298E88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83298E8C: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 83298E90: 3C800010  lis r4, 0x10
	ctx.r[4].s64 = 1048576;
	// 83298E94: 386B597C  addi r3, r11, 0x597c
	ctx.r[3].s64 = ctx.r[11].s64 + 22908;
	// 83298E98: 4B9D18E1  bl 0x82c6a778
	ctx.lr = 0x83298E9C;
	sub_82C6A778(ctx, base);
	// 83298E9C: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 83298EA0: 386A7BF0  addi r3, r10, 0x7bf0
	ctx.r[3].s64 = ctx.r[10].s64 + 31728;
	// 83298EA4: 4BA1107D  bl 0x82ca9f20
	ctx.lr = 0x83298EA8;
	sub_82CA9F20(ctx, base);
	// 83298EA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83298EAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83298EB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83298EB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298EB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83298EB8 size=52
    let mut pc: u32 = 0x83298EB8;
    'dispatch: loop {
        match pc {
            0x83298EB8 => {
    //   block [0x83298EB8..0x83298EEC)
	// 83298EB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83298EBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83298EC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83298EC4: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 83298EC8: 386B5990  addi r3, r11, 0x5990
	ctx.r[3].s64 = ctx.r[11].s64 + 22928;
	// 83298ECC: 4B9D6865  bl 0x82c6f730
	ctx.lr = 0x83298ED0;
	sub_82C6F730(ctx, base);
	// 83298ED0: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 83298ED4: 386A7C88  addi r3, r10, 0x7c88
	ctx.r[3].s64 = ctx.r[10].s64 + 31880;
	// 83298ED8: 4BA11049  bl 0x82ca9f20
	ctx.lr = 0x83298EDC;
	sub_82CA9F20(ctx, base);
	// 83298EDC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83298EE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83298EE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83298EE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298EF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83298EF0 size=52
    let mut pc: u32 = 0x83298EF0;
    'dispatch: loop {
        match pc {
            0x83298EF0 => {
    //   block [0x83298EF0..0x83298F24)
	// 83298EF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83298EF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83298EF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83298EFC: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 83298F00: 386B5998  addi r3, r11, 0x5998
	ctx.r[3].s64 = ctx.r[11].s64 + 22936;
	// 83298F04: 4AF5E8FD  bl 0x821f7800
	ctx.lr = 0x83298F08;
	sub_821F7800(ctx, base);
	// 83298F08: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 83298F0C: 386A7C18  addi r3, r10, 0x7c18
	ctx.r[3].s64 = ctx.r[10].s64 + 31768;
	// 83298F10: 4BA11011  bl 0x82ca9f20
	ctx.lr = 0x83298F14;
	sub_82CA9F20(ctx, base);
	// 83298F14: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83298F18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83298F1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83298F20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298F28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83298F28 size=52
    let mut pc: u32 = 0x83298F28;
    'dispatch: loop {
        match pc {
            0x83298F28 => {
    //   block [0x83298F28..0x83298F5C)
	// 83298F28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83298F2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83298F30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83298F34: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 83298F38: 386B5994  addi r3, r11, 0x5994
	ctx.r[3].s64 = ctx.r[11].s64 + 22932;
	// 83298F3C: 4AF5E8C5  bl 0x821f7800
	ctx.lr = 0x83298F40;
	sub_821F7800(ctx, base);
	// 83298F40: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 83298F44: 386A7C28  addi r3, r10, 0x7c28
	ctx.r[3].s64 = ctx.r[10].s64 + 31784;
	// 83298F48: 4BA10FD9  bl 0x82ca9f20
	ctx.lr = 0x83298F4C;
	sub_82CA9F20(ctx, base);
	// 83298F4C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83298F50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83298F54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83298F58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298F60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83298F60 size=148
    let mut pc: u32 = 0x83298F60;
    'dispatch: loop {
        match pc {
            0x83298F60 => {
    //   block [0x83298F60..0x83298FB4)
	// 83298F60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83298F64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83298F68: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83298F6C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83298F70: 3D60832F  lis r11, -0x7cd1
	ctx.r[11].s64 = -2094071808;
	// 83298F74: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83298F78: 3BEBE8D4  addi r31, r11, -0x172c
	ctx.r[31].s64 = ctx.r[11].s64 + -5932;
	// 83298F7C: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 83298F80: 915F000C  stw r10, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 83298F84: 4B8A492D  bl 0x82b3d8b0
	ctx.lr = 0x83298F88;
	sub_82B3D8B0(ctx, base);
	// 83298F88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83298F8C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83298F90: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 83298F94: 387F0034  addi r3, r31, 0x34
	ctx.r[3].s64 = ctx.r[31].s64 + 52;
	// 83298F98: 915F0030  stw r10, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[10].u32 ) };
	// 83298F9C: 4B9DDB5D  bl 0x82c76af8
	ctx.lr = 0x83298FA0;
	sub_82C76AF8(ctx, base);
	// 83298FA0: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 83298FA4: 4AF862B5  bl 0x8221f258
	ctx.lr = 0x83298FA8;
	sub_8221F258(ctx, base);
	// 83298FA8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83298FAC: 419A0008  beq cr6, 0x83298fb4
	if ctx.cr[6].eq {
	pc = 0x83298FB4; continue 'dispatch;
	}
	// 83298FB0: 90630000  stw r3, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	pc = 0x83298FB4; continue 'dispatch;
            }
            0x83298FB4 => {
    //   block [0x83298FB4..0x83298FC0)
	// 83298FB4: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83298FB8: 41820008  beq 0x83298fc0
	if ctx.cr[0].eq {
	pc = 0x83298FC0; continue 'dispatch;
	}
	// 83298FBC: 906B0000  stw r3, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	pc = 0x83298FC0; continue 'dispatch;
            }
            0x83298FC0 => {
    //   block [0x83298FC0..0x83298FF4)
	// 83298FC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83298FC4: 907F0044  stw r3, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[3].u32 ) };
	// 83298FC8: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 83298FCC: 917F0048  stw r11, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 83298FD0: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83298FD4: 915F004C  stw r10, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[10].u32 ) };
	// 83298FD8: 386B7CC0  addi r3, r11, 0x7cc0
	ctx.r[3].s64 = ctx.r[11].s64 + 31936;
	// 83298FDC: 4BA10F45  bl 0x82ca9f20
	ctx.lr = 0x83298FE0;
	sub_82CA9F20(ctx, base);
	// 83298FE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83298FE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83298FE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83298FEC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83298FF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83298FF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83298FF8 size=52
    let mut pc: u32 = 0x83298FF8;
    'dispatch: loop {
        match pc {
            0x83298FF8 => {
    //   block [0x83298FF8..0x8329902C)
	// 83298FF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83298FFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83299000: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83299004: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 83299008: 386B59AC  addi r3, r11, 0x59ac
	ctx.r[3].s64 = ctx.r[11].s64 + 22956;
	// 8329900C: 4B8A48A5  bl 0x82b3d8b0
	ctx.lr = 0x83299010;
	sub_82B3D8B0(ctx, base);
	// 83299010: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 83299014: 386A7CD0  addi r3, r10, 0x7cd0
	ctx.r[3].s64 = ctx.r[10].s64 + 31952;
	// 83299018: 4BA10F09  bl 0x82ca9f20
	ctx.lr = 0x8329901C;
	sub_82CA9F20(ctx, base);
	// 8329901C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83299020: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83299024: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83299028: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83299030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83299030 size=4
    let mut pc: u32 = 0x83299030;
    'dispatch: loop {
        match pc {
            0x83299030 => {
    //   block [0x83299030..0x83299034)
	// 83299030: 4B9EEFC0  b 0x82c87ff0
	sub_82C87FF0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83299038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83299038 size=12
    let mut pc: u32 = 0x83299038;
    'dispatch: loop {
        match pc {
            0x83299038 => {
    //   block [0x83299038..0x83299044)
	// 83299038: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8329903C: 386B7D90  addi r3, r11, 0x7d90
	ctx.r[3].s64 = ctx.r[11].s64 + 32144;
	// 83299040: 4BA10EE0  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83299048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83299048 size=12
    let mut pc: u32 = 0x83299048;
    'dispatch: loop {
        match pc {
            0x83299048 => {
    //   block [0x83299048..0x83299054)
	// 83299048: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8329904C: 386B7D38  addi r3, r11, 0x7d38
	ctx.r[3].s64 = ctx.r[11].s64 + 32056;
	// 83299050: 4BA10ED0  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83299058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83299058 size=12
    let mut pc: u32 = 0x83299058;
    'dispatch: loop {
        match pc {
            0x83299058 => {
    //   block [0x83299058..0x83299064)
	// 83299058: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8329905C: 386B7CE0  addi r3, r11, 0x7ce0
	ctx.r[3].s64 = ctx.r[11].s64 + 31968;
	// 83299060: 4BA10EC0  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83299068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83299068 size=12
    let mut pc: u32 = 0x83299068;
    'dispatch: loop {
        match pc {
            0x83299068 => {
    //   block [0x83299068..0x83299074)
	// 83299068: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8329906C: 386B7E08  addi r3, r11, 0x7e08
	ctx.r[3].s64 = ctx.r[11].s64 + 32264;
	// 83299070: 4BA10EB0  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83299078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83299078 size=52
    let mut pc: u32 = 0x83299078;
    'dispatch: loop {
        match pc {
            0x83299078 => {
    //   block [0x83299078..0x832990AC)
	// 83299078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329907C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83299080: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83299084: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 83299088: 386B71D0  addi r3, r11, 0x71d0
	ctx.r[3].s64 = ctx.r[11].s64 + 29136;
	// 8329908C: 4BA3F495  bl 0x82cd8520
	ctx.lr = 0x83299090;
	sub_82CD8520(ctx, base);
	// 83299090: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83299094: 386B7E58  addi r3, r11, 0x7e58
	ctx.r[3].s64 = ctx.r[11].s64 + 32344;
	// 83299098: 4BA10E89  bl 0x82ca9f20
	ctx.lr = 0x8329909C;
	sub_82CA9F20(ctx, base);
	// 8329909C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832990A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832990A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832990A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832990B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832990B0 size=52
    let mut pc: u32 = 0x832990B0;
    'dispatch: loop {
        match pc {
            0x832990B0 => {
    //   block [0x832990B0..0x832990E4)
	// 832990B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832990B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832990B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832990BC: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832990C0: 386B721C  addi r3, r11, 0x721c
	ctx.r[3].s64 = ctx.r[11].s64 + 29212;
	// 832990C4: 4BA3F45D  bl 0x82cd8520
	ctx.lr = 0x832990C8;
	sub_82CD8520(ctx, base);
	// 832990C8: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 832990CC: 386B7E68  addi r3, r11, 0x7e68
	ctx.r[3].s64 = ctx.r[11].s64 + 32360;
	// 832990D0: 4BA10E51  bl 0x82ca9f20
	ctx.lr = 0x832990D4;
	sub_82CA9F20(ctx, base);
	// 832990D4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832990D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832990DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832990E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832990E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832990E8 size=12
    let mut pc: u32 = 0x832990E8;
    'dispatch: loop {
        match pc {
            0x832990E8 => {
    //   block [0x832990E8..0x832990F4)
	// 832990E8: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 832990EC: 386B7E78  addi r3, r11, 0x7e78
	ctx.r[3].s64 = ctx.r[11].s64 + 32376;
	// 832990F0: 4BA10E30  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832990F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832990F8 size=56
    let mut pc: u32 = 0x832990F8;
    'dispatch: loop {
        match pc {
            0x832990F8 => {
    //   block [0x832990F8..0x83299130)
	// 832990F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832990FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83299100: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83299104: 3D60832F  lis r11, -0x7cd1
	ctx.r[11].s64 = -2094071808;
	// 83299108: 396B0440  addi r11, r11, 0x440
	ctx.r[11].s64 = ctx.r[11].s64 + 1088;
	// 8329910C: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 83299110: 48020B75  bl 0x832b9c84
	ctx.lr = 0x83299114;
	// extern call 0x832B9C84 → crate::xboxkrnl::RtlInitializeCriticalSection
	crate::xboxkrnl::RtlInitializeCriticalSection(ctx, base);
	// 83299114: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 83299118: 386A7E88  addi r3, r10, 0x7e88
	ctx.r[3].s64 = ctx.r[10].s64 + 32392;
	// 8329911C: 4BA10E05  bl 0x82ca9f20
	ctx.lr = 0x83299120;
	sub_82CA9F20(ctx, base);
	// 83299120: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83299124: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83299128: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329912C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83299130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83299130 size=56
    let mut pc: u32 = 0x83299130;
    'dispatch: loop {
        match pc {
            0x83299130 => {
    //   block [0x83299130..0x83299168)
	// 83299130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83299134: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83299138: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329913C: 3D60832F  lis r11, -0x7cd1
	ctx.r[11].s64 = -2094071808;
	// 83299140: 396B0460  addi r11, r11, 0x460
	ctx.r[11].s64 = ctx.r[11].s64 + 1120;
	// 83299144: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 83299148: 48020B3D  bl 0x832b9c84
	ctx.lr = 0x8329914C;
	// extern call 0x832B9C84 → crate::xboxkrnl::RtlInitializeCriticalSection
	crate::xboxkrnl::RtlInitializeCriticalSection(ctx, base);
	// 8329914C: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 83299150: 386A7EA0  addi r3, r10, 0x7ea0
	ctx.r[3].s64 = ctx.r[10].s64 + 32416;
	// 83299154: 4BA10DCD  bl 0x82ca9f20
	ctx.lr = 0x83299158;
	sub_82CA9F20(ctx, base);
	// 83299158: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8329915C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83299160: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83299164: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83299168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83299168 size=56
    let mut pc: u32 = 0x83299168;
    'dispatch: loop {
        match pc {
            0x83299168 => {
    //   block [0x83299168..0x832991A0)
	// 83299168: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329916C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83299170: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83299174: 3D60832F  lis r11, -0x7cd1
	ctx.r[11].s64 = -2094071808;
	// 83299178: 396B04AC  addi r11, r11, 0x4ac
	ctx.r[11].s64 = ctx.r[11].s64 + 1196;
	// 8329917C: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 83299180: 48020B05  bl 0x832b9c84
	ctx.lr = 0x83299184;
	// extern call 0x832B9C84 → crate::xboxkrnl::RtlInitializeCriticalSection
	crate::xboxkrnl::RtlInitializeCriticalSection(ctx, base);
	// 83299184: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83299188: 386B7EB8  addi r3, r11, 0x7eb8
	ctx.r[3].s64 = ctx.r[11].s64 + 32440;
	// 8329918C: 4BA10D95  bl 0x82ca9f20
	ctx.lr = 0x83299190;
	sub_82CA9F20(ctx, base);
	// 83299190: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83299194: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83299198: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329919C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832991A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832991A0 size=20
    let mut pc: u32 = 0x832991A0;
    'dispatch: loop {
        match pc {
            0x832991A0 => {
    //   block [0x832991A0..0x832991B4)
	// 832991A0: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832991A4: 38A0002C  li r5, 0x2c
	ctx.r[5].s64 = 44;
	// 832991A8: 386B72F8  addi r3, r11, 0x72f8
	ctx.r[3].s64 = ctx.r[11].s64 + 29432;
	// 832991AC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 832991B0: 4BA10800  b 0x82ca99b0
	sub_82CA99B0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832991B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832991B8 size=28
    let mut pc: u32 = 0x832991B8;
    'dispatch: loop {
        match pc {
            0x832991B8 => {
    //   block [0x832991B8..0x832991D4)
	// 832991B8: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 832991BC: 3D60832F  lis r11, -0x7cd1
	ctx.r[11].s64 = -2094071808;
	// 832991C0: 396B6E4C  addi r11, r11, 0x6e4c
	ctx.r[11].s64 = ctx.r[11].s64 + 28236;
	// 832991C4: 812A76FC  lwz r9, 0x76fc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(30460 as u32) ) } as u64;
	// 832991C8: 916A76FC  stw r11, 0x76fc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(30460 as u32), ctx.r[11].u32 ) };
	// 832991CC: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 832991D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832991D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x832991D8 size=56
    let mut pc: u32 = 0x832991D8;
    'dispatch: loop {
        match pc {
            0x832991D8 => {
    //   block [0x832991D8..0x83299210)
	// 832991D8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 832991DC: C00B0C18  lfs f0, 0xc18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832991E0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 832991E4: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 832991E8: D001FFF4  stfs f0, -0xc(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-12 as u32), tmp.u32 ) };
	// 832991EC: D001FFF8  stfs f0, -8(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), tmp.u32 ) };
	// 832991F0: C00B0C14  lfs f0, 0xc14(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3092 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832991F4: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832991F8: D001FFFC  stfs f0, -4(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 832991FC: 3941FFF0  addi r10, r1, -0x10
	ctx.r[10].s64 = ctx.r[1].s64 + -16;
	// 83299200: 396B7420  addi r11, r11, 0x7420
	ctx.r[11].s64 = ctx.r[11].s64 + 29728;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83299210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83299210 size=528
    let mut pc: u32 = 0x83299210;
    'dispatch: loop {
        match pc {
            0x83299210 => {
    //   block [0x83299210..0x83299420)
	// 83299210: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83299214: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 83299218: 3D208333  lis r9, -0x7ccd
	ctx.r[9].s64 = -2093809664;
	// 8329921C: 39297430  addi r9, r9, 0x7430
	ctx.r[9].s64 = ctx.r[9].s64 + 29744;
	// 83299220: 9141FF20  stw r10, -0xe0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-224 as u32), ctx.r[10].u32 ) };
	// 83299224: 9141FF24  stw r10, -0xdc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-220 as u32), ctx.r[10].u32 ) };
	// 83299228: 9141FF28  stw r10, -0xd8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-216 as u32), ctx.r[10].u32 ) };
	// 8329922C: 9141FF2C  stw r10, -0xd4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-212 as u32), ctx.r[10].u32 ) };
	// 83299230: 3901FF20  addi r8, r1, -0xe0
	ctx.r[8].s64 = ctx.r[1].s64 + -224;
	// 83299234: 9141FF30  stw r10, -0xd0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-208 as u32), ctx.r[10].u32 ) };
	// 83299238: 9141FF34  stw r10, -0xcc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-204 as u32), ctx.r[10].u32 ) };
	// 8329923C: 9141FF38  stw r10, -0xc8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-200 as u32), ctx.r[10].u32 ) };
	// 83299240: 9161FF3C  stw r11, -0xc4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-196 as u32), ctx.r[11].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83299420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83299420 size=528
    let mut pc: u32 = 0x83299420;
    'dispatch: loop {
        match pc {
            0x83299420 => {
    //   block [0x83299420..0x83299630)
	// 83299420: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 83299424: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83299428: 3D208333  lis r9, -0x7ccd
	ctx.r[9].s64 = -2093809664;
	// 8329942C: 39297530  addi r9, r9, 0x7530
	ctx.r[9].s64 = ctx.r[9].s64 + 30000;
	// 83299430: 9161FF20  stw r11, -0xe0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-224 as u32), ctx.r[11].u32 ) };
	// 83299434: 9161FF24  stw r11, -0xdc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-220 as u32), ctx.r[11].u32 ) };
	// 83299438: 9161FF28  stw r11, -0xd8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-216 as u32), ctx.r[11].u32 ) };
	// 8329943C: 9161FF2C  stw r11, -0xd4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-212 as u32), ctx.r[11].u32 ) };
	// 83299440: 3901FF20  addi r8, r1, -0xe0
	ctx.r[8].s64 = ctx.r[1].s64 + -224;
	// 83299444: 9161FF30  stw r11, -0xd0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-208 as u32), ctx.r[11].u32 ) };
	// 83299448: 9161FF34  stw r11, -0xcc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-204 as u32), ctx.r[11].u32 ) };
	// 8329944C: 9161FF38  stw r11, -0xc8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-200 as u32), ctx.r[11].u32 ) };
	// 83299450: 9141FF3C  stw r10, -0xc4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-196 as u32), ctx.r[10].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83299630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83299630 size=24
    let mut pc: u32 = 0x83299630;
    'dispatch: loop {
        match pc {
            0x83299630 => {
    //   block [0x83299630..0x83299648)
	// 83299630: 3D60832F  lis r11, -0x7cd1
	ctx.r[11].s64 = -2094071808;
	// 83299634: 3D40832F  lis r10, -0x7cd1
	ctx.r[10].s64 = -2094071808;
	// 83299638: 394A6F38  addi r10, r10, 0x6f38
	ctx.r[10].s64 = ctx.r[10].s64 + 28472;
	// 8329963C: 816B6F24  lwz r11, 0x6f24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28452 as u32) ) } as u64;
	// 83299640: 916A00C8  stw r11, 0xc8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(200 as u32), ctx.r[11].u32 ) };
	// 83299644: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83299648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83299648 size=112
    let mut pc: u32 = 0x83299648;
    'dispatch: loop {
        match pc {
            0x83299648 => {
    //   block [0x83299648..0x832996B8)
	// 83299648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329964C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83299650: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83299654: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 83299658: 3D60832F  lis r11, -0x7cd1
	ctx.r[11].s64 = -2094071808;
	// 8329965C: 392A3CB0  addi r9, r10, 0x3cb0
	ctx.r[9].s64 = ctx.r[10].s64 + 15536;
	// 83299660: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 83299664: 390B6F38  addi r8, r11, 0x6f38
	ctx.r[8].s64 = ctx.r[11].s64 + 28472;
	// 83299668: 38E0000A  li r7, 0xa
	ctx.r[7].s64 = 10;
	// 8329966C: 388A3CD8  addi r4, r10, 0x3cd8
	ctx.r[4].s64 = ctx.r[10].s64 + 15576;
	// 83299670: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83299674: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83299678: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329967C: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 83299680: 386A7634  addi r3, r10, 0x7634
	ctx.r[3].s64 = ctx.r[10].s64 + 30260;
	// 83299684: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 83299688: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8329968C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83299690: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83299694: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83299698: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329969C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832996A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832996A4: 4BABC59D  bl 0x82d55c40
	ctx.lr = 0x832996A8;
	sub_82D55C40(ctx, base);
	// 832996A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832996AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832996B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832996B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832996B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832996B8 size=108
    let mut pc: u32 = 0x832996B8;
    'dispatch: loop {
        match pc {
            0x832996B8 => {
    //   block [0x832996B8..0x83299724)
	// 832996B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832996BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832996C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832996C4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 832996C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 832996CC: 38EB3D18  addi r7, r11, 0x3d18
	ctx.r[7].s64 = ctx.r[11].s64 + 15640;
	// 832996D0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 832996D4: 388A3D48  addi r4, r10, 0x3d48
	ctx.r[4].s64 = ctx.r[10].s64 + 15688;
	// 832996D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832996DC: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 832996E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832996E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832996E8: 386A7664  addi r3, r10, 0x7664
	ctx.r[3].s64 = ctx.r[10].s64 + 30308;
	// 832996EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832996F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832996F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832996F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832996FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83299700: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83299704: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 83299708: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329970C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83299710: 4BABC531  bl 0x82d55c40
	ctx.lr = 0x83299714;
	sub_82D55C40(ctx, base);
	// 83299714: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83299718: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329971C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83299720: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83299728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83299728 size=24
    let mut pc: u32 = 0x83299728;
    'dispatch: loop {
        match pc {
            0x83299728 => {
    //   block [0x83299728..0x83299740)
	// 83299728: 3D60832F  lis r11, -0x7cd1
	ctx.r[11].s64 = -2094071808;
	// 8329972C: 3D40832F  lis r10, -0x7cd1
	ctx.r[10].s64 = -2094071808;
	// 83299730: 394A7050  addi r10, r10, 0x7050
	ctx.r[10].s64 = ctx.r[10].s64 + 28752;
	// 83299734: 816B7028  lwz r11, 0x7028(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28712 as u32) ) } as u64;
	// 83299738: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8329973C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83299740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83299740 size=112
    let mut pc: u32 = 0x83299740;
    'dispatch: loop {
        match pc {
            0x83299740 => {
    //   block [0x83299740..0x832997B0)
	// 83299740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83299744: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83299748: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329974C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 83299750: 3D60832F  lis r11, -0x7cd1
	ctx.r[11].s64 = -2094071808;
	// 83299754: 392A3D04  addi r9, r10, 0x3d04
	ctx.r[9].s64 = ctx.r[10].s64 + 15620;
	// 83299758: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329975C: 390B7050  addi r8, r11, 0x7050
	ctx.r[8].s64 = ctx.r[11].s64 + 28752;
	// 83299760: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 83299764: 388A3D58  addi r4, r10, 0x3d58
	ctx.r[4].s64 = ctx.r[10].s64 + 15704;
	// 83299768: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329976C: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83299770: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 83299774: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 83299778: 386A7694  addi r3, r10, 0x7694
	ctx.r[3].s64 = ctx.r[10].s64 + 30356;
	// 8329977C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 83299780: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83299784: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83299788: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329978C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83299790: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83299794: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83299798: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329979C: 4BABC4A5  bl 0x82d55c40
	ctx.lr = 0x832997A0;
	sub_82D55C40(ctx, base);
	// 832997A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832997A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832997A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832997AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832997B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832997B0 size=40
    let mut pc: u32 = 0x832997B0;
    'dispatch: loop {
        match pc {
            0x832997B0 => {
    //   block [0x832997B0..0x832997D8)
	// 832997B0: 3D60832F  lis r11, -0x7cd1
	ctx.r[11].s64 = -2094071808;
	// 832997B4: 814B70B0  lwz r10, 0x70b0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28848 as u32) ) } as u64;
	// 832997B8: 3D60832F  lis r11, -0x7cd1
	ctx.r[11].s64 = -2094071808;
	// 832997BC: 396B70D0  addi r11, r11, 0x70d0
	ctx.r[11].s64 = ctx.r[11].s64 + 28880;
	// 832997C0: 914B0050  stw r10, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 832997C4: 914B0068  stw r10, 0x68(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(104 as u32), ctx.r[10].u32 ) };
	// 832997C8: 3D40832F  lis r10, -0x7cd1
	ctx.r[10].s64 = -2094071808;
	// 832997CC: 814A70B4  lwz r10, 0x70b4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28852 as u32) ) } as u64;
	// 832997D0: 914B0098  stw r10, 0x98(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(152 as u32), ctx.r[10].u32 ) };
	// 832997D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832997D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832997D8 size=112
    let mut pc: u32 = 0x832997D8;
    'dispatch: loop {
        match pc {
            0x832997D8 => {
    //   block [0x832997D8..0x83299848)
	// 832997D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832997DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832997E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832997E4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 832997E8: 3D60832F  lis r11, -0x7cd1
	ctx.r[11].s64 = -2094071808;
	// 832997EC: 392A4178  addi r9, r10, 0x4178
	ctx.r[9].s64 = ctx.r[10].s64 + 16760;
	// 832997F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 832997F4: 390B70D0  addi r8, r11, 0x70d0
	ctx.r[8].s64 = ctx.r[11].s64 + 28880;
	// 832997F8: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 832997FC: 388A41B4  addi r4, r10, 0x41b4
	ctx.r[4].s64 = ctx.r[10].s64 + 16820;
	// 83299800: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83299804: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83299808: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329980C: 38C00018  li r6, 0x18
	ctx.r[6].s64 = 24;
	// 83299810: 386A76C4  addi r3, r10, 0x76c4
	ctx.r[3].s64 = ctx.r[10].s64 + 30404;
	// 83299814: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 83299818: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 8329981C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83299820: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83299824: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83299828: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329982C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83299830: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83299834: 4BABC40D  bl 0x82d55c40
	ctx.lr = 0x83299838;
	sub_82D55C40(ctx, base);
	// 83299838: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329983C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83299840: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83299844: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83299848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83299848 size=28
    let mut pc: u32 = 0x83299848;
    'dispatch: loop {
        match pc {
            0x83299848 => {
    //   block [0x83299848..0x83299864)
	// 83299848: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 8329984C: 3D60832F  lis r11, -0x7cd1
	ctx.r[11].s64 = -2094071808;
	// 83299850: 396B7368  addi r11, r11, 0x7368
	ctx.r[11].s64 = ctx.r[11].s64 + 29544;
	// 83299854: 812A76FC  lwz r9, 0x76fc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(30460 as u32) ) } as u64;
	// 83299858: 916A76FC  stw r11, 0x76fc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(30460 as u32), ctx.r[11].u32 ) };
	// 8329985C: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 83299860: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83299868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83299868 size=108
    let mut pc: u32 = 0x83299868;
    'dispatch: loop {
        match pc {
            0x83299868 => {
    //   block [0x83299868..0x832998D4)
	// 83299868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329986C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83299870: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83299874: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 83299878: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329987C: 38EB4F8C  addi r7, r11, 0x4f8c
	ctx.r[7].s64 = ctx.r[11].s64 + 20364;
	// 83299880: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 83299884: 388A4FD4  addi r4, r10, 0x4fd4
	ctx.r[4].s64 = ctx.r[10].s64 + 20436;
	// 83299888: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329988C: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83299890: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 83299894: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83299898: 386A7704  addi r3, r10, 0x7704
	ctx.r[3].s64 = ctx.r[10].s64 + 30468;
	// 8329989C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832998A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832998A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832998A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832998AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832998B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832998B4: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 832998B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832998BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832998C0: 4BABC381  bl 0x82d55c40
	ctx.lr = 0x832998C4;
	sub_82D55C40(ctx, base);
	// 832998C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832998C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832998CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832998D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832998D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832998D8 size=108
    let mut pc: u32 = 0x832998D8;
    'dispatch: loop {
        match pc {
            0x832998D8 => {
    //   block [0x832998D8..0x83299944)
	// 832998D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832998DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832998E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832998E4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 832998E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 832998EC: 38EB4FBC  addi r7, r11, 0x4fbc
	ctx.r[7].s64 = ctx.r[11].s64 + 20412;
	// 832998F0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 832998F4: 388A4FF0  addi r4, r10, 0x4ff0
	ctx.r[4].s64 = ctx.r[10].s64 + 20464;
	// 832998F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832998FC: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83299900: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 83299904: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83299908: 386A7734  addi r3, r10, 0x7734
	ctx.r[3].s64 = ctx.r[10].s64 + 30516;
	// 8329990C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 83299910: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83299914: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83299918: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329991C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83299920: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83299924: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 83299928: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329992C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83299930: 4BABC311  bl 0x82d55c40
	ctx.lr = 0x83299934;
	sub_82D55C40(ctx, base);
	// 83299934: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83299938: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329993C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83299940: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83299948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83299948 size=28
    let mut pc: u32 = 0x83299948;
    'dispatch: loop {
        match pc {
            0x83299948 => {
    //   block [0x83299948..0x83299964)
	// 83299948: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 8329994C: 3D60832F  lis r11, -0x7cd1
	ctx.r[11].s64 = -2094071808;
	// 83299950: 396B7718  addi r11, r11, 0x7718
	ctx.r[11].s64 = ctx.r[11].s64 + 30488;
	// 83299954: 812A76FC  lwz r9, 0x76fc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(30460 as u32) ) } as u64;
	// 83299958: 916A76FC  stw r11, 0x76fc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(30460 as u32), ctx.r[11].u32 ) };
	// 8329995C: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 83299960: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83299968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83299968 size=28
    let mut pc: u32 = 0x83299968;
    'dispatch: loop {
        match pc {
            0x83299968 => {
    //   block [0x83299968..0x83299984)
	// 83299968: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 8329996C: 3D60832F  lis r11, -0x7cd1
	ctx.r[11].s64 = -2094071808;
	// 83299970: 396B772C  addi r11, r11, 0x772c
	ctx.r[11].s64 = ctx.r[11].s64 + 30508;
	// 83299974: 812A76FC  lwz r9, 0x76fc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(30460 as u32) ) } as u64;
	// 83299978: 916A76FC  stw r11, 0x76fc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(30460 as u32), ctx.r[11].u32 ) };
	// 8329997C: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 83299980: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83299988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83299988 size=108
    let mut pc: u32 = 0x83299988;
    'dispatch: loop {
        match pc {
            0x83299988 => {
    //   block [0x83299988..0x832999F4)
	// 83299988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329998C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83299990: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83299994: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 83299998: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329999C: 38EB5240  addi r7, r11, 0x5240
	ctx.r[7].s64 = ctx.r[11].s64 + 21056;
	// 832999A0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 832999A4: 388A52B8  addi r4, r10, 0x52b8
	ctx.r[4].s64 = ctx.r[10].s64 + 21176;
	// 832999A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832999AC: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 832999B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832999B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832999B8: 386A7774  addi r3, r10, 0x7774
	ctx.r[3].s64 = ctx.r[10].s64 + 30580;
	// 832999BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832999C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832999C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832999C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832999CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832999D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832999D4: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 832999D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832999DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832999E0: 4BABC261  bl 0x82d55c40
	ctx.lr = 0x832999E4;
	sub_82D55C40(ctx, base);
	// 832999E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832999E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832999EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832999F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832999F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832999F8 size=108
    let mut pc: u32 = 0x832999F8;
    'dispatch: loop {
        match pc {
            0x832999F8 => {
    //   block [0x832999F8..0x83299A64)
	// 832999F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832999FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83299A00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83299A04: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 83299A08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 83299A0C: 38EB5258  addi r7, r11, 0x5258
	ctx.r[7].s64 = ctx.r[11].s64 + 21080;
	// 83299A10: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 83299A14: 388A52D0  addi r4, r10, 0x52d0
	ctx.r[4].s64 = ctx.r[10].s64 + 21200;
	// 83299A18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83299A1C: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83299A20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 83299A24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83299A28: 386A77A4  addi r3, r10, 0x77a4
	ctx.r[3].s64 = ctx.r[10].s64 + 30628;
	// 83299A2C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 83299A30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83299A34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83299A38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83299A3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83299A40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83299A44: 38C00018  li r6, 0x18
	ctx.r[6].s64 = 24;
	// 83299A48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83299A4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83299A50: 4BABC1F1  bl 0x82d55c40
	ctx.lr = 0x83299A54;
	sub_82D55C40(ctx, base);
	// 83299A54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83299A58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83299A5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83299A60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83299A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83299A68 size=108
    let mut pc: u32 = 0x83299A68;
    'dispatch: loop {
        match pc {
            0x83299A68 => {
    //   block [0x83299A68..0x83299AD4)
	// 83299A68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83299A6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83299A70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83299A74: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 83299A78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 83299A7C: 38EB5308  addi r7, r11, 0x5308
	ctx.r[7].s64 = ctx.r[11].s64 + 21256;
	// 83299A80: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 83299A84: 388A5368  addi r4, r10, 0x5368
	ctx.r[4].s64 = ctx.r[10].s64 + 21352;
	// 83299A88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83299A8C: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83299A90: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 83299A94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83299A98: 386A77D4  addi r3, r10, 0x77d4
	ctx.r[3].s64 = ctx.r[10].s64 + 30676;
	// 83299A9C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 83299AA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83299AA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83299AA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83299AAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83299AB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83299AB4: 38C0001C  li r6, 0x1c
	ctx.r[6].s64 = 28;
	// 83299AB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83299ABC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83299AC0: 4BABC181  bl 0x82d55c40
	ctx.lr = 0x83299AC4;
	sub_82D55C40(ctx, base);
	// 83299AC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83299AC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83299ACC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83299AD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83299AD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83299AD8 size=108
    let mut pc: u32 = 0x83299AD8;
    'dispatch: loop {
        match pc {
            0x83299AD8 => {
    //   block [0x83299AD8..0x83299B44)
	// 83299AD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83299ADC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83299AE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83299AE4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 83299AE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 83299AEC: 38EB5450  addi r7, r11, 0x5450
	ctx.r[7].s64 = ctx.r[11].s64 + 21584;
	// 83299AF0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 83299AF4: 388A55B8  addi r4, r10, 0x55b8
	ctx.r[4].s64 = ctx.r[10].s64 + 21944;
	// 83299AF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83299AFC: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83299B00: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 83299B04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83299B08: 386A7804  addi r3, r10, 0x7804
	ctx.r[3].s64 = ctx.r[10].s64 + 30724;
	// 83299B0C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 83299B10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83299B14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83299B18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83299B1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83299B20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83299B24: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 83299B28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83299B2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83299B30: 4BABC111  bl 0x82d55c40
	ctx.lr = 0x83299B34;
	sub_82D55C40(ctx, base);
	// 83299B34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83299B38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83299B3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83299B40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83299B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83299B48 size=108
    let mut pc: u32 = 0x83299B48;
    'dispatch: loop {
        match pc {
            0x83299B48 => {
    //   block [0x83299B48..0x83299BB4)
	// 83299B48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83299B4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83299B50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83299B54: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 83299B58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 83299B5C: 38EB5498  addi r7, r11, 0x5498
	ctx.r[7].s64 = ctx.r[11].s64 + 21656;
	// 83299B60: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 83299B64: 388A55DC  addi r4, r10, 0x55dc
	ctx.r[4].s64 = ctx.r[10].s64 + 21980;
	// 83299B68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83299B6C: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83299B70: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 83299B74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83299B78: 386A7834  addi r3, r10, 0x7834
	ctx.r[3].s64 = ctx.r[10].s64 + 30772;
	// 83299B7C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 83299B80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83299B84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83299B88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83299B8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83299B90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83299B94: 38C00070  li r6, 0x70
	ctx.r[6].s64 = 112;
	// 83299B98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83299B9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83299BA0: 4BABC0A1  bl 0x82d55c40
	ctx.lr = 0x83299BA4;
	sub_82D55C40(ctx, base);
	// 83299BA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83299BA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83299BAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83299BB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83299BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83299BB8 size=108
    let mut pc: u32 = 0x83299BB8;
    'dispatch: loop {
        match pc {
            0x83299BB8 => {
    //   block [0x83299BB8..0x83299C24)
	// 83299BB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83299BBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83299BC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83299BC4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 83299BC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 83299BCC: 38EB5528  addi r7, r11, 0x5528
	ctx.r[7].s64 = ctx.r[11].s64 + 21800;
	// 83299BD0: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 83299BD4: 388A5600  addi r4, r10, 0x5600
	ctx.r[4].s64 = ctx.r[10].s64 + 22016;
	// 83299BD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83299BDC: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83299BE0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 83299BE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83299BE8: 386A7864  addi r3, r10, 0x7864
	ctx.r[3].s64 = ctx.r[10].s64 + 30820;
	// 83299BEC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 83299BF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83299BF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83299BF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83299BFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83299C00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83299C04: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 83299C08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83299C0C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83299C10: 4BABC031  bl 0x82d55c40
	ctx.lr = 0x83299C14;
	sub_82D55C40(ctx, base);
	// 83299C14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83299C18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83299C1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83299C20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83299C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83299C28 size=108
    let mut pc: u32 = 0x83299C28;
    'dispatch: loop {
        match pc {
            0x83299C28 => {
    //   block [0x83299C28..0x83299C94)
	// 83299C28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83299C2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83299C30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83299C34: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 83299C38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 83299C3C: 38EB5798  addi r7, r11, 0x5798
	ctx.r[7].s64 = ctx.r[11].s64 + 22424;
	// 83299C40: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 83299C44: 388A5A20  addi r4, r10, 0x5a20
	ctx.r[4].s64 = ctx.r[10].s64 + 23072;
	// 83299C48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83299C4C: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83299C50: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 83299C54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83299C58: 386A7894  addi r3, r10, 0x7894
	ctx.r[3].s64 = ctx.r[10].s64 + 30868;
	// 83299C5C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 83299C60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83299C64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83299C68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83299C6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83299C70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83299C74: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 83299C78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83299C7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83299C80: 4BABBFC1  bl 0x82d55c40
	ctx.lr = 0x83299C84;
	sub_82D55C40(ctx, base);
	// 83299C84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83299C88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83299C8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83299C90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83299C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83299C98 size=108
    let mut pc: u32 = 0x83299C98;
    'dispatch: loop {
        match pc {
            0x83299C98 => {
    //   block [0x83299C98..0x83299D04)
	// 83299C98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83299C9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83299CA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83299CA4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 83299CA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 83299CAC: 38EB5870  addi r7, r11, 0x5870
	ctx.r[7].s64 = ctx.r[11].s64 + 22640;
	// 83299CB0: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 83299CB4: 388A5A50  addi r4, r10, 0x5a50
	ctx.r[4].s64 = ctx.r[10].s64 + 23120;
	// 83299CB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83299CBC: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83299CC0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 83299CC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83299CC8: 386A78C4  addi r3, r10, 0x78c4
	ctx.r[3].s64 = ctx.r[10].s64 + 30916;
	// 83299CCC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 83299CD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83299CD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83299CD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83299CDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83299CE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83299CE4: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 83299CE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83299CEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83299CF0: 4BABBF51  bl 0x82d55c40
	ctx.lr = 0x83299CF4;
	sub_82D55C40(ctx, base);
	// 83299CF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83299CF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83299CFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83299D00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83299D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83299D08 size=112
    let mut pc: u32 = 0x83299D08;
    'dispatch: loop {
        match pc {
            0x83299D08 => {
    //   block [0x83299D08..0x83299D78)
	// 83299D08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83299D0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83299D10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83299D14: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83299D18: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 83299D1C: 38AA7B64  addi r5, r10, 0x7b64
	ctx.r[5].s64 = ctx.r[10].s64 + 31588;
	// 83299D20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 83299D24: 390B5900  addi r8, r11, 0x5900
	ctx.r[8].s64 = ctx.r[11].s64 + 22784;
	// 83299D28: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 83299D2C: 388A5A80  addi r4, r10, 0x5a80
	ctx.r[4].s64 = ctx.r[10].s64 + 23168;
	// 83299D30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83299D34: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83299D38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 83299D3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83299D40: 386A78F4  addi r3, r10, 0x78f4
	ctx.r[3].s64 = ctx.r[10].s64 + 30964;
	// 83299D44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 83299D48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83299D4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83299D50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83299D54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83299D58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83299D5C: 38C0005C  li r6, 0x5c
	ctx.r[6].s64 = 92;
	// 83299D60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83299D64: 4BABBEDD  bl 0x82d55c40
	ctx.lr = 0x83299D68;
	sub_82D55C40(ctx, base);
	// 83299D68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83299D6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83299D70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83299D74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83299D78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83299D78 size=112
    let mut pc: u32 = 0x83299D78;
    'dispatch: loop {
        match pc {
            0x83299D78 => {
    //   block [0x83299D78..0x83299DE8)
	// 83299D78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83299D7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83299D80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83299D84: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83299D88: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 83299D8C: 38AA7B34  addi r5, r10, 0x7b34
	ctx.r[5].s64 = ctx.r[10].s64 + 31540;
	// 83299D90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 83299D94: 390B5AD0  addi r8, r11, 0x5ad0
	ctx.r[8].s64 = ctx.r[11].s64 + 23248;
	// 83299D98: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 83299D9C: 388A5B4C  addi r4, r10, 0x5b4c
	ctx.r[4].s64 = ctx.r[10].s64 + 23372;
	// 83299DA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83299DA4: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83299DA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 83299DAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83299DB0: 386A7924  addi r3, r10, 0x7924
	ctx.r[3].s64 = ctx.r[10].s64 + 31012;
	// 83299DB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 83299DB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83299DBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83299DC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83299DC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83299DC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83299DCC: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 83299DD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83299DD4: 4BABBE6D  bl 0x82d55c40
	ctx.lr = 0x83299DD8;
	sub_82D55C40(ctx, base);
	// 83299DD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83299DDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83299DE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83299DE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83299DE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83299DE8 size=108
    let mut pc: u32 = 0x83299DE8;
    'dispatch: loop {
        match pc {
            0x83299DE8 => {
    //   block [0x83299DE8..0x83299E54)
	// 83299DE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83299DEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83299DF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83299DF4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 83299DF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 83299DFC: 38EB5BB0  addi r7, r11, 0x5bb0
	ctx.r[7].s64 = ctx.r[11].s64 + 23472;
	// 83299E00: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 83299E04: 388A5D90  addi r4, r10, 0x5d90
	ctx.r[4].s64 = ctx.r[10].s64 + 23952;
	// 83299E08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83299E0C: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83299E10: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 83299E14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83299E18: 386A7954  addi r3, r10, 0x7954
	ctx.r[3].s64 = ctx.r[10].s64 + 31060;
	// 83299E1C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 83299E20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83299E24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83299E28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83299E2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83299E30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83299E34: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 83299E38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83299E3C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83299E40: 4BABBE01  bl 0x82d55c40
	ctx.lr = 0x83299E44;
	sub_82D55C40(ctx, base);
	// 83299E44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83299E48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83299E4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83299E50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83299E58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83299E58 size=112
    let mut pc: u32 = 0x83299E58;
    'dispatch: loop {
        match pc {
            0x83299E58 => {
    //   block [0x83299E58..0x83299EC8)
	// 83299E58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83299E5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83299E60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83299E64: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83299E68: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 83299E6C: 38AA7B64  addi r5, r10, 0x7b64
	ctx.r[5].s64 = ctx.r[10].s64 + 31588;
	// 83299E70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 83299E74: 390B5C40  addi r8, r11, 0x5c40
	ctx.r[8].s64 = ctx.r[11].s64 + 23616;
	// 83299E78: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 83299E7C: 388A5DC8  addi r4, r10, 0x5dc8
	ctx.r[4].s64 = ctx.r[10].s64 + 24008;
	// 83299E80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83299E84: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83299E88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 83299E8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83299E90: 386A7984  addi r3, r10, 0x7984
	ctx.r[3].s64 = ctx.r[10].s64 + 31108;
	// 83299E94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 83299E98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83299E9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83299EA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83299EA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83299EA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83299EAC: 38C00064  li r6, 0x64
	ctx.r[6].s64 = 100;
	// 83299EB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83299EB4: 4BABBD8D  bl 0x82d55c40
	ctx.lr = 0x83299EB8;
	sub_82D55C40(ctx, base);
	// 83299EB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83299EBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83299EC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83299EC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83299EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83299EC8 size=112
    let mut pc: u32 = 0x83299EC8;
    'dispatch: loop {
        match pc {
            0x83299EC8 => {
    //   block [0x83299EC8..0x83299F38)
	// 83299EC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83299ECC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83299ED0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83299ED4: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83299ED8: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 83299EDC: 38AA7B64  addi r5, r10, 0x7b64
	ctx.r[5].s64 = ctx.r[10].s64 + 31588;
	// 83299EE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 83299EE4: 390B5DF8  addi r8, r11, 0x5df8
	ctx.r[8].s64 = ctx.r[11].s64 + 24056;
	// 83299EE8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 83299EEC: 388A5E40  addi r4, r10, 0x5e40
	ctx.r[4].s64 = ctx.r[10].s64 + 24128;
	// 83299EF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83299EF4: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83299EF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 83299EFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83299F00: 386A79B4  addi r3, r10, 0x79b4
	ctx.r[3].s64 = ctx.r[10].s64 + 31156;
	// 83299F04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 83299F08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83299F0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83299F10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83299F14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83299F18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83299F1C: 38C00028  li r6, 0x28
	ctx.r[6].s64 = 40;
	// 83299F20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83299F24: 4BABBD1D  bl 0x82d55c40
	ctx.lr = 0x83299F28;
	sub_82D55C40(ctx, base);
	// 83299F28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83299F2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83299F30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83299F34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83299F38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83299F38 size=112
    let mut pc: u32 = 0x83299F38;
    'dispatch: loop {
        match pc {
            0x83299F38 => {
    //   block [0x83299F38..0x83299FA8)
	// 83299F38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83299F3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83299F40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83299F44: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 83299F48: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 83299F4C: 392B5EBC  addi r9, r11, 0x5ebc
	ctx.r[9].s64 = ctx.r[11].s64 + 24252;
	// 83299F50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83299F54: 39090014  addi r8, r9, 0x14
	ctx.r[8].s64 = ctx.r[9].s64 + 20;
	// 83299F58: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 83299F5C: 38AAAE8C  addi r5, r10, -0x5174
	ctx.r[5].s64 = ctx.r[10].s64 + -20852;
	// 83299F60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 83299F64: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83299F68: 38C00038  li r6, 0x38
	ctx.r[6].s64 = 56;
	// 83299F6C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83299F70: 388A5EF8  addi r4, r10, 0x5ef8
	ctx.r[4].s64 = ctx.r[10].s64 + 24312;
	// 83299F74: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83299F78: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 83299F7C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 83299F80: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83299F84: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 83299F88: 386B79E4  addi r3, r11, 0x79e4
	ctx.r[3].s64 = ctx.r[11].s64 + 31204;
	// 83299F8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83299F90: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83299F94: 4BABBCAD  bl 0x82d55c40
	ctx.lr = 0x83299F98;
	sub_82D55C40(ctx, base);
	// 83299F98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83299F9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83299FA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83299FA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83299FA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83299FA8 size=24
    let mut pc: u32 = 0x83299FA8;
    'dispatch: loop {
        match pc {
            0x83299FA8 => {
    //   block [0x83299FA8..0x83299FC0)
	// 83299FA8: 3D60832F  lis r11, -0x7cd1
	ctx.r[11].s64 = -2094071808;
	// 83299FAC: 3D40832F  lis r10, -0x7cd1
	ctx.r[10].s64 = -2094071808;
	// 83299FB0: 394A79F8  addi r10, r10, 0x79f8
	ctx.r[10].s64 = ctx.r[10].s64 + 31224;
	// 83299FB4: 816B79E0  lwz r11, 0x79e0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(31200 as u32) ) } as u64;
	// 83299FB8: 916A0038  stw r11, 0x38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 83299FBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83299FC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83299FC0 size=112
    let mut pc: u32 = 0x83299FC0;
    'dispatch: loop {
        match pc {
            0x83299FC0 => {
    //   block [0x83299FC0..0x8329A030)
	// 83299FC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83299FC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83299FC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83299FCC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 83299FD0: 3D60832F  lis r11, -0x7cd1
	ctx.r[11].s64 = -2094071808;
	// 83299FD4: 392A5F68  addi r9, r10, 0x5f68
	ctx.r[9].s64 = ctx.r[10].s64 + 24424;
	// 83299FD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 83299FDC: 390B79F8  addi r8, r11, 0x79f8
	ctx.r[8].s64 = ctx.r[11].s64 + 31224;
	// 83299FE0: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 83299FE4: 388A5F7C  addi r4, r10, 0x5f7c
	ctx.r[4].s64 = ctx.r[10].s64 + 24444;
	// 83299FE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83299FEC: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 83299FF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 83299FF4: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 83299FF8: 386A7A14  addi r3, r10, 0x7a14
	ctx.r[3].s64 = ctx.r[10].s64 + 31252;
	// 83299FFC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8329A000: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8329A004: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329A008: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329A00C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329A010: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329A014: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329A018: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329A01C: 4BABBC25  bl 0x82d55c40
	ctx.lr = 0x8329A020;
	sub_82D55C40(ctx, base);
	// 8329A020: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329A024: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329A028: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329A02C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329A030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329A030 size=108
    let mut pc: u32 = 0x8329A030;
    'dispatch: loop {
        match pc {
            0x8329A030 => {
    //   block [0x8329A030..0x8329A09C)
	// 8329A030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329A034: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329A038: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329A03C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8329A040: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329A044: 38EB5FC8  addi r7, r11, 0x5fc8
	ctx.r[7].s64 = ctx.r[11].s64 + 24520;
	// 8329A048: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 8329A04C: 388A6040  addi r4, r10, 0x6040
	ctx.r[4].s64 = ctx.r[10].s64 + 24640;
	// 8329A050: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329A054: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 8329A058: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329A05C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329A060: 386A7A44  addi r3, r10, 0x7a44
	ctx.r[3].s64 = ctx.r[10].s64 + 31300;
	// 8329A064: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329A068: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329A06C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329A070: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329A074: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329A078: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329A07C: 38C00028  li r6, 0x28
	ctx.r[6].s64 = 40;
	// 8329A080: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329A084: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329A088: 4BABBBB9  bl 0x82d55c40
	ctx.lr = 0x8329A08C;
	sub_82D55C40(ctx, base);
	// 8329A08C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329A090: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329A094: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329A098: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329A0A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329A0A0 size=108
    let mut pc: u32 = 0x8329A0A0;
    'dispatch: loop {
        match pc {
            0x8329A0A0 => {
    //   block [0x8329A0A0..0x8329A10C)
	// 8329A0A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329A0A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329A0A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329A0AC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8329A0B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329A0B4: 38EB6064  addi r7, r11, 0x6064
	ctx.r[7].s64 = ctx.r[11].s64 + 24676;
	// 8329A0B8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8329A0BC: 388A60C4  addi r4, r10, 0x60c4
	ctx.r[4].s64 = ctx.r[10].s64 + 24772;
	// 8329A0C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329A0C4: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 8329A0C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329A0CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329A0D0: 386A7A74  addi r3, r10, 0x7a74
	ctx.r[3].s64 = ctx.r[10].s64 + 31348;
	// 8329A0D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329A0D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329A0DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329A0E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329A0E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329A0E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329A0EC: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8329A0F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329A0F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329A0F8: 4BABBB49  bl 0x82d55c40
	ctx.lr = 0x8329A0FC;
	sub_82D55C40(ctx, base);
	// 8329A0FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329A100: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329A104: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329A108: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329A110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329A110 size=108
    let mut pc: u32 = 0x8329A110;
    'dispatch: loop {
        match pc {
            0x8329A110 => {
    //   block [0x8329A110..0x8329A17C)
	// 8329A110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329A114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329A118: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329A11C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8329A120: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329A124: 38EB6094  addi r7, r11, 0x6094
	ctx.r[7].s64 = ctx.r[11].s64 + 24724;
	// 8329A128: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8329A12C: 388A60E4  addi r4, r10, 0x60e4
	ctx.r[4].s64 = ctx.r[10].s64 + 24804;
	// 8329A130: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329A134: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 8329A138: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329A13C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329A140: 386A7AA4  addi r3, r10, 0x7aa4
	ctx.r[3].s64 = ctx.r[10].s64 + 31396;
	// 8329A144: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329A148: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329A14C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329A150: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329A154: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329A158: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329A15C: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 8329A160: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329A164: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329A168: 4BABBAD9  bl 0x82d55c40
	ctx.lr = 0x8329A16C;
	sub_82D55C40(ctx, base);
	// 8329A16C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329A170: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329A174: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329A178: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329A180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329A180 size=108
    let mut pc: u32 = 0x8329A180;
    'dispatch: loop {
        match pc {
            0x8329A180 => {
    //   block [0x8329A180..0x8329A1EC)
	// 8329A180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329A184: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329A188: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329A18C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8329A190: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329A194: 38EB6108  addi r7, r11, 0x6108
	ctx.r[7].s64 = ctx.r[11].s64 + 24840;
	// 8329A198: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8329A19C: 388A6138  addi r4, r10, 0x6138
	ctx.r[4].s64 = ctx.r[10].s64 + 24888;
	// 8329A1A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329A1A4: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 8329A1A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329A1AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329A1B0: 386A7AD4  addi r3, r10, 0x7ad4
	ctx.r[3].s64 = ctx.r[10].s64 + 31444;
	// 8329A1B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329A1B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329A1BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329A1C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329A1C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329A1C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329A1CC: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8329A1D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329A1D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329A1D8: 4BABBA69  bl 0x82d55c40
	ctx.lr = 0x8329A1DC;
	sub_82D55C40(ctx, base);
	// 8329A1DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329A1E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329A1E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329A1E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329A1F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329A1F0 size=108
    let mut pc: u32 = 0x8329A1F0;
    'dispatch: loop {
        match pc {
            0x8329A1F0 => {
    //   block [0x8329A1F0..0x8329A25C)
	// 8329A1F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329A1F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329A1F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329A1FC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8329A200: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329A204: 38EB6170  addi r7, r11, 0x6170
	ctx.r[7].s64 = ctx.r[11].s64 + 24944;
	// 8329A208: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8329A20C: 388A61D0  addi r4, r10, 0x61d0
	ctx.r[4].s64 = ctx.r[10].s64 + 25040;
	// 8329A210: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329A214: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 8329A218: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329A21C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329A220: 386A7B04  addi r3, r10, 0x7b04
	ctx.r[3].s64 = ctx.r[10].s64 + 31492;
	// 8329A224: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329A228: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329A22C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329A230: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329A234: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329A238: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329A23C: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 8329A240: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329A244: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329A248: 4BABB9F9  bl 0x82d55c40
	ctx.lr = 0x8329A24C;
	sub_82D55C40(ctx, base);
	// 8329A24C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329A250: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329A254: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329A258: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329A260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329A260 size=100
    let mut pc: u32 = 0x8329A260;
    'dispatch: loop {
        match pc {
            0x8329A260 => {
    //   block [0x8329A260..0x8329A2C4)
	// 8329A260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329A264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329A268: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329A26C: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 8329A270: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329A274: 38AAAE8C  addi r5, r10, -0x5174
	ctx.r[5].s64 = ctx.r[10].s64 + -20852;
	// 8329A278: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329A27C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329A280: 388A61E4  addi r4, r10, 0x61e4
	ctx.r[4].s64 = ctx.r[10].s64 + 25060;
	// 8329A284: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 8329A288: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329A28C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329A290: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329A294: 386A7B34  addi r3, r10, 0x7b34
	ctx.r[3].s64 = ctx.r[10].s64 + 31540;
	// 8329A298: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329A29C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329A2A0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8329A2A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329A2A8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8329A2AC: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8329A2B0: 4BABB991  bl 0x82d55c40
	ctx.lr = 0x8329A2B4;
	sub_82D55C40(ctx, base);
	// 8329A2B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329A2B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329A2BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329A2C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329A2C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8329A2C8 size=24
    let mut pc: u32 = 0x8329A2C8;
    'dispatch: loop {
        match pc {
            0x8329A2C8 => {
    //   block [0x8329A2C8..0x8329A2E0)
	// 8329A2C8: 3D60832F  lis r11, -0x7cd1
	ctx.r[11].s64 = -2094071808;
	// 8329A2CC: 3D40832F  lis r10, -0x7cd1
	ctx.r[10].s64 = -2094071808;
	// 8329A2D0: 394A7A98  addi r10, r10, 0x7a98
	ctx.r[10].s64 = ctx.r[10].s64 + 31384;
	// 8329A2D4: 816B7A90  lwz r11, 0x7a90(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(31376 as u32) ) } as u64;
	// 8329A2D8: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8329A2DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329A2E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329A2E0 size=116
    let mut pc: u32 = 0x8329A2E0;
    'dispatch: loop {
        match pc {
            0x8329A2E0 => {
    //   block [0x8329A2E0..0x8329A354)
	// 8329A2E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329A2E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329A2E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329A2EC: 3D60832F  lis r11, -0x7cd1
	ctx.r[11].s64 = -2094071808;
	// 8329A2F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329A2F4: 390B7A98  addi r8, r11, 0x7a98
	ctx.r[8].s64 = ctx.r[11].s64 + 31384;
	// 8329A2F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329A2FC: 392A62C4  addi r9, r10, 0x62c4
	ctx.r[9].s64 = ctx.r[10].s64 + 25284;
	// 8329A300: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 8329A304: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8329A308: 38AAAE8C  addi r5, r10, -0x5174
	ctx.r[5].s64 = ctx.r[10].s64 + -20852;
	// 8329A30C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329A310: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329A314: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329A318: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329A31C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329A320: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329A324: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 8329A328: 388A630C  addi r4, r10, 0x630c
	ctx.r[4].s64 = ctx.r[10].s64 + 25356;
	// 8329A32C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8329A330: 386B7B64  addi r3, r11, 0x7b64
	ctx.r[3].s64 = ctx.r[11].s64 + 31588;
	// 8329A334: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8329A338: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329A33C: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 8329A340: 4BABB901  bl 0x82d55c40
	ctx.lr = 0x8329A344;
	sub_82D55C40(ctx, base);
	// 8329A344: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329A348: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329A34C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329A350: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329A358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329A358 size=108
    let mut pc: u32 = 0x8329A358;
    'dispatch: loop {
        match pc {
            0x8329A358 => {
    //   block [0x8329A358..0x8329A3C4)
	// 8329A358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329A35C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329A360: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329A364: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8329A368: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329A36C: 38EB69F8  addi r7, r11, 0x69f8
	ctx.r[7].s64 = ctx.r[11].s64 + 27128;
	// 8329A370: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8329A374: 388A6A58  addi r4, r10, 0x6a58
	ctx.r[4].s64 = ctx.r[10].s64 + 27224;
	// 8329A378: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329A37C: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 8329A380: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329A384: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329A388: 386A7B94  addi r3, r10, 0x7b94
	ctx.r[3].s64 = ctx.r[10].s64 + 31636;
	// 8329A38C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329A390: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329A394: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329A398: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329A39C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329A3A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329A3A4: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 8329A3A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329A3AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329A3B0: 4BABB891  bl 0x82d55c40
	ctx.lr = 0x8329A3B4;
	sub_82D55C40(ctx, base);
	// 8329A3B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329A3B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329A3BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329A3C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329A3C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329A3C8 size=108
    let mut pc: u32 = 0x8329A3C8;
    'dispatch: loop {
        match pc {
            0x8329A3C8 => {
    //   block [0x8329A3C8..0x8329A434)
	// 8329A3C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329A3CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329A3D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329A3D4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8329A3D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329A3DC: 38EB6A40  addi r7, r11, 0x6a40
	ctx.r[7].s64 = ctx.r[11].s64 + 27200;
	// 8329A3E0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8329A3E4: 388A6A7C  addi r4, r10, 0x6a7c
	ctx.r[4].s64 = ctx.r[10].s64 + 27260;
	// 8329A3E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329A3EC: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 8329A3F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329A3F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329A3F8: 386A7BC4  addi r3, r10, 0x7bc4
	ctx.r[3].s64 = ctx.r[10].s64 + 31684;
	// 8329A3FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329A400: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329A404: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329A408: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329A40C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329A410: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329A414: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8329A418: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329A41C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329A420: 4BABB821  bl 0x82d55c40
	ctx.lr = 0x8329A424;
	sub_82D55C40(ctx, base);
	// 8329A424: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329A428: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329A42C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329A430: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329A438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8329A438 size=36
    let mut pc: u32 = 0x8329A438;
    'dispatch: loop {
        match pc {
            0x8329A438 => {
    //   block [0x8329A438..0x8329A45C)
	// 8329A438: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8329A43C: 816B6A94  lwz r11, 0x6a94(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27284 as u32) ) } as u64;
	// 8329A440: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8329A444: 7D6A0034  cntlzw r10, r11
	ctx.r[10].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8329A448: 3D60832F  lis r11, -0x7cd1
	ctx.r[11].s64 = -2094071808;
	// 8329A44C: 554ADFFE  rlwinm r10, r10, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 8329A450: 396B7D58  addi r11, r11, 0x7d58
	ctx.r[11].s64 = ctx.r[11].s64 + 32088;
	// 8329A454: 994B0001  stb r10, 1(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(1 as u32), ctx.r[10].u8 ) };
	// 8329A458: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329A460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8329A460 size=28
    let mut pc: u32 = 0x8329A460;
    'dispatch: loop {
        match pc {
            0x8329A460 => {
    //   block [0x8329A460..0x8329A47C)
	// 8329A460: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 8329A464: 3D60832F  lis r11, -0x7cd1
	ctx.r[11].s64 = -2094071808;
	// 8329A468: 396B7DE4  addi r11, r11, 0x7de4
	ctx.r[11].s64 = ctx.r[11].s64 + 32228;
	// 8329A46C: 812A76FC  lwz r9, 0x76fc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(30460 as u32) ) } as u64;
	// 8329A470: 916A76FC  stw r11, 0x76fc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(30460 as u32), ctx.r[11].u32 ) };
	// 8329A474: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8329A478: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329A480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8329A480 size=28
    let mut pc: u32 = 0x8329A480;
    'dispatch: loop {
        match pc {
            0x8329A480 => {
    //   block [0x8329A480..0x8329A49C)
	// 8329A480: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 8329A484: 3D60832F  lis r11, -0x7cd1
	ctx.r[11].s64 = -2094071808;
	// 8329A488: 396B7E9C  addi r11, r11, 0x7e9c
	ctx.r[11].s64 = ctx.r[11].s64 + 32412;
	// 8329A48C: 812A76FC  lwz r9, 0x76fc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(30460 as u32) ) } as u64;
	// 8329A490: 916A76FC  stw r11, 0x76fc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(30460 as u32), ctx.r[11].u32 ) };
	// 8329A494: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8329A498: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329A4A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329A4A0 size=108
    let mut pc: u32 = 0x8329A4A0;
    'dispatch: loop {
        match pc {
            0x8329A4A0 => {
    //   block [0x8329A4A0..0x8329A50C)
	// 8329A4A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329A4A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329A4A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329A4AC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8329A4B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329A4B4: 38EB6E14  addi r7, r11, 0x6e14
	ctx.r[7].s64 = ctx.r[11].s64 + 28180;
	// 8329A4B8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8329A4BC: 388A6E44  addi r4, r10, 0x6e44
	ctx.r[4].s64 = ctx.r[10].s64 + 28228;
	// 8329A4C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329A4C4: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 8329A4C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329A4CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329A4D0: 386A7BFC  addi r3, r10, 0x7bfc
	ctx.r[3].s64 = ctx.r[10].s64 + 31740;
	// 8329A4D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329A4D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329A4DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329A4E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329A4E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329A4E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329A4EC: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 8329A4F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329A4F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329A4F8: 4BABB749  bl 0x82d55c40
	ctx.lr = 0x8329A4FC;
	sub_82D55C40(ctx, base);
	// 8329A4FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329A500: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329A504: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329A508: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329A510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329A510 size=108
    let mut pc: u32 = 0x8329A510;
    'dispatch: loop {
        match pc {
            0x8329A510 => {
    //   block [0x8329A510..0x8329A57C)
	// 8329A510: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329A514: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329A518: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329A51C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8329A520: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329A524: 38EB6E6C  addi r7, r11, 0x6e6c
	ctx.r[7].s64 = ctx.r[11].s64 + 28268;
	// 8329A528: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8329A52C: 388A6E84  addi r4, r10, 0x6e84
	ctx.r[4].s64 = ctx.r[10].s64 + 28292;
	// 8329A530: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329A534: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 8329A538: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329A53C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329A540: 386A7C2C  addi r3, r10, 0x7c2c
	ctx.r[3].s64 = ctx.r[10].s64 + 31788;
	// 8329A544: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329A548: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329A54C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329A550: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329A554: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329A558: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329A55C: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8329A560: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329A564: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329A568: 4BABB6D9  bl 0x82d55c40
	ctx.lr = 0x8329A56C;
	sub_82D55C40(ctx, base);
	// 8329A56C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329A570: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329A574: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329A578: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329A580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329A580 size=108
    let mut pc: u32 = 0x8329A580;
    'dispatch: loop {
        match pc {
            0x8329A580 => {
    //   block [0x8329A580..0x8329A5EC)
	// 8329A580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329A584: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329A588: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329A58C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8329A590: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329A594: 38EB6EAC  addi r7, r11, 0x6eac
	ctx.r[7].s64 = ctx.r[11].s64 + 28332;
	// 8329A598: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8329A59C: 388A6EDC  addi r4, r10, 0x6edc
	ctx.r[4].s64 = ctx.r[10].s64 + 28380;
	// 8329A5A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329A5A4: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 8329A5A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329A5AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329A5B0: 386A7C5C  addi r3, r10, 0x7c5c
	ctx.r[3].s64 = ctx.r[10].s64 + 31836;
	// 8329A5B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329A5B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329A5BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329A5C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329A5C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329A5C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329A5CC: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 8329A5D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329A5D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329A5D8: 4BABB669  bl 0x82d55c40
	ctx.lr = 0x8329A5DC;
	sub_82D55C40(ctx, base);
	// 8329A5DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329A5E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329A5E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329A5E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329A5F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329A5F0 size=108
    let mut pc: u32 = 0x8329A5F0;
    'dispatch: loop {
        match pc {
            0x8329A5F0 => {
    //   block [0x8329A5F0..0x8329A65C)
	// 8329A5F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329A5F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329A5F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329A5FC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8329A600: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329A604: 38EB6EFC  addi r7, r11, 0x6efc
	ctx.r[7].s64 = ctx.r[11].s64 + 28412;
	// 8329A608: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8329A60C: 388A6F14  addi r4, r10, 0x6f14
	ctx.r[4].s64 = ctx.r[10].s64 + 28436;
	// 8329A610: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329A614: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 8329A618: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329A61C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329A620: 386A7C8C  addi r3, r10, 0x7c8c
	ctx.r[3].s64 = ctx.r[10].s64 + 31884;
	// 8329A624: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329A628: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329A62C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329A630: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329A634: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329A638: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329A63C: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8329A640: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329A644: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329A648: 4BABB5F9  bl 0x82d55c40
	ctx.lr = 0x8329A64C;
	sub_82D55C40(ctx, base);
	// 8329A64C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329A650: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329A654: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329A658: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329A660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329A660 size=108
    let mut pc: u32 = 0x8329A660;
    'dispatch: loop {
        match pc {
            0x8329A660 => {
    //   block [0x8329A660..0x8329A6CC)
	// 8329A660: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329A664: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329A668: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329A66C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8329A670: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329A674: 38EB7000  addi r7, r11, 0x7000
	ctx.r[7].s64 = ctx.r[11].s64 + 28672;
	// 8329A678: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 8329A67C: 388A7120  addi r4, r10, 0x7120
	ctx.r[4].s64 = ctx.r[10].s64 + 28960;
	// 8329A680: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329A684: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 8329A688: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329A68C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329A690: 386A7CBC  addi r3, r10, 0x7cbc
	ctx.r[3].s64 = ctx.r[10].s64 + 31932;
	// 8329A694: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329A698: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329A69C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329A6A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329A6A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329A6A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329A6AC: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 8329A6B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329A6B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329A6B8: 4BABB589  bl 0x82d55c40
	ctx.lr = 0x8329A6BC;
	sub_82D55C40(ctx, base);
	// 8329A6BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329A6C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329A6C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329A6C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329A6D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329A6D0 size=108
    let mut pc: u32 = 0x8329A6D0;
    'dispatch: loop {
        match pc {
            0x8329A6D0 => {
    //   block [0x8329A6D0..0x8329A73C)
	// 8329A6D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329A6D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329A6D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329A6DC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8329A6E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329A6E4: 38EB7140  addi r7, r11, 0x7140
	ctx.r[7].s64 = ctx.r[11].s64 + 28992;
	// 8329A6E8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8329A6EC: 388A7158  addi r4, r10, 0x7158
	ctx.r[4].s64 = ctx.r[10].s64 + 29016;
	// 8329A6F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329A6F4: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 8329A6F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329A6FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329A700: 386A7CEC  addi r3, r10, 0x7cec
	ctx.r[3].s64 = ctx.r[10].s64 + 31980;
	// 8329A704: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329A708: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329A70C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329A710: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329A714: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329A718: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329A71C: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8329A720: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329A724: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329A728: 4BABB519  bl 0x82d55c40
	ctx.lr = 0x8329A72C;
	sub_82D55C40(ctx, base);
	// 8329A72C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329A730: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329A734: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329A738: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329A740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329A740 size=108
    let mut pc: u32 = 0x8329A740;
    'dispatch: loop {
        match pc {
            0x8329A740 => {
    //   block [0x8329A740..0x8329A7AC)
	// 8329A740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329A744: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329A748: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329A74C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8329A750: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329A754: 38EB7188  addi r7, r11, 0x7188
	ctx.r[7].s64 = ctx.r[11].s64 + 29064;
	// 8329A758: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8329A75C: 388A7218  addi r4, r10, 0x7218
	ctx.r[4].s64 = ctx.r[10].s64 + 29208;
	// 8329A760: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329A764: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 8329A768: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329A76C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329A770: 386A7D1C  addi r3, r10, 0x7d1c
	ctx.r[3].s64 = ctx.r[10].s64 + 32028;
	// 8329A774: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329A778: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329A77C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329A780: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329A784: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329A788: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329A78C: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 8329A790: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329A794: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329A798: 4BABB4A9  bl 0x82d55c40
	ctx.lr = 0x8329A79C;
	sub_82D55C40(ctx, base);
	// 8329A79C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329A7A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329A7A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329A7A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329A7B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329A7B0 size=108
    let mut pc: u32 = 0x8329A7B0;
    'dispatch: loop {
        match pc {
            0x8329A7B0 => {
    //   block [0x8329A7B0..0x8329A81C)
	// 8329A7B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329A7B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329A7B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329A7BC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8329A7C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329A7C4: 38EB7230  addi r7, r11, 0x7230
	ctx.r[7].s64 = ctx.r[11].s64 + 29232;
	// 8329A7C8: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 8329A7CC: 388A72F0  addi r4, r10, 0x72f0
	ctx.r[4].s64 = ctx.r[10].s64 + 29424;
	// 8329A7D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329A7D4: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 8329A7D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329A7DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329A7E0: 386A7D4C  addi r3, r10, 0x7d4c
	ctx.r[3].s64 = ctx.r[10].s64 + 32076;
	// 8329A7E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329A7E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329A7EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329A7F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329A7F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329A7F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329A7FC: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 8329A800: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329A804: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329A808: 4BABB439  bl 0x82d55c40
	ctx.lr = 0x8329A80C;
	sub_82D55C40(ctx, base);
	// 8329A80C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329A810: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329A814: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329A818: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329A820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8329A820 size=24
    let mut pc: u32 = 0x8329A820;
    'dispatch: loop {
        match pc {
            0x8329A820 => {
    //   block [0x8329A820..0x8329A838)
	// 8329A820: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 8329A824: 3D40832F  lis r10, -0x7cd1
	ctx.r[10].s64 = -2094071808;
	// 8329A828: 394A7FDC  addi r10, r10, 0x7fdc
	ctx.r[10].s64 = ctx.r[10].s64 + 32732;
	// 8329A82C: 816B80DC  lwz r11, -0x7f24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-32548 as u32) ) } as u64;
	// 8329A830: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8329A834: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329A838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329A838 size=108
    let mut pc: u32 = 0x8329A838;
    'dispatch: loop {
        match pc {
            0x8329A838 => {
    //   block [0x8329A838..0x8329A8A4)
	// 8329A838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329A83C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329A840: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329A844: 3D60832F  lis r11, -0x7cd1
	ctx.r[11].s64 = -2094071808;
	// 8329A848: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329A84C: 38EB7FDC  addi r7, r11, 0x7fdc
	ctx.r[7].s64 = ctx.r[11].s64 + 32732;
	// 8329A850: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8329A854: 388A7318  addi r4, r10, 0x7318
	ctx.r[4].s64 = ctx.r[10].s64 + 29464;
	// 8329A858: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329A85C: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 8329A860: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329A864: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329A868: 386A7D7C  addi r3, r10, 0x7d7c
	ctx.r[3].s64 = ctx.r[10].s64 + 32124;
	// 8329A86C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329A870: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329A874: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329A878: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329A87C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329A880: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329A884: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 8329A888: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329A88C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329A890: 4BABB3B1  bl 0x82d55c40
	ctx.lr = 0x8329A894;
	sub_82D55C40(ctx, base);
	// 8329A894: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329A898: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329A89C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329A8A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329A8A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329A8A8 size=108
    let mut pc: u32 = 0x8329A8A8;
    'dispatch: loop {
        match pc {
            0x8329A8A8 => {
    //   block [0x8329A8A8..0x8329A914)
	// 8329A8A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329A8AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329A8B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329A8B4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8329A8B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329A8BC: 38EB7348  addi r7, r11, 0x7348
	ctx.r[7].s64 = ctx.r[11].s64 + 29512;
	// 8329A8C0: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 8329A8C4: 388A7420  addi r4, r10, 0x7420
	ctx.r[4].s64 = ctx.r[10].s64 + 29728;
	// 8329A8C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329A8CC: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 8329A8D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329A8D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329A8D8: 386A7DAC  addi r3, r10, 0x7dac
	ctx.r[3].s64 = ctx.r[10].s64 + 32172;
	// 8329A8DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329A8E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329A8E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329A8E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329A8EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329A8F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329A8F4: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 8329A8F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329A8FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329A900: 4BABB341  bl 0x82d55c40
	ctx.lr = 0x8329A904;
	sub_82D55C40(ctx, base);
	// 8329A904: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329A908: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329A90C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329A910: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329A918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8329A918 size=24
    let mut pc: u32 = 0x8329A918;
    'dispatch: loop {
        match pc {
            0x8329A918 => {
    //   block [0x8329A918..0x8329A930)
	// 8329A918: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 8329A91C: 3D408330  lis r10, -0x7cd0
	ctx.r[10].s64 = -2094006272;
	// 8329A920: 394A802C  addi r10, r10, -0x7fd4
	ctx.r[10].s64 = ctx.r[10].s64 + -32724;
	// 8329A924: 816B80DC  lwz r11, -0x7f24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-32548 as u32) ) } as u64;
	// 8329A928: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8329A92C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329A930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329A930 size=108
    let mut pc: u32 = 0x8329A930;
    'dispatch: loop {
        match pc {
            0x8329A930 => {
    //   block [0x8329A930..0x8329A99C)
	// 8329A930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329A934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329A938: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329A93C: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 8329A940: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329A944: 38EB802C  addi r7, r11, -0x7fd4
	ctx.r[7].s64 = ctx.r[11].s64 + -32724;
	// 8329A948: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8329A94C: 388A7448  addi r4, r10, 0x7448
	ctx.r[4].s64 = ctx.r[10].s64 + 29768;
	// 8329A950: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329A954: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 8329A958: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329A95C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329A960: 386A7DDC  addi r3, r10, 0x7ddc
	ctx.r[3].s64 = ctx.r[10].s64 + 32220;
	// 8329A964: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329A968: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329A96C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329A970: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329A974: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329A978: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329A97C: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 8329A980: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329A984: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329A988: 4BABB2B9  bl 0x82d55c40
	ctx.lr = 0x8329A98C;
	sub_82D55C40(ctx, base);
	// 8329A98C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329A990: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329A994: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329A998: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329A9A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329A9A0 size=108
    let mut pc: u32 = 0x8329A9A0;
    'dispatch: loop {
        match pc {
            0x8329A9A0 => {
    //   block [0x8329A9A0..0x8329AA0C)
	// 8329A9A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329A9A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329A9A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329A9AC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8329A9B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329A9B4: 38EB7468  addi r7, r11, 0x7468
	ctx.r[7].s64 = ctx.r[11].s64 + 29800;
	// 8329A9B8: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 8329A9BC: 388A7528  addi r4, r10, 0x7528
	ctx.r[4].s64 = ctx.r[10].s64 + 29992;
	// 8329A9C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329A9C4: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 8329A9C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329A9CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329A9D0: 386A7E0C  addi r3, r10, 0x7e0c
	ctx.r[3].s64 = ctx.r[10].s64 + 32268;
	// 8329A9D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329A9D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329A9DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329A9E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329A9E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329A9E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329A9EC: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 8329A9F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329A9F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329A9F8: 4BABB249  bl 0x82d55c40
	ctx.lr = 0x8329A9FC;
	sub_82D55C40(ctx, base);
	// 8329A9FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329AA00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329AA04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329AA08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329AA10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329AA10 size=108
    let mut pc: u32 = 0x8329AA10;
    'dispatch: loop {
        match pc {
            0x8329AA10 => {
    //   block [0x8329AA10..0x8329AA7C)
	// 8329AA10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329AA14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329AA18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329AA1C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8329AA20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329AA24: 38EB754C  addi r7, r11, 0x754c
	ctx.r[7].s64 = ctx.r[11].s64 + 30028;
	// 8329AA28: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8329AA2C: 388A7564  addi r4, r10, 0x7564
	ctx.r[4].s64 = ctx.r[10].s64 + 30052;
	// 8329AA30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329AA34: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 8329AA38: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329AA3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329AA40: 386A7E3C  addi r3, r10, 0x7e3c
	ctx.r[3].s64 = ctx.r[10].s64 + 32316;
	// 8329AA44: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329AA48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329AA4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329AA50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329AA54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329AA58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329AA5C: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8329AA60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329AA64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329AA68: 4BABB1D9  bl 0x82d55c40
	ctx.lr = 0x8329AA6C;
	sub_82D55C40(ctx, base);
	// 8329AA6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329AA70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329AA74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329AA78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329AA80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329AA80 size=108
    let mut pc: u32 = 0x8329AA80;
    'dispatch: loop {
        match pc {
            0x8329AA80 => {
    //   block [0x8329AA80..0x8329AAEC)
	// 8329AA80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329AA84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329AA88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329AA8C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8329AA90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329AA94: 38EB7580  addi r7, r11, 0x7580
	ctx.r[7].s64 = ctx.r[11].s64 + 30080;
	// 8329AA98: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 8329AA9C: 388A7628  addi r4, r10, 0x7628
	ctx.r[4].s64 = ctx.r[10].s64 + 30248;
	// 8329AAA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329AAA4: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 8329AAA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329AAAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329AAB0: 386A7E6C  addi r3, r10, 0x7e6c
	ctx.r[3].s64 = ctx.r[10].s64 + 32364;
	// 8329AAB4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329AAB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329AABC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329AAC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329AAC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329AAC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329AACC: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 8329AAD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329AAD4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329AAD8: 4BABB169  bl 0x82d55c40
	ctx.lr = 0x8329AADC;
	sub_82D55C40(ctx, base);
	// 8329AADC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329AAE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329AAE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329AAE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329AAF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8329AAF0 size=24
    let mut pc: u32 = 0x8329AAF0;
    'dispatch: loop {
        match pc {
            0x8329AAF0 => {
    //   block [0x8329AAF0..0x8329AB08)
	// 8329AAF0: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 8329AAF4: 3D408330  lis r10, -0x7cd0
	ctx.r[10].s64 = -2094006272;
	// 8329AAF8: 394A809C  addi r10, r10, -0x7f64
	ctx.r[10].s64 = ctx.r[10].s64 + -32612;
	// 8329AAFC: 816B80DC  lwz r11, -0x7f24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-32548 as u32) ) } as u64;
	// 8329AB00: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8329AB04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329AB08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329AB08 size=108
    let mut pc: u32 = 0x8329AB08;
    'dispatch: loop {
        match pc {
            0x8329AB08 => {
    //   block [0x8329AB08..0x8329AB74)
	// 8329AB08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329AB0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329AB10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329AB14: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 8329AB18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329AB1C: 38EB809C  addi r7, r11, -0x7f64
	ctx.r[7].s64 = ctx.r[11].s64 + -32612;
	// 8329AB20: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8329AB24: 388A7648  addi r4, r10, 0x7648
	ctx.r[4].s64 = ctx.r[10].s64 + 30280;
	// 8329AB28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329AB2C: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 8329AB30: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329AB34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329AB38: 386A7E9C  addi r3, r10, 0x7e9c
	ctx.r[3].s64 = ctx.r[10].s64 + 32412;
	// 8329AB3C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329AB40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329AB44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329AB48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329AB4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329AB50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329AB54: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 8329AB58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329AB5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329AB60: 4BABB0E1  bl 0x82d55c40
	ctx.lr = 0x8329AB64;
	sub_82D55C40(ctx, base);
	// 8329AB64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329AB68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329AB6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329AB70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329AB78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329AB78 size=108
    let mut pc: u32 = 0x8329AB78;
    'dispatch: loop {
        match pc {
            0x8329AB78 => {
    //   block [0x8329AB78..0x8329ABE4)
	// 8329AB78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329AB7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329AB80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329AB84: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8329AB88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329AB8C: 38EB7670  addi r7, r11, 0x7670
	ctx.r[7].s64 = ctx.r[11].s64 + 30320;
	// 8329AB90: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8329AB94: 388A7688  addi r4, r10, 0x7688
	ctx.r[4].s64 = ctx.r[10].s64 + 30344;
	// 8329AB98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329AB9C: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 8329ABA0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329ABA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329ABA8: 386A7ECC  addi r3, r10, 0x7ecc
	ctx.r[3].s64 = ctx.r[10].s64 + 32460;
	// 8329ABAC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329ABB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329ABB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329ABB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329ABBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329ABC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329ABC4: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8329ABC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329ABCC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329ABD0: 4BABB071  bl 0x82d55c40
	ctx.lr = 0x8329ABD4;
	sub_82D55C40(ctx, base);
	// 8329ABD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329ABD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329ABDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329ABE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329ABE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329ABE8 size=108
    let mut pc: u32 = 0x8329ABE8;
    'dispatch: loop {
        match pc {
            0x8329ABE8 => {
    //   block [0x8329ABE8..0x8329AC54)
	// 8329ABE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329ABEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329ABF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329ABF4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8329ABF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329ABFC: 392B7730  addi r9, r11, 0x7730
	ctx.r[9].s64 = ctx.r[11].s64 + 30512;
	// 8329AC00: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 8329AC04: 39090014  addi r8, r9, 0x14
	ctx.r[8].s64 = ctx.r[9].s64 + 20;
	// 8329AC08: 388A7774  addi r4, r10, 0x7774
	ctx.r[4].s64 = ctx.r[10].s64 + 30580;
	// 8329AC0C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329AC10: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 8329AC14: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8329AC18: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 8329AC1C: 386A7EFC  addi r3, r10, 0x7efc
	ctx.r[3].s64 = ctx.r[10].s64 + 32508;
	// 8329AC20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329AC24: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8329AC28: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329AC2C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329AC30: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329AC34: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329AC38: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329AC3C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329AC40: 4BABB001  bl 0x82d55c40
	ctx.lr = 0x8329AC44;
	sub_82D55C40(ctx, base);
	// 8329AC44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329AC48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329AC4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329AC50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329AC58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329AC58 size=108
    let mut pc: u32 = 0x8329AC58;
    'dispatch: loop {
        match pc {
            0x8329AC58 => {
    //   block [0x8329AC58..0x8329ACC4)
	// 8329AC58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329AC5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329AC60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329AC64: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8329AC68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329AC6C: 38EB7798  addi r7, r11, 0x7798
	ctx.r[7].s64 = ctx.r[11].s64 + 30616;
	// 8329AC70: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8329AC74: 388A77B0  addi r4, r10, 0x77b0
	ctx.r[4].s64 = ctx.r[10].s64 + 30640;
	// 8329AC78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329AC7C: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 8329AC80: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329AC84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329AC88: 386A7F2C  addi r3, r10, 0x7f2c
	ctx.r[3].s64 = ctx.r[10].s64 + 32556;
	// 8329AC8C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329AC90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329AC94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329AC98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329AC9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329ACA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329ACA4: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8329ACA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329ACAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329ACB0: 4BABAF91  bl 0x82d55c40
	ctx.lr = 0x8329ACB4;
	sub_82D55C40(ctx, base);
	// 8329ACB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329ACB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329ACBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329ACC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329ACC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329ACC8 size=108
    let mut pc: u32 = 0x8329ACC8;
    'dispatch: loop {
        match pc {
            0x8329ACC8 => {
    //   block [0x8329ACC8..0x8329AD34)
	// 8329ACC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329ACCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329ACD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329ACD4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8329ACD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329ACDC: 38EB77CC  addi r7, r11, 0x77cc
	ctx.r[7].s64 = ctx.r[11].s64 + 30668;
	// 8329ACE0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8329ACE4: 388A77FC  addi r4, r10, 0x77fc
	ctx.r[4].s64 = ctx.r[10].s64 + 30716;
	// 8329ACE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329ACEC: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 8329ACF0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329ACF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329ACF8: 386A7F5C  addi r3, r10, 0x7f5c
	ctx.r[3].s64 = ctx.r[10].s64 + 32604;
	// 8329ACFC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329AD00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329AD04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329AD08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329AD0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329AD10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329AD14: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 8329AD18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329AD1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329AD20: 4BABAF21  bl 0x82d55c40
	ctx.lr = 0x8329AD24;
	sub_82D55C40(ctx, base);
	// 8329AD24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329AD28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329AD2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329AD30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329AD38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329AD38 size=108
    let mut pc: u32 = 0x8329AD38;
    'dispatch: loop {
        match pc {
            0x8329AD38 => {
    //   block [0x8329AD38..0x8329ADA4)
	// 8329AD38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329AD3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329AD40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329AD44: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8329AD48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329AD4C: 38EB7820  addi r7, r11, 0x7820
	ctx.r[7].s64 = ctx.r[11].s64 + 30752;
	// 8329AD50: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8329AD54: 388A7838  addi r4, r10, 0x7838
	ctx.r[4].s64 = ctx.r[10].s64 + 30776;
	// 8329AD58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329AD5C: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 8329AD60: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329AD64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329AD68: 386A7F8C  addi r3, r10, 0x7f8c
	ctx.r[3].s64 = ctx.r[10].s64 + 32652;
	// 8329AD6C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329AD70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329AD74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329AD78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329AD7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329AD80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329AD84: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8329AD88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329AD8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329AD90: 4BABAEB1  bl 0x82d55c40
	ctx.lr = 0x8329AD94;
	sub_82D55C40(ctx, base);
	// 8329AD94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329AD98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329AD9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329ADA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329ADA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329ADA8 size=108
    let mut pc: u32 = 0x8329ADA8;
    'dispatch: loop {
        match pc {
            0x8329ADA8 => {
    //   block [0x8329ADA8..0x8329AE14)
	// 8329ADA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329ADAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329ADB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329ADB4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8329ADB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329ADBC: 38EB7870  addi r7, r11, 0x7870
	ctx.r[7].s64 = ctx.r[11].s64 + 30832;
	// 8329ADC0: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 8329ADC4: 388A7918  addi r4, r10, 0x7918
	ctx.r[4].s64 = ctx.r[10].s64 + 31000;
	// 8329ADC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329ADCC: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 8329ADD0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329ADD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329ADD8: 386A7FBC  addi r3, r10, 0x7fbc
	ctx.r[3].s64 = ctx.r[10].s64 + 32700;
	// 8329ADDC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329ADE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329ADE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329ADE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329ADEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329ADF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329ADF4: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 8329ADF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329ADFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329AE00: 4BABAE41  bl 0x82d55c40
	ctx.lr = 0x8329AE04;
	sub_82D55C40(ctx, base);
	// 8329AE04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329AE08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329AE0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329AE10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329AE18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329AE18 size=108
    let mut pc: u32 = 0x8329AE18;
    'dispatch: loop {
        match pc {
            0x8329AE18 => {
    //   block [0x8329AE18..0x8329AE84)
	// 8329AE18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329AE1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329AE20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329AE24: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8329AE28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329AE2C: 38EB7934  addi r7, r11, 0x7934
	ctx.r[7].s64 = ctx.r[11].s64 + 31028;
	// 8329AE30: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8329AE34: 388A794C  addi r4, r10, 0x794c
	ctx.r[4].s64 = ctx.r[10].s64 + 31052;
	// 8329AE38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329AE3C: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 8329AE40: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329AE44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329AE48: 386A7FEC  addi r3, r10, 0x7fec
	ctx.r[3].s64 = ctx.r[10].s64 + 32748;
	// 8329AE4C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329AE50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329AE54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329AE58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329AE5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329AE60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329AE64: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8329AE68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329AE6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329AE70: 4BABADD1  bl 0x82d55c40
	ctx.lr = 0x8329AE74;
	sub_82D55C40(ctx, base);
	// 8329AE74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329AE78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329AE7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329AE80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329AE88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329AE88 size=108
    let mut pc: u32 = 0x8329AE88;
    'dispatch: loop {
        match pc {
            0x8329AE88 => {
    //   block [0x8329AE88..0x8329AEF4)
	// 8329AE88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329AE8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329AE90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329AE94: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8329AE98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329AE9C: 38EB7970  addi r7, r11, 0x7970
	ctx.r[7].s64 = ctx.r[11].s64 + 31088;
	// 8329AEA0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8329AEA4: 388A79B8  addi r4, r10, 0x79b8
	ctx.r[4].s64 = ctx.r[10].s64 + 31160;
	// 8329AEA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329AEAC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329AEB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329AEB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329AEB8: 386A801C  addi r3, r10, -0x7fe4
	ctx.r[3].s64 = ctx.r[10].s64 + -32740;
	// 8329AEBC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329AEC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329AEC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329AEC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329AECC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329AED0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329AED4: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8329AED8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329AEDC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329AEE0: 4BABAD61  bl 0x82d55c40
	ctx.lr = 0x8329AEE4;
	sub_82D55C40(ctx, base);
	// 8329AEE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329AEE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329AEEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329AEF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329AEF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329AEF8 size=108
    let mut pc: u32 = 0x8329AEF8;
    'dispatch: loop {
        match pc {
            0x8329AEF8 => {
    //   block [0x8329AEF8..0x8329AF64)
	// 8329AEF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329AEFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329AF00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329AF04: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8329AF08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329AF0C: 38EB79A0  addi r7, r11, 0x79a0
	ctx.r[7].s64 = ctx.r[11].s64 + 31136;
	// 8329AF10: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8329AF14: 388A79D0  addi r4, r10, 0x79d0
	ctx.r[4].s64 = ctx.r[10].s64 + 31184;
	// 8329AF18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329AF1C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329AF20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329AF24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329AF28: 386A804C  addi r3, r10, -0x7fb4
	ctx.r[3].s64 = ctx.r[10].s64 + -32692;
	// 8329AF2C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329AF30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329AF34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329AF38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329AF3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329AF40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329AF44: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 8329AF48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329AF4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329AF50: 4BABACF1  bl 0x82d55c40
	ctx.lr = 0x8329AF54;
	sub_82D55C40(ctx, base);
	// 8329AF54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329AF58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329AF5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329AF60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329AF68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8329AF68 size=24
    let mut pc: u32 = 0x8329AF68;
    'dispatch: loop {
        match pc {
            0x8329AF68 => {
    //   block [0x8329AF68..0x8329AF80)
	// 8329AF68: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 8329AF6C: 3D408330  lis r10, -0x7cd0
	ctx.r[10].s64 = -2094006272;
	// 8329AF70: 394A8178  addi r10, r10, -0x7e88
	ctx.r[10].s64 = ctx.r[10].s64 + -32392;
	// 8329AF74: 816B8160  lwz r11, -0x7ea0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-32416 as u32) ) } as u64;
	// 8329AF78: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8329AF7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329AF80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329AF80 size=112
    let mut pc: u32 = 0x8329AF80;
    'dispatch: loop {
        match pc {
            0x8329AF80 => {
    //   block [0x8329AF80..0x8329AFF0)
	// 8329AF80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329AF84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329AF88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329AF8C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329AF90: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 8329AF94: 392A7A88  addi r9, r10, 0x7a88
	ctx.r[9].s64 = ctx.r[10].s64 + 31368;
	// 8329AF98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329AF9C: 390B8178  addi r8, r11, -0x7e88
	ctx.r[8].s64 = ctx.r[11].s64 + -32392;
	// 8329AFA0: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8329AFA4: 388A7A9C  addi r4, r10, 0x7a9c
	ctx.r[4].s64 = ctx.r[10].s64 + 31388;
	// 8329AFA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329AFAC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329AFB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329AFB4: 38C0001C  li r6, 0x1c
	ctx.r[6].s64 = 28;
	// 8329AFB8: 386A807C  addi r3, r10, -0x7f84
	ctx.r[3].s64 = ctx.r[10].s64 + -32644;
	// 8329AFBC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8329AFC0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8329AFC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329AFC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329AFCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329AFD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329AFD4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329AFD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329AFDC: 4BABAC65  bl 0x82d55c40
	ctx.lr = 0x8329AFE0;
	sub_82D55C40(ctx, base);
	// 8329AFE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329AFE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329AFE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329AFEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329AFF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8329AFF0 size=24
    let mut pc: u32 = 0x8329AFF0;
    'dispatch: loop {
        match pc {
            0x8329AFF0 => {
    //   block [0x8329AFF0..0x8329B008)
	// 8329AFF0: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 8329AFF4: 3D408330  lis r10, -0x7cd0
	ctx.r[10].s64 = -2094006272;
	// 8329AFF8: 394A8208  addi r10, r10, -0x7df8
	ctx.r[10].s64 = ctx.r[10].s64 + -32248;
	// 8329AFFC: 816B81F0  lwz r11, -0x7e10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-32272 as u32) ) } as u64;
	// 8329B000: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8329B004: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329B008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329B008 size=112
    let mut pc: u32 = 0x8329B008;
    'dispatch: loop {
        match pc {
            0x8329B008 => {
    //   block [0x8329B008..0x8329B078)
	// 8329B008: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329B00C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329B010: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329B014: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329B018: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 8329B01C: 392A7B1C  addi r9, r10, 0x7b1c
	ctx.r[9].s64 = ctx.r[10].s64 + 31516;
	// 8329B020: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329B024: 390B8208  addi r8, r11, -0x7df8
	ctx.r[8].s64 = ctx.r[11].s64 + -32248;
	// 8329B028: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8329B02C: 388A7B30  addi r4, r10, 0x7b30
	ctx.r[4].s64 = ctx.r[10].s64 + 31536;
	// 8329B030: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329B034: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329B038: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329B03C: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 8329B040: 386A80AC  addi r3, r10, -0x7f54
	ctx.r[3].s64 = ctx.r[10].s64 + -32596;
	// 8329B044: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8329B048: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8329B04C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329B050: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329B054: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329B058: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329B05C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329B060: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329B064: 4BABABDD  bl 0x82d55c40
	ctx.lr = 0x8329B068;
	sub_82D55C40(ctx, base);
	// 8329B068: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329B06C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329B070: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329B074: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329B078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329B078 size=108
    let mut pc: u32 = 0x8329B078;
    'dispatch: loop {
        match pc {
            0x8329B078 => {
    //   block [0x8329B078..0x8329B0E4)
	// 8329B078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329B07C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329B080: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329B084: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8329B088: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329B08C: 38EB7C64  addi r7, r11, 0x7c64
	ctx.r[7].s64 = ctx.r[11].s64 + 31844;
	// 8329B090: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8329B094: 388A7D58  addi r4, r10, 0x7d58
	ctx.r[4].s64 = ctx.r[10].s64 + 32088;
	// 8329B098: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329B09C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329B0A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329B0A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329B0A8: 386A80DC  addi r3, r10, -0x7f24
	ctx.r[3].s64 = ctx.r[10].s64 + -32548;
	// 8329B0AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329B0B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329B0B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329B0B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329B0BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329B0C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329B0C4: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 8329B0C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329B0CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329B0D0: 4BABAB71  bl 0x82d55c40
	ctx.lr = 0x8329B0D4;
	sub_82D55C40(ctx, base);
	// 8329B0D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329B0D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329B0DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329B0E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329B0E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329B0E8 size=112
    let mut pc: u32 = 0x8329B0E8;
    'dispatch: loop {
        match pc {
            0x8329B0E8 => {
    //   block [0x8329B0E8..0x8329B158)
	// 8329B0E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329B0EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329B0F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329B0F4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8329B0F8: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 8329B0FC: 392B7C50  addi r9, r11, 0x7c50
	ctx.r[9].s64 = ctx.r[11].s64 + 31824;
	// 8329B100: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329B104: 39090048  addi r8, r9, 0x48
	ctx.r[8].s64 = ctx.r[9].s64 + 72;
	// 8329B108: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 8329B10C: 38AA7F8C  addi r5, r10, 0x7f8c
	ctx.r[5].s64 = ctx.r[10].s64 + 32652;
	// 8329B110: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329B114: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329B118: 38C00070  li r6, 0x70
	ctx.r[6].s64 = 112;
	// 8329B11C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329B120: 388A7D70  addi r4, r10, 0x7d70
	ctx.r[4].s64 = ctx.r[10].s64 + 32112;
	// 8329B124: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329B128: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8329B12C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8329B130: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8329B134: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329B138: 386B810C  addi r3, r11, -0x7ef4
	ctx.r[3].s64 = ctx.r[11].s64 + -32500;
	// 8329B13C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329B140: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329B144: 4BABAAFD  bl 0x82d55c40
	ctx.lr = 0x8329B148;
	sub_82D55C40(ctx, base);
	// 8329B148: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329B14C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329B150: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329B154: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329B158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8329B158 size=24
    let mut pc: u32 = 0x8329B158;
    'dispatch: loop {
        match pc {
            0x8329B158 => {
    //   block [0x8329B158..0x8329B170)
	// 8329B158: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 8329B15C: 3D408330  lis r10, -0x7cd0
	ctx.r[10].s64 = -2094006272;
	// 8329B160: 394A82C0  addi r10, r10, -0x7d40
	ctx.r[10].s64 = ctx.r[10].s64 + -32064;
	// 8329B164: 816B82A8  lwz r11, -0x7d58(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-32088 as u32) ) } as u64;
	// 8329B168: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8329B16C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329B170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329B170 size=112
    let mut pc: u32 = 0x8329B170;
    'dispatch: loop {
        match pc {
            0x8329B170 => {
    //   block [0x8329B170..0x8329B1E0)
	// 8329B170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329B174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329B178: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329B17C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329B180: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 8329B184: 392A7E14  addi r9, r10, 0x7e14
	ctx.r[9].s64 = ctx.r[10].s64 + 32276;
	// 8329B188: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329B18C: 390B82C0  addi r8, r11, -0x7d40
	ctx.r[8].s64 = ctx.r[11].s64 + -32064;
	// 8329B190: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8329B194: 388A7E28  addi r4, r10, 0x7e28
	ctx.r[4].s64 = ctx.r[10].s64 + 32296;
	// 8329B198: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329B19C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329B1A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329B1A4: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 8329B1A8: 386A813C  addi r3, r10, -0x7ec4
	ctx.r[3].s64 = ctx.r[10].s64 + -32452;
	// 8329B1AC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8329B1B0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8329B1B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329B1B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329B1BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329B1C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329B1C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329B1C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329B1CC: 4BABAA75  bl 0x82d55c40
	ctx.lr = 0x8329B1D0;
	sub_82D55C40(ctx, base);
	// 8329B1D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329B1D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329B1D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329B1DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329B1E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329B1E0 size=112
    let mut pc: u32 = 0x8329B1E0;
    'dispatch: loop {
        match pc {
            0x8329B1E0 => {
    //   block [0x8329B1E0..0x8329B250)
	// 8329B1E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329B1E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329B1E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329B1EC: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 8329B1F0: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8329B1F4: 38AA7F8C  addi r5, r10, 0x7f8c
	ctx.r[5].s64 = ctx.r[10].s64 + 32652;
	// 8329B1F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329B1FC: 390B7E5C  addi r8, r11, 0x7e5c
	ctx.r[8].s64 = ctx.r[11].s64 + 32348;
	// 8329B200: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8329B204: 388A7EBC  addi r4, r10, 0x7ebc
	ctx.r[4].s64 = ctx.r[10].s64 + 32444;
	// 8329B208: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329B20C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329B210: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329B214: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329B218: 386A816C  addi r3, r10, -0x7e94
	ctx.r[3].s64 = ctx.r[10].s64 + -32404;
	// 8329B21C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329B220: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329B224: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329B228: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329B22C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329B230: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329B234: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 8329B238: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329B23C: 4BABAA05  bl 0x82d55c40
	ctx.lr = 0x8329B240;
	sub_82D55C40(ctx, base);
	// 8329B240: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329B244: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329B248: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329B24C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329B250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329B250 size=108
    let mut pc: u32 = 0x8329B250;
    'dispatch: loop {
        match pc {
            0x8329B250 => {
    //   block [0x8329B250..0x8329B2BC)
	// 8329B250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329B254: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329B258: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329B25C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8329B260: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329B264: 38EB7E8C  addi r7, r11, 0x7e8c
	ctx.r[7].s64 = ctx.r[11].s64 + 32396;
	// 8329B268: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8329B26C: 388A7ED4  addi r4, r10, 0x7ed4
	ctx.r[4].s64 = ctx.r[10].s64 + 32468;
	// 8329B270: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329B274: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329B278: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329B27C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329B280: 386A819C  addi r3, r10, -0x7e64
	ctx.r[3].s64 = ctx.r[10].s64 + -32356;
	// 8329B284: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329B288: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329B28C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329B290: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329B294: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329B298: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329B29C: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 8329B2A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329B2A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329B2A8: 4BABA999  bl 0x82d55c40
	ctx.lr = 0x8329B2AC;
	sub_82D55C40(ctx, base);
	// 8329B2AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329B2B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329B2B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329B2B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329B2C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329B2C0 size=108
    let mut pc: u32 = 0x8329B2C0;
    'dispatch: loop {
        match pc {
            0x8329B2C0 => {
    //   block [0x8329B2C0..0x8329B32C)
	// 8329B2C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329B2C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329B2C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329B2CC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8329B2D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329B2D4: 38EB7F10  addi r7, r11, 0x7f10
	ctx.r[7].s64 = ctx.r[11].s64 + 32528;
	// 8329B2D8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8329B2DC: 388A7F70  addi r4, r10, 0x7f70
	ctx.r[4].s64 = ctx.r[10].s64 + 32624;
	// 8329B2E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329B2E4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329B2E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329B2EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329B2F0: 386A81CC  addi r3, r10, -0x7e34
	ctx.r[3].s64 = ctx.r[10].s64 + -32308;
	// 8329B2F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329B2F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329B2FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329B300: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329B304: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329B308: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329B30C: 38C00018  li r6, 0x18
	ctx.r[6].s64 = 24;
	// 8329B310: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329B314: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329B318: 4BABA929  bl 0x82d55c40
	ctx.lr = 0x8329B31C;
	sub_82D55C40(ctx, base);
	// 8329B31C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329B320: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329B324: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329B328: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329B330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329B330 size=108
    let mut pc: u32 = 0x8329B330;
    'dispatch: loop {
        match pc {
            0x8329B330 => {
    //   block [0x8329B330..0x8329B39C)
	// 8329B330: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329B334: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329B338: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329B33C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8329B340: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329B344: 38EB7FC0  addi r7, r11, 0x7fc0
	ctx.r[7].s64 = ctx.r[11].s64 + 32704;
	// 8329B348: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8329B34C: 388A8098  addi r4, r10, -0x7f68
	ctx.r[4].s64 = ctx.r[10].s64 + -32616;
	// 8329B350: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329B354: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329B358: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329B35C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329B360: 386A81FC  addi r3, r10, -0x7e04
	ctx.r[3].s64 = ctx.r[10].s64 + -32260;
	// 8329B364: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329B368: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329B36C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329B370: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329B374: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329B378: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329B37C: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8329B380: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329B384: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329B388: 4BABA8B9  bl 0x82d55c40
	ctx.lr = 0x8329B38C;
	sub_82D55C40(ctx, base);
	// 8329B38C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329B390: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329B394: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329B398: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329B3A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329B3A0 size=112
    let mut pc: u32 = 0x8329B3A0;
    'dispatch: loop {
        match pc {
            0x8329B3A0 => {
    //   block [0x8329B3A0..0x8329B410)
	// 8329B3A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329B3A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329B3A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329B3AC: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 8329B3B0: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8329B3B4: 38AA7F8C  addi r5, r10, 0x7f8c
	ctx.r[5].s64 = ctx.r[10].s64 + 32652;
	// 8329B3B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329B3BC: 390B7FF0  addi r8, r11, 0x7ff0
	ctx.r[8].s64 = ctx.r[11].s64 + 32752;
	// 8329B3C0: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8329B3C4: 388A80B0  addi r4, r10, -0x7f50
	ctx.r[4].s64 = ctx.r[10].s64 + -32592;
	// 8329B3C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329B3CC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329B3D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329B3D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329B3D8: 386A822C  addi r3, r10, -0x7dd4
	ctx.r[3].s64 = ctx.r[10].s64 + -32212;
	// 8329B3DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329B3E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329B3E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329B3E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329B3EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329B3F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329B3F4: 38C00034  li r6, 0x34
	ctx.r[6].s64 = 52;
	// 8329B3F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329B3FC: 4BABA845  bl 0x82d55c40
	ctx.lr = 0x8329B400;
	sub_82D55C40(ctx, base);
	// 8329B400: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329B404: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329B408: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329B40C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329B410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329B410 size=112
    let mut pc: u32 = 0x8329B410;
    'dispatch: loop {
        match pc {
            0x8329B410 => {
    //   block [0x8329B410..0x8329B480)
	// 8329B410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329B414: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329B418: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329B41C: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 8329B420: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329B424: 38AA7F8C  addi r5, r10, 0x7f8c
	ctx.r[5].s64 = ctx.r[10].s64 + 32652;
	// 8329B428: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329B42C: 390B80C8  addi r8, r11, -0x7f38
	ctx.r[8].s64 = ctx.r[11].s64 + -32568;
	// 8329B430: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8329B434: 388A80F8  addi r4, r10, -0x7f08
	ctx.r[4].s64 = ctx.r[10].s64 + -32520;
	// 8329B438: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329B43C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329B440: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329B444: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329B448: 386A825C  addi r3, r10, -0x7da4
	ctx.r[3].s64 = ctx.r[10].s64 + -32164;
	// 8329B44C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329B450: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329B454: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329B458: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329B45C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329B460: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329B464: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 8329B468: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329B46C: 4BABA7D5  bl 0x82d55c40
	ctx.lr = 0x8329B470;
	sub_82D55C40(ctx, base);
	// 8329B470: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329B474: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329B478: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329B47C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329B480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329B480 size=108
    let mut pc: u32 = 0x8329B480;
    'dispatch: loop {
        match pc {
            0x8329B480 => {
    //   block [0x8329B480..0x8329B4EC)
	// 8329B480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329B484: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329B488: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329B48C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329B490: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329B494: 396B81B8  addi r11, r11, -0x7e48
	ctx.r[11].s64 = ctx.r[11].s64 + -32328;
	// 8329B498: 38E0000D  li r7, 0xd
	ctx.r[7].s64 = 13;
	// 8329B49C: 390B0138  addi r8, r11, 0x138
	ctx.r[8].s64 = ctx.r[11].s64 + 312;
	// 8329B4A0: 388A8354  addi r4, r10, -0x7cac
	ctx.r[4].s64 = ctx.r[10].s64 + -31916;
	// 8329B4A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329B4A8: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329B4AC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8329B4B0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8329B4B4: 38C00080  li r6, 0x80
	ctx.r[6].s64 = 128;
	// 8329B4B8: 386A828C  addi r3, r10, -0x7d74
	ctx.r[3].s64 = ctx.r[10].s64 + -32116;
	// 8329B4BC: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 8329B4C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329B4C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329B4C8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8329B4CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329B4D0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8329B4D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329B4D8: 4BABA769  bl 0x82d55c40
	ctx.lr = 0x8329B4DC;
	sub_82D55C40(ctx, base);
	// 8329B4DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329B4E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329B4E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329B4E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329B4F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329B4F0 size=108
    let mut pc: u32 = 0x8329B4F0;
    'dispatch: loop {
        match pc {
            0x8329B4F0 => {
    //   block [0x8329B4F0..0x8329B55C)
	// 8329B4F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329B4F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329B4F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329B4FC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329B500: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329B504: 38EB8380  addi r7, r11, -0x7c80
	ctx.r[7].s64 = ctx.r[11].s64 + -31872;
	// 8329B508: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8329B50C: 388A83E0  addi r4, r10, -0x7c20
	ctx.r[4].s64 = ctx.r[10].s64 + -31776;
	// 8329B510: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329B514: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329B518: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329B51C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329B520: 386A82BC  addi r3, r10, -0x7d44
	ctx.r[3].s64 = ctx.r[10].s64 + -32068;
	// 8329B524: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329B528: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329B52C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329B530: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329B534: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329B538: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329B53C: 38C00060  li r6, 0x60
	ctx.r[6].s64 = 96;
	// 8329B540: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329B544: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329B548: 4BABA6F9  bl 0x82d55c40
	ctx.lr = 0x8329B54C;
	sub_82D55C40(ctx, base);
	// 8329B54C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329B550: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329B554: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329B558: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329B560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329B560 size=108
    let mut pc: u32 = 0x8329B560;
    'dispatch: loop {
        match pc {
            0x8329B560 => {
    //   block [0x8329B560..0x8329B5CC)
	// 8329B560: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329B564: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329B568: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329B56C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329B570: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329B574: 38EB8400  addi r7, r11, -0x7c00
	ctx.r[7].s64 = ctx.r[11].s64 + -31744;
	// 8329B578: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8329B57C: 388A8430  addi r4, r10, -0x7bd0
	ctx.r[4].s64 = ctx.r[10].s64 + -31696;
	// 8329B580: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329B584: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329B588: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329B58C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329B590: 386A82EC  addi r3, r10, -0x7d14
	ctx.r[3].s64 = ctx.r[10].s64 + -32020;
	// 8329B594: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329B598: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329B59C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329B5A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329B5A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329B5A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329B5AC: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 8329B5B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329B5B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329B5B8: 4BABA689  bl 0x82d55c40
	ctx.lr = 0x8329B5BC;
	sub_82D55C40(ctx, base);
	// 8329B5BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329B5C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329B5C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329B5C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329B5D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329B5D0 size=112
    let mut pc: u32 = 0x8329B5D0;
    'dispatch: loop {
        match pc {
            0x8329B5D0 => {
    //   block [0x8329B5D0..0x8329B640)
	// 8329B5D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329B5D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329B5D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329B5DC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329B5E0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329B5E4: 38AA834C  addi r5, r10, -0x7cb4
	ctx.r[5].s64 = ctx.r[10].s64 + -31924;
	// 8329B5E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329B5EC: 390B8448  addi r8, r11, -0x7bb8
	ctx.r[8].s64 = ctx.r[11].s64 + -31672;
	// 8329B5F0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8329B5F4: 388A8460  addi r4, r10, -0x7ba0
	ctx.r[4].s64 = ctx.r[10].s64 + -31648;
	// 8329B5F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329B5FC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329B600: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329B604: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329B608: 386A831C  addi r3, r10, -0x7ce4
	ctx.r[3].s64 = ctx.r[10].s64 + -31972;
	// 8329B60C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329B610: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329B614: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329B618: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329B61C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329B620: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329B624: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 8329B628: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329B62C: 4BABA615  bl 0x82d55c40
	ctx.lr = 0x8329B630;
	sub_82D55C40(ctx, base);
	// 8329B630: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329B634: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329B638: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329B63C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329B640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329B640 size=108
    let mut pc: u32 = 0x8329B640;
    'dispatch: loop {
        match pc {
            0x8329B640 => {
    //   block [0x8329B640..0x8329B6AC)
	// 8329B640: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329B644: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329B648: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329B64C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329B650: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329B654: 38EB8480  addi r7, r11, -0x7b80
	ctx.r[7].s64 = ctx.r[11].s64 + -31616;
	// 8329B658: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8329B65C: 388A84B0  addi r4, r10, -0x7b50
	ctx.r[4].s64 = ctx.r[10].s64 + -31568;
	// 8329B660: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329B664: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329B668: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329B66C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329B670: 386A834C  addi r3, r10, -0x7cb4
	ctx.r[3].s64 = ctx.r[10].s64 + -31924;
	// 8329B674: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329B678: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329B67C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329B680: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329B684: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329B688: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329B68C: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 8329B690: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329B694: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329B698: 4BABA5A9  bl 0x82d55c40
	ctx.lr = 0x8329B69C;
	sub_82D55C40(ctx, base);
	// 8329B69C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329B6A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329B6A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329B6A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329B6B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329B6B0 size=108
    let mut pc: u32 = 0x8329B6B0;
    'dispatch: loop {
        match pc {
            0x8329B6B0 => {
    //   block [0x8329B6B0..0x8329B71C)
	// 8329B6B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329B6B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329B6B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329B6BC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329B6C0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329B6C4: 38EB84D0  addi r7, r11, -0x7b30
	ctx.r[7].s64 = ctx.r[11].s64 + -31536;
	// 8329B6C8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8329B6CC: 388A8518  addi r4, r10, -0x7ae8
	ctx.r[4].s64 = ctx.r[10].s64 + -31464;
	// 8329B6D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329B6D4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329B6D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329B6DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329B6E0: 386A837C  addi r3, r10, -0x7c84
	ctx.r[3].s64 = ctx.r[10].s64 + -31876;
	// 8329B6E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329B6E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329B6EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329B6F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329B6F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329B6F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329B6FC: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 8329B700: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329B704: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329B708: 4BABA539  bl 0x82d55c40
	ctx.lr = 0x8329B70C;
	sub_82D55C40(ctx, base);
	// 8329B70C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329B710: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329B714: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329B718: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329B720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329B720 size=108
    let mut pc: u32 = 0x8329B720;
    'dispatch: loop {
        match pc {
            0x8329B720 => {
    //   block [0x8329B720..0x8329B78C)
	// 8329B720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329B724: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329B728: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329B72C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329B730: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329B734: 38EB84E8  addi r7, r11, -0x7b18
	ctx.r[7].s64 = ctx.r[11].s64 + -31512;
	// 8329B738: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8329B73C: 388A853C  addi r4, r10, -0x7ac4
	ctx.r[4].s64 = ctx.r[10].s64 + -31428;
	// 8329B740: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329B744: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329B748: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329B74C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329B750: 386A83AC  addi r3, r10, -0x7c54
	ctx.r[3].s64 = ctx.r[10].s64 + -31828;
	// 8329B754: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329B758: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329B75C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329B760: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329B764: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329B768: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329B76C: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 8329B770: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329B774: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329B778: 4BABA4C9  bl 0x82d55c40
	ctx.lr = 0x8329B77C;
	sub_82D55C40(ctx, base);
	// 8329B77C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329B780: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329B784: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329B788: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329B790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329B790 size=108
    let mut pc: u32 = 0x8329B790;
    'dispatch: loop {
        match pc {
            0x8329B790 => {
    //   block [0x8329B790..0x8329B7FC)
	// 8329B790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329B794: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329B798: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329B79C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329B7A0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329B7A4: 38EB8564  addi r7, r11, -0x7a9c
	ctx.r[7].s64 = ctx.r[11].s64 + -31388;
	// 8329B7A8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8329B7AC: 388A857C  addi r4, r10, -0x7a84
	ctx.r[4].s64 = ctx.r[10].s64 + -31364;
	// 8329B7B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329B7B4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329B7B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329B7BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329B7C0: 386A83DC  addi r3, r10, -0x7c24
	ctx.r[3].s64 = ctx.r[10].s64 + -31780;
	// 8329B7C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329B7C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329B7CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329B7D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329B7D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329B7D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329B7DC: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 8329B7E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329B7E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329B7E8: 4BABA459  bl 0x82d55c40
	ctx.lr = 0x8329B7EC;
	sub_82D55C40(ctx, base);
	// 8329B7EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329B7F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329B7F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329B7F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329B800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329B800 size=100
    let mut pc: u32 = 0x8329B800;
    'dispatch: loop {
        match pc {
            0x8329B800 => {
    //   block [0x8329B800..0x8329B864)
	// 8329B800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329B804: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329B808: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329B80C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329B810: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329B814: 38AA8660  addi r5, r10, -0x79a0
	ctx.r[5].s64 = ctx.r[10].s64 + -31136;
	// 8329B818: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329B81C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329B820: 388A8CC0  addi r4, r10, -0x7340
	ctx.r[4].s64 = ctx.r[10].s64 + -29504;
	// 8329B824: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329B828: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329B82C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329B830: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329B834: 386A8410  addi r3, r10, -0x7bf0
	ctx.r[3].s64 = ctx.r[10].s64 + -31728;
	// 8329B838: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329B83C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329B840: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8329B844: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329B848: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8329B84C: 38C00120  li r6, 0x120
	ctx.r[6].s64 = 288;
	// 8329B850: 4BABA3F1  bl 0x82d55c40
	ctx.lr = 0x8329B854;
	sub_82D55C40(ctx, base);
	// 8329B854: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329B858: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329B85C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329B860: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329B868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329B868 size=112
    let mut pc: u32 = 0x8329B868;
    'dispatch: loop {
        match pc {
            0x8329B868 => {
    //   block [0x8329B868..0x8329B8D8)
	// 8329B868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329B86C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329B870: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329B874: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 8329B878: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329B87C: 38AAAE8C  addi r5, r10, -0x5174
	ctx.r[5].s64 = ctx.r[10].s64 + -20852;
	// 8329B880: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329B884: 390B8CF0  addi r8, r11, -0x7310
	ctx.r[8].s64 = ctx.r[11].s64 + -29456;
	// 8329B888: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8329B88C: 388A8D50  addi r4, r10, -0x72b0
	ctx.r[4].s64 = ctx.r[10].s64 + -29360;
	// 8329B890: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329B894: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329B898: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329B89C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329B8A0: 386A8440  addi r3, r10, -0x7bc0
	ctx.r[3].s64 = ctx.r[10].s64 + -31680;
	// 8329B8A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329B8A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329B8AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329B8B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329B8B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329B8B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329B8BC: 38C00018  li r6, 0x18
	ctx.r[6].s64 = 24;
	// 8329B8C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329B8C4: 4BABA37D  bl 0x82d55c40
	ctx.lr = 0x8329B8C8;
	sub_82D55C40(ctx, base);
	// 8329B8C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329B8CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329B8D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329B8D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329B8D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329B8D8 size=92
    let mut pc: u32 = 0x8329B8D8;
    'dispatch: loop {
        match pc {
            0x8329B8D8 => {
    //   block [0x8329B8D8..0x8329B934)
	// 8329B8D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329B8DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329B8E0: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329B8E4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8329B8E8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8329B8EC: 4BAEC0FD  bl 0x82d879e8
	ctx.lr = 0x8329B8F0;
	sub_82D879E8(ctx, base);
	// 8329B8F0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329B8F4: 3D0082D8  lis r8, -0x7d28
	ctx.r[8].s64 = -2099773440;
	// 8329B8F8: 394B8D9C  addi r10, r11, -0x7264
	ctx.r[10].s64 = ctx.r[11].s64 + -29284;
	// 8329B8FC: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8329B900: 3D2082D8  lis r9, -0x7d28
	ctx.r[9].s64 = -2099773440;
	// 8329B904: 396B8470  addi r11, r11, -0x7b90
	ctx.r[11].s64 = ctx.r[11].s64 + -31632;
	// 8329B908: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8329B90C: 39480ED0  addi r10, r8, 0xed0
	ctx.r[10].s64 = ctx.r[8].s64 + 3792;
	// 8329B910: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8329B914: 39490EB8  addi r10, r9, 0xeb8
	ctx.r[10].s64 = ctx.r[9].s64 + 3768;
	// 8329B918: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8329B91C: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8329B920: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8329B924: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 8329B928: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329B92C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329B930: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329B938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329B938 size=112
    let mut pc: u32 = 0x8329B938;
    'dispatch: loop {
        match pc {
            0x8329B938 => {
    //   block [0x8329B938..0x8329B9A8)
	// 8329B938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329B93C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329B940: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329B944: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329B948: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329B94C: 38AA8730  addi r5, r10, -0x78d0
	ctx.r[5].s64 = ctx.r[10].s64 + -30928;
	// 8329B950: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329B954: 390B8D6C  addi r8, r11, -0x7294
	ctx.r[8].s64 = ctx.r[11].s64 + -29332;
	// 8329B958: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8329B95C: 388A8D9C  addi r4, r10, -0x7264
	ctx.r[4].s64 = ctx.r[10].s64 + -29284;
	// 8329B960: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329B964: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329B968: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329B96C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329B970: 386A8480  addi r3, r10, -0x7b80
	ctx.r[3].s64 = ctx.r[10].s64 + -31616;
	// 8329B974: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329B978: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329B97C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329B980: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329B984: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329B988: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329B98C: 38C00058  li r6, 0x58
	ctx.r[6].s64 = 88;
	// 8329B990: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329B994: 4BABA2AD  bl 0x82d55c40
	ctx.lr = 0x8329B998;
	sub_82D55C40(ctx, base);
	// 8329B998: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329B99C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329B9A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329B9A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329B9A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329B9A8 size=112
    let mut pc: u32 = 0x8329B9A8;
    'dispatch: loop {
        match pc {
            0x8329B9A8 => {
    //   block [0x8329B9A8..0x8329BA18)
	// 8329B9A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329B9AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329B9B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329B9B4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329B9B8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329B9BC: 38AA8440  addi r5, r10, -0x7bc0
	ctx.r[5].s64 = ctx.r[10].s64 + -31680;
	// 8329B9C0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329B9C4: 390B8DC4  addi r8, r11, -0x723c
	ctx.r[8].s64 = ctx.r[11].s64 + -29244;
	// 8329B9C8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8329B9CC: 388A8DDC  addi r4, r10, -0x7224
	ctx.r[4].s64 = ctx.r[10].s64 + -29220;
	// 8329B9D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329B9D4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329B9D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329B9DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329B9E0: 386A84B0  addi r3, r10, -0x7b50
	ctx.r[3].s64 = ctx.r[10].s64 + -31568;
	// 8329B9E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329B9E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329B9EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329B9F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329B9F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329B9F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329B9FC: 38C00024  li r6, 0x24
	ctx.r[6].s64 = 36;
	// 8329BA00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329BA04: 4BABA23D  bl 0x82d55c40
	ctx.lr = 0x8329BA08;
	sub_82D55C40(ctx, base);
	// 8329BA08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329BA0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329BA10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329BA14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329BA18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329BA18 size=108
    let mut pc: u32 = 0x8329BA18;
    'dispatch: loop {
        match pc {
            0x8329BA18 => {
    //   block [0x8329BA18..0x8329BA84)
	// 8329BA18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329BA1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329BA20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329BA24: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329BA28: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329BA2C: 392B8E18  addi r9, r11, -0x71e8
	ctx.r[9].s64 = ctx.r[11].s64 + -29160;
	// 8329BA30: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8329BA34: 39090018  addi r8, r9, 0x18
	ctx.r[8].s64 = ctx.r[9].s64 + 24;
	// 8329BA38: 388A8EC4  addi r4, r10, -0x713c
	ctx.r[4].s64 = ctx.r[10].s64 + -28988;
	// 8329BA3C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329BA40: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329BA44: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8329BA48: 38C000A0  li r6, 0xa0
	ctx.r[6].s64 = 160;
	// 8329BA4C: 386A84E0  addi r3, r10, -0x7b20
	ctx.r[3].s64 = ctx.r[10].s64 + -31520;
	// 8329BA50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329BA54: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8329BA58: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329BA5C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329BA60: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329BA64: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329BA68: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329BA6C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329BA70: 4BABA1D1  bl 0x82d55c40
	ctx.lr = 0x8329BA74;
	sub_82D55C40(ctx, base);
	// 8329BA74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329BA78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329BA7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329BA80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329BA88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329BA88 size=112
    let mut pc: u32 = 0x8329BA88;
    'dispatch: loop {
        match pc {
            0x8329BA88 => {
    //   block [0x8329BA88..0x8329BAF8)
	// 8329BA88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329BA8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329BA90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329BA94: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329BA98: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329BA9C: 38AA8730  addi r5, r10, -0x78d0
	ctx.r[5].s64 = ctx.r[10].s64 + -30928;
	// 8329BAA0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329BAA4: 390B8E78  addi r8, r11, -0x7188
	ctx.r[8].s64 = ctx.r[11].s64 + -29064;
	// 8329BAA8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8329BAAC: 388A8EE0  addi r4, r10, -0x7120
	ctx.r[4].s64 = ctx.r[10].s64 + -28960;
	// 8329BAB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329BAB4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329BAB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329BABC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329BAC0: 386A8510  addi r3, r10, -0x7af0
	ctx.r[3].s64 = ctx.r[10].s64 + -31472;
	// 8329BAC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329BAC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329BACC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329BAD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329BAD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329BAD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329BADC: 38C000B0  li r6, 0xb0
	ctx.r[6].s64 = 176;
	// 8329BAE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329BAE4: 4BABA15D  bl 0x82d55c40
	ctx.lr = 0x8329BAE8;
	sub_82D55C40(ctx, base);
	// 8329BAE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329BAEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329BAF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329BAF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329BAF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329BAF8 size=112
    let mut pc: u32 = 0x8329BAF8;
    'dispatch: loop {
        match pc {
            0x8329BAF8 => {
    //   block [0x8329BAF8..0x8329BB68)
	// 8329BAF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329BAFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329BB00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329BB04: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329BB08: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329BB0C: 38AA8440  addi r5, r10, -0x7bc0
	ctx.r[5].s64 = ctx.r[10].s64 + -31680;
	// 8329BB10: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329BB14: 390B8F08  addi r8, r11, -0x70f8
	ctx.r[8].s64 = ctx.r[11].s64 + -28920;
	// 8329BB18: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8329BB1C: 388A8F38  addi r4, r10, -0x70c8
	ctx.r[4].s64 = ctx.r[10].s64 + -28872;
	// 8329BB20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329BB24: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329BB28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329BB2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329BB30: 386A8540  addi r3, r10, -0x7ac0
	ctx.r[3].s64 = ctx.r[10].s64 + -31424;
	// 8329BB34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329BB38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329BB3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329BB40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329BB44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329BB48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329BB4C: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 8329BB50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329BB54: 4BABA0ED  bl 0x82d55c40
	ctx.lr = 0x8329BB58;
	sub_82D55C40(ctx, base);
	// 8329BB58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329BB5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329BB60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329BB64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329BB68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329BB68 size=108
    let mut pc: u32 = 0x8329BB68;
    'dispatch: loop {
        match pc {
            0x8329BB68 => {
    //   block [0x8329BB68..0x8329BBD4)
	// 8329BB68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329BB6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329BB70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329BB74: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329BB78: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329BB7C: 392B8FA0  addi r9, r11, -0x7060
	ctx.r[9].s64 = ctx.r[11].s64 + -28768;
	// 8329BB80: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8329BB84: 39090018  addi r8, r9, 0x18
	ctx.r[8].s64 = ctx.r[9].s64 + 24;
	// 8329BB88: 388A904C  addi r4, r10, -0x6fb4
	ctx.r[4].s64 = ctx.r[10].s64 + -28596;
	// 8329BB8C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329BB90: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329BB94: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8329BB98: 38C00090  li r6, 0x90
	ctx.r[6].s64 = 144;
	// 8329BB9C: 386A8570  addi r3, r10, -0x7a90
	ctx.r[3].s64 = ctx.r[10].s64 + -31376;
	// 8329BBA0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329BBA4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8329BBA8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329BBAC: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329BBB0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329BBB4: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329BBB8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329BBBC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329BBC0: 4BABA081  bl 0x82d55c40
	ctx.lr = 0x8329BBC4;
	sub_82D55C40(ctx, base);
	// 8329BBC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329BBC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329BBCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329BBD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329BBD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329BBD8 size=112
    let mut pc: u32 = 0x8329BBD8;
    'dispatch: loop {
        match pc {
            0x8329BBD8 => {
    //   block [0x8329BBD8..0x8329BC48)
	// 8329BBD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329BBDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329BBE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329BBE4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329BBE8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329BBEC: 38AA8730  addi r5, r10, -0x78d0
	ctx.r[5].s64 = ctx.r[10].s64 + -30928;
	// 8329BBF0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329BBF4: 390B9000  addi r8, r11, -0x7000
	ctx.r[8].s64 = ctx.r[11].s64 + -28672;
	// 8329BBF8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8329BBFC: 388A9064  addi r4, r10, -0x6f9c
	ctx.r[4].s64 = ctx.r[10].s64 + -28572;
	// 8329BC00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329BC04: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329BC08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329BC0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329BC10: 386A85A0  addi r3, r10, -0x7a60
	ctx.r[3].s64 = ctx.r[10].s64 + -31328;
	// 8329BC14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329BC18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329BC1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329BC20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329BC24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329BC28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329BC2C: 38C000A0  li r6, 0xa0
	ctx.r[6].s64 = 160;
	// 8329BC30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329BC34: 4BABA00D  bl 0x82d55c40
	ctx.lr = 0x8329BC38;
	sub_82D55C40(ctx, base);
	// 8329BC38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329BC3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329BC40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329BC44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329BC48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329BC48 size=108
    let mut pc: u32 = 0x8329BC48;
    'dispatch: loop {
        match pc {
            0x8329BC48 => {
    //   block [0x8329BC48..0x8329BCB4)
	// 8329BC48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329BC4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329BC50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329BC54: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329BC58: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329BC5C: 38EB9080  addi r7, r11, -0x6f80
	ctx.r[7].s64 = ctx.r[11].s64 + -28544;
	// 8329BC60: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8329BC64: 388A90FC  addi r4, r10, -0x6f04
	ctx.r[4].s64 = ctx.r[10].s64 + -28420;
	// 8329BC68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329BC6C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329BC70: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329BC74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329BC78: 386A85D0  addi r3, r10, -0x7a30
	ctx.r[3].s64 = ctx.r[10].s64 + -31280;
	// 8329BC7C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329BC80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329BC84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329BC88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329BC8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329BC90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329BC94: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 8329BC98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329BC9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329BCA0: 4BAB9FA1  bl 0x82d55c40
	ctx.lr = 0x8329BCA4;
	sub_82D55C40(ctx, base);
	// 8329BCA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329BCA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329BCAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329BCB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329BCB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329BCB8 size=112
    let mut pc: u32 = 0x8329BCB8;
    'dispatch: loop {
        match pc {
            0x8329BCB8 => {
    //   block [0x8329BCB8..0x8329BD28)
	// 8329BCB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329BCBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329BCC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329BCC4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329BCC8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329BCCC: 38AA8730  addi r5, r10, -0x78d0
	ctx.r[5].s64 = ctx.r[10].s64 + -30928;
	// 8329BCD0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329BCD4: 390B90B0  addi r8, r11, -0x6f50
	ctx.r[8].s64 = ctx.r[11].s64 + -28496;
	// 8329BCD8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8329BCDC: 388A9120  addi r4, r10, -0x6ee0
	ctx.r[4].s64 = ctx.r[10].s64 + -28384;
	// 8329BCE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329BCE4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329BCE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329BCEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329BCF0: 386A8600  addi r3, r10, -0x7a00
	ctx.r[3].s64 = ctx.r[10].s64 + -31232;
	// 8329BCF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329BCF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329BCFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329BD00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329BD04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329BD08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329BD0C: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 8329BD10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329BD14: 4BAB9F2D  bl 0x82d55c40
	ctx.lr = 0x8329BD18;
	sub_82D55C40(ctx, base);
	// 8329BD18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329BD1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329BD20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329BD24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329BD28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329BD28 size=100
    let mut pc: u32 = 0x8329BD28;
    'dispatch: loop {
        match pc {
            0x8329BD28 => {
    //   block [0x8329BD28..0x8329BD8C)
	// 8329BD28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329BD2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329BD30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329BD34: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329BD38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329BD3C: 38AA8730  addi r5, r10, -0x78d0
	ctx.r[5].s64 = ctx.r[10].s64 + -30928;
	// 8329BD40: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329BD44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329BD48: 388A9140  addi r4, r10, -0x6ec0
	ctx.r[4].s64 = ctx.r[10].s64 + -28352;
	// 8329BD4C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329BD50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329BD54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329BD58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329BD5C: 386A8630  addi r3, r10, -0x79d0
	ctx.r[3].s64 = ctx.r[10].s64 + -31184;
	// 8329BD60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329BD64: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329BD68: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8329BD6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329BD70: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8329BD74: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 8329BD78: 4BAB9EC9  bl 0x82d55c40
	ctx.lr = 0x8329BD7C;
	sub_82D55C40(ctx, base);
	// 8329BD7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329BD80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329BD84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329BD88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329BD90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329BD90 size=100
    let mut pc: u32 = 0x8329BD90;
    'dispatch: loop {
        match pc {
            0x8329BD90 => {
    //   block [0x8329BD90..0x8329BDF4)
	// 8329BD90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329BD94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329BD98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329BD9C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329BDA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329BDA4: 38AA89F0  addi r5, r10, -0x7610
	ctx.r[5].s64 = ctx.r[10].s64 + -30224;
	// 8329BDA8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329BDAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329BDB0: 388A91C8  addi r4, r10, -0x6e38
	ctx.r[4].s64 = ctx.r[10].s64 + -28216;
	// 8329BDB4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329BDB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329BDBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329BDC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329BDC4: 386A8660  addi r3, r10, -0x79a0
	ctx.r[3].s64 = ctx.r[10].s64 + -31136;
	// 8329BDC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329BDCC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329BDD0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8329BDD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329BDD8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8329BDDC: 38C00120  li r6, 0x120
	ctx.r[6].s64 = 288;
	// 8329BDE0: 4BAB9E61  bl 0x82d55c40
	ctx.lr = 0x8329BDE4;
	sub_82D55C40(ctx, base);
	// 8329BDE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329BDE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329BDEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329BDF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329BDF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329BDF8 size=100
    let mut pc: u32 = 0x8329BDF8;
    'dispatch: loop {
        match pc {
            0x8329BDF8 => {
    //   block [0x8329BDF8..0x8329BE5C)
	// 8329BDF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329BDFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329BE00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329BE04: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329BE08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329BE0C: 38AA8660  addi r5, r10, -0x79a0
	ctx.r[5].s64 = ctx.r[10].s64 + -31136;
	// 8329BE10: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329BE14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329BE18: 388A91E0  addi r4, r10, -0x6e20
	ctx.r[4].s64 = ctx.r[10].s64 + -28192;
	// 8329BE1C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329BE20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329BE24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329BE28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329BE2C: 386A8690  addi r3, r10, -0x7970
	ctx.r[3].s64 = ctx.r[10].s64 + -31088;
	// 8329BE30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329BE34: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329BE38: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8329BE3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329BE40: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8329BE44: 38C00120  li r6, 0x120
	ctx.r[6].s64 = 288;
	// 8329BE48: 4BAB9DF9  bl 0x82d55c40
	ctx.lr = 0x8329BE4C;
	sub_82D55C40(ctx, base);
	// 8329BE4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329BE50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329BE54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329BE58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329BE60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329BE60 size=108
    let mut pc: u32 = 0x8329BE60;
    'dispatch: loop {
        match pc {
            0x8329BE60 => {
    //   block [0x8329BE60..0x8329BECC)
	// 8329BE60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329BE64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329BE68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329BE6C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329BE70: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329BE74: 38EB9238  addi r7, r11, -0x6dc8
	ctx.r[7].s64 = ctx.r[11].s64 + -28104;
	// 8329BE78: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8329BE7C: 388A92F8  addi r4, r10, -0x6d08
	ctx.r[4].s64 = ctx.r[10].s64 + -27912;
	// 8329BE80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329BE84: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329BE88: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329BE8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329BE90: 386A86C0  addi r3, r10, -0x7940
	ctx.r[3].s64 = ctx.r[10].s64 + -31040;
	// 8329BE94: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329BE98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329BE9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329BEA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329BEA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329BEA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329BEAC: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 8329BEB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329BEB4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329BEB8: 4BAB9D89  bl 0x82d55c40
	ctx.lr = 0x8329BEBC;
	sub_82D55C40(ctx, base);
	// 8329BEBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329BEC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329BEC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329BEC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329BED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329BED0 size=92
    let mut pc: u32 = 0x8329BED0;
    'dispatch: loop {
        match pc {
            0x8329BED0 => {
    //   block [0x8329BED0..0x8329BF2C)
	// 8329BED0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329BED4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329BED8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329BEDC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8329BEE0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8329BEE4: 4BB012ED  bl 0x82d9d1d0
	ctx.lr = 0x8329BEE8;
	sub_82D9D1D0(ctx, base);
	// 8329BEE8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329BEEC: 3D0082D8  lis r8, -0x7d28
	ctx.r[8].s64 = -2099773440;
	// 8329BEF0: 394B9320  addi r10, r11, -0x6ce0
	ctx.r[10].s64 = ctx.r[11].s64 + -27872;
	// 8329BEF4: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8329BEF8: 3D2082D8  lis r9, -0x7d28
	ctx.r[9].s64 = -2099773440;
	// 8329BEFC: 396B86F0  addi r11, r11, -0x7910
	ctx.r[11].s64 = ctx.r[11].s64 + -30992;
	// 8329BF00: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8329BF04: 39481130  addi r10, r8, 0x1130
	ctx.r[10].s64 = ctx.r[8].s64 + 4400;
	// 8329BF08: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8329BF0C: 39491148  addi r10, r9, 0x1148
	ctx.r[10].s64 = ctx.r[9].s64 + 4424;
	// 8329BF10: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8329BF14: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8329BF18: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8329BF1C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8329BF20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329BF24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329BF28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329BF30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329BF30 size=112
    let mut pc: u32 = 0x8329BF30;
    'dispatch: loop {
        match pc {
            0x8329BF30 => {
    //   block [0x8329BF30..0x8329BFA0)
	// 8329BF30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329BF34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329BF38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329BF3C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329BF40: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329BF44: 38AA8630  addi r5, r10, -0x79d0
	ctx.r[5].s64 = ctx.r[10].s64 + -31184;
	// 8329BF48: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329BF4C: 390B9268  addi r8, r11, -0x6d98
	ctx.r[8].s64 = ctx.r[11].s64 + -28056;
	// 8329BF50: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8329BF54: 388A9320  addi r4, r10, -0x6ce0
	ctx.r[4].s64 = ctx.r[10].s64 + -27872;
	// 8329BF58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329BF5C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329BF60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329BF64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329BF68: 386A8700  addi r3, r10, -0x7900
	ctx.r[3].s64 = ctx.r[10].s64 + -30976;
	// 8329BF6C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329BF70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329BF74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329BF78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329BF7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329BF80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329BF84: 38C00034  li r6, 0x34
	ctx.r[6].s64 = 52;
	// 8329BF88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329BF8C: 4BAB9CB5  bl 0x82d55c40
	ctx.lr = 0x8329BF90;
	sub_82D55C40(ctx, base);
	// 8329BF90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329BF94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329BF98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329BF9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329BFA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329BFA0 size=112
    let mut pc: u32 = 0x8329BFA0;
    'dispatch: loop {
        match pc {
            0x8329BFA0 => {
    //   block [0x8329BFA0..0x8329C010)
	// 8329BFA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329BFA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329BFA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329BFAC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329BFB0: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 8329BFB4: 392B9650  addi r9, r11, -0x69b0
	ctx.r[9].s64 = ctx.r[11].s64 + -27056;
	// 8329BFB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329BFBC: 39090014  addi r8, r9, 0x14
	ctx.r[8].s64 = ctx.r[9].s64 + 20;
	// 8329BFC0: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8329BFC4: 38AAAE8C  addi r5, r10, -0x5174
	ctx.r[5].s64 = ctx.r[10].s64 + -20852;
	// 8329BFC8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329BFCC: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329BFD0: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 8329BFD4: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329BFD8: 388A967C  addi r4, r10, -0x6984
	ctx.r[4].s64 = ctx.r[10].s64 + -27012;
	// 8329BFDC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329BFE0: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8329BFE4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8329BFE8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8329BFEC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329BFF0: 386B8730  addi r3, r11, -0x78d0
	ctx.r[3].s64 = ctx.r[11].s64 + -30928;
	// 8329BFF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329BFF8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329BFFC: 4BAB9C45  bl 0x82d55c40
	ctx.lr = 0x8329C000;
	sub_82D55C40(ctx, base);
	// 8329C000: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329C004: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329C008: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329C00C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329C010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329C010 size=108
    let mut pc: u32 = 0x8329C010;
    'dispatch: loop {
        match pc {
            0x8329C010 => {
    //   block [0x8329C010..0x8329C07C)
	// 8329C010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329C014: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329C018: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329C01C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329C020: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329C024: 392B96C0  addi r9, r11, -0x6940
	ctx.r[9].s64 = ctx.r[11].s64 + -26944;
	// 8329C028: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 8329C02C: 39090018  addi r8, r9, 0x18
	ctx.r[8].s64 = ctx.r[9].s64 + 24;
	// 8329C030: 388A97B4  addi r4, r10, -0x684c
	ctx.r[4].s64 = ctx.r[10].s64 + -26700;
	// 8329C034: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329C038: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329C03C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8329C040: 38C000D0  li r6, 0xd0
	ctx.r[6].s64 = 208;
	// 8329C044: 386A8760  addi r3, r10, -0x78a0
	ctx.r[3].s64 = ctx.r[10].s64 + -30880;
	// 8329C048: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329C04C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8329C050: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329C054: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329C058: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329C05C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329C060: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329C064: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329C068: 4BAB9BD9  bl 0x82d55c40
	ctx.lr = 0x8329C06C;
	sub_82D55C40(ctx, base);
	// 8329C06C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329C070: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329C074: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329C078: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329C080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329C080 size=112
    let mut pc: u32 = 0x8329C080;
    'dispatch: loop {
        match pc {
            0x8329C080 => {
    //   block [0x8329C080..0x8329C0F0)
	// 8329C080: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329C084: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329C088: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329C08C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329C090: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329C094: 38AA8730  addi r5, r10, -0x78d0
	ctx.r[5].s64 = ctx.r[10].s64 + -30928;
	// 8329C098: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329C09C: 390B9768  addi r8, r11, -0x6898
	ctx.r[8].s64 = ctx.r[11].s64 + -26776;
	// 8329C0A0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8329C0A4: 388A97D8  addi r4, r10, -0x6828
	ctx.r[4].s64 = ctx.r[10].s64 + -26664;
	// 8329C0A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329C0AC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329C0B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329C0B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329C0B8: 386A8790  addi r3, r10, -0x7870
	ctx.r[3].s64 = ctx.r[10].s64 + -30832;
	// 8329C0BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329C0C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329C0C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329C0C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329C0CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329C0D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329C0D4: 38C000E0  li r6, 0xe0
	ctx.r[6].s64 = 224;
	// 8329C0D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329C0DC: 4BAB9B65  bl 0x82d55c40
	ctx.lr = 0x8329C0E0;
	sub_82D55C40(ctx, base);
	// 8329C0E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329C0E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329C0E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329C0EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329C0F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329C0F0 size=108
    let mut pc: u32 = 0x8329C0F0;
    'dispatch: loop {
        match pc {
            0x8329C0F0 => {
    //   block [0x8329C0F0..0x8329C15C)
	// 8329C0F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329C0F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329C0F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329C0FC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329C100: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329C104: 38EB9A80  addi r7, r11, -0x6580
	ctx.r[7].s64 = ctx.r[11].s64 + -25984;
	// 8329C108: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8329C10C: 388A9D38  addi r4, r10, -0x62c8
	ctx.r[4].s64 = ctx.r[10].s64 + -25288;
	// 8329C110: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329C114: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329C118: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329C11C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329C120: 386A87C0  addi r3, r10, -0x7840
	ctx.r[3].s64 = ctx.r[10].s64 + -30784;
	// 8329C124: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329C128: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329C12C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329C130: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329C134: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329C138: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329C13C: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8329C140: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329C144: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329C148: 4BAB9AF9  bl 0x82d55c40
	ctx.lr = 0x8329C14C;
	sub_82D55C40(ctx, base);
	// 8329C14C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329C150: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329C154: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329C158: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329C160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329C160 size=108
    let mut pc: u32 = 0x8329C160;
    'dispatch: loop {
        match pc {
            0x8329C160 => {
    //   block [0x8329C160..0x8329C1CC)
	// 8329C160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329C164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329C168: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329C16C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329C170: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329C174: 38EB9AC8  addi r7, r11, -0x6538
	ctx.r[7].s64 = ctx.r[11].s64 + -25912;
	// 8329C178: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8329C17C: 388A9D64  addi r4, r10, -0x629c
	ctx.r[4].s64 = ctx.r[10].s64 + -25244;
	// 8329C180: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329C184: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329C188: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329C18C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329C190: 386A87F0  addi r3, r10, -0x7810
	ctx.r[3].s64 = ctx.r[10].s64 + -30736;
	// 8329C194: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329C198: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329C19C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329C1A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329C1A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329C1A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329C1AC: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8329C1B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329C1B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329C1B8: 4BAB9A89  bl 0x82d55c40
	ctx.lr = 0x8329C1BC;
	sub_82D55C40(ctx, base);
	// 8329C1BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329C1C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329C1C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329C1C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329C1D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329C1D0 size=108
    let mut pc: u32 = 0x8329C1D0;
    'dispatch: loop {
        match pc {
            0x8329C1D0 => {
    //   block [0x8329C1D0..0x8329C23C)
	// 8329C1D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329C1D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329C1D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329C1DC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329C1E0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329C1E4: 38EB9B28  addi r7, r11, -0x64d8
	ctx.r[7].s64 = ctx.r[11].s64 + -25816;
	// 8329C1E8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8329C1EC: 388A9D84  addi r4, r10, -0x627c
	ctx.r[4].s64 = ctx.r[10].s64 + -25212;
	// 8329C1F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329C1F4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329C1F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329C1FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329C200: 386A8820  addi r3, r10, -0x77e0
	ctx.r[3].s64 = ctx.r[10].s64 + -30688;
	// 8329C204: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329C208: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329C20C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329C210: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329C214: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329C218: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329C21C: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 8329C220: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329C224: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329C228: 4BAB9A19  bl 0x82d55c40
	ctx.lr = 0x8329C22C;
	sub_82D55C40(ctx, base);
	// 8329C22C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329C230: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329C234: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329C238: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329C240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329C240 size=92
    let mut pc: u32 = 0x8329C240;
    'dispatch: loop {
        match pc {
            0x8329C240 => {
    //   block [0x8329C240..0x8329C29C)
	// 8329C240: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329C244: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329C248: 9421FDA0  stwu r1, -0x260(r1)
	ea = ctx.r[1].u32.wrapping_add(-608 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329C24C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8329C250: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8329C254: 4BAE98DD  bl 0x82d85b30
	ctx.lr = 0x8329C258;
	sub_82D85B30(ctx, base);
	// 8329C258: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329C25C: 3D0082D8  lis r8, -0x7d28
	ctx.r[8].s64 = -2099773440;
	// 8329C260: 394B9DA0  addi r10, r11, -0x6260
	ctx.r[10].s64 = ctx.r[11].s64 + -25184;
	// 8329C264: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8329C268: 3D2082D8  lis r9, -0x7d28
	ctx.r[9].s64 = -2099773440;
	// 8329C26C: 396B8850  addi r11, r11, -0x77b0
	ctx.r[11].s64 = ctx.r[11].s64 + -30640;
	// 8329C270: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8329C274: 39481278  addi r10, r8, 0x1278
	ctx.r[10].s64 = ctx.r[8].s64 + 4728;
	// 8329C278: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8329C27C: 39491260  addi r10, r9, 0x1260
	ctx.r[10].s64 = ctx.r[9].s64 + 4704;
	// 8329C280: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8329C284: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8329C288: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8329C28C: 38210260  addi r1, r1, 0x260
	ctx.r[1].s64 = ctx.r[1].s64 + 608;
	// 8329C290: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329C294: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329C298: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329C2A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329C2A0 size=116
    let mut pc: u32 = 0x8329C2A0;
    'dispatch: loop {
        match pc {
            0x8329C2A0 => {
    //   block [0x8329C2A0..0x8329C314)
	// 8329C2A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329C2A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329C2A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329C2AC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329C2B0: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329C2B4: 392B9A68  addi r9, r11, -0x6598
	ctx.r[9].s64 = ctx.r[11].s64 + -26008;
	// 8329C2B8: 38AA8D50  addi r5, r10, -0x72b0
	ctx.r[5].s64 = ctx.r[10].s64 + -29360;
	// 8329C2BC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329C2C0: 38E90288  addi r7, r9, 0x288
	ctx.r[7].s64 = ctx.r[9].s64 + 648;
	// 8329C2C4: 38C00011  li r6, 0x11
	ctx.r[6].s64 = 17;
	// 8329C2C8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329C2CC: 388A9DA0  addi r4, r10, -0x6260
	ctx.r[4].s64 = ctx.r[10].s64 + -25184;
	// 8329C2D0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329C2D4: 396B9B58  addi r11, r11, -0x64a8
	ctx.r[11].s64 = ctx.r[11].s64 + -25768;
	// 8329C2D8: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8329C2DC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329C2E0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8329C2E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329C2E8: 386A8860  addi r3, r10, -0x77a0
	ctx.r[3].s64 = ctx.r[10].s64 + -30624;
	// 8329C2EC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8329C2F0: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8329C2F4: 38C00200  li r6, 0x200
	ctx.r[6].s64 = 512;
	// 8329C2F8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8329C2FC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8329C300: 4BAB9941  bl 0x82d55c40
	ctx.lr = 0x8329C304;
	sub_82D55C40(ctx, base);
	// 8329C304: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329C308: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329C30C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329C310: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329C318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8329C318 size=24
    let mut pc: u32 = 0x8329C318;
    'dispatch: loop {
        match pc {
            0x8329C318 => {
    //   block [0x8329C318..0x8329C330)
	// 8329C318: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 8329C31C: 3D408330  lis r10, -0x7cd0
	ctx.r[10].s64 = -2094006272;
	// 8329C320: 394A89CC  addi r10, r10, -0x7634
	ctx.r[10].s64 = ctx.r[10].s64 + -30260;
	// 8329C324: 816B89C8  lwz r11, -0x7638(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-30264 as u32) ) } as u64;
	// 8329C328: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8329C32C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329C330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329C330 size=116
    let mut pc: u32 = 0x8329C330;
    'dispatch: loop {
        match pc {
            0x8329C330 => {
    //   block [0x8329C330..0x8329C3A4)
	// 8329C330: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329C334: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329C338: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329C33C: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 8329C340: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329C344: 390B89CC  addi r8, r11, -0x7634
	ctx.r[8].s64 = ctx.r[11].s64 + -30260;
	// 8329C348: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329C34C: 392A9E3C  addi r9, r10, -0x61c4
	ctx.r[9].s64 = ctx.r[10].s64 + -25028;
	// 8329C350: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 8329C354: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8329C358: 38AAAE8C  addi r5, r10, -0x5174
	ctx.r[5].s64 = ctx.r[10].s64 + -20852;
	// 8329C35C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329C360: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329C364: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329C368: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329C36C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329C370: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329C374: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8329C378: 388A9E50  addi r4, r10, -0x61b0
	ctx.r[4].s64 = ctx.r[10].s64 + -25008;
	// 8329C37C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8329C380: 386B8890  addi r3, r11, -0x7770
	ctx.r[3].s64 = ctx.r[11].s64 + -30576;
	// 8329C384: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8329C388: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329C38C: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 8329C390: 4BAB98B1  bl 0x82d55c40
	ctx.lr = 0x8329C394;
	sub_82D55C40(ctx, base);
	// 8329C394: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329C398: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329C39C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329C3A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329C3A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329C3A8 size=112
    let mut pc: u32 = 0x8329C3A8;
    'dispatch: loop {
        match pc {
            0x8329C3A8 => {
    //   block [0x8329C3A8..0x8329C418)
	// 8329C3A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329C3AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329C3B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329C3B4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329C3B8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329C3BC: 38AA8A50  addi r5, r10, -0x75b0
	ctx.r[5].s64 = ctx.r[10].s64 + -30128;
	// 8329C3C0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329C3C4: 390B9EB0  addi r8, r11, -0x6150
	ctx.r[8].s64 = ctx.r[11].s64 + -24912;
	// 8329C3C8: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8329C3CC: 388A9F5C  addi r4, r10, -0x60a4
	ctx.r[4].s64 = ctx.r[10].s64 + -24740;
	// 8329C3D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329C3D4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329C3D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329C3DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329C3E0: 386A88C0  addi r3, r10, -0x7740
	ctx.r[3].s64 = ctx.r[10].s64 + -30528;
	// 8329C3E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329C3E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329C3EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329C3F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329C3F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329C3F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329C3FC: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 8329C400: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329C404: 4BAB983D  bl 0x82d55c40
	ctx.lr = 0x8329C408;
	sub_82D55C40(ctx, base);
	// 8329C408: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329C40C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329C410: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329C414: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329C418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329C418 size=100
    let mut pc: u32 = 0x8329C418;
    'dispatch: loop {
        match pc {
            0x8329C418 => {
    //   block [0x8329C418..0x8329C47C)
	// 8329C418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329C41C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329C420: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329C424: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 8329C428: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329C42C: 38AAAE8C  addi r5, r10, -0x5174
	ctx.r[5].s64 = ctx.r[10].s64 + -20852;
	// 8329C430: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329C434: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329C438: 388A9F78  addi r4, r10, -0x6088
	ctx.r[4].s64 = ctx.r[10].s64 + -24712;
	// 8329C43C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329C440: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329C444: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329C448: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329C44C: 386A88F0  addi r3, r10, -0x7710
	ctx.r[3].s64 = ctx.r[10].s64 + -30480;
	// 8329C450: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329C454: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329C458: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8329C45C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329C460: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8329C464: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8329C468: 4BAB97D9  bl 0x82d55c40
	ctx.lr = 0x8329C46C;
	sub_82D55C40(ctx, base);
	// 8329C46C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329C470: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329C474: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329C478: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329C480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329C480 size=92
    let mut pc: u32 = 0x8329C480;
    'dispatch: loop {
        match pc {
            0x8329C480 => {
    //   block [0x8329C480..0x8329C4DC)
	// 8329C480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329C484: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329C488: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329C48C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8329C490: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8329C494: 4BB01E95  bl 0x82d9e328
	ctx.lr = 0x8329C498;
	sub_82D9E328(ctx, base);
	// 8329C498: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329C49C: 3D0082D8  lis r8, -0x7d28
	ctx.r[8].s64 = -2099773440;
	// 8329C4A0: 394B9FF8  addi r10, r11, -0x6008
	ctx.r[10].s64 = ctx.r[11].s64 + -24584;
	// 8329C4A4: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8329C4A8: 3D2082D8  lis r9, -0x7d28
	ctx.r[9].s64 = -2099773440;
	// 8329C4AC: 396B8920  addi r11, r11, -0x76e0
	ctx.r[11].s64 = ctx.r[11].s64 + -30432;
	// 8329C4B0: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8329C4B4: 39481538  addi r10, r8, 0x1538
	ctx.r[10].s64 = ctx.r[8].s64 + 5432;
	// 8329C4B8: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8329C4BC: 39491520  addi r10, r9, 0x1520
	ctx.r[10].s64 = ctx.r[9].s64 + 5408;
	// 8329C4C0: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8329C4C4: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8329C4C8: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8329C4CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329C4D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329C4D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329C4D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329C4E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329C4E0 size=112
    let mut pc: u32 = 0x8329C4E0;
    'dispatch: loop {
        match pc {
            0x8329C4E0 => {
    //   block [0x8329C4E0..0x8329C550)
	// 8329C4E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329C4E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329C4E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329C4EC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329C4F0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329C4F4: 38AA8730  addi r5, r10, -0x78d0
	ctx.r[5].s64 = ctx.r[10].s64 + -30928;
	// 8329C4F8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329C4FC: 390B9FB0  addi r8, r11, -0x6050
	ctx.r[8].s64 = ctx.r[11].s64 + -24656;
	// 8329C500: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8329C504: 388A9FF8  addi r4, r10, -0x6008
	ctx.r[4].s64 = ctx.r[10].s64 + -24584;
	// 8329C508: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329C50C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329C510: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329C514: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329C518: 386A8930  addi r3, r10, -0x76d0
	ctx.r[3].s64 = ctx.r[10].s64 + -30416;
	// 8329C51C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329C520: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329C524: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329C528: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329C52C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329C530: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329C534: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 8329C538: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329C53C: 4BAB9705  bl 0x82d55c40
	ctx.lr = 0x8329C540;
	sub_82D55C40(ctx, base);
	// 8329C540: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329C544: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329C548: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329C54C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329C550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329C550 size=112
    let mut pc: u32 = 0x8329C550;
    'dispatch: loop {
        match pc {
            0x8329C550 => {
    //   block [0x8329C550..0x8329C5C0)
	// 8329C550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329C554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329C558: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329C55C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329C560: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329C564: 38AA8890  addi r5, r10, -0x7770
	ctx.r[5].s64 = ctx.r[10].s64 + -30576;
	// 8329C568: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329C56C: 390BA02C  addi r8, r11, -0x5fd4
	ctx.r[8].s64 = ctx.r[11].s64 + -24532;
	// 8329C570: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8329C574: 388AA05C  addi r4, r10, -0x5fa4
	ctx.r[4].s64 = ctx.r[10].s64 + -24484;
	// 8329C578: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329C57C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329C580: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329C584: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329C588: 386A8960  addi r3, r10, -0x76a0
	ctx.r[3].s64 = ctx.r[10].s64 + -30368;
	// 8329C58C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329C590: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329C594: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329C598: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329C59C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329C5A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329C5A4: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 8329C5A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329C5AC: 4BAB9695  bl 0x82d55c40
	ctx.lr = 0x8329C5B0;
	sub_82D55C40(ctx, base);
	// 8329C5B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329C5B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329C5B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329C5BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329C5C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329C5C0 size=108
    let mut pc: u32 = 0x8329C5C0;
    'dispatch: loop {
        match pc {
            0x8329C5C0 => {
    //   block [0x8329C5C0..0x8329C62C)
	// 8329C5C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329C5C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329C5C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329C5CC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329C5D0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329C5D4: 38EBA0E8  addi r7, r11, -0x5f18
	ctx.r[7].s64 = ctx.r[11].s64 + -24344;
	// 8329C5D8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8329C5DC: 388AA1C0  addi r4, r10, -0x5e40
	ctx.r[4].s64 = ctx.r[10].s64 + -24128;
	// 8329C5E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329C5E4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329C5E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329C5EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329C5F0: 386A8990  addi r3, r10, -0x7670
	ctx.r[3].s64 = ctx.r[10].s64 + -30320;
	// 8329C5F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329C5F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329C5FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329C600: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329C604: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329C608: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329C60C: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 8329C610: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329C614: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329C618: 4BAB9629  bl 0x82d55c40
	ctx.lr = 0x8329C61C;
	sub_82D55C40(ctx, base);
	// 8329C61C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329C620: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329C624: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329C628: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329C630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329C630 size=108
    let mut pc: u32 = 0x8329C630;
    'dispatch: loop {
        match pc {
            0x8329C630 => {
    //   block [0x8329C630..0x8329C69C)
	// 8329C630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329C634: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329C638: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329C63C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329C640: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329C644: 38EBA148  addi r7, r11, -0x5eb8
	ctx.r[7].s64 = ctx.r[11].s64 + -24248;
	// 8329C648: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 8329C64C: 388AA1F0  addi r4, r10, -0x5e10
	ctx.r[4].s64 = ctx.r[10].s64 + -24080;
	// 8329C650: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329C654: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329C658: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329C65C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329C660: 386A89C0  addi r3, r10, -0x7640
	ctx.r[3].s64 = ctx.r[10].s64 + -30272;
	// 8329C664: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329C668: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329C66C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329C670: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329C674: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329C678: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329C67C: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 8329C680: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329C684: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329C688: 4BAB95B9  bl 0x82d55c40
	ctx.lr = 0x8329C68C;
	sub_82D55C40(ctx, base);
	// 8329C68C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329C690: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329C694: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329C698: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329C6A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8329C6A0 size=24
    let mut pc: u32 = 0x8329C6A0;
    'dispatch: loop {
        match pc {
            0x8329C6A0 => {
    //   block [0x8329C6A0..0x8329C6B8)
	// 8329C6A0: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 8329C6A4: 3D408330  lis r10, -0x7cd0
	ctx.r[10].s64 = -2094006272;
	// 8329C6A8: 394A8A68  addi r10, r10, -0x7598
	ctx.r[10].s64 = ctx.r[10].s64 + -30104;
	// 8329C6AC: 816B8A60  lwz r11, -0x75a0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-30112 as u32) ) } as u64;
	// 8329C6B0: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8329C6B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329C6B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329C6B8 size=116
    let mut pc: u32 = 0x8329C6B8;
    'dispatch: loop {
        match pc {
            0x8329C6B8 => {
    //   block [0x8329C6B8..0x8329C72C)
	// 8329C6B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329C6BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329C6C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329C6C4: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 8329C6C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329C6CC: 390B8A68  addi r8, r11, -0x7598
	ctx.r[8].s64 = ctx.r[11].s64 + -30104;
	// 8329C6D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329C6D4: 392AA358  addi r9, r10, -0x5ca8
	ctx.r[9].s64 = ctx.r[10].s64 + -23720;
	// 8329C6D8: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 8329C6DC: 38E0000B  li r7, 0xb
	ctx.r[7].s64 = 11;
	// 8329C6E0: 38AAAE8C  addi r5, r10, -0x5174
	ctx.r[5].s64 = ctx.r[10].s64 + -20852;
	// 8329C6E4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329C6E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329C6EC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329C6F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329C6F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329C6F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329C6FC: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8329C700: 388AA444  addi r4, r10, -0x5bbc
	ctx.r[4].s64 = ctx.r[10].s64 + -23484;
	// 8329C704: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8329C708: 386B89F0  addi r3, r11, -0x7610
	ctx.r[3].s64 = ctx.r[11].s64 + -30224;
	// 8329C70C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8329C710: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329C714: 38C00120  li r6, 0x120
	ctx.r[6].s64 = 288;
	// 8329C718: 4BAB9529  bl 0x82d55c40
	ctx.lr = 0x8329C71C;
	sub_82D55C40(ctx, base);
	// 8329C71C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329C720: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329C724: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329C728: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329C730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329C730 size=116
    let mut pc: u32 = 0x8329C730;
    'dispatch: loop {
        match pc {
            0x8329C730 => {
    //   block [0x8329C730..0x8329C7A4)
	// 8329C730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329C734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329C738: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329C73C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329C740: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8329C744: 390AA478  addi r8, r10, -0x5b88
	ctx.r[8].s64 = ctx.r[10].s64 + -23432;
	// 8329C748: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 8329C74C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329C750: 38AAAE8C  addi r5, r10, -0x5174
	ctx.r[5].s64 = ctx.r[10].s64 + -20852;
	// 8329C754: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329C758: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8329C75C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329C760: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329C764: 388AA558  addi r4, r10, -0x5aa8
	ctx.r[4].s64 = ctx.r[10].s64 + -23208;
	// 8329C768: 396BA520  addi r11, r11, -0x5ae0
	ctx.r[11].s64 = ctx.r[11].s64 + -23264;
	// 8329C76C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329C770: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329C774: 386A8A20  addi r3, r10, -0x75e0
	ctx.r[3].s64 = ctx.r[10].s64 + -30176;
	// 8329C778: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8329C77C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329C780: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8329C784: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329C788: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329C78C: 38C00044  li r6, 0x44
	ctx.r[6].s64 = 68;
	// 8329C790: 4BAB94B1  bl 0x82d55c40
	ctx.lr = 0x8329C794;
	sub_82D55C40(ctx, base);
	// 8329C794: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329C798: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329C79C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329C7A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329C7A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329C7A8 size=100
    let mut pc: u32 = 0x8329C7A8;
    'dispatch: loop {
        match pc {
            0x8329C7A8 => {
    //   block [0x8329C7A8..0x8329C80C)
	// 8329C7A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329C7AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329C7B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329C7B4: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 8329C7B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329C7BC: 38AAAE8C  addi r5, r10, -0x5174
	ctx.r[5].s64 = ctx.r[10].s64 + -20852;
	// 8329C7C0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329C7C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329C7C8: 388AA56C  addi r4, r10, -0x5a94
	ctx.r[4].s64 = ctx.r[10].s64 + -23188;
	// 8329C7CC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329C7D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329C7D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329C7D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329C7DC: 386A8A50  addi r3, r10, -0x75b0
	ctx.r[3].s64 = ctx.r[10].s64 + -30128;
	// 8329C7E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329C7E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329C7E8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8329C7EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329C7F0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8329C7F4: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8329C7F8: 4BAB9449  bl 0x82d55c40
	ctx.lr = 0x8329C7FC;
	sub_82D55C40(ctx, base);
	// 8329C7FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329C800: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329C804: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329C808: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


