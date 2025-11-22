pub fn sub_82EC6530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EC6530 size=4
    let mut pc: u32 = 0x82EC6530;
    'dispatch: loop {
        match pc {
            0x82EC6530 => {
    //   block [0x82EC6530..0x82EC6534)
	// 82EC6530: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EC6538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EC6538 size=4
    let mut pc: u32 = 0x82EC6538;
    'dispatch: loop {
        match pc {
            0x82EC6538 => {
    //   block [0x82EC6538..0x82EC653C)
	// 82EC6538: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EC6540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EC6540 size=4
    let mut pc: u32 = 0x82EC6540;
    'dispatch: loop {
        match pc {
            0x82EC6540 => {
    //   block [0x82EC6540..0x82EC6544)
	// 82EC6540: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EC6548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EC6548 size=4
    let mut pc: u32 = 0x82EC6548;
    'dispatch: loop {
        match pc {
            0x82EC6548 => {
    //   block [0x82EC6548..0x82EC654C)
	// 82EC6548: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EC6550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EC6550 size=8
    let mut pc: u32 = 0x82EC6550;
    'dispatch: loop {
        match pc {
            0x82EC6550 => {
    //   block [0x82EC6550..0x82EC6558)
	// 82EC6550: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EC6554: 48000004  b 0x82ec6558
	sub_82EC6558(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EC6558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EC6558 size=192
    let mut pc: u32 = 0x82EC6558;
    'dispatch: loop {
        match pc {
            0x82EC6558 => {
    //   block [0x82EC6558..0x82EC6618)
	// 82EC6558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EC655C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EC6560: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EC6564: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EC6568: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EC656C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EC6570: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EC6574: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EC6578: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82EC657C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EC6580: 409A0020  bne cr6, 0x82ec65a0
	if !ctx.cr[6].eq {
	pc = 0x82EC65A0; continue 'dispatch;
	}
	// 82EC6584: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EC6588: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82EC658C: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82EC6590: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EC6594: 55651838  slwi r5, r11, 3
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82EC6598: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82EC659C: 4BFDA215  bl 0x82ea07b0
	ctx.lr = 0x82EC65A0;
	sub_82EA07B0(ctx, base);
	// 82EC65A0: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82EC65A4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EC65A8: 419A0054  beq cr6, 0x82ec65fc
	if ctx.cr[6].eq {
	pc = 0x82EC65FC; continue 'dispatch;
	}
	// 82EC65AC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EC65B0: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82EC65B4: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EC65B8: 812B004C  lwz r9, 0x4c(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 82EC65BC: 810B0034  lwz r8, 0x34(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82EC65C0: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82EC65C4: 4198001C  blt cr6, 0x82ec65e0
	if ctx.cr[6].lt {
	pc = 0x82EC65E0; continue 'dispatch;
	}
	// 82EC65C8: 38C0000A  li r6, 0xa
	ctx.r[6].s64 = 10;
	// 82EC65CC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82EC65D0: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 82EC65D4: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82EC65D8: 4BFD9A89  bl 0x82ea0060
	ctx.lr = 0x82EC65DC;
	sub_82EA0060(ctx, base);
	// 82EC65DC: 48000020  b 0x82ec65fc
	pc = 0x82EC65FC; continue 'dispatch;
	// 82EC65E0: 812B004C  lwz r9, 0x4c(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 82EC65E4: 394B0048  addi r10, r11, 0x48
	ctx.r[10].s64 = ctx.r[11].s64 + 72;
	// 82EC65E8: 814B0048  lwz r10, 0x48(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 82EC65EC: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82EC65F0: 912B004C  stw r9, 0x4c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(76 as u32), ctx.r[9].u32 ) };
	// 82EC65F4: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EC65F8: 93EB0048  stw r31, 0x48(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(72 as u32), ctx.r[31].u32 ) };
	// 82EC65FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EC6600: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EC6604: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EC6608: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EC660C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EC6610: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EC6614: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EC6618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EC6618 size=4
    let mut pc: u32 = 0x82EC6618;
    'dispatch: loop {
        match pc {
            0x82EC6618 => {
    //   block [0x82EC6618..0x82EC661C)
	// 82EC6618: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EC6620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EC6620 size=4
    let mut pc: u32 = 0x82EC6620;
    'dispatch: loop {
        match pc {
            0x82EC6620 => {
    //   block [0x82EC6620..0x82EC6624)
	// 82EC6620: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EC6628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EC6628 size=4
    let mut pc: u32 = 0x82EC6628;
    'dispatch: loop {
        match pc {
            0x82EC6628 => {
    //   block [0x82EC6628..0x82EC662C)
	// 82EC6628: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EC6630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EC6630 size=4
    let mut pc: u32 = 0x82EC6630;
    'dispatch: loop {
        match pc {
            0x82EC6630 => {
    //   block [0x82EC6630..0x82EC6634)
	// 82EC6630: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EC6638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EC6638 size=4
    let mut pc: u32 = 0x82EC6638;
    'dispatch: loop {
        match pc {
            0x82EC6638 => {
    //   block [0x82EC6638..0x82EC663C)
	// 82EC6638: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EC6640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EC6640 size=4
    let mut pc: u32 = 0x82EC6640;
    'dispatch: loop {
        match pc {
            0x82EC6640 => {
    //   block [0x82EC6640..0x82EC6644)
	// 82EC6640: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EC6648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EC6648 size=4
    let mut pc: u32 = 0x82EC6648;
    'dispatch: loop {
        match pc {
            0x82EC6648 => {
    //   block [0x82EC6648..0x82EC664C)
	// 82EC6648: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EC6650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EC6650 size=4
    let mut pc: u32 = 0x82EC6650;
    'dispatch: loop {
        match pc {
            0x82EC6650 => {
    //   block [0x82EC6650..0x82EC6654)
	// 82EC6650: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EC6658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EC6658 size=4
    let mut pc: u32 = 0x82EC6658;
    'dispatch: loop {
        match pc {
            0x82EC6658 => {
    //   block [0x82EC6658..0x82EC665C)
	// 82EC6658: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EC6660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EC6660 size=168
    let mut pc: u32 = 0x82EC6660;
    'dispatch: loop {
        match pc {
            0x82EC6660 => {
    //   block [0x82EC6660..0x82EC6708)
	// 82EC6660: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EC6664: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EC6668: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EC666C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EC6670: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EC6674: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EC6678: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EC667C: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EC6680: 4BFDBA31  bl 0x82ea20b0
	ctx.lr = 0x82EC6684;
	sub_82EA20B0(ctx, base);
	// 82EC6684: 813F000C  lwz r9, 0xc(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EC6688: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EC668C: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82EC6690: 40990050  ble cr6, 0x82ec66e0
	if !ctx.cr[6].gt {
	pc = 0x82EC66E0; continue 'dispatch;
	}
	// 82EC6694: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EC6698: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EC669C: 7F08F040  cmplw cr6, r8, r30
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82EC66A0: 419A0018  beq cr6, 0x82ec66b8
	if ctx.cr[6].eq {
	pc = 0x82EC66B8; continue 'dispatch;
	}
	// 82EC66A4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EC66A8: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82EC66AC: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82EC66B0: 4198FFE8  blt cr6, 0x82ec6698
	if ctx.cr[6].lt {
	pc = 0x82EC6698; continue 'dispatch;
	}
	// 82EC66B4: 4800002C  b 0x82ec66e0
	pc = 0x82EC66E0; continue 'dispatch;
	// 82EC66B8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EC66BC: 41980024  blt cr6, 0x82ec66e0
	if ctx.cr[6].lt {
	pc = 0x82EC66E0; continue 'dispatch;
	}
	// 82EC66C0: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EC66C4: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82EC66C8: 811F0008  lwz r8, 8(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EC66CC: 396AFFFF  addi r11, r10, -1
	ctx.r[11].s64 = ctx.r[10].s64 + -1;
	// 82EC66D0: 5567103A  slwi r7, r11, 2
	ctx.r[7].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82EC66D4: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82EC66D8: 7CC7402E  lwzx r6, r7, r8
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82EC66DC: 7CC9412E  stwx r6, r9, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[8].u32), ctx.r[6].u32) };
	// 82EC66E0: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EC66E4: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82EC66E8: F9630020  std r11, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82EC66EC: 4837C271  bl 0x8324295c
	ctx.lr = 0x82EC66F0;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EC66F0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EC66F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EC66F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EC66FC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EC6700: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EC6704: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EC6708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EC6708 size=72
    let mut pc: u32 = 0x82EC6708;
    'dispatch: loop {
        match pc {
            0x82EC6708 => {
    //   block [0x82EC6708..0x82EC6750)
	// 82EC6708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EC670C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EC6710: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EC6714: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EC6718: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EC671C: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EC6720: 4BFDB991  bl 0x82ea20b0
	ctx.lr = 0x82EC6724;
	sub_82EA20B0(ctx, base);
	// 82EC6724: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EC6728: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EC672C: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82EC6730: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82EC6734: F9430020  std r10, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u64 ) };
	// 82EC6738: 4837C225  bl 0x8324295c
	ctx.lr = 0x82EC673C;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EC673C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EC6740: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EC6744: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EC6748: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EC674C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EC6750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EC6750 size=152
    let mut pc: u32 = 0x82EC6750;
    'dispatch: loop {
        match pc {
            0x82EC6750 => {
    //   block [0x82EC6750..0x82EC67E8)
	// 82EC6750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EC6754: 482E1A05  bl 0x831a8158
	ctx.lr = 0x82EC6758;
	sub_831A8130(ctx, base);
	// 82EC6758: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EC675C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EC6760: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82EC6764: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82EC6768: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82EC676C: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 82EC6770: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EC6774: 7D184378  mr r24, r8
	ctx.r[24].u64 = ctx.r[8].u64;
	// 82EC6778: 4BFDB939  bl 0x82ea20b0
	ctx.lr = 0x82EC677C;
	sub_82EA20B0(ctx, base);
	// 82EC677C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EC6780: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82EC6784: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EC6788: 40990048  ble cr6, 0x82ec67d0
	if !ctx.cr[6].gt {
	pc = 0x82EC67D0; continue 'dispatch;
	}
	// 82EC678C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82EC6790: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EC6794: 7F08C378  mr r8, r24
	ctx.r[8].u64 = ctx.r[24].u64;
	// 82EC6798: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 82EC679C: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82EC67A0: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82EC67A4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82EC67A8: 7C7D582E  lwzx r3, r29, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EC67AC: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EC67B0: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EC67B4: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82EC67B8: 4E800421  bctrl
	ctx.lr = 0x82EC67BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EC67BC: 811F000C  lwz r8, 0xc(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EC67C0: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82EC67C4: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82EC67C8: 7F1E4000  cmpw cr6, r30, r8
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82EC67CC: 4198FFC4  blt cr6, 0x82ec6790
	if ctx.cr[6].lt {
	pc = 0x82EC6790; continue 'dispatch;
	}
	// 82EC67D0: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EC67D4: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82EC67D8: F9630020  std r11, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82EC67DC: 4837C181  bl 0x8324295c
	ctx.lr = 0x82EC67E0;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EC67E0: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82EC67E4: 482E19C4  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EC67E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EC67E8 size=152
    let mut pc: u32 = 0x82EC67E8;
    'dispatch: loop {
        match pc {
            0x82EC67E8 => {
    //   block [0x82EC67E8..0x82EC6880)
	// 82EC67E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EC67EC: 482E196D  bl 0x831a8158
	ctx.lr = 0x82EC67F0;
	sub_831A8130(ctx, base);
	// 82EC67F0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EC67F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EC67F8: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82EC67FC: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82EC6800: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82EC6804: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 82EC6808: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EC680C: 7D184378  mr r24, r8
	ctx.r[24].u64 = ctx.r[8].u64;
	// 82EC6810: 4BFDB8A1  bl 0x82ea20b0
	ctx.lr = 0x82EC6814;
	sub_82EA20B0(ctx, base);
	// 82EC6814: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EC6818: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82EC681C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EC6820: 40990048  ble cr6, 0x82ec6868
	if !ctx.cr[6].gt {
	pc = 0x82EC6868; continue 'dispatch;
	}
	// 82EC6824: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82EC6828: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EC682C: 7F08C378  mr r8, r24
	ctx.r[8].u64 = ctx.r[24].u64;
	// 82EC6830: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 82EC6834: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82EC6838: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82EC683C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82EC6840: 7C7D582E  lwzx r3, r29, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EC6844: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EC6848: 812A0008  lwz r9, 8(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EC684C: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82EC6850: 4E800421  bctrl
	ctx.lr = 0x82EC6854;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EC6854: 811F000C  lwz r8, 0xc(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EC6858: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82EC685C: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82EC6860: 7F1E4000  cmpw cr6, r30, r8
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82EC6864: 4198FFC4  blt cr6, 0x82ec6828
	if ctx.cr[6].lt {
	pc = 0x82EC6828; continue 'dispatch;
	}
	// 82EC6868: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EC686C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82EC6870: F9630020  std r11, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82EC6874: 4837C0E9  bl 0x8324295c
	ctx.lr = 0x82EC6878;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EC6878: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82EC687C: 482E192C  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EC6880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EC6880 size=136
    let mut pc: u32 = 0x82EC6880;
    'dispatch: loop {
        match pc {
            0x82EC6880 => {
    //   block [0x82EC6880..0x82EC6908)
	// 82EC6880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EC6884: 482E18DD  bl 0x831a8160
	ctx.lr = 0x82EC6888;
	sub_831A8130(ctx, base);
	// 82EC6888: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EC688C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EC6890: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82EC6894: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82EC6898: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82EC689C: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EC68A0: 4BFDB811  bl 0x82ea20b0
	ctx.lr = 0x82EC68A4;
	sub_82EA20B0(ctx, base);
	// 82EC68A4: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EC68A8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82EC68AC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EC68B0: 40990040  ble cr6, 0x82ec68f0
	if !ctx.cr[6].gt {
	pc = 0x82EC68F0; continue 'dispatch;
	}
	// 82EC68B4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82EC68B8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EC68BC: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82EC68C0: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82EC68C4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82EC68C8: 7C7D582E  lwzx r3, r29, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EC68CC: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EC68D0: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EC68D4: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82EC68D8: 4E800421  bctrl
	ctx.lr = 0x82EC68DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EC68DC: 811F000C  lwz r8, 0xc(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EC68E0: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82EC68E4: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82EC68E8: 7F1E4000  cmpw cr6, r30, r8
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82EC68EC: 4198FFCC  blt cr6, 0x82ec68b8
	if ctx.cr[6].lt {
	pc = 0x82EC68B8; continue 'dispatch;
	}
	// 82EC68F0: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EC68F4: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82EC68F8: F9630020  std r11, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82EC68FC: 4837C061  bl 0x8324295c
	ctx.lr = 0x82EC6900;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EC6900: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EC6904: 482E18AC  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EC6908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EC6908 size=136
    let mut pc: u32 = 0x82EC6908;
    'dispatch: loop {
        match pc {
            0x82EC6908 => {
    //   block [0x82EC6908..0x82EC6990)
	// 82EC6908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EC690C: 482E1855  bl 0x831a8160
	ctx.lr = 0x82EC6910;
	sub_831A8130(ctx, base);
	// 82EC6910: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EC6914: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EC6918: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82EC691C: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82EC6920: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82EC6924: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EC6928: 4BFDB789  bl 0x82ea20b0
	ctx.lr = 0x82EC692C;
	sub_82EA20B0(ctx, base);
	// 82EC692C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EC6930: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82EC6934: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EC6938: 40990040  ble cr6, 0x82ec6978
	if !ctx.cr[6].gt {
	pc = 0x82EC6978; continue 'dispatch;
	}
	// 82EC693C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82EC6940: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EC6944: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82EC6948: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82EC694C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82EC6950: 7C7D582E  lwzx r3, r29, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EC6954: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EC6958: 812A0010  lwz r9, 0x10(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EC695C: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82EC6960: 4E800421  bctrl
	ctx.lr = 0x82EC6964;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EC6964: 811F000C  lwz r8, 0xc(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EC6968: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82EC696C: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82EC6970: 7F1E4000  cmpw cr6, r30, r8
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82EC6974: 4198FFCC  blt cr6, 0x82ec6940
	if ctx.cr[6].lt {
	pc = 0x82EC6940; continue 'dispatch;
	}
	// 82EC6978: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EC697C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82EC6980: F9630020  std r11, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82EC6984: 4837BFD9  bl 0x8324295c
	ctx.lr = 0x82EC6988;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EC6988: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EC698C: 482E1824  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EC6990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EC6990 size=136
    let mut pc: u32 = 0x82EC6990;
    'dispatch: loop {
        match pc {
            0x82EC6990 => {
    //   block [0x82EC6990..0x82EC6A18)
	// 82EC6990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EC6994: 482E17CD  bl 0x831a8160
	ctx.lr = 0x82EC6998;
	sub_831A8130(ctx, base);
	// 82EC6998: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EC699C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EC69A0: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82EC69A4: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82EC69A8: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82EC69AC: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EC69B0: 4BFDB701  bl 0x82ea20b0
	ctx.lr = 0x82EC69B4;
	sub_82EA20B0(ctx, base);
	// 82EC69B4: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EC69B8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82EC69BC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EC69C0: 40990040  ble cr6, 0x82ec6a00
	if !ctx.cr[6].gt {
	pc = 0x82EC6A00; continue 'dispatch;
	}
	// 82EC69C4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82EC69C8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EC69CC: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82EC69D0: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82EC69D4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82EC69D8: 7C7D582E  lwzx r3, r29, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EC69DC: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EC69E0: 812A0014  lwz r9, 0x14(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EC69E4: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82EC69E8: 4E800421  bctrl
	ctx.lr = 0x82EC69EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EC69EC: 811F000C  lwz r8, 0xc(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EC69F0: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82EC69F4: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82EC69F8: 7F1E4000  cmpw cr6, r30, r8
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82EC69FC: 4198FFCC  blt cr6, 0x82ec69c8
	if ctx.cr[6].lt {
	pc = 0x82EC69C8; continue 'dispatch;
	}
	// 82EC6A00: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EC6A04: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82EC6A08: F9630020  std r11, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82EC6A0C: 4837BF51  bl 0x8324295c
	ctx.lr = 0x82EC6A10;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EC6A10: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EC6A14: 482E179C  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EC6A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EC6A18 size=192
    let mut pc: u32 = 0x82EC6A18;
    'dispatch: loop {
        match pc {
            0x82EC6A18 => {
    //   block [0x82EC6A18..0x82EC6AD8)
	// 82EC6A18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EC6A1C: 482E1741  bl 0x831a815c
	ctx.lr = 0x82EC6A20;
	sub_831A8130(ctx, base);
	// 82EC6A20: DBA1FFA8  stfd f29, -0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-88 as u32), ctx.f[29].u64 ) };
	// 82EC6A24: DBC1FFB0  stfd f30, -0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-80 as u32), ctx.f[30].u64 ) };
	// 82EC6A28: DBE1FFB8  stfd f31, -0x48(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-72 as u32), ctx.f[31].u64 ) };
	// 82EC6A2C: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EC6A30: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EC6A34: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82EC6A38: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82EC6A3C: FFC01090  fmr f30, f2
	ctx.f[30].f64 = ctx.f[2].f64;
	// 82EC6A40: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82EC6A44: FFA01890  fmr f29, f3
	ctx.f[29].f64 = ctx.f[3].f64;
	// 82EC6A48: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82EC6A4C: 7D595378  mr r25, r10
	ctx.r[25].u64 = ctx.r[10].u64;
	// 82EC6A50: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EC6A54: 4BFDB65D  bl 0x82ea20b0
	ctx.lr = 0x82EC6A58;
	sub_82EA20B0(ctx, base);
	// 82EC6A58: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EC6A5C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82EC6A60: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EC6A64: 40990050  ble cr6, 0x82ec6ab4
	if !ctx.cr[6].gt {
	pc = 0x82EC6AB4; continue 'dispatch;
	}
	// 82EC6A68: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82EC6A6C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EC6A70: 7F2ACB78  mr r10, r25
	ctx.r[10].u64 = ctx.r[25].u64;
	// 82EC6A74: FC60E890  fmr f3, f29
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].f64 = ctx.f[29].f64;
	// 82EC6A78: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82EC6A7C: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 82EC6A80: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82EC6A84: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82EC6A88: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82EC6A8C: 7C7D582E  lwzx r3, r29, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EC6A90: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EC6A94: 81090018  lwz r8, 0x18(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EC6A98: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82EC6A9C: 4E800421  bctrl
	ctx.lr = 0x82EC6AA0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EC6AA0: 80FF000C  lwz r7, 0xc(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EC6AA4: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82EC6AA8: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82EC6AAC: 7F1E3800  cmpw cr6, r30, r7
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[7].s32, &mut ctx.xer);
	// 82EC6AB0: 4198FFBC  blt cr6, 0x82ec6a6c
	if ctx.cr[6].lt {
	pc = 0x82EC6A6C; continue 'dispatch;
	}
	// 82EC6AB4: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EC6AB8: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82EC6ABC: F9630020  std r11, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82EC6AC0: 4837BE9D  bl 0x8324295c
	ctx.lr = 0x82EC6AC4;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EC6AC4: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82EC6AC8: CBA1FFA8  lfd f29, -0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[29].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-88 as u32) ) };
	// 82EC6ACC: CBC1FFB0  lfd f30, -0x50(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-80 as u32) ) };
	// 82EC6AD0: CBE1FFB8  lfd f31, -0x48(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-72 as u32) ) };
	// 82EC6AD4: 482E16D8  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EC6AD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EC6AD8 size=136
    let mut pc: u32 = 0x82EC6AD8;
    'dispatch: loop {
        match pc {
            0x82EC6AD8 => {
    //   block [0x82EC6AD8..0x82EC6B60)
	// 82EC6AD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EC6ADC: 482E1685  bl 0x831a8160
	ctx.lr = 0x82EC6AE0;
	sub_831A8130(ctx, base);
	// 82EC6AE0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EC6AE4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EC6AE8: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82EC6AEC: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82EC6AF0: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82EC6AF4: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EC6AF8: 4BFDB5B9  bl 0x82ea20b0
	ctx.lr = 0x82EC6AFC;
	sub_82EA20B0(ctx, base);
	// 82EC6AFC: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EC6B00: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82EC6B04: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EC6B08: 40990040  ble cr6, 0x82ec6b48
	if !ctx.cr[6].gt {
	pc = 0x82EC6B48; continue 'dispatch;
	}
	// 82EC6B0C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82EC6B10: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EC6B14: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82EC6B18: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82EC6B1C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82EC6B20: 7C7D582E  lwzx r3, r29, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EC6B24: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EC6B28: 812A001C  lwz r9, 0x1c(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EC6B2C: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82EC6B30: 4E800421  bctrl
	ctx.lr = 0x82EC6B34;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EC6B34: 811F000C  lwz r8, 0xc(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EC6B38: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82EC6B3C: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82EC6B40: 7F1E4000  cmpw cr6, r30, r8
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82EC6B44: 4198FFCC  blt cr6, 0x82ec6b10
	if ctx.cr[6].lt {
	pc = 0x82EC6B10; continue 'dispatch;
	}
	// 82EC6B48: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EC6B4C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82EC6B50: F9630020  std r11, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82EC6B54: 4837BE09  bl 0x8324295c
	ctx.lr = 0x82EC6B58;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EC6B58: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EC6B5C: 482E1654  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EC6B60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EC6B60 size=144
    let mut pc: u32 = 0x82EC6B60;
    'dispatch: loop {
        match pc {
            0x82EC6B60 => {
    //   block [0x82EC6B60..0x82EC6BF0)
	// 82EC6B60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EC6B64: 482E15F9  bl 0x831a815c
	ctx.lr = 0x82EC6B68;
	sub_831A8130(ctx, base);
	// 82EC6B68: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EC6B6C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EC6B70: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82EC6B74: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82EC6B78: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82EC6B7C: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 82EC6B80: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EC6B84: 4BFDB52D  bl 0x82ea20b0
	ctx.lr = 0x82EC6B88;
	sub_82EA20B0(ctx, base);
	// 82EC6B88: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EC6B8C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82EC6B90: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EC6B94: 40990044  ble cr6, 0x82ec6bd8
	if !ctx.cr[6].gt {
	pc = 0x82EC6BD8; continue 'dispatch;
	}
	// 82EC6B98: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82EC6B9C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EC6BA0: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 82EC6BA4: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82EC6BA8: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82EC6BAC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82EC6BB0: 7C7D582E  lwzx r3, r29, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EC6BB4: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EC6BB8: 812A0020  lwz r9, 0x20(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(32 as u32) ) } as u64;
	// 82EC6BBC: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82EC6BC0: 4E800421  bctrl
	ctx.lr = 0x82EC6BC4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EC6BC4: 811F000C  lwz r8, 0xc(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EC6BC8: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82EC6BCC: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82EC6BD0: 7F1E4000  cmpw cr6, r30, r8
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82EC6BD4: 4198FFC8  blt cr6, 0x82ec6b9c
	if ctx.cr[6].lt {
	pc = 0x82EC6B9C; continue 'dispatch;
	}
	// 82EC6BD8: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EC6BDC: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82EC6BE0: F9630020  std r11, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82EC6BE4: 4837BD79  bl 0x8324295c
	ctx.lr = 0x82EC6BE8;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EC6BE8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EC6BEC: 482E15C0  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EC6BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EC6BF0 size=152
    let mut pc: u32 = 0x82EC6BF0;
    'dispatch: loop {
        match pc {
            0x82EC6BF0 => {
    //   block [0x82EC6BF0..0x82EC6C88)
	// 82EC6BF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EC6BF4: 482E1565  bl 0x831a8158
	ctx.lr = 0x82EC6BF8;
	sub_831A8130(ctx, base);
	// 82EC6BF8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EC6BFC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EC6C00: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82EC6C04: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82EC6C08: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82EC6C0C: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 82EC6C10: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EC6C14: 7D184378  mr r24, r8
	ctx.r[24].u64 = ctx.r[8].u64;
	// 82EC6C18: 4BFDB499  bl 0x82ea20b0
	ctx.lr = 0x82EC6C1C;
	sub_82EA20B0(ctx, base);
	// 82EC6C1C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EC6C20: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82EC6C24: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EC6C28: 40990048  ble cr6, 0x82ec6c70
	if !ctx.cr[6].gt {
	pc = 0x82EC6C70; continue 'dispatch;
	}
	// 82EC6C2C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82EC6C30: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EC6C34: 7F08C378  mr r8, r24
	ctx.r[8].u64 = ctx.r[24].u64;
	// 82EC6C38: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 82EC6C3C: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82EC6C40: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82EC6C44: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82EC6C48: 7C7D582E  lwzx r3, r29, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EC6C4C: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EC6C50: 812A0024  lwz r9, 0x24(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(36 as u32) ) } as u64;
	// 82EC6C54: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82EC6C58: 4E800421  bctrl
	ctx.lr = 0x82EC6C5C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EC6C5C: 811F000C  lwz r8, 0xc(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EC6C60: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82EC6C64: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82EC6C68: 7F1E4000  cmpw cr6, r30, r8
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82EC6C6C: 4198FFC4  blt cr6, 0x82ec6c30
	if ctx.cr[6].lt {
	pc = 0x82EC6C30; continue 'dispatch;
	}
	// 82EC6C70: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EC6C74: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82EC6C78: F9630020  std r11, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82EC6C7C: 4837BCE1  bl 0x8324295c
	ctx.lr = 0x82EC6C80;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EC6C80: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82EC6C84: 482E1524  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EC6C88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EC6C88 size=88
    let mut pc: u32 = 0x82EC6C88;
    'dispatch: loop {
        match pc {
            0x82EC6C88 => {
    //   block [0x82EC6C88..0x82EC6CE0)
	// 82EC6C88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EC6C8C: 482E14D9  bl 0x831a8164
	ctx.lr = 0x82EC6C90;
	sub_831A8130(ctx, base);
	// 82EC6C90: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EC6C94: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EC6C98: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82EC6C9C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EC6CA0: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82EC6CA4: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 82EC6CA8: 7D1B4378  mr r27, r8
	ctx.r[27].u64 = ctx.r[8].u64;
	// 82EC6CAC: 4BFE02C5  bl 0x82ea6f70
	ctx.lr = 0x82EC6CB0;
	sub_82EA6F70(ctx, base);
	// 82EC6CB0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82EC6CB4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82EC6CB8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EC6CBC: 4BFE02B5  bl 0x82ea6f70
	ctx.lr = 0x82EC6CC0;
	sub_82EA6F70(ctx, base);
	// 82EC6CC0: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82EC6CC4: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82EC6CC8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82EC6CCC: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82EC6CD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EC6CD4: 4BFFFE8D  bl 0x82ec6b60
	ctx.lr = 0x82EC6CD8;
	sub_82EC6B60(ctx, base);
	// 82EC6CD8: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82EC6CDC: 482E14D8  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EC6CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EC6CE0 size=104
    let mut pc: u32 = 0x82EC6CE0;
    'dispatch: loop {
        match pc {
            0x82EC6CE0 => {
    //   block [0x82EC6CE0..0x82EC6D48)
	// 82EC6CE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EC6CE4: 482E1481  bl 0x831a8164
	ctx.lr = 0x82EC6CE8;
	sub_831A8130(ctx, base);
	// 82EC6CE8: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EC6CEC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EC6CF0: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82EC6CF4: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82EC6CF8: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82EC6CFC: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 82EC6D00: 7D1B4378  mr r27, r8
	ctx.r[27].u64 = ctx.r[8].u64;
	// 82EC6D04: 4BFE221D  bl 0x82ea8f20
	ctx.lr = 0x82EC6D08;
	sub_82EA8F20(ctx, base);
	// 82EC6D08: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82EC6D0C: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82EC6D10: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82EC6D14: 4BFE025D  bl 0x82ea6f70
	ctx.lr = 0x82EC6D18;
	sub_82EA6F70(ctx, base);
	// 82EC6D18: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82EC6D1C: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82EC6D20: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EC6D24: 4BFE024D  bl 0x82ea6f70
	ctx.lr = 0x82EC6D28;
	sub_82EA6F70(ctx, base);
	// 82EC6D28: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82EC6D2C: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82EC6D30: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82EC6D34: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82EC6D38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EC6D3C: 4BFFFE25  bl 0x82ec6b60
	ctx.lr = 0x82EC6D40;
	sub_82EC6B60(ctx, base);
	// 82EC6D40: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82EC6D44: 482E1470  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EC6D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EC6D48 size=136
    let mut pc: u32 = 0x82EC6D48;
    'dispatch: loop {
        match pc {
            0x82EC6D48 => {
    //   block [0x82EC6D48..0x82EC6DD0)
	// 82EC6D48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EC6D4C: 482E1415  bl 0x831a8160
	ctx.lr = 0x82EC6D50;
	sub_831A8130(ctx, base);
	// 82EC6D50: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EC6D54: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EC6DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EC6DD0 size=164
    let mut pc: u32 = 0x82EC6DD0;
    'dispatch: loop {
        match pc {
            0x82EC6DD0 => {
    //   block [0x82EC6DD0..0x82EC6E74)
	// 82EC6DD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EC6DD4: 482E1391  bl 0x831a8164
	ctx.lr = 0x82EC6DD8;
	sub_831A8130(ctx, base);
	// 82EC6DD8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EC6DDC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EC6DE0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82EC6DE4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82EC6DE8: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82EC6DEC: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 82EC6DF0: 7D1B4378  mr r27, r8
	ctx.r[27].u64 = ctx.r[8].u64;
	// 82EC6DF4: 4BFE017D  bl 0x82ea6f70
	ctx.lr = 0x82EC6DF8;
	sub_82EA6F70(ctx, base);
	// 82EC6DF8: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EC6E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EC6E78 size=76
    let mut pc: u32 = 0x82EC6E78;
    'dispatch: loop {
        match pc {
            0x82EC6E78 => {
    //   block [0x82EC6E78..0x82EC6EC4)
	// 82EC6E78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EC6E7C: 482E12E9  bl 0x831a8164
	ctx.lr = 0x82EC6E80;
	sub_831A8130(ctx, base);
	// 82EC6E80: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EC6E84: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EC6E88: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EC6E8C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82EC6E90: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82EC6E94: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 82EC6E98: 7D1B4378  mr r27, r8
	ctx.r[27].u64 = ctx.r[8].u64;
	// 82EC6E9C: 4BFE2085  bl 0x82ea8f20
	ctx.lr = 0x82EC6EA0;
	sub_82EA8F20(ctx, base);
	// 82EC6EA0: 7F68DB78  mr r8, r27
	ctx.r[8].u64 = ctx.r[27].u64;
	// 82EC6EA4: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82EC6EA8: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82EC6EAC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82EC6EB0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82EC6EB4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EC6EB8: 4BFFFF19  bl 0x82ec6dd0
	ctx.lr = 0x82EC6EBC;
	sub_82EC6DD0(ctx, base);
	// 82EC6EBC: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82EC6EC0: 482E12F4  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EC6EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EC6EC8 size=628
    let mut pc: u32 = 0x82EC6EC8;
    'dispatch: loop {
        match pc {
            0x82EC6EC8 => {
    //   block [0x82EC6EC8..0x82EC713C)
	// 82EC6EC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EC6ECC: 482E1289  bl 0x831a8154
	ctx.lr = 0x82EC6ED0;
	sub_831A8130(ctx, base);
	// 82EC6ED0: 9421FEF0  stwu r1, -0x110(r1)
	ea = ctx.r[1].u32.wrapping_add(-272 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EC7140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EC7140 size=168
    let mut pc: u32 = 0x82EC7140;
    'dispatch: loop {
        match pc {
            0x82EC7140 => {
    //   block [0x82EC7140..0x82EC71E8)
	// 82EC7140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EC7144: 482E101D  bl 0x831a8160
	ctx.lr = 0x82EC7148;
	sub_831A8130(ctx, base);
	// 82EC7148: DBE1FFC0  stfd f31, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.f[31].u64 ) };
	// 82EC714C: 3980FFB0  li r12, -0x50
	ctx.r[12].s64 = -80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EC71E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EC71E8 size=76
    let mut pc: u32 = 0x82EC71E8;
    'dispatch: loop {
        match pc {
            0x82EC71E8 => {
    //   block [0x82EC71E8..0x82EC7234)
	// 82EC71E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EC71EC: 482E0F81  bl 0x831a816c
	ctx.lr = 0x82EC71F0;
	sub_831A8130(ctx, base);
	// 82EC71F0: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82EC71F4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EC71F8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EC71FC: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82EC7200: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EC7204: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82EC7208: 7D1D4378  mr r29, r8
	ctx.r[29].u64 = ctx.r[8].u64;
	// 82EC720C: 4BFDFD65  bl 0x82ea6f70
	ctx.lr = 0x82EC7210;
	sub_82EA6F70(ctx, base);
	// 82EC7210: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 82EC7214: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82EC7218: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82EC721C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82EC7220: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EC7224: 4BFFFF1D  bl 0x82ec7140
	ctx.lr = 0x82EC7228;
	sub_82EC7140(ctx, base);
	// 82EC7228: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EC722C: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 82EC7230: 482E0F8C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EC7238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EC7238 size=76
    let mut pc: u32 = 0x82EC7238;
    'dispatch: loop {
        match pc {
            0x82EC7238 => {
    //   block [0x82EC7238..0x82EC7284)
	// 82EC7238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EC723C: 482E0F31  bl 0x831a816c
	ctx.lr = 0x82EC7240;
	sub_831A8130(ctx, base);
	// 82EC7240: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82EC7244: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EC7248: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EC724C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82EC7250: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EC7254: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82EC7258: 7D1D4378  mr r29, r8
	ctx.r[29].u64 = ctx.r[8].u64;
	// 82EC725C: 4BFDFC0D  bl 0x82ea6e68
	ctx.lr = 0x82EC7260;
	sub_82EA6E68(ctx, base);
	// 82EC7260: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 82EC7264: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82EC7268: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82EC726C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82EC7270: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EC7274: 4BFFFECD  bl 0x82ec7140
	ctx.lr = 0x82EC7278;
	sub_82EC7140(ctx, base);
	// 82EC7278: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EC727C: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 82EC7280: 482E0F3C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EC7288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EC7288 size=728
    let mut pc: u32 = 0x82EC7288;
    'dispatch: loop {
        match pc {
            0x82EC7288 => {
    //   block [0x82EC7288..0x82EC7560)
	// 82EC7288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EC728C: 482E0ED5  bl 0x831a8160
	ctx.lr = 0x82EC7290;
	sub_831A8130(ctx, base);
	// 82EC7290: 3980FFA0  li r12, -0x60
	ctx.r[12].s64 = -96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EC7560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EC7560 size=136
    let mut pc: u32 = 0x82EC7560;
    'dispatch: loop {
        match pc {
            0x82EC7560 => {
    //   block [0x82EC7560..0x82EC75E8)
	// 82EC7560: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EC7564: 482E0BFD  bl 0x831a8160
	ctx.lr = 0x82EC7568;
	sub_831A8130(ctx, base);
	// 82EC7568: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EC756C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EC7570: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82EC7574: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82EC7578: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82EC757C: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EC7580: 4BFDAB31  bl 0x82ea20b0
	ctx.lr = 0x82EC7584;
	sub_82EA20B0(ctx, base);
	// 82EC7584: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EC7588: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82EC758C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EC7590: 40990040  ble cr6, 0x82ec75d0
	if !ctx.cr[6].gt {
	pc = 0x82EC75D0; continue 'dispatch;
	}
	// 82EC7594: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82EC7598: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EC759C: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82EC75A0: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82EC75A4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82EC75A8: 7C7D582E  lwzx r3, r29, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EC75AC: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EC75B0: 812A0028  lwz r9, 0x28(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(40 as u32) ) } as u64;
	// 82EC75B4: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82EC75B8: 4E800421  bctrl
	ctx.lr = 0x82EC75BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EC75BC: 811F000C  lwz r8, 0xc(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EC75C0: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82EC75C4: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82EC75C8: 7F1E4000  cmpw cr6, r30, r8
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82EC75CC: 4198FFCC  blt cr6, 0x82ec7598
	if ctx.cr[6].lt {
	pc = 0x82EC7598; continue 'dispatch;
	}
	// 82EC75D0: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EC75D4: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82EC75D8: F9630020  std r11, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82EC75DC: 4837B381  bl 0x8324295c
	ctx.lr = 0x82EC75E0;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EC75E0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EC75E4: 482E0BCC  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EC75E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EC75E8 size=144
    let mut pc: u32 = 0x82EC75E8;
    'dispatch: loop {
        match pc {
            0x82EC75E8 => {
    //   block [0x82EC75E8..0x82EC7678)
	// 82EC75E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EC75EC: 482E0B71  bl 0x831a815c
	ctx.lr = 0x82EC75F0;
	sub_831A8130(ctx, base);
	// 82EC75F0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EC75F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EC75F8: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82EC75FC: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82EC7600: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82EC7604: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 82EC7608: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EC760C: 4BFDAAA5  bl 0x82ea20b0
	ctx.lr = 0x82EC7610;
	sub_82EA20B0(ctx, base);
	// 82EC7610: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EC7614: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82EC7618: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EC761C: 40990044  ble cr6, 0x82ec7660
	if !ctx.cr[6].gt {
	pc = 0x82EC7660; continue 'dispatch;
	}
	// 82EC7620: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82EC7624: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EC7628: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 82EC762C: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82EC7630: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82EC7634: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82EC7638: 7C7D582E  lwzx r3, r29, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EC763C: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EC7640: 812A002C  lwz r9, 0x2c(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(44 as u32) ) } as u64;
	// 82EC7644: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82EC7648: 4E800421  bctrl
	ctx.lr = 0x82EC764C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EC764C: 811F000C  lwz r8, 0xc(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EC7650: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82EC7654: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82EC7658: 7F1E4000  cmpw cr6, r30, r8
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82EC765C: 4198FFC8  blt cr6, 0x82ec7624
	if ctx.cr[6].lt {
	pc = 0x82EC7624; continue 'dispatch;
	}
	// 82EC7660: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EC7664: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82EC7668: F9630020  std r11, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82EC766C: 4837B2F1  bl 0x8324295c
	ctx.lr = 0x82EC7670;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EC7670: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EC7674: 482E0B38  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EC7678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EC7678 size=144
    let mut pc: u32 = 0x82EC7678;
    'dispatch: loop {
        match pc {
            0x82EC7678 => {
    //   block [0x82EC7678..0x82EC7708)
	// 82EC7678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EC767C: 482E0AE1  bl 0x831a815c
	ctx.lr = 0x82EC7680;
	sub_831A8130(ctx, base);
	// 82EC7680: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EC7684: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EC7688: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82EC768C: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82EC7690: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82EC7694: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 82EC7698: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EC769C: 4BFDAA15  bl 0x82ea20b0
	ctx.lr = 0x82EC76A0;
	sub_82EA20B0(ctx, base);
	// 82EC76A0: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EC76A4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82EC76A8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EC76AC: 40990044  ble cr6, 0x82ec76f0
	if !ctx.cr[6].gt {
	pc = 0x82EC76F0; continue 'dispatch;
	}
	// 82EC76B0: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82EC76B4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EC76B8: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 82EC76BC: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82EC76C0: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82EC76C4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82EC76C8: 7C7D582E  lwzx r3, r29, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EC76CC: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EC76D0: 812A0034  lwz r9, 0x34(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(52 as u32) ) } as u64;
	// 82EC76D4: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82EC76D8: 4E800421  bctrl
	ctx.lr = 0x82EC76DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EC76DC: 811F000C  lwz r8, 0xc(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EC76E0: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82EC76E4: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82EC76E8: 7F1E4000  cmpw cr6, r30, r8
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82EC76EC: 4198FFC8  blt cr6, 0x82ec76b4
	if ctx.cr[6].lt {
	pc = 0x82EC76B4; continue 'dispatch;
	}
	// 82EC76F0: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EC76F4: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82EC76F8: F9630020  std r11, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82EC76FC: 4837B261  bl 0x8324295c
	ctx.lr = 0x82EC7700;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EC7700: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EC7704: 482E0AA8  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EC7708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EC7708 size=136
    let mut pc: u32 = 0x82EC7708;
    'dispatch: loop {
        match pc {
            0x82EC7708 => {
    //   block [0x82EC7708..0x82EC7790)
	// 82EC7708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EC770C: 482E0A55  bl 0x831a8160
	ctx.lr = 0x82EC7710;
	sub_831A8130(ctx, base);
	// 82EC7710: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EC7714: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EC7718: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82EC771C: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82EC7720: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82EC7724: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EC7728: 4BFDA989  bl 0x82ea20b0
	ctx.lr = 0x82EC772C;
	sub_82EA20B0(ctx, base);
	// 82EC772C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EC7730: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82EC7734: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EC7738: 40990040  ble cr6, 0x82ec7778
	if !ctx.cr[6].gt {
	pc = 0x82EC7778; continue 'dispatch;
	}
	// 82EC773C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82EC7740: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EC7744: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82EC7748: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82EC774C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82EC7750: 7C7D582E  lwzx r3, r29, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EC7754: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EC7758: 812A0030  lwz r9, 0x30(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(48 as u32) ) } as u64;
	// 82EC775C: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82EC7760: 4E800421  bctrl
	ctx.lr = 0x82EC7764;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EC7764: 811F000C  lwz r8, 0xc(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EC7768: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82EC776C: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82EC7770: 7F1E4000  cmpw cr6, r30, r8
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82EC7774: 4198FFCC  blt cr6, 0x82ec7740
	if ctx.cr[6].lt {
	pc = 0x82EC7740; continue 'dispatch;
	}
	// 82EC7778: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EC777C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82EC7780: F9630020  std r11, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82EC7784: 4837B1D9  bl 0x8324295c
	ctx.lr = 0x82EC7788;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EC7788: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EC778C: 482E0A24  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EC7790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EC7790 size=264
    let mut pc: u32 = 0x82EC7790;
    'dispatch: loop {
        match pc {
            0x82EC7790 => {
    //   block [0x82EC7790..0x82EC7898)
	// 82EC7790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EC7794: 482E09D5  bl 0x831a8168
	ctx.lr = 0x82EC7798;
	sub_831A8130(ctx, base);
	// 82EC7798: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EC779C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82EC77A0: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EC77A4: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82EC77A8: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82EC77AC: 390B3A6C  addi r8, r11, 0x3a6c
	ctx.r[8].s64 = ctx.r[11].s64 + 14956;
	// 82EC77B0: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82EC77B4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82EC77B8: 911D0000  stw r8, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82EC77BC: 3CC08000  lis r6, -0x8000
	ctx.r[6].s64 = -2147483648;
	// 82EC77C0: B0FD0006  sth r7, 6(r29)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[29].u32.wrapping_add(6 as u32), ctx.r[7].u16 ) };
	// 82EC77C4: 93DD0008  stw r30, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82EC77C8: 93DD000C  stw r30, 0xc(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 82EC77CC: 90DD0010  stw r6, 0x10(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(16 as u32), ctx.r[6].u32 ) };
	// 82EC77D0: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82EC77D4: 83E30060  lwz r31, 0x60(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(96 as u32) ) } as u64;
	// 82EC77D8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EC77DC: 419A001C  beq cr6, 0x82ec77f8
	if ctx.cr[6].eq {
	pc = 0x82EC77F8; continue 'dispatch;
	}
	// 82EC77E0: 81630064  lwz r11, 0x64(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(100 as u32) ) } as u64;
	// 82EC77E4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82EC77E8: 91630064  stw r11, 0x64(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82EC77EC: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EC77F0: 91430060  stw r10, 0x60(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 82EC77F4: 48000014  b 0x82ec7808
	pc = 0x82EC7808; continue 'dispatch;
	// 82EC77F8: 38A00015  li r5, 0x15
	ctx.r[5].s64 = 21;
	// 82EC77FC: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 82EC7800: 4BFD8E61  bl 0x82ea0660
	ctx.lr = 0x82EC7804;
	sub_82EA0660(ctx, base);
	// 82EC7804: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EC7808: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EC780C: 419A007C  beq cr6, 0x82ec7888
	if ctx.cr[6].eq {
	pc = 0x82EC7888; continue 'dispatch;
	}
	// 82EC7810: 394003E8  li r10, 0x3e8
	ctx.r[10].s64 = 1000;
	// 82EC7814: 93DF0030  stw r30, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[30].u32 ) };
	// 82EC7818: 93DF002C  stw r30, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[30].u32 ) };
	// 82EC781C: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82EC7820: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 82EC7824: 3B9F0028  addi r28, r31, 0x28
	ctx.r[28].s64 = ctx.r[31].s64 + 40;
	// 82EC7828: 3BCB70B0  addi r30, r11, 0x70b0
	ctx.r[30].s64 = ctx.r[11].s64 + 28848;
	// 82EC782C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EC7830: 4BFDA881  bl 0x82ea20b0
	ctx.lr = 0x82EC7834;
	sub_82EA20B0(ctx, base);
	// 82EC7834: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 82EC7838: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 82EC783C: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EC7840: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EC7844: 419A0008  beq cr6, 0x82ec784c
	if ctx.cr[6].eq {
	pc = 0x82EC784C; continue 'dispatch;
	}
	// 82EC7848: 93EB002C  stw r31, 0x2c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(44 as u32), ctx.r[31].u32 ) };
	// 82EC784C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82EC7850: 93DC0004  stw r30, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82EC7854: 93FE0030  stw r31, 0x30(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(48 as u32), ctx.r[31].u32 ) };
	// 82EC7858: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EC785C: F97E0020  std r11, 0x20(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82EC7860: 4837B0FD  bl 0x8324295c
	ctx.lr = 0x82EC7864;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EC7864: 388003E8  li r4, 0x3e8
	ctx.r[4].s64 = 1000;
	// 82EC7868: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EC786C: 4837B5E1  bl 0x83242e4c
	ctx.lr = 0x82EC7870;
	// extern call 0x83242E4C → crate::xboxkrnl::RtlInitializeCriticalSectionAndSpinCount
	crate::xboxkrnl::RtlInitializeCriticalSectionAndSpinCount(ctx, base);
	// 82EC7870: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82EC7874: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EC7878: F97F0020  std r11, 0x20(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82EC787C: 93FD0014  stw r31, 0x14(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(20 as u32), ctx.r[31].u32 ) };
	// 82EC7880: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EC7884: 482E0934  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 82EC7888: 93DD0014  stw r30, 0x14(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 82EC788C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EC7890: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EC7894: 482E0924  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EC7898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EC7898 size=116
    let mut pc: u32 = 0x82EC7898;
    'dispatch: loop {
        match pc {
            0x82EC7898 => {
    //   block [0x82EC7898..0x82EC790C)
	// 82EC7898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EC789C: 482E08D1  bl 0x831a816c
	ctx.lr = 0x82EC78A0;
	sub_831A8130(ctx, base);
	// 82EC78A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EC78A4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EC78A8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82EC78AC: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EC78B0: 4BFDA801  bl 0x82ea20b0
	ctx.lr = 0x82EC78B4;
	sub_82EA20B0(ctx, base);
	// 82EC78B4: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EC78B8: 3BFE0008  addi r31, r30, 8
	ctx.r[31].s64 = ctx.r[30].s64 + 8;
	// 82EC78BC: 815E000C  lwz r10, 0xc(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EC78C0: 556900BE  clrlwi r9, r11, 2
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82EC78C4: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82EC78C8: 409A0010  bne cr6, 0x82ec78d8
	if !ctx.cr[6].eq {
	pc = 0x82EC78D8; continue 'dispatch;
	}
	// 82EC78CC: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82EC78D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EC78D4: 4BFDEFAD  bl 0x82ea6880
	ctx.lr = 0x82EC78D8;
	sub_82EA6880(ctx, base);
	// 82EC78D8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EC78DC: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82EC78E0: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EC78E4: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82EC78E8: 7FA8492E  stwx r29, r8, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[9].u32), ctx.r[29].u32) };
	// 82EC78EC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EC78F0: 38EB0001  addi r7, r11, 1
	ctx.r[7].s64 = ctx.r[11].s64 + 1;
	// 82EC78F4: 90FF0004  stw r7, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82EC78F8: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EC78FC: F9430020  std r10, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u64 ) };
	// 82EC7900: 4837B05D  bl 0x8324295c
	ctx.lr = 0x82EC7904;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82EC7904: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EC7908: 482E08B4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EC7910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EC7910 size=384
    let mut pc: u32 = 0x82EC7910;
    'dispatch: loop {
        match pc {
            0x82EC7910 => {
    //   block [0x82EC7910..0x82EC7A90)
	// 82EC7910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EC7914: 482E0855  bl 0x831a8168
	ctx.lr = 0x82EC7918;
	sub_831A8130(ctx, base);
	// 82EC7918: DBC1FFC8  stfd f30, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[30].u64 ) };
	// 82EC791C: DBE1FFD0  stfd f31, -0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 82EC7920: 9421FEF0  stwu r1, -0x110(r1)
	ea = ctx.r[1].u32.wrapping_add(-272 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EC7924: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82EC7928: FFC00890  fmr f30, f1
	ctx.f[30].f64 = ctx.f[1].f64;
	// 82EC792C: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82EC7930: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EC7934: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82EC7938: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EC7A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EC7A90 size=92
    let mut pc: u32 = 0x82EC7A90;
    'dispatch: loop {
        match pc {
            0x82EC7A90 => {
    //   block [0x82EC7A90..0x82EC7AEC)
	// 82EC7A90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EC7A94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EC7A98: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EC7A9C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EC7AA0: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82EC7AA4: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EC7AA8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EC7AAC: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82EC7AB0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EC7AB4: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82EC7AB8: 4BFE1469  bl 0x82ea8f20
	ctx.lr = 0x82EC7ABC;
	sub_82EA8F20(ctx, base);
	// 82EC7ABC: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82EC7AC0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82EC7AC4: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82EC7AC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EC7ACC: 4BFFFE45  bl 0x82ec7910
	ctx.lr = 0x82EC7AD0;
	sub_82EC7910(ctx, base);
	// 82EC7AD0: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82EC7AD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EC7AD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EC7ADC: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82EC7AE0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EC7AE4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EC7AE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EC7AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EC7AF0 size=64
    let mut pc: u32 = 0x82EC7AF0;
    'dispatch: loop {
        match pc {
            0x82EC7AF0 => {
    //   block [0x82EC7AF0..0x82EC7B30)
	// 82EC7AF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EC7AF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EC7AF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EC7AFC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EC7B00: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82EC7B04: 38A00035  li r5, 0x35
	ctx.r[5].s64 = 53;
	// 82EC7B08: 38800018  li r4, 0x18
	ctx.r[4].s64 = 24;
	// 82EC7B0C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EC7B10: 4BFD8C21  bl 0x82ea0730
	ctx.lr = 0x82EC7B14;
	sub_82EA0730(ctx, base);
	// 82EC7B14: 39200018  li r9, 0x18
	ctx.r[9].s64 = 24;
	// 82EC7B18: B1230004  sth r9, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[9].u16 ) };
	// 82EC7B1C: 4BFFFC75  bl 0x82ec7790
	ctx.lr = 0x82EC7B20;
	sub_82EC7790(ctx, base);
	// 82EC7B20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EC7B24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EC7B28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EC7B2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EC7B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EC7B30 size=128
    let mut pc: u32 = 0x82EC7B30;
    'dispatch: loop {
        match pc {
            0x82EC7B30 => {
    //   block [0x82EC7B30..0x82EC7BB0)
	// 82EC7B30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EC7B34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EC7B38: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EC7B3C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EC7B40: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EC7B44: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82EC7B48: 394B3A6C  addi r10, r11, 0x3a6c
	ctx.r[10].s64 = ctx.r[11].s64 + 14956;
	// 82EC7B4C: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EC7B50: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EC7B54: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EC7B58: 419A000C  beq cr6, 0x82ec7b64
	if ctx.cr[6].eq {
	pc = 0x82EC7B64; continue 'dispatch;
	}
	// 82EC7B5C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EC7B60: 48000051  bl 0x82ec7bb0
	ctx.lr = 0x82EC7B64;
	sub_82EC7BB0(ctx, base);
	// 82EC7B64: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EC7B68: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82EC7B6C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EC7B70: 409A0020  bne cr6, 0x82ec7b90
	if !ctx.cr[6].eq {
	pc = 0x82EC7B90; continue 'dispatch;
	}
	// 82EC7B74: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EC7B78: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82EC7B7C: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82EC7B80: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EC7B84: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82EC7B88: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82EC7B8C: 4BFD8C25  bl 0x82ea07b0
	ctx.lr = 0x82EC7B90;
	sub_82EA07B0(ctx, base);
	// 82EC7B90: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82EC7B94: 394B9EAC  addi r10, r11, -0x6154
	ctx.r[10].s64 = ctx.r[11].s64 + -24916;
	// 82EC7B98: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EC7B9C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82EC7BA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EC7BA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EC7BA8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EC7BAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EC7BB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EC7BB0 size=164
    let mut pc: u32 = 0x82EC7BB0;
    'dispatch: loop {
        match pc {
            0x82EC7BB0 => {
    //   block [0x82EC7BB0..0x82EC7C54)
	// 82EC7BB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EC7BB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EC7BB8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EC7BBC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EC7BC0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EC7BC4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EC7BC8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EC7BCC: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 82EC7BD0: 4BFDA5A9  bl 0x82ea2178
	ctx.lr = 0x82EC7BD4;
	sub_82EA2178(ctx, base);
	// 82EC7BD4: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82EC7BD8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EC7BDC: 419A005C  beq cr6, 0x82ec7c38
	if ctx.cr[6].eq {
	pc = 0x82EC7C38; continue 'dispatch;
	}
	// 82EC7BE0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EC7BE4: 419A0054  beq cr6, 0x82ec7c38
	if ctx.cr[6].eq {
	pc = 0x82EC7C38; continue 'dispatch;
	}
	// 82EC7BE8: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EC7BEC: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82EC7BF0: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EC7BF4: 812B0064  lwz r9, 0x64(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(100 as u32) ) } as u64;
	// 82EC7BF8: 810B0034  lwz r8, 0x34(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82EC7BFC: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82EC7C00: 4198001C  blt cr6, 0x82ec7c1c
	if ctx.cr[6].lt {
	pc = 0x82EC7C1C; continue 'dispatch;
	}
	// 82EC7C04: 38C00015  li r6, 0x15
	ctx.r[6].s64 = 21;
	// 82EC7C08: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82EC7C0C: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 82EC7C10: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82EC7C14: 4BFD844D  bl 0x82ea0060
	ctx.lr = 0x82EC7C18;
	sub_82EA0060(ctx, base);
	// 82EC7C18: 48000020  b 0x82ec7c38
	pc = 0x82EC7C38; continue 'dispatch;
	// 82EC7C1C: 812B0064  lwz r9, 0x64(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(100 as u32) ) } as u64;
	// 82EC7C20: 394B0060  addi r10, r11, 0x60
	ctx.r[10].s64 = ctx.r[11].s64 + 96;
	// 82EC7C24: 814B0060  lwz r10, 0x60(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(96 as u32) ) } as u64;
	// 82EC7C28: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82EC7C2C: 912B0064  stw r9, 0x64(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(100 as u32), ctx.r[9].u32 ) };
	// 82EC7C30: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EC7C34: 93EB0060  stw r31, 0x60(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(96 as u32), ctx.r[31].u32 ) };
	// 82EC7C38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EC7C3C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EC7C40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EC7C44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EC7C48: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EC7C4C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EC7C50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EC7C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EC7C58 size=100
    let mut pc: u32 = 0x82EC7C58;
    'dispatch: loop {
        match pc {
            0x82EC7C58 => {
    //   block [0x82EC7C58..0x82EC7CBC)
	// 82EC7C58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EC7C5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EC7C60: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EC7C64: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EC7C68: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EC7C6C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EC7C70: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EC7C74: 4BFFFEBD  bl 0x82ec7b30
	ctx.lr = 0x82EC7C78;
	sub_82EC7B30(ctx, base);
	// 82EC7C78: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82EC7C7C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EC7C80: 419A0020  beq cr6, 0x82ec7ca0
	if ctx.cr[6].eq {
	pc = 0x82EC7CA0; continue 'dispatch;
	}
	// 82EC7C84: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EC7C88: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82EC7C8C: 38C00035  li r6, 0x35
	ctx.r[6].s64 = 53;
	// 82EC7C90: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EC7C94: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EC7C98: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EC7C9C: 4BFD8B15  bl 0x82ea07b0
	ctx.lr = 0x82EC7CA0;
	sub_82EA07B0(ctx, base);
	// 82EC7CA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EC7CA4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EC7CA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EC7CAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EC7CB0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EC7CB4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EC7CB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EC7CC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EC7CC0 size=40
    let mut pc: u32 = 0x82EC7CC0;
    'dispatch: loop {
        match pc {
            0x82EC7CC0 => {
    //   block [0x82EC7CC0..0x82EC7CE8)
	// 82EC7CC0: 54CA442E  rlwinm r10, r6, 8, 0x10, 0x17
	ctx.r[10].u64 = ctx.r[6].u32 as u64 & 0x00FFFFFFu64;
	// 82EC7CC4: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82EC7CC8: 5489063E  clrlwi r9, r4, 0x18
	ctx.r[9].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 82EC7CCC: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82EC7CD0: 54AA063E  clrlwi r10, r5, 0x18
	ctx.r[10].u64 = ctx.r[5].u32 as u64 & 0x000000FFu64;
	// 82EC7CD4: 556B402E  slwi r11, r11, 8
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(8);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EC7CD8: 7D2B4A14  add r9, r11, r9
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82EC7CDC: 552B402E  slwi r11, r9, 8
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(8);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EC7CE0: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82EC7CE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EC7CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82EC7CE8 size=128
    let mut pc: u32 = 0x82EC7CE8;
    'dispatch: loop {
        match pc {
            0x82EC7CE8 => {
    //   block [0x82EC7CE8..0x82EC7D68)
	// 82EC7CE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EC7CEC: 482E0481  bl 0x831a816c
	ctx.lr = 0x82EC7CF0;
	sub_831A8130(ctx, base);
	// 82EC7CF0: 3981FFE0  addi r12, r1, -0x20
	ctx.r[12].s64 = ctx.r[1].s64 + -32;
	// 82EC7CF4: 482E0D85  bl 0x831a8a78
	ctx.lr = 0x82EC7CF8;
	sub_831A8A40(ctx, base);
	// 82EC7CF8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EC7CFC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82EC7D00: FFC00890  fmr f30, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].f64 = ctx.f[1].f64;
	// 82EC7D04: FFA01090  fmr f29, f2
	ctx.f[29].f64 = ctx.f[2].f64;
	// 82EC7D08: FF801890  fmr f28, f3
	ctx.f[28].f64 = ctx.f[3].f64;
	// 82EC7D0C: C3EBC3C8  lfs f31, -0x3c38(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-15416 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82EC7D10: EC2407F2  fmuls f1, f4, f31
	ctx.f[1].f64 = (((ctx.f[4].f64 * ctx.f[31].f64) as f32) as f64);
	// 82EC7D14: 4BFDEE45  bl 0x82ea6b58
	ctx.lr = 0x82EC7D18;
	sub_82EA6B58(ctx, base);
	// 82EC7D18: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82EC7D1C: EC3E07F2  fmuls f1, f30, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = (((ctx.f[30].f64 * ctx.f[31].f64) as f32) as f64);
	// 82EC7D20: 4BFDEE39  bl 0x82ea6b58
	ctx.lr = 0x82EC7D24;
	sub_82EA6B58(ctx, base);
	// 82EC7D24: 547F063E  clrlwi r31, r3, 0x18
	ctx.r[31].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82EC7D28: EC3D07F2  fmuls f1, f29, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = (((ctx.f[29].f64 * ctx.f[31].f64) as f32) as f64);
	// 82EC7D2C: 4BFDEE2D  bl 0x82ea6b58
	ctx.lr = 0x82EC7D30;
	sub_82EA6B58(ctx, base);
	// 82EC7D30: 547E063E  clrlwi r30, r3, 0x18
	ctx.r[30].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82EC7D34: EC3C07F2  fmuls f1, f28, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = (((ctx.f[28].f64 * ctx.f[31].f64) as f32) as f64);
	// 82EC7D38: 4BFDEE21  bl 0x82ea6b58
	ctx.lr = 0x82EC7D3C;
	sub_82EA6B58(ctx, base);
	// 82EC7D3C: 57AB442E  rlwinm r11, r29, 8, 0x10, 0x17
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x00FFFFFFu64;
	// 82EC7D40: 546A063E  clrlwi r10, r3, 0x18
	ctx.r[10].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82EC7D44: 7D2BFA14  add r9, r11, r31
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82EC7D48: 552B402E  slwi r11, r9, 8
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(8);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EC7D4C: 7D0BF214  add r8, r11, r30
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82EC7D50: 550B402E  slwi r11, r8, 8
	ctx.r[11].u32 = ctx.r[8].u32.wrapping_shl(8);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EC7D54: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82EC7D58: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EC7D5C: 3981FFE0  addi r12, r1, -0x20
	ctx.r[12].s64 = ctx.r[1].s64 + -32;
	// 82EC7D60: 482E0D65  bl 0x831a8ac4
	ctx.lr = 0x82EC7D64;
	sub_831A8A8C(ctx, base);
	// 82EC7D64: 482E0458  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EC7D68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82EC7D68 size=88
    let mut pc: u32 = 0x82EC7D68;
    'dispatch: loop {
        match pc {
            0x82EC7D68 => {
    //   block [0x82EC7D68..0x82EC7DC0)
	// 82EC7D68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EC7D6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EC7D70: DBC1FFE8  stfd f30, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[30].u64 ) };
	// 82EC7D74: DBE1FFF0  stfd f31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[31].u64 ) };
	// 82EC7D78: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EC7D7C: 4BFDEC4D  bl 0x82ea69c8
	ctx.lr = 0x82EC7D80;
	sub_82EA69C8(ctx, base);
	// 82EC7D80: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82EC7D84: 4BFDEC45  bl 0x82ea69c8
	ctx.lr = 0x82EC7D88;
	sub_82EA69C8(ctx, base);
	// 82EC7D88: FFC00890  fmr f30, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].f64 = ctx.f[1].f64;
	// 82EC7D8C: 4BFDEC3D  bl 0x82ea69c8
	ctx.lr = 0x82EC7D90;
	sub_82EA69C8(ctx, base);
	// 82EC7D90: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82EC7D94: FC600890  fmr f3, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].f64 = ctx.f[1].f64;
	// 82EC7D98: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 82EC7D9C: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82EC7DA0: C08B08A8  lfs f4, 0x8a8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 82EC7DA4: 4BFFFF45  bl 0x82ec7ce8
	ctx.lr = 0x82EC7DA8;
	sub_82EC7CE8(ctx, base);
	// 82EC7DA8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EC7DAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EC7DB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EC7DB4: CBC1FFE8  lfd f30, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EC7DB8: CBE1FFF0  lfd f31, -0x10(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EC7DBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EC7DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EC7DC0 size=4
    let mut pc: u32 = 0x82EC7DC0;
    'dispatch: loop {
        match pc {
            0x82EC7DC0 => {
    //   block [0x82EC7DC0..0x82EC7DC4)
	// 82EC7DC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EC7DC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EC7DC8 size=4
    let mut pc: u32 = 0x82EC7DC8;
    'dispatch: loop {
        match pc {
            0x82EC7DC8 => {
    //   block [0x82EC7DC8..0x82EC7DCC)
	// 82EC7DC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EC7DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EC7DD0 size=4
    let mut pc: u32 = 0x82EC7DD0;
    'dispatch: loop {
        match pc {
            0x82EC7DD0 => {
    //   block [0x82EC7DD0..0x82EC7DD4)
	// 82EC7DD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EC7DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EC7DD8 size=20
    let mut pc: u32 = 0x82EC7DD8;
    'dispatch: loop {
        match pc {
            0x82EC7DD8 => {
    //   block [0x82EC7DD8..0x82EC7DEC)
	// 82EC7DD8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EC7DDC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82EC7DE0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EC7DE4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EC7DE8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EC7DF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EC7DF0 size=8
    let mut pc: u32 = 0x82EC7DF0;
    'dispatch: loop {
        match pc {
            0x82EC7DF0 => {
    //   block [0x82EC7DF0..0x82EC7DF8)
	// 82EC7DF0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82EC7DF4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EC7DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EC7DF8 size=24
    let mut pc: u32 = 0x82EC7DF8;
    'dispatch: loop {
        match pc {
            0x82EC7DF8 => {
    //   block [0x82EC7DF8..0x82EC7E10)
	// 82EC7DF8: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82EC7DFC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82EC7E00: 392B3ED4  addi r9, r11, 0x3ed4
	ctx.r[9].s64 = ctx.r[11].s64 + 16084;
	// 82EC7E04: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82EC7E08: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EC7E0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EC7E10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EC7E10 size=12
    let mut pc: u32 = 0x82EC7E10;
    'dispatch: loop {
        match pc {
            0x82EC7E10 => {
    //   block [0x82EC7E10..0x82EC7E1C)
	// 82EC7E10: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82EC7E14: 386B3ED4  addi r3, r11, 0x3ed4
	ctx.r[3].s64 = ctx.r[11].s64 + 16084;
	// 82EC7E18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EC7E20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EC7E20 size=100
    let mut pc: u32 = 0x82EC7E20;
    'dispatch: loop {
        match pc {
            0x82EC7E20 => {
    //   block [0x82EC7E20..0x82EC7E84)
	// 82EC7E20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EC7E24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EC7E28: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EC7E2C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EC7E30: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EC7E34: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EC7E38: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EC7E3C: 48000A45  bl 0x82ec8880
	ctx.lr = 0x82EC7E40;
	sub_82EC8880(ctx, base);
	// 82EC7E40: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82EC7E44: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EC7E48: 419A0020  beq cr6, 0x82ec7e68
	if ctx.cr[6].eq {
	pc = 0x82EC7E68; continue 'dispatch;
	}
	// 82EC7E4C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EC7E50: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82EC7E54: 38C0000E  li r6, 0xe
	ctx.r[6].s64 = 14;
	// 82EC7E58: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EC7E5C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EC7E60: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EC7E64: 4BFD894D  bl 0x82ea07b0
	ctx.lr = 0x82EC7E68;
	sub_82EA07B0(ctx, base);
	// 82EC7E68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EC7E6C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EC7E70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EC7E74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EC7E78: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EC7E7C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EC7E80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EC7E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EC7E88 size=80
    let mut pc: u32 = 0x82EC7E88;
    'dispatch: loop {
        match pc {
            0x82EC7E88 => {
    //   block [0x82EC7E88..0x82EC7ED8)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EC7ED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EC7ED8 size=24
    let mut pc: u32 = 0x82EC7ED8;
    'dispatch: loop {
        match pc {
            0x82EC7ED8 => {
    //   block [0x82EC7ED8..0x82EC7EF0)
	// 82EC7ED8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82EC7EDC: 394BC5B0  addi r10, r11, -0x3a50
	ctx.r[10].s64 = ctx.r[11].s64 + -14928;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EC7EF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EC7EF0 size=196
    let mut pc: u32 = 0x82EC7EF0;
    'dispatch: loop {
        match pc {
            0x82EC7EF0 => {
    //   block [0x82EC7EF0..0x82EC7FB4)
	// 82EC7EF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EC7EF4: 482E0269  bl 0x831a815c
	ctx.lr = 0x82EC7EF8;
	sub_831A8130(ctx, base);
	// 82EC7EF8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EC7EFC: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82EC7F00: 8179002C  lwz r11, 0x2c(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(44 as u32) ) } as u64;
	// 82EC7F04: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EC7F08: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EC7F0C: 409900A0  ble cr6, 0x82ec7fac
	if !ctx.cr[6].gt {
	pc = 0x82EC7FAC; continue 'dispatch;
	}
	// 82EC7F10: 7D7B5B78  mr r27, r11
	ctx.r[27].u64 = ctx.r[11].u64;
	// 82EC7F14: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82EC7F18: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82EC7F1C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82EC7F20: 3B4B7060  addi r26, r11, 0x7060
	ctx.r[26].s64 = ctx.r[11].s64 + 28768;
	// 82EC7F24: 81790020  lwz r11, 0x20(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(32 as u32) ) } as u64;
	// 82EC7F28: 7D7C582E  lwzx r11, r28, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EC7F2C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EC7F30: 4198006C  blt cr6, 0x82ec7f9c
	if ctx.cr[6].lt {
	pc = 0x82EC7F9C; continue 'dispatch;
	}
	// 82EC7F34: 81590008  lwz r10, 8(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EC7F38: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82EC7F3C: 7FE9502E  lwzx r31, r9, r10
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82EC7F40: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EC7F44: 419A0058  beq cr6, 0x82ec7f9c
	if ctx.cr[6].eq {
	pc = 0x82EC7F9C; continue 'dispatch;
	}
	// 82EC7F48: 38BD0010  addi r5, r29, 0x10
	ctx.r[5].s64 = ctx.r[29].s64 + 16;
	// 82EC7F4C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82EC7F50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EC7F54: 480040FD  bl 0x82ecc050
	ctx.lr = 0x82EC7F58;
	sub_82ECC050(ctx, base);
	// 82EC7F58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EC7F5C: 48010D0D  bl 0x82ed8c68
	ctx.lr = 0x82EC7F60;
	sub_82ED8C68(ctx, base);
	// 82EC7F60: 817F00D0  lwz r11, 0xd0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(208 as u32) ) } as u64;
	// 82EC7F64: 3BDF00D0  addi r30, r31, 0xd0
	ctx.r[30].s64 = ctx.r[31].s64 + 208;
	// 82EC7F68: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82EC7F6C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EC7F70: 814B0040  lwz r10, 0x40(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 82EC7F74: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EC7F78: 4E800421  bctrl
	ctx.lr = 0x82EC7F7C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EC7F7C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EC7F80: 48010CE9  bl 0x82ed8c68
	ctx.lr = 0x82EC7F84;
	sub_82ED8C68(ctx, base);
	// 82EC7F84: 813F00D0  lwz r9, 0xd0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(208 as u32) ) } as u64;
	// 82EC7F88: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82EC7F8C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EC7F90: 81090044  lwz r8, 0x44(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(68 as u32) ) } as u64;
	// 82EC7F94: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82EC7F98: 4E800421  bctrl
	ctx.lr = 0x82EC7F9C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EC7F9C: 377BFFFF  addic. r27, r27, -1
	ctx.xer.ca = (ctx.r[27].u32 > (!(-1 as u32)));
	ctx.r[27].s64 = ctx.r[27].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 82EC7FA0: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 82EC7FA4: 3BBD0030  addi r29, r29, 0x30
	ctx.r[29].s64 = ctx.r[29].s64 + 48;
	// 82EC7FA8: 4082FF7C  bne 0x82ec7f24
	if !ctx.cr[0].eq {
	pc = 0x82EC7F24; continue 'dispatch;
	}
	// 82EC7FAC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EC7FB0: 482E01FC  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EC7FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EC7FB8 size=212
    let mut pc: u32 = 0x82EC7FB8;
    'dispatch: loop {
        match pc {
            0x82EC7FB8 => {
    //   block [0x82EC7FB8..0x82EC808C)
	// 82EC7FB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EC7FBC: 482E01A9  bl 0x831a8164
	ctx.lr = 0x82EC7FC0;
	sub_831A8130(ctx, base);
	// 82EC7FC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EC7FC4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EC7FC8: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82EC7FCC: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82EC7FD0: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EC7FD4: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EC7FD8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EC7FDC: 419A001C  beq cr6, 0x82ec7ff8
	if ctx.cr[6].eq {
	pc = 0x82EC7FF8; continue 'dispatch;
	}
	// 82EC7FE0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EC7FE4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EC7FE8: 419A0010  beq cr6, 0x82ec7ff8
	if ctx.cr[6].eq {
	pc = 0x82EC7FF8; continue 'dispatch;
	}
	// 82EC7FEC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82EC7FF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EC7FF4: 482E01C0  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 82EC7FF8: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82EC7FFC: 80BF000C  lwz r5, 0xc(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EC8000: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EC8004: 4800AE1D  bl 0x82ed2e20
	ctx.lr = 0x82EC8008;
	sub_82ED2E20(ctx, base);
	// 82EC8008: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EC800C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82EC8010: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EC8014: 4099002C  ble cr6, 0x82ec8040
	if !ctx.cr[6].gt {
	pc = 0x82EC8040; continue 'dispatch;
	}
	// 82EC8018: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82EC801C: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EC8020: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EC8024: 7C8BE82E  lwzx r4, r11, r29
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82EC8028: 4800B941  bl 0x82ed3968
	ctx.lr = 0x82EC802C;
	sub_82ED3968(ctx, base);
	// 82EC802C: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EC8030: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82EC8034: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82EC8038: 7F1E5000  cmpw cr6, r30, r10
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82EC803C: 4198FFE0  blt cr6, 0x82ec801c
	if ctx.cr[6].lt {
	pc = 0x82EC801C; continue 'dispatch;
	}
	// 82EC8040: 7F6B0774  extsb r11, r27
	ctx.r[11].s64 = ctx.r[27].s8 as i64;
	// 82EC8044: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EC8048: 419A0038  beq cr6, 0x82ec8080
	if ctx.cr[6].eq {
	pc = 0x82EC8080; continue 'dispatch;
	}
	// 82EC804C: 83DF000C  lwz r30, 0xc(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EC8050: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82EC8054: 4099002C  ble cr6, 0x82ec8080
	if !ctx.cr[6].gt {
	pc = 0x82EC8080; continue 'dispatch;
	}
	// 82EC8058: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82EC805C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EC8060: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82EC8064: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82EC8068: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EC806C: 7C9D582E  lwzx r4, r29, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EC8070: 48009AA9  bl 0x82ed1b18
	ctx.lr = 0x82EC8074;
	sub_82ED1B18(ctx, base);
	// 82EC8074: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82EC8078: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82EC807C: 4082FFE0  bne 0x82ec805c
	if !ctx.cr[0].eq {
	pc = 0x82EC805C; continue 'dispatch;
	}
	// 82EC8080: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EC8084: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EC8088: 482E012C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EC8090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EC8090 size=144
    let mut pc: u32 = 0x82EC8090;
    'dispatch: loop {
        match pc {
            0x82EC8090 => {
    //   block [0x82EC8090..0x82EC8120)
	// 82EC8090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EC8094: 482E00D5  bl 0x831a8168
	ctx.lr = 0x82EC8098;
	sub_831A8130(ctx, base);
	// 82EC8098: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EC809C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82EC80A0: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EC80A4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EC80A8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EC80AC: 419A0010  beq cr6, 0x82ec80bc
	if ctx.cr[6].eq {
	pc = 0x82EC80BC; continue 'dispatch;
	}
	// 82EC80B0: 838B0008  lwz r28, 8(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EC80B4: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82EC80B8: 409A0010  bne cr6, 0x82ec80c8
	if !ctx.cr[6].eq {
	pc = 0x82EC80C8; continue 'dispatch;
	}
	// 82EC80BC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82EC80C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EC80C4: 482E00F4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 82EC80C8: 817D0018  lwz r11, 0x18(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EC80CC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82EC80D0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EC80D4: 40990030  ble cr6, 0x82ec8104
	if !ctx.cr[6].gt {
	pc = 0x82EC8104; continue 'dispatch;
	}
	// 82EC80D8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82EC80DC: 817D0014  lwz r11, 0x14(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EC80E0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82EC80E4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EC80E8: 7CABF82E  lwzx r5, r11, r31
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82EC80EC: 4800BA3D  bl 0x82ed3b28
	ctx.lr = 0x82EC80F0;
	sub_82ED3B28(ctx, base);
	// 82EC80F0: 815D0018  lwz r10, 0x18(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EC80F4: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82EC80F8: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82EC80FC: 7F1E5000  cmpw cr6, r30, r10
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82EC8100: 4198FFDC  blt cr6, 0x82ec80dc
	if ctx.cr[6].lt {
	pc = 0x82EC80DC; continue 'dispatch;
	}
	// 82EC8104: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82EC8108: 80BD000C  lwz r5, 0xc(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EC810C: 809D0008  lwz r4, 8(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EC8110: 4800B429  bl 0x82ed3538
	ctx.lr = 0x82EC8114;
	sub_82ED3538(ctx, base);
	// 82EC8114: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82EC8118: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EC811C: 482E009C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EC8120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EC8120 size=444
    let mut pc: u32 = 0x82EC8120;
    'dispatch: loop {
        match pc {
            0x82EC8120 => {
    //   block [0x82EC8120..0x82EC82DC)
	// 82EC8120: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EC8124: 482E0049  bl 0x831a816c
	ctx.lr = 0x82EC8128;
	sub_831A8130(ctx, base);
	// 82EC8128: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EC812C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EC8130: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82EC8134: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82EC8138: 57AA103A  slwi r10, r29, 2
	ctx.r[10].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EC813C: 817E0020  lwz r11, 0x20(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 82EC8140: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EC8144: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EC8148: 41980040  blt cr6, 0x82ec8188
	if ctx.cr[6].lt {
	pc = 0x82EC8188; continue 'dispatch;
	}
	// 82EC814C: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EC8150: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82EC8154: 7D69502E  lwzx r11, r9, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82EC8158: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EC815C: 419A002C  beq cr6, 0x82ec8188
	if ctx.cr[6].eq {
	pc = 0x82EC8188; continue 'dispatch;
	}
	// 82EC8160: 388B00E0  addi r4, r11, 0xe0
	ctx.r[4].s64 = ctx.r[11].s64 + 224;
	// 82EC8164: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EC8168: 4BFE0A29  bl 0x82ea8b90
	ctx.lr = 0x82EC816C;
	sub_82EA8B90(ctx, base);
	// 82EC816C: 807E002C  lwz r3, 0x2c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 82EC8170: 38DF0020  addi r6, r31, 0x20
	ctx.r[6].s64 = ctx.r[31].s64 + 32;
	// 82EC8174: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82EC8178: 80830014  lwz r4, 0x14(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EC817C: 480E4E8D  bl 0x82fad008
	ctx.lr = 0x82EC8180;
	sub_82FAD008(ctx, base);
	// 82EC8180: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82EC8184: 482E0038  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 82EC8188: 817E002C  lwz r11, 0x2c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 82EC818C: 57AA083C  slwi r10, r29, 1
	ctx.r[10].u32 = ctx.r[29].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EC8190: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EC8194: 7D09522E  lhzx r8, r9, r10
	ctx.r[8].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82EC8198: 7D040734  extsh r4, r8
	ctx.r[4].s64 = ctx.r[8].s16 as i64;
	// 82EC819C: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82EC81A0: 4198001C  blt cr6, 0x82ec81bc
	if ctx.cr[6].lt {
	pc = 0x82EC81BC; continue 'dispatch;
	}
	// 82EC81A4: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82EC81A8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EC81AC: 4BFFFF75  bl 0x82ec8120
	ctx.lr = 0x82EC81B0;
	sub_82EC8120(ctx, base);
	// 82EC81B0: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EC82E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EC82E0 size=456
    let mut pc: u32 = 0x82EC82E0;
    'dispatch: loop {
        match pc {
            0x82EC82E0 => {
    //   block [0x82EC82E0..0x82EC84A8)
	// 82EC82E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EC82E4: 482DFE89  bl 0x831a816c
	ctx.lr = 0x82EC82E8;
	sub_831A8130(ctx, base);
	// 82EC82E8: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EC82EC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82EC82F0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82EC82F4: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82EC82F8: 57AA103A  slwi r10, r29, 2
	ctx.r[10].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EC82FC: 817E0020  lwz r11, 0x20(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 82EC8300: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EC8304: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EC8308: 4198004C  blt cr6, 0x82ec8354
	if ctx.cr[6].lt {
	pc = 0x82EC8354; continue 'dispatch;
	}
	// 82EC830C: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EC8310: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82EC8314: 7D69502E  lwzx r11, r9, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82EC8318: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EC831C: 419A0038  beq cr6, 0x82ec8354
	if ctx.cr[6].eq {
	pc = 0x82EC8354; continue 'dispatch;
	}
	// 82EC8320: 38A10080  addi r5, r1, 0x80
	ctx.r[5].s64 = ctx.r[1].s64 + 128;
	// 82EC8324: 386B00D0  addi r3, r11, 0xd0
	ctx.r[3].s64 = ctx.r[11].s64 + 208;
	// 82EC8328: 48012291  bl 0x82eda5b8
	ctx.lr = 0x82EC832C;
	sub_82EDA5B8(ctx, base);
	// 82EC832C: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 82EC8330: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EC8334: 4BFE085D  bl 0x82ea8b90
	ctx.lr = 0x82EC8338;
	sub_82EA8B90(ctx, base);
	// 82EC8338: 807E002C  lwz r3, 0x2c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 82EC833C: 38DF0020  addi r6, r31, 0x20
	ctx.r[6].s64 = ctx.r[31].s64 + 32;
	// 82EC8340: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82EC8344: 80830014  lwz r4, 0x14(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EC8348: 480E4CC1  bl 0x82fad008
	ctx.lr = 0x82EC834C;
	sub_82FAD008(ctx, base);
	// 82EC834C: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 82EC8350: 482DFE6C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 82EC8354: 817E002C  lwz r11, 0x2c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 82EC8358: 57AA083C  slwi r10, r29, 1
	ctx.r[10].u32 = ctx.r[29].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EC835C: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EC8360: 7D09522E  lhzx r8, r9, r10
	ctx.r[8].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82EC8364: 7D040734  extsh r4, r8
	ctx.r[4].s64 = ctx.r[8].s16 as i64;
	// 82EC8368: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82EC836C: 4198001C  blt cr6, 0x82ec8388
	if ctx.cr[6].lt {
	pc = 0x82EC8388; continue 'dispatch;
	}
	// 82EC8370: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82EC8374: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EC8378: 4BFFFF69  bl 0x82ec82e0
	ctx.lr = 0x82EC837C;
	sub_82EC82E0(ctx, base);
	// 82EC837C: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EC84A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EC84A8 size=512
    let mut pc: u32 = 0x82EC84A8;
    'dispatch: loop {
        match pc {
            0x82EC84A8 => {
    //   block [0x82EC84A8..0x82EC86A8)
	// 82EC84A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EC84AC: 482DFCB9  bl 0x831a8164
	ctx.lr = 0x82EC84B0;
	sub_831A8130(ctx, base);
	// 82EC84B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EC84B4: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82EC84B8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82EC84BC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82EC84C0: 3BBB0008  addi r29, r27, 8
	ctx.r[29].s64 = ctx.r[27].s64 + 8;
	// 82EC84C4: 90DB002C  stw r6, 0x2c(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(44 as u32), ctx.r[6].u32 ) };
	// 82EC84C8: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EC84CC: 817B0010  lwz r11, 0x10(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EC84D0: 556A00BE  clrlwi r10, r11, 2
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82EC84D4: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82EC84D8: 40980060  bge cr6, 0x82ec8538
	if !ctx.cr[6].lt {
	pc = 0x82EC8538; continue 'dispatch;
	}
	// 82EC84DC: 556B0000  rlwinm r11, r11, 0, 0, 0
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82EC84E0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EC84E4: 409A0020  bne cr6, 0x82ec8504
	if !ctx.cr[6].eq {
	pc = 0x82EC8504; continue 'dispatch;
	}
	// 82EC84E8: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EC84EC: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82EC84F0: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82EC84F4: 809D0000  lwz r4, 0(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EC84F8: 5545103A  slwi r5, r10, 2
	ctx.r[5].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82EC84FC: 7C69582E  lwzx r3, r9, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EC8500: 4BFD82B1  bl 0x82ea07b0
	ctx.lr = 0x82EC8504;
	sub_82EA07B0(ctx, base);
	// 82EC8504: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EC8508: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82EC850C: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EC8510: 38A00017  li r5, 0x17
	ctx.r[5].s64 = 23;
	// 82EC8514: 5524103A  slwi r4, r9, 2
	ctx.r[4].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82EC8518: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EC851C: 4BFD8215  bl 0x82ea0730
	ctx.lr = 0x82EC8520;
	sub_82EA0730(ctx, base);
	// 82EC8520: 811D0008  lwz r8, 8(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EC8524: 907D0000  stw r3, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82EC8528: 55070042  rlwinm r7, r8, 0, 1, 1
	ctx.r[7].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	// 82EC852C: 80DF0004  lwz r6, 4(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EC8530: 7CE53378  or r5, r7, r6
	ctx.r[5].u64 = ctx.r[7].u64 | ctx.r[6].u64;
	// 82EC8534: 90BD0008  stw r5, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 82EC8538: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EC853C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EC8540: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EC8544: 915D0004  stw r10, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82EC8548: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EC854C: 4099001C  ble cr6, 0x82ec8568
	if !ctx.cr[6].gt {
	pc = 0x82EC8568; continue 'dispatch;
	}
	// 82EC8550: 7D2B4850  subf r9, r11, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 82EC8554: 7D09582E  lwzx r8, r9, r11
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EC8558: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EC855C: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82EC8560: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82EC8564: 4082FFF0  bne 0x82ec8554
	if !ctx.cr[0].eq {
	pc = 0x82EC8554; continue 'dispatch;
	}
	// 82EC8568: 817B001C  lwz r11, 0x1c(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EC856C: 3B9B0014  addi r28, r27, 0x14
	ctx.r[28].s64 = ctx.r[27].s64 + 20;
	// 82EC8570: 813E0004  lwz r9, 4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EC8574: 556A00BE  clrlwi r10, r11, 2
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82EC8578: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82EC857C: 40980060  bge cr6, 0x82ec85dc
	if !ctx.cr[6].lt {
	pc = 0x82EC85DC; continue 'dispatch;
	}
	// 82EC8580: 556B0000  rlwinm r11, r11, 0, 0, 0
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82EC8584: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EC8588: 409A0020  bne cr6, 0x82ec85a8
	if !ctx.cr[6].eq {
	pc = 0x82EC85A8; continue 'dispatch;
	}
	// 82EC858C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EC8590: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82EC8594: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82EC8598: 809C0000  lwz r4, 0(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EC859C: 5545103A  slwi r5, r10, 2
	ctx.r[5].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82EC85A0: 7C69582E  lwzx r3, r9, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EC85A4: 4BFD820D  bl 0x82ea07b0
	ctx.lr = 0x82EC85A8;
	sub_82EA07B0(ctx, base);
	// 82EC85A8: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EC85AC: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82EC85B0: 813E0004  lwz r9, 4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EC85B4: 38A00017  li r5, 0x17
	ctx.r[5].s64 = 23;
	// 82EC85B8: 5524103A  slwi r4, r9, 2
	ctx.r[4].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82EC85BC: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EC85C0: 4BFD8171  bl 0x82ea0730
	ctx.lr = 0x82EC85C4;
	sub_82EA0730(ctx, base);
	// 82EC85C4: 811C0008  lwz r8, 8(r28)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EC85C8: 907C0000  stw r3, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82EC85CC: 55070042  rlwinm r7, r8, 0, 1, 1
	ctx.r[7].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	// 82EC85D0: 80DE0004  lwz r6, 4(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EC85D4: 7CE53378  or r5, r7, r6
	ctx.r[5].u64 = ctx.r[7].u64 | ctx.r[6].u64;
	// 82EC85D8: 90BC0008  stw r5, 8(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 82EC85DC: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EC85E0: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EC85E4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EC85E8: 915C0004  stw r10, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82EC85EC: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EC85F0: 4099001C  ble cr6, 0x82ec860c
	if !ctx.cr[6].gt {
	pc = 0x82EC860C; continue 'dispatch;
	}
	// 82EC85F4: 7D2B4850  subf r9, r11, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 82EC85F8: 7D09582E  lwzx r8, r9, r11
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EC85FC: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EC8600: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82EC8604: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82EC8608: 4082FFF0  bne 0x82ec85f8
	if !ctx.cr[0].eq {
	pc = 0x82EC85F8; continue 'dispatch;
	}
	// 82EC860C: 817B000C  lwz r11, 0xc(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EC8610: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82EC8614: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EC8618: 40990034  ble cr6, 0x82ec864c
	if !ctx.cr[6].gt {
	pc = 0x82EC864C; continue 'dispatch;
	}
	// 82EC861C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82EC8620: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EC8624: 7D5F582E  lwzx r10, r31, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EC8628: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EC862C: 419A000C  beq cr6, 0x82ec8638
	if ctx.cr[6].eq {
	pc = 0x82EC8638; continue 'dispatch;
	}
	// 82EC8630: 5543003E  slwi r3, r10, 0
	ctx.r[3].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82EC8634: 4800CAB5  bl 0x82ed50e8
	ctx.lr = 0x82EC8638;
	sub_82ED50E8(ctx, base);
	// 82EC8638: 817B000C  lwz r11, 0xc(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EC863C: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82EC8640: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82EC8644: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82EC8648: 4198FFD8  blt cr6, 0x82ec8620
	if ctx.cr[6].lt {
	pc = 0x82EC8620; continue 'dispatch;
	}
	// 82EC864C: 817B0018  lwz r11, 0x18(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EC8650: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82EC8654: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EC8658: 40990048  ble cr6, 0x82ec86a0
	if !ctx.cr[6].gt {
	pc = 0x82EC86A0; continue 'dispatch;
	}
	// 82EC865C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EC8660: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EC8664: 7D2B502E  lwzx r9, r11, r10
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82EC8668: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82EC866C: 419A0020  beq cr6, 0x82ec868c
	if ctx.cr[6].eq {
	pc = 0x82EC868C; continue 'dispatch;
	}
	// 82EC8670: 552A003E  slwi r10, r9, 0
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EC8674: A12A0004  lhz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EC8678: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82EC867C: 419A0010  beq cr6, 0x82ec868c
	if ctx.cr[6].eq {
	pc = 0x82EC868C; continue 'dispatch;
	}
	// 82EC8680: A12A0006  lhz r9, 6(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(6 as u32) ) } as u64;
	// 82EC8684: 38E90001  addi r7, r9, 1
	ctx.r[7].s64 = ctx.r[9].s64 + 1;
	// 82EC8688: B0EA0006  sth r7, 6(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(6 as u32), ctx.r[7].u16 ) };
	// 82EC868C: 815B0018  lwz r10, 0x18(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EC8690: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 82EC8694: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82EC8698: 7F085000  cmpw cr6, r8, r10
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82EC869C: 4198FFC4  blt cr6, 0x82ec8660
	if ctx.cr[6].lt {
	pc = 0x82EC8660; continue 'dispatch;
	}
	// 82EC86A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EC86A4: 482DFB10  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EC86A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EC86A8 size=204
    let mut pc: u32 = 0x82EC86A8;
    'dispatch: loop {
        match pc {
            0x82EC86A8 => {
    //   block [0x82EC86A8..0x82EC8774)
	// 82EC86A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EC86AC: 482DFAB9  bl 0x831a8164
	ctx.lr = 0x82EC86B0;
	sub_831A8130(ctx, base);
	// 82EC86B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EC86B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EC86B8: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82EC86BC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82EC86C0: 392B3ED4  addi r9, r11, 0x3ed4
	ctx.r[9].s64 = ctx.r[11].s64 + 16084;
	// 82EC86C4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82EC86C8: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 82EC86CC: B15F0006  sth r10, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82EC86D0: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EC86D4: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82EC86D8: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82EC86DC: 3BDF0020  addi r30, r31, 0x20
	ctx.r[30].s64 = ctx.r[31].s64 + 32;
	// 82EC86E0: 93BF000C  stw r29, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 82EC86E4: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82EC86E8: 93BF0014  stw r29, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[29].u32 ) };
	// 82EC86EC: 93BF0018  stw r29, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[29].u32 ) };
	// 82EC86F0: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82EC86F4: 93BF0020  stw r29, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[29].u32 ) };
	// 82EC86F8: 93BF0024  stw r29, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[29].u32 ) };
	// 82EC86FC: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82EC8700: 4BFFFDA9  bl 0x82ec84a8
	ctx.lr = 0x82EC8704;
	sub_82EC84A8(ctx, base);
	// 82EC8704: 811F0028  lwz r8, 0x28(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82EC8708: 550B00BE  clrlwi r11, r8, 2
	ctx.r[11].u64 = ctx.r[8].u32 as u64 & 0x3FFFFFFFu64;
	// 82EC870C: 837C0010  lwz r27, 0x10(r28)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EC8710: 7F0BD800  cmpw cr6, r11, r27
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[27].s32, &mut ctx.xer);
	// 82EC8714: 40980024  bge cr6, 0x82ec8738
	if !ctx.cr[6].lt {
	pc = 0x82EC8738; continue 'dispatch;
	}
	// 82EC8718: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EC871C: 7F1B5800  cmpw cr6, r27, r11
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82EC8720: 41980008  blt cr6, 0x82ec8728
	if ctx.cr[6].lt {
	pc = 0x82EC8728; continue 'dispatch;
	}
	// 82EC8724: 7F6BDB78  mr r11, r27
	ctx.r[11].u64 = ctx.r[27].u64;
	// 82EC8728: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82EC872C: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82EC8730: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82EC8734: 4BFDE0C5  bl 0x82ea67f8
	ctx.lr = 0x82EC8738;
	sub_82EA67F8(ctx, base);
	// 82EC8738: 937E0004  stw r27, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[27].u32 ) };
	// 82EC873C: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82EC8740: 815C0010  lwz r10, 0x10(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EC8744: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EC8748: 40990020  ble cr6, 0x82ec8768
	if !ctx.cr[6].gt {
	pc = 0x82EC8768; continue 'dispatch;
	}
	// 82EC874C: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EC8750: 7D6AE92E  stwx r11, r10, r29
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[29].u32), ctx.r[11].u32) };
	// 82EC8754: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82EC8758: 813C0010  lwz r9, 0x10(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EC875C: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82EC8760: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82EC8764: 4198FFE8  blt cr6, 0x82ec874c
	if ctx.cr[6].lt {
	pc = 0x82EC874C; continue 'dispatch;
	}
	// 82EC8768: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EC876C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82EC8770: 482DFA44  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EC8778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EC8778 size=264
    let mut pc: u32 = 0x82EC8778;
    'dispatch: loop {
        match pc {
            0x82EC8778 => {
    //   block [0x82EC8778..0x82EC8880)
	// 82EC8778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EC877C: 482DF9F1  bl 0x831a816c
	ctx.lr = 0x82EC8780;
	sub_831A8130(ctx, base);
	// 82EC8780: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EC8784: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EC8788: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82EC878C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82EC8790: 392B3ED4  addi r9, r11, 0x3ed4
	ctx.r[9].s64 = ctx.r[11].s64 + 16084;
	// 82EC8794: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82EC8798: B15F0006  sth r10, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82EC879C: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 82EC87A0: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EC87A4: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 82EC87A8: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82EC87AC: 3BDF0020  addi r30, r31, 0x20
	ctx.r[30].s64 = ctx.r[31].s64 + 32;
	// 82EC87B0: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82EC87B4: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82EC87B8: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82EC87BC: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82EC87C0: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 82EC87C4: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82EC87C8: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82EC87CC: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 82EC87D0: 4BFFFCD9  bl 0x82ec84a8
	ctx.lr = 0x82EC87D4;
	sub_82EC84A8(ctx, base);
	// 82EC87D4: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82EC87D8: 811D0004  lwz r8, 4(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EC87DC: 556A00BE  clrlwi r10, r11, 2
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82EC87E0: 7F0A4000  cmpw cr6, r10, r8
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82EC87E4: 40980060  bge cr6, 0x82ec8844
	if !ctx.cr[6].lt {
	pc = 0x82EC8844; continue 'dispatch;
	}
	// 82EC87E8: 556B0000  rlwinm r11, r11, 0, 0, 0
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82EC87EC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EC87F0: 409A0020  bne cr6, 0x82ec8810
	if !ctx.cr[6].eq {
	pc = 0x82EC8810; continue 'dispatch;
	}
	// 82EC87F4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EC87F8: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82EC87FC: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82EC8800: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EC8804: 5545103A  slwi r5, r10, 2
	ctx.r[5].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82EC8808: 7C69582E  lwzx r3, r9, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EC880C: 4BFD7FA5  bl 0x82ea07b0
	ctx.lr = 0x82EC8810;
	sub_82EA07B0(ctx, base);
	// 82EC8810: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EC8814: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82EC8818: 813D0004  lwz r9, 4(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EC881C: 38A00017  li r5, 0x17
	ctx.r[5].s64 = 23;
	// 82EC8820: 5524103A  slwi r4, r9, 2
	ctx.r[4].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82EC8824: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EC8828: 4BFD7F09  bl 0x82ea0730
	ctx.lr = 0x82EC882C;
	sub_82EA0730(ctx, base);
	// 82EC882C: 811E0008  lwz r8, 8(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EC8830: 907E0000  stw r3, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82EC8834: 55070042  rlwinm r7, r8, 0, 1, 1
	ctx.r[7].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	// 82EC8838: 80DD0004  lwz r6, 4(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EC883C: 7CE53378  or r5, r7, r6
	ctx.r[5].u64 = ctx.r[7].u64 | ctx.r[6].u64;
	// 82EC8840: 90BE0008  stw r5, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 82EC8844: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EC8848: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EC884C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EC8850: 915E0004  stw r10, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82EC8854: 813D0000  lwz r9, 0(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EC8858: 4099001C  ble cr6, 0x82ec8874
	if !ctx.cr[6].gt {
	pc = 0x82EC8874; continue 'dispatch;
	}
	// 82EC885C: 7D2B4850  subf r9, r11, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 82EC8860: 7D09582E  lwzx r8, r9, r11
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EC8864: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EC8868: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82EC886C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82EC8870: 4082FFF0  bne 0x82ec8860
	if !ctx.cr[0].eq {
	pc = 0x82EC8860; continue 'dispatch;
	}
	// 82EC8874: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EC8878: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EC887C: 482DF940  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EC8880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EC8880 size=336
    let mut pc: u32 = 0x82EC8880;
    'dispatch: loop {
        match pc {
            0x82EC8880 => {
    //   block [0x82EC8880..0x82EC89D0)
	// 82EC8880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EC8884: 482DF8E9  bl 0x831a816c
	ctx.lr = 0x82EC8888;
	sub_831A8130(ctx, base);
	// 82EC8888: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EC888C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EC8890: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82EC8894: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82EC8898: 392B3ED4  addi r9, r11, 0x3ed4
	ctx.r[9].s64 = ctx.r[11].s64 + 16084;
	// 82EC889C: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EC88A0: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EC88A4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EC88A8: 40990028  ble cr6, 0x82ec88d0
	if !ctx.cr[6].gt {
	pc = 0x82EC88D0; continue 'dispatch;
	}
	// 82EC88AC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82EC88B0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EC88B4: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82EC88B8: 4800C851  bl 0x82ed5108
	ctx.lr = 0x82EC88BC;
	sub_82ED5108(ctx, base);
	// 82EC88BC: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EC88C0: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82EC88C4: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82EC88C8: 7F1D5000  cmpw cr6, r29, r10
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82EC88CC: 4198FFE4  blt cr6, 0x82ec88b0
	if ctx.cr[6].lt {
	pc = 0x82EC88B0; continue 'dispatch;
	}
	// 82EC88D0: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EC88D4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82EC88D8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EC88DC: 4099005C  ble cr6, 0x82ec8938
	if !ctx.cr[6].gt {
	pc = 0x82EC8938; continue 'dispatch;
	}
	// 82EC88E0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82EC88E4: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EC88E8: 7C7E582E  lwzx r3, r30, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EC88EC: A1430004  lhz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EC88F0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EC88F4: 419A0030  beq cr6, 0x82ec8924
	if ctx.cr[6].eq {
	pc = 0x82EC8924; continue 'dispatch;
	}
	// 82EC88F8: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82EC88FC: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 82EC8900: 7D490734  extsh r9, r10
	ctx.r[9].s64 = ctx.r[10].s16 as i64;
	// 82EC8904: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82EC8908: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82EC890C: 409A0018  bne cr6, 0x82ec8924
	if !ctx.cr[6].eq {
	pc = 0x82EC8924; continue 'dispatch;
	}
	// 82EC8910: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EC8914: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EC8918: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EC891C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EC8920: 4E800421  bctrl
	ctx.lr = 0x82EC8924;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EC8924: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EC8928: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82EC892C: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82EC8930: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82EC8934: 4198FFB0  blt cr6, 0x82ec88e4
	if ctx.cr[6].lt {
	pc = 0x82EC88E4; continue 'dispatch;
	}
	// 82EC8938: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82EC893C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82EC8940: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EC8944: 409A0020  bne cr6, 0x82ec8964
	if !ctx.cr[6].eq {
	pc = 0x82EC8964; continue 'dispatch;
	}
	// 82EC8948: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EC894C: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82EC8950: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82EC8954: 809F0020  lwz r4, 0x20(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82EC8958: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82EC895C: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82EC8960: 4BFD7E51  bl 0x82ea07b0
	ctx.lr = 0x82EC8964;
	sub_82EA07B0(ctx, base);
	// 82EC8964: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EC8968: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82EC896C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EC8970: 409A0020  bne cr6, 0x82ec8990
	if !ctx.cr[6].eq {
	pc = 0x82EC8990; continue 'dispatch;
	}
	// 82EC8974: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EC8978: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82EC897C: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82EC8980: 809F0014  lwz r4, 0x14(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EC8984: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82EC8988: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82EC898C: 4BFD7E25  bl 0x82ea07b0
	ctx.lr = 0x82EC8990;
	sub_82EA07B0(ctx, base);
	// 82EC8990: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EC8994: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82EC8998: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EC899C: 409A0020  bne cr6, 0x82ec89bc
	if !ctx.cr[6].eq {
	pc = 0x82EC89BC; continue 'dispatch;
	}
	// 82EC89A0: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EC89A4: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82EC89A8: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82EC89AC: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EC89B0: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82EC89B4: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82EC89B8: 4BFD7DF9  bl 0x82ea07b0
	ctx.lr = 0x82EC89BC;
	sub_82EA07B0(ctx, base);
	// 82EC89BC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82EC89C0: 394B9EAC  addi r10, r11, -0x6154
	ctx.r[10].s64 = ctx.r[11].s64 + -24916;
	// 82EC89C4: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EC89C8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EC89CC: 482DF7F0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EC89D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EC89D0 size=496
    let mut pc: u32 = 0x82EC89D0;
    'dispatch: loop {
        match pc {
            0x82EC89D0 => {
    //   block [0x82EC89D0..0x82EC8BC0)
	// 82EC89D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EC89D4: 482DF771  bl 0x831a8144
	ctx.lr = 0x82EC89D8;
	sub_831A8130(ctx, base);
	// 82EC89D8: DBE1FF88  stfd f31, -0x78(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-120 as u32), ctx.f[31].u64 ) };
	// 82EC89DC: 3980FF70  li r12, -0x90
	ctx.r[12].s64 = -144;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EC8BC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EC8BC0 size=392
    let mut pc: u32 = 0x82EC8BC0;
    'dispatch: loop {
        match pc {
            0x82EC8BC0 => {
    //   block [0x82EC8BC0..0x82EC8D48)
	// 82EC8BC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EC8BC4: 482DF5A9  bl 0x831a816c
	ctx.lr = 0x82EC8BC8;
	sub_831A8130(ctx, base);
	// 82EC8BC8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EC8BCC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82EC8BD0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EC8BD4: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82EC8BD8: 4BFFFDF9  bl 0x82ec89d0
	ctx.lr = 0x82EC8BDC;
	sub_82EC89D0(ctx, base);
	// 82EC8BDC: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EC8D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82EC8D48 size=488
    let mut pc: u32 = 0x82EC8D48;
    'dispatch: loop {
        match pc {
            0x82EC8D48 => {
    //   block [0x82EC8D48..0x82EC8F30)
	// 82EC8D48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EC8D4C: 482DF3FD  bl 0x831a8148
	ctx.lr = 0x82EC8D50;
	sub_831A8130(ctx, base);
	// 82EC8D50: DBE1FF90  stfd f31, -0x70(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-112 as u32), ctx.f[31].u64 ) };
	// 82EC8D54: 9421FEF0  stwu r1, -0x110(r1)
	ea = ctx.r[1].u32.wrapping_add(-272 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EC8D58: 7C741B78  mr r20, r3
	ctx.r[20].u64 = ctx.r[3].u64;
	// 82EC8D5C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82EC8D60: 8174002C  lwz r11, 0x2c(r20)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(44 as u32) ) } as u64;
	// 82EC8D64: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EC8D68: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EC8D6C: 409901B8  ble cr6, 0x82ec8f24
	if !ctx.cr[6].gt {
	pc = 0x82EC8F24; continue 'dispatch;
	}
	// 82EC8D70: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82EC8D74: 7D755B78  mr r21, r11
	ctx.r[21].u64 = ctx.r[11].u64;
	// 82EC8D78: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82EC8D7C: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82EC8D80: 3AC00000  li r22, 0
	ctx.r[22].s64 = 0;
	// 82EC8D84: C3E908A8  lfs f31, 0x8a8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2216 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82EC8D88: 3BA40010  addi r29, r4, 0x10
	ctx.r[29].s64 = ctx.r[4].s64 + 16;
	// 82EC8D8C: 3AE0FFF0  li r23, -0x10
	ctx.r[23].s64 = -16;
	// 82EC8D90: 3B000010  li r24, 0x10
	ctx.r[24].s64 = 16;
	// 82EC8D94: 3B200020  li r25, 0x20
	ctx.r[25].s64 = 32;
	// 82EC8D98: 3B6B7060  addi r27, r11, 0x7060
	ctx.r[27].s64 = ctx.r[11].s64 + 28768;
	// 82EC8D9C: 3B4A0010  addi r26, r10, 0x10
	ctx.r[26].s64 = ctx.r[10].s64 + 16;
	// 82EC8DA0: 81740020  lwz r11, 0x20(r20)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(32 as u32) ) } as u64;
	// 82EC8DA4: 7D76582E  lwzx r11, r22, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[22].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EC8DA8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EC8DAC: 41980168  blt cr6, 0x82ec8f14
	if ctx.cr[6].lt {
	pc = 0x82EC8F14; continue 'dispatch;
	}
	// 82EC8DB0: 81540008  lwz r10, 8(r20)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EC8DB4: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82EC8DB8: 7FE9502E  lwzx r31, r9, r10
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82EC8DBC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82EC8DC0: 419A0154  beq cr6, 0x82ec8f14
	if ctx.cr[6].eq {
	pc = 0x82EC8F14; continue 'dispatch;
	}
	// 82EC8DC4: 397C0010  addi r11, r28, 0x10
	ctx.r[11].s64 = ctx.r[28].s64 + 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EC8F30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EC8F30 size=1228
    let mut pc: u32 = 0x82EC8F30;
    'dispatch: loop {
        match pc {
            0x82EC8F30 => {
    //   block [0x82EC8F30..0x82EC93FC)
	// 82EC8F30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EC8F34: 482DF201  bl 0x831a8134
	ctx.lr = 0x82EC8F38;
	sub_831A8130(ctx, base);
	// 82EC8F38: 3981FF70  addi r12, r1, -0x90
	ctx.r[12].s64 = ctx.r[1].s64 + -144;
	// 82EC8F3C: 482DFB31  bl 0x831a8a6c
	ctx.lr = 0x82EC8F40;
	sub_831A8A40(ctx, base);
	// 82EC8F40: 3980FF10  li r12, -0xf0
	ctx.r[12].s64 = -240;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EC9400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EC9400 size=824
    let mut pc: u32 = 0x82EC9400;
    'dispatch: loop {
        match pc {
            0x82EC9400 => {
    //   block [0x82EC9400..0x82EC9738)
	// 82EC9400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EC9404: 482DED49  bl 0x831a814c
	ctx.lr = 0x82EC9408;
	sub_831A8130(ctx, base);
	// 82EC9408: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EC940C: 82CD0000  lwz r22, 0(r13)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EC9410: 3AA00014  li r21, 0x14
	ctx.r[21].s64 = 20;
	// 82EC9414: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82EC9418: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 82EC941C: 3FE08000  lis r31, -0x8000
	ctx.r[31].s64 = -2147483648;
	// 82EC9420: 93010050  stw r24, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[24].u32 ) };
	// 82EC9424: 7C972378  mr r23, r4
	ctx.r[23].u64 = ctx.r[4].u64;
	// 82EC9428: 7C75B02E  lwzx r3, r21, r22
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[21].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 82EC942C: 93010054  stw r24, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[24].u32 ) };
	// 82EC9430: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 82EC9434: 835B000C  lwz r26, 0xc(r27)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EC9438: 833B0018  lwz r25, 0x18(r27)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EC943C: 8123002C  lwz r9, 0x2c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82EC9440: 397A0004  addi r11, r26, 4
	ctx.r[11].s64 = ctx.r[26].s64 + 4;
	// 82EC9444: 55641036  rlwinm r4, r11, 2, 0, 0x1b
	ctx.r[4].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82EC9448: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82EC944C: 7D4B2214  add r10, r11, r4
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82EC9450: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82EC9454: 4199000C  bgt cr6, 0x82ec9460
	if ctx.cr[6].gt {
	pc = 0x82EC9460; continue 'dispatch;
	}
	// 82EC9458: 91430020  stw r10, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82EC945C: 48000018  b 0x82ec9474
	pc = 0x82EC9474; continue 'dispatch;
	// 82EC9460: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EC9464: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EC9468: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EC946C: 4E800421  bctrl
	ctx.lr = 0x82EC9470;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EC9470: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82EC9474: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82EC9478: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82EC947C: 7F4BFB78  or r11, r26, r31
	ctx.r[11].u64 = ctx.r[26].u64 | ctx.r[31].u64;
	// 82EC9480: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82EC9484: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82EC9488: 7F0BD000  cmpw cr6, r11, r26
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[26].s32, &mut ctx.xer);
	// 82EC948C: 40980024  bge cr6, 0x82ec94b0
	if !ctx.cr[6].lt {
	pc = 0x82EC94B0; continue 'dispatch;
	}
	// 82EC9490: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EC9494: 7F1A5800  cmpw cr6, r26, r11
	ctx.cr[6].compare_i32(ctx.r[26].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82EC9498: 41980008  blt cr6, 0x82ec94a0
	if ctx.cr[6].lt {
	pc = 0x82EC94A0; continue 'dispatch;
	}
	// 82EC949C: 7F4BD378  mr r11, r26
	ctx.r[11].u64 = ctx.r[26].u64;
	// 82EC94A0: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82EC94A4: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82EC94A8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82EC94AC: 4BFDD34D  bl 0x82ea67f8
	ctx.lr = 0x82EC94B0;
	sub_82EA67F8(ctx, base);
	// 82EC94B0: 93010060  stw r24, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[24].u32 ) };
	// 82EC94B4: 39790004  addi r11, r25, 4
	ctx.r[11].s64 = ctx.r[25].s64 + 4;
	// 82EC94B8: 93010064  stw r24, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[24].u32 ) };
	// 82EC94BC: 7C75B02E  lwzx r3, r21, r22
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[21].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 82EC94C0: 55641036  rlwinm r4, r11, 2, 0, 0x1b
	ctx.r[4].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82EC94C4: 93E10068  stw r31, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[31].u32 ) };
	// 82EC94C8: 93410054  stw r26, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[26].u32 ) };
	// 82EC94CC: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82EC94D0: 7D4B2214  add r10, r11, r4
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82EC94D4: 8123002C  lwz r9, 0x2c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82EC94D8: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82EC94DC: 4199000C  bgt cr6, 0x82ec94e8
	if ctx.cr[6].gt {
	pc = 0x82EC94E8; continue 'dispatch;
	}
	// 82EC94E0: 91430020  stw r10, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82EC94E4: 48000018  b 0x82ec94fc
	pc = 0x82EC94FC; continue 'dispatch;
	// 82EC94E8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EC94EC: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EC94F0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EC94F4: 4E800421  bctrl
	ctx.lr = 0x82EC94F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EC94F8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82EC94FC: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82EC9500: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82EC9504: 7F2BFB78  or r11, r25, r31
	ctx.r[11].u64 = ctx.r[25].u64 | ctx.r[31].u64;
	// 82EC9508: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82EC950C: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82EC9510: 7F0BC800  cmpw cr6, r11, r25
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[25].s32, &mut ctx.xer);
	// 82EC9514: 40980024  bge cr6, 0x82ec9538
	if !ctx.cr[6].lt {
	pc = 0x82EC9538; continue 'dispatch;
	}
	// 82EC9518: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EC951C: 7F195800  cmpw cr6, r25, r11
	ctx.cr[6].compare_i32(ctx.r[25].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82EC9520: 41980008  blt cr6, 0x82ec9528
	if ctx.cr[6].lt {
	pc = 0x82EC9528; continue 'dispatch;
	}
	// 82EC9524: 7F2BCB78  mr r11, r25
	ctx.r[11].u64 = ctx.r[25].u64;
	// 82EC9528: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82EC952C: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82EC9530: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82EC9534: 4BFDD2C5  bl 0x82ea67f8
	ctx.lr = 0x82EC9538;
	sub_82EA67F8(ctx, base);
	// 82EC9538: 93210064  stw r25, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[25].u32 ) };
	// 82EC953C: 2F1A0000  cmpwi cr6, r26, 0
	ctx.cr[6].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 82EC9540: 40990038  ble cr6, 0x82ec9578
	if !ctx.cr[6].gt {
	pc = 0x82EC9578; continue 'dispatch;
	}
	// 82EC9544: 7F1FC378  mr r31, r24
	ctx.r[31].u64 = ctx.r[24].u64;
	// 82EC9548: 7F5ED378  mr r30, r26
	ctx.r[30].u64 = ctx.r[26].u64;
	// 82EC954C: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EC9550: 83A10050  lwz r29, 0x50(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EC9554: 7C7F582E  lwzx r3, r31, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EC9558: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EC955C: 812A0018  lwz r9, 0x18(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EC9560: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82EC9564: 4E800421  bctrl
	ctx.lr = 0x82EC9568;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EC9568: 7C7FE92E  stwx r3, r31, r29
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[29].u32), ctx.r[3].u32) };
	// 82EC956C: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82EC9570: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82EC9574: 4082FFD8  bne 0x82ec954c
	if !ctx.cr[0].eq {
	pc = 0x82EC954C; continue 'dispatch;
	}
	// 82EC9578: 2F190000  cmpwi cr6, r25, 0
	ctx.cr[6].compare_i32(ctx.r[25].s32, 0, &mut ctx.xer);
	// 82EC957C: 40990064  ble cr6, 0x82ec95e0
	if !ctx.cr[6].gt {
	pc = 0x82EC95E0; continue 'dispatch;
	}
	// 82EC9580: 7F1FC378  mr r31, r24
	ctx.r[31].u64 = ctx.r[24].u64;
	// 82EC9584: 7F1DC378  mr r29, r24
	ctx.r[29].u64 = ctx.r[24].u64;
	// 82EC9588: 7F3ECB78  mr r30, r25
	ctx.r[30].u64 = ctx.r[25].u64;
	// 82EC958C: 813B002C  lwz r9, 0x2c(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(44 as u32) ) } as u64;
	// 82EC9590: 7EE6BB78  mr r6, r23
	ctx.r[6].u64 = ctx.r[23].u64;
	// 82EC9594: 815B0014  lwz r10, 0x14(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(20 as u32) ) } as u64;
	// 82EC9598: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EC959C: 80E10060  lwz r7, 0x60(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82EC95A0: 7D1F5A14  add r8, r31, r11
	ctx.r[8].u64 = ctx.r[31].u64 + ctx.r[11].u64;
	// 82EC95A4: 81290004  lwz r9, 4(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EC95A8: 7F9F3A14  add r28, r31, r7
	ctx.r[28].u64 = ctx.r[31].u64 + ctx.r[7].u64;
	// 82EC95AC: 7C6AF82E  lwzx r3, r10, r31
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82EC95B0: 7CA9EA14  add r5, r9, r29
	ctx.r[5].u64 = ctx.r[9].u64 + ctx.r[29].u64;
	// 82EC95B4: 80880004  lwz r4, 4(r8)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EC95B8: A1450002  lhz r10, 2(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[5].u32.wrapping_add(2 as u32) ) } as u64;
	// 82EC95BC: 7D490734  extsh r9, r10
	ctx.r[9].s64 = ctx.r[10].s16 as i64;
	// 82EC95C0: 5528103A  slwi r8, r9, 2
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82EC95C4: 7CA8582E  lwzx r5, r8, r11
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EC95C8: 48013FC1  bl 0x82edd588
	ctx.lr = 0x82EC95CC;
	sub_82EDD588(ctx, base);
	// 82EC95CC: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82EC95D0: 907C0000  stw r3, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82EC95D4: 3BBD0002  addi r29, r29, 2
	ctx.r[29].s64 = ctx.r[29].s64 + 2;
	// 82EC95D8: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82EC95DC: 4082FFB0  bne 0x82ec958c
	if !ctx.cr[0].eq {
	pc = 0x82EC958C; continue 'dispatch;
	}
	// 82EC95E0: 38A0000E  li r5, 0xe
	ctx.r[5].s64 = 14;
	// 82EC95E4: 7C75B02E  lwzx r3, r21, r22
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[21].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 82EC95E8: 38800030  li r4, 0x30
	ctx.r[4].s64 = 48;
	// 82EC95EC: 4BFD7145  bl 0x82ea0730
	ctx.lr = 0x82EC95F0;
	sub_82EA0730(ctx, base);
	// 82EC95F0: 39600030  li r11, 0x30
	ctx.r[11].s64 = 48;
	// 82EC95F4: 38FB0020  addi r7, r27, 0x20
	ctx.r[7].s64 = ctx.r[27].s64 + 32;
	// 82EC95F8: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82EC95FC: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82EC9600: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82EC9604: 80DB002C  lwz r6, 0x2c(r27)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(44 as u32) ) } as u64;
	// 82EC9608: 4BFFF171  bl 0x82ec8778
	ctx.lr = 0x82EC960C;
	sub_82EC8778(ctx, base);
	// 82EC960C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82EC9610: 2F1A0000  cmpwi cr6, r26, 0
	ctx.cr[6].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 82EC9614: 40990024  ble cr6, 0x82ec9638
	if !ctx.cr[6].gt {
	pc = 0x82EC9638; continue 'dispatch;
	}
	// 82EC9618: 7F1EC378  mr r30, r24
	ctx.r[30].u64 = ctx.r[24].u64;
	// 82EC961C: 7F5FD378  mr r31, r26
	ctx.r[31].u64 = ctx.r[26].u64;
	// 82EC9620: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EC9624: 7C7E582E  lwzx r3, r30, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EC9628: 4800BAE1  bl 0x82ed5108
	ctx.lr = 0x82EC962C;
	sub_82ED5108(ctx, base);
	// 82EC962C: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82EC9630: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82EC9634: 4082FFEC  bne 0x82ec9620
	if !ctx.cr[0].eq {
	pc = 0x82EC9620; continue 'dispatch;
	}
	// 82EC9638: 2F190000  cmpwi cr6, r25, 0
	ctx.cr[6].compare_i32(ctx.r[25].s32, 0, &mut ctx.xer);
	// 82EC963C: 40990058  ble cr6, 0x82ec9694
	if !ctx.cr[6].gt {
	pc = 0x82EC9694; continue 'dispatch;
	}
	// 82EC9640: 7F1FC378  mr r31, r24
	ctx.r[31].u64 = ctx.r[24].u64;
	// 82EC9644: 7F3ECB78  mr r30, r25
	ctx.r[30].u64 = ctx.r[25].u64;
	// 82EC9648: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82EC964C: 7C7F582E  lwzx r3, r31, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EC9650: A1430004  lhz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EC9654: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EC9658: 419A0030  beq cr6, 0x82ec9688
	if ctx.cr[6].eq {
	pc = 0x82EC9688; continue 'dispatch;
	}
	// 82EC965C: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82EC9660: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 82EC9664: 7D490734  extsh r9, r10
	ctx.r[9].s64 = ctx.r[10].s16 as i64;
	// 82EC9668: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82EC966C: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82EC9670: 409A0018  bne cr6, 0x82ec9688
	if !ctx.cr[6].eq {
	pc = 0x82EC9688; continue 'dispatch;
	}
	// 82EC9674: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EC9678: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EC967C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EC9680: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EC9684: 4E800421  bctrl
	ctx.lr = 0x82EC9688;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EC9688: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82EC968C: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82EC9690: 4082FFB8  bne 0x82ec9648
	if !ctx.cr[0].eq {
	pc = 0x82EC9648; continue 'dispatch;
	}
	// 82EC9694: 7C75B02E  lwzx r3, r21, r22
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[21].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 82EC9698: 8081006C  lwz r4, 0x6c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 82EC969C: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82EC96A0: 90830020  stw r4, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[4].u32 ) };
	// 82EC96A4: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82EC96A8: 409A0014  bne cr6, 0x82ec96bc
	if !ctx.cr[6].eq {
	pc = 0x82EC96BC; continue 'dispatch;
	}
	// 82EC96AC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EC96B0: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EC96B4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EC96B8: 4E800421  bctrl
	ctx.lr = 0x82EC96BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EC96BC: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82EC96C0: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82EC96C4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EC96C8: 409A0018  bne cr6, 0x82ec96e0
	if !ctx.cr[6].eq {
	pc = 0x82EC96E0; continue 'dispatch;
	}
	// 82EC96CC: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82EC96D0: 7C75B02E  lwzx r3, r21, r22
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[21].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 82EC96D4: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82EC96D8: 80810060  lwz r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82EC96DC: 4BFD70D5  bl 0x82ea07b0
	ctx.lr = 0x82EC96E0;
	sub_82EA07B0(ctx, base);
	// 82EC96E0: 7C75B02E  lwzx r3, r21, r22
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[21].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 82EC96E4: 8081005C  lwz r4, 0x5c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82EC96E8: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82EC96EC: 90830020  stw r4, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[4].u32 ) };
	// 82EC96F0: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82EC96F4: 409A0014  bne cr6, 0x82ec9708
	if !ctx.cr[6].eq {
	pc = 0x82EC9708; continue 'dispatch;
	}
	// 82EC96F8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EC96FC: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EC9700: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82EC9704: 4E800421  bctrl
	ctx.lr = 0x82EC9708;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82EC9708: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82EC970C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82EC9710: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EC9714: 409A0018  bne cr6, 0x82ec972c
	if !ctx.cr[6].eq {
	pc = 0x82EC972C; continue 'dispatch;
	}
	// 82EC9718: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82EC971C: 7C75B02E  lwzx r3, r21, r22
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[21].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 82EC9720: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82EC9724: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82EC9728: 4BFD7089  bl 0x82ea07b0
	ctx.lr = 0x82EC972C;
	sub_82EA07B0(ctx, base);
	// 82EC972C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EC9730: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82EC9734: 482DEA68  b 0x831a819c
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EC9738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EC9738 size=52
    let mut pc: u32 = 0x82EC9738;
    'dispatch: loop {
        match pc {
            0x82EC9738 => {
    //   block [0x82EC9738..0x82EC976C)
	// 82EC9738: 39630010  addi r11, r3, 0x10
	ctx.r[11].s64 = ctx.r[3].s64 + 16;
	// 82EC973C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82EC9740: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82EC9744: B1430000  sth r10, 0(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 82EC9748: 39430020  addi r10, r3, 0x20
	ctx.r[10].s64 = ctx.r[3].s64 + 32;
	// 82EC974C: B1230002  sth r9, 2(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(2 as u32), ctx.r[9].u16 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EC9770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EC9770 size=4
    let mut pc: u32 = 0x82EC9770;
    'dispatch: loop {
        match pc {
            0x82EC9770 => {
    //   block [0x82EC9770..0x82EC9774)
	// 82EC9770: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EC9778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EC9778 size=64
    let mut pc: u32 = 0x82EC9778;
    'dispatch: loop {
        match pc {
            0x82EC9778 => {
    //   block [0x82EC9778..0x82EC97B8)
	// 82EC9778: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EC977C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82EC9780: 7D0A4378  mr r10, r8
	ctx.r[10].u64 = ctx.r[8].u64;
	// 82EC9784: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EC9788: 40990028  ble cr6, 0x82ec97b0
	if !ctx.cr[6].gt {
	pc = 0x82EC97B0; continue 'dispatch;
	}
	// 82EC978C: 7D0B4378  mr r11, r8
	ctx.r[11].u64 = ctx.r[8].u64;
	// 82EC9790: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82EC9794: 80E30018  lwz r7, 0x18(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EC9798: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82EC979C: 7D2B392E  stwx r9, r11, r7
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[7].u32), ctx.r[9].u32) };
	// 82EC97A0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82EC97A4: 80C3001C  lwz r6, 0x1c(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EC97A8: 7F0A3000  cmpw cr6, r10, r6
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[6].s32, &mut ctx.xer);
	// 82EC97AC: 4198FFE8  blt cr6, 0x82ec9794
	if ctx.cr[6].lt {
	pc = 0x82EC9794; continue 'dispatch;
	}
	// 82EC97B0: 99030024  stb r8, 0x24(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[8].u8 ) };
	// 82EC97B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EC97B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EC97B8 size=212
    let mut pc: u32 = 0x82EC97B8;
    'dispatch: loop {
        match pc {
            0x82EC97B8 => {
    //   block [0x82EC97B8..0x82EC988C)
	// 82EC97B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EC97BC: 482DE9A5  bl 0x831a8160
	ctx.lr = 0x82EC97C0;
	sub_831A8130(ctx, base);
	// 82EC97C0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EC97C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EC97C8: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82EC97CC: 3B600001  li r27, 1
	ctx.r[27].s64 = 1;
	// 82EC97D0: 394B3F00  addi r10, r11, 0x3f00
	ctx.r[10].s64 = ctx.r[11].s64 + 16128;
	// 82EC97D4: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82EC97D8: B37F0006  sth r27, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[27].u16 ) };
	// 82EC97DC: 3D208000  lis r9, -0x8000
	ctx.r[9].s64 = -2147483648;
	// 82EC97E0: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82EC97E4: 3BBF0018  addi r29, r31, 0x18
	ctx.r[29].s64 = ctx.r[31].s64 + 24;
	// 82EC97E8: 81040000  lwz r8, 0(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EC97EC: 911F0008  stw r8, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 82EC97F0: 80E40004  lwz r7, 4(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EC97F4: 90FF000C  stw r7, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 82EC97F8: 80C40008  lwz r6, 8(r4)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EC97FC: 90DF0010  stw r6, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[6].u32 ) };
	// 82EC9800: 935F0018  stw r26, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[26].u32 ) };
	// 82EC9804: 935F001C  stw r26, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[26].u32 ) };
	// 82EC9808: 913F0020  stw r9, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[9].u32 ) };
	// 82EC980C: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EC9810: 83C50010  lwz r30, 0x10(r5)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EC9814: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 82EC9818: 839F001C  lwz r28, 0x1c(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82EC981C: 7F1EE000  cmpw cr6, r30, r28
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[28].s32, &mut ctx.xer);
	// 82EC9820: 40990058  ble cr6, 0x82ec9878
	if !ctx.cr[6].gt {
	pc = 0x82EC9878; continue 'dispatch;
	}
	// 82EC9824: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EC9828: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82EC982C: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82EC9830: 40980024  bge cr6, 0x82ec9854
	if !ctx.cr[6].lt {
	pc = 0x82EC9854; continue 'dispatch;
	}
	// 82EC9834: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82EC9838: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82EC983C: 41980008  blt cr6, 0x82ec9844
	if ctx.cr[6].lt {
	pc = 0x82EC9844; continue 'dispatch;
	}
	// 82EC9840: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82EC9844: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82EC9848: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82EC984C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82EC9850: 4BFDCFA9  bl 0x82ea67f8
	ctx.lr = 0x82EC9854;
	sub_82EA67F8(ctx, base);
	// 82EC9854: 7F1CF000  cmpw cr6, r28, r30
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82EC9858: 40980020  bge cr6, 0x82ec9878
	if !ctx.cr[6].lt {
	pc = 0x82EC9878; continue 'dispatch;
	}
	// 82EC985C: 578A103A  slwi r10, r28, 2
	ctx.r[10].u32 = ctx.r[28].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82EC9860: 7D7CF050  subf r11, r28, r30
	ctx.r[11].s64 = ctx.r[30].s64 - ctx.r[28].s64;
	// 82EC9864: 813D0000  lwz r9, 0(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EC9868: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82EC986C: 7F69512E  stwx r27, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[27].u32) };
	// 82EC9870: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82EC9874: 4082FFF0  bne 0x82ec9864
	if !ctx.cr[0].eq {
	pc = 0x82EC9864; continue 'dispatch;
	}
	// 82EC9878: 93DD0004  stw r30, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82EC987C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EC9880: 9B5F0024  stb r26, 0x24(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[26].u8 ) };
	// 82EC9884: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82EC9888: 482DE928  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EC9890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EC9890 size=1368
    let mut pc: u32 = 0x82EC9890;
    'dispatch: loop {
        match pc {
            0x82EC9890 => {
    //   block [0x82EC9890..0x82EC9DE8)
	// 82EC9890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EC9894: 482DE8B9  bl 0x831a814c
	ctx.lr = 0x82EC9898;
	sub_831A8130(ctx, base);
	// 82EC9898: 3980FF90  li r12, -0x70
	ctx.r[12].s64 = -112;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EC9DE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82EC9DE8 size=152
    let mut pc: u32 = 0x82EC9DE8;
    'dispatch: loop {
        match pc {
            0x82EC9DE8 => {
    //   block [0x82EC9DE8..0x82EC9E80)
	// 82EC9DE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EC9DEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82EC9DF0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82EC9DF4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82EC9DF8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EC9DFC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82EC9E00: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82EC9E04: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82EC9E08: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82EC9E0C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82EC9E10: 409A0020  bne cr6, 0x82ec9e30
	if !ctx.cr[6].eq {
	pc = 0x82EC9E30; continue 'dispatch;
	}
	// 82EC9E14: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EC9E18: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82EC9E1C: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82EC9E20: 809F0018  lwz r4, 0x18(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82EC9E24: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82EC9E28: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82EC9E2C: 4BFD6985  bl 0x82ea07b0
	ctx.lr = 0x82EC9E30;
	sub_82EA07B0(ctx, base);
	// 82EC9E30: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82EC9E34: 57CA07FE  clrlwi r10, r30, 0x1f
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82EC9E38: 392B9EAC  addi r9, r11, -0x6154
	ctx.r[9].s64 = ctx.r[11].s64 + -24916;
	// 82EC9E3C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82EC9E40: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82EC9E44: 419A0020  beq cr6, 0x82ec9e64
	if ctx.cr[6].eq {
	pc = 0x82EC9E64; continue 'dispatch;
	}
	// 82EC9E48: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EC9E4C: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82EC9E50: 38C0003D  li r6, 0x3d
	ctx.r[6].s64 = 61;
	// 82EC9E54: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EC9E58: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82EC9E5C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82EC9E60: 4BFD6951  bl 0x82ea07b0
	ctx.lr = 0x82EC9E64;
	sub_82EA07B0(ctx, base);
	// 82EC9E64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82EC9E68: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82EC9E6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82EC9E70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82EC9E74: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82EC9E78: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82EC9E7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EC9E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82EC9E80 size=16
    let mut pc: u32 = 0x82EC9E80;
    'dispatch: loop {
        match pc {
            0x82EC9E80 => {
    //   block [0x82EC9E80..0x82EC9E90)
	// 82EC9E80: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82EC9E84: 386B0018  addi r3, r11, 0x18
	ctx.r[3].s64 = ctx.r[11].s64 + 24;
	// 82EC9E88: 808B0028  lwz r4, 0x28(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82EC9E8C: 48000924  b 0x82eca7b0
	sub_82ECA7B0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82EC9E90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82EC9E90 size=836
    let mut pc: u32 = 0x82EC9E90;
    'dispatch: loop {
        match pc {
            0x82EC9E90 => {
    //   block [0x82EC9E90..0x82ECA1D4)
	// 82EC9E90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82EC9E94: 482DE2C5  bl 0x831a8158
	ctx.lr = 0x82EC9E98;
	sub_831A8130(ctx, base);
	// 82EC9E98: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82EC9E9C: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82EC9EA0: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82EC9EA4: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 82EC9EA8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82EC9EAC: 3B1A0028  addi r24, r26, 0x28
	ctx.r[24].s64 = ctx.r[26].s64 + 40;
	// 82EC9EB0: 937A0000  stw r27, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82EC9EB4: 3BDA0038  addi r30, r26, 0x38
	ctx.r[30].s64 = ctx.r[26].s64 + 56;
	// 82EC9EB8: 937A0004  stw r27, 4(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(4 as u32), ctx.r[27].u32 ) };
	// 82EC9EBC: 917A0008  stw r11, 8(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82EC9EC0: 937A000C  stw r27, 0xc(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(12 as u32), ctx.r[27].u32 ) };
	// 82EC9EC4: 937A0010  stw r27, 0x10(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(16 as u32), ctx.r[27].u32 ) };
	// 82EC9EC8: 917A0014  stw r11, 0x14(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82EC9ECC: 937A0024  stw r27, 0x24(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(36 as u32), ctx.r[27].u32 ) };
	// 82EC9ED0: 937A0028  stw r27, 0x28(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(40 as u32), ctx.r[27].u32 ) };
	// 82EC9ED4: 937A002C  stw r27, 0x2c(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(44 as u32), ctx.r[27].u32 ) };
	// 82EC9ED8: 917A0030  stw r11, 0x30(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 82EC9EDC: 93BA0034  stw r29, 0x34(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(52 as u32), ctx.r[29].u32 ) };
	// 82EC9EE0: 937A0038  stw r27, 0x38(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(56 as u32), ctx.r[27].u32 ) };
	// 82EC9EE4: 937A003C  stw r27, 0x3c(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(60 as u32), ctx.r[27].u32 ) };
	// 82EC9EE8: 917A0040  stw r11, 0x40(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 82EC9EEC: A17D0004  lhz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EC9EF0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82EC9EF4: 419A0010  beq cr6, 0x82ec9f04
	if ctx.cr[6].eq {
	pc = 0x82EC9F04; continue 'dispatch;
	}
	// 82EC9EF8: A17D0006  lhz r11, 6(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(6 as u32) ) } as u64;
	// 82EC9EFC: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 82EC9F00: B15D0006  sth r10, 6(r29)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[29].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82EC9F04: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EC9F08: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82EC9F0C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82EC9F10: 40980020  bge cr6, 0x82ec9f30
	if !ctx.cr[6].lt {
	pc = 0x82EC9F30; continue 'dispatch;
	}
	// 82EC9F14: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82EC9F18: 2F040001  cmpwi cr6, r4, 1
	ctx.cr[6].compare_i32(ctx.r[4].s32, 1, &mut ctx.xer);
	// 82EC9F1C: 41990008  bgt cr6, 0x82ec9f24
	if ctx.cr[6].gt {
	pc = 0x82EC9F24; continue 'dispatch;
	}
	// 82EC9F20: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82EC9F24: 38A00030  li r5, 0x30
	ctx.r[5].s64 = 48;
	// 82EC9F28: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82EC9F2C: 4BFDC8CD  bl 0x82ea67f8
	ctx.lr = 0x82EC9F30;
	sub_82EA67F8(ctx, base);
	// 82EC9F30: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82EC9F34: 807A0000  lwz r3, 0(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82EC9F38: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82EC9F3C: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82EC9F40: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 82EC9F44: 3CE08201  lis r7, -0x7dff
	ctx.r[7].s64 = -2113863680;
	// 82EC9F48: C18BC668  lfs f12, -0x3998(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-14744 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82EC9F4C: 3CC08201  lis r6, -0x7dff
	ctx.r[6].s64 = -2113863680;
	// 82EC9F50: C16A08A4  lfs f11, 0x8a4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82EC9F54: 3CA08201  lis r5, -0x7dff
	ctx.r[5].s64 = -2113863680;
	// 82EC9F58: C14908A8  lfs f10, 0x8a8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2216 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82EC9F5C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82EC9F60: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82EC9F64: C128C664  lfs f9, -0x399c(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-14748 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 82EC9F68: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82EC9F6C: C1079F78  lfs f8, -0x6088(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(-24712 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 82EC9F70: C0E6C65C  lfs f7, -0x39a4(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-14756 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 82EC9F74: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82EC9F78: C0C5C658  lfs f6, -0x39a8(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(-14760 as u32) ) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 82EC9F7C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82EC9F80: C00B964C  lfs f0, -0x69b4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27060 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82EC9F84: 38A00030  li r5, 0x30
	ctx.r[5].s64 = 48;
	// 82EC9F88: C1AA7BC8  lfs f13, 0x7bc8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(31688 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82EC9F8C: 911A0004  stw r8, 4(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82EC9F90: C0A9C654  lfs f5, -0x39ac(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-14764 as u32) ) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 82EC9F94: D1810050  stfs f12, 0x50(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82EC9F98: D1610054  stfs f11, 0x54(r1)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82EC9F9C: D1410058  stfs f10, 0x58(r1)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82EC9FA0: D121005C  stfs f9, 0x5c(r1)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 82EC9FA4: D1010060  stfs f8, 0x60(r1)
	tmp.f32 = (ctx.f[8].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 82EC9FA8: D0E10064  stfs f7, 0x64(r1)
	tmp.f32 = (ctx.f[7].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 82EC9FAC: D0C10068  stfs f6, 0x68(r1)
	tmp.f32 = (ctx.f[6].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 82EC9FB0: D001006C  stfs f0, 0x6c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 82EC9FB4: D1A10070  stfs f13, 0x70(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 82EC9FB8: D1A10074  stfs f13, 0x74(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 82EC9FBC: D0A10078  stfs f5, 0x78(r1)
	tmp.f32 = (ctx.f[5].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 82EC9FC0: D001007C  stfs f0, 0x7c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), tmp.u32 ) };
	// 82EC9FC4: 482DE54D  bl 0x831a8510
	ctx.lr = 0x82EC9FC8;
	sub_831A8510(ctx, base);
	// 82EC9FC8: 80FD002C  lwz r7, 0x2c(r29)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(44 as u32) ) } as u64;
	// 82EC9FCC: 80C70010  lwz r6, 0x10(r7)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(16 as u32) ) } as u64;
	// 82EC9FD0: 839D000C  lwz r28, 0xc(r29)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82EC9FD4: 83FE0004  lwz r31, 4(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82EC9FD8: 7F1C3000  cmpw cr6, r28, r6
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[6].s32, &mut ctx.xer);
	// 82EC9FDC: 419A00EC  beq cr6, 0x82eca0c8
	if ctx.cr[6].eq {
	pc = 0x82ECA0C8; continue 'dispatch;
	}
	// 82EC9FE0: 3B20FFFF  li r25, -1
	ctx.r[25].s64 = -1;
	// 82EC9FE4: 7F1CF800  cmpw cr6, r28, r31
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82EC9FE8: 40990058  ble cr6, 0x82eca040
	if !ctx.cr[6].gt {
	pc = 0x82ECA040; continue 'dispatch;
	}
	// 82EC9FEC: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82EC9FF0: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82EC9FF4: 7F0BE000  cmpw cr6, r11, r28
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[28].s32, &mut ctx.xer);
	// 82EC9FF8: 40980024  bge cr6, 0x82eca01c
	if !ctx.cr[6].lt {
	pc = 0x82ECA01C; continue 'dispatch;
	}
	// 82EC9FFC: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82ECA000: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82ECA004: 41980008  blt cr6, 0x82eca00c
	if ctx.cr[6].lt {
	pc = 0x82ECA00C; continue 'dispatch;
	}
	// 82ECA008: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 82ECA00C: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82ECA010: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82ECA014: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82ECA018: 4BFDC7E1  bl 0x82ea67f8
	ctx.lr = 0x82ECA01C;
	sub_82EA67F8(ctx, base);
	// 82ECA01C: 7F1FE000  cmpw cr6, r31, r28
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[28].s32, &mut ctx.xer);
	// 82ECA020: 40980020  bge cr6, 0x82eca040
	if !ctx.cr[6].lt {
	pc = 0x82ECA040; continue 'dispatch;
	}
	// 82ECA024: 57EA083C  slwi r10, r31, 1
	ctx.r[10].u32 = ctx.r[31].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82ECA028: 7D7FE050  subf r11, r31, r28
	ctx.r[11].s64 = ctx.r[28].s64 - ctx.r[31].s64;
	// 82ECA02C: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECA030: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ECA034: 7F2A4B2E  sthx r25, r10, r9
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32), ctx.r[25].u16) };
	// 82ECA038: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 82ECA03C: 4082FFF0  bne 0x82eca02c
	if !ctx.cr[0].eq {
	pc = 0x82ECA02C; continue 'dispatch;
	}
	// 82ECA040: 939E0004  stw r28, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82ECA044: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82ECA048: 817D002C  lwz r11, 0x2c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(44 as u32) ) } as u64;
	// 82ECA04C: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82ECA050: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ECA054: 4099011C  ble cr6, 0x82eca170
	if !ctx.cr[6].gt {
	pc = 0x82ECA170; continue 'dispatch;
	}
	// 82ECA058: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82ECA05C: 811D0020  lwz r8, 0x20(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32 as u32) ) } as u64;
	// 82ECA060: 7CC5402E  lwzx r6, r5, r8
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[5].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82ECA064: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 82ECA068: 41980044  blt cr6, 0x82eca0ac
	if ctx.cr[6].lt {
	pc = 0x82ECA0AC; continue 'dispatch;
	}
	// 82ECA06C: 813D002C  lwz r9, 0x2c(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(44 as u32) ) } as u64;
	// 82ECA070: 7CEB3B78  mr r11, r7
	ctx.r[11].u64 = ctx.r[7].u64;
	// 82ECA074: 7F2ACB78  mr r10, r25
	ctx.r[10].u64 = ctx.r[25].u64;
	// 82ECA078: 81290004  lwz r9, 4(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECA07C: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82ECA080: 7C8B4A2E  lhzx r4, r11, r9
	ctx.r[4].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82ECA084: 7C8B0734  extsh r11, r4
	ctx.r[11].s64 = ctx.r[4].s16 as i64;
	// 82ECA088: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ECA08C: 41980014  blt cr6, 0x82eca0a0
	if ctx.cr[6].lt {
	pc = 0x82ECA0A0; continue 'dispatch;
	}
	// 82ECA090: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82ECA094: 7D4A402E  lwzx r10, r10, r8
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82ECA098: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ECA09C: 4198FFE0  blt cr6, 0x82eca07c
	if ctx.cr[6].lt {
	pc = 0x82ECA07C; continue 'dispatch;
	}
	// 82ECA0A0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECA0A4: 54C9083C  slwi r9, r6, 1
	ctx.r[9].u32 = ctx.r[6].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82ECA0A8: 7D495B2E  sthx r10, r9, r11
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u16) };
	// 82ECA0AC: 817D002C  lwz r11, 0x2c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(44 as u32) ) } as u64;
	// 82ECA0B0: 38E70001  addi r7, r7, 1
	ctx.r[7].s64 = ctx.r[7].s64 + 1;
	// 82ECA0B4: 38A50004  addi r5, r5, 4
	ctx.r[5].s64 = ctx.r[5].s64 + 4;
	// 82ECA0B8: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82ECA0BC: 7F075000  cmpw cr6, r7, r10
	ctx.cr[6].compare_i32(ctx.r[7].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82ECA0C0: 4198FF9C  blt cr6, 0x82eca05c
	if ctx.cr[6].lt {
	pc = 0x82ECA05C; continue 'dispatch;
	}
	// 82ECA0C4: 480000AC  b 0x82eca170
	pc = 0x82ECA170; continue 'dispatch;
	// 82ECA0C8: 7F1CF800  cmpw cr6, r28, r31
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82ECA0CC: 4099005C  ble cr6, 0x82eca128
	if !ctx.cr[6].gt {
	pc = 0x82ECA128; continue 'dispatch;
	}
	// 82ECA0D0: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ECA0D4: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82ECA0D8: 7F0BE000  cmpw cr6, r11, r28
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[28].s32, &mut ctx.xer);
	// 82ECA0DC: 40980024  bge cr6, 0x82eca100
	if !ctx.cr[6].lt {
	pc = 0x82ECA100; continue 'dispatch;
	}
	// 82ECA0E0: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82ECA0E4: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82ECA0E8: 41980008  blt cr6, 0x82eca0f0
	if ctx.cr[6].lt {
	pc = 0x82ECA0F0; continue 'dispatch;
	}
	// 82ECA0EC: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 82ECA0F0: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82ECA0F4: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82ECA0F8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82ECA0FC: 4BFDC6FD  bl 0x82ea67f8
	ctx.lr = 0x82ECA100;
	sub_82EA67F8(ctx, base);
	// 82ECA100: 7F1FE000  cmpw cr6, r31, r28
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[28].s32, &mut ctx.xer);
	// 82ECA104: 40980024  bge cr6, 0x82eca128
	if !ctx.cr[6].lt {
	pc = 0x82ECA128; continue 'dispatch;
	}
	// 82ECA108: 57EA083C  slwi r10, r31, 1
	ctx.r[10].u32 = ctx.r[31].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82ECA10C: 7D7FE050  subf r11, r31, r28
	ctx.r[11].s64 = ctx.r[28].s64 - ctx.r[31].s64;
	// 82ECA110: 3B20FFFF  li r25, -1
	ctx.r[25].s64 = -1;
	// 82ECA114: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECA118: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ECA11C: 7F2A4B2E  sthx r25, r10, r9
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32), ctx.r[25].u16) };
	// 82ECA120: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 82ECA124: 4082FFF0  bne 0x82eca114
	if !ctx.cr[0].eq {
	pc = 0x82ECA114; continue 'dispatch;
	}
	// 82ECA128: 939E0004  stw r28, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82ECA12C: 7F6ADB78  mr r10, r27
	ctx.r[10].u64 = ctx.r[27].u64;
	// 82ECA130: 817D002C  lwz r11, 0x2c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(44 as u32) ) } as u64;
	// 82ECA134: 812B0010  lwz r9, 0x10(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82ECA138: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82ECA13C: 40990034  ble cr6, 0x82eca170
	if !ctx.cr[6].gt {
	pc = 0x82ECA170; continue 'dispatch;
	}
	// 82ECA140: 7F6BDB78  mr r11, r27
	ctx.r[11].u64 = ctx.r[27].u64;
	// 82ECA144: 813D002C  lwz r9, 0x2c(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(44 as u32) ) } as u64;
	// 82ECA148: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82ECA14C: 811E0000  lwz r8, 0(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECA150: 80E90004  lwz r7, 4(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECA154: 7CC75A2E  lhzx r6, r7, r11
	ctx.r[6].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82ECA158: 7CCB432E  sthx r6, r11, r8
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32), ctx.r[6].u16) };
	// 82ECA15C: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82ECA160: 80BD002C  lwz r5, 0x2c(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(44 as u32) ) } as u64;
	// 82ECA164: 80850010  lwz r4, 0x10(r5)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(16 as u32) ) } as u64;
	// 82ECA168: 7F0A2000  cmpw cr6, r10, r4
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[4].s32, &mut ctx.xer);
	// 82ECA16C: 4198FFD8  blt cr6, 0x82eca144
	if ctx.cr[6].lt {
	pc = 0x82ECA144; continue 'dispatch;
	}
	// 82ECA170: 939A0018  stw r28, 0x18(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(24 as u32), ctx.r[28].u32 ) };
	// 82ECA174: 3BFA0018  addi r31, r26, 0x18
	ctx.r[31].s64 = ctx.r[26].s64 + 24;
	// 82ECA178: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECA17C: 815D0008  lwz r10, 8(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ECA180: 917A0020  stw r11, 0x20(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82ECA184: 915A001C  stw r10, 0x1c(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 82ECA188: 81380008  lwz r9, 8(r24)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ECA18C: 552B00BE  clrlwi r11, r9, 2
	ctx.r[11].u64 = ctx.r[9].u32 as u64 & 0x3FFFFFFFu64;
	// 82ECA190: 7F0BE000  cmpw cr6, r11, r28
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[28].s32, &mut ctx.xer);
	// 82ECA194: 40980024  bge cr6, 0x82eca1b8
	if !ctx.cr[6].lt {
	pc = 0x82ECA1B8; continue 'dispatch;
	}
	// 82ECA198: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82ECA19C: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82ECA1A0: 41980008  blt cr6, 0x82eca1a8
	if ctx.cr[6].lt {
	pc = 0x82ECA1A8; continue 'dispatch;
	}
	// 82ECA1A4: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 82ECA1A8: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 82ECA1AC: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82ECA1B0: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82ECA1B4: 4BFDC645  bl 0x82ea67f8
	ctx.lr = 0x82ECA1B8;
	sub_82EA67F8(ctx, base);
	// 82ECA1B8: 93980004  stw r28, 4(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82ECA1BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECA1C0: 80980000  lwz r4, 0(r24)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECA1C4: 480005ED  bl 0x82eca7b0
	ctx.lr = 0x82ECA1C8;
	sub_82ECA7B0(ctx, base);
	// 82ECA1C8: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82ECA1CC: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82ECA1D0: 482DDFD8  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECA1D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ECA1D8 size=288
    let mut pc: u32 = 0x82ECA1D8;
    'dispatch: loop {
        match pc {
            0x82ECA1D8 => {
    //   block [0x82ECA1D8..0x82ECA2F8)
	// 82ECA1D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ECA1DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ECA1E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ECA1E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ECA1E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ECA1EC: 807F0034  lwz r3, 0x34(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82ECA1F0: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECA1F4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ECA1F8: 419A0030  beq cr6, 0x82eca228
	if ctx.cr[6].eq {
	pc = 0x82ECA228; continue 'dispatch;
	}
	// 82ECA1FC: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82ECA200: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 82ECA204: 7D490734  extsh r9, r10
	ctx.r[9].s64 = ctx.r[10].s16 as i64;
	// 82ECA208: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82ECA20C: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82ECA210: 409A0018  bne cr6, 0x82eca228
	if !ctx.cr[6].eq {
	pc = 0x82ECA228; continue 'dispatch;
	}
	// 82ECA214: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECA218: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82ECA21C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECA220: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ECA224: 4E800421  bctrl
	ctx.lr = 0x82ECA228;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECA228: 817F0040  lwz r11, 0x40(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82ECA22C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ECA230: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ECA234: 409A0020  bne cr6, 0x82eca254
	if !ctx.cr[6].eq {
	pc = 0x82ECA254; continue 'dispatch;
	}
	// 82ECA238: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECA23C: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82ECA240: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82ECA244: 809F0038  lwz r4, 0x38(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82ECA248: 5565087C  rlwinm r5, r11, 1, 1, 0x1e
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x7FFFFFFFu64;
	// 82ECA24C: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82ECA250: 4BFD6561  bl 0x82ea07b0
	ctx.lr = 0x82ECA254;
	sub_82EA07B0(ctx, base);
	// 82ECA254: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82ECA258: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ECA25C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ECA260: 409A0020  bne cr6, 0x82eca280
	if !ctx.cr[6].eq {
	pc = 0x82ECA280; continue 'dispatch;
	}
	// 82ECA264: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECA268: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82ECA26C: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82ECA270: 809F0028  lwz r4, 0x28(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82ECA274: 55653032  slwi r5, r11, 6
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(6);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82ECA278: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82ECA27C: 4BFD6535  bl 0x82ea07b0
	ctx.lr = 0x82ECA280;
	sub_82EA07B0(ctx, base);
	// 82ECA280: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82ECA284: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ECA288: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ECA28C: 409A0020  bne cr6, 0x82eca2ac
	if !ctx.cr[6].eq {
	pc = 0x82ECA2AC; continue 'dispatch;
	}
	// 82ECA290: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECA294: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82ECA298: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82ECA29C: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ECA2A0: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82ECA2A4: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82ECA2A8: 4BFD6509  bl 0x82ea07b0
	ctx.lr = 0x82ECA2AC;
	sub_82EA07B0(ctx, base);
	// 82ECA2AC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ECA2B0: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ECA2B4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ECA2B8: 409A002C  bne cr6, 0x82eca2e4
	if !ctx.cr[6].eq {
	pc = 0x82ECA2E4; continue 'dispatch;
	}
	// 82ECA2BC: 556A00BE  clrlwi r10, r11, 2
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82ECA2C0: 812D0000  lwz r9, 0(r13)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECA2C4: 556B087C  rlwinm r11, r11, 1, 1, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x7FFFFFFFu64;
	// 82ECA2C8: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECA2CC: 39000014  li r8, 0x14
	ctx.r[8].s64 = 20;
	// 82ECA2D0: 7CEA5A14  add r7, r10, r11
	ctx.r[7].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82ECA2D4: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82ECA2D8: 54E52036  slwi r5, r7, 4
	ctx.r[5].u32 = ctx.r[7].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82ECA2DC: 7C68482E  lwzx r3, r8, r9
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82ECA2E0: 4BFD64D1  bl 0x82ea07b0
	ctx.lr = 0x82ECA2E4;
	sub_82EA07B0(ctx, base);
	// 82ECA2E4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82ECA2E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ECA2EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ECA2F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ECA2F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECA2F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ECA2F8 size=524
    let mut pc: u32 = 0x82ECA2F8;
    'dispatch: loop {
        match pc {
            0x82ECA2F8 => {
    //   block [0x82ECA2F8..0x82ECA504)
	// 82ECA2F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ECA2FC: 482DDE55  bl 0x831a8150
	ctx.lr = 0x82ECA300;
	sub_831A8130(ctx, base);
	// 82ECA300: DBE1FFA0  stfd f31, -0x60(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-96 as u32), ctx.f[31].u64 ) };
	// 82ECA304: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ECA308: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECA508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82ECA508 size=20
    let mut pc: u32 = 0x82ECA508;
    'dispatch: loop {
        match pc {
            0x82ECA508 => {
    //   block [0x82ECA508..0x82ECA51C)
	// 82ECA508: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82ECA50C: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 82ECA510: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82ECA514: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 82ECA518: 4D980020  bltlr cr6
	if ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECA51C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ECA51C size=8
    let mut pc: u32 = 0x82ECA51C;
    'dispatch: loop {
        match pc {
            0x82ECA51C => {
    //   block [0x82ECA51C..0x82ECA524)
	// 82ECA51C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82ECA520: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECA528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ECA528 size=132
    let mut pc: u32 = 0x82ECA528;
    'dispatch: loop {
        match pc {
            0x82ECA528 => {
    //   block [0x82ECA528..0x82ECA5AC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECA5AC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ECA5AC size=28
    let mut pc: u32 = 0x82ECA5AC;
    'dispatch: loop {
        match pc {
            0x82ECA5AC => {
    //   block [0x82ECA5AC..0x82ECA5C8)
	// 82ECA5AC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECA5C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82ECA5C8 size=488
    let mut pc: u32 = 0x82ECA5C8;
    'dispatch: loop {
        match pc {
            0x82ECA5C8 => {
    //   block [0x82ECA5C8..0x82ECA7B0)
	// 82ECA5C8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82ECA5CC: C1A40000  lfs f13, 0(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82ECA5D0: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82ECA5D4: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82ECA5D8: 99230000  stb r9, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 82ECA5DC: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82ECA5E0: C18A08A8  lfs f12, 0x8a8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2216 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82ECA5E4: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82ECA5E8: 41980010  blt cr6, 0x82eca5f8
	if ctx.cr[6].lt {
	pc = 0x82ECA5F8; continue 'dispatch;
	}
	// 82ECA5EC: FF0D6000  fcmpu cr6, f13, f12
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[12].f64);
	// 82ECA5F0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82ECA5F4: 40990008  ble cr6, 0x82eca5fc
	if !ctx.cr[6].gt {
	pc = 0x82ECA5FC; continue 'dispatch;
	}
	// 82ECA5F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82ECA5FC: 7D6A0774  extsb r10, r11
	ctx.r[10].s64 = ctx.r[11].s8 as i64;
	// 82ECA600: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82ECA604: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ECA608: 419A001C  beq cr6, 0x82eca624
	if ctx.cr[6].eq {
	pc = 0x82ECA624; continue 'dispatch;
	}
	// 82ECA60C: C1A40004  lfs f13, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82ECA610: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82ECA614: 41980010  blt cr6, 0x82eca624
	if ctx.cr[6].lt {
	pc = 0x82ECA624; continue 'dispatch;
	}
	// 82ECA618: FF0D6000  fcmpu cr6, f13, f12
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[12].f64);
	// 82ECA61C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82ECA620: 40990008  ble cr6, 0x82eca628
	if !ctx.cr[6].gt {
	pc = 0x82ECA628; continue 'dispatch;
	}
	// 82ECA624: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82ECA628: 7D6A0774  extsb r10, r11
	ctx.r[10].s64 = ctx.r[11].s8 as i64;
	// 82ECA62C: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82ECA630: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ECA634: 419A001C  beq cr6, 0x82eca650
	if ctx.cr[6].eq {
	pc = 0x82ECA650; continue 'dispatch;
	}
	// 82ECA638: C1A40008  lfs f13, 8(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82ECA63C: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82ECA640: 41980010  blt cr6, 0x82eca650
	if ctx.cr[6].lt {
	pc = 0x82ECA650; continue 'dispatch;
	}
	// 82ECA644: FF0D6000  fcmpu cr6, f13, f12
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[12].f64);
	// 82ECA648: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82ECA64C: 40990008  ble cr6, 0x82eca654
	if !ctx.cr[6].gt {
	pc = 0x82ECA654; continue 'dispatch;
	}
	// 82ECA650: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82ECA654: 7D6A0774  extsb r10, r11
	ctx.r[10].s64 = ctx.r[11].s8 as i64;
	// 82ECA658: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82ECA65C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ECA660: 419A001C  beq cr6, 0x82eca67c
	if ctx.cr[6].eq {
	pc = 0x82ECA67C; continue 'dispatch;
	}
	// 82ECA664: C1A4000C  lfs f13, 0xc(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82ECA668: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82ECA66C: 41980010  blt cr6, 0x82eca67c
	if ctx.cr[6].lt {
	pc = 0x82ECA67C; continue 'dispatch;
	}
	// 82ECA670: FF0D6000  fcmpu cr6, f13, f12
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[12].f64);
	// 82ECA674: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82ECA678: 40990008  ble cr6, 0x82eca680
	if !ctx.cr[6].gt {
	pc = 0x82ECA680; continue 'dispatch;
	}
	// 82ECA67C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82ECA680: 7D6A0774  extsb r10, r11
	ctx.r[10].s64 = ctx.r[11].s8 as i64;
	// 82ECA684: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82ECA688: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ECA68C: 419A001C  beq cr6, 0x82eca6a8
	if ctx.cr[6].eq {
	pc = 0x82ECA6A8; continue 'dispatch;
	}
	// 82ECA690: C1A40010  lfs f13, 0x10(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82ECA694: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82ECA698: 41980010  blt cr6, 0x82eca6a8
	if ctx.cr[6].lt {
	pc = 0x82ECA6A8; continue 'dispatch;
	}
	// 82ECA69C: FF0D6000  fcmpu cr6, f13, f12
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[12].f64);
	// 82ECA6A0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82ECA6A4: 40990008  ble cr6, 0x82eca6ac
	if !ctx.cr[6].gt {
	pc = 0x82ECA6AC; continue 'dispatch;
	}
	// 82ECA6A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82ECA6AC: 7D6A0774  extsb r10, r11
	ctx.r[10].s64 = ctx.r[11].s8 as i64;
	// 82ECA6B0: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82ECA6B4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ECA6B8: 419A0014  beq cr6, 0x82eca6cc
	if ctx.cr[6].eq {
	pc = 0x82ECA6CC; continue 'dispatch;
	}
	// 82ECA6BC: C1A40014  lfs f13, 0x14(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82ECA6C0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82ECA6C4: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82ECA6C8: 40980008  bge cr6, 0x82eca6d0
	if !ctx.cr[6].lt {
	pc = 0x82ECA6D0; continue 'dispatch;
	}
	// 82ECA6CC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82ECA6D0: 7D6A0774  extsb r10, r11
	ctx.r[10].s64 = ctx.r[11].s8 as i64;
	// 82ECA6D4: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82ECA6D8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ECA6DC: 419A0014  beq cr6, 0x82eca6f0
	if ctx.cr[6].eq {
	pc = 0x82ECA6F0; continue 'dispatch;
	}
	// 82ECA6E0: C1A40018  lfs f13, 0x18(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82ECA6E4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82ECA6E8: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82ECA6EC: 40980008  bge cr6, 0x82eca6f4
	if !ctx.cr[6].lt {
	pc = 0x82ECA6F4; continue 'dispatch;
	}
	// 82ECA6F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82ECA6F4: 7D6A0774  extsb r10, r11
	ctx.r[10].s64 = ctx.r[11].s8 as i64;
	// 82ECA6F8: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82ECA6FC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ECA700: 419A0014  beq cr6, 0x82eca714
	if ctx.cr[6].eq {
	pc = 0x82ECA714; continue 'dispatch;
	}
	// 82ECA704: C1A4001C  lfs f13, 0x1c(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(28 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82ECA708: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82ECA70C: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82ECA710: 40980008  bge cr6, 0x82eca718
	if !ctx.cr[6].lt {
	pc = 0x82ECA718; continue 'dispatch;
	}
	// 82ECA714: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82ECA718: 7D6A0774  extsb r10, r11
	ctx.r[10].s64 = ctx.r[11].s8 as i64;
	// 82ECA71C: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82ECA720: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ECA724: 419A0014  beq cr6, 0x82eca738
	if ctx.cr[6].eq {
	pc = 0x82ECA738; continue 'dispatch;
	}
	// 82ECA728: C1A40020  lfs f13, 0x20(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(32 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82ECA72C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82ECA730: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82ECA734: 40980008  bge cr6, 0x82eca73c
	if !ctx.cr[6].lt {
	pc = 0x82ECA73C; continue 'dispatch;
	}
	// 82ECA738: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82ECA73C: 7D6A0774  extsb r10, r11
	ctx.r[10].s64 = ctx.r[11].s8 as i64;
	// 82ECA740: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82ECA744: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ECA748: 419A0014  beq cr6, 0x82eca75c
	if ctx.cr[6].eq {
	pc = 0x82ECA75C; continue 'dispatch;
	}
	// 82ECA74C: C1A40024  lfs f13, 0x24(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(36 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82ECA750: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82ECA754: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82ECA758: 40980008  bge cr6, 0x82eca760
	if !ctx.cr[6].lt {
	pc = 0x82ECA760; continue 'dispatch;
	}
	// 82ECA75C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82ECA760: 7D6A0774  extsb r10, r11
	ctx.r[10].s64 = ctx.r[11].s8 as i64;
	// 82ECA764: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82ECA768: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ECA76C: 419A0014  beq cr6, 0x82eca780
	if ctx.cr[6].eq {
	pc = 0x82ECA780; continue 'dispatch;
	}
	// 82ECA770: C1A40028  lfs f13, 0x28(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(40 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82ECA774: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82ECA778: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82ECA77C: 40980008  bge cr6, 0x82eca784
	if !ctx.cr[6].lt {
	pc = 0x82ECA784; continue 'dispatch;
	}
	// 82ECA780: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82ECA784: 7D6A0774  extsb r10, r11
	ctx.r[10].s64 = ctx.r[11].s8 as i64;
	// 82ECA788: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82ECA78C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ECA790: 419A0014  beq cr6, 0x82eca7a4
	if ctx.cr[6].eq {
	pc = 0x82ECA7A4; continue 'dispatch;
	}
	// 82ECA794: C1A4002C  lfs f13, 0x2c(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(44 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82ECA798: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82ECA79C: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82ECA7A0: 40980008  bge cr6, 0x82eca7a8
	if !ctx.cr[6].lt {
	pc = 0x82ECA7A8; continue 'dispatch;
	}
	// 82ECA7A4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82ECA7A8: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82ECA7AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECA7B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ECA7B0 size=128
    let mut pc: u32 = 0x82ECA7B0;
    'dispatch: loop {
        match pc {
            0x82ECA7B0 => {
    //   block [0x82ECA7B0..0x82ECA830)
	// 82ECA7B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ECA7B4: 482DD9B9  bl 0x831a816c
	ctx.lr = 0x82ECA7B8;
	sub_831A8130(ctx, base);
	// 82ECA7B8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECA7BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82ECA7C0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ECA7C4: 40990068  ble cr6, 0x82eca82c
	if !ctx.cr[6].gt {
	pc = 0x82ECA82C; continue 'dispatch;
	}
	// 82ECA7C8: 39640030  addi r11, r4, 0x30
	ctx.r[11].s64 = ctx.r[4].s64 + 48;
	// 82ECA7CC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82ECA7D0: 3BC001A0  li r30, 0x1a0
	ctx.r[30].s64 = 416;
	// 82ECA7D4: 3BE0FFF0  li r31, -0x10
	ctx.r[31].s64 = -16;
	// 82ECA7D8: 388001B0  li r4, 0x1b0
	ctx.r[4].s64 = 432;
	// 82ECA7DC: 38A00130  li r5, 0x130
	ctx.r[5].s64 = 304;
	// 82ECA7E0: 38C0FFD0  li r6, -0x30
	ctx.r[6].s64 = -48;
	// 82ECA7E4: 38E00150  li r7, 0x150
	ctx.r[7].s64 = 336;
	// 82ECA7E8: 3900FFE0  li r8, -0x20
	ctx.r[8].s64 = -32;
	// 82ECA7EC: 83A30004  lwz r29, 4(r3)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECA7F0: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82ECA7F4: 7FAAE82E  lwzx r29, r10, r29
	ctx.r[29].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82ECA7F8: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECA830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ECA830 size=32
    let mut pc: u32 = 0x82ECA830;
    'dispatch: loop {
        match pc {
            0x82ECA830 => {
    //   block [0x82ECA830..0x82ECA850)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECA850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82ECA850 size=48
    let mut pc: u32 = 0x82ECA850;
    'dispatch: loop {
        match pc {
            0x82ECA850 => {
    //   block [0x82ECA850..0x82ECA880)
	// 82ECA850: EDA0002C  fsqrts f13, f0
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].f64 = ((ctx.f[0].f64).sqrt() as f32) as f64;
	// 82ECA854: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82ECA858: 3941FFF0  addi r10, r1, -0x10
	ctx.r[10].s64 = ctx.r[1].s64 + -16;
	// 82ECA85C: C00B08A8  lfs f0, 0x8a8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82ECA860: ED806824  fdivs f12, f0, f13
	ctx.f[12].f64 = ((ctx.f[0].f64 / ctx.f[13].f64) as f32) as f64;
	// 82ECA864: ED6C0072  fmuls f11, f12, f1
	ctx.f[11].f64 = (((ctx.f[12].f64 * ctx.f[1].f64) as f32) as f64);
	// 82ECA868: D161FFF0  stfs f11, -0x10(r1)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECA880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ECA880 size=40
    let mut pc: u32 = 0x82ECA880;
    'dispatch: loop {
        match pc {
            0x82ECA880 => {
    //   block [0x82ECA880..0x82ECA8A8)
	// 82ECA880: FDA00890  fmr f13, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].f64 = ctx.f[1].f64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECA8A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82ECA8A8 size=48
    let mut pc: u32 = 0x82ECA8A8;
    'dispatch: loop {
        match pc {
            0x82ECA8A8 => {
    //   block [0x82ECA8A8..0x82ECA8D8)
	// 82ECA8A8: ED6200B2  fmuls f11, f2, f2
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[11].f64 = (((ctx.f[2].f64 * ctx.f[2].f64) as f32) as f64);
	// 82ECA8AC: 3961FFF0  addi r11, r1, -0x10
	ctx.r[11].s64 = ctx.r[1].s64 + -16;
	// 82ECA8B0: FF005800  fcmpu cr6, f0, f11
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[11].f64);
	// 82ECA8B4: 40980024  bge cr6, 0x82eca8d8
	if !ctx.cr[6].lt {
		sub_82ECA8D8(ctx, base);
		return;
	}
	// 82ECA8B8: EC0C0024  fdivs f0, f12, f0
	ctx.f[0].f64 = ((ctx.f[12].f64 / ctx.f[0].f64) as f32) as f64;
	// 82ECA8BC: EDA0002C  fsqrts f13, f0
	ctx.f[13].f64 = ((ctx.f[0].f64).sqrt() as f32) as f64;
	// 82ECA8C0: D1A1FFF0  stfs f13, -0x10(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECA8D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82ECA8D8 size=32
    let mut pc: u32 = 0x82ECA8D8;
    'dispatch: loop {
        match pc {
            0x82ECA8D8 => {
    //   block [0x82ECA8D8..0x82ECA8F8)
	// 82ECA8D8: EDAD00B2  fmuls f13, f13, f2
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[2].f64) as f32) as f64);
	// 82ECA8DC: ED8D0024  fdivs f12, f13, f0
	ctx.f[12].f64 = ((ctx.f[13].f64 / ctx.f[0].f64) as f32) as f64;
	// 82ECA8E0: D181FFF0  stfs f12, -0x10(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECA8F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82ECA8F8 size=1008
    let mut pc: u32 = 0x82ECA8F8;
    'dispatch: loop {
        match pc {
            0x82ECA8F8 => {
    //   block [0x82ECA8F8..0x82ECACE8)
	// 82ECA8F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ECA8FC: 482DD835  bl 0x831a8130
	ctx.lr = 0x82ECA900;
	sub_831A8130(ctx, base);
	// 82ECA900: DBC1FF58  stfd f30, -0xa8(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-168 as u32), ctx.f[30].u64 ) };
	// 82ECA904: DBE1FF60  stfd f31, -0xa0(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-160 as u32), ctx.f[31].u64 ) };
	// 82ECA908: 9421FE50  stwu r1, -0x1b0(r1)
	ea = ctx.r[1].u32.wrapping_add(-432 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ECA90C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82ECA910: 90A101D4  stw r5, 0x1d4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(468 as u32), ctx.r[5].u32 ) };
	// 82ECA914: 7CE43B78  mr r4, r7
	ctx.r[4].u64 = ctx.r[7].u64;
	// 82ECA918: 90C101DC  stw r6, 0x1dc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(476 as u32), ctx.r[6].u32 ) };
	// 82ECA91C: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82ECA920: 908101E4  stw r4, 0x1e4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(484 as u32), ctx.r[4].u32 ) };
	// 82ECA924: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECA928: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ECA92C: 409903AC  ble cr6, 0x82ecacd8
	if !ctx.cr[6].gt {
	pc = 0x82ECACD8; continue 'dispatch;
	}
	// 82ECA930: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82ECA934: 3CE08200  lis r7, -0x7e00
	ctx.r[7].s64 = -2113929216;
	// 82ECA938: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82ECA93C: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82ECA940: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 82ECA944: 38CAFFA0  addi r6, r10, -0x60
	ctx.r[6].s64 = ctx.r[10].s64 + -96;
	// 82ECA948: C3C808A8  lfs f30, 0x8a8(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(2216 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 82ECA94C: C3E708A4  lfs f31, 0x8a4(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82ECA950: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82ECA954: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 82ECA958: 90C10058  stw r6, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[6].u32 ) };
	// 82ECA95C: 3BC30010  addi r30, r3, 0x10
	ctx.r[30].s64 = ctx.r[3].s64 + 16;
	// 82ECA960: 3BE40040  addi r31, r4, 0x40
	ctx.r[31].s64 = ctx.r[4].s64 + 64;
	// 82ECA964: 3A600020  li r19, 0x20
	ctx.r[19].s64 = 32;
	// 82ECA968: 3A800010  li r20, 0x10
	ctx.r[20].s64 = 16;
	// 82ECA96C: 3AA0FFC0  li r21, -0x40
	ctx.r[21].s64 = -64;
	// 82ECA970: 3AC0FFD0  li r22, -0x30
	ctx.r[22].s64 = -48;
	// 82ECA974: 3AE0FFE0  li r23, -0x20
	ctx.r[23].s64 = -32;
	// 82ECA978: 3B0B7060  addi r24, r11, 0x7060
	ctx.r[24].s64 = ctx.r[11].s64 + 28768;
	// 82ECA97C: 3B890010  addi r28, r9, 0x10
	ctx.r[28].s64 = ctx.r[9].s64 + 16;
	// 82ECA980: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ECA984: 7D4BCA2E  lhzx r10, r11, r25
	ctx.r[10].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 82ECA988: 7D460734  extsh r6, r10
	ctx.r[6].s64 = ctx.r[10].s16 as i64;
	// 82ECA98C: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 82ECA990: 4198001C  blt cr6, 0x82eca9ac
	if ctx.cr[6].lt {
	pc = 0x82ECA9AC; continue 'dispatch;
	}
	// 82ECA994: 54CB083C  slwi r11, r6, 1
	ctx.r[11].u32 = ctx.r[6].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82ECA998: 7D665A14  add r11, r6, r11
	ctx.r[11].u64 = ctx.r[6].u64 + ctx.r[11].u64;
	// 82ECA99C: 556B2834  slwi r11, r11, 5
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82ECA9A0: 7D6B2214  add r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82ECA9A4: 396B0030  addi r11, r11, 0x30
	ctx.r[11].s64 = ctx.r[11].s64 + 48;
	// 82ECA9A8: 48000008  b 0x82eca9b0
	pc = 0x82ECA9B0; continue 'dispatch;
	// 82ECA9AC: 816101DC  lwz r11, 0x1dc(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(476 as u32) ) } as u64;
	// 82ECA9B0: 394B0010  addi r10, r11, 0x10
	ctx.r[10].s64 = ctx.r[11].s64 + 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECACE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ECACE8 size=1572
    let mut pc: u32 = 0x82ECACE8;
    'dispatch: loop {
        match pc {
            0x82ECACE8 => {
    //   block [0x82ECACE8..0x82ECB30C)
	// 82ECACE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ECACEC: 482DD445  bl 0x831a8130
	ctx.lr = 0x82ECACF0;
	sub_831A8130(ctx, base);
	// 82ECACF0: DBA1FF50  stfd f29, -0xb0(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-176 as u32), ctx.f[29].u64 ) };
	// 82ECACF4: DBC1FF58  stfd f30, -0xa8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-168 as u32), ctx.f[30].u64 ) };
	// 82ECACF8: DBE1FF60  stfd f31, -0xa0(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-160 as u32), ctx.f[31].u64 ) };
	// 82ECACFC: 3980FF30  li r12, -0xd0
	ctx.r[12].s64 = -208;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECB310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ECB310 size=32
    let mut pc: u32 = 0x82ECB310;
    'dispatch: loop {
        match pc {
            0x82ECB310 => {
    //   block [0x82ECB310..0x82ECB330)
	// 82ECB310: 89630009  lbz r11, 9(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(9 as u32) ) } as u64;
	// 82ECB314: 54CA07BE  clrlwi r10, r6, 0x1e
	ctx.r[10].u64 = ctx.r[6].u32 as u64 & 0x00000003u64;
	// 82ECB318: 556907BE  clrlwi r9, r11, 0x1e
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 82ECB31C: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82ECB320: 40980010  bge cr6, 0x82ecb330
	if !ctx.cr[6].lt {
		sub_82ECB330(ctx, base);
		return;
	}
	// 82ECB324: 7C8A20F8  nor r10, r4, r4
	ctx.r[10].u64 = !(ctx.r[4].u64 | ctx.r[4].u64);
	// 82ECB328: 554A7022  slwi r10, r10, 0xe
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(14);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82ECB32C: 48000008  b 0x82ecb334
	sub_82ECB330(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECB330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ECB330 size=24
    let mut pc: u32 = 0x82ECB330;
    'dispatch: loop {
        match pc {
            0x82ECB330 => {
    //   block [0x82ECB330..0x82ECB348)
	// 82ECB330: 548A7022  slwi r10, r4, 0xe
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(14);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82ECB334: 7F065800  cmpw cr6, r6, r11
	ctx.cr[6].compare_i32(ctx.r[6].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82ECB338: 40980010  bge cr6, 0x82ecb348
	if !ctx.cr[6].lt {
		sub_82ECB348(ctx, base);
		return;
	}
	// 82ECB33C: 7CAB28F8  nor r11, r5, r5
	ctx.r[11].u64 = !(ctx.r[5].u64 | ctx.r[5].u64);
	// 82ECB340: 556B7022  slwi r11, r11, 0xe
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(14);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82ECB344: 48000008  b 0x82ecb34c
	sub_82ECB348(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECB348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ECB348 size=40
    let mut pc: u32 = 0x82ECB348;
    'dispatch: loop {
        match pc {
            0x82ECB348 => {
    //   block [0x82ECB348..0x82ECB370)
	// 82ECB348: 54AB7022  slwi r11, r5, 0xe
	ctx.r[11].u32 = ctx.r[5].u32.wrapping_shl(14);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82ECB34C: A123000A  lhz r9, 0xa(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(10 as u32) ) } as u64;
	// 82ECB350: A103000C  lhz r8, 0xc(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ECB354: 552704BE  clrlwi r7, r9, 0x12
	ctx.r[7].u64 = ctx.r[9].u32 as u64 & 0x00003FFFu64;
	// 82ECB358: 550604BE  clrlwi r6, r8, 0x12
	ctx.r[6].u64 = ctx.r[8].u32 as u64 & 0x00003FFFu64;
	// 82ECB35C: 7CE55378  or r5, r7, r10
	ctx.r[5].u64 = ctx.r[7].u64 | ctx.r[10].u64;
	// 82ECB360: 7CC45B78  or r4, r6, r11
	ctx.r[4].u64 = ctx.r[6].u64 | ctx.r[11].u64;
	// 82ECB364: B0A3000A  sth r5, 0xa(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(10 as u32), ctx.r[5].u16 ) };
	// 82ECB368: B083000C  sth r4, 0xc(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[4].u16 ) };
	// 82ECB36C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECB370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ECB370 size=48
    let mut pc: u32 = 0x82ECB370;
    'dispatch: loop {
        match pc {
            0x82ECB370 => {
    //   block [0x82ECB370..0x82ECB3A0)
	// 82ECB370: 548B063E  clrlwi r11, r4, 0x18
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 82ECB374: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ECB378: 419A0068  beq cr6, 0x82ecb3e0
	if ctx.cr[6].eq {
		sub_82ECB3E0(ctx, base);
		return;
	}
	// 82ECB37C: 54AB073E  clrlwi r11, r5, 0x1c
	ctx.r[11].u64 = ctx.r[5].u32 as u64 & 0x0000000Fu64;
	// 82ECB380: 54AA07BE  clrlwi r10, r5, 0x1e
	ctx.r[10].u64 = ctx.r[5].u32 as u64 & 0x00000003u64;
	// 82ECB384: 550907BE  clrlwi r9, r8, 0x1e
	ctx.r[9].u64 = ctx.r[8].u32 as u64 & 0x00000003u64;
	// 82ECB388: 99630009  stb r11, 9(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(9 as u32), ctx.r[11].u8 ) };
	// 82ECB38C: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82ECB390: 40980010  bge cr6, 0x82ecb3a0
	if !ctx.cr[6].lt {
		sub_82ECB3A0(ctx, base);
		return;
	}
	// 82ECB394: 7CCA30F8  nor r10, r6, r6
	ctx.r[10].u64 = !(ctx.r[6].u64 | ctx.r[6].u64);
	// 82ECB398: 554A7022  slwi r10, r10, 0xe
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(14);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82ECB39C: 48000008  b 0x82ecb3a4
	sub_82ECB3A0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECB3A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ECB3A0 size=24
    let mut pc: u32 = 0x82ECB3A0;
    'dispatch: loop {
        match pc {
            0x82ECB3A0 => {
    //   block [0x82ECB3A0..0x82ECB3B8)
	// 82ECB3A0: 54CA7022  slwi r10, r6, 0xe
	ctx.r[10].u32 = ctx.r[6].u32.wrapping_shl(14);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82ECB3A4: 7F085800  cmpw cr6, r8, r11
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82ECB3A8: 40980010  bge cr6, 0x82ecb3b8
	if !ctx.cr[6].lt {
		sub_82ECB3B8(ctx, base);
		return;
	}
	// 82ECB3AC: 7CEB38F8  nor r11, r7, r7
	ctx.r[11].u64 = !(ctx.r[7].u64 | ctx.r[7].u64);
	// 82ECB3B0: 556B7022  slwi r11, r11, 0xe
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(14);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82ECB3B4: 48000008  b 0x82ecb3bc
	sub_82ECB3B8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECB3B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ECB3B8 size=40
    let mut pc: u32 = 0x82ECB3B8;
    'dispatch: loop {
        match pc {
            0x82ECB3B8 => {
    //   block [0x82ECB3B8..0x82ECB3E0)
	// 82ECB3B8: 54EB7022  slwi r11, r7, 0xe
	ctx.r[11].u32 = ctx.r[7].u32.wrapping_shl(14);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82ECB3BC: A123000A  lhz r9, 0xa(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(10 as u32) ) } as u64;
	// 82ECB3C0: A103000C  lhz r8, 0xc(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ECB3C4: 552704BE  clrlwi r7, r9, 0x12
	ctx.r[7].u64 = ctx.r[9].u32 as u64 & 0x00003FFFu64;
	// 82ECB3C8: 550604BE  clrlwi r6, r8, 0x12
	ctx.r[6].u64 = ctx.r[8].u32 as u64 & 0x00003FFFu64;
	// 82ECB3CC: 7CE55378  or r5, r7, r10
	ctx.r[5].u64 = ctx.r[7].u64 | ctx.r[10].u64;
	// 82ECB3D0: 7CC45B78  or r4, r6, r11
	ctx.r[4].u64 = ctx.r[6].u64 | ctx.r[11].u64;
	// 82ECB3D4: B0A3000A  sth r5, 0xa(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(10 as u32), ctx.r[5].u16 ) };
	// 82ECB3D8: B083000C  sth r4, 0xc(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[4].u16 ) };
	// 82ECB3DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECB3E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ECB3E0 size=24
    let mut pc: u32 = 0x82ECB3E0;
    'dispatch: loop {
        match pc {
            0x82ECB3E0 => {
    //   block [0x82ECB3E0..0x82ECB3F8)
	// 82ECB3E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82ECB3E4: 394000FF  li r10, 0xff
	ctx.r[10].s64 = 255;
	// 82ECB3E8: B163000A  sth r11, 0xa(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(10 as u32), ctx.r[11].u16 ) };
	// 82ECB3EC: 99430009  stb r10, 9(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(9 as u32), ctx.r[10].u8 ) };
	// 82ECB3F0: B163000C  sth r11, 0xc(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u16 ) };
	// 82ECB3F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECB3F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82ECB3F8 size=64
    let mut pc: u32 = 0x82ECB3F8;
    'dispatch: loop {
        match pc {
            0x82ECB3F8 => {
    //   block [0x82ECB3F8..0x82ECB438)
	// 82ECB3F8: C0050004  lfs f0, 4(r5)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82ECB3FC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82ECB400: C1850000  lfs f12, 0(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82ECB404: ED6C0028  fsubs f11, f12, f0
	ctx.f[11].f64 = (((ctx.f[12].f64 - ctx.f[0].f64) as f32) as f64);
	// 82ECB408: C1450008  lfs f10, 8(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82ECB40C: C1AB9450  lfs f13, -0x6bb0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27568 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82ECB410: FD2B602E  fsel f9, f11, f0, f12
	ctx.f[9].f64 = if ctx.f[11].f64 >= 0.0 { ctx.f[0].f64 } else { ctx.f[12].f64 };
	// 82ECB414: ED095028  fsubs f8, f9, f10
	ctx.f[8].f64 = (((ctx.f[9].f64 - ctx.f[10].f64) as f32) as f64);
	// 82ECB418: FC084AAE  fsel f0, f8, f10, f9
	ctx.f[0].f64 = if ctx.f[8].f64 >= 0.0 { ctx.f[10].f64 } else { ctx.f[9].f64 };
	// 82ECB41C: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82ECB420: 40980018  bge cr6, 0x82ecb438
	if !ctx.cr[6].lt {
		sub_82ECB438(ctx, base);
		return;
	}
	// 82ECB424: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82ECB428: C1AB6150  lfs f13, 0x6150(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24912 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82ECB42C: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 82ECB430: D0030048  stfs f0, 0x48(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), tmp.u32 ) };
	// 82ECB434: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECB438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82ECB438 size=16
    let mut pc: u32 = 0x82ECB438;
    'dispatch: loop {
        match pc {
            0x82ECB438 => {
    //   block [0x82ECB438..0x82ECB448)
	// 82ECB438: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82ECB43C: C00B964C  lfs f0, -0x69b4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27060 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82ECB440: D0030048  stfs f0, 0x48(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), tmp.u32 ) };
	// 82ECB444: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECB448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ECB448 size=16
    let mut pc: u32 = 0x82ECB448;
    'dispatch: loop {
        match pc {
            0x82ECB448 => {
    //   block [0x82ECB448..0x82ECB458)
	// 82ECB448: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82ECB44C: 394B3F10  addi r10, r11, 0x3f10
	ctx.r[10].s64 = ctx.r[11].s64 + 16144;
	// 82ECB450: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82ECB454: 4800E7CC  b 0x82ed9c20
	sub_82ED9C20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECB458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ECB458 size=8
    let mut pc: u32 = 0x82ECB458;
    'dispatch: loop {
        match pc {
            0x82ECB458 => {
    //   block [0x82ECB458..0x82ECB460)
	// 82ECB458: 386300E0  addi r3, r3, 0xe0
	ctx.r[3].s64 = ctx.r[3].s64 + 224;
	// 82ECB45C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECB460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ECB460 size=20
    let mut pc: u32 = 0x82ECB460;
    'dispatch: loop {
        match pc {
            0x82ECB460 => {
    //   block [0x82ECB460..0x82ECB474)
	// 82ECB460: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82ECB464: 2F040001  cmpwi cr6, r4, 1
	ctx.cr[6].compare_i32(ctx.r[4].s32, 1, &mut ctx.xer);
	// 82ECB468: 419A0048  beq cr6, 0x82ecb4b0
	if ctx.cr[6].eq {
		sub_82ECB4B0(ctx, base);
		return;
	}
	// 82ECB46C: 2F040002  cmpwi cr6, r4, 2
	ctx.cr[6].compare_i32(ctx.r[4].s32, 2, &mut ctx.xer);
	// 82ECB470: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECB474(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ECB474 size=44
    let mut pc: u32 = 0x82ECB474;
    'dispatch: loop {
        match pc {
            0x82ECB474 => {
    //   block [0x82ECB474..0x82ECB4A0)
	// 82ECB474: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ECB478: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82ECB47C: 80AB00C0  lwz r5, 0xc0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(192 as u32) ) } as u64;
	// 82ECB480: 386B00D0  addi r3, r11, 0xd0
	ctx.r[3].s64 = ctx.r[11].s64 + 208;
	// 82ECB484: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82ECB488: 419A0018  beq cr6, 0x82ecb4a0
	if ctx.cr[6].eq {
		sub_82ECB4A0(ctx, base);
		return;
	}
	// 82ECB48C: 554A003E  slwi r10, r10, 0
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82ECB490: 890A02C7  lbz r8, 0x2c7(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(711 as u32) ) } as u64;
	// 82ECB494: 88EA02C6  lbz r7, 0x2c6(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(710 as u32) ) } as u64;
	// 82ECB498: 88CA02C5  lbz r6, 0x2c5(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(709 as u32) ) } as u64;
	// 82ECB49C: 4BFFFED4  b 0x82ecb370
	sub_82ECB370(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECB4A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ECB4A0 size=16
    let mut pc: u32 = 0x82ECB4A0;
    'dispatch: loop {
        match pc {
            0x82ECB4A0 => {
    //   block [0x82ECB4A0..0x82ECB4B0)
	// 82ECB4A0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82ECB4A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82ECB4A8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82ECB4AC: 4BFFFEC4  b 0x82ecb370
	sub_82ECB370(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECB4B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ECB4B0 size=28
    let mut pc: u32 = 0x82ECB4B0;
    'dispatch: loop {
        match pc {
            0x82ECB4B0 => {
    //   block [0x82ECB4B0..0x82ECB4CC)
	// 82ECB4B0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82ECB4B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82ECB4B8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82ECB4BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82ECB4C0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82ECB4C4: 386B00D0  addi r3, r11, 0xd0
	ctx.r[3].s64 = ctx.r[11].s64 + 208;
	// 82ECB4C8: 4BFFFEA8  b 0x82ecb370
	sub_82ECB370(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECB4CC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ECB4CC size=4
    let mut pc: u32 = 0x82ECB4CC;
    'dispatch: loop {
        match pc {
            0x82ECB4CC => {
    //   block [0x82ECB4CC..0x82ECB4D0)
	// 82ECB4CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECB4D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ECB4D0 size=28
    let mut pc: u32 = 0x82ECB4D0;
    'dispatch: loop {
        match pc {
            0x82ECB4D0 => {
    //   block [0x82ECB4D0..0x82ECB4EC)
	// 82ECB4D0: 896300D9  lbz r11, 0xd9(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(217 as u32) ) } as u64;
	// 82ECB4D4: 396BFF01  addi r11, r11, -0xff
	ctx.r[11].s64 = ctx.r[11].s64 + -255;
	// 82ECB4D8: 7D6A0034  cntlzw r10, r11
	ctx.r[10].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82ECB4DC: 5549DFFE  rlwinm r9, r10, 0x1b, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 82ECB4E0: 692B0001  xori r11, r9, 1
	ctx.r[11].u64 = ctx.r[9].u64 ^ 1;
	// 82ECB4E4: 386B0001  addi r3, r11, 1
	ctx.r[3].s64 = ctx.r[11].s64 + 1;
	// 82ECB4E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECB4F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ECB4F0 size=20
    let mut pc: u32 = 0x82ECB4F0;
    'dispatch: loop {
        match pc {
            0x82ECB4F0 => {
    //   block [0x82ECB4F0..0x82ECB504)
	// 82ECB4F0: 816300D0  lwz r11, 0xd0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(208 as u32) ) } as u64;
	// 82ECB4F4: 386300D0  addi r3, r3, 0xd0
	ctx.r[3].s64 = ctx.r[3].s64 + 208;
	// 82ECB4F8: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ECB4FC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ECB500: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECB508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ECB508 size=20
    let mut pc: u32 = 0x82ECB508;
    'dispatch: loop {
        match pc {
            0x82ECB508 => {
    //   block [0x82ECB508..0x82ECB51C)
	// 82ECB508: 816300D0  lwz r11, 0xd0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(208 as u32) ) } as u64;
	// 82ECB50C: 386300D0  addi r3, r3, 0xd0
	ctx.r[3].s64 = ctx.r[3].s64 + 208;
	// 82ECB510: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82ECB514: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ECB518: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECB520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82ECB520 size=8
    let mut pc: u32 = 0x82ECB520;
    'dispatch: loop {
        match pc {
            0x82ECB520 => {
    //   block [0x82ECB520..0x82ECB528)
	// 82ECB520: D0230084  stfs f1, 0x84(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(132 as u32), tmp.u32 ) };
	// 82ECB524: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECB528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82ECB528 size=8
    let mut pc: u32 = 0x82ECB528;
    'dispatch: loop {
        match pc {
            0x82ECB528 => {
    //   block [0x82ECB528..0x82ECB530)
	// 82ECB528: D0230088  stfs f1, 0x88(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(136 as u32), tmp.u32 ) };
	// 82ECB52C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECB530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ECB530 size=20
    let mut pc: u32 = 0x82ECB530;
    'dispatch: loop {
        match pc {
            0x82ECB530 => {
    //   block [0x82ECB530..0x82ECB544)
	// 82ECB530: 816300D0  lwz r11, 0xd0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(208 as u32) ) } as u64;
	// 82ECB534: 386300D0  addi r3, r3, 0xd0
	ctx.r[3].s64 = ctx.r[3].s64 + 208;
	// 82ECB538: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82ECB53C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ECB540: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECB548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ECB548 size=20
    let mut pc: u32 = 0x82ECB548;
    'dispatch: loop {
        match pc {
            0x82ECB548 => {
    //   block [0x82ECB548..0x82ECB55C)
	// 82ECB548: 816300D0  lwz r11, 0xd0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(208 as u32) ) } as u64;
	// 82ECB54C: 386300D0  addi r3, r3, 0xd0
	ctx.r[3].s64 = ctx.r[3].s64 + 208;
	// 82ECB550: 814B0020  lwz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82ECB554: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ECB558: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECB560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82ECB560 size=268
    let mut pc: u32 = 0x82ECB560;
    'dispatch: loop {
        match pc {
            0x82ECB560 => {
    //   block [0x82ECB560..0x82ECB66C)
	// 82ECB560: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ECB564: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ECB568: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82ECB56C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ECB570: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ECB574: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82ECB578: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82ECB57C: 3D008338  lis r8, -0x7cc8
	ctx.r[8].s64 = -2093481984;
	// 82ECB580: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ECB584: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82ECB588: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECB58C: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 82ECB590: C02A08A4  lfs f1, 0x8a4(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82ECB594: 388869F0  addi r4, r8, 0x69f0
	ctx.r[4].s64 = ctx.r[8].s64 + 27120;
	// 82ECB598: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82ECB59C: 80E9001C  lwz r7, 0x1c(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(28 as u32) ) } as u64;
	// 82ECB5A0: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 82ECB5A4: 4E800421  bctrl
	ctx.lr = 0x82ECB5A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECB5A8: 38C10070  addi r6, r1, 0x70
	ctx.r[6].s64 = ctx.r[1].s64 + 112;
	// 82ECB5AC: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82ECB5B0: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82ECB5B4: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82ECB5B8: 39210060  addi r9, r1, 0x60
	ctx.r[9].s64 = ctx.r[1].s64 + 96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECB670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82ECB670 size=620
    let mut pc: u32 = 0x82ECB670;
    'dispatch: loop {
        match pc {
            0x82ECB670 => {
    //   block [0x82ECB670..0x82ECB6E0)
	// 82ECB670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ECB674: 482DCAF5  bl 0x831a8168
	ctx.lr = 0x82ECB678;
	sub_831A8130(ctx, base);
	// 82ECB678: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 82ECB67C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ECB680: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82ECB684: D04100C4  stfs f2, 0xc4(r1)
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(196 as u32), tmp.u32 ) };
	// 82ECB688: D06100CC  stfs f3, 0xcc(r1)
	tmp.f32 = (ctx.f[3].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(204 as u32), tmp.u32 ) };
	// 82ECB68C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82ECB690: 397EFFFF  addi r11, r30, -1
	ctx.r[11].s64 = ctx.r[30].s64 + -1;
	// 82ECB694: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 82ECB698: 7D1C4378  mr r28, r8
	ctx.r[28].u64 = ctx.r[8].u64;
	// 82ECB69C: 2B0B0008  cmplwi cr6, r11, 8
	ctx.cr[6].compare_u32(ctx.r[11].u32, 8 as u32, &mut ctx.xer);
	// 82ECB6A0: 419901AC  bgt cr6, 0x82ecb84c
	if ctx.cr[6].gt {
	pc = 0x82ECB84C; continue 'dispatch;
	}
	// 82ECB6A4: 3D8082ED  lis r12, -0x7d13
	ctx.r[12].s64 = -2098397184;
	// 82ECB6A8: 398CB6BC  addi r12, r12, -0x4944
	ctx.r[12].s64 = ctx.r[12].s64 + -18756;
	// 82ECB6AC: 5560103A  slwi r0, r11, 2
	ctx.r[0].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 82ECB6B0: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 82ECB6B4: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 82ECB6B8: 4E800420  bctr
	match ctx.r[11].u64 {
		0 => {
	pc = 0x82ECB788; continue 'dispatch;
		},
		1 => {
	pc = 0x82ECB6E0; continue 'dispatch;
		},
		2 => {
	pc = 0x82ECB740; continue 'dispatch;
		},
		3 => {
	pc = 0x82ECB710; continue 'dispatch;
		},
		4 => {
	pc = 0x82ECB770; continue 'dispatch;
		},
		5 => {
	pc = 0x82ECB7F4; continue 'dispatch;
		},
		6 => {
	pc = 0x82ECB84C; continue 'dispatch;
		},
		7 => {
	pc = 0x82ECB728; continue 'dispatch;
		},
		8 => {
	pc = 0x82ECB81C; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 82ECB6BC: 82ECB788  lwz r23, -0x4878(r12)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[12].u32.wrapping_add(-18552 as u32) ) } as u64;
	// 82ECB6C0: 82ECB6E0  lwz r23, -0x4920(r12)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[12].u32.wrapping_add(-18720 as u32) ) } as u64;
	// 82ECB6C4: 82ECB740  lwz r23, -0x48c0(r12)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[12].u32.wrapping_add(-18624 as u32) ) } as u64;
	// 82ECB6C8: 82ECB710  lwz r23, -0x48f0(r12)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[12].u32.wrapping_add(-18672 as u32) ) } as u64;
	// 82ECB6CC: 82ECB770  lwz r23, -0x4890(r12)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[12].u32.wrapping_add(-18576 as u32) ) } as u64;
	// 82ECB6D0: 82ECB7F4  lwz r23, -0x480c(r12)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[12].u32.wrapping_add(-18444 as u32) ) } as u64;
	// 82ECB6D4: 82ECB84C  lwz r23, -0x47b4(r12)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[12].u32.wrapping_add(-18356 as u32) ) } as u64;
	// 82ECB6D8: 82ECB728  lwz r23, -0x48d8(r12)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[12].u32.wrapping_add(-18648 as u32) ) } as u64;
	// 82ECB6DC: 82ECB81C  lwz r23, -0x47e4(r12)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[12].u32.wrapping_add(-18404 as u32) ) } as u64;
            }
            0x82ECB6E0 => {
    //   block [0x82ECB6E0..0x82ECB710)
	// 82ECB6E0: 83E100D4  lwz r31, 0xd4(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(212 as u32) ) } as u64;
	// 82ECB6E4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82ECB6E8: 419A017C  beq cr6, 0x82ecb864
	if ctx.cr[6].eq {
	pc = 0x82ECB864; continue 'dispatch;
	}
	// 82ECB6EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82ECB6F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECB6F4: 4800EF15  bl 0x82eda608
	ctx.lr = 0x82ECB6F8;
	sub_82EDA608(ctx, base);
	// 82ECB6F8: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82ECB6FC: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 82ECB700: 392A3F34  addi r9, r10, 0x3f34
	ctx.r[9].s64 = ctx.r[10].s64 + 16180;
	// 82ECB704: 997F0008  stb r11, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82ECB708: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82ECB70C: 4800015C  b 0x82ecb868
	pc = 0x82ECB868; continue 'dispatch;
            }
            0x82ECB710 => {
    //   block [0x82ECB710..0x82ECB728)
	// 82ECB710: 806100D4  lwz r3, 0xd4(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(212 as u32) ) } as u64;
	// 82ECB714: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ECB718: 419A014C  beq cr6, 0x82ecb864
	if ctx.cr[6].eq {
	pc = 0x82ECB864; continue 'dispatch;
	}
	// 82ECB71C: 48012F7D  bl 0x82ede698
	ctx.lr = 0x82ECB720;
	sub_82EDE698(ctx, base);
	// 82ECB720: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ECB724: 48000144  b 0x82ecb868
	pc = 0x82ECB868; continue 'dispatch;
            }
            0x82ECB728 => {
    //   block [0x82ECB728..0x82ECB740)
	// 82ECB728: 806100D4  lwz r3, 0xd4(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(212 as u32) ) } as u64;
	// 82ECB72C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ECB730: 419A0134  beq cr6, 0x82ecb864
	if ctx.cr[6].eq {
	pc = 0x82ECB864; continue 'dispatch;
	}
	// 82ECB734: 48012EFD  bl 0x82ede630
	ctx.lr = 0x82ECB738;
	sub_82EDE630(ctx, base);
	// 82ECB738: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ECB73C: 4800012C  b 0x82ecb868
	pc = 0x82ECB868; continue 'dispatch;
            }
            0x82ECB740 => {
    //   block [0x82ECB740..0x82ECB770)
	// 82ECB740: 83E100D4  lwz r31, 0xd4(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(212 as u32) ) } as u64;
	// 82ECB744: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82ECB748: 419A011C  beq cr6, 0x82ecb864
	if ctx.cr[6].eq {
	pc = 0x82ECB864; continue 'dispatch;
	}
	// 82ECB74C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82ECB750: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECB754: 4800EEB5  bl 0x82eda608
	ctx.lr = 0x82ECB758;
	sub_82EDA608(ctx, base);
	// 82ECB758: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82ECB75C: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 82ECB760: 392A3F9C  addi r9, r10, 0x3f9c
	ctx.r[9].s64 = ctx.r[10].s64 + 16284;
	// 82ECB764: 997F0008  stb r11, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82ECB768: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82ECB76C: 480000FC  b 0x82ecb868
	pc = 0x82ECB868; continue 'dispatch;
            }
            0x82ECB770 => {
    //   block [0x82ECB770..0x82ECB788)
	// 82ECB770: 806100D4  lwz r3, 0xd4(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(212 as u32) ) } as u64;
	// 82ECB774: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ECB778: 419A00EC  beq cr6, 0x82ecb864
	if ctx.cr[6].eq {
	pc = 0x82ECB864; continue 'dispatch;
	}
	// 82ECB77C: 48012E6D  bl 0x82ede5e8
	ctx.lr = 0x82ECB780;
	sub_82EDE5E8(ctx, base);
	// 82ECB780: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ECB784: 480000E4  b 0x82ecb868
	pc = 0x82ECB868; continue 'dispatch;
            }
            0x82ECB788 => {
    //   block [0x82ECB788..0x82ECB7F4)
	// 82ECB788: C1BD0014  lfs f13, 0x14(r29)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82ECB78C: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82ECB790: C19D0028  lfs f12, 0x28(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(40 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82ECB794: ED6D6028  fsubs f11, f13, f12
	ctx.f[11].f64 = (((ctx.f[13].f64 - ctx.f[12].f64) as f32) as f64);
	// 82ECB798: C15D0000  lfs f10, 0(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82ECB79C: C00BACFC  lfs f0, -0x5304(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-21252 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82ECB7A0: FD2B636E  fsel f9, f11, f13, f12
	ctx.f[9].f64 = if ctx.f[11].f64 >= 0.0 { ctx.f[13].f64 } else { ctx.f[12].f64 };
	// 82ECB7A4: FD0B6B2E  fsel f8, f11, f12, f13
	ctx.f[8].f64 = if ctx.f[11].f64 >= 0.0 { ctx.f[12].f64 } else { ctx.f[13].f64 };
	// 82ECB7A8: ECEA4828  fsubs f7, f10, f9
	ctx.f[7].f64 = (((ctx.f[10].f64 - ctx.f[9].f64) as f32) as f64);
	// 82ECB7AC: ECCA4028  fsubs f6, f10, f8
	ctx.f[6].f64 = (((ctx.f[10].f64 - ctx.f[8].f64) as f32) as f64);
	// 82ECB7B0: FCA74AAE  fsel f5, f7, f10, f9
	ctx.f[5].f64 = if ctx.f[7].f64 >= 0.0 { ctx.f[10].f64 } else { ctx.f[9].f64 };
	// 82ECB7B4: FC86522E  fsel f4, f6, f8, f10
	ctx.f[4].f64 = if ctx.f[6].f64 >= 0.0 { ctx.f[8].f64 } else { ctx.f[10].f64 };
	// 82ECB7B8: EC650032  fmuls f3, f5, f0
	ctx.f[3].f64 = (((ctx.f[5].f64 * ctx.f[0].f64) as f32) as f64);
	// 82ECB7BC: FF041800  fcmpu cr6, f4, f3
	ctx.cr[6].compare_f64(ctx.f[4].f64, ctx.f[3].f64);
	// 82ECB7C0: 4099FF50  ble cr6, 0x82ecb710
	if !ctx.cr[6].gt {
	pc = 0x82ECB710; continue 'dispatch;
	}
	// 82ECB7C4: 83E100D4  lwz r31, 0xd4(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(212 as u32) ) } as u64;
	// 82ECB7C8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82ECB7CC: 419A0098  beq cr6, 0x82ecb864
	if ctx.cr[6].eq {
	pc = 0x82ECB864; continue 'dispatch;
	}
	// 82ECB7D0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82ECB7D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECB7D8: 4800EE31  bl 0x82eda608
	ctx.lr = 0x82ECB7DC;
	sub_82EDA608(ctx, base);
	// 82ECB7DC: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82ECB7E0: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 82ECB7E4: 392A3F34  addi r9, r10, 0x3f34
	ctx.r[9].s64 = ctx.r[10].s64 + 16180;
	// 82ECB7E8: 997F0008  stb r11, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82ECB7EC: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82ECB7F0: 48000078  b 0x82ecb868
	pc = 0x82ECB868; continue 'dispatch;
            }
            0x82ECB7F4 => {
    //   block [0x82ECB7F4..0x82ECB81C)
	// 82ECB7F4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82ECB7F8: 806100D4  lwz r3, 0xd4(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(212 as u32) ) } as u64;
	// 82ECB7FC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ECB800: C00B9F64  lfs f0, -0x609c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24732 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82ECB804: D00100C4  stfs f0, 0xc4(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(196 as u32), tmp.u32 ) };
	// 82ECB808: D00100CC  stfs f0, 0xcc(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(204 as u32), tmp.u32 ) };
	// 82ECB80C: 419A0058  beq cr6, 0x82ecb864
	if ctx.cr[6].eq {
	pc = 0x82ECB864; continue 'dispatch;
	}
	// 82ECB810: 48012B11  bl 0x82ede320
	ctx.lr = 0x82ECB814;
	sub_82EDE320(ctx, base);
	// 82ECB814: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ECB818: 48000050  b 0x82ecb868
	pc = 0x82ECB868; continue 'dispatch;
            }
            0x82ECB81C => {
    //   block [0x82ECB81C..0x82ECB84C)
	// 82ECB81C: 83E100D4  lwz r31, 0xd4(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(212 as u32) ) } as u64;
	// 82ECB820: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82ECB824: 419A0040  beq cr6, 0x82ecb864
	if ctx.cr[6].eq {
	pc = 0x82ECB864; continue 'dispatch;
	}
	// 82ECB828: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82ECB82C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECB830: 4800EDD9  bl 0x82eda608
	ctx.lr = 0x82ECB834;
	sub_82EDA608(ctx, base);
	// 82ECB834: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82ECB838: 39600009  li r11, 9
	ctx.r[11].s64 = 9;
	// 82ECB83C: 392A4004  addi r9, r10, 0x4004
	ctx.r[9].s64 = ctx.r[10].s64 + 16388;
	// 82ECB840: 997F0008  stb r11, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82ECB844: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82ECB848: 48000020  b 0x82ecb868
	pc = 0x82ECB868; continue 'dispatch;
            }
            0x82ECB84C => {
    //   block [0x82ECB84C..0x82ECB8DC)
	// 82ECB84C: 806100D4  lwz r3, 0xd4(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(212 as u32) ) } as u64;
	// 82ECB850: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ECB854: 419A0010  beq cr6, 0x82ecb864
	if ctx.cr[6].eq {
	pc = 0x82ECB864; continue 'dispatch;
	}
	// 82ECB858: 48012C71  bl 0x82ede4c8
	ctx.lr = 0x82ECB85C;
	sub_82EDE4C8(ctx, base);
	// 82ECB85C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ECB860: 48000008  b 0x82ecb868
	pc = 0x82ECB868; continue 'dispatch;
	// 82ECB864: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82ECB868: 2F1E0006  cmpwi cr6, r30, 6
	ctx.cr[6].compare_i32(ctx.r[30].s32, 6, &mut ctx.xer);
	// 82ECB86C: 419A004C  beq cr6, 0x82ecb8b8
	if ctx.cr[6].eq {
	pc = 0x82ECB8B8; continue 'dispatch;
	}
	// 82ECB870: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECB874: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82ECB878: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECB87C: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82ECB880: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ECB884: 4E800421  bctrl
	ctx.lr = 0x82ECB888;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECB888: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECB88C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82ECB890: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECB894: 8109002C  lwz r8, 0x2c(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(44 as u32) ) } as u64;
	// 82ECB898: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82ECB89C: 4E800421  bctrl
	ctx.lr = 0x82ECB8A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECB8A0: 80FF0000  lwz r7, 0(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECB8A4: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82ECB8A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECB8AC: 80C7000C  lwz r6, 0xc(r7)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ECB8B0: 7CC903A6  mtctr r6
	ctx.ctr.u64 = ctx.r[6].u64;
	// 82ECB8B4: 4E800421  bctrl
	ctx.lr = 0x82ECB8B8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECB8B8: 388100C4  addi r4, r1, 0xc4
	ctx.r[4].s64 = ctx.r[1].s64 + 196;
	// 82ECB8BC: 387F00BC  addi r3, r31, 0xbc
	ctx.r[3].s64 = ctx.r[31].s64 + 188;
	// 82ECB8C0: 4BFDC191  bl 0x82ea7a50
	ctx.lr = 0x82ECB8C4;
	sub_82EA7A50(ctx, base);
	// 82ECB8C4: 388100CC  addi r4, r1, 0xcc
	ctx.r[4].s64 = ctx.r[1].s64 + 204;
	// 82ECB8C8: 387F00BD  addi r3, r31, 0xbd
	ctx.r[3].s64 = ctx.r[31].s64 + 189;
	// 82ECB8CC: 4BFDC185  bl 0x82ea7a50
	ctx.lr = 0x82ECB8D0;
	sub_82EA7A50(ctx, base);
	// 82ECB8D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82ECB8D4: CBE1FFD0  lfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 82ECB8D8: 482DC8E0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECB8E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82ECB8E0 size=684
    let mut pc: u32 = 0x82ECB8E0;
    'dispatch: loop {
        match pc {
            0x82ECB8E0 => {
    //   block [0x82ECB8E0..0x82ECBB8C)
	// 82ECB8E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ECB8E4: 482DC885  bl 0x831a8168
	ctx.lr = 0x82ECB8E8;
	sub_831A8130(ctx, base);
	// 82ECB8E8: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 82ECB8EC: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ECB8F0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82ECB8F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ECB8F8: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECB8FC: 4800D8CD  bl 0x82ed91c8
	ctx.lr = 0x82ECB900;
	sub_82ED91C8(ctx, base);
	// 82ECB900: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82ECB904: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82ECB908: 392B3F10  addi r9, r11, 0x3f10
	ctx.r[9].s64 = ctx.r[11].s64 + 16144;
	// 82ECB90C: 3BBF00D0  addi r29, r31, 0xd0
	ctx.r[29].s64 = ctx.r[31].s64 + 208;
	// 82ECB910: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82ECB914: 891E0008  lbz r8, 8(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ECB918: 991F0080  stb r8, 0x80(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[8].u8 ) };
	// 82ECB91C: C3EA08A4  lfs f31, 0x8a4(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82ECB920: A0FE000A  lhz r7, 0xa(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(10 as u32) ) } as u64;
	// 82ECB924: B0FF0096  sth r7, 0x96(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(150 as u32), ctx.r[7].u16 ) };
	// 82ECB928: 80DE0000  lwz r6, 0(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECB92C: 90DF002C  stw r6, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[6].u32 ) };
	// 82ECB930: 88BE00B0  lbz r5, 0xb0(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(176 as u32) ) } as u64;
	// 82ECB934: 7CA30774  extsb r3, r5
	ctx.r[3].s64 = ctx.r[5].s8 as i64;
	// 82ECB938: 2F030007  cmpwi cr6, r3, 7
	ctx.cr[6].compare_i32(ctx.r[3].s32, 7, &mut ctx.xer);
	// 82ECB93C: 409A006C  bne cr6, 0x82ecb9a8
	if !ctx.cr[6].eq {
	pc = 0x82ECB9A8; continue 'dispatch;
	}
	// 82ECB940: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82ECB944: 419A001C  beq cr6, 0x82ecb960
	if ctx.cr[6].eq {
	pc = 0x82ECB960; continue 'dispatch;
	}
	// 82ECB948: 38BE0020  addi r5, r30, 0x20
	ctx.r[5].s64 = ctx.r[30].s64 + 32;
	// 82ECB94C: 389E0010  addi r4, r30, 0x10
	ctx.r[4].s64 = ctx.r[30].s64 + 16;
	// 82ECB950: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82ECB954: 48012B75  bl 0x82ede4c8
	ctx.lr = 0x82ECB958;
	sub_82EDE4C8(ctx, base);
	// 82ECB958: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82ECB95C: 48000008  b 0x82ecb964
	pc = 0x82ECB964; continue 'dispatch;
	// 82ECB960: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82ECB964: 389E00A4  addi r4, r30, 0xa4
	ctx.r[4].s64 = ctx.r[30].s64 + 164;
	// 82ECB968: 387C00BC  addi r3, r28, 0xbc
	ctx.r[3].s64 = ctx.r[28].s64 + 188;
	// 82ECB96C: 4BFDC0E5  bl 0x82ea7a50
	ctx.lr = 0x82ECB970;
	sub_82EA7A50(ctx, base);
	// 82ECB970: 389E00A8  addi r4, r30, 0xa8
	ctx.r[4].s64 = ctx.r[30].s64 + 168;
	// 82ECB974: 387C00BD  addi r3, r28, 0xbd
	ctx.r[3].s64 = ctx.r[28].s64 + 189;
	// 82ECB978: 4BFDC0D9  bl 0x82ea7a50
	ctx.lr = 0x82ECB97C;
	sub_82EA7A50(ctx, base);
	// 82ECB97C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82ECB980: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82ECB984: 4800EBED  bl 0x82eda570
	ctx.lr = 0x82ECB988;
	sub_82EDA570(ctx, base);
	// 82ECB988: 397F00E0  addi r11, r31, 0xe0
	ctx.r[11].s64 = ctx.r[31].s64 + 224;
	// 82ECB98C: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82ECB990: C01E00AC  lfs f0, 0xac(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(172 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82ECB994: FF00F800  fcmpu cr6, f0, f31
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[31].f64);
	// 82ECB998: 41990090  bgt cr6, 0x82ecba28
	if ctx.cr[6].gt {
	pc = 0x82ECBA28; continue 'dispatch;
	}
	// 82ECB99C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82ECB9A0: C00BBA78  lfs f0, -0x4588(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17800 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82ECB9A4: 48000084  b 0x82ecba28
	pc = 0x82ECBA28; continue 'dispatch;
	// 82ECB9A8: 391E0080  addi r8, r30, 0x80
	ctx.r[8].s64 = ctx.r[30].s64 + 128;
	// 82ECB9AC: C07E00A8  lfs f3, 0xa8(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(168 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 82ECB9B0: 38FE0050  addi r7, r30, 0x50
	ctx.r[7].s64 = ctx.r[30].s64 + 80;
	// 82ECB9B4: C05E00A4  lfs f2, 0xa4(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(164 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82ECB9B8: 38BE0020  addi r5, r30, 0x20
	ctx.r[5].s64 = ctx.r[30].s64 + 32;
	// 82ECB9BC: C03E0090  lfs f1, 0x90(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(144 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82ECB9C0: 389E0010  addi r4, r30, 0x10
	ctx.r[4].s64 = ctx.r[30].s64 + 16;
	// 82ECB9C4: 93A10054  stw r29, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 82ECB9C8: 4BFFFCA9  bl 0x82ecb670
	ctx.lr = 0x82ECB9CC;
	sub_82ECB670(ctx, base);
	// 82ECB9CC: 897E00B2  lbz r11, 0xb2(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(178 as u32) ) } as u64;
	// 82ECB9D0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82ECB9D4: 7D640774  extsb r4, r11
	ctx.r[4].s64 = ctx.r[11].s8 as i64;
	// 82ECB9D8: 4800EB99  bl 0x82eda570
	ctx.lr = 0x82ECB9DC;
	sub_82EDA570(ctx, base);
	// 82ECB9DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECB9E0: 4800D289  bl 0x82ed8c68
	ctx.lr = 0x82ECB9E4;
	sub_82ED8C68(ctx, base);
	// 82ECB9E4: 815F00D0  lwz r10, 0xd0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(208 as u32) ) } as u64;
	// 82ECB9E8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82ECB9EC: 389E0030  addi r4, r30, 0x30
	ctx.r[4].s64 = ctx.r[30].s64 + 48;
	// 82ECB9F0: 812A0040  lwz r9, 0x40(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(64 as u32) ) } as u64;
	// 82ECB9F4: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82ECB9F8: 4E800421  bctrl
	ctx.lr = 0x82ECB9FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECB9FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECBA00: 4800D269  bl 0x82ed8c68
	ctx.lr = 0x82ECBA04;
	sub_82ED8C68(ctx, base);
	// 82ECBA04: 811F00D0  lwz r8, 0xd0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(208 as u32) ) } as u64;
	// 82ECBA08: 389E0040  addi r4, r30, 0x40
	ctx.r[4].s64 = ctx.r[30].s64 + 64;
	// 82ECBA0C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82ECBA10: 80E80044  lwz r7, 0x44(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(68 as u32) ) } as u64;
	// 82ECBA14: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 82ECBA18: 4E800421  bctrl
	ctx.lr = 0x82ECBA1C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECBA1C: 38DF00E0  addi r6, r31, 0xe0
	ctx.r[6].s64 = ctx.r[31].s64 + 224;
	// 82ECBA20: 90DF0018  stw r6, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[6].u32 ) };
	// 82ECBA24: C01E00AC  lfs f0, 0xac(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(172 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82ECBA28: D01F0058  stfs f0, 0x58(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82ECBA2C: 897E00B1  lbz r11, 0xb1(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(177 as u32) ) } as u64;
	// 82ECBA30: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82ECBA34: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82ECBA38: 419A0034  beq cr6, 0x82ecba6c
	if ctx.cr[6].eq {
	pc = 0x82ECBA6C; continue 'dispatch;
	}
	// 82ECBA3C: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82ECBA40: 409A0048  bne cr6, 0x82ecba88
	if !ctx.cr[6].eq {
	pc = 0x82ECBA88; continue 'dispatch;
	}
	// 82ECBA44: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ECBA48: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82ECBA4C: 80BF00C0  lwz r5, 0xc0(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(192 as u32) ) } as u64;
	// 82ECBA50: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82ECBA54: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ECBA58: 419A0020  beq cr6, 0x82ecba78
	if ctx.cr[6].eq {
	pc = 0x82ECBA78; continue 'dispatch;
	}
	// 82ECBA5C: 890B02C7  lbz r8, 0x2c7(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(711 as u32) ) } as u64;
	// 82ECBA60: 88EB02C6  lbz r7, 0x2c6(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(710 as u32) ) } as u64;
	// 82ECBA64: 88CB02C5  lbz r6, 0x2c5(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(709 as u32) ) } as u64;
	// 82ECBA68: 4800001C  b 0x82ecba84
	pc = 0x82ECBA84; continue 'dispatch;
	// 82ECBA6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82ECBA70: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82ECBA74: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82ECBA78: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82ECBA7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82ECBA80: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82ECBA84: 4BFFF8ED  bl 0x82ecb370
	ctx.lr = 0x82ECBA88;
	sub_82ECB370(ctx, base);
	// 82ECBA88: C01E0094  lfs f0, 0x94(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(148 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82ECBA8C: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82ECBA90: D01F0184  stfs f0, 0x184(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(388 as u32), tmp.u32 ) };
	// 82ECBA94: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82ECBA98: C1BE0098  lfs f13, 0x98(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(152 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82ECBA9C: D1BF0188  stfs f13, 0x188(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(392 as u32), tmp.u32 ) };
	// 82ECBAA0: 419A0064  beq cr6, 0x82ecbb04
	if ctx.cr[6].eq {
	pc = 0x82ECBB04; continue 'dispatch;
	}
	// 82ECBAA4: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82ECBAA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECBAAC: 4BFFFAB5  bl 0x82ecb560
	ctx.lr = 0x82ECBAB0;
	sub_82ECB560(ctx, base);
	// 82ECBAB0: C01F0058  lfs f0, 0x58(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82ECBAB4: FF00F800  fcmpu cr6, f0, f31
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[31].f64);
	// 82ECBAB8: 4199004C  bgt cr6, 0x82ecbb04
	if ctx.cr[6].gt {
	pc = 0x82ECBB04; continue 'dispatch;
	}
	// 82ECBABC: C0010060  lfs f0, 0x60(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82ECBAC0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82ECBAC4: C1A10064  lfs f13, 0x64(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82ECBAC8: ED406828  fsubs f10, f0, f13
	ctx.f[10].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 82ECBACC: C1810068  lfs f12, 0x68(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82ECBAD0: C16B9450  lfs f11, -0x6bb0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27568 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82ECBAD4: FD2A036E  fsel f9, f10, f13, f0
	ctx.f[9].f64 = if ctx.f[10].f64 >= 0.0 { ctx.f[13].f64 } else { ctx.f[0].f64 };
	// 82ECBAD8: ED096028  fsubs f8, f9, f12
	ctx.f[8].f64 = (((ctx.f[9].f64 - ctx.f[12].f64) as f32) as f64);
	// 82ECBADC: FC084B2E  fsel f0, f8, f12, f9
	ctx.f[0].f64 = if ctx.f[8].f64 >= 0.0 { ctx.f[12].f64 } else { ctx.f[9].f64 };
	// 82ECBAE0: FF005800  fcmpu cr6, f0, f11
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[11].f64);
	// 82ECBAE4: 40980014  bge cr6, 0x82ecbaf8
	if !ctx.cr[6].lt {
	pc = 0x82ECBAF8; continue 'dispatch;
	}
	// 82ECBAE8: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82ECBAEC: C1AB6150  lfs f13, 0x6150(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24912 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82ECBAF0: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 82ECBAF4: 4800000C  b 0x82ecbb00
	pc = 0x82ECBB00; continue 'dispatch;
	// 82ECBAF8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82ECBAFC: C00B964C  lfs f0, -0x69b4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27060 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82ECBB00: D01F0058  stfs f0, 0x58(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82ECBB04: 897E00B3  lbz r11, 0xb3(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(179 as u32) ) } as u64;
	// 82ECBB08: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ECBB0C: 419A000C  beq cr6, 0x82ecbb18
	if ctx.cr[6].eq {
	pc = 0x82ECBB18; continue 'dispatch;
	}
	// 82ECBB10: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82ECBB14: 4800002C  b 0x82ecbb40
	pc = 0x82ECBB40; continue 'dispatch;
	// 82ECBB18: 897F00D8  lbz r11, 0xd8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(216 as u32) ) } as u64;
	// 82ECBB1C: 2B0B0007  cmplwi cr6, r11, 7
	ctx.cr[6].compare_u32(ctx.r[11].u32, 7 as u32, &mut ctx.xer);
	// 82ECBB20: 409A000C  bne cr6, 0x82ecbb2c
	if !ctx.cr[6].eq {
	pc = 0x82ECBB2C; continue 'dispatch;
	}
	// 82ECBB24: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82ECBB28: 48000018  b 0x82ecbb40
	pc = 0x82ECBB40; continue 'dispatch;
	// 82ECBB2C: 897E00B0  lbz r11, 0xb0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(176 as u32) ) } as u64;
	// 82ECBB30: 2B0B0006  cmplwi cr6, r11, 6
	ctx.cr[6].compare_u32(ctx.r[11].u32, 6 as u32, &mut ctx.xer);
	// 82ECBB34: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 82ECBB38: 419A0008  beq cr6, 0x82ecbb40
	if ctx.cr[6].eq {
	pc = 0x82ECBB40; continue 'dispatch;
	}
	// 82ECBB3C: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 82ECBB40: B17F002A  sth r11, 0x2a(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(42 as u32), ctx.r[11].u16 ) };
	// 82ECBB44: 897E00B5  lbz r11, 0xb5(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(181 as u32) ) } as u64;
	// 82ECBB48: 997F00BD  stb r11, 0xbd(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(189 as u32), ctx.r[11].u8 ) };
	// 82ECBB4C: 895E00B4  lbz r10, 0xb4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(180 as u32) ) } as u64;
	// 82ECBB50: 995F00BC  stb r10, 0xbc(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(188 as u32), ctx.r[10].u8 ) };
	// 82ECBB54: 893E00B6  lbz r9, 0xb6(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(182 as u32) ) } as u64;
	// 82ECBB58: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82ECBB5C: 419A0010  beq cr6, 0x82ecbb6c
	if ctx.cr[6].eq {
	pc = 0x82ECBB6C; continue 'dispatch;
	}
	// 82ECBB60: 897F0021  lbz r11, 0x21(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(33 as u32) ) } as u64;
	// 82ECBB64: 616A0001  ori r10, r11, 1
	ctx.r[10].u64 = ctx.r[11].u64 | 1;
	// 82ECBB68: 995F0021  stb r10, 0x21(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(33 as u32), ctx.r[10].u8 ) };
	// 82ECBB6C: C01E009C  lfs f0, 0x9c(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(156 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82ECBB70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECBB74: D01F0084  stfs f0, 0x84(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), tmp.u32 ) };
	// 82ECBB78: C1BE00A0  lfs f13, 0xa0(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(160 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82ECBB7C: D1BF0088  stfs f13, 0x88(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(136 as u32), tmp.u32 ) };
	// 82ECBB80: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82ECBB84: CBE1FFD0  lfd f31, -0x30(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 82ECBB88: 482DC630  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECBB90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ECBB90 size=108
    let mut pc: u32 = 0x82ECBB90;
    'dispatch: loop {
        match pc {
            0x82ECBB90 => {
    //   block [0x82ECBB90..0x82ECBBFC)
	// 82ECBB90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ECBB94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ECBB98: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ECBB9C: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ECBBA0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ECBBA4: 419A0044  beq cr6, 0x82ecbbe8
	if ctx.cr[6].eq {
	pc = 0x82ECBBE8; continue 'dispatch;
	}
	// 82ECBBA8: 814B0084  lwz r10, 0x84(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(132 as u32) ) } as u64;
	// 82ECBBAC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ECBBB0: 419A0038  beq cr6, 0x82ecbbe8
	if ctx.cr[6].eq {
	pc = 0x82ECBBE8; continue 'dispatch;
	}
	// 82ECBBB4: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 82ECBBB8: 90610054  stw r3, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 82ECBBBC: 98810058  stb r4, 0x58(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[4].u8 ) };
	// 82ECBBC0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82ECBBC4: 98A10059  stb r5, 0x59(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(89 as u32), ctx.r[5].u8 ) };
	// 82ECBBC8: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82ECBBCC: 98C1005A  stb r6, 0x5a(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(90 as u32), ctx.r[6].u8 ) };
	// 82ECBBD0: 98E10050  stb r7, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[7].u8 ) };
	// 82ECBBD4: 4800142D  bl 0x82ecd000
	ctx.lr = 0x82ECBBD8;
	sub_82ECD000(ctx, base);
	// 82ECBBD8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82ECBBDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ECBBE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ECBBE4: 4E800020  blr
	return;
	// 82ECBBE8: 48016D71  bl 0x82ee2958
	ctx.lr = 0x82ECBBEC;
	sub_82EE2958(ctx, base);
	// 82ECBBEC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82ECBBF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ECBBF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ECBBF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECBC00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82ECBC00 size=496
    let mut pc: u32 = 0x82ECBC00;
    'dispatch: loop {
        match pc {
            0x82ECBC00 => {
    //   block [0x82ECBC00..0x82ECBDF0)
	// 82ECBC00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ECBC04: 482DC569  bl 0x831a816c
	ctx.lr = 0x82ECBC08;
	sub_831A8130(ctx, base);
	// 82ECBC08: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ECBC0C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ECBC10: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82ECBC14: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ECBC18: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ECBC1C: 419A004C  beq cr6, 0x82ecbc68
	if ctx.cr[6].eq {
	pc = 0x82ECBC68; continue 'dispatch;
	}
	// 82ECBC20: 81630084  lwz r11, 0x84(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(132 as u32) ) } as u64;
	// 82ECBC24: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ECBC28: 419A0028  beq cr6, 0x82ecbc50
	if ctx.cr[6].eq {
	pc = 0x82ECBC50; continue 'dispatch;
	}
	// 82ECBC2C: 39600005  li r11, 5
	ctx.r[11].s64 = 5;
	// 82ECBC30: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 82ECBC34: 93A10058  stw r29, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[29].u32 ) };
	// 82ECBC38: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82ECBC3C: 99610050  stb r11, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 82ECBC40: 480013C1  bl 0x82ecd000
	ctx.lr = 0x82ECBC44;
	sub_82ECD000(ctx, base);
	// 82ECBC44: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82ECBC48: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82ECBC4C: 482DC570  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 82ECBC50: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ECBC54: 419A0014  beq cr6, 0x82ecbc68
	if ctx.cr[6].eq {
	pc = 0x82ECBC68; continue 'dispatch;
	}
	// 82ECBC58: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82ECBC5C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82ECBC60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECBC64: 4800CDE5  bl 0x82ed8a48
	ctx.lr = 0x82ECBC68;
	sub_82ED8A48(ctx, base);
	// 82ECBC68: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ECBC6C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ECBC70: 419A001C  beq cr6, 0x82ecbc8c
	if ctx.cr[6].eq {
	pc = 0x82ECBC8C; continue 'dispatch;
	}
	// 82ECBC74: 814B0084  lwz r10, 0x84(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(132 as u32) ) } as u64;
	// 82ECBC78: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82ECBC7C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82ECBC80: 914B0084  stw r10, 0x84(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(132 as u32), ctx.r[10].u32 ) };
	// 82ECBC84: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ECBC88: 48015879  bl 0x82ee1500
	ctx.lr = 0x82ECBC8C;
	sub_82EE1500(ctx, base);
	// 82ECBC8C: 83DF0010  lwz r30, 0x10(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82ECBC90: 93BF0010  stw r29, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[29].u32 ) };
	// 82ECBC94: A17D0004  lhz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECBC98: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ECBC9C: 419A0010  beq cr6, 0x82ecbcac
	if ctx.cr[6].eq {
	pc = 0x82ECBCAC; continue 'dispatch;
	}
	// 82ECBCA0: A17D0006  lhz r11, 6(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(6 as u32) ) } as u64;
	// 82ECBCA4: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 82ECBCA8: B15D0006  sth r10, 6(r29)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[29].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82ECBCAC: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82ECBCB0: 419A0040  beq cr6, 0x82ecbcf0
	if ctx.cr[6].eq {
	pc = 0x82ECBCF0; continue 'dispatch;
	}
	// 82ECBCB4: A17E0004  lhz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECBCB8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ECBCBC: 419A0034  beq cr6, 0x82ecbcf0
	if ctx.cr[6].eq {
	pc = 0x82ECBCF0; continue 'dispatch;
	}
	// 82ECBCC0: A17E0006  lhz r11, 6(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(6 as u32) ) } as u64;
	// 82ECBCC4: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 82ECBCC8: 7D490734  extsh r9, r10
	ctx.r[9].s64 = ctx.r[10].s16 as i64;
	// 82ECBCCC: B13E0006  sth r9, 6(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82ECBCD0: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82ECBCD4: 409A001C  bne cr6, 0x82ecbcf0
	if !ctx.cr[6].eq {
	pc = 0x82ECBCF0; continue 'dispatch;
	}
	// 82ECBCD8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECBCDC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82ECBCE0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82ECBCE4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECBCE8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ECBCEC: 4E800421  bctrl
	ctx.lr = 0x82ECBCF0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECBCF0: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82ECBCF4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82ECBCF8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECBCFC: 4BFFF865  bl 0x82ecb560
	ctx.lr = 0x82ECBD00;
	sub_82ECB560(ctx, base);
	// 82ECBD00: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82ECBD04: 419A0024  beq cr6, 0x82ecbd28
	if ctx.cr[6].eq {
	pc = 0x82ECBD28; continue 'dispatch;
	}
	// 82ECBD08: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82ECBD0C: C1BF0058  lfs f13, 0x58(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82ECBD10: C00BBA78  lfs f0, -0x4588(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17800 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82ECBD14: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82ECBD18: 419A0010  beq cr6, 0x82ecbd28
	if ctx.cr[6].eq {
	pc = 0x82ECBD28; continue 'dispatch;
	}
	// 82ECBD1C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82ECBD20: C00B9534  lfs f0, -0x6acc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27340 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82ECBD24: D01F0058  stfs f0, 0x58(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82ECBD28: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82ECBD2C: C1BF0058  lfs f13, 0x58(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82ECBD30: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82ECBD34: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82ECBD38: 4199004C  bgt cr6, 0x82ecbd84
	if ctx.cr[6].gt {
	pc = 0x82ECBD84; continue 'dispatch;
	}
	// 82ECBD3C: C0010060  lfs f0, 0x60(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82ECBD40: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82ECBD44: C1A10064  lfs f13, 0x64(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82ECBD48: ED406828  fsubs f10, f0, f13
	ctx.f[10].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 82ECBD4C: C1810068  lfs f12, 0x68(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82ECBD50: C16B9450  lfs f11, -0x6bb0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27568 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82ECBD54: FD2A036E  fsel f9, f10, f13, f0
	ctx.f[9].f64 = if ctx.f[10].f64 >= 0.0 { ctx.f[13].f64 } else { ctx.f[0].f64 };
	// 82ECBD58: ED096028  fsubs f8, f9, f12
	ctx.f[8].f64 = (((ctx.f[9].f64 - ctx.f[12].f64) as f32) as f64);
	// 82ECBD5C: FC084B2E  fsel f0, f8, f12, f9
	ctx.f[0].f64 = if ctx.f[8].f64 >= 0.0 { ctx.f[12].f64 } else { ctx.f[9].f64 };
	// 82ECBD60: FF005800  fcmpu cr6, f0, f11
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[11].f64);
	// 82ECBD64: 40980014  bge cr6, 0x82ecbd78
	if !ctx.cr[6].lt {
	pc = 0x82ECBD78; continue 'dispatch;
	}
	// 82ECBD68: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82ECBD6C: C1AB6150  lfs f13, 0x6150(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24912 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82ECBD70: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 82ECBD74: 4800000C  b 0x82ecbd80
	pc = 0x82ECBD80; continue 'dispatch;
	// 82ECBD78: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82ECBD7C: C00B964C  lfs f0, -0x69b4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27060 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82ECBD80: D01F0058  stfs f0, 0x58(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82ECBD84: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ECBD88: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ECBD8C: 419A000C  beq cr6, 0x82ecbd98
	if ctx.cr[6].eq {
	pc = 0x82ECBD98; continue 'dispatch;
	}
	// 82ECBD90: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82ECBD94: 48017E9D  bl 0x82ee3c30
	ctx.lr = 0x82ECBD98;
	sub_82EE3C30(ctx, base);
	// 82ECBD98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECBD9C: 48017535  bl 0x82ee32d0
	ctx.lr = 0x82ECBDA0;
	sub_82EE32D0(ctx, base);
	// 82ECBDA0: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ECBDA4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ECBDA8: 419A003C  beq cr6, 0x82ecbde4
	if ctx.cr[6].eq {
	pc = 0x82ECBDE4; continue 'dispatch;
	}
	// 82ECBDAC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82ECBDB0: 480154A9  bl 0x82ee1258
	ctx.lr = 0x82ECBDB4;
	sub_82EE1258(ctx, base);
	// 82ECBDB4: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ECBDB8: 81630084  lwz r11, 0x84(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(132 as u32) ) } as u64;
	// 82ECBDBC: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ECBDC0: 91630084  stw r11, 0x84(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 82ECBDC4: 40820020  bne 0x82ecbde4
	if !ctx.cr[0].eq {
	pc = 0x82ECBDE4; continue 'dispatch;
	}
	// 82ECBDC8: 8963008C  lbz r11, 0x8c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(140 as u32) ) } as u64;
	// 82ECBDCC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ECBDD0: 409A0014  bne cr6, 0x82ecbde4
	if !ctx.cr[6].eq {
	pc = 0x82ECBDE4; continue 'dispatch;
	}
	// 82ECBDD4: 81630080  lwz r11, 0x80(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(128 as u32) ) } as u64;
	// 82ECBDD8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ECBDDC: 419A0008  beq cr6, 0x82ecbde4
	if ctx.cr[6].eq {
	pc = 0x82ECBDE4; continue 'dispatch;
	}
	// 82ECBDE0: 48001209  bl 0x82eccfe8
	ctx.lr = 0x82ECBDE4;
	sub_82ECCFE8(ctx, base);
	// 82ECBDE4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82ECBDE8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82ECBDEC: 482DC3D0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECBDF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ECBDF0 size=76
    let mut pc: u32 = 0x82ECBDF0;
    'dispatch: loop {
        match pc {
            0x82ECBDF0 => {
    //   block [0x82ECBDF0..0x82ECBE3C)
	// 82ECBDF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ECBDF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ECBDF8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ECBDFC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ECBE00: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ECBE04: 387F00D0  addi r3, r31, 0xd0
	ctx.r[3].s64 = ctx.r[31].s64 + 208;
	// 82ECBE08: 817F00D0  lwz r11, 0xd0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(208 as u32) ) } as u64;
	// 82ECBE0C: 814B002C  lwz r10, 0x2c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82ECBE10: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ECBE14: 4E800421  bctrl
	ctx.lr = 0x82ECBE18;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECBE18: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82ECBE1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECBE20: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82ECBE24: 4BFFF73D  bl 0x82ecb560
	ctx.lr = 0x82ECBE28;
	sub_82ECB560(ctx, base);
	// 82ECBE28: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82ECBE2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ECBE30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ECBE34: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ECBE38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECBE40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ECBE40 size=28
    let mut pc: u32 = 0x82ECBE40;
    'dispatch: loop {
        match pc {
            0x82ECBE40 => {
    //   block [0x82ECBE40..0x82ECBE5C)
	// 82ECBE40: 896300D8  lbz r11, 0xd8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(216 as u32) ) } as u64;
	// 82ECBE44: 2B0B0006  cmplwi cr6, r11, 6
	ctx.cr[6].compare_u32(ctx.r[11].u32, 6 as u32, &mut ctx.xer);
	// 82ECBE48: 419A0014  beq cr6, 0x82ecbe5c
	if ctx.cr[6].eq {
		sub_82ECBE5C(ctx, base);
		return;
	}
	// 82ECBE4C: 2B0B0007  cmplwi cr6, r11, 7
	ctx.cr[6].compare_u32(ctx.r[11].u32, 7 as u32, &mut ctx.xer);
	// 82ECBE50: 419A000C  beq cr6, 0x82ecbe5c
	if ctx.cr[6].eq {
		sub_82ECBE5C(ctx, base);
		return;
	}
	// 82ECBE54: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82ECBE58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECBE5C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ECBE5C size=8
    let mut pc: u32 = 0x82ECBE5C;
    'dispatch: loop {
        match pc {
            0x82ECBE5C => {
    //   block [0x82ECBE5C..0x82ECBE64)
	// 82ECBE5C: 806301E8  lwz r3, 0x1e8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(488 as u32) ) } as u64;
	// 82ECBE60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECBE68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ECBE68 size=28
    let mut pc: u32 = 0x82ECBE68;
    'dispatch: loop {
        match pc {
            0x82ECBE68 => {
    //   block [0x82ECBE68..0x82ECBE84)
	// 82ECBE68: 896300D8  lbz r11, 0xd8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(216 as u32) ) } as u64;
	// 82ECBE6C: 2B0B0006  cmplwi cr6, r11, 6
	ctx.cr[6].compare_u32(ctx.r[11].u32, 6 as u32, &mut ctx.xer);
	// 82ECBE70: 419A0014  beq cr6, 0x82ecbe84
	if ctx.cr[6].eq {
		sub_82ECBE84(ctx, base);
		return;
	}
	// 82ECBE74: 2B0B0007  cmplwi cr6, r11, 7
	ctx.cr[6].compare_u32(ctx.r[11].u32, 7 as u32, &mut ctx.xer);
	// 82ECBE78: 419A000C  beq cr6, 0x82ecbe84
	if ctx.cr[6].eq {
		sub_82ECBE84(ctx, base);
		return;
	}
	// 82ECBE7C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82ECBE80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECBE84(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ECBE84 size=8
    let mut pc: u32 = 0x82ECBE84;
    'dispatch: loop {
        match pc {
            0x82ECBE84 => {
    //   block [0x82ECBE84..0x82ECBE8C)
	// 82ECBE84: 806301E8  lwz r3, 0x1e8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(488 as u32) ) } as u64;
	// 82ECBE88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECBE90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ECBE90 size=304
    let mut pc: u32 = 0x82ECBE90;
    'dispatch: loop {
        match pc {
            0x82ECBE90 => {
    //   block [0x82ECBE90..0x82ECBFC0)
	// 82ECBE90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ECBE94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ECBE98: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82ECBE9C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ECBEA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ECBEA4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82ECBEA8: 83FE0008  lwz r31, 8(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ECBEAC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82ECBEB0: 419A00F8  beq cr6, 0x82ecbfa8
	if ctx.cr[6].eq {
	pc = 0x82ECBFA8; continue 'dispatch;
	}
	// 82ECBEB4: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82ECBEB8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ECBEBC: 419A0020  beq cr6, 0x82ecbedc
	if ctx.cr[6].eq {
	pc = 0x82ECBEDC; continue 'dispatch;
	}
	// 82ECBEC0: 39600015  li r11, 0x15
	ctx.r[11].s64 = 21;
	// 82ECBEC4: 93C1005C  stw r30, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 82ECBEC8: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82ECBECC: 99610058  stb r11, 0x58(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u8 ) };
	// 82ECBED0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECBED4: 4800112D  bl 0x82ecd000
	ctx.lr = 0x82ECBED8;
	sub_82ECD000(ctx, base);
	// 82ECBED8: 480000D0  b 0x82ecbfa8
	pc = 0x82ECBFA8; continue 'dispatch;
	// 82ECBEDC: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ECBEE0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82ECBEE4: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 82ECBEE8: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82ECBEEC: 913F0084  stw r9, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[9].u32 ) };
	// 82ECBEF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82ECBEF4: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82ECBEF8: 99610050  stb r11, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 82ECBEFC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82ECBF00: 81030000  lwz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECBF04: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82ECBF08: 88EA0000  lbz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECBF0C: 81680020  lwz r11, 0x20(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(32 as u32) ) } as u64;
	// 82ECBF10: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82ECBF14: 4E800421  bctrl
	ctx.lr = 0x82ECBF18;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECBF18: 815E0010  lwz r10, 0x10(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82ECBF1C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ECBF20: 419A0014  beq cr6, 0x82ecbf34
	if ctx.cr[6].eq {
	pc = 0x82ECBF34; continue 'dispatch;
	}
	// 82ECBF24: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82ECBF28: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82ECBF2C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82ECBF30: 4801A831  bl 0x82ee6760
	ctx.lr = 0x82ECBF34;
	sub_82EE6760(ctx, base);
	// 82ECBF34: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82ECBF38: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82ECBF3C: 4800CACD  bl 0x82ed8a08
	ctx.lr = 0x82ECBF40;
	sub_82ED8A08(ctx, base);
	// 82ECBF40: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECBF44: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ECBF48: 409A0030  bne cr6, 0x82ecbf78
	if !ctx.cr[6].eq {
	pc = 0x82ECBF78; continue 'dispatch;
	}
	// 82ECBF4C: 897F00C6  lbz r11, 0xc6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(198 as u32) ) } as u64;
	// 82ECBF50: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ECBF54: 419A0018  beq cr6, 0x82ecbf6c
	if ctx.cr[6].eq {
	pc = 0x82ECBF6C; continue 'dispatch;
	}
	// 82ECBF58: 897E00D8  lbz r11, 0xd8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(216 as u32) ) } as u64;
	// 82ECBF5C: 2B0B0007  cmplwi cr6, r11, 7
	ctx.cr[6].compare_u32(ctx.r[11].u32, 7 as u32, &mut ctx.xer);
	// 82ECBF60: 419A000C  beq cr6, 0x82ecbf6c
	if ctx.cr[6].eq {
	pc = 0x82ECBF6C; continue 'dispatch;
	}
	// 82ECBF64: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82ECBF68: 4800CD01  bl 0x82ed8c68
	ctx.lr = 0x82ECBF6C;
	sub_82ED8C68(ctx, base);
	// 82ECBF6C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82ECBF70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECBF74: 48018F85  bl 0x82ee4ef8
	ctx.lr = 0x82ECBF78;
	sub_82EE4EF8(ctx, base);
	// 82ECBF78: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82ECBF7C: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ECBF80: 917F0084  stw r11, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 82ECBF84: 40820024  bne 0x82ecbfa8
	if !ctx.cr[0].eq {
	pc = 0x82ECBFA8; continue 'dispatch;
	}
	// 82ECBF88: 897F008C  lbz r11, 0x8c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) } as u64;
	// 82ECBF8C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ECBF90: 409A0018  bne cr6, 0x82ecbfa8
	if !ctx.cr[6].eq {
	pc = 0x82ECBFA8; continue 'dispatch;
	}
	// 82ECBF94: 817F0080  lwz r11, 0x80(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 82ECBF98: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ECBF9C: 419A000C  beq cr6, 0x82ecbfa8
	if ctx.cr[6].eq {
	pc = 0x82ECBFA8; continue 'dispatch;
	}
	// 82ECBFA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECBFA4: 48001045  bl 0x82eccfe8
	ctx.lr = 0x82ECBFA8;
	sub_82ECCFE8(ctx, base);
	// 82ECBFA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82ECBFAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ECBFB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ECBFB4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82ECBFB8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ECBFBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECBFC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ECBFC0 size=68
    let mut pc: u32 = 0x82ECBFC0;
    'dispatch: loop {
        match pc {
            0x82ECBFC0 => {
    //   block [0x82ECBFC0..0x82ECC004)
	// 82ECBFC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ECBFC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ECBFC8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ECBFCC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ECBFD0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ECBFD4: 387F00D0  addi r3, r31, 0xd0
	ctx.r[3].s64 = ctx.r[31].s64 + 208;
	// 82ECBFD8: 817F00D0  lwz r11, 0xd0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(208 as u32) ) } as u64;
	// 82ECBFDC: 814B0030  lwz r10, 0x30(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82ECBFE0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ECBFE4: 4E800421  bctrl
	ctx.lr = 0x82ECBFE8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECBFE8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECBFEC: 4BFFFEA5  bl 0x82ecbe90
	ctx.lr = 0x82ECBFF0;
	sub_82ECBE90(ctx, base);
	// 82ECBFF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82ECBFF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ECBFF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ECBFFC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ECC000: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECC008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ECC008 size=68
    let mut pc: u32 = 0x82ECC008;
    'dispatch: loop {
        match pc {
            0x82ECC008 => {
    //   block [0x82ECC008..0x82ECC04C)
	// 82ECC008: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ECC00C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ECC010: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ECC014: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ECC018: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ECC01C: 387F00D0  addi r3, r31, 0xd0
	ctx.r[3].s64 = ctx.r[31].s64 + 208;
	// 82ECC020: 817F00D0  lwz r11, 0xd0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(208 as u32) ) } as u64;
	// 82ECC024: 814B0034  lwz r10, 0x34(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82ECC028: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ECC02C: 4E800421  bctrl
	ctx.lr = 0x82ECC030;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECC030: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECC034: 4BFFFE5D  bl 0x82ecbe90
	ctx.lr = 0x82ECC038;
	sub_82ECBE90(ctx, base);
	// 82ECC038: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82ECC03C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ECC040: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ECC044: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ECC048: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECC050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ECC050 size=68
    let mut pc: u32 = 0x82ECC050;
    'dispatch: loop {
        match pc {
            0x82ECC050 => {
    //   block [0x82ECC050..0x82ECC094)
	// 82ECC050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ECC054: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ECC058: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ECC05C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ECC060: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ECC064: 387F00D0  addi r3, r31, 0xd0
	ctx.r[3].s64 = ctx.r[31].s64 + 208;
	// 82ECC068: 817F00D0  lwz r11, 0xd0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(208 as u32) ) } as u64;
	// 82ECC06C: 814B0038  lwz r10, 0x38(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 82ECC070: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ECC074: 4E800421  bctrl
	ctx.lr = 0x82ECC078;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECC078: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECC07C: 4BFFFE15  bl 0x82ecbe90
	ctx.lr = 0x82ECC080;
	sub_82ECBE90(ctx, base);
	// 82ECC080: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82ECC084: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ECC088: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ECC08C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ECC090: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECC098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ECC098 size=68
    let mut pc: u32 = 0x82ECC098;
    'dispatch: loop {
        match pc {
            0x82ECC098 => {
    //   block [0x82ECC098..0x82ECC0DC)
	// 82ECC098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ECC09C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ECC0A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ECC0A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ECC0A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ECC0AC: 387F00D0  addi r3, r31, 0xd0
	ctx.r[3].s64 = ctx.r[31].s64 + 208;
	// 82ECC0B0: 817F00D0  lwz r11, 0xd0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(208 as u32) ) } as u64;
	// 82ECC0B4: 814B003C  lwz r10, 0x3c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 82ECC0B8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ECC0BC: 4E800421  bctrl
	ctx.lr = 0x82ECC0C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECC0C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECC0C4: 4BFFFDCD  bl 0x82ecbe90
	ctx.lr = 0x82ECC0C8;
	sub_82ECBE90(ctx, base);
	// 82ECC0C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82ECC0CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ECC0D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ECC0D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ECC0D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECC0E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ECC0E0 size=176
    let mut pc: u32 = 0x82ECC0E0;
    'dispatch: loop {
        match pc {
            0x82ECC0E0 => {
    //   block [0x82ECC0E0..0x82ECC190)
	// 82ECC0E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ECC0E4: 482DC089  bl 0x831a816c
	ctx.lr = 0x82ECC0E8;
	sub_831A8130(ctx, base);
	// 82ECC0E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ECC0EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ECC0F0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82ECC0F4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82ECC0F8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ECC0FC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ECC100: 419A0064  beq cr6, 0x82ecc164
	if ctx.cr[6].eq {
	pc = 0x82ECC164; continue 'dispatch;
	}
	// 82ECC104: 816B0084  lwz r11, 0x84(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(132 as u32) ) } as u64;
	// 82ECC108: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ECC10C: 419A0058  beq cr6, 0x82ecc164
	if ctx.cr[6].eq {
	pc = 0x82ECC164; continue 'dispatch;
	}
	// 82ECC110: 39600017  li r11, 0x17
	ctx.r[11].s64 = 23;
	// 82ECC114: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 82ECC118: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECC11C: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82ECC120: 99610050  stb r11, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 82ECC124: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82ECC128: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 82ECC12C: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82ECC130: 4BFD4601  bl 0x82ea0730
	ctx.lr = 0x82ECC134;
	sub_82EA0730(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECC190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82ECC190 size=156
    let mut pc: u32 = 0x82ECC190;
    'dispatch: loop {
        match pc {
            0x82ECC190 => {
    //   block [0x82ECC190..0x82ECC22C)
	// 82ECC190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ECC194: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ECC198: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82ECC19C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ECC1A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ECC1A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ECC1A8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82ECC1AC: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ECC1B0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82ECC1B4: 419A0040  beq cr6, 0x82ecc1f4
	if ctx.cr[6].eq {
	pc = 0x82ECC1F4; continue 'dispatch;
	}
	// 82ECC1B8: 81630084  lwz r11, 0x84(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(132 as u32) ) } as u64;
	// 82ECC1BC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ECC1C0: 419A0034  beq cr6, 0x82ecc1f4
	if ctx.cr[6].eq {
	pc = 0x82ECC1F4; continue 'dispatch;
	}
	// 82ECC1C4: 39600018  li r11, 0x18
	ctx.r[11].s64 = 24;
	// 82ECC1C8: C01E0000  lfs f0, 0(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82ECC1CC: C1BE0004  lfs f13, 4(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82ECC1D0: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 82ECC1D4: C19E0008  lfs f12, 8(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82ECC1D8: 99610050  stb r11, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 82ECC1DC: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82ECC1E0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82ECC1E4: D1A1005C  stfs f13, 0x5c(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 82ECC1E8: D1810060  stfs f12, 0x60(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 82ECC1EC: 48000E15  bl 0x82ecd000
	ctx.lr = 0x82ECC1F0;
	sub_82ECD000(ctx, base);
	// 82ECC1F0: 48000024  b 0x82ecc214
	pc = 0x82ECC214; continue 'dispatch;
	// 82ECC1F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECC1F8: 4800CA71  bl 0x82ed8c68
	ctx.lr = 0x82ECC1FC;
	sub_82ED8C68(ctx, base);
	// 82ECC1FC: 817F00D0  lwz r11, 0xd0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(208 as u32) ) } as u64;
	// 82ECC200: 387F00D0  addi r3, r31, 0xd0
	ctx.r[3].s64 = ctx.r[31].s64 + 208;
	// 82ECC204: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82ECC208: 814B0040  lwz r10, 0x40(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 82ECC20C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ECC210: 4E800421  bctrl
	ctx.lr = 0x82ECC214;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECC214: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82ECC218: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ECC21C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ECC220: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82ECC224: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ECC228: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECC230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82ECC230 size=156
    let mut pc: u32 = 0x82ECC230;
    'dispatch: loop {
        match pc {
            0x82ECC230 => {
    //   block [0x82ECC230..0x82ECC2CC)
	// 82ECC230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ECC234: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ECC238: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82ECC23C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ECC240: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ECC244: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ECC248: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82ECC24C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ECC250: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82ECC254: 419A0040  beq cr6, 0x82ecc294
	if ctx.cr[6].eq {
	pc = 0x82ECC294; continue 'dispatch;
	}
	// 82ECC258: 81630084  lwz r11, 0x84(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(132 as u32) ) } as u64;
	// 82ECC25C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ECC260: 419A0034  beq cr6, 0x82ecc294
	if ctx.cr[6].eq {
	pc = 0x82ECC294; continue 'dispatch;
	}
	// 82ECC264: 39600019  li r11, 0x19
	ctx.r[11].s64 = 25;
	// 82ECC268: C01E0000  lfs f0, 0(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82ECC26C: C1BE0004  lfs f13, 4(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82ECC270: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 82ECC274: C19E0008  lfs f12, 8(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82ECC278: 99610050  stb r11, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 82ECC27C: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82ECC280: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82ECC284: D1A1005C  stfs f13, 0x5c(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 82ECC288: D1810060  stfs f12, 0x60(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 82ECC28C: 48000D75  bl 0x82ecd000
	ctx.lr = 0x82ECC290;
	sub_82ECD000(ctx, base);
	// 82ECC290: 48000024  b 0x82ecc2b4
	pc = 0x82ECC2B4; continue 'dispatch;
	// 82ECC294: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECC298: 4800C9D1  bl 0x82ed8c68
	ctx.lr = 0x82ECC29C;
	sub_82ED8C68(ctx, base);
	// 82ECC29C: 817F00D0  lwz r11, 0xd0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(208 as u32) ) } as u64;
	// 82ECC2A0: 387F00D0  addi r3, r31, 0xd0
	ctx.r[3].s64 = ctx.r[31].s64 + 208;
	// 82ECC2A4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82ECC2A8: 814B0044  lwz r10, 0x44(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 82ECC2AC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ECC2B0: 4E800421  bctrl
	ctx.lr = 0x82ECC2B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECC2B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82ECC2B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ECC2BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ECC2C0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82ECC2C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ECC2C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECC2D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82ECC2D0 size=156
    let mut pc: u32 = 0x82ECC2D0;
    'dispatch: loop {
        match pc {
            0x82ECC2D0 => {
    //   block [0x82ECC2D0..0x82ECC36C)
	// 82ECC2D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ECC2D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ECC2D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82ECC2DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ECC2E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ECC2E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ECC2E8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82ECC2EC: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ECC2F0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82ECC2F4: 419A0040  beq cr6, 0x82ecc334
	if ctx.cr[6].eq {
	pc = 0x82ECC334; continue 'dispatch;
	}
	// 82ECC2F8: 81630084  lwz r11, 0x84(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(132 as u32) ) } as u64;
	// 82ECC2FC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ECC300: 419A0034  beq cr6, 0x82ecc334
	if ctx.cr[6].eq {
	pc = 0x82ECC334; continue 'dispatch;
	}
	// 82ECC304: 3960001A  li r11, 0x1a
	ctx.r[11].s64 = 26;
	// 82ECC308: C01E0000  lfs f0, 0(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82ECC30C: C1BE0004  lfs f13, 4(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82ECC310: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 82ECC314: C19E0008  lfs f12, 8(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82ECC318: 99610050  stb r11, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 82ECC31C: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82ECC320: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82ECC324: D1A1005C  stfs f13, 0x5c(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 82ECC328: D1810060  stfs f12, 0x60(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 82ECC32C: 48000CD5  bl 0x82ecd000
	ctx.lr = 0x82ECC330;
	sub_82ECD000(ctx, base);
	// 82ECC330: 48000024  b 0x82ecc354
	pc = 0x82ECC354; continue 'dispatch;
	// 82ECC334: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECC338: 4800C931  bl 0x82ed8c68
	ctx.lr = 0x82ECC33C;
	sub_82ED8C68(ctx, base);
	// 82ECC33C: 817F00D0  lwz r11, 0xd0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(208 as u32) ) } as u64;
	// 82ECC340: 387F00D0  addi r3, r31, 0xd0
	ctx.r[3].s64 = ctx.r[31].s64 + 208;
	// 82ECC344: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82ECC348: 814B0048  lwz r10, 0x48(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 82ECC34C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ECC350: 4E800421  bctrl
	ctx.lr = 0x82ECC354;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECC354: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82ECC358: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ECC35C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ECC360: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82ECC364: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ECC368: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECC370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ECC370 size=176
    let mut pc: u32 = 0x82ECC370;
    'dispatch: loop {
        match pc {
            0x82ECC370 => {
    //   block [0x82ECC370..0x82ECC420)
	// 82ECC370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ECC374: 482DBDF9  bl 0x831a816c
	ctx.lr = 0x82ECC378;
	sub_831A8130(ctx, base);
	// 82ECC378: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ECC37C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ECC380: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82ECC384: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82ECC388: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ECC38C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ECC390: 419A0064  beq cr6, 0x82ecc3f4
	if ctx.cr[6].eq {
	pc = 0x82ECC3F4; continue 'dispatch;
	}
	// 82ECC394: 816B0084  lwz r11, 0x84(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(132 as u32) ) } as u64;
	// 82ECC398: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ECC39C: 419A0058  beq cr6, 0x82ecc3f4
	if ctx.cr[6].eq {
	pc = 0x82ECC3F4; continue 'dispatch;
	}
	// 82ECC3A0: 3960001B  li r11, 0x1b
	ctx.r[11].s64 = 27;
	// 82ECC3A4: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 82ECC3A8: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECC3AC: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82ECC3B0: 99610050  stb r11, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 82ECC3B4: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82ECC3B8: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 82ECC3BC: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82ECC3C0: 4BFD4371  bl 0x82ea0730
	ctx.lr = 0x82ECC3C4;
	sub_82EA0730(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECC420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82ECC420 size=156
    let mut pc: u32 = 0x82ECC420;
    'dispatch: loop {
        match pc {
            0x82ECC420 => {
    //   block [0x82ECC420..0x82ECC4BC)
	// 82ECC420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ECC424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ECC428: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82ECC42C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ECC430: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ECC434: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ECC438: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82ECC43C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ECC440: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82ECC444: 419A0040  beq cr6, 0x82ecc484
	if ctx.cr[6].eq {
	pc = 0x82ECC484; continue 'dispatch;
	}
	// 82ECC448: 81630084  lwz r11, 0x84(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(132 as u32) ) } as u64;
	// 82ECC44C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ECC450: 419A0034  beq cr6, 0x82ecc484
	if ctx.cr[6].eq {
	pc = 0x82ECC484; continue 'dispatch;
	}
	// 82ECC454: 3960001C  li r11, 0x1c
	ctx.r[11].s64 = 28;
	// 82ECC458: C01E0000  lfs f0, 0(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82ECC45C: C1BE0004  lfs f13, 4(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82ECC460: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 82ECC464: C19E0008  lfs f12, 8(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82ECC468: 99610050  stb r11, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 82ECC46C: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82ECC470: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82ECC474: D1A1005C  stfs f13, 0x5c(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 82ECC478: D1810060  stfs f12, 0x60(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 82ECC47C: 48000B85  bl 0x82ecd000
	ctx.lr = 0x82ECC480;
	sub_82ECD000(ctx, base);
	// 82ECC480: 48000024  b 0x82ecc4a4
	pc = 0x82ECC4A4; continue 'dispatch;
	// 82ECC484: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECC488: 4800C7E1  bl 0x82ed8c68
	ctx.lr = 0x82ECC48C;
	sub_82ED8C68(ctx, base);
	// 82ECC48C: 817F00D0  lwz r11, 0xd0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(208 as u32) ) } as u64;
	// 82ECC490: 387F00D0  addi r3, r31, 0xd0
	ctx.r[3].s64 = ctx.r[31].s64 + 208;
	// 82ECC494: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82ECC498: 814B0050  lwz r10, 0x50(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 82ECC49C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ECC4A0: 4E800421  bctrl
	ctx.lr = 0x82ECC4A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECC4A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82ECC4A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ECC4AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ECC4B0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82ECC4B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ECC4B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECC4C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ECC4C0 size=200
    let mut pc: u32 = 0x82ECC4C0;
    'dispatch: loop {
        match pc {
            0x82ECC4C0 => {
    //   block [0x82ECC4C0..0x82ECC588)
	// 82ECC4C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ECC4C4: 482DBCA9  bl 0x831a816c
	ctx.lr = 0x82ECC4C8;
	sub_831A8130(ctx, base);
	// 82ECC4C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ECC4CC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82ECC4D0: 83FD0010  lwz r31, 0x10(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82ECC4D4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82ECC4D8: 419A00A8  beq cr6, 0x82ecc580
	if ctx.cr[6].eq {
	pc = 0x82ECC580; continue 'dispatch;
	}
	// 82ECC4DC: 897D00D8  lbz r11, 0xd8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(216 as u32) ) } as u64;
	// 82ECC4E0: 83DF000C  lwz r30, 0xc(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ECC4E4: 2B0B0007  cmplwi cr6, r11, 7
	ctx.cr[6].compare_u32(ctx.r[11].u32, 7 as u32, &mut ctx.xer);
	// 82ECC4E8: 419A0010  beq cr6, 0x82ecc4f8
	if ctx.cr[6].eq {
	pc = 0x82ECC4F8; continue 'dispatch;
	}
	// 82ECC4EC: 2B0B0006  cmplwi cr6, r11, 6
	ctx.cr[6].compare_u32(ctx.r[11].u32, 6 as u32, &mut ctx.xer);
	// 82ECC4F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82ECC4F4: 409A0008  bne cr6, 0x82ecc4fc
	if !ctx.cr[6].eq {
	pc = 0x82ECC4FC; continue 'dispatch;
	}
	// 82ECC4F8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82ECC4FC: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82ECC500: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ECC504: 409A0038  bne cr6, 0x82ecc53c
	if !ctx.cr[6].eq {
	pc = 0x82ECC53C; continue 'dispatch;
	}
	// 82ECC508: 2F1E000B  cmpwi cr6, r30, 0xb
	ctx.cr[6].compare_i32(ctx.r[30].s32, 11, &mut ctx.xer);
	// 82ECC50C: 419A000C  beq cr6, 0x82ecc518
	if ctx.cr[6].eq {
	pc = 0x82ECC518; continue 'dispatch;
	}
	// 82ECC510: 2F1E0009  cmpwi cr6, r30, 9
	ctx.cr[6].compare_i32(ctx.r[30].s32, 9, &mut ctx.xer);
	// 82ECC514: 409A0028  bne cr6, 0x82ecc53c
	if !ctx.cr[6].eq {
	pc = 0x82ECC53C; continue 'dispatch;
	}
	// 82ECC518: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECC51C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECC520: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82ECC524: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ECC528: 4E800421  bctrl
	ctx.lr = 0x82ECC52C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECC52C: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECC530: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECC534: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82ECC538: 4E800421  bctrl
	ctx.lr = 0x82ECC53C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECC53C: 2F1E0008  cmpwi cr6, r30, 8
	ctx.cr[6].compare_i32(ctx.r[30].s32, 8, &mut ctx.xer);
	// 82ECC540: 409A0024  bne cr6, 0x82ecc564
	if !ctx.cr[6].eq {
	pc = 0x82ECC564; continue 'dispatch;
	}
	// 82ECC544: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82ECC548: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82ECC54C: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82ECC550: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECC554: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ECC558: 4E800421  bctrl
	ctx.lr = 0x82ECC55C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECC55C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82ECC560: 482DBC5C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 82ECC564: 2F1E000A  cmpwi cr6, r30, 0xa
	ctx.cr[6].compare_i32(ctx.r[30].s32, 10, &mut ctx.xer);
	// 82ECC568: 409A0018  bne cr6, 0x82ecc580
	if !ctx.cr[6].eq {
	pc = 0x82ECC580; continue 'dispatch;
	}
	// 82ECC56C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82ECC570: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82ECC574: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECC578: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ECC57C: 4E800421  bctrl
	ctx.lr = 0x82ECC580;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECC580: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82ECC584: 482DBC38  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECC588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82ECC588 size=340
    let mut pc: u32 = 0x82ECC588;
    'dispatch: loop {
        match pc {
            0x82ECC588 => {
    //   block [0x82ECC588..0x82ECC6DC)
	// 82ECC588: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ECC58C: 482DBBE1  bl 0x831a816c
	ctx.lr = 0x82ECC590;
	sub_831A8130(ctx, base);
	// 82ECC590: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ECC594: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82ECC598: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82ECC59C: 394001A0  li r10, 0x1a0
	ctx.r[10].s64 = 416;
	// 82ECC5A0: 39200030  li r9, 0x30
	ctx.r[9].s64 = 48;
	// 82ECC5A4: 38E001B0  li r7, 0x1b0
	ctx.r[7].s64 = 432;
	// 82ECC5A8: 897E0021  lbz r11, 0x21(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(33 as u32) ) } as u64;
	// 82ECC5AC: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 82ECC5B0: 3BBE00D0  addi r29, r30, 0xd0
	ctx.r[29].s64 = ctx.r[30].s64 + 208;
	// 82ECC5B4: 556807FE  clrlwi r8, r11, 0x1f
	ctx.r[8].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 82ECC5B8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82ECC5BC: 991F00B6  stb r8, 0xb6(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(182 as u32), ctx.r[8].u8 ) };
	// 82ECC5C0: 88BE00BC  lbz r5, 0xbc(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(188 as u32) ) } as u64;
	// 82ECC5C4: 98BF00B4  stb r5, 0xb4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[5].u8 ) };
	// 82ECC5C8: 889E00BD  lbz r4, 0xbd(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(189 as u32) ) } as u64;
	// 82ECC5CC: 989F00B5  stb r4, 0xb5(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(181 as u32), ctx.r[4].u8 ) };
	// 82ECC5D0: A17E0096  lhz r11, 0x96(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(150 as u32) ) } as u64;
	// 82ECC5D4: B17F000A  sth r11, 0xa(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(10 as u32), ctx.r[11].u16 ) };
	// 82ECC5D8: 897E00D9  lbz r11, 0xd9(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(217 as u32) ) } as u64;
	// 82ECC5DC: 390BFF01  addi r8, r11, -0xff
	ctx.r[8].s64 = ctx.r[11].s64 + -255;
	// 82ECC5E0: 7D050034  cntlzw r5, r8
	ctx.r[5].u64 = if ctx.r[8].u32 == 0 { 32 } else { ctx.r[8].u32.leading_zeros() as u64 };
	// 82ECC5E4: 54A4DFFE  rlwinm r4, r5, 0x1b, 0x1f, 0x1f
	ctx.r[4].u64 = ctx.r[5].u32 as u64 & 0x0000001Fu64;
	// 82ECC5E8: 688B0001  xori r11, r4, 1
	ctx.r[11].u64 = ctx.r[4].u64 ^ 1;
	// 82ECC5EC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82ECC5F0: 997F00B1  stb r11, 0xb1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(177 as u32), ctx.r[11].u8 ) };
	// 82ECC5F4: C01E0084  lfs f0, 0x84(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(132 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82ECC5F8: D01F009C  stfs f0, 0x9c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(156 as u32), tmp.u32 ) };
	// 82ECC5FC: 891E0080  lbz r8, 0x80(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(128 as u32) ) } as u64;
	// 82ECC600: 991F0008  stb r8, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[8].u8 ) };
	// 82ECC604: C1BE0088  lfs f13, 0x88(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(136 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82ECC608: D1BF00A0  stfs f13, 0xa0(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(160 as u32), tmp.u32 ) };
	// 82ECC60C: C19E0184  lfs f12, 0x184(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(388 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82ECC610: D19F0094  stfs f12, 0x94(r31)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), tmp.u32 ) };
	// 82ECC614: C17E0188  lfs f11, 0x188(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(392 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82ECC618: D17F0098  stfs f11, 0x98(r31)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(152 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECC6E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ECC6E0 size=344
    let mut pc: u32 = 0x82ECC6E0;
    'dispatch: loop {
        match pc {
            0x82ECC6E0 => {
    //   block [0x82ECC6E0..0x82ECC838)
	// 82ECC6E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ECC6E4: 482DBA85  bl 0x831a8168
	ctx.lr = 0x82ECC6E8;
	sub_831A8130(ctx, base);
	// 82ECC6E8: 9421FEC0  stwu r1, -0x140(r1)
	ea = ctx.r[1].u32.wrapping_add(-320 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ECC6EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ECC6F0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82ECC6F4: 480002F5  bl 0x82ecc9e8
	ctx.lr = 0x82ECC6F8;
	sub_82ECC9E8(ctx, base);
	// 82ECC6F8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82ECC6FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECC700: 4BFFFE89  bl 0x82ecc588
	ctx.lr = 0x82ECC704;
	sub_82ECC588(ctx, base);
	// 82ECC704: 83AD0000  lwz r29, 0(r13)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECC708: 3B800014  li r28, 0x14
	ctx.r[28].s64 = 20;
	// 82ECC70C: 38A0002D  li r5, 0x2d
	ctx.r[5].s64 = 45;
	// 82ECC710: 38800200  li r4, 0x200
	ctx.r[4].s64 = 512;
	// 82ECC714: 7C7CE82E  lwzx r3, r28, r29
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82ECC718: 4BFD4019  bl 0x82ea0730
	ctx.lr = 0x82ECC71C;
	sub_82EA0730(ctx, base);
	// 82ECC71C: 39600200  li r11, 0x200
	ctx.r[11].s64 = 512;
	// 82ECC720: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82ECC724: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82ECC728: 4BFFF1B9  bl 0x82ecb8e0
	ctx.lr = 0x82ECC72C;
	sub_82ECB8E0(ctx, base);
	// 82ECC72C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82ECC730: 39600012  li r11, 0x12
	ctx.r[11].s64 = 18;
	// 82ECC734: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82ECC738: 395F00D0  addi r10, r31, 0xd0
	ctx.r[10].s64 = ctx.r[31].s64 + 208;
	// 82ECC73C: 7D3EF850  subf r9, r30, r31
	ctx.r[9].s64 = ctx.r[31].s64 - ctx.r[30].s64;
	// 82ECC740: 7D7F5050  subf r11, r31, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[31].s64;
	// 82ECC744: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82ECC748: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82ECC74C: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECC750: 910BFFF8  stw r8, -8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-8 as u32), ctx.r[8].u32 ) };
	// 82ECC754: 80EA0004  lwz r7, 4(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECC758: 90EBFFFC  stw r7, -4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), ctx.r[7].u32 ) };
	// 82ECC75C: 7CC9582E  lwzx r6, r9, r11
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82ECC760: 90CB0000  stw r6, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82ECC764: 80AA000C  lwz r5, 0xc(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ECC768: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 82ECC76C: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82ECC770: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82ECC774: 4200FFD8  bdnz 0x82ecc74c
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82ECC74C; continue 'dispatch;
	}
	// 82ECC778: 817F01E8  lwz r11, 0x1e8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(488 as u32) ) } as u64;
	// 82ECC77C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ECC780: 419A0078  beq cr6, 0x82ecc7f8
	if ctx.cr[6].eq {
	pc = 0x82ECC7F8; continue 'dispatch;
	}
	// 82ECC784: 38A0002E  li r5, 0x2e
	ctx.r[5].s64 = 46;
	// 82ECC788: 7C7CE82E  lwzx r3, r28, r29
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82ECC78C: 38800120  li r4, 0x120
	ctx.r[4].s64 = 288;
	// 82ECC790: 4BFD3FA1  bl 0x82ea0730
	ctx.lr = 0x82ECC794;
	sub_82EA0730(ctx, base);
	// 82ECC794: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82ECC798: 39400120  li r10, 0x120
	ctx.r[10].s64 = 288;
	// 82ECC79C: 390B4144  addi r8, r11, 0x4144
	ctx.r[8].s64 = ctx.r[11].s64 + 16708;
	// 82ECC7A0: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82ECC7A4: B1430004  sth r10, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 82ECC7A8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82ECC7AC: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82ECC7B0: B0E30006  sth r7, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[7].u16 ) };
	// 82ECC7B4: 39200012  li r9, 0x12
	ctx.r[9].s64 = 18;
	// 82ECC7B8: 90C30118  stw r6, 0x118(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(280 as u32), ctx.r[6].u32 ) };
	// 82ECC7BC: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82ECC7C0: 907E01E8  stw r3, 0x1e8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(488 as u32), ctx.r[3].u32 ) };
	// 82ECC7C4: 817F01E8  lwz r11, 0x1e8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(488 as u32) ) } as u64;
	// 82ECC7C8: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECC7CC: 3529FFFF  addic. r9, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82ECC7D0: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82ECC7D4: 80EB0004  lwz r7, 4(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECC7D8: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82ECC7DC: 80CB0008  lwz r6, 8(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ECC7E0: 90CA0008  stw r6, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 82ECC7E4: 80AB000C  lwz r5, 0xc(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ECC7E8: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82ECC7EC: 90AA000C  stw r5, 0xc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), ctx.r[5].u32 ) };
	// 82ECC7F0: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 82ECC7F4: 4181FFD4  bgt 0x82ecc7c8
	if ctx.cr[0].gt {
	pc = 0x82ECC7C8; continue 'dispatch;
	}
	// 82ECC7F8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECC7FC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82ECC800: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82ECC804: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ECC808: 4E800421  bctrl
	ctx.lr = 0x82ECC80C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECC80C: 907E0018  stw r3, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[3].u32 ) };
	// 82ECC810: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82ECC814: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82ECC818: 480000F1  bl 0x82ecc908
	ctx.lr = 0x82ECC81C;
	sub_82ECC908(ctx, base);
	// 82ECC81C: 813F0070  lwz r9, 0x70(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 82ECC820: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82ECC824: 913E0070  stw r9, 0x70(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(112 as u32), ctx.r[9].u32 ) };
	// 82ECC828: 811F000C  lwz r8, 0xc(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ECC82C: 911E000C  stw r8, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82ECC830: 38210140  addi r1, r1, 0x140
	ctx.r[1].s64 = ctx.r[1].s64 + 320;
	// 82ECC834: 482DB984  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECC838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ECC838 size=112
    let mut pc: u32 = 0x82ECC838;
    'dispatch: loop {
        match pc {
            0x82ECC838 => {
    //   block [0x82ECC838..0x82ECC8A8)
	// 82ECC838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ECC83C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ECC840: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82ECC844: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ECC848: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ECC84C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ECC850: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82ECC854: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82ECC858: 394B3F10  addi r10, r11, 0x3f10
	ctx.r[10].s64 = ctx.r[11].s64 + 16144;
	// 82ECC85C: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82ECC860: 4800D3C1  bl 0x82ed9c20
	ctx.lr = 0x82ECC864;
	sub_82ED9C20(ctx, base);
	// 82ECC864: 57C907FE  clrlwi r9, r30, 0x1f
	ctx.r[9].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82ECC868: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82ECC86C: 419A0020  beq cr6, 0x82ecc88c
	if ctx.cr[6].eq {
	pc = 0x82ECC88C; continue 'dispatch;
	}
	// 82ECC870: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECC874: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82ECC878: 38C0002D  li r6, 0x2d
	ctx.r[6].s64 = 45;
	// 82ECC87C: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECC880: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82ECC884: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82ECC888: 4BFD3F29  bl 0x82ea07b0
	ctx.lr = 0x82ECC88C;
	sub_82EA07B0(ctx, base);
	// 82ECC88C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECC890: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82ECC894: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ECC898: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ECC89C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82ECC8A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ECC8A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECC8A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ECC8A8 size=96
    let mut pc: u32 = 0x82ECC8A8;
    'dispatch: loop {
        match pc {
            0x82ECC8A8 => {
    //   block [0x82ECC8A8..0x82ECC908)
	// 82ECC8A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ECC8AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ECC8B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ECC8B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ECC8B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ECC8BC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82ECC8C0: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 82ECC8C4: 392B9EAC  addi r9, r11, -0x6154
	ctx.r[9].s64 = ctx.r[11].s64 + -24916;
	// 82ECC8C8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82ECC8CC: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82ECC8D0: 419A0020  beq cr6, 0x82ecc8f0
	if ctx.cr[6].eq {
	pc = 0x82ECC8F0; continue 'dispatch;
	}
	// 82ECC8D4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECC8D8: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82ECC8DC: 38C0002E  li r6, 0x2e
	ctx.r[6].s64 = 46;
	// 82ECC8E0: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECC8E4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82ECC8E8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82ECC8EC: 4BFD3EC5  bl 0x82ea07b0
	ctx.lr = 0x82ECC8F0;
	sub_82EA07B0(ctx, base);
	// 82ECC8F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECC8F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82ECC8F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ECC8FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ECC900: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ECC904: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECC908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ECC908 size=224
    let mut pc: u32 = 0x82ECC908;
    'dispatch: loop {
        match pc {
            0x82ECC908 => {
    //   block [0x82ECC908..0x82ECC9E8)
	// 82ECC908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ECC90C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ECC910: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82ECC914: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ECC918: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ECC91C: 8163007C  lwz r11, 0x7c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(124 as u32) ) } as u64;
	// 82ECC920: 3BE30074  addi r31, r3, 0x74
	ctx.r[31].s64 = ctx.r[3].s64 + 116;
	// 82ECC924: 81240078  lwz r9, 0x78(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(120 as u32) ) } as u64;
	// 82ECC928: 3BC40074  addi r30, r4, 0x74
	ctx.r[30].s64 = ctx.r[4].s64 + 116;
	// 82ECC92C: 556A00BE  clrlwi r10, r11, 2
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82ECC930: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82ECC934: 40980060  bge cr6, 0x82ecc994
	if !ctx.cr[6].lt {
	pc = 0x82ECC994; continue 'dispatch;
	}
	// 82ECC938: 556B0000  rlwinm r11, r11, 0, 0, 0
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ECC93C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ECC940: 409A0020  bne cr6, 0x82ecc960
	if !ctx.cr[6].eq {
	pc = 0x82ECC960; continue 'dispatch;
	}
	// 82ECC944: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECC948: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82ECC94C: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82ECC950: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECC954: 55452036  slwi r5, r10, 4
	ctx.r[5].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82ECC958: 7C69582E  lwzx r3, r9, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82ECC95C: 4BFD3E55  bl 0x82ea07b0
	ctx.lr = 0x82ECC960;
	sub_82EA07B0(ctx, base);
	// 82ECC960: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECC964: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82ECC968: 813E0004  lwz r9, 4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECC96C: 38A00017  li r5, 0x17
	ctx.r[5].s64 = 23;
	// 82ECC970: 55242036  slwi r4, r9, 4
	ctx.r[4].u32 = ctx.r[9].u32.wrapping_shl(4);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82ECC974: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82ECC978: 4BFD3DB9  bl 0x82ea0730
	ctx.lr = 0x82ECC97C;
	sub_82EA0730(ctx, base);
	// 82ECC97C: 811F0008  lwz r8, 8(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ECC980: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82ECC984: 55070042  rlwinm r7, r8, 0, 1, 1
	ctx.r[7].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	// 82ECC988: 80DE0004  lwz r6, 4(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECC98C: 7CE53378  or r5, r7, r6
	ctx.r[5].u64 = ctx.r[7].u64 | ctx.r[6].u64;
	// 82ECC990: 90BF0008  stw r5, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 82ECC994: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECC998: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECC99C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ECC9A0: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82ECC9A4: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECC9A8: 40990028  ble cr6, 0x82ecc9d0
	if !ctx.cr[6].gt {
	pc = 0x82ECC9D0; continue 'dispatch;
	}
	// 82ECC9AC: 7D2B4850  subf r9, r11, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 82ECC9B0: 7D09582A  ldx r8, r9, r11
	ctx.r[8].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) };
	// 82ECC9B4: 7CE95A14  add r7, r9, r11
	ctx.r[7].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82ECC9B8: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ECC9BC: F90B0000  std r8, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u64 ) };
	// 82ECC9C0: E8C70008  ld r6, 8(r7)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(8 as u32) ) };
	// 82ECC9C4: F8CB0008  std r6, 8(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[6].u64 ) };
	// 82ECC9C8: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82ECC9CC: 4082FFE4  bne 0x82ecc9b0
	if !ctx.cr[0].eq {
	pc = 0x82ECC9B0; continue 'dispatch;
	}
	// 82ECC9D0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82ECC9D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ECC9D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ECC9DC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82ECC9E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ECC9E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECC9E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ECC9E8 size=296
    let mut pc: u32 = 0x82ECC9E8;
    'dispatch: loop {
        match pc {
            0x82ECC9E8 => {
    //   block [0x82ECC9E8..0x82ECCB10)
	// 82ECC9E8: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 82ECC9EC: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82ECC9F0: 39230010  addi r9, r3, 0x10
	ctx.r[9].s64 = ctx.r[3].s64 + 16;
	// 82ECC9F4: 3961FFE0  addi r11, r1, -0x20
	ctx.r[11].s64 = ctx.r[1].s64 + -32;
	// 82ECC9F8: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82ECC9FC: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 82ECCA00: 38CAFFE0  addi r6, r10, -0x20
	ctx.r[6].s64 = ctx.r[10].s64 + -32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECCB10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ECCB10 size=96
    let mut pc: u32 = 0x82ECCB10;
    'dispatch: loop {
        match pc {
            0x82ECCB10 => {
    //   block [0x82ECCB10..0x82ECCB70)
	// 82ECCB10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ECCB14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ECCB18: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ECCB1C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ECCB20: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82ECCB24: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82ECCB28: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82ECCB2C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECCB30: 388A41DC  addi r4, r10, 0x41dc
	ctx.r[4].s64 = ctx.r[10].s64 + 16860;
	// 82ECCB34: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECCB38: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82ECCB3C: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECCB40: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82ECCB44: 4E800421  bctrl
	ctx.lr = 0x82ECCB48;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECCB48: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECCB4C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECCB50: 80E80018  lwz r7, 0x18(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(24 as u32) ) } as u64;
	// 82ECCB54: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 82ECCB58: 4E800421  bctrl
	ctx.lr = 0x82ECCB5C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECCB5C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82ECCB60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ECCB64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ECCB68: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ECCB6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECCB70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ECCB70 size=12
    let mut pc: u32 = 0x82ECCB70;
    'dispatch: loop {
        match pc {
            0x82ECCB70 => {
    //   block [0x82ECCB70..0x82ECCB7C)
	// 82ECCB70: 39600046  li r11, 0x46
	ctx.r[11].s64 = 70;
	// 82ECCB74: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82ECCB78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECCB80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ECCB80 size=348
    let mut pc: u32 = 0x82ECCB80;
    'dispatch: loop {
        match pc {
            0x82ECCB80 => {
    //   block [0x82ECCB80..0x82ECCCDC)
	// 82ECCB80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ECCB84: 482DB5E5  bl 0x831a8168
	ctx.lr = 0x82ECCB88;
	sub_831A8130(ctx, base);
	// 82ECCB88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ECCB8C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82ECCB90: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82ECCB94: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82ECCB98: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82ECCB9C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82ECCBA0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECCBA4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82ECCBA8: 814B0020  lwz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82ECCBAC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ECCBB0: 4E800421  bctrl
	ctx.lr = 0x82ECCBB4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECCBB4: 813C0008  lwz r9, 8(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ECCBB8: 3BBE0010  addi r29, r30, 0x10
	ctx.r[29].s64 = ctx.r[30].s64 + 16;
	// 82ECCBBC: 389C0008  addi r4, r28, 8
	ctx.r[4].s64 = ctx.r[28].s64 + 8;
	// 82ECCBC0: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82ECCBC4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82ECCBC8: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECCBCC: 38610053  addi r3, r1, 0x53
	ctx.r[3].s64 = ctx.r[1].s64 + 83;
	// 82ECCBD0: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82ECCBD4: 4E800421  bctrl
	ctx.lr = 0x82ECCBD8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECCBD8: 88E30000  lbz r7, 0(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECCBDC: 89610050  lbz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82ECCBE0: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82ECCBE4: 419A0074  beq cr6, 0x82eccc58
	if ctx.cr[6].eq {
	pc = 0x82ECCC58; continue 'dispatch;
	}
	// 82ECCBE8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ECCBEC: 409A001C  bne cr6, 0x82eccc08
	if !ctx.cr[6].eq {
	pc = 0x82ECCC08; continue 'dispatch;
	}
	// 82ECCBF0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECCBF4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82ECCBF8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82ECCBFC: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82ECCC00: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ECCC04: 4E800421  bctrl
	ctx.lr = 0x82ECCC08;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECCC08: 897F0018  lbz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82ECCC0C: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 82ECCC10: 409A00C4  bne cr6, 0x82ecccd4
	if !ctx.cr[6].eq {
	pc = 0x82ECCCD4; continue 'dispatch;
	}
	// 82ECCC14: 897F0010  lbz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82ECCC18: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82ECCC1C: 38610051  addi r3, r1, 0x51
	ctx.r[3].s64 = ctx.r[1].s64 + 81;
	// 82ECCC20: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82ECCC24: 7C8BFA14  add r4, r11, r31
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82ECCC28: 7D4BF82E  lwzx r10, r11, r31
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82ECCC2C: 812A0020  lwz r9, 0x20(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(32 as u32) ) } as u64;
	// 82ECCC30: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82ECCC34: 4E800421  bctrl
	ctx.lr = 0x82ECCC38;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECCC38: 89010051  lbz r8, 0x51(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(81 as u32) ) } as u64;
	// 82ECCC3C: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82ECCC40: 409A0094  bne cr6, 0x82ecccd4
	if !ctx.cr[6].eq {
	pc = 0x82ECCCD4; continue 'dispatch;
	}
	// 82ECCC44: 897F0010  lbz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82ECCC48: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82ECCC4C: 7D4BF82E  lwzx r10, r11, r31
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82ECCC50: 812A001C  lwz r9, 0x1c(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 82ECCC54: 48000070  b 0x82ecccc4
	pc = 0x82ECCCC4; continue 'dispatch;
	// 82ECCC58: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ECCC5C: 419A001C  beq cr6, 0x82eccc78
	if ctx.cr[6].eq {
	pc = 0x82ECCC78; continue 'dispatch;
	}
	// 82ECCC60: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECCC64: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82ECCC68: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82ECCC6C: 814B0024  lwz r10, 0x24(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82ECCC70: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ECCC74: 4E800421  bctrl
	ctx.lr = 0x82ECCC78;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECCC78: 897F0018  lbz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82ECCC7C: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 82ECCC80: 409A0054  bne cr6, 0x82ecccd4
	if !ctx.cr[6].eq {
	pc = 0x82ECCCD4; continue 'dispatch;
	}
	// 82ECCC84: 897F0010  lbz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82ECCC88: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82ECCC8C: 38610052  addi r3, r1, 0x52
	ctx.r[3].s64 = ctx.r[1].s64 + 82;
	// 82ECCC90: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82ECCC94: 7C8BFA14  add r4, r11, r31
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82ECCC98: 7D4BF82E  lwzx r10, r11, r31
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82ECCC9C: 812A0020  lwz r9, 0x20(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(32 as u32) ) } as u64;
	// 82ECCCA0: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82ECCCA4: 4E800421  bctrl
	ctx.lr = 0x82ECCCA8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECCCA8: 89010052  lbz r8, 0x52(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(82 as u32) ) } as u64;
	// 82ECCCAC: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82ECCCB0: 419A0024  beq cr6, 0x82ecccd4
	if ctx.cr[6].eq {
	pc = 0x82ECCCD4; continue 'dispatch;
	}
	// 82ECCCB4: 897F0010  lbz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82ECCCB8: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82ECCCBC: 7D4BF82E  lwzx r10, r11, r31
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82ECCCC0: 812A0024  lwz r9, 0x24(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(36 as u32) ) } as u64;
	// 82ECCCC4: 7C6BFA14  add r3, r11, r31
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82ECCCC8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82ECCCCC: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82ECCCD0: 4E800421  bctrl
	ctx.lr = 0x82ECCCD4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECCCD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82ECCCD8: 482DB4E0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECCCE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ECCCE0 size=16
    let mut pc: u32 = 0x82ECCCE0;
    'dispatch: loop {
        match pc {
            0x82ECCCE0 => {
    //   block [0x82ECCCE0..0x82ECCCF0)
	// 82ECCCE0: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECCCF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82ECCCF0 size=12
    let mut pc: u32 = 0x82ECCCF0;
    'dispatch: loop {
        match pc {
            0x82ECCCF0 => {
    //   block [0x82ECCCF0..0x82ECCCFC)
	// 82ECCCF0: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ECCCF4: C02B0014  lfs f1, 0x14(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82ECCCF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECCD00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82ECCD00 size=12
    let mut pc: u32 = 0x82ECCD00;
    'dispatch: loop {
        match pc {
            0x82ECCD00 => {
    //   block [0x82ECCD00..0x82ECCD0C)
	// 82ECCD00: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ECCD04: C02B0018  lfs f1, 0x18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82ECCD08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECCD10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ECCD10 size=20
    let mut pc: u32 = 0x82ECCD10;
    'dispatch: loop {
        match pc {
            0x82ECCD10 => {
    //   block [0x82ECCD10..0x82ECCD24)
	// 82ECCD10: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ECCD14: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECCD18: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82ECCD1C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ECCD20: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECCD28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ECCD28 size=20
    let mut pc: u32 = 0x82ECCD28;
    'dispatch: loop {
        match pc {
            0x82ECCD28 => {
    //   block [0x82ECCD28..0x82ECCD3C)
	// 82ECCD28: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ECCD2C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECCD30: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82ECCD34: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ECCD38: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECCD40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ECCD40 size=8
    let mut pc: u32 = 0x82ECCD40;
    'dispatch: loop {
        match pc {
            0x82ECCD40 => {
    //   block [0x82ECCD40..0x82ECCD48)
	// 82ECCD40: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ECCD44: 48018314  b 0x82ee5058
	sub_82EE5058(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECCD48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ECCD48 size=8
    let mut pc: u32 = 0x82ECCD48;
    'dispatch: loop {
        match pc {
            0x82ECCD48 => {
    //   block [0x82ECCD48..0x82ECCD50)
	// 82ECCD48: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ECCD4C: 4801831C  b 0x82ee5068
	sub_82EE5068(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECCD50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ECCD50 size=8
    let mut pc: u32 = 0x82ECCD50;
    'dispatch: loop {
        match pc {
            0x82ECCD50 => {
    //   block [0x82ECCD50..0x82ECCD58)
	// 82ECCD50: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ECCD54: 48018334  b 0x82ee5088
	sub_82EE5088(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECCD58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ECCD58 size=8
    let mut pc: u32 = 0x82ECCD58;
    'dispatch: loop {
        match pc {
            0x82ECCD58 => {
    //   block [0x82ECCD58..0x82ECCD60)
	// 82ECCD58: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ECCD5C: 4801CDFC  b 0x82ee9b58
	sub_82EE9B58(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECCD60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ECCD60 size=8
    let mut pc: u32 = 0x82ECCD60;
    'dispatch: loop {
        match pc {
            0x82ECCD60 => {
    //   block [0x82ECCD60..0x82ECCD68)
	// 82ECCD60: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ECCD64: 4801C454  b 0x82ee91b8
	sub_82EE91B8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECCD68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ECCD68 size=8
    let mut pc: u32 = 0x82ECCD68;
    'dispatch: loop {
        match pc {
            0x82ECCD68 => {
    //   block [0x82ECCD68..0x82ECCD70)
	// 82ECCD68: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ECCD6C: 4801C3EC  b 0x82ee9158
	sub_82EE9158(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECCD70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ECCD70 size=8
    let mut pc: u32 = 0x82ECCD70;
    'dispatch: loop {
        match pc {
            0x82ECCD70 => {
    //   block [0x82ECCD70..0x82ECCD78)
	// 82ECCD70: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ECCD74: 4801C0D4  b 0x82ee8e48
	sub_82EE8E48(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECCD78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ECCD78 size=20
    let mut pc: u32 = 0x82ECCD78;
    'dispatch: loop {
        match pc {
            0x82ECCD78 => {
    //   block [0x82ECCD78..0x82ECCD8C)
	// 82ECCD78: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ECCD7C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECCD80: 814B0048  lwz r10, 0x48(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 82ECCD84: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ECCD88: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECCD90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ECCD90 size=20
    let mut pc: u32 = 0x82ECCD90;
    'dispatch: loop {
        match pc {
            0x82ECCD90 => {
    //   block [0x82ECCD90..0x82ECCDA4)
	// 82ECCD90: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ECCD94: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECCD98: 814B004C  lwz r10, 0x4c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 82ECCD9C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ECCDA0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECCDA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ECCDA8 size=20
    let mut pc: u32 = 0x82ECCDA8;
    'dispatch: loop {
        match pc {
            0x82ECCDA8 => {
    //   block [0x82ECCDA8..0x82ECCDBC)
	// 82ECCDA8: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ECCDAC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECCDB0: 814B0050  lwz r10, 0x50(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 82ECCDB4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ECCDB8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECCDC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ECCDC0 size=4
    let mut pc: u32 = 0x82ECCDC0;
    'dispatch: loop {
        match pc {
            0x82ECCDC0 => {
    //   block [0x82ECCDC0..0x82ECCDC4)
	// 82ECCDC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECCDC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ECCDC8 size=8
    let mut pc: u32 = 0x82ECCDC8;
    'dispatch: loop {
        match pc {
            0x82ECCDC8 => {
    //   block [0x82ECCDC8..0x82ECCDD0)
	// 82ECCDC8: 80630050  lwz r3, 0x50(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(80 as u32) ) } as u64;
	// 82ECCDCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECCDD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ECCDD0 size=156
    let mut pc: u32 = 0x82ECCDD0;
    'dispatch: loop {
        match pc {
            0x82ECCDD0 => {
    //   block [0x82ECCDD0..0x82ECCE6C)
	// 82ECCDD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ECCDD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ECCDD8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82ECCDDC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ECCDE0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ECCDE4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82ECCDE8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82ECCDEC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82ECCDF0: 419A001C  beq cr6, 0x82ecce0c
	if ctx.cr[6].eq {
	pc = 0x82ECCE0C; continue 'dispatch;
	}
	// 82ECCDF4: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECCDF8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ECCDFC: 419A0010  beq cr6, 0x82ecce0c
	if ctx.cr[6].eq {
	pc = 0x82ECCE0C; continue 'dispatch;
	}
	// 82ECCE00: A17F0006  lhz r11, 6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 82ECCE04: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 82ECCE08: B15F0006  sth r10, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82ECCE0C: 807E0050  lwz r3, 0x50(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(80 as u32) ) } as u64;
	// 82ECCE10: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ECCE14: 419A003C  beq cr6, 0x82ecce50
	if ctx.cr[6].eq {
	pc = 0x82ECCE50; continue 'dispatch;
	}
	// 82ECCE18: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECCE1C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ECCE20: 419A0030  beq cr6, 0x82ecce50
	if ctx.cr[6].eq {
	pc = 0x82ECCE50; continue 'dispatch;
	}
	// 82ECCE24: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82ECCE28: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 82ECCE2C: 7D490734  extsh r9, r10
	ctx.r[9].s64 = ctx.r[10].s16 as i64;
	// 82ECCE30: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82ECCE34: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82ECCE38: 409A0018  bne cr6, 0x82ecce50
	if !ctx.cr[6].eq {
	pc = 0x82ECCE50; continue 'dispatch;
	}
	// 82ECCE3C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECCE40: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82ECCE44: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECCE48: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ECCE4C: 4E800421  bctrl
	ctx.lr = 0x82ECCE50;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECCE50: 93FE0050  stw r31, 0x50(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82ECCE54: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82ECCE58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ECCE5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ECCE60: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82ECCE64: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ECCE68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECCE70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ECCE70 size=92
    let mut pc: u32 = 0x82ECCE70;
    'dispatch: loop {
        match pc {
            0x82ECCE70 => {
    //   block [0x82ECCE70..0x82ECCECC)
	// 82ECCE70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ECCE74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ECCE78: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ECCE7C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ECCE80: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ECCE84: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82ECCE88: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECCE8C: 814B004C  lwz r10, 0x4c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 82ECCE90: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ECCE94: 4E800421  bctrl
	ctx.lr = 0x82ECCE98;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECCE98: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82ECCE9C: 817F006C  lwz r11, 0x6c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 82ECCEA0: 38AB0030  addi r5, r11, 0x30
	ctx.r[5].s64 = ctx.r[11].s64 + 48;
	// 82ECCEA4: 388B0020  addi r4, r11, 0x20
	ctx.r[4].s64 = ctx.r[11].s64 + 32;
	// 82ECCEA8: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECCEAC: 81090050  lwz r8, 0x50(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(80 as u32) ) } as u64;
	// 82ECCEB0: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82ECCEB4: 4E800421  bctrl
	ctx.lr = 0x82ECCEB8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECCEB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82ECCEBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ECCEC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ECCEC4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ECCEC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECCED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ECCED0 size=4
    let mut pc: u32 = 0x82ECCED0;
    'dispatch: loop {
        match pc {
            0x82ECCED0 => {
    //   block [0x82ECCED0..0x82ECCED4)
	// 82ECCED0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECCED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ECCED8 size=176
    let mut pc: u32 = 0x82ECCED8;
    'dispatch: loop {
        match pc {
            0x82ECCED8 => {
    //   block [0x82ECCED8..0x82ECCF88)
	// 82ECCED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ECCEDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ECCEE0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82ECCEE4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ECCEE8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ECCEEC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ECCEF0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82ECCEF4: 807F018C  lwz r3, 0x18c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(396 as u32) ) } as u64;
	// 82ECCEF8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ECCEFC: 419A0050  beq cr6, 0x82eccf4c
	if ctx.cr[6].eq {
	pc = 0x82ECCF4C; continue 'dispatch;
	}
	// 82ECCF00: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECCF04: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82ECCF08: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ECCF0C: 4E800421  bctrl
	ctx.lr = 0x82ECCF10;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECCF10: 807F018C  lwz r3, 0x18c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(396 as u32) ) } as u64;
	// 82ECCF14: A1230004  lhz r9, 4(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECCF18: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82ECCF1C: 419A0030  beq cr6, 0x82eccf4c
	if ctx.cr[6].eq {
	pc = 0x82ECCF4C; continue 'dispatch;
	}
	// 82ECCF20: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82ECCF24: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 82ECCF28: 7D490734  extsh r9, r10
	ctx.r[9].s64 = ctx.r[10].s16 as i64;
	// 82ECCF2C: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82ECCF30: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82ECCF34: 409A0018  bne cr6, 0x82eccf4c
	if !ctx.cr[6].eq {
	pc = 0x82ECCF4C; continue 'dispatch;
	}
	// 82ECCF38: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECCF3C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82ECCF40: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECCF44: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ECCF48: 4E800421  bctrl
	ctx.lr = 0x82ECCF4C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECCF4C: 93DF018C  stw r30, 0x18c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(396 as u32), ctx.r[30].u32 ) };
	// 82ECCF50: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82ECCF54: 419A001C  beq cr6, 0x82eccf70
	if ctx.cr[6].eq {
	pc = 0x82ECCF70; continue 'dispatch;
	}
	// 82ECCF58: A17E0004  lhz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECCF5C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ECCF60: 419A0010  beq cr6, 0x82eccf70
	if ctx.cr[6].eq {
	pc = 0x82ECCF70; continue 'dispatch;
	}
	// 82ECCF64: A17E0006  lhz r11, 6(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(6 as u32) ) } as u64;
	// 82ECCF68: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 82ECCF6C: B15E0006  sth r10, 6(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82ECCF70: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82ECCF74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ECCF78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ECCF7C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82ECCF80: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ECCF84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECCF88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ECCF88 size=8
    let mut pc: u32 = 0x82ECCF88;
    'dispatch: loop {
        match pc {
            0x82ECCF88 => {
    //   block [0x82ECCF88..0x82ECCF90)
	// 82ECCF88: 8063018C  lwz r3, 0x18c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(396 as u32) ) } as u64;
	// 82ECCF8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECCF90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ECCF90 size=84
    let mut pc: u32 = 0x82ECCF90;
    'dispatch: loop {
        match pc {
            0x82ECCF90 => {
    //   block [0x82ECCF90..0x82ECCFE4)
	// 82ECCF90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ECCF94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ECCF98: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ECCF9C: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82ECCFA0: 80C30070  lwz r6, 0x70(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(112 as u32) ) } as u64;
	// 82ECCFA4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82ECCFA8: 38E9BED4  addi r7, r9, -0x412c
	ctx.r[7].s64 = ctx.r[9].s64 + -16684;
	// 82ECCFAC: 91610090  stw r11, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[11].u32 ) };
	// 82ECCFB0: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 82ECCFB4: 90E10050  stw r7, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[7].u32 ) };
	// 82ECCFB8: 7CA82B78  mr r8, r5
	ctx.r[8].u64 = ctx.r[5].u64;
	// 82ECCFBC: 91610094  stw r11, 0x94(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), ctx.r[11].u32 ) };
	// 82ECCFC0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82ECCFC4: 80830054  lwz r4, 0x54(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(84 as u32) ) } as u64;
	// 82ECCFC8: 7D455378  mr r5, r10
	ctx.r[5].u64 = ctx.r[10].u64;
	// 82ECCFCC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82ECCFD0: 480536D9  bl 0x82f206a8
	ctx.lr = 0x82ECCFD4;
	sub_82F206A8(ctx, base);
	// 82ECCFD4: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82ECCFD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ECCFDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ECCFE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECCFE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ECCFE8 size=20
    let mut pc: u32 = 0x82ECCFE8;
    'dispatch: loop {
        match pc {
            0x82ECCFE8 => {
    //   block [0x82ECCFE8..0x82ECCFFC)
	// 82ECCFE8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82ECCFEC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82ECCFF0: 806B007C  lwz r3, 0x7c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(124 as u32) ) } as u64;
	// 82ECCFF4: 914B0080  stw r10, 0x80(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(128 as u32), ctx.r[10].u32 ) };
	// 82ECCFF8: 4801AE00  b 0x82ee7df8
	sub_82EE7DF8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECD000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ECD000 size=8
    let mut pc: u32 = 0x82ECD000;
    'dispatch: loop {
        match pc {
            0x82ECD000 => {
    //   block [0x82ECD000..0x82ECD008)
	// 82ECD000: 8063007C  lwz r3, 0x7c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(124 as u32) ) } as u64;
	// 82ECD004: 4801A804  b 0x82ee7808
	sub_82EE7808(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECD008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ECD008 size=16
    let mut pc: u32 = 0x82ECD008;
    'dispatch: loop {
        match pc {
            0x82ECD008 => {
    //   block [0x82ECD008..0x82ECD018)
	// 82ECD008: 806300AC  lwz r3, 0xac(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(172 as u32) ) } as u64;
	// 82ECD00C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82ECD010: F9630020  std r11, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82ECD014: 48375948  b 0x8324295c
	sub_8324295C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECD018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ECD018 size=4
    let mut pc: u32 = 0x82ECD018;
    'dispatch: loop {
        match pc {
            0x82ECD018 => {
    //   block [0x82ECD018..0x82ECD01C)
	// 82ECD018: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECD020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ECD020 size=4
    let mut pc: u32 = 0x82ECD020;
    'dispatch: loop {
        match pc {
            0x82ECD020 => {
    //   block [0x82ECD020..0x82ECD024)
	// 82ECD020: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECD028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ECD028 size=32
    let mut pc: u32 = 0x82ECD028;
    'dispatch: loop {
        match pc {
            0x82ECD028 => {
    //   block [0x82ECD028..0x82ECD048)
	// 82ECD028: 816300AC  lwz r11, 0xac(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(172 as u32) ) } as u64;
	// 82ECD02C: E94B0020  ld r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) };
	// 82ECD030: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82ECD034: 2F2AFFFF  cmpdi cr6, r10, -1
	ctx.cr[6].compare_i64(ctx.r[10].s64, -1, &mut ctx.xer);
	// 82ECD038: 409A0008  bne cr6, 0x82ecd040
	if !ctx.cr[6].eq {
	pc = 0x82ECD040; continue 'dispatch;
	}
	// 82ECD03C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82ECD040: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82ECD044: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECD048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ECD048 size=12
    let mut pc: u32 = 0x82ECD048;
    'dispatch: loop {
        match pc {
            0x82ECD048 => {
    //   block [0x82ECD048..0x82ECD054)
	// 82ECD048: 806300A8  lwz r3, 0xa8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(168 as u32) ) } as u64;
	// 82ECD04C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ECD050: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECD054(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ECD054 size=12
    let mut pc: u32 = 0x82ECD054;
    'dispatch: loop {
        match pc {
            0x82ECD054 => {
    //   block [0x82ECD054..0x82ECD060)
	// 82ECD054: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82ECD058: F9630020  std r11, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82ECD05C: 48375900  b 0x8324295c
	sub_8324295C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECD060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ECD060 size=4
    let mut pc: u32 = 0x82ECD060;
    'dispatch: loop {
        match pc {
            0x82ECD060 => {
    //   block [0x82ECD060..0x82ECD064)
	// 82ECD060: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECD068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ECD068 size=12
    let mut pc: u32 = 0x82ECD068;
    'dispatch: loop {
        match pc {
            0x82ECD068 => {
    //   block [0x82ECD068..0x82ECD074)
	// 82ECD068: 806300A8  lwz r3, 0xa8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(168 as u32) ) } as u64;
	// 82ECD06C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ECD070: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECD074(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ECD074 size=12
    let mut pc: u32 = 0x82ECD074;
    'dispatch: loop {
        match pc {
            0x82ECD074 => {
    //   block [0x82ECD074..0x82ECD080)
	// 82ECD074: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82ECD078: F9630020  std r11, 0x20(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 82ECD07C: 483758E0  b 0x8324295c
	sub_8324295C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECD080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ECD080 size=4
    let mut pc: u32 = 0x82ECD080;
    'dispatch: loop {
        match pc {
            0x82ECD080 => {
    //   block [0x82ECD080..0x82ECD084)
	// 82ECD080: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECD088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ECD088 size=412
    let mut pc: u32 = 0x82ECD088;
    'dispatch: loop {
        match pc {
            0x82ECD088 => {
    //   block [0x82ECD088..0x82ECD224)
	// 82ECD088: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ECD08C: 482DB0D5  bl 0x831a8160
	ctx.lr = 0x82ECD090;
	sub_831A8130(ctx, base);
	// 82ECD090: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ECD094: 83ED0000  lwz r31, 0(r13)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECD098: 3BC00014  li r30, 0x14
	ctx.r[30].s64 = 20;
	// 82ECD09C: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82ECD0A0: 38A00022  li r5, 0x22
	ctx.r[5].s64 = 34;
	// 82ECD0A4: 3880000C  li r4, 0xc
	ctx.r[4].s64 = 12;
	// 82ECD0A8: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82ECD0AC: 7C7EF82E  lwzx r3, r30, r31
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82ECD0B0: 4BFD3681  bl 0x82ea0730
	ctx.lr = 0x82ECD0B4;
	sub_82EA0730(ctx, base);
	// 82ECD0B4: 3B80000C  li r28, 0xc
	ctx.r[28].s64 = 12;
	// 82ECD0B8: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82ECD0BC: B3830004  sth r28, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[28].u16 ) };
	// 82ECD0C0: 4800FB29  bl 0x82edcbe8
	ctx.lr = 0x82ECD0C4;
	sub_82EDCBE8(ctx, base);
	// 82ECD0C4: 7D7EF82E  lwzx r11, r30, r31
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82ECD0C8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82ECD0CC: 38A00022  li r5, 0x22
	ctx.r[5].s64 = 34;
	// 82ECD0D0: 3880000C  li r4, 0xc
	ctx.r[4].s64 = 12;
	// 82ECD0D4: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82ECD0D8: 4BFD3659  bl 0x82ea0730
	ctx.lr = 0x82ECD0DC;
	sub_82EA0730(ctx, base);
	// 82ECD0DC: B3830004  sth r28, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[28].u16 ) };
	// 82ECD0E0: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82ECD0E4: 4801DDA5  bl 0x82eeae88
	ctx.lr = 0x82ECD0E8;
	sub_82EEAE88(ctx, base);
	// 82ECD0E8: 7D5EF82E  lwzx r10, r30, r31
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82ECD0EC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82ECD0F0: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82ECD0F4: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 82ECD0F8: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 82ECD0FC: 4BFD3635  bl 0x82ea0730
	ctx.lr = 0x82ECD100;
	sub_82EA0730(ctx, base);
	// 82ECD100: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ECD104: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 82ECD108: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 82ECD10C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82ECD110: 38E94248  addi r7, r9, 0x4248
	ctx.r[7].s64 = ctx.r[9].s64 + 16968;
	// 82ECD114: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 82ECD118: B17F0006  sth r11, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82ECD11C: 38A841B4  addi r5, r8, 0x41b4
	ctx.r[5].s64 = ctx.r[8].s64 + 16820;
	// 82ECD120: 90FF0000  stw r7, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82ECD124: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82ECD128: B0DF0004  sth r6, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[6].u16 ) };
	// 82ECD12C: 90BF0008  stw r5, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 82ECD130: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82ECD134: 909F0010  stw r4, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[4].u32 ) };
	// 82ECD138: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82ECD13C: B17F000E  sth r11, 0xe(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(14 as u32), ctx.r[11].u16 ) };
	// 82ECD140: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82ECD144: 480577CD  bl 0x82f24910
	ctx.lr = 0x82ECD148;
	sub_82F24910(ctx, base);
	// 82ECD148: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82ECD14C: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82ECD150: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82ECD154: 480577BD  bl 0x82f24910
	ctx.lr = 0x82ECD158;
	sub_82F24910(ctx, base);
	// 82ECD158: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 82ECD15C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82ECD160: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82ECD164: 480577AD  bl 0x82f24910
	ctx.lr = 0x82ECD168;
	sub_82F24910(ctx, base);
	// 82ECD168: A07D0004  lhz r3, 4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECD16C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ECD170: 419A0034  beq cr6, 0x82ecd1a4
	if ctx.cr[6].eq {
	pc = 0x82ECD1A4; continue 'dispatch;
	}
	// 82ECD174: A17D0006  lhz r11, 6(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(6 as u32) ) } as u64;
	// 82ECD178: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 82ECD17C: 7D490734  extsh r9, r10
	ctx.r[9].s64 = ctx.r[10].s16 as i64;
	// 82ECD180: B13D0006  sth r9, 6(r29)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[29].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82ECD184: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82ECD188: 409A001C  bne cr6, 0x82ecd1a4
	if !ctx.cr[6].eq {
	pc = 0x82ECD1A4; continue 'dispatch;
	}
	// 82ECD18C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECD190: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82ECD194: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82ECD198: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECD19C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ECD1A0: 4E800421  bctrl
	ctx.lr = 0x82ECD1A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECD1A4: A17E0004  lhz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECD1A8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ECD1AC: 419A0034  beq cr6, 0x82ecd1e0
	if ctx.cr[6].eq {
	pc = 0x82ECD1E0; continue 'dispatch;
	}
	// 82ECD1B0: A17E0006  lhz r11, 6(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(6 as u32) ) } as u64;
	// 82ECD1B4: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 82ECD1B8: 7D490734  extsh r9, r10
	ctx.r[9].s64 = ctx.r[10].s16 as i64;
	// 82ECD1BC: B13E0006  sth r9, 6(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82ECD1C0: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82ECD1C4: 409A001C  bne cr6, 0x82ecd1e0
	if !ctx.cr[6].eq {
	pc = 0x82ECD1E0; continue 'dispatch;
	}
	// 82ECD1C8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECD1CC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82ECD1D0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82ECD1D4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECD1D8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ECD1DC: 4E800421  bctrl
	ctx.lr = 0x82ECD1E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECD1E0: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECD1E4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ECD1E8: 419A0034  beq cr6, 0x82ecd21c
	if ctx.cr[6].eq {
	pc = 0x82ECD21C; continue 'dispatch;
	}
	// 82ECD1EC: A17F0006  lhz r11, 6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 82ECD1F0: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 82ECD1F4: 7D490734  extsh r9, r10
	ctx.r[9].s64 = ctx.r[10].s16 as i64;
	// 82ECD1F8: B13F0006  sth r9, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82ECD1FC: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82ECD200: 409A001C  bne cr6, 0x82ecd21c
	if !ctx.cr[6].eq {
	pc = 0x82ECD21C; continue 'dispatch;
	}
	// 82ECD204: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECD208: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82ECD20C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECD210: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECD214: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ECD218: 4E800421  bctrl
	ctx.lr = 0x82ECD21C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECD21C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82ECD220: 482DAF90  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECD228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ECD228 size=120
    let mut pc: u32 = 0x82ECD228;
    'dispatch: loop {
        match pc {
            0x82ECD228 => {
    //   block [0x82ECD228..0x82ECD2A0)
	// 82ECD228: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ECD22C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ECD230: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ECD234: 81630084  lwz r11, 0x84(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(132 as u32) ) } as u64;
	// 82ECD238: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 82ECD23C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ECD240: 419A0034  beq cr6, 0x82ecd274
	if ctx.cr[6].eq {
	pc = 0x82ECD274; continue 'dispatch;
	}
	// 82ECD244: 39200016  li r9, 0x16
	ctx.r[9].s64 = 22;
	// 82ECD248: 90810054  stw r4, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[4].u32 ) };
	// 82ECD24C: B0A10058  sth r5, 0x58(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[5].u16 ) };
	// 82ECD250: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82ECD254: 98E1005A  stb r7, 0x5a(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(90 as u32), ctx.r[7].u8 ) };
	// 82ECD258: 99210050  stb r9, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u8 ) };
	// 82ECD25C: 8063007C  lwz r3, 0x7c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(124 as u32) ) } as u64;
	// 82ECD260: 4801A5A9  bl 0x82ee7808
	ctx.lr = 0x82ECD264;
	sub_82EE7808(ctx, base);
	// 82ECD264: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82ECD268: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ECD26C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ECD270: 4E800020  blr
	return;
	// 82ECD274: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ECD278: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82ECD27C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82ECD280: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECD284: 812A0030  lwz r9, 0x30(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(48 as u32) ) } as u64;
	// 82ECD288: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82ECD28C: 4E800421  bctrl
	ctx.lr = 0x82ECD290;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECD290: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82ECD294: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ECD298: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ECD29C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECD2A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82ECD2A0 size=416
    let mut pc: u32 = 0x82ECD2A0;
    'dispatch: loop {
        match pc {
            0x82ECD2A0 => {
    //   block [0x82ECD2A0..0x82ECD440)
	// 82ECD2A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ECD2A4: 482DAEC5  bl 0x831a8168
	ctx.lr = 0x82ECD2A8;
	sub_831A8130(ctx, base);
	// 82ECD2A8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ECD2AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ECD2B0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82ECD2B4: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82ECD2B8: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82ECD2BC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ECD2C0: 419A002C  beq cr6, 0x82ecd2ec
	if ctx.cr[6].eq {
	pc = 0x82ECD2EC; continue 'dispatch;
	}
	// 82ECD2C4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82ECD2C8: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 82ECD2CC: 93810058  stw r28, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[28].u32 ) };
	// 82ECD2D0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82ECD2D4: 99610050  stb r11, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 82ECD2D8: 807F007C  lwz r3, 0x7c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 82ECD2DC: 4801A52D  bl 0x82ee7808
	ctx.lr = 0x82ECD2E0;
	sub_82EE7808(ctx, base);
	// 82ECD2E0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82ECD2E4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82ECD2E8: 482DAED0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 82ECD2EC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82ECD2F0: 80BE0010  lwz r5, 0x10(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82ECD2F4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82ECD2F8: 4800B751  bl 0x82ed8a48
	ctx.lr = 0x82ECD2FC;
	sub_82ED8A48(ctx, base);
	// 82ECD2FC: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82ECD300: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ECD304: 409A001C  bne cr6, 0x82ecd320
	if !ctx.cr[6].eq {
	pc = 0x82ECD320; continue 'dispatch;
	}
	// 82ECD308: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECD30C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82ECD310: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82ECD314: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ECD318: 4E800421  bctrl
	ctx.lr = 0x82ECD31C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECD31C: 907E0018  stw r3, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[3].u32 ) };
	// 82ECD320: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82ECD324: 38BE00E0  addi r5, r30, 0xe0
	ctx.r[5].s64 = ctx.r[30].s64 + 224;
	// 82ECD328: C04B08A4  lfs f2, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82ECD32C: FC201090  fmr f1, f2
	ctx.f[1].f64 = ctx.f[2].f64;
	// 82ECD330: 483311C9  bl 0x831fe4f8
	ctx.lr = 0x82ECD334;
	sub_831FE4F8(ctx, base);
	// 82ECD334: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82ECD338: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82ECD33C: B3BE00DA  sth r29, 0xda(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(218 as u32), ctx.r[29].u16 ) };
	// 82ECD340: B3BE00DC  sth r29, 0xdc(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(220 as u32), ctx.r[29].u16 ) };
	// 82ECD344: 817F00E0  lwz r11, 0xe0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(224 as u32) ) } as u64;
	// 82ECD348: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82ECD34C: 9BBF008D  stb r29, 0x8d(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(141 as u32), ctx.r[29].u8 ) };
	// 82ECD350: 917F00E0  stw r11, 0xe0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(224 as u32), ctx.r[11].u32 ) };
	// 82ECD354: 917E00C0  stw r11, 0xc0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(192 as u32), ctx.r[11].u32 ) };
	// 82ECD358: 48007D91  bl 0x82ed50e8
	ctx.lr = 0x82ECD35C;
	sub_82ED50E8(ctx, base);
	// 82ECD35C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82ECD360: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82ECD364: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECD368: 48011B69  bl 0x82edeed0
	ctx.lr = 0x82ECD36C;
	sub_82EDEED0(ctx, base);
	// 82ECD36C: 897F02C7  lbz r11, 0x2c7(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(711 as u32) ) } as u64;
	// 82ECD370: 895E00D9  lbz r10, 0xd9(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(217 as u32) ) } as u64;
	// 82ECD374: 556707BE  clrlwi r7, r11, 0x1e
	ctx.r[7].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 82ECD378: 891F02C6  lbz r8, 0x2c6(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(710 as u32) ) } as u64;
	// 82ECD37C: 554607BE  clrlwi r6, r10, 0x1e
	ctx.r[6].u64 = ctx.r[10].u32 as u64 & 0x00000003u64;
	// 82ECD380: 893F02C5  lbz r9, 0x2c5(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(709 as u32) ) } as u64;
	// 82ECD384: 7F073040  cmplw cr6, r7, r6
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[6].u32, &mut ctx.xer);
	// 82ECD388: 40980008  bge cr6, 0x82ecd390
	if !ctx.cr[6].lt {
	pc = 0x82ECD390; continue 'dispatch;
	}
	// 82ECD38C: 7D2948F8  nor r9, r9, r9
	ctx.r[9].u64 = !(ctx.r[9].u64 | ctx.r[9].u64);
	// 82ECD390: 55297022  slwi r9, r9, 0xe
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(14);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82ECD394: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82ECD398: 40980010  bge cr6, 0x82ecd3a8
	if !ctx.cr[6].lt {
	pc = 0x82ECD3A8; continue 'dispatch;
	}
	// 82ECD39C: 7D0B40F8  nor r11, r8, r8
	ctx.r[11].u64 = !(ctx.r[8].u64 | ctx.r[8].u64);
	// 82ECD3A0: 556B7022  slwi r11, r11, 0xe
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(14);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82ECD3A4: 48000008  b 0x82ecd3ac
	pc = 0x82ECD3AC; continue 'dispatch;
	// 82ECD3A8: 550B7022  slwi r11, r8, 0xe
	ctx.r[11].u32 = ctx.r[8].u32.wrapping_shl(14);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82ECD3AC: A15E00DA  lhz r10, 0xda(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(218 as u32) ) } as u64;
	// 82ECD3B0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82ECD3B4: A0FE00DC  lhz r7, 0xdc(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(220 as u32) ) } as u64;
	// 82ECD3B8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82ECD3BC: 554604BE  clrlwi r6, r10, 0x12
	ctx.r[6].u64 = ctx.r[10].u32 as u64 & 0x00003FFFu64;
	// 82ECD3C0: 54E504BE  clrlwi r5, r7, 0x12
	ctx.r[5].u64 = ctx.r[7].u32 as u64 & 0x00003FFFu64;
	// 82ECD3C4: 7CC34B78  or r3, r6, r9
	ctx.r[3].u64 = ctx.r[6].u64 | ctx.r[9].u64;
	// 82ECD3C8: 7CAB5B78  or r11, r5, r11
	ctx.r[11].u64 = ctx.r[5].u64 | ctx.r[11].u64;
	// 82ECD3CC: B07E00DA  sth r3, 0xda(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(218 as u32), ctx.r[3].u16 ) };
	// 82ECD3D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECD3D4: B17E00DC  sth r11, 0xdc(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(220 as u32), ctx.r[11].u16 ) };
	// 82ECD3D8: 991F008D  stb r8, 0x8d(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(141 as u32), ctx.r[8].u8 ) };
	// 82ECD3DC: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82ECD3E0: 38EB0001  addi r7, r11, 1
	ctx.r[7].s64 = ctx.r[11].s64 + 1;
	// 82ECD3E4: 90FF0084  stw r7, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[7].u32 ) };
	// 82ECD3E8: 48013E71  bl 0x82ee1258
	ctx.lr = 0x82ECD3EC;
	sub_82EE1258(ctx, base);
	// 82ECD3EC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82ECD3F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECD3F4: 4801668D  bl 0x82ee3a80
	ctx.lr = 0x82ECD3F8;
	sub_82EE3A80(ctx, base);
	// 82ECD3F8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82ECD3FC: 48015B75  bl 0x82ee2f70
	ctx.lr = 0x82ECD400;
	sub_82EE2F70(ctx, base);
	// 82ECD400: 80DF0084  lwz r6, 0x84(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82ECD404: 3566FFFF  addic. r11, r6, -1
	ctx.xer.ca = (ctx.r[6].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[6].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ECD408: 917F0084  stw r11, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 82ECD40C: 40820028  bne 0x82ecd434
	if !ctx.cr[0].eq {
	pc = 0x82ECD434; continue 'dispatch;
	}
	// 82ECD410: 897F008C  lbz r11, 0x8c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) } as u64;
	// 82ECD414: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ECD418: 409A001C  bne cr6, 0x82ecd434
	if !ctx.cr[6].eq {
	pc = 0x82ECD434; continue 'dispatch;
	}
	// 82ECD41C: 817F0080  lwz r11, 0x80(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 82ECD420: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ECD424: 419A0010  beq cr6, 0x82ecd434
	if ctx.cr[6].eq {
	pc = 0x82ECD434; continue 'dispatch;
	}
	// 82ECD428: 93BF0080  stw r29, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[29].u32 ) };
	// 82ECD42C: 807F007C  lwz r3, 0x7c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 82ECD430: 4801A9C9  bl 0x82ee7df8
	ctx.lr = 0x82ECD434;
	sub_82EE7DF8(ctx, base);
	// 82ECD434: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82ECD438: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82ECD43C: 482DAD7C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECD440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ECD440 size=300
    let mut pc: u32 = 0x82ECD440;
    'dispatch: loop {
        match pc {
            0x82ECD440 => {
    //   block [0x82ECD440..0x82ECD56C)
	// 82ECD440: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ECD444: 482DAD21  bl 0x831a8164
	ctx.lr = 0x82ECD448;
	sub_831A8130(ctx, base);
	// 82ECD448: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ECD44C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82ECD450: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82ECD454: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82ECD458: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82ECD45C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ECD460: 419A0030  beq cr6, 0x82ecd490
	if ctx.cr[6].eq {
	pc = 0x82ECD490; continue 'dispatch;
	}
	// 82ECD464: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 82ECD468: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 82ECD46C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82ECD470: 807F007C  lwz r3, 0x7c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 82ECD474: 99610050  stb r11, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 82ECD478: 4801A391  bl 0x82ee7808
	ctx.lr = 0x82ECD47C;
	sub_82EE7808(ctx, base);
	// 82ECD47C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82ECD480: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82ECD484: 995B0000  stb r10, 0(r27)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82ECD488: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82ECD48C: 482DAD28  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 82ECD490: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 82ECD494: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82ECD498: 939F0084  stw r28, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[28].u32 ) };
	// 82ECD49C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECD4A0: 48014061  bl 0x82ee1500
	ctx.lr = 0x82ECD4A4;
	sub_82EE1500(ctx, base);
	// 82ECD4A4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82ECD4A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECD4AC: 480166AD  bl 0x82ee3b58
	ctx.lr = 0x82ECD4B0;
	sub_82EE3B58(ctx, base);
	// 82ECD4B0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82ECD4B4: 48015B95  bl 0x82ee3048
	ctx.lr = 0x82ECD4B8;
	sub_82EE3048(ctx, base);
	// 82ECD4B8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82ECD4BC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82ECD4C0: 9BBF008D  stb r29, 0x8d(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(141 as u32), ctx.r[29].u8 ) };
	// 82ECD4C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECD4C8: 48012BC1  bl 0x82ee0088
	ctx.lr = 0x82ECD4CC;
	sub_82EE0088(ctx, base);
	// 82ECD4CC: 809E0054  lwz r4, 0x54(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(84 as u32) ) } as u64;
	// 82ECD4D0: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82ECD4D4: 419A0028  beq cr6, 0x82ecd4fc
	if ctx.cr[6].eq {
	pc = 0x82ECD4FC; continue 'dispatch;
	}
	// 82ECD4D8: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECD4DC: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82ECD4E0: A13E0050  lhz r9, 0x50(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(80 as u32) ) } as u64;
	// 82ECD4E4: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 82ECD4E8: 5525283E  rotlwi r5, r9, 5
	ctx.r[5].u64 = ((ctx.r[9].u32).rotate_left(5)) as u64;
	// 82ECD4EC: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82ECD4F0: 4BFD32C1  bl 0x82ea07b0
	ctx.lr = 0x82ECD4F4;
	sub_82EA07B0(ctx, base);
	// 82ECD4F4: 93BE0054  stw r29, 0x54(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 82ECD4F8: B3BE0050  sth r29, 0x50(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(80 as u32), ctx.r[29].u16 ) };
	// 82ECD4FC: A17E0004  lhz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECD500: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ECD504: 409A0018  bne cr6, 0x82ecd51c
	if !ctx.cr[6].eq {
	pc = 0x82ECD51C; continue 'dispatch;
	}
	// 82ECD508: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECD50C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82ECD510: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82ECD514: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ECD518: 4E800421  bctrl
	ctx.lr = 0x82ECD51C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECD51C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82ECD520: 48007BE9  bl 0x82ed5108
	ctx.lr = 0x82ECD524;
	sub_82ED5108(ctx, base);
	// 82ECD524: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82ECD528: 9B9F008D  stb r28, 0x8d(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(141 as u32), ctx.r[28].u8 ) };
	// 82ECD52C: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ECD530: 917F0084  stw r11, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 82ECD534: 40820028  bne 0x82ecd55c
	if !ctx.cr[0].eq {
	pc = 0x82ECD55C; continue 'dispatch;
	}
	// 82ECD538: 897F008C  lbz r11, 0x8c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) } as u64;
	// 82ECD53C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ECD540: 409A001C  bne cr6, 0x82ecd55c
	if !ctx.cr[6].eq {
	pc = 0x82ECD55C; continue 'dispatch;
	}
	// 82ECD544: 817F0080  lwz r11, 0x80(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 82ECD548: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ECD54C: 419A0010  beq cr6, 0x82ecd55c
	if ctx.cr[6].eq {
	pc = 0x82ECD55C; continue 'dispatch;
	}
	// 82ECD550: 93BF0080  stw r29, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[29].u32 ) };
	// 82ECD554: 807F007C  lwz r3, 0x7c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 82ECD558: 4801A8A1  bl 0x82ee7df8
	ctx.lr = 0x82ECD55C;
	sub_82EE7DF8(ctx, base);
	// 82ECD55C: 9B9B0000  stb r28, 0(r27)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[28].u8 ) };
	// 82ECD560: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82ECD564: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82ECD568: 482DAC4C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECD570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ECD570 size=300
    let mut pc: u32 = 0x82ECD570;
    'dispatch: loop {
        match pc {
            0x82ECD570 => {
    //   block [0x82ECD570..0x82ECD69C)
	// 82ECD570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ECD574: 482DABF9  bl 0x831a816c
	ctx.lr = 0x82ECD578;
	sub_831A8130(ctx, base);
	// 82ECD578: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ECD57C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ECD580: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82ECD584: 815F0088  lwz r10, 0x88(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(136 as u32) ) } as u64;
	// 82ECD588: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82ECD58C: 7D4A5A15  add. r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ECD590: 41820024  beq 0x82ecd5b4
	if ctx.cr[0].eq {
	pc = 0x82ECD5B4; continue 'dispatch;
	}
	// 82ECD594: 3960000E  li r11, 0xe
	ctx.r[11].s64 = 14;
	// 82ECD598: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 82ECD59C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82ECD5A0: 807F007C  lwz r3, 0x7c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 82ECD5A4: 99610050  stb r11, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 82ECD5A8: 4801A261  bl 0x82ee7808
	ctx.lr = 0x82ECD5AC;
	sub_82EE7808(ctx, base);
	// 82ECD5AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82ECD5B0: 482DAC0C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 82ECD5B4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82ECD5B8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82ECD5BC: 917F0084  stw r11, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 82ECD5C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECD5C4: 4801681D  bl 0x82ee3de0
	ctx.lr = 0x82ECD5C8;
	sub_82EE3DE0(ctx, base);
	// 82ECD5C8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82ECD5CC: 48008255  bl 0x82ed5820
	ctx.lr = 0x82ECD5D0;
	sub_82ED5820(ctx, base);
	// 82ECD5D0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82ECD5D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECD5D8: 48014071  bl 0x82ee1648
	ctx.lr = 0x82ECD5DC;
	sub_82EE1648(ctx, base);
	// 82ECD5DC: 813F00E8  lwz r9, 0xe8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(232 as u32) ) } as u64;
	// 82ECD5E0: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82ECD5E4: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82ECD5E8: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82ECD5EC: 40990024  ble cr6, 0x82ecd610
	if !ctx.cr[6].gt {
	pc = 0x82ECD610; continue 'dispatch;
	}
	// 82ECD5F0: 815F00E4  lwz r10, 0xe4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 82ECD5F4: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECD5F8: 7F08F040  cmplw cr6, r8, r30
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82ECD5FC: 419A0018  beq cr6, 0x82ecd614
	if ctx.cr[6].eq {
	pc = 0x82ECD614; continue 'dispatch;
	}
	// 82ECD600: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82ECD604: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82ECD608: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82ECD60C: 4198FFE8  blt cr6, 0x82ecd5f4
	if ctx.cr[6].lt {
	pc = 0x82ECD5F4; continue 'dispatch;
	}
	// 82ECD610: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82ECD614: 815F00E8  lwz r10, 0xe8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(232 as u32) ) } as u64;
	// 82ECD618: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82ECD61C: 811F00E4  lwz r8, 0xe4(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 82ECD620: 396AFFFF  addi r11, r10, -1
	ctx.r[11].s64 = ctx.r[10].s64 + -1;
	// 82ECD624: 5567103A  slwi r7, r11, 2
	ctx.r[7].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82ECD628: 917F00E8  stw r11, 0xe8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(232 as u32), ctx.r[11].u32 ) };
	// 82ECD62C: 7CC7402E  lwzx r6, r7, r8
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82ECD630: 7CC9412E  stwx r6, r9, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[8].u32), ctx.r[6].u32) };
	// 82ECD634: A0BE0004  lhz r5, 4(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECD638: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82ECD63C: 93BE0008  stw r29, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82ECD640: 409A0018  bne cr6, 0x82ecd658
	if !ctx.cr[6].eq {
	pc = 0x82ECD658; continue 'dispatch;
	}
	// 82ECD644: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECD648: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82ECD64C: 814B0030  lwz r10, 0x30(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82ECD650: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ECD654: 4E800421  bctrl
	ctx.lr = 0x82ECD658;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECD658: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82ECD65C: 48007AAD  bl 0x82ed5108
	ctx.lr = 0x82ECD660;
	sub_82ED5108(ctx, base);
	// 82ECD660: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82ECD664: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ECD668: 917F0084  stw r11, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 82ECD66C: 40820028  bne 0x82ecd694
	if !ctx.cr[0].eq {
	pc = 0x82ECD694; continue 'dispatch;
	}
	// 82ECD670: 897F008C  lbz r11, 0x8c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) } as u64;
	// 82ECD674: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ECD678: 409A001C  bne cr6, 0x82ecd694
	if !ctx.cr[6].eq {
	pc = 0x82ECD694; continue 'dispatch;
	}
	// 82ECD67C: 817F0080  lwz r11, 0x80(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 82ECD680: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ECD684: 419A0010  beq cr6, 0x82ecd694
	if ctx.cr[6].eq {
	pc = 0x82ECD694; continue 'dispatch;
	}
	// 82ECD688: 93BF0080  stw r29, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[29].u32 ) };
	// 82ECD68C: 807F007C  lwz r3, 0x7c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 82ECD690: 4801A769  bl 0x82ee7df8
	ctx.lr = 0x82ECD694;
	sub_82EE7DF8(ctx, base);
	// 82ECD694: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82ECD698: 482DAB24  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECD6A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ECD6A0 size=72
    let mut pc: u32 = 0x82ECD6A0;
    'dispatch: loop {
        match pc {
            0x82ECD6A0 => {
    //   block [0x82ECD6A0..0x82ECD6E8)
	// 82ECD6A0: 812300F4  lwz r9, 0xf4(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(244 as u32) ) } as u64;
	// 82ECD6A4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82ECD6A8: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82ECD6AC: 40990024  ble cr6, 0x82ecd6d0
	if !ctx.cr[6].gt {
	pc = 0x82ECD6D0; continue 'dispatch;
	}
	// 82ECD6B0: 814300F0  lwz r10, 0xf0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(240 as u32) ) } as u64;
	// 82ECD6B4: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECD6B8: 7F082040  cmplw cr6, r8, r4
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82ECD6BC: 419A0018  beq cr6, 0x82ecd6d4
	if ctx.cr[6].eq {
	pc = 0x82ECD6D4; continue 'dispatch;
	}
	// 82ECD6C0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82ECD6C4: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82ECD6C8: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82ECD6CC: 4198FFE8  blt cr6, 0x82ecd6b4
	if ctx.cr[6].lt {
	pc = 0x82ECD6B4; continue 'dispatch;
	}
	// 82ECD6D0: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82ECD6D4: 814300F0  lwz r10, 0xf0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(240 as u32) ) } as u64;
	// 82ECD6D8: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82ECD6DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82ECD6E0: 7D09512E  stwx r8, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[8].u32) };
	// 82ECD6E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECD6E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ECD6E8 size=72
    let mut pc: u32 = 0x82ECD6E8;
    'dispatch: loop {
        match pc {
            0x82ECD6E8 => {
    //   block [0x82ECD6E8..0x82ECD730)
	// 82ECD6E8: 81230118  lwz r9, 0x118(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(280 as u32) ) } as u64;
	// 82ECD6EC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82ECD6F0: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82ECD6F4: 40990024  ble cr6, 0x82ecd718
	if !ctx.cr[6].gt {
	pc = 0x82ECD718; continue 'dispatch;
	}
	// 82ECD6F8: 81430114  lwz r10, 0x114(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(276 as u32) ) } as u64;
	// 82ECD6FC: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECD700: 7F082040  cmplw cr6, r8, r4
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82ECD704: 419A0018  beq cr6, 0x82ecd71c
	if ctx.cr[6].eq {
	pc = 0x82ECD71C; continue 'dispatch;
	}
	// 82ECD708: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82ECD70C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82ECD710: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82ECD714: 4198FFE8  blt cr6, 0x82ecd6fc
	if ctx.cr[6].lt {
	pc = 0x82ECD6FC; continue 'dispatch;
	}
	// 82ECD718: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82ECD71C: 81430114  lwz r10, 0x114(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(276 as u32) ) } as u64;
	// 82ECD720: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82ECD724: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82ECD728: 7D09512E  stwx r8, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[8].u32) };
	// 82ECD72C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECD730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ECD730 size=72
    let mut pc: u32 = 0x82ECD730;
    'dispatch: loop {
        match pc {
            0x82ECD730 => {
    //   block [0x82ECD730..0x82ECD778)
	// 82ECD730: 81230100  lwz r9, 0x100(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(256 as u32) ) } as u64;
	// 82ECD734: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82ECD738: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82ECD73C: 40990024  ble cr6, 0x82ecd760
	if !ctx.cr[6].gt {
	pc = 0x82ECD760; continue 'dispatch;
	}
	// 82ECD740: 814300FC  lwz r10, 0xfc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(252 as u32) ) } as u64;
	// 82ECD744: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECD748: 7F082040  cmplw cr6, r8, r4
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82ECD74C: 419A0018  beq cr6, 0x82ecd764
	if ctx.cr[6].eq {
	pc = 0x82ECD764; continue 'dispatch;
	}
	// 82ECD750: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82ECD754: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82ECD758: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82ECD75C: 4198FFE8  blt cr6, 0x82ecd744
	if ctx.cr[6].lt {
	pc = 0x82ECD744; continue 'dispatch;
	}
	// 82ECD760: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82ECD764: 814300FC  lwz r10, 0xfc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(252 as u32) ) } as u64;
	// 82ECD768: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82ECD76C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82ECD770: 7D09512E  stwx r8, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[8].u32) };
	// 82ECD774: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECD778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ECD778 size=72
    let mut pc: u32 = 0x82ECD778;
    'dispatch: loop {
        match pc {
            0x82ECD778 => {
    //   block [0x82ECD778..0x82ECD7C0)
	// 82ECD778: 8123010C  lwz r9, 0x10c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(268 as u32) ) } as u64;
	// 82ECD77C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82ECD780: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82ECD784: 40990024  ble cr6, 0x82ecd7a8
	if !ctx.cr[6].gt {
	pc = 0x82ECD7A8; continue 'dispatch;
	}
	// 82ECD788: 81430108  lwz r10, 0x108(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(264 as u32) ) } as u64;
	// 82ECD78C: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECD790: 7F082040  cmplw cr6, r8, r4
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82ECD794: 419A0018  beq cr6, 0x82ecd7ac
	if ctx.cr[6].eq {
	pc = 0x82ECD7AC; continue 'dispatch;
	}
	// 82ECD798: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82ECD79C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82ECD7A0: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82ECD7A4: 4198FFE8  blt cr6, 0x82ecd78c
	if ctx.cr[6].lt {
	pc = 0x82ECD78C; continue 'dispatch;
	}
	// 82ECD7A8: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82ECD7AC: 81430108  lwz r10, 0x108(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(264 as u32) ) } as u64;
	// 82ECD7B0: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82ECD7B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82ECD7B8: 7D09512E  stwx r8, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[8].u32) };
	// 82ECD7BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECD7C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ECD7C0 size=72
    let mut pc: u32 = 0x82ECD7C0;
    'dispatch: loop {
        match pc {
            0x82ECD7C0 => {
    //   block [0x82ECD7C0..0x82ECD808)
	// 82ECD7C0: 81230130  lwz r9, 0x130(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(304 as u32) ) } as u64;
	// 82ECD7C4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82ECD7C8: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82ECD7CC: 40990024  ble cr6, 0x82ecd7f0
	if !ctx.cr[6].gt {
	pc = 0x82ECD7F0; continue 'dispatch;
	}
	// 82ECD7D0: 8143012C  lwz r10, 0x12c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(300 as u32) ) } as u64;
	// 82ECD7D4: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECD7D8: 7F082040  cmplw cr6, r8, r4
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82ECD7DC: 419A0018  beq cr6, 0x82ecd7f4
	if ctx.cr[6].eq {
	pc = 0x82ECD7F4; continue 'dispatch;
	}
	// 82ECD7E0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82ECD7E4: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82ECD7E8: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82ECD7EC: 4198FFE8  blt cr6, 0x82ecd7d4
	if ctx.cr[6].lt {
	pc = 0x82ECD7D4; continue 'dispatch;
	}
	// 82ECD7F0: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82ECD7F4: 8143012C  lwz r10, 0x12c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(300 as u32) ) } as u64;
	// 82ECD7F8: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82ECD7FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82ECD800: 7D09512E  stwx r8, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[8].u32) };
	// 82ECD804: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECD808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ECD808 size=72
    let mut pc: u32 = 0x82ECD808;
    'dispatch: loop {
        match pc {
            0x82ECD808 => {
    //   block [0x82ECD808..0x82ECD850)
	// 82ECD808: 81230154  lwz r9, 0x154(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(340 as u32) ) } as u64;
	// 82ECD80C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82ECD810: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82ECD814: 40990024  ble cr6, 0x82ecd838
	if !ctx.cr[6].gt {
	pc = 0x82ECD838; continue 'dispatch;
	}
	// 82ECD818: 81430150  lwz r10, 0x150(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(336 as u32) ) } as u64;
	// 82ECD81C: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECD820: 7F082040  cmplw cr6, r8, r4
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82ECD824: 419A0018  beq cr6, 0x82ecd83c
	if ctx.cr[6].eq {
	pc = 0x82ECD83C; continue 'dispatch;
	}
	// 82ECD828: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82ECD82C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82ECD830: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82ECD834: 4198FFE8  blt cr6, 0x82ecd81c
	if ctx.cr[6].lt {
	pc = 0x82ECD81C; continue 'dispatch;
	}
	// 82ECD838: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82ECD83C: 81430150  lwz r10, 0x150(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(336 as u32) ) } as u64;
	// 82ECD840: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82ECD844: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82ECD848: 7D09512E  stwx r8, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[8].u32) };
	// 82ECD84C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECD850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ECD850 size=72
    let mut pc: u32 = 0x82ECD850;
    'dispatch: loop {
        match pc {
            0x82ECD850 => {
    //   block [0x82ECD850..0x82ECD898)
	// 82ECD850: 8123013C  lwz r9, 0x13c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(316 as u32) ) } as u64;
	// 82ECD854: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82ECD858: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82ECD85C: 40990024  ble cr6, 0x82ecd880
	if !ctx.cr[6].gt {
	pc = 0x82ECD880; continue 'dispatch;
	}
	// 82ECD860: 81430138  lwz r10, 0x138(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(312 as u32) ) } as u64;
	// 82ECD864: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECD868: 7F082040  cmplw cr6, r8, r4
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82ECD86C: 419A0018  beq cr6, 0x82ecd884
	if ctx.cr[6].eq {
	pc = 0x82ECD884; continue 'dispatch;
	}
	// 82ECD870: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82ECD874: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82ECD878: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82ECD87C: 4198FFE8  blt cr6, 0x82ecd864
	if ctx.cr[6].lt {
	pc = 0x82ECD864; continue 'dispatch;
	}
	// 82ECD880: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82ECD884: 81430138  lwz r10, 0x138(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(312 as u32) ) } as u64;
	// 82ECD888: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82ECD88C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82ECD890: 7D09512E  stwx r8, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[8].u32) };
	// 82ECD894: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECD898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ECD898 size=72
    let mut pc: u32 = 0x82ECD898;
    'dispatch: loop {
        match pc {
            0x82ECD898 => {
    //   block [0x82ECD898..0x82ECD8E0)
	// 82ECD898: 81230148  lwz r9, 0x148(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(328 as u32) ) } as u64;
	// 82ECD89C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82ECD8A0: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82ECD8A4: 40990024  ble cr6, 0x82ecd8c8
	if !ctx.cr[6].gt {
	pc = 0x82ECD8C8; continue 'dispatch;
	}
	// 82ECD8A8: 81430144  lwz r10, 0x144(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(324 as u32) ) } as u64;
	// 82ECD8AC: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECD8B0: 7F082040  cmplw cr6, r8, r4
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82ECD8B4: 419A0018  beq cr6, 0x82ecd8cc
	if ctx.cr[6].eq {
	pc = 0x82ECD8CC; continue 'dispatch;
	}
	// 82ECD8B8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82ECD8BC: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82ECD8C0: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82ECD8C4: 4198FFE8  blt cr6, 0x82ecd8ac
	if ctx.cr[6].lt {
	pc = 0x82ECD8AC; continue 'dispatch;
	}
	// 82ECD8C8: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82ECD8CC: 81430144  lwz r10, 0x144(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(324 as u32) ) } as u64;
	// 82ECD8D0: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82ECD8D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82ECD8D8: 7D09512E  stwx r8, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[8].u32) };
	// 82ECD8DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECD8E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ECD8E0 size=72
    let mut pc: u32 = 0x82ECD8E0;
    'dispatch: loop {
        match pc {
            0x82ECD8E0 => {
    //   block [0x82ECD8E0..0x82ECD928)
	// 82ECD8E0: 8123016C  lwz r9, 0x16c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(364 as u32) ) } as u64;
	// 82ECD8E4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82ECD8E8: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82ECD8EC: 40990024  ble cr6, 0x82ecd910
	if !ctx.cr[6].gt {
	pc = 0x82ECD910; continue 'dispatch;
	}
	// 82ECD8F0: 81430168  lwz r10, 0x168(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(360 as u32) ) } as u64;
	// 82ECD8F4: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECD8F8: 7F082040  cmplw cr6, r8, r4
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82ECD8FC: 419A0018  beq cr6, 0x82ecd914
	if ctx.cr[6].eq {
	pc = 0x82ECD914; continue 'dispatch;
	}
	// 82ECD900: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82ECD904: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82ECD908: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82ECD90C: 4198FFE8  blt cr6, 0x82ecd8f4
	if ctx.cr[6].lt {
	pc = 0x82ECD8F4; continue 'dispatch;
	}
	// 82ECD910: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82ECD914: 81430168  lwz r10, 0x168(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(360 as u32) ) } as u64;
	// 82ECD918: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82ECD91C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82ECD920: 7D09512E  stwx r8, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[8].u32) };
	// 82ECD924: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECD928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ECD928 size=72
    let mut pc: u32 = 0x82ECD928;
    'dispatch: loop {
        match pc {
            0x82ECD928 => {
    //   block [0x82ECD928..0x82ECD970)
	// 82ECD928: 81230160  lwz r9, 0x160(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(352 as u32) ) } as u64;
	// 82ECD92C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82ECD930: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82ECD934: 40990024  ble cr6, 0x82ecd958
	if !ctx.cr[6].gt {
	pc = 0x82ECD958; continue 'dispatch;
	}
	// 82ECD938: 8143015C  lwz r10, 0x15c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(348 as u32) ) } as u64;
	// 82ECD93C: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECD940: 7F082040  cmplw cr6, r8, r4
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82ECD944: 419A0018  beq cr6, 0x82ecd95c
	if ctx.cr[6].eq {
	pc = 0x82ECD95C; continue 'dispatch;
	}
	// 82ECD948: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82ECD94C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82ECD950: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82ECD954: 4198FFE8  blt cr6, 0x82ecd93c
	if ctx.cr[6].lt {
	pc = 0x82ECD93C; continue 'dispatch;
	}
	// 82ECD958: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82ECD95C: 8143015C  lwz r10, 0x15c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(348 as u32) ) } as u64;
	// 82ECD960: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82ECD964: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82ECD968: 7D09512E  stwx r8, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[8].u32) };
	// 82ECD96C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECD970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ECD970 size=72
    let mut pc: u32 = 0x82ECD970;
    'dispatch: loop {
        match pc {
            0x82ECD970 => {
    //   block [0x82ECD970..0x82ECD9B8)
	// 82ECD970: 81230178  lwz r9, 0x178(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(376 as u32) ) } as u64;
	// 82ECD974: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82ECD978: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82ECD97C: 40990024  ble cr6, 0x82ecd9a0
	if !ctx.cr[6].gt {
	pc = 0x82ECD9A0; continue 'dispatch;
	}
	// 82ECD980: 81430174  lwz r10, 0x174(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(372 as u32) ) } as u64;
	// 82ECD984: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECD988: 7F082040  cmplw cr6, r8, r4
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82ECD98C: 419A0018  beq cr6, 0x82ecd9a4
	if ctx.cr[6].eq {
	pc = 0x82ECD9A4; continue 'dispatch;
	}
	// 82ECD990: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82ECD994: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82ECD998: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82ECD99C: 4198FFE8  blt cr6, 0x82ecd984
	if ctx.cr[6].lt {
	pc = 0x82ECD984; continue 'dispatch;
	}
	// 82ECD9A0: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82ECD9A4: 81430174  lwz r10, 0x174(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(372 as u32) ) } as u64;
	// 82ECD9A8: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82ECD9AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82ECD9B0: 7D09512E  stwx r8, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[8].u32) };
	// 82ECD9B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECD9B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ECD9B8 size=72
    let mut pc: u32 = 0x82ECD9B8;
    'dispatch: loop {
        match pc {
            0x82ECD9B8 => {
    //   block [0x82ECD9B8..0x82ECDA00)
	// 82ECD9B8: 81230124  lwz r9, 0x124(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(292 as u32) ) } as u64;
	// 82ECD9BC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82ECD9C0: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82ECD9C4: 40990024  ble cr6, 0x82ecd9e8
	if !ctx.cr[6].gt {
	pc = 0x82ECD9E8; continue 'dispatch;
	}
	// 82ECD9C8: 81430120  lwz r10, 0x120(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(288 as u32) ) } as u64;
	// 82ECD9CC: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECD9D0: 7F082040  cmplw cr6, r8, r4
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82ECD9D4: 419A0018  beq cr6, 0x82ecd9ec
	if ctx.cr[6].eq {
	pc = 0x82ECD9EC; continue 'dispatch;
	}
	// 82ECD9D8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82ECD9DC: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82ECD9E0: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82ECD9E4: 4198FFE8  blt cr6, 0x82ecd9cc
	if ctx.cr[6].lt {
	pc = 0x82ECD9CC; continue 'dispatch;
	}
	// 82ECD9E8: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82ECD9EC: 81430120  lwz r10, 0x120(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(288 as u32) ) } as u64;
	// 82ECD9F0: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82ECD9F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82ECD9F8: 7D09512E  stwx r8, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[8].u32) };
	// 82ECD9FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECDA00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ECDA00 size=72
    let mut pc: u32 = 0x82ECDA00;
    'dispatch: loop {
        match pc {
            0x82ECDA00 => {
    //   block [0x82ECDA00..0x82ECDA48)
	// 82ECDA00: 81230184  lwz r9, 0x184(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(388 as u32) ) } as u64;
	// 82ECDA04: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82ECDA08: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82ECDA0C: 40990024  ble cr6, 0x82ecda30
	if !ctx.cr[6].gt {
	pc = 0x82ECDA30; continue 'dispatch;
	}
	// 82ECDA10: 81430180  lwz r10, 0x180(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(384 as u32) ) } as u64;
	// 82ECDA14: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECDA18: 7F082040  cmplw cr6, r8, r4
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82ECDA1C: 419A0018  beq cr6, 0x82ecda34
	if ctx.cr[6].eq {
	pc = 0x82ECDA34; continue 'dispatch;
	}
	// 82ECDA20: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82ECDA24: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82ECDA28: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82ECDA2C: 4198FFE8  blt cr6, 0x82ecda14
	if ctx.cr[6].lt {
	pc = 0x82ECDA14; continue 'dispatch;
	}
	// 82ECDA30: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82ECDA34: 81430180  lwz r10, 0x180(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(384 as u32) ) } as u64;
	// 82ECDA38: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82ECDA3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82ECDA40: 7D09512E  stwx r8, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[8].u32) };
	// 82ECDA44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECDA48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ECDA48 size=244
    let mut pc: u32 = 0x82ECDA48;
    'dispatch: loop {
        match pc {
            0x82ECDA48 => {
    //   block [0x82ECDA48..0x82ECDB3C)
	// 82ECDA48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ECDA4C: 482DA719  bl 0x831a8164
	ctx.lr = 0x82ECDA50;
	sub_831A8130(ctx, base);
	// 82ECDA50: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ECDA54: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82ECDA58: 807C0008  lwz r3, 8(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ECDA5C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECDA60: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ECDA64: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ECDA68: 4E800421  bctrl
	ctx.lr = 0x82ECDA6C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECDA6C: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82ECDA70: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 82ECDA74: 409A00BC  bne cr6, 0x82ecdb30
	if !ctx.cr[6].eq {
	pc = 0x82ECDB30; continue 'dispatch;
	}
	// 82ECDA78: 83FC0050  lwz r31, 0x50(r28)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(80 as u32) ) } as u64;
	// 82ECDA7C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82ECDA80: 419A00B0  beq cr6, 0x82ecdb30
	if ctx.cr[6].eq {
	pc = 0x82ECDB30; continue 'dispatch;
	}
	// 82ECDA84: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82ECDA88: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ECDA8C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82ECDA90: 808B6A30  lwz r4, 0x6a30(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27184 as u32) ) } as u64;
	// 82ECDA94: 81440000  lwz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECDA98: 812A0060  lwz r9, 0x60(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(96 as u32) ) } as u64;
	// 82ECDA9C: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82ECDAA0: 4E800421  bctrl
	ctx.lr = 0x82ECDAA4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECDAA4: 89030000  lbz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECDAA8: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82ECDAAC: 409A0084  bne cr6, 0x82ecdb30
	if !ctx.cr[6].eq {
	pc = 0x82ECDB30; continue 'dispatch;
	}
	// 82ECDAB0: 83CD0000  lwz r30, 0(r13)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECDAB4: 3BA00018  li r29, 0x18
	ctx.r[29].s64 = 24;
	// 82ECDAB8: 7D5DF02E  lwzx r10, r29, r30
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82ECDABC: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ECDAC0: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECDAC4: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82ECDAC8: 40980020  bge cr6, 0x82ecdae8
	if !ctx.cr[6].lt {
	pc = 0x82ECDAE8; continue 'dispatch;
	}
	// 82ECDACC: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 82ECDAD0: 390942C0  addi r8, r9, 0x42c0
	ctx.r[8].s64 = ctx.r[9].s64 + 17088;
	// 82ECDAD4: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82ECDAD8: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82ECDADC: 38AB000C  addi r5, r11, 0xc
	ctx.r[5].s64 = ctx.r[11].s64 + 12;
	// 82ECDAE0: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82ECDAE4: 90AA0004  stw r5, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82ECDAE8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECDAEC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82ECDAF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECDAF4: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ECDAF8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ECDAFC: 4E800421  bctrl
	ctx.lr = 0x82ECDB00;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECDB00: 7D5DF02E  lwzx r10, r29, r30
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82ECDB04: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECDB08: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ECDB0C: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82ECDB10: 40980020  bge cr6, 0x82ecdb30
	if !ctx.cr[6].lt {
	pc = 0x82ECDB30; continue 'dispatch;
	}
	// 82ECDB14: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82ECDB18: 3909BADC  addi r8, r9, -0x4524
	ctx.r[8].s64 = ctx.r[9].s64 + -17700;
	// 82ECDB1C: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82ECDB20: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82ECDB24: 38AB000C  addi r5, r11, 0xc
	ctx.r[5].s64 = ctx.r[11].s64 + 12;
	// 82ECDB28: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82ECDB2C: 90AA0004  stw r5, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82ECDB30: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82ECDB34: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82ECDB38: 482DA67C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECDB40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ECDB40 size=244
    let mut pc: u32 = 0x82ECDB40;
    'dispatch: loop {
        match pc {
            0x82ECDB40 => {
    //   block [0x82ECDB40..0x82ECDC34)
	// 82ECDB40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ECDB44: 482DA621  bl 0x831a8164
	ctx.lr = 0x82ECDB48;
	sub_831A8130(ctx, base);
	// 82ECDB48: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ECDB4C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82ECDB50: 807C0008  lwz r3, 8(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ECDB54: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECDB58: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82ECDB5C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ECDB60: 4E800421  bctrl
	ctx.lr = 0x82ECDB64;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECDB64: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82ECDB68: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 82ECDB6C: 409A00BC  bne cr6, 0x82ecdc28
	if !ctx.cr[6].eq {
	pc = 0x82ECDC28; continue 'dispatch;
	}
	// 82ECDB70: 83FC0050  lwz r31, 0x50(r28)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(80 as u32) ) } as u64;
	// 82ECDB74: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82ECDB78: 419A00B0  beq cr6, 0x82ecdc28
	if ctx.cr[6].eq {
	pc = 0x82ECDC28; continue 'dispatch;
	}
	// 82ECDB7C: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82ECDB80: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ECDB84: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82ECDB88: 808B6A30  lwz r4, 0x6a30(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27184 as u32) ) } as u64;
	// 82ECDB8C: 81440000  lwz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECDB90: 812A0060  lwz r9, 0x60(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(96 as u32) ) } as u64;
	// 82ECDB94: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82ECDB98: 4E800421  bctrl
	ctx.lr = 0x82ECDB9C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECDB9C: 89030000  lbz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECDBA0: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82ECDBA4: 409A0084  bne cr6, 0x82ecdc28
	if !ctx.cr[6].eq {
	pc = 0x82ECDC28; continue 'dispatch;
	}
	// 82ECDBA8: 83CD0000  lwz r30, 0(r13)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECDBAC: 3BA00018  li r29, 0x18
	ctx.r[29].s64 = 24;
	// 82ECDBB0: 7D5DF02E  lwzx r10, r29, r30
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82ECDBB4: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ECDBB8: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECDBBC: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82ECDBC0: 40980020  bge cr6, 0x82ecdbe0
	if !ctx.cr[6].lt {
	pc = 0x82ECDBE0; continue 'dispatch;
	}
	// 82ECDBC4: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 82ECDBC8: 390942C0  addi r8, r9, 0x42c0
	ctx.r[8].s64 = ctx.r[9].s64 + 17088;
	// 82ECDBCC: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82ECDBD0: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82ECDBD4: 38AB000C  addi r5, r11, 0xc
	ctx.r[5].s64 = ctx.r[11].s64 + 12;
	// 82ECDBD8: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82ECDBDC: 90AA0004  stw r5, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82ECDBE0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECDBE4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82ECDBE8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECDBEC: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ECDBF0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ECDBF4: 4E800421  bctrl
	ctx.lr = 0x82ECDBF8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECDBF8: 7D5DF02E  lwzx r10, r29, r30
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82ECDBFC: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECDC00: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ECDC04: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82ECDC08: 40980020  bge cr6, 0x82ecdc28
	if !ctx.cr[6].lt {
	pc = 0x82ECDC28; continue 'dispatch;
	}
	// 82ECDC0C: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82ECDC10: 3909BADC  addi r8, r9, -0x4524
	ctx.r[8].s64 = ctx.r[9].s64 + -17700;
	// 82ECDC14: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82ECDC18: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82ECDC1C: 38AB000C  addi r5, r11, 0xc
	ctx.r[5].s64 = ctx.r[11].s64 + 12;
	// 82ECDC20: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82ECDC24: 90AA0004  stw r5, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82ECDC28: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82ECDC2C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82ECDC30: 482DA584  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECDC38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ECDC38 size=216
    let mut pc: u32 = 0x82ECDC38;
    'dispatch: loop {
        match pc {
            0x82ECDC38 => {
    //   block [0x82ECDC38..0x82ECDD10)
	// 82ECDC38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ECDC3C: 482DA52D  bl 0x831a8168
	ctx.lr = 0x82ECDC40;
	sub_831A8130(ctx, base);
	// 82ECDC40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ECDC44: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82ECDC48: 807C0008  lwz r3, 8(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ECDC4C: 4801C245  bl 0x82ee9e90
	ctx.lr = 0x82ECDC50;
	sub_82EE9E90(ctx, base);
	// 82ECDC50: 83FC0050  lwz r31, 0x50(r28)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(80 as u32) ) } as u64;
	// 82ECDC54: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82ECDC58: 419A00B0  beq cr6, 0x82ecdd08
	if ctx.cr[6].eq {
	pc = 0x82ECDD08; continue 'dispatch;
	}
	// 82ECDC5C: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82ECDC60: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ECDC64: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82ECDC68: 808B6A30  lwz r4, 0x6a30(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27184 as u32) ) } as u64;
	// 82ECDC6C: 81440000  lwz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECDC70: 812A0060  lwz r9, 0x60(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(96 as u32) ) } as u64;
	// 82ECDC74: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82ECDC78: 4E800421  bctrl
	ctx.lr = 0x82ECDC7C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECDC7C: 89030000  lbz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECDC80: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82ECDC84: 409A0084  bne cr6, 0x82ecdd08
	if !ctx.cr[6].eq {
	pc = 0x82ECDD08; continue 'dispatch;
	}
	// 82ECDC88: 83CD0000  lwz r30, 0(r13)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECDC8C: 3BA00018  li r29, 0x18
	ctx.r[29].s64 = 24;
	// 82ECDC90: 7D5DF02E  lwzx r10, r29, r30
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82ECDC94: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ECDC98: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECDC9C: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82ECDCA0: 40980020  bge cr6, 0x82ecdcc0
	if !ctx.cr[6].lt {
	pc = 0x82ECDCC0; continue 'dispatch;
	}
	// 82ECDCA4: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 82ECDCA8: 390942C0  addi r8, r9, 0x42c0
	ctx.r[8].s64 = ctx.r[9].s64 + 17088;
	// 82ECDCAC: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82ECDCB0: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82ECDCB4: 38AB000C  addi r5, r11, 0xc
	ctx.r[5].s64 = ctx.r[11].s64 + 12;
	// 82ECDCB8: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82ECDCBC: 90AA0004  stw r5, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82ECDCC0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECDCC4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82ECDCC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECDCCC: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ECDCD0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ECDCD4: 4E800421  bctrl
	ctx.lr = 0x82ECDCD8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECDCD8: 7D5DF02E  lwzx r10, r29, r30
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82ECDCDC: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECDCE0: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ECDCE4: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82ECDCE8: 40980020  bge cr6, 0x82ecdd08
	if !ctx.cr[6].lt {
	pc = 0x82ECDD08; continue 'dispatch;
	}
	// 82ECDCEC: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82ECDCF0: 3909BADC  addi r8, r9, -0x4524
	ctx.r[8].s64 = ctx.r[9].s64 + -17700;
	// 82ECDCF4: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82ECDCF8: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82ECDCFC: 38AB000C  addi r5, r11, 0xc
	ctx.r[5].s64 = ctx.r[11].s64 + 12;
	// 82ECDD00: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82ECDD04: 90AA0004  stw r5, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82ECDD08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82ECDD0C: 482DA4AC  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECDD10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ECDD10 size=100
    let mut pc: u32 = 0x82ECDD10;
    'dispatch: loop {
        match pc {
            0x82ECDD10 => {
    //   block [0x82ECDD10..0x82ECDD74)
	// 82ECDD10: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
	// 82ECDD14: 394002D0  li r10, 0x2d0
	ctx.r[10].s64 = 720;
	// 82ECDD18: 39200030  li r9, 0x30
	ctx.r[9].s64 = 48;
	// 82ECDD1C: 390002E0  li r8, 0x2e0
	ctx.r[8].s64 = 736;
	// 82ECDD20: 38E00040  li r7, 0x40
	ctx.r[7].s64 = 64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECDD74(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82ECDD74 size=308
    let mut pc: u32 = 0x82ECDD74;
    'dispatch: loop {
        match pc {
            0x82ECDD74 => {
    //   block [0x82ECDD74..0x82ECDEA8)
	// 82ECDD74: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 82ECDD78: 99640028  stb r11, 0x28(r4)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[4].u32.wrapping_add(40 as u32), ctx.r[11].u8 ) };
	// 82ECDD7C: 81430074  lwz r10, 0x74(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(116 as u32) ) } as u64;
	// 82ECDD80: C00A1E40  lfs f0, 0x1e40(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(7744 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82ECDD84: D004005C  stfs f0, 0x5c(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 82ECDD88: 81230074  lwz r9, 0x74(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(116 as u32) ) } as u64;
	// 82ECDD8C: C1A91E44  lfs f13, 0x1e44(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(7748 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82ECDD90: D1A40064  stfs f13, 0x64(r4)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 82ECDD94: C18300BC  lfs f12, 0xbc(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(188 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82ECDD98: D1840088  stfs f12, 0x88(r4)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(136 as u32), tmp.u32 ) };
	// 82ECDD9C: C16300C0  lfs f11, 0xc0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(192 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82ECDDA0: D1640088  stfs f11, 0x88(r4)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(136 as u32), tmp.u32 ) };
	// 82ECDDA4: 890300C4  lbz r8, 0xc4(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(196 as u32) ) } as u64;
	// 82ECDDA8: 99040090  stb r8, 0x90(r4)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[4].u32.wrapping_add(144 as u32), ctx.r[8].u8 ) };
	// 82ECDDAC: 80E30050  lwz r7, 0x50(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(80 as u32) ) } as u64;
	// 82ECDDB0: 90E40068  stw r7, 0x68(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(104 as u32), ctx.r[7].u32 ) };
	// 82ECDDB4: 80C302F0  lwz r6, 0x2f0(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(752 as u32) ) } as u64;
	// 82ECDDB8: 90C4006C  stw r6, 0x6c(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(108 as u32), ctx.r[6].u32 ) };
	// 82ECDDBC: 80A302F4  lwz r5, 0x2f4(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(756 as u32) ) } as u64;
	// 82ECDDC0: 90A40060  stw r5, 0x60(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(96 as u32), ctx.r[5].u32 ) };
	// 82ECDDC4: 89630300  lbz r11, 0x300(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(768 as u32) ) } as u64;
	// 82ECDDC8: 99640070  stb r11, 0x70(r4)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[4].u32.wrapping_add(112 as u32), ctx.r[11].u8 ) };
	// 82ECDDCC: C1430200  lfs f10, 0x200(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(512 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82ECDDD0: D1440024  stfs f10, 0x24(r4)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 82ECDDD4: C12301A4  lfs f9, 0x1a4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(420 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 82ECDDD8: D1240074  stfs f9, 0x74(r4)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 82ECDDDC: C10301A8  lfs f8, 0x1a8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(424 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 82ECDDE0: D1040078  stfs f8, 0x78(r4)
	tmp.f32 = (ctx.f[8].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 82ECDDE4: 814302B4  lwz r10, 0x2b4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(692 as u32) ) } as u64;
	// 82ECDDE8: 9144007C  stw r10, 0x7c(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(124 as u32), ctx.r[10].u32 ) };
	// 82ECDDEC: 812302B8  lwz r9, 0x2b8(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(696 as u32) ) } as u64;
	// 82ECDDF0: 91240080  stw r9, 0x80(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(128 as u32), ctx.r[9].u32 ) };
	// 82ECDDF4: 890302C4  lbz r8, 0x2c4(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(708 as u32) ) } as u64;
	// 82ECDDF8: 99040084  stb r8, 0x84(r4)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[4].u32.wrapping_add(132 as u32), ctx.r[8].u8 ) };
	// 82ECDDFC: 88E302C5  lbz r7, 0x2c5(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(709 as u32) ) } as u64;
	// 82ECDE00: 98E400A4  stb r7, 0xa4(r4)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[4].u32.wrapping_add(164 as u32), ctx.r[7].u8 ) };
	// 82ECDE04: 88C302C6  lbz r6, 0x2c6(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(710 as u32) ) } as u64;
	// 82ECDE08: 98C400A5  stb r6, 0xa5(r4)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[4].u32.wrapping_add(165 as u32), ctx.r[6].u8 ) };
	// 82ECDE0C: 88A302C7  lbz r5, 0x2c7(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(711 as u32) ) } as u64;
	// 82ECDE10: 98A400A6  stb r5, 0xa6(r4)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[4].u32.wrapping_add(166 as u32), ctx.r[5].u8 ) };
	// 82ECDE14: 8163006C  lwz r11, 0x6c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(108 as u32) ) } as u64;
	// 82ECDE18: 814B006C  lwz r10, 0x6c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 82ECDE1C: C0EA0000  lfs f7, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 82ECDE20: D0E40094  stfs f7, 0x94(r4)
	tmp.f32 = (ctx.f[7].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(148 as u32), tmp.u32 ) };
	// 82ECDE24: 8123006C  lwz r9, 0x6c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(108 as u32) ) } as u64;
	// 82ECDE28: 8109006C  lwz r8, 0x6c(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(108 as u32) ) } as u64;
	// 82ECDE2C: 80E80004  lwz r7, 4(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECDE30: 90E40098  stw r7, 0x98(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(152 as u32), ctx.r[7].u32 ) };
	// 82ECDE34: 80C3006C  lwz r6, 0x6c(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(108 as u32) ) } as u64;
	// 82ECDE38: 88A60068  lbz r5, 0x68(r6)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[6].u32.wrapping_add(104 as u32) ) } as u64;
	// 82ECDE3C: 98A40091  stb r5, 0x91(r4)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[4].u32.wrapping_add(145 as u32), ctx.r[5].u8 ) };
	// 82ECDE40: 896300C6  lbz r11, 0xc6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(198 as u32) ) } as u64;
	// 82ECDE44: 996400A7  stb r11, 0xa7(r4)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[4].u32.wrapping_add(167 as u32), ctx.r[11].u8 ) };
	// 82ECDE48: C0C300CC  lfs f6, 0xcc(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(204 as u32) ) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 82ECDE4C: D0C400AC  stfs f6, 0xac(r4)
	tmp.f32 = (ctx.f[6].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(172 as u32), tmp.u32 ) };
	// 82ECDE50: 894300D0  lbz r10, 0xd0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(208 as u32) ) } as u64;
	// 82ECDE54: 994400B4  stb r10, 0xb4(r4)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[4].u32.wrapping_add(180 as u32), ctx.r[10].u8 ) };
	// 82ECDE58: 812300D4  lwz r9, 0xd4(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(212 as u32) ) } as u64;
	// 82ECDE5C: 912400B0  stw r9, 0xb0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(176 as u32), ctx.r[9].u32 ) };
	// 82ECDE60: 810300D8  lwz r8, 0xd8(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(216 as u32) ) } as u64;
	// 82ECDE64: 910400B8  stw r8, 0xb8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(184 as u32), ctx.r[8].u32 ) };
	// 82ECDE68: C0A300C8  lfs f5, 0xc8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(200 as u32) ) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 82ECDE6C: D0A400A8  stfs f5, 0xa8(r4)
	tmp.f32 = (ctx.f[5].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(168 as u32), tmp.u32 ) };
	// 82ECDE70: 88E300C5  lbz r7, 0xc5(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(197 as u32) ) } as u64;
	// 82ECDE74: 98E400BC  stb r7, 0xbc(r4)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[4].u32.wrapping_add(188 as u32), ctx.r[7].u8 ) };
	// 82ECDE78: 80C300DC  lwz r6, 0xdc(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(220 as u32) ) } as u64;
	// 82ECDE7C: 98C400BD  stb r6, 0xbd(r4)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[4].u32.wrapping_add(189 as u32), ctx.r[6].u8 ) };
	// 82ECDE80: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ECDE84: C08B0024  lfs f4, 0x24(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 82ECDE88: D08400C8  stfs f4, 0xc8(r4)
	tmp.f32 = (ctx.f[4].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(200 as u32), tmp.u32 ) };
	// 82ECDE8C: 894300B8  lbz r10, 0xb8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(184 as u32) ) } as u64;
	// 82ECDE90: 994400BE  stb r10, 0xbe(r4)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[4].u32.wrapping_add(190 as u32), ctx.r[10].u8 ) };
	// 82ECDE94: 892300A0  lbz r9, 0xa0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(160 as u32) ) } as u64;
	// 82ECDE98: 992400C4  stb r9, 0xc4(r4)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[4].u32.wrapping_add(196 as u32), ctx.r[9].u8 ) };
	// 82ECDE9C: 810300A4  lwz r8, 0xa4(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(164 as u32) ) } as u64;
	// 82ECDEA0: 910400C0  stw r8, 0xc0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(192 as u32), ctx.r[8].u32 ) };
	// 82ECDEA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECDEA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ECDEA8 size=84
    let mut pc: u32 = 0x82ECDEA8;
    'dispatch: loop {
        match pc {
            0x82ECDEA8 => {
    //   block [0x82ECDEA8..0x82ECDEFC)
	// 82ECDEA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ECDEAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ECDEB0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ECDEB4: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 82ECDEB8: 80C30070  lwz r6, 0x70(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(112 as u32) ) } as u64;
	// 82ECDEBC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82ECDEC0: 38E942B8  addi r7, r9, 0x42b8
	ctx.r[7].s64 = ctx.r[9].s64 + 17080;
	// 82ECDEC4: 91610080  stw r11, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 82ECDEC8: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 82ECDECC: 90E10050  stw r7, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[7].u32 ) };
	// 82ECDED0: 7CA82B78  mr r8, r5
	ctx.r[8].u64 = ctx.r[5].u64;
	// 82ECDED4: 91610084  stw r11, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 82ECDED8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82ECDEDC: 80830054  lwz r4, 0x54(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(84 as u32) ) } as u64;
	// 82ECDEE0: 7D455378  mr r5, r10
	ctx.r[5].u64 = ctx.r[10].u64;
	// 82ECDEE4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82ECDEE8: 480563C9  bl 0x82f242b0
	ctx.lr = 0x82ECDEEC;
	sub_82F242B0(ctx, base);
	// 82ECDEEC: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82ECDEF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ECDEF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ECDEF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECDF00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ECDF00 size=320
    let mut pc: u32 = 0x82ECDF00;
    'dispatch: loop {
        match pc {
            0x82ECDF00 => {
    //   block [0x82ECDF00..0x82ECE040)
	// 82ECDF00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ECDF04: 482DA255  bl 0x831a8158
	ctx.lr = 0x82ECDF08;
	sub_831A8130(ctx, base);
	// 82ECDF08: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ECDF0C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82ECDF10: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82ECDF14: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 82ECDF18: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 82ECDF1C: 817D0020  lwz r11, 0x20(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32 as u32) ) } as u64;
	// 82ECDF20: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ECDF24: 419A0070  beq cr6, 0x82ecdf94
	if ctx.cr[6].eq {
	pc = 0x82ECDF94; continue 'dispatch;
	}
	// 82ECDF28: 814B004C  lwz r10, 0x4c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 82ECDF2C: 3BCB0048  addi r30, r11, 0x48
	ctx.r[30].s64 = ctx.r[11].s64 + 72;
	// 82ECDF30: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82ECDF34: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ECDF38: 40990044  ble cr6, 0x82ecdf7c
	if !ctx.cr[6].gt {
	pc = 0x82ECDF7C; continue 'dispatch;
	}
	// 82ECDF3C: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82ECDF40: 409A0018  bne cr6, 0x82ecdf58
	if !ctx.cr[6].eq {
	pc = 0x82ECDF58; continue 'dispatch;
	}
	// 82ECDF44: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECDF48: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECDF4C: 812A0010  lwz r9, 0x10(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82ECDF50: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82ECDF54: 419A0018  beq cr6, 0x82ecdf6c
	if ctx.cr[6].eq {
	pc = 0x82ECDF6C; continue 'dispatch;
	}
	// 82ECDF58: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECDF5C: 57EA103A  slwi r10, r31, 2
	ctx.r[10].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82ECDF60: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82ECDF64: 7C8A582E  lwzx r4, r10, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82ECDF68: 4801D7F9  bl 0x82eeb760
	ctx.lr = 0x82ECDF6C;
	sub_82EEB760(ctx, base);
	// 82ECDF6C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECDF70: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82ECDF74: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82ECDF78: 4198FFC4  blt cr6, 0x82ecdf3c
	if ctx.cr[6].lt {
	pc = 0x82ECDF3C; continue 'dispatch;
	}
	// 82ECDF7C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECDF80: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ECDF84: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82ECDF88: 41990008  bgt cr6, 0x82ecdf90
	if ctx.cr[6].gt {
	pc = 0x82ECDF90; continue 'dispatch;
	}
	// 82ECDF8C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82ECDF90: 7D795B78  mr r25, r11
	ctx.r[25].u64 = ctx.r[11].u64;
	// 82ECDF94: 815D0038  lwz r10, 0x38(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(56 as u32) ) } as u64;
	// 82ECDF98: 3B7D0034  addi r27, r29, 0x34
	ctx.r[27].s64 = ctx.r[29].s64 + 52;
	// 82ECDF9C: 817D0034  lwz r11, 0x34(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(52 as u32) ) } as u64;
	// 82ECDFA0: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82ECDFA4: 7D7C5B78  mr r28, r11
	ctx.r[28].u64 = ctx.r[11].u64;
	// 82ECDFA8: 7D2A5A14  add r9, r10, r11
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82ECDFAC: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82ECDFB0: 419A0060  beq cr6, 0x82ece010
	if ctx.cr[6].eq {
	pc = 0x82ECE010; continue 'dispatch;
	}
	// 82ECDFB4: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECDFB8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82ECDFBC: 3BAB0048  addi r29, r11, 0x48
	ctx.r[29].s64 = ctx.r[11].s64 + 72;
	// 82ECDFC0: 816B004C  lwz r11, 0x4c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 82ECDFC4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ECDFC8: 4099002C  ble cr6, 0x82ecdff4
	if !ctx.cr[6].gt {
	pc = 0x82ECDFF4; continue 'dispatch;
	}
	// 82ECDFCC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82ECDFD0: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECDFD4: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82ECDFD8: 7C9E582E  lwzx r4, r30, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82ECDFDC: 4801D785  bl 0x82eeb760
	ctx.lr = 0x82ECDFE0;
	sub_82EEB760(ctx, base);
	// 82ECDFE0: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECDFE4: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82ECDFE8: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82ECDFEC: 7F1F5000  cmpw cr6, r31, r10
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82ECDFF0: 4198FFE0  blt cr6, 0x82ecdfd0
	if ctx.cr[6].lt {
	pc = 0x82ECDFD0; continue 'dispatch;
	}
	// 82ECDFF4: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECDFF8: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 82ECDFFC: 815B0000  lwz r10, 0(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECE000: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82ECE004: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82ECE008: 7F1C5040  cmplw cr6, r28, r10
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82ECE00C: 409AFFA8  bne cr6, 0x82ecdfb4
	if !ctx.cr[6].eq {
	pc = 0x82ECDFB4; continue 'dispatch;
	}
	// 82ECE010: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECE014: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ECE018: 41990014  bgt cr6, 0x82ece02c
	if ctx.cr[6].gt {
	pc = 0x82ECE02C; continue 'dispatch;
	}
	// 82ECE01C: 7F2B0774  extsb r11, r25
	ctx.r[11].s64 = ctx.r[25].s8 as i64;
	// 82ECE020: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ECE024: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82ECE028: 419A0008  beq cr6, 0x82ece030
	if ctx.cr[6].eq {
	pc = 0x82ECE030; continue 'dispatch;
	}
	// 82ECE02C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82ECE030: 99780000  stb r11, 0(r24)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82ECE034: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82ECE038: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82ECE03C: 482DA16C  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECE040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ECE040 size=180
    let mut pc: u32 = 0x82ECE040;
    'dispatch: loop {
        match pc {
            0x82ECE040 => {
    //   block [0x82ECE040..0x82ECE0F4)
	// 82ECE040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ECE044: 482DA119  bl 0x831a815c
	ctx.lr = 0x82ECE048;
	sub_831A8130(ctx, base);
	// 82ECE048: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ECE04C: 8144002C  lwz r10, 0x2c(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(44 as u32) ) } as u64;
	// 82ECE050: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82ECE054: 81640028  lwz r11, 0x28(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(40 as u32) ) } as u64;
	// 82ECE058: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 82ECE05C: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82ECE060: 3B640028  addi r27, r4, 0x28
	ctx.r[27].s64 = ctx.r[4].s64 + 40;
	// 82ECE064: 7D2A5A14  add r9, r10, r11
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82ECE068: 7D7C5B78  mr r28, r11
	ctx.r[28].u64 = ctx.r[11].u64;
	// 82ECE06C: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82ECE070: 419A0060  beq cr6, 0x82ece0d0
	if ctx.cr[6].eq {
	pc = 0x82ECE0D0; continue 'dispatch;
	}
	// 82ECE074: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECE078: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82ECE07C: 3BAB0048  addi r29, r11, 0x48
	ctx.r[29].s64 = ctx.r[11].s64 + 72;
	// 82ECE080: 816B004C  lwz r11, 0x4c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 82ECE084: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ECE088: 4099002C  ble cr6, 0x82ece0b4
	if !ctx.cr[6].gt {
	pc = 0x82ECE0B4; continue 'dispatch;
	}
	// 82ECE08C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82ECE090: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECE094: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82ECE098: 7C8BF02E  lwzx r4, r11, r30
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82ECE09C: 4801D6C5  bl 0x82eeb760
	ctx.lr = 0x82ECE0A0;
	sub_82EEB760(ctx, base);
	// 82ECE0A0: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECE0A4: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82ECE0A8: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82ECE0AC: 7F1F5000  cmpw cr6, r31, r10
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82ECE0B0: 4198FFE0  blt cr6, 0x82ece090
	if ctx.cr[6].lt {
	pc = 0x82ECE090; continue 'dispatch;
	}
	// 82ECE0B4: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECE0B8: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 82ECE0BC: 815B0000  lwz r10, 0(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECE0C0: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82ECE0C4: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82ECE0C8: 7F1C5040  cmplw cr6, r28, r10
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82ECE0CC: 409AFFA8  bne cr6, 0x82ece074
	if !ctx.cr[6].eq {
	pc = 0x82ECE074; continue 'dispatch;
	}
	// 82ECE0D0: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECE0D4: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82ECE0D8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ECE0DC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82ECE0E0: 41990008  bgt cr6, 0x82ece0e8
	if ctx.cr[6].gt {
	pc = 0x82ECE0E8; continue 'dispatch;
	}
	// 82ECE0E4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82ECE0E8: 99790000  stb r11, 0(r25)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82ECE0EC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82ECE0F0: 482DA0BC  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECE0F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ECE0F8 size=92
    let mut pc: u32 = 0x82ECE0F8;
    'dispatch: loop {
        match pc {
            0x82ECE0F8 => {
    //   block [0x82ECE0F8..0x82ECE154)
	// 82ECE0F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ECE0FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ECE100: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82ECE104: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ECE108: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ECE10C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ECE110: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82ECE114: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82ECE118: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82ECE11C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82ECE120: 4BFFFDE1  bl 0x82ecdf00
	ctx.lr = 0x82ECE124;
	sub_82ECDF00(ctx, base);
	// 82ECE124: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82ECE128: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82ECE12C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82ECE130: 4BFFFF11  bl 0x82ece040
	ctx.lr = 0x82ECE134;
	sub_82ECE040(ctx, base);
	// 82ECE134: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECE138: 997E0040  stb r11, 0x40(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(64 as u32), ctx.r[11].u8 ) };
	// 82ECE13C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82ECE140: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ECE144: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ECE148: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82ECE14C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ECE150: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECE158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ECE158 size=176
    let mut pc: u32 = 0x82ECE158;
    'dispatch: loop {
        match pc {
            0x82ECE158 => {
    //   block [0x82ECE158..0x82ECE208)
	// 82ECE158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ECE15C: 482DA001  bl 0x831a815c
	ctx.lr = 0x82ECE160;
	sub_831A8130(ctx, base);
	// 82ECE160: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ECE164: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82ECE168: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 82ECE16C: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82ECE170: 817B004C  lwz r11, 0x4c(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(76 as u32) ) } as u64;
	// 82ECE174: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ECE178: 40990088  ble cr6, 0x82ece200
	if !ctx.cr[6].gt {
	pc = 0x82ECE200; continue 'dispatch;
	}
	// 82ECE17C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82ECE180: 817B0048  lwz r11, 0x48(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(72 as u32) ) } as u64;
	// 82ECE184: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82ECE188: 7D7C582E  lwzx r11, r28, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82ECE18C: 3BEB0098  addi r31, r11, 0x98
	ctx.r[31].s64 = ctx.r[11].s64 + 152;
	// 82ECE190: A14B009C  lhz r10, 0x9c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(156 as u32) ) } as u64;
	// 82ECE194: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82ECE198: 419A0054  beq cr6, 0x82ece1ec
	if ctx.cr[6].eq {
	pc = 0x82ECE1EC; continue 'dispatch;
	}
	// 82ECE19C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82ECE1A0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECE1A4: 7D4BF214  add r10, r11, r30
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82ECE1A8: 816A000C  lwz r11, 0xc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ECE1AC: A12B0000  lhz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECE1B0: 2B090017  cmplwi cr6, r9, 0x17
	ctx.cr[6].compare_u32(ctx.r[9].u32, 23 as u32, &mut ctx.xer);
	// 82ECE1B4: 4198000C  blt cr6, 0x82ece1c0
	if ctx.cr[6].lt {
	pc = 0x82ECE1C0; continue 'dispatch;
	}
	// 82ECE1B8: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82ECE1BC: 4BFFFFF0  b 0x82ece1ac
	pc = 0x82ECE1AC; continue 'dispatch;
	// 82ECE1C0: A16B0000  lhz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECE1C4: 2B0B0016  cmplwi cr6, r11, 0x16
	ctx.cr[6].compare_u32(ctx.r[11].u32, 22 as u32, &mut ctx.xer);
	// 82ECE1C8: 419A0010  beq cr6, 0x82ece1d8
	if ctx.cr[6].eq {
	pc = 0x82ECE1D8; continue 'dispatch;
	}
	// 82ECE1CC: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82ECE1D0: 808A0000  lwz r4, 0(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECE1D4: 4801D69D  bl 0x82eeb870
	ctx.lr = 0x82ECE1D8;
	sub_82EEB870(ctx, base);
	// 82ECE1D8: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECE1DC: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82ECE1E0: 3BDE0030  addi r30, r30, 0x30
	ctx.r[30].s64 = ctx.r[30].s64 + 48;
	// 82ECE1E4: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82ECE1E8: 4198FFB8  blt cr6, 0x82ece1a0
	if ctx.cr[6].lt {
	pc = 0x82ECE1A0; continue 'dispatch;
	}
	// 82ECE1EC: 817B004C  lwz r11, 0x4c(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(76 as u32) ) } as u64;
	// 82ECE1F0: 3B5A0001  addi r26, r26, 1
	ctx.r[26].s64 = ctx.r[26].s64 + 1;
	// 82ECE1F4: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 82ECE1F8: 7F1A5800  cmpw cr6, r26, r11
	ctx.cr[6].compare_i32(ctx.r[26].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82ECE1FC: 4198FF84  blt cr6, 0x82ece180
	if ctx.cr[6].lt {
	pc = 0x82ECE180; continue 'dispatch;
	}
	// 82ECE200: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82ECE204: 482D9FA8  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECE208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ECE208 size=164
    let mut pc: u32 = 0x82ECE208;
    'dispatch: loop {
        match pc {
            0x82ECE208 => {
    //   block [0x82ECE208..0x82ECE2AC)
	// 82ECE208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ECE20C: 482D9F5D  bl 0x831a8168
	ctx.lr = 0x82ECE210;
	sub_831A8130(ctx, base);
	// 82ECE210: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ECE214: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82ECE218: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82ECE21C: 3BDC0028  addi r30, r28, 0x28
	ctx.r[30].s64 = ctx.r[28].s64 + 40;
	// 82ECE220: 817C002C  lwz r11, 0x2c(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(44 as u32) ) } as u64;
	// 82ECE224: 83FC0028  lwz r31, 0x28(r28)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(40 as u32) ) } as u64;
	// 82ECE228: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82ECE22C: 7D4BFA14  add r10, r11, r31
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82ECE230: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82ECE234: 419A002C  beq cr6, 0x82ece260
	if ctx.cr[6].eq {
	pc = 0x82ECE260; continue 'dispatch;
	}
	// 82ECE238: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82ECE23C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECE240: 4BFFFF19  bl 0x82ece158
	ctx.lr = 0x82ECE244;
	sub_82ECE158(ctx, base);
	// 82ECE244: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECE248: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECE24C: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82ECE250: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82ECE254: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82ECE258: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82ECE25C: 409AFFDC  bne cr6, 0x82ece238
	if !ctx.cr[6].eq {
	pc = 0x82ECE238; continue 'dispatch;
	}
	// 82ECE260: 817C0038  lwz r11, 0x38(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(56 as u32) ) } as u64;
	// 82ECE264: 3BDC0034  addi r30, r28, 0x34
	ctx.r[30].s64 = ctx.r[28].s64 + 52;
	// 82ECE268: 83FC0034  lwz r31, 0x34(r28)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(52 as u32) ) } as u64;
	// 82ECE26C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82ECE270: 7D4BFA14  add r10, r11, r31
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82ECE274: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82ECE278: 419A002C  beq cr6, 0x82ece2a4
	if ctx.cr[6].eq {
	pc = 0x82ECE2A4; continue 'dispatch;
	}
	// 82ECE27C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82ECE280: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECE284: 4BFFFED5  bl 0x82ece158
	ctx.lr = 0x82ECE288;
	sub_82ECE158(ctx, base);
	// 82ECE288: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECE28C: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECE290: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82ECE294: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82ECE298: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82ECE29C: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82ECE2A0: 409AFFDC  bne cr6, 0x82ece27c
	if !ctx.cr[6].eq {
	pc = 0x82ECE27C; continue 'dispatch;
	}
	// 82ECE2A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82ECE2A8: 482D9F10  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECE2B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ECE2B0 size=276
    let mut pc: u32 = 0x82ECE2B0;
    'dispatch: loop {
        match pc {
            0x82ECE2B0 => {
    //   block [0x82ECE2B0..0x82ECE3C4)
	// 82ECE2B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ECE2B4: 482D9EA9  bl 0x831a815c
	ctx.lr = 0x82ECE2B8;
	sub_831A8130(ctx, base);
	// 82ECE2B8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ECE2BC: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82ECE2C0: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82ECE2C4: 3B790028  addi r27, r25, 0x28
	ctx.r[27].s64 = ctx.r[25].s64 + 40;
	// 82ECE2C8: 8159002C  lwz r10, 0x2c(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(44 as u32) ) } as u64;
	// 82ECE2CC: 81790028  lwz r11, 0x28(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(40 as u32) ) } as u64;
	// 82ECE2D0: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82ECE2D4: 7D7C5B78  mr r28, r11
	ctx.r[28].u64 = ctx.r[11].u64;
	// 82ECE2D8: 7D2A5A14  add r9, r10, r11
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82ECE2DC: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82ECE2E0: 419A0060  beq cr6, 0x82ece340
	if ctx.cr[6].eq {
	pc = 0x82ECE340; continue 'dispatch;
	}
	// 82ECE2E4: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECE2E8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82ECE2EC: 3BAB0038  addi r29, r11, 0x38
	ctx.r[29].s64 = ctx.r[11].s64 + 56;
	// 82ECE2F0: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 82ECE2F4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ECE2F8: 4099002C  ble cr6, 0x82ece324
	if !ctx.cr[6].gt {
	pc = 0x82ECE324; continue 'dispatch;
	}
	// 82ECE2FC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82ECE300: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECE304: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82ECE308: 7C8BF02E  lwzx r4, r11, r30
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82ECE30C: 4801D5F5  bl 0x82eeb900
	ctx.lr = 0x82ECE310;
	sub_82EEB900(ctx, base);
	// 82ECE310: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECE314: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82ECE318: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82ECE31C: 7F1F5000  cmpw cr6, r31, r10
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82ECE320: 4198FFE0  blt cr6, 0x82ece300
	if ctx.cr[6].lt {
	pc = 0x82ECE300; continue 'dispatch;
	}
	// 82ECE324: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECE328: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 82ECE32C: 815B0000  lwz r10, 0(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECE330: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82ECE334: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82ECE338: 7F1C5040  cmplw cr6, r28, r10
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82ECE33C: 409AFFA8  bne cr6, 0x82ece2e4
	if !ctx.cr[6].eq {
	pc = 0x82ECE2E4; continue 'dispatch;
	}
	// 82ECE340: 81590038  lwz r10, 0x38(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(56 as u32) ) } as u64;
	// 82ECE344: 3B790034  addi r27, r25, 0x34
	ctx.r[27].s64 = ctx.r[25].s64 + 52;
	// 82ECE348: 81790034  lwz r11, 0x34(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(52 as u32) ) } as u64;
	// 82ECE34C: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82ECE350: 7D7C5B78  mr r28, r11
	ctx.r[28].u64 = ctx.r[11].u64;
	// 82ECE354: 7D2A5A14  add r9, r10, r11
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82ECE358: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82ECE35C: 419A0060  beq cr6, 0x82ece3bc
	if ctx.cr[6].eq {
	pc = 0x82ECE3BC; continue 'dispatch;
	}
	// 82ECE360: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECE364: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82ECE368: 3BAB0038  addi r29, r11, 0x38
	ctx.r[29].s64 = ctx.r[11].s64 + 56;
	// 82ECE36C: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 82ECE370: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ECE374: 4099002C  ble cr6, 0x82ece3a0
	if !ctx.cr[6].gt {
	pc = 0x82ECE3A0; continue 'dispatch;
	}
	// 82ECE378: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82ECE37C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECE380: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82ECE384: 7C9E582E  lwzx r4, r30, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82ECE388: 4801D579  bl 0x82eeb900
	ctx.lr = 0x82ECE38C;
	sub_82EEB900(ctx, base);
	// 82ECE38C: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECE390: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82ECE394: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82ECE398: 7F1F5000  cmpw cr6, r31, r10
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82ECE39C: 4198FFE0  blt cr6, 0x82ece37c
	if ctx.cr[6].lt {
	pc = 0x82ECE37C; continue 'dispatch;
	}
	// 82ECE3A0: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECE3A4: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 82ECE3A8: 815B0000  lwz r10, 0(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECE3AC: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82ECE3B0: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82ECE3B4: 7F1C5040  cmplw cr6, r28, r10
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82ECE3B8: 409AFFA8  bne cr6, 0x82ece360
	if !ctx.cr[6].eq {
	pc = 0x82ECE360; continue 'dispatch;
	}
	// 82ECE3BC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82ECE3C0: 482D9DEC  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECE3C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ECE3C8 size=228
    let mut pc: u32 = 0x82ECE3C8;
    'dispatch: loop {
        match pc {
            0x82ECE3C8 => {
    //   block [0x82ECE3C8..0x82ECE4AC)
	// 82ECE3C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ECE3CC: 482D9D99  bl 0x831a8164
	ctx.lr = 0x82ECE3D0;
	sub_831A8130(ctx, base);
	// 82ECE3D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ECE3D4: 3BA300E4  addi r29, r3, 0xe4
	ctx.r[29].s64 = ctx.r[3].s64 + 228;
	// 82ECE3D8: 83E3018C  lwz r31, 0x18c(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(396 as u32) ) } as u64;
	// 82ECE3DC: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82ECE3E0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82ECE3E4: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECE3E8: 419A0088  beq cr6, 0x82ece470
	if ctx.cr[6].eq {
	pc = 0x82ECE470; continue 'dispatch;
	}
	// 82ECE3EC: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82ECE3F0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ECE3F4: 409900B0  ble cr6, 0x82ece4a4
	if !ctx.cr[6].gt {
	pc = 0x82ECE4A4; continue 'dispatch;
	}
	// 82ECE3F8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82ECE3FC: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECE400: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82ECE404: 7C8BF02E  lwzx r4, r11, r30
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82ECE408: 7F045040  cmplw cr6, r4, r10
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82ECE40C: 419A0048  beq cr6, 0x82ece454
	if ctx.cr[6].eq {
	pc = 0x82ECE454; continue 'dispatch;
	}
	// 82ECE410: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82ECE414: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82ECE418: 419A003C  beq cr6, 0x82ece454
	if ctx.cr[6].eq {
	pc = 0x82ECE454; continue 'dispatch;
	}
	// 82ECE41C: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82ECE420: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82ECE424: 419A0030  beq cr6, 0x82ece454
	if ctx.cr[6].eq {
	pc = 0x82ECE454; continue 'dispatch;
	}
	// 82ECE428: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82ECE42C: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82ECE430: 419A0024  beq cr6, 0x82ece454
	if ctx.cr[6].eq {
	pc = 0x82ECE454; continue 'dispatch;
	}
	// 82ECE434: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82ECE438: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82ECE43C: 419A0018  beq cr6, 0x82ece454
	if ctx.cr[6].eq {
	pc = 0x82ECE454; continue 'dispatch;
	}
	// 82ECE440: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82ECE444: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82ECE448: 419A000C  beq cr6, 0x82ece454
	if ctx.cr[6].eq {
	pc = 0x82ECE454; continue 'dispatch;
	}
	// 82ECE44C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82ECE450: 4801D399  bl 0x82eeb7e8
	ctx.lr = 0x82ECE454;
	sub_82EEB7E8(ctx, base);
	// 82ECE454: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECE458: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82ECE45C: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82ECE460: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82ECE464: 4198FF98  blt cr6, 0x82ece3fc
	if ctx.cr[6].lt {
	pc = 0x82ECE3FC; continue 'dispatch;
	}
	// 82ECE468: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82ECE46C: 482D9D48  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 82ECE470: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82ECE474: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ECE478: 4099002C  ble cr6, 0x82ece4a4
	if !ctx.cr[6].gt {
	pc = 0x82ECE4A4; continue 'dispatch;
	}
	// 82ECE47C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82ECE480: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECE484: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82ECE488: 7C8BF82E  lwzx r4, r11, r31
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82ECE48C: 4801D35D  bl 0x82eeb7e8
	ctx.lr = 0x82ECE490;
	sub_82EEB7E8(ctx, base);
	// 82ECE490: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECE494: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82ECE498: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82ECE49C: 7F1E5000  cmpw cr6, r30, r10
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82ECE4A0: 4198FFE0  blt cr6, 0x82ece480
	if ctx.cr[6].lt {
	pc = 0x82ECE480; continue 'dispatch;
	}
	// 82ECE4A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82ECE4A8: 482D9D0C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECE4B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ECE4B0 size=172
    let mut pc: u32 = 0x82ECE4B0;
    'dispatch: loop {
        match pc {
            0x82ECE4B0 => {
    //   block [0x82ECE4B0..0x82ECE55C)
	// 82ECE4B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ECE4B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ECE4B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82ECE4BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ECE4C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ECE4C4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECE4C8: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82ECE4CC: 38A0002F  li r5, 0x2f
	ctx.r[5].s64 = 47;
	// 82ECE4D0: 38800044  li r4, 0x44
	ctx.r[4].s64 = 68;
	// 82ECE4D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ECE4D8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82ECE4DC: 4BFD2255  bl 0x82ea0730
	ctx.lr = 0x82ECE4E0;
	sub_82EA0730(ctx, base);
	// 82ECE4E0: 39200044  li r9, 0x44
	ctx.r[9].s64 = 68;
	// 82ECE4E4: B1230004  sth r9, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[9].u16 ) };
	// 82ECE4E8: 4801CD19  bl 0x82eeb200
	ctx.lr = 0x82ECE4EC;
	sub_82EEB200(ctx, base);
	// 82ECE4EC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82ECE4F0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82ECE4F4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82ECE4F8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82ECE4FC: 4BFFFA05  bl 0x82ecdf00
	ctx.lr = 0x82ECE500;
	sub_82ECDF00(ctx, base);
	// 82ECE500: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82ECE504: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82ECE508: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82ECE50C: 4BFFFB35  bl 0x82ece040
	ctx.lr = 0x82ECE510;
	sub_82ECE040(ctx, base);
	// 82ECE510: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 82ECE514: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82ECE518: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECE51C: 88E80000  lbz r7, 0(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECE520: 98FE0040  stb r7, 0x40(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(64 as u32), ctx.r[7].u8 ) };
	// 82ECE524: 4BFFFEA5  bl 0x82ece3c8
	ctx.lr = 0x82ECE528;
	sub_82ECE3C8(ctx, base);
	// 82ECE528: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82ECE52C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECE530: 4BFFFCD9  bl 0x82ece208
	ctx.lr = 0x82ECE534;
	sub_82ECE208(ctx, base);
	// 82ECE534: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82ECE538: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECE53C: 4BFFFD75  bl 0x82ece2b0
	ctx.lr = 0x82ECE540;
	sub_82ECE2B0(ctx, base);
	// 82ECE540: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82ECE544: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82ECE548: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ECE54C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ECE550: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82ECE554: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ECE558: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECE560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ECE560 size=112
    let mut pc: u32 = 0x82ECE560;
    'dispatch: loop {
        match pc {
            0x82ECE560 => {
    //   block [0x82ECE560..0x82ECE5D0)
	// 82ECE560: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ECE564: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ECE568: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ECE56C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ECE570: 81630084  lwz r11, 0x84(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(132 as u32) ) } as u64;
	// 82ECE574: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82ECE578: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ECE57C: 419A0024  beq cr6, 0x82ece5a0
	if ctx.cr[6].eq {
	pc = 0x82ECE5A0; continue 'dispatch;
	}
	// 82ECE580: 39600022  li r11, 0x22
	ctx.r[11].s64 = 34;
	// 82ECE584: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 82ECE588: 90A10058  stw r5, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[5].u32 ) };
	// 82ECE58C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82ECE590: 99610050  stb r11, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 82ECE594: 8063007C  lwz r3, 0x7c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(124 as u32) ) } as u64;
	// 82ECE598: 48019271  bl 0x82ee7808
	ctx.lr = 0x82ECE59C;
	sub_82EE7808(ctx, base);
	// 82ECE59C: 4800001C  b 0x82ece5b8
	pc = 0x82ECE5B8; continue 'dispatch;
	// 82ECE5A0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECE5A4: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82ECE5A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECE5AC: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ECE5B0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ECE5B4: 4E800421  bctrl
	ctx.lr = 0x82ECE5B8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECE5B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECE5BC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82ECE5C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ECE5C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ECE5C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ECE5CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECE5D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82ECE5D0 size=124
    let mut pc: u32 = 0x82ECE5D0;
    'dispatch: loop {
        match pc {
            0x82ECE5D0 => {
    //   block [0x82ECE5D0..0x82ECE64C)
	// 82ECE5D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ECE5D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ECE5D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ECE5DC: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82ECE5E0: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82ECE5E4: 81660008  lwz r11, 8(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ECE5E8: C1AA08A4  lfs f13, 0x8a4(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82ECE5EC: C18B0018  lfs f12, 0x18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82ECE5F0: EC0C6028  fsubs f0, f12, f12
	ctx.f[0].f64 = (((ctx.f[12].f64 - ctx.f[12].f64) as f32) as f64);
	// 82ECE5F4: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82ECE5F8: D1810050  stfs f12, 0x50(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82ECE5FC: D1810054  stfs f12, 0x54(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82ECE600: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82ECE604: 409A000C  bne cr6, 0x82ece610
	if !ctx.cr[6].eq {
	pc = 0x82ECE610; continue 'dispatch;
	}
	// 82ECE608: FC006890  fmr f0, f13
	ctx.f[0].f64 = ctx.f[13].f64;
	// 82ECE60C: 48000010  b 0x82ece61c
	pc = 0x82ECE61C; continue 'dispatch;
	// 82ECE610: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82ECE614: C1AB08A8  lfs f13, 0x8a8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82ECE618: EC0D0024  fdivs f0, f13, f0
	ctx.f[0].f64 = ((ctx.f[13].f64 / ctx.f[0].f64) as f32) as f64;
	// 82ECE61C: 80660008  lwz r3, 8(r6)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ECE620: D001005C  stfs f0, 0x5c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 82ECE624: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82ECE628: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82ECE62C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECE630: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82ECE634: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ECE638: 4E800421  bctrl
	ctx.lr = 0x82ECE63C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECE63C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82ECE640: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ECE644: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ECE648: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECE650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82ECE650 size=332
    let mut pc: u32 = 0x82ECE650;
    'dispatch: loop {
        match pc {
            0x82ECE650 => {
    //   block [0x82ECE650..0x82ECE79C)
	// 82ECE650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ECE654: 482D9B15  bl 0x831a8168
	ctx.lr = 0x82ECE658;
	sub_831A8130(ctx, base);
	// 82ECE658: DBC1FFC8  stfd f30, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[30].u64 ) };
	// 82ECE65C: DBE1FFD0  stfd f31, -0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 82ECE660: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ECE664: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ECE668: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82ECE66C: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82ECE670: 3B9F0028  addi r28, r31, 0x28
	ctx.r[28].s64 = ctx.r[31].s64 + 40;
	// 82ECE674: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82ECE678: 815F002C  lwz r10, 0x2c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82ECE67C: C3CB08A8  lfs f30, 0x8a8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 82ECE680: C3E908A4  lfs f31, 0x8a4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82ECE684: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ECE688: 4099007C  ble cr6, 0x82ece704
	if !ctx.cr[6].gt {
	pc = 0x82ECE704; continue 'dispatch;
	}
	// 82ECE68C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82ECE690: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ECE694: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECE698: C1AB0018  lfs f13, 0x18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82ECE69C: EC0D6828  fsubs f0, f13, f13
	ctx.f[0].f64 = (((ctx.f[13].f64 - ctx.f[13].f64) as f32) as f64);
	// 82ECE6A0: 7D3E502E  lwzx r9, r30, r10
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82ECE6A4: 80A9004C  lwz r5, 0x4c(r9)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(76 as u32) ) } as u64;
	// 82ECE6A8: 80890048  lwz r4, 0x48(r9)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(72 as u32) ) } as u64;
	// 82ECE6AC: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82ECE6B0: D1A10050  stfs f13, 0x50(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82ECE6B4: D1A10054  stfs f13, 0x54(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82ECE6B8: FF00F800  fcmpu cr6, f0, f31
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[31].f64);
	// 82ECE6BC: 409A000C  bne cr6, 0x82ece6c8
	if !ctx.cr[6].eq {
	pc = 0x82ECE6C8; continue 'dispatch;
	}
	// 82ECE6C0: FC00F890  fmr f0, f31
	ctx.f[0].f64 = ctx.f[31].f64;
	// 82ECE6C4: 48000008  b 0x82ece6cc
	pc = 0x82ECE6CC; continue 'dispatch;
	// 82ECE6C8: EC1E0024  fdivs f0, f30, f0
	ctx.f[0].f64 = ((ctx.f[30].f64 / ctx.f[0].f64) as f32) as f64;
	// 82ECE6CC: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ECE6D0: D001005C  stfs f0, 0x5c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 82ECE6D4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82ECE6D8: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82ECE6DC: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82ECE6E0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECE6E4: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82ECE6E8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ECE6EC: 4E800421  bctrl
	ctx.lr = 0x82ECE6F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECE6F0: 813C0004  lwz r9, 4(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECE6F4: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82ECE6F8: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82ECE6FC: 7F1D4800  cmpw cr6, r29, r9
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82ECE700: 4198FF90  blt cr6, 0x82ece690
	if ctx.cr[6].lt {
	pc = 0x82ECE690; continue 'dispatch;
	}
	// 82ECE704: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82ECE708: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82ECE70C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ECE710: 4099007C  ble cr6, 0x82ece78c
	if !ctx.cr[6].gt {
	pc = 0x82ECE78C; continue 'dispatch;
	}
	// 82ECE714: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82ECE718: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ECE71C: 815F0034  lwz r10, 0x34(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82ECE720: C1AB0018  lfs f13, 0x18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82ECE724: EC0D6828  fsubs f0, f13, f13
	ctx.f[0].f64 = (((ctx.f[13].f64 - ctx.f[13].f64) as f32) as f64);
	// 82ECE728: 7D3E502E  lwzx r9, r30, r10
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82ECE72C: 80A9004C  lwz r5, 0x4c(r9)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(76 as u32) ) } as u64;
	// 82ECE730: 80890048  lwz r4, 0x48(r9)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(72 as u32) ) } as u64;
	// 82ECE734: D0010068  stfs f0, 0x68(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 82ECE738: D1A10060  stfs f13, 0x60(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 82ECE73C: D1A10064  stfs f13, 0x64(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 82ECE740: FF00F800  fcmpu cr6, f0, f31
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[31].f64);
	// 82ECE744: 409A000C  bne cr6, 0x82ece750
	if !ctx.cr[6].eq {
	pc = 0x82ECE750; continue 'dispatch;
	}
	// 82ECE748: FC00F890  fmr f0, f31
	ctx.f[0].f64 = ctx.f[31].f64;
	// 82ECE74C: 48000008  b 0x82ece754
	pc = 0x82ECE754; continue 'dispatch;
	// 82ECE750: EC1E0024  fdivs f0, f30, f0
	ctx.f[0].f64 = ((ctx.f[30].f64 / ctx.f[0].f64) as f32) as f64;
	// 82ECE754: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ECE758: D001006C  stfs f0, 0x6c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 82ECE75C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82ECE760: 38E10060  addi r7, r1, 0x60
	ctx.r[7].s64 = ctx.r[1].s64 + 96;
	// 82ECE764: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82ECE768: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECE76C: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82ECE770: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ECE774: 4E800421  bctrl
	ctx.lr = 0x82ECE778;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECE778: 813F0038  lwz r9, 0x38(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82ECE77C: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82ECE780: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82ECE784: 7F1D4800  cmpw cr6, r29, r9
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82ECE788: 4198FF90  blt cr6, 0x82ece718
	if ctx.cr[6].lt {
	pc = 0x82ECE718; continue 'dispatch;
	}
	// 82ECE78C: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82ECE790: CBC1FFC8  lfd f30, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 82ECE794: CBE1FFD0  lfd f31, -0x30(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 82ECE798: 482D9A20  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECE7A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ECE7A0 size=20
    let mut pc: u32 = 0x82ECE7A0;
    'dispatch: loop {
        match pc {
            0x82ECE7A0 => {
    //   block [0x82ECE7A0..0x82ECE7B4)
	// 82ECE7A0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82ECE7A4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82ECE7A8: 814B002C  lwz r10, 0x2c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82ECE7AC: 34EAFFFF  addic. r7, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[7].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82ECE7B0: 4D800020  bltlr
	if ctx.cr[0].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECE7B4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ECE7B4 size=76
    let mut pc: u32 = 0x82ECE7B4;
    'dispatch: loop {
        match pc {
            0x82ECE7B4 => {
    //   block [0x82ECE7B4..0x82ECE800)
	// 82ECE7B4: 814B0028  lwz r10, 0x28(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82ECE7B8: 54EB103A  slwi r11, r7, 2
	ctx.r[11].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82ECE7BC: 7D0B5214  add r8, r11, r10
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82ECE7C0: 81480000  lwz r10, 0(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECE7C4: 80CA004C  lwz r6, 0x4c(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(76 as u32) ) } as u64;
	// 82ECE7C8: 812A0014  lwz r9, 0x14(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82ECE7CC: 54CB2834  slwi r11, r6, 5
	ctx.r[11].u32 = ctx.r[6].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82ECE7D0: 814A000C  lwz r10, 0xc(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ECE7D4: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82ECE7D8: 38AB002A  addi r5, r11, 0x2a
	ctx.r[5].s64 = ctx.r[11].s64 + 42;
	// 82ECE7DC: 54AB103A  slwi r11, r5, 2
	ctx.r[11].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82ECE7E0: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82ECE7E4: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82ECE7E8: 41990008  bgt cr6, 0x82ece7f0
	if ctx.cr[6].gt {
	pc = 0x82ECE7F0; continue 'dispatch;
	}
	// 82ECE7EC: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82ECE7F0: 34E7FFFF  addic. r7, r7, -1
	ctx.xer.ca = (ctx.r[7].u32 > (!(-1 as u32)));
	ctx.r[7].s64 = ctx.r[7].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82ECE7F4: 3908FFFC  addi r8, r8, -4
	ctx.r[8].s64 = ctx.r[8].s64 + -4;
	// 82ECE7F8: 4080FFC8  bge 0x82ece7c0
	if !ctx.cr[0].lt {
	pc = 0x82ECE7C0; continue 'dispatch;
	}
	// 82ECE7FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECE800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ECE800 size=8
    let mut pc: u32 = 0x82ECE800;
    'dispatch: loop {
        match pc {
            0x82ECE800 => {
    //   block [0x82ECE800..0x82ECE808)
	// 82ECE800: 806300AC  lwz r3, 0xac(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(172 as u32) ) } as u64;
	// 82ECE804: 4BFD38AC  b 0x82ea20b0
	sub_82EA20B0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECE808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ECE808 size=12
    let mut pc: u32 = 0x82ECE808;
    'dispatch: loop {
        match pc {
            0x82ECE808 => {
    //   block [0x82ECE808..0x82ECE814)
	// 82ECE808: 806300A8  lwz r3, 0xa8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(168 as u32) ) } as u64;
	// 82ECE80C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ECE810: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECE814(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ECE814 size=8
    let mut pc: u32 = 0x82ECE814;
    'dispatch: loop {
        match pc {
            0x82ECE814 => {
    //   block [0x82ECE814..0x82ECE81C)
	// 82ECE814: 4BFD389C  b 0x82ea20b0
	sub_82EA20B0(ctx, base);
	return;
	// 82ECE818: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECE820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ECE820 size=12
    let mut pc: u32 = 0x82ECE820;
    'dispatch: loop {
        match pc {
            0x82ECE820 => {
    //   block [0x82ECE820..0x82ECE82C)
	// 82ECE820: 806300A8  lwz r3, 0xa8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(168 as u32) ) } as u64;
	// 82ECE824: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ECE828: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECE82C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ECE82C size=8
    let mut pc: u32 = 0x82ECE82C;
    'dispatch: loop {
        match pc {
            0x82ECE82C => {
    //   block [0x82ECE82C..0x82ECE834)
	// 82ECE82C: 4BFD3884  b 0x82ea20b0
	sub_82EA20B0(ctx, base);
	return;
	// 82ECE830: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECE838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ECE838 size=316
    let mut pc: u32 = 0x82ECE838;
    'dispatch: loop {
        match pc {
            0x82ECE838 => {
    //   block [0x82ECE838..0x82ECE974)
	// 82ECE838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ECE83C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ECE840: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82ECE844: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ECE848: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ECE84C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ECE850: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82ECE854: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82ECE858: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ECE85C: 419A0044  beq cr6, 0x82ece8a0
	if ctx.cr[6].eq {
	pc = 0x82ECE8A0; continue 'dispatch;
	}
	// 82ECE860: 3960001F  li r11, 0x1f
	ctx.r[11].s64 = 31;
	// 82ECE864: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECE868: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82ECE86C: 99610050  stb r11, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 82ECE870: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82ECE874: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 82ECE878: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82ECE87C: 4BFD1EB5  bl 0x82ea0730
	ctx.lr = 0x82ECE880;
	sub_82EA0730(ctx, base);
	// 82ECE880: 90610054  stw r3, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 82ECE884: 38A00020  li r5, 0x20
	ctx.r[5].s64 = 32;
	// 82ECE888: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82ECE88C: 4BFDBEBD  bl 0x82eaa748
	ctx.lr = 0x82ECE890;
	sub_82EAA748(ctx, base);
	// 82ECE890: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82ECE894: 807F007C  lwz r3, 0x7c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 82ECE898: 48018F71  bl 0x82ee7808
	ctx.lr = 0x82ECE89C;
	sub_82EE7808(ctx, base);
	// 82ECE89C: 480000C0  b 0x82ece95c
	pc = 0x82ECE95C; continue 'dispatch;
	// 82ECE8A0: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82ECE8A4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82ECE8A8: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 82ECE8AC: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 82ECE8B0: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 82ECE8B4: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 82ECE8B8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82ECE8BC: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82ECE8C0: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECE8C4: 812A0040  lwz r9, 0x40(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(64 as u32) ) } as u64;
	// 82ECE8C8: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82ECE8CC: 4E800421  bctrl
	ctx.lr = 0x82ECE8D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECE8D0: 8101005C  lwz r8, 0x5c(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82ECE8D4: 7FFEFB78  mr r30, r31
	ctx.r[30].u64 = ctx.r[31].u64;
	// 82ECE8D8: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82ECE8DC: 40990054  ble cr6, 0x82ece930
	if !ctx.cr[6].gt {
	pc = 0x82ECE930; continue 'dispatch;
	}
	// 82ECE8E0: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82ECE8E4: 7D5F5A14  add r10, r31, r11
	ctx.r[10].u64 = ctx.r[31].u64 + ctx.r[11].u64;
	// 82ECE8E8: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECE8EC: 892B0005  lbz r9, 5(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(5 as u32) ) } as u64;
	// 82ECE8F0: 7D2A0774  extsb r10, r9
	ctx.r[10].s64 = ctx.r[9].s8 as i64;
	// 82ECE8F4: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82ECE8F8: 890B0018  lbz r8, 0x18(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82ECE8FC: 2B080001  cmplwi cr6, r8, 1
	ctx.cr[6].compare_u32(ctx.r[8].u32, 1 as u32, &mut ctx.xer);
	// 82ECE900: 409A001C  bne cr6, 0x82ece91c
	if !ctx.cr[6].eq {
	pc = 0x82ECE91C; continue 'dispatch;
	}
	// 82ECE904: 894B0010  lbz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82ECE908: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 82ECE90C: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82ECE910: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ECE914: 419A0008  beq cr6, 0x82ece91c
	if ctx.cr[6].eq {
	pc = 0x82ECE91C; continue 'dispatch;
	}
	// 82ECE918: 4800A351  bl 0x82ed8c68
	ctx.lr = 0x82ECE91C;
	sub_82ED8C68(ctx, base);
	// 82ECE91C: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82ECE920: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82ECE924: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 82ECE928: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82ECE92C: 4198FFB4  blt cr6, 0x82ece8e0
	if ctx.cr[6].lt {
	pc = 0x82ECE8E0; continue 'dispatch;
	}
	// 82ECE930: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82ECE934: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ECE938: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ECE93C: 409A0020  bne cr6, 0x82ece95c
	if !ctx.cr[6].eq {
	pc = 0x82ECE95C; continue 'dispatch;
	}
	// 82ECE940: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECE944: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82ECE948: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82ECE94C: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82ECE950: 55651838  slwi r5, r11, 3
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82ECE954: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82ECE958: 4BFD1E59  bl 0x82ea07b0
	ctx.lr = 0x82ECE95C;
	sub_82EA07B0(ctx, base);
	// 82ECE95C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82ECE960: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ECE964: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ECE968: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82ECE96C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ECE970: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECE978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ECE978 size=196
    let mut pc: u32 = 0x82ECE978;
    'dispatch: loop {
        match pc {
            0x82ECE978 => {
    //   block [0x82ECE978..0x82ECEA3C)
	// 82ECE978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ECE97C: 482D97ED  bl 0x831a8168
	ctx.lr = 0x82ECE980;
	sub_831A8130(ctx, base);
	// 82ECE980: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ECE984: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82ECE988: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82ECE98C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82ECE990: 3BFD01F8  addi r31, r29, 0x1f8
	ctx.r[31].s64 = ctx.r[29].s64 + 504;
	// 82ECE994: A17D01FE  lhz r11, 0x1fe(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(510 as u32) ) } as u64;
	// 82ECE998: A15D01FC  lhz r10, 0x1fc(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(508 as u32) ) } as u64;
	// 82ECE99C: 556904BE  clrlwi r9, r11, 0x12
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00003FFFu64;
	// 82ECE9A0: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82ECE9A4: 409A0010  bne cr6, 0x82ece9b4
	if !ctx.cr[6].eq {
	pc = 0x82ECE9B4; continue 'dispatch;
	}
	// 82ECE9A8: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82ECE9AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECE9B0: 483309A9  bl 0x831ff358
	ctx.lr = 0x82ECE9B4;
	sub_831FF358(ctx, base);
	// 82ECE9B4: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECE9B8: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECE9BC: 5569103E  rotlwi r9, r11, 2
	ctx.r[9].u64 = ((ctx.r[11].u32).rotate_left(2)) as u64;
	// 82ECE9C0: 7FC9512E  stwx r30, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 82ECE9C4: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECE9C8: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 82ECE9CC: B11F0004  sth r8, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[8].u16 ) };
	// 82ECE9D0: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ECE9D4: A0C30020  lhz r6, 0x20(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82ECE9D8: 2B06FFFF  cmplwi cr6, r6, 0xffff
	ctx.cr[6].compare_u32(ctx.r[6].u32, 65535 as u32, &mut ctx.xer);
	// 82ECE9DC: 409A002C  bne cr6, 0x82ecea08
	if !ctx.cr[6].eq {
	pc = 0x82ECEA08; continue 'dispatch;
	}
	// 82ECE9E0: 897D00D8  lbz r11, 0xd8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(216 as u32) ) } as u64;
	// 82ECE9E4: 2B0B0007  cmplwi cr6, r11, 7
	ctx.cr[6].compare_u32(ctx.r[11].u32, 7 as u32, &mut ctx.xer);
	// 82ECE9E8: 419A0020  beq cr6, 0x82ecea08
	if ctx.cr[6].eq {
	pc = 0x82ECEA08; continue 'dispatch;
	}
	// 82ECE9EC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82ECE9F0: 4801D5D1  bl 0x82eebfc0
	ctx.lr = 0x82ECE9F4;
	sub_82EEBFC0(ctx, base);
	// 82ECE9F4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82ECE9F8: 807D00B8  lwz r3, 0xb8(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(184 as u32) ) } as u64;
	// 82ECE9FC: 4801DA7D  bl 0x82eec478
	ctx.lr = 0x82ECEA00;
	sub_82EEC478(ctx, base);
	// 82ECEA00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82ECEA04: 482D97B4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 82ECEA08: 817D00B8  lwz r11, 0xb8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(184 as u32) ) } as u64;
	// 82ECEA0C: 7F0B1800  cmpw cr6, r11, r3
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[3].s32, &mut ctx.xer);
	// 82ECEA10: 419A0024  beq cr6, 0x82ecea34
	if ctx.cr[6].eq {
	pc = 0x82ECEA34; continue 'dispatch;
	}
	// 82ECEA14: 897D00D8  lbz r11, 0xd8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(216 as u32) ) } as u64;
	// 82ECEA18: 2B0B0007  cmplwi cr6, r11, 7
	ctx.cr[6].compare_u32(ctx.r[11].u32, 7 as u32, &mut ctx.xer);
	// 82ECEA1C: 419A0018  beq cr6, 0x82ecea34
	if ctx.cr[6].eq {
	pc = 0x82ECEA34; continue 'dispatch;
	}
	// 82ECEA20: 81630048  lwz r11, 0x48(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 82ECEA24: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82ECEA28: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82ECEA2C: 80AB0000  lwz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECEA30: 48013CF1  bl 0x82ee2720
	ctx.lr = 0x82ECEA34;
	sub_82EE2720(ctx, base);
	// 82ECEA34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82ECEA38: 482D9780  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECEA40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ECEA40 size=292
    let mut pc: u32 = 0x82ECEA40;
    'dispatch: loop {
        match pc {
            0x82ECEA40 => {
    //   block [0x82ECEA40..0x82ECEB64)
	// 82ECEA40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ECEA44: 482D9729  bl 0x831a816c
	ctx.lr = 0x82ECEA48;
	sub_831A8130(ctx, base);
	// 82ECEA48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ECEA4C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ECEA50: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82ECEA54: 815F0088  lwz r10, 0x88(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(136 as u32) ) } as u64;
	// 82ECEA58: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82ECEA5C: 7D4A5A15  add. r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ECEA60: 41820028  beq 0x82ecea88
	if ctx.cr[0].eq {
	pc = 0x82ECEA88; continue 'dispatch;
	}
	// 82ECEA64: 3960000D  li r11, 0xd
	ctx.r[11].s64 = 13;
	// 82ECEA68: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 82ECEA6C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82ECEA70: 807F007C  lwz r3, 0x7c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 82ECEA74: 99610050  stb r11, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 82ECEA78: 48018D91  bl 0x82ee7808
	ctx.lr = 0x82ECEA7C;
	sub_82EE7808(ctx, base);
	// 82ECEA7C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82ECEA80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82ECEA84: 482D9738  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 82ECEA88: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82ECEA8C: 917F0084  stw r11, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 82ECEA90: 815E0018  lwz r10, 0x18(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82ECEA94: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ECEA98: 409A001C  bne cr6, 0x82eceab4
	if !ctx.cr[6].eq {
	pc = 0x82ECEAB4; continue 'dispatch;
	}
	// 82ECEA9C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECEAA0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82ECEAA4: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82ECEAA8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ECEAAC: 4E800421  bctrl
	ctx.lr = 0x82ECEAB0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECEAB0: 907E0018  stw r3, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[3].u32 ) };
	// 82ECEAB4: 93FE0008  stw r31, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 82ECEAB8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82ECEABC: 4800662D  bl 0x82ed50e8
	ctx.lr = 0x82ECEAC0;
	sub_82ED50E8(ctx, base);
	// 82ECEAC0: 817F00EC  lwz r11, 0xec(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(236 as u32) ) } as u64;
	// 82ECEAC4: 815F00E8  lwz r10, 0xe8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(232 as u32) ) } as u64;
	// 82ECEAC8: 3BBF00E4  addi r29, r31, 0xe4
	ctx.r[29].s64 = ctx.r[31].s64 + 228;
	// 82ECEACC: 556900BE  clrlwi r9, r11, 2
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82ECEAD0: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82ECEAD4: 409A0010  bne cr6, 0x82eceae4
	if !ctx.cr[6].eq {
	pc = 0x82ECEAE4; continue 'dispatch;
	}
	// 82ECEAD8: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82ECEADC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82ECEAE0: 4BFD7DA1  bl 0x82ea6880
	ctx.lr = 0x82ECEAE4;
	sub_82EA6880(ctx, base);
	// 82ECEAE4: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECEAE8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82ECEAEC: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECEAF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECEAF4: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82ECEAF8: 7FC9512E  stwx r30, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 82ECEAFC: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECEB00: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 82ECEB04: 911D0004  stw r8, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82ECEB08: 480128C1  bl 0x82ee13c8
	ctx.lr = 0x82ECEB0C;
	sub_82EE13C8(ctx, base);
	// 82ECEB0C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82ECEB10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECEB14: 480151F5  bl 0x82ee3d08
	ctx.lr = 0x82ECEB18;
	sub_82EE3D08(ctx, base);
	// 82ECEB18: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82ECEB1C: 48006D6D  bl 0x82ed5888
	ctx.lr = 0x82ECEB20;
	sub_82ED5888(ctx, base);
	// 82ECEB20: 80FF0084  lwz r7, 0x84(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82ECEB24: 3567FFFF  addic. r11, r7, -1
	ctx.xer.ca = (ctx.r[7].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[7].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ECEB28: 917F0084  stw r11, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 82ECEB2C: 4082002C  bne 0x82eceb58
	if !ctx.cr[0].eq {
	pc = 0x82ECEB58; continue 'dispatch;
	}
	// 82ECEB30: 897F008C  lbz r11, 0x8c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) } as u64;
	// 82ECEB34: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ECEB38: 409A0020  bne cr6, 0x82eceb58
	if !ctx.cr[6].eq {
	pc = 0x82ECEB58; continue 'dispatch;
	}
	// 82ECEB3C: 817F0080  lwz r11, 0x80(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 82ECEB40: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ECEB44: 419A0014  beq cr6, 0x82eceb58
	if ctx.cr[6].eq {
	pc = 0x82ECEB58; continue 'dispatch;
	}
	// 82ECEB48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82ECEB4C: 807F007C  lwz r3, 0x7c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 82ECEB50: 917F0080  stw r11, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 82ECEB54: 480192A5  bl 0x82ee7df8
	ctx.lr = 0x82ECEB58;
	sub_82EE7DF8(ctx, base);
	// 82ECEB58: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82ECEB5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82ECEB60: 482D965C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECEB68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ECEB68 size=112
    let mut pc: u32 = 0x82ECEB68;
    'dispatch: loop {
        match pc {
            0x82ECEB68 => {
    //   block [0x82ECEB68..0x82ECEBD8)
	// 82ECEB68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ECEB6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ECEB70: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82ECEB74: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ECEB78: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ECEB7C: 816300F8  lwz r11, 0xf8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(248 as u32) ) } as u64;
	// 82ECEB80: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82ECEB84: 814300F4  lwz r10, 0xf4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(244 as u32) ) } as u64;
	// 82ECEB88: 3BE300F0  addi r31, r3, 0xf0
	ctx.r[31].s64 = ctx.r[3].s64 + 240;
	// 82ECEB8C: 556900BE  clrlwi r9, r11, 2
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82ECEB90: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82ECEB94: 409A0010  bne cr6, 0x82eceba4
	if !ctx.cr[6].eq {
	pc = 0x82ECEBA4; continue 'dispatch;
	}
	// 82ECEB98: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82ECEB9C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECEBA0: 4BFD7CE1  bl 0x82ea6880
	ctx.lr = 0x82ECEBA4;
	sub_82EA6880(ctx, base);
	// 82ECEBA4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECEBA8: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECEBAC: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82ECEBB0: 7FC9512E  stwx r30, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 82ECEBB4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECEBB8: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 82ECEBBC: 911F0004  stw r8, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82ECEBC0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82ECEBC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ECEBC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ECEBCC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82ECEBD0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ECEBD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECEBD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ECEBD8 size=112
    let mut pc: u32 = 0x82ECEBD8;
    'dispatch: loop {
        match pc {
            0x82ECEBD8 => {
    //   block [0x82ECEBD8..0x82ECEC48)
	// 82ECEBD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ECEBDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ECEBE0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82ECEBE4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ECEBE8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ECEBEC: 8163011C  lwz r11, 0x11c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(284 as u32) ) } as u64;
	// 82ECEBF0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82ECEBF4: 81430118  lwz r10, 0x118(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(280 as u32) ) } as u64;
	// 82ECEBF8: 3BE30114  addi r31, r3, 0x114
	ctx.r[31].s64 = ctx.r[3].s64 + 276;
	// 82ECEBFC: 556900BE  clrlwi r9, r11, 2
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82ECEC00: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82ECEC04: 409A0010  bne cr6, 0x82ecec14
	if !ctx.cr[6].eq {
	pc = 0x82ECEC14; continue 'dispatch;
	}
	// 82ECEC08: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82ECEC0C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECEC10: 4BFD7C71  bl 0x82ea6880
	ctx.lr = 0x82ECEC14;
	sub_82EA6880(ctx, base);
	// 82ECEC14: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECEC18: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECEC1C: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82ECEC20: 7FC9512E  stwx r30, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 82ECEC24: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECEC28: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 82ECEC2C: 911F0004  stw r8, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82ECEC30: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82ECEC34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ECEC38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ECEC3C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82ECEC40: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ECEC44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECEC48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ECEC48 size=112
    let mut pc: u32 = 0x82ECEC48;
    'dispatch: loop {
        match pc {
            0x82ECEC48 => {
    //   block [0x82ECEC48..0x82ECECB8)
	// 82ECEC48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ECEC4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ECEC50: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82ECEC54: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ECEC58: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ECEC5C: 81630104  lwz r11, 0x104(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(260 as u32) ) } as u64;
	// 82ECEC60: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82ECEC64: 81430100  lwz r10, 0x100(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(256 as u32) ) } as u64;
	// 82ECEC68: 3BE300FC  addi r31, r3, 0xfc
	ctx.r[31].s64 = ctx.r[3].s64 + 252;
	// 82ECEC6C: 556900BE  clrlwi r9, r11, 2
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82ECEC70: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82ECEC74: 409A0010  bne cr6, 0x82ecec84
	if !ctx.cr[6].eq {
	pc = 0x82ECEC84; continue 'dispatch;
	}
	// 82ECEC78: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82ECEC7C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECEC80: 4BFD7C01  bl 0x82ea6880
	ctx.lr = 0x82ECEC84;
	sub_82EA6880(ctx, base);
	// 82ECEC84: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECEC88: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECEC8C: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82ECEC90: 7FC9512E  stwx r30, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 82ECEC94: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECEC98: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 82ECEC9C: 911F0004  stw r8, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82ECECA0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82ECECA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ECECA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ECECAC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82ECECB0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ECECB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECECB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ECECB8 size=112
    let mut pc: u32 = 0x82ECECB8;
    'dispatch: loop {
        match pc {
            0x82ECECB8 => {
    //   block [0x82ECECB8..0x82ECED28)
	// 82ECECB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ECECBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ECECC0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82ECECC4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ECECC8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ECECCC: 81630110  lwz r11, 0x110(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(272 as u32) ) } as u64;
	// 82ECECD0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82ECECD4: 8143010C  lwz r10, 0x10c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(268 as u32) ) } as u64;
	// 82ECECD8: 3BE30108  addi r31, r3, 0x108
	ctx.r[31].s64 = ctx.r[3].s64 + 264;
	// 82ECECDC: 556900BE  clrlwi r9, r11, 2
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82ECECE0: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82ECECE4: 409A0010  bne cr6, 0x82ececf4
	if !ctx.cr[6].eq {
	pc = 0x82ECECF4; continue 'dispatch;
	}
	// 82ECECE8: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82ECECEC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECECF0: 4BFD7B91  bl 0x82ea6880
	ctx.lr = 0x82ECECF4;
	sub_82EA6880(ctx, base);
	// 82ECECF4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECECF8: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECECFC: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82ECED00: 7FC9512E  stwx r30, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 82ECED04: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECED08: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 82ECED0C: 911F0004  stw r8, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82ECED10: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82ECED14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ECED18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ECED1C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82ECED20: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ECED24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECED28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ECED28 size=112
    let mut pc: u32 = 0x82ECED28;
    'dispatch: loop {
        match pc {
            0x82ECED28 => {
    //   block [0x82ECED28..0x82ECED98)
	// 82ECED28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ECED2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ECED30: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82ECED34: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ECED38: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ECED3C: 81630134  lwz r11, 0x134(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(308 as u32) ) } as u64;
	// 82ECED40: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82ECED44: 81430130  lwz r10, 0x130(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(304 as u32) ) } as u64;
	// 82ECED48: 3BE3012C  addi r31, r3, 0x12c
	ctx.r[31].s64 = ctx.r[3].s64 + 300;
	// 82ECED4C: 556900BE  clrlwi r9, r11, 2
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82ECED50: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82ECED54: 409A0010  bne cr6, 0x82eced64
	if !ctx.cr[6].eq {
	pc = 0x82ECED64; continue 'dispatch;
	}
	// 82ECED58: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82ECED5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECED60: 4BFD7B21  bl 0x82ea6880
	ctx.lr = 0x82ECED64;
	sub_82EA6880(ctx, base);
	// 82ECED64: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECED68: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECED6C: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82ECED70: 7FC9512E  stwx r30, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 82ECED74: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECED78: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 82ECED7C: 911F0004  stw r8, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82ECED80: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82ECED84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ECED88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ECED8C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82ECED90: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ECED94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECED98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ECED98 size=112
    let mut pc: u32 = 0x82ECED98;
    'dispatch: loop {
        match pc {
            0x82ECED98 => {
    //   block [0x82ECED98..0x82ECEE08)
	// 82ECED98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ECED9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ECEDA0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82ECEDA4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ECEDA8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ECEDAC: 81630158  lwz r11, 0x158(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(344 as u32) ) } as u64;
	// 82ECEDB0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82ECEDB4: 81430154  lwz r10, 0x154(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(340 as u32) ) } as u64;
	// 82ECEDB8: 3BE30150  addi r31, r3, 0x150
	ctx.r[31].s64 = ctx.r[3].s64 + 336;
	// 82ECEDBC: 556900BE  clrlwi r9, r11, 2
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82ECEDC0: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82ECEDC4: 409A0010  bne cr6, 0x82ecedd4
	if !ctx.cr[6].eq {
	pc = 0x82ECEDD4; continue 'dispatch;
	}
	// 82ECEDC8: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82ECEDCC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECEDD0: 4BFD7AB1  bl 0x82ea6880
	ctx.lr = 0x82ECEDD4;
	sub_82EA6880(ctx, base);
	// 82ECEDD4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECEDD8: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECEDDC: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82ECEDE0: 7FC9512E  stwx r30, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 82ECEDE4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECEDE8: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 82ECEDEC: 911F0004  stw r8, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82ECEDF0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82ECEDF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ECEDF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ECEDFC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82ECEE00: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ECEE04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECEE08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ECEE08 size=112
    let mut pc: u32 = 0x82ECEE08;
    'dispatch: loop {
        match pc {
            0x82ECEE08 => {
    //   block [0x82ECEE08..0x82ECEE78)
	// 82ECEE08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ECEE0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ECEE10: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82ECEE14: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ECEE18: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ECEE1C: 81630140  lwz r11, 0x140(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(320 as u32) ) } as u64;
	// 82ECEE20: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82ECEE24: 8143013C  lwz r10, 0x13c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(316 as u32) ) } as u64;
	// 82ECEE28: 3BE30138  addi r31, r3, 0x138
	ctx.r[31].s64 = ctx.r[3].s64 + 312;
	// 82ECEE2C: 556900BE  clrlwi r9, r11, 2
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82ECEE30: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82ECEE34: 409A0010  bne cr6, 0x82ecee44
	if !ctx.cr[6].eq {
	pc = 0x82ECEE44; continue 'dispatch;
	}
	// 82ECEE38: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82ECEE3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECEE40: 4BFD7A41  bl 0x82ea6880
	ctx.lr = 0x82ECEE44;
	sub_82EA6880(ctx, base);
	// 82ECEE44: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECEE48: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECEE4C: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82ECEE50: 7FC9512E  stwx r30, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 82ECEE54: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECEE58: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 82ECEE5C: 911F0004  stw r8, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82ECEE60: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82ECEE64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ECEE68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ECEE6C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82ECEE70: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ECEE74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECEE78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ECEE78 size=112
    let mut pc: u32 = 0x82ECEE78;
    'dispatch: loop {
        match pc {
            0x82ECEE78 => {
    //   block [0x82ECEE78..0x82ECEEE8)
	// 82ECEE78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ECEE7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ECEE80: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82ECEE84: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ECEE88: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ECEE8C: 8163014C  lwz r11, 0x14c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(332 as u32) ) } as u64;
	// 82ECEE90: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82ECEE94: 81430148  lwz r10, 0x148(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(328 as u32) ) } as u64;
	// 82ECEE98: 3BE30144  addi r31, r3, 0x144
	ctx.r[31].s64 = ctx.r[3].s64 + 324;
	// 82ECEE9C: 556900BE  clrlwi r9, r11, 2
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82ECEEA0: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82ECEEA4: 409A0010  bne cr6, 0x82eceeb4
	if !ctx.cr[6].eq {
	pc = 0x82ECEEB4; continue 'dispatch;
	}
	// 82ECEEA8: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82ECEEAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECEEB0: 4BFD79D1  bl 0x82ea6880
	ctx.lr = 0x82ECEEB4;
	sub_82EA6880(ctx, base);
	// 82ECEEB4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECEEB8: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECEEBC: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82ECEEC0: 7FC9512E  stwx r30, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 82ECEEC4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECEEC8: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 82ECEECC: 911F0004  stw r8, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82ECEED0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82ECEED4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ECEED8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ECEEDC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82ECEEE0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ECEEE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECEEE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ECEEE8 size=112
    let mut pc: u32 = 0x82ECEEE8;
    'dispatch: loop {
        match pc {
            0x82ECEEE8 => {
    //   block [0x82ECEEE8..0x82ECEF58)
	// 82ECEEE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ECEEEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ECEEF0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82ECEEF4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ECEEF8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ECEEFC: 81630170  lwz r11, 0x170(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(368 as u32) ) } as u64;
	// 82ECEF00: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82ECEF04: 8143016C  lwz r10, 0x16c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(364 as u32) ) } as u64;
	// 82ECEF08: 3BE30168  addi r31, r3, 0x168
	ctx.r[31].s64 = ctx.r[3].s64 + 360;
	// 82ECEF0C: 556900BE  clrlwi r9, r11, 2
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82ECEF10: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82ECEF14: 409A0010  bne cr6, 0x82ecef24
	if !ctx.cr[6].eq {
	pc = 0x82ECEF24; continue 'dispatch;
	}
	// 82ECEF18: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82ECEF1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECEF20: 4BFD7961  bl 0x82ea6880
	ctx.lr = 0x82ECEF24;
	sub_82EA6880(ctx, base);
	// 82ECEF24: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECEF28: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECEF2C: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82ECEF30: 7FC9512E  stwx r30, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 82ECEF34: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECEF38: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 82ECEF3C: 911F0004  stw r8, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82ECEF40: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82ECEF44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ECEF48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ECEF4C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82ECEF50: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ECEF54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECEF58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ECEF58 size=112
    let mut pc: u32 = 0x82ECEF58;
    'dispatch: loop {
        match pc {
            0x82ECEF58 => {
    //   block [0x82ECEF58..0x82ECEFC8)
	// 82ECEF58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ECEF5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ECEF60: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82ECEF64: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ECEF68: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ECEF6C: 81630164  lwz r11, 0x164(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(356 as u32) ) } as u64;
	// 82ECEF70: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82ECEF74: 81430160  lwz r10, 0x160(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(352 as u32) ) } as u64;
	// 82ECEF78: 3BE3015C  addi r31, r3, 0x15c
	ctx.r[31].s64 = ctx.r[3].s64 + 348;
	// 82ECEF7C: 556900BE  clrlwi r9, r11, 2
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82ECEF80: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82ECEF84: 409A0010  bne cr6, 0x82ecef94
	if !ctx.cr[6].eq {
	pc = 0x82ECEF94; continue 'dispatch;
	}
	// 82ECEF88: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82ECEF8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECEF90: 4BFD78F1  bl 0x82ea6880
	ctx.lr = 0x82ECEF94;
	sub_82EA6880(ctx, base);
	// 82ECEF94: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECEF98: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECEF9C: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82ECEFA0: 7FC9512E  stwx r30, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 82ECEFA4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECEFA8: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 82ECEFAC: 911F0004  stw r8, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82ECEFB0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82ECEFB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ECEFB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ECEFBC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82ECEFC0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ECEFC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECEFC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ECEFC8 size=112
    let mut pc: u32 = 0x82ECEFC8;
    'dispatch: loop {
        match pc {
            0x82ECEFC8 => {
    //   block [0x82ECEFC8..0x82ECF038)
	// 82ECEFC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ECEFCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ECEFD0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82ECEFD4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ECEFD8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ECEFDC: 8163017C  lwz r11, 0x17c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(380 as u32) ) } as u64;
	// 82ECEFE0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82ECEFE4: 81430178  lwz r10, 0x178(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(376 as u32) ) } as u64;
	// 82ECEFE8: 3BE30174  addi r31, r3, 0x174
	ctx.r[31].s64 = ctx.r[3].s64 + 372;
	// 82ECEFEC: 556900BE  clrlwi r9, r11, 2
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82ECEFF0: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82ECEFF4: 409A0010  bne cr6, 0x82ecf004
	if !ctx.cr[6].eq {
	pc = 0x82ECF004; continue 'dispatch;
	}
	// 82ECEFF8: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82ECEFFC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECF000: 4BFD7881  bl 0x82ea6880
	ctx.lr = 0x82ECF004;
	sub_82EA6880(ctx, base);
	// 82ECF004: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECF008: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECF00C: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82ECF010: 7FC9512E  stwx r30, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 82ECF014: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECF018: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 82ECF01C: 911F0004  stw r8, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82ECF020: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82ECF024: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ECF028: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ECF02C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82ECF030: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ECF034: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECF038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ECF038 size=112
    let mut pc: u32 = 0x82ECF038;
    'dispatch: loop {
        match pc {
            0x82ECF038 => {
    //   block [0x82ECF038..0x82ECF0A8)
	// 82ECF038: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ECF03C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ECF040: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82ECF044: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ECF048: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ECF04C: 81630128  lwz r11, 0x128(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(296 as u32) ) } as u64;
	// 82ECF050: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82ECF054: 81430124  lwz r10, 0x124(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(292 as u32) ) } as u64;
	// 82ECF058: 3BE30120  addi r31, r3, 0x120
	ctx.r[31].s64 = ctx.r[3].s64 + 288;
	// 82ECF05C: 556900BE  clrlwi r9, r11, 2
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82ECF060: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82ECF064: 409A0010  bne cr6, 0x82ecf074
	if !ctx.cr[6].eq {
	pc = 0x82ECF074; continue 'dispatch;
	}
	// 82ECF068: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82ECF06C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECF070: 4BFD7811  bl 0x82ea6880
	ctx.lr = 0x82ECF074;
	sub_82EA6880(ctx, base);
	// 82ECF074: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECF078: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECF07C: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82ECF080: 7FC9512E  stwx r30, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 82ECF084: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECF088: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 82ECF08C: 911F0004  stw r8, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82ECF090: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82ECF094: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ECF098: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ECF09C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82ECF0A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ECF0A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECF0A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ECF0A8 size=112
    let mut pc: u32 = 0x82ECF0A8;
    'dispatch: loop {
        match pc {
            0x82ECF0A8 => {
    //   block [0x82ECF0A8..0x82ECF118)
	// 82ECF0A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ECF0AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ECF0B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82ECF0B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ECF0B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ECF0BC: 81630188  lwz r11, 0x188(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(392 as u32) ) } as u64;
	// 82ECF0C0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82ECF0C4: 81430184  lwz r10, 0x184(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(388 as u32) ) } as u64;
	// 82ECF0C8: 3BE30180  addi r31, r3, 0x180
	ctx.r[31].s64 = ctx.r[3].s64 + 384;
	// 82ECF0CC: 556900BE  clrlwi r9, r11, 2
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82ECF0D0: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82ECF0D4: 409A0010  bne cr6, 0x82ecf0e4
	if !ctx.cr[6].eq {
	pc = 0x82ECF0E4; continue 'dispatch;
	}
	// 82ECF0D8: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82ECF0DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECF0E0: 4BFD77A1  bl 0x82ea6880
	ctx.lr = 0x82ECF0E4;
	sub_82EA6880(ctx, base);
	// 82ECF0E4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECF0E8: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECF0EC: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82ECF0F0: 7FC9512E  stwx r30, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 82ECF0F4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECF0F8: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 82ECF0FC: 911F0004  stw r8, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82ECF100: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82ECF104: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ECF108: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ECF10C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82ECF110: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ECF114: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECF118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ECF118 size=2508
    let mut pc: u32 = 0x82ECF118;
    'dispatch: loop {
        match pc {
            0x82ECF118 => {
    //   block [0x82ECF118..0x82ECFAE4)
	// 82ECF118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ECF11C: 482D9049  bl 0x831a8164
	ctx.lr = 0x82ECF120;
	sub_831A8130(ctx, base);
	// 82ECF120: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ECF124: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82ECF128: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82ECF12C: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82ECF130: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82ECF134: 388A4520  addi r4, r10, 0x4520
	ctx.r[4].s64 = ctx.r[10].s64 + 17696;
	// 82ECF138: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECF13C: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82ECF140: 813E006C  lwz r9, 0x6c(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(108 as u32) ) } as u64;
	// 82ECF144: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECF148: 810B0004  lwz r8, 4(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECF14C: 913F0004  stw r9, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82ECF150: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82ECF154: 4E800421  bctrl
	ctx.lr = 0x82ECF158;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECF158: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 82ECF15C: 55670000  rlwinm r7, r11, 0, 0, 0
	ctx.r[7].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ECF160: 2F070000  cmpwi cr6, r7, 0
	ctx.cr[6].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82ECF164: 409A0034  bne cr6, 0x82ecf198
	if !ctx.cr[6].eq {
	pc = 0x82ECF198; continue 'dispatch;
	}
	// 82ECF168: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECF16C: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 82ECF170: 80FE002C  lwz r7, 0x2c(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 82ECF174: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82ECF178: 80DE0028  lwz r6, 0x28(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82ECF17C: 38894510  addi r4, r9, 0x4510
	ctx.r[4].s64 = ctx.r[9].s64 + 17680;
	// 82ECF180: 54E7103A  slwi r7, r7, 2
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82ECF184: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82ECF188: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ECF18C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECF190: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82ECF194: 4E800421  bctrl
	ctx.lr = 0x82ECF198;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECF198: 817E002C  lwz r11, 0x2c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 82ECF19C: 37ABFFFF  addic. r29, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82ECF1A0: 41800040  blt 0x82ecf1e0
	if ctx.cr[0].lt {
	pc = 0x82ECF1E0; continue 'dispatch;
	}
	// 82ECF1A4: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82ECF1A8: 57BC103A  slwi r28, r29, 2
	ctx.r[28].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 82ECF1AC: 3B6BFFFC  addi r27, r11, -4
	ctx.r[27].s64 = ctx.r[11].s64 + -4;
	// 82ECF1B0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECF1B4: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82ECF1B8: 815E0028  lwz r10, 0x28(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82ECF1BC: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82ECF1C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECF1C4: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ECF1C8: 7CCAE02E  lwzx r6, r10, r28
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82ECF1CC: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82ECF1D0: 4E800421  bctrl
	ctx.lr = 0x82ECF1D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECF1D4: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82ECF1D8: 3B9CFFFC  addi r28, r28, -4
	ctx.r[28].s64 = ctx.r[28].s64 + -4;
	// 82ECF1DC: 4080FFD4  bge 0x82ecf1b0
	if !ctx.cr[0].lt {
	pc = 0x82ECF1B0; continue 'dispatch;
	}
	// 82ECF1E0: 817E003C  lwz r11, 0x3c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(60 as u32) ) } as u64;
	// 82ECF1E4: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ECF1E8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ECF1EC: 409A0034  bne cr6, 0x82ecf220
	if !ctx.cr[6].eq {
	pc = 0x82ECF220; continue 'dispatch;
	}
	// 82ECF1F0: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECF1F4: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 82ECF1F8: 80FE0038  lwz r7, 0x38(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(56 as u32) ) } as u64;
	// 82ECF1FC: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82ECF200: 80DE0034  lwz r6, 0x34(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 82ECF204: 38894500  addi r4, r9, 0x4500
	ctx.r[4].s64 = ctx.r[9].s64 + 17664;
	// 82ECF208: 54E7103A  slwi r7, r7, 2
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82ECF20C: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82ECF210: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ECF214: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECF218: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82ECF21C: 4E800421  bctrl
	ctx.lr = 0x82ECF220;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECF220: 817E0038  lwz r11, 0x38(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(56 as u32) ) } as u64;
	// 82ECF224: 37ABFFFF  addic. r29, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82ECF228: 41800040  blt 0x82ecf268
	if ctx.cr[0].lt {
	pc = 0x82ECF268; continue 'dispatch;
	}
	// 82ECF22C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82ECF230: 57BC103A  slwi r28, r29, 2
	ctx.r[28].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 82ECF234: 3B6B00E8  addi r27, r11, 0xe8
	ctx.r[27].s64 = ctx.r[11].s64 + 232;
	// 82ECF238: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECF23C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82ECF240: 815E0034  lwz r10, 0x34(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 82ECF244: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82ECF248: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECF24C: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ECF250: 7CCAE02E  lwzx r6, r10, r28
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82ECF254: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82ECF258: 4E800421  bctrl
	ctx.lr = 0x82ECF25C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECF25C: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82ECF260: 3B9CFFFC  addi r28, r28, -4
	ctx.r[28].s64 = ctx.r[28].s64 + -4;
	// 82ECF264: 4080FFD4  bge 0x82ecf238
	if !ctx.cr[0].lt {
	pc = 0x82ECF238; continue 'dispatch;
	}
	// 82ECF268: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECF26C: 3D408207  lis r10, -0x7df9
	ctx.r[10].s64 = -2113470464;
	// 82ECF270: 80DE0020  lwz r6, 0x20(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 82ECF274: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82ECF278: 388A2D48  addi r4, r10, 0x2d48
	ctx.r[4].s64 = ctx.r[10].s64 + 11592;
	// 82ECF27C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECF280: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ECF284: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82ECF288: 4E800421  bctrl
	ctx.lr = 0x82ECF28C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECF28C: 817E00EC  lwz r11, 0xec(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(236 as u32) ) } as u64;
	// 82ECF290: 55680000  rlwinm r8, r11, 0, 0, 0
	ctx.r[8].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ECF294: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82ECF298: 409A0034  bne cr6, 0x82ecf2cc
	if !ctx.cr[6].eq {
	pc = 0x82ECF2CC; continue 'dispatch;
	}
	// 82ECF29C: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECF2A0: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 82ECF2A4: 80FE00E8  lwz r7, 0xe8(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(232 as u32) ) } as u64;
	// 82ECF2A8: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82ECF2AC: 80DE00E4  lwz r6, 0xe4(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(228 as u32) ) } as u64;
	// 82ECF2B0: 388944F4  addi r4, r9, 0x44f4
	ctx.r[4].s64 = ctx.r[9].s64 + 17652;
	// 82ECF2B4: 54E7103A  slwi r7, r7, 2
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82ECF2B8: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82ECF2BC: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ECF2C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECF2C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82ECF2C8: 4E800421  bctrl
	ctx.lr = 0x82ECF2CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECF2CC: 817E00E8  lwz r11, 0xe8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(232 as u32) ) } as u64;
	// 82ECF2D0: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82ECF2D4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ECF2D8: 40990048  ble cr6, 0x82ecf320
	if !ctx.cr[6].gt {
	pc = 0x82ECF320; continue 'dispatch;
	}
	// 82ECF2DC: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82ECF2E0: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82ECF2E4: 3B6B44E8  addi r27, r11, 0x44e8
	ctx.r[27].s64 = ctx.r[11].s64 + 17640;
	// 82ECF2E8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECF2EC: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82ECF2F0: 815E00E4  lwz r10, 0xe4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(228 as u32) ) } as u64;
	// 82ECF2F4: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82ECF2F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECF2FC: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ECF300: 7CDD502E  lwzx r6, r29, r10
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82ECF304: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82ECF308: 4E800421  bctrl
	ctx.lr = 0x82ECF30C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECF30C: 811E00E8  lwz r8, 0xe8(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(232 as u32) ) } as u64;
	// 82ECF310: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82ECF314: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82ECF318: 7F1C4000  cmpw cr6, r28, r8
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82ECF31C: 4198FFCC  blt cr6, 0x82ecf2e8
	if ctx.cr[6].lt {
	pc = 0x82ECF2E8; continue 'dispatch;
	}
	// 82ECF320: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECF324: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82ECF328: 80DE00A8  lwz r6, 0xa8(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(168 as u32) ) } as u64;
	// 82ECF32C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82ECF330: 388A44C8  addi r4, r10, 0x44c8
	ctx.r[4].s64 = ctx.r[10].s64 + 17608;
	// 82ECF334: 38E00038  li r7, 0x38
	ctx.r[7].s64 = 56;
	// 82ECF338: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82ECF33C: 812B0008  lwz r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ECF340: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECF344: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82ECF348: 4E800421  bctrl
	ctx.lr = 0x82ECF34C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECF34C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECF350: 3CE08212  lis r7, -0x7dee
	ctx.r[7].s64 = -2112749568;
	// 82ECF354: 80DE00AC  lwz r6, 0xac(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(172 as u32) ) } as u64;
	// 82ECF358: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82ECF35C: 388744BC  addi r4, r7, 0x44bc
	ctx.r[4].s64 = ctx.r[7].s64 + 17596;
	// 82ECF360: 38E00038  li r7, 0x38
	ctx.r[7].s64 = 56;
	// 82ECF364: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82ECF368: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ECF36C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECF370: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ECF374: 4E800421  bctrl
	ctx.lr = 0x82ECF378;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECF378: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECF37C: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 82ECF380: 80DE00B0  lwz r6, 0xb0(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(176 as u32) ) } as u64;
	// 82ECF384: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82ECF388: 3889449C  addi r4, r9, 0x449c
	ctx.r[4].s64 = ctx.r[9].s64 + 17564;
	// 82ECF38C: 38E00038  li r7, 0x38
	ctx.r[7].s64 = 56;
	// 82ECF390: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82ECF394: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ECF398: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECF39C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ECF3A0: 4E800421  bctrl
	ctx.lr = 0x82ECF3A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECF3A4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECF3A8: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 82ECF3AC: 80DE00B4  lwz r6, 0xb4(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(180 as u32) ) } as u64;
	// 82ECF3B0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82ECF3B4: 38894488  addi r4, r9, 0x4488
	ctx.r[4].s64 = ctx.r[9].s64 + 17544;
	// 82ECF3B8: 38E00038  li r7, 0x38
	ctx.r[7].s64 = 56;
	// 82ECF3BC: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82ECF3C0: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ECF3C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECF3C8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ECF3CC: 4E800421  bctrl
	ctx.lr = 0x82ECF3D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECF3D0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECF3D4: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 82ECF3D8: 80DE0068  lwz r6, 0x68(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) } as u64;
	// 82ECF3DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82ECF3E0: 38894468  addi r4, r9, 0x4468
	ctx.r[4].s64 = ctx.r[9].s64 + 17512;
	// 82ECF3E4: 38E00014  li r7, 0x14
	ctx.r[7].s64 = 20;
	// 82ECF3E8: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82ECF3EC: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ECF3F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECF3F4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ECF3F8: 4E800421  bctrl
	ctx.lr = 0x82ECF3FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECF3FC: 817E0090  lwz r11, 0x90(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(144 as u32) ) } as u64;
	// 82ECF400: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ECF404: 419A00B0  beq cr6, 0x82ecf4b4
	if ctx.cr[6].eq {
	pc = 0x82ECF4B4; continue 'dispatch;
	}
	// 82ECF408: 7D7B5B78  mr r27, r11
	ctx.r[27].u64 = ctx.r[11].u64;
	// 82ECF40C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82ECF410: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82ECF414: 3BAB444C  addi r29, r11, 0x444c
	ctx.r[29].s64 = ctx.r[11].s64 + 17484;
	// 82ECF418: 3B8A4434  addi r28, r10, 0x4434
	ctx.r[28].s64 = ctx.r[10].s64 + 17460;
	// 82ECF41C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECF420: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82ECF424: 80DE0090  lwz r6, 0x90(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(144 as u32) ) } as u64;
	// 82ECF428: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82ECF42C: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 82ECF430: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82ECF434: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECF438: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ECF43C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ECF440: 4E800421  bctrl
	ctx.lr = 0x82ECF444;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECF444: 817E0090  lwz r11, 0x90(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(144 as u32) ) } as u64;
	// 82ECF448: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECF44C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82ECF450: 419A0058  beq cr6, 0x82ecf4a8
	if ctx.cr[6].eq {
	pc = 0x82ECF4A8; continue 'dispatch;
	}
	// 82ECF454: 552A003E  slwi r10, r9, 0
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82ECF458: 812A0008  lwz r9, 8(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ECF45C: 552B0000  rlwinm r11, r9, 0, 0, 0
	ctx.r[11].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 82ECF460: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ECF464: 409A0044  bne cr6, 0x82ecf4a8
	if !ctx.cr[6].eq {
	pc = 0x82ECF4A8; continue 'dispatch;
	}
	// 82ECF468: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECF46C: 552800BE  clrlwi r8, r9, 2
	ctx.r[8].u64 = ctx.r[9].u32 as u64 & 0x3FFFFFFFu64;
	// 82ECF470: 80BF0000  lwz r5, 0(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECF474: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82ECF478: 5567103A  slwi r7, r11, 2
	ctx.r[7].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82ECF47C: 80CA0000  lwz r6, 0(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECF480: 7C684A14  add r3, r8, r9
	ctx.r[3].u64 = ctx.r[8].u64 + ctx.r[9].u64;
	// 82ECF484: 7D6B3A14  add r11, r11, r7
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 82ECF488: 5468103A  slwi r8, r3, 2
	ctx.r[8].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82ECF48C: 81450008  lwz r10, 8(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ECF490: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82ECF494: 5567103A  slwi r7, r11, 2
	ctx.r[7].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82ECF498: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82ECF49C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECF4A0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ECF4A4: 4E800421  bctrl
	ctx.lr = 0x82ECF4A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECF4A8: 837B0008  lwz r27, 8(r27)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ECF4AC: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82ECF4B0: 409AFF6C  bne cr6, 0x82ecf41c
	if !ctx.cr[6].eq {
	pc = 0x82ECF41C; continue 'dispatch;
	}
	// 82ECF4B4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECF4B8: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82ECF4BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECF4C0: 388A4428  addi r4, r10, 0x4428
	ctx.r[4].s64 = ctx.r[10].s64 + 17448;
	// 82ECF4C4: 812B0010  lwz r9, 0x10(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82ECF4C8: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82ECF4CC: 4E800421  bctrl
	ctx.lr = 0x82ECF4D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECF4D0: 80FF0000  lwz r7, 0(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECF4D4: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 82ECF4D8: 80DE0008  lwz r6, 8(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ECF4DC: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82ECF4E0: 3888441C  addi r4, r8, 0x441c
	ctx.r[4].s64 = ctx.r[8].s64 + 17436;
	// 82ECF4E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECF4E8: 8167000C  lwz r11, 0xc(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ECF4EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82ECF4F0: 4E800421  bctrl
	ctx.lr = 0x82ECF4F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECF4F4: 817E0048  lwz r11, 0x48(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(72 as u32) ) } as u64;
	// 82ECF4F8: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ECF4FC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ECF500: 409A0034  bne cr6, 0x82ecf534
	if !ctx.cr[6].eq {
	pc = 0x82ECF534; continue 'dispatch;
	}
	// 82ECF504: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECF508: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 82ECF50C: 80FE0044  lwz r7, 0x44(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(68 as u32) ) } as u64;
	// 82ECF510: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82ECF514: 80DE0040  lwz r6, 0x40(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(64 as u32) ) } as u64;
	// 82ECF518: 38894410  addi r4, r9, 0x4410
	ctx.r[4].s64 = ctx.r[9].s64 + 17424;
	// 82ECF51C: 54E7103A  slwi r7, r7, 2
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82ECF520: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82ECF524: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ECF528: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECF52C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82ECF530: 4E800421  bctrl
	ctx.lr = 0x82ECF534;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECF534: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECF538: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82ECF53C: 80DE004C  lwz r6, 0x4c(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(76 as u32) ) } as u64;
	// 82ECF540: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82ECF544: 388A4404  addi r4, r10, 0x4404
	ctx.r[4].s64 = ctx.r[10].s64 + 17412;
	// 82ECF548: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECF54C: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ECF550: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82ECF554: 4E800421  bctrl
	ctx.lr = 0x82ECF558;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECF558: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECF55C: 3CE08212  lis r7, -0x7dee
	ctx.r[7].s64 = -2112749568;
	// 82ECF560: 80DE007C  lwz r6, 0x7c(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(124 as u32) ) } as u64;
	// 82ECF564: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82ECF568: 3BA743FC  addi r29, r7, 0x43fc
	ctx.r[29].s64 = ctx.r[7].s64 + 17404;
	// 82ECF56C: 38E00058  li r7, 0x58
	ctx.r[7].s64 = 88;
	// 82ECF570: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ECF574: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82ECF578: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82ECF57C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECF580: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ECF584: 4E800421  bctrl
	ctx.lr = 0x82ECF588;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECF588: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECF58C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82ECF590: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECF594: 81090010  lwz r8, 0x10(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(16 as u32) ) } as u64;
	// 82ECF598: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82ECF59C: 4E800421  bctrl
	ctx.lr = 0x82ECF5A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECF5A0: 817E007C  lwz r11, 0x7c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(124 as u32) ) } as u64;
	// 82ECF5A4: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ECF5A8: 55470000  rlwinm r7, r10, 0, 0, 0
	ctx.r[7].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82ECF5AC: 2F070000  cmpwi cr6, r7, 0
	ctx.cr[6].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82ECF5B0: 409A0048  bne cr6, 0x82ecf5f8
	if !ctx.cr[6].eq {
	pc = 0x82ECF5F8; continue 'dispatch;
	}
	// 82ECF5B4: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ECF5B8: 554800BE  clrlwi r8, r10, 2
	ctx.r[8].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 82ECF5BC: 80BF0000  lwz r5, 0(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECF5C0: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82ECF5C4: 5527103A  slwi r7, r9, 2
	ctx.r[7].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82ECF5C8: 80CB0000  lwz r6, 0(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECF5CC: 7C885214  add r4, r8, r10
	ctx.r[4].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 82ECF5D0: 7C693A14  add r3, r9, r7
	ctx.r[3].u64 = ctx.r[9].u64 + ctx.r[7].u64;
	// 82ECF5D4: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82ECF5D8: 81450008  lwz r10, 8(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ECF5DC: 5488103A  slwi r8, r4, 2
	ctx.r[8].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82ECF5E0: 5467103A  slwi r7, r3, 2
	ctx.r[7].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82ECF5E4: 388B43F0  addi r4, r11, 0x43f0
	ctx.r[4].s64 = ctx.r[11].s64 + 17392;
	// 82ECF5E8: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82ECF5EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECF5F0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ECF5F4: 4E800421  bctrl
	ctx.lr = 0x82ECF5F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECF5F8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECF5FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECF600: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82ECF604: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ECF608: 4E800421  bctrl
	ctx.lr = 0x82ECF60C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECF60C: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECF610: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 82ECF614: 80DE018C  lwz r6, 0x18c(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(396 as u32) ) } as u64;
	// 82ECF618: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82ECF61C: 388943DC  addi r4, r9, 0x43dc
	ctx.r[4].s64 = ctx.r[9].s64 + 17372;
	// 82ECF620: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECF624: 80E8000C  lwz r7, 0xc(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ECF628: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 82ECF62C: 4E800421  bctrl
	ctx.lr = 0x82ECF630;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECF630: 80BF0000  lwz r5, 0(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECF634: 3CC08212  lis r6, -0x7dee
	ctx.r[6].s64 = -2112749568;
	// 82ECF638: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECF63C: 388643D4  addi r4, r6, 0x43d4
	ctx.r[4].s64 = ctx.r[6].s64 + 17364;
	// 82ECF640: 81650010  lwz r11, 0x10(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(16 as u32) ) } as u64;
	// 82ECF644: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82ECF648: 4E800421  bctrl
	ctx.lr = 0x82ECF64C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECF64C: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECF650: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82ECF654: 80DE0054  lwz r6, 0x54(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(84 as u32) ) } as u64;
	// 82ECF658: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82ECF65C: 388A43C8  addi r4, r10, 0x43c8
	ctx.r[4].s64 = ctx.r[10].s64 + 17352;
	// 82ECF660: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECF664: 8109000C  lwz r8, 0xc(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ECF668: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82ECF66C: 4E800421  bctrl
	ctx.lr = 0x82ECF670;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECF670: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECF674: 3CE08212  lis r7, -0x7dee
	ctx.r[7].s64 = -2112749568;
	// 82ECF678: 80DE0058  lwz r6, 0x58(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(88 as u32) ) } as u64;
	// 82ECF67C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82ECF680: 3BA743BC  addi r29, r7, 0x43bc
	ctx.r[29].s64 = ctx.r[7].s64 + 17340;
	// 82ECF684: 38E0010C  li r7, 0x10c
	ctx.r[7].s64 = 268;
	// 82ECF688: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ECF68C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82ECF690: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82ECF694: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECF698: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ECF69C: 4E800421  bctrl
	ctx.lr = 0x82ECF6A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECF6A0: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECF6A4: 80DE0060  lwz r6, 0x60(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(96 as u32) ) } as u64;
	// 82ECF6A8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82ECF6AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82ECF6B0: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 82ECF6B4: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82ECF6B8: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ECF6BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECF6C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82ECF6C4: 4E800421  bctrl
	ctx.lr = 0x82ECF6C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECF6C8: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECF6CC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82ECF6D0: 80DE005C  lwz r6, 0x5c(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(92 as u32) ) } as u64;
	// 82ECF6D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82ECF6D8: 38E0000C  li r7, 0xc
	ctx.r[7].s64 = 12;
	// 82ECF6DC: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82ECF6E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECF6E4: 812A0008  lwz r9, 8(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ECF6E8: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82ECF6EC: 4E800421  bctrl
	ctx.lr = 0x82ECF6F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECF6F0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECF6F4: 80DE0064  lwz r6, 0x64(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(100 as u32) ) } as u64;
	// 82ECF6F8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82ECF6FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82ECF700: 38E0000C  li r7, 0xc
	ctx.r[7].s64 = 12;
	// 82ECF704: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82ECF708: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ECF70C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECF710: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ECF714: 4E800421  bctrl
	ctx.lr = 0x82ECF718;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECF718: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECF71C: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 82ECF720: 80DE006C  lwz r6, 0x6c(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(108 as u32) ) } as u64;
	// 82ECF724: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82ECF728: 388943B0  addi r4, r9, 0x43b0
	ctx.r[4].s64 = ctx.r[9].s64 + 17328;
	// 82ECF72C: 38E00070  li r7, 0x70
	ctx.r[7].s64 = 112;
	// 82ECF730: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82ECF734: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ECF738: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECF73C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ECF740: 4E800421  bctrl
	ctx.lr = 0x82ECF744;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECF744: 817E006C  lwz r11, 0x6c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(108 as u32) ) } as u64;
	// 82ECF748: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ECF74C: 419A0030  beq cr6, 0x82ecf77c
	if ctx.cr[6].eq {
	pc = 0x82ECF77C; continue 'dispatch;
	}
	// 82ECF750: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECF754: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 82ECF758: 80CB006C  lwz r6, 0x6c(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 82ECF75C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82ECF760: 38894398  addi r4, r9, 0x4398
	ctx.r[4].s64 = ctx.r[9].s64 + 17304;
	// 82ECF764: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 82ECF768: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82ECF76C: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ECF770: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECF774: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82ECF778: 4E800421  bctrl
	ctx.lr = 0x82ECF77C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECF77C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECF780: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82ECF784: 80DE0070  lwz r6, 0x70(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(112 as u32) ) } as u64;
	// 82ECF788: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82ECF78C: 388A4390  addi r4, r10, 0x4390
	ctx.r[4].s64 = ctx.r[10].s64 + 17296;
	// 82ECF790: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECF794: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ECF798: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82ECF79C: 4E800421  bctrl
	ctx.lr = 0x82ECF7A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECF7A0: 80FF0000  lwz r7, 0(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECF7A4: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 82ECF7A8: 80DE0078  lwz r6, 0x78(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(120 as u32) ) } as u64;
	// 82ECF7AC: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82ECF7B0: 3888437C  addi r4, r8, 0x437c
	ctx.r[4].s64 = ctx.r[8].s64 + 17276;
	// 82ECF7B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECF7B8: 8167000C  lwz r11, 0xc(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ECF7BC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82ECF7C0: 4E800421  bctrl
	ctx.lr = 0x82ECF7C4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECF7C4: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECF7C8: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82ECF7CC: 80DE0074  lwz r6, 0x74(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(116 as u32) ) } as u64;
	// 82ECF7D0: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82ECF7D4: 388A4370  addi r4, r10, 0x4370
	ctx.r[4].s64 = ctx.r[10].s64 + 17264;
	// 82ECF7D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECF7DC: 8109000C  lwz r8, 0xc(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 82ECF7E0: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82ECF7E4: 4E800421  bctrl
	ctx.lr = 0x82ECF7E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECF7E8: 80FF0000  lwz r7, 0(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECF7EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECF7F0: 80C70014  lwz r6, 0x14(r7)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(20 as u32) ) } as u64;
	// 82ECF7F4: 7CC903A6  mtctr r6
	ctx.ctr.u64 = ctx.r[6].u64;
	// 82ECF7F8: 4E800421  bctrl
	ctx.lr = 0x82ECF7FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECF7FC: 80BF0000  lwz r5, 0(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECF800: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECF804: 80850014  lwz r4, 0x14(r5)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(20 as u32) ) } as u64;
	// 82ECF808: 7C8903A6  mtctr r4
	ctx.ctr.u64 = ctx.r[4].u64;
	// 82ECF80C: 4E800421  bctrl
	ctx.lr = 0x82ECF810;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECF810: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECF814: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82ECF818: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECF81C: 388B4364  addi r4, r11, 0x4364
	ctx.r[4].s64 = ctx.r[11].s64 + 17252;
	// 82ECF820: 812A0010  lwz r9, 0x10(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82ECF824: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82ECF828: 4E800421  bctrl
	ctx.lr = 0x82ECF82C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECF82C: 817E0104  lwz r11, 0x104(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(260 as u32) ) } as u64;
	// 82ECF830: 55680000  rlwinm r8, r11, 0, 0, 0
	ctx.r[8].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ECF834: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82ECF838: 409A0034  bne cr6, 0x82ecf86c
	if !ctx.cr[6].eq {
	pc = 0x82ECF86C; continue 'dispatch;
	}
	// 82ECF83C: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECF840: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 82ECF844: 80FE0100  lwz r7, 0x100(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(256 as u32) ) } as u64;
	// 82ECF848: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82ECF84C: 80DE00FC  lwz r6, 0xfc(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(252 as u32) ) } as u64;
	// 82ECF850: 38894358  addi r4, r9, 0x4358
	ctx.r[4].s64 = ctx.r[9].s64 + 17240;
	// 82ECF854: 54E7103A  slwi r7, r7, 2
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82ECF858: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82ECF85C: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ECF860: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECF864: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82ECF868: 4E800421  bctrl
	ctx.lr = 0x82ECF86C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECF86C: 817E0110  lwz r11, 0x110(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(272 as u32) ) } as u64;
	// 82ECF870: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ECF874: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ECF878: 409A0034  bne cr6, 0x82ecf8ac
	if !ctx.cr[6].eq {
	pc = 0x82ECF8AC; continue 'dispatch;
	}
	// 82ECF87C: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECF880: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 82ECF884: 80FE010C  lwz r7, 0x10c(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(268 as u32) ) } as u64;
	// 82ECF888: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82ECF88C: 80DE0108  lwz r6, 0x108(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(264 as u32) ) } as u64;
	// 82ECF890: 3889434C  addi r4, r9, 0x434c
	ctx.r[4].s64 = ctx.r[9].s64 + 17228;
	// 82ECF894: 54E7103A  slwi r7, r7, 2
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82ECF898: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82ECF89C: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ECF8A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECF8A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82ECF8A8: 4E800421  bctrl
	ctx.lr = 0x82ECF8AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECF8AC: 817E011C  lwz r11, 0x11c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(284 as u32) ) } as u64;
	// 82ECF8B0: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ECF8B4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ECF8B8: 409A0034  bne cr6, 0x82ecf8ec
	if !ctx.cr[6].eq {
	pc = 0x82ECF8EC; continue 'dispatch;
	}
	// 82ECF8BC: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECF8C0: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 82ECF8C4: 80FE0118  lwz r7, 0x118(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(280 as u32) ) } as u64;
	// 82ECF8C8: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82ECF8CC: 80DE0114  lwz r6, 0x114(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(276 as u32) ) } as u64;
	// 82ECF8D0: 3889433C  addi r4, r9, 0x433c
	ctx.r[4].s64 = ctx.r[9].s64 + 17212;
	// 82ECF8D4: 54E7103A  slwi r7, r7, 2
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82ECF8D8: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82ECF8DC: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ECF8E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECF8E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82ECF8E8: 4E800421  bctrl
	ctx.lr = 0x82ECF8EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECF8EC: 817E0128  lwz r11, 0x128(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(296 as u32) ) } as u64;
	// 82ECF8F0: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ECF8F4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ECF8F8: 409A0034  bne cr6, 0x82ecf92c
	if !ctx.cr[6].eq {
	pc = 0x82ECF92C; continue 'dispatch;
	}
	// 82ECF8FC: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECF900: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 82ECF904: 80FE0124  lwz r7, 0x124(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(292 as u32) ) } as u64;
	// 82ECF908: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82ECF90C: 80DE0120  lwz r6, 0x120(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(288 as u32) ) } as u64;
	// 82ECF910: 3889432C  addi r4, r9, 0x432c
	ctx.r[4].s64 = ctx.r[9].s64 + 17196;
	// 82ECF914: 54E7103A  slwi r7, r7, 2
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82ECF918: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82ECF91C: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ECF920: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECF924: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82ECF928: 4E800421  bctrl
	ctx.lr = 0x82ECF92C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECF92C: 817E0134  lwz r11, 0x134(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(308 as u32) ) } as u64;
	// 82ECF930: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ECF934: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ECF938: 409A0034  bne cr6, 0x82ecf96c
	if !ctx.cr[6].eq {
	pc = 0x82ECF96C; continue 'dispatch;
	}
	// 82ECF93C: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECF940: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 82ECF944: 80FE0130  lwz r7, 0x130(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(304 as u32) ) } as u64;
	// 82ECF948: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82ECF94C: 80DE012C  lwz r6, 0x12c(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(300 as u32) ) } as u64;
	// 82ECF950: 3889431C  addi r4, r9, 0x431c
	ctx.r[4].s64 = ctx.r[9].s64 + 17180;
	// 82ECF954: 54E7103A  slwi r7, r7, 2
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82ECF958: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82ECF95C: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ECF960: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECF964: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82ECF968: 4E800421  bctrl
	ctx.lr = 0x82ECF96C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECF96C: 817E0140  lwz r11, 0x140(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(320 as u32) ) } as u64;
	// 82ECF970: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ECF974: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ECF978: 409A0034  bne cr6, 0x82ecf9ac
	if !ctx.cr[6].eq {
	pc = 0x82ECF9AC; continue 'dispatch;
	}
	// 82ECF97C: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECF980: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 82ECF984: 80FE013C  lwz r7, 0x13c(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(316 as u32) ) } as u64;
	// 82ECF988: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82ECF98C: 80DE0138  lwz r6, 0x138(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(312 as u32) ) } as u64;
	// 82ECF990: 3889430C  addi r4, r9, 0x430c
	ctx.r[4].s64 = ctx.r[9].s64 + 17164;
	// 82ECF994: 54E7103A  slwi r7, r7, 2
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82ECF998: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82ECF99C: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ECF9A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECF9A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82ECF9A8: 4E800421  bctrl
	ctx.lr = 0x82ECF9AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECF9AC: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82ECF9B0: 3BAB42FC  addi r29, r11, 0x42fc
	ctx.r[29].s64 = ctx.r[11].s64 + 17148;
	// 82ECF9B4: 817E014C  lwz r11, 0x14c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(332 as u32) ) } as u64;
	// 82ECF9B8: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ECF9BC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ECF9C0: 409A0030  bne cr6, 0x82ecf9f0
	if !ctx.cr[6].eq {
	pc = 0x82ECF9F0; continue 'dispatch;
	}
	// 82ECF9C4: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECF9C8: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82ECF9CC: 813E0148  lwz r9, 0x148(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(328 as u32) ) } as u64;
	// 82ECF9D0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82ECF9D4: 80DE0144  lwz r6, 0x144(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(324 as u32) ) } as u64;
	// 82ECF9D8: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82ECF9DC: 5527103A  slwi r7, r9, 2
	ctx.r[7].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82ECF9E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECF9E4: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ECF9E8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82ECF9EC: 4E800421  bctrl
	ctx.lr = 0x82ECF9F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECF9F0: 817E0158  lwz r11, 0x158(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(344 as u32) ) } as u64;
	// 82ECF9F4: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ECF9F8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ECF9FC: 409A0034  bne cr6, 0x82ecfa30
	if !ctx.cr[6].eq {
	pc = 0x82ECFA30; continue 'dispatch;
	}
	// 82ECFA00: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECFA04: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 82ECFA08: 80FE0154  lwz r7, 0x154(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(340 as u32) ) } as u64;
	// 82ECFA0C: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82ECFA10: 80DE0150  lwz r6, 0x150(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(336 as u32) ) } as u64;
	// 82ECFA14: 388942EC  addi r4, r9, 0x42ec
	ctx.r[4].s64 = ctx.r[9].s64 + 17132;
	// 82ECFA18: 54E7103A  slwi r7, r7, 2
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82ECFA1C: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82ECFA20: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ECFA24: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECFA28: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82ECFA2C: 4E800421  bctrl
	ctx.lr = 0x82ECFA30;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECFA30: 817E0164  lwz r11, 0x164(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(356 as u32) ) } as u64;
	// 82ECFA34: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ECFA38: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ECFA3C: 409A0030  bne cr6, 0x82ecfa6c
	if !ctx.cr[6].eq {
	pc = 0x82ECFA6C; continue 'dispatch;
	}
	// 82ECFA40: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECFA44: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82ECFA48: 813E0160  lwz r9, 0x160(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(352 as u32) ) } as u64;
	// 82ECFA4C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82ECFA50: 80DE015C  lwz r6, 0x15c(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(348 as u32) ) } as u64;
	// 82ECFA54: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82ECFA58: 5527103A  slwi r7, r9, 2
	ctx.r[7].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82ECFA5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECFA60: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ECFA64: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82ECFA68: 4E800421  bctrl
	ctx.lr = 0x82ECFA6C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECFA6C: 817E017C  lwz r11, 0x17c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(380 as u32) ) } as u64;
	// 82ECFA70: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82ECFA74: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82ECFA78: 409A0034  bne cr6, 0x82ecfaac
	if !ctx.cr[6].eq {
	pc = 0x82ECFAAC; continue 'dispatch;
	}
	// 82ECFA7C: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECFA80: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 82ECFA84: 80FE0178  lwz r7, 0x178(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(376 as u32) ) } as u64;
	// 82ECFA88: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82ECFA8C: 80DE0174  lwz r6, 0x174(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(372 as u32) ) } as u64;
	// 82ECFA90: 388942E0  addi r4, r9, 0x42e0
	ctx.r[4].s64 = ctx.r[9].s64 + 17120;
	// 82ECFA94: 54E7103A  slwi r7, r7, 2
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82ECFA98: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82ECFA9C: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ECFAA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECFAA4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82ECFAA8: 4E800421  bctrl
	ctx.lr = 0x82ECFAAC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECFAAC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECFAB0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECFAB4: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82ECFAB8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ECFABC: 4E800421  bctrl
	ctx.lr = 0x82ECFAC0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECFAC0: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECFAC4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ECFAC8: 81090018  lwz r8, 0x18(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(24 as u32) ) } as u64;
	// 82ECFACC: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82ECFAD0: 4E800421  bctrl
	ctx.lr = 0x82ECFAD4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ECFAD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82ECFAD8: 90FF0004  stw r7, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82ECFADC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82ECFAE0: 482D86D4  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ECFAE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82ECFAE8 size=4672
    //   switch @ 0x82ED0000: r9 with 5 label(s)
    //       case  0 → 0x82ED0018
    //       case  1 → 0x82ED0018
    //       case  2 → 0x82ED0024
    //       case  3 → 0x82ED0030
    //       case  4 → 0x82ED003C
    let mut pc: u32 = 0x82ECFAE8;
    'dispatch: loop {
        match pc {
            0x82ECFAE8 => {
    //   block [0x82ECFAE8..0x82ED0018)
	// 82ECFAE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ECFAEC: 482D8645  bl 0x831a8130
	ctx.lr = 0x82ECFAF0;
	sub_831A8130(ctx, base);
	// 82ECFAF0: 3981FF68  addi r12, r1, -0x98
	ctx.r[12].s64 = ctx.r[1].s64 + -152;
	// 82ECFAF4: 482D8F71  bl 0x831a8a64
	ctx.lr = 0x82ECFAF8;
	sub_831A8A40(ctx, base);
	// 82ECFAF8: 9421FBC0  stwu r1, -0x440(r1)
	ea = ctx.r[1].u32.wrapping_add(-1088 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ECFAFC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82ECFB00: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECFB04: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82ECFB08: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82ECFB0C: 392B45A8  addi r9, r11, 0x45a8
	ctx.r[9].s64 = ctx.r[11].s64 + 17832;
	// 82ECFB10: 39C00001  li r14, 1
	ctx.r[14].s64 = 1;
	// 82ECFB14: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 82ECFB18: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82ECFB1C: B1DF0006  sth r14, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[14].u16 ) };
	// 82ECFB20: 38E0FFD1  li r7, -0x2f
	ctx.r[7].s64 = -47;
	// 82ECFB24: 93DF0028  stw r30, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[30].u32 ) };
	// 82ECFB28: 393F0028  addi r9, r31, 0x28
	ctx.r[9].s64 = ctx.r[31].s64 + 40;
	// 82ECFB2C: 93DF002C  stw r30, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[30].u32 ) };
	// 82ECFB30: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82ECFB34: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 82ECFB38: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 82ECFB3C: 93DF0034  stw r30, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[30].u32 ) };
	// 82ECFB40: 7E495214  add r18, r9, r10
	ctx.r[18].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82ECFB44: 93DF0038  stw r30, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[30].u32 ) };
	// 82ECFB48: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82ECFB4C: 917F003C  stw r11, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 82ECFB50: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82ECFB54: 93DF0040  stw r30, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[30].u32 ) };
	// 82ECFB58: 391F0098  addi r8, r31, 0x98
	ctx.r[8].s64 = ctx.r[31].s64 + 152;
	// 82ECFB5C: 93DF0044  stw r30, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[30].u32 ) };
	// 82ECFB60: 917F0048  stw r11, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 82ECFB64: 90FF0098  stw r7, 0x98(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(152 as u32), ctx.r[7].u32 ) };
	// 82ECFB68: B3DF009C  sth r30, 0x9c(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(156 as u32), ctx.r[30].u16 ) };
	// 82ECFB6C: 93DF00E4  stw r30, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[30].u32 ) };
	// 82ECFB70: 93DF00E8  stw r30, 0xe8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(232 as u32), ctx.r[30].u32 ) };
	// 82ECFB74: 917F00EC  stw r11, 0xec(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(236 as u32), ctx.r[11].u32 ) };
	// 82ECFB78: 93DF00F0  stw r30, 0xf0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(240 as u32), ctx.r[30].u32 ) };
	// 82ECFB7C: 93DF00F4  stw r30, 0xf4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(244 as u32), ctx.r[30].u32 ) };
	// 82ECFB80: 917F00F8  stw r11, 0xf8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(248 as u32), ctx.r[11].u32 ) };
	// 82ECFB84: 93DF00FC  stw r30, 0xfc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(252 as u32), ctx.r[30].u32 ) };
	// 82ECFB88: 93DF0100  stw r30, 0x100(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(256 as u32), ctx.r[30].u32 ) };
	// 82ECFB8C: 917F0104  stw r11, 0x104(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(260 as u32), ctx.r[11].u32 ) };
	// 82ECFB90: 93DF0108  stw r30, 0x108(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(264 as u32), ctx.r[30].u32 ) };
	// 82ECFB94: 93DF010C  stw r30, 0x10c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(268 as u32), ctx.r[30].u32 ) };
	// 82ECFB98: 917F0110  stw r11, 0x110(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(272 as u32), ctx.r[11].u32 ) };
	// 82ECFB9C: 93DF0114  stw r30, 0x114(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(276 as u32), ctx.r[30].u32 ) };
	// 82ECFBA0: 93DF0118  stw r30, 0x118(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(280 as u32), ctx.r[30].u32 ) };
	// 82ECFBA4: 917F011C  stw r11, 0x11c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(284 as u32), ctx.r[11].u32 ) };
	// 82ECFBA8: 93DF0120  stw r30, 0x120(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(288 as u32), ctx.r[30].u32 ) };
	// 82ECFBAC: 93DF0124  stw r30, 0x124(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(292 as u32), ctx.r[30].u32 ) };
	// 82ECFBB0: 917F0128  stw r11, 0x128(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(296 as u32), ctx.r[11].u32 ) };
	// 82ECFBB4: 93DF012C  stw r30, 0x12c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(300 as u32), ctx.r[30].u32 ) };
	// 82ECFBB8: 93DF0130  stw r30, 0x130(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(304 as u32), ctx.r[30].u32 ) };
	// 82ECFBBC: 917F0134  stw r11, 0x134(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(308 as u32), ctx.r[11].u32 ) };
	// 82ECFBC0: 93DF0138  stw r30, 0x138(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(312 as u32), ctx.r[30].u32 ) };
	// 82ECFBC4: 93DF013C  stw r30, 0x13c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(316 as u32), ctx.r[30].u32 ) };
	// 82ECFBC8: 917F0140  stw r11, 0x140(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(320 as u32), ctx.r[11].u32 ) };
	// 82ECFBCC: 93DF0144  stw r30, 0x144(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(324 as u32), ctx.r[30].u32 ) };
	// 82ECFBD0: 93DF0148  stw r30, 0x148(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(328 as u32), ctx.r[30].u32 ) };
	// 82ECFBD4: 917F014C  stw r11, 0x14c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(332 as u32), ctx.r[11].u32 ) };
	// 82ECFBD8: 93DF0150  stw r30, 0x150(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(336 as u32), ctx.r[30].u32 ) };
	// 82ECFBDC: 93DF0154  stw r30, 0x154(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(340 as u32), ctx.r[30].u32 ) };
	// 82ECFBE0: 917F0158  stw r11, 0x158(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(344 as u32), ctx.r[11].u32 ) };
	// 82ECFBE4: 93DF015C  stw r30, 0x15c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(348 as u32), ctx.r[30].u32 ) };
	// 82ECFBE8: 93DF0160  stw r30, 0x160(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(352 as u32), ctx.r[30].u32 ) };
	// 82ECFBEC: 917F0164  stw r11, 0x164(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(356 as u32), ctx.r[11].u32 ) };
	// 82ECFBF0: 93DF0168  stw r30, 0x168(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(360 as u32), ctx.r[30].u32 ) };
	// 82ECFBF4: 93DF016C  stw r30, 0x16c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(364 as u32), ctx.r[30].u32 ) };
	// 82ECFBF8: 917F0170  stw r11, 0x170(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(368 as u32), ctx.r[11].u32 ) };
	// 82ECFBFC: 93DF0174  stw r30, 0x174(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(372 as u32), ctx.r[30].u32 ) };
	// 82ECFC00: 93DF0178  stw r30, 0x178(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(376 as u32), ctx.r[30].u32 ) };
	// 82ECFC04: 917F017C  stw r11, 0x17c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(380 as u32), ctx.r[11].u32 ) };
	// 82ECFC08: 93DF0180  stw r30, 0x180(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(384 as u32), ctx.r[30].u32 ) };
	// 82ECFC0C: 93DF0184  stw r30, 0x184(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(388 as u32), ctx.r[30].u32 ) };
	// 82ECFC10: 917F0188  stw r11, 0x188(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(392 as u32), ctx.r[11].u32 ) };
	// 82ECFC14: 90DF00E0  stw r6, 0xe0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(224 as u32), ctx.r[6].u32 ) };
	// 82ECFC18: 7D69502E  lwzx r11, r9, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82ECFC1C: 814B0068  lwz r10, 0x68(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(104 as u32) ) } as u64;
	// 82ECFC20: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82ECFC24: 419A0020  beq cr6, 0x82ecfc44
	if ctx.cr[6].eq {
	pc = 0x82ECFC44; continue 'dispatch;
	}
	// 82ECFC28: 812B006C  lwz r9, 0x6c(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 82ECFC2C: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 82ECFC30: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 82ECFC34: 912B006C  stw r9, 0x6c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82ECFC38: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ECFC3C: 910B0068  stw r8, 0x68(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(104 as u32), ctx.r[8].u32 ) };
	// 82ECFC40: 48000014  b 0x82ecfc54
	pc = 0x82ECFC54; continue 'dispatch;
	// 82ECFC44: 38A0002F  li r5, 0x2f
	ctx.r[5].s64 = 47;
	// 82ECFC48: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 82ECFC4C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82ECFC50: 4BFD0A11  bl 0x82ea0660
	ctx.lr = 0x82ECFC54;
	sub_82EA0660(ctx, base);
	// 82ECFC54: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ECFC58: 419A0010  beq cr6, 0x82ecfc68
	if ctx.cr[6].eq {
	pc = 0x82ECFC68; continue 'dispatch;
	}
	// 82ECFC5C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82ECFC60: 48017A59  bl 0x82ee76b8
	ctx.lr = 0x82ECFC64;
	sub_82EE76B8(ctx, base);
	// 82ECFC64: 48000008  b 0x82ecfc6c
	pc = 0x82ECFC6C; continue 'dispatch;
	// 82ECFC68: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82ECFC6C: 3B7C0030  addi r27, r28, 0x30
	ctx.r[27].s64 = ctx.r[28].s64 + 48;
	// 82ECFC70: 907F007C  stw r3, 0x7c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(124 as u32), ctx.r[3].u32 ) };
	// 82ECFC74: 93DF0090  stw r30, 0x90(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), ctx.r[30].u32 ) };
	// 82ECFC78: 387F02D0  addi r3, r31, 0x2d0
	ctx.r[3].s64 = ctx.r[31].s64 + 720;
	// 82ECFC7C: 93DF0080  stw r30, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[30].u32 ) };
	// 82ECFC80: 3B5C0040  addi r26, r28, 0x40
	ctx.r[26].s64 = ctx.r[28].s64 + 64;
	// 82ECFC84: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 82ECFC88: 389F02E0  addi r4, r31, 0x2e0
	ctx.r[4].s64 = ctx.r[31].s64 + 736;
	// 82ECFC8C: 93DF0088  stw r30, 0x88(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(136 as u32), ctx.r[30].u32 ) };
	// 82ECFC90: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 82ECFC94: 9BDF008C  stb r30, 0x8c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(140 as u32), ctx.r[30].u8 ) };
	// 82ECFC98: 3B1F00A8  addi r24, r31, 0xa8
	ctx.r[24].s64 = ctx.r[31].s64 + 168;
	// 82ECFC9C: 99DF008D  stb r14, 0x8d(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(141 as u32), ctx.r[14].u8 ) };
	// 82ECFCA0: 3A1F00B4  addi r16, r31, 0xb4
	ctx.r[16].s64 = ctx.r[31].s64 + 180;
	// 82ECFCA4: 93DF00A8  stw r30, 0xa8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(168 as u32), ctx.r[30].u32 ) };
	// 82ECFCA8: 39FF00AC  addi r15, r31, 0xac
	ctx.r[15].s64 = ctx.r[31].s64 + 172;
	// 82ECFCAC: 93DF00B4  stw r30, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[30].u32 ) };
	// 82ECFCB0: 397F00B0  addi r11, r31, 0xb0
	ctx.r[11].s64 = ctx.r[31].s64 + 176;
	// 82ECFCB4: 93DF00AC  stw r30, 0xac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(172 as u32), ctx.r[30].u32 ) };
	// 82ECFCB8: 3B3F0054  addi r25, r31, 0x54
	ctx.r[25].s64 = ctx.r[31].s64 + 84;
	// 82ECFCBC: 93DF00B0  stw r30, 0xb0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(176 as u32), ctx.r[30].u32 ) };
	// 82ECFCC0: 91DF0094  stw r14, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[14].u32 ) };
            }
            0x82ED0018 => {
    //   block [0x82ED0018..0x82ED0024)
	// 82ED0018: FDA05090  fmr f13, f10
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].f64 = ctx.f[10].f64;
	// 82ED001C: FC00F090  fmr f0, f30
	ctx.f[0].f64 = ctx.f[30].f64;
	// 82ED0020: 48000030  b 0x82ed0050
	pc = 0x82ED0050; continue 'dispatch;
            }
            0x82ED0024 => {
    //   block [0x82ED0024..0x82ED0030)
	// 82ED0024: FDA0E890  fmr f13, f29
	ctx.f[13].f64 = ctx.f[29].f64;
	// 82ED0028: FC004890  fmr f0, f9
	ctx.f[0].f64 = ctx.f[9].f64;
	// 82ED002C: 48000024  b 0x82ed0050
	pc = 0x82ED0050; continue 'dispatch;
            }
            0x82ED0030 => {
    //   block [0x82ED0030..0x82ED003C)
	// 82ED0030: FDA04090  fmr f13, f8
	ctx.f[13].f64 = ctx.f[8].f64;
	// 82ED0034: FC003890  fmr f0, f7
	ctx.f[0].f64 = ctx.f[7].f64;
	// 82ED0038: 48000018  b 0x82ed0050
	pc = 0x82ED0050; continue 'dispatch;
            }
            0x82ED003C => {
    //   block [0x82ED003C..0x82ED0D28)
	// 82ED003C: FDA03090  fmr f13, f6
	ctx.f[13].f64 = ctx.f[6].f64;
	// 82ED0040: FC002890  fmr f0, f5
	ctx.f[0].f64 = ctx.f[5].f64;
	// 82ED0044: 4800000C  b 0x82ed0050
	pc = 0x82ED0050; continue 'dispatch;
	// 82ED0048: FDA02090  fmr f13, f4
	ctx.f[13].f64 = ctx.f[4].f64;
	// 82ED004C: FC001890  fmr f0, f3
	ctx.f[0].f64 = ctx.f[3].f64;
	// 82ED0050: EDAD02F2  fmuls f13, f13, f11
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[11].f64) as f32) as f64);
	// 82ED0054: 1D49001C  mulli r10, r9, 0x1c
	ctx.r[10].s64 = ctx.r[9].s64 * 28;
	// 82ED0058: C3080000  lfs f24, 0(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) };
	ctx.f[24].f64 = (tmp.f32 as f64);
	// 82ED005C: FF00F000  fcmpu cr6, f0, f30
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[30].f64);
	// 82ED0060: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82ED0064: EEFF6824  fdivs f23, f31, f13
	ctx.f[23].f64 = ((ctx.f[31].f64 / ctx.f[13].f64) as f32) as f64;
	// 82ED0068: EDAD06B2  fmuls f13, f13, f26
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[26].f64) as f32) as f64);
	// 82ED006C: D2EA0064  stfs f23, 0x64(r10)
	tmp.f32 = (ctx.f[23].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 82ED0070: EEF70032  fmuls f23, f23, f0
	ctx.f[23].f64 = (((ctx.f[23].f64 * ctx.f[0].f64) as f32) as f64);
	// 82ED0074: EDBF6824  fdivs f13, f31, f13
	ctx.f[13].f64 = ((ctx.f[31].f64 / ctx.f[13].f64) as f32) as f64;
	// 82ED0078: D1AA0068  stfs f13, 0x68(r10)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 82ED007C: EDB702F2  fmuls f13, f23, f11
	ctx.f[13].f64 = (((ctx.f[23].f64 * ctx.f[11].f64) as f32) as f64);
	// 82ED0080: EDAD0632  fmuls f13, f13, f24
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[24].f64) as f32) as f64);
	// 82ED0084: EDADFB3C  fnmsubs f13, f13, f12, f31
	ctx.f[13].f64 = -(((ctx.f[13].f64 * ctx.f[12].f64 - ctx.f[31].f64) as f32) as f64);
	// 82ED0088: D1AA006C  stfs f13, 0x6c(r10)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 82ED008C: 40990018  ble cr6, 0x82ed00a4
	if !ctx.cr[6].gt {
	pc = 0x82ED00A4; continue 'dispatch;
	}
	// 82ED0090: C1A80000  lfs f13, 0(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82ED0094: EC0D0024  fdivs f0, f13, f0
	ctx.f[0].f64 = ((ctx.f[13].f64 / ctx.f[0].f64) as f32) as f64;
	// 82ED0098: EDA00332  fmuls f13, f0, f12
	ctx.f[13].f64 = (((ctx.f[0].f64 * ctx.f[12].f64) as f32) as f64);
	// 82ED009C: D1AA0070  stfs f13, 0x70(r10)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 82ED00A0: 48000008  b 0x82ed00a8
	pc = 0x82ED00A8; continue 'dispatch;
	// 82ED00A4: D36A0070  stfs f27, 0x70(r10)
	tmp.f32 = (ctx.f[27].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 82ED00A8: C0070000  lfs f0, 0(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82ED00AC: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82ED00B0: EDA00732  fmuls f13, f0, f28
	ctx.f[13].f64 = (((ctx.f[0].f64 * ctx.f[28].f64) as f32) as f64);
	// 82ED00B4: D1A10060  stfs f13, 0x60(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 82ED00B8: 80810060  lwz r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82ED00BC: EDA00072  fmuls f13, f0, f1
	ctx.f[13].f64 = (((ctx.f[0].f64 * ctx.f[1].f64) as f32) as f64);
	// 82ED00C0: D1A10050  stfs f13, 0x50(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82ED00C4: 80C10050  lwz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82ED00C8: 7CC58670  srawi r5, r6, 0x10
	ctx.xer.ca = (ctx.r[6].s32 < 0) && ((ctx.r[6].u32 & ((1u32 << 16) - 1)) != 0);
	ctx.r[5].s64 = (ctx.r[6].s32 >> 16) as i64;
	// 82ED00CC: EDA000B2  fmuls f13, f0, f2
	ctx.f[13].f64 = (((ctx.f[0].f64 * ctx.f[2].f64) as f32) as f64);
	// 82ED00D0: 7C868670  srawi r6, r4, 0x10
	ctx.xer.ca = (ctx.r[4].s32 < 0) && ((ctx.r[4].u32 & ((1u32 << 16) - 1)) != 0);
	ctx.r[6].s64 = (ctx.r[4].s32 >> 16) as i64;
	// 82ED00D4: D00A0074  stfs f0, 0x74(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 82ED00D8: D1AA0078  stfs f13, 0x78(r10)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 82ED00DC: B0AA007C  sth r5, 0x7c(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(124 as u32), ctx.r[5].u16 ) };
	// 82ED00E0: B0CA007E  sth r6, 0x7e(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(126 as u32), ctx.r[6].u16 ) };
	// 82ED00E4: 2F090006  cmpwi cr6, r9, 6
	ctx.cr[6].compare_i32(ctx.r[9].s32, 6, &mut ctx.xer);
	// 82ED00E8: A08A007C  lhz r4, 0x7c(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(124 as u32) ) } as u64;
	// 82ED00EC: C00A0078  lfs f0, 0x78(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(120 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82ED00F0: C1AA0074  lfs f13, 0x74(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(116 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82ED00F4: EC000032  fmuls f0, f0, f0
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[0].f64) as f32) as f64);
	// 82ED00F8: 5483803E  rotlwi r3, r4, 0x10
	ctx.r[3].u64 = ((ctx.r[4].u32).rotate_left(16)) as u64;
	// 82ED00FC: D00A0078  stfs f0, 0x78(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 82ED0100: EDAD0372  fmuls f13, f13, f13
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[13].f64) as f32) as f64);
	// 82ED0104: D1AA0074  stfs f13, 0x74(r10)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 82ED0108: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82ED010C: 90610060  stw r3, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 82ED0110: C1A10060  lfs f13, 0x60(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82ED0114: C0010050  lfs f0, 0x50(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82ED0118: EDA00372  fmuls f13, f0, f13
	ctx.f[13].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 82ED011C: D1A10060  stfs f13, 0x60(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 82ED0120: 80C10060  lwz r6, 0x60(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82ED0124: 7CC58670  srawi r5, r6, 0x10
	ctx.xer.ca = (ctx.r[6].s32 < 0) && ((ctx.r[6].u32 & ((1u32 << 16) - 1)) != 0);
	ctx.r[5].s64 = (ctx.r[6].s32 >> 16) as i64;
	// 82ED0128: 54A3043E  clrlwi r3, r5, 0x10
	ctx.r[3].u64 = ctx.r[5].u32 as u64 & 0x0000FFFFu64;
	// 82ED012C: B0AA007C  sth r5, 0x7c(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(124 as u32), ctx.r[5].u16 ) };
	// 82ED0130: A0AA007E  lhz r5, 0x7e(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(126 as u32) ) } as u64;
	// 82ED0134: 54A4803E  rotlwi r4, r5, 0x10
	ctx.r[4].u64 = ((ctx.r[5].u32).rotate_left(16)) as u64;
	// 82ED0138: 90810050  stw r4, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[4].u32 ) };
	// 82ED013C: 5466803E  rotlwi r6, r3, 0x10
	ctx.r[6].u64 = ((ctx.r[3].u32).rotate_left(16)) as u64;
	// 82ED0140: 90C10060  stw r6, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[6].u32 ) };
	// 82ED0144: C0010060  lfs f0, 0x60(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82ED0148: C1A10050  lfs f13, 0x50(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82ED014C: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82ED0150: D0010060  stfs f0, 0x60(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 82ED0154: 80610060  lwz r3, 0x60(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82ED0158: 7C668670  srawi r6, r3, 0x10
	ctx.xer.ca = (ctx.r[3].s32 < 0) && ((ctx.r[3].u32 & ((1u32 << 16) - 1)) != 0);
	ctx.r[6].s64 = (ctx.r[3].s32 >> 16) as i64;
	// 82ED015C: B0CA007E  sth r6, 0x7e(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(126 as u32), ctx.r[6].u16 ) };
	// 82ED0160: 4198FE84  blt cr6, 0x82ecffe4
	if ctx.cr[6].lt {
	pc = 0x82ECFFE4; continue 'dispatch;
	}
	// 82ED0164: 817C0068  lwz r11, 0x68(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(104 as u32) ) } as u64;
	// 82ED0168: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED016C: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82ED0170: 419A001C  beq cr6, 0x82ed018c
	if ctx.cr[6].eq {
	pc = 0x82ED018C; continue 'dispatch;
	}
	// 82ED0174: A14B0004  lhz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED0178: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82ED017C: 419A0010  beq cr6, 0x82ed018c
	if ctx.cr[6].eq {
	pc = 0x82ED018C; continue 'dispatch;
	}
	// 82ED0180: A14B0006  lhz r10, 6(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(6 as u32) ) } as u64;
	// 82ED0184: 392A0001  addi r9, r10, 1
	ctx.r[9].s64 = ctx.r[10].s64 + 1;
	// 82ED0188: B12B0006  sth r9, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82ED018C: 897C00BE  lbz r11, 0xbe(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(190 as u32) ) } as u64;
	// 82ED0190: 395F00B8  addi r10, r31, 0xb8
	ctx.r[10].s64 = ctx.r[31].s64 + 184;
	// 82ED0194: 997F00B8  stb r11, 0xb8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(184 as u32), ctx.r[11].u8 ) };
	// 82ED0198: 897C00BC  lbz r11, 0xbc(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(188 as u32) ) } as u64;
	// 82ED019C: 997F00C5  stb r11, 0xc5(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(197 as u32), ctx.r[11].u8 ) };
	// 82ED01A0: 893F00B8  lbz r9, 0xb8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(184 as u32) ) } as u64;
	// 82ED01A4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82ED01A8: 409A0014  bne cr6, 0x82ed01bc
	if !ctx.cr[6].eq {
	pc = 0x82ED01BC; continue 'dispatch;
	}
	// 82ED01AC: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82ED01B0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82ED01B4: 419A0008  beq cr6, 0x82ed01bc
	if ctx.cr[6].eq {
	pc = 0x82ED01BC; continue 'dispatch;
	}
	// 82ED01B8: 9BDF00C5  stb r30, 0xc5(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(197 as u32), ctx.r[30].u8 ) };
	// 82ED01BC: 897C00C4  lbz r11, 0xc4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(196 as u32) ) } as u64;
	// 82ED01C0: 997F00A0  stb r11, 0xa0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(160 as u32), ctx.r[11].u8 ) };
	// 82ED01C4: 81720000  lwz r11, 0(r18)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[18].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED01C8: 814B0090  lwz r10, 0x90(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 82ED01CC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82ED01D0: 419A0020  beq cr6, 0x82ed01f0
	if ctx.cr[6].eq {
	pc = 0x82ED01F0; continue 'dispatch;
	}
	// 82ED01D4: 812B0094  lwz r9, 0x94(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(148 as u32) ) } as u64;
	// 82ED01D8: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 82ED01DC: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 82ED01E0: 912B0094  stw r9, 0x94(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(148 as u32), ctx.r[9].u32 ) };
	// 82ED01E4: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED01E8: 910B0090  stw r8, 0x90(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(144 as u32), ctx.r[8].u32 ) };
	// 82ED01EC: 48000014  b 0x82ed0200
	pc = 0x82ED0200; continue 'dispatch;
	// 82ED01F0: 38A00027  li r5, 0x27
	ctx.r[5].s64 = 39;
	// 82ED01F4: 3880000B  li r4, 0xb
	ctx.r[4].s64 = 11;
	// 82ED01F8: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82ED01FC: 4BFD0465  bl 0x82ea0660
	ctx.lr = 0x82ED0200;
	sub_82EA0660(ctx, base);
	// 82ED0200: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ED0204: 419A000C  beq cr6, 0x82ed0210
	if ctx.cr[6].eq {
	pc = 0x82ED0210; continue 'dispatch;
	}
	// 82ED0208: 48053641  bl 0x82f23848
	ctx.lr = 0x82ED020C;
	sub_82F23848(ctx, base);
	// 82ED020C: 48000008  b 0x82ed0214
	pc = 0x82ED0214; continue 'dispatch;
	// 82ED0210: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82ED0214: 907F0058  stw r3, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[3].u32 ) };
	// 82ED0218: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82ED021C: 3880000C  li r4, 0xc
	ctx.r[4].s64 = 12;
	// 82ED0220: 80720000  lwz r3, 0(r18)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[18].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED0224: 4BFD050D  bl 0x82ea0730
	ctx.lr = 0x82ED0228;
	sub_82EA0730(ctx, base);
	// 82ED0228: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82ED022C: B1C30006  sth r14, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[14].u16 ) };
	// 82ED0230: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82ED0234: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 82ED0238: 3BA0000C  li r29, 0xc
	ctx.r[29].s64 = 12;
	// 82ED023C: 3AEB41F8  addi r23, r11, 0x41f8
	ctx.r[23].s64 = ctx.r[11].s64 + 16888;
	// 82ED0240: 390A4218  addi r8, r10, 0x4218
	ctx.r[8].s64 = ctx.r[10].s64 + 16920;
	// 82ED0244: B3A30004  sth r29, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[29].u16 ) };
	// 82ED0248: 38E94208  addi r7, r9, 0x4208
	ctx.r[7].s64 = ctx.r[9].s64 + 16904;
	// 82ED024C: 92E30008  stw r23, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[23].u32 ) };
	// 82ED0250: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82ED0254: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82ED0258: 90E30008  stw r7, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 82ED025C: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 82ED0260: 907F005C  stw r3, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[3].u32 ) };
	// 82ED0264: 80720000  lwz r3, 0(r18)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[18].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED0268: 4BFD04C9  bl 0x82ea0730
	ctx.lr = 0x82ED026C;
	sub_82EA0730(ctx, base);
	// 82ED026C: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 82ED0270: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82ED0274: B0C30004  sth r6, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[6].u16 ) };
	// 82ED0278: 4801CB11  bl 0x82eecd88
	ctx.lr = 0x82ED027C;
	sub_82EECD88(ctx, base);
	// 82ED027C: 907F0060  stw r3, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 82ED0280: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82ED0284: 80720000  lwz r3, 0(r18)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[18].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED0288: 3880000C  li r4, 0xc
	ctx.r[4].s64 = 12;
	// 82ED028C: 4BFD04A5  bl 0x82ea0730
	ctx.lr = 0x82ED0290;
	sub_82EA0730(ctx, base);
	// 82ED0290: B3A30004  sth r29, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[29].u16 ) };
	// 82ED0294: B1C30006  sth r14, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[14].u16 ) };
	// 82ED0298: 3CA08212  lis r5, -0x7dee
	ctx.r[5].s64 = -2112749568;
	// 82ED029C: 92E30008  stw r23, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[23].u32 ) };
	// 82ED02A0: 3C808212  lis r4, -0x7dee
	ctx.r[4].s64 = -2112749568;
	// 82ED02A4: 39654238  addi r11, r5, 0x4238
	ctx.r[11].s64 = ctx.r[5].s64 + 16952;
	// 82ED02A8: 39444228  addi r10, r4, 0x4228
	ctx.r[10].s64 = ctx.r[4].s64 + 16936;
	// 82ED02AC: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82ED02B0: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82ED02B4: 907F0064  stw r3, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[3].u32 ) };
	// 82ED02B8: 817F005C  lwz r11, 0x5c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 82ED02BC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED02C0: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82ED02C4: 409A0008  bne cr6, 0x82ed02cc
	if !ctx.cr[6].eq {
	pc = 0x82ED02CC; continue 'dispatch;
	}
	// 82ED02C8: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82ED02CC: 815F0058  lwz r10, 0x58(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 82ED02D0: 916A0028  stw r11, 0x28(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82ED02D4: 817F005C  lwz r11, 0x5c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 82ED02D8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED02DC: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82ED02E0: 409A0008  bne cr6, 0x82ed02e8
	if !ctx.cr[6].eq {
	pc = 0x82ED02E8; continue 'dispatch;
	}
	// 82ED02E4: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82ED02E8: 815F0058  lwz r10, 0x58(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 82ED02EC: 916A0044  stw r11, 0x44(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 82ED02F0: 817F005C  lwz r11, 0x5c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 82ED02F4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED02F8: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82ED02FC: 409A0008  bne cr6, 0x82ed0304
	if !ctx.cr[6].eq {
	pc = 0x82ED0304; continue 'dispatch;
	}
	// 82ED0300: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82ED0304: 815F0058  lwz r10, 0x58(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 82ED0308: 916A0048  stw r11, 0x48(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 82ED030C: 817F0060  lwz r11, 0x60(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 82ED0310: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED0314: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82ED0318: 409A0008  bne cr6, 0x82ed0320
	if !ctx.cr[6].eq {
	pc = 0x82ED0320; continue 'dispatch;
	}
	// 82ED031C: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82ED0320: 815F0058  lwz r10, 0x58(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 82ED0324: 916A0024  stw r11, 0x24(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82ED0328: 817F0064  lwz r11, 0x64(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 82ED032C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED0330: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82ED0334: 409A0008  bne cr6, 0x82ed033c
	if !ctx.cr[6].eq {
	pc = 0x82ED033C; continue 'dispatch;
	}
	// 82ED0338: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82ED033C: 815F0058  lwz r10, 0x58(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 82ED0340: 916A002C  stw r11, 0x2c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82ED0344: 817F0064  lwz r11, 0x64(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 82ED0348: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED034C: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82ED0350: 409A0008  bne cr6, 0x82ed0358
	if !ctx.cr[6].eq {
	pc = 0x82ED0358; continue 'dispatch;
	}
	// 82ED0354: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82ED0358: 815F0058  lwz r10, 0x58(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 82ED035C: 916A0064  stw r11, 0x64(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82ED0360: 817F0064  lwz r11, 0x64(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 82ED0364: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED0368: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82ED036C: 409A0008  bne cr6, 0x82ed0374
	if !ctx.cr[6].eq {
	pc = 0x82ED0374; continue 'dispatch;
	}
	// 82ED0370: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82ED0374: 815F0058  lwz r10, 0x58(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 82ED0378: 916A004C  stw r11, 0x4c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(76 as u32), ctx.r[11].u32 ) };
	// 82ED037C: 817F0064  lwz r11, 0x64(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 82ED0380: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED0384: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82ED0388: 409A0008  bne cr6, 0x82ed0390
	if !ctx.cr[6].eq {
	pc = 0x82ED0390; continue 'dispatch;
	}
	// 82ED038C: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82ED0390: 815F0058  lwz r10, 0x58(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 82ED0394: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82ED0398: 817F0064  lwz r11, 0x64(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 82ED039C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED03A0: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82ED03A4: 409A0008  bne cr6, 0x82ed03ac
	if !ctx.cr[6].eq {
	pc = 0x82ED03AC; continue 'dispatch;
	}
	// 82ED03A8: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82ED03AC: 815F0058  lwz r10, 0x58(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 82ED03B0: 38A00022  li r5, 0x22
	ctx.r[5].s64 = 34;
	// 82ED03B4: 3880000C  li r4, 0xc
	ctx.r[4].s64 = 12;
	// 82ED03B8: 916A006C  stw r11, 0x6c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82ED03BC: 80720000  lwz r3, 0(r18)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[18].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED03C0: 4BFD0371  bl 0x82ea0730
	ctx.lr = 0x82ED03C4;
	sub_82EA0730(ctx, base);
	// 82ED03C4: B3A30004  sth r29, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[29].u16 ) };
	// 82ED03C8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82ED03CC: 4800C81D  bl 0x82edcbe8
	ctx.lr = 0x82ED03D0;
	sub_82EDCBE8(ctx, base);
	// 82ED03D0: 81320000  lwz r9, 0(r18)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[18].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED03D4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82ED03D8: 38A00027  li r5, 0x27
	ctx.r[5].s64 = 39;
	// 82ED03DC: 38801E50  li r4, 0x1e50
	ctx.r[4].s64 = 7760;
	// 82ED03E0: 7D234B78  mr r3, r9
	ctx.r[3].u64 = ctx.r[9].u64;
	// 82ED03E4: 4BFD034D  bl 0x82ea0730
	ctx.lr = 0x82ED03E8;
	sub_82EA0730(ctx, base);
	// 82ED03E8: 39001E50  li r8, 0x1e50
	ctx.r[8].s64 = 7760;
	// 82ED03EC: 3CE082F2  lis r7, -0x7d0e
	ctx.r[7].s64 = -2098069504;
	// 82ED03F0: B1030004  sth r8, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[8].u16 ) };
	// 82ED03F4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82ED03F8: 388761F0  addi r4, r7, 0x61f0
	ctx.r[4].s64 = ctx.r[7].s64 + 25072;
	// 82ED03FC: 3ABF0074  addi r21, r31, 0x74
	ctx.r[21].s64 = ctx.r[31].s64 + 116;
	// 82ED0400: 48054DE1  bl 0x82f251e0
	ctx.lr = 0x82ED0404;
	sub_82F251E0(ctx, base);
	// 82ED0404: 907F0074  stw r3, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 82ED0408: A0DD0004  lhz r6, 4(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED040C: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82ED0410: 419A0034  beq cr6, 0x82ed0444
	if ctx.cr[6].eq {
	pc = 0x82ED0444; continue 'dispatch;
	}
	// 82ED0414: A17D0006  lhz r11, 6(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(6 as u32) ) } as u64;
	// 82ED0418: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 82ED041C: 7D490734  extsh r9, r10
	ctx.r[9].s64 = ctx.r[10].s16 as i64;
	// 82ED0420: B13D0006  sth r9, 6(r29)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[29].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82ED0424: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82ED0428: 409A001C  bne cr6, 0x82ed0444
	if !ctx.cr[6].eq {
	pc = 0x82ED0444; continue 'dispatch;
	}
	// 82ED042C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED0430: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82ED0434: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82ED0438: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED043C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82ED0440: 4E800421  bctrl
	ctx.lr = 0x82ED0444;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82ED0444: 817C0054  lwz r11, 0x54(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(84 as u32) ) } as u64;
	// 82ED0448: 3A200030  li r17, 0x30
	ctx.r[17].s64 = 48;
	// 82ED044C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED0450: 409A0068  bne cr6, 0x82ed04b8
	if !ctx.cr[6].eq {
	pc = 0x82ED04B8; continue 'dispatch;
	}
	// 82ED0454: 38A00027  li r5, 0x27
	ctx.r[5].s64 = 39;
	// 82ED0458: 80720000  lwz r3, 0(r18)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[18].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED045C: 38800030  li r4, 0x30
	ctx.r[4].s64 = 48;
	// 82ED0460: 4BFD02D1  bl 0x82ea0730
	ctx.lr = 0x82ED0464;
	sub_82EA0730(ctx, base);
	// 82ED0464: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82ED0468: B23D0004  sth r17, 4(r29)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[17].u16 ) };
	// 82ED046C: 4804D555  bl 0x82f1d9c0
	ctx.lr = 0x82ED0470;
	sub_82F1D9C0(ctx, base);
	// 82ED0470: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82ED0474: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82ED0478: 91DD0020  stw r14, 0x20(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(32 as u32), ctx.r[14].u32 ) };
	// 82ED047C: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 82ED0480: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 82ED0484: 3CE08212  lis r7, -0x7dee
	ctx.r[7].s64 = -2112749568;
	// 82ED0488: 38CB4294  addi r6, r11, 0x4294
	ctx.r[6].s64 = ctx.r[11].s64 + 17044;
	// 82ED048C: 38AA4288  addi r5, r10, 0x4288
	ctx.r[5].s64 = ctx.r[10].s64 + 17032;
	// 82ED0490: 38894268  addi r4, r9, 0x4268
	ctx.r[4].s64 = ctx.r[9].s64 + 17000;
	// 82ED0494: 90DD0000  stw r6, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82ED0498: 3868427C  addi r3, r8, 0x427c
	ctx.r[3].s64 = ctx.r[8].s64 + 17020;
	// 82ED049C: 90BD0008  stw r5, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 82ED04A0: 3967425C  addi r11, r7, 0x425c
	ctx.r[11].s64 = ctx.r[7].s64 + 16988;
	// 82ED04A4: 909D000C  stw r4, 0xc(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 82ED04A8: 907D0010  stw r3, 0x10(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 82ED04AC: 917D0014  stw r11, 0x14(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82ED04B0: 93BF0070  stw r29, 0x70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[29].u32 ) };
	// 82ED04B4: 48000020  b 0x82ed04d4
	pc = 0x82ED04D4; continue 'dispatch;
	// 82ED04B8: 917F0070  stw r11, 0x70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82ED04BC: A14B0004  lhz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED04C0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82ED04C4: 419A0010  beq cr6, 0x82ed04d4
	if ctx.cr[6].eq {
	pc = 0x82ED04D4; continue 'dispatch;
	}
	// 82ED04C8: A14B0006  lhz r10, 6(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(6 as u32) ) } as u64;
	// 82ED04CC: 392A0001  addi r9, r10, 1
	ctx.r[9].s64 = ctx.r[10].s64 + 1;
	// 82ED04D0: B12B0006  sth r9, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82ED04D4: 817C0058  lwz r11, 0x58(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(88 as u32) ) } as u64;
	// 82ED04D8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED04DC: 409A0034  bne cr6, 0x82ed0510
	if !ctx.cr[6].eq {
	pc = 0x82ED0510; continue 'dispatch;
	}
	// 82ED04E0: 38A00027  li r5, 0x27
	ctx.r[5].s64 = 39;
	// 82ED04E4: 80720000  lwz r3, 0(r18)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[18].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED04E8: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 82ED04EC: 4BFD0245  bl 0x82ea0730
	ctx.lr = 0x82ED04F0;
	sub_82EA0730(ctx, base);
	// 82ED04F0: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82ED04F4: 39400008  li r10, 8
	ctx.r[10].s64 = 8;
	// 82ED04F8: B1C30006  sth r14, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[14].u16 ) };
	// 82ED04FC: 392B42A4  addi r9, r11, 0x42a4
	ctx.r[9].s64 = ctx.r[11].s64 + 17060;
	// 82ED0500: B1430004  sth r10, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 82ED0504: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82ED0508: 907F0078  stw r3, 0x78(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(120 as u32), ctx.r[3].u32 ) };
	// 82ED050C: 48000020  b 0x82ed052c
	pc = 0x82ED052C; continue 'dispatch;
	// 82ED0510: 917F0078  stw r11, 0x78(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(120 as u32), ctx.r[11].u32 ) };
	// 82ED0514: A14B0004  lhz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED0518: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82ED051C: 419A0010  beq cr6, 0x82ed052c
	if ctx.cr[6].eq {
	pc = 0x82ED052C; continue 'dispatch;
	}
	// 82ED0520: A14B0006  lhz r10, 6(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(6 as u32) ) } as u64;
	// 82ED0524: 392A0001  addi r9, r10, 1
	ctx.r[9].s64 = ctx.r[10].s64 + 1;
	// 82ED0528: B12B0006  sth r9, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82ED052C: 80720000  lwz r3, 0(r18)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[18].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED0530: 83A30070  lwz r29, 0x70(r3)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(112 as u32) ) } as u64;
	// 82ED0534: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82ED0538: 419A001C  beq cr6, 0x82ed0554
	if ctx.cr[6].eq {
	pc = 0x82ED0554; continue 'dispatch;
	}
	// 82ED053C: 81630074  lwz r11, 0x74(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(116 as u32) ) } as u64;
	// 82ED0540: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82ED0544: 91630074  stw r11, 0x74(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82ED0548: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED054C: 91430070  stw r10, 0x70(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(112 as u32), ctx.r[10].u32 ) };
	// 82ED0550: 48000014  b 0x82ed0564
	pc = 0x82ED0564; continue 'dispatch;
	// 82ED0554: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 82ED0558: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 82ED055C: 4BFD0105  bl 0x82ea0660
	ctx.lr = 0x82ED0560;
	sub_82EA0660(ctx, base);
	// 82ED0560: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82ED0564: 81750000  lwz r11, 0(r21)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED0568: 3A7F006C  addi r19, r31, 0x6c
	ctx.r[19].s64 = ctx.r[31].s64 + 108;
	// 82ED056C: 93BF006C  stw r29, 0x6c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[29].u32 ) };
	// 82ED0570: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82ED0574: C01C0050  lfs f0, 0x50(r28)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(80 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82ED0578: D01D0004  stfs f0, 4(r29)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82ED057C: 817F0070  lwz r11, 0x70(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 82ED0580: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED0584: 396B000C  addi r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 + 12;
	// 82ED0588: 409A0008  bne cr6, 0x82ed0590
	if !ctx.cr[6].eq {
	pc = 0x82ED0590; continue 'dispatch;
	}
	// 82ED058C: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82ED0590: 917D0008  stw r11, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82ED0594: 817F0078  lwz r11, 0x78(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 82ED0598: 917D000C  stw r11, 0xc(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82ED059C: 81720000  lwz r11, 0(r18)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[18].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED05A0: 814B0048  lwz r10, 0x48(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 82ED05A4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82ED05A8: 419A0020  beq cr6, 0x82ed05c8
	if ctx.cr[6].eq {
	pc = 0x82ED05C8; continue 'dispatch;
	}
	// 82ED05AC: 812B004C  lwz r9, 0x4c(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 82ED05B0: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 82ED05B4: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 82ED05B8: 912B004C  stw r9, 0x4c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(76 as u32), ctx.r[9].u32 ) };
	// 82ED05BC: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED05C0: 910B0048  stw r8, 0x48(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(72 as u32), ctx.r[8].u32 ) };
	// 82ED05C4: 48000014  b 0x82ed05d8
	pc = 0x82ED05D8; continue 'dispatch;
	// 82ED05C8: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 82ED05CC: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 82ED05D0: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82ED05D4: 4BFD008D  bl 0x82ea0660
	ctx.lr = 0x82ED05D8;
	sub_82EA0660(ctx, base);
	// 82ED05D8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82ED05DC: 419A0018  beq cr6, 0x82ed05f4
	if ctx.cr[6].eq {
	pc = 0x82ED05F4; continue 'dispatch;
	}
	// 82ED05E0: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82ED05E4: D3A30000  stfs f29, 0(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82ED05E8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82ED05EC: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82ED05F0: 48000008  b 0x82ed05f8
	pc = 0x82ED05F8; continue 'dispatch;
	// 82ED05F4: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82ED05F8: D3C1007C  stfs f30, 0x7c(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), tmp.u32 ) };
	// 82ED05FC: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 82ED0600: 917D006C  stw r11, 0x6c(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82ED0604: 39610070  addi r11, r1, 0x70
	ctx.r[11].s64 = ctx.r[1].s64 + 112;
	// 82ED0608: 3CE08212  lis r7, -0x7dee
	ctx.r[7].s64 = -2112749568;
	// 82ED060C: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82ED0610: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82ED0614: C0094530  lfs f0, 0x4530(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(17712 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82ED0618: 39010070  addi r8, r1, 0x70
	ctx.r[8].s64 = ctx.r[1].s64 + 112;
	// 82ED061C: D0010060  stfs f0, 0x60(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 82ED0620: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82ED0624: C007452C  lfs f0, 0x452c(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(17708 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82ED0628: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82ED062C: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82ED0630: 38DD0040  addi r6, r29, 0x40
	ctx.r[6].s64 = ctx.r[29].s64 + 64;
	// 82ED0634: 3929C5B0  addi r9, r9, -0x3a50
	ctx.r[9].s64 = ctx.r[9].s64 + -14928;
	// 82ED0638: 389D0020  addi r4, r29, 0x20
	ctx.r[4].s64 = ctx.r[29].s64 + 32;
	// 82ED063C: 38BD0030  addi r5, r29, 0x30
	ctx.r[5].s64 = ctx.r[29].s64 + 48;
	// 82ED0640: 3AFC0070  addi r23, r28, 0x70
	ctx.r[23].s64 = ctx.r[28].s64 + 112;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED0D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82ED0D28 size=136
    let mut pc: u32 = 0x82ED0D28;
    'dispatch: loop {
        match pc {
            0x82ED0D28 => {
    //   block [0x82ED0D28..0x82ED0DB0)
	// 82ED0D28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED0D2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82ED0D30: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82ED0D34: 9421FF00  stwu r1, -0x100(r1)
	ea = ctx.r[1].u32.wrapping_add(-256 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED0D38: 7CFF3B78  mr r31, r7
	ctx.r[31].u64 = ctx.r[7].u64;
	// 82ED0D3C: 81630070  lwz r11, 0x70(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(112 as u32) ) } as u64;
	// 82ED0D40: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82ED0D44: 3CE08212  lis r7, -0x7dee
	ctx.r[7].s64 = -2112749568;
	// 82ED0D48: 7CC83378  mr r8, r6
	ctx.r[8].u64 = ctx.r[6].u64;
	// 82ED0D4C: 38C742D8  addi r6, r7, 0x42d8
	ctx.r[6].s64 = ctx.r[7].s64 + 17112;
	// 82ED0D50: 7C892378  mr r9, r4
	ctx.r[9].u64 = ctx.r[4].u64;
	// 82ED0D54: C00AC3C8  lfs f0, -0x3c38(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-15416 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82ED0D58: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED0D5C: D00100E0  stfs f0, 0xe0(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(224 as u32), tmp.u32 ) };
	// 82ED0D60: 90C10060  stw r6, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[6].u32 ) };
	// 82ED0D64: 38EB0008  addi r7, r11, 8
	ctx.r[7].s64 = ctx.r[11].s64 + 8;
	// 82ED0D68: 409A0008  bne cr6, 0x82ed0d70
	if !ctx.cr[6].eq {
	pc = 0x82ED0D70; continue 'dispatch;
	}
	// 82ED0D6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82ED0D70: 8163006C  lwz r11, 0x6c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(108 as u32) ) } as u64;
	// 82ED0D74: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 82ED0D78: 80830054  lwz r4, 0x54(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(84 as u32) ) } as u64;
	// 82ED0D7C: 7D254B78  mr r5, r9
	ctx.r[5].u64 = ctx.r[9].u64;
	// 82ED0D80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82ED0D84: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82ED0D88: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82ED0D8C: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 82ED0D90: 7D685B78  mr r8, r11
	ctx.r[8].u64 = ctx.r[11].u64;
	// 82ED0D94: 812B006C  lwz r9, 0x6c(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 82ED0D98: 48055259  bl 0x82f25ff0
	ctx.lr = 0x82ED0D9C;
	sub_82F25FF0(ctx, base);
	// 82ED0D9C: 38210100  addi r1, r1, 0x100
	ctx.r[1].s64 = ctx.r[1].s64 + 256;
	// 82ED0DA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82ED0DA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82ED0DA8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82ED0DAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED0DB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82ED0DB0 size=316
    let mut pc: u32 = 0x82ED0DB0;
    'dispatch: loop {
        match pc {
            0x82ED0DB0 => {
    //   block [0x82ED0DB0..0x82ED0EEC)
	// 82ED0DB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82ED0DB4: 482D73AD  bl 0x831a8160
	ctx.lr = 0x82ED0DB8;
	sub_831A8130(ctx, base);
	// 82ED0DB8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82ED0DBC: 838D0000  lwz r28, 0(r13)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED0DC0: 3B600014  li r27, 0x14
	ctx.r[27].s64 = 20;
	// 82ED0DC4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82ED0DC8: 38A0002F  li r5, 0x2f
	ctx.r[5].s64 = 47;
	// 82ED0DCC: 38800044  li r4, 0x44
	ctx.r[4].s64 = 68;
	// 82ED0DD0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82ED0DD4: 7C7BE02E  lwzx r3, r27, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82ED0DD8: 4BFCF959  bl 0x82ea0730
	ctx.lr = 0x82ED0DDC;
	sub_82EA0730(ctx, base);
	// 82ED0DDC: 3B400044  li r26, 0x44
	ctx.r[26].s64 = 68;
	// 82ED0DE0: B3430004  sth r26, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[26].u16 ) };
	// 82ED0DE4: 4801A41D  bl 0x82eeb200
	ctx.lr = 0x82ED0DE8;
	sub_82EEB200(ctx, base);
	// 82ED0DE8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82ED0DEC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82ED0DF0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82ED0DF4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82ED0DF8: 4BFFD109  bl 0x82ecdf00
	ctx.lr = 0x82ED0DFC;
	sub_82ECDF00(ctx, base);
	// 82ED0DFC: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED0E00: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82ED0E04: 419A004C  beq cr6, 0x82ed0e50
	if ctx.cr[6].eq {
	pc = 0x82ED0E50; continue 'dispatch;
	}
	// 82ED0E08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82ED0E0C: 997E0040  stb r11, 0x40(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(64 as u32), ctx.r[11].u8 ) };
	// 82ED0E10: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED0E14: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED0E18: 552800BE  clrlwi r8, r9, 2
	ctx.r[8].u64 = ctx.r[9].u32 as u64 & 0x3FFFFFFFu64;
	// 82ED0E1C: 7F0A4000  cmpw cr6, r10, r8
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82ED0E20: 409A0010  bne cr6, 0x82ed0e30
	if !ctx.cr[6].eq {
	pc = 0x82ED0E30; continue 'dispatch;
	}
	// 82ED0E24: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82ED0E28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED0E2C: 4BFD5A55  bl 0x82ea6880
	ctx.lr = 0x82ED0E30;
	sub_82EA6880(ctx, base);
	// 82ED0E30: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED0E34: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED0E38: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82ED0E3C: 7FC9512E  stwx r30, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 82ED0E40: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED0E44: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 82ED0E48: 911F0004  stw r8, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82ED0E4C: 4800000C  b 0x82ed0e58
	pc = 0x82ED0E58; continue 'dispatch;
	// 82ED0E50: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82ED0E54: 409A0020  bne cr6, 0x82ed0e74
	if !ctx.cr[6].eq {
	pc = 0x82ED0E74; continue 'dispatch;
	}
	// 82ED0E58: 38A0002F  li r5, 0x2f
	ctx.r[5].s64 = 47;
	// 82ED0E5C: 7C7BE02E  lwzx r3, r27, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82ED0E60: 38800044  li r4, 0x44
	ctx.r[4].s64 = 68;
	// 82ED0E64: 4BFCF8CD  bl 0x82ea0730
	ctx.lr = 0x82ED0E68;
	sub_82EA0730(ctx, base);
	// 82ED0E68: B3430004  sth r26, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[26].u16 ) };
	// 82ED0E6C: 4801A395  bl 0x82eeb200
	ctx.lr = 0x82ED0E70;
	sub_82EEB200(ctx, base);
	// 82ED0E70: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82ED0E74: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82ED0E78: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED0E7C: 556900BE  clrlwi r9, r11, 2
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82ED0E80: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82ED0E84: 409A0010  bne cr6, 0x82ed0e94
	if !ctx.cr[6].eq {
	pc = 0x82ED0E94; continue 'dispatch;
	}
	// 82ED0E88: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82ED0E8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82ED0E90: 4BFD59F1  bl 0x82ea6880
	ctx.lr = 0x82ED0E94;
	sub_82EA6880(ctx, base);
	// 82ED0E94: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED0E98: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82ED0E9C: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82ED0EA0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82ED0EA4: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82ED0EA8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82ED0EAC: 7FC9512E  stwx r30, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 82ED0EB0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82ED0EB4: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 82ED0EB8: 911F0004  stw r8, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82ED0EBC: 4BFFD185  bl 0x82ece040
	ctx.lr = 0x82ED0EC0;
	sub_82ECE040(ctx, base);
	// 82ED0EC0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82ED0EC4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82ED0EC8: 4BFFD501  bl 0x82ece3c8
	ctx.lr = 0x82ED0ECC;
	sub_82ECE3C8(ctx, base);
	// 82ED0ECC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82ED0ED0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82ED0ED4: 4BFFD335  bl 0x82ece208
	ctx.lr = 0x82ED0ED8;
	sub_82ECE208(ctx, base);
	// 82ED0ED8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82ED0EDC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82ED0EE0: 4BFFD3D1  bl 0x82ece2b0
	ctx.lr = 0x82ED0EE4;
	sub_82ECE2B0(ctx, base);
	// 82ED0EE4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82ED0EE8: 482D72C8  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82ED0EF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82ED0EF0 size=236
    let mut pc: u32 = 0x82ED0EF0;
    'dispatch: loop {
        match pc {
            0x82ED0EF0 => {
    //   block [0x82ED0EF0..0x82ED0FDC)
	// 82ED0EF0: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82ED0EF4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82ED0EF8: 390A45A8  addi r8, r10, 0x45a8
	ctx.r[8].s64 = ctx.r[10].s64 + 17832;
	// 82ED0EFC: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82ED0F00: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 82ED0F04: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82ED0F08: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82ED0F0C: 38E0FFD1  li r7, -0x2f
	ctx.r[7].s64 = -47;
	// 82ED0F10: 91630028  stw r11, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82ED0F14: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82ED0F18: 91430030  stw r10, 0x30(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[10].u32 ) };
	// 82ED0F1C: 91630034  stw r11, 0x34(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 82ED0F20: 91630038  stw r11, 0x38(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82ED0F24: 9143003C  stw r10, 0x3c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), ctx.r[10].u32 ) };
	// 82ED0F28: 91630040  stw r11, 0x40(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 82ED0F2C: 91630044  stw r11, 0x44(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 82ED0F30: 91430048  stw r10, 0x48(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), ctx.r[10].u32 ) };
	// 82ED0F34: 90E30098  stw r7, 0x98(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(152 as u32), ctx.r[7].u32 ) };
	// 82ED0F38: B163009C  sth r11, 0x9c(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(156 as u32), ctx.r[11].u16 ) };
	// 82ED0F3C: 916300F0  stw r11, 0xf0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(240 as u32), ctx.r[11].u32 ) };
	// 82ED0F40: 916300F4  stw r11, 0xf4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(244 as u32), ctx.r[11].u32 ) };
	// 82ED0F44: 914300F8  stw r10, 0xf8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(248 as u32), ctx.r[10].u32 ) };
	// 82ED0F48: 916300FC  stw r11, 0xfc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(252 as u32), ctx.r[11].u32 ) };
	// 82ED0F4C: 91630100  stw r11, 0x100(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(256 as u32), ctx.r[11].u32 ) };
	// 82ED0F50: 91430104  stw r10, 0x104(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(260 as u32), ctx.r[10].u32 ) };
	// 82ED0F54: 91630108  stw r11, 0x108(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(264 as u32), ctx.r[11].u32 ) };
	// 82ED0F58: 9163010C  stw r11, 0x10c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(268 as u32), ctx.r[11].u32 ) };
	// 82ED0F5C: 91430110  stw r10, 0x110(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(272 as u32), ctx.r[10].u32 ) };
	// 82ED0F60: 91630114  stw r11, 0x114(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(276 as u32), ctx.r[11].u32 ) };
	// 82ED0F64: 91630118  stw r11, 0x118(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(280 as u32), ctx.r[11].u32 ) };
	// 82ED0F68: 9143011C  stw r10, 0x11c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(284 as u32), ctx.r[10].u32 ) };
	// 82ED0F6C: 91630120  stw r11, 0x120(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(288 as u32), ctx.r[11].u32 ) };
	// 82ED0F70: 91630124  stw r11, 0x124(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(292 as u32), ctx.r[11].u32 ) };
	// 82ED0F74: 91430128  stw r10, 0x128(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(296 as u32), ctx.r[10].u32 ) };
	// 82ED0F78: 9163012C  stw r11, 0x12c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(300 as u32), ctx.r[11].u32 ) };
	// 82ED0F7C: 91630130  stw r11, 0x130(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(304 as u32), ctx.r[11].u32 ) };
	// 82ED0F80: 91430134  stw r10, 0x134(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(308 as u32), ctx.r[10].u32 ) };
	// 82ED0F84: 91630138  stw r11, 0x138(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(312 as u32), ctx.r[11].u32 ) };
	// 82ED0F88: 9163013C  stw r11, 0x13c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(316 as u32), ctx.r[11].u32 ) };
	// 82ED0F8C: 91430140  stw r10, 0x140(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(320 as u32), ctx.r[10].u32 ) };
	// 82ED0F90: 91630144  stw r11, 0x144(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(324 as u32), ctx.r[11].u32 ) };
	// 82ED0F94: 91630148  stw r11, 0x148(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(328 as u32), ctx.r[11].u32 ) };
	// 82ED0F98: 9143014C  stw r10, 0x14c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(332 as u32), ctx.r[10].u32 ) };
	// 82ED0F9C: 91630150  stw r11, 0x150(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(336 as u32), ctx.r[11].u32 ) };
	// 82ED0FA0: 91630154  stw r11, 0x154(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(340 as u32), ctx.r[11].u32 ) };
	// 82ED0FA4: 91430158  stw r10, 0x158(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(344 as u32), ctx.r[10].u32 ) };
	// 82ED0FA8: 9163015C  stw r11, 0x15c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(348 as u32), ctx.r[11].u32 ) };
	// 82ED0FAC: 91630160  stw r11, 0x160(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(352 as u32), ctx.r[11].u32 ) };
	// 82ED0FB0: 91430164  stw r10, 0x164(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(356 as u32), ctx.r[10].u32 ) };
	// 82ED0FB4: 91630168  stw r11, 0x168(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(360 as u32), ctx.r[11].u32 ) };
	// 82ED0FB8: 9163016C  stw r11, 0x16c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(364 as u32), ctx.r[11].u32 ) };
	// 82ED0FBC: 91430170  stw r10, 0x170(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(368 as u32), ctx.r[10].u32 ) };
	// 82ED0FC0: 91630174  stw r11, 0x174(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(372 as u32), ctx.r[11].u32 ) };
	// 82ED0FC4: 91630178  stw r11, 0x178(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(376 as u32), ctx.r[11].u32 ) };
	// 82ED0FC8: 9143017C  stw r10, 0x17c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(380 as u32), ctx.r[10].u32 ) };
	// 82ED0FCC: 91630180  stw r11, 0x180(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(384 as u32), ctx.r[11].u32 ) };
	// 82ED0FD0: 91630184  stw r11, 0x184(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(388 as u32), ctx.r[11].u32 ) };
	// 82ED0FD4: 91430188  stw r10, 0x188(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(392 as u32), ctx.r[10].u32 ) };
	// 82ED0FD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


