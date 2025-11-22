pub fn sub_82ED7500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7500 size=12
    let mut pc: u32 = 0x82ED7500;
    'dispatch: loop {
        match pc {
            0x82ED7500 => {
    //   block [0x82ED7500..0x82ED750C)
	// 82ED7500: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82ED7504: 386B7904  addi r3, r11, 0x7904
	ctx.r[3].s64 = ctx.r[11].s64 + 30980;
	// 82ED7508: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7510 size=4
    let mut pc: u32 = 0x82ED7510;
    'dispatch: loop {
        match pc {
            0x82ED7510 => {
    //   block [0x82ED7510..0x82ED7514)
	// 82ED7510: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7518 size=4
    let mut pc: u32 = 0x82ED7518;
    'dispatch: loop {
        match pc {
            0x82ED7518 => {
    //   block [0x82ED7518..0x82ED751C)
	// 82ED7518: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7520 size=20
    let mut pc: u32 = 0x82ED7520;
    'dispatch: loop {
        match pc {
            0x82ED7520 => {
    //   block [0x82ED7520..0x82ED7534)
	// 82ED7520: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED7524: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82ED7528: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED752C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED7530: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7538 size=4
    let mut pc: u32 = 0x82ED7538;
    'dispatch: loop {
        match pc {
            0x82ED7538 => {
    //   block [0x82ED7538..0x82ED753C)
	// 82ED7538: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7540 size=8
    let mut pc: u32 = 0x82ED7540;
    'dispatch: loop {
        match pc {
            0x82ED7540 => {
    //   block [0x82ED7540..0x82ED7548)
	// 82ED7540: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ED7544: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7548 size=24
    let mut pc: u32 = 0x82ED7548;
    'dispatch: loop {
        match pc {
            0x82ED7548 => {
    //   block [0x82ED7548..0x82ED7560)
	// 82ED7548: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82ED754C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82ED7550: 392B7B5C  addi r9, r11, 0x7b5c
	ctx.r[9].s64 = ctx.r[11].s64 + 31580;
	// 82ED7554: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82ED7558: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82ED755C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7560 size=12
    let mut pc: u32 = 0x82ED7560;
    'dispatch: loop {
        match pc {
            0x82ED7560 => {
    //   block [0x82ED7560..0x82ED756C)
	// 82ED7560: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82ED7564: 386B7B5C  addi r3, r11, 0x7b5c
	ctx.r[3].s64 = ctx.r[11].s64 + 31580;
	// 82ED7568: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED7570 size=100
    let mut pc: u32 = 0x82ED7570;
    'dispatch: loop {
        match pc {
            0x82ED7570 => {
    //   block [0x82ED7570..0x82ED75D4)
	// 82ED7570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED7574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ED7578: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82ED757C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ED7580: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED7584: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ED7588: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82ED758C: 4801A7C5  bl 0x82ef1d50
	ctx.lr = 0x82ED7590;
	sub_82EF1D50(ctx, base);
	// 82ED7590: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82ED7594: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED7598: 419A0020  beq cr6, 0x82ed75b8
	if ctx.cr[6].eq {
	pc = 0x82ED75B8; continue 'dispatch;
	}
	// 82ED759C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED75A0: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82ED75A4: 38C0002C  li r6, 0x2c
	ctx.r[6].s64 = 44;
	// 82ED75A8: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED75AC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82ED75B0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82ED75B4: 4BFC91FD  bl 0x82ea07b0
	ctx.lr = 0x82ED75B8;
	sub_82EA07B0(ctx, base);
	// 82ED75B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED75BC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82ED75C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ED75C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ED75C8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82ED75CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ED75D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED75D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED75D8 size=4
    let mut pc: u32 = 0x82ED75D8;
    'dispatch: loop {
        match pc {
            0x82ED75D8 => {
    //   block [0x82ED75D8..0x82ED75DC)
	// 82ED75D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED75E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED75E0 size=20
    let mut pc: u32 = 0x82ED75E0;
    'dispatch: loop {
        match pc {
            0x82ED75E0 => {
    //   block [0x82ED75E0..0x82ED75F4)
	// 82ED75E0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED75E4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82ED75E8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED75EC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED75F0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED75F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED75F8 size=4
    let mut pc: u32 = 0x82ED75F8;
    'dispatch: loop {
        match pc {
            0x82ED75F8 => {
    //   block [0x82ED75F8..0x82ED75FC)
	// 82ED75F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7600 size=8
    let mut pc: u32 = 0x82ED7600;
    'dispatch: loop {
        match pc {
            0x82ED7600 => {
    //   block [0x82ED7600..0x82ED7608)
	// 82ED7600: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ED7604: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7608 size=24
    let mut pc: u32 = 0x82ED7608;
    'dispatch: loop {
        match pc {
            0x82ED7608 => {
    //   block [0x82ED7608..0x82ED7620)
	// 82ED7608: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82ED760C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82ED7610: 392B7C64  addi r9, r11, 0x7c64
	ctx.r[9].s64 = ctx.r[11].s64 + 31844;
	// 82ED7614: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82ED7618: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82ED761C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7620 size=12
    let mut pc: u32 = 0x82ED7620;
    'dispatch: loop {
        match pc {
            0x82ED7620 => {
    //   block [0x82ED7620..0x82ED762C)
	// 82ED7620: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82ED7624: 386B7C64  addi r3, r11, 0x7c64
	ctx.r[3].s64 = ctx.r[11].s64 + 31844;
	// 82ED7628: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7630 size=4
    let mut pc: u32 = 0x82ED7630;
    'dispatch: loop {
        match pc {
            0x82ED7630 => {
    //   block [0x82ED7630..0x82ED7634)
	// 82ED7630: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7638 size=20
    let mut pc: u32 = 0x82ED7638;
    'dispatch: loop {
        match pc {
            0x82ED7638 => {
    //   block [0x82ED7638..0x82ED764C)
	// 82ED7638: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED763C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82ED7640: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED7644: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED7648: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7650 size=4
    let mut pc: u32 = 0x82ED7650;
    'dispatch: loop {
        match pc {
            0x82ED7650 => {
    //   block [0x82ED7650..0x82ED7654)
	// 82ED7650: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7658 size=8
    let mut pc: u32 = 0x82ED7658;
    'dispatch: loop {
        match pc {
            0x82ED7658 => {
    //   block [0x82ED7658..0x82ED7660)
	// 82ED7658: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ED765C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7660 size=24
    let mut pc: u32 = 0x82ED7660;
    'dispatch: loop {
        match pc {
            0x82ED7660 => {
    //   block [0x82ED7660..0x82ED7678)
	// 82ED7660: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82ED7664: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82ED7668: 392B7D44  addi r9, r11, 0x7d44
	ctx.r[9].s64 = ctx.r[11].s64 + 32068;
	// 82ED766C: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82ED7670: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82ED7674: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7678 size=12
    let mut pc: u32 = 0x82ED7678;
    'dispatch: loop {
        match pc {
            0x82ED7678 => {
    //   block [0x82ED7678..0x82ED7684)
	// 82ED7678: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82ED767C: 386B7D44  addi r3, r11, 0x7d44
	ctx.r[3].s64 = ctx.r[11].s64 + 32068;
	// 82ED7680: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7688 size=4
    let mut pc: u32 = 0x82ED7688;
    'dispatch: loop {
        match pc {
            0x82ED7688 => {
    //   block [0x82ED7688..0x82ED768C)
	// 82ED7688: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7690 size=12
    let mut pc: u32 = 0x82ED7690;
    'dispatch: loop {
        match pc {
            0x82ED7690 => {
    //   block [0x82ED7690..0x82ED769C)
	// 82ED7690: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82ED7694: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ED7698: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED769C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED769C size=8
    let mut pc: u32 = 0x82ED769C;
    'dispatch: loop {
        match pc {
            0x82ED769C => {
    //   block [0x82ED769C..0x82ED76A4)
	// 82ED769C: 4801B71C  b 0x82ef2db8
	sub_82EF2DB8(ctx, base);
	return;
	// 82ED76A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED76A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED76A8 size=8
    let mut pc: u32 = 0x82ED76A8;
    'dispatch: loop {
        match pc {
            0x82ED76A8 => {
    //   block [0x82ED76A8..0x82ED76B0)
	// 82ED76A8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82ED76AC: 480000DC  b 0x82ed7788
	sub_82ED7788(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED76B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED76B0 size=216
    let mut pc: u32 = 0x82ED76B0;
    'dispatch: loop {
        match pc {
            0x82ED76B0 => {
    //   block [0x82ED76B0..0x82ED7788)
	// 82ED76B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED76B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ED76B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ED76BC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED76C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ED76C4: 817F003C  lwz r11, 0x3c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 82ED76C8: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED76CC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ED76D0: 409A0020  bne cr6, 0x82ed76f0
	if !ctx.cr[6].eq {
	pc = 0x82ED76F0; continue 'dispatch;
	}
	// 82ED76D4: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED76D8: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82ED76DC: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82ED76E0: 809F0034  lwz r4, 0x34(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82ED76E4: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82ED76E8: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82ED76EC: 4BFC90C5  bl 0x82ea07b0
	ctx.lr = 0x82ED76F0;
	sub_82EA07B0(ctx, base);
	// 82ED76F0: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82ED76F4: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED76F8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ED76FC: 409A0020  bne cr6, 0x82ed771c
	if !ctx.cr[6].eq {
	pc = 0x82ED771C; continue 'dispatch;
	}
	// 82ED7700: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED7704: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82ED7708: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82ED770C: 809F0028  lwz r4, 0x28(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82ED7710: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82ED7714: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82ED7718: 4BFC9099  bl 0x82ea07b0
	ctx.lr = 0x82ED771C;
	sub_82EA07B0(ctx, base);
	// 82ED771C: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82ED7720: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED7724: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ED7728: 409A0020  bne cr6, 0x82ed7748
	if !ctx.cr[6].eq {
	pc = 0x82ED7748; continue 'dispatch;
	}
	// 82ED772C: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED7730: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82ED7734: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82ED7738: 809F001C  lwz r4, 0x1c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82ED773C: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82ED7740: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82ED7744: 4BFC906D  bl 0x82ea07b0
	ctx.lr = 0x82ED7748;
	sub_82EA07B0(ctx, base);
	// 82ED7748: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82ED774C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED7750: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ED7754: 409A0020  bne cr6, 0x82ed7774
	if !ctx.cr[6].eq {
	pc = 0x82ED7774; continue 'dispatch;
	}
	// 82ED7758: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED775C: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82ED7760: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82ED7764: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82ED7768: 55652036  slwi r5, r11, 4
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82ED776C: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82ED7770: 4BFC9041  bl 0x82ea07b0
	ctx.lr = 0x82ED7774;
	sub_82EA07B0(ctx, base);
	// 82ED7774: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82ED7778: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ED777C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ED7780: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ED7784: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED7788 size=160
    let mut pc: u32 = 0x82ED7788;
    'dispatch: loop {
        match pc {
            0x82ED7788 => {
    //   block [0x82ED7788..0x82ED7828)
	// 82ED7788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED778C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ED7790: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82ED7794: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ED7798: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED779C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ED77A0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82ED77A4: 4BFFFF0D  bl 0x82ed76b0
	ctx.lr = 0x82ED77A8;
	sub_82ED76B0(ctx, base);
	// 82ED77A8: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82ED77AC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED77B0: 419A005C  beq cr6, 0x82ed780c
	if ctx.cr[6].eq {
	pc = 0x82ED780C; continue 'dispatch;
	}
	// 82ED77B4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82ED77B8: 419A0054  beq cr6, 0x82ed780c
	if ctx.cr[6].eq {
	pc = 0x82ED780C; continue 'dispatch;
	}
	// 82ED77BC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED77C0: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82ED77C4: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82ED77C8: 812B0064  lwz r9, 0x64(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(100 as u32) ) } as u64;
	// 82ED77CC: 810B0034  lwz r8, 0x34(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82ED77D0: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82ED77D4: 4198001C  blt cr6, 0x82ed77f0
	if ctx.cr[6].lt {
	pc = 0x82ED77F0; continue 'dispatch;
	}
	// 82ED77D8: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 82ED77DC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82ED77E0: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 82ED77E4: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82ED77E8: 4BFC8879  bl 0x82ea0060
	ctx.lr = 0x82ED77EC;
	sub_82EA0060(ctx, base);
	// 82ED77EC: 48000020  b 0x82ed780c
	pc = 0x82ED780C; continue 'dispatch;
	// 82ED77F0: 812B0064  lwz r9, 0x64(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(100 as u32) ) } as u64;
	// 82ED77F4: 394B0060  addi r10, r11, 0x60
	ctx.r[10].s64 = ctx.r[11].s64 + 96;
	// 82ED77F8: 814B0060  lwz r10, 0x60(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(96 as u32) ) } as u64;
	// 82ED77FC: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82ED7800: 912B0064  stw r9, 0x64(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(100 as u32), ctx.r[9].u32 ) };
	// 82ED7804: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82ED7808: 93EB0060  stw r31, 0x60(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(96 as u32), ctx.r[31].u32 ) };
	// 82ED780C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED7810: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82ED7814: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ED7818: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ED781C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82ED7820: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ED7824: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7828 size=4
    let mut pc: u32 = 0x82ED7828;
    'dispatch: loop {
        match pc {
            0x82ED7828 => {
    //   block [0x82ED7828..0x82ED782C)
	// 82ED7828: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7830 size=4
    let mut pc: u32 = 0x82ED7830;
    'dispatch: loop {
        match pc {
            0x82ED7830 => {
    //   block [0x82ED7830..0x82ED7834)
	// 82ED7830: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7838 size=20
    let mut pc: u32 = 0x82ED7838;
    'dispatch: loop {
        match pc {
            0x82ED7838 => {
    //   block [0x82ED7838..0x82ED784C)
	// 82ED7838: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED783C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82ED7840: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED7844: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED7848: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7850 size=12
    let mut pc: u32 = 0x82ED7850;
    'dispatch: loop {
        match pc {
            0x82ED7850 => {
    //   block [0x82ED7850..0x82ED785C)
	// 82ED7850: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82ED7854: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ED7858: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED785C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED785C size=8
    let mut pc: u32 = 0x82ED785C;
    'dispatch: loop {
        match pc {
            0x82ED785C => {
    //   block [0x82ED785C..0x82ED7864)
	// 82ED785C: 48001A64  b 0x82ed92c0
	sub_82ED92C0(ctx, base);
	return;
	// 82ED7860: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED7868 size=44
    let mut pc: u32 = 0x82ED7868;
    'dispatch: loop {
        match pc {
            0x82ED7868 => {
    //   block [0x82ED7868..0x82ED7894)
	// 82ED7868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED786C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ED7870: 9421FDA0  stwu r1, -0x260(r1)
	ea = ctx.r[1].u32.wrapping_add(-608 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED7874: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82ED7878: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82ED787C: 48001A45  bl 0x82ed92c0
	ctx.lr = 0x82ED7880;
	sub_82ED92C0(ctx, base);
	// 82ED7880: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82ED7884: 38210260  addi r1, r1, 0x260
	ctx.r[1].s64 = ctx.r[1].s64 + 608;
	// 82ED7888: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ED788C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ED7890: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7898 size=8
    let mut pc: u32 = 0x82ED7898;
    'dispatch: loop {
        match pc {
            0x82ED7898 => {
    //   block [0x82ED7898..0x82ED78A0)
	// 82ED7898: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82ED789C: 48000084  b 0x82ed7920
	sub_82ED7920(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED78A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED78A0 size=128
    let mut pc: u32 = 0x82ED78A0;
    'dispatch: loop {
        match pc {
            0x82ED78A0 => {
    //   block [0x82ED78A0..0x82ED7920)
	// 82ED78A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED78A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ED78A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ED78AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED78B0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ED78B4: A17F000E  lhz r11, 0xe(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(14 as u32) ) } as u64;
	// 82ED78B8: 556A0020  rlwinm r10, r11, 0, 0, 0x10
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED78BC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82ED78C0: 409A0020  bne cr6, 0x82ed78e0
	if !ctx.cr[6].eq {
	pc = 0x82ED78E0; continue 'dispatch;
	}
	// 82ED78C4: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED78C8: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82ED78CC: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82ED78D0: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED78D4: 5565143A  rlwinm r5, r11, 2, 0x10, 0x1d
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82ED78D8: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82ED78DC: 4BFC8ED5  bl 0x82ea07b0
	ctx.lr = 0x82ED78E0;
	sub_82EA07B0(ctx, base);
	// 82ED78E0: A17F0006  lhz r11, 6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 82ED78E4: 556A0020  rlwinm r10, r11, 0, 0, 0x10
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED78E8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82ED78EC: 409A0020  bne cr6, 0x82ed790c
	if !ctx.cr[6].eq {
	pc = 0x82ED790C; continue 'dispatch;
	}
	// 82ED78F0: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED78F4: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82ED78F8: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82ED78FC: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED7900: 5565143A  rlwinm r5, r11, 2, 0x10, 0x1d
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82ED7904: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82ED7908: 4BFC8EA9  bl 0x82ea07b0
	ctx.lr = 0x82ED790C;
	sub_82EA07B0(ctx, base);
	// 82ED790C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82ED7910: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ED7914: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ED7918: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ED791C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED7920 size=160
    let mut pc: u32 = 0x82ED7920;
    'dispatch: loop {
        match pc {
            0x82ED7920 => {
    //   block [0x82ED7920..0x82ED79C0)
	// 82ED7920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED7924: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ED7928: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82ED792C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ED7930: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED7934: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ED7938: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82ED793C: 4BFFFF65  bl 0x82ed78a0
	ctx.lr = 0x82ED7940;
	sub_82ED78A0(ctx, base);
	// 82ED7940: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82ED7944: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED7948: 419A005C  beq cr6, 0x82ed79a4
	if ctx.cr[6].eq {
	pc = 0x82ED79A4; continue 'dispatch;
	}
	// 82ED794C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82ED7950: 419A0054  beq cr6, 0x82ed79a4
	if ctx.cr[6].eq {
	pc = 0x82ED79A4; continue 'dispatch;
	}
	// 82ED7954: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED7958: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82ED795C: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82ED7960: 812B004C  lwz r9, 0x4c(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 82ED7964: 810B0034  lwz r8, 0x34(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82ED7968: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82ED796C: 4198001C  blt cr6, 0x82ed7988
	if ctx.cr[6].lt {
	pc = 0x82ED7988; continue 'dispatch;
	}
	// 82ED7970: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 82ED7974: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82ED7978: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 82ED797C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82ED7980: 4BFC86E1  bl 0x82ea0060
	ctx.lr = 0x82ED7984;
	sub_82EA0060(ctx, base);
	// 82ED7984: 48000020  b 0x82ed79a4
	pc = 0x82ED79A4; continue 'dispatch;
	// 82ED7988: 812B004C  lwz r9, 0x4c(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 82ED798C: 394B0048  addi r10, r11, 0x48
	ctx.r[10].s64 = ctx.r[11].s64 + 72;
	// 82ED7990: 814B0048  lwz r10, 0x48(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 82ED7994: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82ED7998: 912B004C  stw r9, 0x4c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(76 as u32), ctx.r[9].u32 ) };
	// 82ED799C: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82ED79A0: 93EB0048  stw r31, 0x48(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(72 as u32), ctx.r[31].u32 ) };
	// 82ED79A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED79A8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82ED79AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ED79B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ED79B4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82ED79B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ED79BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED79C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED79C0 size=4
    let mut pc: u32 = 0x82ED79C0;
    'dispatch: loop {
        match pc {
            0x82ED79C0 => {
    //   block [0x82ED79C0..0x82ED79C4)
	// 82ED79C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED79C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED79C8 size=12
    let mut pc: u32 = 0x82ED79C8;
    'dispatch: loop {
        match pc {
            0x82ED79C8 => {
    //   block [0x82ED79C8..0x82ED79D4)
	// 82ED79C8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82ED79CC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ED79D0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED79D4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED79D4 size=8
    let mut pc: u32 = 0x82ED79D4;
    'dispatch: loop {
        match pc {
            0x82ED79D4 => {
    //   block [0x82ED79D4..0x82ED79DC)
	// 82ED79D4: 4801B574  b 0x82ef2f48
	sub_82EF2F48(ctx, base);
	return;
	// 82ED79D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED79E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED79E0 size=20
    let mut pc: u32 = 0x82ED79E0;
    'dispatch: loop {
        match pc {
            0x82ED79E0 => {
    //   block [0x82ED79E0..0x82ED79F4)
	// 82ED79E0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED79E4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82ED79E8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED79EC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED79F0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED79F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED79F8 size=44
    let mut pc: u32 = 0x82ED79F8;
    'dispatch: loop {
        match pc {
            0x82ED79F8 => {
    //   block [0x82ED79F8..0x82ED7A24)
	// 82ED79F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED79FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ED7A00: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED7A04: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82ED7A08: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82ED7A0C: 4801B53D  bl 0x82ef2f48
	ctx.lr = 0x82ED7A10;
	sub_82EF2F48(ctx, base);
	// 82ED7A10: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82ED7A14: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82ED7A18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ED7A1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ED7A20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7A28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7A28 size=4
    let mut pc: u32 = 0x82ED7A28;
    'dispatch: loop {
        match pc {
            0x82ED7A28 => {
    //   block [0x82ED7A28..0x82ED7A2C)
	// 82ED7A28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7A30 size=20
    let mut pc: u32 = 0x82ED7A30;
    'dispatch: loop {
        match pc {
            0x82ED7A30 => {
    //   block [0x82ED7A30..0x82ED7A44)
	// 82ED7A30: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED7A34: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82ED7A38: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED7A3C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED7A40: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7A48 size=4
    let mut pc: u32 = 0x82ED7A48;
    'dispatch: loop {
        match pc {
            0x82ED7A48 => {
    //   block [0x82ED7A48..0x82ED7A4C)
	// 82ED7A48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7A50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7A50 size=8
    let mut pc: u32 = 0x82ED7A50;
    'dispatch: loop {
        match pc {
            0x82ED7A50 => {
    //   block [0x82ED7A50..0x82ED7A58)
	// 82ED7A50: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ED7A54: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7A58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7A58 size=24
    let mut pc: u32 = 0x82ED7A58;
    'dispatch: loop {
        match pc {
            0x82ED7A58 => {
    //   block [0x82ED7A58..0x82ED7A70)
	// 82ED7A58: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82ED7A5C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82ED7A60: 392B862C  addi r9, r11, -0x79d4
	ctx.r[9].s64 = ctx.r[11].s64 + -31188;
	// 82ED7A64: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82ED7A68: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82ED7A6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7A70 size=12
    let mut pc: u32 = 0x82ED7A70;
    'dispatch: loop {
        match pc {
            0x82ED7A70 => {
    //   block [0x82ED7A70..0x82ED7A7C)
	// 82ED7A70: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82ED7A74: 386B862C  addi r3, r11, -0x79d4
	ctx.r[3].s64 = ctx.r[11].s64 + -31188;
	// 82ED7A78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7A80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7A80 size=12
    let mut pc: u32 = 0x82ED7A80;
    'dispatch: loop {
        match pc {
            0x82ED7A80 => {
    //   block [0x82ED7A80..0x82ED7A8C)
	// 82ED7A80: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82ED7A84: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ED7A88: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7A8C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7A8C size=8
    let mut pc: u32 = 0x82ED7A8C;
    'dispatch: loop {
        match pc {
            0x82ED7A8C => {
    //   block [0x82ED7A8C..0x82ED7A94)
	// 82ED7A8C: 480030D4  b 0x82edab60
	sub_82EDAB60(ctx, base);
	return;
	// 82ED7A90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7A98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7A98 size=20
    let mut pc: u32 = 0x82ED7A98;
    'dispatch: loop {
        match pc {
            0x82ED7A98 => {
    //   block [0x82ED7A98..0x82ED7AAC)
	// 82ED7A98: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED7A9C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82ED7AA0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED7AA4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED7AA8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7AB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED7AB0 size=44
    let mut pc: u32 = 0x82ED7AB0;
    'dispatch: loop {
        match pc {
            0x82ED7AB0 => {
    //   block [0x82ED7AB0..0x82ED7ADC)
	// 82ED7AB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED7AB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ED7AB8: 9421FED0  stwu r1, -0x130(r1)
	ea = ctx.r[1].u32.wrapping_add(-304 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED7ABC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82ED7AC0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82ED7AC4: 4800309D  bl 0x82edab60
	ctx.lr = 0x82ED7AC8;
	sub_82EDAB60(ctx, base);
	// 82ED7AC8: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82ED7ACC: 38210130  addi r1, r1, 0x130
	ctx.r[1].s64 = ctx.r[1].s64 + 304;
	// 82ED7AD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ED7AD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ED7AD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7AE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7AE0 size=12
    let mut pc: u32 = 0x82ED7AE0;
    'dispatch: loop {
        match pc {
            0x82ED7AE0 => {
    //   block [0x82ED7AE0..0x82ED7AEC)
	// 82ED7AE0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82ED7AE4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ED7AE8: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7AEC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7AEC size=8
    let mut pc: u32 = 0x82ED7AEC;
    'dispatch: loop {
        match pc {
            0x82ED7AEC => {
    //   block [0x82ED7AEC..0x82ED7AF4)
	// 82ED7AEC: 4BFF9404  b 0x82ed0ef0
	sub_82ED0EF0(ctx, base);
	return;
	// 82ED7AF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7AF8 size=20
    let mut pc: u32 = 0x82ED7AF8;
    'dispatch: loop {
        match pc {
            0x82ED7AF8 => {
    //   block [0x82ED7AF8..0x82ED7B0C)
	// 82ED7AF8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED7AFC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82ED7B00: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED7B04: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED7B08: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7B10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED7B10 size=44
    let mut pc: u32 = 0x82ED7B10;
    'dispatch: loop {
        match pc {
            0x82ED7B10 => {
    //   block [0x82ED7B10..0x82ED7B3C)
	// 82ED7B10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED7B14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ED7B18: 9421FC90  stwu r1, -0x370(r1)
	ea = ctx.r[1].u32.wrapping_add(-880 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED7B1C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82ED7B20: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82ED7B24: 4BFF93CD  bl 0x82ed0ef0
	ctx.lr = 0x82ED7B28;
	sub_82ED0EF0(ctx, base);
	// 82ED7B28: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82ED7B2C: 38210370  addi r1, r1, 0x370
	ctx.r[1].s64 = ctx.r[1].s64 + 880;
	// 82ED7B30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ED7B34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ED7B38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7B40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7B40 size=20
    let mut pc: u32 = 0x82ED7B40;
    'dispatch: loop {
        match pc {
            0x82ED7B40 => {
    //   block [0x82ED7B40..0x82ED7B54)
	// 82ED7B40: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED7B44: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82ED7B48: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED7B4C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED7B50: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7B58 size=8
    let mut pc: u32 = 0x82ED7B58;
    'dispatch: loop {
        match pc {
            0x82ED7B58 => {
    //   block [0x82ED7B58..0x82ED7B60)
	// 82ED7B58: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ED7B5C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7B60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7B60 size=24
    let mut pc: u32 = 0x82ED7B60;
    'dispatch: loop {
        match pc {
            0x82ED7B60 => {
    //   block [0x82ED7B60..0x82ED7B78)
	// 82ED7B60: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82ED7B64: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82ED7B68: 392B9F9C  addi r9, r11, -0x6064
	ctx.r[9].s64 = ctx.r[11].s64 + -24676;
	// 82ED7B6C: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82ED7B70: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82ED7B74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7B78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7B78 size=12
    let mut pc: u32 = 0x82ED7B78;
    'dispatch: loop {
        match pc {
            0x82ED7B78 => {
    //   block [0x82ED7B78..0x82ED7B84)
	// 82ED7B78: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82ED7B7C: 386B9F9C  addi r3, r11, -0x6064
	ctx.r[3].s64 = ctx.r[11].s64 + -24676;
	// 82ED7B80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7B88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7B88 size=20
    let mut pc: u32 = 0x82ED7B88;
    'dispatch: loop {
        match pc {
            0x82ED7B88 => {
    //   block [0x82ED7B88..0x82ED7B9C)
	// 82ED7B88: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED7B8C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82ED7B90: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED7B94: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED7B98: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7BA0 size=8
    let mut pc: u32 = 0x82ED7BA0;
    'dispatch: loop {
        match pc {
            0x82ED7BA0 => {
    //   block [0x82ED7BA0..0x82ED7BA8)
	// 82ED7BA0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ED7BA4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7BA8 size=24
    let mut pc: u32 = 0x82ED7BA8;
    'dispatch: loop {
        match pc {
            0x82ED7BA8 => {
    //   block [0x82ED7BA8..0x82ED7BC0)
	// 82ED7BA8: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82ED7BAC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82ED7BB0: 392BA034  addi r9, r11, -0x5fcc
	ctx.r[9].s64 = ctx.r[11].s64 + -24524;
	// 82ED7BB4: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82ED7BB8: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82ED7BBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7BC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7BC0 size=12
    let mut pc: u32 = 0x82ED7BC0;
    'dispatch: loop {
        match pc {
            0x82ED7BC0 => {
    //   block [0x82ED7BC0..0x82ED7BCC)
	// 82ED7BC0: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82ED7BC4: 386BA034  addi r3, r11, -0x5fcc
	ctx.r[3].s64 = ctx.r[11].s64 + -24524;
	// 82ED7BC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7BD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7BD0 size=20
    let mut pc: u32 = 0x82ED7BD0;
    'dispatch: loop {
        match pc {
            0x82ED7BD0 => {
    //   block [0x82ED7BD0..0x82ED7BE4)
	// 82ED7BD0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED7BD4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82ED7BD8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED7BDC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED7BE0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7BE8 size=8
    let mut pc: u32 = 0x82ED7BE8;
    'dispatch: loop {
        match pc {
            0x82ED7BE8 => {
    //   block [0x82ED7BE8..0x82ED7BF0)
	// 82ED7BE8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ED7BEC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7BF0 size=24
    let mut pc: u32 = 0x82ED7BF0;
    'dispatch: loop {
        match pc {
            0x82ED7BF0 => {
    //   block [0x82ED7BF0..0x82ED7C08)
	// 82ED7BF0: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82ED7BF4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82ED7BF8: 392B3F9C  addi r9, r11, 0x3f9c
	ctx.r[9].s64 = ctx.r[11].s64 + 16284;
	// 82ED7BFC: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82ED7C00: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82ED7C04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7C08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7C08 size=12
    let mut pc: u32 = 0x82ED7C08;
    'dispatch: loop {
        match pc {
            0x82ED7C08 => {
    //   block [0x82ED7C08..0x82ED7C14)
	// 82ED7C08: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82ED7C0C: 386B3F9C  addi r3, r11, 0x3f9c
	ctx.r[3].s64 = ctx.r[11].s64 + 16284;
	// 82ED7C10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7C18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7C18 size=20
    let mut pc: u32 = 0x82ED7C18;
    'dispatch: loop {
        match pc {
            0x82ED7C18 => {
    //   block [0x82ED7C18..0x82ED7C2C)
	// 82ED7C18: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED7C1C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82ED7C20: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED7C24: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED7C28: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7C30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7C30 size=8
    let mut pc: u32 = 0x82ED7C30;
    'dispatch: loop {
        match pc {
            0x82ED7C30 => {
    //   block [0x82ED7C30..0x82ED7C38)
	// 82ED7C30: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ED7C34: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7C38 size=24
    let mut pc: u32 = 0x82ED7C38;
    'dispatch: loop {
        match pc {
            0x82ED7C38 => {
    //   block [0x82ED7C38..0x82ED7C50)
	// 82ED7C38: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82ED7C3C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82ED7C40: 392BA0CC  addi r9, r11, -0x5f34
	ctx.r[9].s64 = ctx.r[11].s64 + -24372;
	// 82ED7C44: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82ED7C48: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82ED7C4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7C50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7C50 size=12
    let mut pc: u32 = 0x82ED7C50;
    'dispatch: loop {
        match pc {
            0x82ED7C50 => {
    //   block [0x82ED7C50..0x82ED7C5C)
	// 82ED7C50: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82ED7C54: 386BA0CC  addi r3, r11, -0x5f34
	ctx.r[3].s64 = ctx.r[11].s64 + -24372;
	// 82ED7C58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7C60 size=20
    let mut pc: u32 = 0x82ED7C60;
    'dispatch: loop {
        match pc {
            0x82ED7C60 => {
    //   block [0x82ED7C60..0x82ED7C74)
	// 82ED7C60: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED7C64: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82ED7C68: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED7C6C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED7C70: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7C78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7C78 size=8
    let mut pc: u32 = 0x82ED7C78;
    'dispatch: loop {
        match pc {
            0x82ED7C78 => {
    //   block [0x82ED7C78..0x82ED7C80)
	// 82ED7C78: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ED7C7C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7C80 size=24
    let mut pc: u32 = 0x82ED7C80;
    'dispatch: loop {
        match pc {
            0x82ED7C80 => {
    //   block [0x82ED7C80..0x82ED7C98)
	// 82ED7C80: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82ED7C84: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82ED7C88: 392BA19C  addi r9, r11, -0x5e64
	ctx.r[9].s64 = ctx.r[11].s64 + -24164;
	// 82ED7C8C: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82ED7C90: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82ED7C94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7C98 size=12
    let mut pc: u32 = 0x82ED7C98;
    'dispatch: loop {
        match pc {
            0x82ED7C98 => {
    //   block [0x82ED7C98..0x82ED7CA4)
	// 82ED7C98: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82ED7C9C: 386BA19C  addi r3, r11, -0x5e64
	ctx.r[3].s64 = ctx.r[11].s64 + -24164;
	// 82ED7CA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7CA8 size=20
    let mut pc: u32 = 0x82ED7CA8;
    'dispatch: loop {
        match pc {
            0x82ED7CA8 => {
    //   block [0x82ED7CA8..0x82ED7CBC)
	// 82ED7CA8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED7CAC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82ED7CB0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED7CB4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED7CB8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7CC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7CC0 size=8
    let mut pc: u32 = 0x82ED7CC0;
    'dispatch: loop {
        match pc {
            0x82ED7CC0 => {
    //   block [0x82ED7CC0..0x82ED7CC8)
	// 82ED7CC0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ED7CC4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7CC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7CC8 size=24
    let mut pc: u32 = 0x82ED7CC8;
    'dispatch: loop {
        match pc {
            0x82ED7CC8 => {
    //   block [0x82ED7CC8..0x82ED7CE0)
	// 82ED7CC8: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82ED7CCC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82ED7CD0: 392B3F34  addi r9, r11, 0x3f34
	ctx.r[9].s64 = ctx.r[11].s64 + 16180;
	// 82ED7CD4: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82ED7CD8: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82ED7CDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7CE0 size=12
    let mut pc: u32 = 0x82ED7CE0;
    'dispatch: loop {
        match pc {
            0x82ED7CE0 => {
    //   block [0x82ED7CE0..0x82ED7CEC)
	// 82ED7CE0: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82ED7CE4: 386B3F34  addi r3, r11, 0x3f34
	ctx.r[3].s64 = ctx.r[11].s64 + 16180;
	// 82ED7CE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7CF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7CF0 size=20
    let mut pc: u32 = 0x82ED7CF0;
    'dispatch: loop {
        match pc {
            0x82ED7CF0 => {
    //   block [0x82ED7CF0..0x82ED7D04)
	// 82ED7CF0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED7CF4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82ED7CF8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED7CFC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED7D00: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7D08 size=8
    let mut pc: u32 = 0x82ED7D08;
    'dispatch: loop {
        match pc {
            0x82ED7D08 => {
    //   block [0x82ED7D08..0x82ED7D10)
	// 82ED7D08: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ED7D0C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7D10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7D10 size=24
    let mut pc: u32 = 0x82ED7D10;
    'dispatch: loop {
        match pc {
            0x82ED7D10 => {
    //   block [0x82ED7D10..0x82ED7D28)
	// 82ED7D10: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82ED7D14: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82ED7D18: 392BA2C4  addi r9, r11, -0x5d3c
	ctx.r[9].s64 = ctx.r[11].s64 + -23868;
	// 82ED7D1C: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82ED7D20: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82ED7D24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7D28 size=12
    let mut pc: u32 = 0x82ED7D28;
    'dispatch: loop {
        match pc {
            0x82ED7D28 => {
    //   block [0x82ED7D28..0x82ED7D34)
	// 82ED7D28: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82ED7D2C: 386BA2C4  addi r3, r11, -0x5d3c
	ctx.r[3].s64 = ctx.r[11].s64 + -23868;
	// 82ED7D30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED7D38 size=100
    let mut pc: u32 = 0x82ED7D38;
    'dispatch: loop {
        match pc {
            0x82ED7D38 => {
    //   block [0x82ED7D38..0x82ED7D9C)
	// 82ED7D38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED7D3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ED7D40: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82ED7D44: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ED7D48: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED7D4C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ED7D50: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82ED7D54: 4800D2F5  bl 0x82ee5048
	ctx.lr = 0x82ED7D58;
	sub_82EE5048(ctx, base);
	// 82ED7D58: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82ED7D5C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED7D60: 419A0020  beq cr6, 0x82ed7d80
	if ctx.cr[6].eq {
	pc = 0x82ED7D80; continue 'dispatch;
	}
	// 82ED7D64: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED7D68: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82ED7D6C: 38C00015  li r6, 0x15
	ctx.r[6].s64 = 21;
	// 82ED7D70: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED7D74: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82ED7D78: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82ED7D7C: 4BFC8A35  bl 0x82ea07b0
	ctx.lr = 0x82ED7D80;
	sub_82EA07B0(ctx, base);
	// 82ED7D80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED7D84: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82ED7D88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ED7D8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ED7D90: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82ED7D94: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ED7D98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7DA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7DA0 size=20
    let mut pc: u32 = 0x82ED7DA0;
    'dispatch: loop {
        match pc {
            0x82ED7DA0 => {
    //   block [0x82ED7DA0..0x82ED7DB4)
	// 82ED7DA0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED7DA4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82ED7DA8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED7DAC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED7DB0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7DB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED7DB8 size=68
    let mut pc: u32 = 0x82ED7DB8;
    'dispatch: loop {
        match pc {
            0x82ED7DB8 => {
    //   block [0x82ED7DB8..0x82ED7DFC)
	// 82ED7DB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED7DBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ED7DC0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ED7DC4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED7DC8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ED7DCC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82ED7DD0: 419A0018  beq cr6, 0x82ed7de8
	if ctx.cr[6].eq {
	pc = 0x82ED7DE8; continue 'dispatch;
	}
	// 82ED7DD4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82ED7DD8: 480014E9  bl 0x82ed92c0
	ctx.lr = 0x82ED7DDC;
	sub_82ED92C0(ctx, base);
	// 82ED7DDC: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82ED7DE0: 394B3F10  addi r10, r11, 0x3f10
	ctx.r[10].s64 = ctx.r[11].s64 + 16144;
	// 82ED7DE4: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82ED7DE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82ED7DEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ED7DF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ED7DF4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ED7DF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7E00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED7E00 size=48
    let mut pc: u32 = 0x82ED7E00;
    'dispatch: loop {
        match pc {
            0x82ED7E00 => {
    //   block [0x82ED7E00..0x82ED7E30)
	// 82ED7E00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED7E04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ED7E08: 9421FDA0  stwu r1, -0x260(r1)
	ea = ctx.r[1].u32.wrapping_add(-608 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED7E0C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82ED7E10: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82ED7E14: 480014AD  bl 0x82ed92c0
	ctx.lr = 0x82ED7E18;
	sub_82ED92C0(ctx, base);
	// 82ED7E18: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82ED7E1C: 386B3F10  addi r3, r11, 0x3f10
	ctx.r[3].s64 = ctx.r[11].s64 + 16144;
	// 82ED7E20: 38210260  addi r1, r1, 0x260
	ctx.r[1].s64 = ctx.r[1].s64 + 608;
	// 82ED7E24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ED7E28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ED7E2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7E30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7E30 size=20
    let mut pc: u32 = 0x82ED7E30;
    'dispatch: loop {
        match pc {
            0x82ED7E30 => {
    //   block [0x82ED7E30..0x82ED7E44)
	// 82ED7E30: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED7E34: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82ED7E38: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED7E3C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED7E40: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7E48 size=8
    let mut pc: u32 = 0x82ED7E48;
    'dispatch: loop {
        match pc {
            0x82ED7E48 => {
    //   block [0x82ED7E48..0x82ED7E50)
	// 82ED7E48: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ED7E4C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7E50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7E50 size=24
    let mut pc: u32 = 0x82ED7E50;
    'dispatch: loop {
        match pc {
            0x82ED7E50 => {
    //   block [0x82ED7E50..0x82ED7E68)
	// 82ED7E50: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82ED7E54: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82ED7E58: 392BA41C  addi r9, r11, -0x5be4
	ctx.r[9].s64 = ctx.r[11].s64 + -23524;
	// 82ED7E5C: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82ED7E60: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82ED7E64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7E68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7E68 size=12
    let mut pc: u32 = 0x82ED7E68;
    'dispatch: loop {
        match pc {
            0x82ED7E68 => {
    //   block [0x82ED7E68..0x82ED7E74)
	// 82ED7E68: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82ED7E6C: 386BA41C  addi r3, r11, -0x5be4
	ctx.r[3].s64 = ctx.r[11].s64 + -23524;
	// 82ED7E70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7E78 size=20
    let mut pc: u32 = 0x82ED7E78;
    'dispatch: loop {
        match pc {
            0x82ED7E78 => {
    //   block [0x82ED7E78..0x82ED7E8C)
	// 82ED7E78: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED7E7C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82ED7E80: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED7E84: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED7E88: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7E90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7E90 size=12
    let mut pc: u32 = 0x82ED7E90;
    'dispatch: loop {
        match pc {
            0x82ED7E90 => {
    //   block [0x82ED7E90..0x82ED7E9C)
	// 82ED7E90: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82ED7E94: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED7E98: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7E9C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7E9C size=32
    let mut pc: u32 = 0x82ED7E9C;
    'dispatch: loop {
        match pc {
            0x82ED7E9C => {
    //   block [0x82ED7E9C..0x82ED7EBC)
	// 82ED7E9C: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82ED7EA0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82ED7EA4: 390AA55C  addi r8, r10, -0x5aa4
	ctx.r[8].s64 = ctx.r[10].s64 + -23204;
	// 82ED7EA8: B12B0006  sth r9, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82ED7EAC: 386B000C  addi r3, r11, 0xc
	ctx.r[3].s64 = ctx.r[11].s64 + 12;
	// 82ED7EB0: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82ED7EB4: 808B0014  lwz r4, 0x14(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82ED7EB8: 48015520  b 0x82eed3d8
	sub_82EED3D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7EBC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7EBC size=4
    let mut pc: u32 = 0x82ED7EBC;
    'dispatch: loop {
        match pc {
            0x82ED7EBC => {
    //   block [0x82ED7EBC..0x82ED7EC0)
	// 82ED7EBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7EC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED7EC0 size=64
    let mut pc: u32 = 0x82ED7EC0;
    'dispatch: loop {
        match pc {
            0x82ED7EC0 => {
    //   block [0x82ED7EC0..0x82ED7F00)
	// 82ED7EC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED7EC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ED7EC8: 9421FEF0  stwu r1, -0x110(r1)
	ea = ctx.r[1].u32.wrapping_add(-272 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED7ECC: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82ED7ED0: 80810064  lwz r4, 0x64(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82ED7ED4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82ED7ED8: 392BA55C  addi r9, r11, -0x5aa4
	ctx.r[9].s64 = ctx.r[11].s64 + -23204;
	// 82ED7EDC: B1410056  sth r10, 0x56(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(86 as u32), ctx.r[10].u16 ) };
	// 82ED7EE0: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 82ED7EE4: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82ED7EE8: 480154F1  bl 0x82eed3d8
	ctx.lr = 0x82ED7EEC;
	sub_82EED3D8(ctx, base);
	// 82ED7EEC: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82ED7EF0: 38210110  addi r1, r1, 0x110
	ctx.r[1].s64 = ctx.r[1].s64 + 272;
	// 82ED7EF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ED7EF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ED7EFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED7F00 size=100
    let mut pc: u32 = 0x82ED7F00;
    'dispatch: loop {
        match pc {
            0x82ED7F00 => {
    //   block [0x82ED7F00..0x82ED7F64)
	// 82ED7F00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED7F04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ED7F08: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82ED7F0C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ED7F10: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED7F14: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ED7F18: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82ED7F1C: 4801BA65  bl 0x82ef3980
	ctx.lr = 0x82ED7F20;
	sub_82EF3980(ctx, base);
	// 82ED7F20: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82ED7F24: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED7F28: 419A0020  beq cr6, 0x82ed7f48
	if ctx.cr[6].eq {
	pc = 0x82ED7F48; continue 'dispatch;
	}
	// 82ED7F2C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED7F30: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82ED7F34: 38C0002C  li r6, 0x2c
	ctx.r[6].s64 = 44;
	// 82ED7F38: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED7F3C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82ED7F40: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82ED7F44: 4BFC886D  bl 0x82ea07b0
	ctx.lr = 0x82ED7F48;
	sub_82EA07B0(ctx, base);
	// 82ED7F48: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED7F4C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82ED7F50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ED7F54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ED7F58: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82ED7F5C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ED7F60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7F68 size=20
    let mut pc: u32 = 0x82ED7F68;
    'dispatch: loop {
        match pc {
            0x82ED7F68 => {
    //   block [0x82ED7F68..0x82ED7F7C)
	// 82ED7F68: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED7F6C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82ED7F70: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED7F74: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED7F78: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7F80 size=8
    let mut pc: u32 = 0x82ED7F80;
    'dispatch: loop {
        match pc {
            0x82ED7F80 => {
    //   block [0x82ED7F80..0x82ED7F88)
	// 82ED7F80: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ED7F84: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7F88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7F88 size=24
    let mut pc: u32 = 0x82ED7F88;
    'dispatch: loop {
        match pc {
            0x82ED7F88 => {
    //   block [0x82ED7F88..0x82ED7FA0)
	// 82ED7F88: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82ED7F8C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82ED7F90: 392BA694  addi r9, r11, -0x596c
	ctx.r[9].s64 = ctx.r[11].s64 + -22892;
	// 82ED7F94: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82ED7F98: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82ED7F9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED7FA0 size=12
    let mut pc: u32 = 0x82ED7FA0;
    'dispatch: loop {
        match pc {
            0x82ED7FA0 => {
    //   block [0x82ED7FA0..0x82ED7FAC)
	// 82ED7FA0: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82ED7FA4: 386BA694  addi r3, r11, -0x596c
	ctx.r[3].s64 = ctx.r[11].s64 + -22892;
	// 82ED7FA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED7FB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED7FB0 size=100
    let mut pc: u32 = 0x82ED7FB0;
    'dispatch: loop {
        match pc {
            0x82ED7FB0 => {
    //   block [0x82ED7FB0..0x82ED8014)
	// 82ED7FB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED7FB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ED7FB8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82ED7FBC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ED7FC0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED7FC4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ED7FC8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82ED7FCC: 48013295  bl 0x82eeb260
	ctx.lr = 0x82ED7FD0;
	sub_82EEB260(ctx, base);
	// 82ED7FD0: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82ED7FD4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED7FD8: 419A0020  beq cr6, 0x82ed7ff8
	if ctx.cr[6].eq {
	pc = 0x82ED7FF8; continue 'dispatch;
	}
	// 82ED7FDC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED7FE0: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82ED7FE4: 38C0002F  li r6, 0x2f
	ctx.r[6].s64 = 47;
	// 82ED7FE8: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED7FEC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82ED7FF0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82ED7FF4: 4BFC87BD  bl 0x82ea07b0
	ctx.lr = 0x82ED7FF8;
	sub_82EA07B0(ctx, base);
	// 82ED7FF8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED7FFC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82ED8000: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ED8004: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ED8008: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82ED800C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ED8010: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED8018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED8018 size=20
    let mut pc: u32 = 0x82ED8018;
    'dispatch: loop {
        match pc {
            0x82ED8018 => {
    //   block [0x82ED8018..0x82ED802C)
	// 82ED8018: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED801C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82ED8020: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED8024: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED8028: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED8030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED8030 size=20
    let mut pc: u32 = 0x82ED8030;
    'dispatch: loop {
        match pc {
            0x82ED8030 => {
    //   block [0x82ED8030..0x82ED8044)
	// 82ED8030: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED8034: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82ED8038: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED803C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED8040: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED8048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED8048 size=8
    let mut pc: u32 = 0x82ED8048;
    'dispatch: loop {
        match pc {
            0x82ED8048 => {
    //   block [0x82ED8048..0x82ED8050)
	// 82ED8048: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ED804C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED8050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED8050 size=24
    let mut pc: u32 = 0x82ED8050;
    'dispatch: loop {
        match pc {
            0x82ED8050 => {
    //   block [0x82ED8050..0x82ED8068)
	// 82ED8050: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82ED8054: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82ED8058: 392B40D4  addi r9, r11, 0x40d4
	ctx.r[9].s64 = ctx.r[11].s64 + 16596;
	// 82ED805C: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82ED8060: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82ED8064: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED8068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED8068 size=12
    let mut pc: u32 = 0x82ED8068;
    'dispatch: loop {
        match pc {
            0x82ED8068 => {
    //   block [0x82ED8068..0x82ED8074)
	// 82ED8068: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82ED806C: 386B40D4  addi r3, r11, 0x40d4
	ctx.r[3].s64 = ctx.r[11].s64 + 16596;
	// 82ED8070: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED8078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED8078 size=8
    let mut pc: u32 = 0x82ED8078;
    'dispatch: loop {
        match pc {
            0x82ED8078 => {
    //   block [0x82ED8078..0x82ED8080)
	// 82ED8078: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ED807C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED8080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED8080 size=24
    let mut pc: u32 = 0x82ED8080;
    'dispatch: loop {
        match pc {
            0x82ED8080 => {
    //   block [0x82ED8080..0x82ED8098)
	// 82ED8080: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82ED8084: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82ED8088: 392B4144  addi r9, r11, 0x4144
	ctx.r[9].s64 = ctx.r[11].s64 + 16708;
	// 82ED808C: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82ED8090: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82ED8094: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED8098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED8098 size=12
    let mut pc: u32 = 0x82ED8098;
    'dispatch: loop {
        match pc {
            0x82ED8098 => {
    //   block [0x82ED8098..0x82ED80A4)
	// 82ED8098: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82ED809C: 386B4144  addi r3, r11, 0x4144
	ctx.r[3].s64 = ctx.r[11].s64 + 16708;
	// 82ED80A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED80A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED80A8 size=20
    let mut pc: u32 = 0x82ED80A8;
    'dispatch: loop {
        match pc {
            0x82ED80A8 => {
    //   block [0x82ED80A8..0x82ED80BC)
	// 82ED80A8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED80AC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82ED80B0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED80B4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED80B8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED80C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED80C0 size=12
    let mut pc: u32 = 0x82ED80C0;
    'dispatch: loop {
        match pc {
            0x82ED80C0 => {
    //   block [0x82ED80C0..0x82ED80CC)
	// 82ED80C0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82ED80C4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ED80C8: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED80CC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED80CC size=8
    let mut pc: u32 = 0x82ED80CC;
    'dispatch: loop {
        match pc {
            0x82ED80CC => {
    //   block [0x82ED80CC..0x82ED80D4)
	// 82ED80CC: 480058AC  b 0x82edd978
	sub_82EDD978(ctx, base);
	return;
	// 82ED80D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED80D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED80D8 size=44
    let mut pc: u32 = 0x82ED80D8;
    'dispatch: loop {
        match pc {
            0x82ED80D8 => {
    //   block [0x82ED80D8..0x82ED8104)
	// 82ED80D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED80DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ED80E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED80E4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82ED80E8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82ED80EC: 4800588D  bl 0x82edd978
	ctx.lr = 0x82ED80F0;
	sub_82EDD978(ctx, base);
	// 82ED80F0: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82ED80F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82ED80F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ED80FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ED8100: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED8108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED8108 size=20
    let mut pc: u32 = 0x82ED8108;
    'dispatch: loop {
        match pc {
            0x82ED8108 => {
    //   block [0x82ED8108..0x82ED811C)
	// 82ED8108: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED810C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82ED8110: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED8114: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED8118: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED8120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED8120 size=8
    let mut pc: u32 = 0x82ED8120;
    'dispatch: loop {
        match pc {
            0x82ED8120 => {
    //   block [0x82ED8120..0x82ED8128)
	// 82ED8120: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ED8124: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED8128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED8128 size=24
    let mut pc: u32 = 0x82ED8128;
    'dispatch: loop {
        match pc {
            0x82ED8128 => {
    //   block [0x82ED8128..0x82ED8140)
	// 82ED8128: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82ED812C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82ED8130: 392BA824  addi r9, r11, -0x57dc
	ctx.r[9].s64 = ctx.r[11].s64 + -22492;
	// 82ED8134: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82ED8138: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82ED813C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED8140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED8140 size=12
    let mut pc: u32 = 0x82ED8140;
    'dispatch: loop {
        match pc {
            0x82ED8140 => {
    //   block [0x82ED8140..0x82ED814C)
	// 82ED8140: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82ED8144: 386BA824  addi r3, r11, -0x57dc
	ctx.r[3].s64 = ctx.r[11].s64 + -22492;
	// 82ED8148: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED8150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED8150 size=140
    let mut pc: u32 = 0x82ED8150;
    'dispatch: loop {
        match pc {
            0x82ED8150 => {
    //   block [0x82ED8150..0x82ED81DC)
	// 82ED8150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED8154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ED8158: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ED815C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED8160: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ED8164: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82ED8168: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED816C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ED8170: 409A0020  bne cr6, 0x82ed8190
	if !ctx.cr[6].eq {
	pc = 0x82ED8190; continue 'dispatch;
	}
	// 82ED8174: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED8178: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82ED817C: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82ED8180: 809F002C  lwz r4, 0x2c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82ED8184: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82ED8188: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82ED818C: 4BFC8625  bl 0x82ea07b0
	ctx.lr = 0x82ED8190;
	sub_82EA07B0(ctx, base);
	// 82ED8190: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82ED8194: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED8198: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ED819C: 409A0020  bne cr6, 0x82ed81bc
	if !ctx.cr[6].eq {
	pc = 0x82ED81BC; continue 'dispatch;
	}
	// 82ED81A0: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED81A4: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82ED81A8: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82ED81AC: 809F0020  lwz r4, 0x20(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82ED81B0: 55652036  slwi r5, r11, 4
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82ED81B4: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82ED81B8: 4BFC85F9  bl 0x82ea07b0
	ctx.lr = 0x82ED81BC;
	sub_82EA07B0(ctx, base);
	// 82ED81BC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82ED81C0: 394B9EAC  addi r10, r11, -0x6154
	ctx.r[10].s64 = ctx.r[11].s64 + -24916;
	// 82ED81C4: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82ED81C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82ED81CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ED81D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ED81D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ED81D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED81E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED81E0 size=100
    let mut pc: u32 = 0x82ED81E0;
    'dispatch: loop {
        match pc {
            0x82ED81E0 => {
    //   block [0x82ED81E0..0x82ED8244)
	// 82ED81E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED81E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ED81E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82ED81EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ED81F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED81F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ED81F8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82ED81FC: 4BFFFF55  bl 0x82ed8150
	ctx.lr = 0x82ED8200;
	sub_82ED8150(ctx, base);
	// 82ED8200: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82ED8204: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED8208: 419A0020  beq cr6, 0x82ed8228
	if ctx.cr[6].eq {
	pc = 0x82ED8228; continue 'dispatch;
	}
	// 82ED820C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED8210: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82ED8214: 38C0002C  li r6, 0x2c
	ctx.r[6].s64 = 44;
	// 82ED8218: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED821C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82ED8220: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82ED8224: 4BFC858D  bl 0x82ea07b0
	ctx.lr = 0x82ED8228;
	sub_82EA07B0(ctx, base);
	// 82ED8228: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED822C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82ED8230: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ED8234: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ED8238: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82ED823C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ED8240: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED8248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED8248 size=20
    let mut pc: u32 = 0x82ED8248;
    'dispatch: loop {
        match pc {
            0x82ED8248 => {
    //   block [0x82ED8248..0x82ED825C)
	// 82ED8248: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED824C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82ED8250: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED8254: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED8258: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED8260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED8260 size=12
    let mut pc: u32 = 0x82ED8260;
    'dispatch: loop {
        match pc {
            0x82ED8260 => {
    //   block [0x82ED8260..0x82ED826C)
	// 82ED8260: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82ED8264: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ED8268: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED826C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED826C size=8
    let mut pc: u32 = 0x82ED826C;
    'dispatch: loop {
        match pc {
            0x82ED826C => {
    //   block [0x82ED826C..0x82ED8274)
	// 82ED826C: 4801CCA4  b 0x82ef4f10
	sub_82EF4F10(ctx, base);
	return;
	// 82ED8270: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED8278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED8278 size=44
    let mut pc: u32 = 0x82ED8278;
    'dispatch: loop {
        match pc {
            0x82ED8278 => {
    //   block [0x82ED8278..0x82ED82A4)
	// 82ED8278: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED827C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ED8280: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED8284: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82ED8288: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82ED828C: 4801CC85  bl 0x82ef4f10
	ctx.lr = 0x82ED8290;
	sub_82EF4F10(ctx, base);
	// 82ED8290: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82ED8294: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82ED8298: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ED829C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ED82A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED82A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED82A8 size=20
    let mut pc: u32 = 0x82ED82A8;
    'dispatch: loop {
        match pc {
            0x82ED82A8 => {
    //   block [0x82ED82A8..0x82ED82BC)
	// 82ED82A8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED82AC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82ED82B0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED82B4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED82B8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED82C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED82C0 size=8
    let mut pc: u32 = 0x82ED82C0;
    'dispatch: loop {
        match pc {
            0x82ED82C0 => {
    //   block [0x82ED82C0..0x82ED82C8)
	// 82ED82C0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ED82C4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED82C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED82C8 size=24
    let mut pc: u32 = 0x82ED82C8;
    'dispatch: loop {
        match pc {
            0x82ED82C8 => {
    //   block [0x82ED82C8..0x82ED82E0)
	// 82ED82C8: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82ED82CC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82ED82D0: 392BA8CC  addi r9, r11, -0x5734
	ctx.r[9].s64 = ctx.r[11].s64 + -22324;
	// 82ED82D4: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82ED82D8: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82ED82DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED82E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED82E0 size=12
    let mut pc: u32 = 0x82ED82E0;
    'dispatch: loop {
        match pc {
            0x82ED82E0 => {
    //   block [0x82ED82E0..0x82ED82EC)
	// 82ED82E0: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82ED82E4: 386BA8CC  addi r3, r11, -0x5734
	ctx.r[3].s64 = ctx.r[11].s64 + -22324;
	// 82ED82E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED82F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED82F0 size=100
    let mut pc: u32 = 0x82ED82F0;
    'dispatch: loop {
        match pc {
            0x82ED82F0 => {
    //   block [0x82ED82F0..0x82ED8354)
	// 82ED82F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED82F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ED82F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82ED82FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ED8300: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED8304: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ED8308: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82ED830C: 4800607D  bl 0x82ede388
	ctx.lr = 0x82ED8310;
	sub_82EDE388(ctx, base);
	// 82ED8310: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82ED8314: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED8318: 419A0020  beq cr6, 0x82ed8338
	if ctx.cr[6].eq {
	pc = 0x82ED8338; continue 'dispatch;
	}
	// 82ED831C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED8320: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82ED8324: 38C0002E  li r6, 0x2e
	ctx.r[6].s64 = 46;
	// 82ED8328: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED832C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82ED8330: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82ED8334: 4BFC847D  bl 0x82ea07b0
	ctx.lr = 0x82ED8338;
	sub_82EA07B0(ctx, base);
	// 82ED8338: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED833C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82ED8340: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ED8344: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ED8348: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82ED834C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ED8350: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED8358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED8358 size=20
    let mut pc: u32 = 0x82ED8358;
    'dispatch: loop {
        match pc {
            0x82ED8358 => {
    //   block [0x82ED8358..0x82ED836C)
	// 82ED8358: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED835C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82ED8360: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED8364: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED8368: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED8370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED8370 size=8
    let mut pc: u32 = 0x82ED8370;
    'dispatch: loop {
        match pc {
            0x82ED8370 => {
    //   block [0x82ED8370..0x82ED8378)
	// 82ED8370: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ED8374: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED8378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED8378 size=24
    let mut pc: u32 = 0x82ED8378;
    'dispatch: loop {
        match pc {
            0x82ED8378 => {
    //   block [0x82ED8378..0x82ED8390)
	// 82ED8378: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82ED837C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82ED8380: 392BA954  addi r9, r11, -0x56ac
	ctx.r[9].s64 = ctx.r[11].s64 + -22188;
	// 82ED8384: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82ED8388: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82ED838C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED8390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED8390 size=12
    let mut pc: u32 = 0x82ED8390;
    'dispatch: loop {
        match pc {
            0x82ED8390 => {
    //   block [0x82ED8390..0x82ED839C)
	// 82ED8390: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82ED8394: 386BA954  addi r3, r11, -0x56ac
	ctx.r[3].s64 = ctx.r[11].s64 + -22188;
	// 82ED8398: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED83A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED83A0 size=96
    let mut pc: u32 = 0x82ED83A0;
    'dispatch: loop {
        match pc {
            0x82ED83A0 => {
    //   block [0x82ED83A0..0x82ED8400)
	// 82ED83A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED83A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ED83A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ED83AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED83B0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ED83B4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82ED83B8: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 82ED83BC: 392B9EAC  addi r9, r11, -0x6154
	ctx.r[9].s64 = ctx.r[11].s64 + -24916;
	// 82ED83C0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82ED83C4: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82ED83C8: 419A0020  beq cr6, 0x82ed83e8
	if ctx.cr[6].eq {
	pc = 0x82ED83E8; continue 'dispatch;
	}
	// 82ED83CC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED83D0: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82ED83D4: 38C0002B  li r6, 0x2b
	ctx.r[6].s64 = 43;
	// 82ED83D8: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED83DC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82ED83E0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82ED83E4: 4BFC83CD  bl 0x82ea07b0
	ctx.lr = 0x82ED83E8;
	sub_82EA07B0(ctx, base);
	// 82ED83E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED83EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82ED83F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ED83F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ED83F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ED83FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED8400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED8400 size=20
    let mut pc: u32 = 0x82ED8400;
    'dispatch: loop {
        match pc {
            0x82ED8400 => {
    //   block [0x82ED8400..0x82ED8414)
	// 82ED8400: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED8404: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82ED8408: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED840C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED8410: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED8418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED8418 size=8
    let mut pc: u32 = 0x82ED8418;
    'dispatch: loop {
        match pc {
            0x82ED8418 => {
    //   block [0x82ED8418..0x82ED8420)
	// 82ED8418: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ED841C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED8420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED8420 size=24
    let mut pc: u32 = 0x82ED8420;
    'dispatch: loop {
        match pc {
            0x82ED8420 => {
    //   block [0x82ED8420..0x82ED8438)
	// 82ED8420: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82ED8424: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82ED8428: 392BAB54  addi r9, r11, -0x54ac
	ctx.r[9].s64 = ctx.r[11].s64 + -21676;
	// 82ED842C: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82ED8430: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82ED8434: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED8438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED8438 size=12
    let mut pc: u32 = 0x82ED8438;
    'dispatch: loop {
        match pc {
            0x82ED8438 => {
    //   block [0x82ED8438..0x82ED8444)
	// 82ED8438: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82ED843C: 386BAB54  addi r3, r11, -0x54ac
	ctx.r[3].s64 = ctx.r[11].s64 + -21676;
	// 82ED8440: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED8448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED8448 size=100
    let mut pc: u32 = 0x82ED8448;
    'dispatch: loop {
        match pc {
            0x82ED8448 => {
    //   block [0x82ED8448..0x82ED84AC)
	// 82ED8448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED844C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ED8450: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82ED8454: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ED8458: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED845C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ED8460: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82ED8464: 48005235  bl 0x82edd698
	ctx.lr = 0x82ED8468;
	sub_82EDD698(ctx, base);
	// 82ED8468: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82ED846C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED8470: 419A0020  beq cr6, 0x82ed8490
	if ctx.cr[6].eq {
	pc = 0x82ED8490; continue 'dispatch;
	}
	// 82ED8474: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED8478: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82ED847C: 38C0002C  li r6, 0x2c
	ctx.r[6].s64 = 44;
	// 82ED8480: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED8484: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82ED8488: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82ED848C: 4BFC8325  bl 0x82ea07b0
	ctx.lr = 0x82ED8490;
	sub_82EA07B0(ctx, base);
	// 82ED8490: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED8494: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82ED8498: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ED849C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ED84A0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82ED84A4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ED84A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED84B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED84B0 size=96
    let mut pc: u32 = 0x82ED84B0;
    'dispatch: loop {
        match pc {
            0x82ED84B0 => {
    //   block [0x82ED84B0..0x82ED8510)
	// 82ED84B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED84B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ED84B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ED84BC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED84C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ED84C4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82ED84C8: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 82ED84CC: 392B9EAC  addi r9, r11, -0x6154
	ctx.r[9].s64 = ctx.r[11].s64 + -24916;
	// 82ED84D0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82ED84D4: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82ED84D8: 419A0020  beq cr6, 0x82ed84f8
	if ctx.cr[6].eq {
	pc = 0x82ED84F8; continue 'dispatch;
	}
	// 82ED84DC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED84E0: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82ED84E4: 38C00029  li r6, 0x29
	ctx.r[6].s64 = 41;
	// 82ED84E8: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED84EC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82ED84F0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82ED84F4: 4BFC82BD  bl 0x82ea07b0
	ctx.lr = 0x82ED84F8;
	sub_82EA07B0(ctx, base);
	// 82ED84F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED84FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82ED8500: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ED8504: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ED8508: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ED850C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED8510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED8510 size=8
    let mut pc: u32 = 0x82ED8510;
    'dispatch: loop {
        match pc {
            0x82ED8510 => {
    //   block [0x82ED8510..0x82ED8518)
	// 82ED8510: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ED8514: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED8518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED8518 size=24
    let mut pc: u32 = 0x82ED8518;
    'dispatch: loop {
        match pc {
            0x82ED8518 => {
    //   block [0x82ED8518..0x82ED8530)
	// 82ED8518: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82ED851C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82ED8520: 392BABD0  addi r9, r11, -0x5430
	ctx.r[9].s64 = ctx.r[11].s64 + -21552;
	// 82ED8524: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82ED8528: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82ED852C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED8530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED8530 size=20
    let mut pc: u32 = 0x82ED8530;
    'dispatch: loop {
        match pc {
            0x82ED8530 => {
    //   block [0x82ED8530..0x82ED8544)
	// 82ED8530: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED8534: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82ED8538: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED853C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED8540: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED8548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED8548 size=12
    let mut pc: u32 = 0x82ED8548;
    'dispatch: loop {
        match pc {
            0x82ED8548 => {
    //   block [0x82ED8548..0x82ED8554)
	// 82ED8548: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82ED854C: 386BABD0  addi r3, r11, -0x5430
	ctx.r[3].s64 = ctx.r[11].s64 + -21552;
	// 82ED8550: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED8558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED8558 size=20
    let mut pc: u32 = 0x82ED8558;
    'dispatch: loop {
        match pc {
            0x82ED8558 => {
    //   block [0x82ED8558..0x82ED856C)
	// 82ED8558: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED855C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82ED8560: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED8564: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED8568: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED8570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED8570 size=8
    let mut pc: u32 = 0x82ED8570;
    'dispatch: loop {
        match pc {
            0x82ED8570 => {
    //   block [0x82ED8570..0x82ED8578)
	// 82ED8570: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ED8574: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED8578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED8578 size=24
    let mut pc: u32 = 0x82ED8578;
    'dispatch: loop {
        match pc {
            0x82ED8578 => {
    //   block [0x82ED8578..0x82ED8590)
	// 82ED8578: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82ED857C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82ED8580: 392BAC60  addi r9, r11, -0x53a0
	ctx.r[9].s64 = ctx.r[11].s64 + -21408;
	// 82ED8584: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82ED8588: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82ED858C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED8590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED8590 size=12
    let mut pc: u32 = 0x82ED8590;
    'dispatch: loop {
        match pc {
            0x82ED8590 => {
    //   block [0x82ED8590..0x82ED859C)
	// 82ED8590: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82ED8594: 386BAC60  addi r3, r11, -0x53a0
	ctx.r[3].s64 = ctx.r[11].s64 + -21408;
	// 82ED8598: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED85A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED85A0 size=100
    let mut pc: u32 = 0x82ED85A0;
    'dispatch: loop {
        match pc {
            0x82ED85A0 => {
    //   block [0x82ED85A0..0x82ED8604)
	// 82ED85A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED85A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ED85A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82ED85AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ED85B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED85B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ED85B8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82ED85BC: 4801DE85  bl 0x82ef6440
	ctx.lr = 0x82ED85C0;
	sub_82EF6440(ctx, base);
	// 82ED85C0: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82ED85C4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED85C8: 419A0020  beq cr6, 0x82ed85e8
	if ctx.cr[6].eq {
	pc = 0x82ED85E8; continue 'dispatch;
	}
	// 82ED85CC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED85D0: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82ED85D4: 38C0002C  li r6, 0x2c
	ctx.r[6].s64 = 44;
	// 82ED85D8: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED85DC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82ED85E0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82ED85E4: 4BFC81CD  bl 0x82ea07b0
	ctx.lr = 0x82ED85E8;
	sub_82EA07B0(ctx, base);
	// 82ED85E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED85EC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82ED85F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ED85F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ED85F8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82ED85FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ED8600: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED8608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED8608 size=20
    let mut pc: u32 = 0x82ED8608;
    'dispatch: loop {
        match pc {
            0x82ED8608 => {
    //   block [0x82ED8608..0x82ED861C)
	// 82ED8608: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED860C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82ED8610: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED8614: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED8618: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED8620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED8620 size=8
    let mut pc: u32 = 0x82ED8620;
    'dispatch: loop {
        match pc {
            0x82ED8620 => {
    //   block [0x82ED8620..0x82ED8628)
	// 82ED8620: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ED8624: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED8628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED8628 size=24
    let mut pc: u32 = 0x82ED8628;
    'dispatch: loop {
        match pc {
            0x82ED8628 => {
    //   block [0x82ED8628..0x82ED8640)
	// 82ED8628: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82ED862C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82ED8630: 392B4004  addi r9, r11, 0x4004
	ctx.r[9].s64 = ctx.r[11].s64 + 16388;
	// 82ED8634: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82ED8638: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82ED863C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED8640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED8640 size=12
    let mut pc: u32 = 0x82ED8640;
    'dispatch: loop {
        match pc {
            0x82ED8640 => {
    //   block [0x82ED8640..0x82ED864C)
	// 82ED8640: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82ED8644: 386B4004  addi r3, r11, 0x4004
	ctx.r[3].s64 = ctx.r[11].s64 + 16388;
	// 82ED8648: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED8650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED8650 size=20
    let mut pc: u32 = 0x82ED8650;
    'dispatch: loop {
        match pc {
            0x82ED8650 => {
    //   block [0x82ED8650..0x82ED8664)
	// 82ED8650: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED8654: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82ED8658: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED865C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED8660: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED8668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED8668 size=8
    let mut pc: u32 = 0x82ED8668;
    'dispatch: loop {
        match pc {
            0x82ED8668 => {
    //   block [0x82ED8668..0x82ED8670)
	// 82ED8668: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ED866C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED8670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED8670 size=24
    let mut pc: u32 = 0x82ED8670;
    'dispatch: loop {
        match pc {
            0x82ED8670 => {
    //   block [0x82ED8670..0x82ED8688)
	// 82ED8670: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82ED8674: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82ED8678: 392BADBC  addi r9, r11, -0x5244
	ctx.r[9].s64 = ctx.r[11].s64 + -21060;
	// 82ED867C: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82ED8680: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82ED8684: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED8688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED8688 size=12
    let mut pc: u32 = 0x82ED8688;
    'dispatch: loop {
        match pc {
            0x82ED8688 => {
    //   block [0x82ED8688..0x82ED8694)
	// 82ED8688: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82ED868C: 386BADBC  addi r3, r11, -0x5244
	ctx.r[3].s64 = ctx.r[11].s64 + -21060;
	// 82ED8690: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED8698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED8698 size=20
    let mut pc: u32 = 0x82ED8698;
    'dispatch: loop {
        match pc {
            0x82ED8698 => {
    //   block [0x82ED8698..0x82ED86AC)
	// 82ED8698: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED869C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82ED86A0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED86A4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED86A8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED86B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED86B0 size=12
    let mut pc: u32 = 0x82ED86B0;
    'dispatch: loop {
        match pc {
            0x82ED86B0 => {
    //   block [0x82ED86B0..0x82ED86BC)
	// 82ED86B0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82ED86B4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ED86B8: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED86BC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED86BC size=8
    let mut pc: u32 = 0x82ED86BC;
    'dispatch: loop {
        match pc {
            0x82ED86BC => {
    //   block [0x82ED86BC..0x82ED86C4)
	// 82ED86BC: 4800003C  b 0x82ed86f8
	sub_82ED86F8(ctx, base);
	return;
	// 82ED86C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED86C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED86C8 size=44
    let mut pc: u32 = 0x82ED86C8;
    'dispatch: loop {
        match pc {
            0x82ED86C8 => {
    //   block [0x82ED86C8..0x82ED86F4)
	// 82ED86C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED86CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ED86D0: 9421FE40  stwu r1, -0x1c0(r1)
	ea = ctx.r[1].u32.wrapping_add(-448 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED86D4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82ED86D8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82ED86DC: 4800001D  bl 0x82ed86f8
	ctx.lr = 0x82ED86E0;
	sub_82ED86F8(ctx, base);
	// 82ED86E0: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82ED86E4: 382101C0  addi r1, r1, 0x1c0
	ctx.r[1].s64 = ctx.r[1].s64 + 448;
	// 82ED86E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ED86EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ED86F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED86F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED86F8 size=104
    let mut pc: u32 = 0x82ED86F8;
    'dispatch: loop {
        match pc {
            0x82ED86F8 => {
    //   block [0x82ED86F8..0x82ED8760)
	// 82ED86F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED86FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ED8700: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ED8704: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED8708: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ED870C: 4BFFCE25  bl 0x82ed5530
	ctx.lr = 0x82ED8710;
	sub_82ED5530(ctx, base);
	// 82ED8710: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 82ED8714: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82ED8718: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 82ED871C: 3909483C  addi r8, r9, 0x483c
	ctx.r[8].s64 = ctx.r[9].s64 + 18492;
	// 82ED8720: 917F0080  stw r11, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 82ED8724: 917F0084  stw r11, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 82ED8728: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED872C: 915F0088  stw r10, 0x88(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(136 as u32), ctx.r[10].u32 ) };
	// 82ED8730: 917F008C  stw r11, 0x8c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(140 as u32), ctx.r[11].u32 ) };
	// 82ED8734: 917F0090  stw r11, 0x90(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), ctx.r[11].u32 ) };
	// 82ED8738: 915F0094  stw r10, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[10].u32 ) };
	// 82ED873C: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82ED8740: 917F0150  stw r11, 0x150(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(336 as u32), ctx.r[11].u32 ) };
	// 82ED8744: 917F0154  stw r11, 0x154(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(340 as u32), ctx.r[11].u32 ) };
	// 82ED8748: 915F0158  stw r10, 0x158(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(344 as u32), ctx.r[10].u32 ) };
	// 82ED874C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82ED8750: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ED8754: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ED8758: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ED875C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED8760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED8760 size=20
    let mut pc: u32 = 0x82ED8760;
    'dispatch: loop {
        match pc {
            0x82ED8760 => {
    //   block [0x82ED8760..0x82ED8774)
	// 82ED8760: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED8764: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82ED8768: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED876C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED8770: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED8778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED8778 size=12
    let mut pc: u32 = 0x82ED8778;
    'dispatch: loop {
        match pc {
            0x82ED8778 => {
    //   block [0x82ED8778..0x82ED8784)
	// 82ED8778: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82ED877C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ED8780: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED8784(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED8784 size=8
    let mut pc: u32 = 0x82ED8784;
    'dispatch: loop {
        match pc {
            0x82ED8784 => {
    //   block [0x82ED8784..0x82ED878C)
	// 82ED8784: 4801E024  b 0x82ef67a8
	sub_82EF67A8(ctx, base);
	return;
	// 82ED8788: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED8790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED8790 size=44
    let mut pc: u32 = 0x82ED8790;
    'dispatch: loop {
        match pc {
            0x82ED8790 => {
    //   block [0x82ED8790..0x82ED87BC)
	// 82ED8790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED8794: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ED8798: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED879C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82ED87A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82ED87A4: 4801E005  bl 0x82ef67a8
	ctx.lr = 0x82ED87A8;
	sub_82EF67A8(ctx, base);
	// 82ED87A8: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82ED87AC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82ED87B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ED87B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ED87B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED87C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED87C0 size=20
    let mut pc: u32 = 0x82ED87C0;
    'dispatch: loop {
        match pc {
            0x82ED87C0 => {
    //   block [0x82ED87C0..0x82ED87D4)
	// 82ED87C0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED87C4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82ED87C8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED87CC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED87D0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED87D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED87D8 size=8
    let mut pc: u32 = 0x82ED87D8;
    'dispatch: loop {
        match pc {
            0x82ED87D8 => {
    //   block [0x82ED87D8..0x82ED87E0)
	// 82ED87D8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ED87DC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED87E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED87E0 size=24
    let mut pc: u32 = 0x82ED87E0;
    'dispatch: loop {
        match pc {
            0x82ED87E0 => {
    //   block [0x82ED87E0..0x82ED87F8)
	// 82ED87E0: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82ED87E4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82ED87E8: 392B9FCC  addi r9, r11, -0x6034
	ctx.r[9].s64 = ctx.r[11].s64 + -24628;
	// 82ED87EC: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82ED87F0: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82ED87F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED87F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED87F8 size=12
    let mut pc: u32 = 0x82ED87F8;
    'dispatch: loop {
        match pc {
            0x82ED87F8 => {
    //   block [0x82ED87F8..0x82ED8804)
	// 82ED87F8: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82ED87FC: 386B9FCC  addi r3, r11, -0x6034
	ctx.r[3].s64 = ctx.r[11].s64 + -24628;
	// 82ED8800: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED8808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED8808 size=20
    let mut pc: u32 = 0x82ED8808;
    'dispatch: loop {
        match pc {
            0x82ED8808 => {
    //   block [0x82ED8808..0x82ED881C)
	// 82ED8808: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED880C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82ED8810: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED8814: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED8818: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED8820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED8820 size=12
    let mut pc: u32 = 0x82ED8820;
    'dispatch: loop {
        match pc {
            0x82ED8820 => {
    //   block [0x82ED8820..0x82ED882C)
	// 82ED8820: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82ED8824: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ED8828: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED882C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED882C size=8
    let mut pc: u32 = 0x82ED882C;
    'dispatch: loop {
        match pc {
            0x82ED882C => {
    //   block [0x82ED882C..0x82ED8834)
	// 82ED882C: 4800003C  b 0x82ed8868
	sub_82ED8868(ctx, base);
	return;
	// 82ED8830: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED8838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED8838 size=44
    let mut pc: u32 = 0x82ED8838;
    'dispatch: loop {
        match pc {
            0x82ED8838 => {
    //   block [0x82ED8838..0x82ED8864)
	// 82ED8838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED883C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ED8840: 9421FED0  stwu r1, -0x130(r1)
	ea = ctx.r[1].u32.wrapping_add(-304 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED8844: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82ED8848: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82ED884C: 4800001D  bl 0x82ed8868
	ctx.lr = 0x82ED8850;
	sub_82ED8868(ctx, base);
	// 82ED8850: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82ED8854: 38210130  addi r1, r1, 0x130
	ctx.r[1].s64 = ctx.r[1].s64 + 304;
	// 82ED8858: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ED885C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ED8860: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED8868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED8868 size=200
    let mut pc: u32 = 0x82ED8868;
    'dispatch: loop {
        match pc {
            0x82ED8868 => {
    //   block [0x82ED8868..0x82ED8930)
	// 82ED8868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED886C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ED8870: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82ED8874: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ED8878: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED887C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ED8880: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82ED8884: 4BFFCCAD  bl 0x82ed5530
	ctx.lr = 0x82ED8888;
	sub_82ED5530(ctx, base);
	// 82ED8888: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82ED888C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82ED8890: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 82ED8894: 3909AFF8  addi r8, r9, -0x5008
	ctx.r[8].s64 = ctx.r[9].s64 + -20488;
	// 82ED8898: 917F0080  stw r11, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 82ED889C: 917F0084  stw r11, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 82ED88A0: 387F00C0  addi r3, r31, 0xc0
	ctx.r[3].s64 = ctx.r[31].s64 + 192;
	// 82ED88A4: 915F0088  stw r10, 0x88(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(136 as u32), ctx.r[10].u32 ) };
	// 82ED88A8: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82ED88AC: 917F008C  stw r11, 0x8c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(140 as u32), ctx.r[11].u32 ) };
	// 82ED88B0: 917F0090  stw r11, 0x90(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), ctx.r[11].u32 ) };
	// 82ED88B4: 915F0094  stw r10, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[10].u32 ) };
	// 82ED88B8: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82ED88BC: 917F00C0  stw r11, 0xc0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(192 as u32), ctx.r[11].u32 ) };
	// 82ED88C0: 917F00C4  stw r11, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[11].u32 ) };
	// 82ED88C4: 915F00C8  stw r10, 0xc8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(200 as u32), ctx.r[10].u32 ) };
	// 82ED88C8: 419A004C  beq cr6, 0x82ed8914
	if ctx.cr[6].eq {
	pc = 0x82ED8914; continue 'dispatch;
	}
	// 82ED88CC: 554B003E  slwi r11, r10, 0
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82ED88D0: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82ED88D4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ED88D8: 419A0014  beq cr6, 0x82ed88ec
	if ctx.cr[6].eq {
	pc = 0x82ED88EC; continue 'dispatch;
	}
	// 82ED88DC: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED88E0: 5549073E  clrlwi r9, r10, 0x1c
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x0000000Fu64;
	// 82ED88E4: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82ED88E8: 419A002C  beq cr6, 0x82ed8914
	if ctx.cr[6].eq {
	pc = 0x82ED8914; continue 'dispatch;
	}
	// 82ED88EC: 388B0001  addi r4, r11, 1
	ctx.r[4].s64 = ctx.r[11].s64 + 1;
	// 82ED88F0: 2F040004  cmpwi cr6, r4, 4
	ctx.cr[6].compare_i32(ctx.r[4].s32, 4, &mut ctx.xer);
	// 82ED88F4: 40980008  bge cr6, 0x82ed88fc
	if !ctx.cr[6].lt {
	pc = 0x82ED88FC; continue 'dispatch;
	}
	// 82ED88F8: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82ED88FC: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED8900: 556A00BE  clrlwi r10, r11, 2
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82ED8904: 7F0A2000  cmpw cr6, r10, r4
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[4].s32, &mut ctx.xer);
	// 82ED8908: 4098000C  bge cr6, 0x82ed8914
	if !ctx.cr[6].lt {
	pc = 0x82ED8914; continue 'dispatch;
	}
	// 82ED890C: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82ED8910: 4BFCDEE9  bl 0x82ea67f8
	ctx.lr = 0x82ED8914;
	sub_82EA67F8(ctx, base);
	// 82ED8914: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED8918: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82ED891C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ED8920: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ED8924: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82ED8928: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ED892C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED8930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED8930 size=100
    let mut pc: u32 = 0x82ED8930;
    'dispatch: loop {
        match pc {
            0x82ED8930 => {
    //   block [0x82ED8930..0x82ED8994)
	// 82ED8930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED8934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ED8938: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82ED893C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ED8940: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED8944: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ED8948: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82ED894C: 48002835  bl 0x82edb180
	ctx.lr = 0x82ED8950;
	sub_82EDB180(ctx, base);
	// 82ED8950: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82ED8954: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED8958: 419A0020  beq cr6, 0x82ed8978
	if ctx.cr[6].eq {
	pc = 0x82ED8978; continue 'dispatch;
	}
	// 82ED895C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED8960: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82ED8964: 38C00031  li r6, 0x31
	ctx.r[6].s64 = 49;
	// 82ED8968: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED896C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82ED8970: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82ED8974: 4BFC7E3D  bl 0x82ea07b0
	ctx.lr = 0x82ED8978;
	sub_82EA07B0(ctx, base);
	// 82ED8978: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED897C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82ED8980: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ED8984: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ED8988: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82ED898C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ED8990: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED8998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED8998 size=72
    let mut pc: u32 = 0x82ED8998;
    'dispatch: loop {
        match pc {
            0x82ED8998 => {
    //   block [0x82ED8998..0x82ED89E0)
	// 82ED8998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED899C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ED89A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED89A4: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED89A8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED89AC: 419A0024  beq cr6, 0x82ed89d0
	if ctx.cr[6].eq {
	pc = 0x82ED89D0; continue 'dispatch;
	}
	// 82ED89B0: 81430010  lwz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82ED89B4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82ED89B8: 419A0018  beq cr6, 0x82ed89d0
	if ctx.cr[6].eq {
	pc = 0x82ED89D0; continue 'dispatch;
	}
	// 82ED89BC: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82ED89C0: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82ED89C4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82ED89C8: 806B006C  lwz r3, 0x6c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 82ED89CC: 4801E335  bl 0x82ef6d00
	ctx.lr = 0x82ED89D0;
	sub_82EF6D00(ctx, base);
	// 82ED89D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82ED89D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ED89D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ED89DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED89E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED89E0 size=20
    let mut pc: u32 = 0x82ED89E0;
    'dispatch: loop {
        match pc {
            0x82ED89E0 => {
    //   block [0x82ED89E0..0x82ED89F4)
	// 82ED89E0: 816400B8  lwz r11, 0xb8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(184 as u32) ) } as u64;
	// 82ED89E4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ED89E8: 409A000C  bne cr6, 0x82ed89f4
	if !ctx.cr[6].eq {
		sub_82ED89F4(ctx, base);
		return;
	}
	// 82ED89EC: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82ED89F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED89F4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED89F4 size=16
    let mut pc: u32 = 0x82ED89F4;
    'dispatch: loop {
        match pc {
            0x82ED89F4 => {
    //   block [0x82ED89F4..0x82ED8A04)
	// 82ED89F4: 896B0026  lbz r11, 0x26(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(38 as u32) ) } as u64;
	// 82ED89F8: 556AE7BE  rlwinm r10, r11, 0x1c, 0x1e, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 82ED89FC: 99430000  stb r10, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82ED8A00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED8A08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED8A08 size=20
    let mut pc: u32 = 0x82ED8A08;
    'dispatch: loop {
        match pc {
            0x82ED8A08 => {
    //   block [0x82ED8A08..0x82ED8A1C)
	// 82ED8A08: 816400B8  lwz r11, 0xb8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(184 as u32) ) } as u64;
	// 82ED8A0C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ED8A10: 409A000C  bne cr6, 0x82ed8a1c
	if !ctx.cr[6].eq {
		sub_82ED8A1C(ctx, base);
		return;
	}
	// 82ED8A14: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82ED8A18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED8A1C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED8A1C size=16
    let mut pc: u32 = 0x82ED8A1C;
    'dispatch: loop {
        match pc {
            0x82ED8A1C => {
    //   block [0x82ED8A1C..0x82ED8A2C)
	// 82ED8A1C: 896B0026  lbz r11, 0x26(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(38 as u32) ) } as u64;
	// 82ED8A20: 556AE7BE  rlwinm r10, r11, 0x1c, 0x1e, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 82ED8A24: 99430000  stb r10, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82ED8A28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED8A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED8A30 size=8
    let mut pc: u32 = 0x82ED8A30;
    'dispatch: loop {
        match pc {
            0x82ED8A30 => {
    //   block [0x82ED8A30..0x82ED8A38)
	// 82ED8A30: 38630098  addi r3, r3, 0x98
	ctx.r[3].s64 = ctx.r[3].s64 + 152;
	// 82ED8A34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED8A38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED8A38 size=8
    let mut pc: u32 = 0x82ED8A38;
    'dispatch: loop {
        match pc {
            0x82ED8A38 => {
    //   block [0x82ED8A38..0x82ED8A40)
	// 82ED8A38: 38630098  addi r3, r3, 0x98
	ctx.r[3].s64 = ctx.r[3].s64 + 152;
	// 82ED8A3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED8A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED8A40 size=8
    let mut pc: u32 = 0x82ED8A40;
    'dispatch: loop {
        match pc {
            0x82ED8A40 => {
    //   block [0x82ED8A40..0x82ED8A48)
	// 82ED8A40: 386300A0  addi r3, r3, 0xa0
	ctx.r[3].s64 = ctx.r[3].s64 + 160;
	// 82ED8A44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED8A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED8A48 size=308
    let mut pc: u32 = 0x82ED8A48;
    'dispatch: loop {
        match pc {
            0x82ED8A48 => {
    //   block [0x82ED8A48..0x82ED8B7C)
	// 82ED8A48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED8A4C: 482CF719  bl 0x831a8164
	ctx.lr = 0x82ED8A50;
	sub_831A8130(ctx, base);
	// 82ED8A50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED8A54: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82ED8A58: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82ED8A5C: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82ED8A60: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82ED8A64: 809E0054  lwz r4, 0x54(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(84 as u32) ) } as u64;
	// 82ED8A68: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82ED8A6C: 419A0028  beq cr6, 0x82ed8a94
	if ctx.cr[6].eq {
	pc = 0x82ED8A94; continue 'dispatch;
	}
	// 82ED8A70: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED8A74: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82ED8A78: A13E0050  lhz r9, 0x50(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(80 as u32) ) } as u64;
	// 82ED8A7C: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 82ED8A80: 5525283E  rotlwi r5, r9, 5
	ctx.r[5].u64 = ((ctx.r[9].u32).rotate_left(5)) as u64;
	// 82ED8A84: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82ED8A88: 4BFC7D29  bl 0x82ea07b0
	ctx.lr = 0x82ED8A8C;
	sub_82EA07B0(ctx, base);
	// 82ED8A8C: B37E0050  sth r27, 0x50(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(80 as u32), ctx.r[27].u16 ) };
	// 82ED8A90: 937E0054  stw r27, 0x54(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(84 as u32), ctx.r[27].u32 ) };
	// 82ED8A94: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82ED8A98: 419A00DC  beq cr6, 0x82ed8b74
	if ctx.cr[6].eq {
	pc = 0x82ED8B74; continue 'dispatch;
	}
	// 82ED8A9C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82ED8AA0: 419A00D4  beq cr6, 0x82ed8b74
	if ctx.cr[6].eq {
	pc = 0x82ED8B74; continue 'dispatch;
	}
	// 82ED8AA4: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ED8AA8: 7F7CDB78  mr r28, r27
	ctx.r[28].u64 = ctx.r[27].u64;
	// 82ED8AAC: 815D0074  lwz r10, 0x74(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(116 as u32) ) } as u64;
	// 82ED8AB0: 392B0044  addi r9, r11, 0x44
	ctx.r[9].s64 = ctx.r[11].s64 + 68;
	// 82ED8AB4: 5528103A  slwi r8, r9, 2
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82ED8AB8: 7D68502E  lwzx r11, r8, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82ED8ABC: 556705EE  rlwinm r7, r11, 0, 0x17, 0x17
	ctx.r[7].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED8AC0: 2F070000  cmpwi cr6, r7, 0
	ctx.cr[6].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82ED8AC4: 419A001C  beq cr6, 0x82ed8ae0
	if ctx.cr[6].eq {
	pc = 0x82ED8AE0; continue 'dispatch;
	}
	// 82ED8AC8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED8ACC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED8AD0: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82ED8AD4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED8AD8: 4E800421  bctrl
	ctx.lr = 0x82ED8ADC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED8ADC: 48000028  b 0x82ed8b04
	pc = 0x82ED8B04; continue 'dispatch;
	// 82ED8AE0: 556B05AC  rlwinm r11, r11, 0, 0x16, 0x16
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED8AE4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ED8AE8: 419A008C  beq cr6, 0x82ed8b74
	if ctx.cr[6].eq {
	pc = 0x82ED8B74; continue 'dispatch;
	}
	// 82ED8AEC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED8AF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED8AF4: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82ED8AF8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED8AFC: 4E800421  bctrl
	ctx.lr = 0x82ED8B00;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED8B00: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 82ED8B04: 897E00D8  lbz r11, 0xd8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(216 as u32) ) } as u64;
	// 82ED8B08: 2B0B0007  cmplwi cr6, r11, 7
	ctx.cr[6].compare_u32(ctx.r[11].u32, 7 as u32, &mut ctx.xer);
	// 82ED8B0C: 419A0010  beq cr6, 0x82ed8b1c
	if ctx.cr[6].eq {
	pc = 0x82ED8B1C; continue 'dispatch;
	}
	// 82ED8B10: 2B0B0006  cmplwi cr6, r11, 6
	ctx.cr[6].compare_u32(ctx.r[11].u32, 6 as u32, &mut ctx.xer);
	// 82ED8B14: 7F6BDB78  mr r11, r27
	ctx.r[11].u64 = ctx.r[27].u64;
	// 82ED8B18: 409A0008  bne cr6, 0x82ed8b20
	if !ctx.cr[6].eq {
	pc = 0x82ED8B20; continue 'dispatch;
	}
	// 82ED8B1C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82ED8B20: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82ED8B24: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ED8B28: 419A0010  beq cr6, 0x82ed8b38
	if ctx.cr[6].eq {
	pc = 0x82ED8B38; continue 'dispatch;
	}
	// 82ED8B2C: 578B063E  clrlwi r11, r28, 0x18
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	// 82ED8B30: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED8B34: 409A0040  bne cr6, 0x82ed8b74
	if !ctx.cr[6].eq {
	pc = 0x82ED8B74; continue 'dispatch;
	}
	// 82ED8B38: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED8B3C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED8B40: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED8B44: 4E800421  bctrl
	ctx.lr = 0x82ED8B48;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED8B48: B07E0050  sth r3, 0x50(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(80 as u32), ctx.r[3].u16 ) };
	// 82ED8B4C: 812D0000  lwz r9, 0(r13)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED8B50: 39000014  li r8, 0x14
	ctx.r[8].s64 = 20;
	// 82ED8B54: 54642834  slwi r4, r3, 5
	ctx.r[4].u32 = ctx.r[3].u32.wrapping_shl(5);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82ED8B58: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82ED8B5C: 7C68482E  lwzx r3, r8, r9
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82ED8B60: 4BFC7BD1  bl 0x82ea0730
	ctx.lr = 0x82ED8B64;
	sub_82EA0730(ctx, base);
	// 82ED8B64: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82ED8B68: 907E0054  stw r3, 0x54(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 82ED8B6C: 90DE0030  stw r6, 0x30(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(48 as u32), ctx.r[6].u32 ) };
	// 82ED8B70: 937E0040  stw r27, 0x40(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(64 as u32), ctx.r[27].u32 ) };
	// 82ED8B74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82ED8B78: 482CF63C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED8B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED8B80 size=76
    let mut pc: u32 = 0x82ED8B80;
    'dispatch: loop {
        match pc {
            0x82ED8B80 => {
    //   block [0x82ED8B80..0x82ED8BCC)
	// 82ED8B80: 810300CC  lwz r8, 0xcc(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(204 as u32) ) } as u64;
	// 82ED8B84: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82ED8B88: A1280004  lhz r9, 4(r8)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED8B8C: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82ED8B90: 40990024  ble cr6, 0x82ed8bb4
	if !ctx.cr[6].gt {
	pc = 0x82ED8BB4; continue 'dispatch;
	}
	// 82ED8B94: 81480000  lwz r10, 0(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED8B98: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED8B9C: 7F072040  cmplw cr6, r7, r4
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82ED8BA0: 419A0018  beq cr6, 0x82ed8bb8
	if ctx.cr[6].eq {
	pc = 0x82ED8BB8; continue 'dispatch;
	}
	// 82ED8BA4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82ED8BA8: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82ED8BAC: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82ED8BB0: 4198FFE8  blt cr6, 0x82ed8b98
	if ctx.cr[6].lt {
	pc = 0x82ED8B98; continue 'dispatch;
	}
	// 82ED8BB4: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82ED8BB8: 81480000  lwz r10, 0(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED8BBC: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82ED8BC0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82ED8BC4: 7D09512E  stwx r8, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[8].u32) };
	// 82ED8BC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED8BD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED8BD0 size=72
    let mut pc: u32 = 0x82ED8BD0;
    'dispatch: loop {
        match pc {
            0x82ED8BD0 => {
    //   block [0x82ED8BD0..0x82ED8C18)
	// 82ED8BD0: A12301F4  lhz r9, 0x1f4(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(500 as u32) ) } as u64;
	// 82ED8BD4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82ED8BD8: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82ED8BDC: 40990024  ble cr6, 0x82ed8c00
	if !ctx.cr[6].gt {
	pc = 0x82ED8C00; continue 'dispatch;
	}
	// 82ED8BE0: 814301F0  lwz r10, 0x1f0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(496 as u32) ) } as u64;
	// 82ED8BE4: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED8BE8: 7F082040  cmplw cr6, r8, r4
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82ED8BEC: 419A0018  beq cr6, 0x82ed8c04
	if ctx.cr[6].eq {
	pc = 0x82ED8C04; continue 'dispatch;
	}
	// 82ED8BF0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82ED8BF4: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82ED8BF8: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82ED8BFC: 4198FFE8  blt cr6, 0x82ed8be4
	if ctx.cr[6].lt {
	pc = 0x82ED8BE4; continue 'dispatch;
	}
	// 82ED8C00: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82ED8C04: 814301F0  lwz r10, 0x1f0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(496 as u32) ) } as u64;
	// 82ED8C08: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82ED8C0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82ED8C10: 7D09512E  stwx r8, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[8].u32) };
	// 82ED8C14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED8C18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED8C18 size=64
    let mut pc: u32 = 0x82ED8C18;
    'dispatch: loop {
        match pc {
            0x82ED8C18 => {
    //   block [0x82ED8C18..0x82ED8C58)
	// 82ED8C18: 81230060  lwz r9, 0x60(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(96 as u32) ) } as u64;
	// 82ED8C1C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82ED8C20: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82ED8C24: 4099002C  ble cr6, 0x82ed8c50
	if !ctx.cr[6].gt {
	pc = 0x82ED8C50; continue 'dispatch;
	}
	// 82ED8C28: 80E3005C  lwz r7, 0x5c(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(92 as u32) ) } as u64;
	// 82ED8C2C: 39040010  addi r8, r4, 0x10
	ctx.r[8].s64 = ctx.r[4].s64 + 16;
	// 82ED8C30: 39670004  addi r11, r7, 4
	ctx.r[11].s64 = ctx.r[7].s64 + 4;
	// 82ED8C34: 80CB0000  lwz r6, 0(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED8C38: 7F064040  cmplw cr6, r6, r8
	ctx.cr[6].compare_u32(ctx.r[6].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82ED8C3C: 419A001C  beq cr6, 0x82ed8c58
	if ctx.cr[6].eq {
		sub_82ED8C58(ctx, base);
		return;
	}
	// 82ED8C40: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82ED8C44: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82ED8C48: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82ED8C4C: 4198FFE8  blt cr6, 0x82ed8c34
	if ctx.cr[6].lt {
	pc = 0x82ED8C34; continue 'dispatch;
	}
	// 82ED8C50: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82ED8C54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED8C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED8C58 size=16
    let mut pc: u32 = 0x82ED8C58;
    'dispatch: loop {
        match pc {
            0x82ED8C58 => {
    //   block [0x82ED8C58..0x82ED8C68)
	// 82ED8C58: 554B1838  slwi r11, r10, 3
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82ED8C5C: 7D4B382E  lwzx r10, r11, r7
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 82ED8C60: 806A0008  lwz r3, 8(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED8C64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED8C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED8C68 size=144
    let mut pc: u32 = 0x82ED8C68;
    'dispatch: loop {
        match pc {
            0x82ED8C68 => {
    //   block [0x82ED8C68..0x82ED8CF8)
	// 82ED8C68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED8C6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ED8C70: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ED8C74: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED8C78: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ED8C7C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82ED8C80: 809F00B8  lwz r4, 0xb8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(184 as u32) ) } as u64;
	// 82ED8C84: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82ED8C88: 409A000C  bne cr6, 0x82ed8c94
	if !ctx.cr[6].eq {
	pc = 0x82ED8C94; continue 'dispatch;
	}
	// 82ED8C8C: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82ED8C90: 4800000C  b 0x82ed8c9c
	pc = 0x82ED8C9C; continue 'dispatch;
	// 82ED8C94: 89640026  lbz r11, 0x26(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(38 as u32) ) } as u64;
	// 82ED8C98: 556BE7BE  rlwinm r11, r11, 0x1c, 0x1e, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 82ED8C9C: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82ED8CA0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ED8CA4: 409A0040  bne cr6, 0x82ed8ce4
	if !ctx.cr[6].eq {
	pc = 0x82ED8CE4; continue 'dispatch;
	}
	// 82ED8CA8: 897F00D8  lbz r11, 0xd8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(216 as u32) ) } as u64;
	// 82ED8CAC: 2B0B0007  cmplwi cr6, r11, 7
	ctx.cr[6].compare_u32(ctx.r[11].u32, 7 as u32, &mut ctx.xer);
	// 82ED8CB0: 419A0034  beq cr6, 0x82ed8ce4
	if ctx.cr[6].eq {
	pc = 0x82ED8CE4; continue 'dispatch;
	}
	// 82ED8CB4: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED8CB8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ED8CBC: 419A0028  beq cr6, 0x82ed8ce4
	if ctx.cr[6].eq {
	pc = 0x82ED8CE4; continue 'dispatch;
	}
	// 82ED8CC0: B15F00DA  sth r10, 0xda(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(218 as u32), ctx.r[10].u16 ) };
	// 82ED8CC4: B15F00DC  sth r10, 0xdc(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(220 as u32), ctx.r[10].u16 ) };
	// 82ED8CC8: 48006E81  bl 0x82edfb48
	ctx.lr = 0x82ED8CCC;
	sub_82EDFB48(ctx, base);
	// 82ED8CCC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED8CD0: 387F00D0  addi r3, r31, 0xd0
	ctx.r[3].s64 = ctx.r[31].s64 + 208;
	// 82ED8CD4: 88CB02C7  lbz r6, 0x2c7(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(711 as u32) ) } as u64;
	// 82ED8CD8: 88AB02C6  lbz r5, 0x2c6(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(710 as u32) ) } as u64;
	// 82ED8CDC: 888B02C5  lbz r4, 0x2c5(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(709 as u32) ) } as u64;
	// 82ED8CE0: 4BFF2631  bl 0x82ecb310
	ctx.lr = 0x82ED8CE4;
	sub_82ECB310(ctx, base);
	// 82ED8CE4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82ED8CE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ED8CEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ED8CF0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ED8CF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED8CF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82ED8CF8 size=264
    let mut pc: u32 = 0x82ED8CF8;
    'dispatch: loop {
        match pc {
            0x82ED8CF8 => {
    //   block [0x82ED8CF8..0x82ED8E00)
	// 82ED8CF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED8CFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ED8D00: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82ED8D04: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ED8D08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED8D0C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ED8D10: 817F00B8  lwz r11, 0xb8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(184 as u32) ) } as u64;
	// 82ED8D14: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ED8D18: 409A000C  bne cr6, 0x82ed8d24
	if !ctx.cr[6].eq {
	pc = 0x82ED8D24; continue 'dispatch;
	}
	// 82ED8D1C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82ED8D20: 4800000C  b 0x82ed8d2c
	pc = 0x82ED8D2C; continue 'dispatch;
	// 82ED8D24: 894B0026  lbz r10, 0x26(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(38 as u32) ) } as u64;
	// 82ED8D28: 554AE7BE  rlwinm r10, r10, 0x1c, 0x1e, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000000Fu64;
	// 82ED8D2C: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 82ED8D30: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ED8D34: 419A00B4  beq cr6, 0x82ed8de8
	if ctx.cr[6].eq {
	pc = 0x82ED8DE8; continue 'dispatch;
	}
	// 82ED8D38: 896B0025  lbz r11, 0x25(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(37 as u32) ) } as u64;
	// 82ED8D3C: 556A06B6  rlwinm r10, r11, 0, 0x1a, 0x1b
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED8D40: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82ED8D44: 409A0010  bne cr6, 0x82ed8d54
	if !ctx.cr[6].eq {
	pc = 0x82ED8D54; continue 'dispatch;
	}
	// 82ED8D48: 556B0032  rlwinm r11, r11, 0, 0, 0x19
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED8D4C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED8D50: 419A0050  beq cr6, 0x82ed8da0
	if ctx.cr[6].eq {
	pc = 0x82ED8DA0; continue 'dispatch;
	}
	// 82ED8D54: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED8D58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82ED8D5C: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82ED8D60: 83CB00A4  lwz r30, 0xa4(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(164 as u32) ) } as u64;
	// 82ED8D64: 914B00A4  stw r10, 0xa4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(164 as u32), ctx.r[10].u32 ) };
	// 82ED8D68: 817F00B8  lwz r11, 0xb8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(184 as u32) ) } as u64;
	// 82ED8D6C: 890B0025  lbz r8, 0x25(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(37 as u32) ) } as u64;
	// 82ED8D70: 5507063E  clrlwi r7, r8, 0x18
	ctx.r[7].u64 = ctx.r[8].u32 as u64 & 0x000000FFu64;
	// 82ED8D74: 54E70732  rlwinm r7, r7, 0, 0x1c, 0x19
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED8D78: 98EB0025  stb r7, 0x25(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(37 as u32), ctx.r[7].u8 ) };
	// 82ED8D7C: 817F00B8  lwz r11, 0xb8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(184 as u32) ) } as u64;
	// 82ED8D80: 88CB0025  lbz r6, 0x25(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(37 as u32) ) } as u64;
	// 82ED8D84: 51263032  rlwimi r6, r9, 6, 0, 0x19
	ctx.r[6].u64 = (((ctx.r[9].u32).rotate_left(6) as u64) & 0x00000000FFFFFFC0) | (ctx.r[6].u64 & 0xFFFFFFFF0000003F);
	// 82ED8D88: 98CB0025  stb r6, 0x25(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(37 as u32), ctx.r[6].u8 ) };
	// 82ED8D8C: 809F00B8  lwz r4, 0xb8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(184 as u32) ) } as u64;
	// 82ED8D90: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED8D94: 48009845  bl 0x82ee25d8
	ctx.lr = 0x82ED8D98;
	sub_82EE25D8(ctx, base);
	// 82ED8D98: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED8D9C: 93C500A4  stw r30, 0xa4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(164 as u32), ctx.r[30].u32 ) };
	// 82ED8DA0: 809F00B8  lwz r4, 0xb8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(184 as u32) ) } as u64;
	// 82ED8DA4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82ED8DA8: 8144004C  lwz r10, 0x4c(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(76 as u32) ) } as u64;
	// 82ED8DAC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ED8DB0: 40990030  ble cr6, 0x82ed8de0
	if !ctx.cr[6].gt {
	pc = 0x82ED8DE0; continue 'dispatch;
	}
	// 82ED8DB4: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82ED8DB8: 81440048  lwz r10, 0x48(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(72 as u32) ) } as u64;
	// 82ED8DBC: C009BA78  lfs f0, -0x4588(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-17800 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82ED8DC0: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED8DC4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82ED8DC8: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82ED8DCC: D00901CC  stfs f0, 0x1cc(r9)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(460 as u32), tmp.u32 ) };
	// 82ED8DD0: D00901DC  stfs f0, 0x1dc(r9)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(476 as u32), tmp.u32 ) };
	// 82ED8DD4: 8104004C  lwz r8, 0x4c(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(76 as u32) ) } as u64;
	// 82ED8DD8: 7F0B4000  cmpw cr6, r11, r8
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82ED8DDC: 4198FFE4  blt cr6, 0x82ed8dc0
	if ctx.cr[6].lt {
	pc = 0x82ED8DC0; continue 'dispatch;
	}
	// 82ED8DE0: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED8DE4: 48006C35  bl 0x82edfa18
	ctx.lr = 0x82ED8DE8;
	sub_82EDFA18(ctx, base);
	// 82ED8DE8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82ED8DEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ED8DF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ED8DF4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82ED8DF8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ED8DFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED8E00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED8E00 size=96
    let mut pc: u32 = 0x82ED8E00;
    'dispatch: loop {
        match pc {
            0x82ED8E00 => {
    //   block [0x82ED8E00..0x82ED8E60)
	// 82ED8E00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED8E04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ED8E08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED8E0C: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED8E10: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED8E14: 419A0038  beq cr6, 0x82ed8e4c
	if ctx.cr[6].eq {
	pc = 0x82ED8E4C; continue 'dispatch;
	}
	// 82ED8E18: 814B0084  lwz r10, 0x84(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(132 as u32) ) } as u64;
	// 82ED8E1C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ED8E20: 419A002C  beq cr6, 0x82ed8e4c
	if ctx.cr[6].eq {
	pc = 0x82ED8E4C; continue 'dispatch;
	}
	// 82ED8E24: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 82ED8E28: 90610054  stw r3, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 82ED8E2C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82ED8E30: 99410050  stb r10, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u8 ) };
	// 82ED8E34: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82ED8E38: 4BFF41C9  bl 0x82ecd000
	ctx.lr = 0x82ED8E3C;
	sub_82ECD000(ctx, base);
	// 82ED8E3C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82ED8E40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ED8E44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ED8E48: 4E800020  blr
	return;
	// 82ED8E4C: 4BFFFE1D  bl 0x82ed8c68
	ctx.lr = 0x82ED8E50;
	sub_82ED8C68(ctx, base);
	// 82ED8E50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82ED8E54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ED8E58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ED8E5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED8E60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED8E60 size=96
    let mut pc: u32 = 0x82ED8E60;
    'dispatch: loop {
        match pc {
            0x82ED8E60 => {
    //   block [0x82ED8E60..0x82ED8EC0)
	// 82ED8E60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED8E64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ED8E68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED8E6C: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED8E70: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED8E74: 419A0038  beq cr6, 0x82ed8eac
	if ctx.cr[6].eq {
	pc = 0x82ED8EAC; continue 'dispatch;
	}
	// 82ED8E78: 814B0084  lwz r10, 0x84(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(132 as u32) ) } as u64;
	// 82ED8E7C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ED8E80: 419A002C  beq cr6, 0x82ed8eac
	if ctx.cr[6].eq {
	pc = 0x82ED8EAC; continue 'dispatch;
	}
	// 82ED8E84: 39400021  li r10, 0x21
	ctx.r[10].s64 = 33;
	// 82ED8E88: 90610054  stw r3, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 82ED8E8C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82ED8E90: 99410050  stb r10, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u8 ) };
	// 82ED8E94: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82ED8E98: 4BFF4169  bl 0x82ecd000
	ctx.lr = 0x82ED8E9C;
	sub_82ECD000(ctx, base);
	// 82ED8E9C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82ED8EA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ED8EA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ED8EA8: 4E800020  blr
	return;
	// 82ED8EAC: 4BFFFE4D  bl 0x82ed8cf8
	ctx.lr = 0x82ED8EB0;
	sub_82ED8CF8(ctx, base);
	// 82ED8EB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82ED8EB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ED8EB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ED8EBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED8EC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED8EC0 size=16
    let mut pc: u32 = 0x82ED8EC0;
    'dispatch: loop {
        match pc {
            0x82ED8EC0 => {
    //   block [0x82ED8EC0..0x82ED8ED0)
	// 82ED8EC0: A163009C  lhz r11, 0x9c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(156 as u32) ) } as u64;
	// 82ED8EC4: 814300A4  lwz r10, 0xa4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(164 as u32) ) } as u64;
	// 82ED8EC8: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82ED8ECC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED8ED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED8ED0 size=36
    let mut pc: u32 = 0x82ED8ED0;
    'dispatch: loop {
        match pc {
            0x82ED8ED0 => {
    //   block [0x82ED8ED0..0x82ED8EF4)
	// 82ED8ED0: A163009C  lhz r11, 0x9c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(156 as u32) ) } as u64;
	// 82ED8ED4: 7F045800  cmpw cr6, r4, r11
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82ED8ED8: 4098001C  bge cr6, 0x82ed8ef4
	if !ctx.cr[6].lt {
		sub_82ED8EF4(ctx, base);
		return;
	}
	// 82ED8EDC: 548B083C  slwi r11, r4, 1
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82ED8EE0: 81430098  lwz r10, 0x98(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(152 as u32) ) } as u64;
	// 82ED8EE4: 7D245A14  add r9, r4, r11
	ctx.r[9].u64 = ctx.r[4].u64 + ctx.r[11].u64;
	// 82ED8EE8: 55282036  slwi r8, r9, 4
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(4);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82ED8EEC: 7C6A402E  lwzx r3, r10, r8
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82ED8EF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED8EF4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED8EF4 size=20
    let mut pc: u32 = 0x82ED8EF4;
    'dispatch: loop {
        match pc {
            0x82ED8EF4 => {
    //   block [0x82ED8EF4..0x82ED8F08)
	// 82ED8EF4: 7D6B2050  subf r11, r11, r4
	ctx.r[11].s64 = ctx.r[4].s64 - ctx.r[11].s64;
	// 82ED8EF8: 814300A0  lwz r10, 0xa0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(160 as u32) ) } as u64;
	// 82ED8EFC: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82ED8F00: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82ED8F04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED8F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED8F08 size=36
    let mut pc: u32 = 0x82ED8F08;
    'dispatch: loop {
        match pc {
            0x82ED8F08 => {
    //   block [0x82ED8F08..0x82ED8F2C)
	// 82ED8F08: A163009C  lhz r11, 0x9c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(156 as u32) ) } as u64;
	// 82ED8F0C: 7F045800  cmpw cr6, r4, r11
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82ED8F10: 4098001C  bge cr6, 0x82ed8f2c
	if !ctx.cr[6].lt {
		sub_82ED8F2C(ctx, base);
		return;
	}
	// 82ED8F14: 548B083C  slwi r11, r4, 1
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82ED8F18: 81430098  lwz r10, 0x98(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(152 as u32) ) } as u64;
	// 82ED8F1C: 7D245A14  add r9, r4, r11
	ctx.r[9].u64 = ctx.r[4].u64 + ctx.r[11].u64;
	// 82ED8F20: 55282036  slwi r8, r9, 4
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(4);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82ED8F24: 7C6A402E  lwzx r3, r10, r8
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82ED8F28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED8F2C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED8F2C size=20
    let mut pc: u32 = 0x82ED8F2C;
    'dispatch: loop {
        match pc {
            0x82ED8F2C => {
    //   block [0x82ED8F2C..0x82ED8F40)
	// 82ED8F2C: 7D6B2050  subf r11, r11, r4
	ctx.r[11].s64 = ctx.r[4].s64 - ctx.r[11].s64;
	// 82ED8F30: 814300A0  lwz r10, 0xa0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(160 as u32) ) } as u64;
	// 82ED8F34: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82ED8F38: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82ED8F3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED8F40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED8F40 size=460
    let mut pc: u32 = 0x82ED8F40;
    'dispatch: loop {
        match pc {
            0x82ED8F40 => {
    //   block [0x82ED8F40..0x82ED910C)
	// 82ED8F40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED8F44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ED8F48: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82ED8F4C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ED8F50: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED8F54: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82ED8F58: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82ED8F5C: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82ED8F60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED8F64: 388AB094  addi r4, r10, -0x4f6c
	ctx.r[4].s64 = ctx.r[10].s64 + -20332;
	// 82ED8F68: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED8F6C: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82ED8F70: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82ED8F74: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED8F78: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82ED8F7C: 4E800421  bctrl
	ctx.lr = 0x82ED8F80;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED8F80: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82ED8F84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82ED8F88: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82ED8F8C: 4BFD116D  bl 0x82eaa0f8
	ctx.lr = 0x82ED8F90;
	sub_82EAA0F8(ctx, base);
	// 82ED8F90: 80FF0000  lwz r7, 0(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED8F94: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82ED8F98: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82ED8F9C: 3888A368  addi r4, r8, -0x5c98
	ctx.r[4].s64 = ctx.r[8].s64 + -23704;
	// 82ED8FA0: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82ED8FA4: 81670004  lwz r11, 4(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED8FA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED8FAC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82ED8FB0: 4E800421  bctrl
	ctx.lr = 0x82ED8FB4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED8FB4: A17E01F6  lhz r11, 0x1f6(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(502 as u32) ) } as u64;
	// 82ED8FB8: 556A0020  rlwinm r10, r11, 0, 0, 0x10
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED8FBC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82ED8FC0: 409A0034  bne cr6, 0x82ed8ff4
	if !ctx.cr[6].eq {
	pc = 0x82ED8FF4; continue 'dispatch;
	}
	// 82ED8FC4: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED8FC8: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82ED8FCC: A0FE01F4  lhz r7, 0x1f4(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(500 as u32) ) } as u64;
	// 82ED8FD0: 5568143A  rlwinm r8, r11, 2, 0x10, 0x1d
	ctx.r[8].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82ED8FD4: 80DE01F0  lwz r6, 0x1f0(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(496 as u32) ) } as u64;
	// 82ED8FD8: 3889B084  addi r4, r9, -0x4f7c
	ctx.r[4].s64 = ctx.r[9].s64 + -20348;
	// 82ED8FDC: 54E7103E  rotlwi r7, r7, 2
	ctx.r[7].u64 = ((ctx.r[7].u32).rotate_left(2)) as u64;
	// 82ED8FE0: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82ED8FE4: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED8FE8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED8FEC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82ED8FF0: 4E800421  bctrl
	ctx.lr = 0x82ED8FF4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED8FF4: 80DE00CC  lwz r6, 0xcc(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(204 as u32) ) } as u64;
	// 82ED8FF8: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82ED8FFC: 419A00B4  beq cr6, 0x82ed90b0
	if ctx.cr[6].eq {
	pc = 0x82ED90B0; continue 'dispatch;
	}
	// 82ED9000: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED9004: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82ED9008: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82ED900C: 388AB06C  addi r4, r10, -0x4f94
	ctx.r[4].s64 = ctx.r[10].s64 + -20372;
	// 82ED9010: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 82ED9014: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82ED9018: 812B0008  lwz r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED901C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED9020: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82ED9024: 4E800421  bctrl
	ctx.lr = 0x82ED9028;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED9028: 817E00CC  lwz r11, 0xcc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(204 as u32) ) } as u64;
	// 82ED902C: A14B000E  lhz r10, 0xe(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(14 as u32) ) } as u64;
	// 82ED9030: 55480020  rlwinm r8, r10, 0, 0, 0x10
	ctx.r[8].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED9034: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82ED9038: 409A0034  bne cr6, 0x82ed906c
	if !ctx.cr[6].eq {
	pc = 0x82ED906C; continue 'dispatch;
	}
	// 82ED903C: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED9040: 3CE08213  lis r7, -0x7ded
	ctx.r[7].s64 = -2112684032;
	// 82ED9044: A0AB000C  lhz r5, 0xc(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ED9048: 5548143A  rlwinm r8, r10, 2, 0x10, 0x1d
	ctx.r[8].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 82ED904C: 3887B05C  addi r4, r7, -0x4fa4
	ctx.r[4].s64 = ctx.r[7].s64 + -20388;
	// 82ED9050: 80CB0008  lwz r6, 8(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED9054: 54A7103E  rotlwi r7, r5, 2
	ctx.r[7].u64 = ((ctx.r[5].u32).rotate_left(2)) as u64;
	// 82ED9058: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82ED905C: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED9060: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED9064: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82ED9068: 4E800421  bctrl
	ctx.lr = 0x82ED906C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED906C: 817E00CC  lwz r11, 0xcc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(204 as u32) ) } as u64;
	// 82ED9070: A14B0006  lhz r10, 6(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(6 as u32) ) } as u64;
	// 82ED9074: 55490020  rlwinm r9, r10, 0, 0, 0x10
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED9078: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82ED907C: 409A0034  bne cr6, 0x82ed90b0
	if !ctx.cr[6].eq {
	pc = 0x82ED90B0; continue 'dispatch;
	}
	// 82ED9080: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED9084: 3CE08213  lis r7, -0x7ded
	ctx.r[7].s64 = -2112684032;
	// 82ED9088: A0AB0004  lhz r5, 4(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED908C: 5548143A  rlwinm r8, r10, 2, 0x10, 0x1d
	ctx.r[8].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 82ED9090: 3887B048  addi r4, r7, -0x4fb8
	ctx.r[4].s64 = ctx.r[7].s64 + -20408;
	// 82ED9094: 80CB0000  lwz r6, 0(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED9098: 54A7103E  rotlwi r7, r5, 2
	ctx.r[7].u64 = ((ctx.r[5].u32).rotate_left(2)) as u64;
	// 82ED909C: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82ED90A0: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED90A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED90A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82ED90AC: 4E800421  bctrl
	ctx.lr = 0x82ED90B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED90B0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82ED90B4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82ED90B8: 4BFFC369  bl 0x82ed5420
	ctx.lr = 0x82ED90BC;
	sub_82ED5420(ctx, base);
	// 82ED90BC: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED90C0: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82ED90C4: 80DE01E8  lwz r6, 0x1e8(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(488 as u32) ) } as u64;
	// 82ED90C8: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82ED90CC: 388BB03C  addi r4, r11, -0x4fc4
	ctx.r[4].s64 = ctx.r[11].s64 + -20420;
	// 82ED90D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED90D4: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ED90D8: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82ED90DC: 4E800421  bctrl
	ctx.lr = 0x82ED90E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED90E0: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED90E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED90E8: 80E80018  lwz r7, 0x18(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(24 as u32) ) } as u64;
	// 82ED90EC: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 82ED90F0: 4E800421  bctrl
	ctx.lr = 0x82ED90F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED90F4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82ED90F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ED90FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ED9100: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82ED9104: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ED9108: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED9110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED9110 size=184
    let mut pc: u32 = 0x82ED9110;
    'dispatch: loop {
        match pc {
            0x82ED9110 => {
    //   block [0x82ED9110..0x82ED91C8)
	// 82ED9110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED9114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ED9118: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82ED911C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ED9120: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED9124: A12301F4  lhz r9, 0x1f4(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(500 as u32) ) } as u64;
	// 82ED9128: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82ED912C: 3BE301F0  addi r31, r3, 0x1f0
	ctx.r[31].s64 = ctx.r[3].s64 + 496;
	// 82ED9130: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82ED9134: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82ED9138: 40990024  ble cr6, 0x82ed915c
	if !ctx.cr[6].gt {
	pc = 0x82ED915C; continue 'dispatch;
	}
	// 82ED913C: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED9140: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED9144: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82ED9148: 419A0068  beq cr6, 0x82ed91b0
	if ctx.cr[6].eq {
	pc = 0x82ED91B0; continue 'dispatch;
	}
	// 82ED914C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82ED9150: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82ED9154: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82ED9158: 4198FFE8  blt cr6, 0x82ed9140
	if ctx.cr[6].lt {
	pc = 0x82ED9140; continue 'dispatch;
	}
	// 82ED915C: A17F0006  lhz r11, 6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 82ED9160: A15F0004  lhz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED9164: 556904BE  clrlwi r9, r11, 0x12
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00003FFFu64;
	// 82ED9168: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82ED916C: 409A0010  bne cr6, 0x82ed917c
	if !ctx.cr[6].eq {
	pc = 0x82ED917C; continue 'dispatch;
	}
	// 82ED9170: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82ED9174: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED9178: 483261E1  bl 0x831ff358
	ctx.lr = 0x82ED917C;
	sub_831FF358(ctx, base);
	// 82ED917C: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED9180: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED9184: 5569103E  rotlwi r9, r11, 2
	ctx.r[9].u64 = ((ctx.r[11].u32).rotate_left(2)) as u64;
	// 82ED9188: 7FC9512E  stwx r30, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 82ED918C: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED9190: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 82ED9194: B11F0004  sth r8, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[8].u16 ) };
	// 82ED9198: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82ED919C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ED91A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ED91A4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82ED91A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ED91AC: 4E800020  blr
	return;
	// 82ED91B0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ED91B4: 4198FFA8  blt cr6, 0x82ed915c
	if ctx.cr[6].lt {
	pc = 0x82ED915C; continue 'dispatch;
	}
	// 82ED91B8: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED91BC: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82ED91C0: 7FC9512E  stwx r30, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 82ED91C4: 4BFFFFD4  b 0x82ed9198
	pc = 0x82ED9198; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED91C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82ED91C8 size=244
    let mut pc: u32 = 0x82ED91C8;
    'dispatch: loop {
        match pc {
            0x82ED91C8 => {
    //   block [0x82ED91C8..0x82ED92BC)
	// 82ED91C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED91CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ED91D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ED91D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED91D8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82ED91DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ED91E0: 4BFFC3D9  bl 0x82ed55b8
	ctx.lr = 0x82ED91E4;
	sub_82ED55B8(ctx, base);
	// 82ED91E4: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82ED91E8: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82ED91EC: 3CE08201  lis r7, -0x7dff
	ctx.r[7].s64 = -2113863680;
	// 82ED91F0: 3CC00000  lis r6, 0
	ctx.r[6].s64 = 0;
	// 82ED91F4: 38ABB0A0  addi r5, r11, -0x4f60
	ctx.r[5].s64 = ctx.r[11].s64 + -20320;
	// 82ED91F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82ED91FC: 60CA8000  ori r10, r6, 0x8000
	ctx.r[10].u64 = ctx.r[6].u64 | 32768;
	// 82ED9200: 90BF0000  stw r5, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 82ED9204: C0099450  lfs f0, -0x6bb0(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-27568 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82ED9208: 3D008000  lis r8, -0x8000
	ctx.r[8].s64 = -2147483648;
	// 82ED920C: C1A70A90  lfs f13, 0xa90(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(2704 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82ED9210: 3C808212  lis r4, -0x7dee
	ctx.r[4].s64 = -2112749568;
	// 82ED9214: D01F0084  stfs f0, 0x84(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), tmp.u32 ) };
	// 82ED9218: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82ED921C: D1BF0088  stfs f13, 0x88(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(136 as u32), tmp.u32 ) };
	// 82ED9220: 917F0098  stw r11, 0x98(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(152 as u32), ctx.r[11].u32 ) };
	// 82ED9224: B17F009C  sth r11, 0x9c(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(156 as u32), ctx.r[11].u16 ) };
	// 82ED9228: 38600003  li r3, 3
	ctx.r[3].s64 = 3;
	// 82ED922C: B15F009E  sth r10, 0x9e(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(158 as u32), ctx.r[10].u16 ) };
	// 82ED9230: 38E44144  addi r7, r4, 0x4144
	ctx.r[7].s64 = ctx.r[4].s64 + 16708;
	// 82ED9234: 917F00A0  stw r11, 0xa0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(160 as u32), ctx.r[11].u32 ) };
	// 82ED9238: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 82ED923C: 917F00A4  stw r11, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[11].u32 ) };
	// 82ED9240: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82ED9244: 911F00A8  stw r8, 0xa8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(168 as u32), ctx.r[8].u32 ) };
	// 82ED9248: 917F00AC  stw r11, 0xac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(172 as u32), ctx.r[11].u32 ) };
	// 82ED924C: 917F00B0  stw r11, 0xb0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 82ED9250: 911F00B4  stw r8, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[8].u32 ) };
	// 82ED9254: 987F00CA  stb r3, 0xca(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(202 as u32), ctx.r[3].u8 ) };
	// 82ED9258: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED925C: 917F00C4  stw r11, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[11].u32 ) };
	// 82ED9260: B17F00C8  sth r11, 0xc8(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(200 as u32), ctx.r[11].u16 ) };
	// 82ED9264: 993F00CB  stb r9, 0xcb(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(203 as u32), ctx.r[9].u8 ) };
	// 82ED9268: B13F00D6  sth r9, 0xd6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(214 as u32), ctx.r[9].u16 ) };
	// 82ED926C: 917F01E8  stw r11, 0x1e8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(488 as u32), ctx.r[11].u32 ) };
	// 82ED9270: 90FF00D0  stw r7, 0xd0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(208 as u32), ctx.r[7].u32 ) };
	// 82ED9274: 917F01F0  stw r11, 0x1f0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(496 as u32), ctx.r[11].u32 ) };
	// 82ED9278: B17F01F4  sth r11, 0x1f4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(500 as u32), ctx.r[11].u16 ) };
	// 82ED927C: B15F01F6  sth r10, 0x1f6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(502 as u32), ctx.r[10].u16 ) };
	// 82ED9280: 917F01F8  stw r11, 0x1f8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(504 as u32), ctx.r[11].u32 ) };
	// 82ED9284: B17F01FC  sth r11, 0x1fc(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(508 as u32), ctx.r[11].u16 ) };
	// 82ED9288: B15F01FE  sth r10, 0x1fe(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(510 as u32), ctx.r[10].u16 ) };
	// 82ED928C: B0DF0094  sth r6, 0x94(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[6].u16 ) };
	// 82ED9290: 917F00B8  stw r11, 0xb8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(184 as u32), ctx.r[11].u32 ) };
	// 82ED9294: 917F00CC  stw r11, 0xcc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(204 as u32), ctx.r[11].u32 ) };
	// 82ED9298: 917F008C  stw r11, 0x8c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(140 as u32), ctx.r[11].u32 ) };
	// 82ED929C: 90BF00C0  stw r5, 0xc0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(192 as u32), ctx.r[5].u32 ) };
	// 82ED92A0: 917F0090  stw r11, 0x90(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), ctx.r[11].u32 ) };
	// 82ED92A4: 997F00BD  stb r11, 0xbd(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(189 as u32), ctx.r[11].u8 ) };
	// 82ED92A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82ED92AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ED92B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ED92B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ED92B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED92C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED92C0 size=436
    let mut pc: u32 = 0x82ED92C0;
    'dispatch: loop {
        match pc {
            0x82ED92C0 => {
    //   block [0x82ED92C0..0x82ED93B4)
	// 82ED92C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED92C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ED92C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82ED92CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ED92D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED92D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ED92D8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82ED92DC: 4BFFC255  bl 0x82ed5530
	ctx.lr = 0x82ED92E0;
	sub_82ED5530(ctx, base);
	// 82ED92E0: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82ED92E4: 3D400000  lis r10, 0
	ctx.r[10].s64 = 0;
	// 82ED92E8: 390BB0A0  addi r8, r11, -0x4f60
	ctx.r[8].s64 = ctx.r[11].s64 + -20320;
	// 82ED92EC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82ED92F0: 61498000  ori r9, r10, 0x8000
	ctx.r[9].u64 = ctx.r[10].u64 | 32768;
	// 82ED92F4: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82ED92F8: 3CE08000  lis r7, -0x8000
	ctx.r[7].s64 = -2147483648;
	// 82ED92FC: 917F0098  stw r11, 0x98(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(152 as u32), ctx.r[11].u32 ) };
	// 82ED9300: 3CC08212  lis r6, -0x7dee
	ctx.r[6].s64 = -2112749568;
	// 82ED9304: B17F009C  sth r11, 0x9c(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(156 as u32), ctx.r[11].u16 ) };
	// 82ED9308: B13F009E  sth r9, 0x9e(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(158 as u32), ctx.r[9].u16 ) };
	// 82ED930C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82ED9310: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 82ED9314: 917F00A0  stw r11, 0xa0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(160 as u32), ctx.r[11].u32 ) };
	// 82ED9318: 917F00A4  stw r11, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[11].u32 ) };
	// 82ED931C: 38864144  addi r4, r6, 0x4144
	ctx.r[4].s64 = ctx.r[6].s64 + 16708;
	// 82ED9320: 90FF00A8  stw r7, 0xa8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(168 as u32), ctx.r[7].u32 ) };
	// 82ED9324: 395F00D0  addi r10, r31, 0xd0
	ctx.r[10].s64 = ctx.r[31].s64 + 208;
	// 82ED9328: 917F00AC  stw r11, 0xac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(172 as u32), ctx.r[11].u32 ) };
	// 82ED932C: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82ED9330: 917F00B0  stw r11, 0xb0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 82ED9334: 90FF00B4  stw r7, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[7].u32 ) };
	// 82ED9338: 917F00C4  stw r11, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[11].u32 ) };
	// 82ED933C: B17F00C8  sth r11, 0xc8(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(200 as u32), ctx.r[11].u16 ) };
	// 82ED9340: 98BF00CA  stb r5, 0xca(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(202 as u32), ctx.r[5].u8 ) };
	// 82ED9344: 991F00CB  stb r8, 0xcb(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(203 as u32), ctx.r[8].u8 ) };
	// 82ED9348: 909F00D0  stw r4, 0xd0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(208 as u32), ctx.r[4].u32 ) };
	// 82ED934C: B11F00D6  sth r8, 0xd6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(214 as u32), ctx.r[8].u16 ) };
	// 82ED9350: 917F01F0  stw r11, 0x1f0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(496 as u32), ctx.r[11].u32 ) };
	// 82ED9354: B17F01F4  sth r11, 0x1f4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(500 as u32), ctx.r[11].u16 ) };
	// 82ED9358: B13F01F6  sth r9, 0x1f6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(502 as u32), ctx.r[9].u16 ) };
	// 82ED935C: 917F01F8  stw r11, 0x1f8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(504 as u32), ctx.r[11].u32 ) };
	// 82ED9360: B17F01FC  sth r11, 0x1fc(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(508 as u32), ctx.r[11].u16 ) };
	// 82ED9364: B13F01FE  sth r9, 0x1fe(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(510 as u32), ctx.r[9].u16 ) };
	// 82ED9368: 419A00F0  beq cr6, 0x82ed9458
	if ctx.cr[6].eq {
	pc = 0x82ED9458; continue 'dispatch;
	}
	// 82ED936C: 897F00D8  lbz r11, 0xd8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(216 as u32) ) } as u64;
	// 82ED9370: 396BFFFE  addi r11, r11, -2
	ctx.r[11].s64 = ctx.r[11].s64 + -2;
	// 82ED9374: 2B0B0007  cmplwi cr6, r11, 7
	ctx.cr[6].compare_u32(ctx.r[11].u32, 7 as u32, &mut ctx.xer);
	// 82ED9378: 419900E0  bgt cr6, 0x82ed9458
	if ctx.cr[6].gt {
	pc = 0x82ED9458; continue 'dispatch;
	}
	// 82ED937C: 3D8082EE  lis r12, -0x7d12
	ctx.r[12].s64 = -2098331648;
	// 82ED9380: 398C9394  addi r12, r12, -0x6c6c
	ctx.r[12].s64 = ctx.r[12].s64 + -27756;
	// 82ED9384: 5560103A  slwi r0, r11, 2
	ctx.r[0].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 82ED9388: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 82ED938C: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 82ED9390: 4E800420  bctr
	match ctx.r[11].u64 {
		0 => {
	pc = 0x82ED93B4; continue 'dispatch;
		},
		1 => {
	pc = 0x82ED93C8; continue 'dispatch;
		},
		2 => {
	pc = 0x82ED93DC; continue 'dispatch;
		},
		3 => {
	pc = 0x82ED93F0; continue 'dispatch;
		},
		4 => {
	pc = 0x82ED9404; continue 'dispatch;
		},
		5 => {
	pc = 0x82ED9418; continue 'dispatch;
		},
		6 => {
	pc = 0x82ED942C; continue 'dispatch;
		},
		7 => {
	pc = 0x82ED9440; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 82ED9394: 82ED93B4  lwz r23, -0x6c4c(r13)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(-27724 as u32) ) } as u64;
	// 82ED9398: 82ED93C8  lwz r23, -0x6c38(r13)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(-27704 as u32) ) } as u64;
	// 82ED939C: 82ED93DC  lwz r23, -0x6c24(r13)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(-27684 as u32) ) } as u64;
	// 82ED93A0: 82ED93F0  lwz r23, -0x6c10(r13)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82ED93A4: 82ED9404  lwz r23, -0x6bfc(r13)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(-27644 as u32) ) } as u64;
	// 82ED93A8: 82ED9418  lwz r23, -0x6be8(r13)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(-27624 as u32) ) } as u64;
	// 82ED93AC: 82ED942C  lwz r23, -0x6bd4(r13)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(-27604 as u32) ) } as u64;
	// 82ED93B0: 82ED9440  lwz r23, -0x6bc0(r13)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(-27584 as u32) ) } as u64;
            }
            0x82ED93B4 => {
    //   block [0x82ED93B4..0x82ED93C8)
	// 82ED93B4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82ED93B8: 419A00A0  beq cr6, 0x82ed9458
	if ctx.cr[6].eq {
	pc = 0x82ED9458; continue 'dispatch;
	}
	// 82ED93BC: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82ED93C0: 392B3F34  addi r9, r11, 0x3f34
	ctx.r[9].s64 = ctx.r[11].s64 + 16180;
	// 82ED93C4: 4800008C  b 0x82ed9450
	pc = 0x82ED9450; continue 'dispatch;
            }
            0x82ED93C8 => {
    //   block [0x82ED93C8..0x82ED93DC)
	// 82ED93C8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82ED93CC: 419A008C  beq cr6, 0x82ed9458
	if ctx.cr[6].eq {
	pc = 0x82ED9458; continue 'dispatch;
	}
	// 82ED93D0: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82ED93D4: 392B3F9C  addi r9, r11, 0x3f9c
	ctx.r[9].s64 = ctx.r[11].s64 + 16284;
	// 82ED93D8: 48000078  b 0x82ed9450
	pc = 0x82ED9450; continue 'dispatch;
            }
            0x82ED93DC => {
    //   block [0x82ED93DC..0x82ED93F0)
	// 82ED93DC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82ED93E0: 419A0078  beq cr6, 0x82ed9458
	if ctx.cr[6].eq {
	pc = 0x82ED9458; continue 'dispatch;
	}
	// 82ED93E4: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82ED93E8: 392B9FCC  addi r9, r11, -0x6034
	ctx.r[9].s64 = ctx.r[11].s64 + -24628;
	// 82ED93EC: 48000064  b 0x82ed9450
	pc = 0x82ED9450; continue 'dispatch;
            }
            0x82ED93F0 => {
    //   block [0x82ED93F0..0x82ED9404)
	// 82ED93F0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82ED93F4: 419A0064  beq cr6, 0x82ed9458
	if ctx.cr[6].eq {
	pc = 0x82ED9458; continue 'dispatch;
	}
	// 82ED93F8: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82ED93FC: 392BA0CC  addi r9, r11, -0x5f34
	ctx.r[9].s64 = ctx.r[11].s64 + -24372;
	// 82ED9400: 48000050  b 0x82ed9450
	pc = 0x82ED9450; continue 'dispatch;
            }
            0x82ED9404 => {
    //   block [0x82ED9404..0x82ED9418)
	// 82ED9404: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82ED9408: 419A0050  beq cr6, 0x82ed9458
	if ctx.cr[6].eq {
	pc = 0x82ED9458; continue 'dispatch;
	}
	// 82ED940C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82ED9410: 392B40D4  addi r9, r11, 0x40d4
	ctx.r[9].s64 = ctx.r[11].s64 + 16596;
	// 82ED9414: 4800003C  b 0x82ed9450
	pc = 0x82ED9450; continue 'dispatch;
            }
            0x82ED9418 => {
    //   block [0x82ED9418..0x82ED942C)
	// 82ED9418: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82ED941C: 419A003C  beq cr6, 0x82ed9458
	if ctx.cr[6].eq {
	pc = 0x82ED9458; continue 'dispatch;
	}
	// 82ED9420: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82ED9424: 392BA8CC  addi r9, r11, -0x5734
	ctx.r[9].s64 = ctx.r[11].s64 + -22324;
	// 82ED9428: 48000028  b 0x82ed9450
	pc = 0x82ED9450; continue 'dispatch;
            }
            0x82ED942C => {
    //   block [0x82ED942C..0x82ED9440)
	// 82ED942C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82ED9430: 419A0028  beq cr6, 0x82ed9458
	if ctx.cr[6].eq {
	pc = 0x82ED9458; continue 'dispatch;
	}
	// 82ED9434: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82ED9438: 392BA034  addi r9, r11, -0x5fcc
	ctx.r[9].s64 = ctx.r[11].s64 + -24524;
	// 82ED943C: 48000014  b 0x82ed9450
	pc = 0x82ED9450; continue 'dispatch;
            }
            0x82ED9440 => {
    //   block [0x82ED9440..0x82ED9474)
	// 82ED9440: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82ED9444: 419A0014  beq cr6, 0x82ed9458
	if ctx.cr[6].eq {
	pc = 0x82ED9458; continue 'dispatch;
	}
	// 82ED9448: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82ED944C: 392B4004  addi r9, r11, 0x4004
	ctx.r[9].s64 = ctx.r[11].s64 + 16388;
	// 82ED9450: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82ED9454: B10A0006  sth r8, 6(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 82ED9458: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED945C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82ED9460: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ED9464: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ED9468: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82ED946C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ED9470: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED9478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED9478 size=304
    let mut pc: u32 = 0x82ED9478;
    'dispatch: loop {
        match pc {
            0x82ED9478 => {
    //   block [0x82ED9478..0x82ED95A8)
	// 82ED9478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED947C: 482CECF1  bl 0x831a816c
	ctx.lr = 0x82ED9480;
	sub_831A8130(ctx, base);
	// 82ED9480: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED9484: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82ED9488: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82ED948C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82ED9490: 817E00CC  lwz r11, 0xcc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(204 as u32) ) } as u64;
	// 82ED9494: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED9498: 409A0078  bne cr6, 0x82ed9510
	if !ctx.cr[6].eq {
	pc = 0x82ED9510; continue 'dispatch;
	}
	// 82ED949C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED94A0: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82ED94A4: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82ED94A8: 81630048  lwz r11, 0x48(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 82ED94AC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED94B0: 419A001C  beq cr6, 0x82ed94cc
	if ctx.cr[6].eq {
	pc = 0x82ED94CC; continue 'dispatch;
	}
	// 82ED94B4: 8143004C  lwz r10, 0x4c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(76 as u32) ) } as u64;
	// 82ED94B8: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82ED94BC: 9143004C  stw r10, 0x4c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(76 as u32), ctx.r[10].u32 ) };
	// 82ED94C0: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED94C4: 91230048  stw r9, 0x48(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), ctx.r[9].u32 ) };
	// 82ED94C8: 48000014  b 0x82ed94dc
	pc = 0x82ED94DC; continue 'dispatch;
	// 82ED94CC: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82ED94D0: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 82ED94D4: 4BFC718D  bl 0x82ea0660
	ctx.lr = 0x82ED94D8;
	sub_82EA0660(ctx, base);
	// 82ED94D8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82ED94DC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED94E0: 419A0028  beq cr6, 0x82ed9508
	if ctx.cr[6].eq {
	pc = 0x82ED9508; continue 'dispatch;
	}
	// 82ED94E4: 3D400000  lis r10, 0
	ctx.r[10].s64 = 0;
	// 82ED94E8: 93EB0000  stw r31, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82ED94EC: B3EB0004  sth r31, 4(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[31].u16 ) };
	// 82ED94F0: 614A8000  ori r10, r10, 0x8000
	ctx.r[10].u64 = ctx.r[10].u64 | 32768;
	// 82ED94F4: B14B0006  sth r10, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82ED94F8: 93EB0008  stw r31, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 82ED94FC: B3EB000C  sth r31, 0xc(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[31].u16 ) };
	// 82ED9500: B14B000E  sth r10, 0xe(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(14 as u32), ctx.r[10].u16 ) };
	// 82ED9504: 48000008  b 0x82ed950c
	pc = 0x82ED950C; continue 'dispatch;
	// 82ED9508: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82ED950C: 917E00CC  stw r11, 0xcc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(204 as u32), ctx.r[11].u32 ) };
	// 82ED9510: 815E00CC  lwz r10, 0xcc(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(204 as u32) ) } as u64;
	// 82ED9514: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82ED9518: 3BEA0008  addi r31, r10, 8
	ctx.r[31].s64 = ctx.r[10].s64 + 8;
	// 82ED951C: A12A000C  lhz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ED9520: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82ED9524: 4099002C  ble cr6, 0x82ed9550
	if !ctx.cr[6].gt {
	pc = 0x82ED9550; continue 'dispatch;
	}
	// 82ED9528: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED952C: 7D0A4378  mr r10, r8
	ctx.r[10].u64 = ctx.r[8].u64;
	// 82ED9530: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED9534: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82ED9538: 419A0058  beq cr6, 0x82ed9590
	if ctx.cr[6].eq {
	pc = 0x82ED9590; continue 'dispatch;
	}
	// 82ED953C: A0FF0004  lhz r7, 4(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED9540: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82ED9544: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82ED9548: 7F0B3800  cmpw cr6, r11, r7
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[7].s32, &mut ctx.xer);
	// 82ED954C: 4198FFE4  blt cr6, 0x82ed9530
	if ctx.cr[6].lt {
	pc = 0x82ED9530; continue 'dispatch;
	}
	// 82ED9550: A17F0006  lhz r11, 6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 82ED9554: 556A04BE  clrlwi r10, r11, 0x12
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00003FFFu64;
	// 82ED9558: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82ED955C: 409A0010  bne cr6, 0x82ed956c
	if !ctx.cr[6].eq {
	pc = 0x82ED956C; continue 'dispatch;
	}
	// 82ED9560: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82ED9564: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED9568: 48325DF1  bl 0x831ff358
	ctx.lr = 0x82ED956C;
	sub_831FF358(ctx, base);
	// 82ED956C: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED9570: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED9574: 5569103E  rotlwi r9, r11, 2
	ctx.r[9].u64 = ((ctx.r[11].u32).rotate_left(2)) as u64;
	// 82ED9578: 7FA9512E  stwx r29, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[29].u32) };
	// 82ED957C: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED9580: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 82ED9584: B11F0004  sth r8, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[8].u16 ) };
	// 82ED9588: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82ED958C: 482CEC30  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 82ED9590: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ED9594: 4198FFBC  blt cr6, 0x82ed9550
	if ctx.cr[6].lt {
	pc = 0x82ED9550; continue 'dispatch;
	}
	// 82ED9598: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82ED959C: 7FAB412E  stwx r29, r11, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32), ctx.r[29].u32) };
	// 82ED95A0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82ED95A4: 482CEC18  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED95A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED95A8 size=228
    let mut pc: u32 = 0x82ED95A8;
    'dispatch: loop {
        match pc {
            0x82ED95A8 => {
    //   block [0x82ED95A8..0x82ED968C)
	// 82ED95A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED95AC: 482CEBC1  bl 0x831a816c
	ctx.lr = 0x82ED95B0;
	sub_831A8130(ctx, base);
	// 82ED95B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED95B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ED95B8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82ED95BC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82ED95C0: 817F00CC  lwz r11, 0xcc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 82ED95C4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED95C8: 409A0078  bne cr6, 0x82ed9640
	if !ctx.cr[6].eq {
	pc = 0x82ED9640; continue 'dispatch;
	}
	// 82ED95CC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED95D0: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82ED95D4: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82ED95D8: 81630048  lwz r11, 0x48(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 82ED95DC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED95E0: 419A001C  beq cr6, 0x82ed95fc
	if ctx.cr[6].eq {
	pc = 0x82ED95FC; continue 'dispatch;
	}
	// 82ED95E4: 8143004C  lwz r10, 0x4c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(76 as u32) ) } as u64;
	// 82ED95E8: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82ED95EC: 9143004C  stw r10, 0x4c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(76 as u32), ctx.r[10].u32 ) };
	// 82ED95F0: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED95F4: 91230048  stw r9, 0x48(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), ctx.r[9].u32 ) };
	// 82ED95F8: 48000014  b 0x82ed960c
	pc = 0x82ED960C; continue 'dispatch;
	// 82ED95FC: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82ED9600: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 82ED9604: 4BFC705D  bl 0x82ea0660
	ctx.lr = 0x82ED9608;
	sub_82EA0660(ctx, base);
	// 82ED9608: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82ED960C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED9610: 419A0028  beq cr6, 0x82ed9638
	if ctx.cr[6].eq {
	pc = 0x82ED9638; continue 'dispatch;
	}
	// 82ED9614: 3D400000  lis r10, 0
	ctx.r[10].s64 = 0;
	// 82ED9618: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82ED961C: B3CB0004  sth r30, 4(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[30].u16 ) };
	// 82ED9620: 614A8000  ori r10, r10, 0x8000
	ctx.r[10].u64 = ctx.r[10].u64 | 32768;
	// 82ED9624: B14B0006  sth r10, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82ED9628: 93CB0008  stw r30, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82ED962C: B3CB000C  sth r30, 0xc(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[30].u16 ) };
	// 82ED9630: B14B000E  sth r10, 0xe(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(14 as u32), ctx.r[10].u16 ) };
	// 82ED9634: 48000008  b 0x82ed963c
	pc = 0x82ED963C; continue 'dispatch;
	// 82ED9638: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82ED963C: 917F00CC  stw r11, 0xcc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(204 as u32), ctx.r[11].u32 ) };
	// 82ED9640: 811F00CC  lwz r8, 0xcc(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 82ED9644: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82ED9648: A128000C  lhz r9, 0xc(r8)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[8].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ED964C: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82ED9650: 40990024  ble cr6, 0x82ed9674
	if !ctx.cr[6].gt {
	pc = 0x82ED9674; continue 'dispatch;
	}
	// 82ED9654: 81480008  lwz r10, 8(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED9658: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED965C: 7F07E840  cmplw cr6, r7, r29
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82ED9660: 419A0018  beq cr6, 0x82ed9678
	if ctx.cr[6].eq {
	pc = 0x82ED9678; continue 'dispatch;
	}
	// 82ED9664: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82ED9668: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82ED966C: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82ED9670: 4198FFE8  blt cr6, 0x82ed9658
	if ctx.cr[6].lt {
	pc = 0x82ED9658; continue 'dispatch;
	}
	// 82ED9674: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82ED9678: 81480008  lwz r10, 8(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED967C: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82ED9680: 7FC9512E  stwx r30, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 82ED9684: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82ED9688: 482CEB34  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED9690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED9690 size=300
    let mut pc: u32 = 0x82ED9690;
    'dispatch: loop {
        match pc {
            0x82ED9690 => {
    //   block [0x82ED9690..0x82ED97BC)
	// 82ED9690: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED9694: 482CEAD9  bl 0x831a816c
	ctx.lr = 0x82ED9698;
	sub_831A8130(ctx, base);
	// 82ED9698: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED969C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ED96A0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82ED96A4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82ED96A8: 817F00CC  lwz r11, 0xcc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 82ED96AC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED96B0: 409A0078  bne cr6, 0x82ed9728
	if !ctx.cr[6].eq {
	pc = 0x82ED9728; continue 'dispatch;
	}
	// 82ED96B4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED96B8: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82ED96BC: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82ED96C0: 81630048  lwz r11, 0x48(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 82ED96C4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED96C8: 419A001C  beq cr6, 0x82ed96e4
	if ctx.cr[6].eq {
	pc = 0x82ED96E4; continue 'dispatch;
	}
	// 82ED96CC: 8143004C  lwz r10, 0x4c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(76 as u32) ) } as u64;
	// 82ED96D0: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82ED96D4: 9143004C  stw r10, 0x4c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(76 as u32), ctx.r[10].u32 ) };
	// 82ED96D8: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED96DC: 91230048  stw r9, 0x48(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), ctx.r[9].u32 ) };
	// 82ED96E0: 48000014  b 0x82ed96f4
	pc = 0x82ED96F4; continue 'dispatch;
	// 82ED96E4: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82ED96E8: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 82ED96EC: 4BFC6F75  bl 0x82ea0660
	ctx.lr = 0x82ED96F0;
	sub_82EA0660(ctx, base);
	// 82ED96F0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82ED96F4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED96F8: 419A0028  beq cr6, 0x82ed9720
	if ctx.cr[6].eq {
	pc = 0x82ED9720; continue 'dispatch;
	}
	// 82ED96FC: 3D400000  lis r10, 0
	ctx.r[10].s64 = 0;
	// 82ED9700: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82ED9704: B3CB0004  sth r30, 4(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[30].u16 ) };
	// 82ED9708: 614A8000  ori r10, r10, 0x8000
	ctx.r[10].u64 = ctx.r[10].u64 | 32768;
	// 82ED970C: B14B0006  sth r10, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82ED9710: 93CB0008  stw r30, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82ED9714: B3CB000C  sth r30, 0xc(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[30].u16 ) };
	// 82ED9718: B14B000E  sth r10, 0xe(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(14 as u32), ctx.r[10].u16 ) };
	// 82ED971C: 48000008  b 0x82ed9724
	pc = 0x82ED9724; continue 'dispatch;
	// 82ED9720: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82ED9724: 917F00CC  stw r11, 0xcc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(204 as u32), ctx.r[11].u32 ) };
	// 82ED9728: 83FF00CC  lwz r31, 0xcc(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 82ED972C: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82ED9730: A11F0004  lhz r8, 4(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED9734: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82ED9738: 4099002C  ble cr6, 0x82ed9764
	if !ctx.cr[6].gt {
	pc = 0x82ED9764; continue 'dispatch;
	}
	// 82ED973C: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED9740: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 82ED9744: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED9748: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82ED974C: 419A0058  beq cr6, 0x82ed97a4
	if ctx.cr[6].eq {
	pc = 0x82ED97A4; continue 'dispatch;
	}
	// 82ED9750: A0FF0004  lhz r7, 4(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED9754: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82ED9758: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82ED975C: 7F0B3800  cmpw cr6, r11, r7
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[7].s32, &mut ctx.xer);
	// 82ED9760: 4198FFE4  blt cr6, 0x82ed9744
	if ctx.cr[6].lt {
	pc = 0x82ED9744; continue 'dispatch;
	}
	// 82ED9764: A17F0006  lhz r11, 6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 82ED9768: 556A04BE  clrlwi r10, r11, 0x12
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00003FFFu64;
	// 82ED976C: 7F085040  cmplw cr6, r8, r10
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82ED9770: 409A0010  bne cr6, 0x82ed9780
	if !ctx.cr[6].eq {
	pc = 0x82ED9780; continue 'dispatch;
	}
	// 82ED9774: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82ED9778: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED977C: 48325BDD  bl 0x831ff358
	ctx.lr = 0x82ED9780;
	sub_831FF358(ctx, base);
	// 82ED9780: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED9784: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED9788: 5569103E  rotlwi r9, r11, 2
	ctx.r[9].u64 = ((ctx.r[11].u32).rotate_left(2)) as u64;
	// 82ED978C: 7FA9512E  stwx r29, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[29].u32) };
	// 82ED9790: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED9794: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 82ED9798: B11F0004  sth r8, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[8].u16 ) };
	// 82ED979C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82ED97A0: 482CEA1C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 82ED97A4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ED97A8: 4198FFBC  blt cr6, 0x82ed9764
	if ctx.cr[6].lt {
	pc = 0x82ED9764; continue 'dispatch;
	}
	// 82ED97AC: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82ED97B0: 7FAB492E  stwx r29, r11, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32), ctx.r[29].u32) };
	// 82ED97B4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82ED97B8: 482CEA04  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED97C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED97C0 size=1120
    let mut pc: u32 = 0x82ED97C0;
    'dispatch: loop {
        match pc {
            0x82ED97C0 => {
    //   block [0x82ED97C0..0x82ED9C20)
	// 82ED97C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED97C4: 482CE99D  bl 0x831a8160
	ctx.lr = 0x82ED97C8;
	sub_831A8130(ctx, base);
	// 82ED97C8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED97CC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82ED97D0: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 82ED97D4: 3BFC005C  addi r31, r28, 0x5c
	ctx.r[31].s64 = ctx.r[28].s64 + 92;
	// 82ED97D8: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82ED97DC: 817C0060  lwz r11, 0x60(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(96 as u32) ) } as u64;
	// 82ED97E0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ED97E4: 409A0044  bne cr6, 0x82ed9828
	if !ctx.cr[6].eq {
	pc = 0x82ED9828; continue 'dispatch;
	}
	// 82ED97E8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED97EC: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED97F0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ED97F4: 409A0020  bne cr6, 0x82ed9814
	if !ctx.cr[6].eq {
	pc = 0x82ED9814; continue 'dispatch;
	}
	// 82ED97F8: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED97FC: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82ED9800: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82ED9804: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED9808: 55651838  slwi r5, r11, 3
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82ED980C: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82ED9810: 4BFC6FA1  bl 0x82ea07b0
	ctx.lr = 0x82ED9814;
	sub_82EA07B0(ctx, base);
	// 82ED9814: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED9818: 937F0000  stw r27, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82ED981C: 534BF880  rlwimi r11, r26, 0x1f, 2, 0
	ctx.r[11].u64 = (((ctx.r[26].u32).rotate_left(31) as u64) & 0xFFFFFFFFBFFFFFFF) | (ctx.r[11].u64 & 0x0000000040000000);
	// 82ED9820: 937F0004  stw r27, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[27].u32 ) };
	// 82ED9824: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82ED9828: A17C009C  lhz r11, 0x9c(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[28].u32.wrapping_add(156 as u32) ) } as u64;
	// 82ED982C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED9830: 409A0058  bne cr6, 0x82ed9888
	if !ctx.cr[6].eq {
	pc = 0x82ED9888; continue 'dispatch;
	}
	// 82ED9834: A17C009E  lhz r11, 0x9e(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[28].u32.wrapping_add(158 as u32) ) } as u64;
	// 82ED9838: 3BFC0098  addi r31, r28, 0x98
	ctx.r[31].s64 = ctx.r[28].s64 + 152;
	// 82ED983C: 556A0020  rlwinm r10, r11, 0, 0, 0x10
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED9840: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82ED9844: 409A002C  bne cr6, 0x82ed9870
	if !ctx.cr[6].eq {
	pc = 0x82ED9870; continue 'dispatch;
	}
	// 82ED9848: 556A04BE  clrlwi r10, r11, 0x12
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00003FFFu64;
	// 82ED984C: 812D0000  lwz r9, 0(r13)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED9850: 556B0C7C  rlwinm r11, r11, 1, 0x11, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x7FFFFFFFu64;
	// 82ED9854: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED9858: 39000014  li r8, 0x14
	ctx.r[8].s64 = 20;
	// 82ED985C: 7CEA5A14  add r7, r10, r11
	ctx.r[7].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82ED9860: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82ED9864: 54E52036  slwi r5, r7, 4
	ctx.r[5].u32 = ctx.r[7].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82ED9868: 7C68482E  lwzx r3, r8, r9
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82ED986C: 4BFC6F45  bl 0x82ea07b0
	ctx.lr = 0x82ED9870;
	sub_82EA07B0(ctx, base);
	// 82ED9870: A17F0006  lhz r11, 6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 82ED9874: 937F0000  stw r27, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82ED9878: 556A0462  rlwinm r10, r11, 0, 0x11, 0x11
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED987C: B37F0004  sth r27, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[27].u16 ) };
	// 82ED9880: 61498000  ori r9, r10, 0x8000
	ctx.r[9].u64 = ctx.r[10].u64 | 32768;
	// 82ED9884: B13F0006  sth r9, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82ED9888: 817C00A4  lwz r11, 0xa4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(164 as u32) ) } as u64;
	// 82ED988C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ED9890: 409A0048  bne cr6, 0x82ed98d8
	if !ctx.cr[6].eq {
	pc = 0x82ED98D8; continue 'dispatch;
	}
	// 82ED9894: 817C00A8  lwz r11, 0xa8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(168 as u32) ) } as u64;
	// 82ED9898: 3BFC00A0  addi r31, r28, 0xa0
	ctx.r[31].s64 = ctx.r[28].s64 + 160;
	// 82ED989C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED98A0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ED98A4: 409A0020  bne cr6, 0x82ed98c4
	if !ctx.cr[6].eq {
	pc = 0x82ED98C4; continue 'dispatch;
	}
	// 82ED98A8: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED98AC: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82ED98B0: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82ED98B4: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED98B8: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82ED98BC: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82ED98C0: 4BFC6EF1  bl 0x82ea07b0
	ctx.lr = 0x82ED98C4;
	sub_82EA07B0(ctx, base);
	// 82ED98C4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED98C8: 937F0000  stw r27, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82ED98CC: 534BF880  rlwimi r11, r26, 0x1f, 2, 0
	ctx.r[11].u64 = (((ctx.r[26].u32).rotate_left(31) as u64) & 0xFFFFFFFFBFFFFFFF) | (ctx.r[11].u64 & 0x0000000040000000);
	// 82ED98D0: 937F0004  stw r27, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[27].u32 ) };
	// 82ED98D4: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82ED98D8: 817C00B0  lwz r11, 0xb0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(176 as u32) ) } as u64;
	// 82ED98DC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ED98E0: 409A0048  bne cr6, 0x82ed9928
	if !ctx.cr[6].eq {
	pc = 0x82ED9928; continue 'dispatch;
	}
	// 82ED98E4: 817C00B4  lwz r11, 0xb4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(180 as u32) ) } as u64;
	// 82ED98E8: 3BFC00AC  addi r31, r28, 0xac
	ctx.r[31].s64 = ctx.r[28].s64 + 172;
	// 82ED98EC: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED98F0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ED98F4: 409A0020  bne cr6, 0x82ed9914
	if !ctx.cr[6].eq {
	pc = 0x82ED9914; continue 'dispatch;
	}
	// 82ED98F8: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED98FC: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82ED9900: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82ED9904: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED9908: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82ED990C: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82ED9910: 4BFC6EA1  bl 0x82ea07b0
	ctx.lr = 0x82ED9914;
	sub_82EA07B0(ctx, base);
	// 82ED9914: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED9918: 937F0000  stw r27, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82ED991C: 534BF880  rlwimi r11, r26, 0x1f, 2, 0
	ctx.r[11].u64 = (((ctx.r[26].u32).rotate_left(31) as u64) & 0xFFFFFFFFBFFFFFFF) | (ctx.r[11].u64 & 0x0000000040000000);
	// 82ED9920: 937F0004  stw r27, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[27].u32 ) };
	// 82ED9924: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82ED9928: A15C01F4  lhz r10, 0x1f4(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[28].u32.wrapping_add(500 as u32) ) } as u64;
	// 82ED992C: 7F69DB78  mr r9, r27
	ctx.r[9].u64 = ctx.r[27].u64;
	// 82ED9930: 7F6BDB78  mr r11, r27
	ctx.r[11].u64 = ctx.r[27].u64;
	// 82ED9934: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82ED9938: 419A0030  beq cr6, 0x82ed9968
	if ctx.cr[6].eq {
	pc = 0x82ED9968; continue 'dispatch;
	}
	// 82ED993C: 815C01F0  lwz r10, 0x1f0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(496 as u32) ) } as u64;
	// 82ED9940: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED9944: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82ED9948: 409A001C  bne cr6, 0x82ed9964
	if !ctx.cr[6].eq {
	pc = 0x82ED9964; continue 'dispatch;
	}
	// 82ED994C: A11C01F4  lhz r8, 0x1f4(r28)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[28].u32.wrapping_add(500 as u32) ) } as u64;
	// 82ED9950: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82ED9954: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82ED9958: 7F0B4000  cmpw cr6, r11, r8
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82ED995C: 4198FFE4  blt cr6, 0x82ed9940
	if ctx.cr[6].lt {
	pc = 0x82ED9940; continue 'dispatch;
	}
	// 82ED9960: 48000008  b 0x82ed9968
	pc = 0x82ED9968; continue 'dispatch;
	// 82ED9964: 7F49D378  mr r9, r26
	ctx.r[9].u64 = ctx.r[26].u64;
	// 82ED9968: 7D2B0774  extsb r11, r9
	ctx.r[11].s64 = ctx.r[9].s8 as i64;
	// 82ED996C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ED9970: 409A004C  bne cr6, 0x82ed99bc
	if !ctx.cr[6].eq {
	pc = 0x82ED99BC; continue 'dispatch;
	}
	// 82ED9974: A17C01F6  lhz r11, 0x1f6(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[28].u32.wrapping_add(502 as u32) ) } as u64;
	// 82ED9978: 3BFC01F0  addi r31, r28, 0x1f0
	ctx.r[31].s64 = ctx.r[28].s64 + 496;
	// 82ED997C: 556A0020  rlwinm r10, r11, 0, 0, 0x10
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED9980: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82ED9984: 409A0020  bne cr6, 0x82ed99a4
	if !ctx.cr[6].eq {
	pc = 0x82ED99A4; continue 'dispatch;
	}
	// 82ED9988: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED998C: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82ED9990: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82ED9994: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED9998: 5565143A  rlwinm r5, r11, 2, 0x10, 0x1d
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82ED999C: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82ED99A0: 4BFC6E11  bl 0x82ea07b0
	ctx.lr = 0x82ED99A4;
	sub_82EA07B0(ctx, base);
	// 82ED99A4: A17F0006  lhz r11, 6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 82ED99A8: 937F0000  stw r27, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82ED99AC: 556A0462  rlwinm r10, r11, 0, 0x11, 0x11
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED99B0: B37F0004  sth r27, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[27].u16 ) };
	// 82ED99B4: 61498000  ori r9, r10, 0x8000
	ctx.r[9].u64 = ctx.r[10].u64 | 32768;
	// 82ED99B8: B13F0006  sth r9, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82ED99BC: 83FC00CC  lwz r31, 0xcc(r28)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(204 as u32) ) } as u64;
	// 82ED99C0: 7F7DDB78  mr r29, r27
	ctx.r[29].u64 = ctx.r[27].u64;
	// 82ED99C4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82ED99C8: 419A0090  beq cr6, 0x82ed9a58
	if ctx.cr[6].eq {
	pc = 0x82ED9A58; continue 'dispatch;
	}
	// 82ED99CC: A15F0004  lhz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED99D0: 7F6BDB78  mr r11, r27
	ctx.r[11].u64 = ctx.r[27].u64;
	// 82ED99D4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82ED99D8: 419A0030  beq cr6, 0x82ed9a08
	if ctx.cr[6].eq {
	pc = 0x82ED9A08; continue 'dispatch;
	}
	// 82ED99DC: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED99E0: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED99E4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82ED99E8: 409A001C  bne cr6, 0x82ed9a04
	if !ctx.cr[6].eq {
	pc = 0x82ED9A04; continue 'dispatch;
	}
	// 82ED99EC: A13F0004  lhz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED99F0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82ED99F4: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82ED99F8: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82ED99FC: 4198FFE4  blt cr6, 0x82ed99e0
	if ctx.cr[6].lt {
	pc = 0x82ED99E0; continue 'dispatch;
	}
	// 82ED9A00: 48000008  b 0x82ed9a08
	pc = 0x82ED9A08; continue 'dispatch;
	// 82ED9A04: 7F5DD378  mr r29, r26
	ctx.r[29].u64 = ctx.r[26].u64;
	// 82ED9A08: 7FAB0774  extsb r11, r29
	ctx.r[11].s64 = ctx.r[29].s8 as i64;
	// 82ED9A0C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ED9A10: 409A0048  bne cr6, 0x82ed9a58
	if !ctx.cr[6].eq {
	pc = 0x82ED9A58; continue 'dispatch;
	}
	// 82ED9A14: A17F0006  lhz r11, 6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 82ED9A18: 556A0020  rlwinm r10, r11, 0, 0, 0x10
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED9A1C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82ED9A20: 409A0020  bne cr6, 0x82ed9a40
	if !ctx.cr[6].eq {
	pc = 0x82ED9A40; continue 'dispatch;
	}
	// 82ED9A24: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED9A28: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82ED9A2C: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82ED9A30: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED9A34: 5565143A  rlwinm r5, r11, 2, 0x10, 0x1d
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82ED9A38: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82ED9A3C: 4BFC6D75  bl 0x82ea07b0
	ctx.lr = 0x82ED9A40;
	sub_82EA07B0(ctx, base);
	// 82ED9A40: A17F0006  lhz r11, 6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 82ED9A44: 937F0000  stw r27, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82ED9A48: 556A0462  rlwinm r10, r11, 0, 0x11, 0x11
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED9A4C: B37F0004  sth r27, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[27].u16 ) };
	// 82ED9A50: 61498000  ori r9, r10, 0x8000
	ctx.r[9].u64 = ctx.r[10].u64 | 32768;
	// 82ED9A54: B13F0006  sth r9, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82ED9A58: 817C00CC  lwz r11, 0xcc(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(204 as u32) ) } as u64;
	// 82ED9A5C: 7F7EDB78  mr r30, r27
	ctx.r[30].u64 = ctx.r[27].u64;
	// 82ED9A60: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED9A64: 419A0094  beq cr6, 0x82ed9af8
	if ctx.cr[6].eq {
	pc = 0x82ED9AF8; continue 'dispatch;
	}
	// 82ED9A68: 3BEB0008  addi r31, r11, 8
	ctx.r[31].s64 = ctx.r[11].s64 + 8;
	// 82ED9A6C: 7F6BDB78  mr r11, r27
	ctx.r[11].u64 = ctx.r[27].u64;
	// 82ED9A70: A15F0004  lhz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED9A74: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82ED9A78: 419A0030  beq cr6, 0x82ed9aa8
	if ctx.cr[6].eq {
	pc = 0x82ED9AA8; continue 'dispatch;
	}
	// 82ED9A7C: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED9A80: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED9A84: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82ED9A88: 409A001C  bne cr6, 0x82ed9aa4
	if !ctx.cr[6].eq {
	pc = 0x82ED9AA4; continue 'dispatch;
	}
	// 82ED9A8C: A13F0004  lhz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED9A90: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82ED9A94: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82ED9A98: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82ED9A9C: 4198FFE4  blt cr6, 0x82ed9a80
	if ctx.cr[6].lt {
	pc = 0x82ED9A80; continue 'dispatch;
	}
	// 82ED9AA0: 48000008  b 0x82ed9aa8
	pc = 0x82ED9AA8; continue 'dispatch;
	// 82ED9AA4: 7F5ED378  mr r30, r26
	ctx.r[30].u64 = ctx.r[26].u64;
	// 82ED9AA8: 7FCB0774  extsb r11, r30
	ctx.r[11].s64 = ctx.r[30].s8 as i64;
	// 82ED9AAC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ED9AB0: 409A0048  bne cr6, 0x82ed9af8
	if !ctx.cr[6].eq {
	pc = 0x82ED9AF8; continue 'dispatch;
	}
	// 82ED9AB4: A17F0006  lhz r11, 6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 82ED9AB8: 556A0020  rlwinm r10, r11, 0, 0, 0x10
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED9ABC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82ED9AC0: 409A0020  bne cr6, 0x82ed9ae0
	if !ctx.cr[6].eq {
	pc = 0x82ED9AE0; continue 'dispatch;
	}
	// 82ED9AC4: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED9AC8: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82ED9ACC: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82ED9AD0: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED9AD4: 5565143A  rlwinm r5, r11, 2, 0x10, 0x1d
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82ED9AD8: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82ED9ADC: 4BFC6CD5  bl 0x82ea07b0
	ctx.lr = 0x82ED9AE0;
	sub_82EA07B0(ctx, base);
	// 82ED9AE0: A17F0006  lhz r11, 6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 82ED9AE4: 937F0000  stw r27, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82ED9AE8: 556A0462  rlwinm r10, r11, 0, 0x11, 0x11
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED9AEC: B37F0004  sth r27, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[27].u16 ) };
	// 82ED9AF0: 61498000  ori r9, r10, 0x8000
	ctx.r[9].u64 = ctx.r[10].u64 | 32768;
	// 82ED9AF4: B13F0006  sth r9, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82ED9AF8: 7FAB0774  extsb r11, r29
	ctx.r[11].s64 = ctx.r[29].s8 as i64;
	// 82ED9AFC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ED9B00: 409A00C4  bne cr6, 0x82ed9bc4
	if !ctx.cr[6].eq {
	pc = 0x82ED9BC4; continue 'dispatch;
	}
	// 82ED9B04: 7FCB0774  extsb r11, r30
	ctx.r[11].s64 = ctx.r[30].s8 as i64;
	// 82ED9B08: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ED9B0C: 409A00B8  bne cr6, 0x82ed9bc4
	if !ctx.cr[6].eq {
	pc = 0x82ED9BC4; continue 'dispatch;
	}
	// 82ED9B10: 83FC00CC  lwz r31, 0xcc(r28)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(204 as u32) ) } as u64;
	// 82ED9B14: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82ED9B18: 419A00A8  beq cr6, 0x82ed9bc0
	if ctx.cr[6].eq {
	pc = 0x82ED9BC0; continue 'dispatch;
	}
	// 82ED9B1C: A17F000E  lhz r11, 0xe(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(14 as u32) ) } as u64;
	// 82ED9B20: 556A0020  rlwinm r10, r11, 0, 0, 0x10
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED9B24: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82ED9B28: 409A0020  bne cr6, 0x82ed9b48
	if !ctx.cr[6].eq {
	pc = 0x82ED9B48; continue 'dispatch;
	}
	// 82ED9B2C: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED9B30: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82ED9B34: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82ED9B38: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED9B3C: 5565143A  rlwinm r5, r11, 2, 0x10, 0x1d
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82ED9B40: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82ED9B44: 4BFC6C6D  bl 0x82ea07b0
	ctx.lr = 0x82ED9B48;
	sub_82EA07B0(ctx, base);
	// 82ED9B48: A17F0006  lhz r11, 6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 82ED9B4C: 556A0020  rlwinm r10, r11, 0, 0, 0x10
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED9B50: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82ED9B54: 409A0020  bne cr6, 0x82ed9b74
	if !ctx.cr[6].eq {
	pc = 0x82ED9B74; continue 'dispatch;
	}
	// 82ED9B58: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED9B5C: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82ED9B60: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82ED9B64: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED9B68: 5565143A  rlwinm r5, r11, 2, 0x10, 0x1d
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82ED9B6C: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82ED9B70: 4BFC6C41  bl 0x82ea07b0
	ctx.lr = 0x82ED9B74;
	sub_82EA07B0(ctx, base);
	// 82ED9B74: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED9B78: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82ED9B7C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82ED9B80: 8123004C  lwz r9, 0x4c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(76 as u32) ) } as u64;
	// 82ED9B84: 81030034  lwz r8, 0x34(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 82ED9B88: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82ED9B8C: 41980018  blt cr6, 0x82ed9ba4
	if ctx.cr[6].lt {
	pc = 0x82ED9BA4; continue 'dispatch;
	}
	// 82ED9B90: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 82ED9B94: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82ED9B98: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 82ED9B9C: 4BFC64C5  bl 0x82ea0060
	ctx.lr = 0x82ED9BA0;
	sub_82EA0060(ctx, base);
	// 82ED9BA0: 48000020  b 0x82ed9bc0
	pc = 0x82ED9BC0; continue 'dispatch;
	// 82ED9BA4: 8143004C  lwz r10, 0x4c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(76 as u32) ) } as u64;
	// 82ED9BA8: 39630048  addi r11, r3, 0x48
	ctx.r[11].s64 = ctx.r[3].s64 + 72;
	// 82ED9BAC: 81630048  lwz r11, 0x48(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 82ED9BB0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82ED9BB4: 9143004C  stw r10, 0x4c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(76 as u32), ctx.r[10].u32 ) };
	// 82ED9BB8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82ED9BBC: 93E30048  stw r31, 0x48(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), ctx.r[31].u32 ) };
	// 82ED9BC0: 937C00CC  stw r27, 0xcc(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(204 as u32), ctx.r[27].u32 ) };
	// 82ED9BC4: A17C01FC  lhz r11, 0x1fc(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[28].u32.wrapping_add(508 as u32) ) } as u64;
	// 82ED9BC8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED9BCC: 409A004C  bne cr6, 0x82ed9c18
	if !ctx.cr[6].eq {
	pc = 0x82ED9C18; continue 'dispatch;
	}
	// 82ED9BD0: A17C01FE  lhz r11, 0x1fe(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[28].u32.wrapping_add(510 as u32) ) } as u64;
	// 82ED9BD4: 3BFC01F8  addi r31, r28, 0x1f8
	ctx.r[31].s64 = ctx.r[28].s64 + 504;
	// 82ED9BD8: 556A0020  rlwinm r10, r11, 0, 0, 0x10
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED9BDC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82ED9BE0: 409A0020  bne cr6, 0x82ed9c00
	if !ctx.cr[6].eq {
	pc = 0x82ED9C00; continue 'dispatch;
	}
	// 82ED9BE4: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED9BE8: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82ED9BEC: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82ED9BF0: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED9BF4: 5565143A  rlwinm r5, r11, 2, 0x10, 0x1d
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82ED9BF8: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82ED9BFC: 4BFC6BB5  bl 0x82ea07b0
	ctx.lr = 0x82ED9C00;
	sub_82EA07B0(ctx, base);
	// 82ED9C00: A17F0006  lhz r11, 6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 82ED9C04: 937F0000  stw r27, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82ED9C08: 556A0462  rlwinm r10, r11, 0, 0x11, 0x11
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED9C0C: B37F0004  sth r27, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[27].u16 ) };
	// 82ED9C10: 61498000  ori r9, r10, 0x8000
	ctx.r[9].u64 = ctx.r[10].u64 | 32768;
	// 82ED9C14: B13F0006  sth r9, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82ED9C18: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82ED9C1C: 482CE594  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED9C20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED9C20 size=324
    let mut pc: u32 = 0x82ED9C20;
    'dispatch: loop {
        match pc {
            0x82ED9C20 => {
    //   block [0x82ED9C20..0x82ED9D64)
	// 82ED9C20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED9C24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ED9C28: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ED9C2C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED9C30: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ED9C34: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82ED9C38: 394BB0A0  addi r10, r11, -0x4f60
	ctx.r[10].s64 = ctx.r[11].s64 + -20320;
	// 82ED9C3C: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82ED9C40: 48009769  bl 0x82ee33a8
	ctx.lr = 0x82ED9C44;
	sub_82EE33A8(ctx, base);
	// 82ED9C44: 807F00CC  lwz r3, 0xcc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 82ED9C48: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ED9C4C: 419A000C  beq cr6, 0x82ed9c58
	if ctx.cr[6].eq {
	pc = 0x82ED9C58; continue 'dispatch;
	}
	// 82ED9C50: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82ED9C54: 4BFFDCCD  bl 0x82ed7920
	ctx.lr = 0x82ED9C58;
	sub_82ED7920(ctx, base);
	// 82ED9C58: A17F01FE  lhz r11, 0x1fe(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(510 as u32) ) } as u64;
	// 82ED9C5C: 556A0020  rlwinm r10, r11, 0, 0, 0x10
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED9C60: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82ED9C64: 409A0020  bne cr6, 0x82ed9c84
	if !ctx.cr[6].eq {
	pc = 0x82ED9C84; continue 'dispatch;
	}
	// 82ED9C68: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED9C6C: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82ED9C70: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82ED9C74: 809F01F8  lwz r4, 0x1f8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(504 as u32) ) } as u64;
	// 82ED9C78: 5565143A  rlwinm r5, r11, 2, 0x10, 0x1d
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82ED9C7C: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82ED9C80: 4BFC6B31  bl 0x82ea07b0
	ctx.lr = 0x82ED9C84;
	sub_82EA07B0(ctx, base);
	// 82ED9C84: A17F01F6  lhz r11, 0x1f6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(502 as u32) ) } as u64;
	// 82ED9C88: 556A0020  rlwinm r10, r11, 0, 0, 0x10
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED9C8C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82ED9C90: 409A0020  bne cr6, 0x82ed9cb0
	if !ctx.cr[6].eq {
	pc = 0x82ED9CB0; continue 'dispatch;
	}
	// 82ED9C94: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED9C98: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82ED9C9C: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82ED9CA0: 809F01F0  lwz r4, 0x1f0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(496 as u32) ) } as u64;
	// 82ED9CA4: 5565143A  rlwinm r5, r11, 2, 0x10, 0x1d
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82ED9CA8: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82ED9CAC: 4BFC6B05  bl 0x82ea07b0
	ctx.lr = 0x82ED9CB0;
	sub_82EA07B0(ctx, base);
	// 82ED9CB0: 387F00D0  addi r3, r31, 0xd0
	ctx.r[3].s64 = ctx.r[31].s64 + 208;
	// 82ED9CB4: 480046D5  bl 0x82ede388
	ctx.lr = 0x82ED9CB8;
	sub_82EDE388(ctx, base);
	// 82ED9CB8: 817F00B4  lwz r11, 0xb4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 82ED9CBC: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED9CC0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ED9CC4: 409A0020  bne cr6, 0x82ed9ce4
	if !ctx.cr[6].eq {
	pc = 0x82ED9CE4; continue 'dispatch;
	}
	// 82ED9CC8: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED9CCC: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82ED9CD0: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82ED9CD4: 809F00AC  lwz r4, 0xac(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) } as u64;
	// 82ED9CD8: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82ED9CDC: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82ED9CE0: 4BFC6AD1  bl 0x82ea07b0
	ctx.lr = 0x82ED9CE4;
	sub_82EA07B0(ctx, base);
	// 82ED9CE4: 817F00A8  lwz r11, 0xa8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(168 as u32) ) } as u64;
	// 82ED9CE8: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED9CEC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ED9CF0: 409A0020  bne cr6, 0x82ed9d10
	if !ctx.cr[6].eq {
	pc = 0x82ED9D10; continue 'dispatch;
	}
	// 82ED9CF4: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED9CF8: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82ED9CFC: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82ED9D00: 809F00A0  lwz r4, 0xa0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(160 as u32) ) } as u64;
	// 82ED9D04: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82ED9D08: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82ED9D0C: 4BFC6AA5  bl 0x82ea07b0
	ctx.lr = 0x82ED9D10;
	sub_82EA07B0(ctx, base);
	// 82ED9D10: A17F009E  lhz r11, 0x9e(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(158 as u32) ) } as u64;
	// 82ED9D14: 556A0020  rlwinm r10, r11, 0, 0, 0x10
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ED9D18: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82ED9D1C: 409A002C  bne cr6, 0x82ed9d48
	if !ctx.cr[6].eq {
	pc = 0x82ED9D48; continue 'dispatch;
	}
	// 82ED9D20: 556A04BE  clrlwi r10, r11, 0x12
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00003FFFu64;
	// 82ED9D24: 812D0000  lwz r9, 0(r13)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED9D28: 556B0C7C  rlwinm r11, r11, 1, 0x11, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x7FFFFFFFu64;
	// 82ED9D2C: 809F0098  lwz r4, 0x98(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 82ED9D30: 39000014  li r8, 0x14
	ctx.r[8].s64 = 20;
	// 82ED9D34: 7CEA5A14  add r7, r10, r11
	ctx.r[7].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82ED9D38: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82ED9D3C: 54E52036  slwi r5, r7, 4
	ctx.r[5].u32 = ctx.r[7].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82ED9D40: 7C68482E  lwzx r3, r8, r9
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82ED9D44: 4BFC6A6D  bl 0x82ea07b0
	ctx.lr = 0x82ED9D48;
	sub_82EA07B0(ctx, base);
	// 82ED9D48: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED9D4C: 4BFFB1BD  bl 0x82ed4f08
	ctx.lr = 0x82ED9D50;
	sub_82ED4F08(ctx, base);
	// 82ED9D50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82ED9D54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ED9D58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ED9D5C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ED9D60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED9D68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED9D68 size=100
    let mut pc: u32 = 0x82ED9D68;
    'dispatch: loop {
        match pc {
            0x82ED9D68 => {
    //   block [0x82ED9D68..0x82ED9DCC)
	// 82ED9D68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED9D6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ED9D70: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82ED9D74: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ED9D78: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED9D7C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ED9D80: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82ED9D84: 4BFFFE9D  bl 0x82ed9c20
	ctx.lr = 0x82ED9D88;
	sub_82ED9C20(ctx, base);
	// 82ED9D88: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82ED9D8C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED9D90: 419A0020  beq cr6, 0x82ed9db0
	if ctx.cr[6].eq {
	pc = 0x82ED9DB0; continue 'dispatch;
	}
	// 82ED9D94: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED9D98: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82ED9D9C: 38C0002D  li r6, 0x2d
	ctx.r[6].s64 = 45;
	// 82ED9DA0: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED9DA4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82ED9DA8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82ED9DAC: 4BFC6A05  bl 0x82ea07b0
	ctx.lr = 0x82ED9DB0;
	sub_82EA07B0(ctx, base);
	// 82ED9DB0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED9DB4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82ED9DB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ED9DBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ED9DC0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82ED9DC4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ED9DC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED9DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED9DD0 size=72
    let mut pc: u32 = 0x82ED9DD0;
    'dispatch: loop {
        match pc {
            0x82ED9DD0 => {
    //   block [0x82ED9DD0..0x82ED9E18)
	// 82ED9DD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED9DD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ED9DD8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ED9DDC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED9DE0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ED9DE4: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82ED9DE8: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 82ED9DEC: 392BB0BC  addi r9, r11, -0x4f44
	ctx.r[9].s64 = ctx.r[11].s64 + -20292;
	// 82ED9DF0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82ED9DF4: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82ED9DF8: 419A000C  beq cr6, 0x82ed9e04
	if ctx.cr[6].eq {
	pc = 0x82ED9E04; continue 'dispatch;
	}
	// 82ED9DFC: 4B3E646D  bl 0x822c0268
	ctx.lr = 0x82ED9E00;
	sub_822C0268(ctx, base);
	// 82ED9E00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED9E04: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82ED9E08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ED9E0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ED9E10: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ED9E14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED9E18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED9E18 size=140
    let mut pc: u32 = 0x82ED9E18;
    'dispatch: loop {
        match pc {
            0x82ED9E18 => {
    //   block [0x82ED9E18..0x82ED9EA4)
	// 82ED9E18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED9E1C: 482CE34D  bl 0x831a8168
	ctx.lr = 0x82ED9E20;
	sub_831A8130(ctx, base);
	// 82ED9E20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED9E24: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82ED9E28: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82ED9E2C: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82ED9E30: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82ED9E34: 390BB0E4  addi r8, r11, -0x4f1c
	ctx.r[8].s64 = ctx.r[11].s64 + -20252;
	// 82ED9E38: 38EAB0D8  addi r7, r10, -0x4f28
	ctx.r[7].s64 = ctx.r[10].s64 + -20264;
	// 82ED9E3C: 38C9B0C8  addi r6, r9, -0x4f38
	ctx.r[6].s64 = ctx.r[9].s64 + -20280;
	// 82ED9E40: 911D0000  stw r8, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82ED9E44: 90FD0008  stw r7, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 82ED9E48: 3BFD0014  addi r31, r29, 0x14
	ctx.r[31].s64 = ctx.r[29].s64 + 20;
	// 82ED9E4C: 90DD000C  stw r6, 0xc(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82ED9E50: 3BC00006  li r30, 6
	ctx.r[30].s64 = 6;
	// 82ED9E54: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82ED9E58: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED9E5C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ED9E60: 419A000C  beq cr6, 0x82ed9e6c
	if ctx.cr[6].eq {
	pc = 0x82ED9E6C; continue 'dispatch;
	}
	// 82ED9E64: 4BFFB2A5  bl 0x82ed5108
	ctx.lr = 0x82ED9E68;
	sub_82ED5108(ctx, base);
	// 82ED9E68: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82ED9E6C: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82ED9E70: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82ED9E74: 4082FFE4  bne 0x82ed9e58
	if !ctx.cr[0].eq {
	pc = 0x82ED9E58; continue 'dispatch;
	}
	// 82ED9E78: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82ED9E7C: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82ED9E80: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82ED9E84: 390BAC74  addi r8, r11, -0x538c
	ctx.r[8].s64 = ctx.r[11].s64 + -21388;
	// 82ED9E88: 38EAB0BC  addi r7, r10, -0x4f44
	ctx.r[7].s64 = ctx.r[10].s64 + -20292;
	// 82ED9E8C: 38C99EAC  addi r6, r9, -0x6154
	ctx.r[6].s64 = ctx.r[9].s64 + -24916;
	// 82ED9E90: 911D000C  stw r8, 0xc(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82ED9E94: 90FD0008  stw r7, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 82ED9E98: 90DD0000  stw r6, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82ED9E9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82ED9EA0: 482CE318  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED9EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED9EA8 size=4
    let mut pc: u32 = 0x82ED9EA8;
    'dispatch: loop {
        match pc {
            0x82ED9EA8 => {
    //   block [0x82ED9EA8..0x82ED9EAC)
	// 82ED9EA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED9EB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED9EB0 size=108
    let mut pc: u32 = 0x82ED9EB0;
    'dispatch: loop {
        match pc {
            0x82ED9EB0 => {
    //   block [0x82ED9EB0..0x82ED9F1C)
	// 82ED9EB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED9EB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ED9EB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED9EBC: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82ED9EC0: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82ED9EC4: 816A002C  lwz r11, 0x2c(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(44 as u32) ) } as u64;
	// 82ED9EC8: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82ED9ECC: 41980030  blt cr6, 0x82ed9efc
	if ctx.cr[6].lt {
	pc = 0x82ED9EFC; continue 'dispatch;
	}
	// 82ED9ED0: 419A002C  beq cr6, 0x82ed9efc
	if ctx.cr[6].eq {
	pc = 0x82ED9EFC; continue 'dispatch;
	}
	// 82ED9ED4: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 82ED9ED8: 40980034  bge cr6, 0x82ed9f0c
	if !ctx.cr[6].lt {
	pc = 0x82ED9F0C; continue 'dispatch;
	}
	// 82ED9EDC: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82ED9EE0: 808A0010  lwz r4, 0x10(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82ED9EE4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82ED9EE8: 4BFF3559  bl 0x82ecd440
	ctx.lr = 0x82ED9EEC;
	sub_82ECD440(ctx, base);
	// 82ED9EEC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82ED9EF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ED9EF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ED9EF8: 4E800020  blr
	return;
	// 82ED9EFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82ED9F00: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82ED9F04: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 82ED9F08: 4BFF1C89  bl 0x82ecbb90
	ctx.lr = 0x82ED9F0C;
	sub_82ECBB90(ctx, base);
	// 82ED9F0C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82ED9F10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ED9F14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ED9F18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED9F20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED9F20 size=124
    let mut pc: u32 = 0x82ED9F20;
    'dispatch: loop {
        match pc {
            0x82ED9F20 => {
    //   block [0x82ED9F20..0x82ED9F9C)
	// 82ED9F20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED9F24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ED9F28: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ED9F2C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED9F30: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ED9F34: 357FFFF8  addic. r11, r31, -8
	ctx.xer.ca = (ctx.r[31].u32 > (!(-8 as u32)));
	ctx.r[11].s64 = ctx.r[31].s64 + -8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ED9F38: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82ED9F3C: 40820008  bne 0x82ed9f44
	if !ctx.cr[0].eq {
	pc = 0x82ED9F44; continue 'dispatch;
	}
	// 82ED9F40: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82ED9F44: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED9F48: 4BFF3A71  bl 0x82ecd9b8
	ctx.lr = 0x82ED9F4C;
	sub_82ECD9B8(ctx, base);
	// 82ED9F4C: A17FFFFC  lhz r11, -4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82ED9F50: 387FFFF8  addi r3, r31, -8
	ctx.r[3].s64 = ctx.r[31].s64 + -8;
	// 82ED9F54: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED9F58: 419A0030  beq cr6, 0x82ed9f88
	if ctx.cr[6].eq {
	pc = 0x82ED9F88; continue 'dispatch;
	}
	// 82ED9F5C: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82ED9F60: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 82ED9F64: 7D490734  extsh r9, r10
	ctx.r[9].s64 = ctx.r[10].s16 as i64;
	// 82ED9F68: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82ED9F6C: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82ED9F70: 409A0018  bne cr6, 0x82ed9f88
	if !ctx.cr[6].eq {
	pc = 0x82ED9F88; continue 'dispatch;
	}
	// 82ED9F74: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED9F78: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82ED9F7C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED9F80: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED9F84: 4E800421  bctrl
	ctx.lr = 0x82ED9F88;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED9F88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82ED9F8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ED9F90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ED9F94: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ED9F98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED9FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED9FA0 size=128
    let mut pc: u32 = 0x82ED9FA0;
    'dispatch: loop {
        match pc {
            0x82ED9FA0 => {
    //   block [0x82ED9FA0..0x82EDA020)
	// 82ED9FA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED9FA4: 482CE1C1  bl 0x831a8164
	ctx.lr = 0x82ED9FA8;
	sub_831A8130(ctx, base);
	// 82ED9FA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED9FAC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82ED9FB0: 807D0010  lwz r3, 0x10(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82ED9FB4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ED9FB8: 419A0060  beq cr6, 0x82eda018
	if ctx.cr[6].eq {
	pc = 0x82EDA018; continue 'dispatch;
	}
	// 82ED9FBC: 3BFD0014  addi r31, r29, 0x14
	ctx.r[31].s64 = ctx.r[29].s64 + 20;
	// 82ED9FC0: 38A00006  li r5, 6
	ctx.r[5].s64 = 6;
	// 82ED9FC4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82ED9FC8: 4BFFA059  bl 0x82ed4020
	ctx.lr = 0x82ED9FCC;
	sub_82ED4020(ctx, base);
	// 82ED9FCC: 3B9D000C  addi r28, r29, 0xc
	ctx.r[28].s64 = ctx.r[29].s64 + 12;
	// 82ED9FD0: 3BC00006  li r30, 6
	ctx.r[30].s64 = 6;
	// 82ED9FD4: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82ED9FD8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82ED9FDC: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED9FE0: 4BFFB9F1  bl 0x82ed59d0
	ctx.lr = 0x82ED9FE4;
	sub_82ED59D0(ctx, base);
	// 82ED9FE4: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED9FE8: 4BFFB121  bl 0x82ed5108
	ctx.lr = 0x82ED9FEC;
	sub_82ED5108(ctx, base);
	// 82ED9FEC: 937F0000  stw r27, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82ED9FF0: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82ED9FF4: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82ED9FF8: 4082FFE0  bne 0x82ed9fd8
	if !ctx.cr[0].eq {
	pc = 0x82ED9FD8; continue 'dispatch;
	}
	// 82ED9FFC: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EDA000: 387D0008  addi r3, r29, 8
	ctx.r[3].s64 = ctx.r[29].s64 + 8;
	// 82EDA004: 809D0010  lwz r4, 0x10(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EDA008: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EDA00C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EDA010: 4E800421  bctrl
	ctx.lr = 0x82EDA014;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EDA014: 937D0010  stw r27, 0x10(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(16 as u32), ctx.r[27].u32 ) };
	// 82EDA018: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EDA01C: 482CE198  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EDA020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82EDA020 size=1028
    let mut pc: u32 = 0x82EDA020;
    'dispatch: loop {
        match pc {
            0x82EDA020 => {
    //   block [0x82EDA020..0x82EDA424)
	// 82EDA020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EDA024: 482CE131  bl 0x831a8154
	ctx.lr = 0x82EDA028;
	sub_831A8130(ctx, base);
	// 82EDA028: 3981FFB0  addi r12, r1, -0x50
	ctx.r[12].s64 = ctx.r[1].s64 + -80;
	// 82EDA02C: 482CEA41  bl 0x831a8a6c
	ctx.lr = 0x82EDA030;
	sub_831A8A40(ctx, base);
	// 82EDA030: 9421FEC0  stwu r1, -0x140(r1)
	ea = ctx.r[1].u32.wrapping_add(-320 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EDA034: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EDA038: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82EDA03C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82EDA040: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82EDA044: 390BB0BC  addi r8, r11, -0x4f44
	ctx.r[8].s64 = ctx.r[11].s64 + -20292;
	// 82EDA048: 3CE08213  lis r7, -0x7ded
	ctx.r[7].s64 = -2112684032;
	// 82EDA04C: B13F0006  sth r9, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82EDA050: 3CC08213  lis r6, -0x7ded
	ctx.r[6].s64 = -2112684032;
	// 82EDA054: 911F0008  stw r8, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 82EDA058: 3C608213  lis r3, -0x7ded
	ctx.r[3].s64 = -2112684032;
	// 82EDA05C: 396AAC74  addi r11, r10, -0x538c
	ctx.r[11].s64 = ctx.r[10].s64 + -21388;
	// 82EDA060: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EDA064: 3947B0E4  addi r10, r7, -0x4f1c
	ctx.r[10].s64 = ctx.r[7].s64 + -20252;
	// 82EDA068: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82EDA06C: 3926B0D8  addi r9, r6, -0x4f28
	ctx.r[9].s64 = ctx.r[6].s64 + -20264;
	// 82EDA070: A0FF0004  lhz r7, 4(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EDA074: 3903B0C8  addi r8, r3, -0x4f38
	ctx.r[8].s64 = ctx.r[3].s64 + -20280;
	// 82EDA078: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EDA07C: 389F0008  addi r4, r31, 8
	ctx.r[4].s64 = ctx.r[31].s64 + 8;
	// 82EDA080: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82EDA084: 3BBF000C  addi r29, r31, 0xc
	ctx.r[29].s64 = ctx.r[31].s64 + 12;
	// 82EDA088: 911F000C  stw r8, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82EDA08C: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82EDA090: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 82EDA094: 90BF002C  stw r5, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[5].u32 ) };
	// 82EDA098: 419A0010  beq cr6, 0x82eda0a8
	if ctx.cr[6].eq {
	pc = 0x82EDA0A8; continue 'dispatch;
	}
	// 82EDA09C: A17F0006  lhz r11, 6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 82EDA0A0: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 82EDA0A4: B15F0006  sth r10, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82EDA0A8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EDA0AC: 4BFF4F8D  bl 0x82ecf038
	ctx.lr = 0x82EDA0B0;
	sub_82ECF038(ctx, base);
	// 82EDA0B0: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EDA0B4: 38E10070  addi r7, r1, 0x70
	ctx.r[7].s64 = ctx.r[1].s64 + 112;
	// 82EDA0B8: 838D0000  lwz r28, 0(r13)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EDA0BC: 386B02D0  addi r3, r11, 0x2d0
	ctx.r[3].s64 = ctx.r[11].s64 + 720;
	// 82EDA0C0: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82EDA0C4: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 82EDA0C8: 39410080  addi r10, r1, 0x80
	ctx.r[10].s64 = ctx.r[1].s64 + 128;
	// 82EDA0CC: 39010050  addi r8, r1, 0x50
	ctx.r[8].s64 = ctx.r[1].s64 + 80;
	// 82EDA0D0: 3B410090  addi r26, r1, 0x90
	ctx.r[26].s64 = ctx.r[1].s64 + 144;
	// 82EDA0D4: C3E908A4  lfs f31, 0x8a4(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82EDA0D8: 392100A0  addi r9, r1, 0xa0
	ctx.r[9].s64 = ctx.r[1].s64 + 160;
	// 82EDA0DC: D3E1006C  stfs f31, 0x6c(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 82EDA0E0: 3B600014  li r27, 0x14
	ctx.r[27].s64 = 20;
	// 82EDA0E4: D3E1005C  stfs f31, 0x5c(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 82EDA0E8: 38A00031  li r5, 0x31
	ctx.r[5].s64 = 49;
	// 82EDA0EC: 388000D0  li r4, 0xd0
	ctx.r[4].s64 = 208;
	// 82EDA0F0: EB2B02D0  ld r25, 0x2d0(r11)
	ctx.r[25].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(720 as u32) ) };
	// 82EDA0F4: EB0B02D8  ld r24, 0x2d8(r11)
	ctx.r[24].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(728 as u32) ) };
	// 82EDA0F8: EAEB02E0  ld r23, 0x2e0(r11)
	ctx.r[23].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(736 as u32) ) };
	// 82EDA0FC: E96B02E8  ld r11, 0x2e8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(744 as u32) ) };
	// 82EDA100: 7C7BE02E  lwzx r3, r27, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82EDA104: FB2A0000  std r25, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[25].u64 ) };
	// 82EDA108: FB0A0008  std r24, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[24].u64 ) };
	// 82EDA10C: C3A10088  lfs f29, 0x88(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 82EDA110: FAE70000  std r23, 0(r7)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[23].u64 ) };
	// 82EDA114: C3C10084  lfs f30, 0x84(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 82EDA118: F9670008  std r11, 8(r7)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[7].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82EDA11C: C3810070  lfs f28, 0x70(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) };
	ctx.f[28].f64 = (tmp.f32 as f64);
	// 82EDA120: C3610074  lfs f27, 0x74(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) };
	ctx.f[27].f64 = (tmp.f32 as f64);
	// 82EDA124: C3410078  lfs f26, 0x78(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) };
	ctx.f[26].f64 = (tmp.f32 as f64);
	// 82EDA128: D3C10054  stfs f30, 0x54(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82EDA12C: D3810060  stfs f28, 0x60(r1)
	tmp.f32 = (ctx.f[28].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 82EDA130: D3610064  stfs f27, 0x64(r1)
	tmp.f32 = (ctx.f[27].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 82EDA134: D3410068  stfs f26, 0x68(r1)
	tmp.f32 = (ctx.f[26].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EDA428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EDA428 size=16
    let mut pc: u32 = 0x82EDA428;
    'dispatch: loop {
        match pc {
            0x82EDA428 => {
    //   block [0x82EDA428..0x82EDA438)
	// 82EDA428: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EDA42C: 894B0018  lbz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EDA430: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82EDA434: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EDA438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EDA438 size=20
    let mut pc: u32 = 0x82EDA438;
    'dispatch: loop {
        match pc {
            0x82EDA438 => {
    //   block [0x82EDA438..0x82EDA44C)
	// 82EDA438: 894B0010  lbz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EDA43C: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 82EDA440: 7C8A5A14  add r4, r10, r11
	ctx.r[4].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82EDA444: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82EDA448: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EDA44C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EDA44C size=20
    let mut pc: u32 = 0x82EDA44C;
    'dispatch: loop {
        match pc {
            0x82EDA44C => {
    //   block [0x82EDA44C..0x82EDA460)
	// 82EDA44C: 8163FFF4  lwz r11, -0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-12 as u32) ) } as u64;
	// 82EDA450: 3863FFF4  addi r3, r3, -0xc
	ctx.r[3].s64 = ctx.r[3].s64 + -12;
	// 82EDA454: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EDA458: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EDA45C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EDA460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EDA460 size=4
    let mut pc: u32 = 0x82EDA460;
    'dispatch: loop {
        match pc {
            0x82EDA460 => {
    //   block [0x82EDA460..0x82EDA464)
	// 82EDA460: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EDA468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EDA468 size=8
    let mut pc: u32 = 0x82EDA468;
    'dispatch: loop {
        match pc {
            0x82EDA468 => {
    //   block [0x82EDA468..0x82EDA470)
	// 82EDA468: 3863FFF8  addi r3, r3, -8
	ctx.r[3].s64 = ctx.r[3].s64 + -8;
	// 82EDA46C: 4800000C  b 0x82eda478
	sub_82EDA478(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EDA470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EDA470 size=8
    let mut pc: u32 = 0x82EDA470;
    'dispatch: loop {
        match pc {
            0x82EDA470 => {
    //   block [0x82EDA470..0x82EDA478)
	// 82EDA470: 3863FFF4  addi r3, r3, -0xc
	ctx.r[3].s64 = ctx.r[3].s64 + -12;
	// 82EDA474: 48000004  b 0x82eda478
	sub_82EDA478(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EDA478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EDA478 size=100
    let mut pc: u32 = 0x82EDA478;
    'dispatch: loop {
        match pc {
            0x82EDA478 => {
    //   block [0x82EDA478..0x82EDA4DC)
	// 82EDA478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EDA47C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EDA480: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EDA484: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EDA488: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EDA48C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EDA490: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EDA494: 4BFFF985  bl 0x82ed9e18
	ctx.lr = 0x82EDA498;
	sub_82ED9E18(ctx, base);
	// 82EDA498: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82EDA49C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EDA4A0: 419A0020  beq cr6, 0x82eda4c0
	if ctx.cr[6].eq {
	pc = 0x82EDA4C0; continue 'dispatch;
	}
	// 82EDA4A4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EDA4A8: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82EDA4AC: 38C0000D  li r6, 0xd
	ctx.r[6].s64 = 13;
	// 82EDA4B0: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EDA4B4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EDA4B8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EDA4BC: 4BFC62F5  bl 0x82ea07b0
	ctx.lr = 0x82EDA4C0;
	sub_82EA07B0(ctx, base);
	// 82EDA4C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EDA4C4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EDA4C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EDA4CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EDA4D0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EDA4D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EDA4D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EDA4E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82EDA4E0 size=24
    let mut pc: u32 = 0x82EDA4E0;
    'dispatch: loop {
        match pc {
            0x82EDA4E0 => {
    //   block [0x82EDA4E0..0x82EDA4F8)
	// 82EDA4E0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82EDA4E4: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EDA4E8: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 82EDA4EC: 409A000C  bne cr6, 0x82eda4f8
	if !ctx.cr[6].eq {
		sub_82EDA4F8(ctx, base);
		return;
	}
	// 82EDA4F0: FC200090  fmr f1, f0
	ctx.f[1].f64 = ctx.f[0].f64;
	// 82EDA4F4: 48000010  b 0x82eda504
	sub_82EDA4F8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EDA4F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82EDA4F8 size=28
    let mut pc: u32 = 0x82EDA4F8;
    'dispatch: loop {
        match pc {
            0x82EDA4F8 => {
    //   block [0x82EDA4F8..0x82EDA514)
	// 82EDA4F8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82EDA4FC: C00B08A8  lfs f0, 0x8a8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EDA500: EC200824  fdivs f1, f0, f1
	ctx.f[1].f64 = ((ctx.f[0].f64 / ctx.f[1].f64) as f32) as f64;
	// 82EDA504: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EDA508: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EDA50C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EDA510: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EDA518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EDA518 size=16
    let mut pc: u32 = 0x82EDA518;
    'dispatch: loop {
        match pc {
            0x82EDA518 => {
    //   block [0x82EDA518..0x82EDA528)
	// 82EDA518: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82EDA51C: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82EDA520: 388B0010  addi r4, r11, 0x10
	ctx.r[4].s64 = ctx.r[11].s64 + 16;
	// 82EDA524: 48323FE4  b 0x831fe508
	sub_831FE508(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EDA528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EDA528 size=16
    let mut pc: u32 = 0x82EDA528;
    'dispatch: loop {
        match pc {
            0x82EDA528 => {
    //   block [0x82EDA528..0x82EDA538)
	// 82EDA528: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82EDA52C: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82EDA530: 388B0010  addi r4, r11, 0x10
	ctx.r[4].s64 = ctx.r[11].s64 + 16;
	// 82EDA534: 48324B0C  b 0x831ff040
	sub_831FF040(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EDA538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EDA538 size=16
    let mut pc: u32 = 0x82EDA538;
    'dispatch: loop {
        match pc {
            0x82EDA538 => {
    //   block [0x82EDA538..0x82EDA548)
	// 82EDA538: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82EDA53C: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82EDA540: 388B0010  addi r4, r11, 0x10
	ctx.r[4].s64 = ctx.r[11].s64 + 16;
	// 82EDA544: 48324B84  b 0x831ff0c8
	sub_831FF0C8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EDA548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EDA548 size=20
    let mut pc: u32 = 0x82EDA548;
    'dispatch: loop {
        match pc {
            0x82EDA548 => {
    //   block [0x82EDA548..0x82EDA55C)
	// 82EDA548: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82EDA54C: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82EDA550: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82EDA554: 38AB0010  addi r5, r11, 0x10
	ctx.r[5].s64 = ctx.r[11].s64 + 16;
	// 82EDA558: 48324960  b 0x831feeb8
	sub_831FEEB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EDA560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EDA560 size=16
    let mut pc: u32 = 0x82EDA560;
    'dispatch: loop {
        match pc {
            0x82EDA560 => {
    //   block [0x82EDA560..0x82EDA570)
	// 82EDA560: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82EDA564: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82EDA568: 388B0010  addi r4, r11, 0x10
	ctx.r[4].s64 = ctx.r[11].s64 + 16;
	// 82EDA56C: 483249FC  b 0x831fef68
	sub_831FEF68(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EDA570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EDA570 size=8
    let mut pc: u32 = 0x82EDA570;
    'dispatch: loop {
        match pc {
            0x82EDA570 => {
    //   block [0x82EDA570..0x82EDA578)
	// 82EDA570: 988300BE  stb r4, 0xbe(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(190 as u32), ctx.r[4].u8 ) };
	// 82EDA574: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EDA578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EDA578 size=40
    let mut pc: u32 = 0x82EDA578;
    'dispatch: loop {
        match pc {
            0x82EDA578 => {
    //   block [0x82EDA578..0x82EDA5A0)
	// 82EDA578: 396000C0  li r11, 0xc0
	ctx.r[11].s64 = 192;
	// 82EDA57C: 3941FFF0  addi r10, r1, -0x10
	ctx.r[10].s64 = ctx.r[1].s64 + -16;
	// 82EDA580: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EDA5A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82EDA5A0 size=16
    let mut pc: u32 = 0x82EDA5A0;
    'dispatch: loop {
        match pc {
            0x82EDA5A0 => {
    //   block [0x82EDA5A0..0x82EDA5B0)
	// 82EDA5A0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82EDA5A4: C00B08A8  lfs f0, 0x8a8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EDA5A8: EC206824  fdivs f1, f0, f13
	ctx.f[1].f64 = ((ctx.f[0].f64 / ctx.f[13].f64) as f32) as f64;
	// 82EDA5AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EDA5B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82EDA5B0 size=8
    let mut pc: u32 = 0x82EDA5B0;
    'dispatch: loop {
        match pc {
            0x82EDA5B0 => {
    //   block [0x82EDA5B0..0x82EDA5B8)
	// 82EDA5B0: D02300CC  stfs f1, 0xcc(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(204 as u32), tmp.u32 ) };
	// 82EDA5B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EDA5B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EDA5B8 size=8
    let mut pc: u32 = 0x82EDA5B8;
    'dispatch: loop {
        match pc {
            0x82EDA5B8 => {
    //   block [0x82EDA5B8..0x82EDA5C0)
	// 82EDA5B8: 38630050  addi r3, r3, 0x50
	ctx.r[3].s64 = ctx.r[3].s64 + 80;
	// 82EDA5BC: 48325404  b 0x831ff9c0
	sub_831FF9C0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EDA5C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EDA5C0 size=16
    let mut pc: u32 = 0x82EDA5C0;
    'dispatch: loop {
        match pc {
            0x82EDA5C0 => {
    //   block [0x82EDA5C0..0x82EDA5D0)
	// 82EDA5C0: 396000D0  li r11, 0xd0
	ctx.r[11].s64 = 208;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EDA5D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EDA5D0 size=16
    let mut pc: u32 = 0x82EDA5D0;
    'dispatch: loop {
        match pc {
            0x82EDA5D0 => {
    //   block [0x82EDA5D0..0x82EDA5E0)
	// 82EDA5D0: 396000E0  li r11, 0xe0
	ctx.r[11].s64 = 224;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EDA5E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EDA5E0 size=36
    let mut pc: u32 = 0x82EDA5E0;
    'dispatch: loop {
        match pc {
            0x82EDA5E0 => {
    //   block [0x82EDA5E0..0x82EDA604)
	// 82EDA5E0: 394000C0  li r10, 0xc0
	ctx.r[10].s64 = 192;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EDA608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EDA608 size=216
    let mut pc: u32 = 0x82EDA608;
    'dispatch: loop {
        match pc {
            0x82EDA608 => {
    //   block [0x82EDA608..0x82EDA6E0)
	// 82EDA608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EDA60C: 482CDB61  bl 0x831a816c
	ctx.lr = 0x82EDA610;
	sub_831A8130(ctx, base);
	// 82EDA610: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EDA614: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EDA618: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82EDA61C: 397F00D0  addi r11, r31, 0xd0
	ctx.r[11].s64 = ctx.r[31].s64 + 208;
	// 82EDA620: 392A406C  addi r9, r10, 0x406c
	ctx.r[9].s64 = ctx.r[10].s64 + 16492;
	// 82EDA624: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82EDA628: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EDA62C: 395F00E0  addi r10, r31, 0xe0
	ctx.r[10].s64 = ctx.r[31].s64 + 224;
	// 82EDA630: B11F0006  sth r8, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 82EDA634: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EDA6E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EDA6E0 size=208
    let mut pc: u32 = 0x82EDA6E0;
    'dispatch: loop {
        match pc {
            0x82EDA6E0 => {
    //   block [0x82EDA6E0..0x82EDA7B0)
	// 82EDA6E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EDA6E4: 482CDA85  bl 0x831a8168
	ctx.lr = 0x82EDA6E8;
	sub_831A8130(ctx, base);
	// 82EDA6E8: 39630010  addi r11, r3, 0x10
	ctx.r[11].s64 = ctx.r[3].s64 + 16;
	// 82EDA6EC: 39440010  addi r10, r4, 0x10
	ctx.r[10].s64 = ctx.r[4].s64 + 16;
	// 82EDA6F0: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 82EDA6F4: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 82EDA6F8: 38E00030  li r7, 0x30
	ctx.r[7].s64 = 48;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EDA7B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EDA7B0 size=160
    let mut pc: u32 = 0x82EDA7B0;
    'dispatch: loop {
        match pc {
            0x82EDA7B0 => {
    //   block [0x82EDA7B0..0x82EDA850)
	// 82EDA7B0: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EDA850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EDA850 size=12
    let mut pc: u32 = 0x82EDA850;
    'dispatch: loop {
        match pc {
            0x82EDA850 => {
    //   block [0x82EDA850..0x82EDA85C)
	// 82EDA850: 3964FFFF  addi r11, r4, -1
	ctx.r[11].s64 = ctx.r[4].s64 + -1;
	// 82EDA854: 2B0B0008  cmplwi cr6, r11, 8
	ctx.cr[6].compare_u32(ctx.r[11].u32, 8 as u32, &mut ctx.xer);
	// 82EDA858: 4D990020  bgtlr cr6
	if ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EDA85C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EDA85C size=24
    let mut pc: u32 = 0x82EDA85C;
    'dispatch: loop {
        match pc {
            0x82EDA85C => {
    //   block [0x82EDA85C..0x82EDA874)
	// 82EDA85C: 3D8082EE  lis r12, -0x7d12
	ctx.r[12].s64 = -2098331648;
	// 82EDA860: 398CA874  addi r12, r12, -0x578c
	ctx.r[12].s64 = ctx.r[12].s64 + -22412;
	// 82EDA864: 5560103A  slwi r0, r11, 2
	ctx.r[0].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 82EDA868: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 82EDA86C: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 82EDA870: 4E800420  bctr
	match ctx.r[11].u64 {
		0 => {
			// ERROR: 0x82EDA898
			return;
		},
		1 => {
			// ERROR: 0x82EDA8BC
			return;
		},
		2 => {
			// ERROR: 0x82EDA8E0
			return;
		},
		3 => {
			// ERROR: 0x82EDA8E8
			return;
		},
		4 => {
			// ERROR: 0x82EDA90C
			return;
		},
		5 => {
			// ERROR: 0x82EDA930
			return;
		},
		6 => {
			// ERROR: 0x82EDA938
			return;
		},
		7 => {
			// ERROR: 0x82EDA95C
			return;
		},
		8 => {
			// ERROR: 0x82EDA980
			return;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EDA874(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82EDA874 size=72
    let mut pc: u32 = 0x82EDA874;
    'dispatch: loop {
        match pc {
            0x82EDA874 => {
    //   block [0x82EDA874..0x82EDA8BC)
	// 82EDA874: 82EDA898  lwz r23, -0x5768(r13)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(-22376 as u32) ) } as u64;
	// 82EDA878: 82EDA8BC  lwz r23, -0x5744(r13)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(-22340 as u32) ) } as u64;
	// 82EDA87C: 82EDA8E0  lwz r23, -0x5720(r13)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(-22304 as u32) ) } as u64;
	// 82EDA880: 82EDA8E8  lwz r23, -0x5718(r13)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(-22296 as u32) ) } as u64;
	// 82EDA884: 82EDA90C  lwz r23, -0x56f4(r13)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(-22260 as u32) ) } as u64;
	// 82EDA888: 82EDA930  lwz r23, -0x56d0(r13)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(-22224 as u32) ) } as u64;
	// 82EDA88C: 82EDA938  lwz r23, -0x56c8(r13)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(-22216 as u32) ) } as u64;
	// 82EDA890: 82EDA95C  lwz r23, -0x56a4(r13)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(-22180 as u32) ) } as u64;
	// 82EDA894: 82EDA980  lwz r23, -0x5680(r13)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(-22144 as u32) ) } as u64;
	// 82EDA898: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82EDA89C: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 82EDA8A0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82EDA8A4: 9123007C  stw r9, 0x7c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(124 as u32), ctx.r[9].u32 ) };
	// 82EDA8A8: C00B7BC8  lfs f0, 0x7bc8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(31688 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EDA8AC: C1AAA100  lfs f13, -0x5f00(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-24320 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82EDA8B0: D0030074  stfs f0, 0x74(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 82EDA8B4: D1A30078  stfs f13, 0x78(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 82EDA8B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EDA8BC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82EDA8BC size=36
    let mut pc: u32 = 0x82EDA8BC;
    'dispatch: loop {
        match pc {
            0x82EDA8BC => {
    //   block [0x82EDA8BC..0x82EDA8E0)
	// 82EDA8BC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82EDA8C0: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82EDA8C4: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82EDA8C8: 9123007C  stw r9, 0x7c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(124 as u32), ctx.r[9].u32 ) };
	// 82EDA8CC: C00BC664  lfs f0, -0x399c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-14748 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EDA8D0: C1AA08A8  lfs f13, 0x8a8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2216 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82EDA8D4: D0030074  stfs f0, 0x74(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 82EDA8D8: D1A30078  stfs f13, 0x78(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 82EDA8DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EDA8E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EDA8E0 size=8
    let mut pc: u32 = 0x82EDA8E0;
    'dispatch: loop {
        match pc {
            0x82EDA8E0 => {
    //   block [0x82EDA8E0..0x82EDA8E8)
	// 82EDA8E0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82EDA8E4: 480000A0  b 0x82eda984
	sub_82EDA980(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EDA8E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82EDA8E8 size=36
    let mut pc: u32 = 0x82EDA8E8;
    'dispatch: loop {
        match pc {
            0x82EDA8E8 => {
    //   block [0x82EDA8E8..0x82EDA90C)
	// 82EDA8E8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82EDA8EC: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 82EDA8F0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82EDA8F4: 9123007C  stw r9, 0x7c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(124 as u32), ctx.r[9].u32 ) };
	// 82EDA8F8: C00B7BC8  lfs f0, 0x7bc8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(31688 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EDA8FC: C1AAA100  lfs f13, -0x5f00(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-24320 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82EDA900: D0030074  stfs f0, 0x74(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 82EDA904: D1A30078  stfs f13, 0x78(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 82EDA908: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EDA90C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82EDA90C size=36
    let mut pc: u32 = 0x82EDA90C;
    'dispatch: loop {
        match pc {
            0x82EDA90C => {
    //   block [0x82EDA90C..0x82EDA930)
	// 82EDA90C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82EDA910: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82EDA914: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82EDA918: 9123007C  stw r9, 0x7c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(124 as u32), ctx.r[9].u32 ) };
	// 82EDA91C: C00BC664  lfs f0, -0x399c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-14748 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EDA920: C1AA08A8  lfs f13, 0x8a8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2216 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82EDA924: D0030074  stfs f0, 0x74(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 82EDA928: D1A30078  stfs f13, 0x78(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 82EDA92C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EDA930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EDA930 size=8
    let mut pc: u32 = 0x82EDA930;
    'dispatch: loop {
        match pc {
            0x82EDA930 => {
    //   block [0x82EDA930..0x82EDA938)
	// 82EDA930: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82EDA934: 48000050  b 0x82eda984
	sub_82EDA980(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EDA938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82EDA938 size=36
    let mut pc: u32 = 0x82EDA938;
    'dispatch: loop {
        match pc {
            0x82EDA938 => {
    //   block [0x82EDA938..0x82EDA95C)
	// 82EDA938: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82EDA93C: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 82EDA940: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 82EDA944: 9123007C  stw r9, 0x7c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(124 as u32), ctx.r[9].u32 ) };
	// 82EDA948: C00B7BC8  lfs f0, 0x7bc8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(31688 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EDA94C: C1AAA100  lfs f13, -0x5f00(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-24320 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82EDA950: D0030074  stfs f0, 0x74(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 82EDA954: D1A30078  stfs f13, 0x78(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 82EDA958: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EDA95C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82EDA95C size=36
    let mut pc: u32 = 0x82EDA95C;
    'dispatch: loop {
        match pc {
            0x82EDA95C => {
    //   block [0x82EDA95C..0x82EDA980)
	// 82EDA95C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82EDA960: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82EDA964: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 82EDA968: 9123007C  stw r9, 0x7c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(124 as u32), ctx.r[9].u32 ) };
	// 82EDA96C: C00BC664  lfs f0, -0x399c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-14748 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EDA970: C1AA08A8  lfs f13, 0x8a8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2216 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82EDA974: D0030074  stfs f0, 0x74(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 82EDA978: D1A30078  stfs f13, 0x78(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 82EDA97C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


