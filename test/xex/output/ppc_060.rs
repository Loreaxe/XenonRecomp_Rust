pub fn sub_824F94E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F94E8 size=32
    let mut pc: u32 = 0x824F94E8;
    'dispatch: loop {
        match pc {
            0x824F94E8 => {
    //   block [0x824F94E8..0x824F9508)
	// 824F94E8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824F94EC: 48040174  b 0x82539660
	sub_825393C8(ctx, base);
	return;
	// 824F94F0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824F94F4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824F94F8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824F94FC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824F9500: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F9508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F9508 size=40
    let mut pc: u32 = 0x824F9508;
    'dispatch: loop {
        match pc {
            0x824F9508 => {
    //   block [0x824F9508..0x824F9530)
	// 824F9508: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824F950C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 824F9510: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824F9514: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 824F9518: 396B291C  addi r11, r11, 0x291c
	ctx.r[11].s64 = ctx.r[11].s64 + 10524;
	// 824F951C: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 824F9520: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 824F9524: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824F9528: 9123000C  stw r9, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 824F952C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F9530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F9530 size=12
    let mut pc: u32 = 0x824F9530;
    'dispatch: loop {
        match pc {
            0x824F9530 => {
    //   block [0x824F9530..0x824F953C)
	// 824F9530: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824F9534: 386B291C  addi r3, r11, 0x291c
	ctx.r[3].s64 = ctx.r[11].s64 + 10524;
	// 824F9538: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F9540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F9540 size=8
    let mut pc: u32 = 0x824F9540;
    'dispatch: loop {
        match pc {
            0x824F9540 => {
    //   block [0x824F9540..0x824F9548)
	// 824F9540: 38600040  li r3, 0x40
	ctx.r[3].s64 = 64;
	// 824F9544: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F9548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F9548 size=20
    let mut pc: u32 = 0x824F9548;
    'dispatch: loop {
        match pc {
            0x824F9548 => {
    //   block [0x824F9548..0x824F955C)
	// 824F9548: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824F954C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824F9550: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824F9554: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824F9558: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F9560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F9560 size=16
    let mut pc: u32 = 0x824F9560;
    'dispatch: loop {
        match pc {
            0x824F9560 => {
    //   block [0x824F9560..0x824F9570)
	// 824F9560: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824F9564: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824F9568: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 824F956C: 4800003C  b 0x824f95a8
	sub_824F95A8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F9570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F9570 size=4
    let mut pc: u32 = 0x824F9570;
    'dispatch: loop {
        match pc {
            0x824F9570 => {
    //   block [0x824F9570..0x824F9574)
	// 824F9570: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F9578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824F9578 size=44
    let mut pc: u32 = 0x824F9578;
    'dispatch: loop {
        match pc {
            0x824F9578 => {
    //   block [0x824F9578..0x824F95A4)
	// 824F9578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824F957C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824F9580: 9421FEA0  stwu r1, -0x160(r1)
	ea = ctx.r[1].u32.wrapping_add(-352 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824F9584: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824F9588: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824F958C: 4800001D  bl 0x824f95a8
	ctx.lr = 0x824F9590;
	sub_824F95A8(ctx, base);
	// 824F9590: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824F9594: 38210160  addi r1, r1, 0x160
	ctx.r[1].s64 = ctx.r[1].s64 + 352;
	// 824F9598: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824F959C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824F95A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F95A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824F95A8 size=116
    let mut pc: u32 = 0x824F95A8;
    'dispatch: loop {
        match pc {
            0x824F95A8 => {
    //   block [0x824F95A8..0x824F961C)
	// 824F95A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824F95AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824F95B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824F95B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824F95B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824F95BC: 48005205  bl 0x824fe7c0
	ctx.lr = 0x824F95C0;
	sub_824FE7C0(ctx, base);
	// 824F95C0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824F95C4: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 824F95C8: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 824F95CC: 3D008206  lis r8, -0x7dfa
	ctx.r[8].s64 = -2113536000;
	// 824F95D0: 3CE08206  lis r7, -0x7dfa
	ctx.r[7].s64 = -2113536000;
	// 824F95D4: 396B29E4  addi r11, r11, 0x29e4
	ctx.r[11].s64 = ctx.r[11].s64 + 10724;
	// 824F95D8: 394A29D8  addi r10, r10, 0x29d8
	ctx.r[10].s64 = ctx.r[10].s64 + 10712;
	// 824F95DC: 392929C4  addi r9, r9, 0x29c4
	ctx.r[9].s64 = ctx.r[9].s64 + 10692;
	// 824F95E0: 390829B8  addi r8, r8, 0x29b8
	ctx.r[8].s64 = ctx.r[8].s64 + 10680;
	// 824F95E4: 38E729AC  addi r7, r7, 0x29ac
	ctx.r[7].s64 = ctx.r[7].s64 + 10668;
	// 824F95E8: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 824F95EC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824F95F0: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 824F95F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824F95F8: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 824F95FC: 911F0010  stw r8, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 824F9600: 90FF0014  stw r7, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[7].u32 ) };
	// 824F9604: 90DF0020  stw r6, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[6].u32 ) };
	// 824F9608: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824F960C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824F9610: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824F9614: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824F9618: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F9620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824F9620 size=100
    let mut pc: u32 = 0x824F9620;
    'dispatch: loop {
        match pc {
            0x824F9620 => {
    //   block [0x824F9620..0x824F9668)
	// 824F9620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824F9624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824F9628: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824F962C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824F9630: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824F9634: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824F9638: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824F963C: 480041D5  bl 0x824fd810
	ctx.lr = 0x824F9640;
	sub_824FD810(ctx, base);
	// 824F9640: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 824F9644: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824F9648: 419A0020  beq cr6, 0x824f9668
	if ctx.cr[6].eq {
	pc = 0x824F9668; continue 'dispatch;
	}
	// 824F964C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824F9650: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 824F9654: 38C00027  li r6, 0x27
	ctx.r[6].s64 = 39;
	// 824F9658: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824F965C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824F9660: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824F9664: 4BF6AA55  bl 0x824640b8
	ctx.lr = 0x824F9668;
	sub_824640B8(ctx, base);
	pc = 0x824F9668; continue 'dispatch;
            }
            0x824F9668 => {
    //   block [0x824F9668..0x824F9684)
	// 824F9668: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824F966C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824F9670: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824F9674: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824F9678: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824F967C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824F9680: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F9688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F9688 size=8
    let mut pc: u32 = 0x824F9688;
    'dispatch: loop {
        match pc {
            0x824F9688 => {
    //   block [0x824F9688..0x824F9690)
	// 824F9688: 3863FFF8  addi r3, r3, -8
	ctx.r[3].s64 = ctx.r[3].s64 + -8;
	// 824F968C: 4BFFFF94  b 0x824f9620
	sub_824F9620(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F9690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F9690 size=8
    let mut pc: u32 = 0x824F9690;
    'dispatch: loop {
        match pc {
            0x824F9690 => {
    //   block [0x824F9690..0x824F9698)
	// 824F9690: 3863FFF4  addi r3, r3, -0xc
	ctx.r[3].s64 = ctx.r[3].s64 + -12;
	// 824F9694: 4BFFFF8C  b 0x824f9620
	sub_824F9620(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F9698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F9698 size=8
    let mut pc: u32 = 0x824F9698;
    'dispatch: loop {
        match pc {
            0x824F9698 => {
    //   block [0x824F9698..0x824F96A0)
	// 824F9698: 3863FFF0  addi r3, r3, -0x10
	ctx.r[3].s64 = ctx.r[3].s64 + -16;
	// 824F969C: 4BFFFF84  b 0x824f9620
	sub_824F9620(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F96A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F96A0 size=8
    let mut pc: u32 = 0x824F96A0;
    'dispatch: loop {
        match pc {
            0x824F96A0 => {
    //   block [0x824F96A0..0x824F96A8)
	// 824F96A0: 3863FFEC  addi r3, r3, -0x14
	ctx.r[3].s64 = ctx.r[3].s64 + -20;
	// 824F96A4: 4BFFFF7C  b 0x824f9620
	sub_824F9620(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F96A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824F96A8 size=72
    let mut pc: u32 = 0x824F96A8;
    'dispatch: loop {
        match pc {
            0x824F96A8 => {
    //   block [0x824F96A8..0x824F96DC)
	// 824F96A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824F96AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824F96B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824F96B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824F96B8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824F96BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824F96C0: 396B2A44  addi r11, r11, 0x2a44
	ctx.r[11].s64 = ctx.r[11].s64 + 10820;
	// 824F96C4: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 824F96C8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 824F96CC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824F96D0: 419A000C  beq cr6, 0x824f96dc
	if ctx.cr[6].eq {
	pc = 0x824F96DC; continue 'dispatch;
	}
	// 824F96D4: 480394E5  bl 0x82532bb8
	ctx.lr = 0x824F96D8;
	sub_82532BB8(ctx, base);
	// 824F96D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	pc = 0x824F96DC; continue 'dispatch;
            }
            0x824F96DC => {
    //   block [0x824F96DC..0x824F96F0)
	// 824F96DC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824F96E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824F96E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824F96E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824F96EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F96F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824F96F0 size=124
    let mut pc: u32 = 0x824F96F0;
    'dispatch: loop {
        match pc {
            0x824F96F0 => {
    //   block [0x824F96F0..0x824F974C)
	// 824F96F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824F96F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824F96F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824F96FC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824F9700: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824F9704: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824F9708: 396B2A68  addi r11, r11, 0x2a68
	ctx.r[11].s64 = ctx.r[11].s64 + 10856;
	// 824F970C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824F9710: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824F9714: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 824F9718: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824F971C: 419A0030  beq cr6, 0x824f974c
	if ctx.cr[6].eq {
	pc = 0x824F974C; continue 'dispatch;
	}
	// 824F9720: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 824F9724: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 824F9728: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 824F972C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824F9730: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 824F9734: 409A0018  bne cr6, 0x824f974c
	if !ctx.cr[6].eq {
	pc = 0x824F974C; continue 'dispatch;
	}
	// 824F9738: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824F973C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824F9740: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824F9744: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824F9748: 4E800421  bctrl
	ctx.lr = 0x824F974C;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x824F974C => {
    //   block [0x824F974C..0x824F976C)
	// 824F974C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824F9750: 396B2A44  addi r11, r11, 0x2a44
	ctx.r[11].s64 = ctx.r[11].s64 + 10820;
	// 824F9754: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824F9758: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824F975C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824F9760: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824F9764: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824F9768: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F9770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824F9770 size=48
    let mut pc: u32 = 0x824F9770;
    'dispatch: loop {
        match pc {
            0x824F9770 => {
    //   block [0x824F9770..0x824F97A0)
	// 824F9770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824F9774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824F9778: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824F977C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824F9780: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824F9784: 4BFFFF6D  bl 0x824f96f0
	ctx.lr = 0x824F9788;
	sub_824F96F0(ctx, base);
	// 824F9788: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824F978C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824F9790: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824F9794: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824F9798: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824F979C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F97A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F97A0 size=20
    let mut pc: u32 = 0x824F97A0;
    'dispatch: loop {
        match pc {
            0x824F97A0 => {
    //   block [0x824F97A0..0x824F97B4)
	// 824F97A0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824F97A4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824F97A8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824F97AC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824F97B0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F97B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F97B8 size=52
    let mut pc: u32 = 0x824F97B8;
    'dispatch: loop {
        match pc {
            0x824F97B8 => {
    //   block [0x824F97B8..0x824F97EC)
	// 824F97B8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824F97BC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 824F97C0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824F97C4: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 824F97C8: 396B2A8C  addi r11, r11, 0x2a8c
	ctx.r[11].s64 = ctx.r[11].s64 + 10892;
	// 824F97CC: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 824F97D0: 394A2A68  addi r10, r10, 0x2a68
	ctx.r[10].s64 = ctx.r[10].s64 + 10856;
	// 824F97D4: 3900001C  li r8, 0x1c
	ctx.r[8].s64 = 28;
	// 824F97D8: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824F97DC: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 824F97E0: 91430010  stw r10, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 824F97E4: 9103000C  stw r8, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 824F97E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F97F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F97F0 size=12
    let mut pc: u32 = 0x824F97F0;
    'dispatch: loop {
        match pc {
            0x824F97F0 => {
    //   block [0x824F97F0..0x824F97FC)
	// 824F97F0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824F97F4: 386B2A8C  addi r3, r11, 0x2a8c
	ctx.r[3].s64 = ctx.r[11].s64 + 10892;
	// 824F97F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F9800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F9800 size=20
    let mut pc: u32 = 0x824F9800;
    'dispatch: loop {
        match pc {
            0x824F9800 => {
    //   block [0x824F9800..0x824F9814)
	// 824F9800: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824F9804: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824F9808: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824F980C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824F9810: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F9818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F9818 size=16
    let mut pc: u32 = 0x824F9818;
    'dispatch: loop {
        match pc {
            0x824F9818 => {
    //   block [0x824F9818..0x824F9828)
	// 824F9818: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824F981C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824F9820: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 824F9824: 4800003C  b 0x824f9860
	sub_824F9860(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F9828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F9828 size=4
    let mut pc: u32 = 0x824F9828;
    'dispatch: loop {
        match pc {
            0x824F9828 => {
    //   block [0x824F9828..0x824F982C)
	// 824F9828: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F9830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824F9830 size=44
    let mut pc: u32 = 0x824F9830;
    'dispatch: loop {
        match pc {
            0x824F9830 => {
    //   block [0x824F9830..0x824F985C)
	// 824F9830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824F9834: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824F9838: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824F983C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824F9840: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824F9844: 4800001D  bl 0x824f9860
	ctx.lr = 0x824F9848;
	sub_824F9860(ctx, base);
	// 824F9848: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824F984C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 824F9850: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824F9854: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824F9858: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F9860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824F9860 size=116
    let mut pc: u32 = 0x824F9860;
    'dispatch: loop {
        match pc {
            0x824F9860 => {
    //   block [0x824F9860..0x824F98D4)
	// 824F9860: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824F9864: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824F9868: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824F986C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824F9870: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824F9874: 48004F4D  bl 0x824fe7c0
	ctx.lr = 0x824F9878;
	sub_824FE7C0(ctx, base);
	// 824F9878: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824F987C: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 824F9880: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 824F9884: 3D008206  lis r8, -0x7dfa
	ctx.r[8].s64 = -2113536000;
	// 824F9888: 3CE08206  lis r7, -0x7dfa
	ctx.r[7].s64 = -2113536000;
	// 824F988C: 396B2B04  addi r11, r11, 0x2b04
	ctx.r[11].s64 = ctx.r[11].s64 + 11012;
	// 824F9890: 394A2AF8  addi r10, r10, 0x2af8
	ctx.r[10].s64 = ctx.r[10].s64 + 11000;
	// 824F9894: 39292AE4  addi r9, r9, 0x2ae4
	ctx.r[9].s64 = ctx.r[9].s64 + 10980;
	// 824F9898: 39082AD8  addi r8, r8, 0x2ad8
	ctx.r[8].s64 = ctx.r[8].s64 + 10968;
	// 824F989C: 38E72ACC  addi r7, r7, 0x2acc
	ctx.r[7].s64 = ctx.r[7].s64 + 10956;
	// 824F98A0: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 824F98A4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824F98A8: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 824F98AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824F98B0: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 824F98B4: 911F0010  stw r8, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 824F98B8: 90FF0014  stw r7, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[7].u32 ) };
	// 824F98BC: 90DF0020  stw r6, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[6].u32 ) };
	// 824F98C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824F98C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824F98C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824F98CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824F98D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F98D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824F98D8 size=100
    let mut pc: u32 = 0x824F98D8;
    'dispatch: loop {
        match pc {
            0x824F98D8 => {
    //   block [0x824F98D8..0x824F9920)
	// 824F98D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824F98DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824F98E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824F98E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824F98E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824F98EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824F98F0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824F98F4: 4800A39D  bl 0x82503c90
	ctx.lr = 0x824F98F8;
	sub_82503C90(ctx, base);
	// 824F98F8: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 824F98FC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824F9900: 419A0020  beq cr6, 0x824f9920
	if ctx.cr[6].eq {
	pc = 0x824F9920; continue 'dispatch;
	}
	// 824F9904: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824F9908: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 824F990C: 38C00027  li r6, 0x27
	ctx.r[6].s64 = 39;
	// 824F9910: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824F9914: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824F9918: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824F991C: 4BF6A79D  bl 0x824640b8
	ctx.lr = 0x824F9920;
	sub_824640B8(ctx, base);
	pc = 0x824F9920; continue 'dispatch;
            }
            0x824F9920 => {
    //   block [0x824F9920..0x824F993C)
	// 824F9920: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824F9924: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824F9928: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824F992C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824F9930: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824F9934: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824F9938: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F9940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F9940 size=8
    let mut pc: u32 = 0x824F9940;
    'dispatch: loop {
        match pc {
            0x824F9940 => {
    //   block [0x824F9940..0x824F9948)
	// 824F9940: 3863FFF8  addi r3, r3, -8
	ctx.r[3].s64 = ctx.r[3].s64 + -8;
	// 824F9944: 4BFFFF94  b 0x824f98d8
	sub_824F98D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F9948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F9948 size=8
    let mut pc: u32 = 0x824F9948;
    'dispatch: loop {
        match pc {
            0x824F9948 => {
    //   block [0x824F9948..0x824F9950)
	// 824F9948: 3863FFF4  addi r3, r3, -0xc
	ctx.r[3].s64 = ctx.r[3].s64 + -12;
	// 824F994C: 4BFFFF8C  b 0x824f98d8
	sub_824F98D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F9950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F9950 size=8
    let mut pc: u32 = 0x824F9950;
    'dispatch: loop {
        match pc {
            0x824F9950 => {
    //   block [0x824F9950..0x824F9958)
	// 824F9950: 3863FFF0  addi r3, r3, -0x10
	ctx.r[3].s64 = ctx.r[3].s64 + -16;
	// 824F9954: 4BFFFF84  b 0x824f98d8
	sub_824F98D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F9958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F9958 size=8
    let mut pc: u32 = 0x824F9958;
    'dispatch: loop {
        match pc {
            0x824F9958 => {
    //   block [0x824F9958..0x824F9960)
	// 824F9958: 3863FFEC  addi r3, r3, -0x14
	ctx.r[3].s64 = ctx.r[3].s64 + -20;
	// 824F995C: 4BFFFF7C  b 0x824f98d8
	sub_824F98D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F9960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F9960 size=4
    let mut pc: u32 = 0x824F9960;
    'dispatch: loop {
        match pc {
            0x824F9960 => {
    //   block [0x824F9960..0x824F9964)
	// 824F9960: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F9968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F9968 size=4
    let mut pc: u32 = 0x824F9968;
    'dispatch: loop {
        match pc {
            0x824F9968 => {
    //   block [0x824F9968..0x824F996C)
	// 824F9968: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F9970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F9970 size=16
    let mut pc: u32 = 0x824F9970;
    'dispatch: loop {
        match pc {
            0x824F9970 => {
    //   block [0x824F9970..0x824F9980)
	// 824F9970: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824F9974: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824F9978: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 824F997C: 4800B664  b 0x82504fe0
	sub_82504FE0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F9980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F9980 size=4
    let mut pc: u32 = 0x824F9980;
    'dispatch: loop {
        match pc {
            0x824F9980 => {
    //   block [0x824F9980..0x824F9984)
	// 824F9980: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F9988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F9988 size=20
    let mut pc: u32 = 0x824F9988;
    'dispatch: loop {
        match pc {
            0x824F9988 => {
    //   block [0x824F9988..0x824F999C)
	// 824F9988: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824F998C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824F9990: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824F9994: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824F9998: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F99A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824F99A0 size=44
    let mut pc: u32 = 0x824F99A0;
    'dispatch: loop {
        match pc {
            0x824F99A0 => {
    //   block [0x824F99A0..0x824F99CC)
	// 824F99A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824F99A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824F99A8: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824F99AC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824F99B0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824F99B4: 4800B62D  bl 0x82504fe0
	ctx.lr = 0x824F99B8;
	sub_82504FE0(ctx, base);
	// 824F99B8: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824F99BC: 38210120  addi r1, r1, 0x120
	ctx.r[1].s64 = ctx.r[1].s64 + 288;
	// 824F99C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824F99C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824F99C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F99D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F99D0 size=4
    let mut pc: u32 = 0x824F99D0;
    'dispatch: loop {
        match pc {
            0x824F99D0 => {
    //   block [0x824F99D0..0x824F99D4)
	// 824F99D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F99D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F99D8 size=4
    let mut pc: u32 = 0x824F99D8;
    'dispatch: loop {
        match pc {
            0x824F99D8 => {
    //   block [0x824F99D8..0x824F99DC)
	// 824F99D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F99E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F99E0 size=4
    let mut pc: u32 = 0x824F99E0;
    'dispatch: loop {
        match pc {
            0x824F99E0 => {
    //   block [0x824F99E0..0x824F99E4)
	// 824F99E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F99E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F99E8 size=4
    let mut pc: u32 = 0x824F99E8;
    'dispatch: loop {
        match pc {
            0x824F99E8 => {
    //   block [0x824F99E8..0x824F99EC)
	// 824F99E8: 4800AF30  b 0x82504918
	sub_82504918(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F99F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F99F0 size=20
    let mut pc: u32 = 0x824F99F0;
    'dispatch: loop {
        match pc {
            0x824F99F0 => {
    //   block [0x824F99F0..0x824F9A04)
	// 824F99F0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824F99F4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824F99F8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824F99FC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824F9A00: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F9A08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F9A08 size=24
    let mut pc: u32 = 0x824F9A08;
    'dispatch: loop {
        match pc {
            0x824F9A08 => {
    //   block [0x824F9A08..0x824F9A20)
	// 824F9A08: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824F9A0C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 824F9A10: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824F9A14: 396B2A68  addi r11, r11, 0x2a68
	ctx.r[11].s64 = ctx.r[11].s64 + 10856;
	// 824F9A18: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824F9A1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F9A20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F9A20 size=12
    let mut pc: u32 = 0x824F9A20;
    'dispatch: loop {
        match pc {
            0x824F9A20 => {
    //   block [0x824F9A20..0x824F9A2C)
	// 824F9A20: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824F9A24: 386B2A68  addi r3, r11, 0x2a68
	ctx.r[3].s64 = ctx.r[11].s64 + 10856;
	// 824F9A28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F9A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F9A30 size=20
    let mut pc: u32 = 0x824F9A30;
    'dispatch: loop {
        match pc {
            0x824F9A30 => {
    //   block [0x824F9A30..0x824F9A44)
	// 824F9A30: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824F9A34: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824F9A38: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824F9A3C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824F9A40: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F9A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F9A48 size=88
    let mut pc: u32 = 0x824F9A48;
    'dispatch: loop {
        match pc {
            0x824F9A48 => {
    //   block [0x824F9A48..0x824F9AA0)
	// 824F9A48: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824F9A4C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 824F9A50: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824F9A54: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 824F9A58: 396B2D50  addi r11, r11, 0x2d50
	ctx.r[11].s64 = ctx.r[11].s64 + 11600;
	// 824F9A5C: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 824F9A60: 394A2A68  addi r10, r10, 0x2a68
	ctx.r[10].s64 = ctx.r[10].s64 + 10856;
	// 824F9A64: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 824F9A68: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 824F9A6C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824F9A70: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 824F9A74: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 824F9A78: 91430030  stw r10, 0x30(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[10].u32 ) };
	// 824F9A7C: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 824F9A80: 9103000C  stw r8, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F9AA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F9AA0 size=12
    let mut pc: u32 = 0x824F9AA0;
    'dispatch: loop {
        match pc {
            0x824F9AA0 => {
    //   block [0x824F9AA0..0x824F9AAC)
	// 824F9AA0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824F9AA4: 386B2D50  addi r3, r11, 0x2d50
	ctx.r[3].s64 = ctx.r[11].s64 + 11600;
	// 824F9AA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F9AB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824F9AB0 size=124
    let mut pc: u32 = 0x824F9AB0;
    'dispatch: loop {
        match pc {
            0x824F9AB0 => {
    //   block [0x824F9AB0..0x824F9B0C)
	// 824F9AB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824F9AB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824F9AB8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824F9ABC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824F9AC0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824F9AC4: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824F9AC8: 396B2D1C  addi r11, r11, 0x2d1c
	ctx.r[11].s64 = ctx.r[11].s64 + 11548;
	// 824F9ACC: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 824F9AD0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824F9AD4: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 824F9AD8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824F9ADC: 419A0030  beq cr6, 0x824f9b0c
	if ctx.cr[6].eq {
	pc = 0x824F9B0C; continue 'dispatch;
	}
	// 824F9AE0: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 824F9AE4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 824F9AE8: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 824F9AEC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824F9AF0: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 824F9AF4: 409A0018  bne cr6, 0x824f9b0c
	if !ctx.cr[6].eq {
	pc = 0x824F9B0C; continue 'dispatch;
	}
	// 824F9AF8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824F9AFC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824F9B00: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824F9B04: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824F9B08: 4E800421  bctrl
	ctx.lr = 0x824F9B0C;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x824F9B0C => {
    //   block [0x824F9B0C..0x824F9B2C)
	// 824F9B0C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824F9B10: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 824F9B14: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824F9B18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824F9B1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824F9B20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824F9B24: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824F9B28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F9B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824F9B30 size=100
    let mut pc: u32 = 0x824F9B30;
    'dispatch: loop {
        match pc {
            0x824F9B30 => {
    //   block [0x824F9B30..0x824F9B78)
	// 824F9B30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824F9B34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824F9B38: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824F9B3C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824F9B40: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824F9B44: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824F9B48: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824F9B4C: 4BFFFF65  bl 0x824f9ab0
	ctx.lr = 0x824F9B50;
	sub_824F9AB0(ctx, base);
	// 824F9B50: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 824F9B54: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824F9B58: 419A0020  beq cr6, 0x824f9b78
	if ctx.cr[6].eq {
	pc = 0x824F9B78; continue 'dispatch;
	}
	// 824F9B5C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824F9B60: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 824F9B64: 38C00027  li r6, 0x27
	ctx.r[6].s64 = 39;
	// 824F9B68: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824F9B6C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824F9B70: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824F9B74: 4BF6A545  bl 0x824640b8
	ctx.lr = 0x824F9B78;
	sub_824640B8(ctx, base);
	pc = 0x824F9B78; continue 'dispatch;
            }
            0x824F9B78 => {
    //   block [0x824F9B78..0x824F9B94)
	// 824F9B78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824F9B7C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824F9B80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824F9B84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824F9B88: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824F9B8C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824F9B90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F9B98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F9B98 size=20
    let mut pc: u32 = 0x824F9B98;
    'dispatch: loop {
        match pc {
            0x824F9B98 => {
    //   block [0x824F9B98..0x824F9BAC)
	// 824F9B98: 80630034  lwz r3, 0x34(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 824F9B9C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824F9BA0: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 824F9BA4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824F9BA8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F9BB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824F9BB0 size=100
    let mut pc: u32 = 0x824F9BB0;
    'dispatch: loop {
        match pc {
            0x824F9BB0 => {
    //   block [0x824F9BB0..0x824F9BF8)
	// 824F9BB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824F9BB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824F9BB8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824F9BBC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824F9BC0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824F9BC4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824F9BC8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824F9BCC: 4800C345  bl 0x82505f10
	ctx.lr = 0x824F9BD0;
	sub_82505F10(ctx, base);
	// 824F9BD0: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 824F9BD4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824F9BD8: 419A0020  beq cr6, 0x824f9bf8
	if ctx.cr[6].eq {
	pc = 0x824F9BF8; continue 'dispatch;
	}
	// 824F9BDC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824F9BE0: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 824F9BE4: 38C00027  li r6, 0x27
	ctx.r[6].s64 = 39;
	// 824F9BE8: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824F9BEC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824F9BF0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824F9BF4: 4BF6A4C5  bl 0x824640b8
	ctx.lr = 0x824F9BF8;
	sub_824640B8(ctx, base);
	pc = 0x824F9BF8; continue 'dispatch;
            }
            0x824F9BF8 => {
    //   block [0x824F9BF8..0x824F9C14)
	// 824F9BF8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824F9BFC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824F9C00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824F9C04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824F9C08: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824F9C0C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824F9C10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F9C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824F9C48 size=44
    let mut pc: u32 = 0x824F9C48;
    'dispatch: loop {
        match pc {
            0x824F9C48 => {
    //   block [0x824F9C48..0x824F9C74)
	// 824F9C48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824F9C4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824F9C50: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824F9C54: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824F9C58: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824F9C5C: 4800C91D  bl 0x82506578
	ctx.lr = 0x824F9C60;
	sub_82506578(ctx, base);
	// 824F9C60: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824F9C64: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 824F9C68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824F9C6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824F9C70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F9CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824F9CA8 size=44
    let mut pc: u32 = 0x824F9CA8;
    'dispatch: loop {
        match pc {
            0x824F9CA8 => {
    //   block [0x824F9CA8..0x824F9CD4)
	// 824F9CA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824F9CAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824F9CB0: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824F9CB4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824F9CB8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824F9CBC: 4800C8ED  bl 0x825065a8
	ctx.lr = 0x824F9CC0;
	sub_825065A8(ctx, base);
	// 824F9CC0: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824F9CC4: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 824F9CC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824F9CCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824F9CD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F9CD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F9CD8 size=4
    let mut pc: u32 = 0x824F9CD8;
    'dispatch: loop {
        match pc {
            0x824F9CD8 => {
    //   block [0x824F9CD8..0x824F9CDC)
	// 824F9CD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F9D10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824F9D10 size=44
    let mut pc: u32 = 0x824F9D10;
    'dispatch: loop {
        match pc {
            0x824F9D10 => {
    //   block [0x824F9D10..0x824F9D3C)
	// 824F9D10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824F9D14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824F9D18: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824F9D1C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824F9D20: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824F9D24: 4800D685  bl 0x825073a8
	ctx.lr = 0x824F9D28;
	sub_825073A8(ctx, base);
	// 824F9D28: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824F9D2C: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 824F9D30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824F9D34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824F9D38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F9D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F9D40 size=4
    let mut pc: u32 = 0x824F9D40;
    'dispatch: loop {
        match pc {
            0x824F9D40 => {
    //   block [0x824F9D40..0x824F9D44)
	// 824F9D40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F9D78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824F9D78 size=44
    let mut pc: u32 = 0x824F9D78;
    'dispatch: loop {
        match pc {
            0x824F9D78 => {
    //   block [0x824F9D78..0x824F9DA4)
	// 824F9D78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824F9D7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824F9D80: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824F9D84: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824F9D88: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824F9D8C: 4800EC35  bl 0x825089c0
	ctx.lr = 0x824F9D90;
	sub_825089C0(ctx, base);
	// 824F9D90: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824F9D94: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 824F9D98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824F9D9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824F9DA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F9DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824F9DD8 size=44
    let mut pc: u32 = 0x824F9DD8;
    'dispatch: loop {
        match pc {
            0x824F9DD8 => {
    //   block [0x824F9DD8..0x824F9E04)
	// 824F9DD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824F9DDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824F9DE0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824F9DE4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824F9DE8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824F9DEC: 4800E955  bl 0x82508740
	ctx.lr = 0x824F9DF0;
	sub_82508740(ctx, base);
	// 824F9DF0: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824F9DF4: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 824F9DF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824F9DFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824F9E00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F9E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824F9E38 size=44
    let mut pc: u32 = 0x824F9E38;
    'dispatch: loop {
        match pc {
            0x824F9E38 => {
    //   block [0x824F9E38..0x824F9E64)
	// 824F9E38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824F9E3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824F9E40: 9421FEC0  stwu r1, -0x140(r1)
	ea = ctx.r[1].u32.wrapping_add(-320 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824F9E44: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824F9E48: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824F9E4C: 4800EB8D  bl 0x825089d8
	ctx.lr = 0x824F9E50;
	sub_825089D8(ctx, base);
	// 824F9E50: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824F9E54: 38210140  addi r1, r1, 0x140
	ctx.r[1].s64 = ctx.r[1].s64 + 320;
	// 824F9E58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824F9E5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824F9E60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F9E68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F9E68 size=20
    let mut pc: u32 = 0x824F9E68;
    'dispatch: loop {
        match pc {
            0x824F9E68 => {
    //   block [0x824F9E68..0x824F9E7C)
	// 824F9E68: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824F9E6C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824F9E70: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824F9E74: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824F9E78: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F9E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F9E80 size=52
    let mut pc: u32 = 0x824F9E80;
    'dispatch: loop {
        match pc {
            0x824F9E80 => {
    //   block [0x824F9E80..0x824F9EB4)
	// 824F9E80: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824F9E84: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 824F9E88: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824F9E8C: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 824F9E90: 396B30EC  addi r11, r11, 0x30ec
	ctx.r[11].s64 = ctx.r[11].s64 + 12524;
	// 824F9E94: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 824F9E98: 394A2A68  addi r10, r10, 0x2a68
	ctx.r[10].s64 = ctx.r[10].s64 + 10856;
	// 824F9E9C: 3900001A  li r8, 0x1a
	ctx.r[8].s64 = 26;
	// 824F9EA0: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824F9EA4: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 824F9EA8: 91430014  stw r10, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 824F9EAC: 9103000C  stw r8, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 824F9EB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F9EB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F9EB8 size=12
    let mut pc: u32 = 0x824F9EB8;
    'dispatch: loop {
        match pc {
            0x824F9EB8 => {
    //   block [0x824F9EB8..0x824F9EC4)
	// 824F9EB8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824F9EBC: 386B30EC  addi r3, r11, 0x30ec
	ctx.r[3].s64 = ctx.r[11].s64 + 12524;
	// 824F9EC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F9EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824F9EC8 size=100
    let mut pc: u32 = 0x824F9EC8;
    'dispatch: loop {
        match pc {
            0x824F9EC8 => {
    //   block [0x824F9EC8..0x824F9F10)
	// 824F9EC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824F9ECC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824F9ED0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824F9ED4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824F9ED8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824F9EDC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824F9EE0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824F9EE4: 4800FED5  bl 0x82509db8
	ctx.lr = 0x824F9EE8;
	sub_82509DB8(ctx, base);
	// 824F9EE8: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 824F9EEC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824F9EF0: 419A0020  beq cr6, 0x824f9f10
	if ctx.cr[6].eq {
	pc = 0x824F9F10; continue 'dispatch;
	}
	// 824F9EF4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824F9EF8: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 824F9EFC: 38C00027  li r6, 0x27
	ctx.r[6].s64 = 39;
	// 824F9F00: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824F9F04: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824F9F08: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824F9F0C: 4BF6A1AD  bl 0x824640b8
	ctx.lr = 0x824F9F10;
	sub_824640B8(ctx, base);
	pc = 0x824F9F10; continue 'dispatch;
            }
            0x824F9F10 => {
    //   block [0x824F9F10..0x824F9F2C)
	// 824F9F10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824F9F14: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824F9F18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824F9F1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824F9F20: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824F9F24: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824F9F28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F9F30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F9F30 size=20
    let mut pc: u32 = 0x824F9F30;
    'dispatch: loop {
        match pc {
            0x824F9F30 => {
    //   block [0x824F9F30..0x824F9F44)
	// 824F9F30: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824F9F34: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824F9F38: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824F9F3C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824F9F40: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F9F48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F9F48 size=40
    let mut pc: u32 = 0x824F9F48;
    'dispatch: loop {
        match pc {
            0x824F9F48 => {
    //   block [0x824F9F48..0x824F9F70)
	// 824F9F48: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824F9F4C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 824F9F50: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824F9F54: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 824F9F58: 396B3134  addi r11, r11, 0x3134
	ctx.r[11].s64 = ctx.r[11].s64 + 12596;
	// 824F9F5C: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 824F9F60: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 824F9F64: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824F9F68: 9123000C  stw r9, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 824F9F6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F9F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F9F70 size=12
    let mut pc: u32 = 0x824F9F70;
    'dispatch: loop {
        match pc {
            0x824F9F70 => {
    //   block [0x824F9F70..0x824F9F7C)
	// 824F9F70: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824F9F74: 386B3134  addi r3, r11, 0x3134
	ctx.r[3].s64 = ctx.r[11].s64 + 12596;
	// 824F9F78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F9F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F9F80 size=8
    let mut pc: u32 = 0x824F9F80;
    'dispatch: loop {
        match pc {
            0x824F9F80 => {
    //   block [0x824F9F80..0x824F9F88)
	// 824F9F80: 38600060  li r3, 0x60
	ctx.r[3].s64 = 96;
	// 824F9F84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F9F88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824F9F88 size=96
    let mut pc: u32 = 0x824F9F88;
    'dispatch: loop {
        match pc {
            0x824F9F88 => {
    //   block [0x824F9F88..0x824F9FD0)
	// 824F9F88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824F9F8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824F9F90: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824F9F94: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824F9F98: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824F9F9C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824F9FA0: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 824F9FA4: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 824F9FA8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 824F9FAC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824F9FB0: 419A0020  beq cr6, 0x824f9fd0
	if ctx.cr[6].eq {
	pc = 0x824F9FD0; continue 'dispatch;
	}
	// 824F9FB4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824F9FB8: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 824F9FBC: 38C00027  li r6, 0x27
	ctx.r[6].s64 = 39;
	// 824F9FC0: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824F9FC4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824F9FC8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824F9FCC: 4BF6A0ED  bl 0x824640b8
	ctx.lr = 0x824F9FD0;
	sub_824640B8(ctx, base);
	pc = 0x824F9FD0; continue 'dispatch;
            }
            0x824F9FD0 => {
    //   block [0x824F9FD0..0x824F9FE8)
	// 824F9FD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824F9FD4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824F9FD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824F9FDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824F9FE0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824F9FE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F9FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F9FE8 size=20
    let mut pc: u32 = 0x824F9FE8;
    'dispatch: loop {
        match pc {
            0x824F9FE8 => {
    //   block [0x824F9FE8..0x824F9FFC)
	// 824F9FE8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824F9FEC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824F9FF0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824F9FF4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824F9FF8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FA000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FA000 size=40
    let mut pc: u32 = 0x824FA000;
    'dispatch: loop {
        match pc {
            0x824FA000 => {
    //   block [0x824FA000..0x824FA028)
	// 824FA000: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824FA004: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 824FA008: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824FA00C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 824FA010: 396B31C4  addi r11, r11, 0x31c4
	ctx.r[11].s64 = ctx.r[11].s64 + 12740;
	// 824FA014: 3920001B  li r9, 0x1b
	ctx.r[9].s64 = 27;
	// 824FA018: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 824FA01C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824FA020: 9123000C  stw r9, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 824FA024: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FA028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FA028 size=12
    let mut pc: u32 = 0x824FA028;
    'dispatch: loop {
        match pc {
            0x824FA028 => {
    //   block [0x824FA028..0x824FA034)
	// 824FA028: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824FA02C: 386B31C4  addi r3, r11, 0x31c4
	ctx.r[3].s64 = ctx.r[11].s64 + 12740;
	// 824FA030: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FA038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FA038 size=20
    let mut pc: u32 = 0x824FA038;
    'dispatch: loop {
        match pc {
            0x824FA038 => {
    //   block [0x824FA038..0x824FA04C)
	// 824FA038: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FA03C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824FA040: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FA044: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824FA048: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FA050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FA050 size=32
    let mut pc: u32 = 0x824FA050;
    'dispatch: loop {
        match pc {
            0x824FA050 => {
    //   block [0x824FA050..0x824FA070)
	// 824FA050: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824FA054: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 824FA058: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824FA05C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 824FA060: 396BFBB4  addi r11, r11, -0x44c
	ctx.r[11].s64 = ctx.r[11].s64 + -1100;
	// 824FA064: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 824FA068: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824FA06C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FA070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FA070 size=12
    let mut pc: u32 = 0x824FA070;
    'dispatch: loop {
        match pc {
            0x824FA070 => {
    //   block [0x824FA070..0x824FA07C)
	// 824FA070: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824FA074: 386BFBB4  addi r3, r11, -0x44c
	ctx.r[3].s64 = ctx.r[11].s64 + -1100;
	// 824FA078: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FA080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FA080 size=20
    let mut pc: u32 = 0x824FA080;
    'dispatch: loop {
        match pc {
            0x824FA080 => {
    //   block [0x824FA080..0x824FA094)
	// 824FA080: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FA084: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824FA088: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FA08C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824FA090: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FA098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FA098 size=64
    let mut pc: u32 = 0x824FA098;
    'dispatch: loop {
        match pc {
            0x824FA098 => {
    //   block [0x824FA098..0x824FA0D8)
	// 824FA098: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824FA09C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 824FA0A0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824FA0A4: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 824FA0A8: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 824FA0AC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 824FA0B0: 396B2A44  addi r11, r11, 0x2a44
	ctx.r[11].s64 = ctx.r[11].s64 + 10820;
	// 824FA0B4: 394A32AC  addi r10, r10, 0x32ac
	ctx.r[10].s64 = ctx.r[10].s64 + 12972;
	// 824FA0B8: 39293288  addi r9, r9, 0x3288
	ctx.r[9].s64 = ctx.r[9].s64 + 12936;
	// 824FA0BC: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 824FA0C0: B1030006  sth r8, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 824FA0C4: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 824FA0C8: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824FA0CC: 91230010  stw r9, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 824FA0D0: 90E3000C  stw r7, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 824FA0D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FA0D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FA0D8 size=12
    let mut pc: u32 = 0x824FA0D8;
    'dispatch: loop {
        match pc {
            0x824FA0D8 => {
    //   block [0x824FA0D8..0x824FA0E4)
	// 824FA0D8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824FA0DC: 386B32AC  addi r3, r11, 0x32ac
	ctx.r[3].s64 = ctx.r[11].s64 + 12972;
	// 824FA0E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FA0E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FA0E8 size=20
    let mut pc: u32 = 0x824FA0E8;
    'dispatch: loop {
        match pc {
            0x824FA0E8 => {
    //   block [0x824FA0E8..0x824FA0FC)
	// 824FA0E8: 89630004  lbz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FA0EC: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 824FA0F0: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 824FA0F4: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 824FA0F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FA100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FA100 size=8
    let mut pc: u32 = 0x824FA100;
    'dispatch: loop {
        match pc {
            0x824FA100 => {
    //   block [0x824FA100..0x824FA108)
	// 824FA100: 3863FFF0  addi r3, r3, -0x10
	ctx.r[3].s64 = ctx.r[3].s64 + -16;
	// 824FA104: 48000004  b 0x824fa108
	sub_824FA108(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FA108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824FA108 size=124
    let mut pc: u32 = 0x824FA108;
    'dispatch: loop {
        match pc {
            0x824FA108 => {
    //   block [0x824FA108..0x824FA12C)
	// 824FA108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FA10C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824FA110: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824FA114: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FA118: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824FA11C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 824FA120: 393F0010  addi r9, r31, 0x10
	ctx.r[9].s64 = ctx.r[31].s64 + 16;
	// 824FA124: 409A0008  bne cr6, 0x824fa12c
	if !ctx.cr[6].eq {
	pc = 0x824FA12C; continue 'dispatch;
	}
	// 824FA128: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	pc = 0x824FA12C; continue 'dispatch;
            }
            0x824FA12C => {
    //   block [0x824FA12C..0x824FA16C)
	// 824FA12C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824FA130: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 824FA134: 396B2A44  addi r11, r11, 0x2a44
	ctx.r[11].s64 = ctx.r[11].s64 + 10820;
	// 824FA138: 394A6DD0  addi r10, r10, 0x6dd0
	ctx.r[10].s64 = ctx.r[10].s64 + 28112;
	// 824FA13C: 548807FE  clrlwi r8, r4, 0x1f
	ctx.r[8].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 824FA140: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 824FA144: 91690000  stw r11, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824FA148: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824FA14C: 419A0020  beq cr6, 0x824fa16c
	if ctx.cr[6].eq {
	pc = 0x824FA16C; continue 'dispatch;
	}
	// 824FA150: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FA154: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 824FA158: 38C00027  li r6, 0x27
	ctx.r[6].s64 = 39;
	// 824FA15C: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FA160: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824FA164: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824FA168: 4BF69F51  bl 0x824640b8
	ctx.lr = 0x824FA16C;
	sub_824640B8(ctx, base);
	pc = 0x824FA16C; continue 'dispatch;
            }
            0x824FA16C => {
    //   block [0x824FA16C..0x824FA184)
	// 824FA16C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FA170: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824FA174: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824FA178: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824FA17C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824FA180: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FA188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FA188 size=8
    let mut pc: u32 = 0x824FA188;
    'dispatch: loop {
        match pc {
            0x824FA188 => {
    //   block [0x824FA188..0x824FA190)
	// 824FA188: 3863FFF0  addi r3, r3, -0x10
	ctx.r[3].s64 = ctx.r[3].s64 + -16;
	// 824FA18C: 48000004  b 0x824fa190
	sub_824FA190(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FA190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824FA190 size=100
    let mut pc: u32 = 0x824FA190;
    'dispatch: loop {
        match pc {
            0x824FA190 => {
    //   block [0x824FA190..0x824FA1D8)
	// 824FA190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FA194: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824FA198: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824FA19C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824FA1A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FA1A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824FA1A8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824FA1AC: 48011985  bl 0x8250bb30
	ctx.lr = 0x824FA1B0;
	sub_8250BB30(ctx, base);
	// 824FA1B0: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 824FA1B4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824FA1B8: 419A0020  beq cr6, 0x824fa1d8
	if ctx.cr[6].eq {
	pc = 0x824FA1D8; continue 'dispatch;
	}
	// 824FA1BC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FA1C0: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 824FA1C4: 38C00027  li r6, 0x27
	ctx.r[6].s64 = 39;
	// 824FA1C8: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FA1CC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824FA1D0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824FA1D4: 4BF69EE5  bl 0x824640b8
	ctx.lr = 0x824FA1D8;
	sub_824640B8(ctx, base);
	pc = 0x824FA1D8; continue 'dispatch;
            }
            0x824FA1D8 => {
    //   block [0x824FA1D8..0x824FA1F4)
	// 824FA1D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FA1DC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824FA1E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824FA1E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824FA1E8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824FA1EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824FA1F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FA1F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FA1F8 size=20
    let mut pc: u32 = 0x824FA1F8;
    'dispatch: loop {
        match pc {
            0x824FA1F8 => {
    //   block [0x824FA1F8..0x824FA20C)
	// 824FA1F8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FA1FC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824FA200: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FA204: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824FA208: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FA210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FA210 size=64
    let mut pc: u32 = 0x824FA210;
    'dispatch: loop {
        match pc {
            0x824FA210 => {
    //   block [0x824FA210..0x824FA250)
	// 824FA210: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824FA214: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 824FA218: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824FA21C: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 824FA220: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 824FA224: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 824FA228: 396B2A44  addi r11, r11, 0x2a44
	ctx.r[11].s64 = ctx.r[11].s64 + 10820;
	// 824FA22C: 394A3340  addi r10, r10, 0x3340
	ctx.r[10].s64 = ctx.r[10].s64 + 13120;
	// 824FA230: 3929331C  addi r9, r9, 0x331c
	ctx.r[9].s64 = ctx.r[9].s64 + 13084;
	// 824FA234: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 824FA238: B1030006  sth r8, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 824FA23C: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 824FA240: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824FA244: 91230010  stw r9, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 824FA248: 90E3000C  stw r7, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 824FA24C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FA250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FA250 size=12
    let mut pc: u32 = 0x824FA250;
    'dispatch: loop {
        match pc {
            0x824FA250 => {
    //   block [0x824FA250..0x824FA25C)
	// 824FA250: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824FA254: 386B3340  addi r3, r11, 0x3340
	ctx.r[3].s64 = ctx.r[11].s64 + 13120;
	// 824FA258: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FA260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824FA260 size=100
    let mut pc: u32 = 0x824FA260;
    'dispatch: loop {
        match pc {
            0x824FA260 => {
    //   block [0x824FA260..0x824FA2A8)
	// 824FA260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FA264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824FA268: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824FA26C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824FA270: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FA274: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824FA278: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824FA27C: 48011CD5  bl 0x8250bf50
	ctx.lr = 0x824FA280;
	sub_8250BF50(ctx, base);
	// 824FA280: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 824FA284: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824FA288: 419A0020  beq cr6, 0x824fa2a8
	if ctx.cr[6].eq {
	pc = 0x824FA2A8; continue 'dispatch;
	}
	// 824FA28C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FA290: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 824FA294: 38C00027  li r6, 0x27
	ctx.r[6].s64 = 39;
	// 824FA298: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FA29C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824FA2A0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824FA2A4: 4BF69E15  bl 0x824640b8
	ctx.lr = 0x824FA2A8;
	sub_824640B8(ctx, base);
	pc = 0x824FA2A8; continue 'dispatch;
            }
            0x824FA2A8 => {
    //   block [0x824FA2A8..0x824FA2C4)
	// 824FA2A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FA2AC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824FA2B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824FA2B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824FA2B8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824FA2BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824FA2C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FA2C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FA2C8 size=8
    let mut pc: u32 = 0x824FA2C8;
    'dispatch: loop {
        match pc {
            0x824FA2C8 => {
    //   block [0x824FA2C8..0x824FA2D0)
	// 824FA2C8: 3863FFF0  addi r3, r3, -0x10
	ctx.r[3].s64 = ctx.r[3].s64 + -16;
	// 824FA2CC: 4BFFFF94  b 0x824fa260
	sub_824FA260(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FA2D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FA2D0 size=4
    let mut pc: u32 = 0x824FA2D0;
    'dispatch: loop {
        match pc {
            0x824FA2D0 => {
    //   block [0x824FA2D0..0x824FA2D4)
	// 824FA2D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FA2D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FA2D8 size=4
    let mut pc: u32 = 0x824FA2D8;
    'dispatch: loop {
        match pc {
            0x824FA2D8 => {
    //   block [0x824FA2D8..0x824FA2DC)
	// 824FA2D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FA2E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FA2E0 size=4
    let mut pc: u32 = 0x824FA2E0;
    'dispatch: loop {
        match pc {
            0x824FA2E0 => {
    //   block [0x824FA2E0..0x824FA2E4)
	// 824FA2E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FA2E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FA2E8 size=4
    let mut pc: u32 = 0x824FA2E8;
    'dispatch: loop {
        match pc {
            0x824FA2E8 => {
    //   block [0x824FA2E8..0x824FA2EC)
	// 824FA2E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FA2F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FA2F0 size=20
    let mut pc: u32 = 0x824FA2F0;
    'dispatch: loop {
        match pc {
            0x824FA2F0 => {
    //   block [0x824FA2F0..0x824FA304)
	// 824FA2F0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FA2F4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824FA2F8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FA2FC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824FA300: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FA308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FA308 size=40
    let mut pc: u32 = 0x824FA308;
    'dispatch: loop {
        match pc {
            0x824FA308 => {
    //   block [0x824FA308..0x824FA330)
	// 824FA308: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824FA30C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 824FA310: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824FA314: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 824FA318: 396B35A8  addi r11, r11, 0x35a8
	ctx.r[11].s64 = ctx.r[11].s64 + 13736;
	// 824FA31C: 39200017  li r9, 0x17
	ctx.r[9].s64 = 23;
	// 824FA320: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 824FA324: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824FA328: 9123000C  stw r9, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 824FA32C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FA330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FA330 size=12
    let mut pc: u32 = 0x824FA330;
    'dispatch: loop {
        match pc {
            0x824FA330 => {
    //   block [0x824FA330..0x824FA33C)
	// 824FA330: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824FA334: 386B35A8  addi r3, r11, 0x35a8
	ctx.r[3].s64 = ctx.r[11].s64 + 13736;
	// 824FA338: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FA340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824FA340 size=152
    let mut pc: u32 = 0x824FA340;
    'dispatch: loop {
        match pc {
            0x824FA340 => {
    //   block [0x824FA340..0x824FA388)
	// 824FA340: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FA344: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824FA348: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824FA34C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824FA350: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FA354: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824FA358: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824FA35C: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 824FA360: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824FA364: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824FA368: 409A0020  bne cr6, 0x824fa388
	if !ctx.cr[6].eq {
	pc = 0x824FA388; continue 'dispatch;
	}
	// 824FA36C: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FA370: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 824FA374: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824FA378: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 824FA37C: 55652834  slwi r5, r11, 5
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824FA380: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824FA384: 4BF69D35  bl 0x824640b8
	ctx.lr = 0x824FA388;
	sub_824640B8(ctx, base);
	pc = 0x824FA388; continue 'dispatch;
            }
            0x824FA388 => {
    //   block [0x824FA388..0x824FA3BC)
	// 824FA388: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824FA38C: 57CA07FE  clrlwi r10, r30, 0x1f
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 824FA390: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 824FA394: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 824FA398: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824FA39C: 419A0020  beq cr6, 0x824fa3bc
	if ctx.cr[6].eq {
	pc = 0x824FA3BC; continue 'dispatch;
	}
	// 824FA3A0: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FA3A4: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 824FA3A8: 38C00027  li r6, 0x27
	ctx.r[6].s64 = 39;
	// 824FA3AC: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FA3B0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824FA3B4: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824FA3B8: 4BF69D01  bl 0x824640b8
	ctx.lr = 0x824FA3BC;
	sub_824640B8(ctx, base);
	pc = 0x824FA3BC; continue 'dispatch;
            }
            0x824FA3BC => {
    //   block [0x824FA3BC..0x824FA3D8)
	// 824FA3BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FA3C0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824FA3C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824FA3C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824FA3CC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824FA3D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824FA3D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FA3D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FA3D8 size=4
    let mut pc: u32 = 0x824FA3D8;
    'dispatch: loop {
        match pc {
            0x824FA3D8 => {
    //   block [0x824FA3D8..0x824FA3DC)
	// 824FA3D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FA3E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FA3E0 size=20
    let mut pc: u32 = 0x824FA3E0;
    'dispatch: loop {
        match pc {
            0x824FA3E0 => {
    //   block [0x824FA3E0..0x824FA3F4)
	// 824FA3E0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FA3E4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824FA3E8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FA3EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824FA3F0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FA3F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FA3F8 size=64
    let mut pc: u32 = 0x824FA3F8;
    'dispatch: loop {
        match pc {
            0x824FA3F8 => {
    //   block [0x824FA3F8..0x824FA438)
	// 824FA3F8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824FA3FC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 824FA400: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824FA404: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 824FA408: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 824FA40C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 824FA410: 396B2A44  addi r11, r11, 0x2a44
	ctx.r[11].s64 = ctx.r[11].s64 + 10820;
	// 824FA414: 394A3670  addi r10, r10, 0x3670
	ctx.r[10].s64 = ctx.r[10].s64 + 13936;
	// 824FA418: 3929364C  addi r9, r9, 0x364c
	ctx.r[9].s64 = ctx.r[9].s64 + 13900;
	// 824FA41C: 38E00016  li r7, 0x16
	ctx.r[7].s64 = 22;
	// 824FA420: B1030006  sth r8, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 824FA424: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 824FA428: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824FA42C: 91230010  stw r9, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 824FA430: 90E3000C  stw r7, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 824FA434: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FA438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FA438 size=12
    let mut pc: u32 = 0x824FA438;
    'dispatch: loop {
        match pc {
            0x824FA438 => {
    //   block [0x824FA438..0x824FA444)
	// 824FA438: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824FA43C: 386B3670  addi r3, r11, 0x3670
	ctx.r[3].s64 = ctx.r[11].s64 + 13936;
	// 824FA440: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FA448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FA448 size=8
    let mut pc: u32 = 0x824FA448;
    'dispatch: loop {
        match pc {
            0x824FA448 => {
    //   block [0x824FA448..0x824FA450)
	// 824FA448: 3863FFF0  addi r3, r3, -0x10
	ctx.r[3].s64 = ctx.r[3].s64 + -16;
	// 824FA44C: 480000DC  b 0x824fa528
	sub_824FA528(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FA450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824FA450 size=212
    let mut pc: u32 = 0x824FA450;
    'dispatch: loop {
        match pc {
            0x824FA450 => {
    //   block [0x824FA450..0x824FA490)
	// 824FA450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FA454: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824FA458: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824FA45C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FA460: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824FA464: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 824FA468: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824FA46C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824FA470: 409A0020  bne cr6, 0x824fa490
	if !ctx.cr[6].eq {
	pc = 0x824FA490; continue 'dispatch;
	}
	// 824FA474: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FA478: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 824FA47C: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824FA480: 809F0030  lwz r4, 0x30(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 824FA484: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 824FA488: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824FA48C: 4BF69C2D  bl 0x824640b8
	ctx.lr = 0x824FA490;
	sub_824640B8(ctx, base);
	pc = 0x824FA490; continue 'dispatch;
            }
            0x824FA490 => {
    //   block [0x824FA490..0x824FA4BC)
	// 824FA490: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 824FA494: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824FA498: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824FA49C: 409A0020  bne cr6, 0x824fa4bc
	if !ctx.cr[6].eq {
	pc = 0x824FA4BC; continue 'dispatch;
	}
	// 824FA4A0: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FA4A4: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 824FA4A8: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824FA4AC: 809F0024  lwz r4, 0x24(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 824FA4B0: 55652036  slwi r5, r11, 4
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824FA4B4: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824FA4B8: 4BF69C01  bl 0x824640b8
	ctx.lr = 0x824FA4BC;
	sub_824640B8(ctx, base);
	pc = 0x824FA4BC; continue 'dispatch;
            }
            0x824FA4BC => {
    //   block [0x824FA4BC..0x824FA4E8)
	// 824FA4BC: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 824FA4C0: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824FA4C4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824FA4C8: 409A0020  bne cr6, 0x824fa4e8
	if !ctx.cr[6].eq {
	pc = 0x824FA4E8; continue 'dispatch;
	}
	// 824FA4CC: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FA4D0: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 824FA4D4: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824FA4D8: 809F0018  lwz r4, 0x18(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 824FA4DC: 55652036  slwi r5, r11, 4
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824FA4E0: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824FA4E4: 4BF69BD5  bl 0x824640b8
	ctx.lr = 0x824FA4E8;
	sub_824640B8(ctx, base);
	pc = 0x824FA4E8; continue 'dispatch;
            }
            0x824FA4E8 => {
    //   block [0x824FA4E8..0x824FA4F8)
	// 824FA4E8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 824FA4EC: 393F0010  addi r9, r31, 0x10
	ctx.r[9].s64 = ctx.r[31].s64 + 16;
	// 824FA4F0: 409A0008  bne cr6, 0x824fa4f8
	if !ctx.cr[6].eq {
	pc = 0x824FA4F8; continue 'dispatch;
	}
	// 824FA4F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	pc = 0x824FA4F8; continue 'dispatch;
            }
            0x824FA4F8 => {
    //   block [0x824FA4F8..0x824FA524)
	// 824FA4F8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824FA4FC: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 824FA500: 396B2A44  addi r11, r11, 0x2a44
	ctx.r[11].s64 = ctx.r[11].s64 + 10820;
	// 824FA504: 394A6DD0  addi r10, r10, 0x6dd0
	ctx.r[10].s64 = ctx.r[10].s64 + 28112;
	// 824FA508: 91690000  stw r11, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824FA50C: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824FA510: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824FA514: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824FA518: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824FA51C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824FA520: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FA528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824FA528 size=100
    let mut pc: u32 = 0x824FA528;
    'dispatch: loop {
        match pc {
            0x824FA528 => {
    //   block [0x824FA528..0x824FA570)
	// 824FA528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FA52C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824FA530: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824FA534: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824FA538: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FA53C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824FA540: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824FA544: 4BFFFF0D  bl 0x824fa450
	ctx.lr = 0x824FA548;
	sub_824FA450(ctx, base);
	// 824FA548: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 824FA54C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824FA550: 419A0020  beq cr6, 0x824fa570
	if ctx.cr[6].eq {
	pc = 0x824FA570; continue 'dispatch;
	}
	// 824FA554: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FA558: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 824FA55C: 38C00027  li r6, 0x27
	ctx.r[6].s64 = 39;
	// 824FA560: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FA564: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824FA568: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824FA56C: 4BF69B4D  bl 0x824640b8
	ctx.lr = 0x824FA570;
	sub_824640B8(ctx, base);
	pc = 0x824FA570; continue 'dispatch;
            }
            0x824FA570 => {
    //   block [0x824FA570..0x824FA58C)
	// 824FA570: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FA574: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824FA578: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824FA57C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824FA580: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824FA584: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824FA588: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FA590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FA590 size=20
    let mut pc: u32 = 0x824FA590;
    'dispatch: loop {
        match pc {
            0x824FA590 => {
    //   block [0x824FA590..0x824FA5A4)
	// 824FA590: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FA594: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824FA598: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FA59C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824FA5A0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FA5A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FA5A8 size=52
    let mut pc: u32 = 0x824FA5A8;
    'dispatch: loop {
        match pc {
            0x824FA5A8 => {
    //   block [0x824FA5A8..0x824FA5DC)
	// 824FA5A8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824FA5AC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 824FA5B0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824FA5B4: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 824FA5B8: 396B36E4  addi r11, r11, 0x36e4
	ctx.r[11].s64 = ctx.r[11].s64 + 14052;
	// 824FA5BC: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 824FA5C0: 394A2A68  addi r10, r10, 0x2a68
	ctx.r[10].s64 = ctx.r[10].s64 + 10856;
	// 824FA5C4: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 824FA5C8: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824FA5CC: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 824FA5D0: 91430014  stw r10, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 824FA5D4: 9103000C  stw r8, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 824FA5D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FA5E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FA5E0 size=12
    let mut pc: u32 = 0x824FA5E0;
    'dispatch: loop {
        match pc {
            0x824FA5E0 => {
    //   block [0x824FA5E0..0x824FA5EC)
	// 824FA5E0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824FA5E4: 386B36E4  addi r3, r11, 0x36e4
	ctx.r[3].s64 = ctx.r[11].s64 + 14052;
	// 824FA5E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FA5F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FA5F0 size=4
    let mut pc: u32 = 0x824FA5F0;
    'dispatch: loop {
        match pc {
            0x824FA5F0 => {
    //   block [0x824FA5F0..0x824FA5F4)
	// 824FA5F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FA5F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FA5F8 size=20
    let mut pc: u32 = 0x824FA5F8;
    'dispatch: loop {
        match pc {
            0x824FA5F8 => {
    //   block [0x824FA5F8..0x824FA60C)
	// 824FA5F8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FA5FC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824FA600: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FA604: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824FA608: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FA610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FA610 size=40
    let mut pc: u32 = 0x824FA610;
    'dispatch: loop {
        match pc {
            0x824FA610 => {
    //   block [0x824FA610..0x824FA638)
	// 824FA610: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824FA614: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 824FA618: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824FA61C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 824FA620: 396B3804  addi r11, r11, 0x3804
	ctx.r[11].s64 = ctx.r[11].s64 + 14340;
	// 824FA624: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 824FA628: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 824FA62C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824FA630: 9123000C  stw r9, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 824FA634: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FA638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FA638 size=12
    let mut pc: u32 = 0x824FA638;
    'dispatch: loop {
        match pc {
            0x824FA638 => {
    //   block [0x824FA638..0x824FA644)
	// 824FA638: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824FA63C: 386B3804  addi r3, r11, 0x3804
	ctx.r[3].s64 = ctx.r[11].s64 + 14340;
	// 824FA640: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FA648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824FA648 size=100
    let mut pc: u32 = 0x824FA648;
    'dispatch: loop {
        match pc {
            0x824FA648 => {
    //   block [0x824FA648..0x824FA690)
	// 824FA648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FA64C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824FA650: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824FA654: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824FA658: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FA65C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824FA660: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824FA664: 4800187D  bl 0x824fbee0
	ctx.lr = 0x824FA668;
	sub_824FBEE0(ctx, base);
	// 824FA668: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 824FA66C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824FA670: 419A0020  beq cr6, 0x824fa690
	if ctx.cr[6].eq {
	pc = 0x824FA690; continue 'dispatch;
	}
	// 824FA674: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FA678: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 824FA67C: 38C00027  li r6, 0x27
	ctx.r[6].s64 = 39;
	// 824FA680: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FA684: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824FA688: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824FA68C: 4BF69A2D  bl 0x824640b8
	ctx.lr = 0x824FA690;
	sub_824640B8(ctx, base);
	pc = 0x824FA690; continue 'dispatch;
            }
            0x824FA690 => {
    //   block [0x824FA690..0x824FA6AC)
	// 824FA690: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FA694: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824FA698: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824FA69C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824FA6A0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824FA6A4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824FA6A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FA6B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FA6B0 size=20
    let mut pc: u32 = 0x824FA6B0;
    'dispatch: loop {
        match pc {
            0x824FA6B0 => {
    //   block [0x824FA6B0..0x824FA6C4)
	// 824FA6B0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FA6B4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824FA6B8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FA6BC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824FA6C0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FA6C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FA6C8 size=64
    let mut pc: u32 = 0x824FA6C8;
    'dispatch: loop {
        match pc {
            0x824FA6C8 => {
    //   block [0x824FA6C8..0x824FA708)
	// 824FA6C8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824FA6CC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 824FA6D0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824FA6D4: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 824FA6D8: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 824FA6DC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 824FA6E0: 396B2A44  addi r11, r11, 0x2a44
	ctx.r[11].s64 = ctx.r[11].s64 + 10820;
	// 824FA6E4: 394A38EC  addi r10, r10, 0x38ec
	ctx.r[10].s64 = ctx.r[10].s64 + 14572;
	// 824FA6E8: 392938C4  addi r9, r9, 0x38c4
	ctx.r[9].s64 = ctx.r[9].s64 + 14532;
	// 824FA6EC: 38E00015  li r7, 0x15
	ctx.r[7].s64 = 21;
	// 824FA6F0: B1030006  sth r8, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 824FA6F4: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 824FA6F8: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824FA6FC: 91230014  stw r9, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 824FA700: 90E3000C  stw r7, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 824FA704: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FA708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FA708 size=12
    let mut pc: u32 = 0x824FA708;
    'dispatch: loop {
        match pc {
            0x824FA708 => {
    //   block [0x824FA708..0x824FA714)
	// 824FA708: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824FA70C: 386B38EC  addi r3, r11, 0x38ec
	ctx.r[3].s64 = ctx.r[11].s64 + 14572;
	// 824FA710: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FA718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FA718 size=8
    let mut pc: u32 = 0x824FA718;
    'dispatch: loop {
        match pc {
            0x824FA718 => {
    //   block [0x824FA718..0x824FA720)
	// 824FA718: 3863FFEC  addi r3, r3, -0x14
	ctx.r[3].s64 = ctx.r[3].s64 + -20;
	// 824FA71C: 48000004  b 0x824fa720
	sub_824FA720(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FA720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824FA720 size=100
    let mut pc: u32 = 0x824FA720;
    'dispatch: loop {
        match pc {
            0x824FA720 => {
    //   block [0x824FA720..0x824FA768)
	// 824FA720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FA724: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824FA728: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824FA72C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824FA730: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FA734: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824FA738: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824FA73C: 480132CD  bl 0x8250da08
	ctx.lr = 0x824FA740;
	sub_8250DA08(ctx, base);
	// 824FA740: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 824FA744: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824FA748: 419A0020  beq cr6, 0x824fa768
	if ctx.cr[6].eq {
	pc = 0x824FA768; continue 'dispatch;
	}
	// 824FA74C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FA750: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 824FA754: 38C00027  li r6, 0x27
	ctx.r[6].s64 = 39;
	// 824FA758: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FA75C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824FA760: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824FA764: 4BF69955  bl 0x824640b8
	ctx.lr = 0x824FA768;
	sub_824640B8(ctx, base);
	pc = 0x824FA768; continue 'dispatch;
            }
            0x824FA768 => {
    //   block [0x824FA768..0x824FA784)
	// 824FA768: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FA76C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824FA770: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824FA774: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824FA778: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824FA77C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824FA780: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FA788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824FA788 size=72
    let mut pc: u32 = 0x824FA788;
    'dispatch: loop {
        match pc {
            0x824FA788 => {
    //   block [0x824FA788..0x824FA7BC)
	// 824FA788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FA78C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824FA790: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824FA794: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FA798: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824FA79C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824FA7A0: 396B3960  addi r11, r11, 0x3960
	ctx.r[11].s64 = ctx.r[11].s64 + 14688;
	// 824FA7A4: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 824FA7A8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 824FA7AC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824FA7B0: 419A000C  beq cr6, 0x824fa7bc
	if ctx.cr[6].eq {
	pc = 0x824FA7BC; continue 'dispatch;
	}
	// 824FA7B4: 48038405  bl 0x82532bb8
	ctx.lr = 0x824FA7B8;
	sub_82532BB8(ctx, base);
	// 824FA7B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	pc = 0x824FA7BC; continue 'dispatch;
            }
            0x824FA7BC => {
    //   block [0x824FA7BC..0x824FA7D0)
	// 824FA7BC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824FA7C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824FA7C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824FA7C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824FA7CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FA7D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FA7D0 size=20
    let mut pc: u32 = 0x824FA7D0;
    'dispatch: loop {
        match pc {
            0x824FA7D0 => {
    //   block [0x824FA7D0..0x824FA7E4)
	// 824FA7D0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FA7D4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824FA7D8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FA7DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824FA7E0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FA7E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FA7E8 size=56
    let mut pc: u32 = 0x824FA7E8;
    'dispatch: loop {
        match pc {
            0x824FA7E8 => {
    //   block [0x824FA7E8..0x824FA820)
	// 824FA7E8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824FA7EC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 824FA7F0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824FA7F4: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 824FA7F8: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 824FA7FC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 824FA800: 396B3960  addi r11, r11, 0x3960
	ctx.r[11].s64 = ctx.r[11].s64 + 14688;
	// 824FA804: 394A3980  addi r10, r10, 0x3980
	ctx.r[10].s64 = ctx.r[10].s64 + 14720;
	// 824FA808: 39293970  addi r9, r9, 0x3970
	ctx.r[9].s64 = ctx.r[9].s64 + 14704;
	// 824FA80C: B1030006  sth r8, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 824FA810: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 824FA814: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824FA818: 91230008  stw r9, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 824FA81C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FA820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FA820 size=12
    let mut pc: u32 = 0x824FA820;
    'dispatch: loop {
        match pc {
            0x824FA820 => {
    //   block [0x824FA820..0x824FA82C)
	// 824FA820: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824FA824: 386B3980  addi r3, r11, 0x3980
	ctx.r[3].s64 = ctx.r[11].s64 + 14720;
	// 824FA828: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FA830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824FA830 size=100
    let mut pc: u32 = 0x824FA830;
    'dispatch: loop {
        match pc {
            0x824FA830 => {
    //   block [0x824FA830..0x824FA878)
	// 824FA830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FA834: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824FA838: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824FA83C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824FA840: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FA844: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824FA848: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824FA84C: 48013C75  bl 0x8250e4c0
	ctx.lr = 0x824FA850;
	sub_8250E4C0(ctx, base);
	// 824FA850: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 824FA854: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824FA858: 419A0020  beq cr6, 0x824fa878
	if ctx.cr[6].eq {
	pc = 0x824FA878; continue 'dispatch;
	}
	// 824FA85C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FA860: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 824FA864: 38C00028  li r6, 0x28
	ctx.r[6].s64 = 40;
	// 824FA868: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FA86C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824FA870: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824FA874: 4BF69845  bl 0x824640b8
	ctx.lr = 0x824FA878;
	sub_824640B8(ctx, base);
	pc = 0x824FA878; continue 'dispatch;
            }
            0x824FA878 => {
    //   block [0x824FA878..0x824FA894)
	// 824FA878: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FA87C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824FA880: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824FA884: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824FA888: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824FA88C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824FA890: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FA898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FA898 size=8
    let mut pc: u32 = 0x824FA898;
    'dispatch: loop {
        match pc {
            0x824FA898 => {
    //   block [0x824FA898..0x824FA8A0)
	// 824FA898: 3863FFF8  addi r3, r3, -8
	ctx.r[3].s64 = ctx.r[3].s64 + -8;
	// 824FA89C: 4BFFFF94  b 0x824fa830
	sub_824FA830(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FA8D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824FA8D0 size=44
    let mut pc: u32 = 0x824FA8D0;
    'dispatch: loop {
        match pc {
            0x824FA8D0 => {
    //   block [0x824FA8D0..0x824FA8FC)
	// 824FA8D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FA8D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824FA8D8: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FA8DC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824FA8E0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824FA8E4: 4800739D  bl 0x82501c80
	ctx.lr = 0x824FA8E8;
	sub_82501C80(ctx, base);
	// 824FA8E8: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824FA8EC: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 824FA8F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824FA8F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824FA8F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FA900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FA900 size=4
    let mut pc: u32 = 0x824FA900;
    'dispatch: loop {
        match pc {
            0x824FA900 => {
    //   block [0x824FA900..0x824FA904)
	// 824FA900: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FA908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FA908 size=20
    let mut pc: u32 = 0x824FA908;
    'dispatch: loop {
        match pc {
            0x824FA908 => {
    //   block [0x824FA908..0x824FA91C)
	// 824FA908: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FA90C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824FA910: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FA914: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824FA918: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FA920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FA920 size=32
    let mut pc: u32 = 0x824FA920;
    'dispatch: loop {
        match pc {
            0x824FA920 => {
    //   block [0x824FA920..0x824FA940)
	// 824FA920: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824FA924: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 824FA928: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824FA92C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 824FA930: 396B3B0C  addi r11, r11, 0x3b0c
	ctx.r[11].s64 = ctx.r[11].s64 + 15116;
	// 824FA934: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 824FA938: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824FA93C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FA940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FA940 size=12
    let mut pc: u32 = 0x824FA940;
    'dispatch: loop {
        match pc {
            0x824FA940 => {
    //   block [0x824FA940..0x824FA94C)
	// 824FA940: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824FA944: 386B3B0C  addi r3, r11, 0x3b0c
	ctx.r[3].s64 = ctx.r[11].s64 + 15116;
	// 824FA948: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FA950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824FA950 size=140
    let mut pc: u32 = 0x824FA950;
    'dispatch: loop {
        match pc {
            0x824FA950 => {
    //   block [0x824FA950..0x824FA990)
	// 824FA950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FA954: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824FA958: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824FA95C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FA960: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824FA964: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 824FA968: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824FA96C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824FA970: 409A0020  bne cr6, 0x824fa990
	if !ctx.cr[6].eq {
	pc = 0x824FA990; continue 'dispatch;
	}
	// 824FA974: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FA978: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 824FA97C: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824FA980: 809F0014  lwz r4, 0x14(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 824FA984: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 824FA988: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824FA98C: 4BF6972D  bl 0x824640b8
	ctx.lr = 0x824FA990;
	sub_824640B8(ctx, base);
	pc = 0x824FA990; continue 'dispatch;
            }
            0x824FA990 => {
    //   block [0x824FA990..0x824FA9BC)
	// 824FA990: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 824FA994: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824FA998: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824FA99C: 409A0020  bne cr6, 0x824fa9bc
	if !ctx.cr[6].eq {
	pc = 0x824FA9BC; continue 'dispatch;
	}
	// 824FA9A0: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FA9A4: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 824FA9A8: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824FA9AC: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824FA9B0: 5565087C  rlwinm r5, r11, 1, 1, 0x1e
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x7FFFFFFFu64;
	// 824FA9B4: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824FA9B8: 4BF69701  bl 0x824640b8
	ctx.lr = 0x824FA9BC;
	sub_824640B8(ctx, base);
	pc = 0x824FA9BC; continue 'dispatch;
            }
            0x824FA9BC => {
    //   block [0x824FA9BC..0x824FA9DC)
	// 824FA9BC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824FA9C0: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 824FA9C4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824FA9C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824FA9CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824FA9D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824FA9D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824FA9D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FA9E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824FA9E0 size=100
    let mut pc: u32 = 0x824FA9E0;
    'dispatch: loop {
        match pc {
            0x824FA9E0 => {
    //   block [0x824FA9E0..0x824FAA28)
	// 824FA9E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FA9E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824FA9E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824FA9EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824FA9F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FA9F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824FA9F8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824FA9FC: 4BFFFF55  bl 0x824fa950
	ctx.lr = 0x824FAA00;
	sub_824FA950(ctx, base);
	// 824FAA00: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 824FAA04: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824FAA08: 419A0020  beq cr6, 0x824faa28
	if ctx.cr[6].eq {
	pc = 0x824FAA28; continue 'dispatch;
	}
	// 824FAA0C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FAA10: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 824FAA14: 38C00027  li r6, 0x27
	ctx.r[6].s64 = 39;
	// 824FAA18: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FAA1C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824FAA20: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824FAA24: 4BF69695  bl 0x824640b8
	ctx.lr = 0x824FAA28;
	sub_824640B8(ctx, base);
	pc = 0x824FAA28; continue 'dispatch;
            }
            0x824FAA28 => {
    //   block [0x824FAA28..0x824FAA44)
	// 824FAA28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FAA2C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824FAA30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824FAA34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824FAA38: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824FAA3C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824FAA40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FAA48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824FAA48 size=96
    let mut pc: u32 = 0x824FAA48;
    'dispatch: loop {
        match pc {
            0x824FAA48 => {
    //   block [0x824FAA48..0x824FAAA8)
	// 824FAA48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FAA4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824FAA50: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824FAA54: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FAA58: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 824FAA5C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824FAA60: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 824FAA64: 388B3B1C  addi r4, r11, 0x3b1c
	ctx.r[4].s64 = ctx.r[11].s64 + 15132;
	// 824FAA68: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 824FAA6C: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FAA70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FAA74: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FAA78: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824FAA7C: 4E800421  bctrl
	ctx.lr = 0x824FAA80;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824FAA80: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FAA84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FAA88: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 824FAA8C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824FAA90: 4E800421  bctrl
	ctx.lr = 0x824FAA94;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824FAA94: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824FAA98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824FAA9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824FAAA0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824FAAA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FAAA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FAAA8 size=8
    let mut pc: u32 = 0x824FAAA8;
    'dispatch: loop {
        match pc {
            0x824FAAA8 => {
    //   block [0x824FAAA8..0x824FAAB0)
	// 824FAAA8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 824FAAAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FAAB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FAAB0 size=16
    let mut pc: u32 = 0x824FAAB0;
    'dispatch: loop {
        match pc {
            0x824FAAB0 => {
    //   block [0x824FAAB0..0x824FAAC0)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FAAC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FAAC0 size=44
    let mut pc: u32 = 0x824FAAC0;
    'dispatch: loop {
        match pc {
            0x824FAAC0 => {
    //   block [0x824FAAC0..0x824FAAEC)
	// 824FAAC0: 3965FFFF  addi r11, r5, -1
	ctx.r[11].s64 = ctx.r[5].s64 + -1;
	// 824FAAC4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824FAAC8: 4D980020  bltlr cr6
	if ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FAAF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FAAF0 size=16
    let mut pc: u32 = 0x824FAAF0;
    'dispatch: loop {
        match pc {
            0x824FAAF0 => {
    //   block [0x824FAAF0..0x824FAB00)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FAB00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FAB00 size=16
    let mut pc: u32 = 0x824FAB00;
    'dispatch: loop {
        match pc {
            0x824FAB00 => {
    //   block [0x824FAB00..0x824FAB10)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FAB10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824FAB10 size=60
    let mut pc: u32 = 0x824FAB10;
    'dispatch: loop {
        match pc {
            0x824FAB10 => {
    //   block [0x824FAB10..0x824FAB4C)
	// 824FAB10: C0030010  lfs f0, 0x10(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FAB14: 39640030  addi r11, r4, 0x30
	ctx.r[11].s64 = ctx.r[4].s64 + 48;
	// 824FAB18: EC00082A  fadds f0, f0, f1
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[1].f64) as f32) as f64;
	// 824FAB1C: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 824FAB20: 3921FFF0  addi r9, r1, -0x10
	ctx.r[9].s64 = ctx.r[1].s64 + -16;
	// 824FAB24: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FAB50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FAB50 size=32
    let mut pc: u32 = 0x824FAB50;
    'dispatch: loop {
        match pc {
            0x824FAB50 => {
    //   block [0x824FAB50..0x824FAB70)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FAB70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824FAB70 size=44
    let mut pc: u32 = 0x824FAB70;
    'dispatch: loop {
        match pc {
            0x824FAB70 => {
    //   block [0x824FAB70..0x824FAB9C)
	// 824FAB70: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824FAB74: D0230010  stfs f1, 0x10(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 824FAB78: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 824FAB7C: 396B3B2C  addi r11, r11, 0x3b2c
	ctx.r[11].s64 = ctx.r[11].s64 + 15148;
	// 824FAB80: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 824FAB84: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 824FAB88: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 824FAB8C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824FAB90: 91230008  stw r9, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 824FAB94: 9103000C  stw r8, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 824FAB98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FABA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824FABA0 size=144
    let mut pc: u32 = 0x824FABA0;
    'dispatch: loop {
        match pc {
            0x824FABA0 => {
    //   block [0x824FABA0..0x824FABE8)
	// 824FABA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FABA4: 4803A519  bl 0x825350bc
	ctx.lr = 0x824FABA8;
	sub_82535080(ctx, base);
	// 824FABA8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FABAC: 83CD0000  lwz r30, 0(r13)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FABB0: 3BE00014  li r31, 0x14
	ctx.r[31].s64 = 20;
	// 824FABB4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 824FABB8: 7D7FF02E  lwzx r11, r31, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 824FABBC: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FABC0: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 824FABC4: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 824FABC8: 40980020  bge cr6, 0x824fabe8
	if !ctx.cr[6].lt {
	pc = 0x824FABE8; continue 'dispatch;
	}
	// 824FABCC: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 824FABD0: 39293B6C  addi r9, r9, 0x3b6c
	ctx.r[9].s64 = ctx.r[9].s64 + 15212;
	// 824FABD4: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 824FABD8: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 824FABDC: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 824FABE0: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 824FABE4: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x824FABE8; continue 'dispatch;
            }
            0x824FABE8 => {
    //   block [0x824FABE8..0x824FAC24)
	// 824FABE8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 824FABEC: C0240010  lfs f1, 0x10(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 824FABF0: 48000041  bl 0x824fac30
	ctx.lr = 0x824FABF4;
	sub_824FAC30(ctx, base);
	// 824FABF4: 7D5FF02E  lwzx r10, r31, r30
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 824FABF8: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FABFC: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 824FAC00: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 824FAC04: 40980020  bge cr6, 0x824fac24
	if !ctx.cr[6].lt {
	pc = 0x824FAC24; continue 'dispatch;
	}
	// 824FAC08: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 824FAC0C: 392974DC  addi r9, r9, 0x74dc
	ctx.r[9].s64 = ctx.r[9].s64 + 29916;
	// 824FAC10: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 824FAC14: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 824FAC18: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 824FAC1C: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 824FAC20: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x824FAC24; continue 'dispatch;
            }
            0x824FAC24 => {
    //   block [0x824FAC24..0x824FAC30)
	// 824FAC24: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 824FAC28: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824FAC2C: 4803A4E0  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FAC30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FAC30 size=256
    let mut pc: u32 = 0x824FAC30;
    'dispatch: loop {
        match pc {
            0x824FAC30 => {
    //   block [0x824FAC30..0x824FAD30)
	// 824FAC30: 39650010  addi r11, r5, 0x10
	ctx.r[11].s64 = ctx.r[5].s64 + 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FAD30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824FAD30 size=120
    let mut pc: u32 = 0x824FAD30;
    'dispatch: loop {
        match pc {
            0x824FAD30 => {
    //   block [0x824FAD30..0x824FADA8)
	// 824FAD30: 3921FFF0  addi r9, r1, -0x10
	ctx.r[9].s64 = ctx.r[1].s64 + -16;
	// 824FAD34: EC0D0024  fdivs f0, f13, f0
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ((ctx.f[13].f64 / ctx.f[0].f64) as f32) as f64;
	// 824FAD38: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 824FAD3C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 824FAD40: D0060010  stfs f0, 0x10(r6)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(16 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FADA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FADA8 size=20
    let mut pc: u32 = 0x824FADA8;
    'dispatch: loop {
        match pc {
            0x824FADA8 => {
    //   block [0x824FADA8..0x824FADBC)
	// 824FADA8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FADAC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824FADB0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FADB4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824FADB8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FADC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824FADC0 size=88
    let mut pc: u32 = 0x824FADC0;
    'dispatch: loop {
        match pc {
            0x824FADC0 => {
    //   block [0x824FADC0..0x824FAE04)
	// 824FADC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FADC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824FADC8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824FADCC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FADD0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824FADD4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 824FADD8: 419A002C  beq cr6, 0x824fae04
	if ctx.cr[6].eq {
	pc = 0x824FAE04; continue 'dispatch;
	}
	// 824FADDC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824FADE0: 480140C9  bl 0x8250eea8
	ctx.lr = 0x824FADE4;
	sub_8250EEA8(ctx, base);
	// 824FADE4: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824FADE8: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 824FADEC: 396B3BA0  addi r11, r11, 0x3ba0
	ctx.r[11].s64 = ctx.r[11].s64 + 15264;
	// 824FADF0: 394A3B7C  addi r10, r10, 0x3b7c
	ctx.r[10].s64 = ctx.r[10].s64 + 15228;
	// 824FADF4: 39200016  li r9, 0x16
	ctx.r[9].s64 = 22;
	// 824FADF8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824FADFC: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 824FAE00: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	pc = 0x824FAE04; continue 'dispatch;
            }
            0x824FAE04 => {
    //   block [0x824FAE04..0x824FAE18)
	// 824FAE04: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824FAE08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824FAE0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824FAE10: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824FAE14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FAE18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824FAE18 size=48
    let mut pc: u32 = 0x824FAE18;
    'dispatch: loop {
        match pc {
            0x824FAE18 => {
    //   block [0x824FAE18..0x824FAE48)
	// 824FAE18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FAE1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824FAE20: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FAE24: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824FAE28: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824FAE2C: 4801407D  bl 0x8250eea8
	ctx.lr = 0x824FAE30;
	sub_8250EEA8(ctx, base);
	// 824FAE30: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824FAE34: 386B3BA0  addi r3, r11, 0x3ba0
	ctx.r[3].s64 = ctx.r[11].s64 + 15264;
	// 824FAE38: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 824FAE3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824FAE40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824FAE44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FAE48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824FAE48 size=172
    let mut pc: u32 = 0x824FAE48;
    'dispatch: loop {
        match pc {
            0x824FAE48 => {
    //   block [0x824FAE48..0x824FAE88)
	// 824FAE48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FAE4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824FAE50: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824FAE54: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FAE58: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824FAE5C: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 824FAE60: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824FAE64: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824FAE68: 409A0020  bne cr6, 0x824fae88
	if !ctx.cr[6].eq {
	pc = 0x824FAE88; continue 'dispatch;
	}
	// 824FAE6C: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FAE70: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 824FAE74: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824FAE78: 809F0040  lwz r4, 0x40(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 824FAE7C: 5565087C  rlwinm r5, r11, 1, 1, 0x1e
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x7FFFFFFFu64;
	// 824FAE80: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824FAE84: 4BF69235  bl 0x824640b8
	ctx.lr = 0x824FAE88;
	sub_824640B8(ctx, base);
	pc = 0x824FAE88; continue 'dispatch;
            }
            0x824FAE88 => {
    //   block [0x824FAE88..0x824FAEB8)
	// 824FAE88: 817F003C  lwz r11, 0x3c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 824FAE8C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824FAE90: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824FAE94: 409A0024  bne cr6, 0x824faeb8
	if !ctx.cr[6].eq {
	pc = 0x824FAEB8; continue 'dispatch;
	}
	// 824FAE98: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FAE9C: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 824FAEA0: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 824FAEA4: 809F0034  lwz r4, 0x34(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 824FAEA8: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824FAEAC: 1CAB0038  mulli r5, r11, 0x38
	ctx.r[5].s32 = ((ctx.r[11].s32 as i64 * 56 as i64) as i32);
	ctx.r[5].s64 = ctx.r[5].s32 as i64;
	// 824FAEB0: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824FAEB4: 4BF69205  bl 0x824640b8
	ctx.lr = 0x824FAEB8;
	sub_824640B8(ctx, base);
	pc = 0x824FAEB8; continue 'dispatch;
            }
            0x824FAEB8 => {
    //   block [0x824FAEB8..0x824FAEC8)
	// 824FAEB8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 824FAEBC: 393F0010  addi r9, r31, 0x10
	ctx.r[9].s64 = ctx.r[31].s64 + 16;
	// 824FAEC0: 409A0008  bne cr6, 0x824faec8
	if !ctx.cr[6].eq {
	pc = 0x824FAEC8; continue 'dispatch;
	}
	// 824FAEC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	pc = 0x824FAEC8; continue 'dispatch;
            }
            0x824FAEC8 => {
    //   block [0x824FAEC8..0x824FAEF4)
	// 824FAEC8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824FAECC: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 824FAED0: 396B2A44  addi r11, r11, 0x2a44
	ctx.r[11].s64 = ctx.r[11].s64 + 10820;
	// 824FAED4: 394A6DD0  addi r10, r10, 0x6dd0
	ctx.r[10].s64 = ctx.r[10].s64 + 28112;
	// 824FAED8: 91690000  stw r11, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824FAEDC: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824FAEE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824FAEE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824FAEE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824FAEEC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824FAEF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FAEF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FAEF8 size=8
    let mut pc: u32 = 0x824FAEF8;
    'dispatch: loop {
        match pc {
            0x824FAEF8 => {
    //   block [0x824FAEF8..0x824FAF00)
	// 824FAEF8: 3863FFF0  addi r3, r3, -0x10
	ctx.r[3].s64 = ctx.r[3].s64 + -16;
	// 824FAEFC: 48000004  b 0x824faf00
	sub_824FAF00(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FAF00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824FAF00 size=100
    let mut pc: u32 = 0x824FAF00;
    'dispatch: loop {
        match pc {
            0x824FAF00 => {
    //   block [0x824FAF00..0x824FAF48)
	// 824FAF00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FAF04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824FAF08: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824FAF0C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824FAF10: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FAF14: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824FAF18: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824FAF1C: 4BFFFF2D  bl 0x824fae48
	ctx.lr = 0x824FAF20;
	sub_824FAE48(ctx, base);
	// 824FAF20: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 824FAF24: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824FAF28: 419A0020  beq cr6, 0x824faf48
	if ctx.cr[6].eq {
	pc = 0x824FAF48; continue 'dispatch;
	}
	// 824FAF2C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FAF30: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 824FAF34: 38C00027  li r6, 0x27
	ctx.r[6].s64 = 39;
	// 824FAF38: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FAF3C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824FAF40: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824FAF44: 4BF69175  bl 0x824640b8
	ctx.lr = 0x824FAF48;
	sub_824640B8(ctx, base);
	pc = 0x824FAF48; continue 'dispatch;
            }
            0x824FAF48 => {
    //   block [0x824FAF48..0x824FAF64)
	// 824FAF48: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FAF4C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824FAF50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824FAF54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824FAF58: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824FAF5C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824FAF60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FAF68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FAF68 size=20
    let mut pc: u32 = 0x824FAF68;
    'dispatch: loop {
        match pc {
            0x824FAF68 => {
    //   block [0x824FAF68..0x824FAF7C)
	// 824FAF68: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FAF6C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824FAF70: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FAF74: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824FAF78: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FAF80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FAF80 size=40
    let mut pc: u32 = 0x824FAF80;
    'dispatch: loop {
        match pc {
            0x824FAF80 => {
    //   block [0x824FAF80..0x824FAFA8)
	// 824FAF80: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824FAF84: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 824FAF88: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824FAF8C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 824FAF90: 396B3BFC  addi r11, r11, 0x3bfc
	ctx.r[11].s64 = ctx.r[11].s64 + 15356;
	// 824FAF94: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 824FAF98: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 824FAF9C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824FAFA0: 9123000C  stw r9, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 824FAFA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FAFA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FAFA8 size=12
    let mut pc: u32 = 0x824FAFA8;
    'dispatch: loop {
        match pc {
            0x824FAFA8 => {
    //   block [0x824FAFA8..0x824FAFB4)
	// 824FAFA8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824FAFAC: 386B3BFC  addi r3, r11, 0x3bfc
	ctx.r[3].s64 = ctx.r[11].s64 + 15356;
	// 824FAFB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FAFB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FAFB8 size=4
    let mut pc: u32 = 0x824FAFB8;
    'dispatch: loop {
        match pc {
            0x824FAFB8 => {
    //   block [0x824FAFB8..0x824FAFBC)
	// 824FAFB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FAFC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FAFC0 size=20
    let mut pc: u32 = 0x824FAFC0;
    'dispatch: loop {
        match pc {
            0x824FAFC0 => {
    //   block [0x824FAFC0..0x824FAFD4)
	// 824FAFC0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FAFC4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824FAFC8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FAFCC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824FAFD0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FAFD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FAFD8 size=40
    let mut pc: u32 = 0x824FAFD8;
    'dispatch: loop {
        match pc {
            0x824FAFD8 => {
    //   block [0x824FAFD8..0x824FB000)
	// 824FAFD8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824FAFDC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 824FAFE0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824FAFE4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 824FAFE8: 396B3B2C  addi r11, r11, 0x3b2c
	ctx.r[11].s64 = ctx.r[11].s64 + 15148;
	// 824FAFEC: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 824FAFF0: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 824FAFF4: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824FAFF8: 9123000C  stw r9, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 824FAFFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FB000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FB000 size=12
    let mut pc: u32 = 0x824FB000;
    'dispatch: loop {
        match pc {
            0x824FB000 => {
    //   block [0x824FB000..0x824FB00C)
	// 824FB000: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824FB004: 386B3B2C  addi r3, r11, 0x3b2c
	ctx.r[3].s64 = ctx.r[11].s64 + 15148;
	// 824FB008: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FB010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FB010 size=20
    let mut pc: u32 = 0x824FB010;
    'dispatch: loop {
        match pc {
            0x824FB010 => {
    //   block [0x824FB010..0x824FB024)
	// 824FB010: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FB014: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824FB018: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FB01C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824FB020: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FB028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FB028 size=16
    let mut pc: u32 = 0x824FB028;
    'dispatch: loop {
        match pc {
            0x824FB028 => {
    //   block [0x824FB028..0x824FB038)
	// 824FB028: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824FB02C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824FB030: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 824FB034: 4800003C  b 0x824fb070
	sub_824FB070(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FB038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FB038 size=4
    let mut pc: u32 = 0x824FB038;
    'dispatch: loop {
        match pc {
            0x824FB038 => {
    //   block [0x824FB038..0x824FB03C)
	// 824FB038: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FB040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824FB040 size=44
    let mut pc: u32 = 0x824FB040;
    'dispatch: loop {
        match pc {
            0x824FB040 => {
    //   block [0x824FB040..0x824FB06C)
	// 824FB040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FB044: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824FB048: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FB04C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824FB050: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824FB054: 4800001D  bl 0x824fb070
	ctx.lr = 0x824FB058;
	sub_824FB070(ctx, base);
	// 824FB058: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824FB05C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 824FB060: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824FB064: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824FB068: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FB070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824FB070 size=116
    let mut pc: u32 = 0x824FB070;
    'dispatch: loop {
        match pc {
            0x824FB070 => {
    //   block [0x824FB070..0x824FB0E4)
	// 824FB070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FB074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824FB078: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824FB07C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FB080: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824FB084: 4800373D  bl 0x824fe7c0
	ctx.lr = 0x824FB088;
	sub_824FE7C0(ctx, base);
	// 824FB088: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824FB08C: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 824FB090: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 824FB094: 3D008206  lis r8, -0x7dfa
	ctx.r[8].s64 = -2113536000;
	// 824FB098: 3CE08206  lis r7, -0x7dfa
	ctx.r[7].s64 = -2113536000;
	// 824FB09C: 396BFBA4  addi r11, r11, -0x45c
	ctx.r[11].s64 = ctx.r[11].s64 + -1116;
	// 824FB0A0: 394AFB98  addi r10, r10, -0x468
	ctx.r[10].s64 = ctx.r[10].s64 + -1128;
	// 824FB0A4: 3929FB84  addi r9, r9, -0x47c
	ctx.r[9].s64 = ctx.r[9].s64 + -1148;
	// 824FB0A8: 3908FB78  addi r8, r8, -0x488
	ctx.r[8].s64 = ctx.r[8].s64 + -1160;
	// 824FB0AC: 38E7FB6C  addi r7, r7, -0x494
	ctx.r[7].s64 = ctx.r[7].s64 + -1172;
	// 824FB0B0: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 824FB0B4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824FB0B8: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 824FB0BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FB0C0: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 824FB0C4: 911F0010  stw r8, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 824FB0C8: 90FF0014  stw r7, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[7].u32 ) };
	// 824FB0CC: 90DF0020  stw r6, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[6].u32 ) };
	// 824FB0D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824FB0D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824FB0D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824FB0DC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824FB0E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FB0E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FB0E8 size=20
    let mut pc: u32 = 0x824FB0E8;
    'dispatch: loop {
        match pc {
            0x824FB0E8 => {
    //   block [0x824FB0E8..0x824FB0FC)
	// 824FB0E8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FB0EC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824FB0F0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FB0F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824FB0F8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FB100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FB100 size=52
    let mut pc: u32 = 0x824FB100;
    'dispatch: loop {
        match pc {
            0x824FB100 => {
    //   block [0x824FB100..0x824FB134)
	// 824FB100: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824FB104: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 824FB108: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824FB10C: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 824FB110: 396B3D1C  addi r11, r11, 0x3d1c
	ctx.r[11].s64 = ctx.r[11].s64 + 15644;
	// 824FB114: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 824FB118: 394A2A68  addi r10, r10, 0x2a68
	ctx.r[10].s64 = ctx.r[10].s64 + 10856;
	// 824FB11C: 3900000D  li r8, 0xd
	ctx.r[8].s64 = 13;
	// 824FB120: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824FB124: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 824FB128: 91430014  stw r10, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 824FB12C: 9103000C  stw r8, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 824FB130: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FB138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FB138 size=12
    let mut pc: u32 = 0x824FB138;
    'dispatch: loop {
        match pc {
            0x824FB138 => {
    //   block [0x824FB138..0x824FB144)
	// 824FB138: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824FB13C: 386B3D1C  addi r3, r11, 0x3d1c
	ctx.r[3].s64 = ctx.r[11].s64 + 15644;
	// 824FB140: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FB148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824FB148 size=144
    let mut pc: u32 = 0x824FB148;
    'dispatch: loop {
        match pc {
            0x824FB148 => {
    //   block [0x824FB148..0x824FB188)
	// 824FB148: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FB14C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824FB150: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824FB154: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824FB158: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FB15C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824FB160: 3BC5FFA0  addi r30, r5, -0x60
	ctx.r[30].s64 = ctx.r[5].s64 + -96;
	// 824FB164: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 824FB168: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 824FB16C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FB170: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 824FB174: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824FB178: 4E800421  bctrl
	ctx.lr = 0x824FB17C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824FB17C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 824FB180: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824FB184: 4098000C  bge cr6, 0x824fb190
	if !ctx.cr[6].lt {
	pc = 0x824FB190; continue 'dispatch;
	}
            }
            0x824FB188 => {
    //   block [0x824FB188..0x824FB190)
	// 824FB188: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 824FB18C: 48000034  b 0x824fb1c0
	pc = 0x824FB1C0; continue 'dispatch;
            }
            0x824FB190 => {
    //   block [0x824FB190..0x824FB1B8)
	// 824FB190: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 824FB194: 4199FFF4  bgt cr6, 0x824fb188
	if ctx.cr[6].gt {
	pc = 0x824FB188; continue 'dispatch;
	}
	// 824FB198: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 824FB19C: 393F0060  addi r9, r31, 0x60
	ctx.r[9].s64 = ctx.r[31].s64 + 96;
	// 824FB1A0: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 824FB1A4: 409A0014  bne cr6, 0x824fb1b8
	if !ctx.cr[6].eq {
	pc = 0x824FB1B8; continue 'dispatch;
	}
	// 824FB1A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 824FB1AC: 386B0060  addi r3, r11, 0x60
	ctx.r[3].s64 = ctx.r[11].s64 + 96;
	// 824FB1B0: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 824FB1B4: 4800000C  b 0x824fb1c0
	pc = 0x824FB1C0; continue 'dispatch;
            }
            0x824FB1B8 => {
    //   block [0x824FB1B8..0x824FB1C0)
	// 824FB1B8: 38600060  li r3, 0x60
	ctx.r[3].s64 = 96;
	// 824FB1BC: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	pc = 0x824FB1C0; continue 'dispatch;
            }
            0x824FB1C0 => {
    //   block [0x824FB1C0..0x824FB1D8)
	// 824FB1C0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824FB1C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824FB1C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824FB1CC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824FB1D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824FB1D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FB1D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824FB1D8 size=116
    let mut pc: u32 = 0x824FB1D8;
    'dispatch: loop {
        match pc {
            0x824FB1D8 => {
    //   block [0x824FB1D8..0x824FB230)
	// 824FB1D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FB1DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824FB1E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824FB1E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824FB1E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FB1EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824FB1F0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824FB1F4: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 824FB1F8: 4BFFE4F9  bl 0x824f96f0
	ctx.lr = 0x824FB1FC;
	sub_824F96F0(ctx, base);
	// 824FB1FC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824FB200: 57CA07FE  clrlwi r10, r30, 0x1f
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 824FB204: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 824FB208: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 824FB20C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824FB210: 419A0020  beq cr6, 0x824fb230
	if ctx.cr[6].eq {
	pc = 0x824FB230; continue 'dispatch;
	}
	// 824FB214: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FB218: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 824FB21C: 38C00027  li r6, 0x27
	ctx.r[6].s64 = 39;
	// 824FB220: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FB224: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824FB228: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824FB22C: 4BF68E8D  bl 0x824640b8
	ctx.lr = 0x824FB230;
	sub_824640B8(ctx, base);
	pc = 0x824FB230; continue 'dispatch;
            }
            0x824FB230 => {
    //   block [0x824FB230..0x824FB24C)
	// 824FB230: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FB234: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824FB238: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824FB23C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824FB240: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824FB244: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824FB248: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FB250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FB250 size=4
    let mut pc: u32 = 0x824FB250;
    'dispatch: loop {
        match pc {
            0x824FB250 => {
    //   block [0x824FB250..0x824FB254)
	// 824FB250: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FB288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824FB288 size=44
    let mut pc: u32 = 0x824FB288;
    'dispatch: loop {
        match pc {
            0x824FB288 => {
    //   block [0x824FB288..0x824FB2B4)
	// 824FB288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FB28C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824FB290: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FB294: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824FB298: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824FB29C: 48013C0D  bl 0x8250eea8
	ctx.lr = 0x824FB2A0;
	sub_8250EEA8(ctx, base);
	// 824FB2A0: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824FB2A4: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 824FB2A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824FB2AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824FB2B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FB2B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FB2B8 size=4
    let mut pc: u32 = 0x824FB2B8;
    'dispatch: loop {
        match pc {
            0x824FB2B8 => {
    //   block [0x824FB2B8..0x824FB2BC)
	// 824FB2B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FB2C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FB2C0 size=20
    let mut pc: u32 = 0x824FB2C0;
    'dispatch: loop {
        match pc {
            0x824FB2C0 => {
    //   block [0x824FB2C0..0x824FB2D4)
	// 824FB2C0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FB2C4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824FB2C8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FB2CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824FB2D0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FB2D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FB2D8 size=40
    let mut pc: u32 = 0x824FB2D8;
    'dispatch: loop {
        match pc {
            0x824FB2D8 => {
    //   block [0x824FB2D8..0x824FB300)
	// 824FB2D8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824FB2DC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 824FB2E0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824FB2E4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 824FB2E8: 396B3F3C  addi r11, r11, 0x3f3c
	ctx.r[11].s64 = ctx.r[11].s64 + 16188;
	// 824FB2EC: 3920000E  li r9, 0xe
	ctx.r[9].s64 = 14;
	// 824FB2F0: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 824FB2F4: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824FB2F8: 9123000C  stw r9, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 824FB2FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FB300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FB300 size=12
    let mut pc: u32 = 0x824FB300;
    'dispatch: loop {
        match pc {
            0x824FB300 => {
    //   block [0x824FB300..0x824FB30C)
	// 824FB300: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824FB304: 386B3F3C  addi r3, r11, 0x3f3c
	ctx.r[3].s64 = ctx.r[11].s64 + 16188;
	// 824FB308: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FB310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824FB310 size=100
    let mut pc: u32 = 0x824FB310;
    'dispatch: loop {
        match pc {
            0x824FB310 => {
    //   block [0x824FB310..0x824FB358)
	// 824FB310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FB314: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824FB318: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824FB31C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824FB320: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FB324: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824FB328: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824FB32C: 48015465  bl 0x82510790
	ctx.lr = 0x824FB330;
	sub_82510790(ctx, base);
	// 824FB330: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 824FB334: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824FB338: 419A0020  beq cr6, 0x824fb358
	if ctx.cr[6].eq {
	pc = 0x824FB358; continue 'dispatch;
	}
	// 824FB33C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FB340: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 824FB344: 38C00027  li r6, 0x27
	ctx.r[6].s64 = 39;
	// 824FB348: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FB34C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824FB350: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824FB354: 4BF68D65  bl 0x824640b8
	ctx.lr = 0x824FB358;
	sub_824640B8(ctx, base);
	pc = 0x824FB358; continue 'dispatch;
            }
            0x824FB358 => {
    //   block [0x824FB358..0x824FB374)
	// 824FB358: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FB35C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824FB360: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824FB364: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824FB368: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824FB36C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824FB370: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FB378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FB378 size=12
    let mut pc: u32 = 0x824FB378;
    'dispatch: loop {
        match pc {
            0x824FB378 => {
    //   block [0x824FB378..0x824FB384)
	// 824FB378: 8964006C  lbz r11, 0x6c(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(108 as u32) ) } as u64;
	// 824FB37C: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 824FB380: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FB388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824FB388 size=28
    let mut pc: u32 = 0x824FB388;
    'dispatch: loop {
        match pc {
            0x824FB388 => {
    //   block [0x824FB388..0x824FB3A4)
	// 824FB388: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 824FB38C: 81430060  lwz r10, 0x60(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(96 as u32) ) } as u64;
	// 824FB390: 7D6B29D6  mullw r11, r11, r5
	ctx.r[11].s32 = ((ctx.r[11].s32 as i64 * ctx.r[5].s32 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 824FB394: 7D6B2214  add r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 824FB398: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824FB39C: 7C2B542E  lfsx f1, r11, r10
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 824FB3A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FB3A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824FB3A8 size=844
    let mut pc: u32 = 0x824FB3A8;
    'dispatch: loop {
        match pc {
            0x824FB3A8 => {
    //   block [0x824FB3A8..0x824FB6F0)
	// 824FB3A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FB3AC: 48039CFD  bl 0x825350a8
	ctx.lr = 0x824FB3B0;
	sub_82535080(ctx, base);
	// 824FB3B0: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FB3B4: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 824FB3B8: 83A40000  lwz r29, 0(r4)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FB3BC: 7CAA2B78  mr r10, r5
	ctx.r[10].u64 = ctx.r[5].u64;
	// 824FB3C0: 3B2BFFFF  addi r25, r11, -1
	ctx.r[25].s64 = ctx.r[11].s64 + -1;
	// 824FB3C4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 824FB3C8: 2F190000  cmpwi cr6, r25, 0
	ctx.cr[6].compare_i32(ctx.r[25].s32, 0, &mut ctx.xer);
	// 824FB3CC: C0A91FF8  lfs f5, 0x1ff8(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8184 as u32) ) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 824FB3D0: D0A1FF50  stfs f5, -0xb0(r1)
	tmp.f32 = (ctx.f[5].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-176 as u32), tmp.u32 ) };
	// 824FB3D4: D0A1FF58  stfs f5, -0xa8(r1)
	tmp.f32 = (ctx.f[5].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-168 as u32), tmp.u32 ) };
	// 824FB3D8: C0EB1850  lfs f7, 0x1850(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 824FB3DC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824FB3E0: D0E1FF54  stfs f7, -0xac(r1)
	tmp.f32 = (ctx.f[7].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-172 as u32), tmp.u32 ) };
	// 824FB3E4: C00B8CB4  lfs f0, -0x734c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-29516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FB3E8: D001FF5C  stfs f0, -0xa4(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-164 as u32), tmp.u32 ) };
	// 824FB3EC: 41980304  blt cr6, 0x824fb6f0
	if ctx.cr[6].lt {
	pc = 0x824FB6F0; continue 'dispatch;
	}
	// 824FB3F0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824FB3F4: 3B430040  addi r26, r3, 0x40
	ctx.r[26].s64 = ctx.r[3].s64 + 64;
	// 824FB3F8: 3B6B3F70  addi r27, r11, 0x3f70
	ctx.r[27].s64 = ctx.r[11].s64 + 16240;
	// 824FB3FC: 3961FF50  addi r11, r1, -0xb0
	ctx.r[11].s64 = ctx.r[1].s64 + -176;
	// 824FB400: 3BC30010  addi r30, r3, 0x10
	ctx.r[30].s64 = ctx.r[3].s64 + 16;
	// 824FB404: 3B830030  addi r28, r3, 0x30
	ctx.r[28].s64 = ctx.r[3].s64 + 48;
	// 824FB408: 3B000020  li r24, 0x20
	ctx.r[24].s64 = 32;
	pc = 0x824FB6F0; continue 'dispatch;
            }
            0x824FB6F0 => {
    //   block [0x824FB6F0..0x824FB6F4)
	// 824FB6F0: 48039A08  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FB6F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824FB6F8 size=148
    let mut pc: u32 = 0x824FB6F8;
    'dispatch: loop {
        match pc {
            0x824FB6F8 => {
    //   block [0x824FB6F8..0x824FB740)
	// 824FB6F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FB6FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824FB700: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824FB704: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824FB708: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FB70C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824FB710: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824FB714: 817F0068  lwz r11, 0x68(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 824FB718: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824FB71C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824FB720: 409A0020  bne cr6, 0x824fb740
	if !ctx.cr[6].eq {
	pc = 0x824FB740; continue 'dispatch;
	}
	// 824FB724: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FB728: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 824FB72C: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824FB730: 809F0060  lwz r4, 0x60(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 824FB734: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824FB738: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824FB73C: 4BF6897D  bl 0x824640b8
	ctx.lr = 0x824FB740;
	sub_824640B8(ctx, base);
	pc = 0x824FB740; continue 'dispatch;
            }
            0x824FB740 => {
    //   block [0x824FB740..0x824FB770)
	// 824FB740: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FB744: 4801504D  bl 0x82510790
	ctx.lr = 0x824FB748;
	sub_82510790(ctx, base);
	// 824FB748: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 824FB74C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824FB750: 419A0020  beq cr6, 0x824fb770
	if ctx.cr[6].eq {
	pc = 0x824FB770; continue 'dispatch;
	}
	// 824FB754: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FB758: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 824FB75C: 38C00027  li r6, 0x27
	ctx.r[6].s64 = 39;
	// 824FB760: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FB764: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824FB768: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824FB76C: 4BF6894D  bl 0x824640b8
	ctx.lr = 0x824FB770;
	sub_824640B8(ctx, base);
	pc = 0x824FB770; continue 'dispatch;
            }
            0x824FB770 => {
    //   block [0x824FB770..0x824FB78C)
	// 824FB770: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FB774: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824FB778: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824FB77C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824FB780: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824FB784: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824FB788: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FB790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824FB790 size=156
    let mut pc: u32 = 0x824FB790;
    'dispatch: loop {
        match pc {
            0x824FB790 => {
    //   block [0x824FB790..0x824FB7CC)
	// 824FB790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FB794: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824FB798: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824FB79C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824FB7A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FB7A4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 824FB7A8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824FB7AC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 824FB7B0: 419A001C  beq cr6, 0x824fb7cc
	if ctx.cr[6].eq {
	pc = 0x824FB7CC; continue 'dispatch;
	}
	// 824FB7B4: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FB7B8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824FB7BC: 419A0010  beq cr6, 0x824fb7cc
	if ctx.cr[6].eq {
	pc = 0x824FB7CC; continue 'dispatch;
	}
	// 824FB7C0: A17F0006  lhz r11, 6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 824FB7C4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824FB7C8: B17F0006  sth r11, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	pc = 0x824FB7CC; continue 'dispatch;
            }
            0x824FB7CC => {
    //   block [0x824FB7CC..0x824FB810)
	// 824FB7CC: 807E005C  lwz r3, 0x5c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(92 as u32) ) } as u64;
	// 824FB7D0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824FB7D4: 419A003C  beq cr6, 0x824fb810
	if ctx.cr[6].eq {
	pc = 0x824FB810; continue 'dispatch;
	}
	// 824FB7D8: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FB7DC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824FB7E0: 419A0030  beq cr6, 0x824fb810
	if ctx.cr[6].eq {
	pc = 0x824FB810; continue 'dispatch;
	}
	// 824FB7E4: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 824FB7E8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 824FB7EC: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 824FB7F0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824FB7F4: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 824FB7F8: 409A0018  bne cr6, 0x824fb810
	if !ctx.cr[6].eq {
	pc = 0x824FB810; continue 'dispatch;
	}
	// 824FB7FC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FB800: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824FB804: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FB808: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824FB80C: 4E800421  bctrl
	ctx.lr = 0x824FB810;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x824FB810 => {
    //   block [0x824FB810..0x824FB82C)
	// 824FB810: 93FE005C  stw r31, 0x5c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 824FB814: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824FB818: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824FB81C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824FB820: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824FB824: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824FB828: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FB830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FB830 size=8
    let mut pc: u32 = 0x824FB830;
    'dispatch: loop {
        match pc {
            0x824FB830 => {
    //   block [0x824FB830..0x824FB838)
	// 824FB830: 8063004C  lwz r3, 0x4c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(76 as u32) ) } as u64;
	// 824FB834: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FB838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FB838 size=12
    let mut pc: u32 = 0x824FB838;
    'dispatch: loop {
        match pc {
            0x824FB838 => {
    //   block [0x824FB838..0x824FB844)
	// 824FB838: 548B2036  slwi r11, r4, 4
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824FB83C: 7C6B1A14  add r3, r11, r3
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 824FB840: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FB848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FB848 size=8
    let mut pc: u32 = 0x824FB848;
    'dispatch: loop {
        match pc {
            0x824FB848 => {
    //   block [0x824FB848..0x824FB850)
	// 824FB848: 38630050  addi r3, r3, 0x50
	ctx.r[3].s64 = ctx.r[3].s64 + 80;
	// 824FB84C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FB850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824FB850 size=44
    let mut pc: u32 = 0x824FB850;
    'dispatch: loop {
        match pc {
            0x824FB850 => {
    //   block [0x824FB850..0x824FB87C)
	// 824FB850: 81630040  lwz r11, 0x40(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(64 as u32) ) } as u64;
	// 824FB854: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 824FB858: C1AB0000  lfs f13, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824FB85C: D1A40000  stfs f13, 0(r4)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 824FB860: C1AB0010  lfs f13, 0x10(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824FB864: D1A40004  stfs f13, 4(r4)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 824FB868: C1AB0020  lfs f13, 0x20(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824FB86C: C00A1FF8  lfs f0, 0x1ff8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FB870: D1A40008  stfs f13, 8(r4)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 824FB874: D004000C  stfs f0, 0xc(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 824FB878: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FB880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824FB880 size=448
    let mut pc: u32 = 0x824FB880;
    'dispatch: loop {
        match pc {
            0x824FB880 => {
    //   block [0x824FB880..0x824FB990)
	// 824FB880: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 824FB884: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 824FB888: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 824FB88C: 810A004C  lwz r8, 0x4c(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(76 as u32) ) } as u64;
	// 824FB890: 816A0040  lwz r11, 0x40(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(64 as u32) ) } as u64;
	// 824FB894: 38A8FFFF  addi r5, r8, -1
	ctx.r[5].s64 = ctx.r[8].s64 + -1;
	// 824FB898: 2F050003  cmpwi cr6, r5, 3
	ctx.cr[6].compare_i32(ctx.r[5].s32, 3, &mut ctx.xer);
	// 824FB89C: 419800F4  blt cr6, 0x824fb990
	if ctx.cr[6].lt {
	pc = 0x824FB990; continue 'dispatch;
	}
	// 824FB8A0: 39050001  addi r8, r5, 1
	ctx.r[8].s64 = ctx.r[5].s64 + 1;
	// 824FB8A4: 5508F0BE  srwi r8, r8, 2
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shr(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 824FB8A8: 5507103A  slwi r7, r8, 2
	ctx.r[7].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 824FB8AC: 7CA72850  subf r5, r7, r5
	ctx.r[5].s64 = ctx.r[5].s64 - ctx.r[7].s64;
	// 824FB8B0: C00B0000  lfs f0, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FB8B4: 7D274B78  mr r7, r9
	ctx.r[7].u64 = ctx.r[9].u64;
	// 824FB8B8: D001FFC0  stfs f0, -0x40(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), tmp.u32 ) };
	// 824FB8BC: 39290010  addi r9, r9, 0x10
	ctx.r[9].s64 = ctx.r[9].s64 + 16;
	// 824FB8C0: C00B0010  lfs f0, 0x10(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FB8C4: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 824FB8C8: D001FFC4  stfs f0, -0x3c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-60 as u32), tmp.u32 ) };
	// 824FB8CC: 7D264B78  mr r6, r9
	ctx.r[6].u64 = ctx.r[9].u64;
	// 824FB8D0: C00B0020  lfs f0, 0x20(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FB8D4: 39290010  addi r9, r9, 0x10
	ctx.r[9].s64 = ctx.r[9].s64 + 16;
	// 824FB8D8: D001FFC8  stfs f0, -0x38(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), tmp.u32 ) };
	// 824FB8DC: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 824FB8E0: C00A0010  lfs f0, 0x10(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FB8E4: D001FFCC  stfs f0, -0x34(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-52 as u32), tmp.u32 ) };
	// 824FB8E8: 3881FFC0  addi r4, r1, -0x40
	ctx.r[4].s64 = ctx.r[1].s64 + -64;
	pc = 0x824FB990; continue 'dispatch;
            }
            0x824FB990 => {
    //   block [0x824FB990..0x824FBA40)
	// 824FB990: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 824FB994: 41980038  blt cr6, 0x824fb9cc
	if ctx.cr[6].lt {
	pc = 0x824FB9CC; continue 'dispatch;
	}
	// 824FB998: C00B0000  lfs f0, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FB99C: 7D284B78  mr r8, r9
	ctx.r[8].u64 = ctx.r[9].u64;
	// 824FB9A0: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 824FB9A4: 39290010  addi r9, r9, 0x10
	ctx.r[9].s64 = ctx.r[9].s64 + 16;
	// 824FB9A8: C00B0010  lfs f0, 0x10(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FB9AC: D001FFF4  stfs f0, -0xc(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-12 as u32), tmp.u32 ) };
	// 824FB9B0: C00B0020  lfs f0, 0x20(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FB9B4: D001FFF8  stfs f0, -8(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), tmp.u32 ) };
	// 824FB9B8: C00A0010  lfs f0, 0x10(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FB9BC: D001FFFC  stfs f0, -4(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 824FB9C0: 38E1FFF0  addi r7, r1, -0x10
	ctx.r[7].s64 = ctx.r[1].s64 + -16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FBA40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824FBA40 size=432
    let mut pc: u32 = 0x824FBA40;
    'dispatch: loop {
        match pc {
            0x824FBA40 => {
    //   block [0x824FBA40..0x824FBA80)
	// 824FBA40: 3D20829A  lis r9, -0x7d66
	ctx.r[9].s64 = -2103836672;
	// 824FBA44: 80C30040  lwz r6, 0x40(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(64 as u32) ) } as u64;
	// 824FBA48: 81492250  lwz r10, 0x2250(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8784 as u32) ) } as u64;
	// 824FBA4C: 554B07FE  clrlwi r11, r10, 0x1f
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 824FBA50: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824FBA54: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 824FBA58: 396B2240  addi r11, r11, 0x2240
	ctx.r[11].s64 = ctx.r[11].s64 + 8768;
	// 824FBA5C: 409A0024  bne cr6, 0x824fba80
	if !ctx.cr[6].eq {
	pc = 0x824FBA80; continue 'dispatch;
	}
	// 824FBA60: 614A0001  ori r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u64 | 1;
	// 824FBA64: 91492250  stw r10, 0x2250(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8784 as u32), ctx.r[10].u32 ) };
	// 824FBA68: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 824FBA6C: C00A0DA0  lfs f0, 0xda0(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3488 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FBA70: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 824FBA74: D00B0004  stfs f0, 4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 824FBA78: D00B0008  stfs f0, 8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 824FBA7C: D00B000C  stfs f0, 0xc(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), tmp.u32 ) };
	pc = 0x824FBA80; continue 'dispatch;
            }
            0x824FBA80 => {
    //   block [0x824FBA80..0x824FBBF0)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FBBF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824FBBF0 size=520
    let mut pc: u32 = 0x824FBBF0;
    'dispatch: loop {
        match pc {
            0x824FBBF0 => {
    //   block [0x824FBBF0..0x824FBC1C)
	// 824FBBF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FBBF4: 480394C5  bl 0x825350b8
	ctx.lr = 0x824FBBF8;
	sub_82535080(ctx, base);
	// 824FBBF8: 3965FFFF  addi r11, r5, -1
	ctx.r[11].s64 = ctx.r[5].s64 + -1;
	// 824FBBFC: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 824FBC00: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 824FBC04: 2F0A0004  cmpwi cr6, r10, 4
	ctx.cr[6].compare_i32(ctx.r[10].s32, 4, &mut ctx.xer);
	// 824FBC08: 4198017C  blt cr6, 0x824fbd84
	if ctx.cr[6].lt {
	pc = 0x824FBD84; continue 'dispatch;
	}
	// 824FBC0C: 5547F0BE  srwi r7, r10, 2
	ctx.r[7].u32 = ctx.r[10].u32.wrapping_shr(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 824FBC10: 39660018  addi r11, r6, 0x18
	ctx.r[11].s64 = ctx.r[6].s64 + 24;
	// 824FBC14: 54EA103A  slwi r10, r7, 2
	ctx.r[10].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824FBC18: 7FCAF050  subf r30, r10, r30
	ctx.r[30].s64 = ctx.r[30].s64 - ctx.r[10].s64;
	pc = 0x824FBC1C; continue 'dispatch;
            }
            0x824FBC1C => {
    //   block [0x824FBC1C..0x824FBD84)
	// 824FBC1C: A1440000  lhz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FBC20: 38E7FFFF  addi r7, r7, -1
	ctx.r[7].s64 = ctx.r[7].s64 + -1;
	// 824FBC24: 80A30040  lwz r5, 0x40(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(64 as u32) ) } as u64;
	// 824FBC28: 7D481670  srawi r8, r10, 2
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[10].s32 >> 2) as i64;
	// 824FBC2C: 554907BE  clrlwi r9, r10, 0x1e
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x00000003u64;
	// 824FBC30: 655F3F00  oris r31, r10, 0x3f00
	ctx.r[31].u64 = ctx.r[10].u64 | 1056964608;
	// 824FBC34: 550A083C  slwi r10, r8, 1
	ctx.r[10].u32 = ctx.r[8].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824FBC38: 3BA90004  addi r29, r9, 4
	ctx.r[29].s64 = ctx.r[9].s64 + 4;
	// 824FBC3C: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 824FBC40: 553C103A  slwi r28, r9, 2
	ctx.r[28].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 824FBC44: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824FBC48: 39290008  addi r9, r9, 8
	ctx.r[9].s64 = ctx.r[9].s64 + 8;
	// 824FBC4C: 7D4A2A14  add r10, r10, r5
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[5].u64;
	// 824FBC50: 57A8103A  slwi r8, r29, 2
	ctx.r[8].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 824FBC54: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824FBC58: 7C1C542E  lfsx f0, r28, r10
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FBC5C: D0060000  stfs f0, 0(r6)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 824FBC60: 7C08542E  lfsx f0, r8, r10
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FBC64: D00BFFEC  stfs f0, -0x14(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-20 as u32), tmp.u32 ) };
	// 824FBC68: 7C09542E  lfsx f0, r9, r10
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FBC6C: D00BFFF0  stfs f0, -0x10(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 824FBC70: 93EBFFF4  stw r31, -0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-12 as u32), ctx.r[31].u32 ) };
	// 824FBC74: A1440002  lhz r10, 2(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(2 as u32) ) } as u64;
	// 824FBC78: 80A30040  lwz r5, 0x40(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(64 as u32) ) } as u64;
	// 824FBC7C: 7D481670  srawi r8, r10, 2
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[10].s32 >> 2) as i64;
	// 824FBC80: 554907BE  clrlwi r9, r10, 0x1e
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x00000003u64;
	// 824FBC84: 655F3F00  oris r31, r10, 0x3f00
	ctx.r[31].u64 = ctx.r[10].u64 | 1056964608;
	// 824FBC88: 550A083C  slwi r10, r8, 1
	ctx.r[10].u32 = ctx.r[8].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824FBC8C: 3BA90004  addi r29, r9, 4
	ctx.r[29].s64 = ctx.r[9].s64 + 4;
	// 824FBC90: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 824FBC94: 553C103A  slwi r28, r9, 2
	ctx.r[28].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 824FBC98: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824FBC9C: 39290008  addi r9, r9, 8
	ctx.r[9].s64 = ctx.r[9].s64 + 8;
	// 824FBCA0: 7D4A2A14  add r10, r10, r5
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[5].u64;
	// 824FBCA4: 57A8103A  slwi r8, r29, 2
	ctx.r[8].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 824FBCA8: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824FBCAC: 7C1C542E  lfsx f0, r28, r10
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FBCB0: D00BFFF8  stfs f0, -8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-8 as u32), tmp.u32 ) };
	// 824FBCB4: 7C08542E  lfsx f0, r8, r10
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FBCB8: D00BFFFC  stfs f0, -4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 824FBCBC: 7C09542E  lfsx f0, r9, r10
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FBCC0: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 824FBCC4: 93EB0004  stw r31, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 824FBCC8: A1440004  lhz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FBCCC: 80A30040  lwz r5, 0x40(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(64 as u32) ) } as u64;
	// 824FBCD0: 7D481670  srawi r8, r10, 2
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[10].s32 >> 2) as i64;
	// 824FBCD4: 554907BE  clrlwi r9, r10, 0x1e
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x00000003u64;
	// 824FBCD8: 655F3F00  oris r31, r10, 0x3f00
	ctx.r[31].u64 = ctx.r[10].u64 | 1056964608;
	// 824FBCDC: 550A083C  slwi r10, r8, 1
	ctx.r[10].u32 = ctx.r[8].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824FBCE0: 3BA90004  addi r29, r9, 4
	ctx.r[29].s64 = ctx.r[9].s64 + 4;
	// 824FBCE4: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 824FBCE8: 553C103A  slwi r28, r9, 2
	ctx.r[28].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 824FBCEC: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824FBCF0: 39290008  addi r9, r9, 8
	ctx.r[9].s64 = ctx.r[9].s64 + 8;
	// 824FBCF4: 7D4A2A14  add r10, r10, r5
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[5].u64;
	// 824FBCF8: 57A8103A  slwi r8, r29, 2
	ctx.r[8].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 824FBCFC: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824FBD00: 7C1C542E  lfsx f0, r28, r10
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FBD04: D00B0008  stfs f0, 8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 824FBD08: 7C08542E  lfsx f0, r8, r10
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FBD0C: D00B000C  stfs f0, 0xc(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 824FBD10: 7C09542E  lfsx f0, r9, r10
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FBD14: D00B0010  stfs f0, 0x10(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 824FBD18: 93EB0014  stw r31, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[31].u32 ) };
	// 824FBD1C: A1440006  lhz r10, 6(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(6 as u32) ) } as u64;
	// 824FBD20: 38840008  addi r4, r4, 8
	ctx.r[4].s64 = ctx.r[4].s64 + 8;
	// 824FBD24: 80A30040  lwz r5, 0x40(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(64 as u32) ) } as u64;
	// 824FBD28: 7D481670  srawi r8, r10, 2
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[10].s32 >> 2) as i64;
	// 824FBD2C: 554907BE  clrlwi r9, r10, 0x1e
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x00000003u64;
	// 824FBD30: 551F083C  slwi r31, r8, 1
	ctx.r[31].u32 = ctx.r[8].u32.wrapping_shl(1);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 824FBD34: 553D103A  slwi r29, r9, 2
	ctx.r[29].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 824FBD38: 7D08FA14  add r8, r8, r31
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[31].u64;
	// 824FBD3C: 654A3F00  oris r10, r10, 0x3f00
	ctx.r[10].u64 = ctx.r[10].u64 | 1056964608;
	// 824FBD40: 55082036  slwi r8, r8, 4
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(4);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 824FBD44: 7D082A14  add r8, r8, r5
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[5].u64;
	// 824FBD48: 38A90004  addi r5, r9, 4
	ctx.r[5].s64 = ctx.r[9].s64 + 4;
	// 824FBD4C: 39290008  addi r9, r9, 8
	ctx.r[9].s64 = ctx.r[9].s64 + 8;
	// 824FBD50: 54A5103A  slwi r5, r5, 2
	ctx.r[5].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824FBD54: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824FBD58: 7C1D442E  lfsx f0, r29, r8
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[8].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FBD5C: D00B0018  stfs f0, 0x18(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 824FBD60: 7C05442E  lfsx f0, r5, r8
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[5].u32.wrapping_add(ctx.r[8].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FBD64: 38C60040  addi r6, r6, 0x40
	ctx.r[6].s64 = ctx.r[6].s64 + 64;
	// 824FBD68: D00B001C  stfs f0, 0x1c(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 824FBD6C: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 824FBD70: 7C09442E  lfsx f0, r9, r8
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[8].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FBD74: D00B0020  stfs f0, 0x20(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 824FBD78: 914B0024  stw r10, 0x24(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), ctx.r[10].u32 ) };
	// 824FBD7C: 396B0040  addi r11, r11, 0x40
	ctx.r[11].s64 = ctx.r[11].s64 + 64;
	// 824FBD80: 409AFE9C  bne cr6, 0x824fbc1c
	if !ctx.cr[6].eq {
	pc = 0x824FBC1C; continue 'dispatch;
	}
	pc = 0x824FBD84; continue 'dispatch;
            }
            0x824FBD84 => {
    //   block [0x824FBD84..0x824FBD8C)
	// 824FBD84: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 824FBD88: 4198006C  blt cr6, 0x824fbdf4
	if ctx.cr[6].lt {
	pc = 0x824FBDF4; continue 'dispatch;
	}
	pc = 0x824FBD8C; continue 'dispatch;
            }
            0x824FBD8C => {
    //   block [0x824FBD8C..0x824FBDF4)
	// 824FBD8C: A1640000  lhz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FBD90: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 824FBD94: 81030040  lwz r8, 0x40(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(64 as u32) ) } as u64;
	// 824FBD98: 38840002  addi r4, r4, 2
	ctx.r[4].s64 = ctx.r[4].s64 + 2;
	// 824FBD9C: 7D691670  srawi r9, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 824FBDA0: 556A07BE  clrlwi r10, r11, 0x1e
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 824FBDA4: 65673F00  oris r7, r11, 0x3f00
	ctx.r[7].u64 = ctx.r[11].u64 | 1056964608;
	// 824FBDA8: 552B083C  slwi r11, r9, 1
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824FBDAC: 38AA0004  addi r5, r10, 4
	ctx.r[5].s64 = ctx.r[10].s64 + 4;
	// 824FBDB0: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 824FBDB4: 555F103A  slwi r31, r10, 2
	ctx.r[31].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 824FBDB8: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824FBDBC: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 824FBDC0: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 824FBDC4: 54A9103A  slwi r9, r5, 2
	ctx.r[9].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824FBDC8: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824FBDCC: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 824FBDD0: 7C1F5C2E  lfsx f0, r31, r11
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FBDD4: D0060000  stfs f0, 0(r6)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 824FBDD8: 7C095C2E  lfsx f0, r9, r11
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FBDDC: D0060004  stfs f0, 4(r6)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 824FBDE0: 7C0A5C2E  lfsx f0, r10, r11
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FBDE4: D0060008  stfs f0, 8(r6)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 824FBDE8: 90E6000C  stw r7, 0xc(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 824FBDEC: 38C60010  addi r6, r6, 0x10
	ctx.r[6].s64 = ctx.r[6].s64 + 16;
	// 824FBDF0: 4098FF9C  bge cr6, 0x824fbd8c
	if !ctx.cr[6].lt {
	pc = 0x824FBD8C; continue 'dispatch;
	}
	pc = 0x824FBDF4; continue 'dispatch;
            }
            0x824FBDF4 => {
    //   block [0x824FBDF4..0x824FBDF8)
	// 824FBDF4: 48039314  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FBDF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FBDF8 size=16
    let mut pc: u32 = 0x824FBDF8;
    'dispatch: loop {
        match pc {
            0x824FBDF8 => {
    //   block [0x824FBDF8..0x824FBE08)
	// 824FBDF8: 39600030  li r11, 0x30
	ctx.r[11].s64 = 48;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FBE08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824FBE08 size=212
    let mut pc: u32 = 0x824FBE08;
    'dispatch: loop {
        match pc {
            0x824FBE08 => {
    //   block [0x824FBE08..0x824FBEDC)
	// 824FBE08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FBE0C: 480392A5  bl 0x825350b0
	ctx.lr = 0x824FBE10;
	sub_82535080(ctx, base);
	// 824FBE10: 39250003  addi r9, r5, 3
	ctx.r[9].s64 = ctx.r[5].s64 + 3;
	// 824FBE14: D0230010  stfs f1, 0x10(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 824FBE18: 3FC08206  lis r30, -0x7dfa
	ctx.r[30].s64 = -2113536000;
	// 824FBE1C: 5529003A  rlwinm r9, r9, 0, 0, 0x1d
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 824FBE20: 3BDE3804  addi r30, r30, 0x3804
	ctx.r[30].s64 = ctx.r[30].s64 + 14340;
	// 824FBE24: 3B600001  li r27, 1
	ctx.r[27].s64 = 1;
	// 824FBE28: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 824FBE2C: 3B400007  li r26, 7
	ctx.r[26].s64 = 7;
	// 824FBE30: 7D291670  srawi r9, r9, 2
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[9].s32 >> 2) as i64;
	// 824FBE34: 3F808000  lis r28, -0x8000
	ctx.r[28].s64 = -2147483648;
	// 824FBE38: 93C30000  stw r30, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 824FBE3C: 7D290194  addze r9, r9
	tmp.s64 = ctx.r[9].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[9].u32);
	ctx.r[9].s64 = tmp.s64;
	// 824FBE40: B3630006  sth r27, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[27].u16 ) };
	// 824FBE44: 93A30008  stw r29, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 824FBE48: 7CFEE378  or r30, r7, r28
	ctx.r[30].u64 = ctx.r[7].u64 | ctx.r[28].u64;
	// 824FBE4C: 9343000C  stw r26, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[26].u32 ) };
	// 824FBE50: 3BE80010  addi r31, r8, 0x10
	ctx.r[31].s64 = ctx.r[8].s64 + 16;
	// 824FBE54: 90830040  stw r4, 0x40(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(64 as u32), ctx.r[4].u32 ) };
	// 824FBE58: 7D24E378  or r4, r9, r28
	ctx.r[4].u64 = ctx.r[9].u64 | ctx.r[28].u64;
	// 824FBE5C: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 824FBE60: 91230044  stw r9, 0x44(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(68 as u32), ctx.r[9].u32 ) };
	// 824FBE64: 3B61FFC0  addi r27, r1, -0x40
	ctx.r[27].s64 = ctx.r[1].s64 + -64;
	// 824FBE68: 39630020  addi r11, r3, 0x20
	ctx.r[11].s64 = ctx.r[3].s64 + 32;
	// 824FBE6C: 3B41FFC4  addi r26, r1, -0x3c
	ctx.r[26].s64 = ctx.r[1].s64 + -60;
	// 824FBE70: 90830048  stw r4, 0x48(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), ctx.r[4].u32 ) };
	// 824FBE74: 90A3004C  stw r5, 0x4c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(76 as u32), ctx.r[5].u32 ) };
	// 824FBE78: C00ABFFC  lfs f0, -0x4004(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-16388 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FBE7C: 90C30050  stw r6, 0x50(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(80 as u32), ctx.r[6].u32 ) };
	// 824FBE80: D001FFC0  stfs f0, -0x40(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), tmp.u32 ) };
	// 824FBE84: 90E30054  stw r7, 0x54(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FBEE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824FBEE0 size=232
    let mut pc: u32 = 0x824FBEE0;
    'dispatch: loop {
        match pc {
            0x824FBEE0 => {
    //   block [0x824FBEE0..0x824FBF44)
	// 824FBEE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FBEE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824FBEE8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824FBEEC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FBEF0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824FBEF4: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824FBEF8: 396B3804  addi r11, r11, 0x3804
	ctx.r[11].s64 = ctx.r[11].s64 + 14340;
	// 824FBEFC: 807F005C  lwz r3, 0x5c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 824FBF00: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824FBF04: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824FBF08: 419A003C  beq cr6, 0x824fbf44
	if ctx.cr[6].eq {
	pc = 0x824FBF44; continue 'dispatch;
	}
	// 824FBF0C: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FBF10: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824FBF14: 419A0030  beq cr6, 0x824fbf44
	if ctx.cr[6].eq {
	pc = 0x824FBF44; continue 'dispatch;
	}
	// 824FBF18: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 824FBF1C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 824FBF20: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 824FBF24: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824FBF28: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 824FBF2C: 409A0018  bne cr6, 0x824fbf44
	if !ctx.cr[6].eq {
	pc = 0x824FBF44; continue 'dispatch;
	}
	// 824FBF30: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FBF34: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824FBF38: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FBF3C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824FBF40: 4E800421  bctrl
	ctx.lr = 0x824FBF44;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x824FBF44 => {
    //   block [0x824FBF44..0x824FBF70)
	// 824FBF44: 817F0058  lwz r11, 0x58(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 824FBF48: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824FBF4C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824FBF50: 409A0020  bne cr6, 0x824fbf70
	if !ctx.cr[6].eq {
	pc = 0x824FBF70; continue 'dispatch;
	}
	// 824FBF54: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FBF58: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 824FBF5C: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824FBF60: 809F0050  lwz r4, 0x50(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 824FBF64: 55652036  slwi r5, r11, 4
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824FBF68: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824FBF6C: 4BF6814D  bl 0x824640b8
	ctx.lr = 0x824FBF70;
	sub_824640B8(ctx, base);
	pc = 0x824FBF70; continue 'dispatch;
            }
            0x824FBF70 => {
    //   block [0x824FBF70..0x824FBFA8)
	// 824FBF70: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 824FBF74: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824FBF78: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824FBF7C: 409A002C  bne cr6, 0x824fbfa8
	if !ctx.cr[6].eq {
	pc = 0x824FBFA8; continue 'dispatch;
	}
	// 824FBF80: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FBF84: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 824FBF88: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 824FBF8C: 809F0040  lwz r4, 0x40(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 824FBF90: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824FBF94: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824FBF98: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824FBF9C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824FBFA0: 55652036  slwi r5, r11, 4
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824FBFA4: 4BF68115  bl 0x824640b8
	ctx.lr = 0x824FBFA8;
	sub_824640B8(ctx, base);
	pc = 0x824FBFA8; continue 'dispatch;
            }
            0x824FBFA8 => {
    //   block [0x824FBFA8..0x824FBFC8)
	// 824FBFA8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824FBFAC: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 824FBFB0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824FBFB4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824FBFB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824FBFBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824FBFC0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824FBFC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FBFC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824FBFC8 size=228
    let mut pc: u32 = 0x824FBFC8;
    'dispatch: loop {
        match pc {
            0x824FBFC8 => {
    //   block [0x824FBFC8..0x824FC000)
	// 824FBFC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FBFCC: 480390F1  bl 0x825350bc
	ctx.lr = 0x824FBFD0;
	sub_82535080(ctx, base);
	// 824FBFD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FBFD4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824FBFD8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824FBFDC: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 824FBFE0: 83BF004C  lwz r29, 0x4c(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 824FBFE4: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 824FBFE8: 7F0BE800  cmpw cr6, r11, r29
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[29].s32, &mut ctx.xer);
	// 824FBFEC: 40980024  bge cr6, 0x824fc010
	if !ctx.cr[6].lt {
	pc = 0x824FC010; continue 'dispatch;
	}
	// 824FBFF0: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824FBFF4: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824FBFF8: 41980008  blt cr6, 0x824fc000
	if ctx.cr[6].lt {
	pc = 0x824FC000; continue 'dispatch;
	}
	// 824FBFFC: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	pc = 0x824FC000; continue 'dispatch;
            }
            0x824FC000 => {
    //   block [0x824FC000..0x824FC010)
	// 824FC000: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 824FC004: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 824FC008: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824FC00C: 4BF722BD  bl 0x8246e2c8
	ctx.lr = 0x824FC010;
	sub_8246E2C8(ctx, base);
	pc = 0x824FC010; continue 'dispatch;
            }
            0x824FC010 => {
    //   block [0x824FC010..0x824FC0A4)
	// 824FC010: 93BE0004  stw r29, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 824FC014: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824FC018: 815F004C  lwz r10, 0x4c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 824FC01C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824FC020: 40990084  ble cr6, 0x824fc0a4
	if !ctx.cr[6].gt {
	pc = 0x824FC0A4; continue 'dispatch;
	}
	// 824FC024: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 824FC028: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 824FC02C: C00A1FF8  lfs f0, 0x1ff8(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FC030: D001005C  stfs f0, 0x5c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 824FC034: 7D691670  srawi r9, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 824FC038: 815F0040  lwz r10, 0x40(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 824FC03C: 556707BE  clrlwi r7, r11, 0x1e
	ctx.r[7].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 824FC040: 80BE0000  lwz r5, 0(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FC044: 7D290194  addze r9, r9
	tmp.s64 = ctx.r[9].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[9].u32);
	ctx.r[9].s64 = tmp.s64;
	// 824FC048: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824FC04C: 5526083C  slwi r6, r9, 1
	ctx.r[6].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 824FC050: 7D293214  add r9, r9, r6
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[6].u64;
	// 824FC054: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824FC058: 7D293A14  add r9, r9, r7
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[7].u64;
	// 824FC05C: 38E90004  addi r7, r9, 4
	ctx.r[7].s64 = ctx.r[9].s64 + 4;
	// 824FC060: 38C90008  addi r6, r9, 8
	ctx.r[6].s64 = ctx.r[9].s64 + 8;
	// 824FC064: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824FC068: 54E7103A  slwi r7, r7, 2
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 824FC06C: 54C6103A  slwi r6, r6, 2
	ctx.r[6].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 824FC070: 7C09542E  lfsx f0, r9, r10
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FC074: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 824FC078: 7C07542E  lfsx f0, r7, r10
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FC07C: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 824FC080: 7C06542E  lfsx f0, r6, r10
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[6].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FC084: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 824FC088: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	pc = 0x824FC0A4; continue 'dispatch;
            }
            0x824FC0A4 => {
    //   block [0x824FC0A4..0x824FC0AC)
	// 824FC0A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824FC0A8: 48039064  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FC0B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824FC0B0 size=1200
    let mut pc: u32 = 0x824FC0B0;
    'dispatch: loop {
        match pc {
            0x824FC0B0 => {
    //   block [0x824FC0B0..0x824FC104)
	// 824FC0B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FC0B4: 48038FE5  bl 0x82535098
	ctx.lr = 0x824FC0B8;
	sub_82535080(ctx, base);
	// 824FC0B8: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FC0BC: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 824FC0C0: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 824FC0C4: 397C0003  addi r11, r28, 3
	ctx.r[11].s64 = ctx.r[28].s64 + 3;
	// 824FC0C8: 3BFA0040  addi r31, r26, 0x40
	ctx.r[31].s64 = ctx.r[26].s64 + 64;
	// 824FC0CC: 557B003A  rlwinm r27, r11, 0, 0, 0x1d
	ctx.r[27].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824FC0D0: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 824FC0D4: 7F6B1670  srawi r11, r27, 2
	ctx.xer.ca = (ctx.r[27].s32 < 0) && ((ctx.r[27].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[27].s32 >> 2) as i64;
	// 824FC0D8: 939A004C  stw r28, 0x4c(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(76 as u32), ctx.r[28].u32 ) };
	// 824FC0DC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 824FC0E0: 7FAB0194  addze r29, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[29].s64 = tmp.s64;
	// 824FC0E4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824FC0E8: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 824FC0EC: 7F0BE800  cmpw cr6, r11, r29
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[29].s32, &mut ctx.xer);
	// 824FC0F0: 40980024  bge cr6, 0x824fc114
	if !ctx.cr[6].lt {
	pc = 0x824FC114; continue 'dispatch;
	}
	// 824FC0F4: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824FC0F8: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824FC0FC: 41980008  blt cr6, 0x824fc104
	if ctx.cr[6].lt {
	pc = 0x824FC104; continue 'dispatch;
	}
	// 824FC100: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	pc = 0x824FC104; continue 'dispatch;
            }
            0x824FC104 => {
    //   block [0x824FC104..0x824FC114)
	// 824FC104: 38A00030  li r5, 0x30
	ctx.r[5].s64 = 48;
	// 824FC108: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 824FC10C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FC110: 4BF721B9  bl 0x8246e2c8
	ctx.lr = 0x824FC114;
	sub_8246E2C8(ctx, base);
	pc = 0x824FC114; continue 'dispatch;
            }
            0x824FC114 => {
    //   block [0x824FC114..0x824FC130)
	// 824FC114: 7F29CB78  mr r9, r25
	ctx.r[9].u64 = ctx.r[25].u64;
	// 824FC118: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 824FC11C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824FC120: 2F1C0004  cmpwi cr6, r28, 4
	ctx.cr[6].compare_i32(ctx.r[28].s32, 4, &mut ctx.xer);
	// 824FC124: 4198017C  blt cr6, 0x824fc2a0
	if ctx.cr[6].lt {
	pc = 0x824FC2A0; continue 'dispatch;
	}
	// 824FC128: 387CFFFD  addi r3, r28, -3
	ctx.r[3].s64 = ctx.r[28].s64 + -3;
	// 824FC12C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	pc = 0x824FC130; continue 'dispatch;
            }
            0x824FC130 => {
    //   block [0x824FC130..0x824FC2A0)
	// 824FC130: 5568F0BE  srwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shr(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 824FC134: 83BF0000  lwz r29, 0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FC138: 388A0001  addi r4, r10, 1
	ctx.r[4].s64 = ctx.r[10].s64 + 1;
	// 824FC13C: C0090000  lfs f0, 0(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FC140: 5506083C  slwi r6, r8, 1
	ctx.r[6].u32 = ctx.r[8].u32.wrapping_shl(1);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 824FC144: 5547F0BE  srwi r7, r10, 2
	ctx.r[7].u32 = ctx.r[10].u32.wrapping_shr(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 824FC148: 7D083214  add r8, r8, r6
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[6].u64;
	// 824FC14C: 5486F0BE  srwi r6, r4, 2
	ctx.r[6].u32 = ctx.r[4].u32.wrapping_shr(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 824FC150: 5508103A  slwi r8, r8, 2
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 824FC154: 556507BE  clrlwi r5, r11, 0x1e
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 824FC158: 54E4083C  slwi r4, r7, 1
	ctx.r[4].u32 = ctx.r[7].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 824FC15C: 3AEA0002  addi r23, r10, 2
	ctx.r[23].s64 = ctx.r[10].s64 + 2;
	// 824FC160: 7D082A14  add r8, r8, r5
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[5].u64;
	// 824FC164: 7C872214  add r4, r7, r4
	ctx.r[4].u64 = ctx.r[7].u64 + ctx.r[4].u64;
	// 824FC168: 56E7F0BE  srwi r7, r23, 2
	ctx.r[7].u32 = ctx.r[23].u32.wrapping_shr(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 824FC16C: 5517103A  slwi r23, r8, 2
	ctx.r[23].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[23].u64 = ctx.r[23].u32 as u64;
	// 824FC170: 3AA80004  addi r21, r8, 4
	ctx.r[21].s64 = ctx.r[8].s64 + 4;
	// 824FC174: 5484103A  slwi r4, r4, 2
	ctx.r[4].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 824FC178: 56B5103A  slwi r21, r21, 2
	ctx.r[21].u32 = ctx.r[21].u32.wrapping_shl(2);
	ctx.r[21].u64 = ctx.r[21].u32 as u64;
	// 824FC17C: 554507BE  clrlwi r5, r10, 0x1e
	ctx.r[5].u64 = ctx.r[10].u32 as u64 & 0x00000003u64;
	// 824FC180: 7C17ED2E  stfsx f0, r23, r29
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[23].u32.wrapping_add(ctx.r[29].u32), tmp.u32) };
	// 824FC184: 83BF0000  lwz r29, 0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FC188: 3A880008  addi r20, r8, 8
	ctx.r[20].s64 = ctx.r[8].s64 + 8;
	// 824FC18C: C0090004  lfs f0, 4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FC190: 7D042A14  add r8, r4, r5
	ctx.r[8].u64 = ctx.r[4].u64 + ctx.r[5].u64;
	// 824FC194: 5684103A  slwi r4, r20, 2
	ctx.r[4].u32 = ctx.r[20].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 824FC198: 3B0A0001  addi r24, r10, 1
	ctx.r[24].s64 = ctx.r[10].s64 + 1;
	// 824FC19C: 7C15ED2E  stfsx f0, r21, r29
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[21].u32.wrapping_add(ctx.r[29].u32), tmp.u32) };
	// 824FC1A0: 83BF0000  lwz r29, 0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FC1A4: C0090008  lfs f0, 8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FC1A8: 7D29F214  add r9, r9, r30
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[30].u64;
	// 824FC1AC: 570507BE  clrlwi r5, r24, 0x1e
	ctx.r[5].u64 = ctx.r[24].u32 as u64 & 0x00000003u64;
	// 824FC1B0: 3B080004  addi r24, r8, 4
	ctx.r[24].s64 = ctx.r[8].s64 + 4;
	// 824FC1B4: 3AE80008  addi r23, r8, 8
	ctx.r[23].s64 = ctx.r[8].s64 + 8;
	// 824FC1B8: 7C04ED2E  stfsx f0, r4, r29
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[4].u32.wrapping_add(ctx.r[29].u32), tmp.u32) };
	// 824FC1BC: 54C4083C  slwi r4, r6, 1
	ctx.r[4].u32 = ctx.r[6].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 824FC1C0: 83BF0000  lwz r29, 0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FC1C4: C0090000  lfs f0, 0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FC1C8: 7CC62214  add r6, r6, r4
	ctx.r[6].u64 = ctx.r[6].u64 + ctx.r[4].u64;
	// 824FC1CC: 5504103A  slwi r4, r8, 2
	ctx.r[4].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 824FC1D0: 54C6103A  slwi r6, r6, 2
	ctx.r[6].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 824FC1D4: 5718103A  slwi r24, r24, 2
	ctx.r[24].u32 = ctx.r[24].u32.wrapping_shl(2);
	ctx.r[24].u64 = ctx.r[24].u32 as u64;
	// 824FC1D8: 7D062A14  add r8, r6, r5
	ctx.r[8].u64 = ctx.r[6].u64 + ctx.r[5].u64;
	// 824FC1DC: 3ACAFFFE  addi r22, r10, -2
	ctx.r[22].s64 = ctx.r[10].s64 + -2;
	// 824FC1E0: 7C04ED2E  stfsx f0, r4, r29
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[4].u32.wrapping_add(ctx.r[29].u32), tmp.u32) };
	// 824FC1E4: 80BF0000  lwz r5, 0(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FC1E8: C0090004  lfs f0, 4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FC1EC: 56E4103A  slwi r4, r23, 2
	ctx.r[4].u32 = ctx.r[23].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 824FC1F0: 3BA80004  addi r29, r8, 4
	ctx.r[29].s64 = ctx.r[8].s64 + 4;
	// 824FC1F4: 56C607BE  clrlwi r6, r22, 0x1e
	ctx.r[6].u64 = ctx.r[22].u32 as u64 & 0x00000003u64;
	// 824FC1F8: 57BD103A  slwi r29, r29, 2
	ctx.r[29].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 824FC1FC: 7C182D2E  stfsx f0, r24, r5
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[24].u32.wrapping_add(ctx.r[5].u32), tmp.u32) };
	// 824FC200: 80BF0000  lwz r5, 0(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FC204: C0090008  lfs f0, 8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FC208: 7D29F214  add r9, r9, r30
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[30].u64;
	// 824FC20C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 824FC210: 7C042D2E  stfsx f0, r4, r5
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[4].u32.wrapping_add(ctx.r[5].u32), tmp.u32) };
	// 824FC214: 54E5083C  slwi r5, r7, 1
	ctx.r[5].u32 = ctx.r[7].u32.wrapping_shl(1);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824FC218: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FC21C: C0090000  lfs f0, 0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FC220: 7CE72A14  add r7, r7, r5
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[5].u64;
	// 824FC224: 5505103A  slwi r5, r8, 2
	ctx.r[5].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824FC228: 39080008  addi r8, r8, 8
	ctx.r[8].s64 = ctx.r[8].s64 + 8;
	// 824FC22C: 54E7103A  slwi r7, r7, 2
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 824FC230: 5518103A  slwi r24, r8, 2
	ctx.r[24].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[24].u64 = ctx.r[24].u32 as u64;
	// 824FC234: 7D073214  add r8, r7, r6
	ctx.r[8].u64 = ctx.r[7].u64 + ctx.r[6].u64;
	// 824FC238: 7C05252E  stfsx f0, r5, r4
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[5].u32.wrapping_add(ctx.r[4].u32), tmp.u32) };
	// 824FC23C: 80FF0000  lwz r7, 0(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FC240: C0090004  lfs f0, 4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FC244: 38A80004  addi r5, r8, 4
	ctx.r[5].s64 = ctx.r[8].s64 + 4;
	// 824FC248: 5506103A  slwi r6, r8, 2
	ctx.r[6].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 824FC24C: 39080008  addi r8, r8, 8
	ctx.r[8].s64 = ctx.r[8].s64 + 8;
	// 824FC250: 54A5103A  slwi r5, r5, 2
	ctx.r[5].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824FC254: 7C1D3D2E  stfsx f0, r29, r7
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[29].u32.wrapping_add(ctx.r[7].u32), tmp.u32) };
	// 824FC258: 80FF0000  lwz r7, 0(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FC25C: C0090008  lfs f0, 8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FC260: 7D29F214  add r9, r9, r30
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[30].u64;
	// 824FC264: 5508103A  slwi r8, r8, 2
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 824FC268: 7C183D2E  stfsx f0, r24, r7
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[24].u32.wrapping_add(ctx.r[7].u32), tmp.u32) };
	// 824FC26C: 80FF0000  lwz r7, 0(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FC270: C0090000  lfs f0, 0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FC274: 7C063D2E  stfsx f0, r6, r7
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[6].u32.wrapping_add(ctx.r[7].u32), tmp.u32) };
	// 824FC278: 80FF0000  lwz r7, 0(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FC27C: C0090004  lfs f0, 4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FC280: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 824FC284: 7F0B1800  cmpw cr6, r11, r3
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[3].s32, &mut ctx.xer);
	// 824FC288: 7C053D2E  stfsx f0, r5, r7
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[5].u32.wrapping_add(ctx.r[7].u32), tmp.u32) };
	// 824FC28C: 80FF0000  lwz r7, 0(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FC290: C0090008  lfs f0, 8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FC294: 7D29F214  add r9, r9, r30
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[30].u64;
	// 824FC298: 7C083D2E  stfsx f0, r8, r7
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[7].u32), tmp.u32) };
	// 824FC29C: 4198FE94  blt cr6, 0x824fc130
	if ctx.cr[6].lt {
	pc = 0x824FC130; continue 'dispatch;
	}
	pc = 0x824FC2A0; continue 'dispatch;
            }
            0x824FC2A0 => {
    //   block [0x824FC2A0..0x824FC2A8)
	// 824FC2A0: 7F0BE000  cmpw cr6, r11, r28
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[28].s32, &mut ctx.xer);
	// 824FC2A4: 40980064  bge cr6, 0x824fc308
	if !ctx.cr[6].lt {
	pc = 0x824FC308; continue 'dispatch;
	}
	pc = 0x824FC2A8; continue 'dispatch;
            }
            0x824FC2A8 => {
    //   block [0x824FC2A8..0x824FC308)
	// 824FC2A8: 556AF0BE  srwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shr(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824FC2AC: 80DF0000  lwz r6, 0(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FC2B0: 556807BE  clrlwi r8, r11, 0x1e
	ctx.r[8].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 824FC2B4: C0090000  lfs f0, 0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FC2B8: 5547083C  slwi r7, r10, 1
	ctx.r[7].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 824FC2BC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824FC2C0: 7D4A3A14  add r10, r10, r7
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[7].u64;
	// 824FC2C4: 7F0BE000  cmpw cr6, r11, r28
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[28].s32, &mut ctx.xer);
	// 824FC2C8: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824FC2CC: 7D4A4214  add r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 824FC2D0: 5548103A  slwi r8, r10, 2
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 824FC2D4: 38EA0004  addi r7, r10, 4
	ctx.r[7].s64 = ctx.r[10].s64 + 4;
	// 824FC2D8: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 824FC2DC: 54E7103A  slwi r7, r7, 2
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 824FC2E0: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824FC2E4: 7C08352E  stfsx f0, r8, r6
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[6].u32), tmp.u32) };
	// 824FC2E8: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FC2EC: C0090004  lfs f0, 4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FC2F0: 7C07452E  stfsx f0, r7, r8
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[8].u32), tmp.u32) };
	// 824FC2F4: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FC2F8: C0090008  lfs f0, 8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FC2FC: 7D29F214  add r9, r9, r30
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[30].u64;
	// 824FC300: 7C0A452E  stfsx f0, r10, r8
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32), tmp.u32) };
	// 824FC304: 4198FFA4  blt cr6, 0x824fc2a8
	if ctx.cr[6].lt {
	pc = 0x824FC2A8; continue 'dispatch;
	}
	pc = 0x824FC308; continue 'dispatch;
            }
            0x824FC308 => {
    //   block [0x824FC308..0x824FC320)
	// 824FC308: 7D0BD850  subf r8, r11, r27
	ctx.r[8].s64 = ctx.r[27].s64 - ctx.r[11].s64;
	// 824FC30C: 7D5E4850  subf r10, r30, r9
	ctx.r[10].s64 = ctx.r[9].s64 - ctx.r[30].s64;
	// 824FC310: 2F080004  cmpwi cr6, r8, 4
	ctx.cr[6].compare_i32(ctx.r[8].s32, 4, &mut ctx.xer);
	// 824FC314: 4198016C  blt cr6, 0x824fc480
	if ctx.cr[6].lt {
	pc = 0x824FC480; continue 'dispatch;
	}
	// 824FC318: 3BBBFFFD  addi r29, r27, -3
	ctx.r[29].s64 = ctx.r[27].s64 + -3;
	// 824FC31C: 392B0001  addi r9, r11, 1
	ctx.r[9].s64 = ctx.r[11].s64 + 1;
	pc = 0x824FC320; continue 'dispatch;
            }
            0x824FC320 => {
    //   block [0x824FC320..0x824FC480)
	// 824FC320: 7D681670  srawi r8, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 824FC324: 831F0000  lwz r24, 0(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FC328: 7D271670  srawi r7, r9, 2
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[7].s64 = (ctx.r[9].s32 >> 2) as i64;
	// 824FC32C: C00A0000  lfs f0, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FC330: 5504083C  slwi r4, r8, 1
	ctx.r[4].u32 = ctx.r[8].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 824FC334: 556507BE  clrlwi r5, r11, 0x1e
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 824FC338: 7D082214  add r8, r8, r4
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[4].u64;
	// 824FC33C: 54E3083C  slwi r3, r7, 1
	ctx.r[3].u32 = ctx.r[7].u32.wrapping_shl(1);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 824FC340: 5508103A  slwi r8, r8, 2
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 824FC344: 7CE71A14  add r7, r7, r3
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[3].u64;
	// 824FC348: 7D082A14  add r8, r8, r5
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[5].u64;
	// 824FC34C: 38C90001  addi r6, r9, 1
	ctx.r[6].s64 = ctx.r[9].s64 + 1;
	// 824FC350: 5503103A  slwi r3, r8, 2
	ctx.r[3].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 824FC354: 3AE90002  addi r23, r9, 2
	ctx.r[23].s64 = ctx.r[9].s64 + 2;
	// 824FC358: 7CC61670  srawi r6, r6, 2
	ctx.xer.ca = (ctx.r[6].s32 < 0) && ((ctx.r[6].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[6].s64 = (ctx.r[6].s32 >> 2) as i64;
	// 824FC35C: 54E7103A  slwi r7, r7, 2
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 824FC360: 7EE51670  srawi r5, r23, 2
	ctx.xer.ca = (ctx.r[23].s32 < 0) && ((ctx.r[23].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[5].s64 = (ctx.r[23].s32 >> 2) as i64;
	// 824FC364: 552407BE  clrlwi r4, r9, 0x1e
	ctx.r[4].u64 = ctx.r[9].u32 as u64 & 0x00000003u64;
	// 824FC368: 7C03C52E  stfsx f0, r3, r24
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[3].u32.wrapping_add(ctx.r[24].u32), tmp.u32) };
	// 824FC36C: 3AE80004  addi r23, r8, 4
	ctx.r[23].s64 = ctx.r[8].s64 + 4;
	// 824FC370: C00A0004  lfs f0, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FC374: 3A880008  addi r20, r8, 8
	ctx.r[20].s64 = ctx.r[8].s64 + 8;
	// 824FC378: 7D072214  add r8, r7, r4
	ctx.r[8].u64 = ctx.r[7].u64 + ctx.r[4].u64;
	// 824FC37C: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FC380: 56E3103A  slwi r3, r23, 2
	ctx.r[3].u32 = ctx.r[23].u32.wrapping_shl(2);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 824FC384: 5698103A  slwi r24, r20, 2
	ctx.r[24].u32 = ctx.r[20].u32.wrapping_shl(2);
	ctx.r[24].u64 = ctx.r[24].u32 as u64;
	// 824FC388: 3AC90001  addi r22, r9, 1
	ctx.r[22].s64 = ctx.r[9].s64 + 1;
	// 824FC38C: 3AE80008  addi r23, r8, 8
	ctx.r[23].s64 = ctx.r[8].s64 + 8;
	// 824FC390: 56C707BE  clrlwi r7, r22, 0x1e
	ctx.r[7].u64 = ctx.r[22].u32 as u64 & 0x00000003u64;
	// 824FC394: 7C03252E  stfsx f0, r3, r4
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[3].u32.wrapping_add(ctx.r[4].u32), tmp.u32) };
	// 824FC398: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FC39C: C00A0008  lfs f0, 8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FC3A0: 3AA9FFFE  addi r21, r9, -2
	ctx.r[21].s64 = ctx.r[9].s64 + -2;
	// 824FC3A4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 824FC3A8: 7C18252E  stfsx f0, r24, r4
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[24].u32.wrapping_add(ctx.r[4].u32), tmp.u32) };
	// 824FC3AC: 54C4083C  slwi r4, r6, 1
	ctx.r[4].u32 = ctx.r[6].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 824FC3B0: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FC3B4: 3B080004  addi r24, r8, 4
	ctx.r[24].s64 = ctx.r[8].s64 + 4;
	// 824FC3B8: 7CC62214  add r6, r6, r4
	ctx.r[6].u64 = ctx.r[6].u64 + ctx.r[4].u64;
	// 824FC3BC: C00A0000  lfs f0, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FC3C0: 5504103A  slwi r4, r8, 2
	ctx.r[4].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 824FC3C4: 54C6103A  slwi r6, r6, 2
	ctx.r[6].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 824FC3C8: 7D063A14  add r8, r6, r7
	ctx.r[8].u64 = ctx.r[6].u64 + ctx.r[7].u64;
	// 824FC3CC: 56A707BE  clrlwi r7, r21, 0x1e
	ctx.r[7].u64 = ctx.r[21].u32 as u64 & 0x00000003u64;
	// 824FC3D0: 7C041D2E  stfsx f0, r4, r3
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[4].u32.wrapping_add(ctx.r[3].u32), tmp.u32) };
	// 824FC3D4: 80DF0000  lwz r6, 0(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FC3D8: 5704103A  slwi r4, r24, 2
	ctx.r[4].u32 = ctx.r[24].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 824FC3DC: C00A0004  lfs f0, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FC3E0: 56E3103A  slwi r3, r23, 2
	ctx.r[3].u32 = ctx.r[23].u32.wrapping_shl(2);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 824FC3E4: 3B080008  addi r24, r8, 8
	ctx.r[24].s64 = ctx.r[8].s64 + 8;
	// 824FC3E8: 7C04352E  stfsx f0, r4, r6
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[4].u32.wrapping_add(ctx.r[6].u32), tmp.u32) };
	// 824FC3EC: 80DF0000  lwz r6, 0(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FC3F0: C00A0008  lfs f0, 8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FC3F4: 7C03352E  stfsx f0, r3, r6
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[3].u32.wrapping_add(ctx.r[6].u32), tmp.u32) };
	// 824FC3F8: 54A6083C  slwi r6, r5, 1
	ctx.r[6].u32 = ctx.r[5].u32.wrapping_shl(1);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 824FC3FC: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FC400: 38680004  addi r3, r8, 4
	ctx.r[3].s64 = ctx.r[8].s64 + 4;
	// 824FC404: 7CC53214  add r6, r5, r6
	ctx.r[6].u64 = ctx.r[5].u64 + ctx.r[6].u64;
	// 824FC408: C00A0000  lfs f0, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FC40C: 5505103A  slwi r5, r8, 2
	ctx.r[5].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824FC410: 54C6103A  slwi r6, r6, 2
	ctx.r[6].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 824FC414: 7D063A14  add r8, r6, r7
	ctx.r[8].u64 = ctx.r[6].u64 + ctx.r[7].u64;
	// 824FC418: 5467103A  slwi r7, r3, 2
	ctx.r[7].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 824FC41C: 7C05252E  stfsx f0, r5, r4
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[5].u32.wrapping_add(ctx.r[4].u32), tmp.u32) };
	// 824FC420: 80DF0000  lwz r6, 0(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FC424: C00A0004  lfs f0, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FC428: 5705103A  slwi r5, r24, 2
	ctx.r[5].u32 = ctx.r[24].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824FC42C: 7C07352E  stfsx f0, r7, r6
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[6].u32), tmp.u32) };
	// 824FC430: 80FF0000  lwz r7, 0(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FC434: C00A0008  lfs f0, 8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FC438: 7C053D2E  stfsx f0, r5, r7
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[5].u32.wrapping_add(ctx.r[7].u32), tmp.u32) };
	// 824FC43C: 80DF0000  lwz r6, 0(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FC440: 5507103A  slwi r7, r8, 2
	ctx.r[7].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 824FC444: C00A0000  lfs f0, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FC448: 38A80004  addi r5, r8, 4
	ctx.r[5].s64 = ctx.r[8].s64 + 4;
	// 824FC44C: 39080008  addi r8, r8, 8
	ctx.r[8].s64 = ctx.r[8].s64 + 8;
	// 824FC450: 54A5103A  slwi r5, r5, 2
	ctx.r[5].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824FC454: 5508103A  slwi r8, r8, 2
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 824FC458: 7C07352E  stfsx f0, r7, r6
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[6].u32), tmp.u32) };
	// 824FC45C: 80FF0000  lwz r7, 0(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FC460: C00A0004  lfs f0, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FC464: 7C053D2E  stfsx f0, r5, r7
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[5].u32.wrapping_add(ctx.r[7].u32), tmp.u32) };
	// 824FC468: 80FF0000  lwz r7, 0(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FC46C: C00A0008  lfs f0, 8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FC470: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 824FC474: 7F0BE800  cmpw cr6, r11, r29
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[29].s32, &mut ctx.xer);
	// 824FC478: 7C083D2E  stfsx f0, r8, r7
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[7].u32), tmp.u32) };
	// 824FC47C: 4198FEA4  blt cr6, 0x824fc320
	if ctx.cr[6].lt {
	pc = 0x824FC320; continue 'dispatch;
	}
	pc = 0x824FC480; continue 'dispatch;
            }
            0x824FC480 => {
    //   block [0x824FC480..0x824FC488)
	// 824FC480: 7F0BD800  cmpw cr6, r11, r27
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[27].s32, &mut ctx.xer);
	// 824FC484: 40980060  bge cr6, 0x824fc4e4
	if !ctx.cr[6].lt {
	pc = 0x824FC4E4; continue 'dispatch;
	}
	pc = 0x824FC488; continue 'dispatch;
            }
            0x824FC488 => {
    //   block [0x824FC488..0x824FC4E4)
	// 824FC488: 7D691670  srawi r9, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 824FC48C: 80DF0000  lwz r6, 0(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FC490: 556807BE  clrlwi r8, r11, 0x1e
	ctx.r[8].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 824FC494: C00A0000  lfs f0, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FC498: 5527083C  slwi r7, r9, 1
	ctx.r[7].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 824FC49C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824FC4A0: 7D293A14  add r9, r9, r7
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[7].u64;
	// 824FC4A4: 7F0BD800  cmpw cr6, r11, r27
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[27].s32, &mut ctx.xer);
	// 824FC4A8: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824FC4AC: 7D294214  add r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 824FC4B0: 5528103A  slwi r8, r9, 2
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 824FC4B4: 38E90004  addi r7, r9, 4
	ctx.r[7].s64 = ctx.r[9].s64 + 4;
	// 824FC4B8: 39290008  addi r9, r9, 8
	ctx.r[9].s64 = ctx.r[9].s64 + 8;
	// 824FC4BC: 54E7103A  slwi r7, r7, 2
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 824FC4C0: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824FC4C4: 7C08352E  stfsx f0, r8, r6
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[6].u32), tmp.u32) };
	// 824FC4C8: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FC4CC: C00A0004  lfs f0, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FC4D0: 7C07452E  stfsx f0, r7, r8
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[8].u32), tmp.u32) };
	// 824FC4D4: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FC4D8: C00A0008  lfs f0, 8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FC4DC: 7C09452E  stfsx f0, r9, r8
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[8].u32), tmp.u32) };
	// 824FC4E0: 4198FFA8  blt cr6, 0x824fc488
	if ctx.cr[6].lt {
	pc = 0x824FC488; continue 'dispatch;
	}
	pc = 0x824FC4E4; continue 'dispatch;
            }
            0x824FC4E4 => {
    //   block [0x824FC4E4..0x824FC560)
	// 824FC4E4: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 824FC4E8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 824FC4EC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 824FC4F0: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 824FC4F4: 4800488D  bl 0x82500d80
	ctx.lr = 0x824FC4F8;
	sub_82500D80(ctx, base);
	// 824FC4F8: 39210070  addi r9, r1, 0x70
	ctx.r[9].s64 = ctx.r[1].s64 + 112;
	// 824FC4FC: 39010050  addi r8, r1, 0x50
	ctx.r[8].s64 = ctx.r[1].s64 + 80;
	// 824FC500: 397A0030  addi r11, r26, 0x30
	ctx.r[11].s64 = ctx.r[26].s64 + 48;
	// 824FC504: 395A0020  addi r10, r26, 0x20
	ctx.r[10].s64 = ctx.r[26].s64 + 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FC580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824FC580 size=444
    let mut pc: u32 = 0x824FC580;
    'dispatch: loop {
        match pc {
            0x824FC580 => {
    //   block [0x824FC580..0x824FC5C0)
	// 824FC580: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 824FC584: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FC588: 39600014  li r11, 0x14
	ctx.r[11].s64 = 20;
	// 824FC58C: 7D0B5214  add r8, r11, r10
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824FC590: 81680000  lwz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FC594: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FC598: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 824FC59C: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 824FC5A0: 40980020  bge cr6, 0x824fc5c0
	if !ctx.cr[6].lt {
	pc = 0x824FC5C0; continue 'dispatch;
	}
	// 824FC5A4: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 824FC5A8: 39293FC0  addi r9, r9, 0x3fc0
	ctx.r[9].s64 = ctx.r[9].s64 + 16320;
	// 824FC5AC: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 824FC5B0: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 824FC5B4: 38EA000C  addi r7, r10, 0xc
	ctx.r[7].s64 = ctx.r[10].s64 + 12;
	// 824FC5B8: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 824FC5BC: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	pc = 0x824FC5C0; continue 'dispatch;
            }
            0x824FC5C0 => {
    //   block [0x824FC5C0..0x824FC73C)
	// 824FC5C0: 3941FFE0  addi r10, r1, -0x20
	ctx.r[10].s64 = ctx.r[1].s64 + -32;
	// 824FC5C4: 81640054  lwz r11, 0x54(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(84 as u32) ) } as u64;
	// 824FC5C8: C1860010  lfs f12, 0x10(r6)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(16 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 824FC5CC: 38E60010  addi r7, r6, 0x10
	ctx.r[7].s64 = ctx.r[6].s64 + 16;
	// 824FC5D0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 824FC5D4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FC740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824FC740 size=232
    let mut pc: u32 = 0x824FC740;
    'dispatch: loop {
        match pc {
            0x824FC740 => {
    //   block [0x824FC740..0x824FC7D0)
	// 824FC740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FC744: 48038975  bl 0x825350b8
	ctx.lr = 0x824FC748;
	sub_82535080(ctx, base);
	// 824FC748: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FC74C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824FC750: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824FC754: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824FC758: 388B3FD8  addi r4, r11, 0x3fd8
	ctx.r[4].s64 = ctx.r[11].s64 + 16344;
	// 824FC75C: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 824FC760: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FC764: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 824FC768: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824FC76C: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FC770: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824FC774: 4E800421  bctrl
	ctx.lr = 0x824FC778;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824FC778: 815F0048  lwz r10, 0x48(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 824FC77C: 554B0000  rlwinm r11, r10, 0, 0, 0
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 824FC780: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824FC784: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824FC788: 3BAB3FD0  addi r29, r11, 0x3fd0
	ctx.r[29].s64 = ctx.r[11].s64 + 16336;
	// 824FC78C: 409A0044  bne cr6, 0x824fc7d0
	if !ctx.cr[6].eq {
	pc = 0x824FC7D0; continue 'dispatch;
	}
	// 824FC790: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FC794: 554A00BE  clrlwi r10, r10, 2
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 824FC798: 817F0044  lwz r11, 0x44(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 824FC79C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 824FC7A0: 5548083C  slwi r8, r10, 1
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 824FC7A4: 80DF0040  lwz r6, 0x40(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 824FC7A8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 824FC7AC: 7D4A4214  add r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 824FC7B0: 83890008  lwz r28, 8(r9)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 824FC7B4: 5569083C  slwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824FC7B8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824FC7BC: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 824FC7C0: 55482036  slwi r8, r10, 4
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 824FC7C4: 55672036  slwi r7, r11, 4
	ctx.r[7].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 824FC7C8: 7F8903A6  mtctr r28
	ctx.ctr.u64 = ctx.r[28].u64;
	// 824FC7CC: 4E800421  bctrl
	ctx.lr = 0x824FC7D0;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x824FC7D0 => {
    //   block [0x824FC7D0..0x824FC80C)
	// 824FC7D0: 817F0058  lwz r11, 0x58(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 824FC7D4: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824FC7D8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824FC7DC: 409A0030  bne cr6, 0x824fc80c
	if !ctx.cr[6].eq {
	pc = 0x824FC80C; continue 'dispatch;
	}
	// 824FC7E0: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FC7E4: 55682036  slwi r8, r11, 4
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 824FC7E8: 813F0054  lwz r9, 0x54(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 824FC7EC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 824FC7F0: 80DF0050  lwz r6, 0x50(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 824FC7F4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 824FC7F8: 55272036  slwi r7, r9, 4
	ctx.r[7].u32 = ctx.r[9].u32.wrapping_shl(4);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 824FC7FC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824FC800: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 824FC804: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824FC808: 4E800421  bctrl
	ctx.lr = 0x824FC80C;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x824FC80C => {
    //   block [0x824FC80C..0x824FC828)
	// 824FC80C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FC810: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824FC814: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 824FC818: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824FC81C: 4E800421  bctrl
	ctx.lr = 0x824FC820;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824FC820: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824FC824: 480388E4  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FC828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824FC828 size=296
    let mut pc: u32 = 0x824FC828;
    'dispatch: loop {
        match pc {
            0x824FC828 => {
    //   block [0x824FC828..0x824FC8C8)
	// 824FC828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FC82C: 4803888D  bl 0x825350b8
	ctx.lr = 0x824FC830;
	sub_82535080(ctx, base);
	// 824FC830: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FC834: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824FC838: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 824FC83C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824FC840: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 824FC844: 394B3804  addi r10, r11, 0x3804
	ctx.r[10].s64 = ctx.r[11].s64 + 14340;
	// 824FC848: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824FC84C: D03F0010  stfs f1, 0x10(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 824FC850: B13F0006  sth r9, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 824FC854: 3D208000  lis r9, -0x8000
	ctx.r[9].s64 = -2147483648;
	// 824FC858: 3BDF0050  addi r30, r31, 0x50
	ctx.r[30].s64 = ctx.r[31].s64 + 80;
	// 824FC85C: 911F000C  stw r8, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 824FC860: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 824FC864: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824FC868: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 824FC86C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 824FC870: 917F0040  stw r11, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 824FC874: 917F0044  stw r11, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 824FC878: 913F0048  stw r9, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[9].u32 ) };
	// 824FC87C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824FC880: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 824FC884: 913E0008  stw r9, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 824FC888: 917F005C  stw r11, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 824FC88C: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 824FC890: 813D0004  lwz r9, 4(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FC894: 556A00BE  clrlwi r10, r11, 2
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 824FC898: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 824FC89C: 40980060  bge cr6, 0x824fc8fc
	if !ctx.cr[6].lt {
	pc = 0x824FC8FC; continue 'dispatch;
	}
	// 824FC8A0: 556B0000  rlwinm r11, r11, 0, 0, 0
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824FC8A4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824FC8A8: 409A0020  bne cr6, 0x824fc8c8
	if !ctx.cr[6].eq {
	pc = 0x824FC8C8; continue 'dispatch;
	}
	// 824FC8AC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FC8B0: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 824FC8B4: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 824FC8B8: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FC8BC: 55452036  slwi r5, r10, 4
	ctx.r[5].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824FC8C0: 7C69582E  lwzx r3, r9, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824FC8C4: 4BF677F5  bl 0x824640b8
	ctx.lr = 0x824FC8C8;
	sub_824640B8(ctx, base);
	pc = 0x824FC8C8; continue 'dispatch;
            }
            0x824FC8C8 => {
    //   block [0x824FC8C8..0x824FC8FC)
	// 824FC8C8: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FC8CC: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 824FC8D0: 813D0004  lwz r9, 4(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FC8D4: 38A00017  li r5, 0x17
	ctx.r[5].s64 = 23;
	// 824FC8D8: 55242036  slwi r4, r9, 4
	ctx.r[4].u32 = ctx.r[9].u32.wrapping_shl(4);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 824FC8DC: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824FC8E0: 4BF67759  bl 0x82464038
	ctx.lr = 0x824FC8E4;
	sub_82464038(ctx, base);
	// 824FC8E4: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 824FC8E8: 907E0000  stw r3, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 824FC8EC: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FC8F0: 556B0042  rlwinm r11, r11, 0, 1, 1
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824FC8F4: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 824FC8F8: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	pc = 0x824FC8FC; continue 'dispatch;
            }
            0x824FC8FC => {
    //   block [0x824FC8FC..0x824FC930)
	// 824FC8FC: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FC900: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FC904: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824FC908: 915E0004  stw r10, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 824FC90C: 813D0000  lwz r9, 0(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FC910: 40990020  ble cr6, 0x824fc930
	if !ctx.cr[6].gt {
	pc = 0x824FC930; continue 'dispatch;
	}
	// 824FC914: 7D2B4850  subf r9, r11, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 824FC918: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	pc = 0x824FC930; continue 'dispatch;
            }
            0x824FC930 => {
    //   block [0x824FC930..0x824FC950)
	// 824FC930: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FC934: 80DC0004  lwz r6, 4(r28)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FC938: 80BC0008  lwz r5, 8(r28)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 824FC93C: 809C0000  lwz r4, 0(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FC940: 4BFFF771  bl 0x824fc0b0
	ctx.lr = 0x824FC944;
	sub_824FC0B0(ctx, base);
	// 824FC944: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FC948: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824FC94C: 480387BC  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FC950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FC950 size=176
    let mut pc: u32 = 0x824FC950;
    'dispatch: loop {
        match pc {
            0x824FC950 => {
    //   block [0x824FC950..0x824FCA00)
	// 824FC950: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FCA00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824FCA00 size=124
    let mut pc: u32 = 0x824FCA00;
    'dispatch: loop {
        match pc {
            0x824FCA00 => {
    //   block [0x824FCA00..0x824FCA7C)
	// 824FCA00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FCA04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824FCA08: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824FCA0C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FCA10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 824FCA14: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 824FCA18: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824FCA1C: 99410050  stb r10, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u8 ) };
	// 824FCA20: 888B0000  lbz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FCA24: 48002705  bl 0x824ff128
	ctx.lr = 0x824FCA28;
	sub_824FF128(ctx, base);
	// 824FCA28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FCA2C: 48019B2D  bl 0x82516558
	ctx.lr = 0x824FCA30;
	sub_82516558(ctx, base);
	// 824FCA30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FCA34: 480171F5  bl 0x82513c28
	ctx.lr = 0x824FCA38;
	sub_82513C28(ctx, base);
	// 824FCA38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FCA3C: 48016495  bl 0x82512ed0
	ctx.lr = 0x824FCA40;
	sub_82512ED0(ctx, base);
	// 824FCA40: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FCA44: 48015945  bl 0x82512388
	ctx.lr = 0x824FCA48;
	sub_82512388(ctx, base);
	// 824FCA48: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FCA4C: 480B7245  bl 0x825b3c90
	ctx.lr = 0x824FCA50;
	sub_825B3C90(ctx, base);
	// 824FCA50: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 824FCA54: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 824FCA58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FCA5C: 99410050  stb r10, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u8 ) };
	// 824FCA60: 888B0000  lbz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FCA64: 480026C5  bl 0x824ff128
	ctx.lr = 0x824FCA68;
	sub_824FF128(ctx, base);
	// 824FCA68: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824FCA6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824FCA70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824FCA74: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824FCA78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FCA80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824FCA80 size=76
    let mut pc: u32 = 0x824FCA80;
    'dispatch: loop {
        match pc {
            0x824FCA80 => {
    //   block [0x824FCA80..0x824FCACC)
	// 824FCA80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FCA84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824FCA88: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824FCA8C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FCA90: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824FCA94: 4801D955  bl 0x8251a3e8
	ctx.lr = 0x824FCA98;
	sub_8251A3E8(ctx, base);
	// 824FCA98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FCA9C: 4801C6CD  bl 0x82519168
	ctx.lr = 0x824FCAA0;
	sub_82519168(ctx, base);
	// 824FCAA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FCAA4: 480B7D9D  bl 0x825b4840
	ctx.lr = 0x824FCAA8;
	sub_825B4840(ctx, base);
	// 824FCAA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FCAAC: 4801AA95  bl 0x82517540
	ctx.lr = 0x824FCAB0;
	sub_82517540(ctx, base);
	// 824FCAB0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FCAB4: 480B75CD  bl 0x825b4080
	ctx.lr = 0x824FCAB8;
	sub_825B4080(ctx, base);
	// 824FCAB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824FCABC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824FCAC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824FCAC4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824FCAC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FCAD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824FCAD0 size=164
    let mut pc: u32 = 0x824FCAD0;
    'dispatch: loop {
        match pc {
            0x824FCAD0 => {
    //   block [0x824FCAD0..0x824FCB74)
	// 824FCAD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FCAD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824FCAD8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824FCADC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FCAE0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824FCAE4: 48026A55  bl 0x82523538
	ctx.lr = 0x824FCAE8;
	sub_82523538(ctx, base);
	// 824FCAE8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 824FCAEC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824FCAF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FCAF4: 480B9625  bl 0x825b6118
	ctx.lr = 0x824FCAF8;
	sub_825B6118(ctx, base);
	// 824FCAF8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FCAFC: 480B8D35  bl 0x825b5830
	ctx.lr = 0x824FCB00;
	sub_825B5830(ctx, base);
	// 824FCB00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FCB04: 48026895  bl 0x82523398
	ctx.lr = 0x824FCB08;
	sub_82523398(ctx, base);
	// 824FCB08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FCB0C: 480BA2D5  bl 0x825b6de0
	ctx.lr = 0x824FCB10;
	sub_825B6DE0(ctx, base);
	// 824FCB10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FCB14: 48025FCD  bl 0x82522ae0
	ctx.lr = 0x824FCB18;
	sub_82522AE0(ctx, base);
	// 824FCB18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FCB1C: 480251B5  bl 0x82521cd0
	ctx.lr = 0x824FCB20;
	sub_82521CD0(ctx, base);
	// 824FCB20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FCB24: 480240ED  bl 0x82520c10
	ctx.lr = 0x824FCB28;
	sub_82520C10(ctx, base);
	// 824FCB28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FCB2C: 48023395  bl 0x8251fec0
	ctx.lr = 0x824FCB30;
	sub_8251FEC0(ctx, base);
	// 824FCB30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FCB34: 480223B5  bl 0x8251eee8
	ctx.lr = 0x824FCB38;
	sub_8251EEE8(ctx, base);
	// 824FCB38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FCB3C: 4802111D  bl 0x8251dc58
	ctx.lr = 0x824FCB40;
	sub_8251DC58(ctx, base);
	// 824FCB40: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FCB44: 480B9DAD  bl 0x825b68f0
	ctx.lr = 0x824FCB48;
	sub_825B68F0(ctx, base);
	// 824FCB48: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FCB4C: 4801F935  bl 0x8251c480
	ctx.lr = 0x824FCB50;
	sub_8251C480(ctx, base);
	// 824FCB50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FCB54: 4801E605  bl 0x8251b158
	ctx.lr = 0x824FCB58;
	sub_8251B158(ctx, base);
	// 824FCB58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FCB5C: 48016445  bl 0x82512fa0
	ctx.lr = 0x824FCB60;
	sub_82512FA0(ctx, base);
	// 824FCB60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824FCB64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824FCB68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824FCB6C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824FCB70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FCB78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824FCB78 size=220
    let mut pc: u32 = 0x824FCB78;
    'dispatch: loop {
        match pc {
            0x824FCB78 => {
    //   block [0x824FCB78..0x824FCC54)
	// 824FCB78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FCB7C: 48038541  bl 0x825350bc
	ctx.lr = 0x824FCB80;
	sub_82535080(ctx, base);
	// 824FCB80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FCB84: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824FCB88: 4802BB61  bl 0x825286e8
	ctx.lr = 0x824FCB8C;
	sub_825286E8(ctx, base);
	// 824FCB8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FCB90: 4802B6B1  bl 0x82528240
	ctx.lr = 0x824FCB94;
	sub_82528240(ctx, base);
	// 824FCB94: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FCB98: 4802A089  bl 0x82526c20
	ctx.lr = 0x824FCB9C;
	sub_82526C20(ctx, base);
	// 824FCB9C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FCBA0: 4BFFFE61  bl 0x824fca00
	ctx.lr = 0x824FCBA4;
	sub_824FCA00(ctx, base);
	// 824FCBA4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 824FCBA8: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 824FCBAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FCBB0: 9BA10050  stb r29, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u8 ) };
	// 824FCBB4: 888B0000  lbz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FCBB8: 48002571  bl 0x824ff128
	ctx.lr = 0x824FCBBC;
	sub_824FF128(ctx, base);
	// 824FCBBC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FCBC0: 48029469  bl 0x82526028
	ctx.lr = 0x824FCBC4;
	sub_82526028(ctx, base);
	// 824FCBC4: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 824FCBC8: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 824FCBCC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FCBD0: 9BC10050  stb r30, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u8 ) };
	// 824FCBD4: 888B0000  lbz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FCBD8: 48002551  bl 0x824ff128
	ctx.lr = 0x824FCBDC;
	sub_824FF128(ctx, base);
	// 824FCBDC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FCBE0: 4801D809  bl 0x8251a3e8
	ctx.lr = 0x824FCBE4;
	sub_8251A3E8(ctx, base);
	// 824FCBE4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FCBE8: 4801C581  bl 0x82519168
	ctx.lr = 0x824FCBEC;
	sub_82519168(ctx, base);
	// 824FCBEC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FCBF0: 480B7C51  bl 0x825b4840
	ctx.lr = 0x824FCBF4;
	sub_825B4840(ctx, base);
	// 824FCBF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FCBF8: 4801A949  bl 0x82517540
	ctx.lr = 0x824FCBFC;
	sub_82517540(ctx, base);
	// 824FCBFC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FCC00: 480B7481  bl 0x825b4080
	ctx.lr = 0x824FCC04;
	sub_825B4080(ctx, base);
	// 824FCC04: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 824FCC08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FCC0C: 9BA10050  stb r29, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u8 ) };
	// 824FCC10: 888B0000  lbz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FCC14: 48002515  bl 0x824ff128
	ctx.lr = 0x824FCC18;
	sub_824FF128(ctx, base);
	// 824FCC18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FCC1C: 480B650D  bl 0x825b3128
	ctx.lr = 0x824FCC20;
	sub_825B3128(ctx, base);
	// 824FCC20: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 824FCC24: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FCC28: 9BC10050  stb r30, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u8 ) };
	// 824FCC2C: 888B0000  lbz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FCC30: 480024F9  bl 0x824ff128
	ctx.lr = 0x824FCC34;
	sub_824FF128(ctx, base);
	// 824FCC34: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FCC38: 48027CF9  bl 0x82524930
	ctx.lr = 0x824FCC3C;
	sub_82524930(ctx, base);
	// 824FCC3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FCC40: 480272D1  bl 0x82523f10
	ctx.lr = 0x824FCC44;
	sub_82523F10(ctx, base);
	// 824FCC44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FCC48: 4BFFFE89  bl 0x824fcad0
	ctx.lr = 0x824FCC4C;
	sub_824FCAD0(ctx, base);
	// 824FCC4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824FCC50: 480384BC  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FCC58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824FCC58 size=96
    let mut pc: u32 = 0x824FCC58;
    'dispatch: loop {
        match pc {
            0x824FCC58 => {
    //   block [0x824FCC58..0x824FCCB8)
	// 824FCC58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FCC5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824FCC60: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824FCC64: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FCC68: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 824FCC6C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824FCC70: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 824FCC74: 388B40A0  addi r4, r11, 0x40a0
	ctx.r[4].s64 = ctx.r[11].s64 + 16544;
	// 824FCC78: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 824FCC7C: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FCC80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FCC84: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FCC88: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824FCC8C: 4E800421  bctrl
	ctx.lr = 0x824FCC90;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824FCC90: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FCC94: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FCC98: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 824FCC9C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824FCCA0: 4E800421  bctrl
	ctx.lr = 0x824FCCA4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824FCCA4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824FCCA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824FCCAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824FCCB0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824FCCB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FCCB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FCCB8 size=8
    let mut pc: u32 = 0x824FCCB8;
    'dispatch: loop {
        match pc {
            0x824FCCB8 => {
    //   block [0x824FCCB8..0x824FCCC0)
	// 824FCCB8: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 824FCCBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FCCC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FCCC0 size=16
    let mut pc: u32 = 0x824FCCC0;
    'dispatch: loop {
        match pc {
            0x824FCCC0 => {
    //   block [0x824FCCC0..0x824FCCD0)
	// 824FCCC0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824FCCC4: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 824FCCC8: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824FCCCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FCCD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FCCD0 size=56
    let mut pc: u32 = 0x824FCCD0;
    'dispatch: loop {
        match pc {
            0x824FCCD0 => {
    //   block [0x824FCCD0..0x824FCD08)
	// 824FCCD0: 39630020  addi r11, r3, 0x20
	ctx.r[11].s64 = ctx.r[3].s64 + 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FCD08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824FCD08 size=8
    let mut pc: u32 = 0x824FCD08;
    'dispatch: loop {
        match pc {
            0x824FCD08 => {
    //   block [0x824FCD08..0x824FCD10)
	// 824FCD08: D003002C  stfs f0, 0x2c(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 824FCD0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FCD10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FCD10 size=16
    let mut pc: u32 = 0x824FCD10;
    'dispatch: loop {
        match pc {
            0x824FCD10 => {
    //   block [0x824FCD10..0x824FCD20)
	// 824FCD10: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FCD20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FCD20 size=156
    let mut pc: u32 = 0x824FCD20;
    'dispatch: loop {
        match pc {
            0x824FCD20 => {
    //   block [0x824FCD20..0x824FCDBC)
	// 824FCD20: 39630020  addi r11, r3, 0x20
	ctx.r[11].s64 = ctx.r[3].s64 + 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FCDC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FCDC0 size=204
    let mut pc: u32 = 0x824FCDC0;
    'dispatch: loop {
        match pc {
            0x824FCDC0 => {
    //   block [0x824FCDC0..0x824FCE8C)
	// 824FCDC0: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FCE90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FCE90 size=16
    let mut pc: u32 = 0x824FCE90;
    'dispatch: loop {
        match pc {
            0x824FCE90 => {
    //   block [0x824FCE90..0x824FCEA0)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FCEA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FCEA0 size=80
    let mut pc: u32 = 0x824FCEA0;
    'dispatch: loop {
        match pc {
            0x824FCEA0 => {
    //   block [0x824FCEA0..0x824FCEF0)
	// 824FCEA0: 3965FFFF  addi r11, r5, -1
	ctx.r[11].s64 = ctx.r[5].s64 + -1;
	// 824FCEA4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824FCEA8: 4D980020  bltlr cr6
	if ctx.cr[6].lt { return; }
	// 824FCEAC: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 824FCEB0: 39030020  addi r8, r3, 0x20
	ctx.r[8].s64 = ctx.r[3].s64 + 32;
	// 824FCEB4: 392A4020  addi r9, r10, 0x4020
	ctx.r[9].s64 = ctx.r[10].s64 + 16416;
	// 824FCEB8: A1440000  lhz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FCEF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824FCEF0 size=220
    let mut pc: u32 = 0x824FCEF0;
    'dispatch: loop {
        match pc {
            0x824FCEF0 => {
    //   block [0x824FCEF0..0x824FCFCC)
	// 824FCEF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FCEF4: 480381C1  bl 0x825350b4
	ctx.lr = 0x824FCEF8;
	sub_82535080(ctx, base);
	// 824FCEF8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824FCEFC: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 824FCF00: 396B4020  addi r11, r11, 0x4020
	ctx.r[11].s64 = ctx.r[11].s64 + 16416;
	// 824FCF04: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 824FCF08: 390A0020  addi r8, r10, 0x20
	ctx.r[8].s64 = ctx.r[10].s64 + 32;
	// 824FCF0C: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 824FCF10: C00A0010  lfs f0, 0x10(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FCF14: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FCFD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824FCFD0 size=120
    let mut pc: u32 = 0x824FCFD0;
    'dispatch: loop {
        match pc {
            0x824FCFD0 => {
    //   block [0x824FCFD0..0x824FD02C)
	// 824FCFD0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824FCFD4: D0230010  stfs f1, 0x10(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 824FCFD8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 824FCFDC: 396B40B4  addi r11, r11, 0x40b4
	ctx.r[11].s64 = ctx.r[11].s64 + 16564;
	// 824FCFE0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 824FCFE4: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 824FCFE8: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 824FCFEC: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824FCFF0: 91230008  stw r9, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 824FCFF4: 9103000C  stw r8, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 824FCFF8: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FCFFC: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 824FD000: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FD004: 91630024  stw r11, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 824FD008: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 824FD00C: 91630028  stw r11, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 824FD010: 8164000C  lwz r11, 0xc(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 824FD014: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 824FD018: C0030020  lfs f0, 0x20(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FD01C: C1A30024  lfs f13, 0x24(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824FD020: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 824FD024: 41980008  blt cr6, 0x824fd02c
	if ctx.cr[6].lt {
	pc = 0x824FD02C; continue 'dispatch;
	}
	// 824FD028: FC006890  fmr f0, f13
	ctx.f[0].f64 = ctx.f[13].f64;
	pc = 0x824FD02C; continue 'dispatch;
            }
            0x824FD02C => {
    //   block [0x824FD02C..0x824FD040)
	// 824FD02C: C1A30028  lfs f13, 0x28(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824FD030: D003002C  stfs f0, 0x2c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 824FD034: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 824FD038: 40980008  bge cr6, 0x824fd040
	if !ctx.cr[6].lt {
	pc = 0x824FD040; continue 'dispatch;
	}
	// 824FD03C: FC006890  fmr f0, f13
	ctx.f[0].f64 = ctx.f[13].f64;
	pc = 0x824FD040; continue 'dispatch;
            }
            0x824FD040 => {
    //   block [0x824FD040..0x824FD048)
	// 824FD040: D003002C  stfs f0, 0x2c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 824FD044: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FD048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824FD048 size=908
    let mut pc: u32 = 0x824FD048;
    'dispatch: loop {
        match pc {
            0x824FD048 => {
    //   block [0x824FD048..0x824FD088)
	// 824FD048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FD04C: 48038069  bl 0x825350b4
	ctx.lr = 0x824FD050;
	sub_82535080(ctx, base);
	// 824FD050: 838D0000  lwz r28, 0(r13)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FD054: 3BA00014  li r29, 0x14
	ctx.r[29].s64 = 20;
	// 824FD058: 7D7DE02E  lwzx r11, r29, r28
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 824FD05C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FD060: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 824FD064: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 824FD068: 40980020  bge cr6, 0x824fd088
	if !ctx.cr[6].lt {
	pc = 0x824FD088; continue 'dispatch;
	}
	// 824FD06C: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 824FD070: 392940F4  addi r9, r9, 0x40f4
	ctx.r[9].s64 = ctx.r[9].s64 + 16628;
	// 824FD074: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 824FD078: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 824FD07C: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 824FD080: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 824FD084: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x824FD088; continue 'dispatch;
            }
            0x824FD088 => {
    //   block [0x824FD088..0x824FD3D4)
	// 824FD088: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 824FD08C: C0040010  lfs f0, 0x10(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824FD090: D001FF80  stfs f0, -0x80(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-128 as u32), tmp.u32 ) };
	// 824FD094: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 824FD098: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FD3D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FD3D8 size=8
    let mut pc: u32 = 0x824FD3D8;
    'dispatch: loop {
        match pc {
            0x824FD3D8 => {
    //   block [0x824FD3D8..0x824FD3E0)
	// 824FD3D8: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 824FD3DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FD3E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FD3E0 size=20
    let mut pc: u32 = 0x824FD3E0;
    'dispatch: loop {
        match pc {
            0x824FD3E0 => {
    //   block [0x824FD3E0..0x824FD3F4)
	// 824FD3E0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FD3E4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824FD3E8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FD3EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824FD3F0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FD3F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FD3F8 size=40
    let mut pc: u32 = 0x824FD3F8;
    'dispatch: loop {
        match pc {
            0x824FD3F8 => {
    //   block [0x824FD3F8..0x824FD420)
	// 824FD3F8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824FD3FC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 824FD400: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824FD404: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 824FD408: 396B40B4  addi r11, r11, 0x40b4
	ctx.r[11].s64 = ctx.r[11].s64 + 16564;
	// 824FD40C: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 824FD410: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 824FD414: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824FD418: 9123000C  stw r9, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 824FD41C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FD420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FD420 size=12
    let mut pc: u32 = 0x824FD420;
    'dispatch: loop {
        match pc {
            0x824FD420 => {
    //   block [0x824FD420..0x824FD42C)
	// 824FD420: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824FD424: 386B40B4  addi r3, r11, 0x40b4
	ctx.r[3].s64 = ctx.r[11].s64 + 16564;
	// 824FD428: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FD430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FD430 size=20
    let mut pc: u32 = 0x824FD430;
    'dispatch: loop {
        match pc {
            0x824FD430 => {
    //   block [0x824FD430..0x824FD444)
	// 824FD430: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FD434: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824FD438: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FD43C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824FD440: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FD448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FD448 size=52
    let mut pc: u32 = 0x824FD448;
    'dispatch: loop {
        match pc {
            0x824FD448 => {
    //   block [0x824FD448..0x824FD47C)
	// 824FD448: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824FD44C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 824FD450: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824FD454: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 824FD458: 396B417C  addi r11, r11, 0x417c
	ctx.r[11].s64 = ctx.r[11].s64 + 16764;
	// 824FD45C: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 824FD460: 394A2A68  addi r10, r10, 0x2a68
	ctx.r[10].s64 = ctx.r[10].s64 + 10856;
	// 824FD464: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 824FD468: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824FD46C: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 824FD470: 91430010  stw r10, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 824FD474: 9103000C  stw r8, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 824FD478: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FD480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FD480 size=12
    let mut pc: u32 = 0x824FD480;
    'dispatch: loop {
        match pc {
            0x824FD480 => {
    //   block [0x824FD480..0x824FD48C)
	// 824FD480: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824FD484: 386B417C  addi r3, r11, 0x417c
	ctx.r[3].s64 = ctx.r[11].s64 + 16764;
	// 824FD488: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FD490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824FD490 size=116
    let mut pc: u32 = 0x824FD490;
    'dispatch: loop {
        match pc {
            0x824FD490 => {
    //   block [0x824FD490..0x824FD4E8)
	// 824FD490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FD494: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824FD498: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824FD49C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824FD4A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FD4A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824FD4A8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824FD4AC: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 824FD4B0: 4BFFC241  bl 0x824f96f0
	ctx.lr = 0x824FD4B4;
	sub_824F96F0(ctx, base);
	// 824FD4B4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824FD4B8: 57CA07FE  clrlwi r10, r30, 0x1f
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 824FD4BC: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 824FD4C0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 824FD4C4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824FD4C8: 419A0020  beq cr6, 0x824fd4e8
	if ctx.cr[6].eq {
	pc = 0x824FD4E8; continue 'dispatch;
	}
	// 824FD4CC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FD4D0: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 824FD4D4: 38C00027  li r6, 0x27
	ctx.r[6].s64 = 39;
	// 824FD4D8: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FD4DC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824FD4E0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824FD4E4: 4BF66BD5  bl 0x824640b8
	ctx.lr = 0x824FD4E8;
	sub_824640B8(ctx, base);
	pc = 0x824FD4E8; continue 'dispatch;
            }
            0x824FD4E8 => {
    //   block [0x824FD4E8..0x824FD504)
	// 824FD4E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FD4EC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824FD4F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824FD4F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824FD4F8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824FD4FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824FD500: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FD508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FD508 size=60
    let mut pc: u32 = 0x824FD508;
    'dispatch: loop {
        match pc {
            0x824FD508 => {
    //   block [0x824FD508..0x824FD544)
	// 824FD508: 7CAB3278  xor r11, r5, r6
	ctx.r[11].u64 = ctx.r[5].u64 ^ ctx.r[6].u64;
	// 824FD50C: 556B001E  rlwinm r11, r11, 0, 0, 0xf
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824FD510: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824FD514: 409A0050  bne cr6, 0x824fd564
	if !ctx.cr[6].eq {
		sub_824FD564(ctx, base);
		return;
	}
	// 824FD518: 54AB001E  rlwinm r11, r5, 0, 0, 0xf
	ctx.r[11].u64 = ctx.r[5].u32 as u64 & 0xFFFFFFFFu64;
	// 824FD51C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824FD520: 419A0044  beq cr6, 0x824fd564
	if ctx.cr[6].eq {
		sub_824FD564(ctx, base);
		return;
	}
	// 824FD524: 54CBD97E  srwi r11, r6, 5
	ctx.r[11].u32 = ctx.r[6].u32.wrapping_shr(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824FD528: 7D6B2A78  xor r11, r11, r5
	ctx.r[11].u64 = ctx.r[11].u64 ^ ctx.r[5].u64;
	// 824FD52C: 556B05B4  rlwinm r11, r11, 0, 0x16, 0x1a
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824FD530: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824FD534: 409A0010  bne cr6, 0x824fd544
	if !ctx.cr[6].eq {
		sub_824FD544(ctx, base);
		return;
	}
	// 824FD538: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824FD53C: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 824FD540: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FD544(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FD544 size=32
    let mut pc: u32 = 0x824FD544;
    'dispatch: loop {
        match pc {
            0x824FD544 => {
    //   block [0x824FD544..0x824FD564)
	// 824FD544: 54ABD97E  srwi r11, r5, 5
	ctx.r[11].u32 = ctx.r[5].u32.wrapping_shr(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824FD548: 7D6B3278  xor r11, r11, r6
	ctx.r[11].u64 = ctx.r[11].u64 ^ ctx.r[6].u64;
	// 824FD54C: 556B05B4  rlwinm r11, r11, 0, 0x16, 0x1a
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824FD550: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824FD554: 419AFFE4  beq cr6, 0x824fd538
	if ctx.cr[6].eq {
		sub_824FD508(ctx, base);
		return;
	}
	// 824FD558: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 824FD55C: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 824FD560: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FD564(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FD564 size=56
    let mut pc: u32 = 0x824FD564;
    'dispatch: loop {
        match pc {
            0x824FD564 => {
    //   block [0x824FD564..0x824FD59C)
	// 824FD564: 54CA06FE  clrlwi r10, r6, 0x1b
	ctx.r[10].u64 = ctx.r[6].u32 as u64 & 0x0000001Fu64;
	// 824FD568: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 824FD56C: 54AB06FE  clrlwi r11, r5, 0x1b
	ctx.r[11].u64 = ctx.r[5].u32 as u64 & 0x0000001Fu64;
	// 824FD570: 396B000D  addi r11, r11, 0xd
	ctx.r[11].s64 = ctx.r[11].s64 + 13;
	// 824FD574: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824FD578: 7D6B202E  lwzx r11, r11, r4
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 824FD57C: 7D2A5030  slw r10, r9, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[9].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 824FD580: 7D4B5838  and r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 824FD584: 396B0000  addi r11, r11, 0
	ctx.r[11].s64 = ctx.r[11].s64 + 0;
	// 824FD588: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 824FD58C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 824FD590: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 824FD594: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 824FD598: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FD5A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824FD5A0 size=104
    let mut pc: u32 = 0x824FD5A0;
    'dispatch: loop {
        match pc {
            0x824FD5A0 => {
    //   block [0x824FD5A0..0x824FD608)
	// 824FD5A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FD5A4: 48037B15  bl 0x825350b8
	ctx.lr = 0x824FD5A8;
	sub_82535080(ctx, base);
	// 824FD5A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FD5AC: 81680000  lwz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FD5B0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824FD5B4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 824FD5B8: 7D3E4B78  mr r30, r9
	ctx.r[30].u64 = ctx.r[9].u64;
	// 824FD5BC: 7D445378  mr r4, r10
	ctx.r[4].u64 = ctx.r[10].u64;
	// 824FD5C0: 7D034378  mr r3, r8
	ctx.r[3].u64 = ctx.r[8].u64;
	// 824FD5C4: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 824FD5C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824FD5CC: 4E800421  bctrl
	ctx.lr = 0x824FD5D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824FD5D0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FD5D4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 824FD5D8: 808100D4  lwz r4, 0xd4(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(212 as u32) ) } as u64;
	// 824FD5DC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824FD5E0: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 824FD5E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824FD5E8: 4E800421  bctrl
	ctx.lr = 0x824FD5EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824FD5EC: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 824FD5F0: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 824FD5F4: 389DFFF4  addi r4, r29, -0xc
	ctx.r[4].s64 = ctx.r[29].s64 + -12;
	// 824FD5F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FD5FC: 4BFFFF0D  bl 0x824fd508
	ctx.lr = 0x824FD600;
	sub_824FD508(ctx, base);
	// 824FD600: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824FD604: 48037B04  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FD608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824FD608 size=76
    let mut pc: u32 = 0x824FD608;
    'dispatch: loop {
        match pc {
            0x824FD608 => {
    //   block [0x824FD608..0x824FD654)
	// 824FD608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FD60C: 48037AB1  bl 0x825350bc
	ctx.lr = 0x824FD610;
	sub_82535080(ctx, base);
	// 824FD610: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FD614: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FD618: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824FD61C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824FD620: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 824FD624: 7D044378  mr r4, r8
	ctx.r[4].u64 = ctx.r[8].u64;
	// 824FD628: 7CE33B78  mr r3, r7
	ctx.r[3].u64 = ctx.r[7].u64;
	// 824FD62C: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 824FD630: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824FD634: 4E800421  bctrl
	ctx.lr = 0x824FD638;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824FD638: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 824FD63C: 389EFFF0  addi r4, r30, -0x10
	ctx.r[4].s64 = ctx.r[30].s64 + -16;
	// 824FD640: 80BD0020  lwz r5, 0x20(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32 as u32) ) } as u64;
	// 824FD644: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FD648: 4BFFFEC1  bl 0x824fd508
	ctx.lr = 0x824FD64C;
	sub_824FD508(ctx, base);
	// 824FD64C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824FD650: 48037ABC  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FD658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FD658 size=84
    let mut pc: u32 = 0x824FD658;
    'dispatch: loop {
        match pc {
            0x824FD658 => {
    //   block [0x824FD658..0x824FD664)
	// 824FD658: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 824FD65C: 39630034  addi r11, r3, 0x34
	ctx.r[11].s64 = ctx.r[3].s64 + 52;
	// 824FD660: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	pc = 0x824FD664; continue 'dispatch;
            }
            0x824FD664 => {
    //   block [0x824FD664..0x824FD680)
	// 824FD664: 7D0A4830  slw r10, r8, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[8].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 824FD668: 7D472038  and r7, r10, r4
	ctx.r[7].u64 = ctx.r[10].u64 & ctx.r[4].u64;
	// 824FD66C: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 824FD670: 419A0010  beq cr6, 0x824fd680
	if ctx.cr[6].eq {
	pc = 0x824FD680; continue 'dispatch;
	}
	// 824FD674: 80EB0000  lwz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FD678: 7CE72B78  or r7, r7, r5
	ctx.r[7].u64 = ctx.r[7].u64 | ctx.r[5].u64;
	// 824FD67C: 90EB0000  stw r7, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	pc = 0x824FD680; continue 'dispatch;
            }
            0x824FD680 => {
    //   block [0x824FD680..0x824FD698)
	// 824FD680: 7D4A2838  and r10, r10, r5
	ctx.r[10].u64 = ctx.r[10].u64 & ctx.r[5].u64;
	// 824FD684: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 824FD688: 419A0010  beq cr6, 0x824fd698
	if ctx.cr[6].eq {
	pc = 0x824FD698; continue 'dispatch;
	}
	// 824FD68C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FD690: 7D4A2378  or r10, r10, r4
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[4].u64;
	// 824FD694: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x824FD698; continue 'dispatch;
            }
            0x824FD698 => {
    //   block [0x824FD698..0x824FD6AC)
	// 824FD698: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 824FD69C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 824FD6A0: 2F090020  cmpwi cr6, r9, 0x20
	ctx.cr[6].compare_i32(ctx.r[9].s32, 32, &mut ctx.xer);
	// 824FD6A4: 4198FFC0  blt cr6, 0x824fd664
	if ctx.cr[6].lt {
	pc = 0x824FD664; continue 'dispatch;
	}
	// 824FD6A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FD6B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FD6B0 size=56
    let mut pc: u32 = 0x824FD6B0;
    'dispatch: loop {
        match pc {
            0x824FD6B0 => {
    //   block [0x824FD6B0..0x824FD6E8)
	// 824FD6B0: 3964000D  addi r11, r4, 0xd
	ctx.r[11].s64 = ctx.r[4].s64 + 13;
	// 824FD6B4: 3925000D  addi r9, r5, 0xd
	ctx.r[9].s64 = ctx.r[5].s64 + 13;
	// 824FD6B8: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824FD6BC: 552B103A  slwi r11, r9, 2
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824FD6C0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 824FD6C4: 7CEA182E  lwzx r7, r10, r3
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 824FD6C8: 7D282830  slw r8, r9, r5
	if (ctx.r[5].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[9].u32) << ((ctx.r[5].u8 & 0x1F) as u32)) as u64;
	}
	// 824FD6CC: 7D292030  slw r9, r9, r4
	if (ctx.r[4].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[9].u32) << ((ctx.r[4].u8 & 0x1F) as u32)) as u64;
	}
	// 824FD6D0: 7D083B78  or r8, r8, r7
	ctx.r[8].u64 = ctx.r[8].u64 | ctx.r[7].u64;
	// 824FD6D4: 7D0A192E  stwx r8, r10, r3
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[3].u32), ctx.r[8].u32) };
	// 824FD6D8: 7D4B182E  lwzx r10, r11, r3
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 824FD6DC: 7D2A5378  or r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 | ctx.r[10].u64;
	// 824FD6E0: 7D4B192E  stwx r10, r11, r3
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32), ctx.r[10].u32) };
	// 824FD6E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FD6E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FD6E8 size=56
    let mut pc: u32 = 0x824FD6E8;
    'dispatch: loop {
        match pc {
            0x824FD6E8 => {
    //   block [0x824FD6E8..0x824FD720)
	// 824FD6E8: 3964000D  addi r11, r4, 0xd
	ctx.r[11].s64 = ctx.r[4].s64 + 13;
	// 824FD6EC: 3925000D  addi r9, r5, 0xd
	ctx.r[9].s64 = ctx.r[5].s64 + 13;
	// 824FD6F0: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824FD6F4: 552B103A  slwi r11, r9, 2
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824FD6F8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 824FD6FC: 7CEA182E  lwzx r7, r10, r3
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 824FD700: 7D282830  slw r8, r9, r5
	if (ctx.r[5].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[9].u32) << ((ctx.r[5].u8 & 0x1F) as u32)) as u64;
	}
	// 824FD704: 7D292030  slw r9, r9, r4
	if (ctx.r[4].u8 & 0x20) != 0 {
		ctx.r[9].u64 = 0;
	} else {
		ctx.r[9].u64 = ((ctx.r[9].u32) << ((ctx.r[4].u8 & 0x1F) as u32)) as u64;
	}
	// 824FD708: 7CE84078  andc r8, r7, r8
	ctx.r[8].u64 = ctx.r[7].u64 & !ctx.r[8].u64;
	// 824FD70C: 7D0A192E  stwx r8, r10, r3
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[3].u32), ctx.r[8].u32) };
	// 824FD710: 7D4B182E  lwzx r10, r11, r3
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 824FD714: 7D4A4878  andc r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 & !ctx.r[9].u64;
	// 824FD718: 7D4B192E  stwx r10, r11, r3
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32), ctx.r[10].u32) };
	// 824FD71C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FD720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FD720 size=84
    let mut pc: u32 = 0x824FD720;
    'dispatch: loop {
        match pc {
            0x824FD720 => {
    //   block [0x824FD720..0x824FD72C)
	// 824FD720: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 824FD724: 39630034  addi r11, r3, 0x34
	ctx.r[11].s64 = ctx.r[3].s64 + 52;
	// 824FD728: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	pc = 0x824FD72C; continue 'dispatch;
            }
            0x824FD72C => {
    //   block [0x824FD72C..0x824FD748)
	// 824FD72C: 7D0A4830  slw r10, r8, r9
	if (ctx.r[9].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[8].u32) << ((ctx.r[9].u8 & 0x1F) as u32)) as u64;
	}
	// 824FD730: 7D472038  and r7, r10, r4
	ctx.r[7].u64 = ctx.r[10].u64 & ctx.r[4].u64;
	// 824FD734: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 824FD738: 419A0010  beq cr6, 0x824fd748
	if ctx.cr[6].eq {
	pc = 0x824FD748; continue 'dispatch;
	}
	// 824FD73C: 80EB0000  lwz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FD740: 7CE72878  andc r7, r7, r5
	ctx.r[7].u64 = ctx.r[7].u64 & !ctx.r[5].u64;
	// 824FD744: 90EB0000  stw r7, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	pc = 0x824FD748; continue 'dispatch;
            }
            0x824FD748 => {
    //   block [0x824FD748..0x824FD760)
	// 824FD748: 7D4A2838  and r10, r10, r5
	ctx.r[10].u64 = ctx.r[10].u64 & ctx.r[5].u64;
	// 824FD74C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 824FD750: 419A0010  beq cr6, 0x824fd760
	if ctx.cr[6].eq {
	pc = 0x824FD760; continue 'dispatch;
	}
	// 824FD754: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FD758: 7D4A2078  andc r10, r10, r4
	ctx.r[10].u64 = ctx.r[10].u64 & !ctx.r[4].u64;
	// 824FD75C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x824FD760; continue 'dispatch;
            }
            0x824FD760 => {
    //   block [0x824FD760..0x824FD774)
	// 824FD760: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 824FD764: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 824FD768: 2F090020  cmpwi cr6, r9, 0x20
	ctx.cr[6].compare_i32(ctx.r[9].s32, 32, &mut ctx.xer);
	// 824FD76C: 4198FFC0  blt cr6, 0x824fd72c
	if ctx.cr[6].lt {
	pc = 0x824FD72C; continue 'dispatch;
	}
	// 824FD770: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FD778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824FD778 size=152
    let mut pc: u32 = 0x824FD778;
    'dispatch: loop {
        match pc {
            0x824FD778 => {
    //   block [0x824FD778..0x824FD7E4)
	// 824FD778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FD77C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824FD780: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824FD784: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FD788: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824FD78C: 48001035  bl 0x824fe7c0
	ctx.lr = 0x824FD790;
	sub_824FE7C0(ctx, base);
	// 824FD790: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 824FD794: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 824FD798: 3D008206  lis r8, -0x7dfa
	ctx.r[8].s64 = -2113536000;
	// 824FD79C: 3CE08206  lis r7, -0x7dfa
	ctx.r[7].s64 = -2113536000;
	// 824FD7A0: 3CC08206  lis r6, -0x7dfa
	ctx.r[6].s64 = -2113536000;
	// 824FD7A4: 394A29E4  addi r10, r10, 0x29e4
	ctx.r[10].s64 = ctx.r[10].s64 + 10724;
	// 824FD7A8: 392929D8  addi r9, r9, 0x29d8
	ctx.r[9].s64 = ctx.r[9].s64 + 10712;
	// 824FD7AC: 390829C4  addi r8, r8, 0x29c4
	ctx.r[8].s64 = ctx.r[8].s64 + 10692;
	// 824FD7B0: 38E729B8  addi r7, r7, 0x29b8
	ctx.r[7].s64 = ctx.r[7].s64 + 10680;
	// 824FD7B4: 38C629AC  addi r6, r6, 0x29ac
	ctx.r[6].s64 = ctx.r[6].s64 + 10668;
	// 824FD7B8: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 824FD7BC: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824FD7C0: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 824FD7C4: 397F0034  addi r11, r31, 0x34
	ctx.r[11].s64 = ctx.r[31].s64 + 52;
	// 824FD7C8: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 824FD7CC: 911F000C  stw r8, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 824FD7D0: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 824FD7D4: 90FF0010  stw r7, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[7].u32 ) };
	// 824FD7D8: 90DF0014  stw r6, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[6].u32 ) };
	// 824FD7DC: 90BF0020  stw r5, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[5].u32 ) };
	// 824FD7E0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	pc = 0x824FD7E4; continue 'dispatch;
            }
            0x824FD7E4 => {
    //   block [0x824FD7E4..0x824FD810)
	// 824FD7E4: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 824FD7E8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 824FD7EC: 4200FFF8  bdnz 0x824fd7e4
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x824FD7E4; continue 'dispatch;
	}
	// 824FD7F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824FD7F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FD7F8: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 824FD7FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824FD800: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824FD804: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824FD808: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824FD80C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FD810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FD810 size=64
    let mut pc: u32 = 0x824FD810;
    'dispatch: loop {
        match pc {
            0x824FD810 => {
    //   block [0x824FD810..0x824FD850)
	// 824FD810: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824FD814: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 824FD818: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 824FD81C: 3D008206  lis r8, -0x7dfa
	ctx.r[8].s64 = -2113536000;
	// 824FD820: 3CE08206  lis r7, -0x7dfa
	ctx.r[7].s64 = -2113536000;
	// 824FD824: 396B29E4  addi r11, r11, 0x29e4
	ctx.r[11].s64 = ctx.r[11].s64 + 10724;
	// 824FD828: 394A29D8  addi r10, r10, 0x29d8
	ctx.r[10].s64 = ctx.r[10].s64 + 10712;
	// 824FD82C: 392929C4  addi r9, r9, 0x29c4
	ctx.r[9].s64 = ctx.r[9].s64 + 10692;
	// 824FD830: 390829B8  addi r8, r8, 0x29b8
	ctx.r[8].s64 = ctx.r[8].s64 + 10680;
	// 824FD834: 38E729AC  addi r7, r7, 0x29ac
	ctx.r[7].s64 = ctx.r[7].s64 + 10668;
	// 824FD838: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824FD83C: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 824FD840: 9123000C  stw r9, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 824FD844: 91030010  stw r8, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 824FD848: 90E30014  stw r7, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[7].u32 ) };
	// 824FD84C: 4BF95424  b 0x82492c70
	sub_82492C70(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FD850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FD850 size=16
    let mut pc: u32 = 0x824FD850;
    'dispatch: loop {
        match pc {
            0x824FD850 => {
    //   block [0x824FD850..0x824FD860)
	// 824FD850: 80C6001C  lwz r6, 0x1c(r6)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(28 as u32) ) } as u64;
	// 824FD854: 3884FFF8  addi r4, r4, -8
	ctx.r[4].s64 = ctx.r[4].s64 + -8;
	// 824FD858: 80A5001C  lwz r5, 0x1c(r5)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(28 as u32) ) } as u64;
	// 824FD85C: 4BFFFCAC  b 0x824fd508
	sub_824FD508(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FD860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FD860 size=16
    let mut pc: u32 = 0x824FD860;
    'dispatch: loop {
        match pc {
            0x824FD860 => {
    //   block [0x824FD860..0x824FD870)
	// 824FD860: 80C6001C  lwz r6, 0x1c(r6)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(28 as u32) ) } as u64;
	// 824FD864: 3884FFEC  addi r4, r4, -0x14
	ctx.r[4].s64 = ctx.r[4].s64 + -20;
	// 824FD868: 80A50024  lwz r5, 0x24(r5)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(36 as u32) ) } as u64;
	// 824FD86C: 4BFFFC9C  b 0x824fd508
	sub_824FD508(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FD870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824FD870 size=348
    let mut pc: u32 = 0x824FD870;
    'dispatch: loop {
        match pc {
            0x824FD870 => {
    //   block [0x824FD870..0x824FD8C0)
	// 824FD870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FD874: 48037841  bl 0x825350b4
	ctx.lr = 0x824FD878;
	sub_82535080(ctx, base);
	// 824FD878: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FD87C: 81680000  lwz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FD880: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 824FD884: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 824FD888: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 824FD88C: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 824FD890: 7D244B78  mr r4, r9
	ctx.r[4].u64 = ctx.r[9].u64;
	// 824FD894: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 824FD898: 7D034378  mr r3, r8
	ctx.r[3].u64 = ctx.r[8].u64;
	// 824FD89C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824FD8A0: 4E800421  bctrl
	ctx.lr = 0x824FD8A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824FD8A4: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 824FD8A8: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 824FD8AC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824FD8B0: 409A0028  bne cr6, 0x824fd8d8
	if !ctx.cr[6].eq {
	pc = 0x824FD8D8; continue 'dispatch;
	}
	// 824FD8B4: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824FD8B8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824FD8BC: 419A0014  beq cr6, 0x824fd8d0
	if ctx.cr[6].eq {
	pc = 0x824FD8D0; continue 'dispatch;
	}
            }
            0x824FD8C0 => {
    //   block [0x824FD8C0..0x824FD8D0)
	// 824FD8C0: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 824FD8C4: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 824FD8C8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824FD8CC: 409AFFF4  bne cr6, 0x824fd8c0
	if !ctx.cr[6].eq {
	pc = 0x824FD8C0; continue 'dispatch;
	}
	pc = 0x824FD8D0; continue 'dispatch;
            }
            0x824FD8D0 => {
    //   block [0x824FD8D0..0x824FD8D8)
	// 824FD8D0: 807F001C  lwz r3, 0x1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 824FD8D4: 48000068  b 0x824fd93c
	pc = 0x824FD93C; continue 'dispatch;
            }
            0x824FD8D8 => {
    //   block [0x824FD8D8..0x824FD8E4)
	// 824FD8D8: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FD8DC: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 824FD8E0: 7FFEFB78  mr r30, r31
	ctx.r[30].u64 = ctx.r[31].u64;
	pc = 0x824FD8E4; continue 'dispatch;
            }
            0x824FD8E4 => {
    //   block [0x824FD8E4..0x824FD93C)
	// 824FD8E4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FD8E8: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 824FD8EC: 396B0044  addi r11, r11, 0x44
	ctx.r[11].s64 = ctx.r[11].s64 + 68;
	// 824FD8F0: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824FD8F4: 7D6B502E  lwzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824FD8F8: 556905EE  rlwinm r9, r11, 0, 0x17, 0x17
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824FD8FC: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 824FD900: 409A0058  bne cr6, 0x824fd958
	if !ctx.cr[6].eq {
	pc = 0x824FD958; continue 'dispatch;
	}
	// 824FD904: 556905AC  rlwinm r9, r11, 0, 0x16, 0x16
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824FD908: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 824FD90C: 409A006C  bne cr6, 0x824fd978
	if !ctx.cr[6].eq {
	pc = 0x824FD978; continue 'dispatch;
	}
	// 824FD910: 556902D6  rlwinm r9, r11, 0, 0xb, 0xb
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824FD914: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 824FD918: 409A007C  bne cr6, 0x824fd994
	if !ctx.cr[6].eq {
	pc = 0x824FD994; continue 'dispatch;
	}
	// 824FD91C: 556B0294  rlwinm r11, r11, 0, 0xa, 0xa
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824FD920: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824FD924: 409A0094  bne cr6, 0x824fd9b8
	if !ctx.cr[6].eq {
	pc = 0x824FD9B8; continue 'dispatch;
	}
	// 824FD928: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824FD92C: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 824FD930: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824FD934: 409AFFB0  bne cr6, 0x824fd8e4
	if !ctx.cr[6].eq {
	pc = 0x824FD8E4; continue 'dispatch;
	}
	// 824FD938: 807E001C  lwz r3, 0x1c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	pc = 0x824FD93C; continue 'dispatch;
            }
            0x824FD93C => {
    //   block [0x824FD93C..0x824FD958)
	// 824FD93C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 824FD940: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 824FD944: 389CFFF4  addi r4, r28, -0xc
	ctx.r[4].s64 = ctx.r[28].s64 + -12;
	// 824FD948: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 824FD94C: 4BFFFBBD  bl 0x824fd508
	ctx.lr = 0x824FD950;
	sub_824FD508(ctx, base);
	// 824FD950: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824FD954: 480377B0  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            0x824FD958 => {
    //   block [0x824FD958..0x824FD978)
	// 824FD958: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FD95C: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FD960: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 824FD964: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 824FD968: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 824FD96C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824FD970: 4E800421  bctrl
	ctx.lr = 0x824FD974;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824FD974: 4BFFFFC8  b 0x824fd93c
	pc = 0x824FD93C; continue 'dispatch;
            }
            0x824FD978 => {
    //   block [0x824FD978..0x824FD994)
	// 824FD978: 480B5CD1  bl 0x825b3648
	ctx.lr = 0x824FD97C;
	sub_825B3648(ctx, base);
	// 824FD97C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FD980: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FD984: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 824FD988: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824FD98C: 4E800421  bctrl
	ctx.lr = 0x824FD990;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824FD990: 4BFFFFAC  b 0x824fd93c
	pc = 0x824FD93C; continue 'dispatch;
            }
            0x824FD994 => {
    //   block [0x824FD994..0x824FD9A0)
	// 824FD994: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 824FD998: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824FD99C: 419A0014  beq cr6, 0x824fd9b0
	if ctx.cr[6].eq {
	pc = 0x824FD9B0; continue 'dispatch;
	}
	pc = 0x824FD9A0; continue 'dispatch;
            }
            0x824FD9A0 => {
    //   block [0x824FD9A0..0x824FD9B0)
	// 824FD9A0: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 824FD9A4: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 824FD9A8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824FD9AC: 409AFFF4  bne cr6, 0x824fd9a0
	if !ctx.cr[6].eq {
	pc = 0x824FD9A0; continue 'dispatch;
	}
	pc = 0x824FD9B0; continue 'dispatch;
            }
            0x824FD9B0 => {
    //   block [0x824FD9B0..0x824FD9B8)
	// 824FD9B0: 807F001C  lwz r3, 0x1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 824FD9B4: 4BFFFF88  b 0x824fd93c
	pc = 0x824FD93C; continue 'dispatch;
            }
            0x824FD9B8 => {
    //   block [0x824FD9B8..0x824FD9CC)
	// 824FD9B8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 824FD9BC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 824FD9C0: 997D0000  stb r11, 0(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 824FD9C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824FD9C8: 4803773C  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FD9D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824FD9D0 size=320
    let mut pc: u32 = 0x824FD9D0;
    'dispatch: loop {
        match pc {
            0x824FD9D0 => {
    //   block [0x824FD9D0..0x824FDB04)
	// 824FD9D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FD9D4: 480376D9  bl 0x825350ac
	ctx.lr = 0x824FD9D8;
	sub_82535080(ctx, base);
	// 824FD9D8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FD9DC: 89640005  lbz r11, 5(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(5 as u32) ) } as u64;
	// 824FD9E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824FD9E4: 7D690774  extsb r9, r11
	ctx.r[9].s64 = ctx.r[11].s8 as i64;
	// 824FD9E8: 7FC92214  add r30, r9, r4
	ctx.r[30].u64 = ctx.r[9].u64 + ctx.r[4].u64;
	// 824FD9EC: 811F0010  lwz r8, 0x10(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 824FD9F0: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 824FD9F4: 7D6829D6  mullw r11, r8, r5
	ctx.r[11].s32 = ((ctx.r[8].s32 as i64 * ctx.r[5].s32 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 824FD9F8: 839E0000  lwz r28, 0(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FD9FC: 7F6B5214  add r27, r11, r10
	ctx.r[27].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824FDA00: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 824FDA04: 419A0100  beq cr6, 0x824fdb04
	if ctx.cr[6].eq {
	pc = 0x824FDB04; continue 'dispatch;
	}
	// 824FDA08: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824FDA0C: 54AB083C  slwi r11, r5, 1
	ctx.r[11].u32 = ctx.r[5].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824FDA10: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FDA14: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 824FDA18: 7D655A14  add r11, r5, r11
	ctx.r[11].u64 = ctx.r[5].u64 + ctx.r[11].u64;
	// 824FDA1C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824FDA20: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824FDA24: 81240000  lwz r9, 0(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FDA28: 7FAB5214  add r29, r11, r10
	ctx.r[29].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824FDA2C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 824FDA30: 81290004  lwz r9, 4(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FDA34: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 824FDA38: 4E800421  bctrl
	ctx.lr = 0x824FDA3C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824FDA3C: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FDA40: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824FDA44: 419A00C0  beq cr6, 0x824fdb04
	if ctx.cr[6].eq {
	pc = 0x824FDB04; continue 'dispatch;
	}
	// 824FDA48: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FDA4C: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 824FDA50: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 824FDA54: 389F0020  addi r4, r31, 0x20
	ctx.r[4].s64 = ctx.r[31].s64 + 32;
	// 824FDA58: 394A9F50  addi r10, r10, -0x60b0
	ctx.r[10].s64 = ctx.r[10].s64 + -24752;
	// 824FDA5C: 390B0010  addi r8, r11, 0x10
	ctx.r[8].s64 = ctx.r[11].s64 + 16;
	// 824FDA60: 38EB0020  addi r7, r11, 0x20
	ctx.r[7].s64 = ctx.r[11].s64 + 32;
            }
            0x824FDB04 => {
    //   block [0x824FDB04..0x824FDB10)
	// 824FDB04: C03B0004  lfs f1, 4(r27)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 824FDB08: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 824FDB0C: 480375F0  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FDB10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824FDB10 size=300
    let mut pc: u32 = 0x824FDB10;
    'dispatch: loop {
        match pc {
            0x824FDB10 => {
    //   block [0x824FDB10..0x824FDB64)
	// 824FDB10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FDB14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824FDB18: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824FDB1C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824FDB20: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FDB24: 83CD0000  lwz r30, 0(r13)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FDB28: 3BE00014  li r31, 0x14
	ctx.r[31].s64 = 20;
	// 824FDB2C: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 824FDB30: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 824FDB34: 7D7FF02E  lwzx r11, r31, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 824FDB38: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FDB3C: 810B000C  lwz r8, 0xc(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 824FDB40: 7F0A4040  cmplw cr6, r10, r8
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[8].u32, &mut ctx.xer);
	// 824FDB44: 40980020  bge cr6, 0x824fdb64
	if !ctx.cr[6].lt {
	pc = 0x824FDB64; continue 'dispatch;
	}
	// 824FDB48: 3D008206  lis r8, -0x7dfa
	ctx.r[8].s64 = -2113536000;
	// 824FDB4C: 39084280  addi r8, r8, 0x4280
	ctx.r[8].s64 = ctx.r[8].s64 + 17024;
	// 824FDB50: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 824FDB54: 7D0C42E6  mftb r8, 0x10c
	ctx.r[8].u64 = crate::rt::rdtsc_u64();
	// 824FDB58: 388A000C  addi r4, r10, 0xc
	ctx.r[4].s64 = ctx.r[10].s64 + 12;
	// 824FDB5C: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 824FDB60: 908B0004  stw r4, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	pc = 0x824FDB64; continue 'dispatch;
            }
            0x824FDB64 => {
    //   block [0x824FDB64..0x824FDB84)
	// 824FDB64: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 824FDB68: 90A90004  stw r5, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 824FDB6C: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 824FDB70: 90E9000C  stw r7, 0xc(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 824FDB74: 39660014  addi r11, r6, 0x14
	ctx.r[11].s64 = ctx.r[6].s64 + 20;
	// 824FDB78: 91490010  stw r10, 0x10(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 824FDB7C: 409A0008  bne cr6, 0x824fdb84
	if !ctx.cr[6].eq {
	pc = 0x824FDB84; continue 'dispatch;
	}
	// 824FDB80: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	pc = 0x824FDB84; continue 'dispatch;
            }
            0x824FDB84 => {
    //   block [0x824FDB84..0x824FDBA4)
	// 824FDB84: 91690008  stw r11, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 824FDB88: 89650020  lbz r11, 0x20(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(32 as u32) ) } as u64;
	// 824FDB8C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824FDB90: 419A001C  beq cr6, 0x824fdbac
	if ctx.cr[6].eq {
	pc = 0x824FDBAC; continue 'dispatch;
	}
	// 824FDB94: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 824FDB98: 39660010  addi r11, r6, 0x10
	ctx.r[11].s64 = ctx.r[6].s64 + 16;
	// 824FDB9C: 409A0008  bne cr6, 0x824fdba4
	if !ctx.cr[6].eq {
	pc = 0x824FDBA4; continue 'dispatch;
	}
	// 824FDBA0: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	pc = 0x824FDBA4; continue 'dispatch;
            }
            0x824FDBA4 => {
    //   block [0x824FDBA4..0x824FDBAC)
	// 824FDBA4: 91690044  stw r11, 0x44(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 824FDBA8: 48000008  b 0x824fdbb0
	pc = 0x824FDBB0; continue 'dispatch;
            }
            0x824FDBAC => {
    //   block [0x824FDBAC..0x824FDBB0)
	// 824FDBAC: 91490044  stw r10, 0x44(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(68 as u32), ctx.r[10].u32 ) };
	pc = 0x824FDBB0; continue 'dispatch;
            }
            0x824FDBB0 => {
    //   block [0x824FDBB0..0x824FDC3C)
	// 824FDBB0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FDC40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824FDC40 size=276
    let mut pc: u32 = 0x824FDC40;
    'dispatch: loop {
        match pc {
            0x824FDC40 => {
    //   block [0x824FDC40..0x824FDC8C)
	// 824FDC40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FDC44: 48037479  bl 0x825350bc
	ctx.lr = 0x824FDC48;
	sub_82535080(ctx, base);
	// 824FDC48: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FDC4C: 83CD0000  lwz r30, 0(r13)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FDC50: 3BE00014  li r31, 0x14
	ctx.r[31].s64 = 20;
	// 824FDC54: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 824FDC58: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 824FDC5C: 7D7FF02E  lwzx r11, r31, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 824FDC60: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FDC64: 808B000C  lwz r4, 0xc(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 824FDC68: 7F0A2040  cmplw cr6, r10, r4
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[4].u32, &mut ctx.xer);
	// 824FDC6C: 40980020  bge cr6, 0x824fdc8c
	if !ctx.cr[6].lt {
	pc = 0x824FDC8C; continue 'dispatch;
	}
	// 824FDC70: 3C808206  lis r4, -0x7dfa
	ctx.r[4].s64 = -2113536000;
	// 824FDC74: 3884428C  addi r4, r4, 0x428c
	ctx.r[4].s64 = ctx.r[4].s64 + 17036;
	// 824FDC78: 908A0000  stw r4, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 824FDC7C: 7C8C42E6  mftb r4, 0x10c
	ctx.r[4].u64 = crate::rt::rdtsc_u64();
	// 824FDC80: 3BAA000C  addi r29, r10, 0xc
	ctx.r[29].s64 = ctx.r[10].s64 + 12;
	// 824FDC84: 908A0004  stw r4, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 824FDC88: 93AB0004  stw r29, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	pc = 0x824FDC8C; continue 'dispatch;
            }
            0x824FDC8C => {
    //   block [0x824FDC8C..0x824FDCAC)
	// 824FDC8C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 824FDC90: 90A90004  stw r5, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 824FDC94: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 824FDC98: 9109000C  stw r8, 0xc(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 824FDC9C: 39660014  addi r11, r6, 0x14
	ctx.r[11].s64 = ctx.r[6].s64 + 20;
	// 824FDCA0: 91490010  stw r10, 0x10(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 824FDCA4: 409A0008  bne cr6, 0x824fdcac
	if !ctx.cr[6].eq {
	pc = 0x824FDCAC; continue 'dispatch;
	}
	// 824FDCA8: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	pc = 0x824FDCAC; continue 'dispatch;
            }
            0x824FDCAC => {
    //   block [0x824FDCAC..0x824FDCCC)
	// 824FDCAC: 91690008  stw r11, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 824FDCB0: 89650020  lbz r11, 0x20(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(32 as u32) ) } as u64;
	// 824FDCB4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824FDCB8: 419A001C  beq cr6, 0x824fdcd4
	if ctx.cr[6].eq {
	pc = 0x824FDCD4; continue 'dispatch;
	}
	// 824FDCBC: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 824FDCC0: 39660010  addi r11, r6, 0x10
	ctx.r[11].s64 = ctx.r[6].s64 + 16;
	// 824FDCC4: 409A0008  bne cr6, 0x824fdccc
	if !ctx.cr[6].eq {
	pc = 0x824FDCCC; continue 'dispatch;
	}
	// 824FDCC8: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	pc = 0x824FDCCC; continue 'dispatch;
            }
            0x824FDCCC => {
    //   block [0x824FDCCC..0x824FDCD4)
	// 824FDCCC: 91690044  stw r11, 0x44(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 824FDCD0: 48000008  b 0x824fdcd8
	pc = 0x824FDCD8; continue 'dispatch;
            }
            0x824FDCD4 => {
    //   block [0x824FDCD4..0x824FDCD8)
	// 824FDCD4: 91490044  stw r10, 0x44(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(68 as u32), ctx.r[10].u32 ) };
	pc = 0x824FDCD8; continue 'dispatch;
            }
            0x824FDCD8 => {
    //   block [0x824FDCD8..0x824FDD54)
	// 824FDCD8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FDD58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824FDD58 size=472
    let mut pc: u32 = 0x824FDD58;
    'dispatch: loop {
        match pc {
            0x824FDD58 => {
    //   block [0x824FDD58..0x824FDDB8)
	// 824FDD58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FDD5C: 48037341  bl 0x8253509c
	ctx.lr = 0x824FDD60;
	sub_82535080(ctx, base);
	// 824FDD60: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FDD64: 834D0000  lwz r26, 0(r13)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FDD68: 3AA00014  li r21, 0x14
	ctx.r[21].s64 = 20;
	// 824FDD6C: 7D374B78  mr r23, r9
	ctx.r[23].u64 = ctx.r[9].u64;
	// 824FDD70: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 824FDD74: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 824FDD78: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 824FDD7C: 7D7AA82E  lwzx r11, r26, r21
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[21].u32)) } as u64;
	// 824FDD80: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 824FDD84: 7CF83B78  mr r24, r7
	ctx.r[24].u64 = ctx.r[7].u64;
	// 824FDD88: 7D1D4378  mr r29, r8
	ctx.r[29].u64 = ctx.r[8].u64;
	// 824FDD8C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FDD90: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 824FDD94: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 824FDD98: 40980020  bge cr6, 0x824fddb8
	if !ctx.cr[6].lt {
	pc = 0x824FDDB8; continue 'dispatch;
	}
	// 824FDD9C: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 824FDDA0: 3929429C  addi r9, r9, 0x429c
	ctx.r[9].s64 = ctx.r[9].s64 + 17052;
	// 824FDDA4: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 824FDDA8: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 824FDDAC: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 824FDDB0: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 824FDDB4: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x824FDDB8; continue 'dispatch;
            }
            0x824FDDB8 => {
    //   block [0x824FDDB8..0x824FDF30)
	// 824FDDB8: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FDF30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824FDF30 size=260
    let mut pc: u32 = 0x824FDF30;
    'dispatch: loop {
        match pc {
            0x824FDF30 => {
    //   block [0x824FDF30..0x824FDF78)
	// 824FDF30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FDF34: 48037181  bl 0x825350b4
	ctx.lr = 0x824FDF38;
	sub_82535080(ctx, base);
	// 824FDF38: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FDF3C: 83AD0000  lwz r29, 0(r13)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FDF40: 3BC00014  li r30, 0x14
	ctx.r[30].s64 = 20;
	// 824FDF44: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 824FDF48: 7D7EE82E  lwzx r11, r30, r29
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 824FDF4C: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FDF50: 83EB000C  lwz r31, 0xc(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 824FDF54: 7F04F840  cmplw cr6, r4, r31
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[31].u32, &mut ctx.xer);
	// 824FDF58: 40980020  bge cr6, 0x824fdf78
	if !ctx.cr[6].lt {
	pc = 0x824FDF78; continue 'dispatch;
	}
	// 824FDF5C: 3FE08206  lis r31, -0x7dfa
	ctx.r[31].s64 = -2113536000;
	// 824FDF60: 3BFF42AC  addi r31, r31, 0x42ac
	ctx.r[31].s64 = ctx.r[31].s64 + 17068;
	// 824FDF64: 93E40000  stw r31, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 824FDF68: 7FEC42E6  mftb r31, 0x10c
	ctx.r[31].u64 = crate::rt::rdtsc_u64();
	// 824FDF6C: 3B64000C  addi r27, r4, 0xc
	ctx.r[27].s64 = ctx.r[4].s64 + 12;
	// 824FDF70: 93E40004  stw r31, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 824FDF74: 936B0004  stw r27, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[27].u32 ) };
	pc = 0x824FDF78; continue 'dispatch;
            }
            0x824FDF78 => {
    //   block [0x824FDF78..0x824FDF8C)
	// 824FDF78: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 824FDF7C: 90A30004  stw r5, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 824FDF80: 39670014  addi r11, r7, 0x14
	ctx.r[11].s64 = ctx.r[7].s64 + 20;
	// 824FDF84: 409A0008  bne cr6, 0x824fdf8c
	if !ctx.cr[6].eq {
	pc = 0x824FDF8C; continue 'dispatch;
	}
	// 824FDF88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	pc = 0x824FDF8C; continue 'dispatch;
            }
            0x824FDF8C => {
    //   block [0x824FDF8C..0x824FDFB0)
	// 824FDF8C: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 824FDF90: 9123000C  stw r9, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 824FDF94: 91430010  stw r10, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 824FDF98: 89650020  lbz r11, 0x20(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(32 as u32) ) } as u64;
	// 824FDF9C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824FDFA0: 419A0010  beq cr6, 0x824fdfb0
	if ctx.cr[6].eq {
	pc = 0x824FDFB0; continue 'dispatch;
	}
	// 824FDFA4: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 824FDFA8: 39670010  addi r11, r7, 0x10
	ctx.r[11].s64 = ctx.r[7].s64 + 16;
	// 824FDFAC: 409A0008  bne cr6, 0x824fdfb4
	if !ctx.cr[6].eq {
	pc = 0x824FDFB4; continue 'dispatch;
	}
	pc = 0x824FDFB0; continue 'dispatch;
            }
            0x824FDFB0 => {
    //   block [0x824FDFB0..0x824FDFB4)
	// 824FDFB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	pc = 0x824FDFB4; continue 'dispatch;
            }
            0x824FDFB4 => {
    //   block [0x824FDFB4..0x824FE034)
	// 824FDFB4: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 824FDFB8: 91630044  stw r11, 0x44(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FE040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FE040 size=236
    let mut pc: u32 = 0x824FE040;
    'dispatch: loop {
        match pc {
            0x824FE040 => {
    //   block [0x824FE040..0x824FE078)
	// 824FE040: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 824FE044: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 824FE048: 3D008206  lis r8, -0x7dfa
	ctx.r[8].s64 = -2113536000;
	// 824FE04C: 39630100  addi r11, r3, 0x100
	ctx.r[11].s64 = ctx.r[3].s64 + 256;
	// 824FE050: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 824FE054: 394AFAC4  addi r10, r10, -0x53c
	ctx.r[10].s64 = ctx.r[10].s64 + -1340;
	// 824FE058: 392942D0  addi r9, r9, 0x42d0
	ctx.r[9].s64 = ctx.r[9].s64 + 17104;
	// 824FE05C: 390842C0  addi r8, r8, 0x42c0
	ctx.r[8].s64 = ctx.r[8].s64 + 17088;
	// 824FE060: 38E30008  addi r7, r3, 8
	ctx.r[7].s64 = ctx.r[3].s64 + 8;
	// 824FE064: B0AB0006  sth r5, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[5].u16 ) };
	// 824FE068: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 824FE06C: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 824FE070: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 824FE074: 910B0008  stw r8, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	pc = 0x824FE078; continue 'dispatch;
            }
            0x824FE078 => {
    //   block [0x824FE078..0x824FE088)
	// 824FE078: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824FE07C: 39430108  addi r10, r3, 0x108
	ctx.r[10].s64 = ctx.r[3].s64 + 264;
	// 824FE080: 409A0008  bne cr6, 0x824fe088
	if !ctx.cr[6].eq {
	pc = 0x824FE088; continue 'dispatch;
	}
	// 824FE084: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	pc = 0x824FE088; continue 'dispatch;
            }
            0x824FE088 => {
    //   block [0x824FE088..0x824FE09C)
	// 824FE088: 9147FFF8  stw r10, -8(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(-8 as u32), ctx.r[10].u32 ) };
	// 824FE08C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824FE090: 39430108  addi r10, r3, 0x108
	ctx.r[10].s64 = ctx.r[3].s64 + 264;
	// 824FE094: 409A0008  bne cr6, 0x824fe09c
	if !ctx.cr[6].eq {
	pc = 0x824FE09C; continue 'dispatch;
	}
	// 824FE098: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	pc = 0x824FE09C; continue 'dispatch;
            }
            0x824FE09C => {
    //   block [0x824FE09C..0x824FE0B0)
	// 824FE09C: 9147FFFC  stw r10, -4(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(-4 as u32), ctx.r[10].u32 ) };
	// 824FE0A0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824FE0A4: 39430108  addi r10, r3, 0x108
	ctx.r[10].s64 = ctx.r[3].s64 + 264;
	// 824FE0A8: 409A0008  bne cr6, 0x824fe0b0
	if !ctx.cr[6].eq {
	pc = 0x824FE0B0; continue 'dispatch;
	}
	// 824FE0AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	pc = 0x824FE0B0; continue 'dispatch;
            }
            0x824FE0B0 => {
    //   block [0x824FE0B0..0x824FE0C4)
	// 824FE0B0: 91470000  stw r10, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824FE0B4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824FE0B8: 39430108  addi r10, r3, 0x108
	ctx.r[10].s64 = ctx.r[3].s64 + 264;
	// 824FE0BC: 409A0008  bne cr6, 0x824fe0c4
	if !ctx.cr[6].eq {
	pc = 0x824FE0C4; continue 'dispatch;
	}
	// 824FE0C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	pc = 0x824FE0C4; continue 'dispatch;
            }
            0x824FE0C4 => {
    //   block [0x824FE0C4..0x824FE0D8)
	// 824FE0C4: 91470004  stw r10, 4(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 824FE0C8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824FE0CC: 39430108  addi r10, r3, 0x108
	ctx.r[10].s64 = ctx.r[3].s64 + 264;
	// 824FE0D0: 409A0008  bne cr6, 0x824fe0d8
	if !ctx.cr[6].eq {
	pc = 0x824FE0D8; continue 'dispatch;
	}
	// 824FE0D4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	pc = 0x824FE0D8; continue 'dispatch;
            }
            0x824FE0D8 => {
    //   block [0x824FE0D8..0x824FE0EC)
	// 824FE0D8: 91470008  stw r10, 8(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 824FE0DC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824FE0E0: 39430108  addi r10, r3, 0x108
	ctx.r[10].s64 = ctx.r[3].s64 + 264;
	// 824FE0E4: 409A0008  bne cr6, 0x824fe0ec
	if !ctx.cr[6].eq {
	pc = 0x824FE0EC; continue 'dispatch;
	}
	// 824FE0E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	pc = 0x824FE0EC; continue 'dispatch;
            }
            0x824FE0EC => {
    //   block [0x824FE0EC..0x824FE100)
	// 824FE0EC: 9147000C  stw r10, 0xc(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 824FE0F0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824FE0F4: 39430108  addi r10, r3, 0x108
	ctx.r[10].s64 = ctx.r[3].s64 + 264;
	// 824FE0F8: 409A0008  bne cr6, 0x824fe100
	if !ctx.cr[6].eq {
	pc = 0x824FE100; continue 'dispatch;
	}
	// 824FE0FC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	pc = 0x824FE100; continue 'dispatch;
            }
            0x824FE100 => {
    //   block [0x824FE100..0x824FE114)
	// 824FE100: 91470010  stw r10, 0x10(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 824FE104: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824FE108: 39430108  addi r10, r3, 0x108
	ctx.r[10].s64 = ctx.r[3].s64 + 264;
	// 824FE10C: 409A0008  bne cr6, 0x824fe114
	if !ctx.cr[6].eq {
	pc = 0x824FE114; continue 'dispatch;
	}
	// 824FE110: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	pc = 0x824FE114; continue 'dispatch;
            }
            0x824FE114 => {
    //   block [0x824FE114..0x824FE12C)
	// 824FE114: 38C6FFFF  addi r6, r6, -1
	ctx.r[6].s64 = ctx.r[6].s64 + -1;
	// 824FE118: 91470014  stw r10, 0x14(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 824FE11C: 38E70020  addi r7, r7, 0x20
	ctx.r[7].s64 = ctx.r[7].s64 + 32;
	// 824FE120: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 824FE124: 409AFF54  bne cr6, 0x824fe078
	if !ctx.cr[6].eq {
	pc = 0x824FE078; continue 'dispatch;
	}
	// 824FE128: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FE130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FE130 size=48
    let mut pc: u32 = 0x824FE130;
    'dispatch: loop {
        match pc {
            0x824FE130 => {
    //   block [0x824FE130..0x824FE144)
	// 824FE130: 39230100  addi r9, r3, 0x100
	ctx.r[9].s64 = ctx.r[3].s64 + 256;
	// 824FE134: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 824FE138: 39090008  addi r8, r9, 8
	ctx.r[8].s64 = ctx.r[9].s64 + 8;
	// 824FE13C: 409A0008  bne cr6, 0x824fe144
	if !ctx.cr[6].eq {
	pc = 0x824FE144; continue 'dispatch;
	}
	// 824FE140: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	pc = 0x824FE144; continue 'dispatch;
            }
            0x824FE144 => {
    //   block [0x824FE144..0x824FE160)
	// 824FE144: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824FE148: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 824FE14C: 396BFAC4  addi r11, r11, -0x53c
	ctx.r[11].s64 = ctx.r[11].s64 + -1340;
	// 824FE150: 394A6DD0  addi r10, r10, 0x6dd0
	ctx.r[10].s64 = ctx.r[10].s64 + 28112;
	// 824FE154: 91680000  stw r11, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824FE158: 91490000  stw r10, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824FE15C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FE160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824FE160 size=188
    let mut pc: u32 = 0x824FE160;
    'dispatch: loop {
        match pc {
            0x824FE160 => {
    //   block [0x824FE160..0x824FE184)
	// 824FE160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FE164: 48036F55  bl 0x825350b8
	ctx.lr = 0x824FE168;
	sub_82535080(ctx, base);
	// 824FE168: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FE16C: 3B85FFFF  addi r28, r5, -1
	ctx.r[28].s64 = ctx.r[5].s64 + -1;
	// 824FE170: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 824FE174: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 824FE178: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 824FE17C: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 824FE180: 41980094  blt cr6, 0x824fe214
	if ctx.cr[6].lt {
	pc = 0x824FE214; continue 'dispatch;
	}
	pc = 0x824FE184; continue 'dispatch;
            }
            0x824FE184 => {
    //   block [0x824FE184..0x824FE204)
	// 824FE184: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FE188: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824FE18C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FE190: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824FE194: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FE198: 80E90004  lwz r7, 4(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FE19C: 892B0005  lbz r9, 5(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(5 as u32) ) } as u64;
	// 824FE1A0: 88CA0005  lbz r6, 5(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(5 as u32) ) } as u64;
	// 824FE1A4: 7D280774  extsb r8, r9
	ctx.r[8].s64 = ctx.r[9].s8 as i64;
	// 824FE1A8: 7CC90774  extsb r9, r6
	ctx.r[9].s64 = ctx.r[6].s8 as i64;
	// 824FE1AC: 7CA85A14  add r5, r8, r11
	ctx.r[5].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 824FE1B0: 7CC95214  add r6, r9, r10
	ctx.r[6].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 824FE1B4: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 824FE1B8: 4E800421  bctrl
	ctx.lr = 0x824FE1BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824FE1BC: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FE1C0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824FE1C4: 419A0040  beq cr6, 0x824fe204
	if ctx.cr[6].eq {
	pc = 0x824FE204; continue 'dispatch;
	}
	// 824FE1C8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FE1CC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824FE1D0: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FE1D4: 896B0004  lbz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FE1D8: 894A0004  lbz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FE1DC: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 824FE1E0: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 824FE1E4: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824FE1E8: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824FE1EC: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824FE1F0: 7C6BE82E  lwzx r3, r11, r29
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 824FE1F4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FE1F8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FE1FC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824FE200: 4E800421  bctrl
	ctx.lr = 0x824FE204;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x824FE204 => {
    //   block [0x824FE204..0x824FE214)
	// 824FE204: 3B9CFFFF  addi r28, r28, -1
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	// 824FE208: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 824FE20C: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 824FE210: 4098FF74  bge cr6, 0x824fe184
	if !ctx.cr[6].lt {
	pc = 0x824FE184; continue 'dispatch;
	}
	pc = 0x824FE214; continue 'dispatch;
            }
            0x824FE214 => {
    //   block [0x824FE214..0x824FE21C)
	// 824FE214: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824FE218: 48036EF0  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FE220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824FE220 size=116
    let mut pc: u32 = 0x824FE220;
    'dispatch: loop {
        match pc {
            0x824FE220 => {
    //   block [0x824FE220..0x824FE240)
	// 824FE220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FE224: 48036E99  bl 0x825350bc
	ctx.lr = 0x824FE228;
	sub_82535080(ctx, base);
	// 824FE228: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FE22C: 3BC5FFFF  addi r30, r5, -1
	ctx.r[30].s64 = ctx.r[5].s64 + -1;
	// 824FE230: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 824FE234: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 824FE238: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 824FE23C: 41980050  blt cr6, 0x824fe28c
	if ctx.cr[6].lt {
	pc = 0x824FE28C; continue 'dispatch;
	}
	pc = 0x824FE240; continue 'dispatch;
            }
            0x824FE240 => {
    //   block [0x824FE240..0x824FE28C)
	// 824FE240: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FE244: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824FE248: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FE24C: 896B0004  lbz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FE250: 894A0004  lbz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FE254: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 824FE258: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 824FE25C: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824FE260: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824FE264: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824FE268: 7C6BE82E  lwzx r3, r11, r29
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 824FE26C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FE270: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 824FE274: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824FE278: 4E800421  bctrl
	ctx.lr = 0x824FE27C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824FE27C: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 824FE280: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 824FE284: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 824FE288: 4098FFB8  bge cr6, 0x824fe240
	if !ctx.cr[6].lt {
	pc = 0x824FE240; continue 'dispatch;
	}
            }
            0x824FE28C => {
    //   block [0x824FE28C..0x824FE294)
	// 824FE28C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824FE290: 48036E7C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FE298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824FE298 size=1172
    let mut pc: u32 = 0x824FE298;
    'dispatch: loop {
        match pc {
            0x824FE298 => {
    //   block [0x824FE298..0x824FE2C4)
	// 824FE298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FE29C: 48036E01  bl 0x8253509c
	ctx.lr = 0x824FE2A0;
	sub_82535080(ctx, base);
	// 824FE2A0: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FE2A4: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 824FE2A8: 7C982378  mr r24, r4
	ctx.r[24].u64 = ctx.r[4].u64;
	// 824FE2AC: 807A0004  lwz r3, 4(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FE2B0: 81780004  lwz r11, 4(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FE2B4: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 824FE2B8: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824FE2BC: 41980008  blt cr6, 0x824fe2c4
	if ctx.cr[6].lt {
	pc = 0x824FE2C4; continue 'dispatch;
	}
	// 824FE2C0: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	pc = 0x824FE2C4; continue 'dispatch;
            }
            0x824FE2C4 => {
    //   block [0x824FE2C4..0x824FE2E8)
	// 824FE2C4: 2F0A0020  cmpwi cr6, r10, 0x20
	ctx.cr[6].compare_i32(ctx.r[10].s32, 32, &mut ctx.xer);
	// 824FE2C8: 40980154  bge cr6, 0x824fe41c
	if !ctx.cr[6].lt {
	pc = 0x824FE41C; continue 'dispatch;
	}
	// 824FE2CC: 3AA00000  li r21, 0
	ctx.r[21].s64 = 0;
	// 824FE2D0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824FE2D4: 7EBFAB78  mr r31, r21
	ctx.r[31].u64 = ctx.r[21].u64;
	// 824FE2D8: 7EA4AB78  mr r4, r21
	ctx.r[4].u64 = ctx.r[21].u64;
	// 824FE2DC: 40990104  ble cr6, 0x824fe3e0
	if !ctx.cr[6].gt {
	pc = 0x824FE3E0; continue 'dispatch;
	}
	// 824FE2E0: 7EA5AB78  mr r5, r21
	ctx.r[5].u64 = ctx.r[21].u64;
	// 824FE2E4: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	pc = 0x824FE2E8; continue 'dispatch;
            }
            0x824FE2E8 => {
    //   block [0x824FE2E8..0x824FE308)
	// 824FE2E8: 80DA0004  lwz r6, 4(r26)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FE2EC: 7EAAAB78  mr r10, r21
	ctx.r[10].u64 = ctx.r[21].u64;
	// 824FE2F0: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 824FE2F4: 4099005C  ble cr6, 0x824fe350
	if !ctx.cr[6].gt {
	pc = 0x824FE350; continue 'dispatch;
	}
	// 824FE2F8: 81380000  lwz r9, 0(r24)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FE2FC: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FE300: 7D054A14  add r8, r5, r9
	ctx.r[8].u64 = ctx.r[5].u64 + ctx.r[9].u64;
	// 824FE304: 80E80000  lwz r7, 0(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	pc = 0x824FE308; continue 'dispatch;
            }
            0x824FE308 => {
    //   block [0x824FE308..0x824FE324)
	// 824FE308: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FE30C: 7F093840  cmplw cr6, r9, r7
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[7].u32, &mut ctx.xer);
	// 824FE310: 409A0014  bne cr6, 0x824fe324
	if !ctx.cr[6].eq {
	pc = 0x824FE324; continue 'dispatch;
	}
	// 824FE314: 83CB0004  lwz r30, 4(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FE318: 83A80004  lwz r29, 4(r8)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FE31C: 7F1EE840  cmplw cr6, r30, r29
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[29].u32, &mut ctx.xer);
	// 824FE320: 419A0030  beq cr6, 0x824fe350
	if ctx.cr[6].eq {
	pc = 0x824FE350; continue 'dispatch;
	}
	pc = 0x824FE324; continue 'dispatch;
            }
            0x824FE324 => {
    //   block [0x824FE324..0x824FE33C)
	// 824FE324: 83CB0004  lwz r30, 4(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FE328: 7F1E3840  cmplw cr6, r30, r7
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[7].u32, &mut ctx.xer);
	// 824FE32C: 409A0010  bne cr6, 0x824fe33c
	if !ctx.cr[6].eq {
	pc = 0x824FE33C; continue 'dispatch;
	}
	// 824FE330: 83C80004  lwz r30, 4(r8)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FE334: 7F09F040  cmplw cr6, r9, r30
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[30].u32, &mut ctx.xer);
	// 824FE338: 419A0018  beq cr6, 0x824fe350
	if ctx.cr[6].eq {
	pc = 0x824FE350; continue 'dispatch;
	}
	pc = 0x824FE33C; continue 'dispatch;
            }
            0x824FE33C => {
    //   block [0x824FE33C..0x824FE350)
	// 824FE33C: 813A0004  lwz r9, 4(r26)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FE340: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 824FE344: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 824FE348: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 824FE34C: 4198FFBC  blt cr6, 0x824fe308
	if ctx.cr[6].lt {
	pc = 0x824FE308; continue 'dispatch;
	}
	pc = 0x824FE350; continue 'dispatch;
            }
            0x824FE350 => {
    //   block [0x824FE350..0x824FE37C)
	// 824FE350: 7F0A3000  cmpw cr6, r10, r6
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[6].s32, &mut ctx.xer);
	// 824FE354: 409A0034  bne cr6, 0x824fe388
	if !ctx.cr[6].eq {
	pc = 0x824FE388; continue 'dispatch;
	}
	// 824FE358: 7F1F2000  cmpw cr6, r31, r4
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[4].s32, &mut ctx.xer);
	// 824FE35C: 419A0020  beq cr6, 0x824fe37c
	if ctx.cr[6].eq {
	pc = 0x824FE37C; continue 'dispatch;
	}
	// 824FE360: 81780000  lwz r11, 0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FE364: 7D455A14  add r10, r5, r11
	ctx.r[10].u64 = ctx.r[5].u64 + ctx.r[11].u64;
	// 824FE368: 7D635A14  add r11, r3, r11
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[11].u64;
	// 824FE36C: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FE370: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 824FE374: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FE378: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	pc = 0x824FE37C; continue 'dispatch;
            }
            0x824FE37C => {
    //   block [0x824FE37C..0x824FE388)
	// 824FE37C: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 824FE380: 38630008  addi r3, r3, 8
	ctx.r[3].s64 = ctx.r[3].s64 + 8;
	// 824FE384: 48000048  b 0x824fe3cc
	pc = 0x824FE3CC; continue 'dispatch;
            }
            0x824FE388 => {
    //   block [0x824FE388..0x824FE39C)
	// 824FE388: 3966FFFF  addi r11, r6, -1
	ctx.r[11].s64 = ctx.r[6].s64 + -1;
	// 824FE38C: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824FE390: 917A0004  stw r11, 4(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 824FE394: 40980038  bge cr6, 0x824fe3cc
	if !ctx.cr[6].lt {
	pc = 0x824FE3CC; continue 'dispatch;
	}
	// 824FE398: 55491838  slwi r9, r10, 3
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	pc = 0x824FE39C; continue 'dispatch;
            }
            0x824FE39C => {
    //   block [0x824FE39C..0x824FE3CC)
	// 824FE39C: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FE3A0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 824FE3A4: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 824FE3A8: 39290008  addi r9, r9, 8
	ctx.r[9].s64 = ctx.r[9].s64 + 8;
	// 824FE3AC: 390B0008  addi r8, r11, 8
	ctx.r[8].s64 = ctx.r[11].s64 + 8;
	// 824FE3B0: 80E80000  lwz r7, 0(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FE3B4: 90EB0000  stw r7, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 824FE3B8: 81080004  lwz r8, 4(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FE3BC: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 824FE3C0: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FE3C4: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824FE3C8: 4198FFD4  blt cr6, 0x824fe39c
	if ctx.cr[6].lt {
	pc = 0x824FE39C; continue 'dispatch;
	}
	pc = 0x824FE3CC; continue 'dispatch;
            }
            0x824FE3CC => {
    //   block [0x824FE3CC..0x824FE3E0)
	// 824FE3CC: 81780004  lwz r11, 4(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FE3D0: 38840001  addi r4, r4, 1
	ctx.r[4].s64 = ctx.r[4].s64 + 1;
	// 824FE3D4: 38A50008  addi r5, r5, 8
	ctx.r[5].s64 = ctx.r[5].s64 + 8;
	// 824FE3D8: 7F045800  cmpw cr6, r4, r11
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824FE3DC: 4198FF0C  blt cr6, 0x824fe2e8
	if ctx.cr[6].lt {
	pc = 0x824FE2E8; continue 'dispatch;
	}
	pc = 0x824FE3E0; continue 'dispatch;
            }
            0x824FE3E0 => {
    //   block [0x824FE3E0..0x824FE400)
	// 824FE3E0: 81780008  lwz r11, 8(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(8 as u32) ) } as u64;
	// 824FE3E4: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 824FE3E8: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 824FE3EC: 40980024  bge cr6, 0x824fe410
	if !ctx.cr[6].lt {
	pc = 0x824FE410; continue 'dispatch;
	}
	// 824FE3F0: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824FE3F4: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824FE3F8: 41980008  blt cr6, 0x824fe400
	if ctx.cr[6].lt {
	pc = 0x824FE400; continue 'dispatch;
	}
	// 824FE3FC: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	pc = 0x824FE400; continue 'dispatch;
            }
            0x824FE400 => {
    //   block [0x824FE400..0x824FE410)
	// 824FE400: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 824FE404: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 824FE408: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 824FE40C: 4BF6FEBD  bl 0x8246e2c8
	ctx.lr = 0x824FE410;
	sub_8246E2C8(ctx, base);
	pc = 0x824FE410; continue 'dispatch;
            }
            0x824FE410 => {
    //   block [0x824FE410..0x824FE41C)
	// 824FE410: 93F80004  stw r31, 4(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 824FE414: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 824FE418: 48036CD4  b 0x825350ec
	sub_825350D0(ctx, base);
	return;
            }
            0x824FE41C => {
    //   block [0x824FE41C..0x824FE458)
	// 824FE41C: 4BF6EE45  bl 0x8246d260
	ctx.lr = 0x824FE420;
	sub_8246D260(ctx, base);
	// 824FE420: 82CD0000  lwz r22, 0(r13)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FE424: 3AE00010  li r23, 0x10
	ctx.r[23].s64 = 16;
	// 824FE428: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824FE42C: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
	// 824FE430: 7C77B02E  lwzx r3, r23, r22
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 824FE434: 55640036  rlwinm r4, r11, 0, 0, 0x1b
	ctx.r[4].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 824FE438: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 824FE43C: 8123002C  lwz r9, 0x2c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 824FE440: 7D4B2214  add r10, r11, r4
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 824FE444: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 824FE448: 41990010  bgt cr6, 0x824fe458
	if ctx.cr[6].gt {
	pc = 0x824FE458; continue 'dispatch;
	}
	// 824FE44C: 7D795B78  mr r25, r11
	ctx.r[25].u64 = ctx.r[11].u64;
	// 824FE450: 91430020  stw r10, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 824FE454: 48000018  b 0x824fe46c
	pc = 0x824FE46C; continue 'dispatch;
            }
            0x824FE458 => {
    //   block [0x824FE458..0x824FE46C)
	// 824FE458: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FE45C: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 824FE460: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824FE464: 4E800421  bctrl
	ctx.lr = 0x824FE468;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824FE468: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
            }
            0x824FE46C => {
    //   block [0x824FE46C..0x824FE494)
	// 824FE46C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 824FE470: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 824FE474: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 824FE478: 4BF6EDA1  bl 0x8246d218
	ctx.lr = 0x824FE47C;
	sub_8246D218(ctx, base);
	// 824FE47C: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FE480: 3AA00000  li r21, 0
	ctx.r[21].s64 = 0;
	// 824FE484: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824FE488: 7EBDAB78  mr r29, r21
	ctx.r[29].u64 = ctx.r[21].u64;
	// 824FE48C: 409900B0  ble cr6, 0x824fe53c
	if !ctx.cr[6].gt {
	pc = 0x824FE53C; continue 'dispatch;
	}
	// 824FE490: 7EBFAB78  mr r31, r21
	ctx.r[31].u64 = ctx.r[21].u64;
	pc = 0x824FE494; continue 'dispatch;
            }
            0x824FE494 => {
    //   block [0x824FE494..0x824FE4BC)
	// 824FE494: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FE498: 7FDF582A  ldx r30, r31, r11
	ctx.r[30].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) };
	// 824FE49C: FBC10050  std r30, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u64 ) };
	// 824FE4A0: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824FE4A4: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 824FE4A8: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 824FE4AC: 40990010  ble cr6, 0x824fe4bc
	if !ctx.cr[6].gt {
	pc = 0x824FE4BC; continue 'dispatch;
	}
	// 824FE4B0: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 824FE4B4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 824FE4B8: EBC10050  ld r30, 0x50(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	pc = 0x824FE4BC; continue 'dispatch;
            }
            0x824FE4BC => {
    //   block [0x824FE4BC..0x824FE4DC)
	// 824FE4BC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824FE4C0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 824FE4C4: 4BF6F49D  bl 0x8246d960
	ctx.lr = 0x824FE4C8;
	sub_8246D960(ctx, base);
	// 824FE4C8: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 824FE4CC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 824FE4D0: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824FE4D4: 40990008  ble cr6, 0x824fe4dc
	if !ctx.cr[6].gt {
	pc = 0x824FE4DC; continue 'dispatch;
	}
	// 824FE4D8: 7EAAAB78  mr r10, r21
	ctx.r[10].u64 = ctx.r[21].u64;
	pc = 0x824FE4DC; continue 'dispatch;
            }
            0x824FE4DC => {
    //   block [0x824FE4DC..0x824FE510)
	// 824FE4DC: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 824FE4E0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824FE4E4: 419A002C  beq cr6, 0x824fe510
	if ctx.cr[6].eq {
	pc = 0x824FE510; continue 'dispatch;
	}
	// 824FE4E8: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 824FE4EC: 81410058  lwz r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 824FE4F0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824FE4F4: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824FE4F8: 7D2B502A  ldx r9, r11, r10
	ctx.r[9].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) };
	// 824FE4FC: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 824FE500: 7D2B512A  stdx r9, r11, r10
	unsafe { crate::rt::store_u64(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[9].u64) };
	// 824FE504: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FE508: 7EBF592E  stwx r21, r31, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32), ctx.r[21].u32) };
	// 824FE50C: 4800001C  b 0x824fe528
	pc = 0x824FE528; continue 'dispatch;
            }
            0x824FE510 => {
    //   block [0x824FE510..0x824FE528)
	// 824FE510: 57AB402E  slwi r11, r29, 8
	ctx.r[11].u32 = ctx.r[29].u32.wrapping_shl(8);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824FE514: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824FE518: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 824FE51C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 824FE520: 61650001  ori r5, r11, 1
	ctx.r[5].u64 = ctx.r[11].u64 | 1;
	// 824FE524: 4BF6F35D  bl 0x8246d880
	ctx.lr = 0x824FE528;
	sub_8246D880(ctx, base);
	pc = 0x824FE528; continue 'dispatch;
            }
            0x824FE528 => {
    //   block [0x824FE528..0x824FE53C)
	// 824FE528: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FE52C: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 824FE530: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 824FE534: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824FE538: 4198FF5C  blt cr6, 0x824fe494
	if ctx.cr[6].lt {
	pc = 0x824FE494; continue 'dispatch;
	}
	pc = 0x824FE53C; continue 'dispatch;
            }
            0x824FE53C => {
    //   block [0x824FE53C..0x824FE558)
	// 824FE53C: 81780004  lwz r11, 4(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FE540: 7EBCAB78  mr r28, r21
	ctx.r[28].u64 = ctx.r[21].u64;
	// 824FE544: 7EBBAB78  mr r27, r21
	ctx.r[27].u64 = ctx.r[21].u64;
	// 824FE548: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824FE54C: 409900E0  ble cr6, 0x824fe62c
	if !ctx.cr[6].gt {
	pc = 0x824FE62C; continue 'dispatch;
	}
	// 824FE550: 7EBEAB78  mr r30, r21
	ctx.r[30].u64 = ctx.r[21].u64;
	// 824FE554: 7EBDAB78  mr r29, r21
	ctx.r[29].u64 = ctx.r[21].u64;
	pc = 0x824FE558; continue 'dispatch;
            }
            0x824FE558 => {
    //   block [0x824FE558..0x824FE57C)
	// 824FE558: 81780000  lwz r11, 0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FE55C: 7D7E582A  ldx r11, r30, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) };
	// 824FE560: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 824FE564: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824FE568: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 824FE56C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 824FE570: 4099000C  ble cr6, 0x824fe57c
	if !ctx.cr[6].gt {
	pc = 0x824FE57C; continue 'dispatch;
	}
	// 824FE574: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 824FE578: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	pc = 0x824FE57C; continue 'dispatch;
            }
            0x824FE57C => {
    //   block [0x824FE57C..0x824FE5A0)
	// 824FE57C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 824FE580: E8810050  ld r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 824FE584: 4BF6F3DD  bl 0x8246d960
	ctx.lr = 0x824FE588;
	sub_8246D960(ctx, base);
	// 824FE588: 81410060  lwz r10, 0x60(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 824FE58C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 824FE590: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 824FE594: 7F045000  cmpw cr6, r4, r10
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[10].s32, &mut ctx.xer);
	// 824FE598: 40990008  ble cr6, 0x824fe5a0
	if !ctx.cr[6].gt {
	pc = 0x824FE5A0; continue 'dispatch;
	}
	// 824FE59C: 7EABAB78  mr r11, r21
	ctx.r[11].u64 = ctx.r[21].u64;
	pc = 0x824FE5A0; continue 'dispatch;
            }
            0x824FE5A0 => {
    //   block [0x824FE5A0..0x824FE5DC)
	// 824FE5A0: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 824FE5A4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824FE5A8: 419A004C  beq cr6, 0x824fe5f4
	if ctx.cr[6].eq {
	pc = 0x824FE5F4; continue 'dispatch;
	}
	// 824FE5AC: 7D6A2214  add r11, r10, r4
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[4].u64;
	// 824FE5B0: 81210058  lwz r9, 0x58(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 824FE5B4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824FE5B8: 556A1838  slwi r10, r11, 3
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824FE5BC: 7D6A482A  ldx r11, r10, r9
	ctx.r[11].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	// 824FE5C0: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 824FE5C4: 57E8063E  clrlwi r8, r31, 0x18
	ctx.r[8].u64 = ctx.r[31].u32 as u64 & 0x000000FFu64;
	// 824FE5C8: 2F080001  cmpwi cr6, r8, 1
	ctx.cr[6].compare_i32(ctx.r[8].s32, 1, &mut ctx.xer);
	// 824FE5CC: 40990010  ble cr6, 0x824fe5dc
	if !ctx.cr[6].gt {
	pc = 0x824FE5DC; continue 'dispatch;
	}
	// 824FE5D0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 824FE5D4: 7D6A492A  stdx r11, r10, r9
	unsafe { crate::rt::store_u64(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32), ctx.r[11].u64) };
	// 824FE5D8: 48000040  b 0x824fe618
	pc = 0x824FE618; continue 'dispatch;
            }
            0x824FE5DC => {
    //   block [0x824FE5DC..0x824FE5F4)
	// 824FE5DC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 824FE5E0: 4BF6F3E1  bl 0x8246d9c0
	ctx.lr = 0x824FE5E4;
	sub_8246D9C0(ctx, base);
	// 824FE5E4: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FE5E8: 57EAD978  rlwinm r10, r31, 0x1b, 5, 0x1c
	ctx.r[10].u64 = ctx.r[31].u32 as u64 & 0x0000001Fu64;
	// 824FE5EC: 7EAA592E  stwx r21, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[21].u32) };
	// 824FE5F0: 48000028  b 0x824fe618
	pc = 0x824FE618; continue 'dispatch;
            }
            0x824FE5F4 => {
    //   block [0x824FE5F4..0x824FE618)
	// 824FE5F4: 81780000  lwz r11, 0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FE5F8: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 824FE5FC: 7D5E5A14  add r10, r30, r11
	ctx.r[10].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 824FE600: 7D7D5A14  add r11, r29, r11
	ctx.r[11].u64 = ctx.r[29].u64 + ctx.r[11].u64;
	// 824FE604: 3BBD0008  addi r29, r29, 8
	ctx.r[29].s64 = ctx.r[29].s64 + 8;
	// 824FE608: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FE60C: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 824FE610: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FE614: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	pc = 0x824FE618; continue 'dispatch;
            }
            0x824FE618 => {
    //   block [0x824FE618..0x824FE62C)
	// 824FE618: 81780004  lwz r11, 4(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FE61C: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 824FE620: 3BDE0008  addi r30, r30, 8
	ctx.r[30].s64 = ctx.r[30].s64 + 8;
	// 824FE624: 7F1B5800  cmpw cr6, r27, r11
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824FE628: 4198FF30  blt cr6, 0x824fe558
	if ctx.cr[6].lt {
	pc = 0x824FE558; continue 'dispatch;
	}
	pc = 0x824FE62C; continue 'dispatch;
            }
            0x824FE62C => {
    //   block [0x824FE62C..0x824FE64C)
	// 824FE62C: 81780008  lwz r11, 8(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(8 as u32) ) } as u64;
	// 824FE630: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 824FE634: 7F0BE000  cmpw cr6, r11, r28
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[28].s32, &mut ctx.xer);
	// 824FE638: 40980024  bge cr6, 0x824fe65c
	if !ctx.cr[6].lt {
	pc = 0x824FE65C; continue 'dispatch;
	}
	// 824FE63C: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824FE640: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824FE644: 41980008  blt cr6, 0x824fe64c
	if ctx.cr[6].lt {
	pc = 0x824FE64C; continue 'dispatch;
	}
	// 824FE648: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	pc = 0x824FE64C; continue 'dispatch;
            }
            0x824FE64C => {
    //   block [0x824FE64C..0x824FE65C)
	// 824FE64C: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 824FE650: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 824FE654: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 824FE658: 4BF6FC71  bl 0x8246e2c8
	ctx.lr = 0x824FE65C;
	sub_8246E2C8(ctx, base);
	pc = 0x824FE65C; continue 'dispatch;
            }
            0x824FE65C => {
    //   block [0x824FE65C..0x824FE67C)
	// 824FE65C: 93980004  stw r28, 4(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 824FE660: 7EBFAB78  mr r31, r21
	ctx.r[31].u64 = ctx.r[21].u64;
	// 824FE664: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FE668: 7EA7AB78  mr r7, r21
	ctx.r[7].u64 = ctx.r[21].u64;
	// 824FE66C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824FE670: 40990050  ble cr6, 0x824fe6c0
	if !ctx.cr[6].gt {
	pc = 0x824FE6C0; continue 'dispatch;
	}
	// 824FE674: 7EA9AB78  mr r9, r21
	ctx.r[9].u64 = ctx.r[21].u64;
	// 824FE678: 7EA8AB78  mr r8, r21
	ctx.r[8].u64 = ctx.r[21].u64;
	pc = 0x824FE67C; continue 'dispatch;
            }
            0x824FE67C => {
    //   block [0x824FE67C..0x824FE6AC)
	// 824FE67C: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FE680: 7D495A14  add r10, r9, r11
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 824FE684: 7CC9582E  lwzx r6, r9, r11
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824FE688: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 824FE68C: 419A0020  beq cr6, 0x824fe6ac
	if ctx.cr[6].eq {
	pc = 0x824FE6AC; continue 'dispatch;
	}
	// 824FE690: 80CA0000  lwz r6, 0(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FE694: 7D685A14  add r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 824FE698: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 824FE69C: 39080008  addi r8, r8, 8
	ctx.r[8].s64 = ctx.r[8].s64 + 8;
	// 824FE6A0: 90CB0000  stw r6, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 824FE6A4: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FE6A8: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	pc = 0x824FE6AC; continue 'dispatch;
            }
            0x824FE6AC => {
    //   block [0x824FE6AC..0x824FE6C0)
	// 824FE6AC: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FE6B0: 38E70001  addi r7, r7, 1
	ctx.r[7].s64 = ctx.r[7].s64 + 1;
	// 824FE6B4: 39290008  addi r9, r9, 8
	ctx.r[9].s64 = ctx.r[9].s64 + 8;
	// 824FE6B8: 7F075800  cmpw cr6, r7, r11
	ctx.cr[6].compare_i32(ctx.r[7].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824FE6BC: 4198FFC0  blt cr6, 0x824fe67c
	if ctx.cr[6].lt {
	pc = 0x824FE67C; continue 'dispatch;
	}
	pc = 0x824FE6C0; continue 'dispatch;
            }
            0x824FE6C0 => {
    //   block [0x824FE6C0..0x824FE6E0)
	// 824FE6C0: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 824FE6C4: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 824FE6C8: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 824FE6CC: 40980024  bge cr6, 0x824fe6f0
	if !ctx.cr[6].lt {
	pc = 0x824FE6F0; continue 'dispatch;
	}
	// 824FE6D0: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824FE6D4: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824FE6D8: 41980008  blt cr6, 0x824fe6e0
	if ctx.cr[6].lt {
	pc = 0x824FE6E0; continue 'dispatch;
	}
	// 824FE6DC: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	pc = 0x824FE6E0; continue 'dispatch;
            }
            0x824FE6E0 => {
    //   block [0x824FE6E0..0x824FE6F0)
	// 824FE6E0: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 824FE6E4: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 824FE6E8: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 824FE6EC: 4BF6FBDD  bl 0x8246e2c8
	ctx.lr = 0x824FE6F0;
	sub_8246E2C8(ctx, base);
	pc = 0x824FE6F0; continue 'dispatch;
            }
            0x824FE6F0 => {
    //   block [0x824FE6F0..0x824FE724)
	// 824FE6F0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 824FE6F4: 93FA0004  stw r31, 4(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 824FE6F8: 4BF6F151  bl 0x8246d848
	ctx.lr = 0x824FE6FC;
	sub_8246D848(ctx, base);
	// 824FE6FC: 7C77B02E  lwzx r3, r23, r22
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 824FE700: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 824FE704: 93230020  stw r25, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[25].u32 ) };
	// 824FE708: 7F195840  cmplw cr6, r25, r11
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[11].u32, &mut ctx.xer);
	// 824FE70C: 409A0018  bne cr6, 0x824fe724
	if !ctx.cr[6].eq {
	pc = 0x824FE724; continue 'dispatch;
	}
	// 824FE710: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FE714: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 824FE718: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 824FE71C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824FE720: 4E800421  bctrl
	ctx.lr = 0x824FE724;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x824FE724 => {
    //   block [0x824FE724..0x824FE72C)
	// 824FE724: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 824FE728: 480369C4  b 0x825350ec
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FE730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FE730 size=8
    let mut pc: u32 = 0x824FE730;
    'dispatch: loop {
        match pc {
            0x824FE730 => {
    //   block [0x824FE730..0x824FE738)
	// 824FE730: 3863FFF8  addi r3, r3, -8
	ctx.r[3].s64 = ctx.r[3].s64 + -8;
	// 824FE734: 48000004  b 0x824fe738
	sub_824FE738(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FE738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824FE738 size=124
    let mut pc: u32 = 0x824FE738;
    'dispatch: loop {
        match pc {
            0x824FE738 => {
    //   block [0x824FE738..0x824FE75C)
	// 824FE738: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FE73C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824FE740: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824FE744: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FE748: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824FE74C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 824FE750: 393F0008  addi r9, r31, 8
	ctx.r[9].s64 = ctx.r[31].s64 + 8;
	// 824FE754: 409A0008  bne cr6, 0x824fe75c
	if !ctx.cr[6].eq {
	pc = 0x824FE75C; continue 'dispatch;
	}
	// 824FE758: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	pc = 0x824FE75C; continue 'dispatch;
            }
            0x824FE75C => {
    //   block [0x824FE75C..0x824FE79C)
	// 824FE75C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824FE760: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 824FE764: 396BFAC4  addi r11, r11, -0x53c
	ctx.r[11].s64 = ctx.r[11].s64 + -1340;
	// 824FE768: 394A6DD0  addi r10, r10, 0x6dd0
	ctx.r[10].s64 = ctx.r[10].s64 + 28112;
	// 824FE76C: 548807FE  clrlwi r8, r4, 0x1f
	ctx.r[8].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 824FE770: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 824FE774: 91690000  stw r11, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824FE778: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824FE77C: 419A0020  beq cr6, 0x824fe79c
	if ctx.cr[6].eq {
	pc = 0x824FE79C; continue 'dispatch;
	}
	// 824FE780: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FE784: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 824FE788: 38C00027  li r6, 0x27
	ctx.r[6].s64 = 39;
	// 824FE78C: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FE790: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824FE794: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824FE798: 4BF65921  bl 0x824640b8
	ctx.lr = 0x824FE79C;
	sub_824640B8(ctx, base);
	pc = 0x824FE79C; continue 'dispatch;
            }
            0x824FE79C => {
    //   block [0x824FE79C..0x824FE7B4)
	// 824FE79C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FE7A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824FE7A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824FE7A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824FE7AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824FE7B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FE7B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FE7B8 size=8
    let mut pc: u32 = 0x824FE7B8;
    'dispatch: loop {
        match pc {
            0x824FE7B8 => {
    //   block [0x824FE7B8..0x824FE7C0)
	// 824FE7B8: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 824FE7BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FE7C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FE7C0 size=128
    let mut pc: u32 = 0x824FE7C0;
    'dispatch: loop {
        match pc {
            0x824FE7C0 => {
    //   block [0x824FE7C0..0x824FE840)
	// 824FE7C0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824FE7C4: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 824FE7C8: 396BFAB8  addi r11, r11, -0x548
	ctx.r[11].s64 = ctx.r[11].s64 + -1352;
	// 824FE7CC: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 824FE7D0: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 824FE7D4: 394AFAE0  addi r10, r10, -0x520
	ctx.r[10].s64 = ctx.r[10].s64 + -1312;
	// 824FE7D8: B0C30006  sth r6, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[6].u16 ) };
	// 824FE7DC: 3D008206  lis r8, -0x7dfa
	ctx.r[8].s64 = -2113536000;
	// 824FE7E0: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 824FE7E4: 3CE08206  lis r7, -0x7dfa
	ctx.r[7].s64 = -2113536000;
	// 824FE7E8: 3CC08206  lis r6, -0x7dfa
	ctx.r[6].s64 = -2113536000;
	// 824FE7EC: 3CA08206  lis r5, -0x7dfa
	ctx.r[5].s64 = -2113536000;
	// 824FE7F0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824FE7F4: 9143000C  stw r10, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 824FE7F8: 3C808206  lis r4, -0x7dfa
	ctx.r[4].s64 = -2113536000;
	// 824FE7FC: 3929FAF4  addi r9, r9, -0x50c
	ctx.r[9].s64 = ctx.r[9].s64 + -1292;
	// 824FE800: 3908FAD4  addi r8, r8, -0x52c
	ctx.r[8].s64 = ctx.r[8].s64 + -1324;
	// 824FE804: 38E74318  addi r7, r7, 0x4318
	ctx.r[7].s64 = ctx.r[7].s64 + 17176;
	// 824FE808: 38C6430C  addi r6, r6, 0x430c
	ctx.r[6].s64 = ctx.r[6].s64 + 17164;
	// 824FE80C: 38A542F8  addi r5, r5, 0x42f8
	ctx.r[5].s64 = ctx.r[5].s64 + 17144;
	// 824FE810: 396B42EC  addi r11, r11, 0x42ec
	ctx.r[11].s64 = ctx.r[11].s64 + 17132;
	// 824FE814: 91230010  stw r9, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 824FE818: 388442E0  addi r4, r4, 0x42e0
	ctx.r[4].s64 = ctx.r[4].s64 + 17120;
	// 824FE81C: 91030014  stw r8, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[8].u32 ) };
	// 824FE820: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 824FE824: 90E30000  stw r7, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 824FE828: 90C30008  stw r6, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 824FE82C: 90A3000C  stw r5, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[5].u32 ) };
	// 824FE830: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 824FE834: 90830014  stw r4, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[4].u32 ) };
	// 824FE838: 91430020  stw r10, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 824FE83C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FE840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FE840 size=8
    let mut pc: u32 = 0x824FE840;
    'dispatch: loop {
        match pc {
            0x824FE840 => {
    //   block [0x824FE840..0x824FE848)
	// 824FE840: 3863FFF8  addi r3, r3, -8
	ctx.r[3].s64 = ctx.r[3].s64 + -8;
	// 824FE844: 4800000C  b 0x824fe850
	sub_824FE850(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FE848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FE848 size=8
    let mut pc: u32 = 0x824FE848;
    'dispatch: loop {
        match pc {
            0x824FE848 => {
    //   block [0x824FE848..0x824FE850)
	// 824FE848: 3863FFF0  addi r3, r3, -0x10
	ctx.r[3].s64 = ctx.r[3].s64 + -16;
	// 824FE84C: 48000004  b 0x824fe850
	sub_824FE850(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FE850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824FE850 size=100
    let mut pc: u32 = 0x824FE850;
    'dispatch: loop {
        match pc {
            0x824FE850 => {
    //   block [0x824FE850..0x824FE898)
	// 824FE850: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FE854: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824FE858: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824FE85C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824FE860: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FE864: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824FE868: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824FE86C: 4BF94405  bl 0x82492c70
	ctx.lr = 0x824FE870;
	sub_82492C70(ctx, base);
	// 824FE870: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 824FE874: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824FE878: 419A0020  beq cr6, 0x824fe898
	if ctx.cr[6].eq {
	pc = 0x824FE898; continue 'dispatch;
	}
	// 824FE87C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FE880: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 824FE884: 38C00027  li r6, 0x27
	ctx.r[6].s64 = 39;
	// 824FE888: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FE88C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824FE890: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 824FE894: 4BF65825  bl 0x824640b8
	ctx.lr = 0x824FE898;
	sub_824640B8(ctx, base);
	pc = 0x824FE898; continue 'dispatch;
            }
            0x824FE898 => {
    //   block [0x824FE898..0x824FE8B4)
	// 824FE898: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824FE89C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824FE8A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824FE8A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824FE8A8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824FE8AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824FE8B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FE8B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FE8B8 size=64
    let mut pc: u32 = 0x824FE8B8;
    'dispatch: loop {
        match pc {
            0x824FE8B8 => {
    //   block [0x824FE8B8..0x824FE8F8)
	// 824FE8B8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824FE8BC: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 824FE8C0: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 824FE8C4: 3D008206  lis r8, -0x7dfa
	ctx.r[8].s64 = -2113536000;
	// 824FE8C8: 3CE08206  lis r7, -0x7dfa
	ctx.r[7].s64 = -2113536000;
	// 824FE8CC: 396BFBA4  addi r11, r11, -0x45c
	ctx.r[11].s64 = ctx.r[11].s64 + -1116;
	// 824FE8D0: 394AFB98  addi r10, r10, -0x468
	ctx.r[10].s64 = ctx.r[10].s64 + -1128;
	// 824FE8D4: 3929FB84  addi r9, r9, -0x47c
	ctx.r[9].s64 = ctx.r[9].s64 + -1148;
	// 824FE8D8: 3908FB78  addi r8, r8, -0x488
	ctx.r[8].s64 = ctx.r[8].s64 + -1160;
	// 824FE8DC: 38E7FB6C  addi r7, r7, -0x494
	ctx.r[7].s64 = ctx.r[7].s64 + -1172;
	// 824FE8E0: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824FE8E4: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 824FE8E8: 9123000C  stw r9, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 824FE8EC: 91030010  stw r8, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 824FE8F0: 90E30014  stw r7, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[7].u32 ) };
	// 824FE8F4: 4BF9437C  b 0x82492c70
	sub_82492C70(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FE8F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FE8F8 size=32
    let mut pc: u32 = 0x824FE8F8;
    'dispatch: loop {
        match pc {
            0x824FE8F8 => {
    //   block [0x824FE8F8..0x824FE918)
	// 824FE8F8: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FE8FC: 81460000  lwz r10, 0(r6)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FE900: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 824FE904: 396B0044  addi r11, r11, 0x44
	ctx.r[11].s64 = ctx.r[11].s64 + 68;
	// 824FE908: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824FE90C: 7D6B502E  lwzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824FE910: 5563BFFE  rlwinm r3, r11, 0x17, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000001FFu64;
	// 824FE914: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FE918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824FE918 size=376
    let mut pc: u32 = 0x824FE918;
    'dispatch: loop {
        match pc {
            0x824FE918 => {
    //   block [0x824FE918..0x824FEA80)
	// 824FE918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FE91C: 48036795  bl 0x825350b0
	ctx.lr = 0x824FE920;
	sub_82535080(ctx, base);
	// 824FE920: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FE924: 89640005  lbz r11, 5(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(5 as u32) ) } as u64;
	// 824FE928: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824FE92C: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 824FE930: 7F6B2214  add r27, r11, r4
	ctx.r[27].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 824FE934: 835B0000  lwz r26, 0(r27)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FE938: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 824FE93C: 419A0144  beq cr6, 0x824fea80
	if ctx.cr[6].eq {
	pc = 0x824FEA80; continue 'dispatch;
	}
	// 824FE940: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824FE944: 54AA083C  slwi r10, r5, 1
	ctx.r[10].u32 = ctx.r[5].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824FE948: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FE94C: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 824FE950: 7D455214  add r10, r5, r10
	ctx.r[10].u64 = ctx.r[5].u64 + ctx.r[10].u64;
	// 824FE954: 38610051  addi r3, r1, 0x51
	ctx.r[3].s64 = ctx.r[1].s64 + 81;
	// 824FE958: 555D2036  slwi r29, r10, 4
	ctx.r[29].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 824FE95C: 81240000  lwz r9, 0(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FE960: 7CABEA14  add r5, r11, r29
	ctx.r[5].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 824FE964: 81490004  lwz r10, 4(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FE968: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 824FE96C: 4E800421  bctrl
	ctx.lr = 0x824FE970;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824FE970: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FE974: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824FE978: 419A0108  beq cr6, 0x824fea80
	if ctx.cr[6].eq {
	pc = 0x824FEA80; continue 'dispatch;
	}
	// 824FE97C: 83DB0008  lwz r30, 8(r27)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 824FE980: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 824FE984: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FE988: 38BF0010  addi r5, r31, 0x10
	ctx.r[5].s64 = ctx.r[31].s64 + 16;
	// 824FE98C: 393E0030  addi r9, r30, 0x30
	ctx.r[9].s64 = ctx.r[30].s64 + 48;
	// 824FE990: 80DF000C  lwz r6, 0xc(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 824FE994: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 824FE998: 3BBE0010  addi r29, r30, 0x10
	ctx.r[29].s64 = ctx.r[30].s64 + 16;
	// 824FE99C: 3B9E0020  addi r28, r30, 0x20
	ctx.r[28].s64 = ctx.r[30].s64 + 32;
            }
            0x824FEA80 => {
    //   block [0x824FEA80..0x824FEA90)
	// 824FEA80: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 824FEA84: C02B0010  lfs f1, 0x10(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 824FEA88: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 824FEA8C: 48036674  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FEA90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824FEA90 size=296
    let mut pc: u32 = 0x824FEA90;
    'dispatch: loop {
        match pc {
            0x824FEA90 => {
    //   block [0x824FEA90..0x824FEAE4)
	// 824FEA90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824FEA94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824FEA98: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824FEA9C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824FEAA0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824FEAA4: 83CD0000  lwz r30, 0(r13)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FEAA8: 3BE00014  li r31, 0x14
	ctx.r[31].s64 = 20;
	// 824FEAAC: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 824FEAB0: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 824FEAB4: 7D7FF02E  lwzx r11, r31, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 824FEAB8: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824FEABC: 810B000C  lwz r8, 0xc(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 824FEAC0: 7F0A4040  cmplw cr6, r10, r8
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[8].u32, &mut ctx.xer);
	// 824FEAC4: 40980020  bge cr6, 0x824feae4
	if !ctx.cr[6].lt {
	pc = 0x824FEAE4; continue 'dispatch;
	}
	// 824FEAC8: 3D008206  lis r8, -0x7dfa
	ctx.r[8].s64 = -2113536000;
	// 824FEACC: 39084324  addi r8, r8, 0x4324
	ctx.r[8].s64 = ctx.r[8].s64 + 17188;
	// 824FEAD0: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 824FEAD4: 7D0C42E6  mftb r8, 0x10c
	ctx.r[8].u64 = crate::rt::rdtsc_u64();
	// 824FEAD8: 388A000C  addi r4, r10, 0xc
	ctx.r[4].s64 = ctx.r[10].s64 + 12;
	// 824FEADC: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 824FEAE0: 908B0004  stw r4, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	pc = 0x824FEAE4; continue 'dispatch;
            }
            0x824FEAE4 => {
    //   block [0x824FEAE4..0x824FEB00)
	// 824FEAE4: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 824FEAE8: 90A90004  stw r5, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 824FEAEC: 90E9000C  stw r7, 0xc(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 824FEAF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 824FEAF4: 39660014  addi r11, r6, 0x14
	ctx.r[11].s64 = ctx.r[6].s64 + 20;
	// 824FEAF8: 409A0008  bne cr6, 0x824feb00
	if !ctx.cr[6].eq {
	pc = 0x824FEB00; continue 'dispatch;
	}
	// 824FEAFC: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	pc = 0x824FEB00; continue 'dispatch;
            }
            0x824FEB00 => {
    //   block [0x824FEB00..0x824FEB20)
	// 824FEB00: 91690008  stw r11, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 824FEB04: 89650020  lbz r11, 0x20(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(32 as u32) ) } as u64;
	// 824FEB08: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824FEB0C: 419A001C  beq cr6, 0x824feb28
	if ctx.cr[6].eq {
	pc = 0x824FEB28; continue 'dispatch;
	}
	// 824FEB10: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 824FEB14: 39660010  addi r11, r6, 0x10
	ctx.r[11].s64 = ctx.r[6].s64 + 16;
	// 824FEB18: 409A0008  bne cr6, 0x824feb20
	if !ctx.cr[6].eq {
	pc = 0x824FEB20; continue 'dispatch;
	}
	// 824FEB1C: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	pc = 0x824FEB20; continue 'dispatch;
            }
            0x824FEB20 => {
    //   block [0x824FEB20..0x824FEB28)
	// 824FEB20: 91690034  stw r11, 0x34(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 824FEB24: 48000008  b 0x824feb2c
	pc = 0x824FEB2C; continue 'dispatch;
            }
            0x824FEB28 => {
    //   block [0x824FEB28..0x824FEB2C)
	// 824FEB28: 91490034  stw r10, 0x34(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(52 as u32), ctx.r[10].u32 ) };
	pc = 0x824FEB2C; continue 'dispatch;
            }
            0x824FEB2C => {
    //   block [0x824FEB2C..0x824FEBB8)
	// 824FEB2C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


