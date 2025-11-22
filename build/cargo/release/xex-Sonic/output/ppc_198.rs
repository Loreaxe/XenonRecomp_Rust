pub fn sub_82E68620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E68620 size=24
    let mut pc: u32 = 0x82E68620;
    'dispatch: loop {
        match pc {
            0x82E68620 => {
    //   block [0x82E68620..0x82E68638)
	// 82E68620: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82E68624: 419A0014  beq cr6, 0x82e68638
	if ctx.cr[6].eq {
		sub_82E68638(ctx, base);
		return;
	}
	// 82E68628: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E6862C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E68630: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E68634: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E68638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E68638 size=8
    let mut pc: u32 = 0x82E68638;
    'dispatch: loop {
        match pc {
            0x82E68638 => {
    //   block [0x82E68638..0x82E68640)
	// 82E68638: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E6863C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E68640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E68640 size=88
    let mut pc: u32 = 0x82E68640;
    'dispatch: loop {
        match pc {
            0x82E68640 => {
    //   block [0x82E68640..0x82E68698)
	// 82E68640: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E68644: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E68648: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E6864C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E68650: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E68654: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E68658: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E6865C: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E68660: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E68664: 419A0008  beq cr6, 0x82e6866c
	if ctx.cr[6].eq {
	pc = 0x82E6866C; continue 'dispatch;
	}
	// 82E68668: 4B458229  bl 0x822c0890
	ctx.lr = 0x82E6866C;
	sub_822C0890(ctx, base);
	// 82E6866C: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E68670: 4182000C  beq 0x82e6867c
	if ctx.cr[0].eq {
	pc = 0x82E6867C; continue 'dispatch;
	}
	// 82E68674: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E68678: 4B457BF1  bl 0x822c0268
	ctx.lr = 0x82E6867C;
	sub_822C0268(ctx, base);
	// 82E6867C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E68680: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E68684: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E68688: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E6868C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E68690: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E68694: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E68698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E68698 size=160
    let mut pc: u32 = 0x82E68698;
    'dispatch: loop {
        match pc {
            0x82E68698 => {
    //   block [0x82E68698..0x82E68738)
	// 82E68698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6869C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E686A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E686A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E686A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E686AC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E686B0: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E686B4: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E686B8: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82E686BC: 4800005C  b 0x82e68718
	pc = 0x82E68718; continue 'dispatch;
	// 82E686C0: 83EB0014  lwz r31, 0x14(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E686C4: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E686C8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E686CC: 419A0024  beq cr6, 0x82e686f0
	if ctx.cr[6].eq {
	pc = 0x82E686F0; continue 'dispatch;
	}
	// 82E686D0: 395F0004  addi r10, r31, 4
	ctx.r[10].s64 = ctx.r[31].s64 + 4;
	// 82E686D4: 7D0000A6  mfmsr r8
	ctx.r[8].u64 = ctx.msr;
	// 82E686D8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E686DC: 7D205028  lwarx r9, 0, r10
	// lwarx
	let ea = ctx.r[10].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[9].u64 = ctx.reserved.u32 as u64;
	// 82E686E0: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82E686E4: 7D20512D  stwcx. r9, 0, r10
	// stwcx.
	let addr = ctx.r[10].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[9].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82E686E8: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E686EC: 4082FFE8  bne 0x82e686d4
	if !ctx.cr[0].eq {
	pc = 0x82E686D4; continue 'dispatch;
	}
	// 82E686F0: 386B0038  addi r3, r11, 0x38
	ctx.r[3].s64 = ctx.r[11].s64 + 56;
	// 82E686F4: 480058D5  bl 0x82e6dfc8
	ctx.lr = 0x82E686F8;
	sub_82E6DFC8(ctx, base);
	// 82E686F8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E686FC: 419A000C  beq cr6, 0x82e68708
	if ctx.cr[6].eq {
	pc = 0x82E68708; continue 'dispatch;
	}
	// 82E68700: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E68704: 4B45818D  bl 0x822c0890
	ctx.lr = 0x82E68708;
	sub_822C0890(ctx, base);
	// 82E68708: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E6870C: 4825711D  bl 0x830bf828
	ctx.lr = 0x82E68710;
	sub_830BF828(ctx, base);
	// 82E68710: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E68714: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E68718: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82E6871C: 409AFFA4  bne cr6, 0x82e686c0
	if !ctx.cr[6].eq {
	pc = 0x82E686C0; continue 'dispatch;
	}
	// 82E68720: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E68724: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E68728: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E6872C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E68730: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E68734: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E68738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E68738 size=180
    let mut pc: u32 = 0x82E68738;
    'dispatch: loop {
        match pc {
            0x82E68738 => {
    //   block [0x82E68738..0x82E687EC)
	// 82E68738: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6873C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E68740: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E68744: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E68748: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E6874C: 9081008C  stw r4, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[4].u32 ) };
	// 82E68750: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82E68754: 419A0014  beq cr6, 0x82e68768
	if ctx.cr[6].eq {
	pc = 0x82E68768; continue 'dispatch;
	}
	// 82E68758: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E6875C: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E68760: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82E68764: 40990008  ble cr6, 0x82e6876c
	if !ctx.cr[6].gt {
	pc = 0x82E6876C; continue 'dispatch;
	}
	// 82E68768: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E6876C: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82E68770: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82E68774: 409A0060  bne cr6, 0x82e687d4
	if !ctx.cr[6].eq {
	pc = 0x82E687D4; continue 'dispatch;
	}
	// 82E68778: 38A1008C  addi r5, r1, 0x8c
	ctx.r[5].s64 = ctx.r[1].s64 + 140;
	// 82E6877C: 389F0004  addi r4, r31, 4
	ctx.r[4].s64 = ctx.r[31].s64 + 4;
	// 82E68780: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E68784: 4BFACBF5  bl 0x82e15378
	ctx.lr = 0x82E68788;
	sub_82E15378(ctx, base);
	// 82E68788: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6878C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E68790: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82E68794: 419A0040  beq cr6, 0x82e687d4
	if ctx.cr[6].eq {
	pc = 0x82E687D4; continue 'dispatch;
	}
	// 82E68798: 806B0014  lwz r3, 0x14(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E6879C: 83EB0010  lwz r31, 0x10(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E687A0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E687A4: 419A0028  beq cr6, 0x82e687cc
	if ctx.cr[6].eq {
	pc = 0x82E687CC; continue 'dispatch;
	}
	// 82E687A8: 39630004  addi r11, r3, 4
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	// 82E687AC: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82E687B0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E687B4: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82E687B8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E687BC: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82E687C0: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E687C4: 4082FFE8  bne 0x82e687ac
	if !ctx.cr[0].eq {
	pc = 0x82E687AC; continue 'dispatch;
	}
	// 82E687C8: 4B4580C9  bl 0x822c0890
	ctx.lr = 0x82E687CC;
	sub_822C0890(ctx, base);
	// 82E687CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E687D0: 48000008  b 0x82e687d8
	pc = 0x82E687D8; continue 'dispatch;
	// 82E687D4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E687D8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E687DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E687E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E687E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E687E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E687F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E687F0 size=1028
    let mut pc: u32 = 0x82E687F0;
    'dispatch: loop {
        match pc {
            0x82E687F0 => {
    //   block [0x82E687F0..0x82E68BF4)
	// 82E687F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E687F4: 4833F965  bl 0x831a8158
	ctx.lr = 0x82E687F8;
	sub_831A8130(ctx, base);
	// 82E687F8: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E687FC: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82E68800: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82E68804: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82E68808: 93E10104  stw r31, 0x104(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(260 as u32), ctx.r[31].u32 ) };
	// 82E6880C: 897F0019  lbz r11, 0x19(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(25 as u32) ) } as u64;
	// 82E68810: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E68814: 419A0048  beq cr6, 0x82e6885c
	if ctx.cr[6].eq {
	pc = 0x82E6885C; continue 'dispatch;
	}
	// 82E68818: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82E6881C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E68820: 388B9620  addi r4, r11, -0x69e0
	ctx.r[4].s64 = ctx.r[11].s64 + -27104;
	// 82E68824: 4B45D0A5  bl 0x822c58c8
	ctx.lr = 0x82E68828;
	sub_822C58C8(ctx, base);
	// 82E68828: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E6882C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82E68830: 4B461681  bl 0x822c9eb0
	ctx.lr = 0x82E68834;
	sub_822C9EB0(ctx, base);
	// 82E68834: 4B45BA7D  bl 0x822c42b0
	ctx.lr = 0x82E68838;
	sub_822C42B0(ctx, base);
	// 82E68838: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82E6883C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82E68840: 396B9600  addi r11, r11, -0x6a00
	ctx.r[11].s64 = ctx.r[11].s64 + -27136;
	// 82E68844: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82E68848: 4B45CC29  bl 0x822c5470
	ctx.lr = 0x82E6884C;
	sub_822C5470(ctx, base);
	// 82E6884C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82E68850: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E68854: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E68858: 4B45C489  bl 0x822c4ce0
	ctx.lr = 0x82E6885C;
	sub_822C4CE0(ctx, base);
	// 82E6885C: 38610104  addi r3, r1, 0x104
	ctx.r[3].s64 = ctx.r[1].s64 + 260;
	// 82E68860: 7FFBFB78  mr r27, r31
	ctx.r[27].u64 = ctx.r[31].u64;
	// 82E68864: 48256FC5  bl 0x830bf828
	ctx.lr = 0x82E68868;
	sub_830BF828(ctx, base);
	// 82E68868: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E6886C: 894B0019  lbz r10, 0x19(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(25 as u32) ) } as u64;
	// 82E68870: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E68874: 83210104  lwz r25, 0x104(r1)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(260 as u32) ) } as u64;
	// 82E68878: 419A000C  beq cr6, 0x82e68884
	if ctx.cr[6].eq {
	pc = 0x82E68884; continue 'dispatch;
	}
	// 82E6887C: 839B0008  lwz r28, 8(r27)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E68880: 48000028  b 0x82e688a8
	pc = 0x82E688A8; continue 'dispatch;
	// 82E68884: 815B0008  lwz r10, 8(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E68888: 894A0019  lbz r10, 0x19(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(25 as u32) ) } as u64;
	// 82E6888C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E68890: 419A000C  beq cr6, 0x82e6889c
	if ctx.cr[6].eq {
	pc = 0x82E6889C; continue 'dispatch;
	}
	// 82E68894: 7D7C5B78  mr r28, r11
	ctx.r[28].u64 = ctx.r[11].u64;
	// 82E68898: 48000010  b 0x82e688a8
	pc = 0x82E688A8; continue 'dispatch;
	// 82E6889C: 83990008  lwz r28, 8(r25)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E688A0: 7F19D840  cmplw cr6, r25, r27
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82E688A4: 409A00DC  bne cr6, 0x82e68980
	if !ctx.cr[6].eq {
	pc = 0x82E68980; continue 'dispatch;
	}
	// 82E688A8: 897C0019  lbz r11, 0x19(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(25 as u32) ) } as u64;
	// 82E688AC: 83FB0004  lwz r31, 4(r27)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E688B0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E688B4: 409A0008  bne cr6, 0x82e688bc
	if !ctx.cr[6].eq {
	pc = 0x82E688BC; continue 'dispatch;
	}
	// 82E688B8: 93FC0004  stw r31, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82E688BC: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E688C0: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E688C4: 7F0AD840  cmplw cr6, r10, r27
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82E688C8: 409A000C  bne cr6, 0x82e688d4
	if !ctx.cr[6].eq {
	pc = 0x82E688D4; continue 'dispatch;
	}
	// 82E688CC: 938B0004  stw r28, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82E688D0: 4800001C  b 0x82e688ec
	pc = 0x82E688EC; continue 'dispatch;
	// 82E688D4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E688D8: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82E688DC: 409A000C  bne cr6, 0x82e688e8
	if !ctx.cr[6].eq {
	pc = 0x82E688E8; continue 'dispatch;
	}
	// 82E688E0: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82E688E4: 48000008  b 0x82e688ec
	pc = 0x82E688EC; continue 'dispatch;
	// 82E688E8: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 82E688EC: 813A0004  lwz r9, 4(r26)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E688F0: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E688F4: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82E688F8: 409A003C  bne cr6, 0x82e68934
	if !ctx.cr[6].eq {
	pc = 0x82E68934; continue 'dispatch;
	}
	// 82E688FC: 897C0019  lbz r11, 0x19(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(25 as u32) ) } as u64;
	// 82E68900: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E68904: 419A000C  beq cr6, 0x82e68910
	if ctx.cr[6].eq {
	pc = 0x82E68910; continue 'dispatch;
	}
	// 82E68908: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 82E6890C: 48000024  b 0x82e68930
	pc = 0x82E68930; continue 'dispatch;
	// 82E68910: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E68914: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 82E68918: 4800000C  b 0x82e68924
	pc = 0x82E68924; continue 'dispatch;
	// 82E6891C: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82E68920: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E68924: 890B0019  lbz r8, 0x19(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(25 as u32) ) } as u64;
	// 82E68928: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82E6892C: 419AFFF0  beq cr6, 0x82e6891c
	if ctx.cr[6].eq {
	pc = 0x82E6891C; continue 'dispatch;
	}
	// 82E68930: 91490000  stw r10, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E68934: 813A0004  lwz r9, 4(r26)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E68938: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6893C: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82E68940: 409A00D4  bne cr6, 0x82e68a14
	if !ctx.cr[6].eq {
	pc = 0x82E68A14; continue 'dispatch;
	}
	// 82E68944: 897C0019  lbz r11, 0x19(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(25 as u32) ) } as u64;
	// 82E68948: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E6894C: 419A000C  beq cr6, 0x82e68958
	if ctx.cr[6].eq {
	pc = 0x82E68958; continue 'dispatch;
	}
	// 82E68950: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 82E68954: 48000024  b 0x82e68978
	pc = 0x82E68978; continue 'dispatch;
	// 82E68958: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6895C: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 82E68960: 4800000C  b 0x82e6896c
	pc = 0x82E6896C; continue 'dispatch;
	// 82E68964: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82E68968: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6896C: 890B0019  lbz r8, 0x19(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(25 as u32) ) } as u64;
	// 82E68970: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82E68974: 419AFFF0  beq cr6, 0x82e68964
	if ctx.cr[6].eq {
	pc = 0x82E68964; continue 'dispatch;
	}
	// 82E68978: 91490008  stw r10, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E6897C: 48000098  b 0x82e68a14
	pc = 0x82E68A14; continue 'dispatch;
	// 82E68980: 932B0004  stw r25, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 82E68984: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E68988: 91790000  stw r11, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E6898C: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E68990: 7F195840  cmplw cr6, r25, r11
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E68994: 409A000C  bne cr6, 0x82e689a0
	if !ctx.cr[6].eq {
	pc = 0x82E689A0; continue 'dispatch;
	}
	// 82E68998: 7F3FCB78  mr r31, r25
	ctx.r[31].u64 = ctx.r[25].u64;
	// 82E6899C: 4800002C  b 0x82e689c8
	pc = 0x82E689C8; continue 'dispatch;
	// 82E689A0: 897C0019  lbz r11, 0x19(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(25 as u32) ) } as u64;
	// 82E689A4: 83F90004  lwz r31, 4(r25)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E689A8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E689AC: 409A0008  bne cr6, 0x82e689b4
	if !ctx.cr[6].eq {
	pc = 0x82E689B4; continue 'dispatch;
	}
	// 82E689B0: 93FC0004  stw r31, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82E689B4: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82E689B8: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E689BC: 91790008  stw r11, 8(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E689C0: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E689C4: 932B0004  stw r25, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 82E689C8: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E689CC: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E689D0: 7F0AD840  cmplw cr6, r10, r27
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82E689D4: 409A000C  bne cr6, 0x82e689e0
	if !ctx.cr[6].eq {
	pc = 0x82E689E0; continue 'dispatch;
	}
	// 82E689D8: 932B0004  stw r25, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 82E689DC: 48000020  b 0x82e689fc
	pc = 0x82E689FC; continue 'dispatch;
	// 82E689E0: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E689E4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E689E8: 7F0AD840  cmplw cr6, r10, r27
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82E689EC: 409A000C  bne cr6, 0x82e689f8
	if !ctx.cr[6].eq {
	pc = 0x82E689F8; continue 'dispatch;
	}
	// 82E689F0: 932B0000  stw r25, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 82E689F4: 48000008  b 0x82e689fc
	pc = 0x82E689FC; continue 'dispatch;
	// 82E689F8: 932B0008  stw r25, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[25].u32 ) };
	// 82E689FC: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E68A00: 91790004  stw r11, 4(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E68A04: 897B0018  lbz r11, 0x18(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E68A08: 89590018  lbz r10, 0x18(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[25].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E68A0C: 99790018  stb r11, 0x18(r25)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[25].u32.wrapping_add(24 as u32), ctx.r[11].u8 ) };
	// 82E68A10: 995B0018  stb r10, 0x18(r27)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[27].u32.wrapping_add(24 as u32), ctx.r[10].u8 ) };
	// 82E68A14: 897B0018  lbz r11, 0x18(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E68A18: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82E68A1C: 409A0198  bne cr6, 0x82e68bb4
	if !ctx.cr[6].eq {
	pc = 0x82E68BB4; continue 'dispatch;
	}
	// 82E68A20: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E68A24: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82E68A28: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E68A2C: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E68A30: 419A0180  beq cr6, 0x82e68bb0
	if ctx.cr[6].eq {
	pc = 0x82E68BB0; continue 'dispatch;
	}
	// 82E68A34: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82E68A38: 897C0018  lbz r11, 0x18(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E68A3C: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82E68A40: 409A0170  bne cr6, 0x82e68bb0
	if !ctx.cr[6].eq {
	pc = 0x82E68BB0; continue 'dispatch;
	}
	// 82E68A44: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E68A48: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E68A4C: 409A00A8  bne cr6, 0x82e68af4
	if !ctx.cr[6].eq {
	pc = 0x82E68AF4; continue 'dispatch;
	}
	// 82E68A50: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E68A54: 894B0018  lbz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E68A58: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E68A5C: 409A001C  bne cr6, 0x82e68a78
	if !ctx.cr[6].eq {
	pc = 0x82E68A78; continue 'dispatch;
	}
	// 82E68A60: 9BCB0018  stb r30, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[30].u8 ) };
	// 82E68A64: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E68A68: 9BBF0018  stb r29, 0x18(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[29].u8 ) };
	// 82E68A6C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82E68A70: 4BFA97C9  bl 0x82e12238
	ctx.lr = 0x82E68A74;
	sub_82E12238(ctx, base);
	// 82E68A74: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E68A78: 894B0019  lbz r10, 0x19(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(25 as u32) ) } as u64;
	// 82E68A7C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E68A80: 409A00C8  bne cr6, 0x82e68b48
	if !ctx.cr[6].eq {
	pc = 0x82E68B48; continue 'dispatch;
	}
	// 82E68A84: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E68A88: 894A0018  lbz r10, 0x18(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E68A8C: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82E68A90: 409A0014  bne cr6, 0x82e68aa4
	if !ctx.cr[6].eq {
	pc = 0x82E68AA4; continue 'dispatch;
	}
	// 82E68A94: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E68A98: 894A0018  lbz r10, 0x18(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E68A9C: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82E68AA0: 419A00A4  beq cr6, 0x82e68b44
	if ctx.cr[6].eq {
	pc = 0x82E68B44; continue 'dispatch;
	}
	// 82E68AA4: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E68AA8: 894A0018  lbz r10, 0x18(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E68AAC: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82E68AB0: 409A0020  bne cr6, 0x82e68ad0
	if !ctx.cr[6].eq {
	pc = 0x82E68AD0; continue 'dispatch;
	}
	// 82E68AB4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E68AB8: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82E68ABC: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82E68AC0: 9BCA0018  stb r30, 0x18(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(24 as u32), ctx.r[30].u8 ) };
	// 82E68AC4: 9BAB0018  stb r29, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[29].u8 ) };
	// 82E68AC8: 4BFA97D9  bl 0x82e122a0
	ctx.lr = 0x82E68ACC;
	sub_82E122A0(ctx, base);
	// 82E68ACC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E68AD0: 895F0018  lbz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E68AD4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E68AD8: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82E68ADC: 994B0018  stb r10, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[10].u8 ) };
	// 82E68AE0: 9BDF0018  stb r30, 0x18(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u8 ) };
	// 82E68AE4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E68AE8: 9BCB0018  stb r30, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[30].u8 ) };
	// 82E68AEC: 4BFA974D  bl 0x82e12238
	ctx.lr = 0x82E68AF0;
	sub_82E12238(ctx, base);
	// 82E68AF0: 480000C0  b 0x82e68bb0
	pc = 0x82E68BB0; continue 'dispatch;
	// 82E68AF4: 894B0018  lbz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E68AF8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E68AFC: 409A001C  bne cr6, 0x82e68b18
	if !ctx.cr[6].eq {
	pc = 0x82E68B18; continue 'dispatch;
	}
	// 82E68B00: 9BCB0018  stb r30, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[30].u8 ) };
	// 82E68B04: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E68B08: 9BBF0018  stb r29, 0x18(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[29].u8 ) };
	// 82E68B0C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82E68B10: 4BFA9791  bl 0x82e122a0
	ctx.lr = 0x82E68B14;
	sub_82E122A0(ctx, base);
	// 82E68B14: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E68B18: 894B0019  lbz r10, 0x19(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(25 as u32) ) } as u64;
	// 82E68B1C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E68B20: 409A0028  bne cr6, 0x82e68b48
	if !ctx.cr[6].eq {
	pc = 0x82E68B48; continue 'dispatch;
	}
	// 82E68B24: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E68B28: 894A0018  lbz r10, 0x18(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E68B2C: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82E68B30: 409A0034  bne cr6, 0x82e68b64
	if !ctx.cr[6].eq {
	pc = 0x82E68B64; continue 'dispatch;
	}
	// 82E68B34: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E68B38: 894A0018  lbz r10, 0x18(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E68B3C: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82E68B40: 409A0024  bne cr6, 0x82e68b64
	if !ctx.cr[6].eq {
	pc = 0x82E68B64; continue 'dispatch;
	}
	// 82E68B44: 9BAB0018  stb r29, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[29].u8 ) };
	// 82E68B48: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E68B4C: 7FFCFB78  mr r28, r31
	ctx.r[28].u64 = ctx.r[31].u64;
	// 82E68B50: 83FF0004  lwz r31, 4(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E68B54: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E68B58: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E68B5C: 409AFEDC  bne cr6, 0x82e68a38
	if !ctx.cr[6].eq {
	pc = 0x82E68A38; continue 'dispatch;
	}
	// 82E68B60: 48000050  b 0x82e68bb0
	pc = 0x82E68BB0; continue 'dispatch;
	// 82E68B64: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E68B68: 894A0018  lbz r10, 0x18(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E68B6C: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82E68B70: 409A0020  bne cr6, 0x82e68b90
	if !ctx.cr[6].eq {
	pc = 0x82E68B90; continue 'dispatch;
	}
	// 82E68B74: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E68B78: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82E68B7C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82E68B80: 9BCA0018  stb r30, 0x18(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(24 as u32), ctx.r[30].u8 ) };
	// 82E68B84: 9BAB0018  stb r29, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[29].u8 ) };
	// 82E68B88: 4BFA96B1  bl 0x82e12238
	ctx.lr = 0x82E68B8C;
	sub_82E12238(ctx, base);
	// 82E68B8C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E68B90: 895F0018  lbz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E68B94: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E68B98: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82E68B9C: 994B0018  stb r10, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[10].u8 ) };
	// 82E68BA0: 9BDF0018  stb r30, 0x18(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u8 ) };
	// 82E68BA4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E68BA8: 9BCB0018  stb r30, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[30].u8 ) };
	// 82E68BAC: 4BFA96F5  bl 0x82e122a0
	ctx.lr = 0x82E68BB0;
	sub_82E122A0(ctx, base);
	// 82E68BB0: 9BDC0018  stb r30, 0x18(r28)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[28].u32.wrapping_add(24 as u32), ctx.r[30].u8 ) };
	// 82E68BB4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E68BB8: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82E68BBC: 4BFFFA85  bl 0x82e68640
	ctx.lr = 0x82E68BC0;
	sub_82E68640(ctx, base);
	// 82E68BC0: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82E68BC4: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82E68BC8: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82E68BCC: 4BF895BD  bl 0x82df2188
	ctx.lr = 0x82E68BD0;
	sub_82DF2188(ctx, base);
	// 82E68BD0: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E68BD4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E68BD8: 419A000C  beq cr6, 0x82e68be4
	if ctx.cr[6].eq {
	pc = 0x82E68BE4; continue 'dispatch;
	}
	// 82E68BDC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82E68BE0: 917A0008  stw r11, 8(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E68BE4: 93380000  stw r25, 0(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 82E68BE8: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82E68BEC: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 82E68BF0: 4833F5B8  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E68BF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E68BF8 size=132
    let mut pc: u32 = 0x82E68BF8;
    'dispatch: loop {
        match pc {
            0x82E68BF8 => {
    //   block [0x82E68BF8..0x82E68C7C)
	// 82E68BF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E68BFC: 4833F56D  bl 0x831a8168
	ctx.lr = 0x82E68C00;
	sub_831A8130(ctx, base);
	// 82E68C00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E68C04: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E68C08: 90A100A4  stw r5, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[5].u32 ) };
	// 82E68C0C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E68C10: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82E68C14: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E68C18: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E68C1C: 7F055040  cmplw cr6, r5, r10
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82E68C20: 409A0044  bne cr6, 0x82e68c64
	if !ctx.cr[6].eq {
	pc = 0x82E68C64; continue 'dispatch;
	}
	// 82E68C24: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E68C28: 409A003C  bne cr6, 0x82e68c64
	if !ctx.cr[6].eq {
	pc = 0x82E68C64; continue 'dispatch;
	}
	// 82E68C2C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E68C30: 4BF96D59  bl 0x82dff988
	ctx.lr = 0x82E68C34;
	sub_82DFF988(ctx, base);
	// 82E68C34: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E68C38: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E68C3C: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E68C40: 48000030  b 0x82e68c70
	pc = 0x82E68C70; continue 'dispatch;
	// 82E68C44: 386100A4  addi r3, r1, 0xa4
	ctx.r[3].s64 = ctx.r[1].s64 + 164;
	// 82E68C48: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82E68C4C: 48256BDD  bl 0x830bf828
	ctx.lr = 0x82E68C50;
	sub_830BF828(ctx, base);
	// 82E68C50: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82E68C54: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E68C58: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E68C5C: 4BFFFB95  bl 0x82e687f0
	ctx.lr = 0x82E68C60;
	sub_82E687F0(ctx, base);
	// 82E68C60: 80A100A4  lwz r5, 0xa4(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 82E68C64: 7F05F040  cmplw cr6, r5, r30
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82E68C68: 409AFFDC  bne cr6, 0x82e68c44
	if !ctx.cr[6].eq {
	pc = 0x82E68C44; continue 'dispatch;
	}
	// 82E68C6C: 90BD0000  stw r5, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 82E68C70: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E68C74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E68C78: 4833F540  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E68C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E68C80 size=20
    let mut pc: u32 = 0x82E68C80;
    'dispatch: loop {
        match pc {
            0x82E68C80 => {
    //   block [0x82E68C80..0x82E68C94)
	// 82E68C80: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E68C84: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E68C88: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 82E68C8C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E68C90: 4BF96CF8  b 0x82dff988
	sub_82DFF988(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E68C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E68C98 size=64
    let mut pc: u32 = 0x82E68C98;
    'dispatch: loop {
        match pc {
            0x82E68C98 => {
    //   block [0x82E68C98..0x82E68CD8)
	// 82E68C98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E68C9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E68CA0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E68CA4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E68CA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E68CAC: 3BE30004  addi r31, r3, 4
	ctx.r[31].s64 = ctx.r[3].s64 + 4;
	// 82E68CB0: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E68CB4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E68CB8: 4BF96CD1  bl 0x82dff988
	ctx.lr = 0x82E68CBC;
	sub_82DFF988(ctx, base);
	// 82E68CBC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E68CC0: 4825AD81  bl 0x830c3a40
	ctx.lr = 0x82E68CC4;
	sub_830C3A40(ctx, base);
	// 82E68CC4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E68CC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E68CCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E68CD0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E68CD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E68CD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E68CD8 size=60
    let mut pc: u32 = 0x82E68CD8;
    'dispatch: loop {
        match pc {
            0x82E68CD8 => {
    //   block [0x82E68CD8..0x82E68D14)
	// 82E68CD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E68CDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E68CE0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E68CE4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E68CE8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E68CEC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E68CF0: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82E68CF4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E68CF8: 4825A621  bl 0x830c3318
	ctx.lr = 0x82E68CFC;
	sub_830C3318(ctx, base);
	// 82E68CFC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E68D00: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E68D04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E68D08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E68D0C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E68D10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E68D18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E68D18 size=100
    let mut pc: u32 = 0x82E68D18;
    'dispatch: loop {
        match pc {
            0x82E68D18 => {
    //   block [0x82E68D18..0x82E68D7C)
	// 82E68D18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E68D1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E68D20: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E68D24: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E68D28: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E68D2C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E68D30: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E68D34: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 82E68D38: 396BD0E4  addi r11, r11, -0x2f1c
	ctx.r[11].s64 = ctx.r[11].s64 + -12060;
	// 82E68D3C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E68D40: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E68D44: 4BF8A6E5  bl 0x82df3428
	ctx.lr = 0x82E68D48;
	sub_82DF3428(ctx, base);
	// 82E68D48: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82E68D4C: 4BF8A6DD  bl 0x82df3428
	ctx.lr = 0x82E68D50;
	sub_82DF3428(ctx, base);
	// 82E68D50: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E68D54: 4182000C  beq 0x82e68d60
	if ctx.cr[0].eq {
	pc = 0x82E68D60; continue 'dispatch;
	}
	// 82E68D58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E68D5C: 4B45750D  bl 0x822c0268
	ctx.lr = 0x82E68D60;
	sub_822C0268(ctx, base);
	// 82E68D60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E68D64: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E68D68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E68D6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E68D70: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E68D74: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E68D78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E68D80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E68D80 size=68
    let mut pc: u32 = 0x82E68D80;
    'dispatch: loop {
        match pc {
            0x82E68D80 => {
    //   block [0x82E68D80..0x82E68DC4)
	// 82E68D80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E68D84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E68D88: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E68D8C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E68D90: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E68D94: 483930D5  bl 0x831fbe68
	ctx.lr = 0x82E68D98;
	sub_831FBE68(ctx, base);
	// 82E68D98: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E68D9C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E68DA0: 396BD10C  addi r11, r11, -0x2ef4
	ctx.r[11].s64 = ctx.r[11].s64 + -12020;
	// 82E68DA4: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82E68DA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E68DAC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E68DB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E68DB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E68DB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E68DBC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E68DC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E68DC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E68DC8 size=88
    let mut pc: u32 = 0x82E68DC8;
    'dispatch: loop {
        match pc {
            0x82E68DC8 => {
    //   block [0x82E68DC8..0x82E68E20)
	// 82E68DC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E68DCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E68DD0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E68DD4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E68DD8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E68DDC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E68DE0: 48006139  bl 0x82e6ef18
	ctx.lr = 0x82E68DE4;
	sub_82E6EF18(ctx, base);
	// 82E68DE4: 907E0010  stw r3, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 82E68DE8: 48000691  bl 0x82e69478
	ctx.lr = 0x82E68DEC;
	sub_82E69478(ctx, base);
	// 82E68DEC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E68DF0: 907E0014  stw r3, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 82E68DF4: 9BFE0018  stb r31, 0x18(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[31].u8 ) };
	// 82E68DF8: 807E0010  lwz r3, 0x10(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E68DFC: 4800067D  bl 0x82e69478
	ctx.lr = 0x82E68E00;
	sub_82E69478(ctx, base);
	// 82E68E00: 907E001C  stw r3, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[3].u32 ) };
	// 82E68E04: 9BFE0019  stb r31, 0x19(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(25 as u32), ctx.r[31].u8 ) };
	// 82E68E08: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E68E0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E68E10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E68E14: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E68E18: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E68E1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E68E20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E68E20 size=96
    let mut pc: u32 = 0x82E68E20;
    'dispatch: loop {
        match pc {
            0x82E68E20 => {
    //   block [0x82E68E20..0x82E68E80)
	// 82E68E20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E68E24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E68E28: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E68E2C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E68E30: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E68E34: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E68E38: 396BD10C  addi r11, r11, -0x2ef4
	ctx.r[11].s64 = ctx.r[11].s64 + -12020;
	// 82E68E3C: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E68E40: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E68E44: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E68E48: 419A0008  beq cr6, 0x82e68e50
	if ctx.cr[6].eq {
	pc = 0x82E68E50; continue 'dispatch;
	}
	// 82E68E4C: 480066C5  bl 0x82e6f510
	ctx.lr = 0x82E68E50;
	sub_82E6F510(ctx, base);
	// 82E68E50: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E68E54: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 82E68E58: 396BD0E4  addi r11, r11, -0x2f1c
	ctx.r[11].s64 = ctx.r[11].s64 + -12060;
	// 82E68E5C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E68E60: 4BF8A5C9  bl 0x82df3428
	ctx.lr = 0x82E68E64;
	sub_82DF3428(ctx, base);
	// 82E68E64: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82E68E68: 4BF8A5C1  bl 0x82df3428
	ctx.lr = 0x82E68E6C;
	sub_82DF3428(ctx, base);
	// 82E68E6C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E68E70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E68E74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E68E78: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E68E7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E68E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E68E80 size=100
    let mut pc: u32 = 0x82E68E80;
    'dispatch: loop {
        match pc {
            0x82E68E80 => {
    //   block [0x82E68E80..0x82E68EE4)
	// 82E68E80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E68E84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E68E88: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E68E8C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E68E90: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E68E94: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E68E98: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82E68E9C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E68EA0: 4BF8A311  bl 0x82df31b0
	ctx.lr = 0x82E68EA4;
	sub_82DF31B0(ctx, base);
	// 82E68EA4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E68EA8: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E68EAC: 48000895  bl 0x82e69740
	ctx.lr = 0x82E68EB0;
	sub_82E69740(ctx, base);
	// 82E68EB0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82E68EB4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E68EB8: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E68EBC: 480008D5  bl 0x82e69790
	ctx.lr = 0x82E68EC0;
	sub_82E69790(ctx, base);
	// 82E68EC0: 3880D8EE  li r4, -0x2712
	ctx.r[4].s64 = -10002;
	// 82E68EC4: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E68EC8: 48000A89  bl 0x82e69950
	ctx.lr = 0x82E68ECC;
	sub_82E69950(ctx, base);
	// 82E68ECC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E68ED0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E68ED4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E68ED8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E68EDC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E68EE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E68EE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E68EE8 size=96
    let mut pc: u32 = 0x82E68EE8;
    'dispatch: loop {
        match pc {
            0x82E68EE8 => {
    //   block [0x82E68EE8..0x82E68F48)
	// 82E68EE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E68EEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E68EF0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E68EF4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E68EF8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E68EFC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E68F00: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82E68F04: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E68F08: 4BF8A2A9  bl 0x82df31b0
	ctx.lr = 0x82E68F0C;
	sub_82DF31B0(ctx, base);
	// 82E68F0C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E68F10: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E68F14: 4800082D  bl 0x82e69740
	ctx.lr = 0x82E68F18;
	sub_82E69740(ctx, base);
	// 82E68F18: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E68F1C: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E68F20: 48000979  bl 0x82e69898
	ctx.lr = 0x82E68F24;
	sub_82E69898(ctx, base);
	// 82E68F24: 3880D8EE  li r4, -0x2712
	ctx.r[4].s64 = -10002;
	// 82E68F28: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E68F2C: 48000A25  bl 0x82e69950
	ctx.lr = 0x82E68F30;
	sub_82E69950(ctx, base);
	// 82E68F30: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E68F34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E68F38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E68F3C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E68F40: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E68F44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E68F48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E68F48 size=120
    let mut pc: u32 = 0x82E68F48;
    'dispatch: loop {
        match pc {
            0x82E68F48 => {
    //   block [0x82E68F48..0x82E68FC0)
	// 82E68F48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E68F4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E68F50: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E68F54: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E68F58: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E68F5C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E68F60: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82E68F64: 4BF8A24D  bl 0x82df31b0
	ctx.lr = 0x82E68F68;
	sub_82DF31B0(ctx, base);
	// 82E68F68: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82E68F6C: 3880D8EE  li r4, -0x2712
	ctx.r[4].s64 = -10002;
	// 82E68F70: 807E0010  lwz r3, 0x10(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E68F74: 48000945  bl 0x82e698b8
	ctx.lr = 0x82E68F78;
	sub_82E698B8(ctx, base);
	// 82E68F78: 3880FFFF  li r4, -1
	ctx.r[4].s64 = -1;
	// 82E68F7C: 807E0010  lwz r3, 0x10(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E68F80: 480005D9  bl 0x82e69558
	ctx.lr = 0x82E68F84;
	sub_82E69558(ctx, base);
	// 82E68F84: 2F030006  cmpwi cr6, r3, 6
	ctx.cr[6].compare_i32(ctx.r[3].s32, 6, &mut ctx.xer);
	// 82E68F88: 409A0030  bne cr6, 0x82e68fb8
	if !ctx.cr[6].eq {
	pc = 0x82E68FB8; continue 'dispatch;
	}
	// 82E68F8C: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 82E68F90: 3880FFFE  li r4, -2
	ctx.r[4].s64 = -2;
	// 82E68F94: 807E0010  lwz r3, 0x10(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E68F98: 48000561  bl 0x82e694f8
	ctx.lr = 0x82E68F9C;
	sub_82E694F8(ctx, base);
	// 82E68F9C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E68FA0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E68FA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E68FA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E68FAC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E68FB0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E68FB4: 4E800020  blr
	return;
	// 82E68FB8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E68FBC: 4BFFFFD4  b 0x82e68f90
	pc = 0x82E68F90; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E68FC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E68FC0 size=100
    let mut pc: u32 = 0x82E68FC0;
    'dispatch: loop {
        match pc {
            0x82E68FC0 => {
    //   block [0x82E68FC0..0x82E69024)
	// 82E68FC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E68FC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E68FC8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E68FCC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E68FD0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E68FD4: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82E68FD8: 4BF8A1D9  bl 0x82df31b0
	ctx.lr = 0x82E68FDC;
	sub_82DF31B0(ctx, base);
	// 82E68FDC: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82E68FE0: 3880D8EE  li r4, -0x2712
	ctx.r[4].s64 = -10002;
	// 82E68FE4: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E68FE8: 480008D1  bl 0x82e698b8
	ctx.lr = 0x82E68FEC;
	sub_82E698B8(ctx, base);
	// 82E68FEC: 3880FFFF  li r4, -1
	ctx.r[4].s64 = -1;
	// 82E68FF0: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E68FF4: 48000565  bl 0x82e69558
	ctx.lr = 0x82E68FF8;
	sub_82E69558(ctx, base);
	// 82E68FF8: 2F030006  cmpwi cr6, r3, 6
	ctx.cr[6].compare_i32(ctx.r[3].s32, 6, &mut ctx.xer);
	// 82E68FFC: 409A0014  bne cr6, 0x82e69010
	if !ctx.cr[6].eq {
	pc = 0x82E69010; continue 'dispatch;
	}
	// 82E69000: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82E69004: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E69008: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E6900C: 48000995  bl 0x82e699a0
	ctx.lr = 0x82E69010;
	sub_82E699A0(ctx, base);
	// 82E69010: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E69014: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E69018: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E6901C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E69020: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E69028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E69028 size=184
    let mut pc: u32 = 0x82E69028;
    'dispatch: loop {
        match pc {
            0x82E69028 => {
    //   block [0x82E69028..0x82E690E0)
	// 82E69028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6902C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E69030: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E69034: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E69038: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E6903C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E69040: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E69044: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E69048: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82E6904C: 409A0070  bne cr6, 0x82e690bc
	if !ctx.cr[6].eq {
	pc = 0x82E690BC; continue 'dispatch;
	}
	// 82E69050: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82E69054: 4BF8AB7D  bl 0x82df3bd0
	ctx.lr = 0x82E69058;
	sub_82DF3BD0(ctx, base);
	// 82E69058: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E6905C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82E69060: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E69064: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E69068: 480009B1  bl 0x82e69a18
	ctx.lr = 0x82E6906C;
	sub_82E69A18(ctx, base);
	// 82E6906C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E69070: 4BF8A141  bl 0x82df31b0
	ctx.lr = 0x82E69074;
	sub_82DF31B0(ctx, base);
	// 82E69074: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82E69078: 3880D8EE  li r4, -0x2712
	ctx.r[4].s64 = -10002;
	// 82E6907C: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E69080: 48000839  bl 0x82e698b8
	ctx.lr = 0x82E69084;
	sub_82E698B8(ctx, base);
	// 82E69084: 3880FFFF  li r4, -1
	ctx.r[4].s64 = -1;
	// 82E69088: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E6908C: 480004CD  bl 0x82e69558
	ctx.lr = 0x82E69090;
	sub_82E69558(ctx, base);
	// 82E69090: 2F030006  cmpwi cr6, r3, 6
	ctx.cr[6].compare_i32(ctx.r[3].s32, 6, &mut ctx.xer);
	// 82E69094: 409A0044  bne cr6, 0x82e690d8
	if !ctx.cr[6].eq {
	pc = 0x82E690D8; continue 'dispatch;
	}
	// 82E69098: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E6909C: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E690A0: 480019A1  bl 0x82e6aa40
	ctx.lr = 0x82E690A4;
	sub_82E6AA40(ctx, base);
	// 82E690A4: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82E690A8: 4082000C  bne 0x82e690b4
	if !ctx.cr[0].eq {
	pc = 0x82E690B4; continue 'dispatch;
	}
	// 82E690AC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E690B0: 997F0018  stb r11, 0x18(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u8 ) };
	// 82E690B4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82E690B8: 997F0018  stb r11, 0x18(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u8 ) };
	// 82E690BC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E690C0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E690C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E690C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E690CC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E690D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E690D4: 4E800020  blr
	return;
	// 82E690D8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E690DC: 4BFFFFE4  b 0x82e690c0
	pc = 0x82E690C0; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E690E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E690E0 size=20
    let mut pc: u32 = 0x82E690E0;
    'dispatch: loop {
        match pc {
            0x82E690E0 => {
    //   block [0x82E690E0..0x82E690F4)
	// 82E690E0: 89630018  lbz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E690E4: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82E690E8: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82E690EC: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 82E690F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E690F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E690F8 size=108
    let mut pc: u32 = 0x82E690F8;
    'dispatch: loop {
        match pc {
            0x82E690F8 => {
    //   block [0x82E690F8..0x82E69164)
	// 82E690F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E690FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E69100: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E69104: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E69108: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E6910C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82E69110: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E69114: 388B9BC9  addi r4, r11, -0x6437
	ctx.r[4].s64 = ctx.r[11].s64 + -25655;
	// 82E69118: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E6911C: 4BF8A8ED  bl 0x82df3a08
	ctx.lr = 0x82E69120;
	sub_82DF3A08(ctx, base);
	// 82E69120: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E69124: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E69128: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E6912C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E69130: 4E800421  bctrl
	ctx.lr = 0x82E69134;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E69134: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E69138: 41820010  beq 0x82e69148
	if ctx.cr[0].eq {
	pc = 0x82E69148; continue 'dispatch;
	}
	// 82E6913C: 389F0008  addi r4, r31, 8
	ctx.r[4].s64 = ctx.r[31].s64 + 8;
	// 82E69140: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E69144: 4BF8AA8D  bl 0x82df3bd0
	ctx.lr = 0x82E69148;
	sub_82DF3BD0(ctx, base);
	// 82E69148: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E6914C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E69150: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E69154: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E69158: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E6915C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E69160: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E69168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E69168 size=144
    let mut pc: u32 = 0x82E69168;
    'dispatch: loop {
        match pc {
            0x82E69168 => {
    //   block [0x82E69168..0x82E691F8)
	// 82E69168: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6916C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E69170: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E69174: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E69178: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E6917C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E69180: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E69184: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E69188: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82E6918C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E69190: 807F001C  lwz r3, 0x1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E69194: 48000885  bl 0x82e69a18
	ctx.lr = 0x82E69198;
	sub_82E69A18(ctx, base);
	// 82E69198: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E6919C: 4BF8A015  bl 0x82df31b0
	ctx.lr = 0x82E691A0;
	sub_82DF31B0(ctx, base);
	// 82E691A0: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82E691A4: 3880D8EE  li r4, -0x2712
	ctx.r[4].s64 = -10002;
	// 82E691A8: 807F001C  lwz r3, 0x1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E691AC: 4800070D  bl 0x82e698b8
	ctx.lr = 0x82E691B0;
	sub_82E698B8(ctx, base);
	// 82E691B0: 3880FFFF  li r4, -1
	ctx.r[4].s64 = -1;
	// 82E691B4: 807F001C  lwz r3, 0x1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E691B8: 480003A1  bl 0x82e69558
	ctx.lr = 0x82E691BC;
	sub_82E69558(ctx, base);
	// 82E691BC: 2F030006  cmpwi cr6, r3, 6
	ctx.cr[6].compare_i32(ctx.r[3].s32, 6, &mut ctx.xer);
	// 82E691C0: 409A001C  bne cr6, 0x82e691dc
	if !ctx.cr[6].eq {
	pc = 0x82E691DC; continue 'dispatch;
	}
	// 82E691C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82E691C8: 807F001C  lwz r3, 0x1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E691CC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E691D0: 480007D1  bl 0x82e699a0
	ctx.lr = 0x82E691D4;
	sub_82E699A0(ctx, base);
	// 82E691D4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E691D8: 48000008  b 0x82e691e0
	pc = 0x82E691E0; continue 'dispatch;
	// 82E691DC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E691E0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E691E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E691E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E691EC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E691F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E691F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E691F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E691F8 size=52
    let mut pc: u32 = 0x82E691F8;
    'dispatch: loop {
        match pc {
            0x82E691F8 => {
    //   block [0x82E691F8..0x82E6922C)
	// 82E691F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E691FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E69200: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E69204: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E69208: 3884000C  addi r4, r4, 0xc
	ctx.r[4].s64 = ctx.r[4].s64 + 12;
	// 82E6920C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E69210: 4BF8A9F1  bl 0x82df3c00
	ctx.lr = 0x82E69214;
	sub_82DF3C00(ctx, base);
	// 82E69214: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E69218: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E6921C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E69220: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E69224: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E69228: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E69230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E69230 size=108
    let mut pc: u32 = 0x82E69230;
    'dispatch: loop {
        match pc {
            0x82E69230 => {
    //   block [0x82E69230..0x82E6929C)
	// 82E69230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E69234: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E69238: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E6923C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E69240: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E69244: 897F0018  lbz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E69248: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82E6924C: 409A003C  bne cr6, 0x82e69288
	if !ctx.cr[6].eq {
	pc = 0x82E69288; continue 'dispatch;
	}
	// 82E69250: 897F0019  lbz r11, 0x19(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(25 as u32) ) } as u64;
	// 82E69254: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E69258: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82E6925C: 409A0008  bne cr6, 0x82e69264
	if !ctx.cr[6].eq {
	pc = 0x82E69264; continue 'dispatch;
	}
	// 82E69260: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E69264: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E69268: 48000601  bl 0x82e69868
	ctx.lr = 0x82E6926C;
	sub_82E69868(ctx, base);
	// 82E6926C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E69270: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E69274: 480017CD  bl 0x82e6aa40
	ctx.lr = 0x82E69278;
	sub_82E6AA40(ctx, base);
	// 82E69278: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82E6927C: 4082000C  bne 0x82e69288
	if !ctx.cr[0].eq {
	pc = 0x82E69288; continue 'dispatch;
	}
	// 82E69280: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E69284: 997F0018  stb r11, 0x18(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u8 ) };
	// 82E69288: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E6928C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E69290: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E69294: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E69298: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E692A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E692A0 size=76
    let mut pc: u32 = 0x82E692A0;
    'dispatch: loop {
        match pc {
            0x82E692A0 => {
    //   block [0x82E692A0..0x82E692EC)
	// 82E692A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E692A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E692A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E692AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E692B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E692B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E692B8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E692BC: 4BFFFB65  bl 0x82e68e20
	ctx.lr = 0x82E692C0;
	sub_82E68E20(ctx, base);
	// 82E692C0: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E692C4: 4182000C  beq 0x82e692d0
	if ctx.cr[0].eq {
	pc = 0x82E692D0; continue 'dispatch;
	}
	// 82E692C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E692CC: 4B456F9D  bl 0x822c0268
	ctx.lr = 0x82E692D0;
	sub_822C0268(ctx, base);
	// 82E692D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E692D4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E692D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E692DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E692E0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E692E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E692E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E692F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E692F0 size=176
    let mut pc: u32 = 0x82E692F0;
    'dispatch: loop {
        match pc {
            0x82E692F0 => {
    //   block [0x82E692F0..0x82E693A0)
	// 82E692F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E692F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E692F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E692FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E69300: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E69304: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E69308: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E6930C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E69310: 80850000  lwz r4, 0(r5)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E69314: 4BFD6BF5  bl 0x82e3ff08
	ctx.lr = 0x82E69318;
	sub_82E3FF08(ctx, base);
	// 82E69318: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E6931C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E69320: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E69324: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E69328: 4BFE1889  bl 0x82e4abb0
	ctx.lr = 0x82E6932C;
	sub_82E4ABB0(ctx, base);
	// 82E6932C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E69330: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E69334: 388BD134  addi r4, r11, -0x2ecc
	ctx.r[4].s64 = ctx.r[11].s64 + -11980;
	// 82E69338: 38A00009  li r5, 9
	ctx.r[5].s64 = 9;
	// 82E6933C: 4833F1D5  bl 0x831a8510
	ctx.lr = 0x82E69340;
	sub_831A8510(ctx, base);
	// 82E69340: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E69344: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 82E69348: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E6934C: 80AB0010  lwz r5, 0x10(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E69350: 808B000C  lwz r4, 0xc(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E69354: 48005AE5  bl 0x82e6ee38
	ctx.lr = 0x82E69358;
	sub_82E6EE38(ctx, base);
	// 82E69358: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E6935C: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E69360: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82E69364: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E69368: 480006B1  bl 0x82e69a18
	ctx.lr = 0x82E6936C;
	sub_82E69A18(ctx, base);
	// 82E6936C: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E69370: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82E69374: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E69378: 997F0004  stb r11, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u8 ) };
	// 82E6937C: 419A0008  beq cr6, 0x82e69384
	if ctx.cr[6].eq {
	pc = 0x82E69384; continue 'dispatch;
	}
	// 82E69380: 4B457511  bl 0x822c0890
	ctx.lr = 0x82E69384;
	sub_822C0890(ctx, base);
	// 82E69384: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E69388: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E6938C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E69390: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E69394: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E69398: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E6939C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E693A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E693A0 size=40
    let mut pc: u32 = 0x82E693A0;
    'dispatch: loop {
        match pc {
            0x82E693A0 => {
    //   block [0x82E693A0..0x82E693C8)
	// 82E693A0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E693A4: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82E693A8: 4099002C  ble cr6, 0x82e693d4
	if !ctx.cr[6].gt {
		sub_82E693D4(ctx, base);
		return;
	}
	// 82E693AC: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E693B0: 54892036  slwi r9, r4, 4
	ctx.r[9].u32 = ctx.r[4].u32.wrapping_shl(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82E693B4: 810B0008  lwz r8, 8(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E693B8: 7D6A4A14  add r11, r10, r9
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82E693BC: 386BFFF0  addi r3, r11, -0x10
	ctx.r[3].s64 = ctx.r[11].s64 + -16;
	// 82E693C0: 7F034040  cmplw cr6, r3, r8
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82E693C4: 4D980020  bltlr cr6
	if ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E693C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E693C8 size=12
    let mut pc: u32 = 0x82E693C8;
    'dispatch: loop {
        match pc {
            0x82E693C8 => {
    //   block [0x82E693C8..0x82E693D4)
	// 82E693C8: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E693CC: 386BD398  addi r3, r11, -0x2c68
	ctx.r[3].s64 = ctx.r[11].s64 + -11368;
	// 82E693D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E693D4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E693D4 size=24
    let mut pc: u32 = 0x82E693D4;
    'dispatch: loop {
        match pc {
            0x82E693D4 => {
    //   block [0x82E693D4..0x82E693EC)
	// 82E693D4: 2F04D8F0  cmpwi cr6, r4, -0x2710
	ctx.cr[6].compare_i32(ctx.r[4].s32, -10000, &mut ctx.xer);
	// 82E693D8: 40990014  ble cr6, 0x82e693ec
	if !ctx.cr[6].gt {
		sub_82E693EC(ctx, base);
		return;
	}
	// 82E693DC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E693E0: 548A2036  slwi r10, r4, 4
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E693E4: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82E693E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E693EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E693EC size=68
    let mut pc: u32 = 0x82E693EC;
    'dispatch: loop {
        match pc {
            0x82E693EC => {
    //   block [0x82E693EC..0x82E69430)
	// 82E693EC: 2F04D8EE  cmpwi cr6, r4, -0x2712
	ctx.cr[6].compare_i32(ctx.r[4].s32, -10002, &mut ctx.xer);
	// 82E693F0: 419A0070  beq cr6, 0x82e69460
	if ctx.cr[6].eq {
		sub_82E69460(ctx, base);
		return;
	}
	// 82E693F4: 2F04D8EF  cmpwi cr6, r4, -0x2711
	ctx.cr[6].compare_i32(ctx.r[4].s32, -10001, &mut ctx.xer);
	// 82E693F8: 419A0044  beq cr6, 0x82e6943c
	if ctx.cr[6].eq {
		sub_82E6943C(ctx, base);
		return;
	}
	// 82E693FC: 2F04D8F0  cmpwi cr6, r4, -0x2710
	ctx.cr[6].compare_i32(ctx.r[4].s32, -10000, &mut ctx.xer);
	// 82E69400: 419A0030  beq cr6, 0x82e69430
	if ctx.cr[6].eq {
		sub_82E69430(ctx, base);
		return;
	}
	// 82E69404: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E69408: 2164D8EE  subfic r11, r4, -0x2712
	ctx.xer.ca = ctx.r[4].u32 <= -10002 as u32;
	ctx.r[11].s64 = (-10002 as i64) - ctx.r[4].s64;
	// 82E6940C: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E69410: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E69414: 892A0007  lbz r9, 7(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(7 as u32) ) } as u64;
	// 82E69418: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82E6941C: 4199FFAC  bgt cr6, 0x82e693c8
	if ctx.cr[6].gt {
		sub_82E693C8(ctx, base);
		return;
	}
	// 82E69420: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E69424: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82E69428: 386B0008  addi r3, r11, 8
	ctx.r[3].s64 = ctx.r[11].s64 + 8;
	// 82E6942C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E69430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E69430 size=12
    let mut pc: u32 = 0x82E69430;
    'dispatch: loop {
        match pc {
            0x82E69430 => {
    //   block [0x82E69430..0x82E6943C)
	// 82E69430: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E69434: 386B0060  addi r3, r11, 0x60
	ctx.r[3].s64 = ctx.r[11].s64 + 96;
	// 82E69438: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6943C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E6943C size=36
    let mut pc: u32 = 0x82E6943C;
    'dispatch: loop {
        match pc {
            0x82E6943C => {
    //   block [0x82E6943C..0x82E69460)
	// 82E6943C: 386B0058  addi r3, r11, 0x58
	ctx.r[3].s64 = ctx.r[11].s64 + 88;
	// 82E69440: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E69444: 39400005  li r10, 5
	ctx.r[10].s64 = 5;
	// 82E69448: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E6944C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E69450: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E69454: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E69458: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E6945C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E69460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E69460 size=8
    let mut pc: u32 = 0x82E69460;
    'dispatch: loop {
        match pc {
            0x82E69460 => {
    //   block [0x82E69460..0x82E69468)
	// 82E69460: 386B0048  addi r3, r11, 0x48
	ctx.r[3].s64 = ctx.r[11].s64 + 72;
	// 82E69464: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E69468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E69468 size=16
    let mut pc: u32 = 0x82E69468;
    'dispatch: loop {
        match pc {
            0x82E69468 => {
    //   block [0x82E69468..0x82E69478)
	// 82E69468: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E6946C: 806B0058  lwz r3, 0x58(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E69470: 908B0058  stw r4, 0x58(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(88 as u32), ctx.r[4].u32 ) };
	// 82E69474: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E69478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E69478 size=100
    let mut pc: u32 = 0x82E69478;
    'dispatch: loop {
        match pc {
            0x82E69478 => {
    //   block [0x82E69478..0x82E694DC)
	// 82E69478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6947C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E69480: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E69484: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E69488: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E6948C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E69490: 814B0044  lwz r10, 0x44(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 82E69494: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 82E69498: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E6949C: 41980008  blt cr6, 0x82e694a4
	if ctx.cr[6].lt {
	pc = 0x82E694A4; continue 'dispatch;
	}
	// 82E694A0: 48007A31  bl 0x82e70ed0
	ctx.lr = 0x82E694A4;
	sub_82E70ED0(ctx, base);
	// 82E694A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E694A8: 48005D11  bl 0x82e6f1b8
	ctx.lr = 0x82E694AC;
	sub_82E6F1B8(ctx, base);
	// 82E694AC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E694B0: 39400008  li r10, 8
	ctx.r[10].s64 = 8;
	// 82E694B4: 906B0000  stw r3, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82E694B8: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E694BC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E694C0: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82E694C4: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E694C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E694CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E694D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E694D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E694D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E694E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E694E0 size=20
    let mut pc: u32 = 0x82E694E0;
    'dispatch: loop {
        match pc {
            0x82E694E0 => {
    //   block [0x82E694E0..0x82E694F4)
	// 82E694E0: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E694E4: 8143000C  lwz r10, 0xc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E694E8: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82E694EC: 7D632670  srawi r3, r11, 4
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[3].s64 = (ctx.r[11].s32 >> 4) as i64;
	// 82E694F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E694F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E694F8 size=16
    let mut pc: u32 = 0x82E694F8;
    'dispatch: loop {
        match pc {
            0x82E694F8 => {
    //   block [0x82E694F8..0x82E69508)
	// 82E694F8: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82E694FC: 41980044  blt cr6, 0x82e69540
	if ctx.cr[6].lt {
		sub_82E69540(ctx, base);
		return;
	}
	// 82E69500: 548B2036  slwi r11, r4, 4
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E69504: 4800001C  b 0x82e69520
	sub_82E69508(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E69508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E69508 size=56
    let mut pc: u32 = 0x82E69508;
    'dispatch: loop {
        match pc {
            0x82E69508 => {
    //   block [0x82E69508..0x82E69540)
	// 82E69508: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6950C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82E69510: 912A0008  stw r9, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82E69514: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E69518: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 82E6951C: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E69520: 8143000C  lwz r10, 0xc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E69524: 81230008  lwz r9, 8(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E69528: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82E6952C: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82E69530: 4198FFD8  blt cr6, 0x82e69508
	if ctx.cr[6].lt {
	pc = 0x82E69508; continue 'dispatch;
	}
	// 82E69534: 8143000C  lwz r10, 0xc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E69538: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82E6953C: 48000014  b 0x82e69550
	sub_82E69540(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E69540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E69540 size=24
    let mut pc: u32 = 0x82E69540;
    'dispatch: loop {
        match pc {
            0x82E69540 => {
    //   block [0x82E69540..0x82E69558)
	// 82E69540: 39640001  addi r11, r4, 1
	ctx.r[11].s64 = ctx.r[4].s64 + 1;
	// 82E69544: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E69548: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E6954C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82E69550: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E69554: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E69558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E69558 size=60
    let mut pc: u32 = 0x82E69558;
    'dispatch: loop {
        match pc {
            0x82E69558 => {
    //   block [0x82E69558..0x82E69594)
	// 82E69558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6955C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E69560: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E69564: 4BFFFE3D  bl 0x82e693a0
	ctx.lr = 0x82E69568;
	sub_82E693A0(ctx, base);
	// 82E69568: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E6956C: 396BD398  addi r11, r11, -0x2c68
	ctx.r[11].s64 = ctx.r[11].s64 + -11368;
	// 82E69570: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E69574: 409A000C  bne cr6, 0x82e69580
	if !ctx.cr[6].eq {
	pc = 0x82E69580; continue 'dispatch;
	}
	// 82E69578: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82E6957C: 48000008  b 0x82e69584
	pc = 0x82E69584; continue 'dispatch;
	// 82E69580: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E69584: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E69588: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E6958C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E69590: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E69598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E69598 size=76
    let mut pc: u32 = 0x82E69598;
    'dispatch: loop {
        match pc {
            0x82E69598 => {
    //   block [0x82E69598..0x82E695E4)
	// 82E69598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6959C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E695A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E695A4: 4BFFFDFD  bl 0x82e693a0
	ctx.lr = 0x82E695A8;
	sub_82E693A0(ctx, base);
	// 82E695A8: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E695AC: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 82E695B0: 419A0020  beq cr6, 0x82e695d0
	if ctx.cr[6].eq {
	pc = 0x82E695D0; continue 'dispatch;
	}
	// 82E695B4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E695B8: 480086C1  bl 0x82e71c78
	ctx.lr = 0x82E695BC;
	sub_82E71C78(ctx, base);
	// 82E695BC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E695C0: 40820010  bne 0x82e695d0
	if !ctx.cr[0].eq {
	pc = 0x82E695D0; continue 'dispatch;
	}
	// 82E695C4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82E695C8: C82BD228  lfd f1, -0x2dd8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-11736 as u32) ) };
	// 82E695CC: 48000008  b 0x82e695d4
	pc = 0x82E695D4; continue 'dispatch;
	// 82E695D0: C8230000  lfd f1, 0(r3)
	ctx.f[1].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	// 82E695D4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E695D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E695DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E695E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E695E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E695E8 size=160
    let mut pc: u32 = 0x82E695E8;
    'dispatch: loop {
        match pc {
            0x82E695E8 => {
    //   block [0x82E695E8..0x82E69688)
	// 82E695E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E695EC: 4833EB81  bl 0x831a816c
	ctx.lr = 0x82E695F0;
	sub_831A8130(ctx, base);
	// 82E695F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E695F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E695F8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E695FC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82E69600: 4BFFFDA1  bl 0x82e693a0
	ctx.lr = 0x82E69604;
	sub_82E693A0(ctx, base);
	// 82E69604: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E69608: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 82E6960C: 419A0058  beq cr6, 0x82e69664
	if ctx.cr[6].eq {
	pc = 0x82E69664; continue 'dispatch;
	}
	// 82E69610: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E69614: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E69618: 480086D1  bl 0x82e71ce8
	ctx.lr = 0x82E6961C;
	sub_82E71CE8(ctx, base);
	// 82E6961C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82E69620: 4082001C  bne 0x82e6963c
	if !ctx.cr[0].eq {
	pc = 0x82E6963C; continue 'dispatch;
	}
	// 82E69624: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82E69628: 419A000C  beq cr6, 0x82e69634
	if ctx.cr[6].eq {
	pc = 0x82E69634; continue 'dispatch;
	}
	// 82E6962C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E69630: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E69634: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E69638: 48000048  b 0x82e69680
	pc = 0x82E69680; continue 'dispatch;
	// 82E6963C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E69640: 814B0044  lwz r10, 0x44(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 82E69644: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 82E69648: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E6964C: 4198000C  blt cr6, 0x82e69658
	if ctx.cr[6].lt {
	pc = 0x82E69658; continue 'dispatch;
	}
	// 82E69650: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E69654: 4800787D  bl 0x82e70ed0
	ctx.lr = 0x82E69658;
	sub_82E70ED0(ctx, base);
	// 82E69658: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E6965C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E69660: 4BFFFD41  bl 0x82e693a0
	ctx.lr = 0x82E69664;
	sub_82E693A0(ctx, base);
	// 82E69664: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82E69668: 419A0010  beq cr6, 0x82e69678
	if ctx.cr[6].eq {
	pc = 0x82E69678; continue 'dispatch;
	}
	// 82E6966C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E69670: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E69674: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E69678: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E6967C: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82E69680: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E69684: 4833EB38  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E69688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E69688 size=76
    let mut pc: u32 = 0x82E69688;
    'dispatch: loop {
        match pc {
            0x82E69688 => {
    //   block [0x82E69688..0x82E696D4)
	// 82E69688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6968C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E69690: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E69694: 4BFFFD0D  bl 0x82e693a0
	ctx.lr = 0x82E69698;
	sub_82E693A0(ctx, base);
	// 82E69698: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6969C: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82E696A0: 419A0020  beq cr6, 0x82e696c0
	if ctx.cr[6].eq {
	pc = 0x82E696C0; continue 'dispatch;
	}
	// 82E696A4: 2F0B0007  cmpwi cr6, r11, 7
	ctx.cr[6].compare_i32(ctx.r[11].s32, 7, &mut ctx.xer);
	// 82E696A8: 419A000C  beq cr6, 0x82e696b4
	if ctx.cr[6].eq {
	pc = 0x82E696B4; continue 'dispatch;
	}
	// 82E696AC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E696B0: 48000014  b 0x82e696c4
	pc = 0x82E696C4; continue 'dispatch;
	// 82E696B4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E696B8: 386B0018  addi r3, r11, 0x18
	ctx.r[3].s64 = ctx.r[11].s64 + 24;
	// 82E696BC: 48000008  b 0x82e696c4
	pc = 0x82E696C4; continue 'dispatch;
	// 82E696C0: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E696C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E696C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E696CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E696D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E696D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E696D8 size=100
    let mut pc: u32 = 0x82E696D8;
    'dispatch: loop {
        match pc {
            0x82E696D8 => {
    //   block [0x82E696D8..0x82E6973C)
	// 82E696D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E696DC: 4833EA91  bl 0x831a816c
	ctx.lr = 0x82E696E0;
	sub_831A8130(ctx, base);
	// 82E696E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E696E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E696E8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82E696EC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82E696F0: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E696F4: 814B0044  lwz r10, 0x44(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 82E696F8: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 82E696FC: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E69700: 41980008  blt cr6, 0x82e69708
	if ctx.cr[6].lt {
	pc = 0x82E69708; continue 'dispatch;
	}
	// 82E69704: 480077CD  bl 0x82e70ed0
	ctx.lr = 0x82E69708;
	sub_82E70ED0(ctx, base);
	// 82E69708: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E6970C: 83DF0008  lwz r30, 8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E69710: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E69714: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E69718: 4800B041  bl 0x82e74758
	ctx.lr = 0x82E6971C;
	sub_82E74758(ctx, base);
	// 82E6971C: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 82E69720: 907E0000  stw r3, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82E69724: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E69728: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6972C: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82E69730: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E69734: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E69738: 4833EA84  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E69740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E69740 size=36
    let mut pc: u32 = 0x82E69740;
    'dispatch: loop {
        match pc {
            0x82E69740 => {
    //   block [0x82E69740..0x82E69764)
	// 82E69740: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82E69744: 409A0020  bne cr6, 0x82e69764
	if !ctx.cr[6].eq {
		sub_82E69764(ctx, base);
		return;
	}
	// 82E69748: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6974C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E69750: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E69754: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E69758: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82E6975C: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E69760: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E69764(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E69764 size=40
    let mut pc: u32 = 0x82E69764;
    'dispatch: loop {
        match pc {
            0x82E69764 => {
    //   block [0x82E69764..0x82E6978C)
	// 82E69764: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82E69768: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82E6976C: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E69770: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E69774: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82E69778: 409AFFF4  bne cr6, 0x82e6976c
	if !ctx.cr[6].eq {
	pc = 0x82E6976C; continue 'dispatch;
	}
	// 82E6977C: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82E69780: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82E69784: 5565003E  slwi r5, r11, 0
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E69788: 4BFFFF50  b 0x82e696d8
	sub_82E696D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E69790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E69790 size=212
    let mut pc: u32 = 0x82E69790;
    'dispatch: loop {
        match pc {
            0x82E69790 => {
    //   block [0x82E69790..0x82E69864)
	// 82E69790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E69794: 4833E9D9  bl 0x831a816c
	ctx.lr = 0x82E69798;
	sub_831A8130(ctx, base);
	// 82E69798: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E6979C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E697A0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82E697A4: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82E697A8: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E697AC: 814B0044  lwz r10, 0x44(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 82E697B0: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 82E697B4: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E697B8: 41980008  blt cr6, 0x82e697c0
	if ctx.cr[6].lt {
	pc = 0x82E697C0; continue 'dispatch;
	}
	// 82E697BC: 48007715  bl 0x82e70ed0
	ctx.lr = 0x82E697C0;
	sub_82E70ED0(ctx, base);
	// 82E697C0: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E697C4: 815F0028  lwz r10, 0x28(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82E697C8: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82E697CC: 409A000C  bne cr6, 0x82e697d8
	if !ctx.cr[6].eq {
	pc = 0x82E697D8; continue 'dispatch;
	}
	// 82E697D0: 80BF0048  lwz r5, 0x48(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82E697D4: 48000010  b 0x82e697e4
	pc = 0x82E697E4; continue 'dispatch;
	// 82E697D8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E697DC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E697E0: 80AB000C  lwz r5, 0xc(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E697E4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E697E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E697EC: 4800B13D  bl 0x82e74928
	ctx.lr = 0x82E697F0;
	sub_82E74928(ctx, base);
	// 82E697F0: 93A30010  stw r29, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[29].u32 ) };
	// 82E697F4: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E697F8: 57CB2036  slwi r11, r30, 4
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E697FC: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82E69800: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82E69804: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E69808: 419A0038  beq cr6, 0x82e69840
	if ctx.cr[6].eq {
	pc = 0x82E69840; continue 'dispatch;
	}
	// 82E6980C: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82E69810: 2123FFE8  subfic r9, r3, -0x18
	ctx.xer.ca = ctx.r[3].u32 <= -24 as u32;
	ctx.r[9].s64 = (-24 as i64) - ctx.r[3].s64;
	// 82E69814: 396B0018  addi r11, r11, 0x18
	ctx.r[11].s64 = ctx.r[11].s64 + 24;
	// 82E69818: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6981C: 396BFFF0  addi r11, r11, -0x10
	ctx.r[11].s64 = ctx.r[11].s64 + -16;
	// 82E69820: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82E69824: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82E69828: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82E6982C: E90A0000  ld r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	// 82E69830: F90B0000  std r8, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u64 ) };
	// 82E69834: 814A0008  lwz r10, 8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E69838: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E6983C: 4082FFDC  bne 0x82e69818
	if !ctx.cr[0].eq {
	pc = 0x82E69818; continue 'dispatch;
	}
	// 82E69840: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E69844: 39400006  li r10, 6
	ctx.r[10].s64 = 6;
	// 82E69848: 906B0000  stw r3, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82E6984C: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E69850: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E69854: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82E69858: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E6985C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E69860: 4833E95C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E69868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E69868 size=44
    let mut pc: u32 = 0x82E69868;
    'dispatch: loop {
        match pc {
            0x82E69868 => {
    //   block [0x82E69868..0x82E69894)
	// 82E69868: 7C8A0034  cntlzw r10, r4
	ctx.r[10].u64 = if ctx.r[4].u32 == 0 { 32 } else { ctx.r[4].u32.leading_zeros() as u64 };
	// 82E6986C: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E69870: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82E69874: 554ADFFE  rlwinm r10, r10, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 82E69878: 694A0001  xori r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u64 ^ 1;
	// 82E6987C: 912B0008  stw r9, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82E69880: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E69884: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E69888: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82E6988C: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E69890: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E69898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E69898 size=32
    let mut pc: u32 = 0x82E69898;
    'dispatch: loop {
        match pc {
            0x82E69898 => {
    //   block [0x82E69898..0x82E698B8)
	// 82E69898: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6989C: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82E698A0: 908B0000  stw r4, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82E698A4: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E698A8: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E698AC: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82E698B0: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E698B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E698B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E698B8 size=148
    let mut pc: u32 = 0x82E698B8;
    'dispatch: loop {
        match pc {
            0x82E698B8 => {
    //   block [0x82E698B8..0x82E6994C)
	// 82E698B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E698BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E698C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E698C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E698C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E698CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E698D0: 4BFFFAD1  bl 0x82e693a0
	ctx.lr = 0x82E698D4;
	sub_82E693A0(ctx, base);
	// 82E698D4: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82E698D8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E698DC: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82E698E0: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E698E4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E698E8: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82E698EC: 409AFFF4  bne cr6, 0x82e698e0
	if !ctx.cr[6].eq {
	pc = 0x82E698E0; continue 'dispatch;
	}
	// 82E698F0: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82E698F4: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82E698F8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82E698FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E69900: 5565003E  slwi r5, r11, 0
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E69904: 4800AE55  bl 0x82e74758
	ctx.lr = 0x82E69908;
	sub_82E74758(ctx, base);
	// 82E69908: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 82E6990C: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82E69910: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E69914: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E69918: 80DF0008  lwz r6, 8(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6991C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E69920: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82E69924: 4800853D  bl 0x82e71e60
	ctx.lr = 0x82E69928;
	sub_82E71E60(ctx, base);
	// 82E69928: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6992C: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82E69930: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E69934: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E69938: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E6993C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E69940: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E69944: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E69948: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E69950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E69950 size=80
    let mut pc: u32 = 0x82E69950;
    'dispatch: loop {
        match pc {
            0x82E69950 => {
    //   block [0x82E69950..0x82E699A0)
	// 82E69950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E69954: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E69958: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E6995C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E69960: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E69964: 4BFFFA3D  bl 0x82e693a0
	ctx.lr = 0x82E69968;
	sub_82E693A0(ctx, base);
	// 82E69968: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6996C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E69970: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E69974: 38CBFFF0  addi r6, r11, -0x10
	ctx.r[6].s64 = ctx.r[11].s64 + -16;
	// 82E69978: 38ABFFE0  addi r5, r11, -0x20
	ctx.r[5].s64 = ctx.r[11].s64 + -32;
	// 82E6997C: 480085FD  bl 0x82e71f78
	ctx.lr = 0x82E69980;
	sub_82E71F78(ctx, base);
	// 82E69980: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E69984: 396BFFE0  addi r11, r11, -0x20
	ctx.r[11].s64 = ctx.r[11].s64 + -32;
	// 82E69988: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E6998C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E69990: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E69994: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E69998: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E6999C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E699A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E699A0 size=104
    let mut pc: u32 = 0x82E699A0;
    'dispatch: loop {
        match pc {
            0x82E699A0 => {
    //   block [0x82E699A0..0x82E69A08)
	// 82E699A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E699A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E699A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E699AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E699B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E699B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E699B8: 39640001  addi r11, r4, 1
	ctx.r[11].s64 = ctx.r[4].s64 + 1;
	// 82E699BC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82E699C0: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E699C4: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E699C8: 7C8B5050  subf r4, r11, r10
	ctx.r[4].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82E699CC: 48000F0D  bl 0x82e6a8d8
	ctx.lr = 0x82E699D0;
	sub_82E6A8D8(ctx, base);
	// 82E699D0: 2F1EFFFF  cmpwi cr6, r30, -1
	ctx.cr[6].compare_i32(ctx.r[30].s32, -1, &mut ctx.xer);
	// 82E699D4: 409A001C  bne cr6, 0x82e699f0
	if !ctx.cr[6].eq {
	pc = 0x82E699F0; continue 'dispatch;
	}
	// 82E699D8: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E699DC: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E699E0: 812B0008  lwz r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E699E4: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82E699E8: 41980008  blt cr6, 0x82e699f0
	if ctx.cr[6].lt {
	pc = 0x82E699F0; continue 'dispatch;
	}
	// 82E699EC: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E699F0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E699F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E699F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E699FC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E69A00: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E69A04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E69A08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E69A08 size=12
    let mut pc: u32 = 0x82E69A08;
    'dispatch: loop {
        match pc {
            0x82E69A08 => {
    //   block [0x82E69A08..0x82E69A14)
	// 82E69A08: 80A40004  lwz r5, 4(r4)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E69A0C: 80840000  lwz r4, 0(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E69A10: 48000EC8  b 0x82e6a8d8
	sub_82E6A8D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E69A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E69A18 size=152
    let mut pc: u32 = 0x82E69A18;
    'dispatch: loop {
        match pc {
            0x82E69A18 => {
    //   block [0x82E69A18..0x82E69AB0)
	// 82E69A18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E69A1C: 4833E751  bl 0x831a816c
	ctx.lr = 0x82E69A20;
	sub_831A8130(ctx, base);
	// 82E69A20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E69A24: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E69A28: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82E69A2C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82E69A30: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 82E69A34: 409A000C  bne cr6, 0x82e69a40
	if !ctx.cr[6].eq {
	pc = 0x82E69A40; continue 'dispatch;
	}
	// 82E69A38: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82E69A3C: 48000018  b 0x82e69a54
	pc = 0x82E69A54; continue 'dispatch;
	// 82E69A40: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 82E69A44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E69A48: 4BFFF959  bl 0x82e693a0
	ctx.lr = 0x82E69A4C;
	sub_82E693A0(ctx, base);
	// 82E69A4C: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E69A50: 7CEB1850  subf r7, r11, r3
	ctx.r[7].s64 = ctx.r[3].s64 - ctx.r[11].s64;
	// 82E69A54: 397D0001  addi r11, r29, 1
	ctx.r[11].s64 = ctx.r[29].s64 + 1;
	// 82E69A58: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E69A5C: 811F0020  lwz r8, 0x20(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E69A60: 3D4082E7  lis r10, -0x7d19
	ctx.r[10].s64 = -2098790400;
	// 82E69A64: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E69A68: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 82E69A6C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E69A70: 7D6B4850  subf r11, r11, r9
	ctx.r[11].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 82E69A74: 388A9A08  addi r4, r10, -0x65f8
	ctx.r[4].s64 = ctx.r[10].s64 + -26104;
	// 82E69A78: 7CC85850  subf r6, r8, r11
	ctx.r[6].s64 = ctx.r[11].s64 - ctx.r[8].s64;
	// 82E69A7C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E69A80: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82E69A84: 48001085  bl 0x82e6ab08
	ctx.lr = 0x82E69A88;
	sub_82E6AB08(ctx, base);
	// 82E69A88: 2F1EFFFF  cmpwi cr6, r30, -1
	ctx.cr[6].compare_i32(ctx.r[30].s32, -1, &mut ctx.xer);
	// 82E69A8C: 409A001C  bne cr6, 0x82e69aa8
	if !ctx.cr[6].eq {
	pc = 0x82E69AA8; continue 'dispatch;
	}
	// 82E69A90: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E69A94: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E69A98: 812B0008  lwz r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E69A9C: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82E69AA0: 41980008  blt cr6, 0x82e69aa8
	if ctx.cr[6].lt {
	pc = 0x82E69AA8; continue 'dispatch;
	}
	// 82E69AA4: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E69AA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E69AAC: 4833E710  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E69AB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E69AB0 size=104
    let mut pc: u32 = 0x82E69AB0;
    'dispatch: loop {
        match pc {
            0x82E69AB0 => {
    //   block [0x82E69AB0..0x82E69B18)
	// 82E69AB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E69AB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E69AB8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E69ABC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E69AC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E69AC4: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82E69AC8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E69ACC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E69AD0: 409A000C  bne cr6, 0x82e69adc
	if !ctx.cr[6].eq {
	pc = 0x82E69ADC; continue 'dispatch;
	}
	// 82E69AD4: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 82E69AD8: 3BEB7EA8  addi r31, r11, 0x7ea8
	ctx.r[31].s64 = ctx.r[11].s64 + 32424;
	// 82E69ADC: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 82E69AE0: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82E69AE4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E69AE8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E69AEC: 4800B435  bl 0x82e74f20
	ctx.lr = 0x82E69AF0;
	sub_82E74F20(ctx, base);
	// 82E69AF0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82E69AF4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E69AF8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E69AFC: 480010D5  bl 0x82e6abd0
	ctx.lr = 0x82E69B00;
	sub_82E6ABD0(ctx, base);
	// 82E69B00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E69B04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E69B08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E69B0C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E69B10: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E69B14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E69B18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E69B18 size=160
    let mut pc: u32 = 0x82E69B18;
    'dispatch: loop {
        match pc {
            0x82E69B18 => {
    //   block [0x82E69B18..0x82E69BB8)
	// 82E69B18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E69B1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E69B20: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E69B24: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E69B28: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E69B2C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E69B30: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82E69B34: 2F040002  cmpwi cr6, r4, 2
	ctx.cr[6].compare_i32(ctx.r[4].s32, 2, &mut ctx.xer);
	// 82E69B38: 41980060  blt cr6, 0x82e69b98
	if ctx.cr[6].lt {
	pc = 0x82E69B98; continue 'dispatch;
	}
	// 82E69B3C: 2F040003  cmpwi cr6, r4, 3
	ctx.cr[6].compare_i32(ctx.r[4].s32, 3, &mut ctx.xer);
	// 82E69B40: 40990044  ble cr6, 0x82e69b84
	if !ctx.cr[6].gt {
	pc = 0x82E69B84; continue 'dispatch;
	}
	// 82E69B44: 2F040004  cmpwi cr6, r4, 4
	ctx.cr[6].compare_i32(ctx.r[4].s32, 4, &mut ctx.xer);
	// 82E69B48: 419A002C  beq cr6, 0x82e69b74
	if ctx.cr[6].eq {
	pc = 0x82E69B74; continue 'dispatch;
	}
	// 82E69B4C: 2F040005  cmpwi cr6, r4, 5
	ctx.cr[6].compare_i32(ctx.r[4].s32, 5, &mut ctx.xer);
	// 82E69B50: 409A0048  bne cr6, 0x82e69b98
	if !ctx.cr[6].eq {
	pc = 0x82E69B98; continue 'dispatch;
	}
	// 82E69B54: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E69B58: 38A00017  li r5, 0x17
	ctx.r[5].s64 = 23;
	// 82E69B5C: 388BD1EC  addi r4, r11, -0x2e14
	ctx.r[4].s64 = ctx.r[11].s64 + -11796;
	// 82E69B60: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E69B64: 4800ABF5  bl 0x82e74758
	ctx.lr = 0x82E69B68;
	sub_82E74758(ctx, base);
	// 82E69B68: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 82E69B6C: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82E69B70: 48000024  b 0x82e69b94
	pc = 0x82E69B94; continue 'dispatch;
	// 82E69B74: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E69B78: 38A00011  li r5, 0x11
	ctx.r[5].s64 = 17;
	// 82E69B7C: 388BD1D8  addi r4, r11, -0x2e28
	ctx.r[4].s64 = ctx.r[11].s64 + -11816;
	// 82E69B80: 4BFFFFE0  b 0x82e69b60
	pc = 0x82E69B60; continue 'dispatch;
	// 82E69B84: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E69B88: E94BFFF0  ld r10, -0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-16 as u32) ) };
	// 82E69B8C: F95F0000  std r10, 0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u64 ) };
	// 82E69B90: 816BFFF8  lwz r11, -8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E69B94: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E69B98: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
	// 82E69B9C: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E69BA0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E69BA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E69BA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E69BAC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E69BB0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E69BB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E69BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E69BB8 size=128
    let mut pc: u32 = 0x82E69BB8;
    'dispatch: loop {
        match pc {
            0x82E69BB8 => {
    //   block [0x82E69BB8..0x82E69C38)
	// 82E69BB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E69BBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E69BC0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E69BC4: 9421FA40  stwu r1, -0x5c0(r1)
	ea = ctx.r[1].u32.wrapping_add(-1472 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E69BC8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E69BCC: 908105DC  stw r4, 0x5dc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1500 as u32), ctx.r[4].u32 ) };
	// 82E69BD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E69BD4: 90A105E4  stw r5, 0x5e4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1508 as u32), ctx.r[5].u32 ) };
	// 82E69BD8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E69BDC: 812B0070  lwz r9, 0x70(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(112 as u32) ) } as u64;
	// 82E69BE0: 916105D4  stw r11, 0x5d4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1492 as u32), ctx.r[11].u32 ) };
	// 82E69BE4: 914105A0  stw r10, 0x5a0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1440 as u32), ctx.r[10].u32 ) };
	// 82E69BE8: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82E69BEC: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 82E69BF0: 912B0070  stw r9, 0x70(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(112 as u32), ctx.r[9].u32 ) };
	// 82E69BF4: 4834440D  bl 0x831ae000
	ctx.lr = 0x82E69BF8;
	sub_831AE000(ctx, base);
	// 82E69BF8: 83E105D4  lwz r31, 0x5d4(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(1492 as u32) ) } as u64;
	// 82E69BFC: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82E69C00: 40820018  bne 0x82e69c18
	if !ctx.cr[0].eq {
	pc = 0x82E69C18; continue 'dispatch;
	}
	// 82E69C04: 816105DC  lwz r11, 0x5dc(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(1500 as u32) ) } as u64;
	// 82E69C08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E69C0C: 808105E4  lwz r4, 0x5e4(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(1508 as u32) ) } as u64;
	// 82E69C10: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E69C14: 4E800421  bctrl
	ctx.lr = 0x82E69C18;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E69C18: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E69C1C: 806105A0  lwz r3, 0x5a0(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(1440 as u32) ) } as u64;
	// 82E69C20: 917F0070  stw r11, 0x70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82E69C24: 382105C0  addi r1, r1, 0x5c0
	ctx.r[1].s64 = ctx.r[1].s64 + 1472;
	// 82E69C28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E69C2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E69C30: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E69C34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E69C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E69C38 size=312
    let mut pc: u32 = 0x82E69C38;
    'dispatch: loop {
        match pc {
            0x82E69C38 => {
    //   block [0x82E69C38..0x82E69D70)
	// 82E69C38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E69C3C: 4833E52D  bl 0x831a8168
	ctx.lr = 0x82E69C40;
	sub_831A8130(ctx, base);
	// 82E69C40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E69C44: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82E69C48: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E69C4C: 3BBC0006  addi r29, r28, 6
	ctx.r[29].s64 = ctx.r[28].s64 + 6;
	// 82E69C50: 3D600FFF  lis r11, 0xfff
	ctx.r[11].s64 = 268369920;
	// 82E69C54: 395D0001  addi r10, r29, 1
	ctx.r[10].s64 = ctx.r[29].s64 + 1;
	// 82E69C58: 616BFFFF  ori r11, r11, 0xffff
	ctx.r[11].u64 = ctx.r[11].u64 | 65535;
	// 82E69C5C: 83DF0020  lwz r30, 0x20(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E69C60: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E69C64: 4199001C  bgt cr6, 0x82e69c80
	if ctx.cr[6].gt {
	pc = 0x82E69C80; continue 'dispatch;
	}
	// 82E69C68: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82E69C6C: 57A62036  slwi r6, r29, 4
	ctx.r[6].u32 = ctx.r[29].u32.wrapping_shl(4);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82E69C70: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E69C74: 55652036  slwi r5, r11, 4
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E69C78: 4800B401  bl 0x82e75078
	ctx.lr = 0x82E69C7C;
	sub_82E75078(ctx, base);
	// 82E69C7C: 48000008  b 0x82e69c84
	pc = 0x82E69C84; continue 'dispatch;
	// 82E69C80: 4800B3C9  bl 0x82e75048
	ctx.lr = 0x82E69C84;
	sub_82E75048(ctx, base);
	// 82E69C84: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E69C88: 578A2036  slwi r10, r28, 4
	ctx.r[10].u32 = ctx.r[28].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E69C8C: 817F0068  lwz r11, 0x68(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E69C90: 7D3E4850  subf r9, r30, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[30].s64;
	// 82E69C94: 907F0020  stw r3, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[3].u32 ) };
	// 82E69C98: 7D0A1A14  add r8, r10, r3
	ctx.r[8].u64 = ctx.r[10].u64 + ctx.r[3].u64;
	// 82E69C9C: 93BF002C  stw r29, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[29].u32 ) };
	// 82E69CA0: 7D2A2670  srawi r10, r9, 4
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[9].s32 >> 4) as i64;
	// 82E69CA4: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E69CA8: 7D4A1A14  add r10, r10, r3
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[3].u64;
	// 82E69CAC: 911F001C  stw r8, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[8].u32 ) };
	// 82E69CB0: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E69CB4: 48000024  b 0x82e69cd8
	pc = 0x82E69CD8; continue 'dispatch;
	// 82E69CB8: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E69CBC: 813F0020  lwz r9, 0x20(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E69CC0: 7D5E5050  subf r10, r30, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[30].s64;
	// 82E69CC4: 7D4A2670  srawi r10, r10, 4
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 4) as i64;
	// 82E69CC8: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E69CCC: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82E69CD0: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E69CD4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E69CD8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E69CDC: 4082FFDC  bne 0x82e69cb8
	if !ctx.cr[0].eq {
	pc = 0x82E69CB8; continue 'dispatch;
	}
	// 82E69CE0: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82E69CE4: 4800005C  b 0x82e69d40
	pc = 0x82E69D40; continue 'dispatch;
	// 82E69CE8: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E69CEC: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E69CF0: 7D5E5050  subf r10, r30, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[30].s64;
	// 82E69CF4: 813F0020  lwz r9, 0x20(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E69CF8: 7D1E4050  subf r8, r30, r8
	ctx.r[8].s64 = ctx.r[8].s64 - ctx.r[30].s64;
	// 82E69CFC: 80EB0004  lwz r7, 4(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E69D00: 7D4A2670  srawi r10, r10, 4
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 4) as i64;
	// 82E69D04: 7D082670  srawi r8, r8, 4
	ctx.xer.ca = (ctx.r[8].s32 < 0) && ((ctx.r[8].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[8].s32 >> 4) as i64;
	// 82E69D08: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E69D0C: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82E69D10: 55092036  slwi r9, r8, 4
	ctx.r[9].u32 = ctx.r[8].u32.wrapping_shl(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82E69D14: 7D1E3850  subf r8, r30, r7
	ctx.r[8].s64 = ctx.r[7].s64 - ctx.r[30].s64;
	// 82E69D18: 7D072670  srawi r7, r8, 4
	ctx.xer.ca = (ctx.r[8].s32 < 0) && ((ctx.r[8].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[7].s64 = (ctx.r[8].s32 >> 4) as i64;
	// 82E69D1C: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E69D20: 811F0020  lwz r8, 0x20(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E69D24: 54EA2036  slwi r10, r7, 4
	ctx.r[10].u32 = ctx.r[7].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E69D28: 7D294214  add r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 82E69D2C: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82E69D30: 813F0020  lwz r9, 0x20(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E69D34: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82E69D38: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E69D3C: 396B0018  addi r11, r11, 0x18
	ctx.r[11].s64 = ctx.r[11].s64 + 24;
	// 82E69D40: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E69D44: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82E69D48: 4099FFA0  ble cr6, 0x82e69ce8
	if !ctx.cr[6].gt {
	pc = 0x82E69CE8; continue 'dispatch;
	}
	// 82E69D4C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E69D50: 815F0020  lwz r10, 0x20(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E69D54: 7D7E5850  subf r11, r30, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	// 82E69D58: 7D6B2670  srawi r11, r11, 4
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 4) as i64;
	// 82E69D5C: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E69D60: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82E69D64: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82E69D68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E69D6C: 4833E44C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E69D70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E69D70 size=132
    let mut pc: u32 = 0x82E69D70;
    'dispatch: loop {
        match pc {
            0x82E69D70 => {
    //   block [0x82E69D70..0x82E69DF4)
	// 82E69D70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E69D74: 4833E3F9  bl 0x831a816c
	ctx.lr = 0x82E69D78;
	sub_831A8130(ctx, base);
	// 82E69D78: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E69D7C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E69D80: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E69D84: 3D400AAA  lis r10, 0xaaa
	ctx.r[10].s64 = 178913280;
	// 82E69D88: 397E0001  addi r11, r30, 1
	ctx.r[11].s64 = ctx.r[30].s64 + 1;
	// 82E69D8C: 614AAAAA  ori r10, r10, 0xaaaa
	ctx.r[10].u64 = ctx.r[10].u64 | 43690;
	// 82E69D90: 83BF0028  lwz r29, 0x28(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82E69D94: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82E69D98: 4199001C  bgt cr6, 0x82e69db4
	if ctx.cr[6].gt {
	pc = 0x82E69DB4; continue 'dispatch;
	}
	// 82E69D9C: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E69DA0: 1CDE0018  mulli r6, r30, 0x18
	ctx.r[6].s64 = ctx.r[30].s64 * 24;
	// 82E69DA4: 1CAB0018  mulli r5, r11, 0x18
	ctx.r[5].s64 = ctx.r[11].s64 * 24;
	// 82E69DA8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E69DAC: 4800B2CD  bl 0x82e75078
	ctx.lr = 0x82E69DB0;
	sub_82E75078(ctx, base);
	// 82E69DB0: 48000008  b 0x82e69db8
	pc = 0x82E69DB8; continue 'dispatch;
	// 82E69DB4: 4800B295  bl 0x82e75048
	ctx.lr = 0x82E69DB8;
	sub_82E75048(ctx, base);
	// 82E69DB8: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E69DBC: 39200018  li r9, 0x18
	ctx.r[9].s64 = 24;
	// 82E69DC0: 1D7E0018  mulli r11, r30, 0x18
	ctx.r[11].s64 = ctx.r[30].s64 * 24;
	// 82E69DC4: 907F0028  stw r3, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[3].u32 ) };
	// 82E69DC8: 93DF0030  stw r30, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[30].u32 ) };
	// 82E69DCC: 7D5D5050  subf r10, r29, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[29].s64;
	// 82E69DD0: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82E69DD4: 7D4A4BD6  divw r10, r10, r9
	ctx.r[10].s32 = ctx.r[10].s32 / ctx.r[9].s32;
	// 82E69DD8: 396BFFE8  addi r11, r11, -0x18
	ctx.r[11].s64 = ctx.r[11].s64 + -24;
	// 82E69DDC: 1D4A0018  mulli r10, r10, 0x18
	ctx.r[10].s64 = ctx.r[10].s64 * 24;
	// 82E69DE0: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82E69DE4: 7D4A1A14  add r10, r10, r3
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[3].u64;
	// 82E69DE8: 915F0014  stw r10, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82E69DEC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E69DF0: 4833E3CC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E69DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E69DF8 size=20
    let mut pc: u32 = 0x82E69DF8;
    'dispatch: loop {
        match pc {
            0x82E69DF8 => {
    //   block [0x82E69DF8..0x82E69E0C)
	// 82E69DF8: 8163002C  lwz r11, 0x2c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82E69DFC: 7F045800  cmpw cr6, r4, r11
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E69E00: 4199000C  bgt cr6, 0x82e69e0c
	if ctx.cr[6].gt {
		sub_82E69E0C(ctx, base);
		return;
	}
	// 82E69E04: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82E69E08: 4BFFFE30  b 0x82e69c38
	sub_82E69C38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E69E0C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E69E0C size=8
    let mut pc: u32 = 0x82E69E0C;
    'dispatch: loop {
        match pc {
            0x82E69E0C => {
    //   block [0x82E69E0C..0x82E69E14)
	// 82E69E0C: 7C8B2214  add r4, r11, r4
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82E69E10: 4BFFFE28  b 0x82e69c38
	sub_82E69C38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E69E18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E69E18 size=236
    let mut pc: u32 = 0x82E69E18;
    'dispatch: loop {
        match pc {
            0x82E69E18 => {
    //   block [0x82E69E18..0x82E69F04)
	// 82E69E18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E69E1C: 4833E349  bl 0x831a8164
	ctx.lr = 0x82E69E20;
	sub_831A8130(ctx, base);
	// 82E69E20: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E69E24: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E69E28: 837F0040  lwz r27, 0x40(r31)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82E69E2C: 281B0000  cmplwi r27, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E69E30: 418200CC  beq 0x82e69efc
	if ctx.cr[0].eq {
	pc = 0x82E69EFC; continue 'dispatch;
	}
	// 82E69E34: 897F0037  lbz r11, 0x37(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(55 as u32) ) } as u64;
	// 82E69E38: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E69E3C: 418200C0  beq 0x82e69efc
	if ctx.cr[0].eq {
	pc = 0x82E69EFC; continue 'dispatch;
	}
	// 82E69E40: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E69E44: 2F040004  cmpwi cr6, r4, 4
	ctx.cr[6].compare_i32(ctx.r[4].s32, 4, &mut ctx.xer);
	// 82E69E48: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E69E4C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82E69E50: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E69E54: 7FCB4850  subf r30, r11, r9
	ctx.r[30].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 82E69E58: 810A0008  lwz r8, 8(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E69E5C: 90810050  stw r4, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[4].u32 ) };
	// 82E69E60: 90A10064  stw r5, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[5].u32 ) };
	// 82E69E64: 7FAB4050  subf r29, r11, r8
	ctx.r[29].s64 = ctx.r[8].s64 - ctx.r[11].s64;
	// 82E69E68: 409A000C  bne cr6, 0x82e69e74
	if !ctx.cr[6].eq {
	pc = 0x82E69E74; continue 'dispatch;
	}
	// 82E69E6C: 938100B0  stw r28, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[28].u32 ) };
	// 82E69E70: 48000018  b 0x82e69e88
	pc = 0x82E69E88; continue 'dispatch;
	// 82E69E74: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82E69E78: 39000018  li r8, 0x18
	ctx.r[8].s64 = 24;
	// 82E69E7C: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82E69E80: 7D6B43D6  divw r11, r11, r8
	ctx.r[11].s32 = ctx.r[11].s32 / ctx.r[8].s32;
	// 82E69E84: 916100B0  stw r11, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 82E69E88: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E69E8C: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 82E69E90: 2F0B0140  cmpwi cr6, r11, 0x140
	ctx.cr[6].compare_i32(ctx.r[11].s32, 320, &mut ctx.xer);
	// 82E69E94: 41990020  bgt cr6, 0x82e69eb4
	if ctx.cr[6].gt {
	pc = 0x82E69EB4; continue 'dispatch;
	}
	// 82E69E98: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82E69E9C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E69EA0: 2F0B0014  cmpwi cr6, r11, 0x14
	ctx.cr[6].compare_i32(ctx.r[11].s32, 20, &mut ctx.xer);
	// 82E69EA4: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82E69EA8: 40980008  bge cr6, 0x82e69eb0
	if !ctx.cr[6].lt {
	pc = 0x82E69EB0; continue 'dispatch;
	}
	// 82E69EAC: 388B0014  addi r4, r11, 0x14
	ctx.r[4].s64 = ctx.r[11].s64 + 20;
	// 82E69EB0: 4BFFFD89  bl 0x82e69c38
	ctx.lr = 0x82E69EB4;
	sub_82E69C38(ctx, base);
	// 82E69EB4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E69EB8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E69EBC: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E69EC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E69EC4: 396B0140  addi r11, r11, 0x140
	ctx.r[11].s64 = ctx.r[11].s64 + 320;
	// 82E69EC8: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E69ECC: 9B9F0037  stb r28, 0x37(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(55 as u32), ctx.r[28].u8 ) };
	// 82E69ED0: 7F6903A6  mtctr r27
	ctx.ctr.u64 = ctx.r[27].u64;
	// 82E69ED4: 4E800421  bctrl
	ctx.lr = 0x82E69ED8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E69ED8: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E69EDC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82E69EE0: 813F0014  lwz r9, 0x14(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E69EE4: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82E69EE8: 995F0037  stb r10, 0x37(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(55 as u32), ctx.r[10].u8 ) };
	// 82E69EEC: 91690008  stw r11, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E69EF0: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E69EF4: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82E69EF8: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E69EFC: 382100F0  addi r1, r1, 0xf0
	ctx.r[1].s64 = ctx.r[1].s64 + 240;
	// 82E69F00: 4833E2B4  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E69F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E69F08 size=400
    let mut pc: u32 = 0x82E69F08;
    'dispatch: loop {
        match pc {
            0x82E69F08 => {
    //   block [0x82E69F08..0x82E6A098)
	// 82E69F08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E69F0C: 4833E249  bl 0x831a8154
	ctx.lr = 0x82E69F10;
	sub_831A8130(ctx, base);
	// 82E69F10: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E69F14: 8B040049  lbz r24, 0x49(r4)
	ctx.r[24].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(73 as u32) ) } as u64;
	// 82E69F18: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 82E69F1C: 3AE00000  li r23, 0
	ctx.r[23].s64 = 0;
	// 82E69F20: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E69F24: 7EFABB78  mr r26, r23
	ctx.r[26].u64 = ctx.r[23].u64;
	// 82E69F28: 7F19C000  cmpw cr6, r25, r24
	ctx.cr[6].compare_i32(ctx.r[25].s32, ctx.r[24].s32, &mut ctx.xer);
	// 82E69F2C: 40980028  bge cr6, 0x82e69f54
	if !ctx.cr[6].lt {
	pc = 0x82E69F54; continue 'dispatch;
	}
	// 82E69F30: 7D79C050  subf r11, r25, r24
	ctx.r[11].s64 = ctx.r[24].s64 - ctx.r[25].s64;
	// 82E69F34: 7F2BCA14  add r25, r11, r25
	ctx.r[25].u64 = ctx.r[11].u64 + ctx.r[25].u64;
	// 82E69F38: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E69F3C: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E69F40: 92EA0008  stw r23, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[23].u32 ) };
	// 82E69F44: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E69F48: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 82E69F4C: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E69F50: 4082FFE8  bne 0x82e69f38
	if !ctx.cr[0].eq {
	pc = 0x82E69F38; continue 'dispatch;
	}
	// 82E69F54: 8964004A  lbz r11, 0x4a(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(74 as u32) ) } as u64;
	// 82E69F58: 556B077B  rlwinm. r11, r11, 0, 0x1d, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E69F5C: 418200C8  beq 0x82e6a024
	if ctx.cr[0].eq {
	pc = 0x82E6A024; continue 'dispatch;
	}
	// 82E69F60: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E69F64: 7FD8C850  subf r30, r24, r25
	ctx.r[30].s64 = ctx.r[25].s64 - ctx.r[24].s64;
	// 82E69F68: 814B0044  lwz r10, 0x44(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 82E69F6C: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 82E69F70: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E69F74: 4198000C  blt cr6, 0x82e69f80
	if ctx.cr[6].lt {
	pc = 0x82E69F80; continue 'dispatch;
	}
	// 82E69F78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E69F7C: 48006F55  bl 0x82e70ed0
	ctx.lr = 0x82E69F80;
	sub_82E70ED0(ctx, base);
	// 82E69F80: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82E69F84: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E69F88: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E69F8C: 48009C45  bl 0x82e73bd0
	ctx.lr = 0x82E69F90;
	sub_82E73BD0(ctx, base);
	// 82E69F90: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82E69F94: 7EEBBB78  mr r11, r23
	ctx.r[11].u64 = ctx.r[23].u64;
	// 82E69F98: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82E69F9C: 40990048  ble cr6, 0x82e69fe4
	if !ctx.cr[6].gt {
	pc = 0x82E69FE4; continue 'dispatch;
	}
	// 82E69FA0: 7D5E00D0  neg r10, r30
	ctx.r[10].s64 = -ctx.r[30].s64;
	// 82E69FA4: 555D2036  slwi r29, r10, 4
	ctx.r[29].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 82E69FA8: 3B8B0001  addi r28, r11, 1
	ctx.r[28].s64 = ctx.r[11].s64 + 1;
	// 82E69FAC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E69FB0: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82E69FB4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E69FB8: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82E69FBC: 7F7D5A14  add r27, r29, r11
	ctx.r[27].u64 = ctx.r[29].u64 + ctx.r[11].u64;
	// 82E69FC0: 48009FA9  bl 0x82e73f68
	ctx.lr = 0x82E69FC4;
	sub_82E73F68(ctx, base);
	// 82E69FC4: E95B0000  ld r10, 0(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) };
	// 82E69FC8: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 82E69FCC: 3BBD0010  addi r29, r29, 0x10
	ctx.r[29].s64 = ctx.r[29].s64 + 16;
	// 82E69FD0: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82E69FD4: F9430000  std r10, 0(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u64 ) };
	// 82E69FD8: 815B0008  lwz r10, 8(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E69FDC: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E69FE0: 4198FFC8  blt cr6, 0x82e69fa8
	if ctx.cr[6].lt {
	pc = 0x82E69FA8; continue 'dispatch;
	}
	// 82E69FE4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 82E69FE8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82E69FEC: 388B8320  addi r4, r11, -0x7ce0
	ctx.r[4].s64 = ctx.r[11].s64 + -31968;
	// 82E69FF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E69FF4: 4800A765  bl 0x82e74758
	ctx.lr = 0x82E69FF8;
	sub_82E74758(ctx, base);
	// 82E69FF8: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82E69FFC: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82E6A000: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6A004: 4800A5DD  bl 0x82e745e0
	ctx.lr = 0x82E6A008;
	sub_82E745E0(ctx, base);
	// 82E6A008: 7FCB07B4  extsw r11, r30
	ctx.r[11].s64 = ctx.r[30].s32 as i64;
	// 82E6A00C: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 82E6A010: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 82E6A014: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E6A018: C8010050  lfd f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82E6A01C: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 82E6A020: D8030000  stfd f0, 0(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.f[0].u64 ) };
	// 82E6A024: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6A028: 572A2036  slwi r10, r25, 4
	ctx.r[10].u32 = ctx.r[25].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E6A02C: 2F180000  cmpwi cr6, r24, 0
	ctx.cr[6].compare_i32(ctx.r[24].s32, 0, &mut ctx.xer);
	// 82E6A030: 7D4A5850  subf r10, r10, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82E6A034: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82E6A038: 40990038  ble cr6, 0x82e6a070
	if !ctx.cr[6].gt {
	pc = 0x82E6A070; continue 'dispatch;
	}
	// 82E6A03C: 396A0008  addi r11, r10, 8
	ctx.r[11].s64 = ctx.r[10].s64 + 8;
	// 82E6A040: 7F09C378  mr r9, r24
	ctx.r[9].u64 = ctx.r[24].u64;
	// 82E6A044: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6A048: 3529FFFF  addic. r9, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E6A04C: 390A0010  addi r8, r10, 0x10
	ctx.r[8].s64 = ctx.r[10].s64 + 16;
	// 82E6A050: 911F0008  stw r8, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 82E6A054: E90BFFF8  ld r8, -8(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-8 as u32) ) };
	// 82E6A058: F90A0000  std r8, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u64 ) };
	// 82E6A05C: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E6A060: 910A0008  stw r8, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 82E6A064: 92EB0000  stw r23, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[23].u32 ) };
	// 82E6A068: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82E6A06C: 4082FFD8  bne 0x82e6a044
	if !ctx.cr[0].eq {
	pc = 0x82E6A044; continue 'dispatch;
	}
	// 82E6A070: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 82E6A074: 419A001C  beq cr6, 0x82e6a090
	if ctx.cr[6].eq {
	pc = 0x82E6A090; continue 'dispatch;
	}
	// 82E6A078: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6A07C: 39400005  li r10, 5
	ctx.r[10].s64 = 5;
	// 82E6A080: 392B0010  addi r9, r11, 0x10
	ctx.r[9].s64 = ctx.r[11].s64 + 16;
	// 82E6A084: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82E6A088: 934B0000  stw r26, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[26].u32 ) };
	// 82E6A08C: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E6A090: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82E6A094: 4833E110  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6A098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E6A098 size=204
    let mut pc: u32 = 0x82E6A098;
    'dispatch: loop {
        match pc {
            0x82E6A098 => {
    //   block [0x82E6A098..0x82E6A164)
	// 82E6A098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6A09C: 4833E0CD  bl 0x831a8168
	ctx.lr = 0x82E6A0A0;
	sub_831A8130(ctx, base);
	// 82E6A0A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E6A0A4: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 82E6A0A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E6A0AC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E6A0B0: 48007B69  bl 0x82e71c18
	ctx.lr = 0x82E6A0B4;
	sub_82E71C18(ctx, base);
	// 82E6A0B4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E6A0B8: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E6A0BC: 7F8BF050  subf r28, r11, r30
	ctx.r[28].s64 = ctx.r[30].s64 - ctx.r[11].s64;
	// 82E6A0C0: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6A0C4: 2F0B0006  cmpwi cr6, r11, 6
	ctx.cr[6].compare_i32(ctx.r[11].s32, 6, &mut ctx.xer);
	// 82E6A0C8: 419A0018  beq cr6, 0x82e6a0e0
	if ctx.cr[6].eq {
	pc = 0x82E6A0E0; continue 'dispatch;
	}
	// 82E6A0CC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 82E6A0D0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E6A0D4: 38ABD180  addi r5, r11, -0x2e80
	ctx.r[5].s64 = ctx.r[11].s64 + -11904;
	// 82E6A0D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6A0DC: 480078C5  bl 0x82e719a0
	ctx.lr = 0x82E6A0E0;
	sub_82E719A0(ctx, base);
	// 82E6A0E0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6A0E4: 4800001C  b 0x82e6a100
	pc = 0x82E6A100; continue 'dispatch;
	// 82E6A0E8: E92BFFF0  ld r9, -0x10(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-16 as u32) ) };
	// 82E6A0EC: 394BFFF0  addi r10, r11, -0x10
	ctx.r[10].s64 = ctx.r[11].s64 + -16;
	// 82E6A0F0: F92B0000  std r9, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82E6A0F4: 812BFFF8  lwz r9, -8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E6A0F8: 912B0008  stw r9, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82E6A0FC: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82E6A100: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82E6A104: 4199FFE4  bgt cr6, 0x82e6a0e8
	if ctx.cr[6].gt {
	pc = 0x82E6A0E8; continue 'dispatch;
	}
	// 82E6A108: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E6A10C: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6A110: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82E6A114: 2F0B0010  cmpwi cr6, r11, 0x10
	ctx.cr[6].compare_i32(ctx.r[11].s32, 16, &mut ctx.xer);
	// 82E6A118: 41990020  bgt cr6, 0x82e6a138
	if ctx.cr[6].gt {
	pc = 0x82E6A138; continue 'dispatch;
	}
	// 82E6A11C: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82E6A120: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6A124: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82E6A128: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82E6A12C: 40980008  bge cr6, 0x82e6a134
	if !ctx.cr[6].lt {
	pc = 0x82E6A134; continue 'dispatch;
	}
	// 82E6A130: 388B0001  addi r4, r11, 1
	ctx.r[4].s64 = ctx.r[11].s64 + 1;
	// 82E6A134: 4BFFFB05  bl 0x82e69c38
	ctx.lr = 0x82E6A138;
	sub_82E69C38(ctx, base);
	// 82E6A138: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6A13C: 815F0020  lwz r10, 0x20(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E6A140: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82E6A144: 7C7C5214  add r3, r28, r10
	ctx.r[3].u64 = ctx.r[28].u64 + ctx.r[10].u64;
	// 82E6A148: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E6A14C: E97D0000  ld r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) };
	// 82E6A150: F9630000  std r11, 0(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 82E6A154: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6A158: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E6A15C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E6A160: 4833E058  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6A168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E6A168 size=304
    let mut pc: u32 = 0x82E6A168;
    'dispatch: loop {
        match pc {
            0x82E6A168 => {
    //   block [0x82E6A168..0x82E6A298)
	// 82E6A168: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6A16C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E6A170: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E6A174: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E6A178: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E6A17C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E6A180: 7C892378  mr r9, r4
	ctx.r[9].u64 = ctx.r[4].u64;
	// 82E6A184: 897F0036  lbz r11, 0x36(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(54 as u32) ) } as u64;
	// 82E6A188: 556B07BD  rlwinm. r11, r11, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E6A18C: 4182006C  beq 0x82e6a1f8
	if ctx.cr[0].eq {
	pc = 0x82E6A1F8; continue 'dispatch;
	}
	// 82E6A190: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E6A194: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82E6A198: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E6A19C: 7FCB4850  subf r30, r11, r9
	ctx.r[30].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 82E6A1A0: 4BFFFC79  bl 0x82e69e18
	ctx.lr = 0x82E6A1A4;
	sub_82E69E18(ctx, base);
	// 82E6A1A4: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E6A1A8: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E6A1AC: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E6A1B0: 894A0006  lbz r10, 6(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(6 as u32) ) } as u64;
	// 82E6A1B4: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E6A1B8: 40820038  bne 0x82e6a1f0
	if !ctx.cr[0].eq {
	pc = 0x82E6A1F0; continue 'dispatch;
	}
	// 82E6A1BC: 48000018  b 0x82e6a1d4
	pc = 0x82E6A1D4; continue 'dispatch;
	// 82E6A1C0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82E6A1C4: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82E6A1C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6A1CC: 4BFFFC4D  bl 0x82e69e18
	ctx.lr = 0x82E6A1D0;
	sub_82E69E18(ctx, base);
	// 82E6A1D0: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E6A1D4: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E6A1D8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E6A1DC: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E6A1E0: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E6A1E4: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82E6A1E8: 914B0014  stw r10, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82E6A1EC: 409AFFD4  bne cr6, 0x82e6a1c0
	if !ctx.cr[6].eq {
	pc = 0x82E6A1C0; continue 'dispatch;
	}
	// 82E6A1F0: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E6A1F4: 7D3E5A14  add r9, r30, r11
	ctx.r[9].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 82E6A1F8: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E6A1FC: 394BFFE8  addi r10, r11, -0x18
	ctx.r[10].s64 = ctx.r[11].s64 + -24;
	// 82E6A200: 915F0014  stw r10, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82E6A204: 810BFFE8  lwz r8, -0x18(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24 as u32) ) } as u64;
	// 82E6A208: 80CB0010  lwz r6, 0x10(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E6A20C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E6A210: 2C060000  cmpwi r6, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82E6A214: 911F000C  stw r8, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82E6A218: 816BFFF4  lwz r11, -0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12 as u32) ) } as u64;
	// 82E6A21C: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82E6A220: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 82E6A224: 41820038  beq 0x82e6a25c
	if ctx.cr[0].eq {
	pc = 0x82E6A25C; continue 'dispatch;
	}
	// 82E6A228: 811F0008  lwz r8, 8(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6A22C: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82E6A230: 4098002C  bge cr6, 0x82e6a25c
	if !ctx.cr[6].lt {
	pc = 0x82E6A25C; continue 'dispatch;
	}
	// 82E6A234: 7D284B78  mr r8, r9
	ctx.r[8].u64 = ctx.r[9].u64;
	// 82E6A238: 7D475378  mr r7, r10
	ctx.r[7].u64 = ctx.r[10].u64;
	// 82E6A23C: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E6A240: 39290010  addi r9, r9, 0x10
	ctx.r[9].s64 = ctx.r[9].s64 + 16;
	// 82E6A244: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 82E6A248: E8A80000  ld r5, 0(r8)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) };
	// 82E6A24C: F8A70000  std r5, 0(r7)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[5].u64 ) };
	// 82E6A250: 81080008  lwz r8, 8(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6A254: 91070008  stw r8, 8(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 82E6A258: 4082FFD0  bne 0x82e6a228
	if !ctx.cr[0].eq {
	pc = 0x82E6A228; continue 'dispatch;
	}
	// 82E6A25C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E6A260: 40990018  ble cr6, 0x82e6a278
	if !ctx.cr[6].gt {
	pc = 0x82E6A278; continue 'dispatch;
	}
	// 82E6A264: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82E6A268: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E6A26C: 912A0008  stw r9, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82E6A270: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 82E6A274: 4181FFF0  bgt 0x82e6a264
	if ctx.cr[0].gt {
	pc = 0x82E6A264; continue 'dispatch;
	}
	// 82E6A278: 38660001  addi r3, r6, 1
	ctx.r[3].s64 = ctx.r[6].s64 + 1;
	// 82E6A27C: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E6A280: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E6A284: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E6A288: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E6A28C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E6A290: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E6A294: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6A298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E6A298 size=180
    let mut pc: u32 = 0x82E6A298;
    'dispatch: loop {
        match pc {
            0x82E6A298 => {
    //   block [0x82E6A298..0x82E6A34C)
	// 82E6A298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6A29C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E6A2A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E6A2A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E6A2A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E6A2AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E6A2B0: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82E6A2B4: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E6A2B8: 83CA0000  lwz r30, 0(r10)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E6A2BC: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82E6A2C0: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82E6A2C4: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E6A2C8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E6A2CC: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82E6A2D0: 409AFFF4  bne cr6, 0x82e6a2c4
	if !ctx.cr[6].eq {
	pc = 0x82E6A2C4; continue 'dispatch;
	}
	// 82E6A2D4: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82E6A2D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6A2DC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82E6A2E0: 5565003E  slwi r5, r11, 0
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E6A2E4: 4800A475  bl 0x82e74758
	ctx.lr = 0x82E6A2E8;
	sub_82E74758(ctx, base);
	// 82E6A2E8: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 82E6A2EC: 907E0000  stw r3, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82E6A2F0: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E6A2F4: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E6A2F8: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6A2FC: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82E6A300: 2F0B0010  cmpwi cr6, r11, 0x10
	ctx.cr[6].compare_i32(ctx.r[11].s32, 16, &mut ctx.xer);
	// 82E6A304: 41990020  bgt cr6, 0x82e6a324
	if ctx.cr[6].gt {
	pc = 0x82E6A324; continue 'dispatch;
	}
	// 82E6A308: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82E6A30C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6A310: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82E6A314: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82E6A318: 40980008  bge cr6, 0x82e6a320
	if !ctx.cr[6].lt {
	pc = 0x82E6A320; continue 'dispatch;
	}
	// 82E6A31C: 388B0001  addi r4, r11, 1
	ctx.r[4].s64 = ctx.r[11].s64 + 1;
	// 82E6A320: 4BFFF919  bl 0x82e69c38
	ctx.lr = 0x82E6A324;
	sub_82E69C38(ctx, base);
	// 82E6A324: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6A328: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 82E6A32C: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82E6A330: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E6A334: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E6A338: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E6A33C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E6A340: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E6A344: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E6A348: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6A350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E6A350 size=104
    let mut pc: u32 = 0x82E6A350;
    'dispatch: loop {
        match pc {
            0x82E6A350 => {
    //   block [0x82E6A350..0x82E6A3B8)
	// 82E6A350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6A354: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E6A358: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E6A35C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E6A360: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E6A364: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E6A368: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E6A36C: A17F0034  lhz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82E6A370: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E6A374: 41820010  beq 0x82e6a384
	if ctx.cr[0].eq {
	pc = 0x82E6A384; continue 'dispatch;
	}
	// 82E6A378: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E6A37C: 388BD204  addi r4, r11, -0x2dfc
	ctx.r[4].s64 = ctx.r[11].s64 + -11772;
	// 82E6A380: 480075B9  bl 0x82e71938
	ctx.lr = 0x82E6A384;
	sub_82E71938(ctx, base);
	// 82E6A384: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6A388: 57CA2036  slwi r10, r30, 4
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E6A38C: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82E6A390: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82E6A394: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82E6A398: 993F0006  stb r9, 6(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[9].u8 ) };
	// 82E6A39C: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82E6A3A0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E6A3A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E6A3A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E6A3AC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E6A3B0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E6A3B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6A3B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E6A3B8 size=276
    let mut pc: u32 = 0x82E6A3B8;
    'dispatch: loop {
        match pc {
            0x82E6A3B8 => {
    //   block [0x82E6A3B8..0x82E6A4CC)
	// 82E6A3B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6A3BC: 4833DDA9  bl 0x831a8164
	ctx.lr = 0x82E6A3C0;
	sub_831A8130(ctx, base);
	// 82E6A3C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E6A3C4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E6A3C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E6A3CC: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E6A3D0: 4800AAF1  bl 0x82e74ec0
	ctx.lr = 0x82E6A3D4;
	sub_82E74EC0(ctx, base);
	// 82E6A3D4: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E6A3D8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E6A3DC: 814B0044  lwz r10, 0x44(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 82E6A3E0: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 82E6A3E4: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E6A3E8: 4198000C  blt cr6, 0x82e6a3f4
	if ctx.cr[6].lt {
	pc = 0x82E6A3F4; continue 'dispatch;
	}
	// 82E6A3EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6A3F0: 48006AE1  bl 0x82e70ed0
	ctx.lr = 0x82E6A3F4;
	sub_82E70ED0(ctx, base);
	// 82E6A3F4: 2F1D001B  cmpwi cr6, r29, 0x1b
	ctx.cr[6].compare_i32(ctx.r[29].s32, 27, &mut ctx.xer);
	// 82E6A3F8: 409A0010  bne cr6, 0x82e6a408
	if !ctx.cr[6].eq {
	pc = 0x82E6A408; continue 'dispatch;
	}
	// 82E6A3FC: 3D6082E8  lis r11, -0x7d18
	ctx.r[11].s64 = -2098724864;
	// 82E6A400: 396B8408  addi r11, r11, -0x7bf8
	ctx.r[11].s64 = ctx.r[11].s64 + -31736;
	// 82E6A404: 4800000C  b 0x82e6a410
	pc = 0x82E6A410; continue 'dispatch;
	// 82E6A408: 3D6082E7  lis r11, -0x7d19
	ctx.r[11].s64 = -2098790400;
	// 82E6A40C: 396B7C48  addi r11, r11, 0x7c48
	ctx.r[11].s64 = ctx.r[11].s64 + 31816;
	// 82E6A410: 80DE0010  lwz r6, 0x10(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E6A414: 38BE0004  addi r5, r30, 4
	ctx.r[5].s64 = ctx.r[30].s64 + 4;
	// 82E6A418: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E6A41C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6A420: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E6A424: 4E800421  bctrl
	ctx.lr = 0x82E6A428;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E6A428: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E6A42C: 80BF0048  lwz r5, 0x48(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82E6A430: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6A434: 889E0048  lbz r4, 0x48(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(72 as u32) ) } as u64;
	// 82E6A438: 4800A551  bl 0x82e74988
	ctx.lr = 0x82E6A43C;
	sub_82E74988(ctx, base);
	// 82E6A43C: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82E6A440: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82E6A444: 93DB0010  stw r30, 0x10(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 82E6A448: 897E0048  lbz r11, 0x48(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(72 as u32) ) } as u64;
	// 82E6A44C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E6A450: 41820028  beq 0x82e6a478
	if ctx.cr[0].eq {
	pc = 0x82E6A478; continue 'dispatch;
	}
	// 82E6A454: 3B9B0014  addi r28, r27, 0x14
	ctx.r[28].s64 = ctx.r[27].s64 + 20;
	// 82E6A458: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6A45C: 4800A5AD  bl 0x82e74a08
	ctx.lr = 0x82E6A460;
	sub_82E74A08(ctx, base);
	// 82E6A460: 907C0000  stw r3, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82E6A464: 897E0048  lbz r11, 0x48(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(72 as u32) ) } as u64;
	// 82E6A468: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82E6A46C: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 82E6A470: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E6A474: 4198FFE4  blt cr6, 0x82e6a458
	if ctx.cr[6].lt {
	pc = 0x82E6A458; continue 'dispatch;
	}
	// 82E6A478: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6A47C: 39400006  li r10, 6
	ctx.r[10].s64 = 6;
	// 82E6A480: 936B0000  stw r27, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82E6A484: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E6A488: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E6A48C: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6A490: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82E6A494: 2F0B0010  cmpwi cr6, r11, 0x10
	ctx.cr[6].compare_i32(ctx.r[11].s32, 16, &mut ctx.xer);
	// 82E6A498: 41990020  bgt cr6, 0x82e6a4b8
	if ctx.cr[6].gt {
	pc = 0x82E6A4B8; continue 'dispatch;
	}
	// 82E6A49C: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82E6A4A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6A4A4: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82E6A4A8: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82E6A4AC: 40980008  bge cr6, 0x82e6a4b4
	if !ctx.cr[6].lt {
	pc = 0x82E6A4B4; continue 'dispatch;
	}
	// 82E6A4B0: 388B0001  addi r4, r11, 1
	ctx.r[4].s64 = ctx.r[11].s64 + 1;
	// 82E6A4B4: 4BFFF785  bl 0x82e69c38
	ctx.lr = 0x82E6A4B8;
	sub_82E69C38(ctx, base);
	// 82E6A4B8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6A4BC: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82E6A4C0: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E6A4C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E6A4C8: 4833DCEC  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6A4D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E6A4D0 size=168
    let mut pc: u32 = 0x82E6A4D0;
    'dispatch: loop {
        match pc {
            0x82E6A4D0 => {
    //   block [0x82E6A4D0..0x82E6A578)
	// 82E6A4D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6A4D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E6A4D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E6A4DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E6A4E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E6A4E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E6A4E8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E6A4EC: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82E6A4F0: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82E6A4F4: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E6A4F8: 909F000C  stw r4, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 82E6A4FC: 4800A66D  bl 0x82e74b68
	ctx.lr = 0x82E6A500;
	sub_82E74B68(ctx, base);
	// 82E6A500: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E6A504: 80BF000C  lwz r5, 0xc(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E6A508: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6A50C: 4BFFF60D  bl 0x82e69b18
	ctx.lr = 0x82E6A510;
	sub_82E69B18(ctx, base);
	// 82E6A510: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82E6A514: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82E6A518: 815F0030  lwz r10, 0x30(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E6A51C: 2F0A4E20  cmpwi cr6, r10, 0x4e20
	ctx.cr[6].compare_i32(ctx.r[10].s32, 20000, &mut ctx.xer);
	// 82E6A520: B3DF0034  sth r30, 0x34(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[30].u16 ) };
	// 82E6A524: 997F0037  stb r11, 0x37(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(55 as u32), ctx.r[11].u8 ) };
	// 82E6A528: 40990030  ble cr6, 0x82e6a558
	if !ctx.cr[6].gt {
	pc = 0x82E6A558; continue 'dispatch;
	}
	// 82E6A52C: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E6A530: 39200018  li r9, 0x18
	ctx.r[9].s64 = 24;
	// 82E6A534: 815F0028  lwz r10, 0x28(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82E6A538: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82E6A53C: 7D6B4BD6  divw r11, r11, r9
	ctx.r[11].s32 = ctx.r[11].s32 / ctx.r[9].s32;
	// 82E6A540: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E6A544: 2F0B4E20  cmpwi cr6, r11, 0x4e20
	ctx.cr[6].compare_i32(ctx.r[11].s32, 20000, &mut ctx.xer);
	// 82E6A548: 40980010  bge cr6, 0x82e6a558
	if !ctx.cr[6].lt {
	pc = 0x82E6A558; continue 'dispatch;
	}
	// 82E6A54C: 38804E20  li r4, 0x4e20
	ctx.r[4].s64 = 20000;
	// 82E6A550: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6A554: 4BFFF81D  bl 0x82e69d70
	ctx.lr = 0x82E6A558;
	sub_82E69D70(ctx, base);
	// 82E6A558: 93DF0074  stw r30, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[30].u32 ) };
	// 82E6A55C: 93DF0070  stw r30, 0x70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[30].u32 ) };
	// 82E6A560: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E6A564: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E6A568: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E6A56C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E6A570: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E6A574: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6A578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E6A578 size=116
    let mut pc: u32 = 0x82E6A578;
    'dispatch: loop {
        match pc {
            0x82E6A578 => {
    //   block [0x82E6A578..0x82E6A5EC)
	// 82E6A578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6A57C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E6A580: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E6A584: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E6A588: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E6A58C: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82E6A590: 815F0070  lwz r10, 0x70(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 82E6A594: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E6A598: 41820018  beq 0x82e6a5b0
	if ctx.cr[0].eq {
	pc = 0x82E6A5B0; continue 'dispatch;
	}
	// 82E6A59C: 916A0550  stw r11, 0x550(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(1360 as u32), ctx.r[11].u32 ) };
	// 82E6A5A0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E6A5A4: 817F0070  lwz r11, 0x70(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 82E6A5A8: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82E6A5AC: 48343735  bl 0x831adce0
	ctx.lr = 0x82E6A5B0;
	sub_831ADCE0(ctx, base);
	// 82E6A5B0: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E6A5B4: 997F0006  stb r11, 6(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[11].u8 ) };
	// 82E6A5B8: 81490058  lwz r10, 0x58(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E6A5BC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E6A5C0: 419A0024  beq cr6, 0x82e6a5e4
	if ctx.cr[6].eq {
	pc = 0x82E6A5E4; continue 'dispatch;
	}
	// 82E6A5C4: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82E6A5C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6A5CC: 4BFFFF05  bl 0x82e6a4d0
	ctx.lr = 0x82E6A5D0;
	sub_82E6A4D0(ctx, base);
	// 82E6A5D0: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E6A5D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6A5D8: 816B0058  lwz r11, 0x58(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E6A5DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E6A5E0: 4E800421  bctrl
	ctx.lr = 0x82E6A5E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E6A5E4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E6A5E8: 48342599  bl 0x831acb80
	ctx.lr = 0x82E6A5EC;
	sub_831ACB80(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6A5F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E6A5F0 size=740
    let mut pc: u32 = 0x82E6A5F0;
    'dispatch: loop {
        match pc {
            0x82E6A5F0 => {
    //   block [0x82E6A5F0..0x82E6A8D4)
	// 82E6A5F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6A5F4: 4833DB6D  bl 0x831a8160
	ctx.lr = 0x82E6A5F8;
	sub_831A8130(ctx, base);
	// 82E6A5F8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E6A5FC: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6A600: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E6A604: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 82E6A608: 2F0B0006  cmpwi cr6, r11, 6
	ctx.cr[6].compare_i32(ctx.r[11].s32, 6, &mut ctx.xer);
	// 82E6A60C: 419A000C  beq cr6, 0x82e6a618
	if ctx.cr[6].eq {
	pc = 0x82E6A618; continue 'dispatch;
	}
	// 82E6A610: 4BFFFA89  bl 0x82e6a098
	ctx.lr = 0x82E6A614;
	sub_82E6A098(ctx, base);
	// 82E6A614: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E6A618: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E6A61C: 813F0018  lwz r9, 0x18(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E6A620: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E6A624: 811F0020  lwz r8, 0x20(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E6A628: 7F682050  subf r27, r8, r4
	ctx.r[27].s64 = ctx.r[4].s64 - ctx.r[8].s64;
	// 82E6A62C: 912A000C  stw r9, 0xc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82E6A630: 894B0006  lbz r10, 6(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(6 as u32) ) } as u64;
	// 82E6A634: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E6A638: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6A63C: 40820184  bne 0x82e6a7c0
	if !ctx.cr[0].eq {
	pc = 0x82E6A7C0; continue 'dispatch;
	}
	// 82E6A640: 83AB0010  lwz r29, 0x10(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E6A644: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E6A648: 7D4A5850  subf r10, r10, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82E6A64C: 897D004B  lbz r11, 0x4b(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(75 as u32) ) } as u64;
	// 82E6A650: 5569203E  rotlwi r9, r11, 4
	ctx.r[9].u64 = ((ctx.r[11].u32).rotate_left(4)) as u64;
	// 82E6A654: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82E6A658: 41990020  bgt cr6, 0x82e6a678
	if ctx.cr[6].gt {
	pc = 0x82E6A678; continue 'dispatch;
	}
	// 82E6A65C: 815F002C  lwz r10, 0x2c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82E6A660: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6A664: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82E6A668: 5544083C  slwi r4, r10, 1
	ctx.r[4].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82E6A66C: 40990008  ble cr6, 0x82e6a674
	if !ctx.cr[6].gt {
	pc = 0x82E6A674; continue 'dispatch;
	}
	// 82E6A670: 7C8A5A14  add r4, r10, r11
	ctx.r[4].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82E6A674: 4BFFF5C5  bl 0x82e69c38
	ctx.lr = 0x82E6A678;
	sub_82E69C38(ctx, base);
	// 82E6A678: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E6A67C: 895D004A  lbz r10, 0x4a(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(74 as u32) ) } as u64;
	// 82E6A680: 7F9B5A14  add r28, r27, r11
	ctx.r[28].u64 = ctx.r[27].u64 + ctx.r[11].u64;
	// 82E6A684: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E6A688: 40820028  bne 0x82e6a6b0
	if !ctx.cr[0].eq {
	pc = 0x82E6A6B0; continue 'dispatch;
	}
	// 82E6A68C: 897D0049  lbz r11, 0x49(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(73 as u32) ) } as u64;
	// 82E6A690: 3BDC0010  addi r30, r28, 0x10
	ctx.r[30].s64 = ctx.r[28].s64 + 16;
	// 82E6A694: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6A698: 556B203E  rotlwi r11, r11, 4
	ctx.r[11].u64 = ((ctx.r[11].u32).rotate_left(4)) as u64;
	// 82E6A69C: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82E6A6A0: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E6A6A4: 40990034  ble cr6, 0x82e6a6d8
	if !ctx.cr[6].gt {
	pc = 0x82E6A6D8; continue 'dispatch;
	}
	// 82E6A6A8: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E6A6AC: 4800002C  b 0x82e6a6d8
	pc = 0x82E6A6D8; continue 'dispatch;
	// 82E6A6B0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6A6B4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E6A6B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6A6BC: 7D7C5850  subf r11, r28, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[28].s64;
	// 82E6A6C0: 7D6B2670  srawi r11, r11, 4
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 4) as i64;
	// 82E6A6C4: 38ABFFFF  addi r5, r11, -1
	ctx.r[5].s64 = ctx.r[11].s64 + -1;
	// 82E6A6C8: 4BFFF841  bl 0x82e69f08
	ctx.lr = 0x82E6A6CC;
	sub_82E69F08(ctx, base);
	// 82E6A6CC: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E6A6D0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E6A6D4: 7F9B5A14  add r28, r27, r11
	ctx.r[28].u64 = ctx.r[27].u64 + ctx.r[11].u64;
	// 82E6A6D8: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E6A6DC: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82E6A6E0: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82E6A6E4: 409A0044  bne cr6, 0x82e6a728
	if !ctx.cr[6].eq {
	pc = 0x82E6A728; continue 'dispatch;
	}
	// 82E6A6E8: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E6A6EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6A6F0: 2F0B4E20  cmpwi cr6, r11, 0x4e20
	ctx.cr[6].compare_i32(ctx.r[11].s32, 20000, &mut ctx.xer);
	// 82E6A6F4: 4099000C  ble cr6, 0x82e6a700
	if !ctx.cr[6].gt {
	pc = 0x82E6A700; continue 'dispatch;
	}
	// 82E6A6F8: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 82E6A6FC: 4BFFFE7D  bl 0x82e6a578
	ctx.lr = 0x82E6A700;
	sub_82E6A578(ctx, base);
	// 82E6A700: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82E6A704: 4BFFF66D  bl 0x82e69d70
	ctx.lr = 0x82E6A708;
	sub_82E69D70(ctx, base);
	// 82E6A708: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E6A70C: 2F0B4E20  cmpwi cr6, r11, 0x4e20
	ctx.cr[6].compare_i32(ctx.r[11].s32, 20000, &mut ctx.xer);
	// 82E6A710: 40990014  ble cr6, 0x82e6a724
	if !ctx.cr[6].gt {
	pc = 0x82E6A724; continue 'dispatch;
	}
	// 82E6A714: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E6A718: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6A71C: 388BD238  addi r4, r11, -0x2dc8
	ctx.r[4].s64 = ctx.r[11].s64 + -11720;
	// 82E6A720: 48007219  bl 0x82e71938
	ctx.lr = 0x82E6A724;
	sub_82E71938(ctx, base);
	// 82E6A724: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E6A728: 396B0018  addi r11, r11, 0x18
	ctx.r[11].s64 = ctx.r[11].s64 + 24;
	// 82E6A72C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82E6A730: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82E6A734: 938B0004  stw r28, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82E6A738: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82E6A73C: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 82E6A740: 895D004B  lbz r10, 0x4b(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(75 as u32) ) } as u64;
	// 82E6A744: 554A203E  rotlwi r10, r10, 4
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(4)) as u64;
	// 82E6A748: 7D4AF214  add r10, r10, r30
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 82E6A74C: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E6A750: 815D000C  lwz r10, 0xc(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E6A754: 915F0018  stw r10, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 82E6A758: 912B0014  stw r9, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 82E6A75C: 934B0010  stw r26, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[26].u32 ) };
	// 82E6A760: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6A764: 4800000C  b 0x82e6a770
	pc = 0x82E6A770; continue 'dispatch;
	// 82E6A768: 912A0008  stw r9, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82E6A76C: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 82E6A770: 810B0008  lwz r8, 8(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6A774: 7F0A4040  cmplw cr6, r10, r8
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82E6A778: 4198FFF0  blt cr6, 0x82e6a768
	if ctx.cr[6].lt {
	pc = 0x82E6A768; continue 'dispatch;
	}
	// 82E6A77C: 895F0036  lbz r10, 0x36(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(54 as u32) ) } as u64;
	// 82E6A780: 550B003E  slwi r11, r8, 0
	ctx.r[11].u32 = ctx.r[8].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E6A784: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E6A788: 554A07FF  clrlwi. r10, r10, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E6A78C: 4182002C  beq 0x82e6a7b8
	if ctx.cr[0].eq {
	pc = 0x82E6A7B8; continue 'dispatch;
	}
	// 82E6A790: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E6A794: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82E6A798: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E6A79C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82E6A7A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6A7A4: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82E6A7A8: 4BFFF671  bl 0x82e69e18
	ctx.lr = 0x82E6A7AC;
	sub_82E69E18(ctx, base);
	// 82E6A7AC: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E6A7B0: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 82E6A7B4: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82E6A7B8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E6A7BC: 48000110  b 0x82e6a8cc
	pc = 0x82E6A8CC; continue 'dispatch;
	// 82E6A7C0: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E6A7C4: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82E6A7C8: 2F0B0140  cmpwi cr6, r11, 0x140
	ctx.cr[6].compare_i32(ctx.r[11].s32, 320, &mut ctx.xer);
	// 82E6A7CC: 41990020  bgt cr6, 0x82e6a7ec
	if ctx.cr[6].gt {
	pc = 0x82E6A7EC; continue 'dispatch;
	}
	// 82E6A7D0: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82E6A7D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6A7D8: 2F0B0014  cmpwi cr6, r11, 0x14
	ctx.cr[6].compare_i32(ctx.r[11].s32, 20, &mut ctx.xer);
	// 82E6A7DC: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82E6A7E0: 40980008  bge cr6, 0x82e6a7e8
	if !ctx.cr[6].lt {
	pc = 0x82E6A7E8; continue 'dispatch;
	}
	// 82E6A7E4: 388B0014  addi r4, r11, 0x14
	ctx.r[4].s64 = ctx.r[11].s64 + 20;
	// 82E6A7E8: 4BFFF451  bl 0x82e69c38
	ctx.lr = 0x82E6A7EC;
	sub_82E69C38(ctx, base);
	// 82E6A7EC: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E6A7F0: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82E6A7F4: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82E6A7F8: 409A0044  bne cr6, 0x82e6a83c
	if !ctx.cr[6].eq {
	pc = 0x82E6A83C; continue 'dispatch;
	}
	// 82E6A7FC: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E6A800: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6A804: 2F0B4E20  cmpwi cr6, r11, 0x4e20
	ctx.cr[6].compare_i32(ctx.r[11].s32, 20000, &mut ctx.xer);
	// 82E6A808: 4099000C  ble cr6, 0x82e6a814
	if !ctx.cr[6].gt {
	pc = 0x82E6A814; continue 'dispatch;
	}
	// 82E6A80C: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 82E6A810: 4BFFFD69  bl 0x82e6a578
	ctx.lr = 0x82E6A814;
	sub_82E6A578(ctx, base);
	// 82E6A814: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82E6A818: 4BFFF559  bl 0x82e69d70
	ctx.lr = 0x82E6A81C;
	sub_82E69D70(ctx, base);
	// 82E6A81C: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E6A820: 2F0B4E20  cmpwi cr6, r11, 0x4e20
	ctx.cr[6].compare_i32(ctx.r[11].s32, 20000, &mut ctx.xer);
	// 82E6A824: 40990014  ble cr6, 0x82e6a838
	if !ctx.cr[6].gt {
	pc = 0x82E6A838; continue 'dispatch;
	}
	// 82E6A828: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E6A82C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6A830: 388BD238  addi r4, r11, -0x2dc8
	ctx.r[4].s64 = ctx.r[11].s64 + -11720;
	// 82E6A834: 48007105  bl 0x82e71938
	ctx.lr = 0x82E6A838;
	sub_82E71938(ctx, base);
	// 82E6A838: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E6A83C: 815F0020  lwz r10, 0x20(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E6A840: 396B0018  addi r11, r11, 0x18
	ctx.r[11].s64 = ctx.r[11].s64 + 24;
	// 82E6A844: 7D5B5214  add r10, r27, r10
	ctx.r[10].u64 = ctx.r[27].u64 + ctx.r[10].u64;
	// 82E6A848: 392A0010  addi r9, r10, 0x10
	ctx.r[9].s64 = ctx.r[10].s64 + 16;
	// 82E6A84C: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82E6A850: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E6A854: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82E6A858: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6A85C: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82E6A860: 394A0140  addi r10, r10, 0x140
	ctx.r[10].s64 = ctx.r[10].s64 + 320;
	// 82E6A864: 934B0010  stw r26, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[26].u32 ) };
	// 82E6A868: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E6A86C: 897F0036  lbz r11, 0x36(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(54 as u32) ) } as u64;
	// 82E6A870: 556B07FF  clrlwi. r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E6A874: 41820014  beq 0x82e6a888
	if ctx.cr[0].eq {
	pc = 0x82E6A888; continue 'dispatch;
	}
	// 82E6A878: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82E6A87C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E6A880: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6A884: 4BFFF595  bl 0x82e69e18
	ctx.lr = 0x82E6A888;
	sub_82E69E18(ctx, base);
	// 82E6A888: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E6A88C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6A890: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E6A894: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E6A898: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E6A89C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E6A8A0: 4E800421  bctrl
	ctx.lr = 0x82E6A8A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E6A8A4: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82E6A8A8: 4080000C  bge 0x82e6a8b4
	if !ctx.cr[0].lt {
	pc = 0x82E6A8B4; continue 'dispatch;
	}
	// 82E6A8AC: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 82E6A8B0: 4800001C  b 0x82e6a8cc
	pc = 0x82E6A8CC; continue 'dispatch;
	// 82E6A8B4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6A8B8: 546A2036  slwi r10, r3, 4
	ctx.r[10].u32 = ctx.r[3].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E6A8BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6A8C0: 7C8A5850  subf r4, r10, r11
	ctx.r[4].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82E6A8C4: 4BFFF8A5  bl 0x82e6a168
	ctx.lr = 0x82E6A8C8;
	sub_82E6A168(ctx, base);
	// 82E6A8C8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E6A8CC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E6A8D0: 4833D8E0  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6A8D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E6A8D8 size=184
    let mut pc: u32 = 0x82E6A8D8;
    'dispatch: loop {
        match pc {
            0x82E6A8D8 => {
    //   block [0x82E6A8D8..0x82E6A990)
	// 82E6A8D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6A8DC: 4833D891  bl 0x831a816c
	ctx.lr = 0x82E6A8E0;
	sub_831A8130(ctx, base);
	// 82E6A8E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E6A8E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E6A8E8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E6A8EC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82E6A8F0: A17F0034  lhz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82E6A8F4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E6A8F8: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 82E6A8FC: 2B0B00C8  cmplwi cr6, r11, 0xc8
	ctx.cr[6].compare_u32(ctx.r[11].u32, 200 as u32, &mut ctx.xer);
	// 82E6A900: B17F0034  sth r11, 0x34(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u16 ) };
	// 82E6A904: 4198001C  blt cr6, 0x82e6a920
	if ctx.cr[6].lt {
	pc = 0x82E6A920; continue 'dispatch;
	}
	// 82E6A908: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 82E6A90C: 2B0B00C8  cmplwi cr6, r11, 0xc8
	ctx.cr[6].compare_u32(ctx.r[11].u32, 200 as u32, &mut ctx.xer);
	// 82E6A910: 409A0068  bne cr6, 0x82e6a978
	if !ctx.cr[6].eq {
	pc = 0x82E6A978; continue 'dispatch;
	}
	// 82E6A914: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E6A918: 388BD248  addi r4, r11, -0x2db8
	ctx.r[4].s64 = ctx.r[11].s64 + -11704;
	// 82E6A91C: 4800701D  bl 0x82e71938
	ctx.lr = 0x82E6A920;
	sub_82E71938(ctx, base);
	// 82E6A920: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E6A924: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E6A928: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6A92C: 4BFFFCC5  bl 0x82e6a5f0
	ctx.lr = 0x82E6A930;
	sub_82E6A5F0(ctx, base);
	// 82E6A930: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82E6A934: 40820010  bne 0x82e6a944
	if !ctx.cr[0].eq {
	pc = 0x82E6A944; continue 'dispatch;
	}
	// 82E6A938: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E6A93C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6A940: 48007F31  bl 0x82e72870
	ctx.lr = 0x82E6A944;
	sub_82E72870(ctx, base);
	// 82E6A944: A15F0034  lhz r10, 0x34(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82E6A948: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E6A94C: 3D4A0001  addis r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 65536;
	// 82E6A950: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82E6A954: B15F0034  sth r10, 0x34(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[10].u16 ) };
	// 82E6A958: 814B0044  lwz r10, 0x44(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 82E6A95C: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 82E6A960: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E6A964: 4198000C  blt cr6, 0x82e6a970
	if ctx.cr[6].lt {
	pc = 0x82E6A970; continue 'dispatch;
	}
	// 82E6A968: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6A96C: 48006565  bl 0x82e70ed0
	ctx.lr = 0x82E6A970;
	sub_82E70ED0(ctx, base);
	// 82E6A970: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E6A974: 4833D848  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 82E6A978: A17F0034  lhz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82E6A97C: 2B0B00E1  cmplwi cr6, r11, 0xe1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 225 as u32, &mut ctx.xer);
	// 82E6A980: 4198FFA0  blt cr6, 0x82e6a920
	if ctx.cr[6].lt {
	pc = 0x82E6A920; continue 'dispatch;
	}
	// 82E6A984: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 82E6A988: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6A98C: 4BFFFBED  bl 0x82e6a578
	ctx.lr = 0x82E6A990;
	sub_82E6A578(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6A990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E6A990 size=176
    let mut pc: u32 = 0x82E6A990;
    'dispatch: loop {
        match pc {
            0x82E6A990 => {
    //   block [0x82E6A990..0x82E6AA40)
	// 82E6A990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6A994: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E6A998: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E6A99C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E6A9A0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E6A9A4: 895F0006  lbz r10, 6(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 82E6A9A8: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E6A9AC: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E6A9B0: 4082001C  bne 0x82e6a9cc
	if !ctx.cr[0].eq {
	pc = 0x82E6A9CC; continue 'dispatch;
	}
	// 82E6A9B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82E6A9B8: 3884FFF0  addi r4, r4, -0x10
	ctx.r[4].s64 = ctx.r[4].s64 + -16;
	// 82E6A9BC: 4BFFFC35  bl 0x82e6a5f0
	ctx.lr = 0x82E6A9C0;
	sub_82E6A5F0(ctx, base);
	// 82E6A9C0: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82E6A9C4: 40820068  bne 0x82e6aa2c
	if !ctx.cr[0].eq {
	pc = 0x82E6AA2C; continue 'dispatch;
	}
	// 82E6A9C8: 48000048  b 0x82e6aa10
	pc = 0x82E6AA10; continue 'dispatch;
	// 82E6A9CC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E6A9D0: 995F0006  stb r10, 6(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[10].u8 ) };
	// 82E6A9D4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E6A9D8: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E6A9DC: 894A0006  lbz r10, 6(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(6 as u32) ) } as u64;
	// 82E6A9E0: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E6A9E4: 41820024  beq 0x82e6aa08
	if ctx.cr[0].eq {
	pc = 0x82E6AA08; continue 'dispatch;
	}
	// 82E6A9E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6A9EC: 4BFFF77D  bl 0x82e6a168
	ctx.lr = 0x82E6A9F0;
	sub_82E6A168(ctx, base);
	// 82E6A9F0: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82E6A9F4: 4182001C  beq 0x82e6aa10
	if ctx.cr[0].eq {
	pc = 0x82E6AA10; continue 'dispatch;
	}
	// 82E6A9F8: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E6A9FC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6AA00: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E6AA04: 4800000C  b 0x82e6aa10
	pc = 0x82E6AA10; continue 'dispatch;
	// 82E6AA08: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E6AA0C: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82E6AA10: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E6AA14: 39400018  li r10, 0x18
	ctx.r[10].s64 = 24;
	// 82E6AA18: 813F0028  lwz r9, 0x28(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82E6AA1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6AA20: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 82E6AA24: 7C8B53D6  divw r4, r11, r10
	ctx.r[4].s32 = ctx.r[11].s32 / ctx.r[10].s32;
	// 82E6AA28: 48007E49  bl 0x82e72870
	ctx.lr = 0x82E6AA2C;
	sub_82E72870(ctx, base);
	// 82E6AA2C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E6AA30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E6AA34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E6AA38: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E6AA3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6AA40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E6AA40 size=196
    let mut pc: u32 = 0x82E6AA40;
    'dispatch: loop {
        match pc {
            0x82E6AA40 => {
    //   block [0x82E6AA40..0x82E6AB04)
	// 82E6AA40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6AA44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E6AA48: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E6AA4C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E6AA50: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E6AA54: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E6AA58: 897F0006  lbz r11, 6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 82E6AA5C: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82E6AA60: 419A003C  beq cr6, 0x82e6aa9c
	if ctx.cr[6].eq {
	pc = 0x82E6AA9C; continue 'dispatch;
	}
	// 82E6AA64: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E6AA68: 419A0018  beq cr6, 0x82e6aa80
	if ctx.cr[6].eq {
	pc = 0x82E6AA80; continue 'dispatch;
	}
	// 82E6AA6C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E6AA70: 388BD284  addi r4, r11, -0x2d7c
	ctx.r[4].s64 = ctx.r[11].s64 + -11644;
	// 82E6AA74: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6AA78: 4BFFF821  bl 0x82e6a298
	ctx.lr = 0x82E6AA7C;
	sub_82E6A298(ctx, base);
	// 82E6AA7C: 48000070  b 0x82e6aaec
	pc = 0x82E6AAEC; continue 'dispatch;
	// 82E6AA80: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E6AA84: 815F0028  lwz r10, 0x28(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82E6AA88: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82E6AA8C: 419A0010  beq cr6, 0x82e6aa9c
	if ctx.cr[6].eq {
	pc = 0x82E6AA9C; continue 'dispatch;
	}
	// 82E6AA90: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E6AA94: 388BD25C  addi r4, r11, -0x2da4
	ctx.r[4].s64 = ctx.r[11].s64 + -11684;
	// 82E6AA98: 4BFFFFDC  b 0x82e6aa74
	pc = 0x82E6AA74; continue 'dispatch;
	// 82E6AA9C: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6AAA0: 54892036  slwi r9, r4, 4
	ctx.r[9].u32 = ctx.r[4].u32.wrapping_shl(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82E6AAA4: 3D6082E7  lis r11, -0x7d19
	ctx.r[11].s64 = -2098790400;
	// 82E6AAA8: 7CA95050  subf r5, r9, r10
	ctx.r[5].s64 = ctx.r[10].s64 - ctx.r[9].s64;
	// 82E6AAAC: 388BA990  addi r4, r11, -0x5670
	ctx.r[4].s64 = ctx.r[11].s64 + -22128;
	// 82E6AAB0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6AAB4: 4BFFF105  bl 0x82e69bb8
	ctx.lr = 0x82E6AAB8;
	sub_82E69BB8(ctx, base);
	// 82E6AAB8: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82E6AABC: 41820028  beq 0x82e6aae4
	if ctx.cr[0].eq {
	pc = 0x82E6AAE4; continue 'dispatch;
	}
	// 82E6AAC0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E6AAC4: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6AAC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6AACC: 9BDF0006  stb r30, 6(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[30].u8 ) };
	// 82E6AAD0: 4BFFF049  bl 0x82e69b18
	ctx.lr = 0x82E6AAD4;
	sub_82E69B18(ctx, base);
	// 82E6AAD4: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E6AAD8: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6AADC: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E6AAE0: 48000008  b 0x82e6aae8
	pc = 0x82E6AAE8; continue 'dispatch;
	// 82E6AAE4: 8BDF0006  lbz r30, 6(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 82E6AAE8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E6AAEC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E6AAF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E6AAF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E6AAF8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E6AAFC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E6AB00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6AB08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E6AB08 size=196
    let mut pc: u32 = 0x82E6AB08;
    'dispatch: loop {
        match pc {
            0x82E6AB08 => {
    //   block [0x82E6AB08..0x82E6ABCC)
	// 82E6AB08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6AB0C: 4833D651  bl 0x831a815c
	ctx.lr = 0x82E6AB10;
	sub_831A8130(ctx, base);
	// 82E6AB10: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E6AB14: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E6AB18: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82E6AB1C: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E6AB20: 815F0028  lwz r10, 0x28(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82E6AB24: 833F0074  lwz r25, 0x74(r31)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) } as u64;
	// 82E6AB28: A37F0034  lhz r27, 0x34(r31)
	ctx.r[27].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82E6AB2C: 7FAA5850  subf r29, r10, r11
	ctx.r[29].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82E6AB30: 8B5F0037  lbz r26, 0x37(r31)
	ctx.r[26].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(55 as u32) ) } as u64;
	// 82E6AB34: 90FF0074  stw r7, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[7].u32 ) };
	// 82E6AB38: 4BFFF081  bl 0x82e69bb8
	ctx.lr = 0x82E6AB3C;
	sub_82E69BB8(ctx, base);
	// 82E6AB3C: 7C7C1B79  or. r28, r3, r3
	ctx.r[28].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82E6AB40: 4182007C  beq 0x82e6abbc
	if ctx.cr[0].eq {
	pc = 0x82E6ABBC; continue 'dispatch;
	}
	// 82E6AB44: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E6AB48: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6AB4C: 7FCBF214  add r30, r11, r30
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82E6AB50: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E6AB54: 4800A015  bl 0x82e74b68
	ctx.lr = 0x82E6AB58;
	sub_82E74B68(ctx, base);
	// 82E6AB58: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E6AB5C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E6AB60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6AB64: 4BFFEFB5  bl 0x82e69b18
	ctx.lr = 0x82E6AB68;
	sub_82E69B18(ctx, base);
	// 82E6AB68: 815F0030  lwz r10, 0x30(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E6AB6C: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82E6AB70: 2F0A4E20  cmpwi cr6, r10, 0x4e20
	ctx.cr[6].compare_i32(ctx.r[10].s32, 20000, &mut ctx.xer);
	// 82E6AB74: B37F0034  sth r27, 0x34(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[27].u16 ) };
	// 82E6AB78: 7D4BEA14  add r10, r11, r29
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82E6AB7C: 915F0014  stw r10, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82E6AB80: 7D2BE82E  lwzx r9, r11, r29
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82E6AB84: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82E6AB88: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E6AB8C: 9B5F0037  stb r26, 0x37(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(55 as u32), ctx.r[26].u8 ) };
	// 82E6AB90: 913F0018  stw r9, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[9].u32 ) };
	// 82E6AB94: 40990028  ble cr6, 0x82e6abbc
	if !ctx.cr[6].gt {
	pc = 0x82E6ABBC; continue 'dispatch;
	}
	// 82E6AB98: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82E6AB9C: 39400018  li r10, 0x18
	ctx.r[10].s64 = 24;
	// 82E6ABA0: 7D6B53D6  divw r11, r11, r10
	ctx.r[11].s32 = ctx.r[11].s32 / ctx.r[10].s32;
	// 82E6ABA4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E6ABA8: 2F0B4E20  cmpwi cr6, r11, 0x4e20
	ctx.cr[6].compare_i32(ctx.r[11].s32, 20000, &mut ctx.xer);
	// 82E6ABAC: 40980010  bge cr6, 0x82e6abbc
	if !ctx.cr[6].lt {
	pc = 0x82E6ABBC; continue 'dispatch;
	}
	// 82E6ABB0: 38804E20  li r4, 0x4e20
	ctx.r[4].s64 = 20000;
	// 82E6ABB4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6ABB8: 4BFFF1B9  bl 0x82e69d70
	ctx.lr = 0x82E6ABBC;
	sub_82E69D70(ctx, base);
	// 82E6ABBC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82E6ABC0: 933F0074  stw r25, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[25].u32 ) };
	// 82E6ABC4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E6ABC8: 4833D5E4  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6ABD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E6ABD0 size=136
    let mut pc: u32 = 0x82E6ABD0;
    'dispatch: loop {
        match pc {
            0x82E6ABD0 => {
    //   block [0x82E6ABD0..0x82E6AC58)
	// 82E6ABD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6ABD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E6ABD8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E6ABDC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E6ABE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E6ABE4: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82E6ABE8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E6ABEC: 7CAA2B78  mr r10, r5
	ctx.r[10].u64 = ctx.r[5].u64;
	// 82E6ABF0: 3D2082E7  lis r9, -0x7d19
	ctx.r[9].s64 = -2098790400;
	// 82E6ABF4: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E6ABF8: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82E6ABFC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E6AC00: 811F0008  lwz r8, 8(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6AC04: 3889A3B8  addi r4, r9, -0x5c48
	ctx.r[4].s64 = ctx.r[9].s64 + -23624;
	// 82E6AC08: 80DF0020  lwz r6, 0x20(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E6AC0C: 80FF0074  lwz r7, 0x74(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) } as u64;
	// 82E6AC10: 7CC64050  subf r6, r6, r8
	ctx.r[6].s64 = ctx.r[8].s64 - ctx.r[6].s64;
	// 82E6AC14: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 82E6AC18: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82E6AC1C: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82E6AC20: 4BFFFEE9  bl 0x82e6ab08
	ctx.lr = 0x82E6AC24;
	sub_82E6AB08(ctx, base);
	// 82E6AC24: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E6AC28: 80A1005C  lwz r5, 0x5c(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E6AC2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E6AC30: 80810054  lwz r4, 0x54(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E6AC34: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6AC38: 4800A441  bl 0x82e75078
	ctx.lr = 0x82E6AC3C;
	sub_82E75078(ctx, base);
	// 82E6AC3C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E6AC40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E6AC44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E6AC48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E6AC4C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E6AC50: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E6AC54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6AC58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E6AC58 size=76
    let mut pc: u32 = 0x82E6AC58;
    'dispatch: loop {
        match pc {
            0x82E6AC58 => {
    //   block [0x82E6AC58..0x82E6ACA4)
	// 82E6AC58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6AC5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E6AC60: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E6AC64: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E6AC68: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E6AC6C: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82E6AC70: 4800D8F1  bl 0x82e78560
	ctx.lr = 0x82E6AC74;
	sub_82E78560(ctx, base);
	// 82E6AC74: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E6AC78: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E6AC7C: 81230008  lwz r9, 8(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6AC80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6AC84: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E6AC88: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E6AC8C: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82E6AC90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E6AC94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E6AC98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E6AC9C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E6ACA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6ACA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E6ACA8 size=32
    let mut pc: u32 = 0x82E6ACA8;
    'dispatch: loop {
        match pc {
            0x82E6ACA8 => {
    //   block [0x82E6ACA8..0x82E6ACC8)
	// 82E6ACA8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82E6ACAC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82E6ACB0: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 82E6ACB4: 99430004  stb r10, 4(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u8 ) };
	// 82E6ACB8: 91230008  stw r9, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82E6ACBC: C00B9450  lfs f0, -0x6bb0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27568 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E6ACC0: D0030000  stfs f0, 0(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82E6ACC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6ACC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E6ACC8 size=20
    let mut pc: u32 = 0x82E6ACC8;
    'dispatch: loop {
        match pc {
            0x82E6ACC8 => {
    //   block [0x82E6ACC8..0x82E6ACDC)
	// 82E6ACC8: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82E6ACCC: D0230000  stfs f1, 0(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82E6ACD0: 98A30004  stb r5, 4(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[5].u8 ) };
	// 82E6ACD4: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E6ACD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6ACE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E6ACE0 size=64
    let mut pc: u32 = 0x82E6ACE0;
    'dispatch: loop {
        match pc {
            0x82E6ACE0 => {
    //   block [0x82E6ACE0..0x82E6AD20)
	// 82E6ACE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6ACE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E6ACE8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E6ACEC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E6ACF0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E6ACF4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E6ACF8: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82E6ACFC: 48003C2D  bl 0x82e6e928
	ctx.lr = 0x82E6AD00;
	sub_82E6E928(ctx, base);
	// 82E6AD00: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 82E6AD04: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6AD08: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E6AD0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E6AD10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E6AD14: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E6AD18: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E6AD1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6AD20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E6AD20 size=80
    let mut pc: u32 = 0x82E6AD20;
    'dispatch: loop {
        match pc {
            0x82E6AD20 => {
    //   block [0x82E6AD20..0x82E6AD70)
	// 82E6AD20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6AD24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E6AD28: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E6AD2C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E6AD30: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82E6AD34: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E6AD38: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E6AD3C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82E6AD40: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82E6AD44: 48003BE5  bl 0x82e6e928
	ctx.lr = 0x82E6AD48;
	sub_82E6E928(ctx, base);
	// 82E6AD48: D3FF0020  stfs f31, 0x20(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 82E6AD4C: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 82E6AD50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6AD54: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E6AD58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E6AD5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E6AD60: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82E6AD64: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E6AD68: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E6AD6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6AD70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E6AD70 size=1044
    let mut pc: u32 = 0x82E6AD70;
    'dispatch: loop {
        match pc {
            0x82E6AD70 => {
    //   block [0x82E6AD70..0x82E6B184)
	// 82E6AD70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6AD74: 4833D3D9  bl 0x831a814c
	ctx.lr = 0x82E6AD78;
	sub_831A8130(ctx, base);
	// 82E6AD78: DBE1FF98  stfd f31, -0x68(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-104 as u32), ctx.f[31].u64 ) };
	// 82E6AD7C: 9421FEB0  stwu r1, -0x150(r1)
	ea = ctx.r[1].u32.wrapping_add(-336 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E6AD80: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E6AD84: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E6AD88: 7CB72B78  mr r23, r5
	ctx.r[23].u64 = ctx.r[5].u64;
	// 82E6AD8C: 7CD63378  mr r22, r6
	ctx.r[22].u64 = ctx.r[6].u64;
	// 82E6AD90: 7CF53B78  mr r21, r7
	ctx.r[21].u64 = ctx.r[7].u64;
	// 82E6AD94: C19E000C  lfs f12, 0xc(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E6AD98: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E6AD9C: C15E0004  lfs f10, 4(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82E6ADA0: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6ADA4: C01E0000  lfs f0, 0(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E6ADA8: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E6ADAC: C17E0010  lfs f11, 0x10(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82E6ADB0: EC0C0028  fsubs f0, f12, f0
	ctx.f[0].f64 = (((ctx.f[12].f64 - ctx.f[0].f64) as f32) as f64);
	// 82E6ADB4: ED6B5028  fsubs f11, f11, f10
	ctx.f[11].f64 = (((ctx.f[11].f64 - ctx.f[10].f64) as f32) as f64);
	// 82E6ADB8: C13E0014  lfs f9, 0x14(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 82E6ADBC: C15F000C  lfs f10, 0xc(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82E6ADC0: 813E0004  lwz r9, 4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E6ADC4: C1BF0000  lfs f13, 0(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E6ADC8: 811E0008  lwz r8, 8(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6ADCC: C19E0008  lfs f12, 8(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E6ADD0: EDAA6828  fsubs f13, f10, f13
	ctx.f[13].f64 = (((ctx.f[10].f64 - ctx.f[13].f64) as f32) as f64);
	// 82E6ADD4: ED896028  fsubs f12, f9, f12
	ctx.f[12].f64 = (((ctx.f[9].f64 - ctx.f[12].f64) as f32) as f64);
	// 82E6ADD8: C13F0010  lfs f9, 0x10(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 82E6ADDC: 80FF0000  lwz r7, 0(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E6ADE0: C15F0004  lfs f10, 4(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82E6ADE4: 80DF0004  lwz r6, 4(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E6ADE8: ED495028  fsubs f10, f9, f10
	ctx.f[10].f64 = (((ctx.f[9].f64 - ctx.f[10].f64) as f32) as f64);
	// 82E6ADEC: C11F0014  lfs f8, 0x14(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 82E6ADF0: 90A100E4  stw r5, 0xe4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(228 as u32), ctx.r[5].u32 ) };
	// 82E6ADF4: C13F0008  lfs f9, 8(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 82E6ADF8: 914100D0  stw r10, 0xd0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(208 as u32), ctx.r[10].u32 ) };
	// 82E6ADFC: C3EB08A4  lfs f31, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82E6AE00: ED284828  fsubs f9, f8, f9
	ctx.f[9].f64 = (((ctx.f[8].f64 - ctx.f[9].f64) as f32) as f64);
	// 82E6AE04: D00100B0  stfs f0, 0xb0(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), tmp.u32 ) };
	// 82E6AE08: 912100D4  stw r9, 0xd4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(212 as u32), ctx.r[9].u32 ) };
	// 82E6AE0C: D16100B4  stfs f11, 0xb4(r1)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(180 as u32), tmp.u32 ) };
	// 82E6AE10: 910100D8  stw r8, 0xd8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(216 as u32), ctx.r[8].u32 ) };
	// 82E6AE14: D18100B8  stfs f12, 0xb8(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(184 as u32), tmp.u32 ) };
	// 82E6AE18: 90E100DC  stw r7, 0xdc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(220 as u32), ctx.r[7].u32 ) };
	// 82E6AE1C: D1A100BC  stfs f13, 0xbc(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(188 as u32), tmp.u32 ) };
	// 82E6AE20: 90C100E0  stw r6, 0xe0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(224 as u32), ctx.r[6].u32 ) };
	// 82E6AE24: D14100C0  stfs f10, 0xc0(r1)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(192 as u32), tmp.u32 ) };
	// 82E6AE28: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 82E6AE2C: D3E10058  stfs f31, 0x58(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82E6AE30: 388100B0  addi r4, r1, 0xb0
	ctx.r[4].s64 = ctx.r[1].s64 + 176;
	// 82E6AE34: D12100C4  stfs f9, 0xc4(r1)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(196 as u32), tmp.u32 ) };
	// 82E6AE38: 386100D0  addi r3, r1, 0xd0
	ctx.r[3].s64 = ctx.r[1].s64 + 208;
	// 82E6AE3C: D3E1005C  stfs f31, 0x5c(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 82E6AE40: 3BBE000C  addi r29, r30, 0xc
	ctx.r[29].s64 = ctx.r[30].s64 + 12;
	// 82E6AE44: D3E10060  stfs f31, 0x60(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 82E6AE48: 3B9F000C  addi r28, r31, 0xc
	ctx.r[28].s64 = ctx.r[31].s64 + 12;
	// 82E6AE4C: 3B7F0008  addi r27, r31, 8
	ctx.r[27].s64 = ctx.r[31].s64 + 8;
	// 82E6AE50: 48003B21  bl 0x82e6e970
	ctx.lr = 0x82E6AE54;
	sub_82E6E970(ctx, base);
	// 82E6AE54: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E6AE58: 40820024  bne 0x82e6ae7c
	if !ctx.cr[0].eq {
	pc = 0x82E6AE7C; continue 'dispatch;
	}
	// 82E6AE5C: C01E0000  lfs f0, 0(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E6AE60: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82E6AE64: D0170000  stfs f0, 0(r23)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[23].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82E6AE68: C01E0004  lfs f0, 4(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E6AE6C: D0170004  stfs f0, 4(r23)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[23].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82E6AE70: C01E0008  lfs f0, 8(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E6AE74: D0170008  stfs f0, 8(r23)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[23].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82E6AE78: 48000300  b 0x82e6b178
	pc = 0x82E6B178; continue 'dispatch;
	// 82E6AE7C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82E6AE80: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E6AE84: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E6AE88: 3B20FFFF  li r25, -1
	ctx.r[25].s64 = -1;
	// 82E6AE8C: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82E6AE90: 48003C61  bl 0x82e6eaf0
	ctx.lr = 0x82E6AE94;
	sub_82E6EAF0(ctx, base);
	// 82E6AE94: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82E6AE98: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E6AE9C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E6AEA0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E6AEA4: 48003C4D  bl 0x82e6eaf0
	ctx.lr = 0x82E6AEA8;
	sub_82E6EAF0(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6B188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E6B188 size=852
    let mut pc: u32 = 0x82E6B188;
    'dispatch: loop {
        match pc {
            0x82E6B188 => {
    //   block [0x82E6B188..0x82E6B4DC)
	// 82E6B188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6B18C: 4833CFD1  bl 0x831a815c
	ctx.lr = 0x82E6B190;
	sub_831A8130(ctx, base);
	// 82E6B190: 3981FFC0  addi r12, r1, -0x40
	ctx.r[12].s64 = ctx.r[1].s64 + -64;
	// 82E6B194: 4833D8E5  bl 0x831a8a78
	ctx.lr = 0x82E6B198;
	sub_831A8A40(ctx, base);
	// 82E6B198: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E6B19C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E6B1A0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E6B1A4: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82E6B1A8: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82E6B1AC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82E6B1B0: C38B08A4  lfs f28, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[28].f64 = (tmp.f32 as f64);
	// 82E6B1B4: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82E6B1B8: D39F0000  stfs f28, 0(r31)
	tmp.f32 = (ctx.f[28].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82E6B1BC: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 82E6B1C0: D39F0004  stfs f28, 4(r31)
	tmp.f32 = (ctx.f[28].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82E6B1C4: 3BDF0004  addi r30, r31, 4
	ctx.r[30].s64 = ctx.r[31].s64 + 4;
	// 82E6B1C8: D39F0008  stfs f28, 8(r31)
	tmp.f32 = (ctx.f[28].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82E6B1CC: 3BBF0008  addi r29, r31, 8
	ctx.r[29].s64 = ctx.r[31].s64 + 8;
	// 82E6B1D0: 4800D399  bl 0x82e78568
	ctx.lr = 0x82E6B1D4;
	sub_82E78568(ctx, base);
	// 82E6B1D4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E6B1D8: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82E6B1DC: 418202D0  beq 0x82e6b4ac
	if ctx.cr[0].eq {
	pc = 0x82E6B4AC; continue 'dispatch;
	}
	// 82E6B1E0: 4800D391  bl 0x82e78570
	ctx.lr = 0x82E6B1E4;
	sub_82E78570(ctx, base);
	// 82E6B1E4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E6B1E8: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E6B1EC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82E6B1F0: 813C0004  lwz r9, 4(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E6B1F4: 811C0008  lwz r8, 8(r28)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6B1F8: C00B0000  lfs f0, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E6B1FC: C1AB0004  lfs f13, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E6B200: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82E6B204: C18B0008  lfs f12, 8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E6B208: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82E6B20C: D01F0000  stfs f0, 0(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82E6B210: 91010058  stw r8, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[8].u32 ) };
	// 82E6B214: D1BE0000  stfs f13, 0(r30)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82E6B218: D19D0000  stfs f12, 0(r29)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82E6B21C: 48003695  bl 0x82e6e8b0
	ctx.lr = 0x82E6B220;
	sub_82E6E8B0(ctx, base);
	// 82E6B220: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82E6B224: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82E6B228: 409A0014  bne cr6, 0x82e6b23c
	if !ctx.cr[6].eq {
	pc = 0x82E6B23C; continue 'dispatch;
	}
	// 82E6B22C: C3BF0000  lfs f29, 0(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 82E6B230: C3DE0000  lfs f30, 0(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 82E6B234: C3FD0000  lfs f31, 0(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82E6B238: 48000010  b 0x82e6b248
	pc = 0x82E6B248; continue 'dispatch;
	// 82E6B23C: C3E10058  lfs f31, 0x58(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82E6B240: C3C10054  lfs f30, 0x54(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 82E6B244: C3A10050  lfs f29, 0x50(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 82E6B248: C1BA0000  lfs f13, 0(r26)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E6B24C: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82E6B250: C01F0000  lfs f0, 0(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E6B254: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E6B258: EC006828  fsubs f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 82E6B25C: C19E0000  lfs f12, 0(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E6B260: C1BA0004  lfs f13, 4(r26)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E6B264: EDAC6828  fsubs f13, f12, f13
	ctx.f[13].f64 = (((ctx.f[12].f64 - ctx.f[13].f64) as f32) as f64);
	// 82E6B268: C17D0000  lfs f11, 0(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82E6B26C: C19A0008  lfs f12, 8(r26)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E6B270: ED8B6028  fsubs f12, f11, f12
	ctx.f[12].f64 = (((ctx.f[11].f64 - ctx.f[12].f64) as f32) as f64);
	// 82E6B274: D0010060  stfs f0, 0x60(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 82E6B278: D1A10064  stfs f13, 0x64(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 82E6B27C: D1810068  stfs f12, 0x68(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 82E6B280: 4BDA49B9  bl 0x82c0fc38
	ctx.lr = 0x82E6B284;
	sub_82C0FC38(ctx, base);
	// 82E6B284: C01F0000  lfs f0, 0(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E6B288: C1BE0000  lfs f13, 0(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E6B28C: EC1D0028  fsubs f0, f29, f0
	ctx.f[0].f64 = (((ctx.f[29].f64 - ctx.f[0].f64) as f32) as f64);
	// 82E6B290: C19D0000  lfs f12, 0(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E6B294: EDBE6828  fsubs f13, f30, f13
	ctx.f[13].f64 = (((ctx.f[30].f64 - ctx.f[13].f64) as f32) as f64);
	// 82E6B298: ED9F6028  fsubs f12, f31, f12
	ctx.f[12].f64 = (((ctx.f[31].f64 - ctx.f[12].f64) as f32) as f64);
	// 82E6B29C: D001006C  stfs f0, 0x6c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 82E6B2A0: D1A10070  stfs f13, 0x70(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 82E6B2A4: 3881006C  addi r4, r1, 0x6c
	ctx.r[4].s64 = ctx.r[1].s64 + 108;
	// 82E6B2A8: D1810074  stfs f12, 0x74(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 82E6B2AC: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 82E6B2B0: 4BDA4989  bl 0x82c0fc38
	ctx.lr = 0x82E6B2B4;
	sub_82C0FC38(ctx, base);
	// 82E6B2B4: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6B4E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E6B4E0 size=28
    let mut pc: u32 = 0x82E6B4E0;
    'dispatch: loop {
        match pc {
            0x82E6B4E0 => {
    //   block [0x82E6B4E0..0x82E6B4FC)
	// 82E6B4E0: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82E6B4E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E6B4E8: 396B6910  addi r11, r11, 0x6910
	ctx.r[11].s64 = ctx.r[11].s64 + 26896;
	// 82E6B4EC: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82E6B4F0: 91430010  stw r10, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6B500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E6B500 size=16
    let mut pc: u32 = 0x82E6B500;
    'dispatch: loop {
        match pc {
            0x82E6B500 => {
    //   block [0x82E6B500..0x82E6B510)
	// 82E6B500: 13E020C7  vcmpequd (lvx128) v31, v0, v4
	tmp.u32 = ctx.r[4].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82E6B504: 90A30010  stw r5, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[5].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6B510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E6B510 size=148
    let mut pc: u32 = 0x82E6B510;
    'dispatch: loop {
        match pc {
            0x82E6B510 => {
    //   block [0x82E6B510..0x82E6B5A4)
	// 82E6B510: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6B514: 4833CC49  bl 0x831a815c
	ctx.lr = 0x82E6B518;
	sub_831A8130(ctx, base);
	// 82E6B518: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E6B51C: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82E6B520: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82E6B524: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6B528: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82E6B52C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82E6B530: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 82E6B534: 4800D015  bl 0x82e78548
	ctx.lr = 0x82E6B538;
	sub_82E78548(ctx, base);
	// 82E6B538: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82E6B53C: 3B20FFFF  li r25, -1
	ctx.r[25].s64 = -1;
	// 82E6B540: 4182003C  beq 0x82e6b57c
	if ctx.cr[0].eq {
	pc = 0x82E6B57C; continue 'dispatch;
	}
	// 82E6B544: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E6B548: 4BFFA259  bl 0x82e657a0
	ctx.lr = 0x82E6B54C;
	sub_82E657A0(ctx, base);
	// 82E6B54C: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82E6B550: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82E6B554: 409A0028  bne cr6, 0x82e6b57c
	if !ctx.cr[6].eq {
	pc = 0x82E6B57C; continue 'dispatch;
	}
	// 82E6B558: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E6B55C: 4801961D  bl 0x82e84b78
	ctx.lr = 0x82E6B560;
	sub_82E84B78(ctx, base);
	// 82E6B560: C0030000  lfs f0, 0(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E6B564: D01E0000  stfs f0, 0(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82E6B568: C0030004  lfs f0, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E6B56C: D01E0004  stfs f0, 4(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82E6B570: C0030008  lfs f0, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E6B574: D01E0008  stfs f0, 8(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82E6B578: 48000020  b 0x82e6b598
	pc = 0x82E6B598; continue 'dispatch;
	// 82E6B57C: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 82E6B580: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82E6B584: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E6B588: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82E6B58C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82E6B590: 4BFFF7E1  bl 0x82e6ad70
	ctx.lr = 0x82E6B594;
	sub_82E6AD70(ctx, base);
	// 82E6B594: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82E6B598: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82E6B59C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E6B5A0: 4833CC0C  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6B5A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E6B5A8 size=204
    let mut pc: u32 = 0x82E6B5A8;
    'dispatch: loop {
        match pc {
            0x82E6B5A8 => {
    //   block [0x82E6B5A8..0x82E6B674)
	// 82E6B5A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6B5AC: 4833CBC1  bl 0x831a816c
	ctx.lr = 0x82E6B5B0;
	sub_831A8130(ctx, base);
	// 82E6B5B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E6B5B4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E6B5B8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E6B5BC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E6B5C0: 3BC50001  addi r30, r5, 1
	ctx.r[30].s64 = ctx.r[5].s64 + 1;
	// 82E6B5C4: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E6B5C8: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E6B5CC: D01D0000  stfs f0, 0(r29)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82E6B5D0: 7F1E5040  cmplw cr6, r30, r10
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82E6B5D4: D01D0004  stfs f0, 4(r29)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82E6B5D8: D01D0008  stfs f0, 8(r29)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82E6B5DC: 4098008C  bge cr6, 0x82e6b668
	if !ctx.cr[6].lt {
	pc = 0x82E6B668; continue 'dispatch;
	}
	// 82E6B5E0: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E6B5E4: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6B5E8: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82E6B5EC: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E6B5F0: 41990008  bgt cr6, 0x82e6b5f8
	if ctx.cr[6].gt {
	pc = 0x82E6B5F8; continue 'dispatch;
	}
	// 82E6B5F4: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82E6B5F8: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E6B5FC: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E6B600: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E6B604: 4800CF55  bl 0x82e78558
	ctx.lr = 0x82E6B608;
	sub_82E78558(ctx, base);
	// 82E6B608: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82E6B60C: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82E6B610: 419A0018  beq cr6, 0x82e6b628
	if ctx.cr[6].eq {
	pc = 0x82E6B628; continue 'dispatch;
	}
	// 82E6B614: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E6B618: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82E6B61C: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E6B620: 4198FFC0  blt cr6, 0x82e6b5e0
	if ctx.cr[6].lt {
	pc = 0x82E6B5E0; continue 'dispatch;
	}
	// 82E6B624: 48000044  b 0x82e6b668
	pc = 0x82E6B668; continue 'dispatch;
	// 82E6B628: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E6B62C: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6B630: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82E6B634: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E6B638: 41990008  bgt cr6, 0x82e6b640
	if ctx.cr[6].gt {
	pc = 0x82E6B640; continue 'dispatch;
	}
	// 82E6B63C: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82E6B640: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E6B644: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E6B648: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E6B64C: 4800CF15  bl 0x82e78560
	ctx.lr = 0x82E6B650;
	sub_82E78560(ctx, base);
	// 82E6B650: C0030000  lfs f0, 0(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E6B654: C1A30004  lfs f13, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E6B658: C1830008  lfs f12, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E6B65C: D01D0000  stfs f0, 0(r29)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82E6B660: D1BD0004  stfs f13, 4(r29)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82E6B664: D19D0008  stfs f12, 8(r29)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82E6B668: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E6B66C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E6B670: 4833CB4C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6B678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E6B678 size=232
    let mut pc: u32 = 0x82E6B678;
    'dispatch: loop {
        match pc {
            0x82E6B678 => {
    //   block [0x82E6B678..0x82E6B760)
	// 82E6B678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6B67C: 4833CAED  bl 0x831a8168
	ctx.lr = 0x82E6B680;
	sub_831A8130(ctx, base);
	// 82E6B680: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E6B684: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E6B688: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E6B68C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82E6B690: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82E6B694: 83BF0010  lwz r29, 0x10(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E6B698: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E6B69C: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82E6B6A0: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82E6B6A4: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82E6B6A8: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82E6B6AC: 419A00AC  beq cr6, 0x82e6b758
	if ctx.cr[6].eq {
	pc = 0x82E6B758; continue 'dispatch;
	}
	// 82E6B6B0: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82E6B6B4: 409A0010  bne cr6, 0x82e6b6c4
	if !ctx.cr[6].eq {
	pc = 0x82E6B6C4; continue 'dispatch;
	}
	// 82E6B6B8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82E6B6BC: 4819DE1D  bl 0x830094d8
	ctx.lr = 0x82E6B6C0;
	sub_830094D8(ctx, base);
	// 82E6B6C0: 48000048  b 0x82e6b708
	pc = 0x82E6B708; continue 'dispatch;
	// 82E6B6C4: 397DFFFF  addi r11, r29, -1
	ctx.r[11].s64 = ctx.r[29].s64 + -1;
	// 82E6B6C8: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E6B6CC: 409A0010  bne cr6, 0x82e6b6dc
	if !ctx.cr[6].eq {
	pc = 0x82E6B6DC; continue 'dispatch;
	}
	// 82E6B6D0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82E6B6D4: 4819D755  bl 0x83008e28
	ctx.lr = 0x82E6B6D8;
	sub_83008E28(ctx, base);
	// 82E6B6D8: 48000030  b 0x82e6b708
	pc = 0x82E6B708; continue 'dispatch;
	// 82E6B6DC: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E6B6E0: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6B6E4: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82E6B6E8: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E6B6EC: 41990008  bgt cr6, 0x82e6b6f4
	if ctx.cr[6].gt {
	pc = 0x82E6B6F4; continue 'dispatch;
	}
	// 82E6B6F0: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82E6B6F4: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E6B6F8: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E6B6FC: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E6B700: 4800CE49  bl 0x82e78548
	ctx.lr = 0x82E6B704;
	sub_82E78548(ctx, base);
	// 82E6B704: 48019475  bl 0x82e84b78
	ctx.lr = 0x82E6B708;
	sub_82E84B78(ctx, base);
	// 82E6B708: C0030000  lfs f0, 0(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E6B70C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E6B710: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82E6B714: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6B718: C0030004  lfs f0, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E6B71C: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82E6B720: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82E6B724: C0030008  lfs f0, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E6B728: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E6B72C: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82E6B730: 41990008  bgt cr6, 0x82e6b738
	if ctx.cr[6].gt {
	pc = 0x82E6B738; continue 'dispatch;
	}
	// 82E6B734: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82E6B738: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E6B73C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E6B740: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E6B744: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E6B748: 4800CF81  bl 0x82e786c8
	ctx.lr = 0x82E6B74C;
	sub_82E786C8(ctx, base);
	// 82E6B74C: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82E6B750: 7F1EE840  cmplw cr6, r30, r29
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82E6B754: 4198FF5C  blt cr6, 0x82e6b6b0
	if ctx.cr[6].lt {
	pc = 0x82E6B6B0; continue 'dispatch;
	}
	// 82E6B758: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E6B75C: 4833CA5C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6B760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E6B760 size=212
    let mut pc: u32 = 0x82E6B760;
    'dispatch: loop {
        match pc {
            0x82E6B760 => {
    //   block [0x82E6B760..0x82E6B834)
	// 82E6B760: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6B764: 4833CA01  bl 0x831a8164
	ctx.lr = 0x82E6B768;
	sub_831A8130(ctx, base);
	// 82E6B768: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E6B76C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E6B770: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E6B774: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82E6B778: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82E6B77C: 83BF0010  lwz r29, 0x10(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E6B780: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E6B784: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82E6B788: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82E6B78C: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82E6B790: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82E6B794: 419A0098  beq cr6, 0x82e6b82c
	if ctx.cr[6].eq {
	pc = 0x82E6B82C; continue 'dispatch;
	}
	// 82E6B798: 3B9DFFFF  addi r28, r29, -1
	ctx.r[28].s64 = ctx.r[29].s64 + -1;
	// 82E6B79C: 7F1EE040  cmplw cr6, r30, r28
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82E6B7A0: 409A0010  bne cr6, 0x82e6b7b0
	if !ctx.cr[6].eq {
	pc = 0x82E6B7B0; continue 'dispatch;
	}
	// 82E6B7A4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82E6B7A8: 4819D681  bl 0x83008e28
	ctx.lr = 0x82E6B7AC;
	sub_83008E28(ctx, base);
	// 82E6B7AC: 48000030  b 0x82e6b7dc
	pc = 0x82E6B7DC; continue 'dispatch;
	// 82E6B7B0: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E6B7B4: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6B7B8: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82E6B7BC: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E6B7C0: 41990008  bgt cr6, 0x82e6b7c8
	if ctx.cr[6].gt {
	pc = 0x82E6B7C8; continue 'dispatch;
	}
	// 82E6B7C4: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82E6B7C8: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E6B7CC: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E6B7D0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E6B7D4: 7C8B502E  lwzx r4, r11, r10
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E6B7D8: 4800CE39  bl 0x82e78610
	ctx.lr = 0x82E6B7DC;
	sub_82E78610(ctx, base);
	// 82E6B7DC: C0030000  lfs f0, 0(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E6B7E0: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E6B7E4: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82E6B7E8: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6B7EC: C0030004  lfs f0, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E6B7F0: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82E6B7F4: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82E6B7F8: C0030008  lfs f0, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E6B7FC: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E6B800: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82E6B804: 41990008  bgt cr6, 0x82e6b80c
	if ctx.cr[6].gt {
	pc = 0x82E6B80C; continue 'dispatch;
	}
	// 82E6B808: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82E6B80C: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E6B810: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E6B814: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E6B818: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E6B81C: 4800CEAD  bl 0x82e786c8
	ctx.lr = 0x82E6B820;
	sub_82E786C8(ctx, base);
	// 82E6B820: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82E6B824: 7F1EE840  cmplw cr6, r30, r29
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82E6B828: 4198FF74  blt cr6, 0x82e6b79c
	if ctx.cr[6].lt {
	pc = 0x82E6B79C; continue 'dispatch;
	}
	// 82E6B82C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82E6B830: 4833C984  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6B838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E6B838 size=484
    let mut pc: u32 = 0x82E6B838;
    'dispatch: loop {
        match pc {
            0x82E6B838 => {
    //   block [0x82E6B838..0x82E6BA1C)
	// 82E6B838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6B83C: 4833C921  bl 0x831a815c
	ctx.lr = 0x82E6B840;
	sub_831A8130(ctx, base);
	// 82E6B840: DBE1FFB8  stfd f31, -0x48(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-72 as u32), ctx.f[31].u64 ) };
	// 82E6B844: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E6B848: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E6B84C: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82E6B850: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82E6B854: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82E6B858: 837F0010  lwz r27, 0x10(r31)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E6B85C: 4819D5CD  bl 0x83008e28
	ctx.lr = 0x82E6B860;
	sub_83008E28(ctx, base);
	// 82E6B860: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E6B864: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82E6B868: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E6B86C: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82E6B870: 81230008  lwz r9, 8(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6B874: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82E6B878: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82E6B87C: 91210058  stw r9, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[9].u32 ) };
	// 82E6B880: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E6B884: 81230004  lwz r9, 4(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E6B888: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6B88C: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82E6B890: 91210064  stw r9, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[9].u32 ) };
	// 82E6B894: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 82E6B898: 419A0178  beq cr6, 0x82e6ba10
	if ctx.cr[6].eq {
	pc = 0x82E6BA10; continue 'dispatch;
	}
	// 82E6B89C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E6B8A0: C1810068  lfs f12, 0x68(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E6B8A4: C1A10064  lfs f13, 0x64(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E6B8A8: 3B3BFFFF  addi r25, r27, -1
	ctx.r[25].s64 = ctx.r[27].s64 + -1;
	// 82E6B8AC: C0010060  lfs f0, 0x60(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E6B8B0: C3EB08A4  lfs f31, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82E6B8B4: 7F1EC840  cmplw cr6, r30, r25
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[25].u32, &mut ctx.xer);
	// 82E6B8B8: 409A0028  bne cr6, 0x82e6b8e0
	if !ctx.cr[6].eq {
	pc = 0x82E6B8E0; continue 'dispatch;
	}
	// 82E6B8BC: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82E6B8C0: 4819D569  bl 0x83008e28
	ctx.lr = 0x82E6B8C4;
	sub_83008E28(ctx, base);
	// 82E6B8C4: C0030000  lfs f0, 0(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E6B8C8: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82E6B8CC: C0030004  lfs f0, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E6B8D0: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82E6B8D4: C0030008  lfs f0, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E6B8D8: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82E6B8DC: 480000F0  b 0x82e6b9cc
	pc = 0x82E6B9CC; continue 'dispatch;
	// 82E6B8E0: 7D7EE214  add r11, r30, r28
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[28].u64;
	// 82E6B8E4: D3E1007C  stfs f31, 0x7c(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), tmp.u32 ) };
	// 82E6B8E8: D3E10080  stfs f31, 0x80(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), tmp.u32 ) };
	// 82E6B8EC: D3E10084  stfs f31, 0x84(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), tmp.u32 ) };
	// 82E6B8F0: 7F0BC840  cmplw cr6, r11, r25
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[25].u32, &mut ctx.xer);
	// 82E6B8F4: D0010070  stfs f0, 0x70(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 82E6B8F8: D1A10074  stfs f13, 0x74(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 82E6B8FC: D1810078  stfs f12, 0x78(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 82E6B900: 41980010  blt cr6, 0x82e6b910
	if ctx.cr[6].lt {
	pc = 0x82E6B910; continue 'dispatch;
	}
	// 82E6B904: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82E6B908: 4819DBD1  bl 0x830094d8
	ctx.lr = 0x82E6B90C;
	sub_830094D8(ctx, base);
	// 82E6B90C: 48000034  b 0x82e6b940
	pc = 0x82E6B940; continue 'dispatch;
	// 82E6B910: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E6B914: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6B918: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82E6B91C: 7D6BE214  add r11, r11, r28
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 82E6B920: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E6B924: 41990008  bgt cr6, 0x82e6b92c
	if ctx.cr[6].gt {
	pc = 0x82E6B92C; continue 'dispatch;
	}
	// 82E6B928: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82E6B92C: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E6B930: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E6B934: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E6B938: 7C8B502E  lwzx r4, r11, r10
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E6B93C: 4800CCD5  bl 0x82e78610
	ctx.lr = 0x82E6B940;
	sub_82E78610(ctx, base);
	// 82E6B940: C0030000  lfs f0, 0(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E6B944: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E6B948: D001007C  stfs f0, 0x7c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), tmp.u32 ) };
	// 82E6B94C: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6B950: C0030004  lfs f0, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E6B954: 7D0BF214  add r8, r11, r30
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82E6B958: D0010080  stfs f0, 0x80(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), tmp.u32 ) };
	// 82E6B95C: C0030008  lfs f0, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E6B960: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82E6B964: D0010084  stfs f0, 0x84(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), tmp.u32 ) };
	// 82E6B968: 41990008  bgt cr6, 0x82e6b970
	if ctx.cr[6].gt {
	pc = 0x82E6B970; continue 'dispatch;
	}
	// 82E6B96C: 7D094050  subf r8, r9, r8
	ctx.r[8].s64 = ctx.r[8].s64 - ctx.r[9].s64;
	// 82E6B970: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E6B974: 5508103A  slwi r8, r8, 2
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82E6B978: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82E6B97C: 7F095840  cmplw cr6, r9, r11
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E6B980: 7CC8502E  lwzx r6, r8, r10
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E6B984: 41990008  bgt cr6, 0x82e6b98c
	if ctx.cr[6].gt {
	pc = 0x82E6B98C; continue 'dispatch;
	}
	// 82E6B988: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 82E6B98C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E6B990: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 82E6B994: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E6B998: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82E6B99C: 7C8B502E  lwzx r4, r11, r10
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E6B9A0: 4BFFFB71  bl 0x82e6b510
	ctx.lr = 0x82E6B9A4;
	sub_82E6B510(ctx, base);
	// 82E6B9A4: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E6B9A8: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6B9AC: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82E6B9B0: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E6B9B4: 41990008  bgt cr6, 0x82e6b9bc
	if ctx.cr[6].gt {
	pc = 0x82E6B9BC; continue 'dispatch;
	}
	// 82E6B9B8: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82E6B9BC: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E6B9C0: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E6B9C4: 7D6B502E  lwzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E6B9C8: 906B0018  stw r3, 0x18(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[3].u32 ) };
	// 82E6B9CC: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E6B9D0: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6B9D4: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82E6B9D8: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E6B9DC: 41990008  bgt cr6, 0x82e6b9e4
	if ctx.cr[6].gt {
	pc = 0x82E6B9E4; continue 'dispatch;
	}
	// 82E6B9E0: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82E6B9E4: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E6B9E8: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E6B9EC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E6B9F0: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E6B9F4: 4800CCD5  bl 0x82e786c8
	ctx.lr = 0x82E6B9F8;
	sub_82E786C8(ctx, base);
	// 82E6B9F8: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82E6B9FC: C0010050  lfs f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E6BA00: C1A10054  lfs f13, 0x54(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E6BA04: C1810058  lfs f12, 0x58(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E6BA08: 7F1ED840  cmplw cr6, r30, r27
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82E6BA0C: 4198FEA8  blt cr6, 0x82e6b8b4
	if ctx.cr[6].lt {
	pc = 0x82E6B8B4; continue 'dispatch;
	}
	// 82E6BA10: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82E6BA14: CBE1FFB8  lfd f31, -0x48(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-72 as u32) ) };
	// 82E6BA18: 4833C794  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6BA20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E6BA20 size=628
    let mut pc: u32 = 0x82E6BA20;
    'dispatch: loop {
        match pc {
            0x82E6BA20 => {
    //   block [0x82E6BA20..0x82E6BC94)
	// 82E6BA20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6BA24: 4833C725  bl 0x831a8148
	ctx.lr = 0x82E6BA28;
	sub_831A8130(ctx, base);
	// 82E6BA28: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E6BA2C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E6BA30: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82E6BA34: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82E6BA38: 7C942378  mr r20, r4
	ctx.r[20].u64 = ctx.r[4].u64;
	// 82E6BA3C: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82E6BA40: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E6BA44: 7CF63B78  mr r22, r7
	ctx.r[22].u64 = ctx.r[7].u64;
	// 82E6BA48: D0010070  stfs f0, 0x70(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 82E6BA4C: 7D154378  mr r21, r8
	ctx.r[21].u64 = ctx.r[8].u64;
	// 82E6BA50: D0010074  stfs f0, 0x74(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 82E6BA54: 2F1A0000  cmpwi cr6, r26, 0
	ctx.cr[6].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 82E6BA58: D0010078  stfs f0, 0x78(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 82E6BA5C: D001007C  stfs f0, 0x7c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), tmp.u32 ) };
	// 82E6BA60: D0010080  stfs f0, 0x80(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), tmp.u32 ) };
	// 82E6BA64: D0010084  stfs f0, 0x84(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), tmp.u32 ) };
	// 82E6BA68: 409A0010  bne cr6, 0x82e6ba78
	if !ctx.cr[6].eq {
	pc = 0x82E6BA78; continue 'dispatch;
	}
	// 82E6BA6C: 7E83A378  mr r3, r20
	ctx.r[3].u64 = ctx.r[20].u64;
	// 82E6BA70: 4819DA69  bl 0x830094d8
	ctx.lr = 0x82E6BA74;
	sub_830094D8(ctx, base);
	// 82E6BA74: 48000030  b 0x82e6baa4
	pc = 0x82E6BAA4; continue 'dispatch;
	// 82E6BA78: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E6BA7C: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6BA80: 7D6BD214  add r11, r11, r26
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[26].u64;
	// 82E6BA84: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82E6BA88: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E6BA8C: 41990008  bgt cr6, 0x82e6ba94
	if ctx.cr[6].gt {
	pc = 0x82E6BA94; continue 'dispatch;
	}
	// 82E6BA90: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82E6BA94: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E6BA98: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E6BA9C: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E6BAA0: 4800CAC1  bl 0x82e78560
	ctx.lr = 0x82E6BAA4;
	sub_82E78560(ctx, base);
	// 82E6BAA4: C0030000  lfs f0, 0(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E6BAA8: D0010070  stfs f0, 0x70(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 82E6BAAC: C0030004  lfs f0, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E6BAB0: D0010074  stfs f0, 0x74(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 82E6BAB4: C0030008  lfs f0, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E6BAB8: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E6BABC: D0010078  stfs f0, 0x78(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 82E6BAC0: 48002E51  bl 0x82e6e910
	ctx.lr = 0x82E6BAC4;
	sub_82E6E910(ctx, base);
	// 82E6BAC4: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82E6BAC8: 7F5CD378  mr r28, r26
	ctx.r[28].u64 = ctx.r[26].u64;
	// 82E6BACC: 3AF8FFFF  addi r23, r24, -1
	ctx.r[23].s64 = ctx.r[24].s64 + -1;
	// 82E6BAD0: 7F1AC000  cmpw cr6, r26, r24
	ctx.cr[6].compare_i32(ctx.r[26].s32, ctx.r[24].s32, &mut ctx.xer);
	// 82E6BAD4: 92F60000  stw r23, 0(r22)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[22].u32.wrapping_add(0 as u32), ctx.r[23].u32 ) };
	// 82E6BAD8: 409801B0  bge cr6, 0x82e6bc88
	if !ctx.cr[6].lt {
	pc = 0x82E6BC88; continue 'dispatch;
	}
	// 82E6BADC: 7F1CB800  cmpw cr6, r28, r23
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[23].s32, &mut ctx.xer);
	// 82E6BAE0: 409A0010  bne cr6, 0x82e6baf0
	if !ctx.cr[6].eq {
	pc = 0x82E6BAF0; continue 'dispatch;
	}
	// 82E6BAE4: 7E83A378  mr r3, r20
	ctx.r[3].u64 = ctx.r[20].u64;
	// 82E6BAE8: 4819D341  bl 0x83008e28
	ctx.lr = 0x82E6BAEC;
	sub_83008E28(ctx, base);
	// 82E6BAEC: 48000030  b 0x82e6bb1c
	pc = 0x82E6BB1C; continue 'dispatch;
	// 82E6BAF0: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E6BAF4: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6BAF8: 7D6BE214  add r11, r11, r28
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 82E6BAFC: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E6BB00: 41990008  bgt cr6, 0x82e6bb08
	if ctx.cr[6].gt {
	pc = 0x82E6BB08; continue 'dispatch;
	}
	// 82E6BB04: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82E6BB08: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E6BB0C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E6BB10: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E6BB14: 7C8B502E  lwzx r4, r11, r10
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E6BB18: 4800CAF9  bl 0x82e78610
	ctx.lr = 0x82E6BB1C;
	sub_82E78610(ctx, base);
	// 82E6BB1C: C1830008  lfs f12, 8(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E6BB20: 7F1CD000  cmpw cr6, r28, r26
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[26].s32, &mut ctx.xer);
	// 82E6BB24: C1A30004  lfs f13, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E6BB28: C0030000  lfs f0, 0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E6BB2C: D1990008  stfs f12, 8(r25)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82E6BB30: D1B90004  stfs f13, 4(r25)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82E6BB34: D0190000  stfs f0, 0(r25)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82E6BB38: D001007C  stfs f0, 0x7c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), tmp.u32 ) };
	// 82E6BB3C: D1A10080  stfs f13, 0x80(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), tmp.u32 ) };
	// 82E6BB40: D1810084  stfs f12, 0x84(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), tmp.u32 ) };
	// 82E6BB44: 419A0138  beq cr6, 0x82e6bc7c
	if ctx.cr[6].eq {
	pc = 0x82E6BC7C; continue 'dispatch;
	}
	// 82E6BB48: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E6BB4C: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6BB50: 7D6BE214  add r11, r11, r28
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 82E6BB54: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E6BB58: 41990008  bgt cr6, 0x82e6bb60
	if ctx.cr[6].gt {
	pc = 0x82E6BB60; continue 'dispatch;
	}
	// 82E6BB5C: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82E6BB60: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E6BB64: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E6BB68: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E6BB6C: 4800C9E5  bl 0x82e78550
	ctx.lr = 0x82E6BB70;
	sub_82E78550(ctx, base);
	// 82E6BB70: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E6BB74: 41820108  beq 0x82e6bc7c
	if ctx.cr[0].eq {
	pc = 0x82E6BC7C; continue 'dispatch;
	}
	// 82E6BB78: 3BDCFFFF  addi r30, r28, -1
	ctx.r[30].s64 = ctx.r[28].s64 + -1;
	// 82E6BB7C: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82E6BB80: 7F1ED000  cmpw cr6, r30, r26
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[26].s32, &mut ctx.xer);
	// 82E6BB84: 419800F8  blt cr6, 0x82e6bc7c
	if ctx.cr[6].lt {
	pc = 0x82E6BC7C; continue 'dispatch;
	}
	// 82E6BB88: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E6BB8C: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6BB90: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82E6BB94: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E6BB98: 41990008  bgt cr6, 0x82e6bba0
	if ctx.cr[6].gt {
	pc = 0x82E6BBA0; continue 'dispatch;
	}
	// 82E6BB9C: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82E6BBA0: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E6BBA4: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E6BBA8: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E6BBAC: 4800C9A5  bl 0x82e78550
	ctx.lr = 0x82E6BBB0;
	sub_82E78550(ctx, base);
	// 82E6BBB0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E6BBB4: 418200B4  beq 0x82e6bc68
	if ctx.cr[0].eq {
	pc = 0x82E6BC68; continue 'dispatch;
	}
	// 82E6BBB8: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E6BBBC: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6BBC0: 7D0BF214  add r8, r11, r30
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82E6BBC4: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82E6BBC8: 41990008  bgt cr6, 0x82e6bbd0
	if ctx.cr[6].gt {
	pc = 0x82E6BBD0; continue 'dispatch;
	}
	// 82E6BBCC: 7D094050  subf r8, r9, r8
	ctx.r[8].s64 = ctx.r[8].s64 - ctx.r[9].s64;
	// 82E6BBD0: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E6BBD4: 5508103A  slwi r8, r8, 2
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82E6BBD8: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82E6BBDC: 7F095840  cmplw cr6, r9, r11
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E6BBE0: 7CC8502E  lwzx r6, r8, r10
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E6BBE4: 41990008  bgt cr6, 0x82e6bbec
	if ctx.cr[6].gt {
	pc = 0x82E6BBEC; continue 'dispatch;
	}
	// 82E6BBE8: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 82E6BBEC: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E6BBF0: 7EA7AB78  mr r7, r21
	ctx.r[7].u64 = ctx.r[21].u64;
	// 82E6BBF4: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E6BBF8: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82E6BBFC: 7C8B502E  lwzx r4, r11, r10
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E6BC00: 4BFFF911  bl 0x82e6b510
	ctx.lr = 0x82E6BC04;
	sub_82E6B510(ctx, base);
	// 82E6BC04: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E6BC08: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E6BC0C: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6BC10: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82E6BC14: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E6BC18: 41990008  bgt cr6, 0x82e6bc20
	if ctx.cr[6].gt {
	pc = 0x82E6BC20; continue 'dispatch;
	}
	// 82E6BC1C: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82E6BC20: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E6BC24: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E6BC28: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E6BC2C: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E6BC30: 4800CA99  bl 0x82e786c8
	ctx.lr = 0x82E6BC34;
	sub_82E786C8(ctx, base);
	// 82E6BC34: 2F1DFFFF  cmpwi cr6, r29, -1
	ctx.cr[6].compare_i32(ctx.r[29].s32, -1, &mut ctx.xer);
	// 82E6BC38: 419A0030  beq cr6, 0x82e6bc68
	if ctx.cr[6].eq {
	pc = 0x82E6BC68; continue 'dispatch;
	}
	// 82E6BC3C: C0010050  lfs f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E6BC40: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 82E6BC44: C1A10054  lfs f13, 0x54(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E6BC48: 93D60000  stw r30, 0(r22)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[22].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82E6BC4C: C1810058  lfs f12, 0x58(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E6BC50: D0190000  stfs f0, 0(r25)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82E6BC54: D1B90004  stfs f13, 4(r25)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82E6BC58: D1990008  stfs f12, 8(r25)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82E6BC5C: D001007C  stfs f0, 0x7c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), tmp.u32 ) };
	// 82E6BC60: D1A10080  stfs f13, 0x80(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), tmp.u32 ) };
	// 82E6BC64: D1810084  stfs f12, 0x84(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), tmp.u32 ) };
	// 82E6BC68: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 82E6BC6C: 7F1ED000  cmpw cr6, r30, r26
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[26].s32, &mut ctx.xer);
	// 82E6BC70: 4098FF18  bge cr6, 0x82e6bb88
	if !ctx.cr[6].lt {
	pc = 0x82E6BB88; continue 'dispatch;
	}
	// 82E6BC74: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 82E6BC78: 41990010  bgt cr6, 0x82e6bc88
	if ctx.cr[6].gt {
	pc = 0x82E6BC88; continue 'dispatch;
	}
	// 82E6BC7C: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82E6BC80: 7F1CC000  cmpw cr6, r28, r24
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[24].s32, &mut ctx.xer);
	// 82E6BC84: 4198FE58  blt cr6, 0x82e6badc
	if ctx.cr[6].lt {
	pc = 0x82E6BADC; continue 'dispatch;
	}
	// 82E6BC88: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82E6BC8C: 382100F0  addi r1, r1, 0xf0
	ctx.r[1].s64 = ctx.r[1].s64 + 240;
	// 82E6BC90: 4833C508  b 0x831a8198
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6BC98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E6BC98 size=144
    let mut pc: u32 = 0x82E6BC98;
    'dispatch: loop {
        match pc {
            0x82E6BC98 => {
    //   block [0x82E6BC98..0x82E6BD28)
	// 82E6BC98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6BC9C: 4833C4C9  bl 0x831a8164
	ctx.lr = 0x82E6BCA0;
	sub_831A8130(ctx, base);
	// 82E6BCA0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E6BCA4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E6BCA8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E6BCAC: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82E6BCB0: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E6BCB4: 48002C5D  bl 0x82e6e910
	ctx.lr = 0x82E6BCB8;
	sub_82E6E910(ctx, base);
	// 82E6BCB8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E6BCBC: 7C7B1B79  or. r27, r3, r3
	ctx.r[27].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 82E6BCC0: 90C10050  stw r6, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[6].u32 ) };
	// 82E6BCC4: 4081005C  ble 0x82e6bd20
	if !ctx.cr[0].gt {
	pc = 0x82E6BD20; continue 'dispatch;
	}
	// 82E6BCC8: 7F88E378  mr r8, r28
	ctx.r[8].u64 = ctx.r[28].u64;
	// 82E6BCCC: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82E6BCD0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82E6BCD4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E6BCD8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E6BCDC: 4BFFFD45  bl 0x82e6ba20
	ctx.lr = 0x82E6BCE0;
	sub_82E6BA20(ctx, base);
	// 82E6BCE0: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E6BCE4: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6BCE8: 83C10050  lwz r30, 0x50(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E6BCEC: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82E6BCF0: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E6BCF4: 41990008  bgt cr6, 0x82e6bcfc
	if ctx.cr[6].gt {
	pc = 0x82E6BCFC; continue 'dispatch;
	}
	// 82E6BCF8: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82E6BCFC: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E6BD00: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E6BD04: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82E6BD08: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E6BD0C: 4800C9BD  bl 0x82e786c8
	ctx.lr = 0x82E6BD10;
	sub_82E786C8(ctx, base);
	// 82E6BD10: 38DE0001  addi r6, r30, 1
	ctx.r[6].s64 = ctx.r[30].s64 + 1;
	// 82E6BD14: 90C10050  stw r6, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[6].u32 ) };
	// 82E6BD18: 7F06D800  cmpw cr6, r6, r27
	ctx.cr[6].compare_i32(ctx.r[6].s32, ctx.r[27].s32, &mut ctx.xer);
	// 82E6BD1C: 4198FFAC  blt cr6, 0x82e6bcc8
	if ctx.cr[6].lt {
	pc = 0x82E6BCC8; continue 'dispatch;
	}
	// 82E6BD20: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82E6BD24: 4833C490  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6BD28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E6BD28 size=88
    let mut pc: u32 = 0x82E6BD28;
    'dispatch: loop {
        match pc {
            0x82E6BD28 => {
    //   block [0x82E6BD28..0x82E6BD80)
	// 82E6BD28: 7CCA3378  mr r10, r6
	ctx.r[10].u64 = ctx.r[6].u64;
	// 82E6BD2C: 7F055040  cmplw cr6, r5, r10
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82E6BD30: 419A0048  beq cr6, 0x82e6bd78
	if ctx.cr[6].eq {
	pc = 0x82E6BD78; continue 'dispatch;
	}
	// 82E6BD34: 81240008  lwz r9, 8(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6BD38: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82E6BD3C: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82E6BD40: 419A0034  beq cr6, 0x82e6bd74
	if ctx.cr[6].eq {
	pc = 0x82E6BD74; continue 'dispatch;
	}
	// 82E6BD44: 7D055050  subf r8, r5, r10
	ctx.r[8].s64 = ctx.r[10].s64 - ctx.r[5].s64;
	// 82E6BD48: 7CE85A14  add r7, r8, r11
	ctx.r[7].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 82E6BD4C: 13E050C7  vcmpequd (lvx128) v31, v0, v10
	tmp.u32 = ctx.r[10].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6BD80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E6BD80 size=24
    let mut pc: u32 = 0x82E6BD80;
    'dispatch: loop {
        match pc {
            0x82E6BD80 => {
    //   block [0x82E6BD80..0x82E6BD98)
	// 82E6BD80: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E6BD84: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82E6BD88: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 82E6BD8C: 409A000C  bne cr6, 0x82e6bd98
	if !ctx.cr[6].eq {
		sub_82E6BD98(ctx, base);
		return;
	}
	// 82E6BD90: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82E6BD94: 4BFFF8E4  b 0x82e6b678
	sub_82E6B678(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6BD98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E6BD98 size=16
    let mut pc: u32 = 0x82E6BD98;
    'dispatch: loop {
        match pc {
            0x82E6BD98 => {
    //   block [0x82E6BD98..0x82E6BDA8)
	// 82E6BD98: 2F060001  cmpwi cr6, r6, 1
	ctx.cr[6].compare_i32(ctx.r[6].s32, 1, &mut ctx.xer);
	// 82E6BD9C: 409A000C  bne cr6, 0x82e6bda8
	if !ctx.cr[6].eq {
		sub_82E6BDA8(ctx, base);
		return;
	}
	// 82E6BDA0: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82E6BDA4: 4BFFF9BC  b 0x82e6b760
	sub_82E6B760(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6BDA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E6BDA8 size=20
    let mut pc: u32 = 0x82E6BDA8;
    'dispatch: loop {
        match pc {
            0x82E6BDA8 => {
    //   block [0x82E6BDA8..0x82E6BDBC)
	// 82E6BDA8: 2F060002  cmpwi cr6, r6, 2
	ctx.cr[6].compare_i32(ctx.r[6].s32, 2, &mut ctx.xer);
	// 82E6BDAC: 409A0010  bne cr6, 0x82e6bdbc
	if !ctx.cr[6].eq {
		sub_82E6BDBC(ctx, base);
		return;
	}
	// 82E6BDB0: 80CB0034  lwz r6, 0x34(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82E6BDB4: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82E6BDB8: 4BFFFA80  b 0x82e6b838
	sub_82E6B838(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6BDBC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E6BDBC size=8
    let mut pc: u32 = 0x82E6BDBC;
    'dispatch: loop {
        match pc {
            0x82E6BDBC => {
    //   block [0x82E6BDBC..0x82E6BDC4)
	// 82E6BDBC: 2F060003  cmpwi cr6, r6, 3
	ctx.cr[6].compare_i32(ctx.r[6].s32, 3, &mut ctx.xer);
	// 82E6BDC0: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6BDC4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E6BDC4 size=8
    let mut pc: u32 = 0x82E6BDC4;
    'dispatch: loop {
        match pc {
            0x82E6BDC4 => {
    //   block [0x82E6BDC4..0x82E6BDCC)
	// 82E6BDC4: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82E6BDC8: 4BFFFED0  b 0x82e6bc98
	sub_82E6BC98(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6BDCC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E6BDCC size=4
    let mut pc: u32 = 0x82E6BDCC;
    'dispatch: loop {
        match pc {
            0x82E6BDCC => {
    //   block [0x82E6BDCC..0x82E6BDD0)
	// 82E6BDCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6BDD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E6BDD0 size=16
    let mut pc: u32 = 0x82E6BDD0;
    'dispatch: loop {
        match pc {
            0x82E6BDD0 => {
    //   block [0x82E6BDD0..0x82E6BDE0)
	// 82E6BDD0: 81430018  lwz r10, 0x18(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E6BDD4: 39630014  addi r11, r3, 0x14
	ctx.r[11].s64 = ctx.r[3].s64 + 20;
	// 82E6BDD8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E6BDDC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6BDE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E6BDE0 size=20
    let mut pc: u32 = 0x82E6BDE0;
    'dispatch: loop {
        match pc {
            0x82E6BDE0 => {
    //   block [0x82E6BDE0..0x82E6BDF4)
	// 82E6BDE0: 812B0008  lwz r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6BDE4: 39000030  li r8, 0x30
	ctx.r[8].s64 = 48;
	// 82E6BDE8: 7D4A4850  subf r10, r10, r9
	ctx.r[10].s64 = ctx.r[9].s64 - ctx.r[10].s64;
	// 82E6BDEC: 7D4A43D7  divw. r10, r10, r8
	ctx.r[10].s32 = ctx.r[10].s32 / ctx.r[8].s32;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E6BDF0: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6BDF4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E6BDF4 size=12
    let mut pc: u32 = 0x82E6BDF4;
    'dispatch: loop {
        match pc {
            0x82E6BDF4 => {
    //   block [0x82E6BDF4..0x82E6BE00)
	// 82E6BDF4: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E6BDF8: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82E6BDFC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6BE00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E6BE00 size=16
    let mut pc: u32 = 0x82E6BE00;
    'dispatch: loop {
        match pc {
            0x82E6BE00 => {
    //   block [0x82E6BE00..0x82E6BE10)
	// 82E6BE00: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6BE04: 7D295050  subf r9, r9, r10
	ctx.r[9].s64 = ctx.r[10].s64 - ctx.r[9].s64;
	// 82E6BE08: 7D2943D7  divw. r9, r9, r8
	ctx.r[9].s32 = ctx.r[9].s32 / ctx.r[8].s32;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E6BE0C: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6BE10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E6BE10 size=12
    let mut pc: u32 = 0x82E6BE10;
    'dispatch: loop {
        match pc {
            0x82E6BE10 => {
    //   block [0x82E6BE10..0x82E6BE1C)
	// 82E6BE10: 394AFFD0  addi r10, r10, -0x30
	ctx.r[10].s64 = ctx.r[10].s64 + -48;
	// 82E6BE14: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E6BE18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6BE20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E6BE20 size=72
    let mut pc: u32 = 0x82E6BE20;
    'dispatch: loop {
        match pc {
            0x82E6BE20 => {
    //   block [0x82E6BE20..0x82E6BE68)
	// 82E6BE20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6BE24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E6BE28: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E6BE2C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E6BE30: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E6BE34: 4B7C34A5  bl 0x8262f2d8
	ctx.lr = 0x82E6BE38;
	sub_8262F2D8(ctx, base);
	// 82E6BE38: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 82E6BE3C: 4B98691D  bl 0x827f2758
	ctx.lr = 0x82E6BE40;
	sub_827F2758(ctx, base);
	// 82E6BE40: 389F0014  addi r4, r31, 0x14
	ctx.r[4].s64 = ctx.r[31].s64 + 20;
	// 82E6BE44: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E6BE48: 80DF001C  lwz r6, 0x1c(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E6BE4C: 80BF0018  lwz r5, 0x18(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E6BE50: 4BFFFED9  bl 0x82e6bd28
	ctx.lr = 0x82E6BE54;
	sub_82E6BD28(ctx, base);
	// 82E6BE54: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E6BE58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E6BE5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E6BE60: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E6BE64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6BE68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E6BE68 size=480
    let mut pc: u32 = 0x82E6BE68;
    'dispatch: loop {
        match pc {
            0x82E6BE68 => {
    //   block [0x82E6BE68..0x82E6C048)
	// 82E6BE68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6BE6C: 4833C2F1  bl 0x831a815c
	ctx.lr = 0x82E6BE70;
	sub_831A8130(ctx, base);
	// 82E6BE70: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E6BE74: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E6BE78: 3D60030C  lis r11, 0x30c
	ctx.r[11].s64 = 51118080;
	// 82E6BE7C: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82E6BE80: 617F30C3  ori r31, r11, 0x30c3
	ctx.r[31].u64 = ctx.r[11].u64 | 12483;
	// 82E6BE84: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6BE88: 7D6AF850  subf r11, r10, r31
	ctx.r[11].s64 = ctx.r[31].s64 - ctx.r[10].s64;
	// 82E6BE8C: 7F0BD040  cmplw cr6, r11, r26
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82E6BE90: 40980008  bge cr6, 0x82e6be98
	if !ctx.cr[6].lt {
	pc = 0x82E6BE98; continue 'dispatch;
	}
	// 82E6BE94: 4BF9D03D  bl 0x82e08ed0
	ctx.lr = 0x82E6BE98;
	sub_82E08ED0(ctx, base);
	// 82E6BE98: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6BE9C: 556AF87E  srwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shr(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E6BEA0: 2B0A0008  cmplwi cr6, r10, 8
	ctx.cr[6].compare_u32(ctx.r[10].u32, 8 as u32, &mut ctx.xer);
	// 82E6BEA4: 40980008  bge cr6, 0x82e6beac
	if !ctx.cr[6].lt {
	pc = 0x82E6BEAC; continue 'dispatch;
	}
	// 82E6BEA8: 39400008  li r10, 8
	ctx.r[10].s64 = 8;
	// 82E6BEAC: 7F1A5040  cmplw cr6, r26, r10
	ctx.cr[6].compare_u32(ctx.r[26].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82E6BEB0: 40980014  bge cr6, 0x82e6bec4
	if !ctx.cr[6].lt {
	pc = 0x82E6BEC4; continue 'dispatch;
	}
	// 82E6BEB4: 7D2AF850  subf r9, r10, r31
	ctx.r[9].s64 = ctx.r[31].s64 - ctx.r[10].s64;
	// 82E6BEB8: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82E6BEBC: 41990008  bgt cr6, 0x82e6bec4
	if ctx.cr[6].gt {
	pc = 0x82E6BEC4; continue 'dispatch;
	}
	// 82E6BEC0: 7D5A5378  mr r26, r10
	ctx.r[26].u64 = ctx.r[10].u64;
	// 82E6BEC4: 3F208335  lis r25, -0x7ccb
	ctx.r[25].s64 = -2093678592;
	// 82E6BEC8: 83BE000C  lwz r29, 0xc(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E6BECC: 7D6BD214  add r11, r11, r26
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[26].u64;
	// 82E6BED0: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82E6BED4: 5566103A  slwi r6, r11, 2
	ctx.r[6].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82E6BED8: 388A08B0  addi r4, r10, 0x8b0
	ctx.r[4].s64 = ctx.r[10].s64 + 2224;
	// 82E6BEDC: 38A0002D  li r5, 0x2d
	ctx.r[5].s64 = 45;
	// 82E6BEE0: 8079110C  lwz r3, 0x110c(r25)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82E6BEE4: 4BF861E5  bl 0x82df20c8
	ctx.lr = 0x82E6BEE8;
	sub_82DF20C8(ctx, base);
	// 82E6BEE8: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E6BEEC: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6BEF0: 57BF103A  slwi r31, r29, 2
	ctx.r[31].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 82E6BEF4: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82E6BEF8: 7CABFA14  add r5, r11, r31
	ctx.r[5].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82E6BEFC: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E6BF00: 7C7FDA14  add r3, r31, r27
	ctx.r[3].u64 = ctx.r[31].u64 + ctx.r[27].u64;
	// 82E6BF04: 7D455050  subf r10, r5, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[5].s64;
	// 82E6BF08: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82E6BF0C: 7D6B1671  srawi. r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E6BF10: 5566103A  slwi r6, r11, 2
	ctx.r[6].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82E6BF14: 7F861A14  add r28, r6, r3
	ctx.r[28].u64 = ctx.r[6].u64 + ctx.r[3].u64;
	// 82E6BF18: 4182000C  beq 0x82e6bf24
	if ctx.cr[0].eq {
	pc = 0x82E6BF24; continue 'dispatch;
	}
	// 82E6BF1C: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 82E6BF20: 4833CDE9  bl 0x831a8d08
	ctx.lr = 0x82E6BF24;
	sub_831A8D08(ctx, base);
	// 82E6BF24: 80BE0004  lwz r5, 4(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E6BF28: 7F1DD040  cmplw cr6, r29, r26
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82E6BF2C: 4199007C  bgt cr6, 0x82e6bfa8
	if ctx.cr[6].gt {
	pc = 0x82E6BFA8; continue 'dispatch;
	}
	// 82E6BF30: 7D652850  subf r11, r5, r5
	ctx.r[11].s64 = ctx.r[5].s64 - ctx.r[5].s64;
	// 82E6BF34: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82E6BF38: 7D6B1671  srawi. r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E6BF3C: 5566103A  slwi r6, r11, 2
	ctx.r[6].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82E6BF40: 7FE6E214  add r31, r6, r28
	ctx.r[31].u64 = ctx.r[6].u64 + ctx.r[28].u64;
	// 82E6BF44: 41820010  beq 0x82e6bf54
	if ctx.cr[0].eq {
	pc = 0x82E6BF54; continue 'dispatch;
	}
	// 82E6BF48: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 82E6BF4C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82E6BF50: 4833CDB9  bl 0x831a8d08
	ctx.lr = 0x82E6BF54;
	sub_831A8D08(ctx, base);
	// 82E6BF54: 7D7DD051  subf. r11, r29, r26
	ctx.r[11].s64 = ctx.r[26].s64 - ctx.r[29].s64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E6BF58: 41820024  beq 0x82e6bf7c
	if ctx.cr[0].eq {
	pc = 0x82E6BF7C; continue 'dispatch;
	}
	// 82E6BF5C: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 82E6BF60: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82E6BF64: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E6BF68: 41820014  beq 0x82e6bf7c
	if ctx.cr[0].eq {
	pc = 0x82E6BF7C; continue 'dispatch;
	}
	// 82E6BF6C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E6BF70: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82E6BF74: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82E6BF78: 4200FFF8  bdnz 0x82e6bf70
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82E6BF70; continue 'dispatch;
	}
	// 82E6BF7C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82E6BF80: 419A009C  beq cr6, 0x82e6c01c
	if ctx.cr[6].eq {
	pc = 0x82E6C01C; continue 'dispatch;
	}
	// 82E6BF84: 7F6BDB78  mr r11, r27
	ctx.r[11].u64 = ctx.r[27].u64;
	// 82E6BF88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E6BF8C: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E6BF90: 4182008C  beq 0x82e6c01c
	if ctx.cr[0].eq {
	pc = 0x82E6C01C; continue 'dispatch;
	}
	// 82E6BF94: 7FA903A6  mtctr r29
	ctx.ctr.u64 = ctx.r[29].u64;
	// 82E6BF98: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E6BF9C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82E6BFA0: 4200FFF8  bdnz 0x82e6bf98
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82E6BF98; continue 'dispatch;
	}
	// 82E6BFA4: 48000078  b 0x82e6c01c
	pc = 0x82E6C01C; continue 'dispatch;
	// 82E6BFA8: 575D103A  slwi r29, r26, 2
	ctx.r[29].u32 = ctx.r[26].u32.wrapping_shl(2);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 82E6BFAC: 7D65E850  subf r11, r5, r29
	ctx.r[11].s64 = ctx.r[29].s64 - ctx.r[5].s64;
	// 82E6BFB0: 7D6B2A14  add r11, r11, r5
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 82E6BFB4: 7D6B1671  srawi. r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E6BFB8: 41820014  beq 0x82e6bfcc
	if ctx.cr[0].eq {
	pc = 0x82E6BFCC; continue 'dispatch;
	}
	// 82E6BFBC: 5564103A  slwi r4, r11, 2
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82E6BFC0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82E6BFC4: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 82E6BFC8: 4833CD41  bl 0x831a8d08
	ctx.lr = 0x82E6BFCC;
	sub_831A8D08(ctx, base);
	// 82E6BFCC: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E6BFD0: 7CBD5A14  add r5, r29, r11
	ctx.r[5].u64 = ctx.r[29].u64 + ctx.r[11].u64;
	// 82E6BFD4: 7D655850  subf r11, r5, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[5].s64;
	// 82E6BFD8: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82E6BFDC: 7D6B1671  srawi. r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E6BFE0: 5566103A  slwi r6, r11, 2
	ctx.r[6].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82E6BFE4: 7FE6DA14  add r31, r6, r27
	ctx.r[31].u64 = ctx.r[6].u64 + ctx.r[27].u64;
	// 82E6BFE8: 41820010  beq 0x82e6bff8
	if ctx.cr[0].eq {
	pc = 0x82E6BFF8; continue 'dispatch;
	}
	// 82E6BFEC: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 82E6BFF0: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82E6BFF4: 4833CD15  bl 0x831a8d08
	ctx.lr = 0x82E6BFF8;
	sub_831A8D08(ctx, base);
	// 82E6BFF8: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 82E6BFFC: 419A0020  beq cr6, 0x82e6c01c
	if ctx.cr[6].eq {
	pc = 0x82E6C01C; continue 'dispatch;
	}
	// 82E6C000: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E6C004: 281A0000  cmplwi r26, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E6C008: 41820014  beq 0x82e6c01c
	if ctx.cr[0].eq {
	pc = 0x82E6C01C; continue 'dispatch;
	}
	// 82E6C00C: 7F4903A6  mtctr r26
	ctx.ctr.u64 = ctx.r[26].u64;
	// 82E6C010: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E6C014: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82E6C018: 4200FFF8  bdnz 0x82e6c010
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82E6C010; continue 'dispatch;
	}
	// 82E6C01C: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E6C020: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82E6C024: 419A000C  beq cr6, 0x82e6c030
	if ctx.cr[6].eq {
	pc = 0x82E6C030; continue 'dispatch;
	}
	// 82E6C028: 8079110C  lwz r3, 0x110c(r25)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82E6C02C: 4BF8615D  bl 0x82df2188
	ctx.lr = 0x82E6C030;
	sub_82DF2188(ctx, base);
	// 82E6C030: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6C034: 937E0004  stw r27, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[27].u32 ) };
	// 82E6C038: 7D6BD214  add r11, r11, r26
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[26].u64;
	// 82E6C03C: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E6C040: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E6C044: 4833C168  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6C048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E6C048 size=988
    let mut pc: u32 = 0x82E6C048;
    'dispatch: loop {
        match pc {
            0x82E6C048 => {
    //   block [0x82E6C048..0x82E6C424)
	// 82E6C048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6C04C: 4833C10D  bl 0x831a8158
	ctx.lr = 0x82E6C050;
	sub_831A8130(ctx, base);
	// 82E6C050: DBE1FFB0  stfd f31, -0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-80 as u32), ctx.f[31].u64 ) };
	// 82E6C054: 3980FFA0  li r12, -0x60
	ctx.r[12].s64 = -96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6C428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E6C428 size=176
    let mut pc: u32 = 0x82E6C428;
    'dispatch: loop {
        match pc {
            0x82E6C428 => {
    //   block [0x82E6C428..0x82E6C4D8)
	// 82E6C428: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6C42C: 4833BD3D  bl 0x831a8168
	ctx.lr = 0x82E6C430;
	sub_831A8130(ctx, base);
	// 82E6C430: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E6C434: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E6C438: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82E6C43C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E6C440: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6C444: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E6C448: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E6C44C: 4199000C  bgt cr6, 0x82e6c458
	if ctx.cr[6].gt {
	pc = 0x82E6C458; continue 'dispatch;
	}
	// 82E6C450: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E6C454: 4BFFFA15  bl 0x82e6be68
	ctx.lr = 0x82E6C458;
	sub_82E6BE68(ctx, base);
	// 82E6C458: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E6C45C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E6C460: 409A0008  bne cr6, 0x82e6c468
	if !ctx.cr[6].eq {
	pc = 0x82E6C468; continue 'dispatch;
	}
	// 82E6C464: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6C468: 3BABFFFF  addi r29, r11, -1
	ctx.r[29].s64 = ctx.r[11].s64 + -1;
	// 82E6C46C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E6C470: 57BE103A  slwi r30, r29, 2
	ctx.r[30].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 82E6C474: 7D6BF02E  lwzx r11, r11, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82E6C478: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E6C47C: 409A0028  bne cr6, 0x82e6c4a4
	if !ctx.cr[6].eq {
	pc = 0x82E6C4A4; continue 'dispatch;
	}
	// 82E6C480: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82E6C484: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82E6C488: 38C00054  li r6, 0x54
	ctx.r[6].s64 = 84;
	// 82E6C48C: 388A08B0  addi r4, r10, 0x8b0
	ctx.r[4].s64 = ctx.r[10].s64 + 2224;
	// 82E6C490: 38A0002D  li r5, 0x2d
	ctx.r[5].s64 = 45;
	// 82E6C494: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82E6C498: 4BF85C31  bl 0x82df20c8
	ctx.lr = 0x82E6C49C;
	sub_82DF20C8(ctx, base);
	// 82E6C49C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E6C4A0: 7C6BF12E  stwx r3, r11, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32), ctx.r[3].u32) };
	// 82E6C4A4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E6C4A8: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82E6C4AC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E6C4B0: 419A0010  beq cr6, 0x82e6c4c0
	if ctx.cr[6].eq {
	pc = 0x82E6C4C0; continue 'dispatch;
	}
	// 82E6C4B4: 38A00054  li r5, 0x54
	ctx.r[5].s64 = 84;
	// 82E6C4B8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E6C4BC: 4833C055  bl 0x831a8510
	ctx.lr = 0x82E6C4C0;
	sub_831A8510(ctx, base);
	// 82E6C4C0: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E6C4C4: 93BF000C  stw r29, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 82E6C4C8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E6C4CC: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82E6C4D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E6C4D4: 4833BCE4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6C4D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E6C4D8 size=112
    let mut pc: u32 = 0x82E6C4D8;
    'dispatch: loop {
        match pc {
            0x82E6C4D8 => {
    //   block [0x82E6C4D8..0x82E6C548)
	// 82E6C4D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6C4DC: 4833BC91  bl 0x831a816c
	ctx.lr = 0x82E6C4E0;
	sub_831A8130(ctx, base);
	// 82E6C4E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E6C4E4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E6C4E8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E6C4EC: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82E6C4F0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E6C4F4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E6C4F8: 419A0018  beq cr6, 0x82e6c510
	if ctx.cr[6].eq {
	pc = 0x82E6C510; continue 'dispatch;
	}
	// 82E6C4FC: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6C500: 39400030  li r10, 0x30
	ctx.r[10].s64 = 48;
	// 82E6C504: 7D2B4850  subf r9, r11, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 82E6C508: 7D2953D7  divw. r9, r9, r10
	ctx.r[9].s32 = ctx.r[9].s32 / ctx.r[10].s32;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E6C50C: 4082000C  bne 0x82e6c518
	if !ctx.cr[0].eq {
	pc = 0x82E6C518; continue 'dispatch;
	}
	// 82E6C510: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82E6C514: 4800000C  b 0x82e6c520
	pc = 0x82E6C520; continue 'dispatch;
	// 82E6C518: 7D6B2050  subf r11, r11, r4
	ctx.r[11].s64 = ctx.r[4].s64 - ctx.r[11].s64;
	// 82E6C51C: 7FCB53D6  divw r30, r11, r10
	ctx.r[30].s32 = ctx.r[11].s32 / ctx.r[10].s32;
	// 82E6C520: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82E6C524: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6C528: 4BFFFB21  bl 0x82e6c048
	ctx.lr = 0x82E6C52C;
	sub_82E6C048(ctx, base);
	// 82E6C52C: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E6C530: 1D7E0030  mulli r11, r30, 0x30
	ctx.r[11].s64 = ctx.r[30].s64 * 48;
	// 82E6C534: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82E6C538: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E6C53C: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E6C540: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E6C544: 4833BC78  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6C548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E6C548 size=160
    let mut pc: u32 = 0x82E6C548;
    'dispatch: loop {
        match pc {
            0x82E6C548 => {
    //   block [0x82E6C548..0x82E6C5E8)
	// 82E6C548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6C54C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E6C550: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E6C554: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E6C558: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 82E6C55C: 39200030  li r9, 0x30
	ctx.r[9].s64 = 48;
	// 82E6C560: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E6C564: 409A000C  bne cr6, 0x82e6c570
	if !ctx.cr[6].eq {
	pc = 0x82E6C570; continue 'dispatch;
	}
	// 82E6C568: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E6C56C: 48000010  b 0x82e6c57c
	pc = 0x82E6C57C; continue 'dispatch;
	// 82E6C570: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6C574: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82E6C578: 7D4A4BD6  divw r10, r10, r9
	ctx.r[10].s32 = ctx.r[10].s32 / ctx.r[9].s32;
	// 82E6C57C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E6C580: 419A0048  beq cr6, 0x82e6c5c8
	if ctx.cr[6].eq {
	pc = 0x82E6C5C8; continue 'dispatch;
	}
	// 82E6C584: 8103000C  lwz r8, 0xc(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E6C588: 7D6B4050  subf r11, r11, r8
	ctx.r[11].s64 = ctx.r[8].s64 - ctx.r[11].s64;
	// 82E6C58C: 7D6B4BD6  divw r11, r11, r9
	ctx.r[11].s32 = ctx.r[11].s32 / ctx.r[9].s32;
	// 82E6C590: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E6C594: 40980034  bge cr6, 0x82e6c5c8
	if !ctx.cr[6].lt {
	pc = 0x82E6C5C8; continue 'dispatch;
	}
	// 82E6C598: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6C59C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E6C5A0: 419A001C  beq cr6, 0x82e6c5bc
	if ctx.cr[6].eq {
	pc = 0x82E6C5BC; continue 'dispatch;
	}
	// 82E6C5A4: 13E030C7  vcmpequd (lvx128) v31, v0, v6
	tmp.u32 = ctx.r[6].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6C5E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E6C5E8 size=108
    let mut pc: u32 = 0x82E6C5E8;
    'dispatch: loop {
        match pc {
            0x82E6C5E8 => {
    //   block [0x82E6C5E8..0x82E6C654)
	// 82E6C5E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6C5EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E6C5F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E6C5F4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E6C5F8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E6C5FC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E6C600: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 82E6C604: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E6C608: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E6C60C: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82E6C610: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82E6C614: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82E6C618: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82E6C61C: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82E6C620: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82E6C624: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82E6C628: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 82E6C62C: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 82E6C630: 915F0038  stw r10, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[10].u32 ) };
	// 82E6C634: 917F003C  stw r11, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 82E6C638: 4BFFF7E9  bl 0x82e6be20
	ctx.lr = 0x82E6C63C;
	sub_82E6BE20(ctx, base);
	// 82E6C63C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6C640: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E6C644: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E6C648: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E6C64C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E6C650: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6C658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E6C658 size=480
    let mut pc: u32 = 0x82E6C658;
    'dispatch: loop {
        match pc {
            0x82E6C658 => {
    //   block [0x82E6C658..0x82E6C838)
	// 82E6C658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6C65C: 4833BB01  bl 0x831a815c
	ctx.lr = 0x82E6C660;
	sub_831A8130(ctx, base);
	// 82E6C660: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E6C664: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82E6C668: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82E6C66C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82E6C670: 4800BED9  bl 0x82e78548
	ctx.lr = 0x82E6C674;
	sub_82E78548(ctx, base);
	// 82E6C674: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82E6C678: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6C67C: 4800BECD  bl 0x82e78548
	ctx.lr = 0x82E6C680;
	sub_82E78548(ctx, base);
	// 82E6C680: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82E6C684: 3940000C  li r10, 0xc
	ctx.r[10].s64 = 12;
	// 82E6C688: 93C10074  stw r30, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[30].u32 ) };
	// 82E6C68C: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82E6C690: 93C10078  stw r30, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[30].u32 ) };
	// 82E6C694: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82E6C698: 93C1007C  stw r30, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[30].u32 ) };
	// 82E6C69C: 93C10064  stw r30, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[30].u32 ) };
	// 82E6C6A0: 93C10068  stw r30, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[30].u32 ) };
	// 82E6C6A4: 93C1006C  stw r30, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[30].u32 ) };
	// 82E6C6A8: 419A0020  beq cr6, 0x82e6c6c8
	if ctx.cr[6].eq {
	pc = 0x82E6C6C8; continue 'dispatch;
	}
	// 82E6C6AC: 817C001C  lwz r11, 0x1c(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E6C6B0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E6C6B4: 419A0014  beq cr6, 0x82e6c6c8
	if ctx.cr[6].eq {
	pc = 0x82E6C6C8; continue 'dispatch;
	}
	// 82E6C6B8: 813C0020  lwz r9, 0x20(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E6C6BC: 7D6B4850  subf r11, r11, r9
	ctx.r[11].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 82E6C6C0: 7F2B53D6  divw r25, r11, r10
	ctx.r[25].s32 = ctx.r[11].s32 / ctx.r[10].s32;
	// 82E6C6C4: 48000008  b 0x82e6c6cc
	pc = 0x82E6C6CC; continue 'dispatch;
	// 82E6C6C8: 7FD9F378  mr r25, r30
	ctx.r[25].u64 = ctx.r[30].u64;
	// 82E6C6CC: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82E6C6D0: 419A0020  beq cr6, 0x82e6c6f0
	if ctx.cr[6].eq {
	pc = 0x82E6C6F0; continue 'dispatch;
	}
	// 82E6C6D4: 817B001C  lwz r11, 0x1c(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E6C6D8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E6C6DC: 419A0014  beq cr6, 0x82e6c6f0
	if ctx.cr[6].eq {
	pc = 0x82E6C6F0; continue 'dispatch;
	}
	// 82E6C6E0: 813B0020  lwz r9, 0x20(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E6C6E4: 7D6B4850  subf r11, r11, r9
	ctx.r[11].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 82E6C6E8: 7FAB53D6  divw r29, r11, r10
	ctx.r[29].s32 = ctx.r[11].s32 / ctx.r[10].s32;
	// 82E6C6EC: 48000008  b 0x82e6c6f4
	pc = 0x82E6C6F4; continue 'dispatch;
	// 82E6C6F0: 7FDDF378  mr r29, r30
	ctx.r[29].u64 = ctx.r[30].u64;
	// 82E6C6F4: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 82E6C6F8: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 82E6C6FC: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82E6C700: 419A0120  beq cr6, 0x82e6c820
	if ctx.cr[6].eq {
	pc = 0x82E6C820; continue 'dispatch;
	}
	// 82E6C704: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 82E6C708: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82E6C70C: 419A006C  beq cr6, 0x82e6c778
	if ctx.cr[6].eq {
	pc = 0x82E6C778; continue 'dispatch;
	}
	// 82E6C710: 813C001C  lwz r9, 0x1c(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E6C714: 1D5F000C  mulli r10, r31, 0xc
	ctx.r[10].s64 = ctx.r[31].s64 * 12;
	// 82E6C718: 817B001C  lwz r11, 0x1c(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E6C71C: 7C0A4C2E  lfsx f0, r10, r9
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E6C720: 7C8A4A14  add r4, r10, r9
	ctx.r[4].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82E6C724: C1AB0000  lfs f13, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E6C728: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82E6C72C: 409A0024  bne cr6, 0x82e6c750
	if !ctx.cr[6].eq {
	pc = 0x82E6C750; continue 'dispatch;
	}
	// 82E6C730: C1A40004  lfs f13, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E6C734: C18B0004  lfs f12, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E6C738: FF0D6000  fcmpu cr6, f13, f12
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[12].f64);
	// 82E6C73C: 409A0014  bne cr6, 0x82e6c750
	if !ctx.cr[6].eq {
	pc = 0x82E6C750; continue 'dispatch;
	}
	// 82E6C740: C1A40008  lfs f13, 8(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E6C744: C18B0008  lfs f12, 8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E6C748: FF0D6000  fcmpu cr6, f13, f12
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[12].f64);
	// 82E6C74C: 419A0018  beq cr6, 0x82e6c764
	if ctx.cr[6].eq {
	pc = 0x82E6C764; continue 'dispatch;
	}
	// 82E6C750: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 82E6C754: 396B000C  addi r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 + 12;
	// 82E6C758: 7F08E840  cmplw cr6, r8, r29
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82E6C75C: 4198FFC8  blt cr6, 0x82e6c724
	if ctx.cr[6].lt {
	pc = 0x82E6C724; continue 'dispatch;
	}
	// 82E6C760: 48000018  b 0x82e6c778
	pc = 0x82E6C778; continue 'dispatch;
	// 82E6C764: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82E6C768: 4B7ED159  bl 0x826598c0
	ctx.lr = 0x82E6C76C;
	sub_826598C0(ctx, base);
	// 82E6C76C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E6C770: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E6C774: 4B64C99D  bl 0x824b9110
	ctx.lr = 0x82E6C778;
	sub_824B9110(ctx, base);
	// 82E6C778: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82E6C77C: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82E6C780: 7F1FC840  cmplw cr6, r31, r25
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[25].u32, &mut ctx.xer);
	// 82E6C784: 4198FF80  blt cr6, 0x82e6c704
	if ctx.cr[6].lt {
	pc = 0x82E6C704; continue 'dispatch;
	}
	// 82E6C788: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82E6C78C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E6C790: 419A0090  beq cr6, 0x82e6c820
	if ctx.cr[6].eq {
	pc = 0x82E6C820; continue 'dispatch;
	}
	// 82E6C794: 81410068  lwz r10, 0x68(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E6C798: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82E6C79C: 7D4A1670  srawi r10, r10, 2
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 2) as i64;
	// 82E6C7A0: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82E6C7A4: 4099007C  ble cr6, 0x82e6c820
	if !ctx.cr[6].gt {
	pc = 0x82E6C820; continue 'dispatch;
	}
	// 82E6C7A8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E6C7AC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E6C7B0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E6C7B4: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E6C7B8: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82E6C7BC: 409A0034  bne cr6, 0x82e6c7f0
	if !ctx.cr[6].eq {
	pc = 0x82E6C7F0; continue 'dispatch;
	}
	// 82E6C7C0: C00B0000  lfs f0, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E6C7C4: D01A0000  stfs f0, 0(r26)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82E6C7C8: C00B0004  lfs f0, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E6C7CC: D01A0004  stfs f0, 4(r26)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82E6C7D0: C00B0008  lfs f0, 8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E6C7D4: D01A0008  stfs f0, 8(r26)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82E6C7D8: C00B000C  lfs f0, 0xc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E6C7DC: D01A000C  stfs f0, 0xc(r26)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82E6C7E0: C00B0010  lfs f0, 0x10(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E6C7E4: D01A0010  stfs f0, 0x10(r26)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82E6C7E8: C00B0014  lfs f0, 0x14(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E6C7EC: 48000030  b 0x82e6c81c
	pc = 0x82E6C81C; continue 'dispatch;
	// 82E6C7F0: C00B000C  lfs f0, 0xc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E6C7F4: D01A0000  stfs f0, 0(r26)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82E6C7F8: C00B0010  lfs f0, 0x10(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E6C7FC: D01A0004  stfs f0, 4(r26)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82E6C800: C00B0014  lfs f0, 0x14(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E6C804: D01A0008  stfs f0, 8(r26)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82E6C808: C00B0000  lfs f0, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E6C80C: D01A000C  stfs f0, 0xc(r26)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82E6C810: C00B0004  lfs f0, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E6C814: D01A0010  stfs f0, 0x10(r26)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82E6C818: C00B0008  lfs f0, 8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E6C81C: D01A0014  stfs f0, 0x14(r26)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82E6C820: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E6C824: 4B5FD7DD  bl 0x8246a000
	ctx.lr = 0x82E6C828;
	sub_8246A000(ctx, base);
	// 82E6C828: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82E6C82C: 4B5FD7D5  bl 0x8246a000
	ctx.lr = 0x82E6C830;
	sub_8246A000(ctx, base);
	// 82E6C830: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82E6C834: 4833B978  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6C838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E6C838 size=1600
    let mut pc: u32 = 0x82E6C838;
    'dispatch: loop {
        match pc {
            0x82E6C838 => {
    //   block [0x82E6C838..0x82E6CE78)
	// 82E6C838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6C83C: 4833B8F5  bl 0x831a8130
	ctx.lr = 0x82E6C840;
	sub_831A8130(ctx, base);
	// 82E6C840: 3981FF68  addi r12, r1, -0x98
	ctx.r[12].s64 = ctx.r[1].s64 + -152;
	// 82E6C844: 4833C235  bl 0x831a8a78
	ctx.lr = 0x82E6C848;
	sub_831A8A40(ctx, base);
	// 82E6C848: 9421FE70  stwu r1, -0x190(r1)
	ea = ctx.r[1].u32.wrapping_add(-400 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E6C84C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E6C850: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82E6C854: 7C902378  mr r16, r4
	ctx.r[16].u64 = ctx.r[4].u64;
	// 82E6C858: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82E6C85C: 7CCF3378  mr r15, r6
	ctx.r[15].u64 = ctx.r[6].u64;
	// 82E6C860: C38B08A4  lfs f28, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[28].f64 = (tmp.f32 as f64);
	// 82E6C864: 7CF63B78  mr r22, r7
	ctx.r[22].u64 = ctx.r[7].u64;
	// 82E6C868: D39B0000  stfs f28, 0(r27)
	tmp.f32 = (ctx.f[28].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82E6C86C: 7D0E4378  mr r14, r8
	ctx.r[14].u64 = ctx.r[8].u64;
	// 82E6C870: D39B0004  stfs f28, 4(r27)
	tmp.f32 = (ctx.f[28].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82E6C874: 3A5B0004  addi r18, r27, 4
	ctx.r[18].s64 = ctx.r[27].s64 + 4;
	// 82E6C878: D39B0008  stfs f28, 8(r27)
	tmp.f32 = (ctx.f[28].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82E6C87C: 81500008  lwz r10, 8(r16)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[16].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6C880: 8170000C  lwz r11, 0xc(r16)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[16].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E6C884: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82E6C888: 3A3B0008  addi r17, r27, 8
	ctx.r[17].s64 = ctx.r[27].s64 + 8;
	// 82E6C88C: 3BD0000C  addi r30, r16, 0xc
	ctx.r[30].s64 = ctx.r[16].s64 + 12;
	// 82E6C890: 3BB00008  addi r29, r16, 8
	ctx.r[29].s64 = ctx.r[16].s64 + 8;
	// 82E6C894: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E6C898: 41990008  bgt cr6, 0x82e6c8a0
	if ctx.cr[6].gt {
	pc = 0x82E6C8A0; continue 'dispatch;
	}
	// 82E6C89C: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82E6C8A0: 81500004  lwz r10, 4(r16)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[16].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E6C8A4: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E6C8A8: 3AF00004  addi r23, r16, 4
	ctx.r[23].s64 = ctx.r[16].s64 + 4;
	// 82E6C8AC: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E6C8B0: 4800BCB9  bl 0x82e78568
	ctx.lr = 0x82E6C8B4;
	sub_82E78568(ctx, base);
	// 82E6C8B4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E6C8B8: 4182056C  beq 0x82e6ce24
	if ctx.cr[0].eq {
	pc = 0x82E6CE24; continue 'dispatch;
	}
	// 82E6C8BC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E6C8C0: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E6C8C4: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82E6C8C8: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E6C8CC: 41990008  bgt cr6, 0x82e6c8d4
	if ctx.cr[6].gt {
	pc = 0x82E6C8D4; continue 'dispatch;
	}
	// 82E6C8D0: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82E6C8D4: 81570000  lwz r10, 0(r23)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E6C8D8: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E6C8DC: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E6C8E0: 4800BCB9  bl 0x82e78598
	ctx.lr = 0x82E6C8E4;
	sub_82E78598(ctx, base);
	// 82E6C8E4: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82E6C8E8: 419A053C  beq cr6, 0x82e6ce24
	if ctx.cr[6].eq {
	pc = 0x82E6CE24; continue 'dispatch;
	}
	// 82E6C8EC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E6C8F0: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E6C8F4: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82E6C8F8: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E6C8FC: 41990008  bgt cr6, 0x82e6c904
	if ctx.cr[6].gt {
	pc = 0x82E6C904; continue 'dispatch;
	}
	// 82E6C900: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82E6C904: 81570000  lwz r10, 0(r23)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E6C908: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E6C90C: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E6C910: 4800BC89  bl 0x82e78598
	ctx.lr = 0x82E6C914;
	sub_82E78598(ctx, base);
	// 82E6C914: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E6C918: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82E6C91C: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E6C920: 7D2BFA14  add r9, r11, r31
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82E6C924: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82E6C928: 41990008  bgt cr6, 0x82e6c930
	if ctx.cr[6].gt {
	pc = 0x82E6C930; continue 'dispatch;
	}
	// 82E6C92C: 7D2A4850  subf r9, r10, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[10].s64;
	// 82E6C930: 81170000  lwz r8, 0(r23)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E6C934: 5527103A  slwi r7, r9, 2
	ctx.r[7].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82E6C938: 1D38000C  mulli r9, r24, 0xc
	ctx.r[9].s64 = ctx.r[24].s64 * 12;
	// 82E6C93C: 7CE7402E  lwzx r7, r7, r8
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82E6C940: 7CC74A14  add r6, r7, r9
	ctx.r[6].u64 = ctx.r[7].u64 + ctx.r[9].u64;
	// 82E6C944: 7D27482E  lwzx r9, r7, r9
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82E6C948: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82E6C94C: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E6C950: 80E60004  lwz r7, 4(r6)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E6C954: 80C60008  lwz r6, 8(r6)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6C958: 91210060  stw r9, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[9].u32 ) };
	// 82E6C95C: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82E6C960: 90C10068  stw r6, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[6].u32 ) };
	// 82E6C964: 41990008  bgt cr6, 0x82e6c96c
	if ctx.cr[6].gt {
	pc = 0x82E6C96C; continue 'dispatch;
	}
	// 82E6C968: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82E6C96C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E6C970: 7C6B402E  lwzx r3, r11, r8
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82E6C974: 4800BBED  bl 0x82e78560
	ctx.lr = 0x82E6C978;
	sub_82E78560(ctx, base);
	// 82E6C978: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E6C97C: C1B60008  lfs f13, 8(r22)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E6C980: C0160000  lfs f0, 0(r22)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E6C984: 38810090  addi r4, r1, 0x90
	ctx.r[4].s64 = ctx.r[1].s64 + 144;
	// 82E6C988: C1960004  lfs f12, 4(r22)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E6C98C: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 82E6C990: 3B560008  addi r26, r22, 8
	ctx.r[26].s64 = ctx.r[22].s64 + 8;
	// 82E6C994: 3B360004  addi r25, r22, 4
	ctx.r[25].s64 = ctx.r[22].s64 + 4;
	// 82E6C998: C16B0008  lfs f11, 8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82E6C99C: EDAB6828  fsubs f13, f11, f13
	ctx.f[13].f64 = (((ctx.f[11].f64 - ctx.f[13].f64) as f32) as f64);
	// 82E6C9A0: C14B0004  lfs f10, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82E6C9A4: C16B0000  lfs f11, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82E6C9A8: ED8A6028  fsubs f12, f10, f12
	ctx.f[12].f64 = (((ctx.f[10].f64 - ctx.f[12].f64) as f32) as f64);
	// 82E6C9AC: EC0B0028  fsubs f0, f11, f0
	ctx.f[0].f64 = (((ctx.f[11].f64 - ctx.f[0].f64) as f32) as f64);
	// 82E6C9B0: D0010090  stfs f0, 0x90(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), tmp.u32 ) };
	// 82E6C9B4: D1A10098  stfs f13, 0x98(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(152 as u32), tmp.u32 ) };
	// 82E6C9B8: D1810094  stfs f12, 0x94(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), tmp.u32 ) };
	// 82E6C9BC: 4BDA327D  bl 0x82c0fc38
	ctx.lr = 0x82E6C9C0;
	sub_82C0FC38(ctx, base);
	// 82E6C9C0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E6C9C4: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E6C9C8: D3810070  stfs f28, 0x70(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[28].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 82E6C9CC: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82E6C9D0: D3810074  stfs f28, 0x74(r1)
	tmp.f32 = (ctx.f[28].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 82E6C9D4: D3810078  stfs f28, 0x78(r1)
	tmp.f32 = (ctx.f[28].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 82E6C9D8: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E6C9DC: 41990008  bgt cr6, 0x82e6c9e4
	if ctx.cr[6].gt {
	pc = 0x82E6C9E4; continue 'dispatch;
	}
	// 82E6C9E0: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82E6C9E4: 81570000  lwz r10, 0(r23)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E6C9E8: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E6C9EC: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E6C9F0: 4800BB59  bl 0x82e78548
	ctx.lr = 0x82E6C9F4;
	sub_82E78548(ctx, base);
	// 82E6C9F4: 3961009B  addi r11, r1, 0x9b
	ctx.r[11].s64 = ctx.r[1].s64 + 155;
	// 82E6C9F8: 13A018C7  vcmpequd (lvx128) v29, v0, v3
	tmp.u32 = ctx.r[3].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[61] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82E6C9FC: 39410090  addi r10, r1, 0x90
	ctx.r[10].s64 = ctx.r[1].s64 + 144;
	// 82E6CA00: 7C00180C  lvsl v0, 0, r3
	tmp.u32 = ctx.r[3].u32;
	// ctx.v[0] = VectorShiftTableL[(tmp.u32 & 0xF)]
	// 82E6CA04: 39210090  addi r9, r1, 0x90
	ctx.r[9].s64 = ctx.r[1].s64 + 144;
	// 82E6CA08: 3A60000B  li r19, 0xb
	ctx.r[19].s64 = 11;
	// 82E6CA0C: 3A800004  li r20, 4
	ctx.r[20].s64 = 4;
	// 82E6CA10: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82E6CA14: 39610070  addi r11, r1, 0x70
	ctx.r[11].s64 = ctx.r[1].s64 + 112;
	// 82E6CA18: 13C050C7  vcmpequd (lvx128) v30, v0, v10
	tmp.u32 = ctx.r[10].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82E6CA1C: 39410070  addi r10, r1, 0x70
	ctx.r[10].s64 = ctx.r[1].s64 + 112;
	// 82E6CA20: 7CE0480C  lvsl v7, 0, r9
	tmp.u32 = ctx.r[9].u32;
	// ctx.v[7] = VectorShiftTableL[(tmp.u32 & 0xF)]
	// 82E6CA24: 39010070  addi r8, r1, 0x70
	ctx.r[8].s64 = ctx.r[1].s64 + 112;
	// 82E6CA28: 138398C7  vcmpequd (lvx128) v28, v3, v19
	tmp.u32 = ctx.r[3].u32 + ctx.r[19].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[60] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6CE78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E6CE78 size=420
    let mut pc: u32 = 0x82E6CE78;
    'dispatch: loop {
        match pc {
            0x82E6CE78 => {
    //   block [0x82E6CE78..0x82E6D01C)
	// 82E6CE78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6CE7C: 4833B2E5  bl 0x831a8160
	ctx.lr = 0x82E6CE80;
	sub_831A8130(ctx, base);
	// 82E6CE80: DBE1FFC0  stfd f31, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.f[31].u64 ) };
	// 82E6CE84: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E6CE88: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E6CE8C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E6CE90: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E6CE94: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82E6CE98: C3EB08A4  lfs f31, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82E6CE9C: D3E10050  stfs f31, 0x50(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82E6CEA0: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E6CEA4: D3E10054  stfs f31, 0x54(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82E6CEA8: 813E0004  lwz r9, 4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E6CEAC: D3E10058  stfs f31, 0x58(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82E6CEB0: 811E0008  lwz r8, 8(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6CEB4: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E6CEB8: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 82E6CEBC: 91210064  stw r9, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[9].u32 ) };
	// 82E6CEC0: 91010068  stw r8, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[8].u32 ) };
	// 82E6CEC4: 48001A4D  bl 0x82e6e910
	ctx.lr = 0x82E6CEC8;
	sub_82E6E910(ctx, base);
	// 82E6CEC8: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82E6CECC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82E6CED0: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82E6CED4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E6CED8: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 82E6CEDC: 3B5F0014  addi r26, r31, 0x14
	ctx.r[26].s64 = ctx.r[31].s64 + 20;
	// 82E6CEE0: 4BFFDE41  bl 0x82e6ad20
	ctx.lr = 0x82E6CEE4;
	sub_82E6AD20(ctx, base);
	// 82E6CEE4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E6CEE8: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82E6CEEC: 4BFFF65D  bl 0x82e6c548
	ctx.lr = 0x82E6CEF0;
	sub_82E6C548(ctx, base);
	// 82E6CEF0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82E6CEF4: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82E6CEF8: 40990118  ble cr6, 0x82e6d010
	if !ctx.cr[6].gt {
	pc = 0x82E6D010; continue 'dispatch;
	}
	// 82E6CEFC: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E6CF00: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6CF04: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82E6CF08: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E6CF0C: 41990008  bgt cr6, 0x82e6cf14
	if ctx.cr[6].gt {
	pc = 0x82E6CF14; continue 'dispatch;
	}
	// 82E6CF10: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82E6CF14: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E6CF18: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E6CF1C: 7FAB502E  lwzx r29, r11, r10
	ctx.r[29].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E6CF20: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E6CF24: 4800B635  bl 0x82e78558
	ctx.lr = 0x82E6CF28;
	sub_82E78558(ctx, base);
	// 82E6CF28: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82E6CF2C: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82E6CF30: 409A00D4  bne cr6, 0x82e6d004
	if !ctx.cr[6].eq {
	pc = 0x82E6D004; continue 'dispatch;
	}
	// 82E6CF34: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E6CF38: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E6CF3C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82E6CF40: 4BFFE669  bl 0x82e6b5a8
	ctx.lr = 0x82E6CF44;
	sub_82E6B5A8(ctx, base);
	// 82E6CF44: 817F003C  lwz r11, 0x3c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 82E6CF48: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E6CF4C: 409A0014  bne cr6, 0x82e6cf60
	if !ctx.cr[6].eq {
	pc = 0x82E6CF60; continue 'dispatch;
	}
	// 82E6CF50: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E6CF54: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82E6CF58: 4BFFDD01  bl 0x82e6ac58
	ctx.lr = 0x82E6CF5C;
	sub_82E6AC58(ctx, base);
	// 82E6CF5C: 4800004C  b 0x82e6cfa8
	pc = 0x82E6CFA8; continue 'dispatch;
	// 82E6CF60: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82E6CF64: 409A0020  bne cr6, 0x82e6cf84
	if !ctx.cr[6].eq {
	pc = 0x82E6CF84; continue 'dispatch;
	}
	// 82E6CF68: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82E6CF6C: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 82E6CF70: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 82E6CF74: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E6CF78: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 82E6CF7C: 4BFFE20D  bl 0x82e6b188
	ctx.lr = 0x82E6CF80;
	sub_82E6B188(ctx, base);
	// 82E6CF80: 48000028  b 0x82e6cfa8
	pc = 0x82E6CFA8; continue 'dispatch;
	// 82E6CF84: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82E6CF88: 409A0038  bne cr6, 0x82e6cfc0
	if !ctx.cr[6].eq {
	pc = 0x82E6CFC0; continue 'dispatch;
	}
	// 82E6CF8C: 7F68DB78  mr r8, r27
	ctx.r[8].u64 = ctx.r[27].u64;
	// 82E6CF90: 38E10060  addi r7, r1, 0x60
	ctx.r[7].s64 = ctx.r[1].s64 + 96;
	// 82E6CF94: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82E6CF98: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E6CF9C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E6CFA0: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 82E6CFA4: 4BFFF895  bl 0x82e6c838
	ctx.lr = 0x82E6CFA8;
	sub_82E6C838(ctx, base);
	// 82E6CFA8: C0030000  lfs f0, 0(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E6CFAC: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82E6CFB0: C0030004  lfs f0, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E6CFB4: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82E6CFB8: C0030008  lfs f0, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E6CFBC: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82E6CFC0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E6CFC4: C0010050  lfs f0, 0x50(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E6CFC8: C1A10054  lfs f13, 0x54(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E6CFCC: C1810058  lfs f12, 0x58(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E6CFD0: D0010060  stfs f0, 0x60(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 82E6CFD4: D1A10064  stfs f13, 0x64(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 82E6CFD8: D1810068  stfs f12, 0x68(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 82E6CFDC: 4800B56D  bl 0x82e78548
	ctx.lr = 0x82E6CFE0;
	sub_82E78548(ctx, base);
	// 82E6CFE0: 3963004C  addi r11, r3, 0x4c
	ctx.r[11].s64 = ctx.r[3].s64 + 76;
	// 82E6CFE4: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82E6CFE8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E6CFEC: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 82E6CFF0: 80AB0004  lwz r5, 4(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E6CFF4: 4BFFDD2D  bl 0x82e6ad20
	ctx.lr = 0x82E6CFF8;
	sub_82E6AD20(ctx, base);
	// 82E6CFF8: 388100B0  addi r4, r1, 0xb0
	ctx.r[4].s64 = ctx.r[1].s64 + 176;
	// 82E6CFFC: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82E6D000: 4BFFF549  bl 0x82e6c548
	ctx.lr = 0x82E6D004;
	sub_82E6C548(ctx, base);
	// 82E6D004: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82E6D008: 7F1EE000  cmpw cr6, r30, r28
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[28].s32, &mut ctx.xer);
	// 82E6D00C: 4198FEF0  blt cr6, 0x82e6cefc
	if ctx.cr[6].lt {
	pc = 0x82E6CEFC; continue 'dispatch;
	}
	// 82E6D010: 38210120  addi r1, r1, 0x120
	ctx.r[1].s64 = ctx.r[1].s64 + 288;
	// 82E6D014: CBE1FFC0  lfd f31, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-64 as u32) ) };
	// 82E6D018: 4833B198  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6D020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E6D020 size=192
    let mut pc: u32 = 0x82E6D020;
    'dispatch: loop {
        match pc {
            0x82E6D020 => {
    //   block [0x82E6D020..0x82E6D0E0)
	// 82E6D020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6D024: 4833B135  bl 0x831a8158
	ctx.lr = 0x82E6D028;
	sub_831A8130(ctx, base);
	// 82E6D028: DBE1FFB0  stfd f31, -0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-80 as u32), ctx.f[31].u64 ) };
	// 82E6D02C: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E6D030: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E6D034: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82E6D038: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82E6D03C: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 82E6D040: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 82E6D044: 7D394B78  mr r25, r9
	ctx.r[25].u64 = ctx.r[9].u64;
	// 82E6D048: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E6D04C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82E6D050: 419A0084  beq cr6, 0x82e6d0d4
	if ctx.cr[6].eq {
	pc = 0x82E6D0D4; continue 'dispatch;
	}
	// 82E6D054: 395E0001  addi r10, r30, 1
	ctx.r[10].s64 = ctx.r[30].s64 + 1;
	// 82E6D058: 3B030024  addi r24, r3, 0x24
	ctx.r[24].s64 = ctx.r[3].s64 + 36;
	// 82E6D05C: 794A0020  clrldi r10, r10, 0x20
	ctx.r[10].u64 = ctx.r[10].u64 & 0x00000000FFFFFFFFu64;
	// 82E6D060: F9410050  std r10, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u64 ) };
	// 82E6D064: C8010050  lfd f0, 0x50(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82E6D068: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 82E6D06C: FFE00018  frsp f31, f0
	ctx.f[31].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82E6D070: 3BEB0001  addi r31, r11, 1
	ctx.r[31].s64 = ctx.r[11].s64 + 1;
	// 82E6D074: 7F29CB78  mr r9, r25
	ctx.r[9].u64 = ctx.r[25].u64;
	// 82E6D078: 7BEB0020  clrldi r11, r31, 0x20
	ctx.r[11].u64 = ctx.r[31].u64 & 0x00000000FFFFFFFFu64;
	// 82E6D07C: 7F48D378  mr r8, r26
	ctx.r[8].u64 = ctx.r[26].u64;
	// 82E6D080: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 82E6D084: C8010050  lfd f0, 0x50(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82E6D088: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 82E6D08C: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82E6D090: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82E6D094: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82E6D098: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E6D09C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E6D0A0: EC20F824  fdivs f1, f0, f31
	ctx.f[1].f64 = ((ctx.f[0].f64 / ctx.f[31].f64) as f32) as f64;
	// 82E6D0A4: 48001B35  bl 0x82e6ebd8
	ctx.lr = 0x82E6D0A8;
	sub_82E6EBD8(ctx, base);
	// 82E6D0A8: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82E6D0AC: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82E6D0B0: 48001879  bl 0x82e6e928
	ctx.lr = 0x82E6D0B4;
	sub_82E6E928(ctx, base);
	// 82E6D0B4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E6D0B8: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82E6D0BC: 91610080  stw r11, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 82E6D0C0: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82E6D0C4: 4B985D95  bl 0x827f2e58
	ctx.lr = 0x82E6D0C8;
	sub_827F2E58(ctx, base);
	// 82E6D0C8: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82E6D0CC: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82E6D0D0: 4198FFA0  blt cr6, 0x82e6d070
	if ctx.cr[6].lt {
	pc = 0x82E6D070; continue 'dispatch;
	}
	// 82E6D0D4: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 82E6D0D8: CBE1FFB0  lfd f31, -0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-80 as u32) ) };
	// 82E6D0DC: 4833B0CC  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6D0E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E6D0E0 size=780
    let mut pc: u32 = 0x82E6D0E0;
    'dispatch: loop {
        match pc {
            0x82E6D0E0 => {
    //   block [0x82E6D0E0..0x82E6D3EC)
	// 82E6D0E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6D0E4: 4833B071  bl 0x831a8154
	ctx.lr = 0x82E6D0E8;
	sub_831A8130(ctx, base);
	// 82E6D0E8: DBE1FFA8  stfd f31, -0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-88 as u32), ctx.f[31].u64 ) };
	// 82E6D0EC: 3980FF80  li r12, -0x80
	ctx.r[12].s64 = -128;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6D3F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E6D3F0 size=464
    let mut pc: u32 = 0x82E6D3F0;
    'dispatch: loop {
        match pc {
            0x82E6D3F0 => {
    //   block [0x82E6D3F0..0x82E6D5C0)
	// 82E6D3F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6D3F4: 4833AD71  bl 0x831a8164
	ctx.lr = 0x82E6D3F8;
	sub_831A8130(ctx, base);
	// 82E6D3F8: DBE1FFC8  stfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[31].u64 ) };
	// 82E6D3FC: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E6D400: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82E6D404: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E6D408: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82E6D40C: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82E6D410: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E6D414: 419A01A0  beq cr6, 0x82e6d5b4
	if ctx.cr[6].eq {
	pc = 0x82E6D5B4; continue 'dispatch;
	}
	// 82E6D418: 4B7C1EC1  bl 0x8262f2d8
	ctx.lr = 0x82E6D41C;
	sub_8262F2D8(ctx, base);
	// 82E6D41C: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 82E6D420: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E6D424: 556A07BD  rlwinm. r10, r11, 0, 0x1e, 0x1e
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E6D428: 4082000C  bne 0x82e6d434
	if !ctx.cr[0].eq {
	pc = 0x82E6D434; continue 'dispatch;
	}
	// 82E6D42C: 556B07FF  clrlwi. r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E6D430: 41820018  beq 0x82e6d448
	if ctx.cr[0].eq {
	pc = 0x82E6D448; continue 'dispatch;
	}
	// 82E6D434: 57AB063F  clrlwi. r11, r29, 0x18
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E6D438: 4182000C  beq 0x82e6d444
	if ctx.cr[0].eq {
	pc = 0x82E6D444; continue 'dispatch;
	}
	// 82E6D43C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E6D440: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82E6D444: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 82E6D448: 57AB063F  clrlwi. r11, r29, 0x18
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E6D44C: 4182001C  beq 0x82e6d468
	if ctx.cr[0].eq {
	pc = 0x82E6D468; continue 'dispatch;
	}
	// 82E6D450: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E6D454: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 82E6D458: 4800B149  bl 0x82e785a0
	ctx.lr = 0x82E6D45C;
	sub_82E785A0(ctx, base);
	// 82E6D45C: 38810090  addi r4, r1, 0x90
	ctx.r[4].s64 = ctx.r[1].s64 + 144;
	// 82E6D460: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E6D464: 4BFFEFC5  bl 0x82e6c428
	ctx.lr = 0x82E6D468;
	sub_82E6C428(ctx, base);
	// 82E6D468: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E6D46C: 556B077B  rlwinm. r11, r11, 0, 0x1d, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E6D470: 41820008  beq 0x82e6d478
	if ctx.cr[0].eq {
	pc = 0x82E6D478; continue 'dispatch;
	}
	// 82E6D474: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82E6D478: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E6D47C: 2B0B1388  cmplwi cr6, r11, 0x1388
	ctx.cr[6].compare_u32(ctx.r[11].u32, 5000 as u32, &mut ctx.xer);
	// 82E6D480: 41990080  bgt cr6, 0x82e6d500
	if ctx.cr[6].gt {
	pc = 0x82E6D500; continue 'dispatch;
	}
	// 82E6D484: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 82E6D488: 48000B71  bl 0x82e6dff8
	ctx.lr = 0x82E6D48C;
	sub_82E6DFF8(ctx, base);
	// 82E6D48C: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82E6D490: 4082FF90  bne 0x82e6d420
	if !ctx.cr[0].eq {
	pc = 0x82E6D420; continue 'dispatch;
	}
	// 82E6D494: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E6D498: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 82E6D49C: 40980070  bge cr6, 0x82e6d50c
	if !ctx.cr[6].lt {
	pc = 0x82E6D50C; continue 'dispatch;
	}
	// 82E6D4A0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82E6D4A4: 4819C035  bl 0x830094d8
	ctx.lr = 0x82E6D4A8;
	sub_830094D8(ctx, base);
	// 82E6D4A8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E6D4AC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E6D4B0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E6D4B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82E6D4B8: C3EB08A4  lfs f31, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82E6D4BC: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82E6D4C0: 4BFFD861  bl 0x82e6ad20
	ctx.lr = 0x82E6D4C4;
	sub_82E6AD20(ctx, base);
	// 82E6D4C4: 3BFE0014  addi r31, r30, 0x14
	ctx.r[31].s64 = ctx.r[30].s64 + 20;
	// 82E6D4C8: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82E6D4CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6D4D0: 4BFFF079  bl 0x82e6c548
	ctx.lr = 0x82E6D4D4;
	sub_82E6C548(ctx, base);
	// 82E6D4D4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82E6D4D8: 4819B951  bl 0x83008e28
	ctx.lr = 0x82E6D4DC;
	sub_83008E28(ctx, base);
	// 82E6D4DC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E6D4E0: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 82E6D4E4: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82E6D4E8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82E6D4EC: 4BFFD835  bl 0x82e6ad20
	ctx.lr = 0x82E6D4F0;
	sub_82E6AD20(ctx, base);
	// 82E6D4F0: 38810090  addi r4, r1, 0x90
	ctx.r[4].s64 = ctx.r[1].s64 + 144;
	// 82E6D4F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6D4F8: 4BFFF051  bl 0x82e6c548
	ctx.lr = 0x82E6D4FC;
	sub_82E6C548(ctx, base);
	// 82E6D4FC: 480000B8  b 0x82e6d5b4
	pc = 0x82E6D5B4; continue 'dispatch;
	// 82E6D500: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E6D504: 4B7C1DD5  bl 0x8262f2d8
	ctx.lr = 0x82E6D508;
	sub_8262F2D8(ctx, base);
	// 82E6D508: 480000AC  b 0x82e6d5b4
	pc = 0x82E6D5B4; continue 'dispatch;
	// 82E6D50C: 37ABFFFF  addic. r29, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82E6D510: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E6D514: 41820058  beq 0x82e6d56c
	if ctx.cr[0].eq {
	pc = 0x82E6D56C; continue 'dispatch;
	}
	// 82E6D518: 815E000C  lwz r10, 0xc(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E6D51C: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6D520: 7D2AFA14  add r9, r10, r31
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 82E6D524: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82E6D528: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82E6D52C: 41990008  bgt cr6, 0x82e6d534
	if ctx.cr[6].gt {
	pc = 0x82E6D534; continue 'dispatch;
	}
	// 82E6D530: 7D2B4850  subf r9, r11, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 82E6D534: 7D4AFA14  add r10, r10, r31
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 82E6D538: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82E6D53C: 41990008  bgt cr6, 0x82e6d544
	if ctx.cr[6].gt {
	pc = 0x82E6D544; continue 'dispatch;
	}
	// 82E6D540: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82E6D544: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E6D548: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82E6D54C: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E6D550: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E6D554: 7CA9582E  lwzx r5, r9, r11
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E6D558: 7C8A582E  lwzx r4, r10, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E6D55C: 4BFFF0FD  bl 0x82e6c658
	ctx.lr = 0x82E6D560;
	sub_82E6C658(ctx, base);
	// 82E6D560: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82E6D564: 7F1FE840  cmplw cr6, r31, r29
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82E6D568: 4198FFB0  blt cr6, 0x82e6d518
	if ctx.cr[6].lt {
	pc = 0x82E6D518; continue 'dispatch;
	}
	// 82E6D56C: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82E6D570: 80DE0038  lwz r6, 0x38(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(56 as u32) ) } as u64;
	// 82E6D574: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E6D578: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E6D57C: 4BFFE805  bl 0x82e6bd80
	ctx.lr = 0x82E6D580;
	sub_82E6BD80(ctx, base);
	// 82E6D580: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82E6D584: 4819BF55  bl 0x830094d8
	ctx.lr = 0x82E6D588;
	sub_830094D8(ctx, base);
	// 82E6D588: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E6D58C: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82E6D590: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E6D594: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E6D598: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E6D59C: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E6D5A0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6D5A4: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82E6D5A8: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82E6D5AC: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82E6D5B0: 4BFFF8C9  bl 0x82e6ce78
	ctx.lr = 0x82E6D5B4;
	sub_82E6CE78(ctx, base);
	// 82E6D5B4: 38210120  addi r1, r1, 0x120
	ctx.r[1].s64 = ctx.r[1].s64 + 288;
	// 82E6D5B8: CBE1FFC8  lfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 82E6D5BC: 4833ABF8  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6D5C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E6D5C0 size=148
    let mut pc: u32 = 0x82E6D5C0;
    'dispatch: loop {
        match pc {
            0x82E6D5C0 => {
    //   block [0x82E6D5C0..0x82E6D654)
	// 82E6D5C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6D5C4: 4833ABA9  bl 0x831a816c
	ctx.lr = 0x82E6D5C8;
	sub_831A8130(ctx, base);
	// 82E6D5C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E6D5CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E6D5D0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E6D5D4: 3BBF0024  addi r29, r31, 0x24
	ctx.r[29].s64 = ctx.r[31].s64 + 36;
	// 82E6D5D8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E6D5DC: 4B98517D  bl 0x827f2758
	ctx.lr = 0x82E6D5E0;
	sub_827F2758(ctx, base);
	// 82E6D5E0: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E6D5E4: 389F0014  addi r4, r31, 0x14
	ctx.r[4].s64 = ctx.r[31].s64 + 20;
	// 82E6D5E8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E6D5EC: 409A000C  bne cr6, 0x82e6d5f8
	if !ctx.cr[6].eq {
	pc = 0x82E6D5F8; continue 'dispatch;
	}
	// 82E6D5F0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82E6D5F4: 48000014  b 0x82e6d608
	pc = 0x82E6D608; continue 'dispatch;
	// 82E6D5F8: 81440008  lwz r10, 8(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6D5FC: 39200030  li r9, 0x30
	ctx.r[9].s64 = 48;
	// 82E6D600: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82E6D604: 7CAB4BD6  divw r5, r11, r9
	ctx.r[5].s32 = ctx.r[11].s32 / ctx.r[9].s32;
	// 82E6D608: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82E6D60C: 4198001C  blt cr6, 0x82e6d628
	if ctx.cr[6].lt {
	pc = 0x82E6D628; continue 'dispatch;
	}
	// 82E6D610: 2B050002  cmplwi cr6, r5, 2
	ctx.cr[6].compare_u32(ctx.r[5].u32, 2 as u32, &mut ctx.xer);
	// 82E6D614: 40990014  ble cr6, 0x82e6d628
	if !ctx.cr[6].gt {
	pc = 0x82E6D628; continue 'dispatch;
	}
	// 82E6D618: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82E6D61C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6D620: 4BFFFAC1  bl 0x82e6d0e0
	ctx.lr = 0x82E6D624;
	sub_82E6D0E0(ctx, base);
	// 82E6D624: 48000028  b 0x82e6d64c
	pc = 0x82E6D64C; continue 'dispatch;
	// 82E6D628: 83DF0018  lwz r30, 0x18(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E6D62C: 48000014  b 0x82e6d640
	pc = 0x82E6D640; continue 'dispatch;
	// 82E6D630: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E6D634: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E6D638: 4B985821  bl 0x827f2e58
	ctx.lr = 0x82E6D63C;
	sub_827F2E58(ctx, base);
	// 82E6D63C: 3BDE0030  addi r30, r30, 0x30
	ctx.r[30].s64 = ctx.r[30].s64 + 48;
	// 82E6D640: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E6D644: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E6D648: 409AFFE8  bne cr6, 0x82e6d630
	if !ctx.cr[6].eq {
	pc = 0x82E6D630; continue 'dispatch;
	}
	// 82E6D64C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E6D650: 4833AB6C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6D658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E6D658 size=8
    let mut pc: u32 = 0x82E6D658;
    'dispatch: loop {
        match pc {
            0x82E6D658 => {
    //   block [0x82E6D658..0x82E6D660)
	// 82E6D658: 38630014  addi r3, r3, 0x14
	ctx.r[3].s64 = ctx.r[3].s64 + 20;
	// 82E6D65C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6D660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E6D660 size=64
    let mut pc: u32 = 0x82E6D660;
    'dispatch: loop {
        match pc {
            0x82E6D660 => {
    //   block [0x82E6D660..0x82E6D6A0)
	// 82E6D660: 81240000  lwz r9, 0(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E6D664: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82E6D668: 81040004  lwz r8, 4(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E6D66C: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6D670: 90A30020  stw r5, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[5].u32 ) };
	// 82E6D674: 99430024  stb r10, 0x24(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[10].u8 ) };
	// 82E6D678: 9121FFE0  stw r9, -0x20(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.r[9].u32 ) };
	// 82E6D67C: C1A1FFE0  lfs f13, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E6D680: 9101FFE4  stw r8, -0x1c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-28 as u32), ctx.r[8].u32 ) };
	// 82E6D684: C181FFE4  lfs f12, -0x1c(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-28 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E6D688: 9161FFE8  stw r11, -0x18(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[11].u32 ) };
	// 82E6D68C: C001FFE8  lfs f0, -0x18(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E6D690: D003001C  stfs f0, 0x1c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 82E6D694: D1830018  stfs f12, 0x18(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82E6D698: D1A30014  stfs f13, 0x14(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82E6D69C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6D6A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E6D6A0 size=76
    let mut pc: u32 = 0x82E6D6A0;
    'dispatch: loop {
        match pc {
            0x82E6D6A0 => {
    //   block [0x82E6D6A0..0x82E6D6EC)
	// 82E6D6A0: 8103002C  lwz r8, 0x2c(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82E6D6A4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E6D6A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E6D6AC: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82E6D6B0: 419A004C  beq cr6, 0x82e6d6fc
	if ctx.cr[6].eq {
		sub_82E6D6FC(ctx, base);
		return;
	}
	// 82E6D6B4: 81230030  lwz r9, 0x30(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E6D6B8: 38E00014  li r7, 0x14
	ctx.r[7].s64 = 20;
	// 82E6D6BC: 7D284850  subf r9, r8, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[8].s64;
	// 82E6D6C0: 7D293BD6  divw r9, r9, r7
	ctx.r[9].s32 = ctx.r[9].s32 / ctx.r[7].s32;
	// 82E6D6C4: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82E6D6C8: 40980034  bge cr6, 0x82e6d6fc
	if !ctx.cr[6].lt {
		sub_82E6D6FC(ctx, base);
		return;
	}
	// 82E6D6CC: 8123002C  lwz r9, 0x2c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82E6D6D0: 7D295214  add r9, r9, r10
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82E6D6D4: 89290010  lbz r9, 0x10(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E6D6D8: 2B090001  cmplwi cr6, r9, 1
	ctx.cr[6].compare_u32(ctx.r[9].u32, 1 as u32, &mut ctx.xer);
	// 82E6D6DC: 419A0010  beq cr6, 0x82e6d6ec
	if ctx.cr[6].eq {
		sub_82E6D6EC(ctx, base);
		return;
	}
	// 82E6D6E0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E6D6E4: 394A0014  addi r10, r10, 0x14
	ctx.r[10].s64 = ctx.r[10].s64 + 20;
	// 82E6D6E8: 4BFFFFC4  b 0x82e6d6ac
	pc = 0x82E6D6AC; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6D6EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E6D6EC size=16
    let mut pc: u32 = 0x82E6D6EC;
    'dispatch: loop {
        match pc {
            0x82E6D6EC => {
    //   block [0x82E6D6EC..0x82E6D6FC)
	// 82E6D6EC: 8143002C  lwz r10, 0x2c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82E6D6F0: 1D6B0014  mulli r11, r11, 0x14
	ctx.r[11].s64 = ctx.r[11].s64 * 20;
	// 82E6D6F4: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82E6D6F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6D6FC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E6D6FC size=8
    let mut pc: u32 = 0x82E6D6FC;
    'dispatch: loop {
        match pc {
            0x82E6D6FC => {
    //   block [0x82E6D6FC..0x82E6D704)
	// 82E6D6FC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E6D700: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6D708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E6D708 size=56
    let mut pc: u32 = 0x82E6D708;
    'dispatch: loop {
        match pc {
            0x82E6D708 => {
    //   block [0x82E6D708..0x82E6D740)
	// 82E6D708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6D70C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E6D710: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E6D714: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82E6D718: 90C60038  stw r6, 0x38(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(56 as u32), ctx.r[6].u32 ) };
	// 82E6D71C: 4BFFFF85  bl 0x82e6d6a0
	ctx.lr = 0x82E6D720;
	sub_82E6D6A0(ctx, base);
	// 82E6D720: 7C6B1B79  or. r11, r3, r3
	ctx.r[11].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E6D724: 40820008  bne 0x82e6d72c
	if !ctx.cr[0].eq {
	pc = 0x82E6D72C; continue 'dispatch;
	}
	// 82E6D728: 39660014  addi r11, r6, 0x14
	ctx.r[11].s64 = ctx.r[6].s64 + 20;
	// 82E6D72C: 9166003C  stw r11, 0x3c(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 82E6D730: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E6D734: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E6D738: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E6D73C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6D740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E6D740 size=64
    let mut pc: u32 = 0x82E6D740;
    'dispatch: loop {
        match pc {
            0x82E6D740 => {
    //   block [0x82E6D740..0x82E6D780)
	// 82E6D740: 81240000  lwz r9, 0(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E6D744: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82E6D748: 81040004  lwz r8, 4(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E6D74C: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6D750: 90A3000C  stw r5, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[5].u32 ) };
	// 82E6D754: 99430010  stb r10, 0x10(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u8 ) };
	// 82E6D758: 9121FFE0  stw r9, -0x20(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.r[9].u32 ) };
	// 82E6D75C: C1A1FFE0  lfs f13, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E6D760: 9101FFE4  stw r8, -0x1c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-28 as u32), ctx.r[8].u32 ) };
	// 82E6D764: C181FFE4  lfs f12, -0x1c(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-28 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E6D768: 9161FFE8  stw r11, -0x18(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[11].u32 ) };
	// 82E6D76C: C001FFE8  lfs f0, -0x18(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E6D770: D0030008  stfs f0, 8(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82E6D774: D1830004  stfs f12, 4(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82E6D778: D1A30000  stfs f13, 0(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82E6D77C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6D780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E6D780 size=16
    let mut pc: u32 = 0x82E6D780;
    'dispatch: loop {
        match pc {
            0x82E6D780 => {
    //   block [0x82E6D780..0x82E6D790)
	// 82E6D780: 81430030  lwz r10, 0x30(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E6D784: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82E6D788: 8163002C  lwz r11, 0x2c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82E6D78C: 4800002C  b 0x82e6d7b8
	sub_82E6D790(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6D790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E6D790 size=76
    let mut pc: u32 = 0x82E6D790;
    'dispatch: loop {
        match pc {
            0x82E6D790 => {
    //   block [0x82E6D790..0x82E6D7DC)
	// 82E6D790: 890B0010  lbz r8, 0x10(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E6D794: 28080000  cmplwi r8, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E6D798: 4182001C  beq 0x82e6d7b4
	if ctx.cr[0].eq {
	pc = 0x82E6D7B4; continue 'dispatch;
	}
	// 82E6D79C: 5528063E  clrlwi r8, r9, 0x18
	ctx.r[8].u64 = ctx.r[9].u32 as u64 & 0x000000FFu64;
	// 82E6D7A0: 2B080001  cmplwi cr6, r8, 1
	ctx.cr[6].compare_u32(ctx.r[8].u32, 1 as u32, &mut ctx.xer);
	// 82E6D7A4: 419A0030  beq cr6, 0x82e6d7d4
	if ctx.cr[6].eq {
	pc = 0x82E6D7D4; continue 'dispatch;
	}
	// 82E6D7A8: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E6D7AC: 409A0008  bne cr6, 0x82e6d7b4
	if !ctx.cr[6].eq {
	pc = 0x82E6D7B4; continue 'dispatch;
	}
	// 82E6D7B0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82E6D7B4: 396B0014  addi r11, r11, 0x14
	ctx.r[11].s64 = ctx.r[11].s64 + 20;
	// 82E6D7B8: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82E6D7BC: 409AFFD4  bne cr6, 0x82e6d790
	if !ctx.cr[6].eq {
	pc = 0x82E6D790; continue 'dispatch;
	}
	// 82E6D7C0: 8143003C  lwz r10, 0x3c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(60 as u32) ) } as u64;
	// 82E6D7C4: 39630014  addi r11, r3, 0x14
	ctx.r[11].s64 = ctx.r[3].s64 + 20;
	// 82E6D7C8: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E6D7CC: 409A0008  bne cr6, 0x82e6d7d4
	if !ctx.cr[6].eq {
	pc = 0x82E6D7D4; continue 'dispatch;
	}
	// 82E6D7D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E6D7D4: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82E6D7D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6D7E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E6D7E0 size=48
    let mut pc: u32 = 0x82E6D7E0;
    'dispatch: loop {
        match pc {
            0x82E6D7E0 => {
    //   block [0x82E6D7E0..0x82E6D810)
	// 82E6D7E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6D7E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E6D7E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E6D7EC: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 82E6D7F0: 8087003C  lwz r4, 0x3c(r7)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(60 as u32) ) } as u64;
	// 82E6D7F4: 90870038  stw r4, 0x38(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(56 as u32), ctx.r[4].u32 ) };
	// 82E6D7F8: 4BFFFF89  bl 0x82e6d780
	ctx.lr = 0x82E6D7FC;
	sub_82E6D780(ctx, base);
	// 82E6D7FC: 9067003C  stw r3, 0x3c(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(60 as u32), ctx.r[3].u32 ) };
	// 82E6D800: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E6D804: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E6D808: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E6D80C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6D810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E6D810 size=76
    let mut pc: u32 = 0x82E6D810;
    'dispatch: loop {
        match pc {
            0x82E6D810 => {
    //   block [0x82E6D810..0x82E6D85C)
	// 82E6D810: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82E6D814: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E6D818: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82E6D81C: 99630010  stb r11, 0x10(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u8 ) };
	// 82E6D820: C00A08A4  lfs f0, 0x8a4(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E6D824: D0030000  stfs f0, 0(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82E6D828: D0030004  stfs f0, 4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82E6D82C: D0030008  stfs f0, 8(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82E6D830: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82E6D834: D0030014  stfs f0, 0x14(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82E6D838: 99630024  stb r11, 0x24(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u8 ) };
	// 82E6D83C: D0030018  stfs f0, 0x18(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82E6D840: D003001C  stfs f0, 0x1c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 82E6D844: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82E6D848: 91630030  stw r11, 0x30(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 82E6D84C: 91630034  stw r11, 0x34(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 82E6D850: 91630038  stw r11, 0x38(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82E6D854: 9163003C  stw r11, 0x3c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 82E6D858: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6D860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E6D860 size=92
    let mut pc: u32 = 0x82E6D860;
    'dispatch: loop {
        match pc {
            0x82E6D860 => {
    //   block [0x82E6D860..0x82E6D8BC)
	// 82E6D860: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6D864: 4833A909  bl 0x831a816c
	ctx.lr = 0x82E6D868;
	sub_831A8130(ctx, base);
	// 82E6D868: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E6D86C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82E6D870: 81440000  lwz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E6D874: 81240004  lwz r9, 4(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E6D878: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E6D87C: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82E6D880: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E6D884: 7FEA5A14  add r31, r10, r11
	ctx.r[31].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82E6D888: 7FA95A14  add r29, r9, r11
	ctx.r[29].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82E6D88C: 48000020  b 0x82e6d8ac
	pc = 0x82E6D8AC; continue 'dispatch;
	// 82E6D890: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E6D894: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E6D898: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E6D89C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E6D8A0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E6D8A4: 4E800421  bctrl
	ctx.lr = 0x82E6D8A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E6D8A8: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82E6D8AC: 7F1FE840  cmplw cr6, r31, r29
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82E6D8B0: 409AFFE0  bne cr6, 0x82e6d890
	if !ctx.cr[6].eq {
	pc = 0x82E6D890; continue 'dispatch;
	}
	// 82E6D8B4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E6D8B8: 4833A904  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6D8C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E6D8C0 size=152
    let mut pc: u32 = 0x82E6D8C0;
    'dispatch: loop {
        match pc {
            0x82E6D8C0 => {
    //   block [0x82E6D8C0..0x82E6D958)
	// 82E6D8C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6D8C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E6D8C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E6D8CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E6D8D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E6D8D4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E6D8D8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E6D8DC: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82E6D8E0: 409A0030  bne cr6, 0x82e6d910
	if !ctx.cr[6].eq {
	pc = 0x82E6D910; continue 'dispatch;
	}
	// 82E6D8E4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E6D8E8: 419A0058  beq cr6, 0x82e6d940
	if ctx.cr[6].eq {
	pc = 0x82E6D940; continue 'dispatch;
	}
	// 82E6D8EC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E6D8F0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E6D8F4: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E6D8F8: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E6D8FC: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6D900: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E6D904: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E6D908: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82E6D90C: 48000034  b 0x82e6d940
	pc = 0x82E6D940; continue 'dispatch;
	// 82E6D910: 2F050001  cmpwi cr6, r5, 1
	ctx.cr[6].compare_i32(ctx.r[5].s32, 1, &mut ctx.xer);
	// 82E6D914: 419A002C  beq cr6, 0x82e6d940
	if ctx.cr[6].eq {
	pc = 0x82E6D940; continue 'dispatch;
	}
	// 82E6D918: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E6D91C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E6D920: 388BB878  addi r4, r11, -0x4788
	ctx.r[4].s64 = ctx.r[11].s64 + -18312;
	// 82E6D924: 4833A7D5  bl 0x831a80f8
	ctx.lr = 0x82E6D928;
	sub_831A80F8(ctx, base);
	// 82E6D928: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E6D92C: 4182000C  beq 0x82e6d938
	if ctx.cr[0].eq {
	pc = 0x82E6D938; continue 'dispatch;
	}
	// 82E6D930: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82E6D934: 4800000C  b 0x82e6d940
	pc = 0x82E6D940; continue 'dispatch;
	// 82E6D938: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E6D93C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E6D940: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E6D944: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E6D948: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E6D94C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E6D950: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E6D954: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6D958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E6D958 size=68
    let mut pc: u32 = 0x82E6D958;
    'dispatch: loop {
        match pc {
            0x82E6D958 => {
    //   block [0x82E6D958..0x82E6D99C)
	// 82E6D958: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6D95C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E6D960: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E6D964: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E6D968: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E6D96C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E6D970: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E6D974: 396BD2C4  addi r11, r11, -0x2d3c
	ctx.r[11].s64 = ctx.r[11].s64 + -11580;
	// 82E6D978: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E6D97C: 41820008  beq 0x82e6d984
	if ctx.cr[0].eq {
	pc = 0x82E6D984; continue 'dispatch;
	}
	// 82E6D980: 4B4528E9  bl 0x822c0268
	ctx.lr = 0x82E6D984;
	sub_822C0268(ctx, base);
	// 82E6D984: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6D988: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E6D98C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E6D990: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E6D994: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E6D998: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6D9A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E6D9A0 size=72
    let mut pc: u32 = 0x82E6D9A0;
    'dispatch: loop {
        match pc {
            0x82E6D9A0 => {
    //   block [0x82E6D9A0..0x82E6D9E8)
	// 82E6D9A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6D9A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E6D9A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E6D9AC: 2F050003  cmpwi cr6, r5, 3
	ctx.cr[6].compare_i32(ctx.r[5].s32, 3, &mut ctx.xer);
	// 82E6D9B0: 419A001C  beq cr6, 0x82e6d9cc
	if ctx.cr[6].eq {
	pc = 0x82E6D9CC; continue 'dispatch;
	}
	// 82E6D9B4: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82E6D9B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E6D9BC: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82E6D9C0: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E6D9C4: 4BFFFEFD  bl 0x82e6d8c0
	ctx.lr = 0x82E6D9C8;
	sub_82E6D8C0(ctx, base);
	// 82E6D9C8: 48000010  b 0x82e6d9d8
	pc = 0x82E6D9D8; continue 'dispatch;
	// 82E6D9CC: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E6D9D0: 396BB878  addi r11, r11, -0x4788
	ctx.r[11].s64 = ctx.r[11].s64 + -18312;
	// 82E6D9D4: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E6D9D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E6D9DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E6D9E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E6D9E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6D9E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E6D9E8 size=364
    let mut pc: u32 = 0x82E6D9E8;
    'dispatch: loop {
        match pc {
            0x82E6D9E8 => {
    //   block [0x82E6D9E8..0x82E6DB54)
	// 82E6D9E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6D9EC: 4833A771  bl 0x831a815c
	ctx.lr = 0x82E6D9F0;
	sub_831A8130(ctx, base);
	// 82E6D9F0: DBE1FFB8  stfd f31, -0x48(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-72 as u32), ctx.f[31].u64 ) };
	// 82E6D9F4: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E6D9F8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82E6D9FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E6DA00: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82E6DA04: 3B3F0004  addi r25, r31, 4
	ctx.r[25].s64 = ctx.r[31].s64 + 4;
	// 82E6DA08: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82E6DA0C: C3EB952C  lfs f31, -0x6ad4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27348 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82E6DA10: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82E6DA14: 81790004  lwz r11, 4(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E6DA18: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E6DA1C: 419A012C  beq cr6, 0x82e6db48
	if ctx.cr[6].eq {
	pc = 0x82E6DB48; continue 'dispatch;
	}
	// 82E6DA20: 81590008  lwz r10, 8(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6DA24: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82E6DA28: 7D6B2670  srawi r11, r11, 4
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 4) as i64;
	// 82E6DA2C: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E6DA30: 40980118  bge cr6, 0x82e6db48
	if !ctx.cr[6].lt {
	pc = 0x82E6DB48; continue 'dispatch;
	}
	// 82E6DA34: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6DA38: 7D6BDA14  add r11, r11, r27
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 82E6DA3C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E6DA40: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E6DA44: 419A00F8  beq cr6, 0x82e6db3c
	if ctx.cr[6].eq {
	pc = 0x82E6DB3C; continue 'dispatch;
	}
	// 82E6DA48: 812B0008  lwz r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6DA4C: 7D4A4850  subf r10, r10, r9
	ctx.r[10].s64 = ctx.r[9].s64 - ctx.r[10].s64;
	// 82E6DA50: 7D4A1671  srawi. r10, r10, 2
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 2) as i64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E6DA54: 418200E8  beq 0x82e6db3c
	if ctx.cr[0].eq {
	pc = 0x82E6DB3C; continue 'dispatch;
	}
	// 82E6DA58: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E6DA5C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E6DA60: 419A0010  beq cr6, 0x82e6da70
	if ctx.cr[6].eq {
	pc = 0x82E6DA70; continue 'dispatch;
	}
	// 82E6DA64: 812B0008  lwz r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6DA68: 7D4A4850  subf r10, r10, r9
	ctx.r[10].s64 = ctx.r[9].s64 - ctx.r[10].s64;
	// 82E6DA6C: 7D4A1670  srawi r10, r10, 2
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 2) as i64;
	// 82E6DA70: 79490020  clrldi r9, r10, 0x20
	ctx.r[9].u64 = ctx.r[10].u64 & 0x00000000FFFFFFFFu64;
	// 82E6DA74: 810B0004  lwz r8, 4(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E6DA78: 935F0048  stw r26, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[26].u32 ) };
	// 82E6DA7C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E6DA80: F9210050  std r9, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u64 ) };
	// 82E6DA84: C8010050  lfd f0, 0x50(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82E6DA88: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 82E6DA8C: 397F0018  addi r11, r31, 0x18
	ctx.r[11].s64 = ctx.r[31].s64 + 24;
	// 82E6DA90: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82E6DA94: 911F0044  stw r8, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[8].u32 ) };
	// 82E6DA98: EC0007F2  fmuls f0, f0, f31
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 82E6DA9C: 392A0001  addi r9, r10, 1
	ctx.r[9].s64 = ctx.r[10].s64 + 1;
	// 82E6DAA0: 7D4A07B4  extsw r10, r10
	ctx.r[10].s64 = ctx.r[10].s32 as i64;
	// 82E6DAA4: 7D2807B4  extsw r8, r9
	ctx.r[8].s64 = ctx.r[9].s32 as i64;
	// 82E6DAA8: F9410058  std r10, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u64 ) };
	// 82E6DAAC: C9A10058  lfd f13, 0x58(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 82E6DAB0: F9010060  std r8, 0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[8].u64 ) };
	// 82E6DAB4: C9810060  lfd f12, 0x60(r1)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 82E6DAB8: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 82E6DABC: 3900FFFC  li r8, -4
	ctx.r[8].s64 = -4;
	// 82E6DAC0: FD80669C  fcfid f12, f12
	ctx.f[12].f64 = (ctx.f[12].s64 as f64);
	// 82E6DAC4: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 82E6DAC8: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 82E6DACC: 2F090006  cmpwi cr6, r9, 6
	ctx.cr[6].compare_i32(ctx.r[9].s32, 6, &mut ctx.xer);
	// 82E6DAD0: FD806018  frsp f12, f12
	ctx.f[12].f64 = (ctx.f[12].f64 as f32) as f64;
	// 82E6DAD4: EDAD0032  fmuls f13, f13, f0
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82E6DAD8: ED8C0032  fmuls f12, f12, f0
	ctx.f[12].f64 = (((ctx.f[12].f64 * ctx.f[0].f64) as f32) as f64);
	// 82E6DADC: FDA06E5E  fctidz f13, f13
	ctx.f[13].s64 = if ctx.f[13].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[13].f64.trunc() as i64 };
	// 82E6DAE0: 7DAB47AE  stfiwx f13, r11, r8
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32), tmp.u32) };
	// 82E6DAE4: FDA0665E  fctidz f13, f12
	ctx.f[13].s64 = if ctx.f[12].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[12].f64.trunc() as i64 };
	// 82E6DAE8: 7DA05FAE  stfiwx f13, 0, r11
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32, tmp.u32) };
	// 82E6DAEC: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82E6DAF0: 4198FFAC  blt cr6, 0x82e6da9c
	if ctx.cr[6].lt {
	pc = 0x82E6DA9C; continue 'dispatch;
	}
	// 82E6DAF4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82E6DAF8: 4BFF7BF1  bl 0x82e656e8
	ctx.lr = 0x82E6DAFC;
	sub_82E656E8(ctx, base);
	// 82E6DAFC: 3BC00005  li r30, 5
	ctx.r[30].s64 = 5;
	// 82E6DB00: 3BBF0074  addi r29, r31, 0x74
	ctx.r[29].s64 = ctx.r[31].s64 + 116;
	// 82E6DB04: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E6DB08: 4BF8BE69  bl 0x82df9970
	ctx.lr = 0x82E6DB0C;
	sub_82DF9970(ctx, base);
	// 82E6DB0C: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82E6DB10: 3BBDFFF8  addi r29, r29, -8
	ctx.r[29].s64 = ctx.r[29].s64 + -8;
	// 82E6DB14: 4080FFF0  bge 0x82e6db04
	if !ctx.cr[0].lt {
	pc = 0x82E6DB04; continue 'dispatch;
	}
	// 82E6DB18: 3BBF004C  addi r29, r31, 0x4c
	ctx.r[29].s64 = ctx.r[31].s64 + 76;
	// 82E6DB1C: 3BC00006  li r30, 6
	ctx.r[30].s64 = 6;
	// 82E6DB20: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E6DB24: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E6DB28: 4BF8BE99  bl 0x82df99c0
	ctx.lr = 0x82E6DB2C;
	sub_82DF99C0(ctx, base);
	// 82E6DB2C: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82E6DB30: 3BBD0008  addi r29, r29, 8
	ctx.r[29].s64 = ctx.r[29].s64 + 8;
	// 82E6DB34: 4082FFEC  bne 0x82e6db20
	if !ctx.cr[0].eq {
	pc = 0x82E6DB20; continue 'dispatch;
	}
	// 82E6DB38: 4BFF7C01  bl 0x82e65738
	ctx.lr = 0x82E6DB3C;
	sub_82E65738(ctx, base);
	// 82E6DB3C: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82E6DB40: 3B7B0010  addi r27, r27, 0x10
	ctx.r[27].s64 = ctx.r[27].s64 + 16;
	// 82E6DB44: 4BFFFED0  b 0x82e6da14
	pc = 0x82E6DA14; continue 'dispatch;
	// 82E6DB48: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82E6DB4C: CBE1FFB8  lfd f31, -0x48(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-72 as u32) ) };
	// 82E6DB50: 4833A65C  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6DB58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E6DB58 size=28
    let mut pc: u32 = 0x82E6DB58;
    'dispatch: loop {
        match pc {
            0x82E6DB58 => {
    //   block [0x82E6DB58..0x82E6DB74)
	// 82E6DB58: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E6DB5C: 80AB000C  lwz r5, 0xc(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E6DB60: 808B0008  lwz r4, 8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6DB64: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E6DB68: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E6DB6C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E6DB70: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6DB78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E6DB78 size=128
    let mut pc: u32 = 0x82E6DB78;
    'dispatch: loop {
        match pc {
            0x82E6DB78 => {
    //   block [0x82E6DB78..0x82E6DBF8)
	// 82E6DB78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6DB7C: 4833A5F1  bl 0x831a816c
	ctx.lr = 0x82E6DB80;
	sub_831A8130(ctx, base);
	// 82E6DB80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E6DB84: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 82E6DB88: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82E6DB8C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E6DB90: 3BEB6870  addi r31, r11, 0x6870
	ctx.r[31].s64 = ctx.r[11].s64 + 26736;
	// 82E6DB94: 816A6878  lwz r11, 0x6878(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(26744 as u32) ) } as u64;
	// 82E6DB98: 556907FF  clrlwi. r9, r11, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E6DB9C: 40820024  bne 0x82e6dbc0
	if !ctx.cr[0].eq {
	pc = 0x82E6DBC0; continue 'dispatch;
	}
	// 82E6DBA0: 3D2082E7  lis r9, -0x7d19
	ctx.r[9].s64 = -2098790400;
	// 82E6DBA4: 3D0082E7  lis r8, -0x7d19
	ctx.r[8].s64 = -2098790400;
	// 82E6DBA8: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 82E6DBAC: 3929DB58  addi r9, r9, -0x24a8
	ctx.r[9].s64 = ctx.r[9].s64 + -9384;
	// 82E6DBB0: 3908D9A0  addi r8, r8, -0x2660
	ctx.r[8].s64 = ctx.r[8].s64 + -9824;
	// 82E6DBB4: 916A6878  stw r11, 0x6878(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(26744 as u32), ctx.r[11].u32 ) };
	// 82E6DBB8: 913F0004  stw r9, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82E6DBBC: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82E6DBC0: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82E6DBC4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82E6DBC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6DBCC: 38DE0008  addi r6, r30, 8
	ctx.r[6].s64 = ctx.r[30].s64 + 8;
	// 82E6DBD0: 9BAB0000  stb r29, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u8 ) };
	// 82E6DBD4: 88E10050  lbz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E6DBD8: 4BFC25B9  bl 0x82e30190
	ctx.lr = 0x82E6DBDC;
	sub_82E30190(ctx, base);
	// 82E6DBDC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E6DBE0: 4182000C  beq 0x82e6dbec
	if ctx.cr[0].eq {
	pc = 0x82E6DBEC; continue 'dispatch;
	}
	// 82E6DBE4: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82E6DBE8: 48000008  b 0x82e6dbf0
	pc = 0x82E6DBF0; continue 'dispatch;
	// 82E6DBEC: 93BE0000  stw r29, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82E6DBF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E6DBF4: 4833A5C8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6DBF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E6DBF8 size=96
    let mut pc: u32 = 0x82E6DBF8;
    'dispatch: loop {
        match pc {
            0x82E6DBF8 => {
    //   block [0x82E6DBF8..0x82E6DC58)
	// 82E6DBF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6DBFC: 4833A571  bl 0x831a816c
	ctx.lr = 0x82E6DC00;
	sub_831A8130(ctx, base);
	// 82E6DC00: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E6DC04: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E6DC08: 83DF0004  lwz r30, 4(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E6DC0C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82E6DC10: 419A0030  beq cr6, 0x82e6dc40
	if ctx.cr[6].eq {
	pc = 0x82E6DC40; continue 'dispatch;
	}
	// 82E6DC14: 83BF0008  lwz r29, 8(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6DC18: 48000010  b 0x82e6dc28
	pc = 0x82E6DC28; continue 'dispatch;
	// 82E6DC1C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E6DC20: 4BF914B9  bl 0x82dff0d8
	ctx.lr = 0x82E6DC24;
	sub_82DFF0D8(ctx, base);
	// 82E6DC24: 3BDE0010  addi r30, r30, 0x10
	ctx.r[30].s64 = ctx.r[30].s64 + 16;
	// 82E6DC28: 7F1EE840  cmplw cr6, r30, r29
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82E6DC2C: 409AFFF0  bne cr6, 0x82e6dc1c
	if !ctx.cr[6].eq {
	pc = 0x82E6DC1C; continue 'dispatch;
	}
	// 82E6DC30: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82E6DC34: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E6DC38: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82E6DC3C: 4BF8454D  bl 0x82df2188
	ctx.lr = 0x82E6DC40;
	sub_82DF2188(ctx, base);
	// 82E6DC40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E6DC44: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E6DC48: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E6DC4C: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82E6DC50: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E6DC54: 4833A568  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6DC58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E6DC58 size=104
    let mut pc: u32 = 0x82E6DC58;
    'dispatch: loop {
        match pc {
            0x82E6DC58 => {
    //   block [0x82E6DC58..0x82E6DCC0)
	// 82E6DC58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6DC5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E6DC60: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E6DC64: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E6DC68: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E6DC6C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E6DC70: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E6DC74: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82E6DC78: 396BD2CC  addi r11, r11, -0x2d34
	ctx.r[11].s64 = ctx.r[11].s64 + -11572;
	// 82E6DC7C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E6DC80: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E6DC84: 4BFFFF75  bl 0x82e6dbf8
	ctx.lr = 0x82E6DC88;
	sub_82E6DBF8(ctx, base);
	// 82E6DC88: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E6DC8C: 57CA07FF  clrlwi. r10, r30, 0x1f
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E6DC90: 396BD2C4  addi r11, r11, -0x2d3c
	ctx.r[11].s64 = ctx.r[11].s64 + -11580;
	// 82E6DC94: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E6DC98: 4182000C  beq 0x82e6dca4
	if ctx.cr[0].eq {
	pc = 0x82E6DCA4; continue 'dispatch;
	}
	// 82E6DC9C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6DCA0: 4BF84739  bl 0x82df23d8
	ctx.lr = 0x82E6DCA4;
	sub_82DF23D8(ctx, base);
	// 82E6DCA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6DCA8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E6DCAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E6DCB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E6DCB4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E6DCB8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E6DCBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6DCC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E6DCC0 size=348
    let mut pc: u32 = 0x82E6DCC0;
    'dispatch: loop {
        match pc {
            0x82E6DCC0 => {
    //   block [0x82E6DCC0..0x82E6DE1C)
	// 82E6DCC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6DCC4: 4833A48D  bl 0x831a8150
	ctx.lr = 0x82E6DCC8;
	sub_831A8130(ctx, base);
	// 82E6DCC8: 9421FE10  stwu r1, -0x1f0(r1)
	ea = ctx.r[1].u32.wrapping_add(-496 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E6DCCC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E6DCD0: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E6DCD4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82E6DCD8: 396BD34C  addi r11, r11, -0x2cb4
	ctx.r[11].s64 = ctx.r[11].s64 + -11444;
	// 82E6DCDC: 395F004C  addi r10, r31, 0x4c
	ctx.r[10].s64 = ctx.r[31].s64 + 76;
	// 82E6DCE0: 7FDAF378  mr r26, r30
	ctx.r[26].u64 = ctx.r[30].u64;
	// 82E6DCE4: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82E6DCE8: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 82E6DCEC: 7C962378  mr r22, r4
	ctx.r[22].u64 = ctx.r[4].u64;
	// 82E6DCF0: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 82E6DCF4: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 82E6DCF8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E6DCFC: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82E6DD00: 93410050  stw r26, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[26].u32 ) };
	// 82E6DD04: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82E6DD08: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82E6DD0C: 3529FFFF  addic. r9, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E6DD10: 93CB0004  stw r30, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82E6DD14: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82E6DD18: 4080FFF0  bge 0x82e6dd08
	if !ctx.cr[0].lt {
	pc = 0x82E6DD08; continue 'dispatch;
	}
	// 82E6DD1C: 7D5B5378  mr r27, r10
	ctx.r[27].u64 = ctx.r[10].u64;
	// 82E6DD20: 93DF007C  stw r30, 0x7c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(124 as u32), ctx.r[30].u32 ) };
	// 82E6DD24: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E6DD28: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82E6DD2C: 7FDCF378  mr r28, r30
	ctx.r[28].u64 = ctx.r[30].u64;
	// 82E6DD30: 3B0BD2F8  addi r24, r11, -0x2d08
	ctx.r[24].s64 = ctx.r[11].s64 + -11528;
	// 82E6DD34: 3AEAD2DC  addi r23, r10, -0x2d24
	ctx.r[23].s64 = ctx.r[10].s64 + -11556;
	// 82E6DD38: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 82E6DD3C: 4BF85475  bl 0x82df31b0
	ctx.lr = 0x82E6DD40;
	sub_82DF31B0(ctx, base);
	// 82E6DD40: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82E6DD44: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 82E6DD48: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 82E6DD4C: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82E6DD50: 4833AD89  bl 0x831a8ad8
	ctx.lr = 0x82E6DD54;
	sub_831A8AD8(ctx, base);
	// 82E6DD54: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82E6DD58: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E6DD5C: 38A0002C  li r5, 0x2c
	ctx.r[5].s64 = 44;
	// 82E6DD60: 38600040  li r3, 0x40
	ctx.r[3].s64 = 64;
	// 82E6DD64: 4BF84685  bl 0x82df23e8
	ctx.lr = 0x82E6DD68;
	sub_82DF23E8(ctx, base);
	// 82E6DD68: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82E6DD6C: 41820068  beq 0x82e6ddd4
	if ctx.cr[0].eq {
	pc = 0x82E6DDD4; continue 'dispatch;
	}
	// 82E6DD70: 38810090  addi r4, r1, 0x90
	ctx.r[4].s64 = ctx.r[1].s64 + 144;
	// 82E6DD74: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E6DD78: 4BF85C91  bl 0x82df3a08
	ctx.lr = 0x82E6DD7C;
	sub_82DF3A08(ctx, base);
	// 82E6DD7C: 3D2082E7  lis r9, -0x7d19
	ctx.r[9].s64 = -2098790400;
	// 82E6DD80: 395BFFC8  addi r10, r27, -0x38
	ctx.r[10].s64 = ctx.r[27].s64 + -56;
	// 82E6DD84: 93E10064  stw r31, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[31].u32 ) };
	// 82E6DD88: 397F0044  addi r11, r31, 0x44
	ctx.r[11].s64 = ctx.r[31].s64 + 68;
	// 82E6DD8C: 93C10070  stw r30, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[30].u32 ) };
	// 82E6DD90: 3929D860  addi r9, r9, -0x27a0
	ctx.r[9].s64 = ctx.r[9].s64 + -10144;
	// 82E6DD94: 91410068  stw r10, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[10].u32 ) };
	// 82E6DD98: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82E6DD9C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82E6DDA0: 91210060  stw r9, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[9].u32 ) };
	// 82E6DDA4: 635A0001  ori r26, r26, 1
	ctx.r[26].u64 = ctx.r[26].u64 | 1;
	// 82E6DDA8: E8810060  ld r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 82E6DDAC: E8A10068  ld r5, 0x68(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) };
	// 82E6DDB0: 4BFFFDC9  bl 0x82e6db78
	ctx.lr = 0x82E6DDB4;
	sub_82E6DB78(ctx, base);
	// 82E6DDB4: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 82E6DDB8: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82E6DDBC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E6DDC0: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82E6DDC4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E6DDC8: 4BF8BEB1  bl 0x82df9c78
	ctx.lr = 0x82E6DDCC;
	sub_82DF9C78(ctx, base);
	// 82E6DDCC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E6DDD0: 48000008  b 0x82e6ddd8
	pc = 0x82E6DDD8; continue 'dispatch;
	// 82E6DDD4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E6DDD8: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82E6DDDC: 4B47DB3D  bl 0x822eb918
	ctx.lr = 0x82E6DDE0;
	sub_822EB918(ctx, base);
	// 82E6DDE0: 574B07FF  clrlwi. r11, r26, 0x1f
	ctx.r[11].u64 = ctx.r[26].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E6DDE4: 41820010  beq 0x82e6ddf4
	if ctx.cr[0].eq {
	pc = 0x82E6DDF4; continue 'dispatch;
	}
	// 82E6DDE8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E6DDEC: 575A003C  rlwinm r26, r26, 0, 0, 0x1e
	ctx.r[26].u64 = ctx.r[26].u32 as u64 & 0xFFFFFFFFu64;
	// 82E6DDF0: 4BF85639  bl 0x82df3428
	ctx.lr = 0x82E6DDF4;
	sub_82DF3428(ctx, base);
	// 82E6DDF4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82E6DDF8: 807B0000  lwz r3, 0(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E6DDFC: 4BF8BBDD  bl 0x82df99d8
	ctx.lr = 0x82E6DE00;
	sub_82DF99D8(ctx, base);
	// 82E6DE00: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82E6DE04: 3B7B0008  addi r27, r27, 8
	ctx.r[27].s64 = ctx.r[27].s64 + 8;
	// 82E6DE08: 2F1C0006  cmpwi cr6, r28, 6
	ctx.cr[6].compare_i32(ctx.r[28].s32, 6, &mut ctx.xer);
	// 82E6DE0C: 4198FF2C  blt cr6, 0x82e6dd38
	if ctx.cr[6].lt {
	pc = 0x82E6DD38; continue 'dispatch;
	}
	// 82E6DE10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6DE14: 382101F0  addi r1, r1, 0x1f0
	ctx.r[1].s64 = ctx.r[1].s64 + 496;
	// 82E6DE18: 4833A388  b 0x831a81a0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6DE20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E6DE20 size=96
    let mut pc: u32 = 0x82E6DE20;
    'dispatch: loop {
        match pc {
            0x82E6DE20 => {
    //   block [0x82E6DE20..0x82E6DE80)
	// 82E6DE20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6DE24: 4833A349  bl 0x831a816c
	ctx.lr = 0x82E6DE28;
	sub_831A8130(ctx, base);
	// 82E6DE28: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E6DE2C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E6DE30: 3BA00005  li r29, 5
	ctx.r[29].s64 = 5;
	// 82E6DE34: 397E007C  addi r11, r30, 0x7c
	ctx.r[11].s64 = ctx.r[30].s64 + 124;
	// 82E6DE38: 3BEB0004  addi r31, r11, 4
	ctx.r[31].s64 = ctx.r[11].s64 + 4;
	// 82E6DE3C: 3BFFFFF8  addi r31, r31, -8
	ctx.r[31].s64 = ctx.r[31].s64 + -8;
	// 82E6DE40: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E6DE44: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E6DE48: 419A0008  beq cr6, 0x82e6de50
	if ctx.cr[6].eq {
	pc = 0x82E6DE50; continue 'dispatch;
	}
	// 82E6DE4C: 4B452A45  bl 0x822c0890
	ctx.lr = 0x82E6DE50;
	sub_822C0890(ctx, base);
	// 82E6DE50: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82E6DE54: 4080FFE8  bge 0x82e6de3c
	if !ctx.cr[0].lt {
	pc = 0x82E6DE3C; continue 'dispatch;
	}
	// 82E6DE58: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E6DE5C: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 82E6DE60: 396BD2CC  addi r11, r11, -0x2d34
	ctx.r[11].s64 = ctx.r[11].s64 + -11572;
	// 82E6DE64: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E6DE68: 4BFFFD91  bl 0x82e6dbf8
	ctx.lr = 0x82E6DE6C;
	sub_82E6DBF8(ctx, base);
	// 82E6DE6C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E6DE70: 396BD2C4  addi r11, r11, -0x2d3c
	ctx.r[11].s64 = ctx.r[11].s64 + -11580;
	// 82E6DE74: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E6DE78: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E6DE7C: 4833A340  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6DE80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E6DE80 size=76
    let mut pc: u32 = 0x82E6DE80;
    'dispatch: loop {
        match pc {
            0x82E6DE80 => {
    //   block [0x82E6DE80..0x82E6DECC)
	// 82E6DE80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6DE84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E6DE88: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E6DE8C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E6DE90: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E6DE94: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E6DE98: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E6DE9C: 4BFFFF85  bl 0x82e6de20
	ctx.lr = 0x82E6DEA0;
	sub_82E6DE20(ctx, base);
	// 82E6DEA0: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E6DEA4: 4182000C  beq 0x82e6deb0
	if ctx.cr[0].eq {
	pc = 0x82E6DEB0; continue 'dispatch;
	}
	// 82E6DEA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6DEAC: 4BF8452D  bl 0x82df23d8
	ctx.lr = 0x82E6DEB0;
	sub_82DF23D8(ctx, base);
	// 82E6DEB0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6DEB4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E6DEB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E6DEBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E6DEC0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E6DEC4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E6DEC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6DED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E6DED0 size=136
    let mut pc: u32 = 0x82E6DED0;
    'dispatch: loop {
        match pc {
            0x82E6DED0 => {
    //   block [0x82E6DED0..0x82E6DF58)
	// 82E6DED0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6DED4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E6DED8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E6DEDC: DBC1FFE0  stfd f30, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[30].u64 ) };
	// 82E6DEE0: DBE1FFE8  stfd f31, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 82E6DEE4: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E6DEE8: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82E6DEEC: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82E6DEF0: FFC01090  fmr f30, f2
	ctx.f[30].f64 = ctx.f[2].f64;
	// 82E6DEF4: 4802CAE5  bl 0x82e9a9d8
	ctx.lr = 0x82E6DEF8;
	sub_82E9A9D8(ctx, base);
	// 82E6DEF8: 7C69F838  and r9, r3, r31
	ctx.r[9].u64 = ctx.r[3].u64 & ctx.r[31].u64;
	// 82E6DEFC: 552A863E  rlwinm r10, r9, 0x10, 0x18, 0x1f
	ctx.r[10].u64 = ctx.r[9].u32 as u64 & 0x0000FFFFu64;
	// 82E6DF00: 552BC63E  rlwinm r11, r9, 0x18, 0x18, 0x1f
	ctx.r[11].u64 = ctx.r[9].u32 as u64 & 0x000000FFu64;
	// 82E6DF04: 5529063E  clrlwi r9, r9, 0x18
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x000000FFu64;
	// 82E6DF08: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82E6DF0C: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82E6DF10: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 82E6DF14: 409A0010  bne cr6, 0x82e6df24
	if !ctx.cr[6].eq {
	pc = 0x82E6DF24; continue 'dispatch;
	}
	// 82E6DF18: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82E6DF1C: C02B9534  lfs f1, -0x6acc(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27340 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82E6DF20: 4800001C  b 0x82e6df3c
	pc = 0x82E6DF3C; continue 'dispatch;
	// 82E6DF24: 796B0020  clrldi r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64 & 0x00000000FFFFFFFFu64;
	// 82E6DF28: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 82E6DF2C: C8010050  lfd f0, 0x50(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82E6DF30: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 82E6DF34: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82E6DF38: EC20F7FA  fmadds f1, f0, f31, f30
	ctx.f[1].f64 = (((ctx.f[0].f64 * ctx.f[31].f64 + ctx.f[30].f64) as f32) as f64);
	// 82E6DF3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E6DF40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E6DF44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E6DF48: CBC1FFE0  lfd f30, -0x20(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82E6DF4C: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E6DF50: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E6DF54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6DF58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E6DF58 size=24
    let mut pc: u32 = 0x82E6DF58;
    'dispatch: loop {
        match pc {
            0x82E6DF58 => {
    //   block [0x82E6DF58..0x82E6DF70)
	// 82E6DF58: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E6DF5C: C1A30004  lfs f13, 4(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E6DF60: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E6DF64: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E6DF68: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82E6DF6C: 4D990020  bgtlr cr6
	if ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6DF70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E6DF70 size=8
    let mut pc: u32 = 0x82E6DF70;
    'dispatch: loop {
        match pc {
            0x82E6DF70 => {
    //   block [0x82E6DF70..0x82E6DF78)
	// 82E6DF70: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E6DF74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6DF78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E6DF78 size=24
    let mut pc: u32 = 0x82E6DF78;
    'dispatch: loop {
        match pc {
            0x82E6DF78 => {
    //   block [0x82E6DF78..0x82E6DF90)
	// 82E6DF78: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E6DF7C: C1A30004  lfs f13, 4(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E6DF80: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E6DF84: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E6DF88: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82E6DF8C: 4D980020  bltlr cr6
	if ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6DF90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E6DF90 size=8
    let mut pc: u32 = 0x82E6DF90;
    'dispatch: loop {
        match pc {
            0x82E6DF90 => {
    //   block [0x82E6DF90..0x82E6DF98)
	// 82E6DF90: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E6DF94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6DF98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E6DF98 size=48
    let mut pc: u32 = 0x82E6DF98;
    'dispatch: loop {
        match pc {
            0x82E6DF98 => {
    //   block [0x82E6DF98..0x82E6DFC8)
	// 82E6DF98: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82E6DF9C: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82E6DFA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E6DFA4: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E6DFA8: C00A08A4  lfs f0, 0x8a4(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E6DFAC: 99630010  stb r11, 0x10(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u8 ) };
	// 82E6DFB0: C1A99534  lfs f13, -0x6acc(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-27340 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E6DFB4: 99630011  stb r11, 0x11(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(17 as u32), ctx.r[11].u8 ) };
	// 82E6DFB8: D0030004  stfs f0, 4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82E6DFBC: D1A30008  stfs f13, 8(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82E6DFC0: D003000C  stfs f0, 0xc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82E6DFC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6DFC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E6DFC8 size=48
    let mut pc: u32 = 0x82E6DFC8;
    'dispatch: loop {
        match pc {
            0x82E6DFC8 => {
    //   block [0x82E6DFC8..0x82E6DFF8)
	// 82E6DFC8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82E6DFCC: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82E6DFD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E6DFD4: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E6DFD8: C1AA9534  lfs f13, -0x6acc(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27340 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E6DFDC: 99630010  stb r11, 0x10(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u8 ) };
	// 82E6DFE0: C00908A4  lfs f0, 0x8a4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E6DFE4: 99630011  stb r11, 0x11(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(17 as u32), ctx.r[11].u8 ) };
	// 82E6DFE8: D1A30008  stfs f13, 8(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82E6DFEC: D0030004  stfs f0, 4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82E6DFF0: D003000C  stfs f0, 0xc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82E6DFF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6DFF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E6DFF8 size=8
    let mut pc: u32 = 0x82E6DFF8;
    'dispatch: loop {
        match pc {
            0x82E6DFF8 => {
    //   block [0x82E6DFF8..0x82E6E000)
	// 82E6DFF8: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E6DFFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6E000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E6E000 size=8
    let mut pc: u32 = 0x82E6E000;
    'dispatch: loop {
        match pc {
            0x82E6E000 => {
    //   block [0x82E6E000..0x82E6E008)
	// 82E6E000: 88630010  lbz r3, 0x10(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E6E004: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6E008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E6E008 size=8
    let mut pc: u32 = 0x82E6E008;
    'dispatch: loop {
        match pc {
            0x82E6E008 => {
    //   block [0x82E6E008..0x82E6E010)
	// 82E6E008: 98830010  stb r4, 0x10(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[4].u8 ) };
	// 82E6E00C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6E010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E6E010 size=8
    let mut pc: u32 = 0x82E6E010;
    'dispatch: loop {
        match pc {
            0x82E6E010 => {
    //   block [0x82E6E010..0x82E6E018)
	// 82E6E010: 88630011  lbz r3, 0x11(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(17 as u32) ) } as u64;
	// 82E6E014: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6E018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E6E018 size=8
    let mut pc: u32 = 0x82E6E018;
    'dispatch: loop {
        match pc {
            0x82E6E018 => {
    //   block [0x82E6E018..0x82E6E020)
	// 82E6E018: 98830011  stb r4, 0x11(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(17 as u32), ctx.r[4].u8 ) };
	// 82E6E01C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6E020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E6E020 size=160
    let mut pc: u32 = 0x82E6E020;
    'dispatch: loop {
        match pc {
            0x82E6E020 => {
    //   block [0x82E6E020..0x82E6E0C0)
	// 82E6E020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6E024: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E6E028: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E6E02C: DBA1FFD8  stfd f29, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[29].u64 ) };
	// 82E6E030: DBC1FFE0  stfd f30, -0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[30].u64 ) };
	// 82E6E034: DBE1FFE8  stfd f31, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 82E6E038: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E6E03C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E6E040: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82E6E044: FFC01090  fmr f30, f2
	ctx.f[30].f64 = ctx.f[2].f64;
	// 82E6E048: FFA01890  fmr f29, f3
	ctx.f[29].f64 = ctx.f[3].f64;
	// 82E6E04C: 897F0048  lbz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82E6E050: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E6E054: 40820010  bne 0x82e6e064
	if !ctx.cr[0].eq {
	pc = 0x82E6E064; continue 'dispatch;
	}
	// 82E6E058: 897F0049  lbz r11, 0x49(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(73 as u32) ) } as u64;
	// 82E6E05C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E6E060: 41820034  beq 0x82e6e094
	if ctx.cr[0].eq {
	pc = 0x82E6E094; continue 'dispatch;
	}
	// 82E6E064: C01F003C  lfs f0, 0x3c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E6E068: FF00F800  fcmpu cr6, f0, f31
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[31].f64);
	// 82E6E06C: 40990034  ble cr6, 0x82e6e0a0
	if !ctx.cr[6].gt {
	pc = 0x82E6E0A0; continue 'dispatch;
	}
	// 82E6E070: 897F0049  lbz r11, 0x49(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(73 as u32) ) } as u64;
	// 82E6E074: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E6E078: 40820010  bne 0x82e6e088
	if !ctx.cr[0].eq {
	pc = 0x82E6E088; continue 'dispatch;
	}
	// 82E6E07C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E6E080: 7CE33B78  mr r3, r7
	ctx.r[3].u64 = ctx.r[7].u64;
	// 82E6E084: 4BFF89E5  bl 0x82e66a68
	ctx.lr = 0x82E6E088;
	sub_82E66A68(ctx, base);
	// 82E6E088: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E6E08C: 997F0048  stb r11, 0x48(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[11].u8 ) };
	// 82E6E090: 997F0049  stb r11, 0x49(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(73 as u32), ctx.r[11].u8 ) };
	// 82E6E094: D3DF0044  stfs f30, 0x44(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), tmp.u32 ) };
	// 82E6E098: D3BF0040  stfs f29, 0x40(r31)
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), tmp.u32 ) };
	// 82E6E09C: D3FF003C  stfs f31, 0x3c(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), tmp.u32 ) };
	// 82E6E0A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E6E0A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E6E0A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E6E0AC: CBA1FFD8  lfd f29, -0x28(r1)
	ctx.f[29].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 82E6E0B0: CBC1FFE0  lfd f30, -0x20(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82E6E0B4: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E6E0B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E6E0BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6E0C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E6E0C0 size=228
    let mut pc: u32 = 0x82E6E0C0;
    'dispatch: loop {
        match pc {
            0x82E6E0C0 => {
    //   block [0x82E6E0C0..0x82E6E1A4)
	// 82E6E0C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6E0C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E6E0C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E6E0CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E6E0D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E6E0D4: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82E6E0D8: 4BFF8891  bl 0x82e66968
	ctx.lr = 0x82E6E0DC;
	sub_82E66968(ctx, base);
	// 82E6E0DC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E6E0E0: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82E6E0E4: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82E6E0E8: 40820014  bne 0x82e6e0fc
	if !ctx.cr[0].eq {
	pc = 0x82E6E0FC; continue 'dispatch;
	}
	// 82E6E0EC: C01F0008  lfs f0, 8(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E6E0F0: EC000032  fmuls f0, f0, f0
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[0].f64) as f32) as f64);
	// 82E6E0F4: C1BF0000  lfs f13, 0(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E6E0F8: EC2D037A  fmadds f1, f13, f13, f0
	ctx.f[1].f64 = (((ctx.f[13].f64 * ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64);
	// 82E6E0FC: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82E6E100: 409A0018  bne cr6, 0x82e6e118
	if !ctx.cr[6].eq {
	pc = 0x82E6E118; continue 'dispatch;
	}
	// 82E6E104: C01F0000  lfs f0, 0(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E6E108: EC000032  fmuls f0, f0, f0
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[0].f64) as f32) as f64);
	// 82E6E10C: C1BF0008  lfs f13, 8(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E6E110: EC0D037A  fmadds f0, f13, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64);
	// 82E6E114: EC20002C  fsqrts f1, f0
	ctx.f[1].f64 = ((ctx.f[0].f64).sqrt() as f32) as f64;
	// 82E6E118: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 82E6E11C: 409A0018  bne cr6, 0x82e6e134
	if !ctx.cr[6].eq {
	pc = 0x82E6E134; continue 'dispatch;
	}
	// 82E6E120: C01F0008  lfs f0, 8(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E6E124: C1BF0000  lfs f13, 0(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E6E128: FC000210  fabs f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 & !0x8000_0000_0000_0000u64;
	// 82E6E12C: FDA06A10  fabs f13, f13
	ctx.f[13].u64 = ctx.f[13].u64 & !0x8000_0000_0000_0000u64;
	// 82E6E130: EC20682A  fadds f1, f0, f13
	ctx.f[1].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 82E6E134: 2F030003  cmpwi cr6, r3, 3
	ctx.cr[6].compare_i32(ctx.r[3].s32, 3, &mut ctx.xer);
	// 82E6E138: 409A0058  bne cr6, 0x82e6e190
	if !ctx.cr[6].eq {
	pc = 0x82E6E190; continue 'dispatch;
	}
	// 82E6E13C: C01F0000  lfs f0, 0(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E6E140: C1BF0008  lfs f13, 8(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E6E144: FD800210  fabs f12, f0
	ctx.f[12].u64 = ctx.f[0].u64 & !0x8000_0000_0000_0000u64;
	// 82E6E148: FD606A10  fabs f11, f13
	ctx.f[11].u64 = ctx.f[13].u64 & !0x8000_0000_0000_0000u64;
	// 82E6E14C: FF0C5800  fcmpu cr6, f12, f11
	ctx.cr[6].compare_f64(ctx.f[12].f64, ctx.f[11].f64);
	// 82E6E150: 4098000C  bge cr6, 0x82e6e15c
	if !ctx.cr[6].lt {
	pc = 0x82E6E15C; continue 'dispatch;
	}
	// 82E6E154: FD800210  fabs f12, f0
	ctx.f[12].u64 = ctx.f[0].u64 & !0x8000_0000_0000_0000u64;
	// 82E6E158: 48000008  b 0x82e6e160
	pc = 0x82E6E160; continue 'dispatch;
	// 82E6E15C: FD806A10  fabs f12, f13
	ctx.f[12].u64 = ctx.f[13].u64 & !0x8000_0000_0000_0000u64;
	// 82E6E160: FD600210  fabs f11, f0
	ctx.f[11].u64 = ctx.f[0].u64 & !0x8000_0000_0000_0000u64;
	// 82E6E164: FD406A10  fabs f10, f13
	ctx.f[10].u64 = ctx.f[13].u64 & !0x8000_0000_0000_0000u64;
	// 82E6E168: FF0B5000  fcmpu cr6, f11, f10
	ctx.cr[6].compare_f64(ctx.f[11].f64, ctx.f[10].f64);
	// 82E6E16C: 4099000C  ble cr6, 0x82e6e178
	if !ctx.cr[6].gt {
	pc = 0x82E6E178; continue 'dispatch;
	}
	// 82E6E170: FC000210  fabs f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 & !0x8000_0000_0000_0000u64;
	// 82E6E174: 48000008  b 0x82e6e17c
	pc = 0x82E6E17C; continue 'dispatch;
	// 82E6E178: FC006A10  fabs f0, f13
	ctx.f[0].u64 = ctx.f[13].u64 & !0x8000_0000_0000_0000u64;
	// 82E6E17C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82E6E180: EDA06028  fsubs f13, f0, f12
	ctx.f[13].f64 = (((ctx.f[0].f64 - ctx.f[12].f64) as f32) as f64);
	// 82E6E184: C80BAA10  lfd f0, -0x55f0(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-22000 as u32) ) };
	// 82E6E188: EC00002C  fsqrts f0, f0
	ctx.f[0].f64 = ((ctx.f[0].f64).sqrt() as f32) as f64;
	// 82E6E18C: EC206B3A  fmadds f1, f0, f12, f13
	ctx.f[1].f64 = (((ctx.f[0].f64 * ctx.f[12].f64 + ctx.f[13].f64) as f32) as f64);
	// 82E6E190: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E6E194: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E6E198: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E6E19C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E6E1A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6E1A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E6E1A8 size=100
    let mut pc: u32 = 0x82E6E1A8;
    'dispatch: loop {
        match pc {
            0x82E6E1A8 => {
    //   block [0x82E6E1A8..0x82E6E20C)
	// 82E6E1A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6E1AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E6E1B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E6E1B4: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 82E6E1B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E6E1BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E6E1C0: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82E6E1C4: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82E6E1C8: 4BFF87A9  bl 0x82e66970
	ctx.lr = 0x82E6E1CC;
	sub_82E66970(ctx, base);
	// 82E6E1CC: C01F0008  lfs f0, 8(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E6E1D0: C1830000  lfs f12, 0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E6E1D4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82E6E1D8: ED800332  fmuls f12, f0, f12
	ctx.f[12].f64 = (((ctx.f[0].f64 * ctx.f[12].f64) as f32) as f64);
	// 82E6E1DC: C1BF0000  lfs f13, 0(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E6E1E0: C1630008  lfs f11, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82E6E1E4: C00B9F7C  lfs f0, -0x6084(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24708 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E6E1E8: EDAB6378  fmsubs f13, f11, f13, f12
	ctx.f[13].f64 = (((ctx.f[11].f64 * ctx.f[13].f64 - ctx.f[12].f64) as f32) as f64);
	// 82E6E1EC: FDA06A10  fabs f13, f13
	ctx.f[13].u64 = ctx.f[13].u64 & !0x8000_0000_0000_0000u64;
	// 82E6E1F0: EC2DF83A  fmadds f1, f13, f0, f31
	ctx.f[1].f64 = (((ctx.f[13].f64 * ctx.f[0].f64 + ctx.f[31].f64) as f32) as f64);
	// 82E6E1F4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E6E1F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E6E1FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E6E200: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E6E204: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E6E208: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6E210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E6E210 size=60
    let mut pc: u32 = 0x82E6E210;
    'dispatch: loop {
        match pc {
            0x82E6E210 => {
    //   block [0x82E6E210..0x82E6E24C)
	// 82E6E210: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E6E214: C1250000  lfs f9, 0(r5)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 82E6E218: C1A40000  lfs f13, 0(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E6E21C: FF096800  fcmpu cr6, f9, f13
	ctx.cr[6].compare_f64(ctx.f[9].f64, ctx.f[13].f64);
	// 82E6E220: C18B08A4  lfs f12, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E6E224: D1830000  stfs f12, 0(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82E6E228: D1830004  stfs f12, 4(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82E6E22C: D1830008  stfs f12, 8(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82E6E230: 419A001C  beq cr6, 0x82e6e24c
	if ctx.cr[6].eq {
		sub_82E6E24C(ctx, base);
		return;
	}
	// 82E6E234: C1440008  lfs f10, 8(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82E6E238: ED696828  fsubs f11, f9, f13
	ctx.f[11].f64 = (((ctx.f[9].f64 - ctx.f[13].f64) as f32) as f64);
	// 82E6E23C: C0050008  lfs f0, 8(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E6E240: EC005028  fsubs f0, f0, f10
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[10].f64) as f32) as f64);
	// 82E6E244: ED405824  fdivs f10, f0, f11
	ctx.f[10].f64 = ((ctx.f[0].f64 / ctx.f[11].f64) as f32) as f64;
	// 82E6E248: 48000008  b 0x82e6e250
	sub_82E6E24C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6E24C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E6E24C size=48
    let mut pc: u32 = 0x82E6E24C;
    'dispatch: loop {
        match pc {
            0x82E6E24C => {
    //   block [0x82E6E24C..0x82E6E27C)
	// 82E6E24C: FD406090  fmr f10, f12
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[10].f64 = ctx.f[12].f64;
	// 82E6E250: C1670000  lfs f11, 0(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82E6E254: C0060000  lfs f0, 0(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E6E258: FF0B0000  fcmpu cr6, f11, f0
	ctx.cr[6].compare_f64(ctx.f[11].f64, ctx.f[0].f64);
	// 82E6E25C: 419A0018  beq cr6, 0x82e6e274
	if ctx.cr[6].eq {
	pc = 0x82E6E274; continue 'dispatch;
	}
	// 82E6E260: C1870008  lfs f12, 8(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E6E264: ED0B0028  fsubs f8, f11, f0
	ctx.f[8].f64 = (((ctx.f[11].f64 - ctx.f[0].f64) as f32) as f64);
	// 82E6E268: C0E60008  lfs f7, 8(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 82E6E26C: ED8C3828  fsubs f12, f12, f7
	ctx.f[12].f64 = (((ctx.f[12].f64 - ctx.f[7].f64) as f32) as f64);
	// 82E6E270: ED8C4024  fdivs f12, f12, f8
	ctx.f[12].f64 = ((ctx.f[12].f64 / ctx.f[8].f64) as f32) as f64;
	// 82E6E274: FF0A6000  fcmpu cr6, f10, f12
	ctx.cr[6].compare_f64(ctx.f[10].f64, ctx.f[12].f64);
	// 82E6E278: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6E27C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E6E27C size=60
    let mut pc: u32 = 0x82E6E27C;
    'dispatch: loop {
        match pc {
            0x82E6E27C => {
    //   block [0x82E6E27C..0x82E6E2B8)
	// 82E6E27C: FF0D4800  fcmpu cr6, f13, f9
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[9].f64);
	// 82E6E280: 419A0038  beq cr6, 0x82e6e2b8
	if ctx.cr[6].eq {
		sub_82E6E2B8(ctx, base);
		return;
	}
	// 82E6E284: FF005800  fcmpu cr6, f0, f11
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[11].f64);
	// 82E6E288: 419A0030  beq cr6, 0x82e6e2b8
	if ctx.cr[6].eq {
		sub_82E6E2B8(ctx, base);
		return;
	}
	// 82E6E28C: C1640008  lfs f11, 8(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82E6E290: ED2A6028  fsubs f9, f10, f12
	ctx.f[9].f64 = (((ctx.f[10].f64 - ctx.f[12].f64) as f32) as f64);
	// 82E6E294: ED0D5AB8  fmsubs f8, f13, f10, f11
	ctx.f[8].f64 = (((ctx.f[13].f64 * ctx.f[10].f64 - ctx.f[11].f64) as f32) as f64);
	// 82E6E298: C0E60008  lfs f7, 8(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 82E6E29C: EC00433C  fnmsubs f0, f0, f12, f8
	ctx.f[0].f64 = -(((ctx.f[0].f64 * ctx.f[12].f64 - ctx.f[8].f64) as f32) as f64);
	// 82E6E2A0: EC00382A  fadds f0, f0, f7
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[7].f64) as f32) as f64;
	// 82E6E2A4: EC004824  fdivs f0, f0, f9
	ctx.f[0].f64 = ((ctx.f[0].f64 / ctx.f[9].f64) as f32) as f64;
	// 82E6E2A8: D0030000  stfs f0, 0(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82E6E2AC: EC006828  fsubs f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 82E6E2B0: EC005ABA  fmadds f0, f0, f10, f11
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[10].f64 + ctx.f[11].f64) as f32) as f64);
	// 82E6E2B4: 48000048  b 0x82e6e2fc
	sub_82E6E2EC(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6E2B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E6E2B8 size=36
    let mut pc: u32 = 0x82E6E2B8;
    'dispatch: loop {
        match pc {
            0x82E6E2B8 => {
    //   block [0x82E6E2B8..0x82E6E2DC)
	// 82E6E2B8: FF0D4800  fcmpu cr6, f13, f9
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[9].f64);
	// 82E6E2BC: 409A0020  bne cr6, 0x82e6e2dc
	if !ctx.cr[6].eq {
		sub_82E6E2DC(ctx, base);
		return;
	}
	// 82E6E2C0: FF005800  fcmpu cr6, f0, f11
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[11].f64);
	// 82E6E2C4: 419A0018  beq cr6, 0x82e6e2dc
	if ctx.cr[6].eq {
		sub_82E6E2DC(ctx, base);
		return;
	}
	// 82E6E2C8: EC0D0028  fsubs f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 82E6E2CC: C1660008  lfs f11, 8(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82E6E2D0: D1A30000  stfs f13, 0(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82E6E2D4: EC005B3A  fmadds f0, f0, f12, f11
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[12].f64 + ctx.f[11].f64) as f32) as f64);
	// 82E6E2D8: 48000024  b 0x82e6e2fc
	sub_82E6E2EC(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6E2DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E6E2DC size=8
    let mut pc: u32 = 0x82E6E2DC;
    'dispatch: loop {
        match pc {
            0x82E6E2DC => {
    //   block [0x82E6E2DC..0x82E6E2E4)
	// 82E6E2DC: FF0D4800  fcmpu cr6, f13, f9
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[9].f64);
	// 82E6E2E0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6E2E4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E6E2E4 size=8
    let mut pc: u32 = 0x82E6E2E4;
    'dispatch: loop {
        match pc {
            0x82E6E2E4 => {
    //   block [0x82E6E2E4..0x82E6E2EC)
	// 82E6E2E4: FF005800  fcmpu cr6, f0, f11
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[11].f64);
	// 82E6E2E8: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6E2EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E6E2EC size=24
    let mut pc: u32 = 0x82E6E2EC;
    'dispatch: loop {
        match pc {
            0x82E6E2EC => {
    //   block [0x82E6E2EC..0x82E6E304)
	// 82E6E2EC: EDA06828  fsubs f13, f0, f13
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 82E6E2F0: C1840008  lfs f12, 8(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E6E2F4: D0030000  stfs f0, 0(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82E6E2F8: EC0D62BA  fmadds f0, f13, f10, f12
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[10].f64 + ctx.f[12].f64) as f32) as f64);
	// 82E6E2FC: D0030008  stfs f0, 8(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82E6E300: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6E308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E6E308 size=104
    let mut pc: u32 = 0x82E6E308;
    'dispatch: loop {
        match pc {
            0x82E6E308 => {
    //   block [0x82E6E308..0x82E6E370)
	// 82E6E308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6E30C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E6E310: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E6E314: 3940000B  li r10, 0xb
	ctx.r[10].s64 = 11;
	// 82E6E318: 13C018C7  vcmpequd (lvx128) v30, v0, v3
	tmp.u32 = ctx.r[3].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82E6E31C: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E6E320: 7C00180C  lvsl v0, 0, r3
	tmp.u32 = ctx.r[3].u32;
	// ctx.v[0] = VectorShiftTableL[(tmp.u32 & 0xF)]
	// 82E6E324: 3920000B  li r9, 0xb
	ctx.r[9].s64 = 11;
	// 82E6E328: 396BBA58  addi r11, r11, -0x45a8
	ctx.r[11].s64 = ctx.r[11].s64 + -17832;
	// 82E6E32C: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82E6E330: 13A350C7  vcmpequd (lvx128) v29, v3, v10
	tmp.u32 = ctx.r[3].u32 + ctx.r[10].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[61] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82E6E334: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6E370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E6E370 size=124
    let mut pc: u32 = 0x82E6E370;
    'dispatch: loop {
        match pc {
            0x82E6E370 => {
    //   block [0x82E6E370..0x82E6E3EC)
	// 82E6E370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6E374: 48339DF9  bl 0x831a816c
	ctx.lr = 0x82E6E378;
	sub_831A8130(ctx, base);
	// 82E6E378: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E6E37C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E6E380: 8064000C  lwz r3, 0xc(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E6E384: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82E6E388: 480167F1  bl 0x82e84b78
	ctx.lr = 0x82E6E38C;
	sub_82E84B78(ctx, base);
	// 82E6E38C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E6E390: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6E394: 480167E5  bl 0x82e84b78
	ctx.lr = 0x82E6E398;
	sub_82E84B78(ctx, base);
	// 82E6E398: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E6E39C: C01D0000  lfs f0, 0(r29)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E6E3A0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E6E3A4: C1BD0004  lfs f13, 4(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E6E3A8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E6E3AC: C19D0008  lfs f12, 8(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E6E3B0: C16B0000  lfs f11, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82E6E3B4: EC0B0028  fsubs f0, f11, f0
	ctx.f[0].f64 = (((ctx.f[11].f64 - ctx.f[0].f64) as f32) as f64);
	// 82E6E3B8: C14B0004  lfs f10, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82E6E3BC: C16B0008  lfs f11, 8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82E6E3C0: EDAA6828  fsubs f13, f10, f13
	ctx.f[13].f64 = (((ctx.f[10].f64 - ctx.f[13].f64) as f32) as f64);
	// 82E6E3C4: ED8B6028  fsubs f12, f11, f12
	ctx.f[12].f64 = (((ctx.f[11].f64 - ctx.f[12].f64) as f32) as f64);
	// 82E6E3C8: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82E6E3CC: D1A10054  stfs f13, 0x54(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82E6E3D0: D1810058  stfs f12, 0x58(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82E6E3D4: 4BFFFCED  bl 0x82e6e0c0
	ctx.lr = 0x82E6E3D8;
	sub_82E6E0C0(ctx, base);
	// 82E6E3D8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E6E3DC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E6E3E0: 4BFFFDC9  bl 0x82e6e1a8
	ctx.lr = 0x82E6E3E4;
	sub_82E6E1A8(ctx, base);
	// 82E6E3E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E6E3E8: 48339DD4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6E3F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E6E3F0 size=236
    let mut pc: u32 = 0x82E6E3F0;
    'dispatch: loop {
        match pc {
            0x82E6E3F0 => {
    //   block [0x82E6E3F0..0x82E6E4DC)
	// 82E6E3F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6E3F4: 48339D69  bl 0x831a815c
	ctx.lr = 0x82E6E3F8;
	sub_831A8130(ctx, base);
	// 82E6E3F8: DBC1FFB0  stfd f30, -0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-80 as u32), ctx.f[30].u64 ) };
	// 82E6E3FC: DBE1FFB8  stfd f31, -0x48(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-72 as u32), ctx.f[31].u64 ) };
	// 82E6E400: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E6E404: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E6E408: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82E6E40C: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 82E6E410: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82E6E414: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82E6E418: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E6E41C: 419A0014  beq cr6, 0x82e6e430
	if ctx.cr[6].eq {
	pc = 0x82E6E430; continue 'dispatch;
	}
	// 82E6E420: 815F0030  lwz r10, 0x30(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E6E424: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82E6E428: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82E6E42C: 7D6B4BD6  divw r11, r11, r9
	ctx.r[11].s32 = ctx.r[11].s32 / ctx.r[9].s32;
	// 82E6E430: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E6E434: 419A0098  beq cr6, 0x82e6e4cc
	if ctx.cr[6].eq {
	pc = 0x82E6E4CC; continue 'dispatch;
	}
	// 82E6E438: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82E6E43C: 7D7B5B78  mr r27, r11
	ctx.r[27].u64 = ctx.r[11].u64;
	// 82E6E440: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82E6E444: 7C7D5A14  add r3, r29, r11
	ctx.r[3].u64 = ctx.r[29].u64 + ctx.r[11].u64;
	// 82E6E448: 4BFFFBB1  bl 0x82e6dff8
	ctx.lr = 0x82E6E44C;
	sub_82E6DFF8(ctx, base);
	// 82E6E44C: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82E6E450: 41820070  beq 0x82e6e4c0
	if ctx.cr[0].eq {
	pc = 0x82E6E4C0; continue 'dispatch;
	}
	// 82E6E454: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82E6E458: 7C7D5A14  add r3, r29, r11
	ctx.r[3].u64 = ctx.r[29].u64 + ctx.r[11].u64;
	// 82E6E45C: 4BFF734D  bl 0x82e657a8
	ctx.lr = 0x82E6E460;
	sub_82E657A8(ctx, base);
	// 82E6E460: 7F26CB78  mr r6, r25
	ctx.r[6].u64 = ctx.r[25].u64;
	// 82E6E464: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E6E468: C05F003C  lfs f2, 0x3c(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82E6E46C: 4BFFFA65  bl 0x82e6ded0
	ctx.lr = 0x82E6E470;
	sub_82E6DED0(ctx, base);
	// 82E6E470: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82E6E474: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82E6E478: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82E6E47C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6E480: 4BFFFEF1  bl 0x82e6e370
	ctx.lr = 0x82E6E484;
	sub_82E6E370(ctx, base);
	// 82E6E484: FFC00890  fmr f30, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].f64 = ctx.f[1].f64;
	// 82E6E488: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82E6E48C: D3E10050  stfs f31, 0x50(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82E6E490: D3C10054  stfs f30, 0x54(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82E6E494: EC1EF82A  fadds f0, f30, f31
	ctx.f[0].f64 = ((ctx.f[30].f64 + ctx.f[31].f64) as f32) as f64;
	// 82E6E498: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82E6E49C: 4BFF84C5  bl 0x82e66960
	ctx.lr = 0x82E6E4A0;
	sub_82E66960(ctx, base);
	// 82E6E4A0: 546B103A  slwi r11, r3, 2
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E6E4A4: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82E6E4A8: FC40F090  fmr f2, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[2].f64 = ctx.f[30].f64;
	// 82E6E4AC: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82E6E4B0: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82E6E4B4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E6E4B8: 7C6B542E  lfsx f3, r11, r10
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 82E6E4BC: 4BFFFB65  bl 0x82e6e020
	ctx.lr = 0x82E6E4C0;
	sub_82E6E020(ctx, base);
	// 82E6E4C0: 377BFFFF  addic. r27, r27, -1
	ctx.xer.ca = (ctx.r[27].u32 > (!(-1 as u32)));
	ctx.r[27].s64 = ctx.r[27].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 82E6E4C4: 3BBD0014  addi r29, r29, 0x14
	ctx.r[29].s64 = ctx.r[29].s64 + 20;
	// 82E6E4C8: 4082FF78  bne 0x82e6e440
	if !ctx.cr[0].eq {
	pc = 0x82E6E440; continue 'dispatch;
	}
	// 82E6E4CC: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82E6E4D0: CBC1FFB0  lfd f30, -0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-80 as u32) ) };
	// 82E6E4D4: CBE1FFB8  lfd f31, -0x48(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-72 as u32) ) };
	// 82E6E4D8: 48339CD4  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6E4E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E6E4E0 size=280
    let mut pc: u32 = 0x82E6E4E0;
    'dispatch: loop {
        match pc {
            0x82E6E4E0 => {
    //   block [0x82E6E4E0..0x82E6E5F8)
	// 82E6E4E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6E4E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E6E4E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E6E4EC: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 82E6E4F0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E6E4F4: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82E6E4F8: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82E6E4FC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E6E500: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E6E504: 4BFF7D4D  bl 0x82e66250
	ctx.lr = 0x82E6E508;
	sub_82E66250(ctx, base);
	// 82E6E508: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82E6E50C: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 82E6E510: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E6E514: 419A00C0  beq cr6, 0x82e6e5d4
	if ctx.cr[6].eq {
	pc = 0x82E6E5D4; continue 'dispatch;
	}
	// 82E6E518: 81410068  lwz r10, 0x68(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E6E51C: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 82E6E520: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82E6E524: 7D4A4BD6  divw r10, r10, r9
	ctx.r[10].s32 = ctx.r[10].s32 / ctx.r[9].s32;
	// 82E6E528: 2B0A0002  cmplwi cr6, r10, 2
	ctx.cr[6].compare_u32(ctx.r[10].u32, 2 as u32, &mut ctx.xer);
	// 82E6E52C: 409A00A8  bne cr6, 0x82e6e5d4
	if !ctx.cr[6].eq {
	pc = 0x82E6E5D4; continue 'dispatch;
	}
	// 82E6E530: C1AB0000  lfs f13, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E6E534: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82E6E538: C16B0004  lfs f11, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82E6E53C: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 82E6E540: C00B000C  lfs f0, 0xc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E6E544: 3901005B  addi r8, r1, 0x5b
	ctx.r[8].s64 = ctx.r[1].s64 + 91;
	// 82E6E548: C18B0010  lfs f12, 0x10(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6E5F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E6E5F8 size=308
    let mut pc: u32 = 0x82E6E5F8;
    'dispatch: loop {
        match pc {
            0x82E6E5F8 => {
    //   block [0x82E6E5F8..0x82E6E72C)
	// 82E6E5F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6E5FC: 48339B65  bl 0x831a8160
	ctx.lr = 0x82E6E600;
	sub_831A8130(ctx, base);
	// 82E6E600: 3981FFC8  addi r12, r1, -0x38
	ctx.r[12].s64 = ctx.r[1].s64 + -56;
	// 82E6E604: 4833A475  bl 0x831a8a78
	ctx.lr = 0x82E6E608;
	sub_831A8A40(ctx, base);
	// 82E6E608: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E6E60C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E6E610: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E6E614: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82E6E618: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82E6E61C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E6E620: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82E6E624: 4BFF7C2D  bl 0x82e66250
	ctx.lr = 0x82E6E628;
	sub_82E66250(ctx, base);
	// 82E6E628: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82E6E62C: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82E6E630: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E6E634: C3AA08A4  lfs f29, 0x8a4(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 82E6E638: 419A00D8  beq cr6, 0x82e6e710
	if ctx.cr[6].eq {
	pc = 0x82E6E710; continue 'dispatch;
	}
	// 82E6E63C: 81410078  lwz r10, 0x78(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 82E6E640: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 82E6E644: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82E6E648: 7D4A4BD6  divw r10, r10, r9
	ctx.r[10].s32 = ctx.r[10].s32 / ctx.r[9].s32;
	// 82E6E64C: 2B0A0002  cmplwi cr6, r10, 2
	ctx.cr[6].compare_u32(ctx.r[10].u32, 2 as u32, &mut ctx.xer);
	// 82E6E650: 409A00C0  bne cr6, 0x82e6e710
	if !ctx.cr[6].eq {
	pc = 0x82E6E710; continue 'dispatch;
	}
	// 82E6E654: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6E658: 3BAB000C  addi r29, r11, 0xc
	ctx.r[29].s64 = ctx.r[11].s64 + 12;
	// 82E6E65C: 7D7B5B78  mr r27, r11
	ctx.r[27].u64 = ctx.r[11].u64;
	// 82E6E660: 48016519  bl 0x82e84b78
	ctx.lr = 0x82E6E664;
	sub_82E84B78(ctx, base);
	// 82E6E664: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82E6E668: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E6E66C: 4801650D  bl 0x82e84b78
	ctx.lr = 0x82E6E670;
	sub_82E84B78(ctx, base);
	// 82E6E670: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E6E674: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E6E678: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 82E6E67C: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82E6E680: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 82E6E684: 4BFFFB8D  bl 0x82e6e210
	ctx.lr = 0x82E6E688;
	sub_82E6E210(ctx, base);
	// 82E6E688: C3C30000  lfs f30, 0(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 82E6E68C: C3E30008  lfs f31, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82E6E690: FF1EE800  fcmpu cr6, f30, f29
	ctx.cr[6].compare_f64(ctx.f[30].f64, ctx.f[29].f64);
	// 82E6E694: 409A000C  bne cr6, 0x82e6e6a0
	if !ctx.cr[6].eq {
	pc = 0x82E6E6A0; continue 'dispatch;
	}
	// 82E6E698: FF1FE800  fcmpu cr6, f31, f29
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[29].f64);
	// 82E6E69C: 419A0074  beq cr6, 0x82e6e710
	if ctx.cr[6].eq {
	pc = 0x82E6E710; continue 'dispatch;
	}
	// 82E6E6A0: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82E6E6A4: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82E6E6A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6E6AC: C00B0010  lfs f0, 0x10(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E6E6B0: C1AB0004  lfs f13, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E6E6B4: EDA0682A  fadds f13, f0, f13
	ctx.f[13].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 82E6E6B8: C00A9450  lfs f0, -0x6bb0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27568 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E6E6BC: EF8D0032  fmuls f28, f13, f0
	ctx.f[28].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82E6E6C0: 480164B9  bl 0x82e84b78
	ctx.lr = 0x82E6E6C4;
	sub_82E84B78(ctx, base);
	// 82E6E6C4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E6E6C8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E6E6CC: C00B0008  lfs f0, 8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E6E6D0: C1AB0004  lfs f13, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E6E6D4: EC00F828  fsubs f0, f0, f31
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[31].f64) as f32) as f64);
	// 82E6E6D8: C18B0000  lfs f12, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E6E6DC: EDADE028  fsubs f13, f13, f28
	ctx.f[13].f64 = (((ctx.f[13].f64 - ctx.f[28].f64) as f32) as f64);
	// 82E6E6E0: ED8CF028  fsubs f12, f12, f30
	ctx.f[12].f64 = (((ctx.f[12].f64 - ctx.f[30].f64) as f32) as f64);
	// 82E6E6E4: D1810050  stfs f12, 0x50(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82E6E6E8: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82E6E6EC: D1A10054  stfs f13, 0x54(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82E6E6F0: 7F8903A6  mtctr r28
	ctx.ctr.u64 = ctx.r[28].u64;
	// 82E6E6F4: 4E800421  bctrl
	ctx.lr = 0x82E6E6F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E6E6F8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E6E6FC: 41820014  beq 0x82e6e710
	if ctx.cr[0].eq {
	pc = 0x82E6E710; continue 'dispatch;
	}
	// 82E6E700: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6E704: 4BFFFC05  bl 0x82e6e308
	ctx.lr = 0x82E6E708;
	sub_82E6E308(ctx, base);
	// 82E6E708: 4B4A51B9  bl 0x823138c0
	ctx.lr = 0x82E6E70C;
	sub_823138C0(ctx, base);
	// 82E6E70C: FFA00890  fmr f29, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[29].f64 = ctx.f[1].f64;
	// 82E6E710: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82E6E714: 4B5FB8ED  bl 0x8246a000
	ctx.lr = 0x82E6E718;
	sub_8246A000(ctx, base);
	// 82E6E718: FC20E890  fmr f1, f29
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[29].f64;
	// 82E6E71C: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 82E6E720: 3981FFC8  addi r12, r1, -0x38
	ctx.r[12].s64 = ctx.r[1].s64 + -56;
	// 82E6E724: 4833A3A1  bl 0x831a8ac4
	ctx.lr = 0x82E6E728;
	sub_831A8A8C(ctx, base);
	// 82E6E728: 48339A88  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6E730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E6E730 size=192
    let mut pc: u32 = 0x82E6E730;
    'dispatch: loop {
        match pc {
            0x82E6E730 => {
    //   block [0x82E6E730..0x82E6E7F0)
	// 82E6E730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6E734: 48339A39  bl 0x831a816c
	ctx.lr = 0x82E6E738;
	sub_831A8130(ctx, base);
	// 82E6E738: DBA1FFC8  stfd f29, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[29].u64 ) };
	// 82E6E73C: DBC1FFD0  stfd f30, -0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[30].u64 ) };
	// 82E6E740: DBE1FFD8  stfd f31, -0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82E6E744: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E6E748: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E6E74C: FFA00890  fmr f29, f1
	ctx.f[29].f64 = ctx.f[1].f64;
	// 82E6E750: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E6E754: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E6E758: C3CB08A4  lfs f30, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 82E6E75C: 4BFFFBAD  bl 0x82e6e308
	ctx.lr = 0x82E6E760;
	sub_82E6E308(ctx, base);
	// 82E6E760: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82E6E764: C3EB9528  lfs f31, -0x6ad8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27352 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82E6E768: FF01F800  fcmpu cr6, f1, f31
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[31].f64);
	// 82E6E76C: 40980048  bge cr6, 0x82e6e7b4
	if !ctx.cr[6].lt {
	pc = 0x82E6E7B4; continue 'dispatch;
	}
	// 82E6E770: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6E774: 4BFFFB95  bl 0x82e6e308
	ctx.lr = 0x82E6E778;
	sub_82E6E308(ctx, base);
	// 82E6E778: FF01F800  fcmpu cr6, f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[31].f64);
	// 82E6E77C: 40980038  bge cr6, 0x82e6e7b4
	if !ctx.cr[6].lt {
	pc = 0x82E6E7B4; continue 'dispatch;
	}
	// 82E6E780: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6E784: 480163F5  bl 0x82e84b78
	ctx.lr = 0x82E6E788;
	sub_82E84B78(ctx, base);
	// 82E6E788: 3BA30004  addi r29, r3, 4
	ctx.r[29].s64 = ctx.r[3].s64 + 4;
	// 82E6E78C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E6E790: 480163E9  bl 0x82e84b78
	ctx.lr = 0x82E6E794;
	sub_82E84B78(ctx, base);
	// 82E6E794: C01D0000  lfs f0, 0(r29)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E6E798: C1A30004  lfs f13, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E6E79C: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82E6E7A0: 4099002C  ble cr6, 0x82e6e7cc
	if !ctx.cr[6].gt {
	pc = 0x82E6E7CC; continue 'dispatch;
	}
	// 82E6E7A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6E7A8: 4BFFFB61  bl 0x82e6e308
	ctx.lr = 0x82E6E7AC;
	sub_82E6E308(ctx, base);
	// 82E6E7AC: 4B4A5115  bl 0x823138c0
	ctx.lr = 0x82E6E7B0;
	sub_823138C0(ctx, base);
	// 82E6E7B0: 48000018  b 0x82e6e7c8
	pc = 0x82E6E7C8; continue 'dispatch;
	// 82E6E7B4: 3D6082E7  lis r11, -0x7d19
	ctx.r[11].s64 = -2098790400;
	// 82E6E7B8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E6E7BC: 38ABDF58  addi r5, r11, -0x20a8
	ctx.r[5].s64 = ctx.r[11].s64 + -8360;
	// 82E6E7C0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E6E7C4: 4BFFFE35  bl 0x82e6e5f8
	ctx.lr = 0x82E6E7C8;
	sub_82E6E5F8(ctx, base);
	// 82E6E7C8: FFC00890  fmr f30, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].f64 = ctx.f[1].f64;
	// 82E6E7CC: FF1EE800  fcmpu cr6, f30, f29
	ctx.cr[6].compare_f64(ctx.f[30].f64, ctx.f[29].f64);
	// 82E6E7D0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E6E7D4: 41980008  blt cr6, 0x82e6e7dc
	if ctx.cr[6].lt {
	pc = 0x82E6E7DC; continue 'dispatch;
	}
	// 82E6E7D8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E6E7DC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E6E7E0: CBA1FFC8  lfd f29, -0x38(r1)
	ctx.f[29].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 82E6E7E4: CBC1FFD0  lfd f30, -0x30(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 82E6E7E8: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 82E6E7EC: 483399D0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6E7F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E6E7F0 size=192
    let mut pc: u32 = 0x82E6E7F0;
    'dispatch: loop {
        match pc {
            0x82E6E7F0 => {
    //   block [0x82E6E7F0..0x82E6E8B0)
	// 82E6E7F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6E7F4: 48339979  bl 0x831a816c
	ctx.lr = 0x82E6E7F8;
	sub_831A8130(ctx, base);
	// 82E6E7F8: DBA1FFC8  stfd f29, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[29].u64 ) };
	// 82E6E7FC: DBC1FFD0  stfd f30, -0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[30].u64 ) };
	// 82E6E800: DBE1FFD8  stfd f31, -0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82E6E804: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E6E808: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E6E80C: FFA00890  fmr f29, f1
	ctx.f[29].f64 = ctx.f[1].f64;
	// 82E6E810: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E6E814: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E6E818: C3CB08A4  lfs f30, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 82E6E81C: 4BFFFAED  bl 0x82e6e308
	ctx.lr = 0x82E6E820;
	sub_82E6E308(ctx, base);
	// 82E6E820: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82E6E824: C3EB9528  lfs f31, -0x6ad8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27352 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82E6E828: FF01F800  fcmpu cr6, f1, f31
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[31].f64);
	// 82E6E82C: 40980048  bge cr6, 0x82e6e874
	if !ctx.cr[6].lt {
	pc = 0x82E6E874; continue 'dispatch;
	}
	// 82E6E830: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6E834: 4BFFFAD5  bl 0x82e6e308
	ctx.lr = 0x82E6E838;
	sub_82E6E308(ctx, base);
	// 82E6E838: FF01F800  fcmpu cr6, f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[31].f64);
	// 82E6E83C: 40980038  bge cr6, 0x82e6e874
	if !ctx.cr[6].lt {
	pc = 0x82E6E874; continue 'dispatch;
	}
	// 82E6E840: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6E844: 48016335  bl 0x82e84b78
	ctx.lr = 0x82E6E848;
	sub_82E84B78(ctx, base);
	// 82E6E848: 3BA30004  addi r29, r3, 4
	ctx.r[29].s64 = ctx.r[3].s64 + 4;
	// 82E6E84C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E6E850: 48016329  bl 0x82e84b78
	ctx.lr = 0x82E6E854;
	sub_82E84B78(ctx, base);
	// 82E6E854: C01D0000  lfs f0, 0(r29)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E6E858: C1A30004  lfs f13, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E6E85C: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82E6E860: 4098002C  bge cr6, 0x82e6e88c
	if !ctx.cr[6].lt {
	pc = 0x82E6E88C; continue 'dispatch;
	}
	// 82E6E864: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6E868: 4BFFFAA1  bl 0x82e6e308
	ctx.lr = 0x82E6E86C;
	sub_82E6E308(ctx, base);
	// 82E6E86C: 4B4A5055  bl 0x823138c0
	ctx.lr = 0x82E6E870;
	sub_823138C0(ctx, base);
	// 82E6E870: 48000018  b 0x82e6e888
	pc = 0x82E6E888; continue 'dispatch;
	// 82E6E874: 3D6082E7  lis r11, -0x7d19
	ctx.r[11].s64 = -2098790400;
	// 82E6E878: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E6E87C: 38ABDF78  addi r5, r11, -0x2088
	ctx.r[5].s64 = ctx.r[11].s64 + -8328;
	// 82E6E880: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E6E884: 4BFFFD75  bl 0x82e6e5f8
	ctx.lr = 0x82E6E888;
	sub_82E6E5F8(ctx, base);
	// 82E6E888: FFC00890  fmr f30, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].f64 = ctx.f[1].f64;
	// 82E6E88C: FF1EE800  fcmpu cr6, f30, f29
	ctx.cr[6].compare_f64(ctx.f[30].f64, ctx.f[29].f64);
	// 82E6E890: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E6E894: 41980008  blt cr6, 0x82e6e89c
	if ctx.cr[6].lt {
	pc = 0x82E6E89C; continue 'dispatch;
	}
	// 82E6E898: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E6E89C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E6E8A0: CBA1FFC8  lfd f29, -0x38(r1)
	ctx.f[29].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 82E6E8A4: CBC1FFD0  lfd f30, -0x30(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 82E6E8A8: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 82E6E8AC: 48339910  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6E8B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E6E8B0 size=48
    let mut pc: u32 = 0x82E6E8B0;
    'dispatch: loop {
        match pc {
            0x82E6E8B0 => {
    //   block [0x82E6E8B0..0x82E6E8E0)
	// 82E6E8B0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E6E8B4: C1A30000  lfs f13, 0(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E6E8B8: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E6E8BC: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82E6E8C0: 409A0020  bne cr6, 0x82e6e8e0
	if !ctx.cr[6].eq {
		sub_82E6E8E0(ctx, base);
		return;
	}
	// 82E6E8C4: C1A30004  lfs f13, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E6E8C8: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82E6E8CC: 409A0014  bne cr6, 0x82e6e8e0
	if !ctx.cr[6].eq {
		sub_82E6E8E0(ctx, base);
		return;
	}
	// 82E6E8D0: C1A30008  lfs f13, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E6E8D4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E6E8D8: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82E6E8DC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6E8E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E6E8E0 size=8
    let mut pc: u32 = 0x82E6E8E0;
    'dispatch: loop {
        match pc {
            0x82E6E8E0 => {
    //   block [0x82E6E8E0..0x82E6E8E8)
	// 82E6E8E0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E6E8E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6E8E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E6E8E8 size=8
    let mut pc: u32 = 0x82E6E8E8;
    'dispatch: loop {
        match pc {
            0x82E6E8E8 => {
    //   block [0x82E6E8E8..0x82E6E8F0)
	// 82E6E8E8: FF011000  fcmpu cr6, f1, f2
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[2].f64);
	// 82E6E8EC: 4D980020  bltlr cr6
	if ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6E8F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E6E8F0 size=8
    let mut pc: u32 = 0x82E6E8F0;
    'dispatch: loop {
        match pc {
            0x82E6E8F0 => {
    //   block [0x82E6E8F0..0x82E6E8F8)
	// 82E6E8F0: FC201090  fmr f1, f2
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[2].f64;
	// 82E6E8F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6E8F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E6E8F8 size=24
    let mut pc: u32 = 0x82E6E8F8;
    'dispatch: loop {
        match pc {
            0x82E6E8F8 => {
    //   block [0x82E6E8F8..0x82E6E910)
	// 82E6E8F8: 2B037FFF  cmplwi cr6, r3, 0x7fff
	ctx.cr[6].compare_u32(ctx.r[3].u32, 32767 as u32, &mut ctx.xer);
	// 82E6E8FC: 546B043E  clrlwi r11, r3, 0x10
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 82E6E900: 40990008  ble cr6, 0x82e6e908
	if !ctx.cr[6].gt {
	pc = 0x82E6E908; continue 'dispatch;
	}
	// 82E6E904: 39607FFF  li r11, 0x7fff
	ctx.r[11].s64 = 32767;
	// 82E6E908: 5563043E  clrlwi r3, r11, 0x10
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 82E6E90C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6E910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E6E910 size=16
    let mut pc: u32 = 0x82E6E910;
    'dispatch: loop {
        match pc {
            0x82E6E910 => {
    //   block [0x82E6E910..0x82E6E920)
	// 82E6E910: 3D607FFF  lis r11, 0x7fff
	ctx.r[11].s64 = 2147418112;
	// 82E6E914: 616BFFFF  ori r11, r11, 0xffff
	ctx.r[11].u64 = ctx.r[11].u64 | 65535;
	// 82E6E918: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E6E91C: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6E920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E6E920 size=8
    let mut pc: u32 = 0x82E6E920;
    'dispatch: loop {
        match pc {
            0x82E6E920 => {
    //   block [0x82E6E920..0x82E6E928)
	// 82E6E920: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82E6E924: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6E928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E6E928 size=40
    let mut pc: u32 = 0x82E6E928;
    'dispatch: loop {
        match pc {
            0x82E6E928 => {
    //   block [0x82E6E928..0x82E6E950)
	// 82E6E928: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E6E92C: C0040000  lfs f0, 0(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E6E930: D0030000  stfs f0, 0(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82E6E934: C1A40004  lfs f13, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E6E938: C1840008  lfs f12, 8(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E6E93C: D1A30004  stfs f13, 4(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82E6E940: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E6E944: D1830008  stfs f12, 8(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82E6E948: D003000C  stfs f0, 0xc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82E6E94C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6E950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E6E950 size=28
    let mut pc: u32 = 0x82E6E950;
    'dispatch: loop {
        match pc {
            0x82E6E950 => {
    //   block [0x82E6E950..0x82E6E96C)
	// 82E6E950: C0040000  lfs f0, 0(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E6E954: C1A40004  lfs f13, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E6E958: C1840008  lfs f12, 8(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E6E95C: D0030000  stfs f0, 0(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82E6E960: D1A30004  stfs f13, 4(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82E6E964: D1830008  stfs f12, 8(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82E6E968: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6E970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E6E970 size=380
    let mut pc: u32 = 0x82E6E970;
    'dispatch: loop {
        match pc {
            0x82E6E970 => {
    //   block [0x82E6E970..0x82E6EAEC)
	// 82E6E970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6E974: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E6E978: 3981FFF8  addi r12, r1, -8
	ctx.r[12].s64 = ctx.r[1].s64 + -8;
	// 82E6E97C: 4833A0FD  bl 0x831a8a78
	ctx.lr = 0x82E6E980;
	sub_831A8A40(ctx, base);
	// 82E6E980: C1240010  lfs f9, 0x10(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 82E6E984: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82E6E988: FCE04890  fmr f7, f9
	ctx.f[7].f64 = ctx.f[9].f64;
	// 82E6E98C: C0C40014  lfs f6, 0x14(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 82E6E990: C0040004  lfs f0, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E6E994: 3964000C  addi r11, r4, 0xc
	ctx.r[11].s64 = ctx.r[4].s64 + 12;
	// 82E6E998: C1A40008  lfs f13, 8(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E6E99C: ED800032  fmuls f12, f0, f0
	ctx.f[12].f64 = (((ctx.f[0].f64 * ctx.f[0].f64) as f32) as f64);
	// 82E6E9A0: ED6D0372  fmuls f11, f13, f13
	ctx.f[11].f64 = (((ctx.f[13].f64 * ctx.f[13].f64) as f32) as f64);
	// 82E6E9A4: C044000C  lfs f2, 0xc(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82E6E9A8: C1440000  lfs f10, 0(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82E6E9AC: FC201090  fmr f1, f2
	ctx.f[1].f64 = ctx.f[2].f64;
	// 82E6E9B0: C0A40010  lfs f5, 0x10(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 82E6E9B4: ED0A02B2  fmuls f8, f10, f10
	ctx.f[8].f64 = (((ctx.f[10].f64 * ctx.f[10].f64) as f32) as f64);
	// 82E6E9B8: ECA50032  fmuls f5, f5, f0
	ctx.f[5].f64 = (((ctx.f[5].f64 * ctx.f[0].f64) as f32) as f64);
	// 82E6E9BC: C3A30004  lfs f29, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 82E6E9C0: C0840010  lfs f4, 0x10(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 82E6E9C4: 3943000C  addi r10, r3, 0xc
	ctx.r[10].s64 = ctx.r[3].s64 + 12;
	// 82E6E9C8: C0640014  lfs f3, 0x14(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 82E6E9CC: EC840032  fmuls f4, f4, f0
	ctx.f[4].f64 = (((ctx.f[4].f64 * ctx.f[0].f64) as f32) as f64);
	// 82E6E9D0: EC630372  fmuls f3, f3, f13
	ctx.f[3].f64 = (((ctx.f[3].f64 * ctx.f[13].f64) as f32) as f64);
	// 82E6E9D4: C3C3000C  lfs f30, 0xc(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 82E6E9D8: ED2901F2  fmuls f9, f9, f7
	ctx.f[9].f64 = (((ctx.f[9].f64 * ctx.f[7].f64) as f32) as f64);
	// 82E6E9DC: C3840014  lfs f28, 0x14(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) };
	ctx.f[28].f64 = (tmp.f32 as f64);
	// 82E6E9E0: FCE03090  fmr f7, f6
	ctx.f[7].f64 = ctx.f[6].f64;
	// 82E6E9E4: ED8B602A  fadds f12, f11, f12
	ctx.f[12].f64 = ((ctx.f[11].f64 + ctx.f[12].f64) as f32) as f64;
	// 82E6E9E8: FD601090  fmr f11, f2
	ctx.f[11].f64 = ctx.f[2].f64;
	// 82E6E9EC: EC2A0072  fmuls f1, f10, f1
	ctx.f[1].f64 = (((ctx.f[10].f64 * ctx.f[1].f64) as f32) as f64);
	// 82E6E9F0: ECE601F2  fmuls f7, f6, f7
	ctx.f[7].f64 = (((ctx.f[6].f64 * ctx.f[7].f64) as f32) as f64);
	// 82E6E9F4: ECC60372  fmuls f6, f6, f13
	ctx.f[6].f64 = (((ctx.f[6].f64 * ctx.f[13].f64) as f32) as f64);
	// 82E6E9F8: ED8C402A  fadds f12, f12, f8
	ctx.f[12].f64 = ((ctx.f[12].f64 + ctx.f[8].f64) as f32) as f64;
	// 82E6E9FC: EC4202F2  fmuls f2, f2, f11
	ctx.f[2].f64 = (((ctx.f[2].f64 * ctx.f[11].f64) as f32) as f64);
	// 82E6EA00: EFEA02F2  fmuls f31, f10, f11
	ctx.f[31].f64 = (((ctx.f[10].f64 * ctx.f[11].f64) as f32) as f64);
	// 82E6EA04: C1630000  lfs f11, 0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82E6EA08: EFDE5828  fsubs f30, f30, f11
	ctx.f[30].f64 = (((ctx.f[30].f64 - ctx.f[11].f64) as f32) as f64);
	// 82E6EA0C: C1630014  lfs f11, 0x14(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82E6EA10: ED27482A  fadds f9, f7, f9
	ctx.f[9].f64 = ((ctx.f[7].f64 + ctx.f[9].f64) as f32) as f64;
	// 82E6EA14: C0E30010  lfs f7, 0x10(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 82E6EA18: EFA7E828  fsubs f29, f7, f29
	ctx.f[29].f64 = (((ctx.f[7].f64 - ctx.f[29].f64) as f32) as f64);
	// 82E6EA1C: ECC6282A  fadds f6, f6, f5
	ctx.f[6].f64 = ((ctx.f[6].f64 + ctx.f[5].f64) as f32) as f64;
	// 82E6EA20: C0A30008  lfs f5, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 82E6EA24: ECAB2828  fsubs f5, f11, f5
	ctx.f[5].f64 = (((ctx.f[11].f64 - ctx.f[5].f64) as f32) as f64);
	// 82E6EA28: C16908A4  lfs f11, 0x8a4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2212 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82E6EA2C: ECE3202A  fadds f7, f3, f4
	ctx.f[7].f64 = ((ctx.f[3].f64 + ctx.f[4].f64) as f32) as f64;
	// 82E6EA30: C084000C  lfs f4, 0xc(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 82E6EA34: C0640010  lfs f3, 0x10(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 82E6EA38: ED4A07B2  fmuls f10, f10, f30
	ctx.f[10].f64 = (((ctx.f[10].f64 * ctx.f[30].f64) as f32) as f64);
	// 82E6EA3C: EC49102A  fadds f2, f9, f2
	ctx.f[2].f64 = ((ctx.f[9].f64 + ctx.f[2].f64) as f32) as f64;
	// 82E6EA40: ED200772  fmuls f9, f0, f29
	ctx.f[9].f64 = (((ctx.f[0].f64 * ctx.f[29].f64) as f32) as f64);
	// 82E6EA44: EC26082A  fadds f1, f6, f1
	ctx.f[1].f64 = ((ctx.f[6].f64 + ctx.f[1].f64) as f32) as f64;
	// 82E6EA48: FC006050  fneg f0, f12
	ctx.f[0].u64 = ctx.f[12].u64 ^ 0x8000_0000_0000_0000u64;
	// 82E6EA4C: ED0D0172  fmuls f8, f13, f5
	ctx.f[8].f64 = (((ctx.f[13].f64 * ctx.f[5].f64) as f32) as f64);
	// 82E6EA50: EFE7F82A  fadds f31, f7, f31
	ctx.f[31].f64 = ((ctx.f[7].f64 + ctx.f[31].f64) as f32) as f64;
	// 82E6EA54: ECE407B2  fmuls f7, f4, f30
	ctx.f[7].f64 = (((ctx.f[4].f64 * ctx.f[30].f64) as f32) as f64);
	// 82E6EA58: ECC30772  fmuls f6, f3, f29
	ctx.f[6].f64 = (((ctx.f[3].f64 * ctx.f[29].f64) as f32) as f64);
	// 82E6EA5C: ECBC0172  fmuls f5, f28, f5
	ctx.f[5].f64 = (((ctx.f[28].f64 * ctx.f[5].f64) as f32) as f64);
	// 82E6EA60: FDA00850  fneg f13, f1
	ctx.f[13].u64 = ctx.f[1].u64 ^ 0x8000_0000_0000_0000u64;
	// 82E6EA64: ED820032  fmuls f12, f2, f0
	ctx.f[12].f64 = (((ctx.f[2].f64 * ctx.f[0].f64) as f32) as f64);
	// 82E6EA68: ED9F6378  fmsubs f12, f31, f13, f12
	ctx.f[12].f64 = (((ctx.f[31].f64 * ctx.f[13].f64 - ctx.f[12].f64) as f32) as f64);
	// 82E6EA6C: FF0C5800  fcmpu cr6, f12, f11
	ctx.cr[6].compare_f64(ctx.f[12].f64, ctx.f[11].f64);
	// 82E6EA70: 419A0064  beq cr6, 0x82e6ead4
	if ctx.cr[6].eq {
	pc = 0x82E6EAD4; continue 'dispatch;
	}
	// 82E6EA74: ED68482A  fadds f11, f8, f9
	ctx.f[11].f64 = ((ctx.f[8].f64 + ctx.f[9].f64) as f32) as f64;
	// 82E6EA78: C12B0000  lfs f9, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 82E6EA7C: ED05302A  fadds f8, f5, f6
	ctx.f[8].f64 = ((ctx.f[5].f64 + ctx.f[6].f64) as f32) as f64;
	// 82E6EA80: C0CB0004  lfs f6, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 82E6EA84: C0AB0008  lfs f5, 8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 82E6EA88: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E6EA8C: C08A0000  lfs f4, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 82E6EA90: C06A0004  lfs f3, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 82E6EA94: C04A0008  lfs f2, 8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82E6EA98: ED6B502A  fadds f11, f11, f10
	ctx.f[11].f64 = ((ctx.f[11].f64 + ctx.f[10].f64) as f32) as f64;
	// 82E6EA9C: ED48382A  fadds f10, f8, f7
	ctx.f[10].f64 = ((ctx.f[8].f64 + ctx.f[7].f64) as f32) as f64;
	// 82E6EAA0: EDAB0372  fmuls f13, f11, f13
	ctx.f[13].f64 = (((ctx.f[11].f64 * ctx.f[13].f64) as f32) as f64);
	// 82E6EAA4: EC0A6838  fmsubs f0, f10, f0, f13
	ctx.f[0].f64 = (((ctx.f[10].f64 * ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 82E6EAA8: EC006024  fdivs f0, f0, f12
	ctx.f[0].f64 = ((ctx.f[0].f64 / ctx.f[12].f64) as f32) as f64;
	// 82E6EAAC: EDA90032  fmuls f13, f9, f0
	ctx.f[13].f64 = (((ctx.f[9].f64 * ctx.f[0].f64) as f32) as f64);
	// 82E6EAB0: ED860032  fmuls f12, f6, f0
	ctx.f[12].f64 = (((ctx.f[6].f64 * ctx.f[0].f64) as f32) as f64);
	// 82E6EAB4: EC050032  fmuls f0, f5, f0
	ctx.f[0].f64 = (((ctx.f[5].f64 * ctx.f[0].f64) as f32) as f64);
	// 82E6EAB8: EDA4682A  fadds f13, f4, f13
	ctx.f[13].f64 = ((ctx.f[4].f64 + ctx.f[13].f64) as f32) as f64;
	// 82E6EABC: D1A50000  stfs f13, 0(r5)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82E6EAC0: EDA3602A  fadds f13, f3, f12
	ctx.f[13].f64 = ((ctx.f[3].f64 + ctx.f[12].f64) as f32) as f64;
	// 82E6EAC4: D1A50004  stfs f13, 4(r5)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82E6EAC8: EC02002A  fadds f0, f2, f0
	ctx.f[0].f64 = ((ctx.f[2].f64 + ctx.f[0].f64) as f32) as f64;
	// 82E6EACC: D0050008  stfs f0, 8(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82E6EAD0: 48000008  b 0x82e6ead8
	pc = 0x82E6EAD8; continue 'dispatch;
	// 82E6EAD4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E6EAD8: 3981FFF8  addi r12, r1, -8
	ctx.r[12].s64 = ctx.r[1].s64 + -8;
	// 82E6EADC: 48339FE9  bl 0x831a8ac4
	ctx.lr = 0x82E6EAE0;
	sub_831A8A8C(ctx, base);
	// 82E6EAE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E6EAE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E6EAE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6EAF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E6EAF0 size=148
    let mut pc: u32 = 0x82E6EAF0;
    'dispatch: loop {
        match pc {
            0x82E6EAF0 => {
    //   block [0x82E6EAF0..0x82E6EB84)
	// 82E6EAF0: 3920000B  li r9, 0xb
	ctx.r[9].s64 = 11;
	// 82E6EAF4: 13E028C7  vcmpequd (lvx128) v31, v0, v5
	tmp.u32 = ctx.r[5].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82E6EAF8: 13C020C7  vcmpequd (lvx128) v30, v0, v4
	tmp.u32 = ctx.r[4].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82E6EAFC: 3901FFF0  addi r8, r1, -0x10
	ctx.r[8].s64 = ctx.r[1].s64 + -16;
	// 82E6EB00: 7C00280C  lvsl v0, 0, r5
	tmp.u32 = ctx.r[5].u32;
	// ctx.v[0] = VectorShiftTableL[(tmp.u32 & 0xF)]
	// 82E6EB04: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82E6EB08: 7CE0200C  lvsl v7, 0, r4
	tmp.u32 = ctx.r[4].u32;
	// ctx.v[7] = VectorShiftTableL[(tmp.u32 & 0xF)]
	// 82E6EB0C: 38E1FFF0  addi r7, r1, -0x10
	ctx.r[7].s64 = ctx.r[1].s64 + -16;
	// 82E6EB10: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 82E6EB14: 13A548C7  vcmpequd (lvx128) v29, v5, v9
	tmp.u32 = ctx.r[5].u32 + ctx.r[9].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[61] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82E6EB18: 38C1FFF0  addi r6, r1, -0x10
	ctx.r[6].s64 = ctx.r[1].s64 + -16;
	// 82E6EB1C: 138448C7  vcmpequd (lvx128) v28, v4, v9
	tmp.u32 = ctx.r[4].u32 + ctx.r[9].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[60] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6EB84(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E6EB84 size=72
    let mut pc: u32 = 0x82E6EB84;
    'dispatch: loop {
        match pc {
            0x82E6EB84 => {
    //   block [0x82E6EB84..0x82E6EBCC)
	// 82E6EB84: C1A30004  lfs f13, 4(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E6EB88: C001FFE4  lfs f0, -0x1c(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E6EB8C: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82E6EB90: 4198FFEC  blt cr6, 0x82e6eb7c
	if ctx.cr[6].lt {
		sub_82E6EAF0(ctx, base);
		return;
	}
	// 82E6EB94: C0030008  lfs f0, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E6EB98: C161FFE8  lfs f11, -0x18(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82E6EB9C: FF005800  fcmpu cr6, f0, f11
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[11].f64);
	// 82E6EBA0: 4198FFDC  blt cr6, 0x82e6eb7c
	if ctx.cr[6].lt {
		sub_82E6EAF0(ctx, base);
		return;
	}
	// 82E6EBA4: C161FFF0  lfs f11, -0x10(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82E6EBA8: FF0C5800  fcmpu cr6, f12, f11
	ctx.cr[6].compare_f64(ctx.f[12].f64, ctx.f[11].f64);
	// 82E6EBAC: 4199FFD0  bgt cr6, 0x82e6eb7c
	if ctx.cr[6].gt {
		sub_82E6EAF0(ctx, base);
		return;
	}
	// 82E6EBB0: C181FFF4  lfs f12, -0xc(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-12 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82E6EBB4: FF0D6000  fcmpu cr6, f13, f12
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[12].f64);
	// 82E6EBB8: 4199FFC4  bgt cr6, 0x82e6eb7c
	if ctx.cr[6].gt {
		sub_82E6EAF0(ctx, base);
		return;
	}
	// 82E6EBBC: C1A1FFF8  lfs f13, -8(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82E6EBC0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E6EBC4: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82E6EBC8: 4D990020  bgtlr cr6
	if ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6EBCC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E6EBCC size=8
    let mut pc: u32 = 0x82E6EBCC;
    'dispatch: loop {
        match pc {
            0x82E6EBCC => {
    //   block [0x82E6EBCC..0x82E6EBD4)
	// 82E6EBCC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E6EBD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6EBD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E6EBD8 size=564
    let mut pc: u32 = 0x82E6EBD8;
    'dispatch: loop {
        match pc {
            0x82E6EBD8 => {
    //   block [0x82E6EBD8..0x82E6EE0C)
	// 82E6EBD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6EBDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E6EBE0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E6EBE4: 3981FFF0  addi r12, r1, -0x10
	ctx.r[12].s64 = ctx.r[1].s64 + -16;
	// 82E6EBE8: 48339E69  bl 0x831a8a50
	ctx.lr = 0x82E6EBEC;
	sub_831A8A40(ctx, base);
	// 82E6EBEC: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E6EBF0: C3050000  lfs f24, 0(r5)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	ctx.f[24].f64 = (tmp.f32 as f64);
	// 82E6EBF4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E6EBF8: C2E50004  lfs f23, 4(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) };
	ctx.f[23].f64 = (tmp.f32 as f64);
	// 82E6EBFC: FE400890  fmr f18, f1
	ctx.f[18].f64 = ctx.f[1].f64;
	// 82E6EC00: C2C50008  lfs f22, 8(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
	ctx.f[22].f64 = (tmp.f32 as f64);
	// 82E6EC04: 552B063F  clrlwi. r11, r9, 0x18
	ctx.r[11].u64 = ctx.r[9].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E6EC08: C3C60000  lfs f30, 0(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 82E6EC0C: C3A60004  lfs f29, 4(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 82E6EC10: C3860008  lfs f28, 8(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
	ctx.f[28].f64 = (tmp.f32 as f64);
	// 82E6EC14: C3670000  lfs f27, 0(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) };
	ctx.f[27].f64 = (tmp.f32 as f64);
	// 82E6EC18: C3470004  lfs f26, 4(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) };
	ctx.f[26].f64 = (tmp.f32 as f64);
	// 82E6EC1C: C3270008  lfs f25, 8(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(8 as u32) ) };
	ctx.f[25].f64 = (tmp.f32 as f64);
	// 82E6EC20: C2A80000  lfs f21, 0(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) };
	ctx.f[21].f64 = (tmp.f32 as f64);
	// 82E6EC24: C2880004  lfs f20, 4(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) };
	ctx.f[20].f64 = (tmp.f32 as f64);
	// 82E6EC28: C2680008  lfs f19, 8(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(8 as u32) ) };
	ctx.f[19].f64 = (tmp.f32 as f64);
	// 82E6EC2C: 41820108  beq 0x82e6ed34
	if ctx.cr[0].eq {
	pc = 0x82E6ED34; continue 'dispatch;
	}
	// 82E6EC30: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6EE10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E6EE10 size=20
    let mut pc: u32 = 0x82E6EE10;
    'dispatch: loop {
        match pc {
            0x82E6EE10 => {
    //   block [0x82E6EE10..0x82E6EE24)
	// 82E6EE10: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E6EE14: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E6EE18: 4082000C  bne 0x82e6ee24
	if !ctx.cr[0].eq {
		sub_82E6EE24(ctx, base);
		return;
	}
	// 82E6EE1C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E6EE20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6EE24(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E6EE24 size=20
    let mut pc: u32 = 0x82E6EE24;
    'dispatch: loop {
        match pc {
            0x82E6EE24 => {
    //   block [0x82E6EE24..0x82E6EE38)
	// 82E6EE24: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E6EE28: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E6EE2C: 80640000  lwz r3, 0(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E6EE30: 91440004  stw r10, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E6EE34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6EE38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E6EE38 size=60
    let mut pc: u32 = 0x82E6EE38;
    'dispatch: loop {
        match pc {
            0x82E6EE38 => {
    //   block [0x82E6EE38..0x82E6EE74)
	// 82E6EE38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6EE3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E6EE40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E6EE44: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82E6EE48: 7CAA2B78  mr r10, r5
	ctx.r[10].u64 = ctx.r[5].u64;
	// 82E6EE4C: 3D2082E7  lis r9, -0x7d19
	ctx.r[9].s64 = -2098790400;
	// 82E6EE50: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E6EE54: 3889EE10  addi r4, r9, -0x11f0
	ctx.r[4].s64 = ctx.r[9].s64 + -4592;
	// 82E6EE58: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82E6EE5C: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82E6EE60: 4BFFAC51  bl 0x82e69ab0
	ctx.lr = 0x82E6EE64;
	sub_82E69AB0(ctx, base);
	// 82E6EE64: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E6EE68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E6EE6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E6EE70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6EE78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E6EE78 size=60
    let mut pc: u32 = 0x82E6EE78;
    'dispatch: loop {
        match pc {
            0x82E6EE78 => {
    //   block [0x82E6EE78..0x82E6EEB4)
	// 82E6EE78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6EE7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E6EE80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E6EE84: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82E6EE88: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82E6EE8C: 409A0010  bne cr6, 0x82e6ee9c
	if !ctx.cr[6].eq {
	pc = 0x82E6EE9C; continue 'dispatch;
	}
	// 82E6EE90: 4833CC49  bl 0x831abad8
	ctx.lr = 0x82E6EE94;
	sub_831ABAD8(ctx, base);
	// 82E6EE94: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E6EE98: 4800000C  b 0x82e6eea4
	pc = 0x82E6EEA4; continue 'dispatch;
	// 82E6EE9C: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 82E6EEA0: 483420B9  bl 0x831b0f58
	ctx.lr = 0x82E6EEA4;
	sub_831B0F58(ctx, base);
	// 82E6EEA4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E6EEA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E6EEAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E6EEB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6EEB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E6EEB8 size=92
    let mut pc: u32 = 0x82E6EEB8;
    'dispatch: loop {
        match pc {
            0x82E6EEB8 => {
    //   block [0x82E6EEB8..0x82E6EF14)
	// 82E6EEB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6EEBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E6EEC0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E6EEC4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E6EEC8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E6EECC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82E6EED0: 3880FFFF  li r4, -1
	ctx.r[4].s64 = -1;
	// 82E6EED4: 4BFFA715  bl 0x82e695e8
	ctx.lr = 0x82E6EED8;
	sub_82E695E8(ctx, base);
	// 82E6EED8: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E6EEDC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E6EEE0: 3BEBD35C  addi r31, r11, -0x2ca4
	ctx.r[31].s64 = ctx.r[11].s64 + -11428;
	// 82E6EEE4: 4834150D  bl 0x831b03f0
	ctx.lr = 0x82E6EEE8;
	sub_831B03F0(ctx, base);
	// 82E6EEE8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E6EEEC: 38630040  addi r3, r3, 0x40
	ctx.r[3].s64 = ctx.r[3].s64 + 64;
	// 82E6EEF0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E6EEF4: 4834220D  bl 0x831b1100
	ctx.lr = 0x82E6EEF8;
	sub_831B1100(ctx, base);
	// 82E6EEF8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E6EEFC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E6EF00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E6EF04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E6EF08: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E6EF0C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E6EF10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6EF18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E6EF18 size=80
    let mut pc: u32 = 0x82E6EF18;
    'dispatch: loop {
        match pc {
            0x82E6EF18 => {
    //   block [0x82E6EF18..0x82E6EF68)
	// 82E6EF18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6EF1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E6EF20: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E6EF24: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E6EF28: 3D6082E7  lis r11, -0x7d19
	ctx.r[11].s64 = -2098790400;
	// 82E6EF2C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E6EF30: 386BEE78  addi r3, r11, -0x1188
	ctx.r[3].s64 = ctx.r[11].s64 + -4488;
	// 82E6EF34: 480003F5  bl 0x82e6f328
	ctx.lr = 0x82E6EF38;
	sub_82E6F328(ctx, base);
	// 82E6EF38: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82E6EF3C: 41820014  beq 0x82e6ef50
	if ctx.cr[0].eq {
	pc = 0x82E6EF50; continue 'dispatch;
	}
	// 82E6EF40: 3D6082E7  lis r11, -0x7d19
	ctx.r[11].s64 = -2098790400;
	// 82E6EF44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6EF48: 388BEEB8  addi r4, r11, -0x1148
	ctx.r[4].s64 = ctx.r[11].s64 + -4424;
	// 82E6EF4C: 4BFFA51D  bl 0x82e69468
	ctx.lr = 0x82E6EF50;
	sub_82E69468(ctx, base);
	// 82E6EF50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6EF54: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E6EF58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E6EF5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E6EF60: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E6EF64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6EF68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E6EF68 size=208
    let mut pc: u32 = 0x82E6EF68;
    'dispatch: loop {
        match pc {
            0x82E6EF68 => {
    //   block [0x82E6EF68..0x82E6F038)
	// 82E6EF68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6EF6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E6EF70: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E6EF74: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E6EF78: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E6EF7C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E6EF80: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E6EF84: 38C000C0  li r6, 0xc0
	ctx.r[6].s64 = 192;
	// 82E6EF88: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82E6EF8C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E6EF90: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E6EF94: 480060E5  bl 0x82e75078
	ctx.lr = 0x82E6EF98;
	sub_82E75078(ctx, base);
	// 82E6EF98: 39400008  li r10, 8
	ctx.r[10].s64 = 8;
	// 82E6EF9C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E6EFA0: 38C002D0  li r6, 0x2d0
	ctx.r[6].s64 = 720;
	// 82E6EFA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82E6EFA8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E6EFAC: 915F0030  stw r10, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[10].u32 ) };
	// 82E6EFB0: 394B00A8  addi r10, r11, 0xa8
	ctx.r[10].s64 = ctx.r[11].s64 + 168;
	// 82E6EFB4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E6EFB8: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82E6EFBC: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82E6EFC0: 915F0024  stw r10, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[10].u32 ) };
	// 82E6EFC4: 480060B5  bl 0x82e75078
	ctx.lr = 0x82E6EFC8;
	sub_82E75078(ctx, base);
	// 82E6EFC8: 3940002D  li r10, 0x2d
	ctx.r[10].s64 = 45;
	// 82E6EFCC: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E6EFD0: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 82E6EFD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82E6EFD8: 907F0020  stw r3, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[3].u32 ) };
	// 82E6EFDC: 915F002C  stw r10, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[10].u32 ) };
	// 82E6EFE0: 39430270  addi r10, r3, 0x270
	ctx.r[10].s64 = ctx.r[3].s64 + 624;
	// 82E6EFE4: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 82E6EFE8: 906B0004  stw r3, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82E6EFEC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6EFF0: 912B0008  stw r9, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82E6EFF4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6EFF8: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E6EFFC: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82E6F000: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E6F004: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E6F008: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E6F00C: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6F010: 394A0140  addi r10, r10, 0x140
	ctx.r[10].s64 = ctx.r[10].s64 + 320;
	// 82E6F014: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E6F018: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82E6F01C: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E6F020: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E6F024: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E6F028: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E6F02C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E6F030: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E6F034: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6F038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E6F038 size=172
    let mut pc: u32 = 0x82E6F038;
    'dispatch: loop {
        match pc {
            0x82E6F038 => {
    //   block [0x82E6F038..0x82E6F0E4)
	// 82E6F038: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6F03C: 4833912D  bl 0x831a8168
	ctx.lr = 0x82E6F040;
	sub_831A8130(ctx, base);
	// 82E6F040: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E6F044: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E6F048: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E6F04C: 83DF0010  lwz r30, 0x10(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E6F050: 4BFFFF19  bl 0x82e6ef68
	ctx.lr = 0x82E6F054;
	sub_82E6EF68(ctx, base);
	// 82E6F054: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82E6F058: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E6F05C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6F060: 48004B71  bl 0x82e73bd0
	ctx.lr = 0x82E6F064;
	sub_82E73BD0(ctx, base);
	// 82E6F064: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E6F068: 3B800005  li r28, 5
	ctx.r[28].s64 = 5;
	// 82E6F06C: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82E6F070: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E6F074: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6F078: 917F0048  stw r11, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 82E6F07C: 939F0050  stw r28, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 82E6F080: 83BF0010  lwz r29, 0x10(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E6F084: 48004B4D  bl 0x82e73bd0
	ctx.lr = 0x82E6F088;
	sub_82E73BD0(ctx, base);
	// 82E6F088: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E6F08C: 939D0068  stw r28, 0x68(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(104 as u32), ctx.r[28].u32 ) };
	// 82E6F090: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 82E6F094: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6F098: 917D0060  stw r11, 0x60(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82E6F09C: 480055AD  bl 0x82e74648
	ctx.lr = 0x82E6F0A0;
	sub_82E74648(ctx, base);
	// 82E6F0A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6F0A4: 48002A95  bl 0x82e71b38
	ctx.lr = 0x82E6F0A8;
	sub_82E71B38(ctx, base);
	// 82E6F0A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6F0AC: 48009735  bl 0x82e787e0
	ctx.lr = 0x82E6F0B0;
	sub_82E787E0(ctx, base);
	// 82E6F0B0: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E6F0B4: 38A00011  li r5, 0x11
	ctx.r[5].s64 = 17;
	// 82E6F0B8: 388BD1D8  addi r4, r11, -0x2e28
	ctx.r[4].s64 = ctx.r[11].s64 + -11816;
	// 82E6F0BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6F0C0: 48005699  bl 0x82e74758
	ctx.lr = 0x82E6F0C4;
	sub_82E74758(ctx, base);
	// 82E6F0C4: 89630005  lbz r11, 5(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(5 as u32) ) } as u64;
	// 82E6F0C8: 616B0020  ori r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64 | 32;
	// 82E6F0CC: 99630005  stb r11, 5(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(5 as u32), ctx.r[11].u8 ) };
	// 82E6F0D0: 817E0044  lwz r11, 0x44(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(68 as u32) ) } as u64;
	// 82E6F0D4: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E6F0D8: 917E0040  stw r11, 0x40(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 82E6F0DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E6F0E0: 483390D8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6F0E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E6F0E8 size=208
    let mut pc: u32 = 0x82E6F0E8;
    'dispatch: loop {
        match pc {
            0x82E6F0E8 => {
    //   block [0x82E6F0E8..0x82E6F1B8)
	// 82E6F0E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6F0EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E6F0F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E6F0F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E6F0F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E6F0FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E6F100: 83DF0010  lwz r30, 0x10(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E6F104: 809F0020  lwz r4, 0x20(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E6F108: 48005A61  bl 0x82e74b68
	ctx.lr = 0x82E6F10C;
	sub_82E74B68(ctx, base);
	// 82E6F10C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6F110: 480018F9  bl 0x82e70a08
	ctx.lr = 0x82E6F114;
	sub_82E70A08(ctx, base);
	// 82E6F114: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E6F118: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E6F11C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6F120: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6F124: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E6F128: 5545103A  slwi r5, r10, 2
	ctx.r[5].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E6F12C: 48005F4D  bl 0x82e75078
	ctx.lr = 0x82E6F130;
	sub_82E75078(ctx, base);
	// 82E6F130: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E6F134: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6F138: 80BE003C  lwz r5, 0x3c(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(60 as u32) ) } as u64;
	// 82E6F13C: 809E0034  lwz r4, 0x34(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 82E6F140: 48005F39  bl 0x82e75078
	ctx.lr = 0x82E6F144;
	sub_82E75078(ctx, base);
	// 82E6F144: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E6F148: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E6F14C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E6F150: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6F154: 917E0034  stw r11, 0x34(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 82E6F158: 915E003C  stw r10, 0x3c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(60 as u32), ctx.r[10].u32 ) };
	// 82E6F15C: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E6F160: 809F0028  lwz r4, 0x28(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82E6F164: 1CAB0018  mulli r5, r11, 0x18
	ctx.r[5].s64 = ctx.r[11].s64 * 24;
	// 82E6F168: 48005F11  bl 0x82e75078
	ctx.lr = 0x82E6F16C;
	sub_82E75078(ctx, base);
	// 82E6F16C: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82E6F170: 809F0020  lwz r4, 0x20(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E6F174: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E6F178: 55652036  slwi r5, r11, 4
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E6F17C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6F180: 48005EF9  bl 0x82e75078
	ctx.lr = 0x82E6F184;
	sub_82E75078(ctx, base);
	// 82E6F184: 807E0010  lwz r3, 0x10(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E6F188: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E6F18C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E6F190: 38A00178  li r5, 0x178
	ctx.r[5].s64 = 376;
	// 82E6F194: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E6F198: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E6F19C: 4E800421  bctrl
	ctx.lr = 0x82E6F1A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E6F1A0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E6F1A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E6F1A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E6F1AC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E6F1B0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E6F1B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6F1B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E6F1B8 size=232
    let mut pc: u32 = 0x82E6F1B8;
    'dispatch: loop {
        match pc {
            0x82E6F1B8 => {
    //   block [0x82E6F1B8..0x82E6F2A0)
	// 82E6F1B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6F1BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E6F1C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E6F1C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E6F1C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E6F1CC: 38C00078  li r6, 0x78
	ctx.r[6].s64 = 120;
	// 82E6F1D0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82E6F1D4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E6F1D8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E6F1DC: 48005E9D  bl 0x82e75078
	ctx.lr = 0x82E6F1E0;
	sub_82E75078(ctx, base);
	// 82E6F1E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E6F1E4: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82E6F1E8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E6F1EC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E6F1F0: 48001DF9  bl 0x82e70fe8
	ctx.lr = 0x82E6F1F4;
	sub_82E70FE8(ctx, base);
	// 82E6F1F4: 815E0010  lwz r10, 0x10(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E6F1F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E6F1FC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E6F200: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6F204: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82E6F208: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82E6F20C: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82E6F210: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82E6F214: 917F0070  stw r11, 0x70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82E6F218: 917F0040  stw r11, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 82E6F21C: 997F0036  stb r11, 0x36(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(54 as u32), ctx.r[11].u8 ) };
	// 82E6F220: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82E6F224: 995F0037  stb r10, 0x37(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(55 as u32), ctx.r[10].u8 ) };
	// 82E6F228: 917F003C  stw r11, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 82E6F22C: 917F0068  stw r11, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82E6F230: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 82E6F234: B17F0034  sth r11, 0x34(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u16 ) };
	// 82E6F238: 997F0006  stb r11, 6(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[11].u8 ) };
	// 82E6F23C: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82E6F240: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82E6F244: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82E6F248: 917F0074  stw r11, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82E6F24C: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82E6F250: 4BFFFD19  bl 0x82e6ef68
	ctx.lr = 0x82E6F254;
	sub_82E6EF68(ctx, base);
	// 82E6F254: E97E0048  ld r11, 0x48(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(72 as u32) ) };
	// 82E6F258: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6F25C: F97F0048  std r11, 0x48(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[11].u64 ) };
	// 82E6F260: 817E0050  lwz r11, 0x50(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E6F264: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82E6F268: 897E0036  lbz r11, 0x36(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(54 as u32) ) } as u64;
	// 82E6F26C: 997F0036  stb r11, 0x36(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(54 as u32), ctx.r[11].u8 ) };
	// 82E6F270: 817E0038  lwz r11, 0x38(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(56 as u32) ) } as u64;
	// 82E6F274: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82E6F278: 817E0040  lwz r11, 0x40(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(64 as u32) ) } as u64;
	// 82E6F27C: 815F0038  lwz r10, 0x38(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82E6F280: 917F0040  stw r11, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 82E6F284: 915F003C  stw r10, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[10].u32 ) };
	// 82E6F288: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E6F28C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E6F290: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E6F294: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E6F298: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E6F29C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6F2A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E6F2A0 size=132
    let mut pc: u32 = 0x82E6F2A0;
    'dispatch: loop {
        match pc {
            0x82E6F2A0 => {
    //   block [0x82E6F2A0..0x82E6F324)
	// 82E6F2A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6F2A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E6F2A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E6F2AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E6F2B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E6F2B4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E6F2B8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E6F2BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6F2C0: 809F0020  lwz r4, 0x20(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E6F2C4: 480058A5  bl 0x82e74b68
	ctx.lr = 0x82E6F2C8;
	sub_82E74B68(ctx, base);
	// 82E6F2C8: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E6F2CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E6F2D0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E6F2D4: 809F0028  lwz r4, 0x28(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82E6F2D8: 1CAB0018  mulli r5, r11, 0x18
	ctx.r[5].s64 = ctx.r[11].s64 * 24;
	// 82E6F2DC: 48005D9D  bl 0x82e75078
	ctx.lr = 0x82E6F2E0;
	sub_82E75078(ctx, base);
	// 82E6F2E0: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82E6F2E4: 809F0020  lwz r4, 0x20(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E6F2E8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E6F2EC: 55652036  slwi r5, r11, 4
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E6F2F0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E6F2F4: 48005D85  bl 0x82e75078
	ctx.lr = 0x82E6F2F8;
	sub_82E75078(ctx, base);
	// 82E6F2F8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E6F2FC: 38A00078  li r5, 0x78
	ctx.r[5].s64 = 120;
	// 82E6F300: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E6F304: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E6F308: 48005D71  bl 0x82e75078
	ctx.lr = 0x82E6F30C;
	sub_82E75078(ctx, base);
	// 82E6F30C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E6F310: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E6F314: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E6F318: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E6F31C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E6F320: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6F328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E6F328 size=476
    let mut pc: u32 = 0x82E6F328;
    'dispatch: loop {
        match pc {
            0x82E6F328 => {
    //   block [0x82E6F328..0x82E6F504)
	// 82E6F328: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6F32C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E6F330: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E6F334: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E6F338: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E6F33C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E6F340: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82E6F344: 38C00178  li r6, 0x178
	ctx.r[6].s64 = 376;
	// 82E6F348: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82E6F34C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E6F350: 916100A4  stw r11, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[11].u32 ) };
	// 82E6F354: 906100AC  stw r3, 0xac(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(172 as u32), ctx.r[3].u32 ) };
	// 82E6F358: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E6F35C: 4E800421  bctrl
	ctx.lr = 0x82E6F360;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E6F360: 90610054  stw r3, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 82E6F364: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E6F368: 41820184  beq 0x82e6f4ec
	if ctx.cr[0].eq {
	pc = 0x82E6F4EC; continue 'dispatch;
	}
	// 82E6F36C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E6F370: 83C10054  lwz r30, 0x54(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E6F374: 39630078  addi r11, r3, 0x78
	ctx.r[11].s64 = ctx.r[3].s64 + 120;
	// 82E6F378: 90610068  stw r3, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[3].u32 ) };
	// 82E6F37C: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 82E6F380: 39000021  li r8, 0x21
	ctx.r[8].s64 = 33;
	// 82E6F384: 38E00061  li r7, 0x61
	ctx.r[7].s64 = 97;
	// 82E6F388: 9141005C  stw r10, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 82E6F38C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E6F390: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82E6F394: 99230004  stb r9, 4(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[9].u8 ) };
	// 82E6F398: 392B0078  addi r9, r11, 0x78
	ctx.r[9].s64 = ctx.r[11].s64 + 120;
	// 82E6F39C: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E6F3A0: 990B0014  stb r8, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[8].u8 ) };
	// 82E6F3A4: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E6F3A8: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 82E6F3AC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82E6F3B0: 91210064  stw r9, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[9].u32 ) };
	// 82E6F3B4: 54E9063E  clrlwi r9, r7, 0x18
	ctx.r[9].u64 = ctx.r[7].u32 as u64 & 0x000000FFu64;
	// 82E6F3B8: 93FE0070  stw r31, 0x70(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(112 as u32), ctx.r[31].u32 ) };
	// 82E6F3BC: 93FE0040  stw r31, 0x40(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(64 as u32), ctx.r[31].u32 ) };
	// 82E6F3C0: 9BFE0036  stb r31, 0x36(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(54 as u32), ctx.r[31].u8 ) };
	// 82E6F3C4: 995E0037  stb r10, 0x37(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(55 as u32), ctx.r[10].u8 ) };
	// 82E6F3C8: 992B0005  stb r9, 5(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(5 as u32), ctx.r[9].u8 ) };
	// 82E6F3CC: 814100A4  lwz r10, 0xa4(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 82E6F3D0: 93FE0038  stw r31, 0x38(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(56 as u32), ctx.r[31].u32 ) };
	// 82E6F3D4: 93FE003C  stw r31, 0x3c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(60 as u32), ctx.r[31].u32 ) };
	// 82E6F3D8: 93FE0068  stw r31, 0x68(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(104 as u32), ctx.r[31].u32 ) };
	// 82E6F3DC: 93FE0030  stw r31, 0x30(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(48 as u32), ctx.r[31].u32 ) };
	// 82E6F3E0: B3FE0034  sth r31, 0x34(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(52 as u32), ctx.r[31].u16 ) };
	// 82E6F3E4: 9BFE0006  stb r31, 6(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(6 as u32), ctx.r[31].u8 ) };
	// 82E6F3E8: 93FE0014  stw r31, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[31].u32 ) };
	// 82E6F3EC: 93FE0028  stw r31, 0x28(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), ctx.r[31].u32 ) };
	// 82E6F3F0: 93FE0018  stw r31, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[31].u32 ) };
	// 82E6F3F4: 93FE0074  stw r31, 0x74(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(116 as u32), ctx.r[31].u32 ) };
	// 82E6F3F8: 93FE0050  stw r31, 0x50(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82E6F3FC: 98E10050  stb r7, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[7].u8 ) };
	// 82E6F400: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 82E6F404: 8121005C  lwz r9, 0x5c(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E6F408: 912B0020  stw r9, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[9].u32 ) };
	// 82E6F40C: 39200178  li r9, 0x178
	ctx.r[9].s64 = 376;
	// 82E6F410: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E6F414: 396B001C  addi r11, r11, 0x1c
	ctx.r[11].s64 = ctx.r[11].s64 + 28;
	// 82E6F418: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82E6F41C: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E6F420: 917E002C  stw r11, 0x2c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82E6F424: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E6F428: 917E0010  stw r11, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82E6F42C: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82E6F430: 93CB0070  stw r30, 0x70(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(112 as u32), ctx.r[30].u32 ) };
	// 82E6F434: 93EB0040  stw r31, 0x40(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(64 as u32), ctx.r[31].u32 ) };
	// 82E6F438: 93EB0008  stw r31, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 82E6F43C: 93EB0004  stw r31, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82E6F440: 93EB0000  stw r31, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82E6F444: 814100AC  lwz r10, 0xac(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(172 as u32) ) } as u64;
	// 82E6F448: 8101005C  lwz r8, 0x5c(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E6F44C: 914B0010  stw r10, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82E6F450: 81410064  lwz r10, 0x64(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82E6F454: 914B0088  stw r10, 0x88(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(136 as u32), ctx.r[10].u32 ) };
	// 82E6F458: 914B008C  stw r10, 0x8c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(140 as u32), ctx.r[10].u32 ) };
	// 82E6F45C: 815E0010  lwz r10, 0x10(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E6F460: 93EA0068  stw r31, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[31].u32 ) };
	// 82E6F464: 394B0098  addi r10, r11, 0x98
	ctx.r[10].s64 = ctx.r[11].s64 + 152;
	// 82E6F468: 910B0020  stw r8, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[8].u32 ) };
	// 82E6F46C: 390000C8  li r8, 0xc8
	ctx.r[8].s64 = 200;
	// 82E6F470: 912B0044  stw r9, 0x44(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(68 as u32), ctx.r[9].u32 ) };
	// 82E6F474: 39200009  li r9, 9
	ctx.r[9].s64 = 9;
	// 82E6F478: 93EB0034  stw r31, 0x34(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(52 as u32), ctx.r[31].u32 ) };
	// 82E6F47C: 93EB003C  stw r31, 0x3c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(60 as u32), ctx.r[31].u32 ) };
	// 82E6F480: 93EB0058  stw r31, 0x58(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 82E6F484: 9BEB0015  stb r31, 0x15(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(21 as u32), ctx.r[31].u8 ) };
	// 82E6F488: 93CB001C  stw r30, 0x1c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[30].u32 ) };
	// 82E6F48C: 93EB0018  stw r31, 0x18(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[31].u32 ) };
	// 82E6F490: 93EB0024  stw r31, 0x24(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), ctx.r[31].u32 ) };
	// 82E6F494: 93EB0028  stw r31, 0x28(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(40 as u32), ctx.r[31].u32 ) };
	// 82E6F498: 93EB002C  stw r31, 0x2c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(44 as u32), ctx.r[31].u32 ) };
	// 82E6F49C: 93EB0030  stw r31, 0x30(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(48 as u32), ctx.r[31].u32 ) };
	// 82E6F4A0: 910B0050  stw r8, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82E6F4A4: 910B0054  stw r8, 0x54(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82E6F4A8: 93EB004C  stw r31, 0x4c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(76 as u32), ctx.r[31].u32 ) };
	// 82E6F4AC: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82E6F4B0: 90EA0000  stw r7, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82E6F4B4: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82E6F4B8: 4200FFF8  bdnz 0x82e6f4b0
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82E6F4B0; continue 'dispatch;
	}
	// 82E6F4BC: 3D6082E7  lis r11, -0x7d19
	ctx.r[11].s64 = -2098790400;
	// 82E6F4C0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82E6F4C4: 388BF038  addi r4, r11, -0xfc8
	ctx.r[4].s64 = ctx.r[11].s64 + -4040;
	// 82E6F4C8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E6F4CC: 4BFFA6ED  bl 0x82e69bb8
	ctx.lr = 0x82E6F4D0;
	sub_82E69BB8(ctx, base);
	// 82E6F4D0: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82E6F4D4: 41820014  beq 0x82e6f4e8
	if ctx.cr[0].eq {
	pc = 0x82E6F4E8; continue 'dispatch;
	}
	// 82E6F4D8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E6F4DC: 4BFFFC0D  bl 0x82e6f0e8
	ctx.lr = 0x82E6F4E0;
	sub_82E6F0E8(ctx, base);
	// 82E6F4E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6F4E4: 48000008  b 0x82e6f4ec
	pc = 0x82E6F4EC; continue 'dispatch;
	// 82E6F4E8: 80610068  lwz r3, 0x68(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E6F4EC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E6F4F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E6F4F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E6F4F8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E6F4FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E6F500: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6F508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E6F508 size=4
    let mut pc: u32 = 0x82E6F508;
    'dispatch: loop {
        match pc {
            0x82E6F508 => {
    //   block [0x82E6F508..0x82E6F50C)
	// 82E6F508: 480014B8  b 0x82e709c0
	sub_82E709C0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6F510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E6F510 size=144
    let mut pc: u32 = 0x82E6F510;
    'dispatch: loop {
        match pc {
            0x82E6F510 => {
    //   block [0x82E6F510..0x82E6F5A0)
	// 82E6F510: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6F514: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E6F518: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E6F51C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E6F520: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E6F524: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E6F528: 83EB0070  lwz r31, 0x70(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(112 as u32) ) } as u64;
	// 82E6F52C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6F530: 809F0020  lwz r4, 0x20(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E6F534: 48005635  bl 0x82e74b68
	ctx.lr = 0x82E6F538;
	sub_82E74B68(ctx, base);
	// 82E6F538: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E6F53C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6F540: 48000881  bl 0x82e6fdc0
	ctx.lr = 0x82E6F544;
	sub_82E6FDC0(ctx, base);
	// 82E6F544: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82E6F548: 93DF0074  stw r30, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[30].u32 ) };
	// 82E6F54C: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82E6F550: 3D4082E7  lis r10, -0x7d19
	ctx.r[10].s64 = -2098790400;
	// 82E6F554: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82E6F558: 388AF508  addi r4, r10, -0xaf8
	ctx.r[4].s64 = ctx.r[10].s64 + -2808;
	// 82E6F55C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6F560: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82E6F564: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E6F568: B3DF0034  sth r30, 0x34(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[30].u16 ) };
	// 82E6F56C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E6F570: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82E6F574: 4BFFA645  bl 0x82e69bb8
	ctx.lr = 0x82E6F578;
	sub_82E69BB8(ctx, base);
	// 82E6F578: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82E6F57C: 4082FFD0  bne 0x82e6f54c
	if !ctx.cr[0].eq {
	pc = 0x82E6F54C; continue 'dispatch;
	}
	// 82E6F580: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6F584: 4BFFFB65  bl 0x82e6f0e8
	ctx.lr = 0x82E6F588;
	sub_82E6F0E8(ctx, base);
	// 82E6F588: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E6F58C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E6F590: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E6F594: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E6F598: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E6F59C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6F5A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E6F5A0 size=8
    let mut pc: u32 = 0x82E6F5A0;
    'dispatch: loop {
        match pc {
            0x82E6F5A0 => {
    //   block [0x82E6F5A0..0x82E6F5A8)
	// 82E6F5A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E6F5A4: 48000010  b 0x82e6f5b4
	sub_82E6F5A8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6F5A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E6F5A8 size=28
    let mut pc: u32 = 0x82E6F5A8;
    'dispatch: loop {
        match pc {
            0x82E6F5A8 => {
    //   block [0x82E6F5A8..0x82E6F5C4)
	// 82E6F5A8: 39430001  addi r10, r3, 1
	ctx.r[10].s64 = ctx.r[3].s64 + 1;
	// 82E6F5AC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E6F5B0: 5543F87E  srwi r3, r10, 1
	ctx.r[3].u32 = ctx.r[10].u32.wrapping_shr(1);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82E6F5B4: 2B030010  cmplwi cr6, r3, 0x10
	ctx.cr[6].compare_u32(ctx.r[3].u32, 16 as u32, &mut ctx.xer);
	// 82E6F5B8: 4098FFF0  bge cr6, 0x82e6f5a8
	if !ctx.cr[6].lt {
	pc = 0x82E6F5A8; continue 'dispatch;
	}
	// 82E6F5BC: 2B030008  cmplwi cr6, r3, 8
	ctx.cr[6].compare_u32(ctx.r[3].u32, 8 as u32, &mut ctx.xer);
	// 82E6F5C0: 4D980020  bltlr cr6
	if ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6F5C4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E6F5C4 size=20
    let mut pc: u32 = 0x82E6F5C4;
    'dispatch: loop {
        match pc {
            0x82E6F5C4 => {
    //   block [0x82E6F5C4..0x82E6F5D8)
	// 82E6F5C4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E6F5C8: 3943FFF8  addi r10, r3, -8
	ctx.r[10].s64 = ctx.r[3].s64 + -8;
	// 82E6F5CC: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E6F5D0: 7D635378  or r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82E6F5D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6F5D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E6F5D8 size=12
    let mut pc: u32 = 0x82E6F5D8;
    'dispatch: loop {
        match pc {
            0x82E6F5D8 => {
    //   block [0x82E6F5D8..0x82E6F5E4)
	// 82E6F5D8: 7C6B1E70  srawi r11, r3, 3
	ctx.xer.ca = (ctx.r[3].s32 < 0) && ((ctx.r[3].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[3].s32 >> 3) as i64;
	// 82E6F5DC: 556B06FF  clrlwi. r11, r11, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E6F5E0: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6F5E4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E6F5E4 size=20
    let mut pc: u32 = 0x82E6F5E4;
    'dispatch: loop {
        match pc {
            0x82E6F5E4 => {
    //   block [0x82E6F5E4..0x82E6F5F8)
	// 82E6F5E4: 546A077E  clrlwi r10, r3, 0x1d
	ctx.r[10].u64 = ctx.r[3].u32 as u64 & 0x00000007u64;
	// 82E6F5E8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82E6F5EC: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82E6F5F0: 7D435830  slw r3, r10, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[3].u64 = 0;
	} else {
		ctx.r[3].u64 = ((ctx.r[10].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 82E6F5F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6F5F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E6F5F8 size=8
    let mut pc: u32 = 0x82E6F5F8;
    'dispatch: loop {
        match pc {
            0x82E6F5F8 => {
    //   block [0x82E6F5F8..0x82E6F600)
	// 82E6F5F8: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82E6F5FC: 4800000C  b 0x82e6f608
	sub_82E6F600(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6F600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E6F600 size=36
    let mut pc: u32 = 0x82E6F600;
    'dispatch: loop {
        match pc {
            0x82E6F600 => {
    //   block [0x82E6F600..0x82E6F624)
	// 82E6F600: 5463C23E  srwi r3, r3, 8
	ctx.r[3].u32 = ctx.r[3].u32.wrapping_shr(8);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82E6F604: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82E6F608: 2B030100  cmplwi cr6, r3, 0x100
	ctx.cr[6].compare_u32(ctx.r[3].u32, 256 as u32, &mut ctx.xer);
	// 82E6F60C: 4098FFF4  bge cr6, 0x82e6f600
	if !ctx.cr[6].lt {
	pc = 0x82E6F600; continue 'dispatch;
	}
	// 82E6F610: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82E6F614: 394AD3A8  addi r10, r10, -0x2c58
	ctx.r[10].s64 = ctx.r[10].s64 + -11352;
	// 82E6F618: 7D4A18AE  lbzx r10, r10, r3
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 82E6F61C: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82E6F620: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6F628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E6F628 size=24
    let mut pc: u32 = 0x82E6F628;
    'dispatch: loop {
        match pc {
            0x82E6F628 => {
    //   block [0x82E6F628..0x82E6F640)
	// 82E6F628: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6F62C: 81440008  lwz r10, 8(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6F630: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82E6F634: 419A000C  beq cr6, 0x82e6f640
	if ctx.cr[6].eq {
		sub_82E6F640(ctx, base);
		return;
	}
	// 82E6F638: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E6F63C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6F640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E6F640 size=44
    let mut pc: u32 = 0x82E6F640;
    'dispatch: loop {
        match pc {
            0x82E6F640 => {
    //   block [0x82E6F640..0x82E6F66C)
	// 82E6F640: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82E6F644: 41980044  blt cr6, 0x82e6f688
	if ctx.cr[6].lt {
		sub_82E6F688(ctx, base);
		return;
	}
	// 82E6F648: 419A0028  beq cr6, 0x82e6f670
	if ctx.cr[6].eq {
		sub_82E6F670(ctx, base);
		return;
	}
	// 82E6F64C: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 82E6F650: 41980020  blt cr6, 0x82e6f670
	if ctx.cr[6].lt {
		sub_82E6F670(ctx, base);
		return;
	}
	// 82E6F654: 409A001C  bne cr6, 0x82e6f670
	if !ctx.cr[6].eq {
		sub_82E6F670(ctx, base);
		return;
	}
	// 82E6F658: C8030000  lfd f0, 0(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	// 82E6F65C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E6F660: C9A40000  lfd f13, 0(r4)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	// 82E6F664: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82E6F668: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6F66C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E6F66C size=4
    let mut pc: u32 = 0x82E6F66C;
    'dispatch: loop {
        match pc {
            0x82E6F66C => {
    //   block [0x82E6F66C..0x82E6F670)
	// 82E6F66C: 4BFFFFCC  b 0x82e6f638
	sub_82E6F628(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6F670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E6F670 size=24
    let mut pc: u32 = 0x82E6F670;
    'dispatch: loop {
        match pc {
            0x82E6F670 => {
    //   block [0x82E6F670..0x82E6F688)
	// 82E6F670: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E6F674: 81440000  lwz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E6F678: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82E6F67C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82E6F680: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82E6F684: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6F688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E6F688 size=8
    let mut pc: u32 = 0x82E6F688;
    'dispatch: loop {
        match pc {
            0x82E6F688 => {
    //   block [0x82E6F688..0x82E6F690)
	// 82E6F688: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E6F68C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6F690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E6F690 size=220
    let mut pc: u32 = 0x82E6F690;
    'dispatch: loop {
        match pc {
            0x82E6F690 => {
    //   block [0x82E6F690..0x82E6F76C)
	// 82E6F690: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6F694: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E6F698: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E6F69C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E6F6A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E6F6A4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E6F6A8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E6F6AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E6F6B0: 4833B039  bl 0x831aa6e8
	ctx.lr = 0x82E6F6B4;
	sub_831AA6E8(ctx, base);
	// 82E6F6B4: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E6F6B8: D83E0000  stfd f1, 0(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.f[1].u64 ) };
	// 82E6F6BC: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82E6F6C0: 409A000C  bne cr6, 0x82e6f6cc
	if !ctx.cr[6].eq {
	pc = 0x82E6F6CC; continue 'dispatch;
	}
	// 82E6F6C4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E6F6C8: 4800008C  b 0x82e6f754
	pc = 0x82E6F754; continue 'dispatch;
	// 82E6F6CC: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E6F6D0: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 82E6F6D4: 2F0A0078  cmpwi cr6, r10, 0x78
	ctx.cr[6].compare_i32(ctx.r[10].s32, 120, &mut ctx.xer);
	// 82E6F6D8: 419A000C  beq cr6, 0x82e6f6e4
	if ctx.cr[6].eq {
	pc = 0x82E6F6E4; continue 'dispatch;
	}
	// 82E6F6DC: 2F0A0058  cmpwi cr6, r10, 0x58
	ctx.cr[6].compare_i32(ctx.r[10].s32, 88, &mut ctx.xer);
	// 82E6F6E0: 409A002C  bne cr6, 0x82e6f70c
	if !ctx.cr[6].eq {
	pc = 0x82E6F70C; continue 'dispatch;
	}
	// 82E6F6E4: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 82E6F6E8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E6F6EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6F6F0: 4833AE41  bl 0x831aa530
	ctx.lr = 0x82E6F6F4;
	sub_831AA530(ctx, base);
	// 82E6F6F4: 786B0020  clrldi r11, r3, 0x20
	ctx.r[11].u64 = ctx.r[3].u64 & 0x00000000FFFFFFFFu64;
	// 82E6F6F8: F9610058  std r11, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u64 ) };
	// 82E6F6FC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E6F700: C8010058  lfd f0, 0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 82E6F704: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 82E6F708: D81E0000  stfd f0, 0(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.f[0].u64 ) };
	// 82E6F70C: 896B0000  lbz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E6F710: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E6F714: 409A0014  bne cr6, 0x82e6f728
	if !ctx.cr[6].eq {
	pc = 0x82E6F728; continue 'dispatch;
	}
	// 82E6F718: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E6F71C: 48000038  b 0x82e6f754
	pc = 0x82E6F754; continue 'dispatch;
	// 82E6F720: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E6F724: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82E6F728: 48341589  bl 0x831b0cb0
	ctx.lr = 0x82E6F72C;
	sub_831B0CB0(ctx, base);
	// 82E6F72C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E6F730: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E6F734: 554A083E  rotlwi r10, r10, 1
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(1)) as u64;
	// 82E6F738: 7D43522E  lhzx r10, r3, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[3].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E6F73C: 554A0739  rlwinm. r10, r10, 0, 0x1c, 0x1c
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E6F740: 4082FFE0  bne 0x82e6f720
	if !ctx.cr[0].eq {
	pc = 0x82E6F720; continue 'dispatch;
	}
	// 82E6F744: 896B0000  lbz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E6F748: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82E6F74C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82E6F750: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82E6F754: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E6F758: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E6F75C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E6F760: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E6F764: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E6F768: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6F770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E6F770 size=152
    let mut pc: u32 = 0x82E6F770;
    'dispatch: loop {
        match pc {
            0x82E6F770 => {
    //   block [0x82E6F770..0x82E6F808)
	// 82E6F770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6F774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E6F778: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E6F77C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E6F780: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E6F784: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E6F788: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82E6F78C: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82E6F790: 83DF0008  lwz r30, 8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6F794: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E6F798: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E6F79C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82E6F7A0: 409AFFF4  bne cr6, 0x82e6f794
	if !ctx.cr[6].eq {
	pc = 0x82E6F794; continue 'dispatch;
	}
	// 82E6F7A4: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82E6F7A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6F7AC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82E6F7B0: 5565003E  slwi r5, r11, 0
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82E6F7B4: 48004FA5  bl 0x82e74758
	ctx.lr = 0x82E6F7B8;
	sub_82E74758(ctx, base);
	// 82E6F7B8: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 82E6F7BC: 907E0000  stw r3, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82E6F7C0: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E6F7C4: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E6F7C8: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6F7CC: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82E6F7D0: 2F0B0010  cmpwi cr6, r11, 0x10
	ctx.cr[6].compare_i32(ctx.r[11].s32, 16, &mut ctx.xer);
	// 82E6F7D4: 41990010  bgt cr6, 0x82e6f7e4
	if ctx.cr[6].gt {
	pc = 0x82E6F7E4; continue 'dispatch;
	}
	// 82E6F7D8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E6F7DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6F7E0: 4BFFA619  bl 0x82e69df8
	ctx.lr = 0x82E6F7E4;
	sub_82E69DF8(ctx, base);
	// 82E6F7E4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6F7E8: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82E6F7EC: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E6F7F0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E6F7F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E6F7F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E6F7FC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E6F800: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E6F804: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6F808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E6F808 size=568
    let mut pc: u32 = 0x82E6F808;
    'dispatch: loop {
        match pc {
            0x82E6F808 => {
    //   block [0x82E6F808..0x82E6FA40)
	// 82E6F808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6F80C: 48338941  bl 0x831a814c
	ctx.lr = 0x82E6F810;
	sub_831A8130(ctx, base);
	// 82E6F810: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E6F814: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82E6F818: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82E6F81C: 388B9BC9  addi r4, r11, -0x6437
	ctx.r[4].s64 = ctx.r[11].s64 + -25655;
	// 82E6F820: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E6F824: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82E6F828: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 82E6F82C: 4BFFFF45  bl 0x82e6f770
	ctx.lr = 0x82E6F830;
	sub_82E6F770(ctx, base);
	// 82E6F830: 38800025  li r4, 0x25
	ctx.r[4].s64 = 37;
	// 82E6F834: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E6F838: 4833D5E9  bl 0x831ace20
	ctx.lr = 0x82E6F83C;
	sub_831ACE20(ctx, base);
	// 82E6F83C: 7C7B1B79  or. r27, r3, r3
	ctx.r[27].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 82E6F840: 418201B4  beq 0x82e6f9f4
	if ctx.cr[0].eq {
	pc = 0x82E6F9F4; continue 'dispatch;
	}
	// 82E6F844: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E6F848: 3AE00000  li r23, 0
	ctx.r[23].s64 = 0;
	// 82E6F84C: 3ACB5BC0  addi r22, r11, 0x5bc0
	ctx.r[22].s64 = ctx.r[11].s64 + 23488;
	// 82E6F850: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82E6F854: 3B200003  li r25, 3
	ctx.r[25].s64 = 3;
	// 82E6F858: 3B0BD230  addi r24, r11, -0x2dd0
	ctx.r[24].s64 = ctx.r[11].s64 + -11728;
	// 82E6F85C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E6F860: 3AABD4A8  addi r21, r11, -0x2b58
	ctx.r[21].s64 = ctx.r[11].s64 + -11096;
	// 82E6F864: 7CBDD850  subf r5, r29, r27
	ctx.r[5].s64 = ctx.r[27].s64 - ctx.r[29].s64;
	// 82E6F868: 839F0008  lwz r28, 8(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6F86C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E6F870: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6F874: 48004EE5  bl 0x82e74758
	ctx.lr = 0x82E6F878;
	sub_82E74758(ctx, base);
	// 82E6F878: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 82E6F87C: 907C0000  stw r3, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82E6F880: 917C0008  stw r11, 8(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E6F884: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E6F888: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6F88C: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82E6F890: 2F0B0010  cmpwi cr6, r11, 0x10
	ctx.cr[6].compare_i32(ctx.r[11].s32, 16, &mut ctx.xer);
	// 82E6F894: 41990010  bgt cr6, 0x82e6f8a4
	if ctx.cr[6].gt {
	pc = 0x82E6F8A4; continue 'dispatch;
	}
	// 82E6F898: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E6F89C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6F8A0: 4BFFA559  bl 0x82e69df8
	ctx.lr = 0x82E6F8A4;
	sub_82E69DF8(ctx, base);
	// 82E6F8A4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6F8A8: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82E6F8AC: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E6F8B0: 893B0001  lbz r9, 1(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(1 as u32) ) } as u64;
	// 82E6F8B4: 7D2A0774  extsb r10, r9
	ctx.r[10].s64 = ctx.r[9].s8 as i64;
	// 82E6F8B8: 2F0A0025  cmpwi cr6, r10, 0x25
	ctx.cr[6].compare_i32(ctx.r[10].s32, 37, &mut ctx.xer);
	// 82E6F8BC: 419A0110  beq cr6, 0x82e6f9cc
	if ctx.cr[6].eq {
	pc = 0x82E6F9CC; continue 'dispatch;
	}
	// 82E6F8C0: 2F0A0063  cmpwi cr6, r10, 0x63
	ctx.cr[6].compare_i32(ctx.r[10].s32, 99, &mut ctx.xer);
	// 82E6F8C4: 419A00E8  beq cr6, 0x82e6f9ac
	if ctx.cr[6].eq {
	pc = 0x82E6F9AC; continue 'dispatch;
	}
	// 82E6F8C8: 2F0A0064  cmpwi cr6, r10, 0x64
	ctx.cr[6].compare_i32(ctx.r[10].s32, 100, &mut ctx.xer);
	// 82E6F8CC: 419A008C  beq cr6, 0x82e6f958
	if ctx.cr[6].eq {
	pc = 0x82E6F958; continue 'dispatch;
	}
	// 82E6F8D0: 2F0A0066  cmpwi cr6, r10, 0x66
	ctx.cr[6].compare_i32(ctx.r[10].s32, 102, &mut ctx.xer);
	// 82E6F8D4: 419A0070  beq cr6, 0x82e6f944
	if ctx.cr[6].eq {
	pc = 0x82E6F944; continue 'dispatch;
	}
	// 82E6F8D8: 2F0A0070  cmpwi cr6, r10, 0x70
	ctx.cr[6].compare_i32(ctx.r[10].s32, 112, &mut ctx.xer);
	// 82E6F8DC: 419A0044  beq cr6, 0x82e6f920
	if ctx.cr[6].eq {
	pc = 0x82E6F920; continue 'dispatch;
	}
	// 82E6F8E0: 2F0A0073  cmpwi cr6, r10, 0x73
	ctx.cr[6].compare_i32(ctx.r[10].s32, 115, &mut ctx.xer);
	// 82E6F8E4: 419A001C  beq cr6, 0x82e6f900
	if ctx.cr[6].eq {
	pc = 0x82E6F900; continue 'dispatch;
	}
	// 82E6F8E8: 39600025  li r11, 0x25
	ctx.r[11].s64 = 37;
	// 82E6F8EC: 99210053  stb r9, 0x53(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(83 as u32), ctx.r[9].u8 ) };
	// 82E6F8F0: 38810052  addi r4, r1, 0x52
	ctx.r[4].s64 = ctx.r[1].s64 + 82;
	// 82E6F8F4: 9AE10054  stb r23, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[23].u8 ) };
	// 82E6F8F8: 99610052  stb r11, 0x52(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(82 as u32), ctx.r[11].u8 ) };
	// 82E6F8FC: 480000D4  b 0x82e6f9d0
	pc = 0x82E6F9D0; continue 'dispatch;
	// 82E6F900: 397E0007  addi r11, r30, 7
	ctx.r[11].s64 = ctx.r[30].s64 + 7;
	// 82E6F904: 556B0038  rlwinm r11, r11, 0, 0, 0x1c
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E6F908: 3BCB0008  addi r30, r11, 8
	ctx.r[30].s64 = ctx.r[11].s64 + 8;
	// 82E6F90C: 809EFFFC  lwz r4, -4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82E6F910: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E6F914: 408200BC  bne 0x82e6f9d0
	if !ctx.cr[0].eq {
	pc = 0x82E6F9D0; continue 'dispatch;
	}
	// 82E6F918: 7EA4AB78  mr r4, r21
	ctx.r[4].u64 = ctx.r[21].u64;
	// 82E6F91C: 480000B4  b 0x82e6f9d0
	pc = 0x82E6F9D0; continue 'dispatch;
	// 82E6F920: 397E0007  addi r11, r30, 7
	ctx.r[11].s64 = ctx.r[30].s64 + 7;
	// 82E6F924: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82E6F928: 556B0038  rlwinm r11, r11, 0, 0, 0x1c
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E6F92C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E6F930: 3BCB0008  addi r30, r11, 8
	ctx.r[30].s64 = ctx.r[11].s64 + 8;
	// 82E6F934: 80BEFFFC  lwz r5, -4(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82E6F938: 483391A1  bl 0x831a8ad8
	ctx.lr = 0x82E6F93C;
	sub_831A8AD8(ctx, base);
	// 82E6F93C: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82E6F940: 48000090  b 0x82e6f9d0
	pc = 0x82E6F9D0; continue 'dispatch;
	// 82E6F944: 395E0007  addi r10, r30, 7
	ctx.r[10].s64 = ctx.r[30].s64 + 7;
	// 82E6F948: 554A0038  rlwinm r10, r10, 0, 0, 0x1c
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82E6F94C: 3BCA0008  addi r30, r10, 8
	ctx.r[30].s64 = ctx.r[10].s64 + 8;
	// 82E6F950: C81EFFF8  lfd f0, -8(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(-8 as u32) ) };
	// 82E6F954: 48000020  b 0x82e6f974
	pc = 0x82E6F974; continue 'dispatch;
	// 82E6F958: 395E0007  addi r10, r30, 7
	ctx.r[10].s64 = ctx.r[30].s64 + 7;
	// 82E6F95C: 554A0038  rlwinm r10, r10, 0, 0, 0x1c
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82E6F960: 3BCA0008  addi r30, r10, 8
	ctx.r[30].s64 = ctx.r[10].s64 + 8;
	// 82E6F964: E95EFFFE  lwa r10, -4(r30)
	ctx.r[10].s64 = (unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-4 as u32) ) } as i32) as i64;
	// 82E6F968: F9410058  std r10, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u64 ) };
	// 82E6F96C: C8010058  lfd f0, 0x58(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 82E6F970: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 82E6F974: 932B0008  stw r25, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[25].u32 ) };
	// 82E6F978: D80B0000  stfd f0, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.f[0].u64 ) };
	// 82E6F97C: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E6F980: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6F984: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82E6F988: 2F0B0010  cmpwi cr6, r11, 0x10
	ctx.cr[6].compare_i32(ctx.r[11].s32, 16, &mut ctx.xer);
	// 82E6F98C: 41990010  bgt cr6, 0x82e6f99c
	if ctx.cr[6].gt {
	pc = 0x82E6F99C; continue 'dispatch;
	}
	// 82E6F990: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E6F994: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6F998: 4BFFA461  bl 0x82e69df8
	ctx.lr = 0x82E6F99C;
	sub_82E69DF8(ctx, base);
	// 82E6F99C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6F9A0: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82E6F9A4: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E6F9A8: 48000030  b 0x82e6f9d8
	pc = 0x82E6F9D8; continue 'dispatch;
	// 82E6F9AC: 397E0007  addi r11, r30, 7
	ctx.r[11].s64 = ctx.r[30].s64 + 7;
	// 82E6F9B0: 9AE10051  stb r23, 0x51(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(81 as u32), ctx.r[23].u8 ) };
	// 82E6F9B4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E6F9B8: 556B0038  rlwinm r11, r11, 0, 0, 0x1c
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E6F9BC: 3BCB0008  addi r30, r11, 8
	ctx.r[30].s64 = ctx.r[11].s64 + 8;
	// 82E6F9C0: 817EFFFC  lwz r11, -4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82E6F9C4: 99610050  stb r11, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 82E6F9C8: 48000008  b 0x82e6f9d0
	pc = 0x82E6F9D0; continue 'dispatch;
	// 82E6F9CC: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 82E6F9D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6F9D4: 4BFFFD9D  bl 0x82e6f770
	ctx.lr = 0x82E6F9D8;
	sub_82E6F770(ctx, base);
	// 82E6F9D8: 3BBB0002  addi r29, r27, 2
	ctx.r[29].s64 = ctx.r[27].s64 + 2;
	// 82E6F9DC: 38800025  li r4, 0x25
	ctx.r[4].s64 = 37;
	// 82E6F9E0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E6F9E4: 3B5A0002  addi r26, r26, 2
	ctx.r[26].s64 = ctx.r[26].s64 + 2;
	// 82E6F9E8: 4833D439  bl 0x831ace20
	ctx.lr = 0x82E6F9EC;
	sub_831ACE20(ctx, base);
	// 82E6F9EC: 7C7B1B79  or. r27, r3, r3
	ctx.r[27].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 82E6F9F0: 4082FE74  bne 0x82e6f864
	if !ctx.cr[0].eq {
	pc = 0x82E6F864; continue 'dispatch;
	}
	// 82E6F9F4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E6F9F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6F9FC: 4BFFFD75  bl 0x82e6f770
	ctx.lr = 0x82E6FA00;
	sub_82E6F770(ctx, base);
	// 82E6FA00: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6FA04: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E6FA08: 389A0001  addi r4, r26, 1
	ctx.r[4].s64 = ctx.r[26].s64 + 1;
	// 82E6FA0C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6FA10: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82E6FA14: 7D6B2670  srawi r11, r11, 4
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 4) as i64;
	// 82E6FA18: 38ABFFFF  addi r5, r11, -1
	ctx.r[5].s64 = ctx.r[11].s64 + -1;
	// 82E6FA1C: 48002B4D  bl 0x82e72568
	ctx.lr = 0x82E6FA20;
	sub_82E72568(ctx, base);
	// 82E6FA20: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6FA24: 574A2036  slwi r10, r26, 4
	ctx.r[10].u32 = ctx.r[26].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E6FA28: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82E6FA2C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E6FA30: 816BFFF0  lwz r11, -0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16 as u32) ) } as u64;
	// 82E6FA34: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82E6FA38: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 82E6FA3C: 48338760  b 0x831a819c
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6FA40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E6FA40 size=72
    let mut pc: u32 = 0x82E6FA40;
    'dispatch: loop {
        match pc {
            0x82E6FA40 => {
    //   block [0x82E6FA40..0x82E6FA88)
	// 82E6FA40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6FA44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E6FA48: F8A10020  std r5, 0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(32 as u32), ctx.r[5].u64 ) };
	// 82E6FA4C: F8C10028  std r6, 0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(40 as u32), ctx.r[6].u64 ) };
	// 82E6FA50: F8E10030  std r7, 0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(48 as u32), ctx.r[7].u64 ) };
	// 82E6FA54: F9010038  std r8, 0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(56 as u32), ctx.r[8].u64 ) };
	// 82E6FA58: F9210040  std r9, 0x40(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(64 as u32), ctx.r[9].u64 ) };
	// 82E6FA5C: F9410048  std r10, 0x48(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(72 as u32), ctx.r[10].u64 ) };
	// 82E6FA60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E6FA64: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82E6FA68: 39410080  addi r10, r1, 0x80
	ctx.r[10].s64 = ctx.r[1].s64 + 128;
	// 82E6FA6C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E6FA70: 80A10050  lwz r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E6FA74: 4BFFFD95  bl 0x82e6f808
	ctx.lr = 0x82E6FA78;
	sub_82E6F808(ctx, base);
	// 82E6FA78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E6FA7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E6FA80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E6FA84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6FA88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E6FA88 size=496
    let mut pc: u32 = 0x82E6FA88;
    'dispatch: loop {
        match pc {
            0x82E6FA88 => {
    //   block [0x82E6FA88..0x82E6FC78)
	// 82E6FA88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6FA8C: 483386DD  bl 0x831a8168
	ctx.lr = 0x82E6FA90;
	sub_831A8130(ctx, base);
	// 82E6FA90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E6FA94: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E6FA98: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E6FA9C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82E6FAA0: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E6FAA4: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82E6FAA8: 2F0B003D  cmpwi cr6, r11, 0x3d
	ctx.cr[6].compare_i32(ctx.r[11].s32, 61, &mut ctx.xer);
	// 82E6FAAC: 409A001C  bne cr6, 0x82e6fac8
	if !ctx.cr[6].eq {
	pc = 0x82E6FAC8; continue 'dispatch;
	}
	// 82E6FAB0: 389F0001  addi r4, r31, 1
	ctx.r[4].s64 = ctx.r[31].s64 + 1;
	// 82E6FAB4: 4833CA3D  bl 0x831ac4f0
	ctx.lr = 0x82E6FAB8;
	sub_831AC4F0(ctx, base);
	// 82E6FAB8: 7D7DE214  add r11, r29, r28
	ctx.r[11].u64 = ctx.r[29].u64 + ctx.r[28].u64;
	// 82E6FABC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E6FAC0: 994BFFFF  stb r10, -1(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-1 as u32), ctx.r[10].u8 ) };
	// 82E6FAC4: 480001AC  b 0x82e6fc70
	pc = 0x82E6FC70; continue 'dispatch;
	// 82E6FAC8: 2F0B0040  cmpwi cr6, r11, 0x40
	ctx.cr[6].compare_i32(ctx.r[11].s32, 64, &mut ctx.xer);
	// 82E6FACC: 409A00B0  bne cr6, 0x82e6fb7c
	if !ctx.cr[6].eq {
	pc = 0x82E6FB7C; continue 'dispatch;
	}
	// 82E6FAD0: 397F0001  addi r11, r31, 1
	ctx.r[11].s64 = ctx.r[31].s64 + 1;
	// 82E6FAD4: 393CFFF8  addi r9, r28, -8
	ctx.r[9].s64 = ctx.r[28].s64 + -8;
	// 82E6FAD8: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82E6FADC: 7D485378  mr r8, r10
	ctx.r[8].u64 = ctx.r[10].u64;
	// 82E6FAE0: 88EA0000  lbz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E6FAE4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E6FAE8: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82E6FAEC: 409AFFF4  bne cr6, 0x82e6fae0
	if !ctx.cr[6].eq {
	pc = 0x82E6FAE0; continue 'dispatch;
	}
	// 82E6FAF0: 7D485050  subf r10, r8, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[8].s64;
	// 82E6FAF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82E6FAF8: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82E6FAFC: 554A003E  slwi r10, r10, 0
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E6FB00: 991D0000  stb r8, 0(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[8].u8 ) };
	// 82E6FB04: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82E6FB08: 40990044  ble cr6, 0x82e6fb4c
	if !ctx.cr[6].gt {
	pc = 0x82E6FB4C; continue 'dispatch;
	}
	// 82E6FB0C: 7D295050  subf r9, r9, r10
	ctx.r[9].s64 = ctx.r[10].s64 - ctx.r[9].s64;
	// 82E6FB10: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82E6FB14: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82E6FB18: 394AD4C4  addi r10, r10, -0x2b3c
	ctx.r[10].s64 = ctx.r[10].s64 + -11068;
	// 82E6FB1C: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	// 82E6FB20: 89090000  lbz r8, 0(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E6FB24: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82E6FB28: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82E6FB2C: 409AFFF4  bne cr6, 0x82e6fb20
	if !ctx.cr[6].eq {
	pc = 0x82E6FB20; continue 'dispatch;
	}
	// 82E6FB30: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 82E6FB34: 890A0000  lbz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E6FB38: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E6FB3C: 28080000  cmplwi r8, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E6FB40: 99090000  stb r8, 0(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u8 ) };
	// 82E6FB44: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82E6FB48: 4082FFEC  bne 0x82e6fb34
	if !ctx.cr[0].eq {
	pc = 0x82E6FB34; continue 'dispatch;
	}
	// 82E6FB4C: 895D0000  lbz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E6FB50: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82E6FB54: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E6FB58: 409AFFF4  bne cr6, 0x82e6fb4c
	if !ctx.cr[6].eq {
	pc = 0x82E6FB4C; continue 'dispatch;
	}
	// 82E6FB5C: 395DFFFF  addi r10, r29, -1
	ctx.r[10].s64 = ctx.r[29].s64 + -1;
	// 82E6FB60: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E6FB64: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E6FB68: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E6FB6C: 992A0000  stb r9, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 82E6FB70: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E6FB74: 4082FFEC  bne 0x82e6fb60
	if !ctx.cr[0].eq {
	pc = 0x82E6FB60; continue 'dispatch;
	}
	// 82E6FB78: 480000F8  b 0x82e6fc70
	pc = 0x82E6FC70; continue 'dispatch;
	// 82E6FB7C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E6FB80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E6FB84: 388BD4C0  addi r4, r11, -0x2b40
	ctx.r[4].s64 = ctx.r[11].s64 + -11072;
	// 82E6FB88: 4833C091  bl 0x831abc18
	ctx.lr = 0x82E6FB8C;
	sub_831ABC18(ctx, base);
	// 82E6FB8C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E6FB90: 397CFFEF  addi r11, r28, -0x11
	ctx.r[11].s64 = ctx.r[28].s64 + -17;
	// 82E6FB94: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E6FB98: 40990008  ble cr6, 0x82e6fba0
	if !ctx.cr[6].gt {
	pc = 0x82E6FBA0; continue 'dispatch;
	}
	// 82E6FB9C: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 82E6FBA0: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E6FBA4: 38A0000A  li r5, 0xa
	ctx.r[5].s64 = 10;
	// 82E6FBA8: 388BD4B4  addi r4, r11, -0x2b4c
	ctx.r[4].s64 = ctx.r[11].s64 + -11084;
	// 82E6FBAC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E6FBB0: 48338961  bl 0x831a8510
	ctx.lr = 0x82E6FBB4;
	sub_831A8510(ctx, base);
	// 82E6FBB4: 7D7EF8AE  lbzx r11, r30, r31
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82E6FBB8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E6FBBC: 419A0050  beq cr6, 0x82e6fc0c
	if ctx.cr[6].eq {
	pc = 0x82E6FC0C; continue 'dispatch;
	}
	// 82E6FBC0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E6FBC4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E6FBC8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E6FBCC: 4834175D  bl 0x831b1328
	ctx.lr = 0x82E6FBD0;
	sub_831B1328(ctx, base);
	// 82E6FBD0: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E6FBD4: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	// 82E6FBD8: 396BD4C4  addi r11, r11, -0x2b3c
	ctx.r[11].s64 = ctx.r[11].s64 + -11068;
	// 82E6FBDC: 892A0000  lbz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E6FBE0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E6FBE4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82E6FBE8: 409AFFF4  bne cr6, 0x82e6fbdc
	if !ctx.cr[6].eq {
	pc = 0x82E6FBDC; continue 'dispatch;
	}
	// 82E6FBEC: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82E6FBF0: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E6FBF4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E6FBF8: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E6FBFC: 992A0000  stb r9, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 82E6FC00: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E6FC04: 4082FFEC  bne 0x82e6fbf0
	if !ctx.cr[0].eq {
	pc = 0x82E6FBF0; continue 'dispatch;
	}
	// 82E6FC08: 48000034  b 0x82e6fc3c
	pc = 0x82E6FC3C; continue 'dispatch;
	// 82E6FC0C: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82E6FC10: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E6FC14: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E6FC18: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E6FC1C: 409AFFF4  bne cr6, 0x82e6fc10
	if !ctx.cr[6].eq {
	pc = 0x82E6FC10; continue 'dispatch;
	}
	// 82E6FC20: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82E6FC24: 895F0000  lbz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E6FC28: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82E6FC2C: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E6FC30: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82E6FC34: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E6FC38: 4082FFEC  bne 0x82e6fc24
	if !ctx.cr[0].eq {
	pc = 0x82E6FC24; continue 'dispatch;
	}
	// 82E6FC3C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E6FC40: 396BD4B0  addi r11, r11, -0x2b50
	ctx.r[11].s64 = ctx.r[11].s64 + -11088;
	// 82E6FC44: 895D0000  lbz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E6FC48: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82E6FC4C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E6FC50: 409AFFF4  bne cr6, 0x82e6fc44
	if !ctx.cr[6].eq {
	pc = 0x82E6FC44; continue 'dispatch;
	}
	// 82E6FC54: 395DFFFF  addi r10, r29, -1
	ctx.r[10].s64 = ctx.r[29].s64 + -1;
	// 82E6FC58: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E6FC5C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E6FC60: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E6FC64: 992A0000  stb r9, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 82E6FC68: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E6FC6C: 4082FFEC  bne 0x82e6fc58
	if !ctx.cr[0].eq {
	pc = 0x82E6FC58; continue 'dispatch;
	}
	// 82E6FC70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E6FC74: 48338544  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6FC78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E6FC78 size=324
    let mut pc: u32 = 0x82E6FC78;
    'dispatch: loop {
        match pc {
            0x82E6FC78 => {
    //   block [0x82E6FC78..0x82E6FDBC)
	// 82E6FC78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6FC7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E6FC80: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E6FC84: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E6FC88: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E6FC8C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E6FC90: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E6FC94: 895F0005  lbz r10, 5(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(5 as u32) ) } as u64;
	// 82E6FC98: 554A003A  rlwinm r10, r10, 0, 0, 0x1d
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82E6FC9C: 995F0005  stb r10, 5(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(5 as u32), ctx.r[10].u8 ) };
	// 82E6FCA0: 48000064  b 0x82e6fd04
	pc = 0x82E6FD04; continue 'dispatch;
	// 82E6FCA4: 2F0B0005  cmpwi cr6, r11, 5
	ctx.cr[6].compare_i32(ctx.r[11].s32, 5, &mut ctx.xer);
	// 82E6FCA8: 419A00F0  beq cr6, 0x82e6fd98
	if ctx.cr[6].eq {
	pc = 0x82E6FD98; continue 'dispatch;
	}
	// 82E6FCAC: 2F0B0006  cmpwi cr6, r11, 6
	ctx.cr[6].compare_i32(ctx.r[11].s32, 6, &mut ctx.xer);
	// 82E6FCB0: 419A00DC  beq cr6, 0x82e6fd8c
	if ctx.cr[6].eq {
	pc = 0x82E6FD8C; continue 'dispatch;
	}
	// 82E6FCB4: 2F0B0007  cmpwi cr6, r11, 7
	ctx.cr[6].compare_i32(ctx.r[11].s32, 7, &mut ctx.xer);
	// 82E6FCB8: 409A005C  bne cr6, 0x82e6fd14
	if !ctx.cr[6].eq {
	pc = 0x82E6FD14; continue 'dispatch;
	}
	// 82E6FCBC: 897F0005  lbz r11, 5(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(5 as u32) ) } as u64;
	// 82E6FCC0: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6FCC4: 616B0004  ori r11, r11, 4
	ctx.r[11].u64 = ctx.r[11].u64 | 4;
	// 82E6FCC8: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E6FCCC: 997F0005  stb r11, 5(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(5 as u32), ctx.r[11].u8 ) };
	// 82E6FCD0: 41820018  beq 0x82e6fce8
	if ctx.cr[0].eq {
	pc = 0x82E6FCE8; continue 'dispatch;
	}
	// 82E6FCD4: 89640005  lbz r11, 5(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(5 as u32) ) } as u64;
	// 82E6FCD8: 556B07BF  clrlwi. r11, r11, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E6FCDC: 4182000C  beq 0x82e6fce8
	if ctx.cr[0].eq {
	pc = 0x82E6FCE8; continue 'dispatch;
	}
	// 82E6FCE0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E6FCE4: 4BFFFF95  bl 0x82e6fc78
	ctx.lr = 0x82E6FCE8;
	sub_82E6FC78(ctx, base);
	// 82E6FCE8: 83FF000C  lwz r31, 0xc(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E6FCEC: 897F0005  lbz r11, 5(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(5 as u32) ) } as u64;
	// 82E6FCF0: 556B07BF  clrlwi. r11, r11, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E6FCF4: 418200B0  beq 0x82e6fda4
	if ctx.cr[0].eq {
	pc = 0x82E6FDA4; continue 'dispatch;
	}
	// 82E6FCF8: 897F0005  lbz r11, 5(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(5 as u32) ) } as u64;
	// 82E6FCFC: 556B003A  rlwinm r11, r11, 0, 0, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E6FD00: 997F0005  stb r11, 5(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(5 as u32), ctx.r[11].u8 ) };
	// 82E6FD04: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E6FD08: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 82E6FD0C: 409AFF98  bne cr6, 0x82e6fca4
	if !ctx.cr[6].eq {
	pc = 0x82E6FCA4; continue 'dispatch;
	}
	// 82E6FD10: 48000094  b 0x82e6fda4
	pc = 0x82E6FDA4; continue 'dispatch;
	// 82E6FD14: 2F0B0008  cmpwi cr6, r11, 8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 8, &mut ctx.xer);
	// 82E6FD18: 419A0068  beq cr6, 0x82e6fd80
	if ctx.cr[6].eq {
	pc = 0x82E6FD80; continue 'dispatch;
	}
	// 82E6FD1C: 2F0B0009  cmpwi cr6, r11, 9
	ctx.cr[6].compare_i32(ctx.r[11].s32, 9, &mut ctx.xer);
	// 82E6FD20: 419A0054  beq cr6, 0x82e6fd74
	if ctx.cr[6].eq {
	pc = 0x82E6FD74; continue 'dispatch;
	}
	// 82E6FD24: 2F0B000A  cmpwi cr6, r11, 0xa
	ctx.cr[6].compare_i32(ctx.r[11].s32, 10, &mut ctx.xer);
	// 82E6FD28: 409A007C  bne cr6, 0x82e6fda4
	if !ctx.cr[6].eq {
	pc = 0x82E6FDA4; continue 'dispatch;
	}
	// 82E6FD2C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6FD30: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6FD34: 2F0A0004  cmpwi cr6, r10, 4
	ctx.cr[6].compare_i32(ctx.r[10].s32, 4, &mut ctx.xer);
	// 82E6FD38: 4198001C  blt cr6, 0x82e6fd54
	if ctx.cr[6].lt {
	pc = 0x82E6FD54; continue 'dispatch;
	}
	// 82E6FD3C: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E6FD40: 89640005  lbz r11, 5(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(5 as u32) ) } as u64;
	// 82E6FD44: 556B07BF  clrlwi. r11, r11, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E6FD48: 4182000C  beq 0x82e6fd54
	if ctx.cr[0].eq {
	pc = 0x82E6FD54; continue 'dispatch;
	}
	// 82E6FD4C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E6FD50: 4BFFFF29  bl 0x82e6fc78
	ctx.lr = 0x82E6FD54;
	sub_82E6FC78(ctx, base);
	// 82E6FD54: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6FD58: 395F0010  addi r10, r31, 0x10
	ctx.r[10].s64 = ctx.r[31].s64 + 16;
	// 82E6FD5C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82E6FD60: 409A0044  bne cr6, 0x82e6fda4
	if !ctx.cr[6].eq {
	pc = 0x82E6FDA4; continue 'dispatch;
	}
	// 82E6FD64: 897F0005  lbz r11, 5(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(5 as u32) ) } as u64;
	// 82E6FD68: 616B0004  ori r11, r11, 4
	ctx.r[11].u64 = ctx.r[11].u64 | 4;
	// 82E6FD6C: 997F0005  stb r11, 5(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(5 as u32), ctx.r[11].u8 ) };
	// 82E6FD70: 48000034  b 0x82e6fda4
	pc = 0x82E6FDA4; continue 'dispatch;
	// 82E6FD74: 817E0024  lwz r11, 0x24(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 82E6FD78: 917F0044  stw r11, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 82E6FD7C: 48000024  b 0x82e6fda0
	pc = 0x82E6FDA0; continue 'dispatch;
	// 82E6FD80: 817E0024  lwz r11, 0x24(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 82E6FD84: 917F006C  stw r11, 0x6c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82E6FD88: 48000018  b 0x82e6fda0
	pc = 0x82E6FDA0; continue 'dispatch;
	// 82E6FD8C: 817E0024  lwz r11, 0x24(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 82E6FD90: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E6FD94: 4800000C  b 0x82e6fda0
	pc = 0x82E6FDA0; continue 'dispatch;
	// 82E6FD98: 817E0024  lwz r11, 0x24(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 82E6FD9C: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82E6FDA0: 93FE0024  stw r31, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[31].u32 ) };
	// 82E6FDA4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E6FDA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E6FDAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E6FDB0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E6FDB4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E6FDB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6FDC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E6FDC0 size=236
    let mut pc: u32 = 0x82E6FDC0;
    'dispatch: loop {
        match pc {
            0x82E6FDC0 => {
    //   block [0x82E6FDC0..0x82E6FEAC)
	// 82E6FDC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6FDC4: 4833839D  bl 0x831a8160
	ctx.lr = 0x82E6FDC8;
	sub_831A8130(ctx, base);
	// 82E6FDC8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E6FDCC: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82E6FDD0: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82E6FDD4: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82E6FDD8: 83DB0010  lwz r30, 0x10(r27)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E6FDDC: 83BE0070  lwz r29, 0x70(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(112 as u32) ) } as u64;
	// 82E6FDE0: 480000B4  b 0x82e6fe94
	pc = 0x82E6FE94; continue 'dispatch;
	// 82E6FDE4: 897F0005  lbz r11, 5(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(5 as u32) ) } as u64;
	// 82E6FDE8: 556A07BF  clrlwi. r10, r11, 0x1e
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E6FDEC: 4082000C  bne 0x82e6fdf8
	if !ctx.cr[0].eq {
	pc = 0x82E6FDF8; continue 'dispatch;
	}
	// 82E6FDF0: 2F1A0000  cmpwi cr6, r26, 0
	ctx.cr[6].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 82E6FDF4: 419A000C  beq cr6, 0x82e6fe00
	if ctx.cr[6].eq {
	pc = 0x82E6FE00; continue 'dispatch;
	}
	// 82E6FDF8: 556B0739  rlwinm. r11, r11, 0, 0x1c, 0x1c
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E6FDFC: 4182000C  beq 0x82e6fe08
	if ctx.cr[0].eq {
	pc = 0x82E6FE08; continue 'dispatch;
	}
	// 82E6FE00: 7FFDFB78  mr r29, r31
	ctx.r[29].u64 = ctx.r[31].u64;
	// 82E6FE04: 48000090  b 0x82e6fe94
	pc = 0x82E6FE94; continue 'dispatch;
	// 82E6FE08: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6FE0C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E6FE10: 41820028  beq 0x82e6fe38
	if ctx.cr[0].eq {
	pc = 0x82E6FE38; continue 'dispatch;
	}
	// 82E6FE14: 89630006  lbz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82E6FE18: 556B077B  rlwinm. r11, r11, 0, 0x1d, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E6FE1C: 4082001C  bne 0x82e6fe38
	if !ctx.cr[0].eq {
	pc = 0x82E6FE38; continue 'dispatch;
	}
	// 82E6FE20: 817B0010  lwz r11, 0x10(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E6FE24: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 82E6FE28: 80AB00C4  lwz r5, 0xc4(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(196 as u32) ) } as u64;
	// 82E6FE2C: 48001D8D  bl 0x82e71bb8
	ctx.lr = 0x82E6FE30;
	sub_82E71BB8(ctx, base);
	// 82E6FE30: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E6FE34: 40820018  bne 0x82e6fe4c
	if !ctx.cr[0].eq {
	pc = 0x82E6FE4C; continue 'dispatch;
	}
	// 82E6FE38: 897F0005  lbz r11, 5(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(5 as u32) ) } as u64;
	// 82E6FE3C: 7FFDFB78  mr r29, r31
	ctx.r[29].u64 = ctx.r[31].u64;
	// 82E6FE40: 616B0008  ori r11, r11, 8
	ctx.r[11].u64 = ctx.r[11].u64 | 8;
	// 82E6FE44: 997F0005  stb r11, 5(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(5 as u32), ctx.r[11].u8 ) };
	// 82E6FE48: 4800004C  b 0x82e6fe94
	pc = 0x82E6FE94; continue 'dispatch;
	// 82E6FE4C: 897F0005  lbz r11, 5(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(5 as u32) ) } as u64;
	// 82E6FE50: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E6FE54: 61690008  ori r9, r11, 8
	ctx.r[9].u64 = ctx.r[11].u64 | 8;
	// 82E6FE58: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E6FE5C: 7D6BE214  add r11, r11, r28
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 82E6FE60: 3B8B0018  addi r28, r11, 0x18
	ctx.r[28].s64 = ctx.r[11].s64 + 24;
	// 82E6FE64: 993F0005  stb r9, 5(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(5 as u32), ctx.r[9].u8 ) };
	// 82E6FE68: 915D0000  stw r10, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E6FE6C: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E6FE70: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E6FE74: 4082000C  bne 0x82e6fe80
	if !ctx.cr[0].eq {
	pc = 0x82E6FE80; continue 'dispatch;
	}
	// 82E6FE78: 93FF0000  stw r31, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82E6FE7C: 48000014  b 0x82e6fe90
	pc = 0x82E6FE90; continue 'dispatch;
	// 82E6FE80: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E6FE84: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E6FE88: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E6FE8C: 93EB0000  stw r31, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82E6FE90: 93FE0030  stw r31, 0x30(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(48 as u32), ctx.r[31].u32 ) };
	// 82E6FE94: 83FD0000  lwz r31, 0(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E6FE98: 281F0000  cmplwi r31, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E6FE9C: 4082FF48  bne 0x82e6fde4
	if !ctx.cr[0].eq {
	pc = 0x82E6FDE4; continue 'dispatch;
	}
	// 82E6FEA0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82E6FEA4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E6FEA8: 48338308  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E6FEB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E6FEB0 size=528
    let mut pc: u32 = 0x82E6FEB0;
    'dispatch: loop {
        match pc {
            0x82E6FEB0 => {
    //   block [0x82E6FEB0..0x82E700C0)
	// 82E6FEB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E6FEB4: 483382B1  bl 0x831a8164
	ctx.lr = 0x82E6FEB8;
	sub_831A8130(ctx, base);
	// 82E6FEB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E6FEBC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E6FEC0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E6FEC4: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82E6FEC8: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82E6FECC: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6FED0: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E6FED4: 41820014  beq 0x82e6fee8
	if ctx.cr[0].eq {
	pc = 0x82E6FEE8; continue 'dispatch;
	}
	// 82E6FED8: 89640005  lbz r11, 5(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(5 as u32) ) } as u64;
	// 82E6FEDC: 556B07BF  clrlwi. r11, r11, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E6FEE0: 41820008  beq 0x82e6fee8
	if ctx.cr[0].eq {
	pc = 0x82E6FEE8; continue 'dispatch;
	}
	// 82E6FEE4: 4BFFFD95  bl 0x82e6fc78
	ctx.lr = 0x82E6FEE8;
	sub_82E6FC78(ctx, base);
	// 82E6FEE8: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6FEEC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E6FEF0: 418200BC  beq 0x82e6ffac
	if ctx.cr[0].eq {
	pc = 0x82E6FFAC; continue 'dispatch;
	}
	// 82E6FEF4: 89630006  lbz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82E6FEF8: 556B0739  rlwinm. r11, r11, 0, 0x1c, 0x1c
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E6FEFC: 408200B0  bne 0x82e6ffac
	if !ctx.cr[0].eq {
	pc = 0x82E6FFAC; continue 'dispatch;
	}
	// 82E6FF00: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82E6FF04: 80BD00C8  lwz r5, 0xc8(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(200 as u32) ) } as u64;
	// 82E6FF08: 48001CB1  bl 0x82e71bb8
	ctx.lr = 0x82E6FF0C;
	sub_82E71BB8(ctx, base);
	// 82E6FF0C: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82E6FF10: 4182009C  beq 0x82e6ffac
	if ctx.cr[0].eq {
	pc = 0x82E6FFAC; continue 'dispatch;
	}
	// 82E6FF14: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6FF18: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 82E6FF1C: 409A0090  bne cr6, 0x82e6ffac
	if !ctx.cr[6].eq {
	pc = 0x82E6FFAC; continue 'dispatch;
	}
	// 82E6FF20: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E6FF24: 3880006B  li r4, 0x6b
	ctx.r[4].s64 = 107;
	// 82E6FF28: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82E6FF2C: 4833CEF5  bl 0x831ace20
	ctx.lr = 0x82E6FF30;
	sub_831ACE20(ctx, base);
	// 82E6FF30: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E6FF34: 7C6A0034  cntlzw r10, r3
	ctx.r[10].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 82E6FF38: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82E6FF3C: 554BDFFE  rlwinm r11, r10, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 82E6FF40: 38800076  li r4, 0x76
	ctx.r[4].s64 = 118;
	// 82E6FF44: 697C0001  xori r28, r11, 1
	ctx.r[28].u64 = ctx.r[11].u64 ^ 1;
	// 82E6FF48: 4833CED9  bl 0x831ace20
	ctx.lr = 0x82E6FF4C;
	sub_831ACE20(ctx, base);
	// 82E6FF4C: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 82E6FF50: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82E6FF54: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82E6FF58: 697B0001  xori r27, r11, 1
	ctx.r[27].u64 = ctx.r[11].u64 ^ 1;
	// 82E6FF5C: 409A000C  bne cr6, 0x82e6ff68
	if !ctx.cr[6].eq {
	pc = 0x82E6FF68; continue 'dispatch;
	}
	// 82E6FF60: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 82E6FF64: 419A002C  beq cr6, 0x82e6ff90
	if ctx.cr[6].eq {
	pc = 0x82E6FF90; continue 'dispatch;
	}
	// 82E6FF68: 895E0005  lbz r10, 5(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(5 as u32) ) } as u64;
	// 82E6FF6C: 576B083C  slwi r11, r27, 1
	ctx.r[11].u32 = ctx.r[27].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E6FF70: 7D6BE378  or r11, r11, r28
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[28].u64;
	// 82E6FF74: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E6FF78: 714A00E7  andi. r10, r10, 0xe7
	ctx.r[10].u64 = ctx.r[10].u64 & 231;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E6FF7C: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 82E6FF80: 997E0005  stb r11, 5(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(5 as u32), ctx.r[11].u8 ) };
	// 82E6FF84: 817D002C  lwz r11, 0x2c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(44 as u32) ) } as u64;
	// 82E6FF88: 917E0018  stw r11, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82E6FF8C: 93DD002C  stw r30, 0x2c(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(44 as u32), ctx.r[30].u32 ) };
	// 82E6FF90: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82E6FF94: 419A0010  beq cr6, 0x82e6ffa4
	if ctx.cr[6].eq {
	pc = 0x82E6FFA4; continue 'dispatch;
	}
	// 82E6FF98: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 82E6FF9C: 419A0010  beq cr6, 0x82e6ffac
	if ctx.cr[6].eq {
	pc = 0x82E6FFAC; continue 'dispatch;
	}
	// 82E6FFA0: 48000114  b 0x82e700b4
	pc = 0x82E700B4; continue 'dispatch;
	// 82E6FFA4: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 82E6FFA8: 409A0050  bne cr6, 0x82e6fff8
	if !ctx.cr[6].eq {
	pc = 0x82E6FFF8; continue 'dispatch;
	}
	// 82E6FFAC: 813E001C  lwz r9, 0x1c(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E6FFB0: 2C090000  cmpwi r9, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82E6FFB4: 41820044  beq 0x82e6fff8
	if ctx.cr[0].eq {
	pc = 0x82E6FFF8; continue 'dispatch;
	}
	// 82E6FFB8: 55282036  slwi r8, r9, 4
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(4);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82E6FFBC: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E6FFC0: 3908FFF0  addi r8, r8, -0x10
	ctx.r[8].s64 = ctx.r[8].s64 + -16;
	// 82E6FFC4: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 82E6FFC8: 7D685A14  add r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 82E6FFCC: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E6FFD0: 2F0A0004  cmpwi cr6, r10, 4
	ctx.cr[6].compare_i32(ctx.r[10].s32, 4, &mut ctx.xer);
	// 82E6FFD4: 4198001C  blt cr6, 0x82e6fff0
	if ctx.cr[6].lt {
	pc = 0x82E6FFF0; continue 'dispatch;
	}
	// 82E6FFD8: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E6FFDC: 89640005  lbz r11, 5(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(5 as u32) ) } as u64;
	// 82E6FFE0: 556B07BF  clrlwi. r11, r11, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E6FFE4: 4182000C  beq 0x82e6fff0
	if ctx.cr[0].eq {
	pc = 0x82E6FFF0; continue 'dispatch;
	}
	// 82E6FFE8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E6FFEC: 4BFFFC8D  bl 0x82e6fc78
	ctx.lr = 0x82E6FFF0;
	sub_82E6FC78(ctx, base);
	// 82E6FFF0: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E6FFF4: 409AFFC8  bne cr6, 0x82e6ffbc
	if !ctx.cr[6].eq {
	pc = 0x82E6FFBC; continue 'dispatch;
	}
	// 82E6FFF8: 897E0007  lbz r11, 7(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(7 as u32) ) } as u64;
	// 82E6FFFC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82E70000: 7D485831  slw. r8, r10, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[10].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82E70004: 4182009C  beq 0x82e700a0
	if ctx.cr[0].eq {
	pc = 0x82E700A0; continue 'dispatch;
	}
	// 82E70008: 55072834  slwi r7, r8, 5
	ctx.r[7].u32 = ctx.r[8].u32.wrapping_shl(5);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82E7000C: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E70010: 38E7FFE0  addi r7, r7, -0x20
	ctx.r[7].s64 = ctx.r[7].s64 + -32;
	// 82E70014: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 82E70018: 7D2B3A14  add r9, r11, r7
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 82E7001C: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E70020: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E70024: 409A001C  bne cr6, 0x82e70040
	if !ctx.cr[6].eq {
	pc = 0x82E70040; continue 'dispatch;
	}
	// 82E70028: 81690018  lwz r11, 0x18(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E7002C: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 82E70030: 41980068  blt cr6, 0x82e70098
	if ctx.cr[6].lt {
	pc = 0x82E70098; continue 'dispatch;
	}
	// 82E70034: 3960000B  li r11, 0xb
	ctx.r[11].s64 = 11;
	// 82E70038: 91690018  stw r11, 0x18(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82E7003C: 4800005C  b 0x82e70098
	pc = 0x82E70098; continue 'dispatch;
	// 82E70040: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82E70044: 409A0028  bne cr6, 0x82e7006c
	if !ctx.cr[6].eq {
	pc = 0x82E7006C; continue 'dispatch;
	}
	// 82E70048: 81690018  lwz r11, 0x18(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E7004C: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 82E70050: 4198001C  blt cr6, 0x82e7006c
	if ctx.cr[6].lt {
	pc = 0x82E7006C; continue 'dispatch;
	}
	// 82E70054: 80890010  lwz r4, 0x10(r9)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E70058: 89640005  lbz r11, 5(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(5 as u32) ) } as u64;
	// 82E7005C: 556B07BF  clrlwi. r11, r11, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E70060: 4182000C  beq 0x82e7006c
	if ctx.cr[0].eq {
	pc = 0x82E7006C; continue 'dispatch;
	}
	// 82E70064: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E70068: 4BFFFC11  bl 0x82e6fc78
	ctx.lr = 0x82E7006C;
	sub_82E6FC78(ctx, base);
	// 82E7006C: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 82E70070: 409A0028  bne cr6, 0x82e70098
	if !ctx.cr[6].eq {
	pc = 0x82E70098; continue 'dispatch;
	}
	// 82E70074: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E70078: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 82E7007C: 4198001C  blt cr6, 0x82e70098
	if ctx.cr[6].lt {
	pc = 0x82E70098; continue 'dispatch;
	}
	// 82E70080: 80890000  lwz r4, 0(r9)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E70084: 89640005  lbz r11, 5(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(5 as u32) ) } as u64;
	// 82E70088: 556B07BF  clrlwi. r11, r11, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E7008C: 4182000C  beq 0x82e70098
	if ctx.cr[0].eq {
	pc = 0x82E70098; continue 'dispatch;
	}
	// 82E70090: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E70094: 4BFFFBE5  bl 0x82e6fc78
	ctx.lr = 0x82E70098;
	sub_82E6FC78(ctx, base);
	// 82E70098: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82E7009C: 409AFF70  bne cr6, 0x82e7000c
	if !ctx.cr[6].eq {
	pc = 0x82E7000C; continue 'dispatch;
	}
	// 82E700A0: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82E700A4: 409A0010  bne cr6, 0x82e700b4
	if !ctx.cr[6].eq {
	pc = 0x82E700B4; continue 'dispatch;
	}
	// 82E700A8: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 82E700AC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E700B0: 419A0008  beq cr6, 0x82e700b8
	if ctx.cr[6].eq {
	pc = 0x82E700B8; continue 'dispatch;
	}
	// 82E700B4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E700B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E700BC: 483380F8  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E700C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E700C0 size=384
    let mut pc: u32 = 0x82E700C0;
    'dispatch: loop {
        match pc {
            0x82E700C0 => {
    //   block [0x82E700C0..0x82E70240)
	// 82E700C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E700C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E700C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E700CC: 7C892378  mr r9, r4
	ctx.r[9].u64 = ctx.r[4].u64;
	// 82E700D0: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82E700D4: 81690020  lwz r11, 0x20(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E700D8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E700DC: 419A0014  beq cr6, 0x82e700f0
	if ctx.cr[6].eq {
	pc = 0x82E700F0; continue 'dispatch;
	}
	// 82E700E0: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E700E4: 894B0005  lbz r10, 5(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(5 as u32) ) } as u64;
	// 82E700E8: 554A003A  rlwinm r10, r10, 0, 0, 0x1d
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82E700EC: 994B0005  stb r10, 5(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(5 as u32), ctx.r[10].u8 ) };
	// 82E700F0: 81690028  lwz r11, 0x28(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(40 as u32) ) } as u64;
	// 82E700F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82E700F8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E700FC: 40990048  ble cr6, 0x82e70144
	if !ctx.cr[6].gt {
	pc = 0x82E70144; continue 'dispatch;
	}
	// 82E70100: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82E70104: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E70108: 7D685A14  add r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 82E7010C: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E70110: 2F0A0004  cmpwi cr6, r10, 4
	ctx.cr[6].compare_i32(ctx.r[10].s32, 4, &mut ctx.xer);
	// 82E70114: 4198001C  blt cr6, 0x82e70130
	if ctx.cr[6].lt {
	pc = 0x82E70130; continue 'dispatch;
	}
	// 82E70118: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E7011C: 89640005  lbz r11, 5(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(5 as u32) ) } as u64;
	// 82E70120: 556B07BF  clrlwi. r11, r11, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E70124: 4182000C  beq 0x82e70130
	if ctx.cr[0].eq {
	pc = 0x82E70130; continue 'dispatch;
	}
	// 82E70128: 7CC33378  mr r3, r6
	ctx.r[3].u64 = ctx.r[6].u64;
	// 82E7012C: 4BFFFB4D  bl 0x82e6fc78
	ctx.lr = 0x82E70130;
	sub_82E6FC78(ctx, base);
	// 82E70130: 81690028  lwz r11, 0x28(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(40 as u32) ) } as u64;
	// 82E70134: 38E70001  addi r7, r7, 1
	ctx.r[7].s64 = ctx.r[7].s64 + 1;
	// 82E70138: 39080010  addi r8, r8, 0x10
	ctx.r[8].s64 = ctx.r[8].s64 + 16;
	// 82E7013C: 7F075800  cmpw cr6, r7, r11
	ctx.cr[6].compare_i32(ctx.r[7].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E70140: 4198FFC4  blt cr6, 0x82e70104
	if ctx.cr[6].lt {
	pc = 0x82E70104; continue 'dispatch;
	}
	// 82E70144: 81690024  lwz r11, 0x24(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(36 as u32) ) } as u64;
	// 82E70148: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82E7014C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E70150: 40990040  ble cr6, 0x82e70190
	if !ctx.cr[6].gt {
	pc = 0x82E70190; continue 'dispatch;
	}
	// 82E70154: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E70158: 8149001C  lwz r10, 0x1c(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E7015C: 7D4A582E  lwzx r10, r10, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E70160: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E70164: 419A0018  beq cr6, 0x82e7017c
	if ctx.cr[6].eq {
	pc = 0x82E7017C; continue 'dispatch;
	}
	// 82E70168: 8149001C  lwz r10, 0x1c(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E7016C: 7D4A582E  lwzx r10, r10, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E70170: 88EA0005  lbz r7, 5(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(5 as u32) ) } as u64;
	// 82E70174: 54E7003A  rlwinm r7, r7, 0, 0, 0x1d
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0xFFFFFFFFu64;
	// 82E70178: 98EA0005  stb r7, 5(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(5 as u32), ctx.r[7].u8 ) };
	// 82E7017C: 81490024  lwz r10, 0x24(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(36 as u32) ) } as u64;
	// 82E70180: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 82E70184: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82E70188: 7F085000  cmpw cr6, r8, r10
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82E7018C: 4198FFCC  blt cr6, 0x82e70158
	if ctx.cr[6].lt {
	pc = 0x82E70158; continue 'dispatch;
	}
	// 82E70190: 81690034  lwz r11, 0x34(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(52 as u32) ) } as u64;
	// 82E70194: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82E70198: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E7019C: 40990044  ble cr6, 0x82e701e0
	if !ctx.cr[6].gt {
	pc = 0x82E701E0; continue 'dispatch;
	}
	// 82E701A0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82E701A4: 81690010  lwz r11, 0x10(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E701A8: 7D48582E  lwzx r10, r8, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E701AC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E701B0: 419A001C  beq cr6, 0x82e701cc
	if ctx.cr[6].eq {
	pc = 0x82E701CC; continue 'dispatch;
	}
	// 82E701B4: 5544003E  slwi r4, r10, 0
	ctx.r[4].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82E701B8: 89640005  lbz r11, 5(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(5 as u32) ) } as u64;
	// 82E701BC: 556B07BF  clrlwi. r11, r11, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E701C0: 4182000C  beq 0x82e701cc
	if ctx.cr[0].eq {
	pc = 0x82E701CC; continue 'dispatch;
	}
	// 82E701C4: 7CC33378  mr r3, r6
	ctx.r[3].u64 = ctx.r[6].u64;
	// 82E701C8: 4BFFFAB1  bl 0x82e6fc78
	ctx.lr = 0x82E701CC;
	sub_82E6FC78(ctx, base);
	// 82E701CC: 81690034  lwz r11, 0x34(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(52 as u32) ) } as u64;
	// 82E701D0: 38E70001  addi r7, r7, 1
	ctx.r[7].s64 = ctx.r[7].s64 + 1;
	// 82E701D4: 39080004  addi r8, r8, 4
	ctx.r[8].s64 = ctx.r[8].s64 + 4;
	// 82E701D8: 7F075800  cmpw cr6, r7, r11
	ctx.cr[6].compare_i32(ctx.r[7].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E701DC: 4198FFC8  blt cr6, 0x82e701a4
	if ctx.cr[6].lt {
	pc = 0x82E701A4; continue 'dispatch;
	}
	// 82E701E0: 81690038  lwz r11, 0x38(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(56 as u32) ) } as u64;
	// 82E701E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82E701E8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E701EC: 40990044  ble cr6, 0x82e70230
	if !ctx.cr[6].gt {
	pc = 0x82E70230; continue 'dispatch;
	}
	// 82E701F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E701F4: 81490018  lwz r10, 0x18(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E701F8: 7D4B502E  lwzx r10, r11, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E701FC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E70200: 419A001C  beq cr6, 0x82e7021c
	if ctx.cr[6].eq {
	pc = 0x82E7021C; continue 'dispatch;
	}
	// 82E70204: 81490018  lwz r10, 0x18(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E70208: 7CEB502E  lwzx r7, r11, r10
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E7020C: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 82E70210: 88E70005  lbz r7, 5(r7)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[7].u32.wrapping_add(5 as u32) ) } as u64;
	// 82E70214: 54E7003A  rlwinm r7, r7, 0, 0, 0x1d
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0xFFFFFFFFu64;
	// 82E70218: 98EA0005  stb r7, 5(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(5 as u32), ctx.r[7].u8 ) };
	// 82E7021C: 81490038  lwz r10, 0x38(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(56 as u32) ) } as u64;
	// 82E70220: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 82E70224: 396B000C  addi r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 + 12;
	// 82E70228: 7F085000  cmpw cr6, r8, r10
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82E7022C: 4198FFC8  blt cr6, 0x82e701f4
	if ctx.cr[6].lt {
	pc = 0x82E701F4; continue 'dispatch;
	}
	// 82E70230: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E70234: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E70238: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E7023C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E70240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E70240 size=236
    let mut pc: u32 = 0x82E70240;
    'dispatch: loop {
        match pc {
            0x82E70240 => {
    //   block [0x82E70240..0x82E7032C)
	// 82E70240: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E70244: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E70248: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E7024C: 7C892378  mr r9, r4
	ctx.r[9].u64 = ctx.r[4].u64;
	// 82E70250: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82E70254: 8089000C  lwz r4, 0xc(r9)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E70258: 89640005  lbz r11, 5(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(5 as u32) ) } as u64;
	// 82E7025C: 556B07BF  clrlwi. r11, r11, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E70260: 41820008  beq 0x82e70268
	if ctx.cr[0].eq {
	pc = 0x82E70268; continue 'dispatch;
	}
	// 82E70264: 4BFFFA15  bl 0x82e6fc78
	ctx.lr = 0x82E70268;
	sub_82E6FC78(ctx, base);
	// 82E70268: 89690006  lbz r11, 6(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(6 as u32) ) } as u64;
	// 82E7026C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E70270: 41820054  beq 0x82e702c4
	if ctx.cr[0].eq {
	pc = 0x82E702C4; continue 'dispatch;
	}
	// 82E70274: 89690007  lbz r11, 7(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(7 as u32) ) } as u64;
	// 82E70278: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82E7027C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E70280: 4182009C  beq 0x82e7031c
	if ctx.cr[0].eq {
	pc = 0x82E7031C; continue 'dispatch;
	}
	// 82E70284: 39090018  addi r8, r9, 0x18
	ctx.r[8].s64 = ctx.r[9].s64 + 24;
	// 82E70288: 81680008  lwz r11, 8(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E7028C: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 82E70290: 4198001C  blt cr6, 0x82e702ac
	if ctx.cr[6].lt {
	pc = 0x82E702AC; continue 'dispatch;
	}
	// 82E70294: 80880000  lwz r4, 0(r8)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E70298: 89640005  lbz r11, 5(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(5 as u32) ) } as u64;
	// 82E7029C: 556B07BF  clrlwi. r11, r11, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E702A0: 4182000C  beq 0x82e702ac
	if ctx.cr[0].eq {
	pc = 0x82E702AC; continue 'dispatch;
	}
	// 82E702A4: 7CC33378  mr r3, r6
	ctx.r[3].u64 = ctx.r[6].u64;
	// 82E702A8: 4BFFF9D1  bl 0x82e6fc78
	ctx.lr = 0x82E702AC;
	sub_82E6FC78(ctx, base);
	// 82E702AC: 89690007  lbz r11, 7(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(7 as u32) ) } as u64;
	// 82E702B0: 38E70001  addi r7, r7, 1
	ctx.r[7].s64 = ctx.r[7].s64 + 1;
	// 82E702B4: 39080010  addi r8, r8, 0x10
	ctx.r[8].s64 = ctx.r[8].s64 + 16;
	// 82E702B8: 7F075800  cmpw cr6, r7, r11
	ctx.cr[6].compare_i32(ctx.r[7].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E702BC: 4198FFCC  blt cr6, 0x82e70288
	if ctx.cr[6].lt {
	pc = 0x82E70288; continue 'dispatch;
	}
	// 82E702C0: 4800005C  b 0x82e7031c
	pc = 0x82E7031C; continue 'dispatch;
	// 82E702C4: 80890010  lwz r4, 0x10(r9)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E702C8: 89640005  lbz r11, 5(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(5 as u32) ) } as u64;
	// 82E702CC: 556B07BF  clrlwi. r11, r11, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E702D0: 4182000C  beq 0x82e702dc
	if ctx.cr[0].eq {
	pc = 0x82E702DC; continue 'dispatch;
	}
	// 82E702D4: 7CC33378  mr r3, r6
	ctx.r[3].u64 = ctx.r[6].u64;
	// 82E702D8: 4BFFF9A1  bl 0x82e6fc78
	ctx.lr = 0x82E702DC;
	sub_82E6FC78(ctx, base);
	// 82E702DC: 89690007  lbz r11, 7(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(7 as u32) ) } as u64;
	// 82E702E0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82E702E4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E702E8: 41820034  beq 0x82e7031c
	if ctx.cr[0].eq {
	pc = 0x82E7031C; continue 'dispatch;
	}
	// 82E702EC: 39090014  addi r8, r9, 0x14
	ctx.r[8].s64 = ctx.r[9].s64 + 20;
	// 82E702F0: 80880000  lwz r4, 0(r8)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E702F4: 89640005  lbz r11, 5(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(5 as u32) ) } as u64;
	// 82E702F8: 556B07BF  clrlwi. r11, r11, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E702FC: 4182000C  beq 0x82e70308
	if ctx.cr[0].eq {
	pc = 0x82E70308; continue 'dispatch;
	}
	// 82E70300: 7CC33378  mr r3, r6
	ctx.r[3].u64 = ctx.r[6].u64;
	// 82E70304: 4BFFF975  bl 0x82e6fc78
	ctx.lr = 0x82E70308;
	sub_82E6FC78(ctx, base);
	// 82E70308: 89690007  lbz r11, 7(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(7 as u32) ) } as u64;
	// 82E7030C: 38E70001  addi r7, r7, 1
	ctx.r[7].s64 = ctx.r[7].s64 + 1;
	// 82E70310: 39080004  addi r8, r8, 4
	ctx.r[8].s64 = ctx.r[8].s64 + 4;
	// 82E70314: 7F075800  cmpw cr6, r7, r11
	ctx.cr[6].compare_i32(ctx.r[7].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E70318: 4198FFD8  blt cr6, 0x82e702f0
	if ctx.cr[6].lt {
	pc = 0x82E702F0; continue 'dispatch;
	}
	// 82E7031C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E70320: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E70324: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E70328: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E70330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E70330 size=340
    let mut pc: u32 = 0x82E70330;
    'dispatch: loop {
        match pc {
            0x82E70330 => {
    //   block [0x82E70330..0x82E70484)
	// 82E70330: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E70334: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E70338: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E7033C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E70340: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E70344: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E70348: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82E7034C: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E70350: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 82E70354: 41980018  blt cr6, 0x82e7036c
	if ctx.cr[6].lt {
	pc = 0x82E7036C; continue 'dispatch;
	}
	// 82E70358: 809F0048  lwz r4, 0x48(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82E7035C: 89640005  lbz r11, 5(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(5 as u32) ) } as u64;
	// 82E70360: 556B07BF  clrlwi. r11, r11, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E70364: 41820008  beq 0x82e7036c
	if ctx.cr[0].eq {
	pc = 0x82E7036C; continue 'dispatch;
	}
	// 82E70368: 4BFFF911  bl 0x82e6fc78
	ctx.lr = 0x82E7036C;
	sub_82E6FC78(ctx, base);
	// 82E7036C: 80FF0008  lwz r7, 8(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E70370: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82E70374: 813F0014  lwz r9, 0x14(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E70378: 7CE83B78  mr r8, r7
	ctx.r[8].u64 = ctx.r[7].u64;
	// 82E7037C: 48000018  b 0x82e70394
	pc = 0x82E70394; continue 'dispatch;
	// 82E70380: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E70384: 7F085040  cmplw cr6, r8, r10
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82E70388: 40980008  bge cr6, 0x82e70390
	if !ctx.cr[6].lt {
	pc = 0x82E70390; continue 'dispatch;
	}
	// 82E7038C: 7D485378  mr r8, r10
	ctx.r[8].u64 = ctx.r[10].u64;
	// 82E70390: 396B0018  addi r11, r11, 0x18
	ctx.r[11].s64 = ctx.r[11].s64 + 24;
	// 82E70394: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82E70398: 4099FFE8  ble cr6, 0x82e70380
	if !ctx.cr[6].gt {
	pc = 0x82E70380; continue 'dispatch;
	}
	// 82E7039C: 813F0020  lwz r9, 0x20(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E703A0: 7F093840  cmplw cr6, r9, r7
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82E703A4: 40980048  bge cr6, 0x82e703ec
	if !ctx.cr[6].lt {
	pc = 0x82E703EC; continue 'dispatch;
	}
	// 82E703A8: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E703AC: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 82E703B0: 4198001C  blt cr6, 0x82e703cc
	if ctx.cr[6].lt {
	pc = 0x82E703CC; continue 'dispatch;
	}
	// 82E703B4: 80890000  lwz r4, 0(r9)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E703B8: 89640005  lbz r11, 5(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(5 as u32) ) } as u64;
	// 82E703BC: 556B07BF  clrlwi. r11, r11, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E703C0: 4182000C  beq 0x82e703cc
	if ctx.cr[0].eq {
	pc = 0x82E703CC; continue 'dispatch;
	}
	// 82E703C4: 7CC33378  mr r3, r6
	ctx.r[3].u64 = ctx.r[6].u64;
	// 82E703C8: 4BFFF8B1  bl 0x82e6fc78
	ctx.lr = 0x82E703CC;
	sub_82E6FC78(ctx, base);
	// 82E703CC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E703D0: 39290010  addi r9, r9, 0x10
	ctx.r[9].s64 = ctx.r[9].s64 + 16;
	// 82E703D4: 7F095840  cmplw cr6, r9, r11
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E703D8: 4198FFD0  blt cr6, 0x82e703a8
	if ctx.cr[6].lt {
	pc = 0x82E703A8; continue 'dispatch;
	}
	// 82E703DC: 48000010  b 0x82e703ec
	pc = 0x82E703EC; continue 'dispatch;
	// 82E703E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E703E4: 91690008  stw r11, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E703E8: 39290010  addi r9, r9, 0x10
	ctx.r[9].s64 = ctx.r[9].s64 + 16;
	// 82E703EC: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82E703F0: 4099FFF0  ble cr6, 0x82e703e0
	if !ctx.cr[6].gt {
	pc = 0x82E703E0; continue 'dispatch;
	}
	// 82E703F4: 815F0020  lwz r10, 0x20(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E703F8: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E703FC: 7D4A4050  subf r10, r10, r8
	ctx.r[10].s64 = ctx.r[8].s64 - ctx.r[10].s64;
	// 82E70400: 2F0B4E20  cmpwi cr6, r11, 0x4e20
	ctx.cr[6].compare_i32(ctx.r[11].s32, 20000, &mut ctx.xer);
	// 82E70404: 7D5E2670  srawi r30, r10, 4
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[30].s64 = (ctx.r[10].s32 >> 4) as i64;
	// 82E70408: 41990064  bgt cr6, 0x82e7046c
	if ctx.cr[6].gt {
	pc = 0x82E7046C; continue 'dispatch;
	}
	// 82E7040C: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E70410: 39000018  li r8, 0x18
	ctx.r[8].s64 = 24;
	// 82E70414: 813F0028  lwz r9, 0x28(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82E70418: 7D495050  subf r10, r9, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[9].s64;
	// 82E7041C: 7D4A43D6  divw r10, r10, r8
	ctx.r[10].s32 = ctx.r[10].s32 / ctx.r[8].s32;
	// 82E70420: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E70424: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E70428: 4098001C  bge cr6, 0x82e70444
	if !ctx.cr[6].lt {
	pc = 0x82E70444; continue 'dispatch;
	}
	// 82E7042C: 2F0B0010  cmpwi cr6, r11, 0x10
	ctx.cr[6].compare_i32(ctx.r[11].s32, 16, &mut ctx.xer);
	// 82E70430: 40990014  ble cr6, 0x82e70444
	if !ctx.cr[6].gt {
	pc = 0x82E70444; continue 'dispatch;
	}
	// 82E70434: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82E70438: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E7043C: 7C8B0194  addze r4, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[4].s64 = tmp.s64;
	// 82E70440: 4BFF9931  bl 0x82e69d70
	ctx.lr = 0x82E70444;
	sub_82E69D70(ctx, base);
	// 82E70444: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82E70448: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E7044C: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E70450: 4098001C  bge cr6, 0x82e7046c
	if !ctx.cr[6].lt {
	pc = 0x82E7046C; continue 'dispatch;
	}
	// 82E70454: 2F0B005A  cmpwi cr6, r11, 0x5a
	ctx.cr[6].compare_i32(ctx.r[11].s32, 90, &mut ctx.xer);
	// 82E70458: 40990014  ble cr6, 0x82e7046c
	if !ctx.cr[6].gt {
	pc = 0x82E7046C; continue 'dispatch;
	}
	// 82E7045C: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82E70460: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E70464: 7C8B0194  addze r4, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[4].s64 = tmp.s64;
	// 82E70468: 4BFF97D1  bl 0x82e69c38
	ctx.lr = 0x82E7046C;
	sub_82E69C38(ctx, base);
	// 82E7046C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E70470: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E70474: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E70478: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E7047C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E70480: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E70488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E70488 size=368
    let mut pc: u32 = 0x82E70488;
    'dispatch: loop {
        match pc {
            0x82E70488 => {
    //   block [0x82E70488..0x82E705F8)
	// 82E70488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E7048C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E70490: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E70494: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E70498: 83E30024  lwz r31, 0x24(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 82E7049C: 895F0005  lbz r10, 5(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(5 as u32) ) } as u64;
	// 82E704A0: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E704A4: 614A0004  ori r10, r10, 4
	ctx.r[10].u64 = ctx.r[10].u64 | 4;
	// 82E704A8: 2F0B0005  cmpwi cr6, r11, 5
	ctx.cr[6].compare_i32(ctx.r[11].s32, 5, &mut ctx.xer);
	// 82E704AC: 995F0005  stb r10, 5(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(5 as u32), ctx.r[10].u8 ) };
	// 82E704B0: 419A00F0  beq cr6, 0x82e705a0
	if ctx.cr[6].eq {
	pc = 0x82E705A0; continue 'dispatch;
	}
	// 82E704B4: 2F0B0006  cmpwi cr6, r11, 6
	ctx.cr[6].compare_i32(ctx.r[11].s32, 6, &mut ctx.xer);
	// 82E704B8: 419A00AC  beq cr6, 0x82e70564
	if ctx.cr[6].eq {
	pc = 0x82E70564; continue 'dispatch;
	}
	// 82E704BC: 2F0B0008  cmpwi cr6, r11, 8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 8, &mut ctx.xer);
	// 82E704C0: 419A0060  beq cr6, 0x82e70520
	if ctx.cr[6].eq {
	pc = 0x82E70520; continue 'dispatch;
	}
	// 82E704C4: 2F0B0009  cmpwi cr6, r11, 9
	ctx.cr[6].compare_i32(ctx.r[11].s32, 9, &mut ctx.xer);
	// 82E704C8: 419A000C  beq cr6, 0x82e704d4
	if ctx.cr[6].eq {
	pc = 0x82E704D4; continue 'dispatch;
	}
	// 82E704CC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E704D0: 48000114  b 0x82e705e4
	pc = 0x82E705E4; continue 'dispatch;
	// 82E704D4: 817F0044  lwz r11, 0x44(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 82E704D8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E704DC: 91630024  stw r11, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82E704E0: 4BFFFBE1  bl 0x82e700c0
	ctx.lr = 0x82E704E4;
	sub_82E700C0(ctx, base);
	// 82E704E4: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82E704E8: 80DF0028  lwz r6, 0x28(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82E704EC: 1D6B0003  mulli r11, r11, 3
	ctx.r[11].s64 = ctx.r[11].s64 * 3;
	// 82E704F0: 80FF0034  lwz r7, 0x34(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82E704F4: 811F0030  lwz r8, 0x30(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E704F8: 813F002C  lwz r9, 0x2c(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82E704FC: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82E70500: 54C6103A  slwi r6, r6, 2
	ctx.r[6].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82E70504: 7D6B3214  add r11, r11, r6
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[6].u64;
	// 82E70508: 7D6B3A14  add r11, r11, r7
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 82E7050C: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82E70510: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82E70514: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82E70518: 396B0013  addi r11, r11, 0x13
	ctx.r[11].s64 = ctx.r[11].s64 + 19;
	// 82E7051C: 4800007C  b 0x82e70598
	pc = 0x82E70598; continue 'dispatch;
	// 82E70520: 817F006C  lwz r11, 0x6c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 82E70524: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E70528: 81430028  lwz r10, 0x28(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82E7052C: 91630024  stw r11, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82E70530: 915F006C  stw r10, 0x6c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[10].u32 ) };
	// 82E70534: 93E30028  stw r31, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[31].u32 ) };
	// 82E70538: 897F0005  lbz r11, 5(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(5 as u32) ) } as u64;
	// 82E7053C: 716B00FB  andi. r11, r11, 0xfb
	ctx.r[11].u64 = ctx.r[11].u64 & 251;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E70540: 997F0005  stb r11, 5(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(5 as u32), ctx.r[11].u8 ) };
	// 82E70544: 4BFFFDED  bl 0x82e70330
	ctx.lr = 0x82E70548;
	sub_82E70330(ctx, base);
	// 82E70548: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E7054C: 815F002C  lwz r10, 0x2c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82E70550: 396B0005  addi r11, r11, 5
	ctx.r[11].s64 = ctx.r[11].s64 + 5;
	// 82E70554: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E70558: 1D6B0018  mulli r11, r11, 0x18
	ctx.r[11].s64 = ctx.r[11].s64 * 24;
	// 82E7055C: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82E70560: 48000084  b 0x82e705e4
	pc = 0x82E705E4; continue 'dispatch;
	// 82E70564: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E70568: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E7056C: 91630024  stw r11, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82E70570: 4BFFFCD1  bl 0x82e70240
	ctx.lr = 0x82E70574;
	sub_82E70240(ctx, base);
	// 82E70574: 897F0006  lbz r11, 6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 82E70578: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E7057C: 41820014  beq 0x82e70590
	if ctx.cr[0].eq {
	pc = 0x82E70590; continue 'dispatch;
	}
	// 82E70580: 897F0007  lbz r11, 7(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(7 as u32) ) } as u64;
	// 82E70584: 556B203E  rotlwi r11, r11, 4
	ctx.r[11].u64 = ((ctx.r[11].u32).rotate_left(4)) as u64;
	// 82E70588: 386B0018  addi r3, r11, 0x18
	ctx.r[3].s64 = ctx.r[11].s64 + 24;
	// 82E7058C: 48000058  b 0x82e705e4
	pc = 0x82E705E4; continue 'dispatch;
	// 82E70590: 897F0007  lbz r11, 7(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(7 as u32) ) } as u64;
	// 82E70594: 396B0005  addi r11, r11, 5
	ctx.r[11].s64 = ctx.r[11].s64 + 5;
	// 82E70598: 5563103A  slwi r3, r11, 2
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82E7059C: 48000048  b 0x82e705e4
	pc = 0x82E705E4; continue 'dispatch;
	// 82E705A0: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E705A4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E705A8: 91630024  stw r11, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82E705AC: 4BFFF905  bl 0x82e6feb0
	ctx.lr = 0x82E705B0;
	sub_82E6FEB0(ctx, base);
	// 82E705B0: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82E705B4: 41820010  beq 0x82e705c4
	if ctx.cr[0].eq {
	pc = 0x82E705C4; continue 'dispatch;
	}
	// 82E705B8: 897F0005  lbz r11, 5(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(5 as u32) ) } as u64;
	// 82E705BC: 716B00FB  andi. r11, r11, 0xfb
	ctx.r[11].u64 = ctx.r[11].u64 & 251;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E705C0: 997F0005  stb r11, 5(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(5 as u32), ctx.r[11].u8 ) };
	// 82E705C4: 897F0007  lbz r11, 7(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(7 as u32) ) } as u64;
	// 82E705C8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82E705CC: 815F001C  lwz r10, 0x1c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E705D0: 7D2B5830  slw r11, r9, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[9].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 82E705D4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E705D8: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E705DC: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82E705E0: 55632036  slwi r3, r11, 4
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82E705E4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E705E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E705EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E705F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E705F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E705F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E705F8 size=8
    let mut pc: u32 = 0x82E705F8;
    'dispatch: loop {
        match pc {
            0x82E705F8 => {
    //   block [0x82E705F8..0x82E70600)
	// 82E705F8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E705FC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E70600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E70600 size=84
    let mut pc: u32 = 0x82E70600;
    'dispatch: loop {
        match pc {
            0x82E70600 => {
    //   block [0x82E70600..0x82E70654)
	// 82E70600: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E70604: 89630005  lbz r11, 5(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(5 as u32) ) } as u64;
	// 82E70608: 8103001C  lwz r8, 0x1c(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E7060C: 556B06F7  rlwinm. r11, r11, 0, 0x1b, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E70610: 41820070  beq 0x82e70680
	if ctx.cr[0].eq {
		sub_82E70654(ctx, base);
		return;
	}
	// 82E70614: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82E70618: 419A0068  beq cr6, 0x82e70680
	if ctx.cr[6].eq {
		sub_82E70654(ctx, base);
		return;
	}
	// 82E7061C: 55092036  slwi r9, r8, 4
	ctx.r[9].u32 = ctx.r[8].u32.wrapping_shl(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82E70620: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E70624: 3929FFF0  addi r9, r9, -0x10
	ctx.r[9].s64 = ctx.r[9].s64 + -16;
	// 82E70628: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 82E7062C: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82E70630: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E70634: 2F0A0004  cmpwi cr6, r10, 4
	ctx.cr[6].compare_i32(ctx.r[10].s32, 4, &mut ctx.xer);
	// 82E70638: 41980040  blt cr6, 0x82e70678
	if ctx.cr[6].lt {
		sub_82E70654(ctx, base);
		return;
	}
	// 82E7063C: 409A0018  bne cr6, 0x82e70654
	if !ctx.cr[6].eq {
		sub_82E70654(ctx, base);
		return;
	}
	// 82E70640: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E70644: 894B0005  lbz r10, 5(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(5 as u32) ) } as u64;
	// 82E70648: 554A003A  rlwinm r10, r10, 0, 0, 0x1d
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82E7064C: 994B0005  stb r10, 5(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(5 as u32), ctx.r[10].u8 ) };
	// 82E70650: 48000028  b 0x82e70678
	sub_82E70654(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E70654(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E70654 size=128
    let mut pc: u32 = 0x82E70654;
    'dispatch: loop {
        match pc {
            0x82E70654 => {
    //   block [0x82E70654..0x82E706D4)
	// 82E70654: 80EB0000  lwz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E70658: 88E70005  lbz r7, 5(r7)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[7].u32.wrapping_add(5 as u32) ) } as u64;
	// 82E7065C: 54E507BF  clrlwi. r5, r7, 0x1e
	ctx.r[5].u64 = ctx.r[7].u32 as u64 & 0x00000003u64;
	ctx.cr[0].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82E70660: 40820014  bne 0x82e70674
	if !ctx.cr[0].eq {
	pc = 0x82E70674; continue 'dispatch;
	}
	// 82E70664: 2F0A0007  cmpwi cr6, r10, 7
	ctx.cr[6].compare_i32(ctx.r[10].s32, 7, &mut ctx.xer);
	// 82E70668: 409A0010  bne cr6, 0x82e70678
	if !ctx.cr[6].eq {
	pc = 0x82E70678; continue 'dispatch;
	}
	// 82E7066C: 54EA0739  rlwinm. r10, r7, 0, 0x1c, 0x1c
	ctx.r[10].u64 = ctx.r[7].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E70670: 41820008  beq 0x82e70678
	if ctx.cr[0].eq {
	pc = 0x82E70678; continue 'dispatch;
	}
	// 82E70674: 90CB0008  stw r6, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 82E70678: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82E7067C: 409AFFA4  bne cr6, 0x82e70620
	if !ctx.cr[6].eq {
		sub_82E70600(ctx, base);
		return;
	}
	// 82E70680: 89630007  lbz r11, 7(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(7 as u32) ) } as u64;
	// 82E70684: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82E70688: 7D485831  slw. r8, r10, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[8].u64 = 0;
	} else {
		ctx.r[8].u64 = ((ctx.r[10].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82E7068C: 418200B8  beq 0x82e70744
	if ctx.cr[0].eq {
		sub_82E70704(ctx, base);
		return;
	}
	// 82E70690: 55072834  slwi r7, r8, 5
	ctx.r[7].u32 = ctx.r[8].u32.wrapping_shl(5);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82E70694: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E70698: 38E7FFE0  addi r7, r7, -0x20
	ctx.r[7].s64 = ctx.r[7].s64 + -32;
	// 82E7069C: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 82E706A0: 7D675A14  add r11, r7, r11
	ctx.r[11].u64 = ctx.r[7].u64 + ctx.r[11].u64;
	// 82E706A4: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E706A8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E706AC: 419A0090  beq cr6, 0x82e7073c
	if ctx.cr[6].eq {
		sub_82E70704(ctx, base);
		return;
	}
	// 82E706B0: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E706B4: 2F0A0004  cmpwi cr6, r10, 4
	ctx.cr[6].compare_i32(ctx.r[10].s32, 4, &mut ctx.xer);
	// 82E706B8: 41980028  blt cr6, 0x82e706e0
	if ctx.cr[6].lt {
		sub_82E706D4(ctx, base);
		return;
	}
	// 82E706BC: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E706C0: 409A0014  bne cr6, 0x82e706d4
	if !ctx.cr[6].eq {
		sub_82E706D4(ctx, base);
		return;
	}
	// 82E706C4: 892A0005  lbz r9, 5(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(5 as u32) ) } as u64;
	// 82E706C8: 5529003A  rlwinm r9, r9, 0, 0, 0x1d
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 82E706CC: 992A0005  stb r9, 5(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(5 as u32), ctx.r[9].u8 ) };
	// 82E706D0: 48000010  b 0x82e706e0
	sub_82E706D4(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E706D4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E706D4 size=48
    let mut pc: u32 = 0x82E706D4;
    'dispatch: loop {
        match pc {
            0x82E706D4 => {
    //   block [0x82E706D4..0x82E70704)
	// 82E706D4: 894A0005  lbz r10, 5(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(5 as u32) ) } as u64;
	// 82E706D8: 554A07BF  clrlwi. r10, r10, 0x1e
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000003u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E706DC: 40820048  bne 0x82e70724
	if !ctx.cr[0].eq {
		sub_82E70704(ctx, base);
		return;
	}
	// 82E706E0: 812B0008  lwz r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E706E4: 2F090004  cmpwi cr6, r9, 4
	ctx.cr[6].compare_i32(ctx.r[9].s32, 4, &mut ctx.xer);
	// 82E706E8: 41980054  blt cr6, 0x82e7073c
	if ctx.cr[6].lt {
		sub_82E70704(ctx, base);
		return;
	}
	// 82E706EC: 409A0018  bne cr6, 0x82e70704
	if !ctx.cr[6].eq {
		sub_82E70704(ctx, base);
		return;
	}
	// 82E706F0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E706F4: 894B0005  lbz r10, 5(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(5 as u32) ) } as u64;
	// 82E706F8: 554A003A  rlwinm r10, r10, 0, 0, 0x1d
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82E706FC: 994B0005  stb r10, 5(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(5 as u32), ctx.r[10].u8 ) };
	// 82E70700: 4800003C  b 0x82e7073c
	sub_82E70704(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E70704(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E70704 size=80
    let mut pc: u32 = 0x82E70704;
    'dispatch: loop {
        match pc {
            0x82E70704 => {
    //   block [0x82E70704..0x82E70754)
	// 82E70704: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E70708: 894A0005  lbz r10, 5(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(5 as u32) ) } as u64;
	// 82E7070C: 554507BF  clrlwi. r5, r10, 0x1e
	ctx.r[5].u64 = ctx.r[10].u32 as u64 & 0x00000003u64;
	ctx.cr[0].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82E70710: 40820014  bne 0x82e70724
	if !ctx.cr[0].eq {
	pc = 0x82E70724; continue 'dispatch;
	}
	// 82E70714: 2F090007  cmpwi cr6, r9, 7
	ctx.cr[6].compare_i32(ctx.r[9].s32, 7, &mut ctx.xer);
	// 82E70718: 409A0024  bne cr6, 0x82e7073c
	if !ctx.cr[6].eq {
	pc = 0x82E7073C; continue 'dispatch;
	}
	// 82E7071C: 554A0739  rlwinm. r10, r10, 0, 0x1c, 0x1c
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E70720: 4182001C  beq 0x82e7073c
	if ctx.cr[0].eq {
	pc = 0x82E7073C; continue 'dispatch;
	}
	// 82E70724: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E70728: 90CB0008  stw r6, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 82E7072C: 2F0A0004  cmpwi cr6, r10, 4
	ctx.cr[6].compare_i32(ctx.r[10].s32, 4, &mut ctx.xer);
	// 82E70730: 4198000C  blt cr6, 0x82e7073c
	if ctx.cr[6].lt {
	pc = 0x82E7073C; continue 'dispatch;
	}
	// 82E70734: 3940000B  li r10, 0xb
	ctx.r[10].s64 = 11;
	// 82E70738: 914B0018  stw r10, 0x18(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 82E7073C: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82E70740: 409AFF54  bne cr6, 0x82e70694
	if !ctx.cr[6].eq {
		sub_82E70654(ctx, base);
		return;
	}
	// 82E70744: 80630018  lwz r3, 0x18(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E70748: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E7074C: 4082FEB8  bne 0x82e70604
	if !ctx.cr[0].eq {
		sub_82E70600(ctx, base);
		return;
	}
	// 82E70750: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E70758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E70758 size=364
    let mut pc: u32 = 0x82E70758;
    'dispatch: loop {
        match pc {
            0x82E70758 => {
    //   block [0x82E70758..0x82E708C4)
	// 82E70758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E7075C: 48337A05  bl 0x831a8160
	ctx.lr = 0x82E70760;
	sub_831A8130(ctx, base);
	// 82E70760: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E70764: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E70768: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82E7076C: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 82E70770: 839E0010  lwz r28, 0x10(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E70774: 897C0014  lbz r11, 0x14(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E70778: 697B0003  xori r27, r11, 3
	ctx.r[27].u64 = ctx.r[11].u64 ^ 3;
	// 82E7077C: 48000130  b 0x82e708ac
	pc = 0x82E708AC; continue 'dispatch;
	// 82E70780: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 82E70784: 3B5AFFFF  addi r26, r26, -1
	ctx.r[26].s64 = ctx.r[26].s64 + -1;
	// 82E70788: 419A0130  beq cr6, 0x82e708b8
	if ctx.cr[6].eq {
	pc = 0x82E708B8; continue 'dispatch;
	}
	// 82E7078C: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E70790: 2B0B0008  cmplwi cr6, r11, 8
	ctx.cr[6].compare_u32(ctx.r[11].u32, 8 as u32, &mut ctx.xer);
	// 82E70794: 409A0014  bne cr6, 0x82e707a8
	if !ctx.cr[6].eq {
	pc = 0x82E707A8; continue 'dispatch;
	}
	// 82E70798: 38A0FFFD  li r5, -3
	ctx.r[5].s64 = -3;
	// 82E7079C: 389F0068  addi r4, r31, 0x68
	ctx.r[4].s64 = ctx.r[31].s64 + 104;
	// 82E707A0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E707A4: 4BFFFFB5  bl 0x82e70758
	ctx.lr = 0x82E707A8;
	sub_82E70758(ctx, base);
	// 82E707A8: 897F0005  lbz r11, 5(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(5 as u32) ) } as u64;
	// 82E707AC: 696A0003  xori r10, r11, 3
	ctx.r[10].u64 = ctx.r[11].u64 ^ 3;
	// 82E707B0: 7D4AD839  and. r10, r10, r27
	ctx.r[10].u64 = ctx.r[10].u64 & ctx.r[27].u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E707B4: 4182001C  beq 0x82e707d0
	if ctx.cr[0].eq {
	pc = 0x82E707D0; continue 'dispatch;
	}
	// 82E707B8: 895C0014  lbz r10, 0x14(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E707BC: 7FFDFB78  mr r29, r31
	ctx.r[29].u64 = ctx.r[31].u64;
	// 82E707C0: 516A0638  rlwimi r10, r11, 0, 0x18, 0x1c
	ctx.r[10].u64 = (((ctx.r[11].u32).rotate_left(0) as u64) & 0x00000000000000F8) | (ctx.r[10].u64 & 0xFFFFFFFFFFFFFF07);
	// 82E707C4: 714B00FB  andi. r11, r10, 0xfb
	ctx.r[11].u64 = ctx.r[10].u64 & 251;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E707C8: 997F0005  stb r11, 5(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(5 as u32), ctx.r[11].u8 ) };
	// 82E707CC: 480000E0  b 0x82e708ac
	pc = 0x82E708AC; continue 'dispatch;
	// 82E707D0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E707D4: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E707D8: 817C001C  lwz r11, 0x1c(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E707DC: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E707E0: 409A000C  bne cr6, 0x82e707ec
	if !ctx.cr[6].eq {
	pc = 0x82E707EC; continue 'dispatch;
	}
	// 82E707E4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E707E8: 917C001C  stw r11, 0x1c(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82E707EC: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E707F0: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 82E707F4: 419A0090  beq cr6, 0x82e70884
	if ctx.cr[6].eq {
	pc = 0x82E70884; continue 'dispatch;
	}
	// 82E707F8: 2F0B0005  cmpwi cr6, r11, 5
	ctx.cr[6].compare_i32(ctx.r[11].s32, 5, &mut ctx.xer);
	// 82E707FC: 419A0078  beq cr6, 0x82e70874
	if ctx.cr[6].eq {
	pc = 0x82E70874; continue 'dispatch;
	}
	// 82E70800: 2F0B0006  cmpwi cr6, r11, 6
	ctx.cr[6].compare_i32(ctx.r[11].s32, 6, &mut ctx.xer);
	// 82E70804: 419A0060  beq cr6, 0x82e70864
	if ctx.cr[6].eq {
	pc = 0x82E70864; continue 'dispatch;
	}
	// 82E70808: 2F0B0007  cmpwi cr6, r11, 7
	ctx.cr[6].compare_i32(ctx.r[11].s32, 7, &mut ctx.xer);
	// 82E7080C: 419A004C  beq cr6, 0x82e70858
	if ctx.cr[6].eq {
	pc = 0x82E70858; continue 'dispatch;
	}
	// 82E70810: 2F0B0008  cmpwi cr6, r11, 8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 8, &mut ctx.xer);
	// 82E70814: 419A0034  beq cr6, 0x82e70848
	if ctx.cr[6].eq {
	pc = 0x82E70848; continue 'dispatch;
	}
	// 82E70818: 2F0B0009  cmpwi cr6, r11, 9
	ctx.cr[6].compare_i32(ctx.r[11].s32, 9, &mut ctx.xer);
	// 82E7081C: 419A001C  beq cr6, 0x82e70838
	if ctx.cr[6].eq {
	pc = 0x82E70838; continue 'dispatch;
	}
	// 82E70820: 2F0B000A  cmpwi cr6, r11, 0xa
	ctx.cr[6].compare_i32(ctx.r[11].s32, 10, &mut ctx.xer);
	// 82E70824: 409A0088  bne cr6, 0x82e708ac
	if !ctx.cr[6].eq {
	pc = 0x82E708AC; continue 'dispatch;
	}
	// 82E70828: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E7082C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E70830: 48004301  bl 0x82e74b30
	ctx.lr = 0x82E70834;
	sub_82E74B30(ctx, base);
	// 82E70834: 48000078  b 0x82e708ac
	pc = 0x82E708AC; continue 'dispatch;
	// 82E70838: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E7083C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E70840: 480044A1  bl 0x82e74ce0
	ctx.lr = 0x82E70844;
	sub_82E74CE0(ctx, base);
	// 82E70844: 48000068  b 0x82e708ac
	pc = 0x82E708AC; continue 'dispatch;
	// 82E70848: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E7084C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E70850: 4BFFEA51  bl 0x82e6f2a0
	ctx.lr = 0x82E70854;
	sub_82E6F2A0(ctx, base);
	// 82E70854: 48000058  b 0x82e708ac
	pc = 0x82E708AC; continue 'dispatch;
	// 82E70858: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E7085C: 38AB0018  addi r5, r11, 0x18
	ctx.r[5].s64 = ctx.r[11].s64 + 24;
	// 82E70860: 4800003C  b 0x82e7089c
	pc = 0x82E7089C; continue 'dispatch;
	// 82E70864: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E70868: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E7086C: 4800454D  bl 0x82e74db8
	ctx.lr = 0x82E70870;
	sub_82E74DB8(ctx, base);
	// 82E70870: 4800003C  b 0x82e708ac
	pc = 0x82E708AC; continue 'dispatch;
	// 82E70874: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E70878: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E7087C: 480033E5  bl 0x82e73c60
	ctx.lr = 0x82E70880;
	sub_82E73C60(ctx, base);
	// 82E70880: 4800002C  b 0x82e708ac
	pc = 0x82E708AC; continue 'dispatch;
	// 82E70884: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E70888: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E7088C: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82E70890: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E70894: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E70898: 38AB0011  addi r5, r11, 0x11
	ctx.r[5].s64 = ctx.r[11].s64 + 17;
	// 82E7089C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E708A0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E708A4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E708A8: 480047D1  bl 0x82e75078
	ctx.lr = 0x82E708AC;
	sub_82E75078(ctx, base);
	// 82E708AC: 83FD0000  lwz r31, 0(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E708B0: 281F0000  cmplwi r31, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E708B4: 4082FECC  bne 0x82e70780
	if !ctx.cr[0].eq {
	pc = 0x82E70780; continue 'dispatch;
	}
	// 82E708B8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E708BC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E708C0: 483378F0  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E708C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E708C8 size=248
    let mut pc: u32 = 0x82E708C8;
    'dispatch: loop {
        match pc {
            0x82E708C8 => {
    //   block [0x82E708C8..0x82E709C0)
	// 82E708C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E708CC: 48337895  bl 0x831a8160
	ctx.lr = 0x82E708D0;
	sub_831A8130(ctx, base);
	// 82E708D0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E708D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E708D8: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82E708DC: 83DF0010  lwz r30, 0x10(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E708E0: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E708E4: 83AB0000  lwz r29, 0(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E708E8: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E708EC: 409A000C  bne cr6, 0x82e708f8
	if !ctx.cr[6].eq {
	pc = 0x82E708F8; continue 'dispatch;
	}
	// 82E708F0: 935E0030  stw r26, 0x30(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(48 as u32), ctx.r[26].u32 ) };
	// 82E708F4: 4800000C  b 0x82e70900
	pc = 0x82E70900; continue 'dispatch;
	// 82E708F8: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E708FC: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E70900: 817E0070  lwz r11, 0x70(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(112 as u32) ) } as u64;
	// 82E70904: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E70908: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E7090C: 817E0070  lwz r11, 0x70(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(112 as u32) ) } as u64;
	// 82E70910: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82E70914: 897E0014  lbz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E70918: 895D0005  lbz r10, 5(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(5 as u32) ) } as u64;
	// 82E7091C: 807D0008  lwz r3, 8(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E70920: 514B0638  rlwimi r11, r10, 0, 0x18, 0x1c
	ctx.r[11].u64 = (((ctx.r[10].u32).rotate_left(0) as u64) & 0x00000000000000F8) | (ctx.r[11].u64 & 0xFFFFFFFFFFFFFF07);
	// 82E70924: 716B00FB  andi. r11, r11, 0xfb
	ctx.r[11].u64 = ctx.r[11].u64 & 251;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E70928: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E7092C: 997D0005  stb r11, 5(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(5 as u32), ctx.r[11].u8 ) };
	// 82E70930: 41820088  beq 0x82e709b8
	if ctx.cr[0].eq {
	pc = 0x82E709B8; continue 'dispatch;
	}
	// 82E70934: 89630006  lbz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82E70938: 556B077B  rlwinm. r11, r11, 0, 0x1d, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E7093C: 4082007C  bne 0x82e709b8
	if !ctx.cr[0].eq {
	pc = 0x82E709B8; continue 'dispatch;
	}
	// 82E70940: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E70944: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 82E70948: 80AB00C4  lwz r5, 0xc4(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(196 as u32) ) } as u64;
	// 82E7094C: 4800126D  bl 0x82e71bb8
	ctx.lr = 0x82E70950;
	sub_82E71BB8(ctx, base);
	// 82E70950: 7C6A1B79  or. r10, r3, r3
	ctx.r[10].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E70954: 41820064  beq 0x82e709b8
	if ctx.cr[0].eq {
	pc = 0x82E709B8; continue 'dispatch;
	}
	// 82E70958: 839E0040  lwz r28, 0x40(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(64 as u32) ) } as u64;
	// 82E7095C: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82E70960: 8B7F0037  lbz r27, 0x37(r31)
	ctx.r[27].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(55 as u32) ) } as u64;
	// 82E70964: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82E70968: 9B5F0037  stb r26, 0x37(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(55 as u32), ctx.r[26].u8 ) };
	// 82E7096C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E70970: 817E0044  lwz r11, 0x44(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(68 as u32) ) } as u64;
	// 82E70974: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E70978: 917E0040  stw r11, 0x40(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 82E7097C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E70980: E90A0000  ld r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	// 82E70984: F90B0000  std r8, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u64 ) };
	// 82E70988: 814A0008  lwz r10, 8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E7098C: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E70990: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E70994: 93AB0010  stw r29, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[29].u32 ) };
	// 82E70998: 912B0018  stw r9, 0x18(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[9].u32 ) };
	// 82E7099C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E709A0: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 82E709A4: 388BFFE0  addi r4, r11, -0x20
	ctx.r[4].s64 = ctx.r[11].s64 + -32;
	// 82E709A8: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E709AC: 4BFF9F2D  bl 0x82e6a8d8
	ctx.lr = 0x82E709B0;
	sub_82E6A8D8(ctx, base);
	// 82E709B0: 9B7F0037  stb r27, 0x37(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(55 as u32), ctx.r[27].u8 ) };
	// 82E709B4: 939E0040  stw r28, 0x40(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(64 as u32), ctx.r[28].u32 ) };
	// 82E709B8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E709BC: 483377F4  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E709C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E709C0 size=68
    let mut pc: u32 = 0x82E709C0;
    'dispatch: loop {
        match pc {
            0x82E709C0 => {
    //   block [0x82E709C0..0x82E70A04)
	// 82E709C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E709C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E709C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E709CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E709D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E709D4: 4800000C  b 0x82e709e0
	pc = 0x82E709E0; continue 'dispatch;
	// 82E709D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E709DC: 4BFFFEED  bl 0x82e708c8
	ctx.lr = 0x82E709E0;
	sub_82E708C8(ctx, base);
	// 82E709E0: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E709E4: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E709E8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E709EC: 409AFFEC  bne cr6, 0x82e709d8
	if !ctx.cr[6].eq {
	pc = 0x82E709D8; continue 'dispatch;
	}
	// 82E709F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E709F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E709F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E709FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E70A00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E70A08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E70A08 size=112
    let mut pc: u32 = 0x82E70A08;
    'dispatch: loop {
        match pc {
            0x82E70A08 => {
    //   block [0x82E70A08..0x82E70A78)
	// 82E70A08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E70A0C: 48337759  bl 0x831a8164
	ctx.lr = 0x82E70A10;
	sub_831A8130(ctx, base);
	// 82E70A10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E70A14: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82E70A18: 39600043  li r11, 0x43
	ctx.r[11].s64 = 67;
	// 82E70A1C: 3B60FFFD  li r27, -3
	ctx.r[27].s64 = -3;
	// 82E70A20: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82E70A24: 83FC0010  lwz r31, 0x10(r28)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E70A28: 389F001C  addi r4, r31, 0x1c
	ctx.r[4].s64 = ctx.r[31].s64 + 28;
	// 82E70A2C: 997F0014  stb r11, 0x14(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u8 ) };
	// 82E70A30: 4BFFFD29  bl 0x82e70758
	ctx.lr = 0x82E70A34;
	sub_82E70758(ctx, base);
	// 82E70A34: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E70A38: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82E70A3C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E70A40: 40990030  ble cr6, 0x82e70a70
	if !ctx.cr[6].gt {
	pc = 0x82E70A70; continue 'dispatch;
	}
	// 82E70A44: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82E70A48: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E70A4C: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82E70A50: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82E70A54: 7C8BEA14  add r4, r11, r29
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82E70A58: 4BFFFD01  bl 0x82e70758
	ctx.lr = 0x82E70A5C;
	sub_82E70758(ctx, base);
	// 82E70A5C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E70A60: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82E70A64: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82E70A68: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E70A6C: 4198FFDC  blt cr6, 0x82e70a48
	if ctx.cr[6].lt {
	pc = 0x82E70A48; continue 'dispatch;
	}
	// 82E70A70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E70A74: 48337740  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E70A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E70A78 size=216
    let mut pc: u32 = 0x82E70A78;
    'dispatch: loop {
        match pc {
            0x82E70A78 => {
    //   block [0x82E70A78..0x82E70B50)
	// 82E70A78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E70A7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E70A80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E70A84: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 82E70A88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E70A8C: 81280010  lwz r9, 0x10(r8)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E70A90: 80890070  lwz r4, 0x70(r9)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(112 as u32) ) } as u64;
	// 82E70A94: 91690024  stw r11, 0x24(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82E70A98: 91690028  stw r11, 0x28(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82E70A9C: 9169002C  stw r11, 0x2c(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82E70AA0: 89640005  lbz r11, 5(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(5 as u32) ) } as u64;
	// 82E70AA4: 556B07BF  clrlwi. r11, r11, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E70AA8: 4182000C  beq 0x82e70ab4
	if ctx.cr[0].eq {
	pc = 0x82E70AB4; continue 'dispatch;
	}
	// 82E70AAC: 7D234B78  mr r3, r9
	ctx.r[3].u64 = ctx.r[9].u64;
	// 82E70AB0: 4BFFF1C9  bl 0x82e6fc78
	ctx.lr = 0x82E70AB4;
	sub_82E6FC78(ctx, base);
	// 82E70AB4: 81690070  lwz r11, 0x70(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(112 as u32) ) } as u64;
	// 82E70AB8: 814B0050  lwz r10, 0x50(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E70ABC: 2F0A0004  cmpwi cr6, r10, 4
	ctx.cr[6].compare_i32(ctx.r[10].s32, 4, &mut ctx.xer);
	// 82E70AC0: 4198001C  blt cr6, 0x82e70adc
	if ctx.cr[6].lt {
	pc = 0x82E70ADC; continue 'dispatch;
	}
	// 82E70AC4: 808B0048  lwz r4, 0x48(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 82E70AC8: 89640005  lbz r11, 5(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(5 as u32) ) } as u64;
	// 82E70ACC: 556B07BF  clrlwi. r11, r11, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E70AD0: 4182000C  beq 0x82e70adc
	if ctx.cr[0].eq {
	pc = 0x82E70ADC; continue 'dispatch;
	}
	// 82E70AD4: 7D234B78  mr r3, r9
	ctx.r[3].u64 = ctx.r[9].u64;
	// 82E70AD8: 4BFFF1A1  bl 0x82e6fc78
	ctx.lr = 0x82E70ADC;
	sub_82E6FC78(ctx, base);
	// 82E70ADC: 81680010  lwz r11, 0x10(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E70AE0: 814B0068  lwz r10, 0x68(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E70AE4: 2F0A0004  cmpwi cr6, r10, 4
	ctx.cr[6].compare_i32(ctx.r[10].s32, 4, &mut ctx.xer);
	// 82E70AE8: 4198001C  blt cr6, 0x82e70b04
	if ctx.cr[6].lt {
	pc = 0x82E70B04; continue 'dispatch;
	}
	// 82E70AEC: 808B0060  lwz r4, 0x60(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E70AF0: 89640005  lbz r11, 5(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(5 as u32) ) } as u64;
	// 82E70AF4: 556B07BF  clrlwi. r11, r11, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E70AF8: 4182000C  beq 0x82e70b04
	if ctx.cr[0].eq {
	pc = 0x82E70B04; continue 'dispatch;
	}
	// 82E70AFC: 7D234B78  mr r3, r9
	ctx.r[3].u64 = ctx.r[9].u64;
	// 82E70B00: 4BFFF179  bl 0x82e6fc78
	ctx.lr = 0x82E70B04;
	sub_82E6FC78(ctx, base);
	// 82E70B04: 39090098  addi r8, r9, 0x98
	ctx.r[8].s64 = ctx.r[9].s64 + 152;
	// 82E70B08: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 82E70B0C: 80880000  lwz r4, 0(r8)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E70B10: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E70B14: 41820018  beq 0x82e70b2c
	if ctx.cr[0].eq {
	pc = 0x82E70B2C; continue 'dispatch;
	}
	// 82E70B18: 89640005  lbz r11, 5(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(5 as u32) ) } as u64;
	// 82E70B1C: 556B07BF  clrlwi. r11, r11, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E70B20: 4182000C  beq 0x82e70b2c
	if ctx.cr[0].eq {
	pc = 0x82E70B2C; continue 'dispatch;
	}
	// 82E70B24: 7D234B78  mr r3, r9
	ctx.r[3].u64 = ctx.r[9].u64;
	// 82E70B28: 4BFFF151  bl 0x82e6fc78
	ctx.lr = 0x82E70B2C;
	sub_82E6FC78(ctx, base);
	// 82E70B2C: 34E7FFFF  addic. r7, r7, -1
	ctx.xer.ca = (ctx.r[7].u32 > (!(-1 as u32)));
	ctx.r[7].s64 = ctx.r[7].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82E70B30: 39080004  addi r8, r8, 4
	ctx.r[8].s64 = ctx.r[8].s64 + 4;
	// 82E70B34: 4082FFD8  bne 0x82e70b0c
	if !ctx.cr[0].eq {
	pc = 0x82E70B0C; continue 'dispatch;
	}
	// 82E70B38: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82E70B3C: 99690015  stb r11, 0x15(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(21 as u32), ctx.r[11].u8 ) };
	// 82E70B40: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E70B44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E70B48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E70B4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E70B50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E70B50 size=452
    let mut pc: u32 = 0x82E70B50;
    'dispatch: loop {
        match pc {
            0x82E70B50 => {
    //   block [0x82E70B50..0x82E70D14)
	// 82E70B50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E70B54: 48337615  bl 0x831a8168
	ctx.lr = 0x82E70B58;
	sub_831A8130(ctx, base);
	// 82E70B58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E70B5C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E70B60: 83FE0010  lwz r31, 0x10(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E70B64: 391F0078  addi r8, r31, 0x78
	ctx.r[8].s64 = ctx.r[31].s64 + 120;
	// 82E70B68: 813F008C  lwz r9, 0x8c(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) } as u64;
	// 82E70B6C: 48000044  b 0x82e70bb0
	pc = 0x82E70BB0; continue 'dispatch;
	// 82E70B70: 89690005  lbz r11, 5(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(5 as u32) ) } as u64;
	// 82E70B74: 556A077B  rlwinm. r10, r11, 0, 0x1d, 0x1d
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E70B78: 40820034  bne 0x82e70bac
	if !ctx.cr[0].eq {
	pc = 0x82E70BAC; continue 'dispatch;
	}
	// 82E70B7C: 556B07BF  clrlwi. r11, r11, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E70B80: 4082002C  bne 0x82e70bac
	if !ctx.cr[0].eq {
	pc = 0x82E70BAC; continue 'dispatch;
	}
	// 82E70B84: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E70B88: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E70B8C: 2F0A0004  cmpwi cr6, r10, 4
	ctx.cr[6].compare_i32(ctx.r[10].s32, 4, &mut ctx.xer);
	// 82E70B90: 4198001C  blt cr6, 0x82e70bac
	if ctx.cr[6].lt {
	pc = 0x82E70BAC; continue 'dispatch;
	}
	// 82E70B94: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E70B98: 89640005  lbz r11, 5(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(5 as u32) ) } as u64;
	// 82E70B9C: 556B07BF  clrlwi. r11, r11, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E70BA0: 4182000C  beq 0x82e70bac
	if ctx.cr[0].eq {
	pc = 0x82E70BAC; continue 'dispatch;
	}
	// 82E70BA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E70BA8: 4BFFF0D1  bl 0x82e6fc78
	ctx.lr = 0x82E70BAC;
	sub_82E6FC78(ctx, base);
	// 82E70BAC: 81290014  lwz r9, 0x14(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E70BB0: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82E70BB4: 409AFFBC  bne cr6, 0x82e70b70
	if !ctx.cr[6].eq {
	pc = 0x82E70B70; continue 'dispatch;
	}
	// 82E70BB8: 4800000C  b 0x82e70bc4
	pc = 0x82E70BC4; continue 'dispatch;
	// 82E70BBC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E70BC0: 4BFFF8C9  bl 0x82e70488
	ctx.lr = 0x82E70BC4;
	sub_82E70488(ctx, base);
	// 82E70BC4: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82E70BC8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E70BCC: 409AFFF0  bne cr6, 0x82e70bbc
	if !ctx.cr[6].eq {
	pc = 0x82E70BBC; continue 'dispatch;
	}
	// 82E70BD0: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82E70BD4: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82E70BD8: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82E70BDC: 939F002C  stw r28, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[28].u32 ) };
	// 82E70BE0: 897E0005  lbz r11, 5(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(5 as u32) ) } as u64;
	// 82E70BE4: 556B07BF  clrlwi. r11, r11, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E70BE8: 41820010  beq 0x82e70bf8
	if ctx.cr[0].eq {
	pc = 0x82E70BF8; continue 'dispatch;
	}
	// 82E70BEC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E70BF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E70BF4: 4BFFF085  bl 0x82e6fc78
	ctx.lr = 0x82E70BF8;
	sub_82E6FC78(ctx, base);
	// 82E70BF8: 393F0098  addi r9, r31, 0x98
	ctx.r[9].s64 = ctx.r[31].s64 + 152;
	// 82E70BFC: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 82E70C00: 80890000  lwz r4, 0(r9)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E70C04: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E70C08: 41820018  beq 0x82e70c20
	if ctx.cr[0].eq {
	pc = 0x82E70C20; continue 'dispatch;
	}
	// 82E70C0C: 89640005  lbz r11, 5(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(5 as u32) ) } as u64;
	// 82E70C10: 556B07BF  clrlwi. r11, r11, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E70C14: 4182000C  beq 0x82e70c20
	if ctx.cr[0].eq {
	pc = 0x82E70C20; continue 'dispatch;
	}
	// 82E70C18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E70C1C: 4BFFF05D  bl 0x82e6fc78
	ctx.lr = 0x82E70C20;
	sub_82E6FC78(ctx, base);
	// 82E70C20: 3508FFFF  addic. r8, r8, -1
	ctx.xer.ca = (ctx.r[8].u32 > (!(-1 as u32)));
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82E70C24: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 82E70C28: 4082FFD8  bne 0x82e70c00
	if !ctx.cr[0].eq {
	pc = 0x82E70C00; continue 'dispatch;
	}
	// 82E70C2C: 4800000C  b 0x82e70c38
	pc = 0x82E70C38; continue 'dispatch;
	// 82E70C30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E70C34: 4BFFF855  bl 0x82e70488
	ctx.lr = 0x82E70C38;
	sub_82E70488(ctx, base);
	// 82E70C38: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82E70C3C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E70C40: 409AFFF0  bne cr6, 0x82e70c30
	if !ctx.cr[6].eq {
	pc = 0x82E70C30; continue 'dispatch;
	}
	// 82E70C44: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82E70C48: 939F0028  stw r28, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[28].u32 ) };
	// 82E70C4C: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82E70C50: 4800000C  b 0x82e70c5c
	pc = 0x82E70C5C; continue 'dispatch;
	// 82E70C54: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E70C58: 4BFFF831  bl 0x82e70488
	ctx.lr = 0x82E70C5C;
	sub_82E70488(ctx, base);
	// 82E70C5C: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82E70C60: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E70C64: 409AFFF0  bne cr6, 0x82e70c54
	if !ctx.cr[6].eq {
	pc = 0x82E70C54; continue 'dispatch;
	}
	// 82E70C68: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E70C6C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E70C70: 4BFFF151  bl 0x82e6fdc0
	ctx.lr = 0x82E70C74;
	sub_82E6FDC0(ctx, base);
	// 82E70C74: 813F0030  lwz r9, 0x30(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E70C78: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E70C7C: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E70C80: 41820034  beq 0x82e70cb4
	if ctx.cr[0].eq {
	pc = 0x82E70CB4; continue 'dispatch;
	}
	// 82E70C84: 81290000  lwz r9, 0(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E70C88: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E70C8C: 897F0014  lbz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E70C90: 7D244B78  mr r4, r9
	ctx.r[4].u64 = ctx.r[9].u64;
	// 82E70C94: 89490005  lbz r10, 5(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(5 as u32) ) } as u64;
	// 82E70C98: 514B0638  rlwimi r11, r10, 0, 0x18, 0x1c
	ctx.r[11].u64 = (((ctx.r[10].u32).rotate_left(0) as u64) & 0x00000000000000F8) | (ctx.r[11].u64 & 0xFFFFFFFFFFFFFF07);
	// 82E70C9C: 716B00FB  andi. r11, r11, 0xfb
	ctx.r[11].u64 = ctx.r[11].u64 & 251;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E70CA0: 99690005  stb r11, 5(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(5 as u32), ctx.r[11].u8 ) };
	// 82E70CA4: 4BFFEFD5  bl 0x82e6fc78
	ctx.lr = 0x82E70CA8;
	sub_82E6FC78(ctx, base);
	// 82E70CA8: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E70CAC: 7F095840  cmplw cr6, r9, r11
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E70CB0: 409AFFD4  bne cr6, 0x82e70c84
	if !ctx.cr[6].eq {
	pc = 0x82E70C84; continue 'dispatch;
	}
	// 82E70CB4: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 82E70CB8: 48000010  b 0x82e70cc8
	pc = 0x82E70CC8; continue 'dispatch;
	// 82E70CBC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E70CC0: 4BFFF7C9  bl 0x82e70488
	ctx.lr = 0x82E70CC4;
	sub_82E70488(ctx, base);
	// 82E70CC4: 7FC3F214  add r30, r3, r30
	ctx.r[30].u64 = ctx.r[3].u64 + ctx.r[30].u64;
	// 82E70CC8: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82E70CCC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E70CD0: 409AFFEC  bne cr6, 0x82e70cbc
	if !ctx.cr[6].eq {
	pc = 0x82E70CBC; continue 'dispatch;
	}
	// 82E70CD4: 807F002C  lwz r3, 0x2c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82E70CD8: 4BFFF921  bl 0x82e705f8
	ctx.lr = 0x82E70CDC;
	sub_82E705F8(ctx, base);
	// 82E70CDC: 815F0044  lwz r10, 0x44(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 82E70CE0: 393F001C  addi r9, r31, 0x1c
	ctx.r[9].s64 = ctx.r[31].s64 + 28;
	// 82E70CE4: 897F0014  lbz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E70CE8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82E70CEC: 7D5E5050  subf r10, r30, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[30].s64;
	// 82E70CF0: 939F0018  stw r28, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[28].u32 ) };
	// 82E70CF4: 696B0003  xori r11, r11, 3
	ctx.r[11].u64 = ctx.r[11].u64 ^ 3;
	// 82E70CF8: 7D5D5050  subf r10, r29, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[29].s64;
	// 82E70CFC: 913F0020  stw r9, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[9].u32 ) };
	// 82E70D00: 991F0015  stb r8, 0x15(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(21 as u32), ctx.r[8].u8 ) };
	// 82E70D04: 997F0014  stb r11, 0x14(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u8 ) };
	// 82E70D08: 915F0048  stw r10, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[10].u32 ) };
	// 82E70D0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E70D10: 483374A8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E70D18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E70D18 size=440
    let mut pc: u32 = 0x82E70D18;
    'dispatch: loop {
        match pc {
            0x82E70D18 => {
    //   block [0x82E70D18..0x82E70ED0)
	// 82E70D18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E70D1C: 48337449  bl 0x831a8164
	ctx.lr = 0x82E70D20;
	sub_831A8130(ctx, base);
	// 82E70D20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E70D24: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E70D28: 83FE0010  lwz r31, 0x10(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E70D2C: 897F0015  lbz r11, 0x15(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(21 as u32) ) } as u64;
	// 82E70D30: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82E70D34: 41980188  blt cr6, 0x82e70ebc
	if ctx.cr[6].lt {
	pc = 0x82E70EBC; continue 'dispatch;
	}
	// 82E70D38: 419A0160  beq cr6, 0x82e70e98
	if ctx.cr[6].eq {
	pc = 0x82E70E98; continue 'dispatch;
	}
	// 82E70D3C: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 82E70D40: 419800FC  blt cr6, 0x82e70e3c
	if ctx.cr[6].lt {
	pc = 0x82E70E3C; continue 'dispatch;
	}
	// 82E70D44: 419A0048  beq cr6, 0x82e70d8c
	if ctx.cr[6].eq {
	pc = 0x82E70D8C; continue 'dispatch;
	}
	// 82E70D48: 2B0B0005  cmplwi cr6, r11, 5
	ctx.cr[6].compare_u32(ctx.r[11].u32, 5 as u32, &mut ctx.xer);
	// 82E70D4C: 40980178  bge cr6, 0x82e70ec4
	if !ctx.cr[6].lt {
	pc = 0x82E70EC4; continue 'dispatch;
	}
	// 82E70D50: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E70D54: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E70D58: 419A0024  beq cr6, 0x82e70d7c
	if ctx.cr[6].eq {
	pc = 0x82E70D7C; continue 'dispatch;
	}
	// 82E70D5C: 4BFFFB6D  bl 0x82e708c8
	ctx.lr = 0x82E70D60;
	sub_82E708C8(ctx, base);
	// 82E70D60: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82E70D64: 2B0B0064  cmplwi cr6, r11, 0x64
	ctx.cr[6].compare_u32(ctx.r[11].u32, 100 as u32, &mut ctx.xer);
	// 82E70D68: 4099000C  ble cr6, 0x82e70d74
	if !ctx.cr[6].gt {
	pc = 0x82E70D74; continue 'dispatch;
	}
	// 82E70D6C: 396BFF9C  addi r11, r11, -0x64
	ctx.r[11].s64 = ctx.r[11].s64 + -100;
	// 82E70D70: 917F0048  stw r11, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 82E70D74: 38600064  li r3, 0x64
	ctx.r[3].s64 = 100;
	// 82E70D78: 48000150  b 0x82e70ec8
	pc = 0x82E70EC8; continue 'dispatch;
	// 82E70D7C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E70D80: 997F0015  stb r11, 0x15(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(21 as u32), ctx.r[11].u8 ) };
	// 82E70D84: 917F004C  stw r11, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[11].u32 ) };
	// 82E70D88: 4800013C  b 0x82e70ec4
	pc = 0x82E70EC4; continue 'dispatch;
	// 82E70D8C: 38A00028  li r5, 0x28
	ctx.r[5].s64 = 40;
	// 82E70D90: 837F0044  lwz r27, 0x44(r31)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 82E70D94: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E70D98: 809F0020  lwz r4, 0x20(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E70D9C: 4BFFF9BD  bl 0x82e70758
	ctx.lr = 0x82E70DA0;
	sub_82E70758(ctx, base);
	// 82E70DA0: 907F0020  stw r3, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[3].u32 ) };
	// 82E70DA4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E70DA8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E70DAC: 409A0080  bne cr6, 0x82e70e2c
	if !ctx.cr[6].eq {
	pc = 0x82E70E2C; continue 'dispatch;
	}
	// 82E70DB0: 83BE0010  lwz r29, 0x10(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E70DB4: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E70DB8: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E70DBC: 7D691670  srawi r9, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 82E70DC0: 7D290194  addze r9, r9
	tmp.s64 = ctx.r[9].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[9].u32);
	ctx.r[9].s64 = tmp.s64;
	// 82E70DC4: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82E70DC8: 4098001C  bge cr6, 0x82e70de4
	if !ctx.cr[6].lt {
	pc = 0x82E70DE4; continue 'dispatch;
	}
	// 82E70DCC: 2F0B0040  cmpwi cr6, r11, 0x40
	ctx.cr[6].compare_i32(ctx.r[11].s32, 64, &mut ctx.xer);
	// 82E70DD0: 40990014  ble cr6, 0x82e70de4
	if !ctx.cr[6].gt {
	pc = 0x82E70DE4; continue 'dispatch;
	}
	// 82E70DD4: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82E70DD8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E70DDC: 7C8B0194  addze r4, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[4].s64 = tmp.s64;
	// 82E70DE0: 48003869  bl 0x82e74648
	ctx.lr = 0x82E70DE4;
	sub_82E74648(ctx, base);
	// 82E70DE4: 80BD003C  lwz r5, 0x3c(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(60 as u32) ) } as u64;
	// 82E70DE8: 2B050040  cmplwi cr6, r5, 0x40
	ctx.cr[6].compare_u32(ctx.r[5].u32, 64 as u32, &mut ctx.xer);
	// 82E70DEC: 40990038  ble cr6, 0x82e70e24
	if !ctx.cr[6].gt {
	pc = 0x82E70E24; continue 'dispatch;
	}
	// 82E70DF0: 54BCF87E  srwi r28, r5, 1
	ctx.r[28].u32 = ctx.r[5].u32.wrapping_shr(1);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 82E70DF4: 3960FFFD  li r11, -3
	ctx.r[11].s64 = -3;
	// 82E70DF8: 395C0001  addi r10, r28, 1
	ctx.r[10].s64 = ctx.r[28].s64 + 1;
	// 82E70DFC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E70E00: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E70E04: 41990014  bgt cr6, 0x82e70e18
	if ctx.cr[6].gt {
	pc = 0x82E70E18; continue 'dispatch;
	}
	// 82E70E08: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82E70E0C: 809D0034  lwz r4, 0x34(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(52 as u32) ) } as u64;
	// 82E70E10: 48004269  bl 0x82e75078
	ctx.lr = 0x82E70E14;
	sub_82E75078(ctx, base);
	// 82E70E14: 48000008  b 0x82e70e1c
	pc = 0x82E70E1C; continue 'dispatch;
	// 82E70E18: 48004231  bl 0x82e75048
	ctx.lr = 0x82E70E1C;
	sub_82E75048(ctx, base);
	// 82E70E1C: 907D0034  stw r3, 0x34(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(52 as u32), ctx.r[3].u32 ) };
	// 82E70E20: 939D003C  stw r28, 0x3c(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(60 as u32), ctx.r[28].u32 ) };
	// 82E70E24: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 82E70E28: 997F0015  stb r11, 0x15(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(21 as u32), ctx.r[11].u8 ) };
	// 82E70E2C: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82E70E30: 38600190  li r3, 0x190
	ctx.r[3].s64 = 400;
	// 82E70E34: 7D7B5850  subf r11, r27, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[27].s64;
	// 82E70E38: 48000050  b 0x82e70e88
	pc = 0x82E70E88; continue 'dispatch;
	// 82E70E3C: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E70E40: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E70E44: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E70E48: 38A0FFFD  li r5, -3
	ctx.r[5].s64 = -3;
	// 82E70E4C: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E70E50: 83DF0044  lwz r30, 0x44(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 82E70E54: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82E70E58: 7C8A4A14  add r4, r10, r9
	ctx.r[4].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82E70E5C: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82E70E60: 4BFFF8F9  bl 0x82e70758
	ctx.lr = 0x82E70E64;
	sub_82E70758(ctx, base);
	// 82E70E64: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E70E68: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E70E6C: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82E70E70: 4198000C  blt cr6, 0x82e70e7c
	if ctx.cr[6].lt {
	pc = 0x82E70E7C; continue 'dispatch;
	}
	// 82E70E74: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 82E70E78: 997F0015  stb r11, 0x15(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(21 as u32), ctx.r[11].u8 ) };
	// 82E70E7C: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82E70E80: 3860000A  li r3, 0xa
	ctx.r[3].s64 = 10;
	// 82E70E84: 7D7E5850  subf r11, r30, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	// 82E70E88: 815F0044  lwz r10, 0x44(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 82E70E8C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82E70E90: 917F0048  stw r11, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 82E70E94: 48000034  b 0x82e70ec8
	pc = 0x82E70EC8; continue 'dispatch;
	// 82E70E98: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82E70E9C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E70EA0: 419A0010  beq cr6, 0x82e70eb0
	if ctx.cr[6].eq {
	pc = 0x82E70EB0; continue 'dispatch;
	}
	// 82E70EA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E70EA8: 4BFFF5E1  bl 0x82e70488
	ctx.lr = 0x82E70EAC;
	sub_82E70488(ctx, base);
	// 82E70EAC: 4800001C  b 0x82e70ec8
	pc = 0x82E70EC8; continue 'dispatch;
	// 82E70EB0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E70EB4: 4BFFFC9D  bl 0x82e70b50
	ctx.lr = 0x82E70EB8;
	sub_82E70B50(ctx, base);
	// 82E70EB8: 4800000C  b 0x82e70ec4
	pc = 0x82E70EC4; continue 'dispatch;
	// 82E70EBC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E70EC0: 4BFFFBB9  bl 0x82e70a78
	ctx.lr = 0x82E70EC4;
	sub_82E70A78(ctx, base);
	// 82E70EC4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E70EC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E70ECC: 483372E8  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E70ED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E70ED0 size=188
    let mut pc: u32 = 0x82E70ED0;
    'dispatch: loop {
        match pc {
            0x82E70ED0 => {
    //   block [0x82E70ED0..0x82E70F8C)
	// 82E70ED0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E70ED4: 48337299  bl 0x831a816c
	ctx.lr = 0x82E70ED8;
	sub_831A8130(ctx, base);
	// 82E70ED8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E70EDC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E70EE0: 83FD0010  lwz r31, 0x10(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E70EE4: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E70EE8: 1FCB000A  mulli r30, r11, 0xa
	ctx.r[30].s64 = ctx.r[11].s64 * 10;
	// 82E70EEC: 2C1E0000  cmpwi r30, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82E70EF0: 4082000C  bne 0x82e70efc
	if !ctx.cr[0].eq {
	pc = 0x82E70EFC; continue 'dispatch;
	}
	// 82E70EF4: 3FC07FFF  lis r30, 0x7fff
	ctx.r[30].s64 = 2147418112;
	// 82E70EF8: 63DEFFFE  ori r30, r30, 0xfffe
	ctx.r[30].u64 = ctx.r[30].u64 | 65534;
	// 82E70EFC: 817F0044  lwz r11, 0x44(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 82E70F00: 813F0040  lwz r9, 0x40(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82E70F04: 815F004C  lwz r10, 0x4c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 82E70F08: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 82E70F0C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82E70F10: 917F004C  stw r11, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[11].u32 ) };
	// 82E70F14: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E70F18: 4BFFFE01  bl 0x82e70d18
	ctx.lr = 0x82E70F1C;
	sub_82E70D18(ctx, base);
	// 82E70F1C: 897F0015  lbz r11, 0x15(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(21 as u32) ) } as u64;
	// 82E70F20: 7FC3F050  subf r30, r3, r30
	ctx.r[30].s64 = ctx.r[30].s64 - ctx.r[3].s64;
	// 82E70F24: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E70F28: 4182000C  beq 0x82e70f34
	if ctx.cr[0].eq {
	pc = 0x82E70F34; continue 'dispatch;
	}
	// 82E70F2C: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82E70F30: 4199FFE4  bgt cr6, 0x82e70f14
	if ctx.cr[6].gt {
	pc = 0x82E70F14; continue 'dispatch;
	}
	// 82E70F34: 897F0015  lbz r11, 0x15(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(21 as u32) ) } as u64;
	// 82E70F38: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E70F3C: 41820030  beq 0x82e70f6c
	if ctx.cr[0].eq {
	pc = 0x82E70F6C; continue 'dispatch;
	}
	// 82E70F40: 817F004C  lwz r11, 0x4c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 82E70F44: 2B0B0400  cmplwi cr6, r11, 0x400
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1024 as u32, &mut ctx.xer);
	// 82E70F48: 40980010  bge cr6, 0x82e70f58
	if !ctx.cr[6].lt {
	pc = 0x82E70F58; continue 'dispatch;
	}
	// 82E70F4C: 817F0044  lwz r11, 0x44(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 82E70F50: 396B0400  addi r11, r11, 0x400
	ctx.r[11].s64 = ctx.r[11].s64 + 1024;
	// 82E70F54: 4800002C  b 0x82e70f80
	pc = 0x82E70F80; continue 'dispatch;
	// 82E70F58: 815F0044  lwz r10, 0x44(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 82E70F5C: 396BFC00  addi r11, r11, -0x400
	ctx.r[11].s64 = ctx.r[11].s64 + -1024;
	// 82E70F60: 917F004C  stw r11, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[11].u32 ) };
	// 82E70F64: 915F0040  stw r10, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[10].u32 ) };
	// 82E70F68: 4800001C  b 0x82e70f84
	pc = 0x82E70F84; continue 'dispatch;
	// 82E70F6C: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82E70F70: 39400064  li r10, 0x64
	ctx.r[10].s64 = 100;
	// 82E70F74: 813F0050  lwz r9, 0x50(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E70F78: 7D6B5396  divwu r11, r11, r10
	ctx.r[11].u32 = ctx.r[11].u32 / ctx.r[10].u32;
	// 82E70F7C: 7D6B49D6  mullw r11, r11, r9
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[9].s32 as i64);
	// 82E70F80: 917F0040  stw r11, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 82E70F84: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E70F88: 48337234  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E70F90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E70F90 size=28
    let mut pc: u32 = 0x82E70F90;
    'dispatch: loop {
        match pc {
            0x82E70F90 => {
    //   block [0x82E70F90..0x82E70FAC)
	// 82E70F90: 80630010  lwz r3, 0x10(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E70F94: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82E70F98: 89430015  lbz r10, 0x15(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(21 as u32) ) } as u64;
	// 82E70F9C: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82E70FA0: 409A000C  bne cr6, 0x82e70fac
	if !ctx.cr[6].eq {
		sub_82E70FAC(ctx, base);
		return;
	}
	// 82E70FA4: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82E70FA8: 4BFFECD0  b 0x82e6fc78
	sub_82E6FC78(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E70FAC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E70FAC size=24
    let mut pc: u32 = 0x82E70FAC;
    'dispatch: loop {
        match pc {
            0x82E70FAC => {
    //   block [0x82E70FAC..0x82E70FC4)
	// 82E70FAC: 89430014  lbz r10, 0x14(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E70FB0: 892B0005  lbz r9, 5(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(5 as u32) ) } as u64;
	// 82E70FB4: 512A0638  rlwimi r10, r9, 0, 0x18, 0x1c
	ctx.r[10].u64 = (((ctx.r[9].u32).rotate_left(0) as u64) & 0x00000000000000F8) | (ctx.r[10].u64 & 0xFFFFFFFFFFFFFF07);
	// 82E70FB8: 714A00FB  andi. r10, r10, 0xfb
	ctx.r[10].u64 = ctx.r[10].u64 & 251;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E70FBC: 994B0005  stb r10, 5(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(5 as u32), ctx.r[10].u8 ) };
	// 82E70FC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E70FC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E70FC8 size=32
    let mut pc: u32 = 0x82E70FC8;
    'dispatch: loop {
        match pc {
            0x82E70FC8 => {
    //   block [0x82E70FC8..0x82E70FE8)
	// 82E70FC8: 89440005  lbz r10, 5(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(5 as u32) ) } as u64;
	// 82E70FCC: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E70FD0: 714A00FB  andi. r10, r10, 0xfb
	ctx.r[10].u64 = ctx.r[10].u64 & 251;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E70FD4: 99440005  stb r10, 5(r4)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[4].u32.wrapping_add(5 as u32), ctx.r[10].u8 ) };
	// 82E70FD8: 814B0028  lwz r10, 0x28(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82E70FDC: 91440018  stw r10, 0x18(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 82E70FE0: 908B0028  stw r4, 0x28(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(40 as u32), ctx.r[4].u32 ) };
	// 82E70FE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E70FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E70FE8 size=36
    let mut pc: u32 = 0x82E70FE8;
    'dispatch: loop {
        match pc {
            0x82E70FE8 => {
    //   block [0x82E70FE8..0x82E7100C)
	// 82E70FE8: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E70FEC: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E70FF0: 91440000  stw r10, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E70FF4: 894B0014  lbz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E70FF8: 908B001C  stw r4, 0x1c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[4].u32 ) };
	// 82E70FFC: 554B07BE  clrlwi r11, r10, 0x1e
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x00000003u64;
	// 82E71000: 98A40004  stb r5, 4(r4)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[5].u8 ) };
	// 82E71004: 99640005  stb r11, 5(r4)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[4].u32.wrapping_add(5 as u32), ctx.r[11].u8 ) };
	// 82E71008: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E71010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E71010 size=32
    let mut pc: u32 = 0x82E71010;
    'dispatch: loop {
        match pc {
            0x82E71010 => {
    //   block [0x82E71010..0x82E71030)
	// 82E71010: 81430010  lwz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E71014: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82E71018: 812A001C  lwz r9, 0x1c(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E7101C: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82E71020: 916A001C  stw r11, 0x1c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82E71024: 892B0005  lbz r9, 5(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(5 as u32) ) } as u64;
	// 82E71028: 5528077B  rlwinm. r8, r9, 0, 0x1d, 0x1d
	ctx.r[8].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82E7102C: 4C820020  bnelr
	if !ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E71030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E71030 size=8
    let mut pc: u32 = 0x82E71030;
    'dispatch: loop {
        match pc {
            0x82E71030 => {
    //   block [0x82E71030..0x82E71038)
	// 82E71030: 552807BF  clrlwi. r8, r9, 0x1e
	ctx.r[8].u64 = ctx.r[9].u32 as u64 & 0x00000003u64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82E71034: 4C820020  bnelr
	if !ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E71038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E71038 size=40
    let mut pc: u32 = 0x82E71038;
    'dispatch: loop {
        match pc {
            0x82E71038 => {
    //   block [0x82E71038..0x82E71060)
	// 82E71038: 890A0015  lbz r8, 0x15(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(21 as u32) ) } as u64;
	// 82E7103C: 2B080001  cmplwi cr6, r8, 1
	ctx.cr[6].compare_u32(ctx.r[8].u32, 1 as u32, &mut ctx.xer);
	// 82E71040: 409A0058  bne cr6, 0x82e71098
	if !ctx.cr[6].eq {
		sub_82E71098(ctx, base);
		return;
	}
	// 82E71044: 5529063E  clrlwi r9, r9, 0x18
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x000000FFu64;
	// 82E71048: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E7104C: 61290004  ori r9, r9, 4
	ctx.r[9].u64 = ctx.r[9].u64 | 4;
	// 82E71050: 992B0005  stb r9, 5(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(5 as u32), ctx.r[9].u8 ) };
	// 82E71054: 810A0008  lwz r8, 8(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E71058: 2F080004  cmpwi cr6, r8, 4
	ctx.cr[6].compare_i32(ctx.r[8].s32, 4, &mut ctx.xer);
	// 82E7105C: 4D980020  bltlr cr6
	if ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E71060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E71060 size=16
    let mut pc: u32 = 0x82E71060;
    'dispatch: loop {
        match pc {
            0x82E71060 => {
    //   block [0x82E71060..0x82E71070)
	// 82E71060: 808A0000  lwz r4, 0(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E71064: 89440005  lbz r10, 5(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(5 as u32) ) } as u64;
	// 82E71068: 554A07BF  clrlwi. r10, r10, 0x1e
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000003u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E7106C: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E71070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E71070 size=8
    let mut pc: u32 = 0x82E71070;
    'dispatch: loop {
        match pc {
            0x82E71070 => {
    //   block [0x82E71070..0x82E71078)
	// 82E71070: 552A077B  rlwinm. r10, r9, 0, 0x1d, 0x1d
	ctx.r[10].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E71074: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E71078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E71078 size=20
    let mut pc: u32 = 0x82E71078;
    'dispatch: loop {
        match pc {
            0x82E71078 => {
    //   block [0x82E71078..0x82E7108C)
	// 82E71078: 80630010  lwz r3, 0x10(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E7107C: 89430015  lbz r10, 0x15(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(21 as u32) ) } as u64;
	// 82E71080: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82E71084: 409A0008  bne cr6, 0x82e7108c
	if !ctx.cr[6].eq {
		sub_82E7108C(ctx, base);
		return;
	}
	// 82E71088: 4BFFEBF0  b 0x82e6fc78
	sub_82E6FC78(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E7108C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E7108C size=12
    let mut pc: u32 = 0x82E7108C;
    'dispatch: loop {
        match pc {
            0x82E7108C => {
    //   block [0x82E7108C..0x82E71098)
	// 82E7108C: 89430014  lbz r10, 0x14(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E71090: 892B0005  lbz r9, 5(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(5 as u32) ) } as u64;
	// 82E71094: 48000008  b 0x82e7109c
	sub_82E71098(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E71098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E71098 size=20
    let mut pc: u32 = 0x82E71098;
    'dispatch: loop {
        match pc {
            0x82E71098 => {
    //   block [0x82E71098..0x82E710AC)
	// 82E71098: 894A0014  lbz r10, 0x14(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E7109C: 512A0638  rlwimi r10, r9, 0, 0x18, 0x1c
	ctx.r[10].u64 = (((ctx.r[9].u32).rotate_left(0) as u64) & 0x00000000000000F8) | (ctx.r[10].u64 & 0xFFFFFFFFFFFFFF07);
	// 82E710A0: 714A00FB  andi. r10, r10, 0xfb
	ctx.r[10].u64 = ctx.r[10].u64 & 251;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E710A4: 994B0005  stb r10, 5(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(5 as u32), ctx.r[10].u8 ) };
	// 82E710A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E710B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E710B0 size=1152
    let mut pc: u32 = 0x82E710B0;
    'dispatch: loop {
        match pc {
            0x82E710B0 => {
    //   block [0x82E710B0..0x82E71530)
	// 82E710B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E710B4: 483370A1  bl 0x831a8154
	ctx.lr = 0x82E710B8;
	sub_831A8130(ctx, base);
	// 82E710B8: 8363002C  lwz r27, 0x2c(r3)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82E710BC: 8B83004B  lbz r28, 0x4b(r3)
	ctx.r[28].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(75 as u32) ) } as u64;
	// 82E710C0: 3B3BFFFF  addi r25, r27, -1
	ctx.r[25].s64 = ctx.r[27].s64 + -1;
	// 82E710C4: 2B1C00FA  cmplwi cr6, r28, 0xfa
	ctx.cr[6].compare_u32(ctx.r[28].u32, 250 as u32, &mut ctx.xer);
	// 82E710C8: 41990044  bgt cr6, 0x82e7110c
	if ctx.cr[6].gt {
	pc = 0x82E7110C; continue 'dispatch;
	}
	// 82E710CC: 8AE30048  lbz r23, 0x48(r3)
	ctx.r[23].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 82E710D0: 81630024  lwz r11, 0x24(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 82E710D4: 7F0BB800  cmpw cr6, r11, r23
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[23].s32, &mut ctx.xer);
	// 82E710D8: 41990034  bgt cr6, 0x82e7110c
	if ctx.cr[6].gt {
	pc = 0x82E7110C; continue 'dispatch;
	}
	// 82E710DC: 81630030  lwz r11, 0x30(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E710E0: 7F0BD800  cmpw cr6, r11, r27
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[27].s32, &mut ctx.xer);
	// 82E710E4: 419A000C  beq cr6, 0x82e710f0
	if ctx.cr[6].eq {
	pc = 0x82E710F0; continue 'dispatch;
	}
	// 82E710E8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E710EC: 409A0020  bne cr6, 0x82e7110c
	if !ctx.cr[6].eq {
	pc = 0x82E7110C; continue 'dispatch;
	}
	// 82E710F0: 8343000C  lwz r26, 0xc(r3)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E710F4: 576B103A  slwi r11, r27, 2
	ctx.r[11].u32 = ctx.r[27].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E710F8: 7D6BD214  add r11, r11, r26
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[26].u64;
	// 82E710FC: 816BFFFC  lwz r11, -4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82E71100: 556B06BE  clrlwi r11, r11, 0x1a
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000003Fu64;
	// 82E71104: 2F0B001E  cmpwi cr6, r11, 0x1e
	ctx.cr[6].compare_i32(ctx.r[11].s32, 30, &mut ctx.xer);
	// 82E71108: 419A000C  beq cr6, 0x82e71114
	if ctx.cr[6].eq {
	pc = 0x82E71114; continue 'dispatch;
	}
	// 82E7110C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E71110: 4800041C  b 0x82e7152c
	pc = 0x82E7152C; continue 'dispatch;
	// 82E71114: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82E71118: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82E7111C: 40990408  ble cr6, 0x82e71524
	if !ctx.cr[6].gt {
	pc = 0x82E71524; continue 'dispatch;
	}
	// 82E71120: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E71124: 3B0BDEF4  addi r24, r11, -0x210c
	ctx.r[24].s64 = ctx.r[11].s64 + -8460;
	// 82E71128: 57AB103A  slwi r11, r29, 2
	ctx.r[11].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E7112C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E71130: 7FCBD214  add r30, r11, r26
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[26].u64;
	// 82E71134: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82E71138: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E7113C: 556606BE  clrlwi r6, r11, 0x1a
	ctx.r[6].u64 = ctx.r[11].u32 as u64 & 0x0000003Fu64;
	// 82E71140: 557FD63E  rlwinm r31, r11, 0x1a, 0x18, 0x1f
	ctx.r[31].u64 = ctx.r[11].u32 as u64 & 0x0000003Fu64;
	// 82E71144: 2F060026  cmpwi cr6, r6, 0x26
	ctx.cr[6].compare_i32(ctx.r[6].s32, 38, &mut ctx.xer);
	// 82E71148: 4098FFC4  bge cr6, 0x82e7110c
	if !ctx.cr[6].lt {
	pc = 0x82E7110C; continue 'dispatch;
	}
	// 82E7114C: 7F1FE000  cmpw cr6, r31, r28
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[28].s32, &mut ctx.xer);
	// 82E71150: 4098FFBC  bge cr6, 0x82e7110c
	if !ctx.cr[6].lt {
	pc = 0x82E7110C; continue 'dispatch;
	}
	// 82E71154: 7D06C0AE  lbzx r8, r6, r24
	ctx.r[8].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[6].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82E71158: 550907BE  clrlwi r9, r8, 0x1e
	ctx.r[9].u64 = ctx.r[8].u32 as u64 & 0x00000003u64;
	// 82E7115C: 2B090001  cmplwi cr6, r9, 1
	ctx.cr[6].compare_u32(ctx.r[9].u32, 1 as u32, &mut ctx.xer);
	// 82E71160: 41980084  blt cr6, 0x82e711e4
	if ctx.cr[6].lt {
	pc = 0x82E711E4; continue 'dispatch;
	}
	// 82E71164: 419A0064  beq cr6, 0x82e711c8
	if ctx.cr[6].eq {
	pc = 0x82E711C8; continue 'dispatch;
	}
	// 82E71168: 2B090003  cmplwi cr6, r9, 3
	ctx.cr[6].compare_u32(ctx.r[9].u32, 3 as u32, &mut ctx.xer);
	// 82E7116C: 40980108  bge cr6, 0x82e71274
	if !ctx.cr[6].lt {
	pc = 0x82E71274; continue 'dispatch;
	}
	// 82E71170: 556B93BE  srwi r11, r11, 0xe
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shr(14);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E71174: 550906B6  rlwinm r9, r8, 0, 0x1a, 0x1b
	ctx.r[9].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	// 82E71178: 3D4BFFFE  addis r10, r11, -2
	ctx.r[10].s64 = ctx.r[11].s64 + -131072;
	// 82E7117C: 2B090020  cmplwi cr6, r9, 0x20
	ctx.cr[6].compare_u32(ctx.r[9].u32, 32 as u32, &mut ctx.xer);
	// 82E71180: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E71184: 409A00F0  bne cr6, 0x82e71274
	if !ctx.cr[6].eq {
	pc = 0x82E71274; continue 'dispatch;
	}
	// 82E71188: 7D6AEA14  add r11, r10, r29
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[29].u64;
	// 82E7118C: 356B0001  addic. r11, r11, 1
	ctx.xer.ca = (ctx.r[11].u32 > (!(1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E71190: 4180FF7C  blt 0x82e7110c
	if ctx.cr[0].lt {
	pc = 0x82E7110C; continue 'dispatch;
	}
	// 82E71194: 7F0BD800  cmpw cr6, r11, r27
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[27].s32, &mut ctx.xer);
	// 82E71198: 4098FF74  bge cr6, 0x82e7110c
	if !ctx.cr[6].lt {
	pc = 0x82E7110C; continue 'dispatch;
	}
	// 82E7119C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E711A0: 409900D4  ble cr6, 0x82e71274
	if !ctx.cr[6].gt {
	pc = 0x82E71274; continue 'dispatch;
	}
	// 82E711A4: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E711A8: 7D6BD214  add r11, r11, r26
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[26].u64;
	// 82E711AC: 816BFFFC  lwz r11, -4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82E711B0: 556906BE  clrlwi r9, r11, 0x1a
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x0000003Fu64;
	// 82E711B4: 2F090022  cmpwi cr6, r9, 0x22
	ctx.cr[6].compare_i32(ctx.r[9].s32, 34, &mut ctx.xer);
	// 82E711B8: 409A00BC  bne cr6, 0x82e71274
	if !ctx.cr[6].eq {
	pc = 0x82E71274; continue 'dispatch;
	}
	// 82E711BC: 556B0263  rlwinm. r11, r11, 0, 9, 0x11
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E711C0: 4182FF4C  beq 0x82e7110c
	if ctx.cr[0].eq {
	pc = 0x82E7110C; continue 'dispatch;
	}
	// 82E711C4: 480000B0  b 0x82e71274
	pc = 0x82E71274; continue 'dispatch;
	// 82E711C8: 556A93BE  srwi r10, r11, 0xe
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shr(14);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E711CC: 550B06B6  rlwinm r11, r8, 0, 0x1a, 0x1b
	ctx.r[11].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	// 82E711D0: 2B0B0030  cmplwi cr6, r11, 0x30
	ctx.cr[6].compare_u32(ctx.r[11].u32, 48 as u32, &mut ctx.xer);
	// 82E711D4: 409A00A0  bne cr6, 0x82e71274
	if !ctx.cr[6].eq {
	pc = 0x82E71274; continue 'dispatch;
	}
	// 82E711D8: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82E711DC: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E711E0: 48000084  b 0x82e71264
	pc = 0x82E71264; continue 'dispatch;
	// 82E711E4: 5509E7BF  rlwinm. r9, r8, 0x1c, 0x1e, 0x1f
	ctx.r[9].u64 = ctx.r[8].u32 as u64 & 0x0000000Fu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E711E8: 556A4DFE  srwi r10, r11, 0x17
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shr(23);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E711EC: 556795FE  rlwinm r7, r11, 0x12, 0x17, 0x1f
	ctx.r[7].u64 = ctx.r[11].u32 as u64 & 0x00003FFFu64;
	// 82E711F0: 41820038  beq 0x82e71228
	if ctx.cr[0].eq {
	pc = 0x82E71228; continue 'dispatch;
	}
	// 82E711F4: 2F090002  cmpwi cr6, r9, 2
	ctx.cr[6].compare_i32(ctx.r[9].s32, 2, &mut ctx.xer);
	// 82E711F8: 419A0024  beq cr6, 0x82e7121c
	if ctx.cr[6].eq {
	pc = 0x82E7121C; continue 'dispatch;
	}
	// 82E711FC: 2F090003  cmpwi cr6, r9, 3
	ctx.cr[6].compare_i32(ctx.r[9].s32, 3, &mut ctx.xer);
	// 82E71200: 409A0030  bne cr6, 0x82e71230
	if !ctx.cr[6].eq {
	pc = 0x82E71230; continue 'dispatch;
	}
	// 82E71204: 554B05EF  rlwinm. r11, r10, 0, 0x17, 0x17
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E71208: 41820014  beq 0x82e7121c
	if ctx.cr[0].eq {
	pc = 0x82E7121C; continue 'dispatch;
	}
	// 82E7120C: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82E71210: 5549062C  rlwinm r9, r10, 0, 0x18, 0x16
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82E71214: 7F095800  cmpw cr6, r9, r11
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E71218: 48000008  b 0x82e71220
	pc = 0x82E71220; continue 'dispatch;
	// 82E7121C: 7F0AE000  cmpw cr6, r10, r28
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[28].s32, &mut ctx.xer);
	// 82E71220: 4098FEEC  bge cr6, 0x82e7110c
	if !ctx.cr[6].lt {
	pc = 0x82E7110C; continue 'dispatch;
	}
	// 82E71224: 4800000C  b 0x82e71230
	pc = 0x82E71230; continue 'dispatch;
	// 82E71228: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E7122C: 409AFEE0  bne cr6, 0x82e7110c
	if !ctx.cr[6].eq {
	pc = 0x82E7110C; continue 'dispatch;
	}
	// 82E71230: 550BF7BF  rlwinm. r11, r8, 0x1e, 0x1e, 0x1f
	ctx.r[11].u64 = ctx.r[8].u32 as u64 & 0x00000003u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E71234: 41820038  beq 0x82e7126c
	if ctx.cr[0].eq {
	pc = 0x82E7126C; continue 'dispatch;
	}
	// 82E71238: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82E7123C: 419A0024  beq cr6, 0x82e71260
	if ctx.cr[6].eq {
	pc = 0x82E71260; continue 'dispatch;
	}
	// 82E71240: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 82E71244: 409A0030  bne cr6, 0x82e71274
	if !ctx.cr[6].eq {
	pc = 0x82E71274; continue 'dispatch;
	}
	// 82E71248: 54EB05EF  rlwinm. r11, r7, 0, 0x17, 0x17
	ctx.r[11].u64 = ctx.r[7].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E7124C: 41820014  beq 0x82e71260
	if ctx.cr[0].eq {
	pc = 0x82E71260; continue 'dispatch;
	}
	// 82E71250: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82E71254: 54E9062C  rlwinm r9, r7, 0, 0x18, 0x16
	ctx.r[9].u64 = ctx.r[7].u32 as u64 & 0xFFFFFFFFu64;
	// 82E71258: 7F095800  cmpw cr6, r9, r11
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E7125C: 48000008  b 0x82e71264
	pc = 0x82E71264; continue 'dispatch;
	// 82E71260: 7F07E000  cmpw cr6, r7, r28
	ctx.cr[6].compare_i32(ctx.r[7].s32, ctx.r[28].s32, &mut ctx.xer);
	// 82E71264: 4098FEA8  bge cr6, 0x82e7110c
	if !ctx.cr[6].lt {
	pc = 0x82E7110C; continue 'dispatch;
	}
	// 82E71268: 4800000C  b 0x82e71274
	pc = 0x82E71274; continue 'dispatch;
	// 82E7126C: 2F070000  cmpwi cr6, r7, 0
	ctx.cr[6].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82E71270: 409AFE9C  bne cr6, 0x82e7110c
	if !ctx.cr[6].eq {
	pc = 0x82E7110C; continue 'dispatch;
	}
	// 82E71274: 550B0673  rlwinm. r11, r8, 0, 0x19, 0x19
	ctx.r[11].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E71278: 41820010  beq 0x82e71288
	if ctx.cr[0].eq {
	pc = 0x82E71288; continue 'dispatch;
	}
	// 82E7127C: 7F1F2800  cmpw cr6, r31, r5
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[5].s32, &mut ctx.xer);
	// 82E71280: 409A0008  bne cr6, 0x82e71288
	if !ctx.cr[6].eq {
	pc = 0x82E71288; continue 'dispatch;
	}
	// 82E71284: 7FB9EB78  mr r25, r29
	ctx.r[25].u64 = ctx.r[29].u64;
	// 82E71288: 550B0631  rlwinm. r11, r8, 0, 0x18, 0x18
	ctx.r[11].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E7128C: 41820020  beq 0x82e712ac
	if ctx.cr[0].eq {
	pc = 0x82E712AC; continue 'dispatch;
	}
	// 82E71290: 397D0002  addi r11, r29, 2
	ctx.r[11].s64 = ctx.r[29].s64 + 2;
	// 82E71294: 7F0BD800  cmpw cr6, r11, r27
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[27].s32, &mut ctx.xer);
	// 82E71298: 4098FE74  bge cr6, 0x82e7110c
	if !ctx.cr[6].lt {
	pc = 0x82E7110C; continue 'dispatch;
	}
	// 82E7129C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E712A0: 556B06BE  clrlwi r11, r11, 0x1a
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000003Fu64;
	// 82E712A4: 2F0B0016  cmpwi cr6, r11, 0x16
	ctx.cr[6].compare_i32(ctx.r[11].s32, 22, &mut ctx.xer);
	// 82E712A8: 409AFE64  bne cr6, 0x82e7110c
	if !ctx.cr[6].eq {
	pc = 0x82E7110C; continue 'dispatch;
	}
	// 82E712AC: 3966FFFE  addi r11, r6, -2
	ctx.r[11].s64 = ctx.r[6].s64 + -2;
	// 82E712B0: 2B0B0023  cmplwi cr6, r11, 0x23
	ctx.cr[6].compare_u32(ctx.r[11].u32, 35 as u32, &mut ctx.xer);
	// 82E712B4: 41990264  bgt cr6, 0x82e71518
	if ctx.cr[6].gt {
	pc = 0x82E71518; continue 'dispatch;
	}
	// 82E712B8: 3D808212  lis r12, -0x7dee
	ctx.r[12].s64 = -2112749568;
	// 82E712BC: 398CD4C8  addi r12, r12, -0x2b38
	ctx.r[12].s64 = ctx.r[12].s64 + -11064;
	// 82E712C0: 7C0C58AE  lbzx r0, r12, r11
	ctx.r[0].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E712C4: 5400103A  slwi r0, r0, 2
	ctx.r[0].u32 = ctx.r[0].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 82E712C8: 3D8082E7  lis r12, -0x7d19
	ctx.r[12].s64 = -2098790400;
	// 82E712CC: 398C12E0  addi r12, r12, 0x12e0
	ctx.r[12].s64 = ctx.r[12].s64 + 4832;
	// 82E712D0: 7D8C0214  add r12, r12, r0
	ctx.r[12].u64 = ctx.r[12].u64 + ctx.r[0].u64;
	// 82E712D4: 7D8903A6  mtctr r12
	ctx.ctr.u64 = ctx.r[12].u64;
	// 82E712D8: 60000000  nop
	// 82E712DC: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
	// 82E712E0: 2F070000  cmpwi cr6, r7, 0
	ctx.cr[6].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82E712E4: 419A0234  beq cr6, 0x82e71518
	if ctx.cr[6].eq {
	pc = 0x82E71518; continue 'dispatch;
	}
	// 82E712E8: 397D0002  addi r11, r29, 2
	ctx.r[11].s64 = ctx.r[29].s64 + 2;
	// 82E712EC: 7F0BD800  cmpw cr6, r11, r27
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[27].s32, &mut ctx.xer);
	// 82E712F0: 48000224  b 0x82e71514
	pc = 0x82E71514; continue 'dispatch;
	// 82E712F4: 7F1F2800  cmpw cr6, r31, r5
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[5].s32, &mut ctx.xer);
	// 82E712F8: 41990220  bgt cr6, 0x82e71518
	if ctx.cr[6].gt {
	pc = 0x82E71518; continue 'dispatch;
	}
	// 82E712FC: 7F055000  cmpw cr6, r5, r10
	ctx.cr[6].compare_i32(ctx.r[5].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82E71300: 41990218  bgt cr6, 0x82e71518
	if ctx.cr[6].gt {
	pc = 0x82E71518; continue 'dispatch;
	}
	// 82E71304: 7FB9EB78  mr r25, r29
	ctx.r[25].u64 = ctx.r[29].u64;
	// 82E71308: 48000210  b 0x82e71518
	pc = 0x82E71518; continue 'dispatch;
	// 82E7130C: 7F0AB800  cmpw cr6, r10, r23
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[23].s32, &mut ctx.xer);
	// 82E71310: 48000204  b 0x82e71514
	pc = 0x82E71514; continue 'dispatch;
	// 82E71314: 81230008  lwz r9, 8(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E71318: 554B2036  slwi r11, r10, 4
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E7131C: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82E71320: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E71324: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 82E71328: 409AFDE4  bne cr6, 0x82e7110c
	if !ctx.cr[6].eq {
	pc = 0x82E7110C; continue 'dispatch;
	}
	// 82E7132C: 480001EC  b 0x82e71518
	pc = 0x82E71518; continue 'dispatch;
	// 82E71330: 397F0001  addi r11, r31, 1
	ctx.r[11].s64 = ctx.r[31].s64 + 1;
	// 82E71334: 7F0BE000  cmpw cr6, r11, r28
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[28].s32, &mut ctx.xer);
	// 82E71338: 4098FDD4  bge cr6, 0x82e7110c
	if !ctx.cr[6].lt {
	pc = 0x82E7110C; continue 'dispatch;
	}
	// 82E7133C: 7F055800  cmpw cr6, r5, r11
	ctx.cr[6].compare_i32(ctx.r[5].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E71340: 409A01D8  bne cr6, 0x82e71518
	if !ctx.cr[6].eq {
	pc = 0x82E71518; continue 'dispatch;
	}
	// 82E71344: 4BFFFFC0  b 0x82e71304
	pc = 0x82E71304; continue 'dispatch;
	// 82E71348: 7F0A3800  cmpw cr6, r10, r7
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[7].s32, &mut ctx.xer);
	// 82E7134C: 480001C8  b 0x82e71514
	pc = 0x82E71514; continue 'dispatch;
	// 82E71350: 2F070001  cmpwi cr6, r7, 1
	ctx.cr[6].compare_i32(ctx.r[7].s32, 1, &mut ctx.xer);
	// 82E71354: 4198FDB8  blt cr6, 0x82e7110c
	if ctx.cr[6].lt {
	pc = 0x82E7110C; continue 'dispatch;
	}
	// 82E71358: 7D67FA14  add r11, r7, r31
	ctx.r[11].u64 = ctx.r[7].u64 + ctx.r[31].u64;
	// 82E7135C: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82E71360: 7F0BE000  cmpw cr6, r11, r28
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[28].s32, &mut ctx.xer);
	// 82E71364: 4098FDA8  bge cr6, 0x82e7110c
	if !ctx.cr[6].lt {
	pc = 0x82E7110C; continue 'dispatch;
	}
	// 82E71368: 397F0002  addi r11, r31, 2
	ctx.r[11].s64 = ctx.r[31].s64 + 2;
	// 82E7136C: 7F055800  cmpw cr6, r5, r11
	ctx.cr[6].compare_i32(ctx.r[5].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E71370: 419801A8  blt cr6, 0x82e71518
	if ctx.cr[6].lt {
	pc = 0x82E71518; continue 'dispatch;
	}
	// 82E71374: 4BFFFF90  b 0x82e71304
	pc = 0x82E71304; continue 'dispatch;
	// 82E71378: 397F0003  addi r11, r31, 3
	ctx.r[11].s64 = ctx.r[31].s64 + 3;
	// 82E7137C: 7F0BE000  cmpw cr6, r11, r28
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[28].s32, &mut ctx.xer);
	// 82E71380: 4098FD8C  bge cr6, 0x82e7110c
	if !ctx.cr[6].lt {
	pc = 0x82E7110C; continue 'dispatch;
	}
	// 82E71384: 7D4AEA14  add r10, r10, r29
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[29].u64;
	// 82E71388: 2F0500FF  cmpwi cr6, r5, 0xff
	ctx.cr[6].compare_i32(ctx.r[5].s32, 255, &mut ctx.xer);
	// 82E7138C: 396A0001  addi r11, r10, 1
	ctx.r[11].s64 = ctx.r[10].s64 + 1;
	// 82E71390: 419A0188  beq cr6, 0x82e71518
	if ctx.cr[6].eq {
	pc = 0x82E71518; continue 'dispatch;
	}
	// 82E71394: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E71398: 40980180  bge cr6, 0x82e71518
	if !ctx.cr[6].lt {
	pc = 0x82E71518; continue 'dispatch;
	}
	// 82E7139C: 7F0B2000  cmpw cr6, r11, r4
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[4].s32, &mut ctx.xer);
	// 82E713A0: 41990178  bgt cr6, 0x82e71518
	if ctx.cr[6].gt {
	pc = 0x82E71518; continue 'dispatch;
	}
	// 82E713A4: 7D5D5378  mr r29, r10
	ctx.r[29].u64 = ctx.r[10].u64;
	// 82E713A8: 48000170  b 0x82e71518
	pc = 0x82E71518; continue 'dispatch;
	// 82E713AC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E713B0: 419A0014  beq cr6, 0x82e713c4
	if ctx.cr[6].eq {
	pc = 0x82E713C4; continue 'dispatch;
	}
	// 82E713B4: 7D6AFA14  add r11, r10, r31
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 82E713B8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82E713BC: 7F0BE000  cmpw cr6, r11, r28
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[28].s32, &mut ctx.xer);
	// 82E713C0: 4098FD4C  bge cr6, 0x82e7110c
	if !ctx.cr[6].lt {
	pc = 0x82E7110C; continue 'dispatch;
	}
	// 82E713C4: 3967FFFF  addi r11, r7, -1
	ctx.r[11].s64 = ctx.r[7].s64 + -1;
	// 82E713C8: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 82E713CC: 409A0030  bne cr6, 0x82e713fc
	if !ctx.cr[6].eq {
	pc = 0x82E713FC; continue 'dispatch;
	}
	// 82E713D0: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E713D4: 554B06BE  clrlwi r11, r10, 0x1a
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x0000003Fu64;
	// 82E713D8: 2F0B001C  cmpwi cr6, r11, 0x1c
	ctx.cr[6].compare_i32(ctx.r[11].s32, 28, &mut ctx.xer);
	// 82E713DC: 4198FD30  blt cr6, 0x82e7110c
	if ctx.cr[6].lt {
	pc = 0x82E7110C; continue 'dispatch;
	}
	// 82E713E0: 2F0B001E  cmpwi cr6, r11, 0x1e
	ctx.cr[6].compare_i32(ctx.r[11].s32, 30, &mut ctx.xer);
	// 82E713E4: 4099000C  ble cr6, 0x82e713f0
	if !ctx.cr[6].gt {
	pc = 0x82E713F0; continue 'dispatch;
	}
	// 82E713E8: 2F0B0022  cmpwi cr6, r11, 0x22
	ctx.cr[6].compare_i32(ctx.r[11].s32, 34, &mut ctx.xer);
	// 82E713EC: 409AFD20  bne cr6, 0x82e7110c
	if !ctx.cr[6].eq {
	pc = 0x82E7110C; continue 'dispatch;
	}
	// 82E713F0: 554B0011  rlwinm. r11, r10, 0, 0, 8
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E713F4: 4082FD18  bne 0x82e7110c
	if !ctx.cr[0].eq {
	pc = 0x82E7110C; continue 'dispatch;
	}
	// 82E713F8: 4800001C  b 0x82e71414
	pc = 0x82E71414; continue 'dispatch;
	// 82E713FC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E71400: 419A0014  beq cr6, 0x82e71414
	if ctx.cr[6].eq {
	pc = 0x82E71414; continue 'dispatch;
	}
	// 82E71404: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82E71408: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82E7140C: 7F0BE000  cmpw cr6, r11, r28
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[28].s32, &mut ctx.xer);
	// 82E71410: 4098FCFC  bge cr6, 0x82e7110c
	if !ctx.cr[6].lt {
	pc = 0x82E7110C; continue 'dispatch;
	}
	// 82E71414: 7F05F800  cmpw cr6, r5, r31
	ctx.cr[6].compare_i32(ctx.r[5].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82E71418: 4BFFFF58  b 0x82e71370
	pc = 0x82E71370; continue 'dispatch;
	// 82E7141C: 356AFFFF  addic. r11, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E71420: 408100F8  ble 0x82e71518
	if !ctx.cr[0].gt {
	pc = 0x82E71518; continue 'dispatch;
	}
	// 82E71424: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82E71428: 480000E4  b 0x82e7150c
	pc = 0x82E7150C; continue 'dispatch;
	// 82E7142C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E71430: 40990010  ble cr6, 0x82e71440
	if !ctx.cr[6].gt {
	pc = 0x82E71440; continue 'dispatch;
	}
	// 82E71434: 7D6AFA14  add r11, r10, r31
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 82E71438: 7F0BE000  cmpw cr6, r11, r28
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[28].s32, &mut ctx.xer);
	// 82E7143C: 4098FCD0  bge cr6, 0x82e7110c
	if !ctx.cr[6].lt {
	pc = 0x82E7110C; continue 'dispatch;
	}
	// 82E71440: 2F070000  cmpwi cr6, r7, 0
	ctx.cr[6].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82E71444: 409A00D4  bne cr6, 0x82e71518
	if !ctx.cr[6].eq {
	pc = 0x82E71518; continue 'dispatch;
	}
	// 82E71448: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82E7144C: 480000CC  b 0x82e71518
	pc = 0x82E71518; continue 'dispatch;
	// 82E71450: 81630034  lwz r11, 0x34(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 82E71454: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E71458: 4098FCB4  bge cr6, 0x82e7110c
	if !ctx.cr[6].lt {
	pc = 0x82E7110C; continue 'dispatch;
	}
	// 82E7145C: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E71460: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E71464: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E71468: 896B0048  lbz r11, 0x48(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 82E7146C: 7CEBEA14  add r7, r11, r29
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82E71470: 7F07D800  cmpw cr6, r7, r27
	ctx.cr[6].compare_i32(ctx.r[7].s32, ctx.r[27].s32, &mut ctx.xer);
	// 82E71474: 4098FC98  bge cr6, 0x82e7110c
	if !ctx.cr[6].lt {
	pc = 0x82E7110C; continue 'dispatch;
	}
	// 82E71478: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82E7147C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82E71480: 41980030  blt cr6, 0x82e714b0
	if ctx.cr[6].lt {
	pc = 0x82E714B0; continue 'dispatch;
	}
	// 82E71484: 395E0004  addi r10, r30, 4
	ctx.r[10].s64 = ctx.r[30].s64 + 4;
	// 82E71488: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E7148C: 550806BE  clrlwi r8, r8, 0x1a
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0x0000003Fu64;
	// 82E71490: 2F080004  cmpwi cr6, r8, 4
	ctx.cr[6].compare_i32(ctx.r[8].s32, 4, &mut ctx.xer);
	// 82E71494: 419A000C  beq cr6, 0x82e714a0
	if ctx.cr[6].eq {
	pc = 0x82E714A0; continue 'dispatch;
	}
	// 82E71498: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82E7149C: 409AFC70  bne cr6, 0x82e7110c
	if !ctx.cr[6].eq {
	pc = 0x82E7110C; continue 'dispatch;
	}
	// 82E714A0: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82E714A4: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82E714A8: 7F095800  cmpw cr6, r9, r11
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82E714AC: 4099FFDC  ble cr6, 0x82e71488
	if !ctx.cr[6].gt {
	pc = 0x82E71488; continue 'dispatch;
	}
	// 82E714B0: 2F0500FF  cmpwi cr6, r5, 0xff
	ctx.cr[6].compare_i32(ctx.r[5].s32, 255, &mut ctx.xer);
	// 82E714B4: 419A0064  beq cr6, 0x82e71518
	if ctx.cr[6].eq {
	pc = 0x82E71518; continue 'dispatch;
	}
	// 82E714B8: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 82E714BC: 4800005C  b 0x82e71518
	pc = 0x82E71518; continue 'dispatch;
	// 82E714C0: 8963004A  lbz r11, 0x4a(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(74 as u32) ) } as u64;
	// 82E714C4: 556907BD  rlwinm. r9, r11, 0, 0x1e, 0x1e
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E714C8: 4182FC44  beq 0x82e7110c
	if ctx.cr[0].eq {
	pc = 0x82E7110C; continue 'dispatch;
	}
	// 82E714CC: 556B077B  rlwinm. r11, r11, 0, 0x1d, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E714D0: 4082FC3C  bne 0x82e7110c
	if !ctx.cr[0].eq {
	pc = 0x82E7110C; continue 'dispatch;
	}
	// 82E714D4: 392AFFFF  addi r9, r10, -1
	ctx.r[9].s64 = ctx.r[10].s64 + -1;
	// 82E714D8: 2F09FFFF  cmpwi cr6, r9, -1
	ctx.cr[6].compare_i32(ctx.r[9].s32, -1, &mut ctx.xer);
	// 82E714DC: 409A002C  bne cr6, 0x82e71508
	if !ctx.cr[6].eq {
	pc = 0x82E71508; continue 'dispatch;
	}
	// 82E714E0: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E714E4: 554B06BE  clrlwi r11, r10, 0x1a
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x0000003Fu64;
	// 82E714E8: 2F0B001C  cmpwi cr6, r11, 0x1c
	ctx.cr[6].compare_i32(ctx.r[11].s32, 28, &mut ctx.xer);
	// 82E714EC: 4198FC20  blt cr6, 0x82e7110c
	if ctx.cr[6].lt {
	pc = 0x82E7110C; continue 'dispatch;
	}
	// 82E714F0: 2F0B001E  cmpwi cr6, r11, 0x1e
	ctx.cr[6].compare_i32(ctx.r[11].s32, 30, &mut ctx.xer);
	// 82E714F4: 4099000C  ble cr6, 0x82e71500
	if !ctx.cr[6].gt {
	pc = 0x82E71500; continue 'dispatch;
	}
	// 82E714F8: 2F0B0022  cmpwi cr6, r11, 0x22
	ctx.cr[6].compare_i32(ctx.r[11].s32, 34, &mut ctx.xer);
	// 82E714FC: 409AFC10  bne cr6, 0x82e7110c
	if !ctx.cr[6].eq {
	pc = 0x82E7110C; continue 'dispatch;
	}
	// 82E71500: 554B0011  rlwinm. r11, r10, 0, 0, 8
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E71504: 4082FC08  bne 0x82e7110c
	if !ctx.cr[0].eq {
	pc = 0x82E7110C; continue 'dispatch;
	}
	// 82E71508: 7D69FA14  add r11, r9, r31
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[31].u64;
	// 82E7150C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82E71510: 7F0BE000  cmpw cr6, r11, r28
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[28].s32, &mut ctx.xer);
	// 82E71514: 4098FBF8  bge cr6, 0x82e7110c
	if !ctx.cr[6].lt {
	pc = 0x82E7110C; continue 'dispatch;
	}
	// 82E71518: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82E7151C: 7F1D2000  cmpw cr6, r29, r4
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[4].s32, &mut ctx.xer);
	// 82E71520: 4198FC08  blt cr6, 0x82e71128
	if ctx.cr[6].lt {
	pc = 0x82E71128; continue 'dispatch;
	}
	// 82E71524: 572B103A  slwi r11, r25, 2
	ctx.r[11].u32 = ctx.r[25].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E71528: 7C6BD02E  lwzx r3, r11, r26
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82E7152C: 48336C78  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E71530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E71530 size=52
    let mut pc: u32 = 0x82E71530;
    'dispatch: loop {
        match pc {
            0x82E71530 => {
    //   block [0x82E71530..0x82E71564)
	// 82E71530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E71534: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E71538: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E7153C: 38A000FF  li r5, 0xff
	ctx.r[5].s64 = 255;
	// 82E71540: 8083002C  lwz r4, 0x2c(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82E71544: 4BFFFB6D  bl 0x82e710b0
	ctx.lr = 0x82E71548;
	sub_82E710B0(ctx, base);
	// 82E71548: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 82E7154C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82E71550: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 82E71554: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E71558: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E7155C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E71560: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E71568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E71568 size=500
    let mut pc: u32 = 0x82E71568;
    'dispatch: loop {
        match pc {
            0x82E71568 => {
    //   block [0x82E71568..0x82E7175C)
	// 82E71568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E7156C: 48336BF5  bl 0x831a8160
	ctx.lr = 0x82E71570;
	sub_831A8130(ctx, base);
	// 82E71570: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E71574: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82E71578: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82E7157C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82E71580: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82E71584: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E71588: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E7158C: 480000B0  b 0x82e7163c
	pc = 0x82E7163C; continue 'dispatch;
	// 82E71590: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E71594: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E71598: 892A0006  lbz r9, 6(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(6 as u32) ) } as u64;
	// 82E7159C: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E715A0: 408200A4  bne 0x82e71644
	if !ctx.cr[0].eq {
	pc = 0x82E71644; continue 'dispatch;
	}
	// 82E715A4: 812B0008  lwz r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E715A8: 83CA0010  lwz r30, 0x10(r10)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E715AC: 2F090006  cmpwi cr6, r9, 6
	ctx.cr[6].compare_i32(ctx.r[9].s32, 6, &mut ctx.xer);
	// 82E715B0: 409A0038  bne cr6, 0x82e715e8
	if !ctx.cr[6].eq {
	pc = 0x82E715E8; continue 'dispatch;
	}
	// 82E715B4: 815B0014  lwz r10, 0x14(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E715B8: 7F1D5040  cmplw cr6, r29, r10
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82E715BC: 409A000C  bne cr6, 0x82e715c8
	if !ctx.cr[6].eq {
	pc = 0x82E715C8; continue 'dispatch;
	}
	// 82E715C0: 815B0018  lwz r10, 0x18(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E715C4: 915D000C  stw r10, 0xc(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82E715C8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E715CC: 815D000C  lwz r10, 0xc(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E715D0: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E715D4: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E715D8: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82E715DC: 7D6B1670  srawi r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 82E715E0: 3BEBFFFF  addi r31, r11, -1
	ctx.r[31].s64 = ctx.r[11].s64 + -1;
	// 82E715E4: 48000008  b 0x82e715ec
	pc = 0x82E715EC; continue 'dispatch;
	// 82E715E8: 3BE0FFFF  li r31, -1
	ctx.r[31].s64 = -1;
	// 82E715EC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82E715F0: 389C0001  addi r4, r28, 1
	ctx.r[4].s64 = ctx.r[28].s64 + 1;
	// 82E715F4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E715F8: 480037F1  bl 0x82e74de8
	ctx.lr = 0x82E715FC;
	sub_82E74DE8(ctx, base);
	// 82E715FC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E71600: 907A0000  stw r3, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82E71604: 4082004C  bne 0x82e71650
	if !ctx.cr[0].eq {
	pc = 0x82E71650; continue 'dispatch;
	}
	// 82E71608: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82E7160C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E71610: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E71614: 4BFFFA9D  bl 0x82e710b0
	ctx.lr = 0x82E71618;
	sub_82E710B0(ctx, base);
	// 82E71618: 546B06BF  clrlwi. r11, r3, 0x1a
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x0000003Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E7161C: 40820040  bne 0x82e7165c
	if !ctx.cr[0].eq {
	pc = 0x82E7165C; continue 'dispatch;
	}
	// 82E71620: 546B4DFE  srwi r11, r3, 0x17
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shr(23);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E71624: 546AD63E  rlwinm r10, r3, 0x1a, 0x18, 0x1f
	ctx.r[10].u64 = ctx.r[3].u32 as u64 & 0x0000003Fu64;
	// 82E71628: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82E7162C: 40980018  bge cr6, 0x82e71644
	if !ctx.cr[6].lt {
	pc = 0x82E71644; continue 'dispatch;
	}
	// 82E71630: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E71634: 7D7C5B78  mr r28, r11
	ctx.r[28].u64 = ctx.r[11].u64;
	// 82E71638: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E7163C: 2F0B0006  cmpwi cr6, r11, 6
	ctx.cr[6].compare_i32(ctx.r[11].s32, 6, &mut ctx.xer);
	// 82E71640: 419AFF50  beq cr6, 0x82e71590
	if ctx.cr[6].eq {
	pc = 0x82E71590; continue 'dispatch;
	}
	// 82E71644: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E71648: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E7164C: 48336B64  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
	// 82E71650: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E71654: 386BD4FC  addi r3, r11, -0x2b04
	ctx.r[3].s64 = ctx.r[11].s64 + -11012;
	// 82E71658: 4BFFFFF0  b 0x82e71648
	pc = 0x82E71648; continue 'dispatch;
	// 82E7165C: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 82E71660: 419A00CC  beq cr6, 0x82e7172c
	if ctx.cr[6].eq {
	pc = 0x82E7172C; continue 'dispatch;
	}
	// 82E71664: 2F0B0005  cmpwi cr6, r11, 5
	ctx.cr[6].compare_i32(ctx.r[11].s32, 5, &mut ctx.xer);
	// 82E71668: 419A00A4  beq cr6, 0x82e7170c
	if ctx.cr[6].eq {
	pc = 0x82E7170C; continue 'dispatch;
	}
	// 82E7166C: 2F0B0006  cmpwi cr6, r11, 6
	ctx.cr[6].compare_i32(ctx.r[11].s32, 6, &mut ctx.xer);
	// 82E71670: 419A0054  beq cr6, 0x82e716c4
	if ctx.cr[6].eq {
	pc = 0x82E716C4; continue 'dispatch;
	}
	// 82E71674: 2F0B000B  cmpwi cr6, r11, 0xb
	ctx.cr[6].compare_i32(ctx.r[11].s32, 11, &mut ctx.xer);
	// 82E71678: 409AFFCC  bne cr6, 0x82e71644
	if !ctx.cr[6].eq {
	pc = 0x82E71644; continue 'dispatch;
	}
	// 82E7167C: 546B95FE  rlwinm r11, r3, 0x12, 0x17, 0x1f
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x00003FFFu64;
	// 82E71680: 556A05EF  rlwinm. r10, r11, 0, 0x17, 0x17
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E71684: 4182002C  beq 0x82e716b0
	if ctx.cr[0].eq {
	pc = 0x82E716B0; continue 'dispatch;
	}
	// 82E71688: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E7168C: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E71690: 556B0524  rlwinm r11, r11, 0, 0x14, 0x12
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E71694: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82E71698: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E7169C: 2F0A0004  cmpwi cr6, r10, 4
	ctx.cr[6].compare_i32(ctx.r[10].s32, 4, &mut ctx.xer);
	// 82E716A0: 409A0010  bne cr6, 0x82e716b0
	if !ctx.cr[6].eq {
	pc = 0x82E716B0; continue 'dispatch;
	}
	// 82E716A4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E716A8: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82E716AC: 4800000C  b 0x82e716b8
	pc = 0x82E716B8; continue 'dispatch;
	// 82E716B0: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 82E716B4: 396B7EA8  addi r11, r11, 0x7ea8
	ctx.r[11].s64 = ctx.r[11].s64 + 32424;
	// 82E716B8: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82E716BC: 386AD390  addi r3, r10, -0x2c70
	ctx.r[3].s64 = ctx.r[10].s64 + -11376;
	// 82E716C0: 48000064  b 0x82e71724
	pc = 0x82E71724; continue 'dispatch;
	// 82E716C4: 546B95FE  rlwinm r11, r3, 0x12, 0x17, 0x1f
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x00003FFFu64;
	// 82E716C8: 556A05EF  rlwinm. r10, r11, 0, 0x17, 0x17
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82E716CC: 4182002C  beq 0x82e716f8
	if ctx.cr[0].eq {
	pc = 0x82E716F8; continue 'dispatch;
	}
	// 82E716D0: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E716D4: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E716D8: 556B0524  rlwinm r11, r11, 0, 0x14, 0x12
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82E716DC: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82E716E0: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E716E4: 2F0A0004  cmpwi cr6, r10, 4
	ctx.cr[6].compare_i32(ctx.r[10].s32, 4, &mut ctx.xer);
	// 82E716E8: 409A0010  bne cr6, 0x82e716f8
	if !ctx.cr[6].eq {
	pc = 0x82E716F8; continue 'dispatch;
	}
	// 82E716EC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E716F0: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82E716F4: 4800000C  b 0x82e71700
	pc = 0x82E71700; continue 'dispatch;
	// 82E716F8: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 82E716FC: 396B7EA8  addi r11, r11, 0x7ea8
	ctx.r[11].s64 = ctx.r[11].s64 + 32424;
	// 82E71700: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82E71704: 386AD4F4  addi r3, r10, -0x2b0c
	ctx.r[3].s64 = ctx.r[10].s64 + -11020;
	// 82E71708: 4800001C  b 0x82e71724
	pc = 0x82E71724; continue 'dispatch;
	// 82E7170C: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E71710: 5469B2B6  rlwinm r9, r3, 0x16, 0xa, 0x1b
	ctx.r[9].u64 = ctx.r[3].u32 as u64 & 0x000003FFu64;
	// 82E71714: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E71718: 386B06BC  addi r3, r11, 0x6bc
	ctx.r[3].s64 = ctx.r[11].s64 + 1724;
	// 82E7171C: 7D69502E  lwzx r11, r9, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82E71720: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82E71724: 917A0000  stw r11, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E71728: 4BFFFF20  b 0x82e71648
	pc = 0x82E71648; continue 'dispatch;
	// 82E7172C: 817E001C  lwz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E71730: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E71734: 41820014  beq 0x82e71748
	if ctx.cr[0].eq {
	pc = 0x82E71748; continue 'dispatch;
	}
	// 82E71738: 546A5D7A  rlwinm r10, r3, 0xb, 0x15, 0x1d
	ctx.r[10].u64 = ctx.r[3].u32 as u64 & 0x001FFFFFu64;
	// 82E7173C: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E71740: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82E71744: 4800000C  b 0x82e71750
	pc = 0x82E71750; continue 'dispatch;
	// 82E71748: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 82E7174C: 396B7EA8  addi r11, r11, 0x7ea8
	ctx.r[11].s64 = ctx.r[11].s64 + 32424;
	// 82E71750: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82E71754: 386AD4EC  addi r3, r10, -0x2b14
	ctx.r[3].s64 = ctx.r[10].s64 + -11028;
	// 82E71758: 4BFFFFCC  b 0x82e71724
	pc = 0x82E71724; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


